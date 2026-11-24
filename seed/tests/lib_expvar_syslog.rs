//! Integration tests for Effect.Expvar (publishable live metrics and /debug/vars JSON)
//! and Effect.Syslog (RFC 3164 / RFC 5424 formatting, custom sink, Effect.Log bridge,
//! and live loopback UDP and TCP syslog server capture).
mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode};
use std::io::Read;
use std::net::{TcpListener, UdpSocket};
use std::sync::mpsc;
use std::time::Duration;

/// All source-distributed modules required by Effect.Expvar and Effect.Syslog.
fn expvar_syslog_modules<'a>(main: &'a str) -> Vec<(&'a str, &'a str)> {
    vec![
        (
            "Data.Bytes.Build.cinch",
            include_str!("../prelude/Data/Bytes/Build.cinch"),
        ),
        (
            "Data.IpAddr.cinch",
            include_str!("../prelude/Data/IpAddr.cinch"),
        ),
        (
            "Text.Uri.cinch",
            include_str!("../prelude/Text/Uri.cinch"),
        ),
        (
            "Text.Punycode.cinch",
            include_str!("../prelude/Text/Punycode.cinch"),
        ),
        (
            "Text.Cookie.cinch",
            include_str!("../prelude/Text/Cookie.cinch"),
        ),
        (
            "Text.TextProto.cinch",
            include_str!("../prelude/Text/TextProto.cinch"),
        ),
        (
            "Text.Url.cinch",
            include_str!("../prelude/Text/Url.cinch"),
        ),
        (
            "Text.Url.Codec.cinch",
            include_str!("../prelude/Text/Url/Codec.cinch"),
        ),
        (
            "Text.Url.Error.cinch",
            include_str!("../prelude/Text/Url/Error.cinch"),
        ),
        (
            "Text.Url.Ip.cinch",
            include_str!("../prelude/Text/Url/Ip.cinch"),
        ),
        (
            "Text.Url.Model.cinch",
            include_str!("../prelude/Text/Url/Model.cinch"),
        ),
        (
            "Text.Url.Parser.cinch",
            include_str!("../prelude/Text/Url/Parser.cinch"),
        ),
        (
            "Text.Url.Query.cinch",
            include_str!("../prelude/Text/Url/Query.cinch"),
        ),
        (
            "Text.Url.Resolve.cinch",
            include_str!("../prelude/Text/Url/Resolve.cinch"),
        ),
        (
            "Text.Url.Scan.cinch",
            include_str!("../prelude/Text/Url/Scan.cinch"),
        ),
        (
            "Text.Url.Userinfo.cinch",
            include_str!("../prelude/Text/Url/Userinfo.cinch"),
        ),
        (
            "Text.Http.cinch",
            include_str!("../prelude/Text/Http.cinch"),
        ),
        (
            "Data.Mutable.Var.cinch",
            include_str!("../prelude/Data/Mutable/Var.cinch"),
        ),
        (
            "Effect.Net.cinch",
            include_str!("../prelude/Effect/Net.cinch"),
        ),
        (
            "Effect.Log.cinch",
            include_str!("../prelude/Effect/Log.cinch"),
        ),
        (
            "Effect.Expvar.cinch",
            include_str!("../prelude/Effect/Expvar.cinch"),
        ),
        (
            "Effect.Syslog.cinch",
            include_str!("../prelude/Effect/Syslog.cinch"),
        ),
        ("Main.cinch", main),
    ]
}

/// Build `Main.cinch` at the requested optimization level and return the executable path.
fn build_at_opt(dir: &std::path::Path, out_name: &str, opt_level: u8) -> std::path::PathBuf {
    let exe = dir.join(out_name);
    let profile = TargetProfile {
        reclaim: true,
        ..TargetProfile::native()
    };
    let opts = DriverOptions {
        opt_level: Some(opt_level),
        ..DriverOptions::default()
    };
    let code = common::drive_with_profile_opts(
        Mode::Build,
        StageMode::Refresh,
        dir,
        "Main.cinch",
        Some(exe.clone()),
        &profile,
        &opts,
    )
    .unwrap_or_else(|e| panic!("build at -O{opt_level} failed: {e}"));
    assert_eq!(code, 0, "driver build returned {code} at -O{opt_level}");
    exe
}

/// Run a compiled executable with `CINCH_LEAK_CHECK=1`, asserting exit code 0 and zero leaks.
fn run_leak_checked(exe: &std::path::Path, label: &str) {
    let output = std::process::Command::new(exe)
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .unwrap_or_else(|e| panic!("execute {label}: {e}"));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{label} failed with exit={:?}, stderr={stderr}",
        output.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{label} reported memory leaks: {stderr}"
    );
}

/// Every declaration in Effect.Expvar and Effect.Syslog carries canonical documentation.
#[test]
fn expvar_and_syslog_have_canonical_documentation() {
    common::assert_documented(
        "Effect.Expvar",
        include_str!("../prelude/Effect/Expvar.cinch"),
    );
    common::assert_documented(
        "Effect.Syslog",
        include_str!("../prelude/Effect/Syslog.cinch"),
    );
}

/// Exercise all 5 Effect.Expvar variable types, duplicate publish rejection, explicit registry
/// isolation, nested maps, dynamic callbacks, sorted JSON snapshots, and /debug/vars HTTP
/// responses at both -O0 and -O2 with leak checking.
#[test]
fn expvar_metrics_registry_and_http_response() {
    let main = r#"-- | Runtime tests for Effect.Expvar.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Int
  import Data.Json
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Expvar
  import Text.Http (resp-status, resp-header, resp-body, http-render-response, http-parse-response)
  import Text.Json (parse-json, json-eq)
  import Text.Parse (ParseOutcome, Parsed, Failed)

  -- | Extract keys from a list of (String, a) pairs.
  pair-keys :: forall a. List (String, a) -> List String
  pair-keys xs = case xs of
    Nil -> Nil
    Cons p rest -> case p of (k, _) -> Cons k (pair-keys rest)

  -- | Compare two lists of strings for exact equality.
  str-list-eq :: List String -> List String -> Bool
  str-list-eq xs ys = case xs of
    Nil -> case ys of Nil -> True; _ -> False
    Cons a ra -> case ys of
      Nil -> False
      Cons b rb -> a == b && str-list-eq ra rb

  -- | Run all Expvar assertions within one mutable region.
  run-expvar :: forall (r :: Region). Unit ->{Mut r} Int
  run-expvar u =
    let reg = expvar-registry-new () in
    let reg2 = expvar-registry-new () in
    case expvar-publish-int reg "zeta" of
      Err _ -> 1
      Ok iv ->
        let u1 = expvar-int-add iv 10 in
        let u2 = expvar-int-add iv (wrapping-neg-i64 3) in
        let ok-int = eq-i64 (expvar-int-value iv) 7 in
        let u3 = expvar-int-set iv 42 in
        let ok-int2 = eq-i64 (expvar-int-value iv) 42 in
        let ok-dup = (case expvar-publish reg "zeta" (VarInt (expvar-new-int 99)) of
          Err ExpvarAlreadyPublished -> not (expvar-error-message ExpvarAlreadyPublished == "")
          _ -> False) in
        let ok-dup2 = (case expvar-publish-int reg "zeta" of
          Err ExpvarAlreadyPublished -> True
          _ -> False) in
        let ok-iso = (case expvar-get reg2 "zeta" of None -> True; Some _ -> False) in
        case expvar-publish-float reg "alpha" of
          Err _ -> 2
          Ok fv ->
            let uf1 = expvar-float-set fv (div-f64 3 2) in
            let uf2 = expvar-float-add fv (i64-to-f64 2) in
            let ok-flt = feq-f64 (expvar-float-value fv) (div-f64 7 2) in
            case expvar-publish-string reg "middle" of
              Err _ -> 3
              Ok sv ->
                let us1 = expvar-string-set sv "hello \"world\"" in
                let ok-str = expvar-string-value sv == "hello \"world\"" in
                case expvar-publish-map reg "beta" of
                  Err _ -> 4
                  Ok mv ->
                    let um1 = expvar-map-add mv "hits" 5 in
                    let um2 = expvar-map-add mv "hits" 7 in
                    let um3 = expvar-map-add-float mv "avg" (div-f64 5 2) in
                    let um4 = expvar-map-add-float mv "avg" (div-f64 3 2) in
                    let um5 = expvar-map-set mv "mode" (VarString (expvar-new-string "prod")) in
                    let um6 = expvar-map-add mv "mode" 99 in
                    let um7 = expvar-map-add-float mv "hits" (i64-to-f64 1) in
                    let sub = expvar-new-map () in
                    let usub = expvar-map-add sub "k" 3 in
                    let um8 = expvar-map-set mv "nested" (VarMap sub) in
                    let um9 = expvar-map-set mv "temp" (VarInt (expvar-new-int 1)) in
                    let um10 = expvar-map-delete mv "temp" in
                    let ok-map-len = expvar-map-len mv == 4 in
                    let ok-map-keys = str-list-eq (pair-keys (expvar-map-entries mv)) ["avg", "hits", "mode", "nested"] in
                    let ok-map-get = (case expvar-map-get mv "hits" of
                      Some (VarInt h) -> eq-i64 (expvar-int-value h) 12
                      _ -> False) in
                    let tmp-map = expvar-new-map () in
                    let utm = expvar-map-add tmp-map "x" 1 in
                    let tmp-map2 = expvar-map-init tmp-map in
                    let ok-init = expvar-map-len tmp-map2 == 0 in
                    case expvar-publish-func reg "dynamic" (\_ -> JNum (int-from-i64 (wrapping-mul-i64 (expvar-int-value iv) 2))) of
                      Err _ -> 5
                      Ok _ ->
                        let ok-entries = str-list-eq (pair-keys (expvar-entries reg)) ["alpha", "beta", "dynamic", "middle", "zeta"] in
                        let snap1 = expvar-snapshot-string reg in
                        let want1 = "{\"alpha\":3.5e0,\"beta\":{\"avg\":4e0,\"hits\":12,\"mode\":\"prod\",\"nested\":{\"k\":3}},\"dynamic\":84,\"middle\":\"hello \\\"world\\\"\",\"zeta\":42}" in
                        let ok-snap1 = snap1 == want1 in
                        let u4 = expvar-int-add iv 8 in
                        let snap2 = expvar-snapshot-string reg in
                        let want2 = "{\"alpha\":3.5e0,\"beta\":{\"avg\":4e0,\"hits\":12,\"mode\":\"prod\",\"nested\":{\"k\":3}},\"dynamic\":100,\"middle\":\"hello \\\"world\\\"\",\"zeta\":50}" in
                        let ok-dyn = snap2 == want2 in
                        let resp = expvar-http-response reg in
                        let ok-http = (resp-status resp == 200
                          && (case resp-header resp "Content-Type" of Some ct -> ct == "application/json; charset=utf-8"; None -> False)
                          && (case string-from-bytes (resp-body resp) of Some b -> b == snap2; None -> False)) in
                        let wire = http-render-response resp in
                        let ok-wire = (case http-parse-response wire of
                          Ok (parsed-resp, _) ->
                            resp-status parsed-resp == 200
                            && (case parse-json "vars.json" (resp-body parsed-resp) of
                              Parsed j _ -> json-eq j (expvar-snapshot-json reg)
                              Failed _ -> False)
                          Err _ -> False) in
                        let ok-get-func = (case expvar-get reg "dynamic" of
                          Some dv -> expvar-var-string dv == "100"
                          None -> False) in
                        if not ok-int then 10
                        else if not ok-int2 then 11
                        else if not ok-dup then 12
                        else if not ok-dup2 then 13
                        else if not ok-iso then 14
                        else if not ok-flt then 15
                        else if not ok-str then 16
                        else if not ok-map-len then 17
                        else if not ok-map-keys then 18
                        else if not ok-map-get then 19
                        else if not ok-init then 20
                        else if not ok-entries then 21
                        else if not ok-snap1 then 22
                        else if not ok-dyn then 23
                        else if not ok-http then 24
                        else if not ok-wire then 25
                        else if not ok-get-func then 26
                        else 0

  -- | Entry point running Expvar verification.
  main :: forall (r :: Region). Unit ->{Mut r} Int
  main u = run-expvar u
"#;

    let dir = common::project("expvar_suite", &expvar_syslog_modules(main));
    for opt in [0u8, 2u8] {
        let exe = build_at_opt(&dir, &format!("expvar_O{opt}"), opt);
        run_leak_checked(&exe, &format!("expvar -O{opt}"));
    }
    common::cleanup(&dir);
}

/// Exercise pure Effect.Syslog facilities, severities, priority roundtrips, UTC timestamps,
/// RFC 3164 / RFC 5424 formatting, CR/LF sanitization, and Effect.Log level mapping.
#[test]
fn syslog_pure_formatting_priorities_and_timestamps() {
    let main = r#"-- | Pure formatting and priority tests for Effect.Syslog.
module Main where
  import Data.Bool
  import Data.List
  import Data.Nat
  import Data.Result
  import Data.String
  import Effect.Log (Debug, Info, Warn, Error)
  import Effect.Syslog

  -- | Check priority encoding and roundtrip decoding for one facility/severity pair.
  check-pri-pair :: SyslogFacility -> SyslogSeverity -> Nat -> Bool
  check-pri-pair fac sev want =
    syslog-priority fac sev == want
    && syslog-priority-code (SyslogPriority fac sev) == want
    && (case syslog-parse-priority want of
      Ok (f2, s2) -> f2 == fac && s2 == sev
      Err _ -> False)

  -- | Run all pure Syslog checks.
  main :: Unit -> Int
  main u =
    let ok-facs = (syslog-facility-code LogKern == 0
      && syslog-facility-code LogUser == 8
      && syslog-facility-code LogMail == 16
      && syslog-facility-code LogDaemon == 24
      && syslog-facility-code LogAuth == 32
      && syslog-facility-code LogSyslog == 40
      && syslog-facility-code LogLpr == 48
      && syslog-facility-code LogNews == 56
      && syslog-facility-code LogUucp == 64
      && syslog-facility-code LogCron == 72
      && syslog-facility-code LogAuthpriv == 80
      && syslog-facility-code LogFtp == 88
      && syslog-facility-code LogLocal0 == 128
      && syslog-facility-code LogLocal1 == 136
      && syslog-facility-code LogLocal2 == 144
      && syslog-facility-code LogLocal3 == 152
      && syslog-facility-code LogLocal4 == 160
      && syslog-facility-code LogLocal5 == 168
      && syslog-facility-code LogLocal6 == 176
      && syslog-facility-code LogLocal7 == 184) in
    let ok-sevs = (syslog-severity-code LogEmerg == 0
      && syslog-severity-code LogAlert == 1
      && syslog-severity-code LogCrit == 2
      && syslog-severity-code LogErr == 3
      && syslog-severity-code LogWarning == 4
      && syslog-severity-code LogNotice == 5
      && syslog-severity-code LogInfo == 6
      && syslog-severity-code LogDebug == 7) in
    let ok-pris = (check-pri-pair LogKern LogEmerg 0
      && check-pri-pair LogUser LogInfo 14
      && check-pri-pair LogDaemon LogErr 27
      && check-pri-pair LogAuthpriv LogAlert 81
      && check-pri-pair LogLocal0 LogInfo 134
      && check-pri-pair LogLocal7 LogDebug 191) in
    let ok-bad-pri = (case syslog-parse-priority 192 of
      Err SyslogInvalidPriority -> not (syslog-error-message SyslogInvalidPriority == "")
      _ -> False) in
    let ok-bad-pri2 = (case syslog-parse-priority 100 of
      Err SyslogInvalidPriority -> True
      _ -> False) in
    let ok-ts = (syslog-timestamp-rfc3164 0 == "Jan  1 00:00:00"
      && syslog-timestamp-rfc5424 0 == "1970-01-01T00:00:00Z"
      && syslog-timestamp-rfc3164 1709210096000000000 == "Feb 29 12:34:56"
      && syslog-timestamp-rfc5424 1709210096000000000 == "2024-02-29T12:34:56Z") in
    let ok-rfc3164 = (syslog-format-rfc3164 LogLocal0 LogInfo "Nov 24 19:03:54" "web01" "myapp" 4242 "line1\r\nline2\n"
      == "<134>Nov 24 19:03:54 web01 myapp[4242]: line1 line2") in
    let ok-rfc3164-nopid = (syslog-format-rfc3164 LogDaemon LogErr "Nov 24 19:03:54" "web01" "daemon" 0 "disk full"
      == "<27>Nov 24 19:03:54 web01 daemon: disk full") in
    let ok-rfc5424 = (syslog-format-rfc5424 LogLocal0 LogInfo "2026-11-24T19:03:54Z" "web01" "myapp" "4242" "ID47" "hello\nworld\r\n"
      == "<134>1 2026-11-24T19:03:54Z web01 myapp 4242 ID47 - hello world") in
    let ok-rfc5424-nil = (syslog-format-rfc5424 LogUser LogInfo "" "" "" "" "" "minimal"
      == "<14>1 - - - - - - minimal") in
    let ok-rfc5424-empty = (syslog-format-rfc5424 LogUser LogNotice "2026-11-24T19:03:54Z" "h" "a" "1" "m" ""
      == "<13>1 2026-11-24T19:03:54Z h a 1 m -") in
    let ok-lvl = (syslog-level-from-log Debug == LogDebug
      && syslog-level-from-log Info == LogInfo
      && syslog-level-from-log Warn == LogWarning
      && syslog-level-from-log Error == LogErr) in
    if not ok-facs then 1
    else if not ok-sevs then 2
    else if not ok-pris then 3
    else if not ok-bad-pri then 4
    else if not ok-bad-pri2 then 5
    else if not ok-ts then 6
    else if not ok-rfc3164 then 7
    else if not ok-rfc3164-nopid then 8
    else if not ok-rfc5424 then 9
    else if not ok-rfc5424-nil then 10
    else if not ok-rfc5424-empty then 11
    else if not ok-lvl then 12
    else 0
"#;

    let dir = common::project("syslog_pure", &expvar_syslog_modules(main));
    for opt in [0u8, 2u8] {
        let exe = build_at_opt(&dir, &format!("syslog_pure_O{opt}"), opt);
        run_leak_checked(&exe, &format!("syslog_pure -O{opt}"));
    }
    common::cleanup(&dir);
}

/// Exercise custom sink capture, all 8 severity helpers, Effect.Log bridge, and real loopback
/// UDP and TCP syslog server capture (including post-close Err SyslogClosed) at -O0 and -O2
/// with leak checking.
#[test]
fn syslog_sink_log_bridge_and_live_udp_tcp_loopback() {
    for opt in [0u8, 2u8] {
        let udp_rx = UdpSocket::bind("127.0.0.1:0").expect("bind loopback UDP syslog server");
        let udp_port = udp_rx.local_addr().unwrap().port();

        let tcp_srv = TcpListener::bind("127.0.0.1:0").expect("bind loopback TCP syslog server");
        let tcp_port = tcp_srv.local_addr().unwrap().port();

        let dir = common::project(&format!("syslog_live_O{opt}"), &[]);
        let sink_path = dir.join("captured_sink.log");
        let sink_path_str = sink_path.to_str().expect("UTF-8 sink path");

        let main = format!(
            r#"-- | Live UDP, TCP, sink, and Effect.Log bridge tests for Effect.Syslog.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Path (path-from-string)
  import Data.Result
  import Data.String
  import Effect.Fs (fs-read, fs-write)
  import Effect.Log (Info, Warn, Error, fields, num, flag)
  import Effect.Net (NetInvalidArgument)
  import Effect.Syslog

  -- | Check whether haystack contains needle.
  contains-str :: String -> String -> Bool
  contains-str hay needle =
    let bh = string-to-bytes hay in
    let bn = string-to-bytes needle in
    contains-loop bh bn 0

  -- | Linear substring search over bytes.
  contains-loop :: Bytes -> Bytes -> Nat -> Bool
  contains-loop bh bn i =
    let nh = bytes-len bh in
    let nn = bytes-len bn in
    if nn == 0 then True
    else if i + nn > nh then False
    else
      let sub = bytes-new nn in
      let u = bytes-copy-range sub bh 0 i nn in
      if eq-i64 (bytes-compare sub bn) 0 then True
      else contains-loop bh bn (i + 1)

  -- | Append one captured syslog line to a file under IO r.
  append-sink-line :: forall (r :: Region). using (io :: Io r) => String -> String ->{{IO r}} Result SyslogError Unit
  append-sink-line path-str line =
    let p = path-from-string path-str in
    let prev = (case fs-read p of
      Ok bs -> (case string-from-bytes bs of Some s -> s; None -> "")
      Err _ -> "") in
    let next = string-concat-all [prev, line, "\n"] in
    case fs-write p (string-to-bytes next) of
      Ok _ -> Ok ()
      Err _ -> Err SyslogClosed

  -- | Exercise sink, UDP, TCP, close, and Effect.Log bridge.
  main :: forall (r :: Region). Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    let sink-path = "{sink_path_str}" in
    let w-sink = syslog-with-pid 777 (syslog-with-hostname "node1" (syslog-with-sink LogLocal1 "svc" SyslogRfc3164 (append-sink-line sink-path))) in
    let r1 = syslog-emerg w-sink "e0" in
    let r2 = syslog-alert w-sink "e1" in
    let r3 = syslog-crit w-sink "e2" in
    let r4 = syslog-err w-sink "e3" in
    let r5 = syslog-warning w-sink "e4" in
    let r6 = syslog-notice w-sink "e5" in
    let r7 = syslog-info w-sink "e6" in
    let r8 = syslog-debug w-sink "e7" in
    let w-sink-rfc5424 = syslog-with-msgid "LOG1" (syslog-with-format SyslogRfc5424 w-sink) in
    let r9 = syslog-emit-log-record w-sink-rfc5424 Warn "slow query" (fields [("ms", num 250), ("ok", flag False)]) in
    let sink-ok = (case fs-read (path-from-string sink-path) of
      Err _ -> False
      Ok bs -> case string-from-bytes bs of
        None -> False
        Some txt ->
          contains-str txt "<136>"
          && contains-str txt "node1 svc[777]: e0"
          && contains-str txt "<137>"
          && contains-str txt "<138>"
          && contains-str txt "<139>"
          && contains-str txt "<140>"
          && contains-str txt "<141>"
          && contains-str txt "<142>"
          && contains-str txt "<143>"
          && contains-str txt "node1 svc[777]: e7"
          && contains-str txt "<140>1 "
          && contains-str txt " node1 svc 777 LOG1 - {{\"fields\":{{\"ms\":250,\"ok\":false}},\"level\":\"warn\",\"message\":\"slow query\"}}") in
    let bad-udp = (case syslog-dial-udp "127.0.0.1" 0 LogLocal0 "app" of
      Err (SyslogNetError NetInvalidArgument) -> True
      _ -> False) in
    let bad-udp2 = (case syslog-dial-udp "127.0.0.1" 70000 LogLocal0 "app" of
      Err (SyslogNetError NetInvalidArgument) -> True
      _ -> False) in
    let udp-ok = (case syslog-dial-udp "127.0.0.1" {udp_port} LogLocal0 "udptest" of
      Err _ -> False
      Ok wu0 ->
        let wu1 = syslog-with-pid 1234 (syslog-with-hostname "udphost" wu0) in
        let a = syslog-info wu1 "udp-rfc3164-msg" in
        let wu2 = syslog-with-msgid "U54" (syslog-with-format SyslogRfc5424 wu1) in
        let b = syslog-err wu2 "udp-rfc5424-msg" in
        let c = syslog-emit-log-record wu2 Error "udp-log-bridge" (fields [("code", num 500)]) in
        let d = syslog-close wu2 in
        (case (a, b, c, d) of (Ok _, Ok _, Ok _, Ok _) -> True; _ -> False)) in
    let tcp-ok = (case syslog-dial-tcp "127.0.0.1" {tcp_port} LogLocal2 "tcptest" of
      Err _ -> False
      Ok wt0 ->
        let wt1 = syslog-with-pid 5678 (syslog-with-hostname "tcphost" wt0) in
        let a = syslog-notice wt1 "tcp-first\nline" in
        let wt2 = syslog-with-msgid "T54" (syslog-with-format SyslogRfc5424 wt1) in
        let b = syslog-emit-log-record wt2 Info "tcp-started" (fields [("port", num 8080)]) in
        let c = syslog-close wt2 in
        let after-close = (case syslog-info wt2 "should-fail" of
          Err SyslogClosed -> True
          _ -> False) in
        (case (a, b, c) of (Ok _, Ok _, Ok _) -> after-close; _ -> False)) in
    if not sink-ok then 10
    else if not bad-udp then 11
    else if not bad-udp2 then 12
    else if not udp-ok then 13
    else if not tcp-ok then 14
    else 0
"#
        );

        for (name, src) in expvar_syslog_modules(&main) {
            std::fs::write(dir.join(name), src).expect("write module");
        }
        let exe = build_at_opt(&dir, &format!("syslog_live_O{opt}"), opt);

        let (udp_tx, udp_rx_chan) = mpsc::channel::<Vec<String>>();
        std::thread::spawn(move || {
            let _ = udp_rx.set_read_timeout(Some(Duration::from_secs(10)));
            let mut msgs = Vec::new();
            let mut buf = [0u8; 65535];
            for _ in 0..3 {
                match udp_rx.recv(&mut buf) {
                    Ok(n) => msgs.push(String::from_utf8_lossy(&buf[..n]).into_owned()),
                    Err(_) => break,
                }
            }
            let _ = udp_tx.send(msgs);
        });

        let (tcp_tx, tcp_rx_chan) = mpsc::channel::<String>();
        std::thread::spawn(move || {
            let mut received = String::new();
            if let Ok((mut stream, _)) = tcp_srv.accept() {
                let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
                let mut bytes = Vec::new();
                let _ = stream.read_to_end(&mut bytes);
                received = String::from_utf8_lossy(&bytes).into_owned();
            }
            let _ = tcp_tx.send(received);
        });

        run_leak_checked(&exe, &format!("syslog_live -O{opt}"));

        let udp_msgs = udp_rx_chan
            .recv_timeout(Duration::from_secs(10))
            .expect("receive captured UDP syslog datagrams");
        assert_eq!(udp_msgs.len(), 3, "UDP msgs at -O{opt}: {udp_msgs:?}");
        assert!(
            udp_msgs[0].starts_with("<134>")
                && udp_msgs[0].ends_with(" udphost udptest[1234]: udp-rfc3164-msg"),
            "unexpected UDP msg 0: {}",
            udp_msgs[0]
        );
        assert!(
            udp_msgs[1].starts_with("<131>1 ")
                && udp_msgs[1].ends_with(" udphost udptest 1234 U54 - udp-rfc5424-msg"),
            "unexpected UDP msg 1: {}",
            udp_msgs[1]
        );
        assert!(
            udp_msgs[2].starts_with("<131>1 ")
                && udp_msgs[2].contains(
                    "\"fields\":{\"code\":500},\"level\":\"error\",\"message\":\"udp-log-bridge\""
                ),
            "unexpected UDP msg 2: {}",
            udp_msgs[2]
        );

        let tcp_raw = tcp_rx_chan
            .recv_timeout(Duration::from_secs(10))
            .expect("receive captured TCP syslog stream");
        assert!(
            tcp_raw.ends_with('\n'),
            "TCP stream must be newline-framed: {tcp_raw:?}"
        );
        let tcp_lines: Vec<&str> = tcp_raw.trim_end_matches('\n').split('\n').collect();
        assert_eq!(tcp_lines.len(), 2, "TCP lines at -O{opt}: {tcp_lines:?}");
        assert!(
            tcp_lines[0].starts_with("<149>")
                && tcp_lines[0].ends_with(" tcphost tcptest[5678]: tcp-first line"),
            "unexpected TCP line 0: {}",
            tcp_lines[0]
        );
        assert!(
            tcp_lines[1].starts_with("<150>1 ")
                && tcp_lines[1].contains(
                    "\"fields\":{\"port\":8080},\"level\":\"info\",\"message\":\"tcp-started\""
                ),
            "unexpected TCP line 1: {}",
            tcp_lines[1]
        );

        common::cleanup(&dir);
    }
}

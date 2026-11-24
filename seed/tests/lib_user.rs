//! Consumer semantics for `Text.User` and `Effect.User`:
//!   1. Canonical documentation on every declaration in both modules.
//!   2. Pure parsing and lookup of `/etc/passwd`, `/etc/group`, and Linux
//!      `/proc/self/status` records (`parse-passwd`, `parse-group`,
//!      `parse-proc-status`, `lookup-user-in`, `lookup-user-id-in`,
//!      `lookup-group-in`, `lookup-group-id-in`, `user-group-ids-in`,
//!      `user-error-message`, and structural equality).
//!   3. Hermetic file-backed lookups against temporary passwd and group files
//!      (`user-lookup-in-file`, `user-lookup-id-in-file`,
//!      `user-lookup-group-in-file`, `user-lookup-group-id-in-file`,
//!      `user-group-ids-in-file`, and missing-file `UserIoError`).
//!   4. Live operating system lookups (`user-current`, `user-lookup`,
//!      `user-lookup-id`, `user-lookup-group`, `user-lookup-group-id`,
//!      `user-group-ids`) cross-checked against host `id` and `HOME`.

mod common;

use std::path::Path;
use std::process::Command;

const TEXT_USER: &str = include_str!("../prelude/Text/User.cinch");
const EFFECT_USER: &str = include_str!("../prelude/Effect/User.cinch");

fn run_user_program(tag: &str, main_src: &str) -> i32 {
    let dir = common::project(
        tag,
        &[
            ("Text.User.cinch", TEXT_USER),
            ("Effect.User.cinch", EFFECT_USER),
            ("Main.cinch", main_src),
        ],
    );
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    code
}

fn cinch_str(s: &str) -> String {
    let mut out = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn cinch_path(p: &Path) -> String {
    cinch_str(&p.to_string_lossy())
}

fn cmd_stdout(prog: &str, args: &[&str]) -> String {
    let out = Command::new(prog)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {prog} {args:?}: {e}"));
    assert!(
        out.status.success(),
        "{prog} {args:?} exited with {:?}",
        out.status
    );
    String::from_utf8(out.stdout)
        .expect("UTF-8 command output")
        .trim()
        .to_string()
}

/// Every declaration in `Text.User` carries canonical non-legacy documentation.
#[test]
fn text_user_declarations_have_canonical_documentation() {
    common::assert_documented("Text.User", TEXT_USER);
}

/// Every declaration in `Effect.User` carries canonical non-legacy documentation.
#[test]
fn effect_user_declarations_have_canonical_documentation() {
    common::assert_documented("Effect.User", EFFECT_USER);
}

/// Pure parsing and lookup over passwd, group, and `/proc/self/status` text.
#[test]
fn user_pure_passwd_group_and_proc_status_parsing() {
    let main_src = r##"
module Main where

  import Data.Bool
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Relation.Eq
  import Text.User

  -- | Compare two string lists for exact equality.
  eq-strs :: List String -> List String -> Bool
  eq-strs xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons _ _ -> False
    Cons x xr -> case ys of
      Nil -> False
      Cons y yr -> if x == y then eq-strs xr yr else False

  -- | Check passwd parsing, GECOS trimming, CRLF handling, and comment/malformed skipping.
  check-passwd :: Unit -> Bool
  check-passwd u =
    let sample = "# comment\n   # indented comment\n\nroot:x:0:0:root:/root:/bin/bash\nalice:x:1000:1000:Alice Example,Room 42,555-0100,:/home/alice:/bin/zsh\r\nbob:x:1001:1001:Bob Builder:/home/bob\ncharlie:x:1002:1002:,Office 2:/home/charlie:\nshort:x:1003\n:x:1004:1004:NoName:/home/x:/bin/sh\nnouid:x::1005:NoUid:/home/y:/bin/sh\nsoren:x:1006:1006:Søren Kierkegaard,Copenhagen:/home/soren:/bin/sh" in
    let users = parse-passwd sample in
    let ok-len = list-length users == 5 in
    let ok-alice = (
      case lookup-user-in users "alice" of
        Ok a ->
          user-uid a == "1000"
            && user-gid a == "1000"
            && user-username a == "alice"
            && user-name a == "Alice Example"
            && user-home-dir a == "/home/alice"
            && user-shell a == "/bin/zsh"
        Err _ -> False
    ) in
    let ok-bob = (
      case lookup-user-id-in users "1001" of
        Ok b ->
          user-username b == "bob"
            && user-name b == "Bob Builder"
            && user-home-dir b == "/home/bob"
            && user-shell b == ""
        Err _ -> False
    ) in
    let ok-charlie = (
      case lookup-user-in users "charlie" of
        Ok c -> user-name c == "" && user-shell c == ""
        Err _ -> False
    ) in
    let ok-soren = (
      case lookup-user-id-in users "1006" of
        Ok s -> user-username s == "soren" && user-name s == "Søren Kierkegaard"
        Err _ -> False
    ) in
    let ok-miss-user = (
      case lookup-user-in users "missing" of
        Err e -> e == UnknownUserError "missing" && user-error-message e == "user: unknown user missing"
        Ok _ -> False
    ) in
    let ok-miss-uid = (
      case lookup-user-id-in users "99999" of
        Err e -> e == UnknownUserIdError "99999" && user-error-message e == "user: unknown userid 99999"
        Ok _ -> False
    ) in
    ok-len && ok-alice && ok-bob && ok-charlie && ok-soren && ok-miss-user && ok-miss-uid

  -- | Check group parsing, lookup, and deduplicated user-group-ids-in.
  check-group :: Unit -> Bool
  check-group u =
    let sample = "# groups\n\nroot:x:0:\nstaff:x:50:alice,bob\r\nwheel:x:10:,alice,,carol,\nprimary:x:1000:alice\ndupwheel:x:10:alice\nthreefield:x:77\nbad:line\n:x:88:alice" in
    let groups = parse-group sample in
    let ok-len = list-length groups == 6 in
    let ok-wheel = (
      case lookup-group-in groups "wheel" of
        Ok g -> group-gid g == "10" && group-name g == "wheel" && eq-strs (group-members g) ["alice", "carol"]
        Err _ -> False
    ) in
    let ok-three = (
      case lookup-group-id-in groups "77" of
        Ok g -> group-name g == "threefield" && eq-strs (group-members g) Nil
        Err _ -> False
    ) in
    let ok-root = (
      case lookup-group-in groups "root" of
        Ok g -> group-gid g == "0" && eq-strs (group-members g) Nil
        Err _ -> False
    ) in
    let ok-miss-g = (
      case lookup-group-in groups "nogrp" of
        Err e -> e == UnknownGroupError "nogrp" && user-error-message e == "group: unknown group nogrp"
        Ok _ -> False
    ) in
    let ok-miss-gid = (
      case lookup-group-id-in groups "424242" of
        Err e -> e == UnknownGroupIdError "424242" && user-error-message e == "group: unknown groupid 424242"
        Ok _ -> False
    ) in
    let alice = UserInfo "1000" "1000" "alice" "Alice" "/home/alice" "/bin/sh" in
    let ok-gids = eq-strs (user-group-ids-in alice groups) ["1000", "50", "10"] in
    let ok-io-err = user-error-message (UserIoError "permission denied") == "user: permission denied" in
    ok-len && ok-wheel && ok-three && ok-root && ok-miss-g && ok-miss-gid && ok-gids && ok-io-err

  -- | Check Linux /proc/self/status parsing across full, empty-groups, and malformed inputs.
  check-proc-status :: Unit -> Bool
  check-proc-status u =
    let full = "Name:\tcinch\nState:\tR (running)\nUid:\t1000\t0\t0\t0\nGid:\t1000\t42\t42\t42\r\nGroups:\t27 990 1000 \n" in
    let ok-full = (
      case parse-proc-status full of
        Some p ->
          proc-real-uid p == "1000"
            && proc-effective-uid p == "0"
            && proc-real-gid p == "1000"
            && proc-effective-gid p == "42"
            && eq-strs (proc-supplementary-gids p) ["27", "990", "1000"]
            && p == ProcIdentity "1000" "0" "1000" "42" ["27", "990", "1000"]
        None -> False
    ) in
    let empty-groups = "Uid: 500 500 500 500\nGid: 500 500 500 500\nGroups:\t\n" in
    let ok-empty-g = (
      case parse-proc-status empty-groups of
        Some p -> proc-real-uid p == "500" && eq-strs (proc-supplementary-gids p) Nil
        None -> False
    ) in
    let no-groups = "Uid:\t100\t100\nGid:\t200\t200\n" in
    let ok-no-g = (
      case parse-proc-status no-groups of
        Some p -> proc-real-uid p == "100" && proc-real-gid p == "200" && eq-strs (proc-supplementary-gids p) Nil
        None -> False
    ) in
    let ok-missing-gid = (
      case parse-proc-status "Uid:\t100\t100\n" of
        None -> True
        Some _ -> False
    ) in
    let ok-missing-uid = (
      case parse-proc-status "Gid:\t100\t100\n" of
        None -> True
        Some _ -> False
    ) in
    let ok-empty = (
      case parse-proc-status "" of
        None -> True
        Some _ -> False
    ) in
    ok-full && ok-empty-g && ok-no-g && ok-missing-gid && ok-missing-uid && ok-empty

  -- | Return 0 on full pass or a bitmask naming the failed pure checks.
  main :: Int
  main =
    let c1 = if check-passwd () then 0 else 1 in
    let c2 = if check-group () then 0 else 2 in
    let c3 = if check-proc-status () then 0 else 4 in
    c1 + c2 + c3
"##;

    let code = run_user_program("user-pure", main_src);
    assert_eq!(code, 0, "user-pure: check bitmask {code}");
}

/// Hermetic file-backed lookups against custom temporary passwd and group files.
#[test]
fn user_hermetic_file_lookups() {
    let scratch = std::env::temp_dir().join(format!("cinch_user_hermetic_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).expect("create scratch dir");

    let passwd_file = scratch.join("passwd");
    let group_file = scratch.join("group");
    let missing_file = scratch.join("missing");

    std::fs::write(
        &passwd_file,
        concat!(
            "# synthetic passwd\n",
            "root:x:0:0:root:/root:/bin/bash\n",
            "alice:x:1000:1000:Alice Example,Room 1:/home/alice:/bin/zsh\r\n",
            "bob:x:1001:1001:Bob Builder:/home/bob\n",
            "soren:x:1002:1002:Søren Kierkegaard,Copenhagen:/home/soren:/bin/sh\n",
        ),
    )
    .expect("write synthetic passwd");

    std::fs::write(
        &group_file,
        concat!(
            "# synthetic group\n",
            "root:x:0:\n",
            "staff:x:50:alice,bob\r\n",
            "wheel:x:10:alice,,soren\n",
            "alice:x:1000:alice\n",
            "docker:x:990:alice\n",
        ),
    )
    .expect("write synthetic group");

    let main_src = format!(
        r#"
module Main where

  import Data.Bool
  import Data.List
  import Data.Path
  import Data.Result
  import Data.String
  import Effect.User
  import Relation.Eq
  import Text.User

  -- | Compare two string lists for exact equality.
  eq-strs :: List String -> List String -> Bool
  eq-strs xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons _ _ -> False
    Cons x xr -> case ys of
      Nil -> False
      Cons y yr -> if x == y then eq-strs xr yr else False

  -- | Run hermetic file-backed user and group lookup checks.
  checks :: forall r. using (io :: Io r) => Unit ->{{IO r}} Int
  checks u =
    let ppath = path-from-string {passwd} in
    let gpath = path-from-string {group} in
    let missing = path-from-string {missing} in
    let r1 = user-lookup-in-file ppath "alice" in
    let c1 = (
      case r1 of
        Ok a ->
          if user-uid a == "1000" && user-gid a == "1000" && user-name a == "Alice Example" && user-shell a == "/bin/zsh"
            then 0 else 1
        Err _ -> 1
    ) in
    let r2 = user-lookup-id-in-file ppath "1002" in
    let c2 = (
      case r2 of
        Ok s -> if user-username s == "soren" && user-name s == "Søren Kierkegaard" then 0 else 2
        Err _ -> 2
    ) in
    let r3 = user-lookup-group-in-file gpath "wheel" in
    let c3 = (
      case r3 of
        Ok g -> if group-gid g == "10" && eq-strs (group-members g) ["alice", "soren"] then 0 else 4
        Err _ -> 4
    ) in
    let r4 = user-lookup-group-id-in-file gpath "50" in
    let c4 = (
      case r4 of
        Ok g -> if group-name g == "staff" && eq-strs (group-members g) ["alice", "bob"] then 0 else 8
        Err _ -> 8
    ) in
    let c5 = (
      case r1 of
        Err _ -> 16
        Ok alice ->
          let gids-res = user-group-ids-in-file gpath alice in
          case gids-res of
            Ok gids -> if eq-strs gids ["1000", "50", "10", "990"] then 0 else 16
            Err _ -> 16
    ) in
    let r6 = user-lookup-in-file missing "alice" in
    let c6 = (
      case r6 of
        Err e -> case e of
          UserIoError _ -> 0
          _ -> 32
        Ok _ -> 32
    ) in
    let r7 = user-lookup-in-file ppath "unknown-person" in
    let c7 = (
      case r7 of
        Err e -> if e == UnknownUserError "unknown-person" then 0 else 64
        Ok _ -> 64
    ) in
    c1 + c2 + c3 + c4 + c5 + c6 + c7

  -- | Entry point under the IO capability.
  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    checks ()
"#,
        passwd = cinch_path(&passwd_file),
        group = cinch_path(&group_file),
        missing = cinch_path(&missing_file),
    );

    let code = run_user_program("user-hermetic", &main_src);
    let _ = std::fs::remove_dir_all(&scratch);
    assert_eq!(code, 0, "user-hermetic: check bitmask {code}");
}

/// Live OS checks matching the running process's host identity (`id -u`, `id -un`, `id -g`, `id -G`, `HOME`).
#[test]
fn user_live_system_lookups() {
    let uid = cmd_stdout("id", &["-u"]);
    let uname = cmd_stdout("id", &["-un"]);
    let gid = cmd_stdout("id", &["-g"]);
    let gname = cmd_stdout("id", &["-gn"]);
    let gids_raw = cmd_stdout("id", &["-G"]);
    let gids_list = format!(
        "[{}]",
        gids_raw
            .split_whitespace()
            .map(cinch_str)
            .collect::<Vec<_>>()
            .join(", ")
    );
    let home = std::env::var("HOME").unwrap_or_default();

    let main_src = format!(
        r#"
module Main where

  import Data.Bool
  import Data.List
  import Data.Result
  import Data.String
  import Effect.User
  import Relation.Eq
  import Text.User

  -- | Return true if `needle` occurs in `xs`.
  has-str :: String -> List String -> Bool
  has-str needle xs = case xs of
    Nil -> False
    Cons x rest -> if x == needle then True else has-str needle rest

  -- | Return true if `xs` contains no duplicate strings.
  no-dups :: List String -> Bool
  no-dups xs = case xs of
    Nil -> True
    Cons x rest -> if has-str x rest then False else no-dups rest

  -- | Return true if every element of `needles` appears in `hay`.
  all-present :: List String -> List String -> Bool
  all-present needles hay = case needles of
    Nil -> True
    Cons n rest -> if has-str n hay then all-present rest hay else False

  -- | Verify live OS account lookups against host identity.
  checks :: forall r. using (io :: Io r) => Unit ->{{IO r}} Int
  checks u =
    let cur-res = user-current () in
    case cur-res of
      Err _ -> 1
      Ok cur ->
        let exp-home = {exp_home} in
        let ok-home = string-byte-length exp-home == 0 || user-home-dir cur == exp-home in
        let c1 = (
          if user-uid cur == {exp_uid}
            && user-username cur == {exp_uname}
            && user-gid cur == {exp_gid}
            && ok-home
          then 0 else 1
        ) in
        let by-name = user-lookup {exp_uname} in
        let c2 = (
          case by-name of
            Ok info -> if info == cur then 0 else 2
            Err _ -> 2
        ) in
        let by-uid = user-lookup-id {exp_uid} in
        let c3 = (
          case by-uid of
            Ok info -> if info == cur then 0 else 4
            Err _ -> 4
        ) in
        let grp-by-name = user-lookup-group {exp_gname} in
        let grp-by-id = user-lookup-group-id {exp_gid} in
        let c4 = (
          case grp-by-name of
            Err _ -> 8
            Ok g1 -> case grp-by-id of
              Err _ -> 8
              Ok g2 -> if group-gid g1 == {exp_gid} && group-name g2 == {exp_gname} then 0 else 8
        ) in
        let gids-res = user-group-ids cur in
        let c5 = (
          case gids-res of
            Err _ -> 16
            Ok gids -> case gids of
              Nil -> 16
              Cons first _ ->
                if first == {exp_gid} && no-dups gids && all-present {exp_gids} gids
                  then 0 else 16
        ) in
        let miss-u = user-lookup "cinch_nonexistent_user_probe_999" in
        let miss-id = user-lookup-id "4294967291" in
        let miss-g = user-lookup-group "cinch_nonexistent_group_probe_999" in
        let miss-gid = user-lookup-group-id "4294967291" in
        let c6 = (
          let ok-mu = (
            case miss-u of
              Err e -> e == UnknownUserError "cinch_nonexistent_user_probe_999"
              Ok _ -> False
          ) in
          let ok-mid = (
            case miss-id of
              Err e -> e == UnknownUserIdError "4294967291"
              Ok _ -> False
          ) in
          let ok-mg = (
            case miss-g of
              Err e -> e == UnknownGroupError "cinch_nonexistent_group_probe_999"
              Ok _ -> False
          ) in
          let ok-mgid = (
            case miss-gid of
              Err e -> e == UnknownGroupIdError "4294967291"
              Ok _ -> False
          ) in
          if ok-mu && ok-mid && ok-mg && ok-mgid then 0 else 32
        ) in
        let root-res = user-lookup-id "0" in
        let c7 = (
          case root-res of
            Err _ -> 64
            Ok root-info ->
              let root-gids = user-group-ids root-info in
              case root-gids of
                Err _ -> 64
                Ok rgids ->
                  if user-username root-info == "root" && no-dups rgids && has-str "0" rgids
                    then 0 else 64
        ) in
        c1 + c2 + c3 + c4 + c5 + c6 + c7

  -- | Entry point under the IO capability.
  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    checks ()
"#,
        exp_uid = cinch_str(&uid),
        exp_uname = cinch_str(&uname),
        exp_gid = cinch_str(&gid),
        exp_gname = cinch_str(&gname),
        exp_home = cinch_str(&home),
        exp_gids = gids_list,
    );

    let code = run_user_program("user-live", &main_src);
    assert_eq!(code, 0, "user-live: check bitmask {code}");
}

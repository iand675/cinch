//! Runtime networking and piped-process primitives (`compiler/runtime/src/net.rs`
//! and the piped section of `compiler/runtime/src/platform.rs`), driven
//! through the real compiler pipeline: the module under test
//! (`prelude/Effect/Net.cinch`, joined with `Effect.Process`, whose
//! `foreign import ccall` declarations bind the `cinch.runtime.net*` and
//! `cinch.runtime.process*` units to the `cinch_net_*`/`cinch_process_*`
//! externs) plus a `Main`, written into a fresh temp project directory
//! beside the entry and compiled and run by `cinch::driver::{driver,
//! Mode}` like the CLI does.
//!
//! Nothing here touches the public internet. The TCP peer is a loopback
//! listener this file stands up (`std::net::TcpListener` plus a thread
//! that echoes one line and closes its side, so the client can reach the
//! end of the stream), a refused connection is aimed at a loopback port
//! nothing listens on, and the process half drives `/bin/sh` over the
//! piped ABI.
//!
//! Each Cinch check is a top-level `check-*` function returning 0 or its
//! bit; `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks (check k contributes bit 2^(k-1)) and the total stays
//! inside the 8-bit exit status. The checks are consumer-visible
//! semantics, never wiring: the socket round trip and its end of stream,
//! the named error of a refused connection, a closed or invented handle
//! and a `which` outside the two output pipes, the resolved-address
//! lines, and the piped child's three streams, exit code and close.

use cinch::driver::{driver, Mode};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The module under test: the `Effect.Net` home (the socket and
/// name-resolution capability) with its intrinsic declarations.
const NET: &str = include_str!("../prelude/Effect/Net.cinch");

fn write_project(name: &str, main_src: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_netprims_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("Net.cinch", NET), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    dir
}

/// Compile and run the project's entry on a wide-stack thread (the
/// checker recurses per nested expression, so a project of this size
/// needs more than the default test thread has; the `lib_*` harnesses in
/// this directory do the same).
fn run_main(name: &str, main_src: &str) -> i32 {
    let dir = write_project(name, main_src);
    let entry = dir.join("Main.cinch");
    let compiled = std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &entry, None).map_err(|e| e.to_string()))
        .expect("spawn compiler thread")
        .join();
    match compiled {
        Ok(Ok(code)) => code,
        Ok(Err(e)) => panic!("{name}: compilation failed: {e}"),
        Err(_) => panic!("{name}: the compiler thread panicked"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// A loopback listener with a peer thread that echoes one line back and
/// then closes its side (the client's next read is the end of the
/// stream). The port answers.
fn echoing_peer() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().expect("local addr").port();
    std::thread::spawn(move || {
        if let Ok((mut s, _)) = listener.accept() {
            let mut line = Vec::new();
            let mut one = [0u8; 1];
            while std::io::Read::read(&mut s, &mut one).unwrap_or(0) == 1 {
                line.push(one[0]);
                if one[0] == b'\n' {
                    break;
                }
            }
            let mut reply = b"echo:".to_vec();
            reply.extend_from_slice(&line);
            let _ = std::io::Write::write_all(&mut s, &reply);
            let _ = std::io::Write::flush(&mut s);
        }
    });
    port
}

/// A loopback port nothing listens on (bound, then dropped): the
/// connection is refused rather than answered. The port is verified
/// free again after the drop, because the OS can re-hand a freed port
/// to another process (the only way this probe goes wrong, and it was
/// observed once under load).
fn dead_port() -> u16 {
    for _ in 0..16 {
        let l = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let port = l.local_addr().expect("local addr").port();
        drop(l);
        if std::net::TcpStream::connect(("127.0.0.1", port)).is_err() {
            return port;
        }
    }
    panic!("no free loopback port after 16 attempts");
}

/// A path as a Cinch string literal (the probes splice their scratch
/// directory into the program text).
fn lit(p: &Path) -> String {
    let mut out = String::from("\"");
    for ch in p.to_string_lossy().chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[test]
fn socket_round_trip_eof_refusal_and_resolution() {
    // The socket probes: connect to a real loopback peer, write a line,
    // read the peer's echo back whole, read the end of the stream as an
    // EMPTY buffer (not an error), see the closed handle rejected, aim a
    // connection at a port nothing listens on (the documented error, no
    // hang and no panic), see an invented handle rejected, and resolve
    // the local name into at least one newline-terminated address line.
    let port = echoing_peer();
    let dead = dead_port();
    let src = r#"
module Main where

  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.Result
  import Data.String
  import Effect.Net

  -- Read the socket to its end, accumulating the text it produced (a
  -- short read is legal, so a caller that wants a length loops).
  read-all :: forall r. using (io :: Io r) => Socket -> String ->{IO r} Result NetError String
  read-all s acc = case net-read s 4096 of
    Err e -> Err e
    Ok b -> case string-from-bytes b of
      None -> Err NetIoError
      Some t -> case string-byte-length t == 0 of
        True -> Ok acc
        False -> read-all s (string-concat acc t)

  -- Connect, write, read the echo, read the end of the stream, close,
  -- and the closed handle.
  check-round-trip :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-round-trip u = case net-connect "127.0.0.1" __PORT__ of
    Err e -> 1
    Ok s -> case net-write s (string-to-bytes "hello\n") of
      Err e -> 2
      Ok n -> case n == 6 of
        False -> 3
        True -> case read-all s "" of
          Err e -> 4
          Ok text -> case text == "echo:hello\n" of
            False -> 5
            True -> case net-read s 4096 of
              Err e -> 6
              Ok b -> case bytes-len b == 0 of
                False -> 7
                True -> case net-close s of
                  Err e -> 8
                  Ok u -> case net-read s 16 of
                    Err e -> case e of
                      NetInvalidArgument -> 0
                      _ -> 9
                    Ok b -> 10

  -- A refused connection and an invented handle are named errors.
  check-refusal-and-stale-handles :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-refusal-and-stale-handles u = case net-connect "127.0.0.1" __DEAD__ of
    Ok s -> 16
    Err e -> case net-read (Socket 4096) 16 of
      Ok b -> 17
      Err e2 -> case e2 of
        NetInvalidArgument -> case net-close (Socket 4096) of
          Ok u -> 18
          Err e3 -> case e3 of
            NetInvalidArgument -> 0
            _ -> 19
        _ -> 20

  -- Resolution answers at least one newline-terminated line.
  check-resolve :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-resolve u = case net-resolve "localhost" __PORT__ of
    Err e -> 32
    Ok lines -> case bytes-len lines == 0 of
      True -> 33
      False -> 0

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let c1 = check-round-trip () in
    let c2 = check-refusal-and-stale-handles () in
    let c3 = check-resolve () in
    c1 + c2 + c3
"#
    .replace("__PORT__", &port.to_string())
    .replace("__DEAD__", &dead.to_string());
    expect_ok("net-sockets", &src);
}

#[test]
fn piped_child_streams_cwd_env_exit_code_and_named_errors() {
    // The piped-process probes: spawn `/bin/sh` behind three pipes, feed
    // its stdin, read stdout and stderr APART (each to its own end), see
    // the environment addition arrive and the child's working directory
    // take effect (it writes a file beside its cwd), observe the exit
    // code through the wait, and see the closed handle and the two
    // malformed-argument cases rejected by name.
    let dir = std::env::temp_dir().join(format!("cinch_netprims_pipe_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let src = r#"
module Main where

  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Path
  import Data.Result
  import Data.String
  import Effect.Process

  -- Read one output pipe to its end, accumulating the text it produced.
  read-pipe :: forall r. using (io :: Io r) => Child -> Stream -> String ->{IO r} Result ProcessError String
  read-pipe c s acc = case child-read c s 4096 of
    Err e -> Err e
    Ok b -> case string-from-bytes b of
      None -> Err ProcessIoError
      Some t -> case string-byte-length t == 0 of
        True -> Ok acc
        False -> read-pipe c s (string-concat acc t)

  -- Spawn, write the input, read both streams apart, wait, close, and
  -- the closed handle.
  check-streams :: forall r. using (io :: Io r) => Path -> Path ->{IO r} Int
  check-streams sh scratch =
    let probe = command-with-env sh (Cons "-c" (Cons "echo \"$PROBE\"; cat; echo err >&2; echo here > cwd-check.txt" Nil)) (Cons ("PROBE", "visible") Nil) in
    case process-spawn (Some scratch) probe of
      Err e -> 1
      Ok c -> case child-write-stdin c (string-to-bytes "payload\n") of
        Err e -> 2
        Ok n -> case n == 8 of
          False -> 3
          True -> case child-close-stdin c of
            Err e -> 4
            Ok u -> case read-pipe c Stdout "" of
              Err e -> 5
              Ok out -> case out == "visible\npayload\n" of
                False -> 6
                True -> case read-pipe c Stderr "" of
                  Err e -> 7
                  Ok err -> case err == "err\n" of
                    False -> 8
                    True -> case child-wait c of
                      Err e -> 9
                      Ok code -> case code == 0 of
                        False -> 10
                        True -> case child-close c of
                          Err e -> 11
                          Ok u -> case child-read c Stdout 16 of
                            Err e -> case e of
                              ProcessInvalidArgument -> 0
                              _ -> 12
                            Ok b -> 13

  -- A program that does not exist is `ProcessNotFound`, and an argument
  -- list the NUL encoding cannot carry is refused rather than dropped.
  check-named-errors :: forall r. using (io :: Io r) => Path ->{IO r} Int
  check-named-errors sh = case process-spawn None (command (path-from-string "/nonexistent/cinch-program") Nil) of
    Ok c -> 16
    Err e -> case e of
      ProcessNotFound -> case process-spawn None (command sh (Cons "a" (Cons "" (Cons "b" Nil)))) of
        Ok c -> 17
        Err e2 -> case e2 of
          ProcessInvalidArgument -> 0
          _ -> 18
      _ -> 19

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let sh = path-from-string "/bin/sh" in
    let scratch = path-from-string __DIR__ in
    case check-streams sh scratch of
      c1 -> case check-named-errors sh of
        c2 -> c1 + c2
"#
    .replace("__DIR__", &lit(&dir));
    expect_ok("net-pipes", &src);
    // The child ran in the directory it was given.
    assert_eq!(
        std::fs::read_to_string(dir.join("cwd-check.txt")).expect("the child's file"),
        "here\n"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

//! Structured-concurrency tests (P12): structured scopes with `spawn`
//! and task handles, `joinAllOrCancel` with the documented failure
//! policy, task captures under the ownership discipline, the blocking
//! call policy, and the `Io r` runtime operations end to end
//! (concurrency specification sections 2, 3, 4 and 6, and the runtime
//! capabilities, effects and sequencing spec).

use safelang::driver::{check_sources, run_source_io, run_source_objects};

fn expect_ok(name: &str, src: &str, expected: i32) {
    match safelang::driver::run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

/// Compile, run with `input` on the runtime's input source, and assert
/// the exit code and the captured output.
fn expect_io(name: &str, src: &str, input: &[u8], expected: i32, output: &str) {
    match run_source_io(src, input) {
        Ok((code, out)) => {
            assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
            assert_eq!(out, output, "{name}: expected output {output:?}, got {out:?}");
        }
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// Compile a C fixture to an object file and return its path (the
/// foreign binding tests' helper).
fn c_fixture(name: &str, src: &str) -> String {
    let dir = std::env::temp_dir().join(format!("slconc-{}-{}", name, std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let c_path = dir.join(format!("{name}.c"));
    let o_path = dir.join(format!("{name}.o"));
    std::fs::write(&c_path, src).expect("write fixture");
    let clang = std::env::var("SAFELANG_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = std::process::Command::new(&clang)
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&o_path)
        .status()
        .expect("run clang");
    assert!(status.success(), "{name}: clang failed to compile the C fixture");
    o_path.to_string_lossy().into_owned()
}

// ------------------------------------------- structured scopes and joins (section 2)

#[test]
fn spawned_task_results_join_correctly() {
    // Two tasks joined in handle order: the join's tuple carries each
    // body's `Ok` payload beside its handle.
    expect_ok(
        "join-pair",
        r#"
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok 20)
    h2 <- spawn s (\() -> Ok 2)
    joinAllOrCancel s h h2

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> case p of
      (a, b) -> if a * 10 + b == 202 then 0 else 1
    Err e -> 2
"#,
        0,
    );
    // The single-handle shape joins one task's payload.
    expect_ok(
        "join-one",
        r#"
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Nat
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok 7)
    joinAllOrCancel s h

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok n -> if n == 7 then 0 else 1
    Err e -> 2
"#,
        0,
    );
}

#[test]
fn dropping_a_handle_is_rejected() {
    // The handle reaches a sink that could drop it: refused at the call.
    expect_err(
        "handle-to-sink",
        r#"
module Main where
  ignore :: forall a. a -> Unit
  ignore x = ()

  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Diverge} Unit
  bad u = withScope $ \s -> ignore (spawn s (\() -> Ok 1))
"#,
        "consumed exactly once",
    );
    // A bound handle that is never joined: refused by its usage
    // obligation (dropping would silently detach execution).
    expect_err(
        "handle-never-joined",
        r#"
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Diverge} Unit
  bad u = withScope $ \s -> case spawn s (\() -> Ok 1) of
    h -> ()
"#,
        "task handle",
    );
}

#[test]
fn task_handles_cannot_escape_their_scope() {
    expect_err(
        "handle-escapes",
        r#"
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Diverge} Nat
  bad u = withScope $ \s -> spawn s (\() -> Ok 1)
"#,
        "leaves its `withScope`",
    );
}

// ------------------------------------------- cancellation and the failure policy (section 4)

#[test]
fn join_all_or_cancel_cancels_siblings_on_failure() {
    // The first task fails with `Err (LoadError 2)`. The sibling is a
    // running loop that observes the cancellation request at its
    // `pollCancel` point, writes `C` to the output sink, and returns:
    // the join waits for that cleanup acknowledgment before returning
    // the first failure.
    expect_io(
        "cancel-sibling",
        r#"
module Main where
  setByte :: Bytes -> Nat -> Nat -> Bytes
  setByte b i v = case bytes-set b i v of
    _ -> b

  fill :: Nat -> Bytes
  fill v = setByte (bytes-new 1) 0 v

  putByte :: forall r. using (io :: Io r) => Nat ->{IO r} Unit
  putByte v = ioWrite (fill v)

  -- A long-running task: cancellation is observed at a defined point
  -- (the `pollCancel` suspension), then cleanup completes.
  spin :: forall r. using (io :: Io r) => Unit ->{IO r, Cancel r} Result LoadError Nat
  spin u = case pollCancel () of
    True -> case putByte 67 of
      _ -> Ok 0
    False -> case ioSleep 5 of
      _ -> spin ()

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Err (LoadError 2))
    h2 <- spawn s (\() -> spin ())
    joinAllOrCancel s h h2

  main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> 1
    Err e -> case e of
      LoadError c -> if c == 2 then 0 else 3
"#,
        b"",
        0,
        "C",
    );
}

// ------------------------------------------- task captures under P5 (section 3)

#[test]
fn a_linear_capture_transfers_into_a_task_exactly_once() {
    // The `%1` capture is consumed by the task body exactly once: the
    // transfer is visible to the usage checker at the capture.
    expect_ok(
        "linear-transfer",
        r#"
module Main where
  consume :: forall r. using (io :: Io r) => Bytes %1 ->{IO r} Nat
  consume b = bytes-len b

  work :: forall r. using (io :: Io r) => Bytes %1 ->{Async r, IO r, Cancel r, Diverge} Result LoadError Nat
  work x = withScope $ \s -> do
    h <- spawn s (\() -> Ok (consume x))
    joinAllOrCancel s h

  main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work (bytes-new 3) of
    Ok n -> if n == 3 then 0 else 1
    Err e -> 2
"#,
        0,
    );
}

#[test]
fn double_capture_of_a_linear_value_is_rejected() {
    // Two tasks capture the same `%1` value: it would be consumed twice.
    expect_err(
        "double-capture",
        r#"
module Main where
  consume :: forall r. using (io :: Io r) => Bytes %1 ->{IO r} Nat
  consume b = bytes-len b

  work :: forall r. using (io :: Io r) => Bytes %1 ->{Async r, IO r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work x = withScope $ \s -> do
    h <- spawn s (\() -> Ok (consume x))
    h2 <- spawn s (\() -> Ok (consume x))
    joinAllOrCancel s h h2
"#,
        "used 2 time(s)",
    );
}

#[test]
fn an_affine_thunk_moves_into_a_task_by_transfer() {
    // The lazy specification's ownership rule (section 5): an affine
    // thunk moves across the task boundary by ownership transfer.
    expect_ok(
        "thunk-transfer",
        r#"
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Nat
  work u = let t = delay (\() -> 21) in withScope $ \s -> do
    h <- spawn s (\() -> Ok (force t))
    joinAllOrCancel s h

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok n -> if n == 21 then 0 else 1
    Err e -> 2
"#,
        0,
    );
}

#[test]
fn a_mut_borrow_capture_is_rejected() {
    // An exclusive borrow cannot cross a task boundary (section 3).
    expect_err(
        "mut-capture",
        r#"
module Main where
  readMut :: forall g. Mut g Nat -> Nat
  readMut m = copyOut m

  bad :: forall r g. using (io :: Io r) => Mut g Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad m = withScope $ \s -> do
    h <- spawn s (\() -> Ok (readMut m))
    joinAllOrCancel s h
"#,
        "exclusive `Mut` borrow cannot cross a task boundary",
    );
}

#[test]
fn ref_and_copy_captures_are_accepted() {
    // A shared `Ref` borrow is `Share` evidence: read-only access may
    // cross into the task.
    expect_ok(
        "ref-capture",
        r#"
module Main where
  work :: forall r g. using (io :: Io r) => Ref g Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat
  work b = withScope $ \s -> do
    h <- spawn s (\() -> Ok (copyOut b))
    joinAllOrCancel s h

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work (borrow 41) of
    Ok n -> if n == 41 then 0 else 1
    Err e -> 2
"#,
        0,
    );
    // A `Copy` value is shareable evidence by structure.
    expect_ok(
        "copy-capture",
        r#"
module Main where
  work :: forall r. using (io :: Io r) => Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat
  work n = withScope $ \s -> do
    h <- spawn s (\() -> Ok (n + 1))
    joinAllOrCancel s h

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work 41 of
    Ok n -> if n == 42 then 0 else 1
    Err e -> 2
"#,
        0,
    );
}

#[test]
fn an_unshared_mutable_capture_is_rejected() {
    // `Bytes` is a mutable buffer: sharing it across tasks without a
    // transfer would race (section 3).
    expect_err(
        "bytes-capture",
        r#"
module Main where
  bad :: forall r. using (io :: Io r) => Bytes ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad b = withScope $ \s -> do
    h <- spawn s (\() -> Ok (bytes-len b))
    joinAllOrCancel s h
"#,
        "without shareable evidence",
    );
}

#[test]
fn a_thread_shared_memo_cell_is_rejected() {
    // The lazy specification's required rejection case (section 9): a
    // `Lazy` cell is single-threaded by construction and does not cross
    // its region's thread; a shared memo needs an explicit synchronized
    // structure, not this mechanism.
    expect_err(
        "memo-capture",
        r#"
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad u = let ~y = bytes-len (bytes-new 2) in withScope $ \s -> do
    h <- spawn s (\() -> Ok (copyOut (demand y)))
    joinAllOrCancel s h
"#,
        "thread-shared memo",
    );
}

// ------------------------------------------- the blocking call policy (section 9)

#[test]
fn blocking_foreign_call_is_accounted_in_the_task_row() {
    let o = c_fixture(
        "blockinc",
        r#"
long long fix_block_inc(long long x) { return x + 1; }
"#,
    );
    // The `Blocking` foreign call runs on the task's own thread (the
    // bootstrap's offload contract) and its label is accounted in the
    // task body's row, which `spawn` folds into the spawning scope's
    // row and the program's declared row.
    let src = r#"
module Main where
  foreign import ccall "fix_block_inc" blockingInc :: I64 ->{IO, Blocking} I64

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO, Blocking, Cancel r, Diverge} Result LoadError I64
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok (blockingInc 41))
    joinAllOrCancel s h

  main :: Unit ->{Async r, IO, Blocking, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok n -> if n == 42 then 0 else 1
    Err e -> 2
"#;
    match run_source_objects(src, &[&o]) {
        Ok(code) => assert_eq!(code, 0, "blocking-task: expected exit 0, got {code}"),
        Err(e) => panic!("blocking-task: compilation failed: {e}"),
    }
    // The row must account for it: a program declaring a row without
    // `Blocking` is refused by effect honesty.
    expect_err(
        "blocking-unaccounted",
        r#"
module Main where
  foreign import ccall "fix_block_inc" blockingInc :: I64 ->{IO, Blocking} I64

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO, Cancel r, Diverge} Result LoadError I64
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok (blockingInc 41))
    joinAllOrCancel s h
"#,
        "Blocking",
    );
}

// ------------------------------------------- the Io r operations end to end (section 9)

#[test]
fn io_read_and_write_round_trip() {
    // `fetch` (the sketch's vocabulary) reads the runtime's input
    // source through `ioRead`; the program writes the bytes back
    // through `ioWrite`, and the test observes the output.
    expect_io(
        "fetch-round-trip",
        r#"
module Main where
  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in case fetch (Request 8) of
    Ok b -> case ioWrite b of
      _ -> if bytes-len b == 5 then 0 else 1
    Err e -> 2
"#,
        b"hello",
        0,
        "hello",
    );
    // A short read is the `LoadError` outcome of the sketch.
    expect_io(
        "fetch-short-read",
        r#"
module Main where
  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in case fetch (Request 8) of
    Ok b -> 1
    Err e -> case e of
      LoadError c -> if c == 1 then 0 else 3
"#,
        b"",
        0,
        "",
    );
}

// ------------------------------------------- the acceptance probe (P15 catalogue)

#[test]
fn example_11_structured_tasks_accepts() {
    let src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../examples/source/11-structured-tasks.lang"
    ));
    match check_sources(src, "11-structured-tasks.lang") {
        Ok(_) => {}
        Err(e) => panic!("11-structured-tasks: rejected: {e}"),
    }
}

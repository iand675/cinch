//! End-to-end tests for `Data.Sql` and `Effect.Sql`:
//! pure SQL value model, NULLs, column metadata, isolation levels, tx options,
//! named parameters, errors, typed row scanning, connection pooling limits,
//! reuse, idle eviction, `SqlDbStats`, prepared statements, transactions
//! (commit/rollback/double-finish/read-only), `SqlErrNoRows`, `SqlErrBadConn`
//! retries, `SqlErrSkip` fallback, concurrent multi-task pool contention,
//! leak-free reference counting, and canonical documentation.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::Mode;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

/// Build a project with `Data.Sql`, `Effect.Sql`, and `Main.cinch` and run it
/// with reference counting + leak detection (plain and `rc_debug` builds).
fn run_clean(tag: &str, main_src: &str, rc_debug: bool) {
    let dir = common::project(tag, &[
        ("Data.Sql.cinch", include_str!("../prelude/Data/Sql.cinch")),
        ("Effect.Sql.cinch", include_str!("../prelude/Effect/Sql.cinch")),
        ("Main.cinch", main_src),
    ]);
    let exe = dir.join(if rc_debug { "sql_prog_dbg" } else { "sql_prog" });
    let profile = TargetProfile {
        reclaim: true,
        rc_debug,
        ..TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        Some(exe.clone()),
        &profile,
    )
    .unwrap_or_else(|e| panic!("{tag}: build failed: {e}"));

    let out = run_with_timeout(&exe, rc_debug);
    let code = out.status.code();
    assert_eq!(
        code,
        Some(200),
        "{tag} (rc_debug={rc_debug}): expected exit 200 (all checks passed), got {code:?}\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let leaked = parse_leaked(&out);
    assert_eq!(
        leaked,
        0,
        "{tag} (rc_debug={rc_debug}): leaked {leaked} blocks\nstderr: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    common::cleanup(&dir);
}

fn run_with_timeout(exe: &Path, rc_debug: bool) -> Output {
    let mut cmd = Command::new(exe);
    cmd.env("CINCH_HEAP_LIMIT", "512M")
        .env("CINCH_LEAK_CHECK", "1")
        .env_remove("CINCH_ALLOC_STATS")
        .env("CINCH_THREADS", "4");
    if rc_debug {
        cmd.env("CINCH_RC_DEBUG", "1");
    }
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn sql test binary");
    let start = Instant::now();
    while child.try_wait().expect("poll sql test binary").is_none() {
        if start.elapsed() > Duration::from_secs(30) {
            let _ = child.kill();
            panic!("sql test timed out: {:?}", child.wait_with_output());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    child.wait_with_output().expect("collect sql test output")
}

fn parse_leaked(out: &Output) -> u64 {
    let err = String::from_utf8_lossy(&out.stderr);
    let line = err
        .lines()
        .find(|l| l.starts_with("cinch-leak:"))
        .unwrap_or_else(|| panic!("no cinch-leak line in stderr: {err}"));
    line.split_whitespace()
        .find_map(|w| w.strip_prefix("leaked="))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("no leaked= field in {line}"))
}

/// Every declaration in `Data.Sql` and `Effect.Sql` has canonical module-owned documentation.
#[test]
fn sql_declarations_have_canonical_documentation() {
    common::assert_documented("Data.Sql", include_str!("../prelude/Data/Sql.cinch"));
    common::assert_documented("Effect.Sql", include_str!("../prelude/Effect/Sql.cinch"));
}

/// Pure `Data.Sql` tests: values, named args, isolation levels, tx options,
/// column metadata, exec results, errors, immutable rows, and typed scanners.
#[test]
fn data_sql_pure_model_and_scanners() {
    run_clean("sql_pure", PURE_TEST, false);
    run_clean("sql_pure_dbg", PURE_TEST, true);
}

/// Full `Effect.Sql` lifecycle: CRUD, named/positional params, NULLs, all scalar
/// types, `QueryRow`/`SqlErrNoRows`, streaming cursors, prepared statements,
/// transactions (commit, rollback, double-finish, read-only, tx statements).
#[test]
fn effect_sql_lifecycle_stmts_and_tx() {
    run_clean("sql_lifecycle", LIFECYCLE_TEST, false);
    run_clean("sql_lifecycle_dbg", LIFECYCLE_TEST, true);
}

/// Connection pooling: lazy open, reuse, `SetMaxIdleConns`, `SetMaxOpenConns`,
/// idle eviction, `SqlDbStats`, pool close, `SqlErrBadConn` retries, `SqlErrSkip`
/// fallback, and driver errors.
#[test]
fn effect_sql_pooling_retries_and_errors() {
    run_clean("sql_pool", POOL_AND_ERRORS_TEST, false);
    run_clean("sql_pool_dbg", POOL_AND_ERRORS_TEST, true);
}

/// Concurrent multi-task queries and blocking `max-open-conns` pool contention
/// over shared `SqlPoolGate` and `SqlMemoryStore`, leak-free under RC debug.
#[test]
fn effect_sql_concurrent_pool_and_contention() {
    run_clean("sql_concurrent", CONCURRENT_TEST, false);
    run_clean("sql_concurrent_dbg", CONCURRENT_TEST, true);
}

const PURE_TEST: &str = r#"-- | Pure Data.Sql unit test suite.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Option
  import Data.Result
  import Data.Sql
  import Data.String
  import Relation.Eq
  import Text.Show

  -- | Return the 1-based index of the first failing check, or 200 when all pass.
  checks :: List Bool -> Int -> Int
  checks xs i = case xs of
    Nil -> 200
    Cons ok rest -> if ok then checks rest (i + 1) else i

  -- | Compare two optional values for equality.
  opt-eq :: forall a. using (eq :: Eq a) => Option a -> Option a -> Bool
  opt-eq x y = case (x, y) of
    (None, None) -> True
    (Some a, Some b) -> a == b
    _ -> False

  -- | Check that a Result is Ok and equal to the expected value.
  res-ok :: forall e a. using (eq :: Eq a) => Result e a -> a -> Bool
  res-ok r expected = case r of
    Ok v -> v == expected
    Err _ -> False

  -- | Check that a Result is Err and equal to the expected error.
  res-err :: forall e a. using (eq :: Eq e) => Result e a -> e -> Bool
  res-err r expected = case r of
    Err e -> e == expected
    Ok _ -> False

  -- | Check that a Result carrying an Option is Ok and matches the expected Option.
  res-ok-opt :: forall e a. using (eq :: Eq a) => Result e (Option a) -> Option a -> Bool
  res-ok-opt r expected = case r of
    Ok o -> opt-eq o expected
    Err _ -> False

  -- | Build a small byte buffer [0x41, 0x7f, 0x00].
  sample-bytes :: Bytes
  sample-bytes = bytes-from-list [65, 127, 0]

  -- | Verify bytes mutation isolation after scan-bytes.
  check-bytes-isolation :: SqlRow -> Bool
  check-bytes-isolation row = case scan-bytes row 5 of
    Err _ -> False
    Ok buf ->
      let mutated = (if 0 < bytes-len buf then bytes-set buf 0 99 else ()) in
      case mutated of
        _ -> case scan-bytes row 5 of
          Err _ -> False
          Ok buf2 -> bytes-len buf2 == 3 && (if 0 < bytes-len buf2 then bytes-get buf2 0 == 65 else False)

  -- | Run all pure Data.Sql checks.
  main :: Int
  main =
    let blob = sample-bytes in
    let f-zero = i64-to-f64 0 in
    let f-one = i64-to-f64 1 in
    let f-half = div-f64 (i64-to-f64 3) (i64-to-f64 2) in
    let f-two-half = div-f64 (i64-to-f64 5) (i64-to-f64 2) in
    let f-score = div-f64 (i64-to-f64 7) (i64-to-f64 2) in
    let row = (sql-row
      ["id", "ok", "score", "name", "note", "payload", "ts", "opt_n", "opt_s", "opt_f", "opt_b", "opt_raw", "opt_ts"]
      [SqlInt 42, SqlBool True, SqlFloat f-score, SqlString "alice", SqlNull, SqlBytes blob, SqlTimestamp 1700000000000000000,
       SqlNull, SqlString "hi", SqlNull, SqlNull, SqlNull, SqlNull]) in
    let col = sql-column-type "price" "DECIMAL" (Some False) (Some 16) (Some 10) (Some 2) in
    let col-partial = sql-column-type "id" "BIGINT" (Some False) None (Some 10) None in
    let exec-res = sql-exec-result (Some 99) (Some 3) in
    let tx-opts = sql-tx-options IsolationSerializable True in
    let nan-val = div-f64 f-zero f-zero in
    checks [
      -- 1..8: SqlValue Eq, Show, type-name, is-null
      sql-value-is-null SqlNull && not (sql-value-is-null (SqlInt 0)),
      sql-value-type-name SqlNull == "NULL" && sql-value-type-name (SqlBool True) == "BOOL" && sql-value-type-name (SqlInt 1) == "INT",
      sql-value-type-name (SqlFloat f-one) == "FLOAT" && sql-value-type-name (SqlString "a") == "STRING" && sql-value-type-name (SqlBytes blob) == "BYTES" && sql-value-type-name (SqlTimestamp 5) == "TIMESTAMP",
      SqlInt 42 == SqlInt 42 && SqlInt 42 /= SqlInt 43 && SqlNull == SqlNull && SqlNull /= SqlInt 0,
      SqlFloat nan-val == SqlFloat nan-val && SqlFloat f-half == SqlFloat f-half && SqlFloat f-half /= SqlFloat f-two-half,
      show SqlNull == "NULL" && show (SqlBool True) == "TRUE" && show (SqlBool False) == "FALSE" && show (SqlInt -7) == "-7",
      show (SqlString "abc") == "abc" && show (SqlBytes blob) == "0x417f00" && show (SqlTimestamp 12345) == "TIMESTAMP(12345)",
      show (SqlFloat nan-val) == "NaN",

      -- 9..11: SqlNamedArg
      sql-arg (SqlInt 7) == SqlPositional (SqlInt 7) && sql-arg-value (sql-arg (SqlInt 7)) == SqlInt 7,
      (case sql-arg-name (sql-arg (SqlInt 7)) of None -> True; Some _ -> False),
      sql-named "id" (SqlInt 7) == SqlNamed "id" (SqlInt 7) && (case sql-arg-name (sql-named "id" (SqlInt 7)) of Some s -> s == "id"; None -> False),

      -- 12..14: SqlIsolation & SqlTxOptions
      sql-isolation-name IsolationDefault == "Default" && sql-isolation-name IsolationReadUncommitted == "Read Uncommitted" && sql-isolation-name IsolationReadCommitted == "Read Committed" && sql-isolation-name IsolationWriteCommitted == "Write Committed",
      sql-isolation-name IsolationRepeatableRead == "Repeatable Read" && sql-isolation-name IsolationSnapshot == "Snapshot" && sql-isolation-name IsolationSerializable == "Serializable" && sql-isolation-name IsolationLinearizable == "Linearizable",
      sql-tx-isolation sql-tx-options-default == IsolationDefault && not (sql-tx-read-only sql-tx-options-default) && sql-tx-isolation tx-opts == IsolationSerializable && sql-tx-read-only tx-opts && tx-opts /= sql-tx-options-default,

      -- 15..17: SqlColumnType & SqlExecResult
      sql-column-name col == "price" && sql-column-database-type-name col == "DECIMAL" && opt-eq (sql-column-nullable col) (Some False) && opt-eq (sql-column-length col) (Some 16) && opt-eq (sql-column-precision col) (Some 10) && opt-eq (sql-column-scale col) (Some 2),
      (case sql-column-decimal-size col of Some p -> (case p of (pr, sc) -> pr == 10 && sc == 2); None -> False) && (case sql-column-decimal-size col-partial of None -> True; Some _ -> False),
      opt-eq (sql-last-insert-id exec-res) (Some 99) && opt-eq (sql-rows-affected exec-res) (Some 3) && opt-eq (sql-last-insert-id sql-exec-result-no-rows) None && opt-eq (sql-rows-affected sql-exec-result-no-rows) None,

      -- 18..20: SqlError Eq & messages
      SqlErrNoRows == SqlErrNoRows && SqlErrTxDone /= SqlErrConnDone && SqlErrPoolClosed == SqlErrPoolClosed && SqlErrBadConn == SqlErrBadConn && SqlErrSkip == SqlErrSkip,
      sql-error-message SqlErrNoRows == "sql: no rows in result set" && sql-error-message SqlErrTxDone == "sql: transaction has already been committed or rolled back" && sql-error-message SqlErrPoolClosed == "sql: database is closed",
      sql-error-message (SqlErrTypeMismatch "INT" "NULL") == "sql: type mismatch: expected INT, got NULL" && sql-error-message (SqlErrColumnOutOfRange 9 3) == "sql: column index 9 out of range for row with 3 columns",

      -- 21..26: SqlRow non-null scanners
      row-len row == 13 && res-ok (row-get row 0) (SqlInt 42) && res-ok (row-get-by-name row "name") (SqlString "alice"),
      res-ok (scan-i64 row 0) 42 && res-ok (scan-bool row 1) True && res-ok (scan-string row 3) "alice" && res-ok (scan-timestamp row 6) 1700000000000000000,
      (case scan-f64 row 2 of Ok f -> feq-f64 f f-score; Err _ -> False),
      check-bytes-isolation row,
      res-ok (scan-i64-by-name row "id") 42 && res-ok (scan-string-by-name row "name") "alice" && res-ok (scan-bool-by-name row "ok") True && res-ok (scan-timestamp-by-name row "ts") 1700000000000000000,
      (case scan-f64-by-name row "score" of Ok f -> feq-f64 f f-score; Err _ -> False),

      -- 27..32: SqlRow nullable scanners & error cases
      res-ok-opt (scan-null-string row 4) None && res-ok-opt (scan-null-string row 8) (Some "hi") && res-ok-opt (scan-null-i64 row 7) None && res-ok-opt (scan-null-i64 row 0) (Some 42),
      res-ok-opt (scan-null-bool row 10) None && res-ok-opt (scan-null-bool row 1) (Some True) && res-ok-opt (scan-null-timestamp row 12) None && res-ok-opt (scan-null-timestamp row 6) (Some 1700000000000000000),
      (case scan-null-f64 row 9 of Ok None -> True; _ -> False) && (case scan-null-f64 row 2 of Ok (Some f) -> feq-f64 f f-score; _ -> False),
      (case scan-null-bytes row 11 of Ok None -> True; _ -> False) && (case scan-null-bytes row 5 of Ok (Some b) -> b == blob; _ -> False),
      res-err (scan-i64 row 4) (SqlErrTypeMismatch "INT" "NULL") && res-err (scan-string row 0) (SqlErrTypeMismatch "STRING" "INT") && res-err (scan-null-i64 row 3) (SqlErrTypeMismatch "INT" "STRING"),
      res-err (row-get row 99) (SqlErrColumnOutOfRange 99 13) && (case row-get-by-name row "missing" of Err (SqlErrScan _) -> True; _ -> False)
    ] 1
"#;

const LIFECYCLE_TEST: &str = r#"-- | Full Effect.Sql lifecycle: CRUD, named args, NULLs, cursors, stmts, transactions.
module Main where
  import Algebra.Semigroup
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Option hiding (or-else)
  import Data.Result
  import Data.Sql
  import Data.String
  import Effect.Sql
  import Effect.Stm
  import Relation.Eq

  -- | Return the 1-based index of the first failing check, or 200 when all pass.
  checks :: List Bool -> Int -> Int
  checks xs i = case xs of
    Nil -> 200
    Cons ok rest -> if ok then checks rest (i + 1) else i

  -- | Check that a Result is Ok and equal to the expected value.
  res-ok :: forall e a. using (eq :: Eq a) => Result e a -> a -> Bool
  res-ok r expected = case r of
    Ok v -> v == expected
    Err _ -> False

  -- | Check that a Result is Err and equal to the expected error.
  res-err :: forall e a. using (eq :: Eq e) => Result e a -> e -> Bool
  res-err r expected = case r of
    Err e -> e == expected
    Ok _ -> False

  -- | Check that a Result Unit succeeded.
  res-ok-unit :: forall e. Result e Unit -> Bool
  res-ok-unit r = case r of
    Ok _ -> True
    Err _ -> False

  -- | Check that a Result carrying an Option is Ok None.
  res-ok-none :: forall e a. Result e (Option a) -> Bool
  res-ok-none r = case r of
    Ok None -> True
    _ -> False

  -- | Check CRUD, named/positional params, all scalar types, NULLs, QueryRow, cursors.
  test-crud :: forall r. SqlDb r ->{IO r, STM, Diverge} List Bool
  test-crud db =
    let r-ping = sql-db-ping db in
    let r-create = sql-db-exec db "CREATE TABLE items (id, active, price, name, note, raw, created)" [] in
    let blob = bytes-from-list [10, 20, 30] in
    let p-widget = div-f64 (i64-to-f64 37) (i64-to-f64 4) in
    let p-gadget = div-f64 (i64-to-f64 39) (i64-to-f64 2) in
    let p-bolt = i64-to-f64 5 in
    let r-ins1 = (sql-db-exec db "INSERT INTO items VALUES (?, ?, ?, ?, ?, ?, ?)"
      [sql-arg (SqlInt 1), sql-arg (SqlBool True), sql-arg (SqlFloat p-widget), sql-arg (SqlString "widget"),
       sql-arg SqlNull, sql-arg (SqlBytes blob), sql-arg (SqlTimestamp 1000)]) in
    let r-ins2 = (sql-db-exec db "INSERT INTO items VALUES (@id, @active, @price, @name, @note, @raw, @ts)"
      [sql-named "name" (SqlString "gadget"), sql-named "id" (SqlInt 2), sql-named "active" (SqlBool False),
       sql-named "price" (SqlFloat p-gadget), sql-named "note" (SqlString "sale"), sql-named "raw" SqlNull,
       sql-named "ts" (SqlTimestamp 2000)]) in
    let r-ins3 = (sql-db-exec db "INSERT INTO items VALUES ($1, $2, $3, $4, $5, $6, $7)"
      [sql-arg (SqlInt 3), sql-arg (SqlBool True), sql-arg (SqlFloat p-bolt), sql-arg (SqlString "bolt"),
       sql-arg SqlNull, sql-arg SqlNull, sql-arg (SqlTimestamp 3000)]) in
    let check-row1 = (case sql-db-query-row db "SELECT * FROM items WHERE id = ?" [sql-arg (SqlInt 1)] of
      Err _ -> False
      Ok row ->
        let ok-id = res-ok (scan-i64 row 0) 1 in
        let ok-act = res-ok (scan-bool row 1) True in
        let ok-price = (case scan-f64 row 2 of Ok f -> feq-f64 f p-widget; Err _ -> False) in
        let ok-name = res-ok (scan-string-by-name row "name") "widget" in
        let ok-note = res-ok-none (scan-null-string row 4) in
        let ok-raw = res-ok (scan-bytes row 5) blob in
        let ok-ts = res-ok (scan-timestamp row 6) 1000 in
        ok-id && ok-act && ok-price && ok-name && ok-note && ok-raw && ok-ts) in
    let check-no-rows = res-err (sql-db-query-row db "SELECT * FROM items WHERE id = ?" [sql-arg (SqlInt 999)]) SqlErrNoRows in
    let check-cursor = (case sql-db-query db "SELECT name, note FROM items" [] of
      Err _ -> False
      Ok rows ->
        let cols = sql-rows-columns rows in
        let ctypes = sql-rows-column-types rows in
        let ok-meta = list-length cols == 2 && list-length ctypes == 2 in
        case sql-rows-all rows of
          Err _ -> False
          Ok all-rows ->
            let ok-cnt = list-length all-rows == 3 in
            let ok-close2 = res-ok-unit (sql-rows-close rows) in
            let ok-eof = (case sql-rows-next rows of Ok None -> True; _ -> False) in
            ok-meta && ok-cnt && ok-close2 && ok-eof) in
    let r-upd = (sql-db-exec db "UPDATE items SET note = @note WHERE id = @id"
      [sql-named "note" (SqlString "updated"), sql-named "id" (SqlInt 1)]) in
    let check-upd = (case sql-db-query-row db "SELECT note FROM items WHERE id = ?" [sql-arg (SqlInt 1)] of
      Ok row -> res-ok (scan-string row 0) "updated"
      Err _ -> False) in
    let r-del = sql-db-exec db "DELETE FROM items WHERE id = ?" [sql-arg (SqlInt 3)] in
    let check-cnt = (case sql-db-query-row db "SELECT COUNT(*) FROM items" [] of
      Ok row -> res-ok (scan-i64 row 0) 2
      Err _ -> False) in
    [
      res-ok-unit r-ping,
      res-ok r-create (sql-exec-result None (Some 0)),
      res-ok r-ins1 (sql-exec-result (Some 1) (Some 1)),
      res-ok r-ins2 (sql-exec-result (Some 2) (Some 1)),
      res-ok r-ins3 (sql-exec-result (Some 3) (Some 1)),
      check-row1,
      check-no-rows,
      check-cursor,
      res-ok r-upd (sql-exec-result None (Some 1)),
      check-upd,
      res-ok r-del (sql-exec-result None (Some 1)),
      check-cnt,
      stats-in-use (sql-db-stats db) == 0
    ]

  -- | Check pool-level prepared statements (exec, query, query-row, close, use-after-close, invalid prepare).
  test-prepared :: forall r. SqlDb r ->{IO r, STM, Diverge} List Bool
  test-prepared db =
    let bad-prep = (case sql-db-prepare db "BOGUS SYNTAX" of Err (SqlErrDriver _) -> True; _ -> False) in
    let after-bad = stats-in-use (sql-db-stats db) == 0 in
    let ok-ins-stmt = (case sql-db-prepare db "INSERT INTO items VALUES (?, ?, ?, ?, ?, ?, ?)" of
      Err _ -> False
      Ok stmt ->
        let e1 = sql-stmt-exec stmt [sql-arg (SqlInt 10), sql-arg (SqlBool True), sql-arg (SqlFloat (i64-to-f64 1)), sql-arg (SqlString "p1"), sql-arg SqlNull, sql-arg SqlNull, sql-arg (SqlTimestamp 10)] in
        let e2 = sql-stmt-exec stmt [sql-arg (SqlInt 11), sql-arg (SqlBool False), sql-arg (SqlFloat (i64-to-f64 2)), sql-arg (SqlString "p2"), sql-arg SqlNull, sql-arg SqlNull, sql-arg (SqlTimestamp 11)] in
        let c1 = sql-stmt-close stmt in
        let c2 = sql-stmt-close stmt in
        let after-close = (case sql-stmt-exec stmt [] of Err (SqlErrDriver _) -> True; _ -> False) in
        (case (e1, e2) of (Ok _, Ok _) -> True; _ -> False) && res-ok-unit c1 && res-ok-unit c2 && after-close) in
    let ok-sel-stmt = (case sql-db-prepare db "SELECT name FROM items WHERE id = ?" of
      Err _ -> False
      Ok stmt ->
        let r1 = sql-stmt-query-row stmt [sql-arg (SqlInt 10)] in
        let r2 = sql-stmt-query-row stmt [sql-arg (SqlInt 999)] in
        let q1 = (case sql-stmt-query stmt [sql-arg (SqlInt 11)] of
          Err _ -> False
          Ok rows -> case sql-rows-all rows of
            Ok (Cons row Nil) -> res-ok (scan-string row 0) "p2"
            _ -> False) in
        let cl = sql-stmt-close stmt in
        let ok1 = (case r1 of Ok row -> res-ok (scan-string row 0) "p1"; Err _ -> False) in
        ok1 && res-err r2 SqlErrNoRows && q1 && res-ok-unit cl) in
    [bad-prep, after-bad, ok-ins-stmt, ok-sel-stmt, stats-in-use (sql-db-stats db) == 0]

  -- | Check transactions: commit, rollback, double-finish errors, read-only, tx prepared statements.
  test-tx :: forall r. SqlDb r ->{IO r, STM, Diverge} List Bool
  test-tx db =
    let r-tbl = sql-db-exec db "CREATE TABLE accounts (id, balance)" [] in
    let r-seed = sql-db-exec db "INSERT INTO accounts VALUES (?, ?)" [sql-arg (SqlInt 1), sql-arg (SqlInt 100)] in
    -- Commit path: sees uncommitted write inside tx, persists after commit, double-finish returns SqlErrTxDone.
    let ok-commit = (case sql-db-begin db of
      Err _ -> False
      Ok tx ->
        let in-use-during = stats-in-use (sql-db-stats db) == 1 in
        let u1 = sql-tx-exec tx "UPDATE accounts SET balance = ? WHERE id = ?" [sql-arg (SqlInt 250), sql-arg (SqlInt 1)] in
        let seen-in-tx = (case sql-tx-query-row tx "SELECT balance FROM accounts WHERE id = ?" [sql-arg (SqlInt 1)] of
          Ok row -> res-ok (scan-i64 row 0) 250
          Err _ -> False) in
        let tx-stmt-ok = (case sql-tx-prepare tx "SELECT balance FROM accounts WHERE id = ?" of
          Err _ -> False
          Ok st ->
            let r = (case sql-stmt-query-row st [sql-arg (SqlInt 1)] of
              Ok row -> res-ok (scan-i64 row 0) 250
              Err _ -> False) in
            let cm = sql-tx-commit tx in
            let cm2 = sql-tx-commit tx in
            let rb2 = sql-tx-rollback tx in
            let exec-after = sql-tx-exec tx "DELETE FROM accounts" [] in
            let stmt-after = sql-stmt-query-row st [sql-arg (SqlInt 1)] in
            r && res-ok-unit cm && res-err cm2 SqlErrTxDone && res-err rb2 SqlErrTxDone && res-err exec-after SqlErrTxDone && res-err stmt-after SqlErrTxDone) in
        in-use-during && res-ok u1 (sql-exec-result None (Some 1)) && seen-in-tx && tx-stmt-ok) in
    let persisted = (case sql-db-query-row db "SELECT balance FROM accounts WHERE id = ?" [sql-arg (SqlInt 1)] of
      Ok row -> res-ok (scan-i64 row 0) 250
      Err _ -> False) in
    -- Rollback path: writes discarded, double-rollback returns SqlErrTxDone.
    let ok-rollback = (case sql-db-begin db of
      Err _ -> False
      Ok tx ->
        let u1 = sql-tx-exec tx "UPDATE accounts SET balance = ? WHERE id = ?" [sql-arg (SqlInt 999), sql-arg (SqlInt 1)] in
        let seen = (case sql-tx-query-row tx "SELECT balance FROM accounts WHERE id = ?" [sql-arg (SqlInt 1)] of
          Ok row -> res-ok (scan-i64 row 0) 999
          Err _ -> False) in
        let rb1 = sql-tx-rollback tx in
        let rb2 = sql-tx-rollback tx in
        let cm2 = sql-tx-commit tx in
        (case u1 of Ok _ -> True; Err _ -> False) && seen && res-ok-unit rb1 && res-err rb2 SqlErrTxDone && res-err cm2 SqlErrTxDone) in
    let unchanged = (case sql-db-query-row db "SELECT balance FROM accounts WHERE id = ?" [sql-arg (SqlInt 1)] of
      Ok row -> res-ok (scan-i64 row 0) 250
      Err _ -> False) in
    -- Read-only tx with Serializable isolation: reads work, writes fail cleanly.
    let ok-ro = (case sql-db-begin-tx db (sql-tx-options IsolationSerializable True) of
      Err _ -> False
      Ok tx ->
        let rd = (case sql-tx-query-row tx "SELECT balance FROM accounts WHERE id = ?" [sql-arg (SqlInt 1)] of
          Ok row -> res-ok (scan-i64 row 0) 250
          Err _ -> False) in
        let wr = (case sql-tx-exec tx "DELETE FROM accounts" [] of
          Err (SqlErrDriver _) -> True
          _ -> False) in
        let cm = sql-tx-commit tx in
        rd && wr && res-ok-unit cm) in
    [
      (case (r-tbl, r-seed) of (Ok _, Ok _) -> True; _ -> False),
      ok-commit,
      persisted,
      ok-rollback,
      unchanged,
      ok-ro,
      stats-in-use (sql-db-stats db) == 0
    ]

  -- | Entry point running all lifecycle checks and closing cleanly.
  main :: Unit ->{IO r, STM, Diverge} Int
  main u =
    let drv = sql-memory-driver-new () in
    case sql-open drv "memory://lifecycle" of
      Err _ -> 1
      Ok db ->
        let c1 = test-crud db in
        let c2 = test-prepared db in
        let c3 = test-tx db in
        let closed = res-ok-unit (sql-db-close db) in
        checks (c1 <> c2 <> c3 <> [closed]) 1
"#;

const POOL_AND_ERRORS_TEST: &str = r#"-- | Connection pooling, eviction, stats, SqlErrBadConn retry, and SqlErrSkip fallback tests.
module Main where
  import Algebra.Semigroup
  import Data.Bool
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option hiding (or-else)
  import Data.Result
  import Data.Sql
  import Data.String
  import Effect.Sql
  import Effect.Stm
  import Relation.Eq

  -- | Return the 1-based index of the first failing check, or 200 when all pass.
  checks :: List Bool -> Int -> Int
  checks xs i = case xs of
    Nil -> 200
    Cons ok rest -> if ok then checks rest (i + 1) else i

  -- | Check that a Result is Ok and equal to the expected value.
  res-ok :: forall e a. using (eq :: Eq a) => Result e a -> a -> Bool
  res-ok r expected = case r of
    Ok v -> v == expected
    Err _ -> False

  -- | Check that a Result is Err and equal to the expected error.
  res-err :: forall e a. using (eq :: Eq e) => Result e a -> e -> Bool
  res-err r expected = case r of
    Err e -> e == expected
    Ok _ -> False

  -- | Check that a Result Unit succeeded.
  res-ok-unit :: forall e. Result e Unit -> Bool
  res-ok-unit r = case r of
    Ok _ -> True
    Err _ -> False

  -- | Wrap a driver so the first `fail-count` exec/query/ping/begin calls fail with `SqlErrBadConn`.
  bad-conn-driver :: forall (r :: Region). SqlDriver r -> TVar Nat -> SqlDriver r
  bad-conn-driver inner remaining =
    sql-driver (\dsn ->
      case driver-open inner dsn of
        Err e -> Err e
        Ok conn ->
          let consume-fail = (\() ->
            atomically (stm-bind (tvar-read remaining) (\n ->
              if n == 0 then stm-pure False
              else stm-then (tvar-write remaining (n - 1)) (stm-pure True)))) in
          Ok (sql-driver-conn
            (\() -> if consume-fail () then Err SqlErrBadConn else driver-conn-ping conn)
            (\q args -> if consume-fail () then Err SqlErrBadConn else driver-conn-exec conn q args)
            (\q args -> if consume-fail () then Err SqlErrBadConn else driver-conn-query conn q args)
            (\q -> driver-conn-prepare conn q)
            (\opts -> if consume-fail () then Err SqlErrBadConn else driver-conn-begin conn opts)
            (\() -> driver-conn-close conn)))

  -- | Wrap a driver so direct exec/query return `SqlErrSkip`, forcing prepare+stmt fallback.
  skip-fastpath-driver :: forall (r :: Region). SqlDriver r -> TVar Nat -> SqlDriver r
  skip-fastpath-driver inner prepare-hits =
    sql-driver (\dsn ->
      case driver-open inner dsn of
        Err e -> Err e
        Ok conn ->
          Ok (sql-driver-conn
            (\() -> driver-conn-ping conn)
            (\_ _ -> Err SqlErrSkip)
            (\_ _ -> Err SqlErrSkip)
            (\q -> case atomically (tvar-modify prepare-hits (\n -> n + 1)) of
              _ -> driver-conn-prepare conn q)
            (\opts -> driver-conn-begin conn opts)
            (\() -> driver-conn-close conn)))

  -- | Test pool laziness, reuse, eviction, limits, and close semantics.
  test-pool-lifecycle :: forall r. Unit ->{IO r, STM, Diverge} List Bool
  test-pool-lifecycle u =
    let store = sql-memory-store-new () in
    let drv = sql-memory-driver store in
    case sql-open drv "memory://pool" of
      Err _ -> [False]
      Ok db ->
        -- 1. Lazy open: 0 connections opened initially.
        let s0 = sql-db-stats db in
        let ok-lazy = sql-memory-store-open-count store == 0 && stats-open-connections s0 == 0 && stats-idle s0 == 0 in
        -- 2. Reuse: 4 sequential operations reuse 1 single connection.
        let _1 = sql-db-exec db "CREATE TABLE t (k)" [] in
        let _2 = sql-db-exec db "INSERT INTO t VALUES (?)" [sql-arg (SqlInt 1)] in
        let _3 = sql-db-exec db "INSERT INTO t VALUES (?)" [sql-arg (SqlInt 2)] in
        let _4 = sql-db-ping db in
        let s1 = sql-db-stats db in
        let ok-reuse = sql-memory-store-open-count store == 1 && stats-open-connections s1 == 1 && stats-in-use s1 == 0 && stats-idle s1 == 1 in
        -- 3. Two simultaneous open cursors open a 2nd connection; closing both retains 2 idle (default max-idle = 2).
        let ok-two = (case (sql-db-query db "SELECT * FROM t" [], sql-db-query db "SELECT * FROM t" []) of
          (Ok r1, Ok r2) ->
            let s-mid = sql-db-stats db in
            let mid-ok = stats-open-connections s-mid == 2 && stats-in-use s-mid == 2 && stats-idle s-mid == 0 in
            let _c1 = sql-rows-close r1 in
            let _c2 = sql-rows-close r2 in
            let s-after = sql-db-stats db in
            mid-ok && stats-open-connections s-after == 2 && stats-in-use s-after == 0 && stats-idle s-after == 2 && sql-memory-store-open-count store == 2
          _ -> False) in
        -- 4. Shrinking max-idle to 1 immediately evicts 1 idle connection and bumps max-idle-closed.
        let _ev = sql-db-set-max-idle-conns db 1 in
        let s2 = sql-db-stats db in
        let ok-evict = stats-open-connections s2 == 1 && stats-idle s2 == 1 && stats-max-idle-closed s2 == 1 && sql-memory-store-close-count store == 1 in
        -- 5. Opening 2 cursors with max-idle = 1 evicts the 2nd on release.
        let ok-release-evict = (case (sql-db-query db "SELECT * FROM t" [], sql-db-query db "SELECT * FROM t" []) of
          (Ok r1, Ok r2) ->
            let _c1 = sql-rows-close r1 in
            let _c2 = sql-rows-close r2 in
            let s3 = sql-db-stats db in
            stats-open-connections s3 == 1 && stats-idle s3 == 1 && stats-max-idle-closed s3 == 2 && sql-memory-store-close-count store == 2
          _ -> False) in
        -- 6. Setting max-open = 1 after max-idle = 5 clamps max-idle to 1; setting max-idle = 0 evicts all idle.
        let _m1 = sql-db-set-max-idle-conns db 5 in
        let _m2 = sql-db-set-max-open-conns db 1 in
        let s-clamped = sql-db-stats db in
        let ok-clamp = stats-max-open-connections s-clamped == 1 && stats-idle s-clamped == 1 in
        let _m3 = sql-db-set-max-idle-conns db 0 in
        let s-zero-idle = sql-db-stats db in
        let ok-zero-idle = stats-open-connections s-zero-idle == 0 && stats-idle s-zero-idle == 0 && sql-memory-store-close-count store == 3 in
        -- 7. Restore idle=2, run query, then sql-db-close closes idle conns and rejects future ops.
        let _m4 = sql-db-set-max-open-conns db 0 in
        let _m5 = sql-db-set-max-idle-conns db 2 in
        let _p = sql-db-ping db in
        let c1 = sql-db-close db in
        let c2 = sql-db-close db in
        let s-closed = sql-db-stats db in
        let all-closed = res-ok-unit c1 && res-ok-unit c2 && stats-open-connections s-closed == 0 && stats-idle s-closed == 0 && sql-memory-store-open-count store == sql-memory-store-close-count store in
        let rejected = (res-err (sql-db-ping db) SqlErrPoolClosed &&
          res-err (sql-db-exec db "SELECT * FROM t" []) SqlErrPoolClosed &&
          res-err (sql-db-query-row db "SELECT * FROM t" []) SqlErrPoolClosed &&
          (case sql-db-prepare db "SELECT * FROM t" of Err SqlErrPoolClosed -> True; _ -> False) &&
          (case sql-db-begin db of Err SqlErrPoolClosed -> True; _ -> False)) in
        [ok-lazy, ok-reuse, ok-two, ok-evict, ok-release-evict, ok-clamp, ok-zero-idle, all-closed, rejected]

  -- | Test automatic retry on SqlErrBadConn, exhaustion after 2 retries, SqlErrSkip fallback, and driver errors.
  test-retries-and-errors :: forall r. Unit ->{IO r, STM, Diverge} List Bool
  test-retries-and-errors u =
    let store = sql-memory-store-new () in
    let base = sql-memory-driver store in
    let fails = tvar-new-io (2 :: Nat) in
    let flaky = bad-conn-driver base fails in
    let ok-retry-succeeds = (case sql-open flaky "memory://retry" of
      Err _ -> False
      Ok db ->
        -- 2 failures then 3rd attempt succeeds (initial + 2 fresh retries).
        let r-ping = sql-db-ping db in
        let rem0 = tvar-read-io fails == 0 in
        -- Driver opened 3 conns (2 bad closed + 1 good kept idle).
        let open-ok = sql-memory-store-open-count store == 3 && sql-memory-store-close-count store == 2 in
        -- Now inject 3 failures: exceeds 2 retries and returns SqlErrBadConn.
        let _set3 = atomically (tvar-write fails 3) in
        let r-fail = sql-db-ping db in
        let clean-after = stats-in-use (sql-db-stats db) == 0 && stats-idle (sql-db-stats db) == 0 in
        let _cl = sql-db-close db in
        res-ok-unit r-ping && rem0 && open-ok && res-err r-fail SqlErrBadConn && clean-after) in
    -- SqlErrSkip fallback to prepare + stmt-exec / stmt-query.
    let prep-hits = tvar-new-io (0 :: Nat) in
    let skip-drv = skip-fastpath-driver (sql-memory-driver-new ()) prep-hits in
    let ok-skip = (case sql-open skip-drv "memory://skip" of
      Err _ -> False
      Ok db ->
        let e1 = sql-db-exec db "CREATE TABLE s (v)" [] in
        let e2 = sql-db-exec db "INSERT INTO s VALUES (?)" [sql-arg (SqlInt 77)] in
        let q1 = sql-db-query-row db "SELECT v FROM s" [] in
        let hits = tvar-read-io prep-hits in
        let _cl = sql-db-close db in
        let ok-val = (case q1 of Ok row -> res-ok (scan-i64 row 0) 77; Err _ -> False) in
        (case (e1, e2) of (Ok _, Ok _) -> True; _ -> False) && ok-val && hits == 3) in
    -- Driver open error rolls back reserved slot cleanly.
    let ok-open-err = (case sql-open (sql-memory-driver-new ()) "fail-open" of
      Err _ -> False
      Ok db ->
        let r = sql-db-ping db in
        let s = sql-db-stats db in
        let _cl = sql-db-close db in
        (case r of Err (SqlErrDriver _) -> True; _ -> False) && stats-open-connections s == 0 && stats-in-use s == 0) in
    -- Forced driver exec error keeps healthy connection reusable.
    let ok-exec-err = (case sql-open (sql-memory-driver-new ()) "memory://err" of
      Err _ -> False
      Ok db ->
        let r = sql-db-exec db "FAIL" [] in
        let s = sql-db-stats db in
        let _cl = sql-db-close db in
        (case r of Err (SqlErrDriver _) -> True; _ -> False) && stats-in-use s == 0 && stats-idle s == 1) in
    [ok-retry-succeeds, ok-skip, ok-open-err, ok-exec-err]

  -- | Run all pool, retry, and error checks.
  main :: Unit ->{IO r, STM, Diverge} Int
  main u =
    let c1 = test-pool-lifecycle () in
    let c2 = test-retries-and-errors () in
    checks (c1 <> c2) 1
"#;

const CONCURRENT_TEST: &str = r#"-- | Concurrent multi-task pool contention and shared-store queries.
module Main where
  import Data.Bool
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option hiding (or-else)
  import Data.Result
  import Data.Sql
  import Data.String
  import Effect.Concurrent
  import Effect.Sql
  import Effect.Stm
  import Relation.Eq

  -- | Insert `n` rows for a given worker ID via a shared pool gate and memory store.
  worker-insert :: forall r. SqlPoolGate -> SqlMemoryStore -> I64 -> I64 ->{IO r, STM, Diverge} Result LoadError Nat
  worker-insert gate store worker-id count =
    case sql-open-with-gate gate (sql-memory-driver store) "memory://conc" of
      Err _ -> Err (LoadError 1)
      Ok db -> insert-loop db worker-id count 0

  -- | Tail-recursive insert loop.
  insert-loop :: forall r. SqlDb r -> I64 -> I64 -> Nat ->{IO r, STM, Diverge} Result LoadError Nat
  insert-loop db worker-id rem done =
    if rem <= 0 then Ok done
    else
      case sql-db-exec db "INSERT INTO events VALUES (?, ?)" [sql-arg (SqlInt worker-id), sql-arg (SqlInt rem)] of
        Err _ -> Err (LoadError 2)
        Ok _ -> insert-loop db worker-id (wrapping-sub-i64 rem 1) (done + 1)

  -- | Hold an active transaction until `release-flag` turns True, signaling `entered` once acquired.
  tx-holder :: forall r. SqlPoolGate -> SqlMemoryStore -> TVar Nat -> TVar Bool ->{IO r, STM, Diverge} Result LoadError Nat
  tx-holder gate store entered release-flag =
    case sql-open-with-gate gate (sql-memory-driver store) "memory://hold" of
      Err _ -> Err (LoadError 3)
      Ok db ->
        case sql-db-begin db of
          Err _ -> Err (LoadError 4)
          Ok tx ->
            case atomically (tvar-modify entered (\n -> n + 1)) of
              _ ->
                case atomically (stm-bind (tvar-read release-flag) (\go -> check go)) of
                  _ -> case sql-tx-commit tx of
                    Err _ -> Err (LoadError 5)
                    Ok _ -> Ok 1

  -- | Wait until both holders have checked out connections, then ping (which blocks on max-open = 2),
  -- while a fourth task waits until wait-count >= 1 and releases the holders.
  blocked-pinger :: forall r. SqlPoolGate -> SqlMemoryStore -> TVar Nat ->{IO r, STM, Diverge} Result LoadError Nat
  blocked-pinger gate store entered =
    case atomically (stm-bind (tvar-read entered) (\n -> check (n == 2))) of
      _ -> case sql-open-with-gate gate (sql-memory-driver store) "memory://wait" of
        Err _ -> Err (LoadError 6)
        Ok db -> case sql-db-ping db of
          Err _ -> Err (LoadError 7)
          Ok _ -> Ok 1

  -- | Wait until the pool gate records at least one waiter, then release the transaction holders.
  waiter-releaser :: SqlPoolGate -> TVar Bool ->{STM, Diverge} Result LoadError Nat
  waiter-releaser gate release-flag =
    let _ = wait-for-waiter gate in
    case atomically (tvar-write release-flag True) of
      _ -> Ok 1

  -- | Poll until wait-count >= 1 using STM retry on a helper read.
  wait-for-waiter :: SqlPoolGate ->{STM, Diverge} Nat
  wait-for-waiter gate =
    if stats-wait-count (sql-pool-gate-stats gate) >= 1 then 1
    else wait-for-waiter gate

  -- | Spawn 4 concurrent workers inserting 20 rows each through a max-open=2 shared gate.
  run-parallel-inserts :: forall r. using (io :: Io r) => SqlPoolGate -> SqlMemoryStore ->{Async r, IO r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat, Nat, Nat)
  run-parallel-inserts gate store = withScope $ \s -> do
    h1 <- spawn s (\() -> worker-insert gate store 1 20)
    h2 <- spawn s (\() -> worker-insert gate store 2 20)
    h3 <- spawn s (\() -> worker-insert gate store 3 20)
    h4 <- spawn s (\() -> worker-insert gate store 4 20)
    joinAllOrCancel s h1 h2 h3 h4

  -- | Spawn 2 tx holders + 1 blocked pinger + 1 releaser to prove blocking wait-count and wakeup.
  run-contention :: forall r. using (io :: Io r) => SqlPoolGate -> SqlMemoryStore -> TVar Nat -> TVar Bool ->{Async r, IO r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat, Nat, Nat)
  run-contention gate store entered release-flag = withScope $ \s -> do
    h1 <- spawn s (\() -> tx-holder gate store entered release-flag)
    h2 <- spawn s (\() -> tx-holder gate store entered release-flag)
    h3 <- spawn s (\() -> blocked-pinger gate store entered)
    h4 <- spawn s (\() -> waiter-releaser gate release-flag)
    joinAllOrCancel s h1 h2 h3 h4

  -- | Run concurrent pool and contention tests.
  main :: Unit ->{Async r, IO r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let store = sql-memory-store-new () in
    let gate = sql-pool-gate-new () in
    let _lim = sql-pool-gate-set-max-open gate 2 in
    case sql-open-with-gate gate (sql-memory-driver store) "memory://init" of
      Err _ -> 1
      Ok db ->
        case sql-db-exec db "CREATE TABLE events (worker, seq)" [] of
          Err _ -> 2
          Ok _ ->
            case run-parallel-inserts gate store of
              Err _ -> 3
              Ok counts -> case counts of
                (c1, c2, c3, c4) ->
                  if c1 + c2 + c3 + c4 /= 80 then 4
                  else case sql-db-query-row db "SELECT COUNT(*) FROM events" [] of
                    Err _ -> 5
                    Ok row ->
                      case scan-i64 row 0 of
                        Err _ -> 6
                        Ok total ->
                          if total /= 80 then 6
                          else
                            let entered = tvar-new-io (0 :: Nat) in
                            let release-flag = tvar-new-io False in
                            case run-contention gate store entered release-flag of
                              Err _ -> 7
                              Ok _ ->
                                let st = sql-pool-gate-stats gate in
                                if stats-wait-count st < 1 then 8
                                else if stats-in-use st /= 0 then 9
                                else if stats-open-connections st /= 0 then 10
                                else if sql-memory-store-open-count store /= sql-memory-store-close-count store then 11
                                else 200
"#;

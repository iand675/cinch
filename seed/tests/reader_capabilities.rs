//! Generalized lexical capabilities (`#[capability]`), higher-rank `using`
//! constraints, and `Effect.Reader` (N readers, `local`, `with-reader`,
//! `TaggedRead`, explicit `@{...}` supply, borrowed `Ref r`, and purity).
//!
//! Routed through `common::build_executable` / `common::check` so the same
//! suite runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;
use std::process::Command;

fn run_leak_checked(tag: &str, source: &str) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let exe = common::build_executable(&dir, "Main.cinch", "reader-cap")
        .unwrap_or_else(|e| panic!("{tag}: {e}"));
    let out = Command::new(&exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_HEAP_LIMIT", "256M")
        .output()
        .expect("run the built program");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(42), "{tag}: exit status; stderr:\n{stderr}");
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: every block must be released exactly once; stderr:\n{stderr}"
    );
    common::cleanup(&dir);
}

fn refused_any(tag: &str, source: &str, needles: &[&str]) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let error = common::check(&dir, "Main.cinch").expect_err(tag);
    assert!(
        needles.iter().any(|needle| error.contains(needle)),
        "{tag}: expected one of {needles:?} in:\n{error}"
    );
    common::cleanup(&dir);
}

#[test]
fn multi_reader_local_and_with_reader() {
    run_leak_checked(
        "reader-multi",
        r#"module Main where
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)
  import Effect.Reader

  record AppConfig where
    base :: Int
    db-port :: Int

  record DbConfig where
    port :: Int

  data Primary where
    Primary :: Primary

  data Secondary where
    Secondary :: Secondary

  read-db :: using (db :: Read DbConfig) => Int
  read-db = asks (\d -> d.port)

  read-primary :: using (p :: TaggedRead Primary Int) => Int
  read-primary = ask-tagged

  read-secondary :: using (s :: TaggedRead Secondary Int) => Int
  read-secondary = ask-tagged

  compute :: using (cfg :: Read AppConfig, label :: Read String, p :: TaggedRead Primary Int, s :: TaggedRead Secondary Int) => Int
  compute =
    let x = asks (\c -> c.base) in
    let y = nat-to-int (string-length ask) in
    let inner-base = local (\c -> AppConfig { base = c.base + 10, db-port = c.db-port }) (\() -> asks (\c -> c.base)) in
    let after-local = asks (\c -> c.base) in
    let port = with-reader (\c -> DbConfig { port = c.db-port }) (\() -> read-db) in
    let tagged = local-tagged (\n -> n + 3) (\() -> read-primary + read-secondary) in
    let after-tagged = read-primary in
    -- 2 + 3 + 12 + 2 + 5 + 20 + 7 = 51; 51 - 9 = 42
    x + y + inner-base + after-local + port + tagged + after-tagged - 9

  main :: Int
  main =
    run-reader (AppConfig { base = 2, db-port = 5 }) (\() ->
      run-reader "abc" (\() ->
        with (p = (TaggedRead 7 :: TaggedRead Primary Int)) in
        with (s = (TaggedRead 10 :: TaggedRead Secondary Int)) in
          compute))
"#,
    );
}

#[test]
fn explicit_capability_supply_and_transparent_capability() {
    run_leak_checked(
        "reader-explicit",
        r#"module Main where
  import Effect.Reader

  #[capability]
  #[repr(transparent)]
  data TokenCap where
    TokenCap :: Int -> TokenCap

  reveal :: using (s :: TokenCap) => Int
  reveal = case s of
    TokenCap n -> n

  apply-cap :: (using (r :: Read Int) => Int -> Int) -> Int
  apply-cap f = f @{r = Read 20} 22

  main :: Int
  main =
    let a = reveal @{s = TokenCap 10} in
    let b = apply-cap (\x -> ask + x) in
    -- a=10, b=42 => b + a - 10 = 42
    b + a - 10
"#,
    );
}

#[test]
fn reader_with_counted_strings_and_purity() {
    run_leak_checked(
        "reader-strings",
        r#"module Main where
  import Algebra.Semigroup
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)
  import Effect.Reader

  greet-len :: using (prefix :: Read String) => String -> Int
  greet-len name = nat-to-int (string-length (ask <> name))

  main :: Int
  main =
    run-reader "hello-" (\() ->
      let a = greet-len "world" in
      let b = local (\p -> p <> "extra-") (\() -> greet-len "cinch") in
      let c = greet-len "world" in
      -- a=11, b=17, c=11 => 39 + 3 = 42
      a + b + c + 3)
"#,
    );
}

#[test]
fn reader_with_borrowed_ref_and_region_escape_refused() {
    run_leak_checked(
        "reader-ref-ok",
        r#"module Main where
  import Effect.Reader

  read-ref :: forall r. using (cap :: Read (Ref r Int)) => Int
  read-ref =
    let ref = (ask :: Ref r Int) in
    copyOut ref

  run-with-ref :: forall r. Ref r Int -> Int
  run-with-ref ref = with (c = Read ref) in read-ref

  main :: Int
  main =
    let cfg = 42 in
    run-with-ref (borrow cfg)
"#,
    );
    refused_any(
        "reader-ref-escape",
        r#"module Main where
  import Effect.Reader

  data List2 a where
    Nil2 :: List2 a
    Cons2 :: a -> List2 a -> List2 a

  bad :: forall region. Ref region (List2 Nat)
  bad = with (c = Read (borrow (Cons2 1 Nil2))) in ask

  main :: Int
  main = 0
"#,
        &["escape", "region", "borrow", "signature"],
    );
}

#[test]
fn missing_capability_is_refused() {
    refused_any(
        "reader-missing",
        r#"module Main where
  import Effect.Reader

  main :: Int
  main = ask
"#,
        &["capability in scope", "no Read"],
    );
}

#[test]
fn ordinary_data_is_refused_in_with() {
    refused_any(
        "reader-ordinary-refused",
        r#"module Main where
  data Plain where
    Plain :: Int -> Plain

  main :: Int
  main = with (p = Plain 1) in 0
"#,
        &["checked signature", "cannot supply"],
    );
}

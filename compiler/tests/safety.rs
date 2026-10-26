//! P20: the safety assertion attributes (attribute annotations
//! specification section 10; design/safety-assertion-attributes.md;
//! decisions D111-D114).

use safelang::check::{check_program_multi_diag, Diagnostic, StagingCx};
use safelang::core::Program;
use safelang::driver::{check_sources, parse_prelude, parse_source, run_source};
use safelang::interface::Iface;
use safelang::Result;
use std::collections::HashMap;

/// Check `src` against the prelude family, returning the elaborated
/// program, the interfaces (the family first), and the non-fatal
/// diagnostics.
#[allow(dead_code)]
fn check_with_diags(src: &str) -> Result<(Program, Vec<Iface>, Vec<Diagnostic>)> {
    let prelude = parse_prelude(&[])?;
    let user = parse_source(src, "<test>", &prelude.base, &HashMap::new())?;
    let mut hashes = HashMap::new();
    let mut files = HashMap::new();
    let mut fixities = prelude.fixities.clone();
    for (m, f) in &user.fixities {
        fixities.insert(m.clone(), f.clone());
    }
    for m in &user.modules {
        hashes.insert(m.name.text.clone(), String::new());
        files.insert(m.name.text.clone(), "<test>".to_string());
    }
    let mut scx = StagingCx::default();
    scx.fixities = fixities;
    scx.files = files;
    scx.entry_modules = user.modules.iter().map(|m| m.name.text.clone()).collect();
    let (program, ifaces, _report, diags) =
        check_program_multi_diag(&prelude.modules, &user.modules, &hashes, &scx)?;
    Ok((program, ifaces, diags))
}

#[allow(dead_code)]
fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

#[allow(dead_code)]
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

#[test]
#[ignore]
fn scratch_tail_shapes() {
    let src = r#"
module Main where

  sum-to :: Int -> Int -> Int
  sum-to acc n = if n == 0 then acc else sum-to (acc + n) (n - 1)

  ping :: Int -> Int
  ping n = if n == 0 then 0 else pong (n - 1)

  pong :: Int -> Int
  pong n = if n == 0 then 1 else ping (n - 1)

  notail :: Int -> Int
  notail n = if n == 0 then 0 else 1 + notail (n - 1)

  hid :: Int -> Int
  hid n = if n == 0 then 0 else (\x -> hid (x - 1)) n

  steppy :: using (seq :: ResultSequencing Int) => Result Int Int -> Result Int Int
  steppy r = do
    x <- r
    if x == 0 then pure 0 else steppy (pure (x - 1))

  main :: Int
  main = sum-to 0 10 + ping 4 + notail 2 + hid 1
"#;
    let prog = check_sources(src, "<scratch>").expect("check");
    for f in &prog.fns {
        if f.id.0.starts_with("Main.") {
            println!("=== {} params={:?}\n{:?}\n", f.id.0, f.params, f.body);
        }
    }
    for g in safelang::core::tail_report(&prog) {
        if g.members.iter().any(|m| m.starts_with("Main.")) {
            println!("GROUP {:?} mark={}", g.members, g.mark);
        }
    }
    panic!("scratch: show output");
}

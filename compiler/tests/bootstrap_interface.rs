//! Rung: reading a compiled module interface (`Bootstrap.Interface`).
//!
//! A self-hosted compiler consumes its dependencies' `.cinchif` artifacts
//! instead of re-parsing their sources, so this test proves the rung reads a
//! real one, with nothing taken on trust:
//!
//!   1. a small project (`Dep`, `Lib`, `Main`) is compiled by the driver,
//!      which writes `build/Lib.cinchif`;
//!   2. the artifact's bytes, the module's own source bytes, and the answers
//!      computed from the compiler's own model of that artifact are handed to
//!      `bootstrap/programs/Run.Interface.cinch` as literals;
//!   3. the program, compiled and RUN through the real driver, exits 0 only
//!      when every answer holds, and its report on standard output carries the
//!      contract text the interface identity was recomputed over;
//!   4. one byte of the artifact (or of the source, or of a second read of the
//!      artifact) is altered per negative case, and the exit code names which
//!      answer failed.
//!
//! The expectations come from `cinch::interface`, the writer's own model, and
//! from `fnv1a64`/SHA-256 implemented here, so the Cinch reader is checked
//! against an independent reader rather than against a literal copied out of
//! its own report. The identity rule is checked twice: `Iface::iface_hash`
//! (the compiler's implementation) and the blanked-contract FNV computed in
//! this file must agree before either is used as an expectation.
//!
//! Answer numbers the exit code can be, in the order the program evaluates
//! them: 1 the artifact does not parse, 2 module name, 3 schema version,
//! 4 source hash, 5 value names, 6 scheme non-empty, 7 scheme stability,
//! 8 imported module names, 9 fixity rows, 12 a field the identity rule needs,
//! 10 the recomputed interface identity, 11 the source's SHA-256.

mod common;

use std::path::Path;
use std::process::Command;

const INTERFACE_SRC: &str = include_str!("../../bootstrap/Bootstrap.Interface.cinch");
const RUN_SRC: &str = include_str!("../../bootstrap/programs/Run.Interface.cinch");

/// The module whose interface is read: a data type, an operator with a fixity
/// declaration, a value used through an operator, and an import, so the
/// artifact has an import list, a value list and a fixity row to read. Raw
/// literals on purpose: the bytes written into the project and the bytes
/// handed to the program as "the module's own source" must be identical, and
/// `\`-continuations would strip the indentation the language's layout needs.
const LIB_SRC: &str = r#"module Lib where
  import Dep
  export Token, Tok, describe, add
  infixl 6 |+|
  data Token where
    Tok :: Int -> Token
  (|+|) :: Int -> Int -> Int
  (|+|) a b = a + b
  add :: Int -> Int -> Int
  add a b = a |+| b
  describe :: Token -> Int
  describe t = case t of
    Tok n -> Dep.twice n
"#;

const DEP_SRC: &str = r#"module Dep where
  export twice
  twice :: Int -> Int
  twice x = x + x
"#;

const MAIN_SRC: &str = r#"module Main where
  import Lib
  main :: Int
  main = if Lib.describe (Lib.Tok 4) == 8 then 0 else 1
"#;

// ---------------------------------------------------------------- hashing

/// FNV-1a 64, the hash the interface records: `interface::content_hash` for
/// the source hash, and the rule `Iface::iface_hash` applies to the contract.
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// SHA-256 (FIPS 180-4), lowercase hex: an implementation independent of the
/// runtime's `cinch.runtime.textSha256`, pinned to the standard vector in
/// `sha256_is_sha256` below. It is what the program's answer 11 is checked
/// against, so the check is not the runtime confirming itself.
fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
        0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
        0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
        0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
        0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c,
        0x1f83d9ab, 0x5be0cd19,
    ];
    let mut msg = data.to_vec();
    let bits = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bits.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[4 * i], chunk[4 * i + 1], chunk[4 * i + 2], chunk[4 * i + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d) = (h[0], h[1], h[2], h[3]);
        let (mut e, mut f, mut g, mut hh) = (h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (slot, v) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(v);
        }
    }
    let mut out = String::new();
    for v in h {
        out.push_str(&format!("{v:08x}"));
    }
    out
}

#[test]
fn sha256_is_sha256() {
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        "the test's own SHA-256 must be SHA-256"
    );
    assert_eq!(
        sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

// ------------------------------------------------------------ the program

/// A Cinch string literal for these bytes. They must be UTF-8 (a Cinch
/// string is UTF-8 text); every byte outside printable ASCII is escaped, so
/// the literal is one line and the parser's escape set is respected.
fn cinch_lit(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ' '..='~' => out.push(c),
            other => out.push_str(&format!("\\u{{{:X}}}", other as u32)),
        }
    }
    out.push('"');
    out
}

/// A `List String` literal in artifact order.
fn cinch_list(xs: &[String]) -> String {
    let mut out = String::from("Nil");
    for x in xs.iter().rev() {
        out = format!("Cons {} ({})", cinch_lit(x), out);
    }
    out
}

/// Everything the program is handed: the artifact's bytes, a second read of
/// them, the module's source bytes, and the SHA-256 the harness computed.
#[derive(Clone)]
struct Inputs {
    artifact: Vec<u8>,
    copy: Vec<u8>,
    source: String,
    sha256: String,
}

/// The compiled program for these inputs, with the placeholder bindings
/// substituted. Each placeholder must occur exactly once, so a rename in the
/// artifact cannot silently skip a substitution.
fn program_of(inputs: &Inputs, exp: &Expect) -> String {
    let artifact = std::str::from_utf8(&inputs.artifact).expect("the artifact is UTF-8");
    let copy = std::str::from_utf8(&inputs.copy).expect("the second read is UTF-8");
    let subs: [(&str, String); 10] = [
        ("artifact-text = \"\"", format!("artifact-text = {}", cinch_lit(artifact))),
        ("artifact-copy-text = \"\"", format!("artifact-copy-text = {}", cinch_lit(copy))),
        ("source-text = \"\"", format!("source-text = {}", cinch_lit(&inputs.source))),
        ("expected-module = \"\"", format!("expected-module = {}", cinch_lit(&exp.module))),
        (
            "expected-source-hash = \"\"",
            format!("expected-source-hash = {}", cinch_lit(&exp.source_hash)),
        ),
        (
            "expected-iface-hash = \"\"",
            format!("expected-iface-hash = {}", cinch_lit(&exp.iface_hash)),
        ),
        (
            "expected-sha256-hex = \"\"",
            format!("expected-sha256-hex = {}", cinch_lit(&inputs.sha256)),
        ),
        ("expected-imports = Nil", format!("expected-imports = {}", cinch_list(&exp.imports))),
        (
            "expected-value-names = Nil",
            format!("expected-value-names = {}", cinch_list(&exp.values)),
        ),
        (
            "expected-fixity-rows = Nil",
            format!("expected-fixity-rows = {}", cinch_list(&exp.fixities)),
        ),
    ];
    let mut src = RUN_SRC.to_string();
    for (placeholder, replacement) in subs {
        assert_eq!(
            src.matches(placeholder).count(),
            1,
            "the program must carry `{placeholder}` exactly once"
        );
        src = src.replace(placeholder, &replacement);
    }
    src
}

fn rewrite(dir: &Path, program: &str) {
    std::fs::write(dir.join("Main.cinch"), program).expect("write the program");
}

// --------------------------------------------------------------- the run

/// Compile the program with `common::build` and run the executable, so the
/// program's own standard output (its report) is captured and not just its
/// exit code.
fn build_and_run(dir: &Path) -> (i32, String) {
    let exe = common::build(dir, "Main.cinch", "program").expect("the program must build");
    let out = Command::new(&exe)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("run {}: {e}", exe.display()));
    (
        out.status.code().unwrap_or(1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

// ---------------------------------------------------------- expectations

/// The answers read from the compiler's own model of the artifact.
struct Expect {
    module: String,
    source_hash: String,
    iface_hash: String,
    imports: Vec<String>,
    values: Vec<String>,
    fixities: Vec<String>,
    contract: String,
}

/// The artifact for `Lib` from a freshly compiled project, and the answers
/// read from it by `cinch::interface`.
fn compiled_artifact() -> (Vec<u8>, Expect) {
    let dir = common::project(
        "iface-src",
        &[
            ("Dep.cinch", DEP_SRC),
            ("Lib.cinch", LIB_SRC),
            ("Main.cinch", MAIN_SRC),
        ],
    );
    common::check(&dir, "Main.cinch").expect("the sample project must check");
    let artifact = std::fs::read(dir.join("build/Lib.cinchif")).expect("the driver wrote an interface");
    common::cleanup(&dir);

    let text = String::from_utf8(artifact.clone()).expect("the artifact is UTF-8 JSON");
    // The driver writes the artifact in Text.Json's canonical form: keys in
    // byte order, no insignificant whitespace, no trailing newline. The file
    // is a fixed point of parse-then-canonical-print; pinning that here means
    // a regression to a pretty writer fails loudly rather than silently
    // changing every layout-dependent expectation.
    let iface = cinch::interface::Iface::from_json(&text).expect("the artifact parses as an interface");
    assert_eq!(
        text,
        iface.to_json(),
        "the artifact on disk is the canonical serialization of the interface"
    );
    assert!(
        !text.contains('\n') && !text.contains('\t'),
        "the canonical form has no insignificant whitespace"
    );

    // The artifact records the FNV-1a 64 of its own source text.
    let source_hash = cinch::interface::content_hash(LIB_SRC);
    assert_eq!(
        iface.source_hash, source_hash,
        "the artifact records the FNV of the source it was compiled from"
    );
    assert_eq!(iface.module, "Lib");

    // The identity rule, as this test understands it: blank the fields that
    // are not the contract, clear the representation table, hash the text.
    let mut blanked = iface.clone();
    blanked.source_hash.clear();
    blanked.compiler.clear();
    blanked.units.clear();
    let contract = blanked.to_json();
    let rule = fnv1a64(contract.as_bytes()).to_string();
    assert_eq!(
        rule,
        iface.iface_hash(),
        "the identity rule is the FNV of the blanked canonical contract"
    );
    // The contract must be the artifact's own bytes with only those regions
    // changed: that is what lets the Cinch reader recompute it by substitution
    // rather than by re-serializing. Checked as a substitution here, so the
    // claim is about the rule and not about the printer's layout.
    let substituted = text.replace(
        &format!("\"source_hash\":\"{}\"", iface.source_hash),
        "\"source_hash\":\"\"",
    );
    let substituted = substituted.replace(
        &format!("\"compiler\":\"{}\"", iface.compiler),
        "\"compiler\":\"\"",
    );
    assert_eq!(
        substituted, contract,
        "the contract is the artifact's bytes with the three regions blanked"
    );
    assert!(contract.contains("\"source_hash\":\"\""), "source hash blanked");
    assert!(contract.contains("\"compiler\":\"\""), "compiler blanked");
    assert!(contract.contains("\"units\":[]"), "units cleared");
    assert!(
        contract.contains(&format!("\"schema\":{}", iface.schema)),
        "the contract keeps every other field as written"
    );

    let exp = Expect {
        module: iface.module.clone(),
        source_hash: source_hash.clone(),
        iface_hash: iface.iface_hash(),
        imports: iface.imports.iter().map(|r| r.module.clone()).collect(),
        values: iface.exports.values.iter().map(|v| v.name.clone()).collect(),
        fixities: iface
            .exports
            .fixities
            .iter()
            .map(|f| format!("{} {} {}", f.op, f.assoc, f.prec))
            .collect(),
        contract,
    };
    (artifact, exp)
}

/// One `key value` line of the report.
fn report_field<'a>(report: &'a str, key: &str) -> &'a str {
    for line in report.lines() {
        if let Some(rest) = line.strip_prefix(key) {
            if let Some(value) = rest.strip_prefix(' ') {
                return value;
            }
        }
    }
    panic!("the report has no `{key}` line:\n{report}");
}

/// The contract text the report carries between its markers.
fn report_contract(report: &str) -> &str {
    let after = report
        .split_once("--- contract ---\n")
        .expect("the report marks the contract")
        .1;
    after
        .split_once("--- end ---")
        .expect("the report closes the contract")
        .0
}

// ------------------------------------------------------------------ tests

#[test]
fn the_interface_reader_answers_a_real_artifact() {
    let (artifact, exp) = compiled_artifact();
    let dir = common::project(
        "iface-run",
        &[
            ("Bootstrap.Interface.cinch", INTERFACE_SRC),
            ("Main.cinch", RUN_SRC),
        ],
    );
    let inputs = Inputs {
        artifact: artifact.clone(),
        copy: artifact.clone(),
        source: LIB_SRC.to_string(),
        sha256: sha256_hex(LIB_SRC.as_bytes()),
    };

    // The positive case: every answer holds, and the report says so.
    rewrite(&dir, &program_of(&inputs, &exp));
    let (code, report) = build_and_run(&dir);
    assert_eq!(
        code, 0,
        "every answer must hold; exit {code} names the first that did not.\n{report}"
    );
    assert_eq!(report_field(&report, "module"), "Lib");
    assert_eq!(report_field(&report, "schema"), "1");
    assert_eq!(report_field(&report, "source-hash-recorded"), exp.source_hash);
    assert_eq!(report_field(&report, "source-hash-fnv"), exp.source_hash);
    assert_eq!(report_field(&report, "source-sha256"), inputs.sha256);
    assert_eq!(report_field(&report, "imports"), "2");
    assert_eq!(report_field(&report, "values"), "2");
    assert_eq!(report_field(&report, "fixities"), "1");
    assert_eq!(report_field(&report, "iface-hash"), exp.iface_hash);
    assert_eq!(report_field(&report, "schemes-stable"), "yes");
    for name in &exp.imports {
        assert!(
            report.contains(&format!("import {name}\n")),
            "the report names the import {name}:\n{report}"
        );
    }
    for name in &exp.values {
        assert!(
            report.contains(&format!("value {name} :: ")),
            "the report names the value {name}:\n{report}"
        );
    }
    for row in &exp.fixities {
        assert!(
            report.contains(&format!("fixity {row}\n")),
            "the report carries the fixity row `{row}`:\n{report}"
        );
    }

    // The strongest evidence for the identity rule: the contract text the
    // Cinch reader derived by substitution is byte for byte the text the
    // compiler's own `iface_hash` hashes.
    let derived = report_contract(&report);
    assert_eq!(
        derived.len(),
        exp.contract.len(),
        "the derived contract is the same length"
    );
    assert_eq!(derived, exp.contract, "the derived contract is the compiler's contract");

    // Each negative case alters one thing and must name the answer it breaks.
    let mut cases: Vec<(&str, i32, Inputs)> = Vec::new();

    let mut not_json = inputs.clone();
    not_json.artifact = b"this is not a JSON document".to_vec();
    cases.push(("not a JSON document", 1, not_json));

    let mut bad_module = inputs.clone();
    bad_module.artifact = fixed(&bad_module.artifact, "\"module\":\"Lib\"", "\"module\":\"Libx\"");
    cases.push(("a changed module name", 2, bad_module));

    let mut bad_schema = inputs.clone();
    bad_schema.artifact = fixed(&bad_schema.artifact, "\"schema\":1", "\"schema\":2");
    cases.push(("a changed schema version", 3, bad_schema));

    let mut bad_source = inputs.clone();
    bad_source.source = format!("{LIB_SRC}\n");
    cases.push(("a changed source byte", 4, bad_source));

    let mut bad_name = inputs.clone();
    bad_name.artifact = fixed(&bad_name.artifact, "\"name\":\"add\"", "\"name\":\"addy\"");
    cases.push(("a changed value name", 5, bad_name));

    let mut bad_scheme = inputs.clone();
    bad_scheme.artifact = fixed(
        &bad_scheme.artifact,
        "\"scheme\":\"(Int -> (Int -> Int))\"",
        "\"scheme\":\"\"",
    );
    cases.push(("an emptied scheme text", 6, bad_scheme));

    let mut bad_copy = inputs.clone();
    bad_copy.copy = fixed(
        &bad_copy.copy,
        "\"scheme\":\"(Int -> (Int -> Int))\"",
        "\"scheme\":\"(Int -> (Int -> Int!))\"",
    );
    cases.push(("a second read with a different scheme", 7, bad_copy));

    let mut bad_import = inputs.clone();
    bad_import.artifact = fixed(&bad_import.artifact, "\"module\":\"Dep\"", "\"module\":\"Depx\"");
    cases.push(("a changed import name", 8, bad_import));

    let mut bad_fixity = inputs.clone();
    bad_fixity.artifact = fixed(&bad_fixity.artifact, "\"prec\":6", "\"prec\":7");
    cases.push(("a changed fixity precedence", 9, bad_fixity));

    let mut bad_arity = inputs.clone();
    bad_arity.artifact = fixed(&bad_arity.artifact, "\"arity\":2", "\"arity\":3");
    cases.push(("a changed value arity (only the identity covers it)", 10, bad_arity));

    let mut bad_digest = inputs.clone();
    bad_digest.sha256 = "0".repeat(64);
    cases.push(("a digest the harness got wrong", 11, bad_digest));

    let mut no_compiler = inputs.clone();
    no_compiler.artifact = fixed(&no_compiler.artifact, "\"compiler\"", "\"compilerx\"");
    cases.push(("an artifact without the compiler field", 12, no_compiler));

    for (what, expected, case) in cases {
        rewrite(&dir, &program_of(&case, &exp));
        // `common::run` drives the program through the real driver, which
        // reports a process killed by a signal as exit 1 (the same number the
        // program itself uses for "the artifact is not a JSON document"), so a
        // killed run shows up here as a mismatched 1 rather than as a wrong
        // answer.
        let code = common::run(&dir, "Main.cinch");
        assert_eq!(code, expected, "{what}: the exit code must name the failed answer");
    }

    common::cleanup(&dir);
}

/// `bytes` with the first occurrence of `from` replaced by `to`; the case is
/// wrong if the needle is not there exactly once, which also pins the
/// artifact's canonical spelling.
fn fixed(bytes: &[u8], from: &str, to: &str) -> Vec<u8> {
    let text = std::str::from_utf8(bytes).expect("the artifact is UTF-8");
    assert_eq!(text.matches(from).count(), 1, "`{from}` must occur once");
    text.replace(from, to).into_bytes()
}

//! End-to-end tests for temporal model-checking debugging tools:
//! - CLI: `cinch model inspect` (text & `--json`), `cinch model trace` (`--state`, `--deadlock`, `--where`), `cinch model dot`
//! - REPL: `:machines`, `:machine`, `:explore`, `:trace`, `:dot`, and tab completion
//! - LSP / Analysis: rich Markdown hover on `machine` and `contract` symbols (including failing models), go-to-definition from `contract`/`refines`/`actions`

use cinch::analysis::Analysis;
use cinch::model_cli;
use cinch::repl::{Repl, ReplOutcome};
use std::path::PathBuf;

const SAMPLE_OK: &str = r#"module Main where
  data Node where
    N0 :: Node
    N1 :: Node

  record Sys where
    owner :: Option Node
    left :: Nat
    right :: Nat

  acquire :: Node -> Sys -> Option Sys
  acquire n s = case s.owner of
    None -> Some (Sys { owner = Some n, left = s.left, right = s.right })
    Some _ -> None

  release :: Sys -> Option Sys
  release s = case s.owner of
    Some _ -> Some (Sys { owner = None, left = s.left, right = s.right })
    None -> None

  bumpLeft :: Sys -> Option Sys
  bumpLeft s = if s.left == 0 then Some (Sys { owner = s.owner, left = 1, right = s.right }) else None

  bumpRight :: Sys -> Option Sys
  bumpRight s = if s.right == 0 then Some (Sys { owner = s.owner, left = s.left, right = 1 }) else None

  machine LockSys where
    state = Sys
    init = Sys { owner = None, left = 0, right = 0 }
    actions = [acquire, release, bumpLeft, bumpRight]
    symmetry = [Node]

  contract safety bounds for LockSys where
    always = (\s -> s.left <= 1 && s.right <= 1)

  main :: Int
  main = 0
"#;

const SAMPLE_BUGGY: &str = r#"module Main where
  record Limiter where
    n :: Nat

  inc :: Limiter -> Option Limiter
  inc s = if s.n < 2 then Some (Limiter { n = s.n + 1 }) else None

  machine Stuck where
    state = Limiter
    init = Limiter { n = 0 }
    actions = [inc]

  contract safety capOne for Stuck where
    always = (\s -> s.n <= 1)

  contract safety noDeadlock for Stuck where
    deadlock_free = (True)

  main :: Int
  main = 0
"#;

fn write_temp_file(tag: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_model_tool_{}_{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("Main.cinch");
    std::fs::write(&file, src).unwrap();
    file
}

#[test]
fn cli_model_inspect_trace_and_dot_work_on_passing_and_buggy_models() {
    let ok_file = write_temp_file("ok", SAMPLE_OK);
    let ok_str = ok_file.to_string_lossy().to_string();

    assert_eq!(
        model_cli::cli(&["inspect".into(), ok_str.clone()]),
        0,
        "cinch model inspect should return 0 on passing machine"
    );
    assert_eq!(
        model_cli::cli(&["inspect".into(), ok_str.clone(), "LockSys".into(), "--json".into()]),
        0,
        "cinch model inspect --json should return 0"
    );
    assert_eq!(
        model_cli::cli(&[
            "trace".into(),
            ok_str.clone(),
            "LockSys".into(),
            "--where".into(),
            "(\\s -> s.left == 1 && s.right == 1)".into(),
        ]),
        0,
        "cinch model trace --where should find reachable state"
    );

    let dot_out = ok_file.with_extension("dot");
    assert_eq!(
        model_cli::cli(&[
            "dot".into(),
            ok_str,
            "LockSys".into(),
            "-o".into(),
            dot_out.to_string_lossy().to_string(),
        ]),
        0
    );
    let dot_text = std::fs::read_to_string(&dot_out).unwrap();
    assert!(dot_text.contains("digraph \"LockSys\""), "got: {dot_text}");
    assert!(dot_text.contains("acquire(N0)"), "got: {dot_text}");

    let bad_file = write_temp_file("bad", SAMPLE_BUGGY);
    let bad_str = bad_file.to_string_lossy().to_string();
    // Inspect returns exit 1 when any contract fails, while still reporting all machines/contracts!
    assert_eq!(model_cli::cli(&["inspect".into(), bad_str.clone()]), 1);
    assert_eq!(
        model_cli::cli(&["trace".into(), bad_str, "Stuck".into(), "--deadlock".into()]),
        0
    );
}

#[test]
fn repl_machine_debugging_commands_explore_trace_and_dot() {
    let file = write_temp_file("repl", SAMPLE_OK);
    let mut repl = Repl::with_root(file.parent().unwrap());

    let loaded = repl.eval(&format!(":load {}", file.display()));
    assert!(matches!(loaded, ReplOutcome::Output(_)), "got: {loaded:?}");

    let ReplOutcome::Output(ms) = repl.eval(":machines") else { panic!("expected Output") };
    assert!(ms.contains("LockSys"), "got: {ms}");
    assert!(ms.contains("1/1 contracts ok"), "got: {ms}");

    let ReplOutcome::Output(detail) = repl.eval(":machine LockSys") else { panic!("expected Output") };
    assert!(detail.contains("POR independent pairs"), "got: {detail}");
    assert!(detail.contains("bumpLeft || bumpRight"), "got: {detail}");
    assert!(detail.contains("params: [N0, N1]"), "got: {detail}");

    let ReplOutcome::Output(exp0) = repl.eval(":explore LockSys") else { panic!("expected Output") };
    assert!(exp0.contains("State #0 (init)"), "got: {exp0}");
    assert!(exp0.contains("changed:"), "got: {exp0}");

    let ReplOutcome::Output(tr) = repl.eval(":trace LockSys 1") else { panic!("expected Output") };
    assert!(tr.contains("State 0 (init)"), "got: {tr}");

    let ReplOutcome::Output(dot) = repl.eval(":dot LockSys") else { panic!("expected Output") };
    assert!(dot.contains("digraph \"LockSys\""), "got: {dot}");

    let cmds = repl.complete(":ex");
    assert!(cmds.iter().any(|c| c == ":explore"), "got: {cmds:?}");
}

#[test]
fn lsp_analysis_provides_machine_hover_and_goto_def_even_when_contract_fails() {
    let dir = std::env::temp_dir().join(format!("cinch_lsp_mc_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    // 1. Passing model: rich Markdown hover on `LockSys` and `bounds`, go-to-def from contract & actions.
    let mut analysis = Analysis::new(&dir);
    analysis.set_document("Main.cinch", SAMPLE_OK);
    let report = analysis.update();
    assert!(report.diagnostics.is_empty(), "diags: {:?}", report.diagnostics);

    let find_pos = |src: &str, needle_line: &str, ident: &str| -> (u32, u32) {
        let (idx, line) = src
            .lines()
            .enumerate()
            .find(|(_, l)| l.contains(needle_line))
            .unwrap_or_else(|| panic!("line `{needle_line}` not found"));
        let col = line
            .find(ident)
            .unwrap_or_else(|| panic!("ident `{ident}` not found in `{line}`"));
        ((idx + 1) as u32, (col + 1) as u32)
    };

    // Machine definition hover on `LockSys`
    let (mach_line, mach_col) = find_pos(SAMPLE_OK, "machine LockSys where", "LockSys");
    let hover_machine = analysis
        .hover("Main.cinch", mach_line, mach_col)
        .expect("hover on machine LockSys");
    assert!(hover_machine.contains("machine LockSys where"), "got: {hover_machine}");
    assert!(hover_machine.contains("Explored:"), "got: {hover_machine}");
    assert!(hover_machine.contains("✓ `bounds`"), "got: {hover_machine}");

    // Contract reference to `LockSys`: hover AND go-to-definition back to `machine LockSys`
    let (c_line, lock_col) = find_pos(SAMPLE_OK, "contract safety bounds for LockSys", "LockSys");
    let hover_ref = analysis
        .hover("Main.cinch", c_line, lock_col)
        .expect("hover on LockSys ref");
    assert!(hover_ref.contains("Explored:"), "got: {hover_ref}");
    let def_loc = analysis
        .definition("Main.cinch", c_line, lock_col)
        .expect("def of LockSys");
    assert_eq!(def_loc.1, mach_line, "should jump to machine LockSys definition line");

    // Hover on contract name `bounds`:
    let (_, bounds_col) = find_pos(SAMPLE_OK, "contract safety bounds for LockSys", "bounds");
    let hover_contract = analysis
        .hover("Main.cinch", c_line, bounds_col)
        .expect("hover on contract bounds");
    assert!(hover_contract.contains("✓ verified"), "got: {hover_contract}");

    // 2. Failing model: hover STILL reports explored states, deadlocks, and failing contracts!
    let mut bad_analysis = Analysis::new(&dir);
    bad_analysis.set_document("Main.cinch", SAMPLE_BUGGY);
    let bad_report = bad_analysis.update();
    assert!(!bad_report.diagnostics.is_empty(), "expected temporal violation diagnostic");
    let (stuck_line, stuck_col) = find_pos(SAMPLE_BUGGY, "machine Stuck where", "Stuck");
    let bad_hover = bad_analysis
        .hover("Main.cinch", stuck_line, stuck_col)
        .expect("hover on buggy machine Stuck");
    assert!(bad_hover.contains("machine Stuck where"), "got: {bad_hover}");
    assert!(bad_hover.contains("Deadlocks:"), "got: {bad_hover}");
    assert!(bad_hover.contains("✗ `capOne`"), "got: {bad_hover}");
    assert!(bad_hover.contains("✗ `noDeadlock`"), "got: {bad_hover}");
}

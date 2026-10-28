//! P21 acceptance: the REPL driven through its API (not a pty):
//! definitions and expressions with session state across inputs, the
//! session commands, `:load`/`:reload` of a multi-file project,
//! multi-line continuation, spanned errors, and completion over the
//! session's and the prelude's names.

use cinch::repl::{Repl, ReplOutcome};

fn out(r: ReplOutcome) -> String {
    match r {
        ReplOutcome::Output(s) => s,
        ReplOutcome::Incomplete => panic!("the input was unexpectedly incomplete"),
        ReplOutcome::Quit => panic!("the input unexpectedly quit"),
    }
}

#[test]
fn definition_and_expression_evaluation_persist_across_inputs() {
    let mut repl = Repl::with_root(std::env::temp_dir());
    // A signature waits for its defining equation.
    let noted = out(repl.eval("x :: Int"));
    assert!(noted.contains("noted x"), "{noted}");
    let defined = out(repl.eval("x = 5"));
    assert!(defined.contains("defined x"), "{defined}");
    assert!(defined.contains("Int"), "{defined}");
    // The session state persists: x is visible in later inputs.
    let sum = out(repl.eval("x + 2"));
    assert!(sum.contains("7"), "{sum}");
    assert!(sum.contains("Int"), "{sum}");
    // A second definition reads the first, and both persist.
    let d2 = out(repl.eval("y = x * 2"));
    assert!(d2.contains("defined y"), "{d2}");
    let both = out(repl.eval("y + x"));
    assert!(both.contains("15"), "{both}");
    // Definitions can also state their type in one block.
    let one = out(repl.eval("z :: Int\nz = 3"));
    assert!(one.contains("defined z"), "{one}");
    assert!(out(repl.eval("z * y")).contains("30"));
}

#[test]
fn type_kind_and_browse_outputs() {
    let mut repl = Repl::with_root(std::env::temp_dir());
    repl.eval("x :: Int");
    repl.eval("x = 7");
    // `:type` reports the inferred scheme of an expression.
    let t = out(repl.eval(":type x"));
    assert_eq!(t, "Int");
    let t2 = out(repl.eval(":type x + 1"));
    assert!(t2.contains("Int"), "{t2}");
    // `:kind` reports a type constructor's kind.
    let k = out(repl.eval(":kind List"));
    assert!(k.contains("List"), "{k}");
    assert!(k.contains("Type -> Type"), "{k}");
    // `:browse` lists the session's names with their schemes.
    let b = out(repl.eval(":browse"));
    assert!(b.contains("x :: Int"), "{b}");
}

#[test]
fn load_and_reload_a_multi_file_project() {
    let dir = std::env::temp_dir().join(format!("cinch-repl-load-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    std::fs::write(
        dir.join("Util.cinch"),
        "\nmodule Util where\n  double :: Int -> Int\n  double n = n + n\n",
    )
    .expect("Util.cinch");
    std::fs::write(
        dir.join("Main.cinch"),
        "\nmodule Main where\n  import Util\n  quad :: Int -> Int\n  quad n = double (double n)\n",
    )
    .expect("Main.cinch");
    let mut repl = Repl::with_root(std::env::temp_dir());
    let loaded = out(repl.eval(&format!(":load {}", dir.join("Main.cinch").display())));
    assert!(loaded.contains("loaded"), "{loaded}");
    assert!(loaded.contains("rechecked Util"), "{loaded}");
    assert!(loaded.contains("rechecked Main"), "{loaded}");
    // No absolute path ever reaches the output.
    if let Some(abs) = dir.to_str() {
        assert!(!loaded.contains(abs), "{loaded}");
    }
    // The project's names resolve inside the session.
    let e = out(repl.eval("quad 21"));
    assert!(e.contains("84"), "{e}");
    assert!(e.contains("Int"), "{e}");
    // `:reload` re-reads the sources and rechecks the project.
    let reloaded = out(repl.eval(":reload"));
    assert!(reloaded.contains("rechecked Util"), "{reloaded}");
    assert!(reloaded.contains("rechecked Main"), "{reloaded}");
    assert!(out(repl.eval("double 4")).contains("8"));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn multi_line_input_continues_until_the_block_closes() {
    let mut repl = Repl::with_root(std::env::temp_dir());
    // A trailing `then` waits for the rest of the expression.
    assert_eq!(repl.eval("y = if True then"), ReplOutcome::Incomplete);
    let d = out(repl.eval("  1 else 2"));
    assert!(d.contains("defined y"), "{d}");
    assert!(out(repl.eval("y")).contains("1"));
    // An open delimiter waits for its close.
    assert_eq!(repl.eval("w = (1 +"), ReplOutcome::Incomplete);
    let d = out(repl.eval("  2) * 3"));
    assert!(d.contains("defined w"), "{d}");
    assert!(out(repl.eval("w")).contains("9"));
    // A `where` block continues until the layout closes.
    assert_eq!(repl.eval("data Color where"), ReplOutcome::Incomplete);
    let d = out(repl.eval("  Red :: Color"));
    assert!(d.contains("defined Red"), "{d}");
}

#[test]
fn an_error_shows_its_source_span() {
    let mut repl = Repl::with_root(std::env::temp_dir());
    // A scope error in an expression: the span is in the input's own
    // coordinates, with the offending line and a caret.
    let e = out(repl.eval("zzz"));
    assert!(e.contains("error"), "{e}");
    assert!(e.contains("at 1:1"), "{e}");
    assert!(e.contains("zzz"), "{e}");
    // A type error in a definition maps back to the failing input.
    repl.eval("x :: Int");
    let t = out(repl.eval("x = True"));
    assert!(t.contains("error"), "{t}");
    assert!(t.contains("at 1:"), "{t}");
    // The session rolled the failure back and stays usable.
    assert!(out(repl.eval("x = 5")).contains("defined x"));
}

#[test]
fn completion_covers_session_and_prelude_names() {
    let mut repl = Repl::with_root(std::env::temp_dir());
    repl.eval("x :: Int");
    repl.eval("x = 5");
    repl.eval("xyzzy :: Int");
    repl.eval("xyzzy = 1");
    let c = repl.complete("x");
    assert!(c.contains(&"x".to_string()), "{c:?}");
    assert!(c.contains(&"xyzzy".to_string()), "{c:?}");
    // Prelude names complete too (the implicit base layer).
    let p = repl.complete("no");
    assert!(p.iter().any(|n| n == "not"), "{p:?}");
    let l = repl.complete("len");
    assert!(l.iter().any(|n| n == "length"), "{l:?}");
    // And the session commands complete after `:`.
    let cmds = repl.complete(":re");
    assert!(cmds.contains(&":reload".to_string()), "{cmds:?}");
}

#[test]
fn history_records_the_accepted_inputs() {
    let mut repl = Repl::with_root(std::env::temp_dir());
    repl.eval("x :: Int");
    repl.eval("x = 5");
    repl.eval(":type x");
    assert_eq!(repl.history(), &["x :: Int".to_string(), "x = 5".to_string(), ":type x".to_string()]);
}

//! Rank-N polymorphism (design/rank-n-polymorphism.md): nested `forall`
//! in argument, result and constructor-field positions, checked
//! predicatively with skolemisation at generalisation sites and
//! instantiation at uses, and compiled as anonymous instances of the
//! `#poly.<k>` dictionary families.
//!
//! Every accepted fixture under `fixtures/rank_n/` exits 42 and releases
//! every block exactly once: it runs under `CINCH_LEAK_CHECK=1` and
//! `CINCH_RC_DEBUG=1`, so a dictionary slot whose class is wrong (a counted
//! word read as raw, or the reverse) is a leak or a trap, not a silent
//! pass. Several fixtures use one polymorphic value at words of different
//! classes (`I64`, `Int`, `String`, `Bool`), which needs distinct slots of
//! the family's shared table. The rejected fixtures pin each refusal's
//! diagnostic.

mod common;
use std::process::Command;

fn run_leak_checked(tag: &str, source: &str) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let exe = common::build(&dir, "Main.cinch", "rank-n").unwrap_or_else(|e| panic!("{tag}: {e}"));
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

fn refused(tag: &str, source: &str, needle: &str) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let error = common::check(&dir, "Main.cinch").expect_err(tag);
    assert!(error.contains(needle), "{tag}: expected `{needle}` in:\n{error}");
    common::cleanup(&dir);
}

macro_rules! accepted {
    ($($name:ident),* $(,)?) => {$(
        #[test]
        fn $name() {
            run_leak_checked(
                concat!("rank-n-", stringify!($name)),
                include_str!(concat!("fixtures/rank_n/", stringify!($name), ".cinch")),
            );
        }
    )*};
}

accepted!(
    // A rank-2 argument used at two raw words.
    rank2,
    // One argument used at a raw word and a counted one: two slots.
    mixed,
    // A generalisation site that captures a raw and a counted local, and a
    // call that returns a polytype passed through without rebuilding.
    capture,
    // A polytype constructor field: Church numerals at Int and String.
    church,
    // A generalisation site inside a polymorphic definition, generic over
    // the enclosing parameter and its own.
    generic,
    // Rank 3: the argument itself takes a polymorphic argument.
    rank3,
    // A polymorphic argument forwarded unchanged to another rank-2 callee.
    pass_through,
    // A nested quantifier over an effect row.
    eff_row,
    // A function whose result is a polytype, applied past it.
    ret,
    // A recursive (never inlined) callee projecting a dictionary it cannot
    // see, and a polytype applied across a list.
    opaque,
    // A higher-kinded nested binder: a natural transformation.
    hkt,
    // Polytypes stored in a data structure and used at three classes.
    stored,
);

#[test]
fn escape_through_an_outer_metavariable_is_refused() {
    refused(
        "rank-n-escape",
        include_str!("fixtures/rank_n/bad_escape.cinch"),
        "the polymorphic type variable `a` would escape its `forall`",
    );
}

#[test]
fn impredicative_instantiation_is_refused() {
    refused(
        "rank-n-impredicative",
        include_str!("fixtures/rank_n/bad_impredicative.cinch"),
        "a type variable cannot stand for a polymorphic type",
    );
}

#[test]
fn nested_using_constraints_are_refused() {
    refused(
        "rank-n-using",
        include_str!("fixtures/rank_n/bad_using.cinch"),
        "a nested forall cannot carry `using` constraints yet",
    );
}

#[test]
fn quantifier_mismatch_is_refused() {
    refused(
        "rank-n-arity",
        include_str!("fixtures/rank_n/bad_arity.cinch"),
        "polymorphic types differ in their quantifiers: expected (forall a b. (a -> (b -> a))), got (forall a. (a -> a))",
    );
}

#[test]
fn a_monomorphic_function_is_not_polymorphic() {
    refused("rank-n-mono", include_str!("fixtures/rank_n/bad_mono.cinch"), "type mismatch: expected I64, got a");
}

#[test]
fn a_negative_occurrence_under_a_polytype_is_refused() {
    refused(
        "rank-n-positivity",
        include_str!("fixtures/rank_n/bad_positive.cinch"),
        "Main.Bad occurs negatively in field",
    );
}

#[test]
fn an_effectful_generalisation_body_is_refused() {
    // The dictionary of a polymorphic value is built once per reached class
    // vector, so building it twice must be unobservable: the body is pure.
    refused(
        "rank-n-impure",
        r#"module Main where
  run :: (forall a. a -> a) -> I64
  run f = f 1
  loud :: forall r. using (io :: Io r) => Unit ->{IO r} (I64 -> I64)
  loud u = do
    written <- ioWrite (string-to-bytes "built")
    pure (\x -> x)
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in int-from-i64 (run (let g = loud () in \x -> x))
"#,
        "a polymorphic value must be pure to build",
    );
}

#[test]
fn shadowing_an_outer_type_variable_is_refused() {
    refused(
        "rank-n-shadow",
        r#"module Main where
  shadow :: forall a. (forall a. a -> a) -> a -> a
  shadow f x = f x
  main :: Int
  main = 0
"#,
        "the nested forall binder `a` shadows a type variable already in scope",
    );
}

#[test]
fn a_rank_n_signature_renders_back_to_source() {
    // The interface text of a rank-N export is the source spelling, so a
    // separately compiled consumer parses back the same polytype.
    let dir = common::project(
        "rank-n-interface",
        &[(
            "Main.cinch",
            r#"module Main where
  export both
  both :: (forall a. a -> a) -> (I64, Bool) -> (I64, Bool)
  both f p = case p of (n, b) -> (f n, f b)
  main :: Int
  main = 0
"#,
        )],
    );
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check");
    let main = ifaces.iter().find(|i| i.module == "Main").expect("Main interface");
    let both = main.exports.values.iter().find(|v| v.name == "both").expect("both exported");
    assert_eq!(both.scheme, "((forall a. (a -> a)) -> ((I64, Bool) -> (I64, Bool)))");
    common::cleanup(&dir);
}

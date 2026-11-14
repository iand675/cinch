//! The inliner's copy (design/stack-allocation.md section 7, item 1).
//!
//! A callee body that binds a local, a `case` with its pattern binder, is
//! copied into the caller. The copy must be closed over the caller's
//! binders and must keep the callee's own binders distinct, and the
//! ownership verifier is the oracle: `crate::rc` re-derives every
//! reference and refuses a body on which a binder is read while dead.
//!
//! The programs here are Core, not source, so the shape under test is
//! exactly the shape the toolkit produces in `Data.Path.path-to-string`.

use cinch::core::{
    CAlt, CPat, Class, Core, FnDef, FnHints, FnMeta, GlobalId, InstanceDef, LocalId, Program,
    SimplifyReport, TargetProfile, UnitRepr,
};
use cinch::simplify::{simplify, SimplifyOptions};

fn gid(name: &str) -> GlobalId {
    GlobalId(name.to_string())
}

fn unit(name: &str, params: Vec<Class>, body: Core) -> FnDef {
    FnDef {
        id: gid(name),
        params: (0..params.len()).map(|i| format!("p{i}")).collect(),
        body,
        prim: None,
        ctor: None,
        hints: FnHints::default(),
        meta: FnMeta::default(),
        repr: UnitRepr { params, ..UnitRepr::default() },
    }
}

fn ctor_unit(name: &str, tag: usize, fields: usize) -> FnDef {
    let mut f = unit(name, vec![Class::Obj; fields], Core::Unit);
    f.ctor = Some((tag, fields));
    f
}

fn program(fns: Vec<FnDef>) -> Program {
    Program {
        total_bodies: cinch::total::CheckedGraph::empty(),
        fns,
        instances: Vec::<InstanceDef>::new(),
        main: None,
        rewrites: vec![],
        plans: Default::default(),
        foreigns: Default::default(),
        prims: vec![],
        laws: vec![],
        lent: Default::default(),
        frame: Default::default(),
        stack: Default::default(),
        borrowed: Default::default(),
        lent_entries: Default::default(),
        inline: Default::default(),
    }
}

/// `path-to-string p = case p of Path s -> s`: the body the toolkit copies.
fn case_bearing_callee() -> FnDef {
    unit(
        "Data.Path.path-to-string",
        vec![Class::Obj],
        Core::Case {
            scrut: Box::new(Core::Var(LocalId(0))),
            alts: vec![CAlt::new(
                CPat::Con { tag: 0, fields: vec![LocalId(1)], classes: vec![] },
                Core::Var(LocalId(1)),
            )],
        },
    )
}

fn call_callee(arg: Core) -> Core {
    Core::App(Box::new(Core::Global(gid("Data.Path.path-to-string"), vec![])), vec![arg])
}

/// The threshold model, deliberately. These tests pin the *copy* machinery
/// (fresh binder ids, simultaneous parameter substitution, a copy that the
/// ownership verifier accepts) on shapes the benefit model refuses by its
/// own rule: a decision-bearing body copied to more than one site where the
/// scrutinee does not fold. That refusal is the benefit model's correct
/// answer, and it is pinned in `tests/simplify.rs`; here the copy is what is
/// under test, so the model that admits it is the one selected.
fn options() -> SimplifyOptions {
    SimplifyOptions {
        max_iterations: 1,
        inline_model: cinch::simplify::InlineModel::Threshold,
        ..SimplifyOptions::default()
    }
}

fn run(mut p: Program, opts: &SimplifyOptions) -> (Program, SimplifyReport) {
    let mut report = SimplifyReport::default();
    simplify(&mut p, opts, &mut report);
    (p, report)
}

/// A body that binds a local is copied and the copy verifies: one call
/// site, a `case` with a pattern binder, a local argument.
#[test]
fn case_bearing_copy_at_one_site_verifies() {
    let p = program(vec![
        ctor_unit("Data.Path.Path", 0, 1),
        case_bearing_callee(),
        // f q = path-to-string q
        unit(
            "Main.f",
            vec![Class::Obj],
            call_callee(Core::Var(LocalId(0))),
        ),
    ]);
    let (p, report) = run(p, &options());
    assert_eq!(report.inlined, 1, "the case-bearing body is inlined at its one site");
    assert_eq!(report.inlined_units, ["Main.f"]);
    let p = cinch::rc::insert_closed(p, &TargetProfile::native())
        .unwrap_or_else(|e| panic!("the copy's ownership: {e}\n  body: {:?}", report.inlined_units));
    assert!(!p.fns.is_empty());
}

/// The same body copied at three sites within one unit, which is the shape
/// the toolkit reports: the second and third copies must not read a binder
/// the first copy made.
#[test]
fn case_bearing_copy_at_three_sites_verifies() {
    let p = program(vec![
        ctor_unit("Data.Path.Path", 0, 1),
        case_bearing_callee(),
        // three q r s = let a = path-to-string q in
        //               let b = path-to-string r in
        //               path-to-string s
        unit(
            "Main.three",
            vec![Class::Obj, Class::Obj, Class::Obj],
            Core::Let(
                LocalId(3),
                Class::Obj,
                Box::new(call_callee(Core::Var(LocalId(0)))),
                Box::new(Core::Let(
                    LocalId(4),
                    Class::Obj,
                    Box::new(call_callee(Core::Var(LocalId(1)))),
                    Box::new(call_callee(Core::Var(LocalId(2)))),
                )),
            ),
        ),
    ]);
    let (p, report) = run(p, &options());
    assert_eq!(report.inlined, 3, "three sites, one callee");
    assert_eq!(report.inlined_units, ["Main.three"]);
    let p = cinch::rc::insert_closed(p, &TargetProfile::native())
        .unwrap_or_else(|e| panic!("the copies' ownership: {e}"));
    assert!(!p.fns.is_empty());
}

/// The parameters are substituted simultaneously, through fresh ids, so an
/// argument that lands in the body cannot be mistaken for a parameter that
/// has not been substituted yet. `first b 7` in a two-parameter caller reads
/// the caller's second parameter, whose id is also the callee's second
/// parameter id: substituting parameter 1 after parameter 0 would rewrite
/// the argument parameter 0 placed there.
#[test]
fn substitution_of_two_parameters_is_simultaneous() {
    let p = program(vec![
        // first x y = x
        unit("Main.first", vec![Class::Obj, Class::Obj], Core::Var(LocalId(0))),
        // g a b = first b 7
        unit(
            "Main.g",
            vec![Class::Obj, Class::Obj],
            Core::App(
                Box::new(Core::Global(gid("Main.first"), vec![])),
                vec![Core::Var(LocalId(1)), Core::Int(7)],
            ),
        ),
    ]);
    let (p, report) = run(p, &options());
    assert_eq!(report.inlined, 1, "the binder-free body is inlined at its one site");
    let body = &p.fns[1].body;
    // The copy is `b`, the caller's second parameter: `Var(1)`, not the
    // constant `7` that substituting the second parameter used to leave.
    assert!(
        matches!(body, Core::Var(LocalId(1))),
        "the inlined body must read the caller's second parameter: {body:?}"
    );
}

/// A constructor wrapper, the shape the inliner already reaches, still
/// verifies: the regression guard for the binder-free rule.
#[test]
fn binder_free_copy_verifies() {
    let p = program(vec![
        ctor_unit("Data.Path.Path", 0, 1),
        unit(
            "Data.Path.wrap",
            vec![Class::Obj],
            Core::App(Box::new(Core::Ctor(gid("Data.Path.Path"), vec![])), vec![Core::Var(LocalId(0))]),
        ),
        unit("Main.g", vec![Class::Obj], Core::App(Box::new(Core::Global(gid("Data.Path.wrap"), vec![])), vec![Core::Var(LocalId(0))])),
    ]);
    let (p, report) = run(p, &options());
    assert_eq!(report.inlined, 1);
    cinch::rc::insert_closed(p, &TargetProfile::native()).expect("the wrapper's copy verifies");
}

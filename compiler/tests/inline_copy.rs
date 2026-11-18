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
    CAlt, CPat, Class, Core, FnDef, FnHints, FnMeta, GlobalId, LocalId, Program,
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
    Program::from_functions(fns)
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
/// ownership verifier accepts), independently of the benefit model's size
/// and fold policy. The benefit model also admits a single-arm field
/// projector now; its broader decision refusals remain pinned separately
/// in `tests/simplify.rs`.
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

/// Unknown owned products at several sites expose their field reads under
/// the default model, and the ordinary ownership pass balances every copy.
#[test]
fn benefit_field_projector_copies_at_unknown_sites_and_preserves_ownership() {
    let p = program(vec![
        ctor_unit("Data.Path.Path", 0, 1),
        case_bearing_callee(),
        unit(
            "Main.two",
            vec![Class::Obj, Class::Obj],
            Core::Let(
                LocalId(2),
                Class::Obj,
                Box::new(call_callee(Core::Var(LocalId(0)))),
                Box::new(call_callee(Core::Var(LocalId(1)))),
            ),
        ),
    ]);
    let opts = SimplifyOptions { max_iterations: 1, ..SimplifyOptions::default() };
    let (p, report) = run(p, &opts);
    assert_eq!(report.inlined, 2, "both unknown product arguments take the field read");
    assert_eq!(report.benefit.refused_fold, 0);
    let body = &p.fns[2].body;
    assert!(!format!("{body:?}").contains("path-to-string"), "a projector call survived: {body:?}");
    cinch::rc::insert_closed(p, &TargetProfile::native())
        .expect("both owned field reads retain their live payload and release unused owners");
}

/// Even an unused computed argument stays before the field read, in its
/// original left-to-right position; inlining does not turn a read into a
/// deferred computation or erase it.
#[test]
fn benefit_field_projector_keeps_computed_argument_order() {
    let mut projector = case_bearing_callee();
    projector.params.push("ignored".into());
    projector.repr.params.push(Class::Raw);
    projector.body = Core::Case {
        scrut: Box::new(Core::Var(LocalId(0))),
        alts: vec![CAlt::new(
            CPat::Con { tag: 0, fields: vec![LocalId(2)], classes: vec![] },
            Core::Var(LocalId(2)),
        )],
    };
    let call = || Core::App(
        Box::new(Core::Global(gid("Data.Path.path-to-string"), vec![])),
        vec![
            Core::BoxGet(Box::new(Core::Var(LocalId(0))), Class::Obj),
            Core::BoxGet(Box::new(Core::Var(LocalId(1))), Class::Raw),
        ],
    );
    let p = program(vec![
        ctor_unit("Data.Path.Path", 0, 1),
        projector,
        unit(
            "Main.reads",
            vec![Class::Obj, Class::Obj],
            Core::Let(LocalId(2), Class::Obj, Box::new(call()), Box::new(call())),
        ),
    ]);
    let opts = SimplifyOptions { max_iterations: 1, ..SimplifyOptions::default() };
    let (p, report) = run(p, &opts);
    assert_eq!(report.inlined, 2);
    let Core::Let(_, _, first, _) = &p.fns[2].body else { panic!("the first result stays at its original site") };
    let Core::Let(owner, Class::Obj, value, rest) = &**first else { panic!("bind the first computed argument: {first:?}") };
    assert!(matches!(&**value, Core::BoxGet(inner, Class::Obj) if matches!(&**inner, Core::Var(LocalId(0)))));
    let Core::Let(_, Class::Raw, value, rest) = &**rest else { panic!("bind even the unused second argument") };
    assert!(matches!(&**value, Core::BoxGet(inner, Class::Raw) if matches!(&**inner, Core::Var(LocalId(1)))));
    assert!(matches!(&**rest, Core::Case { scrut, .. } if matches!(&**scrut, Core::Var(local) if local == owner)));
    cinch::rc::insert_closed(p, &TargetProfile::native())
        .expect("strict cell observations and owned selected fields verify after the copies");
}

/// Keep a failed runtime qualification intact even when a driver thread panics.
struct ProjectorQualification(Option<tempfile::TempDir>);

impl ProjectorQualification {
    /// Borrow the project without transferring cleanup responsibility.
    fn path(&self) -> &std::path::Path {
        self.0.as_ref().unwrap().path()
    }

    /// Retain a completed failed command's exact inputs and generated artifacts.
    fn keep(mut self) -> std::path::PathBuf {
        self.0.take().unwrap().keep()
    }
}

impl Drop for ProjectorQualification {
    /// Preserve the project during any failed assertion or driver-thread unwind.
    fn drop(&mut self) {
        if std::thread::panicking() {
            if let Some(directory) = self.0.take() {
                eprintln!("retained failed projector qualification: {}", directory.keep().display());
            }
        }
    }
}

/// Actual byte owners and boxed naturals survive their product's lifetime,
/// and strict unused byte-write arguments remain observable at O0 and O2.
#[test]
fn benefit_field_projector_owned_payloads_execute_with_checked_rc() {
    for optimized in [false, true] {
        let directory = ProjectorQualification(Some(tempfile::tempdir().expect("owned projector project")));
        let entry = directory.path().join("Main.cinch");
        std::fs::write(&entry, include_str!("fixtures/inline-field-projector-owned.cinch"))
            .expect("owned projector source");
        let executable = directory.path().join("consumer");
        let output = executable.clone();
        let profile = TargetProfile { reclaim: true, rc_debug: true, ..TargetProfile::native() };
        let compiled = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || cinch::driver::driver_with_profile_opts(
                cinch::driver::Mode::Build,
                cinch::check::StageMode::Refresh,
                &entry,
                Some(&output),
                &profile,
                &cinch::driver::DriverOptions { opt_level: Some(if optimized { 2 } else { 0 }), ..Default::default() },
            ))
            .expect("driver thread")
            .join()
            .expect("owned projector driver did not panic");
        if let Err(error) = compiled {
            let retained = directory.keep();
            panic!("owned projector compile failed: {error:?}; retained {}", retained.display());
        }
        for poison in [false, true] {
            let result = std::process::Command::new(&executable)
                .env("CINCH_RC_DEBUG", if poison { "1" } else { "0" })
                .env("CINCH_HEAP_LIMIT", if poison { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .env("CINCH_LEAK_CHECK", "1")
                .output()
                .expect("owned projector process");
            if result.status.code() != Some(0)
                || !String::from_utf8_lossy(&result.stderr).contains("leaked=0")
                || !result.stdout.is_empty()
            {
                std::fs::write(directory.path().join(format!("run-{poison}.stdout")), &result.stdout).unwrap();
                std::fs::write(directory.path().join(format!("run-{poison}.stderr")), &result.stderr).unwrap();
                let retained = directory.keep();
                panic!("owned projector O{} poison={poison}: {:?}; retained {}",
                    if optimized { 2 } else { 0 }, result.status, retained.display());
            }
        }
    }
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

//! Physical constructor reuse must preserve complete wide fields and ownership.
//! These tests are a source-only proposal until the matching compiler is built.

use cinch::core::{CAlt, CPat, Core, CtorLayout, FnDef, FnHints, FnMeta, GlobalId,
    LocalId, PrimOp, Program, TargetProfile, UnitRepr};
use cinch::repr::Class;
use cinch::types::Scalar;

/// Construct a nominal constructor with its actual typed runtime field classes.
fn constructor(name: &str, classes: &[Class]) -> FnDef {
    FnDef {
        id: GlobalId(name.into()),
        params: (0..classes.len()).map(|i| format!("p{i}")).collect(),
        body: Core::Unit, prim: None, ctor: Some((0, classes.len())),
        hints: FnHints::default(), meta: FnMeta::default(),
        repr: UnitRepr { params: classes.to_vec(), fields: classes.to_vec(),
            result: Some(Class::Obj), data: Some(format!("{name}.Type")),
            ..UnitRepr::default() },
    }
}

/// Build a read-transform-rebuild worker without relying on optimizer inlining.
fn rebuild(classes: &[Class], old: Option<&str>, target: &str) -> FnDef {
    let fields: Vec<_> = (1..=classes.len()).map(LocalId).collect();
    let args = fields.iter().zip(classes).map(|(field, class)| {
        if *class == Class::Wide {
            Core::App(Box::new(Core::Prim(PrimOp::BitNot { ty: Scalar::U128 })),
                      vec![Core::Var(*field)])
        } else {
            Core::Var(*field)
        }
    }).collect();
    FnDef {
        id: GlobalId("Main.step".into()), params: vec!["state".into()],
        body: Core::Case {
            scrut: Box::new(Core::Var(LocalId(0))),
            alts: vec![CAlt::new(CPat::Con {
                tag: 0, fields, classes: classes.to_vec(),
                ctor: old.map(|name| GlobalId(name.into())),
            }, Core::App(Box::new(Core::Ctor(GlobalId(target.into()), vec![])), args))],
        },
        prim: None, ctor: None, hints: FnHints::default(), meta: FnMeta::default(),
        repr: UnitRepr { params: vec![Class::Obj], result: Some(Class::Obj),
                        ..UnitRepr::default() },
    }
}

/// Lower the worker and independently verify every ownership path.
fn lower(fns: Vec<FnDef>, boundary: bool) -> Program {
    let mut program = Program::from_functions(fns);
    if boundary {
        // Only the existence of a genuine distinct boundary layout is relevant
        // to this refusal; no emission is attempted for this synthetic layout.
        program.plans.insert("Main.State".into(), CtorLayout { payload: 16, fields: vec![] });
    }
    let counted = cinch::rc::insert_closed(program, &TargetProfile::native()).expect("RC insertion");
    cinch::rc::verify_program(&counted).expect("independent ownership verification");
    counted
}

/// Find reuse nodes including constructor calls nested in computed call heads.
fn reuses(body: &Core) -> usize {
    format!("{body:?}").matches("Reuse {").count()
}

/// A wide rebuild needs an owned parameter, a reset token and complete stores.
#[test]
fn exact_wide_constructor_reuses_its_physical_payload() {
    let classes = [Class::Wide];
    let counted = lower(vec![constructor("Main.State", &classes),
        rebuild(&classes, Some("Main.State"), "Main.State")], false);
    let worker = counted.fns.iter().find(|f| f.id.0 == "Main.step").unwrap();
    assert_eq!(reuses(&worker.body), 1, "{:?}", worker.body);
    assert!(!counted.borrowed.get("Main.step").is_some_and(|modes| modes[0]),
            "reuse cannot mutate a borrowed input");
    let ir = cinch::codegen::emit(&counted).expect("typed physical reuse IR");
    let body = ir.split("define ").find(|body| body.lines().next().is_some_and(|header|
        header.contains("@\"Main.step\"(") || header.contains("@\"Main.step#cc\"(")))
        .expect("worker IR");
    assert!(body.contains("@cinch_alloc_ctor(i64 16, i64 0)"), "{body}");
    assert!(body.matches("store i128 ").count() >= 2, "both hot/cold paths must store all128bits: {body}");
    assert!(body.contains("@cinch.rc.unique"), "{body}");
}

/// A tag and width alone cannot authorize overwriting an unknown nominal cell.
#[test]
fn wide_legacy_patterns_and_other_constructors_keep_fresh_allocation() {
    let classes = [Class::Wide];
    for old in [None, Some("Main.Other")] {
        let counted = lower(vec![constructor("Main.State", &classes),
            constructor("Main.Other", &classes), rebuild(&classes, old, "Main.State")], false);
        let worker = counted.fns.iter().find(|f| f.id.0 == "Main.step").unwrap();
        assert_eq!(reuses(&worker.body), 0, "{old:?}: {:?}", worker.body);
    }
}

/// Nominal identity does not override a custom physical layout or field width.
#[test]
fn boundary_layouts_and_different_wide_field_positions_refuse_reuse() {
    let classes = [Class::Wide, Class::Raw];
    let boundary = lower(vec![constructor("Main.State", &classes),
        rebuild(&classes, Some("Main.State"), "Main.State")], true);
    assert_eq!(reuses(&boundary.fns.last().unwrap().body), 0);
    let mismatch = lower(vec![constructor("Main.State", &[Class::Raw, Class::Wide]),
        rebuild(&classes, Some("Main.State"), "Main.State")], false);
    assert_eq!(reuses(&mismatch.fns.last().unwrap().body), 0);
}

/// A counted neighbor must remain owned while its raw wide neighbor is rebuilt.
#[test]
fn counted_neighbors_keep_their_release_and_descriptor_discipline() {
    for classes in [[Class::Wide, Class::Obj], [Class::Obj, Class::Wide]] {
        let counted = lower(vec![constructor("Main.State", &classes),
            rebuild(&classes, Some("Main.State"), "Main.State")], false);
        let worker = counted.fns.iter().find(|f| f.id.0 == "Main.step").unwrap();
        assert_eq!(reuses(&worker.body), 1, "{:?}", worker.body);
        let ir = cinch::codegen::emit(&counted).expect("mixed physical reuse IR");
        assert!(ir.contains("@cinch_alloc_ctor(i64 24,"), "{ir}");
    }
}

/// Legacy optimization bodies do not mint a locally checked layout identity.
#[test]
fn optimization_roundtrip_drops_untrusted_reuse_identity() {
    let classes = [Class::Wide];
    let mut worker = rebuild(&classes, Some("Main.State"), "Main.State");
    let encoded = cinch::ifaceopt::encode_core(&worker.body).expect("supported shallow body");
    worker.body = cinch::ifaceopt::decode_core(&encoded).expect("legacy pattern decoding");
    let Core::Case { alts, .. } = &worker.body else { panic!("case") };
    assert!(matches!(&alts[0].pat, CPat::Con { ctor: None, .. }));
    let counted = lower(vec![constructor("Main.State", &classes), worker], false);
    assert_eq!(reuses(&counted.fns.last().unwrap().body), 0);
}

/// Actual checked source must preserve the defining pattern identity to RC.
#[test]
fn source_nominal_patterns_authorize_wide_reuse() {
    let source = r#"module Main where
  import Data.Wide
  -- | An immutable wide state, retained independently by aliases.
  data State where
    State :: U128 -> State
  -- | Rebuild a state without consuming any externally retained snapshot.
  next :: State -> State
  next state = case state of
    State word -> State (bit-not-u128 word)
  -- | Keep the worker in the checked source without staging a literal loop.
  main :: Int
  main = 0
"#;
    let profile = TargetProfile::native();
    let checked = cinch::driver::check_sources_profile(source, "wide-reuse.cinch", &profile)
        .expect("real source checking");
    let counted = cinch::rc::insert_closed(checked, &profile).expect("RC insertion");
    let worker = counted.fns.iter().find(|f| f.id.0.starts_with("Main.next")).expect("worker");
    assert!(reuses(&worker.body) > 0, "{:?}", worker.body);
}

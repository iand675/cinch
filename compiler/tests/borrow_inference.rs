//! Borrow inference must preserve aliases without creating an owned
//! reference. An alias that escapes or is written through keeps its source
//! parameter owned, including for the stack allocation pass.
use cinch::core::{CAlt, CPat, Core, FnDef, FnHints, FnMeta, GlobalId, LocalId, Program, TargetProfile, UnitRepr};
use cinch::repr::Class;

fn unit(name: &str, params: Vec<Class>, body: Core) -> FnDef {
    FnDef {
        id: GlobalId(name.into()), params: (0..params.len()).map(|i| format!("p{i}")).collect(),
        body, prim: None, ctor: None, hints: FnHints::default(), meta: FnMeta::default(),
        repr: UnitRepr { params, ..UnitRepr::default() },
    }
}

fn program(fns: Vec<FnDef>) -> Program {
    Program {
        total_bodies: cinch::total::CheckedGraph::empty(), fns, instances: vec![],
        main: None, rewrites: vec![], plans: Default::default(), foreigns: Default::default(),
        prims: vec![], laws: vec![], lent: Default::default(), frame: Default::default(),
        stack: Default::default(), borrowed: Default::default(), lent_entries: Default::default(),
        private_modules: vec![], inline: Default::default(),
    }
}

fn var(i: usize) -> Core { Core::Var(LocalId(i)) }
fn call(name: &str, arg: Core) -> Core {
    Core::App(Box::new(Core::Global(GlobalId(name.into()), vec![])), vec![arg])
}
fn alias_case(body: Core) -> Core {
    Core::Case { scrut: Box::new(var(0)), alts: vec![CAlt {
        pat: CPat::Var(LocalId(1), Class::Obj), weight: None, body: Box::new(body),
    }] }
}
fn reader() -> FnDef {
    unit("Main.read", vec![Class::Obj], Core::Case {
        scrut: Box::new(var(0)), alts: vec![CAlt {
            pat: CPat::Con { tag: 0, fields: vec![], classes: vec![] },
            weight: None, body: Box::new(Core::Bool(true)),
        }],
    })
}
fn counted(fns: Vec<FnDef>) -> Program {
    let p = cinch::rc::insert_closed(program(fns), &TargetProfile::native()).expect("counting verifies");
    cinch::rc::verify_program(&p).expect("the result verifies independently");
    p
}
fn operations(body: &Core) -> usize {
    // Debug output contains exactly one enum tag per ownership operation.
    let text = format!("{body:?}");
    ["Dup(", "Drop(", "Release {"].iter().map(|op| text.matches(op).count()).sum()
}

#[test]
fn a_variable_pattern_lends_its_borrowed_scrutinee_to_a_tail_call() {
    let p = counted(vec![reader(), unit("Main.relay", vec![Class::Obj], alias_case(call("Main.read", var(1))))]);
    assert_eq!(p.borrowed.get("Main.read"), Some(&vec![true]));
    assert_eq!(p.borrowed.get("Main.relay"), Some(&vec![true]));
    assert_eq!(p.fns.iter().map(|f| operations(&f.body)).sum::<usize>(), 0);
}

#[test]
fn nested_variable_patterns_keep_a_recursive_borrow_in_tail_position() {
    let nested = Core::Case { scrut: Box::new(var(1)), alts: vec![CAlt {
        pat: CPat::Var(LocalId(2), Class::Obj), weight: None,
        body: Box::new(call("Main.loop", var(2))),
    }] };
    let p = counted(vec![unit("Main.loop", vec![Class::Obj], alias_case(nested))]);
    assert_eq!(p.borrowed.get("Main.loop"), Some(&vec![true]));
    assert_eq!(operations(&p.fns[0].body), 0, "no cleanup may follow the recursive call");
}

#[test]
fn returning_a_pattern_alias_keeps_its_parameter_owned() {
    let p = counted(vec![unit("Main.escape", vec![Class::Obj], alias_case(var(1)))]);
    assert!(!p.borrowed.contains_key("Main.escape"));
    assert_eq!(operations(&p.fns[0].body), 0, "the owned reference transfers to the result");
}

#[test]
fn writes_through_a_pattern_alias_keep_its_parameter_owned() {
    let body = Core::Let(LocalId(2), Class::Raw,
        Box::new(Core::RefSet(LocalId(1), Box::new(Core::BoxAlloc), Class::Obj)),
        Box::new(call("Main.read", var(1))));
    let p = counted(vec![unit("Main.read", vec![Class::Obj], call("Main.read", var(0))), unit("Main.write", vec![Class::Obj], alias_case(body))]);
    assert!(!p.borrowed.contains_key("Main.read"), "the overwritten owned binder must be consumed on the tail edge");
    assert!(!p.borrowed.contains_key("Main.write"));
    assert_eq!(operations(&p.fns[1].body), 0, "the owned reference transfers through the alias");
}

mod common;

#[test]
fn pattern_borrows_and_escaping_aliases_run_without_leaks() {
    const SOURCE: &str = r#"
module Main where
  import Data.List
  data Box where
    Box :: List Int -> Box

  #[inline(never)]
  build :: Int -> List Int -> List Int
  build n xs = if n == 0 then xs else build (n - 1) (Cons n xs)

  #[inline(never)]
  count :: Int -> List Int -> Int
  count n xs = case xs of
    same -> case same of
      Nil -> n
      Cons h rest -> count (n + 1) rest

  #[inline(never)]
  keep :: Box -> Box
  keep box = case box of same -> same

  main :: Int
  main =
    let xs = build 1000 Nil in
    let saved = keep (Box (build 257 Nil)) in
    case saved of
      Box ys -> if count 0 xs == 1000 then (if count 0 ys == 257 then 0 else 2) else 1
"#;
    let dir = common::project("borrow_patterns", &[("Main.cinch", SOURCE)]);
    for debug in [false, true] {
        let exe = dir.join(if debug { "debug" } else { "plain" });
        let profile = TargetProfile { reclaim: true, rc_debug: debug, ..TargetProfile::native() };
        common::drive_with_profile(cinch::driver::Mode::Build, cinch::check::StageMode::Refresh,
            &dir, "Main.cinch", Some(exe.clone()), &profile).expect("compile alias program in process");
        let output = std::process::Command::new(exe)
            .env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_HEAP_LIMIT", "64M").output().expect("run alias program");
        let err = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(0), "debug {debug}: {err}");
        assert!(err.contains("cinch-leak: live=0 immortal=0 leaked=0"), "debug {debug}: {err}");
    }
    common::cleanup(&dir);
}

#[test]
fn owned_pattern_aliases_still_transfer_the_scrutinees_reference() {
    let p = cinch::rc::insert(program(vec![unit("Main.take", vec![Class::Obj], alias_case(var(1)))]),
        &TargetProfile::native()).expect("all-owned convention verifies");
    cinch::rc::verify_program(&p).expect("the owned convention remains balanced");
    assert!(p.borrowed.is_empty());
    assert_eq!(operations(&p.fns[0].body), 0);
}

#[test]
fn dumps_report_inference_separately_from_the_primitive_convention() {
    let p = counted(vec![reader()]);
    assert!(p.lent.is_empty());
    let text = cinch::coretext::print(&p, cinch::coretext::Stage::Owned, &TargetProfile::native(), "test");
    assert!(text.contains("@params p0 :: - @obj"), "{text}");
    assert!(text.contains("@borrow-modes(1)"), "{text}");
    assert!(!text.contains("@borrowed("), "{text}");
}

#[test]
fn returning_an_alias_does_not_let_the_caller_return_a_dead_frame_block() {
    let mut ctor = unit("Main.Box", vec![Class::Raw], Core::Unit);
    ctor.ctor = Some((0, 1));
    ctor.repr.fields = vec![Class::Raw];
    ctor.repr.data = Some("Main.Box".into());
    let make = unit("Main.make", vec![], Core::Let(LocalId(0), Class::Obj,
        Box::new(Core::App(Box::new(Core::Ctor(GlobalId("Main.Box".into()), vec![])), vec![Core::Int(1)])),
        Box::new(Core::Let(LocalId(1), Class::Obj,
            Box::new(call("Main.escape", var(0))), Box::new(var(1))))));
    let mut p = counted(vec![ctor, unit("Main.escape", vec![Class::Obj], alias_case(var(1))), make]);
    cinch::stack::allocate(&mut p, &TargetProfile::native(), &cinch::stack::Options::default());
    assert!(p.frame.get("Main.make").is_none_or(|plan| plan.is_empty()),
        "the returned block must outlive Main.make's frame: {:?}", p.frame);
}

#[test]
fn nested_aliases_that_are_stored_or_captured_keep_the_parameter_owned() {
    let store = Core::App(Box::new(Core::Ctor(GlobalId("Main.Hold".into()), vec![])), vec![var(2)]);
    let capture = Core::Lam { fn_id: GlobalId("Main.worker".into()), env: vec![var(2)], classes: vec![] };
    for body in [store, capture] {
        let body = alias_case(Core::Let(LocalId(2), Class::Obj, Box::new(var(1)), Box::new(body)));
        let p = counted(vec![unit("Main.retain", vec![Class::Obj], body)]);
        assert!(!p.borrowed.contains_key("Main.retain"), "an owned use of a nested alias retains the parameter");
        assert_eq!(operations(&p.fns[0].body), 0);
    }
}

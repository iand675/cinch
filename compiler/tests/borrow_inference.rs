//! Borrow inference must preserve aliases without creating an owned
//! reference. An alias that escapes or is written through keeps its source
//! parameter owned, including for the stack allocation pass.
use cinch::core::{CAlt, CBranch, CPat, Core, FnDef, FnHints, FnMeta, GlobalId, LocalId, Program, TargetProfile, UnitRepr};
use cinch::repr::Class;

fn unit(name: &str, params: Vec<Class>, body: Core) -> FnDef {
    FnDef {
        id: GlobalId(name.into()), params: (0..params.len()).map(|i| format!("p{i}")).collect(),
        body, prim: None, ctor: None, hints: FnHints::default(), meta: FnMeta::default(),
        repr: UnitRepr { params, ..UnitRepr::default() },
    }
}

/// Build test-owned Core without granting checked builtin-constructor authority.
fn program(fns: Vec<FnDef>) -> Program {
    Program::from_functions(fns)
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

fn imported_reader_program(arg: Core, computed_head: bool) -> Program {
    let mut inspect = unit("Dep.inspect", vec![Class::Obj],
        Core::Global(GlobalId("Dep.function".into()), vec![]));
    inspect.repr.result = Some(Class::Obj);
    let read = call("Dep.inspect", arg);
    let body = if computed_head { Core::App(Box::new(read), vec![Core::Unit]) } else { read };
    let mut thunk = unit("Main.thunk", vec![], Core::ConstBytes(vec![4, 5, 6]));
    thunk.repr.result = Some(Class::Obj);
    program(vec![inspect, unit("Main.caller", vec![], body), thunk,
        unit("Dep.function", vec![Class::Raw], Core::Unit)])
}

fn imported_reader(arg: Core, computed_head: bool) -> Program {
    let p = imported_reader_program(arg, computed_head);
    let imported = [("Dep.inspect".into(), (vec![true], 1))].into_iter().collect();
    cinch::rc::insert_open(p, &TargetProfile::native(), &cinch::rc::Boundary {
        modules: &["Main".into()], imported: &imported, closed: false,
    }).expect("imported borrowing convention verifies")
}

#[test]
fn an_imported_borrow_in_a_computed_call_head_releases_its_temporary() {
    let p = imported_reader(Core::ConstBytes(vec![1, 2, 3]), true);
    assert_eq!(operations(&p.fns[1].body), 1, "a borrowed buffer needs one caller drop: {:?}", p.fns[1].body);
}

#[test]
fn an_imported_borrow_of_a_global_thunk_releases_its_result() {
    let p = imported_reader(Core::Global(GlobalId("Main.thunk".into()), vec![]), false);
    assert_eq!(operations(&p.fns[1].body), 1, "a thunk result is an owned temporary: {:?}", p.fns[1].body);
}

#[test]
fn an_imported_borrow_of_a_static_function_needs_no_cleanup() {
    let p = imported_reader(Core::Global(GlobalId("Dep.function".into()), vec![]), false);
    assert_eq!(operations(&p.fns[1].body), 0, "a static closure owns no temporary reference");
}

#[test]
fn the_verifier_rejects_anonymous_owned_arguments_to_borrowers() {
    for computed_head in [false, true] {
        for arg in [Core::ConstBytes(vec![1, 2, 3]),
            Core::Global(GlobalId("Main.thunk".into()), vec![]),
            Core::Lam { fn_id: GlobalId("Dep.function".into()), env: vec![], classes: vec![] }] {
            // Recreate the pre-fix Core directly, bypassing argument naming.
            let mut p = imported_reader_program(arg, computed_head);
            p.borrowed.insert("Dep.inspect".into(), vec![true]);
            let error = cinch::rc::verify_program(&p).expect_err("an anonymous owned loan leaks");
            assert!(error.to_string().contains("unbound owned argument"), "{error}");
        }
    }
}

#[test]
fn the_verifier_accepts_static_and_immediate_borrow_arguments() {
    for arg in [Core::Global(GlobalId("Dep.function".into()), vec![]), Core::Int(7), Core::Unit] {
        let mut p = imported_reader_program(arg, false);
        p.borrowed.insert("Dep.inspect".into(), vec![true]);
        cinch::rc::verify_program(&p).expect("a static or immediate word needs no cleanup");
    }
}

#[test]
fn imported_borrowed_temporaries_run_across_objects_without_leaks() {
    use std::process::Command;
    let dir = common::project("borrow_imported", &[]);
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime");
    let built = Command::new("cargo").args(["build", "--release", "--manifest-path"])
        .arg(runtime.join("Cargo.toml")).arg("--target-dir").arg(runtime.join("target"))
        .output().expect("build runtime");
    assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
    let profile = TargetProfile::native();
    for (name, arg) in [("buffer", Core::ConstBytes(vec![1, 2, 3])),
        ("thunk", Core::Global(GlobalId("Main.thunk".into()), vec![])),
        ("closure", Core::Global(GlobalId("Dep.function".into()), vec![]))] {
        let mut source = imported_reader_program(arg, true);
        let provider = cinch::rc::insert_open(source.clone(), &profile, &cinch::rc::Boundary {
            modules: &["Dep".into()], imported: &Default::default(), closed: false,
        }).expect("count provider");
        assert_eq!(provider.borrowed.get("Dep.inspect"), Some(&vec![true]));
        source.main = Some(GlobalId("Main.caller".into()));
        let imported = [("Dep.inspect".into(), (vec![true], 1))].into_iter().collect();
        let consumer = cinch::rc::insert_open(source, &profile, &cinch::rc::Boundary {
            modules: &["Main".into()], imported: &imported, closed: false,
        }).expect("count consumer");
        let mut objects = Vec::new();
        for (module, p) in [("Dep", provider), ("Main", consumer)] {
            let ll = dir.join(format!("{name}-{module}.ll"));
            let object = ll.with_extension("o");
            let ir = if module == "Dep" {
                cinch::codegen::emit_object_for_modules(&profile, &p, &[module.into()])
                    .expect("emit library object").ir
            } else {
                cinch::codegen::emit_for_modules(&profile, &p, &[module.into()])
                    .expect("emit executable module")
            };
            std::fs::write(&ll, ir).expect("write IR");
            let cc = Command::new(std::env::var_os("CINCH_CLANG").unwrap_or_else(|| "clang".into()))
                .arg("-c").arg(&ll).arg("-o").arg(&object).output().expect("compile object");
            assert!(cc.status.success(), "{}", String::from_utf8_lossy(&cc.stderr));
            objects.push(object);
        }
        let exe = dir.join(name);
        let mut linker = Command::new(std::env::var_os("CINCH_CLANG").unwrap_or_else(|| "clang".into()));
        linker.args(&objects).arg(runtime.join("target/release/libcinchruntime.a")).arg("-o").arg(&exe);
        if cfg!(target_os = "linux") { linker.args(["-ldl", "-lpthread", "-lm"]); }
        let linked = linker.output().expect("link objects");
        assert!(linked.status.success(), "{}", String::from_utf8_lossy(&linked.stderr));
        let out = Command::new(exe).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1")
            .output().expect("run imported borrow");
        let err = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(0), "{name}: {err}");
        assert!(err.contains("cinch-leak: live=0 immortal=0 leaked=0"), "{name}: {err}");
    }
    common::cleanup(&dir);
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
fn mutual_recursion_propagates_owned_and_borrowed_modes_independently() {
    let relay = |name: &str| Core::App(Box::new(Core::Global(GlobalId(name.into()), vec![])), vec![var(0), var(1)]);
    let right = Core::If(Box::new(Core::Bool(true)),
        CBranch { weight: None, body: Box::new(relay("Main.left")) },
        CBranch { weight: None, body: Box::new(Core::Let(LocalId(2), Class::Raw,
            Box::new(call("Main.read", var(0))), Box::new(var(1)))) });
    let p = counted(vec![reader(), unit("Main.left", vec![Class::Obj, Class::Obj], relay("Main.right")),
        unit("Main.right", vec![Class::Obj, Class::Obj], right)]);
    for name in ["Main.left", "Main.right"] {
        assert_eq!(p.borrowed.get(name), Some(&vec![true, false]));
    }
    assert_eq!(p.fns.iter().map(|f| operations(&f.body)).sum::<usize>(), 0);
}

#[test]
fn reuse_facts_survive_revisits_but_do_not_escape_the_inference_pass() {
    // Reusing the first parameter withdraws its loan and revisits the
    // function with its second parameter still borrowed. Recompile the
    // same unit names with different bodies to rule out cross-pass facts.
    for reuses in [false, true, false] {
        let mut ctor = unit("Main.Box", vec![Class::Raw], Core::Unit);
        ctor.ctor = Some((0, 1));
        ctor.repr.fields = vec![Class::Raw];
        ctor.repr.data = Some("Main.Box".into());
        let result = if reuses {
            Core::App(Box::new(Core::Ctor(GlobalId("Main.Box".into()), vec![])), vec![var(2)])
        } else { Core::Int(0) };
        let body = Core::Case { scrut: Box::new(var(0)), alts: vec![CAlt {
            pat: CPat::Con { tag: 0, fields: vec![LocalId(2)], classes: vec![Class::Raw] },
            weight: None, body: Box::new(Core::Let(LocalId(3), Class::Raw,
                Box::new(call("Main.read", var(1))), Box::new(result))),
        }] };
        let p = counted(vec![ctor, reader(), unit("Main.rebuild", vec![Class::Obj, Class::Obj], body)]);
        assert_eq!(p.borrowed.get("Main.rebuild"), Some(&vec![!reuses, true]));
        if reuses {
            assert!(format!("{:?}", p.fns[2].body).contains("Reuse {"), "the owned scrutinee still reuses its cell");
        }
    }
}

#[test]
fn computed_heads_record_writes_before_alias_elimination() {
    for write in [Core::RefSet(LocalId(1), Box::new(Core::Int(0)), Class::Raw),
        Core::BoxInit(LocalId(1), Box::new(Core::Int(0)), Class::Raw),
        Core::LazyInit(LocalId(1), Box::new(Core::Global(GlobalId("Main.worker".into()), vec![])))] {
        let head = Core::Let(LocalId(2), Class::Raw, Box::new(write),
            Box::new(Core::Global(GlobalId("Main.worker".into()), vec![])));
        let body = Core::Let(LocalId(1), Class::Obj, Box::new(var(0)),
            Box::new(Core::App(Box::new(head), vec![var(0)])));
        let p = counted(vec![unit("Main.worker", vec![Class::Obj], var(0)),
            unit("Main.write", vec![Class::Obj], body)]);
        assert!(matches!(p.fns[1].body, Core::Let(LocalId(1), Class::Obj, _, _)),
            "a written alias must keep its own binding: {:?}", p.fns[1].body);
    }
}

#[test]
fn computed_heads_reuse_cells_inside_and_outside_the_match() {
    use std::process::Command;
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime");
    let built = Command::new("cargo").args(["build", "--release", "--manifest-path"])
        .arg(runtime.join("Cargo.toml")).arg("--target-dir").arg(runtime.join("target"))
        .output().expect("build runtime");
    assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
    for match_in_head in [false, true] {
        let mut ctor = unit("Main.Box", vec![Class::Raw], Core::Unit);
        ctor.ctor = Some((0, 1));
        ctor.repr.fields = vec![Class::Raw];
        ctor.repr.data = Some("Main.Box".into());
        let head = Core::Let(LocalId(2), Class::Obj,
            Box::new(Core::App(Box::new(Core::Ctor(GlobalId("Main.Box".into()), vec![])), vec![var(1)])),
            Box::new(Core::Let(LocalId(3), Class::Raw, Box::new(call("Main.read", var(2))),
                Box::new(Core::Global(GlobalId("Main.worker".into()), vec![])))));
        let apply = |h| Core::App(Box::new(h), vec![Core::Unit]);
        let matched = Core::Case { scrut: Box::new(var(0)), alts: vec![CAlt {
            pat: CPat::Con { tag: 0, fields: vec![LocalId(1)], classes: vec![Class::Raw] },
            weight: None, body: Box::new(if match_in_head { head.clone() } else { apply(head.clone()) }),
        }] };
        let body = if match_in_head { apply(matched) } else { matched };
        let entry = call("Main.rebuild", Core::App(
            Box::new(Core::Ctor(GlobalId("Main.Box".into()), vec![])), vec![Core::Int(5)]));
        let mut p = counted(vec![ctor, reader(), unit("Main.worker", vec![Class::Raw], Core::Unit),
            unit("Main.rebuild", vec![Class::Obj], body), unit("Main.main", vec![], entry)]);
        assert!(!p.borrowed.contains_key("Main.rebuild"), "a reused parameter must be owned");
        assert!(format!("{:?}", p.fns[3].body).contains("Reuse {"), "{:?}", p.fns[3].body);
        p.main = Some(GlobalId("Main.main".into()));
        let dir = common::project("borrow_computed_head", &[]);
        let ll = dir.join("program.ll");
        let exe = dir.join("program");
        std::fs::write(&ll, cinch::codegen::emit(&p).expect("emit the exact counted head shape")).expect("write IR");
        let mut linker = Command::new(std::env::var_os("CINCH_CLANG").unwrap_or_else(|| "clang".into()));
        linker.arg(&ll).arg(runtime.join("target/release/libcinchruntime.a")).arg("-o").arg(&exe);
        if cfg!(target_os = "linux") { linker.args(["-ldl", "-lpthread", "-lm"]); }
        let linked = linker.output().expect("link computed-head reuse");
        assert!(linked.status.success(), "{}", String::from_utf8_lossy(&linked.stderr));
        let out = Command::new(exe).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1")
            .output().expect("run computed-head reuse");
        let err = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(0), "match_in_head={match_in_head}: {err}");
        assert!(err.contains("cinch-leak: live=0 immortal=0 leaked=0"), "{err}");
        common::cleanup(&dir);
    }
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
    assert_eq!(text.matches("@borrow-modes(").count(), 1, "the printer must not repeat a mode annotation");
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

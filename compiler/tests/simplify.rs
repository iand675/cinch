//! The Core simplifier's rules (design/compilation-speed.md section 10):
//! case-merge, known-constructor reduction, direct static dictionary
//! calls, the tick budget, and the refusal conditions each rule states.
//!
//! The programs here are built as Core directly, so a rule's precondition
//! is exact rather than whatever the frontend happens to produce.

use cinch::core::{
    CAlt, CBranch, CPat, Class, Core, FnDef, FnHints, FnMeta, GlobalId, InstanceDef, LocalId, Program,
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
        instances: vec![],
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
            inline: Default::default(),
    }
}

fn con(pattern_tag: usize, fields: &[usize]) -> CPat {
    CPat::Con { tag: pattern_tag, fields: fields.iter().map(|i| LocalId(*i)).collect(), classes: vec![] }
}

/// `case x of Red -> 1; _ -> case x of Blue -> 2; Green -> 3` becomes one
/// decision with the three constructors, and the default alternative goes.
#[test]
fn case_merge_folds_the_shared_default_decision() {
    let outer_alts = vec![
        CAlt::new(con(0, &[]), Core::Int(1)),
        CAlt::new(con(1, &[]), Core::Case { scrut: Box::new(Core::Var(LocalId(0))), alts: vec![
            CAlt::new(con(1, &[]), Core::Int(2)),
            CAlt::new(con(2, &[]), Core::Int(3)),
        ] }),
    ];
    let body = Core::Case { scrut: Box::new(Core::Var(LocalId(0))), alts: outer_alts };
    let mut p = program(vec![
        ctor_unit("Red", 0, 0),
        ctor_unit("Blue", 1, 0),
        ctor_unit("Green", 2, 0),
        unit("Main.f", vec![Class::Obj], body),
    ]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    let Core::Case { alts, .. } = &p.fns[3].body else { panic!("the decision stays") };
    let tags: Vec<usize> =
        alts.iter().map(|a| match &a.pat { CPat::Con { tag, .. } => *tag, _ => panic!("constructor") }).collect();
    assert_eq!(tags, [0, 1, 2]);
    let bodies: Vec<String> =
        alts.iter().map(|a| match &*a.body { Core::Int(n) => n.to_string(), _ => "other".into() }).collect();
    assert_eq!(bodies, ["1", "2", "3"]);
    assert_eq!(report.case_merge, 1);
    assert_eq!(report.ticks, 1);
}

/// The inner decision's alternative for a tag the outer decision already
/// names is dead under the default and is dropped, not duplicated.
#[test]
fn case_merge_drops_the_defaults_alternative_for_a_covered_tag() {
    let outer_alts = vec![
        CAlt::new(con(0, &[]), Core::Int(1)),
        CAlt::new(con(1, &[]), Core::Case { scrut: Box::new(Core::Var(LocalId(0))), alts: vec![
            CAlt::new(con(0, &[]), Core::Int(9)),
            CAlt::new(con(2, &[]), Core::Int(3)),
        ] }),
    ];
    let body = Core::Case { scrut: Box::new(Core::Var(LocalId(0))), alts: outer_alts };
    let mut p = program(vec![
        ctor_unit("Red", 0, 0),
        ctor_unit("Blue", 1, 0),
        ctor_unit("Green", 2, 0),
        unit("Main.f", vec![Class::Obj], body),
    ]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    let Core::Case { alts, .. } = &p.fns[3].body else { panic!("the decision stays") };
    let tags: Vec<usize> =
        alts.iter().map(|a| match &a.pat { CPat::Con { tag, .. } => *tag, _ => panic!("constructor") }).collect();
    assert_eq!(tags, [0, 2]);
    assert_eq!(report.case_merge, 1);
}

/// The shape the match compiler produces for a source wildcard: one
/// explicit alternative, then a wildcard whose body decides the same local
/// again. The wildcard and the inner alternatives' dead branches go.
#[test]
fn case_merge_folds_a_wildcard_default() {
    let body = Core::Case {
        scrut: Box::new(Core::Var(LocalId(0))),
        alts: vec![
            CAlt::new(con(0, &[]), Core::Int(1)),
            CAlt::new(CPat::Wild, Core::Case {
                scrut: Box::new(Core::Var(LocalId(0))),
                alts: vec![
                    CAlt::new(con(0, &[]), Core::Int(2)),
                    CAlt::new(con(1, &[]), Core::Int(4)),
                    CAlt::new(con(2, &[]), Core::Int(3)),
                ],
            }),
        ],
    };
    let mut p = program(vec![
        ctor_unit("Red", 0, 0),
        ctor_unit("Blue", 1, 0),
        ctor_unit("Green", 2, 0),
        unit("Main.f", vec![Class::Obj], body),
    ]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    let Core::Case { alts, .. } = &p.fns[3].body else { panic!("the decision stays") };
    let rows: Vec<(usize, i64)> = alts
        .iter()
        .map(|a| match (&a.pat, &*a.body) {
            (CPat::Con { tag, .. }, Core::Int(n)) => (*tag, *n),
            (pat, body) => panic!("unexpected alternative {pat:?} {body:?}"),
        })
        .collect();
    assert_eq!(rows, [(0, 1), (1, 4), (2, 3)]);
    assert_eq!(report.case_merge, 1);
}

/// A variable default binds the scrutinee under a name its body may use;
/// the merged alternatives name the outer local instead.
#[test]
fn case_merge_substitutes_a_variable_defaults_binder() {
    let body = Core::Case {
        scrut: Box::new(Core::Var(LocalId(0))),
        alts: vec![
            CAlt::new(con(0, &[]), Core::Int(1)),
            CAlt::new(CPat::Var(LocalId(1), Class::Obj), Core::Case {
                scrut: Box::new(Core::Var(LocalId(1))),
                alts: vec![
                    CAlt::new(con(1, &[]), Core::Var(LocalId(1))),
                    CAlt::new(con(2, &[]), Core::Int(3)),
                ],
            }),
        ],
    };
    let mut p = program(vec![
        ctor_unit("Red", 0, 0),
        ctor_unit("Blue", 1, 0),
        ctor_unit("Green", 2, 0),
        unit("Main.f", vec![Class::Obj], body),
    ]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    let Core::Case { alts, .. } = &p.fns[3].body else { panic!("the decision stays") };
    assert!(matches!(&*alts[1].body, Core::Var(LocalId(0))));
    assert!(matches!(alts[1].pat, CPat::Con { tag: 1, .. }));
    assert_eq!(report.case_merge, 1);
}

/// An explicit alternative after the wildcard was unreachable, and moving
/// it ahead of the inner decision would make it match: the merge refuses.
#[test]
fn case_merge_refuses_an_explicit_alternative_after_the_default() {
    let body = Core::Case {
        scrut: Box::new(Core::Var(LocalId(0))),
        alts: vec![
            CAlt::new(CPat::Wild, Core::Case {
                scrut: Box::new(Core::Var(LocalId(0))),
                alts: vec![
                    CAlt::new(con(1, &[]), Core::Int(4)),
                    CAlt::new(con(2, &[]), Core::Int(3)),
                ],
            }),
            CAlt::new(con(0, &[]), Core::Int(1)),
        ],
    };
    let mut p = program(vec![
        ctor_unit("Red", 0, 0),
        ctor_unit("Blue", 1, 0),
        ctor_unit("Green", 2, 0),
        unit("Main.f", vec![Class::Obj], body),
    ]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert_eq!(report.case_merge, 0);
}

/// A weighted decision does not merge: the weight spaces would need
/// rescaling, which the driving pass owns.
#[test]
fn case_merge_refuses_a_weighted_decision() {
    let outer_alts = vec![
        CAlt::weighted(con(0, &[]), Some(1), Core::Int(1)),
        CAlt::weighted(con(1, &[]), Some(1), Core::Case { scrut: Box::new(Core::Var(LocalId(0))), alts: vec![
            CAlt::weighted(con(2, &[]), Some(1), Core::Int(3)),
        ] }),
    ];
    let body = Core::Case { scrut: Box::new(Core::Var(LocalId(0))), alts: outer_alts };
    let mut p = program(vec![
        ctor_unit("Red", 0, 0),
        ctor_unit("Blue", 1, 0),
        ctor_unit("Green", 2, 0),
        unit("Main.f", vec![Class::Obj], body),
    ]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert_eq!(report.case_merge, 0);
    assert!(matches!(&p.fns[3].body, Core::Case { alts, .. } if alts.len() == 2));
}

/// `let t = Some v; case t of None -> 0; Some x -> x` is `v`.
#[test]
fn known_constructor_reduces_a_let_bound_application() {
    let body = Core::Let(
        LocalId(1),
        Class::Obj,
        Box::new(Core::App(Box::new(Core::Ctor(gid("Some"), vec![])), vec![Core::Var(LocalId(0))])),
        Box::new(Core::Case {
            scrut: Box::new(Core::Var(LocalId(1))),
            alts: vec![
                CAlt::new(con(0, &[]), Core::Int(0)),
                CAlt::new(con(1, &[2]), Core::Var(LocalId(2))),
            ],
        }),
    );
    let mut p = program(vec![
        ctor_unit("None", 0, 0),
        ctor_unit("Some", 1, 1),
        unit("Main.f", vec![Class::Obj], body),
    ]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert!(matches!(&p.fns[2].body, Core::Var(LocalId(0))), "{:?}", p.fns[2].body);
    assert_eq!(report.known_ctor, 1);
}

/// A constructor whose argument is a call keeps its decision: the argument
/// would move past the branch's effects.
#[test]
fn known_constructor_refuses_a_computed_argument() {
    let call = Core::App(
        Box::new(Core::Global(gid("Main.g"), vec![])),
        vec![Core::Var(LocalId(0))],
    );
    let body = Core::Let(
        LocalId(1),
        Class::Obj,
        Box::new(Core::App(Box::new(Core::Ctor(gid("Some"), vec![])), vec![call])),
        Box::new(Core::Case {
            scrut: Box::new(Core::Var(LocalId(1))),
            alts: vec![CAlt::new(con(1, &[2]), Core::Var(LocalId(2)))],
        }),
    );
    let mut p = program(vec![
        ctor_unit("None", 0, 0),
        ctor_unit("Some", 1, 1),
        unit("Main.g", vec![Class::Raw], Core::Var(LocalId(0))),
        unit("Main.f", vec![Class::Obj], body),
    ]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions { inline: false, inline_small: 0, ..SimplifyOptions::default() }, &mut report);
    assert_eq!(report.known_ctor, 0);
    assert!(matches!(&p.fns[3].body, Core::Let(..)));
}

/// `dictproj (instdict I) 0` applied at the method's arity is a direct
/// call to the method unit; any other arity keeps the projection.
#[test]
fn direct_dict_call_needs_the_method_arity() {
    let instance = InstanceDef {
        id: gid("Main.Add_Int"),
        method_count: 1,
        subdict_count: 0,
        methods: vec![gid("Main.Add_Int.add")],
        class: "Add".to_string(),
        tparams: 0,
        own: vec![0],
    };
    let add = unit("Main.Add_Int.add", vec![Class::Raw, Class::Raw], Core::Var(LocalId(0)));
    let site = |args: Vec<Core>| {
        Core::App(
            Box::new(Core::DictProj(
                Box::new(Core::InstDict(gid("Main.Add_Int"), vec![], vec![])),
                0,
                None,
                Class::Obj,
            )),
            args,
        )
    };
    let body = site(vec![Core::Int(1), Core::Int(2)]);
    let mut p = program(vec![add.clone(), unit("Main.f", vec![], body)]);
    p.instances.push(instance.clone());
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions { inline: false, inline_small: 0, ..SimplifyOptions::default() }, &mut report);
    assert!(matches!(
        &p.fns[1].body,
        Core::App(head, args) if matches!(&**head, Core::Global(id, _) if id.0 == "Main.Add_Int.add") && args.len() == 2
    ));
    assert_eq!(report.dict_call, 1);

    // The switch keeps the projection.
    let body = site(vec![Core::Int(1), Core::Int(2)]);
    let mut p = program(vec![add.clone(), unit("Main.f", vec![], body)]);
    p.instances.push(instance.clone());
    let mut report = SimplifyReport::default();
    simplify(
        &mut p,
        &SimplifyOptions { direct_dict_calls: false, ..SimplifyOptions::default() },
        &mut report,
    );
    assert_eq!(report.dict_call, 0);
    assert!(matches!(&p.fns[1].body, Core::App(head, _) if matches!(&**head, Core::DictProj(..))));

    // An under-applied projection keeps it too.
    let body = site(vec![Core::Int(1)]);
    let mut p = program(vec![add, unit("Main.f", vec![], body)]);
    p.instances.push(instance);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions { inline: false, inline_small: 0, ..SimplifyOptions::default() }, &mut report);
    assert_eq!(report.dict_call, 0);
}

/// Zero passes disable the simplifier and cost no ticks.
#[test]
fn the_iteration_cap_bounds_the_passes() {
    let body = Core::Case {
        scrut: Box::new(Core::Var(LocalId(0))),
        alts: vec![CAlt::new(CPat::Wild, Core::Int(1))],
    };
    let mut p = program(vec![unit("Main.f", vec![Class::Obj], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::with_iterations(0), &mut report);
    assert_eq!(report.iterations, 0);
    assert_eq!(report.ticks, 0);

    // A program with nothing to rewrite stops after one pass.
    let body = Core::Case {
        scrut: Box::new(Core::Var(LocalId(0))),
        alts: vec![CAlt::new(CPat::Wild, Core::Int(1))],
    };
    let mut p = program(vec![unit("Main.f", vec![Class::Obj], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert_eq!(report.iterations, 1);
    assert_eq!(report.ticks, 0);
}

/// A binder's only use may be an application's head (`let f = prim in
/// f x`); the dead-let rule must see that reader and keep the binding.
/// `child_slots` leaves an application's head out, so the mention test
/// walks it separately, and dropping the let would leave an unbound local
/// for code generation.
#[test]
fn an_application_head_is_a_reader() {
    // The head of an application is a reader of its binder, so the `let` is not
    // dead. The binder holds a constructor application rather than a trivial
    // word or a unit name: those are substituted (that is the local inliner's
    // job) and would prove nothing about the reader analysis.
    let body = Core::Let(
        LocalId(1),
        Class::Obj,
        Box::new(Core::App(Box::new(Core::Ctor(gid("None"), vec![])), vec![])),
        Box::new(Core::App(Box::new(Core::Var(LocalId(1))), vec![Core::Var(LocalId(0))])),
    );
    let mut p = program(vec![ctor_unit("None", 0, 0), unit("Main.f", vec![Class::Raw], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions { inline: false, inline_small: 0, ..SimplifyOptions::default() }, &mut report);
    assert_eq!(report.ticks, 0, "{:?}", p.fns[1].body);
    assert!(matches!(&p.fns[1].body, Core::Let(..)), "{:?}", p.fns[1].body);
}

/// A `let` whose binder no rewrite reads is dropped when its value is a
/// value, and kept when it is a call.
#[test]
fn a_dead_let_over_a_value_is_dropped() {
    let body = Core::Let(LocalId(0), Class::Raw, Box::new(Core::Int(1)), Box::new(Core::Int(2)));
    let mut p = program(vec![unit("Main.f", vec![], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions { inline: false, inline_small: 0, ..SimplifyOptions::default() }, &mut report);
    assert!(matches!(p.fns[0].body, Core::Int(2)));

    let body = Core::Let(
        LocalId(0),
        Class::Raw,
        Box::new(Core::App(Box::new(Core::Global(gid("Main.g"), vec![])), vec![])),
        Box::new(Core::Int(2)),
    );
    let mut p = program(vec![unit("Main.g", vec![], Core::Int(0)), unit("Main.f", vec![], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions { inline: false, inline_small: 0, ..SimplifyOptions::default() }, &mut report);
    assert!(matches!(p.fns[1].body, Core::Let(..)), "a call's effects stay");
}

/// A reduction that exposes a second reduction in the same pass: the
/// outer decision sees the inner one's result because children go first.
#[test]
fn rewrites_compose_within_one_pass() {
    let some = Core::App(Box::new(Core::Ctor(gid("Some"), vec![])), vec![Core::Int(7)]);
    let inner = Core::Case {
        scrut: Box::new(Core::Var(LocalId(1))),
        alts: vec![CAlt::new(con(1, &[2]), Core::Var(LocalId(2)))],
    };
    // `let t = Some 7 in case t of Some x -> x`
    let body = Core::Let(LocalId(1), Class::Obj, Box::new(some), Box::new(inner));
    let mut p = program(vec![ctor_unit("None", 0, 0), ctor_unit("Some", 1, 1), unit("Main.f", vec![], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert!(matches!(p.fns[2].body, Core::Int(7)));
    assert_eq!(report.known_ctor, 1);
    assert_eq!(report.iterations, 2, "the dead let goes on the second pass");

    // The `If`-shaped prologue of a compiled guard is untouched.
    let body = Core::If(
        Box::new(Core::Bool(true)),
        CBranch::new(Core::Int(1)),
        CBranch::new(Core::Int(2)),
    );
    let mut p = program(vec![unit("Main.h", vec![], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert_eq!(report.ticks, 0);
}

/// The dead-let rule is what the second pass does after a
/// known-constructor reduction leaves the constructor with no reader.
#[test]
fn dead_let_follows_the_reduction_that_orphaned_it() {
    let body = Core::Let(
        LocalId(3),
        Class::Obj,
        Box::new(Core::App(Box::new(Core::Ctor(gid("Some"), vec![])), vec![Core::Var(LocalId(0))])),
        Box::new(Core::Case {
            scrut: Box::new(Core::Var(LocalId(3))),
            alts: vec![CAlt::new(con(1, &[4]), Core::Var(LocalId(0)))],
        }),
    );
    let mut p = program(vec![ctor_unit("Some", 1, 1), unit("Main.f", vec![Class::Obj], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert!(matches!(p.fns[1].body, Core::Var(LocalId(0))));
    let _ = TargetProfile::native();
}

// ------------------------------------------------- the unfoldable class
//
// A declaration exposes its body when the body is cheap by construction: a
// primop application, a projection, a literal, a shallow decision over a
// known constructor, or a call to another such declaration. These tests pin
// the class, the recursive rule, and the two site conditions
// (design/inline-unfolding-policy.md).

fn call(name: &str, args: Vec<Core>) -> Core {
    Core::App(Box::new(Core::Global(gid(name), vec![])), args)
}

fn prim(op: cinch::core::PrimOp, args: Vec<Core>) -> Core {
    Core::App(Box::new(Core::Prim(op)), args)
}

/// `Eq_I64.== a b = prim-eq a b`: a dictionary method that is one primop.
/// Both sites take the copy, so no `Global` naming it survives.
#[test]
fn a_primop_method_is_copied_to_every_site() {
    use cinch::core::PrimOp;
    let method = unit(
        "Data.Machine.Eq_I64.==",
        vec![Class::Raw, Class::Raw],
        prim(PrimOp::EqBool, vec![Core::Var(LocalId(0)), Core::Var(LocalId(1))]),
    );
    let body = prim(
        PrimOp::AndBool,
        vec![
            call("Data.Machine.Eq_I64.==", vec![Core::Var(LocalId(0)), Core::Var(LocalId(1))]),
            call("Data.Machine.Eq_I64.==", vec![Core::Var(LocalId(1)), Core::Var(LocalId(0))]),
        ],
    );
    let mut p = program(vec![method, unit("Main.f", vec![Class::Raw, Class::Raw], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert_eq!(report.inlined, 2, "both sites take the primop body");
    let text = format!("{:?}", p.fns[1].body);
    assert!(!text.contains("Data.Machine.Eq_I64.=="), "a call survived: {text}");
}

/// `A.== a b = B.== a b` and `B.== a b = prim a b`: the copy of `A` carries
/// a call to `B`, and the recursive rule closes the chain in the same
/// sweep.
#[test]
fn a_delegating_method_closes_its_chain() {
    use cinch::core::PrimOp;
    let outer = unit(
        "Data.Machine.Eq_I64.=2=",
        vec![Class::Raw, Class::Raw],
        call("Data.Machine.Eq_I64.==", vec![Core::Var(LocalId(0)), Core::Var(LocalId(1))]),
    );
    let inner = unit(
        "Data.Machine.Eq_I64.==",
        vec![Class::Raw, Class::Raw],
        prim(PrimOp::EqBool, vec![Core::Var(LocalId(0)), Core::Var(LocalId(1))]),
    );
    let body = prim(
        PrimOp::OrBool,
        vec![
            call("Data.Machine.Eq_I64.=2=", vec![Core::Var(LocalId(0)), Core::Var(LocalId(1))]),
            call("Data.Machine.Eq_I64.=2=", vec![Core::Var(LocalId(1)), Core::Var(LocalId(0))]),
        ],
    );
    let mut p = program(vec![outer, inner, unit("Main.f", vec![Class::Raw, Class::Raw], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    // Two sites, two links each, plus the sweep over `outer`'s own body.
    assert_eq!(report.inlined, 5, "two sites, two links each");
    let text = format!("{:?}", p.fns[2].body);
    assert!(!text.contains("Eq_I64.=2="), "the outer call survived: {text}");
    assert!(!text.contains("Eq_I64.=="), "the inner call survived: {text}");
}

/// A unit in a recursive group is refused, so nothing unfolds a body that
/// can reach itself.
#[test]
fn a_recursive_method_is_refused() {
    use cinch::core::PrimOp;
    let mut again = unit("Main.again", vec![Class::Raw], Core::Unit);
    again.body = call("Main.again", vec![prim(PrimOp::NotBool, vec![Core::Var(LocalId(0))])]);
    let body = call("Main.again", vec![Core::Bool(true)]);
    let mut p = program(vec![again, unit("Main.f", vec![Class::Raw], body)]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert_eq!(report.inlined, 0);
}

/// A decision-bearing body over `inline_unfold_small` nodes is copied only
/// where the scrutinee argument is a constructor application, which the
/// known-constructor rule then folds.
#[test]
fn a_decision_is_copied_only_where_it_folds() {
    use cinch::core::PrimOp;
    // case p of Path s -> not s; Path s -> not s; Path s -> not s
    let alternative = |field: usize| {
        CAlt::new(
            con(0, &[field]),
            prim(PrimOp::AndBool, vec![Core::Var(LocalId(field)), Core::Var(LocalId(field))]),
        )
    };
    let method = unit(
        "Data.Path.unpath",
        vec![Class::Obj],
        Core::Case {
            scrut: Box::new(Core::Var(LocalId(0))),
            alts: vec![alternative(1), alternative(2), alternative(3)],
        },
    );
    let known = Core::App(Box::new(Core::Ctor(gid("Path"), vec![])), vec![Core::Var(LocalId(0))]);
    let body = prim(
        PrimOp::AndBool,
        vec![
            call("Data.Path.unpath", vec![known]),
            call("Data.Path.unpath", vec![Core::Var(LocalId(0))]),
            call("Data.Path.unpath", vec![Core::Var(LocalId(0))]),
        ],
    );
    let mut p = program(vec![
        ctor_unit("Path", 0, 1),
        method,
        unit("Main.f", vec![Class::Obj], body),
    ]);
    let mut report = SimplifyReport::default();
    simplify(&mut p, &SimplifyOptions::default(), &mut report);
    assert_eq!(report.inlined, 1, "only the known-constructor site takes the decision");
}

//! Coalescing the count operations of a straight-line run
//! (`design/reclamation.md` section 5; Lean 4's `CoalesceRC.lean`).
//!
//! Within one basic block a later `inc` of a value proves the value never
//! passed through a state where it could be freed, so every `inc` of that
//! value may be issued at the position of the first; symmetrically the
//! value cannot die before its last `dec`, so every `dec` may be issued at
//! the position of the last. Both moves are unobservable for the same
//! reason: a count operation that is neither the last reference nor the
//! only one cannot be the operation that frees the block.
//!
//! Our `Core` carries no count *number*: a `Dup` and a `Drop` are one
//! reference each, so coalescing here is a move and never a sum. That makes
//! the pass smaller than Lean's and the same in effect. What it buys is the
//! thing the backend cares about: the count operations end up in the fewest
//! places, which shortens the live ranges the register allocator carries
//! across a run and takes count operations out from between the field reads
//! that a reuse wants to see.
//!
//! # Why it runs last
//!
//! `CoalesceRC.lean` states the constraint in its own module doc and it
//! holds here unchanged: this pass runs *after* the reuse insertion, which
//! reads the `Dup`s where reference counting put them. A `Dup` moved to the
//! front of a run is still one the insertion could have used, but one moved
//! across a `Release` would not be, so the order is fixed: counting, then
//! reuse, then this.
//!
//! # What a run is
//!
//! A run is a spine of `Let` and the count operations, ending in anything
//! else: a call, a branch, a case, a jump, a fault. Nothing crosses the
//! node that ends a run, which is what makes the moves legal; in particular
//! nothing crosses a call, which keeps the constant-stack rule that no
//! count operation follows a `musttail` call. Every branch, alternative and
//! join body is its own run.
//!
//! # The two moves
//!
//! A `Dup` of `x` moves earlier only, to the run's first `Dup` of the same
//! value, and never later. When the run holds several `Dup`s of one value
//! they move together and keep their multiplicity, because a `Dup` is one
//! reference and emitting one fewer would leave the run holding one less
//! than it took. A `Drop` of `x` moves later, but only across steps that do
//! not mention `x` at all: the counting pass puts a drop where a path stops
//! needing the value, which can be before a later step reads it again, and
//! sinking a drop past such a read would release a reference the read
//! borrows.
//!
//! Both moves preserve the run's meaning step by step. Before the run's
//! first `Dup` of a value that value is still held by whoever the run
//! inherited it from, so moving the other `Dup`s up to that point only
//! extends the range over which its count is positive; and a `Drop` that
//! crosses only steps free of its value releases a reference nobody can
//! observe.
//!
//! Moving a `Dup` *down* to the first step that reads the value is unsound,
//! and that is what this pass did until it was caught. The step that ends a
//! value's life need not read it: a `Release` frees the block a field was
//! loaded from, and a `Drop` releases the block itself, so a `Dup` placed
//! after either reads a dead word. `design/reclamation.md` section 5 states
//! the rule as the live range of the value, and Lean's `CoalesceRC` issues
//! every `inc` at the first. Three reclamation programs and the bootstrap's
//! own seed crashed under the earlier rule; the ownership verifier sees the
//! moved program rather than the move, so it caught the fault only once it
//! was run after this pass rather than inside the counting pass.

use crate::core::{CAlt, CBranch, Core, LocalId, Program};
use crate::repr::Class;
use std::collections::BTreeMap;

/// Coalesce every unit's count operations in place. A profile that does not
/// reclaim has none.
pub fn coalesce(program: &mut Program, profile: &crate::core::TargetProfile) -> crate::Result<()> {
    if !profile.reclaim {
        return Ok(());
    }
    for f in &mut program.fns {
        if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
            continue;
        }
        let body = std::mem::replace(&mut f.body, Core::Unit);
        f.body = run(&body);
    }
    // This pass moves the count operations of a body the counting pass
    // already balanced, so the balance has to be re-established on what it
    // produced: a `Dup` moved past the step that ended the value's life is
    // balanced where counting left it and unbalanced here, and only the
    // moved program shows that.
    if crate::rc::verifying() {
        crate::rc::verify_program(program)?;
    }
    Ok(())
}

/// One step of a run: the node, minus the continuation the run carries.
#[derive(Clone)]
enum Step {
    /// `let x = v`; `v`'s own runs are already rewritten.
    Let(LocalId, Class, Core),
    /// One reference taken on `x`.
    Dup(LocalId, Class),
    /// One reference released on `x`.
    Drop(LocalId, Class),
}

/// The run `e` begins, with the node that ends it.
struct Run {
    steps: Vec<Step>,
    tail: Core,
}

/// Rewrite `e`'s runs.
fn run(e: &Core) -> Core {
    let Run { steps, tail } = split(e);
    rebuild(&steps, tail)
}

fn split(e: &Core) -> Run {
    let mut steps = Vec::new();
    let mut cur = e;
    loop {
        match cur {
            Core::Let(x, c, v, b) => {
                steps.push(Step::Let(*x, *c, run(v)));
                cur = b;
            }
            Core::Dup(x, c, b) => {
                steps.push(Step::Dup(*x, *c));
                cur = b;
            }
            Core::Drop(x, c, b) => {
                steps.push(Step::Drop(*x, *c));
                cur = b;
            }
            other => return Run { steps, tail: nested(other) },
        }
    }
}

/// The node that ends a run, with every run nested inside it rewritten.
fn nested(e: &Core) -> Core {
    match e {
        Core::If(c, t, f) => Core::If(
            Box::new(run(c)),
            CBranch { weight: t.weight, body: Box::new(run(&t.body)) },
            CBranch { weight: f.weight, body: Box::new(run(&f.body)) },
        ),
        Core::Case { scrut, alts } => Core::Case {
            scrut: Box::new(run(scrut)),
            alts: alts
                .iter()
                .map(|a| CAlt { pat: a.pat.clone(), weight: a.weight, body: Box::new(run(&a.body)) })
                .collect(),
        },
        Core::Join { id, params, body, rest } => Core::Join {
            id: *id,
            params: params.clone(),
            body: Box::new(run(body)),
            rest: Box::new(run(rest)),
        },
        Core::Release { block, moved, rest, token, body } => Core::Release {
            block: *block,
            moved: moved.clone(),
            rest: rest.clone(),
            token: *token,
            body: Box::new(run(body)),
        },
        Core::Quote { template, holes, body } => Core::Quote {
            template: template.clone(),
            holes: holes.iter().map(run).collect(),
            body: body.as_ref().map(|b| Box::new(run(b))),
        },
        other => rewrite_children(other),
    }
}

/// Rewrite the runs inside a node that is not itself a run spine: every
/// subexpression is a value of its own, so each is a run.
fn rewrite_children(e: &Core) -> Core {
    let mut out = e.clone();
    for slot in crate::core::child_slots(&mut out) {
        let taken = std::mem::replace(slot, Core::Unit);
        *slot = run(&taken);
    }
    out
}

/// The values a step *uses*: the locals of a `let` value. A count
/// operation is not a use, so a value whose only steps are its own counts
/// has no use at all and its counts stay where they are.
fn uses(step: &Step) -> Vec<usize> {
    match step {
        Step::Let(_, _, v) => {
            let mut out = Vec::new();
            crate::core::walk_locals(v, &mut |l| out.push(l.0));
            out
        }
        Step::Dup(..) | Step::Drop(..) => Vec::new(),
    }
}

/// Rebuild a run, moving a value's `Dup`s earlier and its `Drop`s later,
/// but never across a step that reads the value.
///
/// The counting pass places a `Drop` at the point a path stops needing the
/// value, which can be before a later step that mentions it, so "after the
/// last mention" is the wrong anchor: sinking a drop past a use would
/// release a reference a step after it still borrows. The anchor is
/// therefore the last step before the drop that reads the value, and the
/// move is confined to the gap of steps that do not mention it at all.
/// That gap is exactly the freedom the coalescing argument licenses: within
/// it no operation can observe the count.
fn rebuild(steps: &[Step], tail: Core) -> Core {
    // Each step's mentions, so a move can be checked against them.
    let mut mentions: Vec<Vec<usize>> = Vec::with_capacity(steps.len());
    for s in steps {
        let mut v = uses(s);
        match s {
            Step::Dup(x, _) | Step::Drop(x, _) => v.push(x.0),
            Step::Let(..) => {}
        }
        mentions.push(v);
    }
    // Where a value's `Dup`s go: the position of the run's first `Dup` of
    // it, and never any later. The first `Dup` is the earliest point the
    // run is known to hold the value by its own reference, so moving the
    // others up to it only lengthens the range over which the count is
    // positive, which no operation can observe. Moving one *down* is what
    // the earlier rule did, to the first step that read the value, and that
    // is unsound: a step in between can end the value's life without
    // reading it, because a `Release` or a `Drop` of the block a field was
    // taken from frees the field's cell, and a `Dup` of it placed after
    // that reads a dead word (`design/reclamation.md` section 5, the
    // live-range rule; Lean's `CoalesceRC` moves every `inc` to the first).
    let mut dup_target: BTreeMap<usize, usize> = BTreeMap::new();
    for (i, s) in steps.iter().enumerate() {
        if let Step::Dup(x, _) = s {
            dup_target.entry(x.0).or_insert(i);
        }
    }
    // A `Drop` stays where it is unless the steps after it, up to the next
    // step that mentions the value, are all free of it: then it may move
    // past them.
    let mut drop_target: BTreeMap<usize, usize> = BTreeMap::new();
    for (i, s) in steps.iter().enumerate() {
        let Step::Drop(x, _) = s else { continue };
        let mut to = i;
        for (j, m) in mentions.iter().enumerate().skip(i + 1) {
            if m.contains(&x.0) {
                break;
            }
            if matches!(steps[j], Step::Let(..)) {
                to = j;
            }
        }
        // Several drops of one value keep their order relative to each
        // other by moving to the same place in reverse.
        drop_target.insert(i, to);
    }
    let mut dup_class: BTreeMap<usize, Class> = BTreeMap::new();
    let mut drop_class: BTreeMap<usize, Class> = BTreeMap::new();
    for s in steps {
        match s {
            Step::Dup(x, c) => {
                dup_class.entry(x.0).or_insert(*c);
            }
            Step::Drop(x, c) => {
                drop_class.entry(x.0).or_insert(*c);
            }
            Step::Let(..) => {}
        }
    }
    let mut before: Vec<Vec<(usize, Class)>> = (0..steps.len()).map(|_| Vec::new()).collect();
    let mut after: Vec<Vec<(usize, Class)>> = (0..steps.len()).map(|_| Vec::new()).collect();
    let mut stay: Vec<Vec<Step>> = (0..steps.len()).map(|_| Vec::new()).collect();
    let mut dup_count: BTreeMap<usize, usize> = BTreeMap::new();
    for s in steps {
        if let Step::Dup(x, c) = s {
            let e = dup_class.entry(x.0).or_insert(*c);
            let _ = e;
            *dup_count.entry(x.0).or_insert(0) += 1;
        }
    }
    for (x, n) in &dup_count {
        match dup_target.get(x) {
            Some(i) => before[*i].extend(std::iter::repeat_n((*x, dup_class[x]), *n)),
            None => stay[0].extend(std::iter::repeat_n(Step::Dup(LocalId(*x), dup_class[x]), *n)),
        }
    }
    let mut drop_at: Vec<Vec<(usize, Class)>> = (0..steps.len()).map(|_| Vec::new()).collect();
    for (i, s) in steps.iter().enumerate() {
        if let Step::Drop(x, c) = s {
            match drop_target.get(&i) {
                Some(to) if *to != i => drop_at[*to].push((x.0, *c)),
                _ => stay[i].push(Step::Drop(*x, *c)),
            }
        }
    }
    for v in before.iter_mut().chain(after.iter_mut()).chain(drop_at.iter_mut()) {
        v.sort();
    }
    let mut body = tail;
    for i in (0..steps.len()).rev() {
        for (x, c) in drop_at[i].iter().rev() {
            body = Core::Drop(LocalId(*x), *c, Box::new(body));
        }
        if let Step::Let(x, c, v) = &steps[i] {
            body = Core::Let(*x, *c, Box::new(v.clone()), Box::new(body));
        }
        for s in stay[i].iter().rev() {
            match s {
                Step::Dup(x, c) => body = Core::Dup(*x, *c, Box::new(body)),
                Step::Drop(x, c) => body = Core::Drop(*x, *c, Box::new(body)),
                Step::Let(..) => {}
            }
        }
        for (x, c) in before[i].iter().rev() {
            body = Core::Dup(LocalId(*x), *c, Box::new(body));
        }
    }
    body
}

/// The number of count operations a body holds, for the report a caller
/// prints: a measure of what the pass had to move.
pub fn count_operations(program: &Program) -> (usize, usize) {
    let mut dups = 0;
    let mut drops = 0;
    for f in &program.fns {
        walk(&f.body, &mut dups, &mut drops);
    }
    (dups, drops)
}

fn walk(e: &Core, dups: &mut usize, drops: &mut usize) {
    match e {
        Core::Dup(_, _, b) => {
            *dups += 1;
            walk(b, dups, drops);
        }
        Core::Drop(_, _, b) => {
            *drops += 1;
            walk(b, dups, drops);
        }
        other => {
            for k in crate::core::child_slots_imm(other) {
                walk(k, dups, drops);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{CAlt, CPat, Core, GlobalId};

    fn var(n: usize) -> Core {
        Core::Var(LocalId(n))
    }

    fn dup(n: usize, b: Core) -> Core {
        Core::Dup(LocalId(n), Class::Obj, Box::new(b))
    }

    fn drop_(n: usize, b: Core) -> Core {
        Core::Drop(LocalId(n), Class::Obj, Box::new(b))
    }

    fn app(name: &str, args: Vec<Core>) -> Core {
        Core::App(Box::new(Core::Global(GlobalId(name.to_string()), Vec::new())), args)
    }

    /// The order of a run's count operations, as `(dup|drop, local)`.
    fn order(e: &Core, out: &mut Vec<(char, usize)>) {
        match e {
            Core::Dup(x, _, b) => {
                out.push(('d', x.0));
                order(b, out);
            }
            Core::Drop(x, _, b) => {
                out.push(('x', x.0));
                order(b, out);
            }
            Core::Let(_, _, v, b) => {
                order(v, out);
                order(b, out);
            }
            _ => {}
        }
    }

    #[test]
    fn a_dup_moves_up_to_the_first_dup_and_keeps_its_multiplicity() {
        // let a = f (v1)
        // let b = g (v1)
        // dup v0
        // let c = h (v0)
        // dup v0
        //   tail
        let body = Core::Let(
            LocalId(10),
            Class::Raw,
            Box::new(app("f", vec![var(1)])),
            Box::new(Core::Let(
                LocalId(11),
                Class::Raw,
                Box::new(app("g", vec![var(1)])),
                Box::new(dup(
                    0,
                    Core::Let(
                        LocalId(12),
                        Class::Raw,
                        Box::new(app("h", vec![var(0)])),
                        Box::new(dup(0, app("k", vec![var(12)]))),
                    ),
                )),
            )),
        );
        let mut before = Vec::new();
        order(&body, &mut before);
        assert_eq!(before, vec![('d', 0), ('d', 0)]);
        let after = run(&body);
        let mut got = Vec::new();
        order(&after, &mut got);
        // Both dups now sit at the run's first `Dup` of v0, so the count is
        // positive across the whole run and neither can read a released
        // word.
        assert_eq!(got, vec![('d', 0), ('d', 0)]);
        let Core::Let(..) = after else { panic!("the first let is the run's head") };
        let Core::Let(..) = nth_body(&after, 0) else { panic!("the second let follows") };
        let Core::Dup(..) = nth_body(&after, 1) else {
            panic!("the dups did not move up to the first: {after:?}");
        };
    }

    #[test]
    fn a_dup_does_not_move_below_the_step_that_frees_its_cell() {
        // A `Release` frees the block a pattern field was loaded from
        // without reading that field, so a `Dup` of the field placed after
        // the release dups a dead word. This is the bootstrap's own shape:
        // `decls-refusals` dups the tail of its scrutinee after an arm has
        // released the cell the tail came from, and the bootstrap then lost
        // 14 of its 24 refusals.
        //
        // dup v1
        // let a = case v0 of (release v0 (moved v2) ...)   <- v1's cell dies
        // let b = case v1 of ...                            <- reads v1
        //   dup v1                                          <- must not land here
        let body = dup(
            1,
            Core::Let(
                LocalId(10),
                Class::Raw,
                Box::new(Core::Release {
                    block: LocalId(0),
                    moved: vec![(LocalId(2), Class::Obj)],
                    rest: Vec::new(),
                    token: None,
                    body: Box::new(app("f", vec![var(2)])),
                }),
                Box::new(Core::Let(
                    LocalId(11),
                    Class::Raw,
                    Box::new(Core::Case {
                        scrut: Box::new(var(1)),
                        alts: vec![CAlt {
                            pat: CPat::Wild,
                            weight: None,
                            body: Box::new(app("g", vec![])),
                        }],
                    }),
                    Box::new(dup(1, app("k", vec![var(11)]))),
                )),
            ),
        );
        let after = run(&body);
        let text = format!("{after:?}");
        let release_at = text.find("Release").expect("the release survives");
        let first_dup_at = text.find("Dup(LocalId(1)").expect("the dup survives");
        assert!(
            first_dup_at < release_at,
            "a dup moved below the release that freed its cell: {text}"
        );
    }

    /// The `n`-th continuation of a `let` spine.
    fn nth_body(e: &Core, n: usize) -> &Core {
        let mut cur = e;
        for _ in 0..n {
            match cur {
                Core::Let(_, _, _, b) => cur = b,
                other => return other,
            }
        }
        match cur {
            Core::Let(_, _, _, b) => b,
            other => other,
        }
    }

    #[test]
    fn a_drop_does_not_cross_a_step_that_reads_the_value() {
        // let a = f (v0)          <- v0 is read here
        // drop v0
        // let b = g (v1)          <- free of v0, so the drop may pass it
        // let c = h (v0)          <- reads v0 again: the drop must stay
        //   above this step
        let body = Core::Let(
            LocalId(10),
            Class::Raw,
            Box::new(app("f", vec![var(0)])),
            Box::new(drop_(
                0,
                Core::Let(
                    LocalId(11),
                    Class::Raw,
                    Box::new(app("g", vec![var(1)])),
                    Box::new(Core::Let(
                        LocalId(12),
                        Class::Raw,
                        Box::new(app("h", vec![var(0)])),
                        Box::new(app("k", vec![var(12)])),
                    )),
                ),
            )),
        );
        let after = run(&body);
        // The drop moved past the `g` step and no further: reading v0 after
        // it would be a use of a released reference.
        let text = format!("{after:?}");
        let drop_at = text.find("Drop(LocalId(0)").expect("the drop survives");
        let h_at = text.find("\"h\"").expect("the h call survives");
        assert!(drop_at < h_at, "the drop crossed the step that reads v0: {text}");
    }

    #[test]
    fn a_value_with_no_use_in_the_run_keeps_its_counts() {
        // dup v0 ; drop v0 ; tail: nothing reads v0, so nothing anchors a
        // move and the pair stays where it was.
        let body = dup(0, drop_(0, app("k", vec![var(1)])));
        let after = run(&body);
        let mut got = Vec::new();
        order(&after, &mut got);
        assert_eq!(got, vec![('d', 0), ('x', 0)]);
    }
}

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
//! A `Dup` of `x` is placed immediately before the earliest step of the run
//! that mentions `x`. When the run holds several `Dup`s of the same value
//! the earliest placement takes them all (they are one position), which is
//! the sum Lean writes. A `Drop` of `x` is placed immediately after the
//! latest step that mentions `x`, so a run holding several `Drop`s of one
//! value ends with all of them together.
//!
//! Both moves preserve the run's meaning step by step. Between the earliest
//! and the latest mention of `x` the value is held either way, so no
//! intervening operation can free it; before the earliest mention a `Dup`
//! only adds a reference to a value that is still held by whoever the run
//! inherited it from, and after the latest mention a `Drop` only releases a
//! reference nobody can observe again.

use crate::core::{CAlt, CBranch, Core, LocalId, Program};
use crate::repr::Class;
use std::collections::BTreeMap;

/// Coalesce every unit's count operations in place. A profile that does not
/// reclaim has none.
pub fn coalesce(program: &mut Program, profile: &crate::core::TargetProfile) {
    if !profile.reclaim {
        return;
    }
    for f in &mut program.fns {
        if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
            continue;
        }
        let body = std::mem::replace(&mut f.body, Core::Unit);
        f.body = run(&body);
    }
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

/// Rebuild a run, placing each value's `Dup`s before its first use and its
/// `Drop`s after its last.
///
/// A value with no use in the run keeps its counts where they are: without
/// a use there is nothing to move relative to, and moving a count of a
/// value nobody reads would be a change with no argument behind it. That
/// is also what keeps the pass sound, because the anchor of a move is
/// always a step that stays put.
fn rebuild(steps: &[Step], tail: Core) -> Core {
    let mut first_use: BTreeMap<usize, usize> = BTreeMap::new();
    let mut last_use: BTreeMap<usize, usize> = BTreeMap::new();
    for (i, s) in steps.iter().enumerate() {
        for x in uses(s) {
            first_use.entry(x).or_insert(i);
            last_use.insert(x, i);
        }
    }
    // The multiplicity is kept, never summed into one operation: a `Dup`
    // is exactly one reference, so two of them are two references and
    // emitting one would leave the run holding one less than it took.
    // Lean sums because its `inc` carries a count; ours does not.
    let mut dup_class: BTreeMap<usize, (Class, usize)> = BTreeMap::new();
    let mut drop_class: BTreeMap<usize, (Class, usize)> = BTreeMap::new();
    for s in steps {
        match s {
            Step::Dup(x, c) => {
                let e = dup_class.entry(x.0).or_insert((*c, 0));
                e.1 += 1;
            }
            Step::Drop(x, c) => {
                let e = drop_class.entry(x.0).or_insert((*c, 0));
                e.1 += 1;
            }
            Step::Let(..) => {}
        }
    }
    // Where each count goes, and which counts stay put.
    let mut before: Vec<Vec<(usize, Class)>> = vec![Vec::new(); steps.len()];
    let mut after: Vec<Vec<(usize, Class)>> = vec![Vec::new(); steps.len()];
    let mut stay: Vec<Vec<Step>> = (0..steps.len()).map(|_| Vec::new()).collect();
    for (x, (c, n)) in &dup_class {
        match first_use.get(x) {
            Some(i) => before[*i].extend(std::iter::repeat_n((*x, *c), *n)),
            None => stay[0].extend(std::iter::repeat_n(Step::Dup(LocalId(*x), *c), *n)),
        }
    }
    for (x, (c, n)) in &drop_class {
        match last_use.get(x) {
            Some(i) => after[*i].extend(std::iter::repeat_n((*x, *c), *n)),
            None => stay[0].extend(std::iter::repeat_n(Step::Drop(LocalId(*x), *c), *n)),
        }
    }
    for v in before.iter_mut().chain(after.iter_mut()) {
        v.sort();
    }
    let mut body = tail;
    for i in (0..steps.len()).rev() {
        // The step that ends at this position, in reverse: the counts
        // placed after it, then the step, then the counts placed before it.
        for (x, c) in after[i].iter().rev() {
            body = Core::Drop(LocalId(*x), *c, Box::new(body));
        }
        if let Step::Let(x, c, v) = &steps[i] {
            body = Core::Let(*x, *c, Box::new(v.clone()), Box::new(body));
        }
        for s in stay[i].iter().rev() {
            if let Step::Dup(x, c) = s {
                body = Core::Dup(*x, *c, Box::new(body));
            } else if let Step::Drop(x, c) = s {
                body = Core::Drop(*x, *c, Box::new(body));
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
    fn a_dup_moves_to_the_first_use_and_a_drop_after_the_last() {
        // let a = f (v1)
        // dup v0
        // let b = g (v0)
        // dup v0
        // drop v0
        // let c = h (v0)
        // drop v0
        //   tail
        let body = Core::Let(
            LocalId(10),
            Class::Raw,
            Box::new(app("f", vec![var(1)])),
            Box::new(dup(
                0,
                Core::Let(
                    LocalId(11),
                    Class::Raw,
                    Box::new(app("g", vec![var(0)])),
                    Box::new(dup(
                        0,
                        drop_(
                            0,
                            Core::Let(
                                LocalId(12),
                                Class::Raw,
                                Box::new(app("h", vec![var(0)])),
                                Box::new(drop_(0, app("k", vec![var(11)]))),
                            ),
                        ),
                    )),
                ),
            )),
        );
        let mut before = Vec::new();
        order(&body, &mut before);
        assert_eq!(before, vec![('d', 0), ('d', 0), ('x', 0), ('x', 0)]);
        let after = run(&body);
        let mut got = Vec::new();
        order(&after, &mut got);
        // Both dups now sit before the first use of v0 and both drops
        // after its last, so the run holds the value across all of it.
        assert_eq!(got, vec![('d', 0), ('d', 0), ('x', 0), ('x', 0)]);
        let Core::Let(..) = after else { panic!("the first let is the run's head") };
        let Core::Dup(..) = nth_body(&after, 0) else {
            panic!("the dups did not move to the first use: {after:?}");
        };
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

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
    // Where each count goes, and which counts stay put.
    let mut before: Vec<Vec<(usize, Class)>> = vec![Vec::new(); steps.len()];
    let mut after: Vec<Vec<(usize, Class)>> = vec![Vec::new(); steps.len()];
    let mut stay: Vec<Vec<Step>> = (0..steps.len()).map(|_| Vec::new()).collect();
    for (x, c) in &dup_class {
        match first_use.get(x) {
            Some(i) => before[*i].push((*x, *c)),
            None => stay[0].push(Step::Dup(LocalId(*x), *c)),
        }
    }
    for (x, c) in &drop_class {
        match last_use.get(x) {
            Some(i) => after[*i].push((*x, *c)),
            None => stay[0].push(Step::Drop(LocalId(*x), *c)),
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

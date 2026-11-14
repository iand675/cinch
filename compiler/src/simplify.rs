//! The Core simplifier (design/compilation-speed.md section 10): local
//! rewrites over the specialized program, applied until a pass changes
//! nothing, under an iteration cap and a reported tick count.
//!
//! The shape is GHC's, minus the analyses: a pass walks every unit
//! bottom up and applies local rewrites; a tick is one rewrite
//! application; passes repeat while the previous one fired, to at most
//! `--max-simplifier-iterations` passes (default 4). The reported ticks
//! say which rule does the work, and the cap is what keeps a rewrite
//! pair that undoes itself from looping.
//!
//! Six rules, in this slice:
//!
//! - `case-merge`: an outer decision has alternatives whose body is a
//!   decision on the same local, so the two decisions become one. The
//!   shape is what the match compiler produces for a source wildcard
//!   (`case c of Red -> 1; _ -> case c of Blue -> 4; ...`), with one
//!   wildcard or variable default, or one constructor alternative per
//!   default; aliases the lowering wraps around a fallthrough
//!   (`let y = x in case y of ...`) are resolved, and a variable
//!   default's binder is substituted by the outer local. The rule assumes
//!   the inner decision is keyed on constructor tags alone (a wildcard
//!   alternative would shadow the outer alternatives it must be merged
//!   under), that defaults come after every explicit alternative (an
//!   explicit alternative after a wildcard was unreachable and moving it
//!   ahead of the inner decision would make it match), that both
//!   decisions carry no weights (a weighted merge rescales the weight
//!   spaces, attribute annotations specification section 2.4, and belongs
//!   to the driving pass that implements it), and that no binder a
//!   default pattern introduced is read by a kept alternative. An inner
//!   alternative whose tag an explicit outer alternative names is dead
//!   (the default is only reached for the other tags) and is dropped
//!   without rescaling; if no inner alternative survives, the rewrite
//!   refuses rather than deleting the decision.
//! - `known-constructor`: a decision whose scrutinee is a local bound to
//!   a constructor application (directly, or through the alias `let`s
//!   lowering adds) reduces to the alternative that tag selects, with the
//!   alternative's field binders substituted by the application's
//!   arguments. A body that carries a quotation keeps its decision: a
//!   quotation's retained body is a binding scope of its own and the
//!   substitution is not defined across it. This is GHC's
//!   case-of-known-constructor and
//!   it removes the `Option`/`Result` scaffolding the checker's match
//!   compilation produces. The rewrite assumes the constructor's
//!   arguments are values (a local, a literal, a unit or primitive
//!   reference): then no evaluation moves past an effect and no argument
//!   expression is duplicated, so substituting an argument at every use
//!   of its field binder is observationally identical. An application
//!   with any other argument keeps its decision.
//! - `direct-dict-call`: `dictproj (instdict I) idx` applied to exactly
//!   the method unit's arity is a direct call to that unit instead of a
//!   projection plus `__apply`. The rule assumes the instance is static
//!   (no subdictionaries, every method's arity known and nonzero), which
//!   is exactly what code generation's `static_dict` requires: the
//!   projection then yields a shared closure with no environment, so the
//!   direct call computes what the application computed. Disable it with
//!   [`SimplifyOptions::direct_dict_calls`] `= false`.
//!
//! - `case-of-case`: a decision scrutinized by another decision pushes the
//!   outer decision into the alternatives of the inner one, so the value
//!   the inner decision produced is scrutinized where it is produced
//!   instead of being handed to a decision of its own:
//!   `case (case s of A -> eA; B -> eB) of P -> fP; Q -> fQ` becomes
//!   `case s of A -> case eA of P -> fP; Q -> fQ; B -> ...`. A strict
//!   language evaluates `eA` in that branch either way, so the two orders
//!   agree; what changes is that the intermediate decision `case t of ...`
//!   on a temporary disappears. The rule also takes the shape our lowering
//!   actually produces, a `let` binding the value before it is
//!   scrutinized (`let t = case s of ... in case t of ...`) when `t` is
//!   read exactly once (at the outer decision's scrutinee position): the
//!   `let`s that open the value, including the alias `let y = x` the
//!   lowering adds, stay in place around the pushed decision.
//!
//!   This rule duplicates the outer alternatives, once per inner
//!   alternative, so it is paired with join points: an outer alternative
//!   whose body is larger than [`SimplifyOptions::join_threshold`] nodes is
//!   lifted into a [`Core::Join`] over its own pattern binders, and each
//!   copy becomes a [`Core::Jump`] to it, so the body's code exists once.
//!   A smaller body is copied, with its pattern binders renamed to fresh
//!   locals per copy, and the copied node count is reported as
//!   `duplicated`. Nothing is duplicated when the inner decision has one
//!   alternative, so no join is introduced then.
//!
//!   It refuses when the inner decision carries weights (a push moves its
//!   alternatives into outer branches, where its weight space has no
//!   meaning, and the driving pass owns the weighted forms), when any
//!   alternative of either decision carries a quotation (a quotation's
//!   retained body is a binding scope of its own and the copies would share
//!   it), when the inner scrutinee is used inside either decision's
//!   alternatives (the running condition: the pushed copies re-read the
//!   scrutinee, and a use of it in an alternative would make the two reads
//!   one value visible to each other), and when the inner scrutinee is not
//!   a local (a computed scrutinee would be evaluated again per copy).
//!
//! - `known-constructor`'s computed-argument half (off by default; see the
//!   rule's own documentation and design/compilation-speed.md section 10.6):
//!   a decision on `Ctor a1 a2` whose arguments are computed is folded by
//!   rebinding each argument to a fresh local *where the construction stood*,
//!   in argument order, and substituting the field binders by those locals.
//!   Nothing moves, so evaluation order is preserved by construction and no
//!   single-use condition is needed. The rule is sound and removes 5,420 Core
//!   nodes on the compiler's own bundle, but it makes the inliner duplicate:
//!   deleting the constructor a body's decision reads stops the body being
//!   decision-bearing at the site test, which admits a copy the benefit model
//!   refused before, and two units then reach 57,719 and 37,538 lines. It
//!   ships off until the inliner credits a fold it enables.
//! - `projection`: `proj (Ctor a1 a2) k` is `ak` when `ak` is a value. The
//!   shape is reachable from source and from `case-of-case`; it fires zero
//!   times on this corpus, where every projection reads a parameter.
//!
//! A pass also deletes a `let` whose binder the body no longer mentions
//! when its value is a value: after a known-constructor reduction the
//! constructor application that only the decision read has no reader,
//! and dropping the binding removes the allocation. The rule assumes the
//! value is effect free, which is the same value predicate the
//! known-constructor rule uses, and it counts an application's head as a
//! reader: `child_slots` leaves the head out, and a binder read only
//! there (`let f = prim in f x`) must keep its binding.
//!
//! Both the mention test and substitution walk the application head for
//! that reason. A substitution that stopped at the arguments left a
//! projection or case binder in closure position unbound, which code
//! generation reports as `unbound local <n>`.
//!
//! The report records the pass cap, the passes actually run, the ticks, and
//! the count per rule (`case_merge`, `known_constructor`, `direct_dict_call`,
//! `case_of_case`), plus what the join introduction did: `joins` (the join
//! points it made) and `duplicated` (the nodes it copied instead of sharing
//! them). The artifact manifest carries all of them under `simplifier`, so a
//! change in the pass's work is visible in the artifact rather than inferred
//! from IR size.
//!
//! Join points carry two invariants beyond their local shape, and every
//! consumer holds them: a `Jump` targets exactly one enclosing `Join` of its
//! id and sits where its value is that join's result, and the join's
//! parameters are owned exactly as function parameters are (a jump hands one
//! reference over per parameter; the body consumes each exactly once on
//! every path). `crate::rc` inserts the count operations for both, and code
//! generation gives the pair a block and a branch: no closure, no stack
//! frame, no call.
use crate::core::{
    child_slots, child_slots_imm, alpha_eq, CAlt, CPat, Class, Core, FnDef, GlobalId, JoinId,
    LocalId, Program, SimplifyReport,
};
use std::collections::{HashMap, HashSet};

/// Which inlining decision the pass makes at a call site.
///
/// Two models, one rule: both answer "may this call be replaced by the
/// callee's body?", and both are reached through the same admission walk
/// and the same copy machinery. They differ only in what the site test
/// compares, which is the whole of the difference between GHC's two
/// thresholds and Flambda 2's benefit against size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InlineModel {
    /// GHC's shape (design/inline-unfolding-policy.md section 10): the
    /// body's weighted size minus the call's discount, compared against
    /// [`SimplifyOptions::inline_use_threshold`], with the unfoldable class
    /// bypassing the test and the per-shape node caps as the only bound on
    /// a certain copy.
    Threshold,
    /// Flambda 2's shape: an estimate of the work a copy removes (the
    /// benefit) against the code size the copy adds (the size), so a copy
    /// that deletes an allocation and a call is admitted even when it is
    /// larger than the call it replaces, and a copy that deletes nothing is
    /// refused however small it is.
    Benefit,
}

/// The simplifier's budget and switches.
#[derive(Clone, Debug)]
pub struct SimplifyOptions {
    /// The most passes over the program (`--max-simplifier-iterations`,
    /// default 4). Zero disables the simplifier.
    pub max_iterations: u32,
    /// Fold a decision on a computed-argument constructor application by
    /// rebinding its arguments where the construction stood
    /// (`known-constructor`'s computed-argument half).
    pub known_constructor_computed: bool,
    /// Replace a record field projection of a constructor application by the
    /// field itself (`projection`).
    pub projection: bool,
    /// Resolve a projection from a known static dictionary to a direct
    /// call (`direct-dict-call`).
    pub direct_dict_calls: bool,
    /// Merge a partial application applied exactly once into one saturated
    /// direct call (`saturate-partial`, `--no-saturate-partial` turns it
    /// off).
    pub saturate_partial: bool,
    /// Push an outer decision into the alternatives of the decision it
    /// scrutinizes (`case-of-case`, `--case-of-case`). Off by default: on
    /// the self-build corpus it grows Core instead of shrinking it (see the
    /// module documentation for the measurement).
    pub case_of_case: bool,
    /// The node count up to which case-of-case duplicates an alternative
    /// rather than sharing it through a join point. A decision the rule
    /// would duplicate at this size or below is copied; a larger one is
    /// lifted into a join and reached by a jump, so its code exists once.
    pub join_threshold: usize,
    /// Inline a call at the one site that reaches it (`inline`).
    pub inline: bool,
    /// Which inlining decision the pass uses ([`InlineModel`]).
    pub inline_model: InlineModel,
    /// [`InlineModel::Benefit`]'s floor: a body whose code size
    /// ([`code_size`]) is at most this is inlined at every site with no
    /// site test. Flambda 2's `small_function_size`, which makes a
    /// function's declaration `Always_inline`.
    pub inline_small_size: usize,
    /// [`InlineModel::Benefit`]'s ceiling: a body larger than this is never
    /// considered, so no site test runs for it. Flambda 2's
    /// `large_function_size`.
    pub inline_large_size: usize,
    /// [`InlineModel::Benefit`]'s site test: the most a body's duplication
    /// may grow the program by, in code-size units, after the site's
    /// benefit. Flambda 2's `threshold`.
    pub inline_size_threshold: usize,
    /// [`InlineModel::Benefit`]'s discount for a call site inside a loop
    /// body (the caller is a tail worker or a member of a recursive
    /// group): the site test's allowance is divided by this, because a
    /// copy in a loop body is entered on every iteration.
    pub inline_loop_discount: usize,
    /// The most a copy may add at a site, on the model's scale (`weighted`
    /// tenths of a node), after the site's discount. GHC's
    /// `-funfolding-use-threshold`, on the same scale.
    pub inline_use_threshold: usize,
    /// The most a body may weigh and still be admitted to the table at all.
    /// GHC's `-funfolding-creation-threshold`: beyond it a body is not
    /// considered, so no site test runs for it.
    pub inline_creation_threshold: usize,
    /// The node cap of a body with no unit call and no decision that the
    /// class copies to every site. A certain copy is not size tested at the
    /// site, so this is the only bound on it.
    pub inline_small: usize,
    /// The node cap of an unfoldable body that applies another unfoldable
    /// unit, whose copy carries a call the recursive rule then expands.
    pub inline_unfold: usize,
    /// The node cap of an unfoldable body with a decision in it. A body at
    /// or below `inline_unfold_small` is copied to every site (a compare and
    /// a branch cost no more than the call); a larger one only where the
    /// scrutinized argument is a known constructor, so the copy folds rather
    /// than duplicating the branch.
    pub inline_unfold_case: usize,
    /// The node count up to which a decision-bearing body needs no fold
    /// test at its site.
    pub inline_unfold_small: usize,
    /// The most links one unfolding may close. A call to a unit already on
    /// the inline stack is refused regardless, so this bounds the cost of a
    /// long chain, not its termination.
    pub inline_depth: usize,
    /// Run the occurrence analysis and its two unconditional rules: a
    /// binder read once in the spine is substituted whatever its size, and
    /// one read never is dropped when its value is harmless to discard.
    pub occurrence: bool,
}

impl Default for SimplifyOptions {
    fn default() -> Self {
        SimplifyOptions {
            max_iterations: 4,
            direct_dict_calls: true,
            // Off by default, and the reason is measured rather than
            // suspected. The fold is sound (see its documentation) and on the
            // compiler's own bundle it removes 5,420 Core nodes over 2,093
            // firings, but it changes what the inliner's benefit-and-size model
            // sees, and two units then explode:
            //
            // | arm | Core lines | Bootstrap.Type.read-carrier-tree |
            // | off | 1,656,750  | 1,519 |
            // | on  | 1,728,431  | 57,719 |
            //
            // `Bootstrap.Ast.default-expression` goes from 1,598 to 37,538 the
            // same way. The interaction is not a bug in either rule: the fold
            // deletes the constructor a body's decision reads, so the body
            // stops being decision-bearing at the site test, which admits a
            // copy the inliner refused before. That is exactly the "two rules
            // fighting" case, and until the inliner's credit accounts for it the
            // rule stays off. Turn it on with `--known-ctor-computed`.
            known_constructor_computed: false,
            projection: true,
            saturate_partial: true,
            // Off by default: measured on the compiler's own bundle it grows
            // Core by about a tenth (3.22M nodes to 3.54M) even with the
            // constructor fold that collapses the decisions it produces, and
            // two units reach tens of thousands of lines from duplicated
            // alternatives. Turn it on with `--case-of-case` to measure it.
            case_of_case: false,
            join_threshold: 20,
            inline: true,
            inline_model: InlineModel::Benefit,
            inline_small_size: 10,
            inline_large_size: 50,
            inline_size_threshold: 10,
            inline_loop_discount: 4,
            inline_use_threshold: 90,
            inline_creation_threshold: 750,
            inline_small: 12,
            inline_unfold: 40,
            inline_unfold_case: 24,
            inline_unfold_small: 8,
            inline_depth: 6,
            occurrence: true,
        }
    }
}

impl SimplifyOptions {
    /// The defaults, at the given iteration cap; zero disables the pass.
    pub fn with_iterations(max_iterations: u32) -> Self {
        SimplifyOptions { max_iterations, ..SimplifyOptions::default() }
    }
}

/// Static facts the rules read, built once per program: they name units
/// the simplifier never adds to or removes, so they stay valid for the
/// whole run.
struct Tables {
    /// Constructor tag and field count by unit name.
    ctors: HashMap<String, (usize, usize)>,
    /// Parameter count by unit name.
    arity: HashMap<String, usize>,
    /// Instances: subdictionary count and the method unit per physical
    /// slot.
    instances: HashMap<String, (usize, Vec<GlobalId>)>,
}

impl Tables {
    fn new(program: &Program) -> Tables {
        let mut tables =
            Tables { ctors: HashMap::new(), arity: HashMap::new(), instances: HashMap::new() };
        for f in &program.fns {
            tables.arity.insert(f.id.0.clone(), f.params.len());
            if let Some((tag, fields)) = f.ctor {
                tables.ctors.insert(f.id.0.clone(), (tag, fields));
            }
        }
        for inst in &program.instances {
            tables
                .instances
                .insert(inst.id.0.clone(), (inst.subdict_count, inst.methods.clone()));
        }
        tables
    }

    /// Add the units a foreign body table defines. A foreign unit has no
    /// `InstanceDef` of its own here: an artifact carries bodies, not the
    /// dictionary tables, so only its arity and constructor fact matter to
    /// the admission rule and to a saturated copy.
    fn extend(&mut self, foreign: &[FnDef]) {
        for f in foreign {
            self.arity.insert(f.id.0.clone(), f.params.len());
            if let Some((tag, fields)) = f.ctor {
                self.ctors.insert(f.id.0.clone(), (tag, fields));
            }
        }
    }
}

/// What the pass knows about the locals in scope: the alias chains the
/// checker's lowering introduces (`let y = x in ...`, which the match
/// compiler produces around a fallthrough decision) and the locals bound
/// to a constructor application. Both are per unit and pushed and popped
/// with the `let` that introduced them.
#[derive(Default)]
struct Scope {
    /// `LocalId` to the local it aliases.
    alias: HashMap<LocalId, LocalId>,
    /// A resolved local to the constructor application it is bound to.
    ctor: HashMap<LocalId, Core>,
}

impl Scope {
    /// The local an alias chain ends at. The chain a lowered body builds is
    /// one or two links deep; the bound keeps a malformed cycle from
    /// hanging the pass.
    fn root_of(&self, mut l: LocalId) -> LocalId {
        for _ in 0..16 {
            match self.alias.get(&l) {
                Some(next) if *next != l => l = *next,
                _ => break,
            }
        }
        l
    }

    /// The constructor application a local's value is, through aliases.
    fn ctor_of(&self, l: LocalId) -> Option<&Core> {
        self.ctor.get(&self.root_of(l))
    }
}

/// A value in the sense the rules need: evaluating it cannot run an
/// effect, cannot allocate a memo cell, and cannot read one. A
/// constructor application whose arguments are all values is a value too
/// (its block allocation is unobservable), which is what makes the
/// known-constructor substitution safe in a strict language.
fn is_value(e: &Core, tables: &Tables) -> bool {
    match e {
        Core::Int(_)
        | Core::ConstBig { .. }
        | Core::ConstBytes(_)
        | Core::ConstString { .. }
        | Core::ConstSym(_)
        | Core::Bool(_)
        | Core::Unit
        | Core::Var(_)
        | Core::Global(..)
        | Core::Ctor(..)
        | Core::Prim(_)
        | Core::Foreign(_)
        | Core::SpliceVal(_)
        | Core::StrLit(_)
        | Core::SyntaxLit(..) => true,
        Core::App(head, args) => {
            matches!(&**head, Core::Ctor(id, _) if tables.ctors.contains_key(&id.0))
                && args.iter().all(|a| is_value(a, tables))
        }
        _ => false,
    }
}


/// The constructor a value is, when it is one and every argument is a
/// value. `App` carries at least one argument, so a nullary constructor
/// is the bare `Ctor` node.
fn ctor_app(e: &Core, tables: &Tables) -> Option<usize> {
    match e {
        Core::App(head, args) => match &**head {
            Core::Ctor(id, _) => {
                let (tag, _) = *tables.ctors.get(&id.0)?;
                args.iter().all(|a| is_value(a, tables)).then_some(tag)
            }
            _ => None,
        },
        Core::Ctor(id, _) => {
            let (tag, 0) = *tables.ctors.get(&id.0)? else { return None };
            Some(tag)
        }
        _ => None,
    }
}

/// The arguments of a constructor application, empty for a nullary one.
fn ctor_args(e: &Core) -> Vec<Core> {
    match e {
        Core::App(_, args) => args.clone(),
        _ => Vec::new(),
    }
}

/// Substitute `from` with `to` throughout `e`. Local ids are unique in a
/// unit, so no binder shadows `from`; the substitution is capture free.
///
/// The walk visits every child position, including an application's head:
/// `child_slots` leaves the head out (an application's head is a unit name
/// in almost every body), and a projection or case binder substituted into
/// closure position would otherwise stay behind as an unbound local. A
/// quotation's retained body is not walked: it is a separate binding
/// scope, and [`known_constructor`] refuses a body that carries one.
fn subst(e: &mut Core, from: LocalId, to: &Core) {
    match e {
        Core::Var(l) if *l == from => *e = to.clone(),
        Core::App(head, args) => {
            subst(head, from, to);
            for a in args {
                subst(a, from, to);
            }
        }
        Core::DeclsQuote { holes, .. } => {
            for h in holes {
                subst(h, from, to);
            }
        }
        _ => {
            for slot in child_slots(e) {
                subst(slot, from, to);
            }
        }
    }
}

/// Visit every child of `e`, application heads and quotation holes
/// included; a quotation's retained body counts as a child for the
/// mention test (a binder it reads must not be dropped).
fn visit_children<'a>(e: &'a Core, f: &mut impl FnMut(&'a Core)) {
    match e {
        Core::App(head, args) => {
            f(head);
            for a in args {
                f(a);
            }
        }
        Core::Quote { holes, body, .. } => {
            for h in holes {
                f(h);
            }
            if let Some(body) = body {
                f(body);
            }
        }
        _ => {
            for child in child_slots_imm(e) {
                f(child);
            }
        }
    }
}

/// Whether the term mentions `l`.
fn mentions(e: &Core, l: LocalId) -> bool {
    if matches!(e, Core::Var(x) if *x == l) {
        return true;
    }
    let mut found = false;
    visit_children(e, &mut |child| {
        if !found && mentions(child, l) {
            found = true;
        }
    });
    found
}

// ------------------------------------------------- occurrence analysis
//
// GHC decides whether a binding is worth keeping before it decides on size:
// a binder read exactly once is substituted whatever the size of its right
// hand side, and a binder read never is dropped when dropping it discards
// nothing observable. Those two rules are what make a chain of one-shot
// helpers vanish without any size heuristic, and they are the half of
// GHC's simplifier this pass was missing (OccurAnal's `OnceAndOnly`, and
// `preInlineUnconditionally` in the simplifier).
//
// The analysis is one walk per unit per pass, recomputed because a rewrite
// can change a count. A stale count can only make the pass do less, except
// for the one hazard that matters in a strict language: a substitution
// moves the right hand side's work to its read, so a read under a branch
// that may not be taken would drop the work instead of moving it. That is
// why the rule also requires the read to be in the spine, and why a read
// inside a join body (reached once per jump) is excluded.

/// One binder's occurrence in a unit body.
#[derive(Clone, Copy, Default)]
struct Occ {
    /// Reads of the binder.
    uses: usize,
    /// A read inside a join body, reached once per jump.
    in_join: bool,
}

/// Count every binder's reads in `e`, including the reads a count operation
/// names directly rather than as a child term. The walk covers every child
/// position, an application's head and a quotation's retained body included:
/// missing a read makes a live binder look dead, and the dead-binder rule
/// then removes it.
fn occurrences(e: &Core, in_join: bool, out: &mut HashMap<LocalId, Occ>) {
    let read = |x: &LocalId, out: &mut HashMap<LocalId, Occ>| {
        let entry = out.entry(*x).or_default();
        entry.uses += 1;
        entry.in_join |= in_join;
    };
    match e {
        Core::Var(x) => {
            read(x, out);
            return;
        }
        Core::Dup(x, _, body) | Core::Drop(x, _, body) => {
            read(x, out);
            occurrences(body, in_join, out);
            return;
        }
        Core::Release { block, moved, rest, token, body } => {
            read(block, out);
            for (x, _) in moved.iter().chain(rest.iter()) {
                read(x, out);
            }
            if let Some(t) = token {
                read(t, out);
            }
            occurrences(body, in_join, out);
            return;
        }
        Core::Reuse { token, args, .. } => {
            read(token, out);
            for a in args {
                occurrences(a, in_join, out);
            }
            return;
        }
        Core::App(head, args) => {
            occurrences(head, in_join, out);
            for a in args {
                occurrences(a, in_join, out);
            }
            return;
        }
        Core::Join { body, rest, .. } => {
            occurrences(body, true, out);
            occurrences(rest, in_join, out);
            return;
        }
        Core::Quote { holes, body, .. } => {
            for h in holes {
                occurrences(h, in_join, out);
            }
            if let Some(body) = body {
                occurrences(body, true, out);
            }
            return;
        }
        Core::DeclsQuote { holes, .. } => {
            for h in holes {
                occurrences(h, in_join, out);
            }
            return;
        }
        _ => {}
    }
    for child in child_slots_imm(e) {
        occurrences(child, in_join, out);
    }
}

/// Whether every read of `l` in `e` is on the evaluator's spine: not inside
/// a decision's alternative, an `If` branch, a join body or a retained
/// quotation, any of which may be skipped or repeated. The walk covers the
/// same positions as [`occurrences`], so a read can never be counted as
/// spine-free by one and missed by the other.
fn spine_only(e: &Core, l: LocalId, spine: bool) -> bool {
    match e {
        Core::Var(x) => *x != l || spine,
        Core::Case { scrut, alts } => {
            spine_only(scrut, l, spine) && alts.iter().all(|a| spine_only(&a.body, l, false))
        }
        Core::If(condition, then, otherwise) => {
            spine_only(condition, l, spine)
                && spine_only(&then.body, l, false)
                && spine_only(&otherwise.body, l, false)
        }
        Core::Join { body, rest, .. } => spine_only(body, l, false) && spine_only(rest, l, spine),
        Core::App(head, args) => {
            spine_only(head, l, spine) && args.iter().all(|a| spine_only(a, l, spine))
        }
        Core::Quote { holes, body, .. } => {
            holes.iter().all(|h| spine_only(h, l, spine))
                && body.as_deref().is_none_or(|b| spine_only(b, l, false))
        }
        Core::DeclsQuote { holes, .. } => holes.iter().all(|h| spine_only(h, l, spine)),
        _ => child_slots_imm(e).into_iter().all(|child| spine_only(child, l, spine)),
    }
}

/// Whether discarding an expression discards nothing observable: a value,
/// or a field read from one. A count operation is not here (it changes the
/// count), and neither is a dictionary selection, which hands out an owned
/// reference.
fn speculatable(e: &Core, tables: &Tables) -> bool {
    match e {
        Core::Proj(inner, _) => speculatable(inner, tables) || is_value(inner, tables),
        _ => is_value(e, tables),
    }
}

/// Whether the term carries a quotation, whose retained body is a binding
/// scope of its own.
fn carries_quote(e: &Core) -> bool {
    if matches!(e, Core::Quote { .. } | Core::DeclsQuote { .. }) {
        return true;
    }
    let mut found = false;
    visit_children(e, &mut |child| {
        if !found && carries_quote(child) {
            found = true;
        }
    });
    found
}

#[derive(Clone, Copy)]
enum Rule {
    CaseMerge,
    KnownConstructor,
    DirectDictCall,
    /// A partial application applied exactly once, merged into one
    /// saturated call (`saturate-partial`).
    SaturatePartial,
    CaseOfCase,
    Inline,
    /// A decision on a computed-argument constructor application, folded by
    /// rebinding the arguments where the construction stood.
    KnownConstructorComputed,
    /// A record field projection of a constructor application, replaced by
    /// the field.
    Projection,
    /// A binding whose one read is in the spine, substituted whatever its
    /// size (GHC's `preInlineUnconditionally`).
    PreInline,
    /// A binding nothing reads and whose value is harmless to discard
    /// (GHC's dead-binder drop).
    DeadBinder,
}

/// The accumulator every pass shares: the tick total, the per-rule
/// counts, and whether the pass changed anything.
#[derive(Default)]
struct Ticks {
    ticks: usize,
    case_merge: usize,
    known_ctor: usize,
    known_ctor_computed: usize,
    projection: usize,
    dict_call: usize,
    saturate: usize,
    case_of_case: usize,
    joins: usize,
    duplicated: usize,
    inline: usize,
    pre_inline: usize,
    dead_binder: usize,
    changed: bool,
}

impl Ticks {
    fn fire(&mut self, rule: Rule) {
        self.ticks += 1;
        self.changed = true;
        match rule {
            Rule::CaseMerge => self.case_merge += 1,
            Rule::KnownConstructor => self.known_ctor += 1,
            Rule::KnownConstructorComputed => self.known_ctor_computed += 1,
            Rule::Projection => self.projection += 1,
            Rule::DirectDictCall => self.dict_call += 1,
            Rule::SaturatePartial => self.saturate += 1,
            Rule::CaseOfCase => self.case_of_case += 1,
            Rule::Inline => self.inline += 1,
            Rule::PreInline => self.pre_inline += 1,
            Rule::DeadBinder => self.dead_binder += 1,
        }
    }
}

/// One pass over a unit: children first, then the node's own rewrites.
/// `scope` maps a local to the constructor application a surrounding
/// `let` bound it to, which is the only form of scrutinee knowledge the
/// pass uses (the analysis-free version of GHC's occurrence information).
fn pass(
    e: &mut Core,
    scope: &mut Scope,
    tables: &Tables,
    classes: &HashMap<LocalId, Class>,
    occ: &HashMap<LocalId, Occ>,
    opts: &SimplifyOptions,
    fresh: &mut Fresh,
    t: &mut Ticks,
) {
    // Children first: a rewrite below can expose a rewrite above it in the
    // same pass, and the reverse order would need another pass for the
    // same result.
    match e {
        Core::Let(l, _, value, body) => {
            pass(value, scope, tables, classes, occ, opts, fresh, t);
            let alias = match &**value {
                Core::Var(w) => Some(scope.root_of(*w)),
                _ => None,
            };
            let bound = match alias {
                Some(_) => None,
                None => ctor_app(value, tables).map(|_| (**value).clone()),
            };
            let is_bound = bound.is_some();
            if let Some(w) = alias {
                scope.alias.insert(*l, w);
            }
            if let Some(v) = bound {
                scope.ctor.insert(*l, v);
            }
            pass(body, scope, tables, classes, occ, opts, fresh, t);
            if alias.is_some() {
                scope.alias.remove(&*l);
            }
            if is_bound {
                scope.ctor.remove(&*l);
            }
        }
        _ => {
            for slot in child_slots(e) {
                pass(slot, scope, tables, classes, occ, opts, fresh, t);
            }
        }
    }
    // A node may admit more than one rewrite; re-examine it after each
    // firing so a chain resolves without waiting for the next pass, up to
    // a small bound in case two rules undo each other.
    for _ in 0..8 {
        let fired = if known_constructor(e, scope, tables) {
            Some(Rule::KnownConstructor)
        } else if opts.known_constructor_computed && known_constructor_computed(e, tables, classes, fresh) {
            Some(Rule::KnownConstructorComputed)
        } else if opts.projection && projection(e, tables) {
            Some(Rule::Projection)
        } else if case_merge(e, scope) {
            Some(Rule::CaseMerge)
        } else if opts.direct_dict_calls && direct_dict_call(e, tables) {
            Some(Rule::DirectDictCall)
        } else if opts.saturate_partial && saturate_partial(e, occ, tables) {
            Some(Rule::SaturatePartial)
        } else if opts.case_of_case && case_of_case(e, classes, opts, fresh, t) {
            Some(Rule::CaseOfCase)
        } else {
            None
        };
        match fired {
            Some(rule) => t.fire(rule),
            None => break,
        }
    }
    // The occurrence rules, after the node's own rewrites so a count reflects
    // what the rewrites left (design/inline-unfolding-policy.md section 9).
    // With the analysis off there is no count, and a missing count must not
    // read as "every binder is dead".
    //
    // A binder nothing reads, whose value is harmless to discard, goes. That
    // is what removes the allocation an abandoned decision left behind, and
    // it is the same rule GHC states as the dead-binder drop.
    if let Core::Let(l, _, value, body) = &*e {
        let count = occ.get(l).copied().unwrap_or_default();
        if !opts.occurrence {
            return;
        }
        if count.uses == 0 && speculatable(value, tables) {
            let body = (**body).clone();
            *e = body;
            t.fire(Rule::DeadBinder);
            return;
        }
        // A binder read exactly once, in the spine, is substituted whatever
        // the size of its right hand side. The spine condition is the strict
        // language's addition to GHC's rule: the work moves to the read, so a
        // read under a branch that may not be taken would drop it. A join
        // body is excluded for the same reason: it runs once per jump.
        // The right hand side must be worth moving: a value, or a load from
        // one. GHC substitutes any RHS here because in a lazy language the
        // substitution cannot lose work; in a strict one it moves the work,
        // so a memo cell or a call is left where it is.
        if count.uses == 1
            && !count.in_join
            && speculatable(value, tables)
            && spine_only(body, *l, true)
        {
            let (l, value) = (*l, (**value).clone());
            let mut body = (**body).clone();
            subst(&mut body, l, &value);
            *e = body;
            t.fire(Rule::PreInline);
        }
    }
}

/// The fresh-name supply of one simplifier run: join ids and locals. A
/// local id must exceed every local the unit mentions, and the counter is
/// program wide so a copy made in one unit can never collide with another
/// unit's binder either.
struct Fresh {
    local: usize,
    join: usize,
}

impl Fresh {
    fn local(&mut self) -> LocalId {
        let id = LocalId(self.local);
        self.local += 1;
        id
    }

    fn join(&mut self) -> JoinId {
        let id = JoinId(self.join);
        self.join += 1;
        id
    }
}

/// The node count of a term, the size the duplication threshold is in.
fn nodes(e: &Core) -> usize {
    let mut n = 1;
    visit_children(e, &mut |child| n += nodes(child));
    n
}

/// The cost of a copy, on the scale the thresholds are stated in (tenths of
/// a node; design/inline-unfolding-policy.md section 10). An allocation is
/// the expensive thing a copy can duplicate, so it is priced six times a
/// call; a zero-field constructor is a static block and costs nothing.
fn weighted(e: &Core, tables: &Tables) -> usize {
    let here = match e {
        Core::Prim(_) => 10,
        Core::Foreign(_) => 20,
        Core::Var(_)
        | Core::Int(_)
        | Core::Bool(_)
        | Core::Unit
        | Core::ConstBig { .. }
        | Core::ConstBytes(_)
        | Core::ConstString { .. }
        | Core::ConstSym(_)
        | Core::StrLit(_) => 0,
        Core::Proj(..) | Core::DictProj(..) => 10,
        Core::Lam { .. } => 30,
        Core::Case { alts, .. } => alts.len() * 10,
        Core::If(..) => 20,
        Core::Global(..) => 10,
        Core::Ctor(id, _) => {
            if tables.ctors.get(&id.0).is_some_and(|(_, fields)| *fields == 0) {
                0
            } else {
                60
            }
        }
        _ => 0,
    };
    let mut total = here;
    if let Core::App(head, args) = e {
        total += weighted(head, tables);
        for a in args {
            total += weighted(a, tables);
        }
        return total;
    }
    for child in child_slots_imm(e) {
        total += weighted(child, tables);
    }
    total
}

/// Whether a call's argument makes the site more worth inlining into, GHC's
/// `ValueArg` shapes we have: a constructor application, a literal, a
/// lambda, a unit whose body is a value, a primop.
fn interesting_arg(e: &Core, tables: &Tables) -> bool {
    match e {
        Core::Int(_)
        | Core::Bool(_)
        | Core::Unit
        | Core::ConstBig { .. }
        | Core::ConstBytes(_)
        | Core::ConstString { .. }
        | Core::ConstSym(_)
        | Core::StrLit(_)
        | Core::Lam { .. }
        | Core::Prim(_) => true,
        Core::Ctor(..) => true,
        Core::Global(id, _) => tables.ctors.contains_key(&id.0),
        _ => matches!(e, Core::App(head, _) if matches!(&**head, Core::Ctor(..))),
    }
}

/// Whether an argument is trivial, which is what makes a call cheap to leave
/// in place: a variable, a literal, or a static block.
fn trivial_arg(e: &Core, tables: &Tables) -> bool {
    matches!(e, Core::Var(_)) || weighted(e, tables) == 0
}

/// A class with an open parameter resolved to `Raw`: the convention a
/// count operation is decided by (`crate::rc`).
fn concrete(c: Class) -> Class {
    if c.is_param() {
        Class::Raw
    } else {
        c
    }
}

/// Every binder's class in a unit: its parameters', and the `let`, pattern
/// and join-parameter binder classes. The `case-of-case` rule needs it to
/// give the join points it makes parameters of the right class, because
/// `crate::rc` decides counted-ness from the class alone.
fn unit_classes(f: &FnDef) -> HashMap<LocalId, Class> {
    fn walk(e: &Core, out: &mut HashMap<LocalId, Class>) {
        match e {
            Core::Let(x, c, v, body) => {
                out.insert(*x, concrete(*c));
                walk(v, out);
                walk(body, out);
            }
            Core::Case { scrut, alts } => {
                walk(scrut, out);
                for a in alts {
                    match &a.pat {
                        CPat::Var(x, c) => {
                            out.insert(*x, concrete(*c));
                        }
                        CPat::Con { fields, classes, .. } => {
                            for (i, x) in fields.iter().enumerate() {
                                out.insert(*x, concrete(classes.get(i).copied().unwrap_or(Class::Raw)));
                            }
                        }
                        CPat::Wild => {}
                    }
                    walk(&a.body, out);
                }
            }
            Core::Join { params, body, rest, .. } => {
                for (x, c) in params {
                    out.insert(*x, concrete(*c));
                }
                walk(body, out);
                walk(rest, out);
            }
            _ => {
                for k in child_slots_imm(e) {
                    walk(k, out);
                }
            }
        }
    }
    let mut out: HashMap<LocalId, Class> = HashMap::new();
    for i in 0..f.arity() {
        let c = f.repr.params.get(i).copied().map_or(Class::Raw, concrete);
        out.insert(LocalId(i), c);
    }
    walk(&f.body, &mut out);
    out
}

/// The locals a term reads from outside it, in first-mention order: every
/// `Var`, and every binder a count, cell or `release` operation names,
/// whose binder is not inside the term. These are what a join point takes
/// as parameters: the body then owns them, which is the only form
/// `crate::rc` can balance, because the enclosing code does not run again
/// after the jump.
fn free_locals(e: &Core, out: &mut Vec<LocalId>) {
    free_locals_in(e, out, false);
}

/// [`free_locals`] with a switch for application heads. A head is a child of
/// the node ([`subst`] walks it), but `child_slots` leaves it out because it
/// is a unit name in almost every body; a copy's closure check needs every
/// local an application could evaluate, head included.
fn free_locals_in(e: &Core, out: &mut Vec<LocalId>, heads: bool) {
    let mut bound: HashSet<LocalId> = HashSet::new();
    fn go(e: &Core, bound: &mut HashSet<LocalId>, out: &mut Vec<LocalId>, heads: bool) {
        let use_local = |x: LocalId, bound: &HashSet<LocalId>, out: &mut Vec<LocalId>| {
            if !bound.contains(&x) && !out.contains(&x) {
                out.push(x);
            }
        };
        match e {
            Core::Var(x) => use_local(*x, bound, out),
            Core::App(h, args) if heads => {
                go(h, bound, out, heads);
                for a in args {
                    go(a, bound, out, heads);
                }
            }
            Core::Let(x, _, v, body) => {
                go(v, bound, out, heads);
                bound.insert(*x);
                go(body, bound, out, heads);
                bound.remove(x);
            }
            Core::Case { scrut, alts } => {
                go(scrut, bound, out, heads);
                for a in alts {
                    let binders: Vec<LocalId> = pat_binders(&a.pat).iter().map(|(l, _)| *l).collect();
                    for b in &binders {
                        bound.insert(*b);
                    }
                    go(&a.body, bound, out, heads);
                    for b in &binders {
                        bound.remove(b);
                    }
                }
            }
            Core::Join { params, body, rest, .. } => {
                for (p, _) in params {
                    bound.insert(*p);
                }
                go(body, bound, out, heads);
                for (p, _) in params {
                    bound.remove(p);
                }
                go(rest, bound, out, heads);
            }
            Core::Dup(x, _, body) | Core::Drop(x, _, body) => {
                use_local(*x, bound, out);
                go(body, bound, out, heads);
            }
            Core::Release { block, moved, rest, token, body } => {
                use_local(*block, bound, out);
                for (l, _) in moved.iter().chain(rest) {
                    use_local(*l, bound, out);
                }
                if let Some(t) = token {
                    use_local(*t, bound, out);
                }
                go(body, bound, out, heads);
            }
            Core::Reuse { token, args, .. } => {
                use_local(*token, bound, out);
                for a in args {
                    go(a, bound, out, heads);
                }
            }
            Core::BoxInit(l, v, _) | Core::LazyInit(l, v) | Core::RefSet(l, v, _) => {
                go(v, bound, out, heads);
                use_local(*l, bound, out);
            }
            _ => {
                for k in child_slots_imm(e) {
                    go(k, bound, out, heads);
                }
            }
        }
    }
    go(e, &mut bound, out, heads);
}

/// Whether the term carries a join point or a jump. `case-of-case` neither
/// duplicates nor lifts such a body: a copy would share the join's id, and
/// `crate::rc`'s join convention places the body's releases before its
/// value, which a jump as the value would bypass.
fn carries_join(e: &Core) -> bool {
    if matches!(e, Core::Join { .. } | Core::Jump { .. }) {
        return true;
    }
    let mut found = false;
    visit_children(e, &mut |child| {
        if !found && carries_join(child) {
            found = true;
        }
    });
    found
}

/// The locals a pattern binds, with their classes: a constructor's fields
/// or a variable alternative's scrutinee binder. A wildcard binds none.
fn pat_binders(p: &CPat) -> Vec<(LocalId, Class)> {
    match p {
        CPat::Var(x, c) => vec![(*x, *c)],
        CPat::Con { fields, classes, .. } => fields
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, classes.get(i).copied().unwrap_or(Class::Raw)))
            .collect(),
        CPat::Wild => Vec::new(),
    }
}

/// Rename a pattern's binders to fresh locals, returning this copy's binder
/// list. The body must be renamed with the same pairs.
fn rename_pat(pat: &mut CPat, fresh: &mut Fresh, renames: &mut Vec<(LocalId, LocalId)>) {
    let mut fresh_for = |x: &mut LocalId, fresh: &mut Fresh, renames: &mut Vec<(LocalId, LocalId)>| {
        let n = fresh.local();
        renames.push((*x, n));
        *x = n;
    };
    match pat {
        CPat::Var(x, _) => fresh_for(x, fresh, renames),
        CPat::Con { fields, .. } => {
            for f in fields.iter_mut() {
                fresh_for(f, fresh, renames);
            }
        }
        CPat::Wild => {}
    }
}

/// `case (case s of alts) of alts2` pushes `alts2` into every alternative of
/// the inner decision: `case s of A -> e; B -> f` becomes
/// `case s of A -> case e of alts2; B -> case f of alts2`, so the value the
/// inner decision produced is scrutinized where it is produced instead of
/// being handed to a decision of its own.
///
/// An outer alternative is duplicated once per inner alternative, so the
/// rule is paired with join points: an alternative whose body is larger than
/// [`SimplifyOptions::join_threshold`] nodes is lifted into a `Join` over its
/// own pattern binders (its only free locals the join's position does not
/// already have), and every copy becomes a `Jump` to it. A smaller body is
/// copied, with its pattern binders renamed to fresh locals per copy, and the
/// copied node count is reported as `duplicated`.
///
/// It refuses when the inner decision carries weights (pushing moves its
/// alternatives into outer branches, where its weight space has no meaning,
/// and the driving pass owns the weighted forms), when any alternative of
/// either decision carries a quotation (a quotation's retained body is a
/// binding scope of its own, and the copies would share it), when the inner
/// scrutinee is used inside either decision's alternatives (the usual GHC
/// condition: the scrutinee is re-read at every pushed copy, and a use of it
/// there makes the two reads of one value visible to each other), and when
/// the inner scrutinee is not a local (a computed scrutinee would be
/// evaluated again per copy).
fn case_of_case(
    e: &mut Core,
    classes: &HashMap<LocalId, Class>,
    opts: &SimplifyOptions,
    fresh: &mut Fresh,
    t: &mut Ticks,
) -> bool {
    // The two shapes the rule takes: a decision whose scrutinee is a
    // decision, and the `let` the checker's lowering produces for one (the
    // value is bound, then scrutinized), which is the shape that actually
    // appears in our Core.
    let mut prefix: Vec<(LocalId, Class, Core)> = Vec::new();
    let (inner_scrut, inner_alts, outer_alts) = match &*e {
        Core::Case { scrut, alts } => match &**scrut {
            Core::Case { scrut: s, alts: inner } => ((**s).clone(), inner.clone(), alts.clone()),
            _ => return false,
        },
        Core::Let(x, _, value, body) => {
            let Core::Case { scrut: v, alts: outer } = &**body else { return false };
            if !matches!(&**v, Core::Var(y) if y == x) {
                return false;
            }
            // The local is only the scrutinee: a second use would mean the
            // decision's value is shared, and pushing would duplicate it.
            if uses_of(body, *x) != 1 {
                return false;
            }
            // The value may open with `let`s: the computation, and the
            // alias the lowering puts before a decision on a parameter.
            // Both belong outside the pushed decision, because a strict
            // language evaluates them either way, and neither can name a
            // binder of the outer alternatives.
            let mut rest = &**value;
            while let Core::Let(y, c, v, body) = rest {
                prefix.push((*y, *c, (**v).clone()));
                rest = body;
            }
            let Core::Case { scrut: s, alts: inner } = rest else { return false };
            ((**s).clone(), inner.clone(), outer.clone())
        }
        _ => return false,
    };
    let Core::Var(s) = &inner_scrut else { return false };
    let s = *s;
    if inner_alts.is_empty() || outer_alts.is_empty() {
        return false;
    }
    if inner_alts.iter().any(|a| a.weight.is_some()) {
        return false;
    }
    if inner_alts.iter().any(|a| carries_quote(&a.body)) || outer_alts.iter().any(|a| carries_quote(&a.body))
    {
        return false;
    }
    if inner_alts.iter().any(|a| mentions(&a.body, s)) || outer_alts.iter().any(|a| mentions(&a.body, s)) {
        return false;
    }
    // A body that already carries a join point is neither copied nor lifted:
    // a copy would share the join's id, and a lifted body's releases assume
    // its value is reached by falling through, which a jump is not.
    if outer_alts.iter().any(|a| carries_join(&a.body)) {
        return false;
    }
    // The alternatives of the outer decision, and whether each is shared
    // through a join. Nothing is duplicated when the inner decision has one
    // alternative, so no join pays there. A shared body takes its free
    // locals as parameters: the jump hands one reference per parameter over,
    // so the body owns what it reads, which is the only shape `crate::rc`
    // can balance (nothing in the enclosing code runs after a jump).
    let mut joins: Vec<(JoinId, Vec<(LocalId, Class)>, Core)> = Vec::new();
    let mut shared: Vec<(Option<JoinId>, Vec<(LocalId, Class)>)> = Vec::with_capacity(outer_alts.len());
    for a in outer_alts.iter() {
        if inner_alts.len() > 1 && nodes(&a.body) > opts.join_threshold {
            // A join's parameters are the alternative's own pattern
            // binders. A body that reads an enclosing local too cannot be
            // shared: no jump can hand that value over without the enclosing
            // scope losing the release it still owes, so such a body is
            // never lifted (the rule refuses when it is too big to copy).
            let mut free: Vec<LocalId> = Vec::new();
            free_locals(&a.body, &mut free);
            let binders: Vec<LocalId> = pat_binders(&a.pat).iter().map(|(l, _)| *l).collect();
            if free.iter().any(|l| !binders.contains(l)) {
                return false;
            }
            let id = fresh.join();
            let params: Vec<(LocalId, Class)> = binders
                .into_iter()
                .map(|l| (l, concrete(classes.get(&l).copied().unwrap_or(Class::Raw))))
                .collect();
            joins.push((id, params.clone(), (*a.body).clone()));
            shared.push((Some(id), params));
        } else {
            shared.push((None, Vec::new()));
        }
    }
    let mut pushed: Vec<CAlt<Core>> = Vec::with_capacity(inner_alts.len());
    for (k, ia) in inner_alts.iter().enumerate() {
        let mut copies: Vec<CAlt<Core>> = Vec::with_capacity(outer_alts.len());
        for (a, (join, params)) in outer_alts.iter().zip(&shared) {
            match join {
                Some(id) => {
                    // The copy is a jump that hands this copy's binding of
                    // every free local to the shared body.
                    let mut pat = a.pat.clone();
                    let mut renames: Vec<(LocalId, LocalId)> = Vec::new();
                    rename_pat(&mut pat, fresh, &mut renames);
                    let args = params
                        .iter()
                        .map(|(l, _)| {
                            let copy = renames.iter().find(|(from, _)| from == l).map(|(_, to)| *to);
                            Core::Var(copy.unwrap_or(*l))
                        })
                        .collect();
                    copies.push(CAlt {
                        pat,
                        weight: a.weight,
                        body: Box::new(Core::Jump { id: *id, args }),
                    });
                }
                None if k == 0 => copies.push(a.clone()),
                None => {
                    // A copy: fresh binders, and the body renamed to them.
                    let mut alt = a.clone();
                    let mut renames: Vec<(LocalId, LocalId)> = Vec::new();
                    rename_pat(&mut alt.pat, fresh, &mut renames);
                    for (from, to) in renames {
                        subst(&mut alt.body, from, &Core::Var(to));
                    }
                    t.duplicated += nodes(&a.body);
                    copies.push(alt);
                }
            }
        }
        pushed.push(CAlt {
            pat: ia.pat.clone(),
            weight: ia.weight,
            body: Box::new(Core::Case { scrut: ia.body.clone(), alts: copies }),
        });
    }
    let mut out = Core::Case { scrut: Box::new(Core::Var(s)), alts: pushed };
    // The joins wrap the decision, so their bodies are in the scope the
    // outer decision was in and see the binders of its patterns only
    // through the parameters.
    for (id, params, body) in joins.into_iter().rev() {
        t.joins += 1;
        out = Core::Join { id, params, body: Box::new(body), rest: Box::new(out) };
    }
    for (x, c, v) in prefix.into_iter().rev() {
        out = Core::Let(x, c, Box::new(v), Box::new(out));
    }
    *e = out;
    true
}

/// How many times `e` reads the local `l`, application heads included.
fn uses_of(e: &Core, l: LocalId) -> usize {
    let mut n = usize::from(matches!(e, Core::Var(x) if *x == l));
    visit_children(e, &mut |child| n += uses_of(child, l));
    n
}

/// `case v of C -> e; default -> case v of ...` becomes one decision.
fn case_merge(e: &mut Core, scope: &Scope) -> bool {
    let Core::Case { scrut, alts } = e else { return false };
    let Core::Var(v) = &**scrut else { return false };
    let v = *v;
    let root = scope.root_of(v);
    // The alternatives whose body decides the same local again. All of them
    // must share one inner decision: one default can be expanded into one
    // alternative per uncovered constructor, all with the same body.
    let mut defaults: Vec<usize> = Vec::new();
    let mut shared: Option<(Vec<LocalId>, Vec<CAlt<Core>>)> = None;
    for (i, a) in alts.iter().enumerate() {
        // A `case v of x -> ...` alternative binds the scrutinee under a
        // fresh name, and its body's decision names that binder, not `v`.
        let own = match &a.pat {
            CPat::Var(l, _) => Some(*l),
            _ => None,
        };
        let Some(inner) = inner_case(&a.body, scope, root, own) else { continue };
        match &shared {
            None => shared = Some(inner),
            Some((binders, first)) => {
                let (more, inner_alts) = inner;
                let same = binders.len() == more.len()
                    && first.len() == inner_alts.len()
                    && first.iter().zip(&inner_alts).all(|(x, y)| {
                        x.weight == y.weight && same_pat(&x.pat, &y.pat) && alpha_eq(&x.body, &y.body)
                    });
                if !same {
                    return false;
                }
            }
        }
        defaults.push(i);
    }
    let Some((mut binders, inner)) = shared else { return false };
    if defaults.is_empty() || defaults.len() == alts.len() {
        return false;
    }
    // The default alternatives must come after every explicit one; an
    // explicit alternative after a wildcard is unreachable, and moving it
    // ahead of the inner decision would make it match.
    if let Some(last_default) = defaults.iter().copied().max() {
        if (last_default + 1..alts.len()).any(|i| !defaults.contains(&i)) {
            return false;
        }
    }
    // The outer decision must be keyed on constructor tags (a wildcard
    // default aside), the inner one on tags alone, and neither carries
    // weights: a weighted merge rescales, and the driving pass owns that
    // rule.
    for (i, a) in alts.iter().enumerate() {
        let is_default = defaults.contains(&i);
        match &a.pat {
            CPat::Con { .. } => {}
            CPat::Wild => {
                if !is_default {
                    // An explicit wildcard shadows every alternative below
                    // it, including each one the merge would keep.
                    return false;
                }
            }
            CPat::Var(l, _) => {
                if !is_default {
                    return false;
                }
                // The binder named the scrutinee inside the default body:
                // the kept alternatives name the outer local instead.
                binders.push(*l);
            }
        }
    }
    if inner.iter().any(|a| !matches!(a.pat, CPat::Con { .. })) {
        return false;
    }
    if alts.iter().any(|a| a.weight.is_some()) || inner.iter().any(|a| a.weight.is_some()) {
        return false;
    }
    let tag_of = |p: &CPat| match p {
        CPat::Con { tag, .. } => Some(*tag),
        _ => None,
    };
    let covered: Vec<usize> = alts
        .iter()
        .enumerate()
        .filter(|(i, _)| !defaults.contains(i))
        .filter_map(|(_, a)| tag_of(&a.pat))
        .collect();
    let mut seen: Vec<usize> = Vec::new();
    for a in &inner {
        let Some(tag) = tag_of(&a.pat) else { return false };
        if seen.contains(&tag) {
            return false;
        }
        seen.push(tag);
    }
    let kept: Vec<CAlt<Core>> = inner
        .into_iter()
        .filter(|a| tag_of(&a.pat).is_some_and(|tag| !covered.contains(&tag)))
        .collect();
    if kept.is_empty() {
        return false;
    }
    // The default alternatives leave the decision, so a pattern binder they
    // introduced is lost for the inner alternatives' bodies: refuse if one
    // of them reads it. (A variable default's binder is substituted above.)
    let mut lost: Vec<LocalId> = Vec::new();
    for &i in &defaults {
        if let CPat::Con { fields, .. } = &alts[i].pat {
            lost.extend(fields.iter().copied());
        }
    }
    if kept.iter().any(|a| lost.iter().any(|l| mentions(&a.body, *l))) {
        return false;
    }
    let mut out: Vec<CAlt<Core>> = Vec::with_capacity(alts.len() + kept.len());
    for (i, a) in alts.iter().enumerate() {
        if defaults.contains(&i) {
            continue;
        }
        out.push(a.clone());
    }
    for mut a in kept {
        for binder in &binders {
            subst(&mut a.body, *binder, &Core::Var(v));
        }
        out.push(a);
    }
    *e = Core::Case { scrut: scrut.clone(), alts: out };
    true
}

/// The decision a default alternative's body holds on `root`: directly,
/// or behind the alias `let` the lowering puts around a fallthrough. The
/// alias binders come back so the merged alternatives can name the outer
/// scrutinee where they named the alias. `own` is the alternative's own
/// binder for the scrutinee, when its pattern introduces one.
fn inner_case(
    body: &Core,
    scope: &Scope,
    root: LocalId,
    own: Option<LocalId>,
) -> Option<(Vec<LocalId>, Vec<CAlt<Core>>)> {
    let decision = |e: &Core, via: Option<LocalId>| match e {
        Core::Case { scrut, alts } => match &**scrut {
            Core::Var(w) if *w == root || Some(*w) == own || Some(*w) == via || scope.root_of(*w) == root => {
                Some(alts.clone())
            }
            _ => None,
        },
        _ => None,
    };
    match body {
        Core::Case { .. } => Some((Vec::new(), decision(body, None)?)),
        // `let y = x in case y of ...`: the alias names the same value, and
        // the decision usually names the alias rather than `x`.
        Core::Let(l, _, value, rest) => match &**value {
            Core::Var(w) if *w == root || Some(*w) == own || scope.root_of(*w) == root => {
                Some((vec![*l], decision(rest, Some(*l))?))
            }
            _ => None,
        },
        _ => None,
    }
}

/// A decision on a local whose surrounding `let` bound a constructor
/// application reduces to the tag's alternative.
fn known_constructor(e: &mut Core, scope: &Scope, tables: &Tables) -> bool {
    let Core::Case { scrut, .. } = e else { return false };
    // The scrutinee is a constructor application: the value a local was bound
    // to (through the alias `let`s lowering adds), or the application itself.
    // The second form is what `case-of-case` produces when it scrutinizes the
    // value an inner alternative built (`case (Some x) of ...`), and folding
    // it removes the block the inner alternative allocated.
    let (value, tag, args) = match &**scrut {
        Core::Var(v) => {
            let Some(value) = scope.ctor_of(*v) else { return false };
            let Some(tag) = ctor_app(value, tables) else { return false };
            (value.clone(), tag, ctor_args(value))
        }
        other => {
            let Some(tag) = ctor_app(other, tables) else { return false };
            (other.clone(), tag, ctor_args(other))
        }
    };
    // Alternatives are tried in order: a constructor alternative for
    // another tag cannot match a value of this tag, and a variable or
    // wildcard alternative matches it.
    let mut chosen: Option<Core> = None;
    let mut substs: Vec<(LocalId, Core)> = Vec::new();
    if let Core::Case { alts, .. } = e {
        for a in alts.iter() {
            match &a.pat {
                CPat::Con { tag: alt_tag, fields, .. } if *alt_tag == tag => {
                    if fields.len() != args.len() {
                        return false;
                    }
                    for (field, arg) in fields.iter().zip(&args) {
                        substs.push((*field, arg.clone()));
                    }
                    chosen = Some((*a.body).clone());
                    break;
                }
                CPat::Var(l, _) => {
                    substs.push((*l, value.clone()));
                    chosen = Some((*a.body).clone());
                    break;
                }
                CPat::Wild => {
                    chosen = Some((*a.body).clone());
                    break;
                }
                CPat::Con { .. } => {}
            }
        }
    }
    let Some(mut body) = chosen else { return false };
    // A quotation's retained body is its own binding scope, so a
    // substitution across it is not defined here; keep the decision.
    if carries_quote(&body) {
        return false;
    }
    for (from, to) in substs {
        subst(&mut body, from, &to);
    }
    *e = body;
    true
}

/// A decision on a constructor application whose arguments are *computed*,
/// folded by rebinding the arguments as `let`s where the construction stood.
///
/// This is the half of the fold section 10.2 records as missing, and the
/// formulation that makes it unconditionally sound is not the substitution
/// one. [`known_constructor`] substitutes a field binder by an argument, which
/// requires the argument to be a value: substituting a computed argument at
/// *n* reads would run its work *n* times, and in a strict language it would
/// also move the work past whatever sits between the construction and the
/// read. Requiring the field to be read at most once rescues the second
/// objection but not the first, because a read that a nested decision skips
/// still drops the work.
///
/// So this rule does not substitute at all. It binds each argument to a fresh
/// local *at the position the construction occupied*, in the argument order,
/// and substitutes the field binders by those locals:
///
/// ```text
/// let t = Ctor a1 a2 in case t of (con k f1 f2) -> body
/// let x1 = a1 in let x2 = a2 in body[x1/f1, x2/f2]
/// ```
///
/// The order is preserved by construction: the constructor form evaluates
/// `a1`, then `a2`, then allocates, then dispatches; the folded form evaluates
/// `a1`, then `a2`, then `body`. The allocation and the dispatch are what the
/// rule deletes, and neither is observable. A field read any number of times
/// costs exactly what it cost before, because `crate::rc` counts a `let`
/// binder and a pattern binder the same way. Nothing moves, so nothing about
/// evaluation order has to be argued beyond the argument order.
///
/// The rule takes both shapes: the `let`-bound form above, which is what the
/// checker's lowering produces, and the direct form `case (Ctor a1 a2) of
/// ...`, which is what `case-of-case` produces when it scrutinizes the value an
/// inner alternative built.
///
/// It refuses when:
///
/// - the scrutinee's tag selects a variable or wildcard alternative, which
///   binds the block itself and so cannot be had without the block;
/// - the decision is weighted (removing it rescales the weight space, and the
///   driving pass owns the weighted forms);
/// - the chosen alternative carries a quotation, whose retained body is a
///   binding scope of its own;
/// - the body carries a `Release`, `Reuse` or `Drop`, which means
///   `crate::rc` has already run and the block this rule deletes has count
///   operations naming it (`Release` names the block directly);
/// - the `let`-bound form's binder is read anywhere but the decision's
///   scrutinee position, so the block has another reader;
/// - the constructor is identity-bearing (`MutCell`, a thunk or memo cell),
///   where two evaluations are two cells.
///
/// `crate::rc` runs after this pass, so the fresh `let` binders are counted
/// where they now sit: each argument is one owned binding, exactly as the
/// constructor's field was.
fn known_constructor_computed(
    e: &mut Core,
    tables: &Tables,
    classes: &HashMap<LocalId, Class>,
    fresh: &mut Fresh,
) -> bool {
    // The two shapes, and for the `let` form the binder whose block the fold
    // deletes.
    let (args, ctor_tag, ctor_name, alts) = match &*e {
        Core::Case { scrut, alts } => match &**scrut {
            Core::App(head, args) => match &**head {
                Core::Ctor(id, _) => {
                    let Some((tag, _)) = tables.ctors.get(&id.0).copied() else { return false };
                    (args.clone(), tag, id.0.clone(), alts.clone())
                }
                _ => return false,
            },
            _ => return false,
        },
        Core::Let(x, _, value, body) => {
            let Core::Case { scrut, alts } = &**body else { return false };
            let Core::Var(v) = &**scrut else { return false };
            if v != x {
                return false;
            }
            let Core::App(head, args) = &**value else { return false };
            let Core::Ctor(id, _) = &**head else { return false };
            let Some((tag, _)) = tables.ctors.get(&id.0).copied() else { return false };
            // The block's only reader is the decision this rule deletes.
            if uses_of(body, *x) != 1 {
                return false;
            }
            (args.clone(), tag, id.0.clone(), alts.clone())
        }
        _ => return false,
    };
    // A constructor whose every argument is already a value is the plain
    // fold's population, and it fires first; requiring one computed argument
    // keeps this rule's counter honest about what it did.
    if args.iter().all(|a| is_value(a, tables)) {
        return false;
    }
    if alts.is_empty() || alts.iter().any(|a| a.weight.is_some()) {
        return false;
    }
    // The alternative the tag selects, first match in source order: a
    // constructor alternative for another tag cannot match.
    let chosen = alts.iter().find(|a| match &a.pat {
        CPat::Con { tag: t, .. } => *t == ctor_tag,
        CPat::Var(..) | CPat::Wild => true,
    });
    let Some(chosen) = chosen else { return false };
    // A variable or wildcard alternative binds the block, which is going away.
    let CPat::Con { fields: binders, classes: field_classes, .. } = &chosen.pat else {
        return false;
    };
    if binders.len() != args.len() {
        return false;
    }
    if carries_quote(&chosen.body) || carries_count_ops(&chosen.body) {
        return false;
    }
    // The identity-bearing constructors, where two evaluations are two cells.
    if is_identity_bearing(&ctor_name) {
        return false;
    }
    // Bind every argument, in order, to a fresh local of the field binder's
    // own class, and substitute the binder by it.
    let mut body = (*chosen.body).clone();
    let mut binders_out: Vec<(LocalId, Class, Core)> = Vec::with_capacity(args.len());
    for (i, (binder, arg)) in binders.iter().zip(args.into_iter()).enumerate() {
        let class = field_classes
            .get(i)
            .copied()
            .or_else(|| classes.get(binder).copied())
            .unwrap_or(Class::Raw);
        let local = fresh.local();
        subst(&mut body, *binder, &Core::Var(local));
        binders_out.push((local, concrete(class), arg));
    }
    let mut out = body;
    for (local, class, value) in binders_out.into_iter().rev() {
        out = Core::Let(local, class, Box::new(value), Box::new(out));
    }
    *e = out;
    true
}

/// Whether a constructor's name marks it identity bearing: a mutable cell or
/// a thunk, where two constructions are two distinct cells and no rewrite may
/// move or delete one.
fn is_identity_bearing(name: &str) -> bool {
    name == crate::core::MUTABLE_CELL || name.contains("Thunk") || name.contains("Memo")
}

/// Whether the term carries a count operation or a `Release`/`Reuse`, which
/// means `crate::rc` has already run: those nodes name binders by identity and
/// a rewrite that deletes a block must not leave them behind.
fn carries_count_ops(e: &Core) -> bool {
    if matches!(e, Core::Dup(..) | Core::Drop(..) | Core::Release { .. } | Core::Reuse { .. }) {
        return true;
    }
    let mut found = false;
    visit_children(e, &mut |child| {
        if !found && carries_count_ops(child) {
            found = true;
        }
    });
    found
}

/// `proj (Some x) idx` is `x`: a record field read from a constructor
/// application whose field at that slot is a value.
///
/// The projection a constructor's own field read lowers to survives the
/// known-constructor fold whenever the fold did not reach it (the decision
/// that read the block is elsewhere, or there is none), and each one is a
/// load the block never needed. The rule requires the slot to name an actual
/// field (so the block has that word) and the field to be a value, because a
/// computed field substituted at the projection's use would move its work
/// there; that is the same condition the value-only fold states and the same
/// reason.
fn projection(e: &mut Core, tables: &Tables) -> bool {
    let Core::Proj(inner, field) = e else { return false };
    if field.access.is_some() || field.mutable {
        return false;
    }
    let Core::App(head, args) = &**inner else { return false };
    let Core::Ctor(id, _) = &**head else { return false };
    let Some((_, fields)) = tables.ctors.get(&id.0).copied() else { return false };
    if fields != args.len() || field.slot >= fields {
        return false;
    }
    let Some(arg) = args.get(field.slot) else { return false };
    if !is_value(arg, tables) {
        return false;
    }
    *e = arg.clone();
    true
}

/// `dictproj (instdict I) idx` applied to a static instance's method
/// arity calls the method unit directly.
fn direct_dict_call(e: &mut Core, tables: &Tables) -> bool {
    let replacement = {
        let Core::App(head, args) = &*e else { return false };
        let Core::DictProj(dict, idx, _, _) = &**head else { return false };
        let Core::InstDict(inst, subs, _) = &**dict else { return false };
        let Some((subdicts, methods)) = tables.instances.get(&inst.0) else { return false };
        // A static instance: no subdictionaries, so the method closure the
        // projection yields has no environment.
        if *subdicts != 0 || !subs.is_empty() {
            return false;
        }
        let Some(method) = methods.get(*idx) else { return false };
        let Some(arity) = tables.arity.get(&method.0).copied() else { return false };
        if arity == 0 || arity != args.len() {
            return false;
        }
        method.clone()
    };
    if let Core::App(_, args) = e {
        let args = std::mem::take(args);
        *e = Core::App(Box::new(Core::Global(replacement, Vec::new())), args);
        return true;
    }
    false
}

/// `let f = g a b in f c` with `g`'s arity exactly the combined count
/// becomes `g a b c`: one saturated direct call instead of a closure record
/// built to be applied once and released.
///
/// The shape is what a source partial application lowers to, and each site
/// it fires on removes a closure block, the `__apply` that consumed it and
/// the stub's ownership sequence (dup every saved word, branch on
/// uniqueness, free or decrement). On the compiler's own source the census
/// finds the shape under-applied at 9 sites, of which 5 apply it exactly
/// once in head position; the toolkit has 1. The rule is small because the
/// population is: it is here for the sites that exist, not for a general
/// arity optimisation.
///
/// Four conditions, each of which a refusal test pins:
///
/// - The value is an application whose head is a unit with a known arity
///   (`Global` or `Prim`). A dynamic head has no arity to saturate to.
/// - The value is under-applied (`given < arity`). A saturated or
///   over-applied value is already a direct call, and a call site that
///   supplied too much is handled by the over-application split below.
/// - The binder is read exactly once, and that read is on the spine (not
///   under a decision, an `If` branch or a join body). Substituting moves
///   the value's work to its read, so a read that may not happen would drop
///   the work, and a read in a join body runs once per jump.
/// - The single read is an application's head. Any other position wants the
///   value, not the call, so the closure has to exist.
///
/// Over-application stops at saturation: `let f = g a in f b c` with `g`'s
/// arity 2 becomes `(g a b) c`, so the closure record and its stub call go
/// and the excess stays applied to the result exactly as before. Fewer than
/// the arity after the merge is a partial application still, which needs the
/// closure, so the rule refuses it.
fn saturate_partial(
    e: &mut Core,
    occ: &HashMap<LocalId, Occ>,
    tables: &Tables,
) -> bool {
    let Core::Let(l, _, value, body) = e else { return false };
    let count = occ.get(l).copied().unwrap_or_default();
    if count.uses != 1 || count.in_join {
        return false;
    }
    let Core::App(head, given) = &**value else { return false };
    let (unit, arity) = match &**head {
        Core::Global(id, _) => (id.clone(), tables.arity.get(&id.0).copied()),
        Core::Prim(op) => (GlobalId(op.symbol().to_string()), Some(op.arity())),
        _ => return false,
    };
    let Some(arity) = arity else { return false };
    if given.len() >= arity {
        return false;
    }
    // A value that allocates or runs an effect cannot move to its read:
    // the substitution would move the work, and in a strict language that
    // changes when it happens. The head is a unit and the arguments are
    // evaluated where they are, so what moves is the call, which is why the
    // spine condition below is the whole of the ordering argument.
    if !spine_only(body, *l, true) {
        return false;
    }
    let given = given.clone();
    let mut body = (**body).clone();
    if !merge_in_head(&mut body, *l, &unit, &given, arity) {
        return false;
    }
    *e = body;
    true
}

/// Replace the one application whose head is `l` with the merged call: the
/// value's own arguments followed by the arguments that application
/// supplied, up to the callee's arity. Arguments beyond it stay applied to
/// the result. Answers `false` without touching `e` when the read is not in
/// head position or the merge would not reach saturation.
fn merge_in_head(e: &mut Core, l: LocalId, unit: &GlobalId, given: &[Core], arity: usize) -> bool {
    match e {
        Core::App(head, args) => {
            if matches!(&**head, Core::Var(x) if *x == l) {
                let total = given.len() + args.len();
                if total < arity {
                    return false;
                }
                let take = arity - given.len();
                let mut merged: Vec<Core> = given.to_vec();
                merged.extend(args.drain(..take));
                let call = Core::App(Box::new(Core::Global(unit.clone(), Vec::new())), merged);
                *e = if args.is_empty() {
                    call
                } else {
                    Core::App(Box::new(call), std::mem::take(args))
                };
                return true;
            }
            if merge_in_head(head, l, unit, given, arity) {
                return true;
            }
            for a in args {
                if merge_in_head(a, l, unit, given, arity) {
                    return true;
                }
            }
            false
        }
        Core::Let(_, _, value, body) => {
            if merge_in_head(value, l, unit, given, arity) {
                return true;
            }
            merge_in_head(body, l, unit, given, arity)
        }
        _ => {
            for slot in child_slots(e) {
                if merge_in_head(slot, l, unit, given, arity) {
                    return true;
                }
            }
            false
        }
    }
}

/// Patterns of two alternatives that share a default: same tag or the
/// same binder for the same class.
fn same_pat(a: &CPat, b: &CPat) -> bool {
    match (a, b) {
        (CPat::Wild, CPat::Wild) => true,
        (CPat::Var(x, cx), CPat::Var(y, cy)) => x == y && cx == cy,
        (
            CPat::Con { tag: t1, fields: f1, classes: c1 },
            CPat::Con { tag: t2, fields: f2, classes: c2 },
        ) => t1 == t2 && f1 == f2 && c1 == c2,
        _ => false,
    }
}

/// Simplify every unit with a `Core` body, to the fixed point or the
/// iteration cap. `report` records the passes actually run, the ticks, and
/// the per-rule counts.
pub fn simplify(program: &mut Program, opts: &SimplifyOptions, report: &mut SimplifyReport) {
    simplify_with_foreign(program, &[], opts, report)
}

/// [`simplify`] with a table of foreign bodies: units this program calls
/// but does not define, whose unfoldings came from an interface
/// optimisation artifact (`crate::ifaceopt`). They join the class exactly
/// as the program's own units do, so a cross-object call site meets the
/// same admission rule and the same copy machinery it would inside the
/// program. Nothing is emitted for a foreign unit: its body is only ever
/// copied into a caller of this program, and the definition stays in the
/// object the artifact describes.
pub fn simplify_with_foreign(
    program: &mut Program,
    foreign: &[FnDef],
    opts: &SimplifyOptions,
    report: &mut SimplifyReport,
) {
    report.max_iterations = opts.max_iterations as usize;
    if opts.max_iterations == 0 {
        return;
    }
    let mut tables = Tables::new(program);
    tables.extend(foreign);
    let mut ticks = Ticks::default();
    let mut fresh = Fresh { local: max_local(program), join: max_join(program) };
    // Two percent of the program is the most inlining may add: a copy is
    // code the linker and the instruction cache pay for, so the budget is
    // what keeps the pass from trading a run-time win for an unbounded
    // growth.
    let mut budget = (program.fns.iter().map(|f| nodes(&f.body)).sum::<usize>() / 50).max(1_000);
    program.inline.clear();
    let mut facts: HashMap<String, InlineFacts> = HashMap::new();
    // The benefit model's site tally, accumulated over every pass so the
    // artifact reports the whole run rather than the last pass.
    let mut benefit = BenefitTally::default();
    for _ in 0..opts.max_iterations {
        ticks.changed = false;
        if opts.inline {
            let table = inline_table(program, foreign, &tables, opts);
            if !table.is_empty() {
                for f in program.fns.iter_mut() {
                    if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
                        continue;
                    }
                    CURRENT.with(|c| c.borrow_mut().clone_from(&f.id.0));
                    let mut body = std::mem::replace(&mut f.body, Core::Unit);
                    let mut here = InlineFacts::default();
                    let mut stack: Vec<String> = Vec::new();
                    let loop_body = table.is_loop(&f.id.0);
                    inline_unit(
                        &mut body,
                        &table,
                        &tables,
                        &mut fresh,
                        &mut budget,
                        &mut here,
                        &mut ticks,
                        &mut stack,
                        0,
                        loop_body,
                        &mut benefit,
                        opts,
                    );
                    f.body = body;
                    if here.sites > 0 {
                        let entry = facts.entry(f.id.0.clone()).or_default();
                        entry.sites += here.sites;
                        entry.nodes += here.nodes;
                        entry.callees.extend(here.callees);
                    }
                }
            }
        }
        for f in program.fns.iter_mut() {
            if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
                continue;
            }
            let mut scope = Scope::default();
            let classes = unit_classes(f);
            let mut body = std::mem::replace(&mut f.body, Core::Unit);
            CURRENT.with(|c| c.borrow_mut().clone_from(&f.id.0));
            let mut occ: HashMap<LocalId, Occ> = HashMap::new();
            if opts.occurrence {
                occurrences(&body, false, &mut occ);
            }
            pass(&mut body, &mut scope, &tables, &classes, &occ, opts, &mut fresh, &mut ticks);
            f.body = body;
        }
        report.iterations += 1;
        if !ticks.changed {
            break;
        }
    }
    report.ticks = ticks.ticks;
    report.case_merge = ticks.case_merge;
    report.known_ctor = ticks.known_ctor;
    report.known_ctor_once = ticks.known_ctor_computed;
    report.projection = ticks.projection;
    report.dict_call = ticks.dict_call;
    report.saturate = ticks.saturate;
    report.case_of_case = ticks.case_of_case;
    report.joins = ticks.joins;
    report.duplicated = ticks.duplicated;
    report.inlined = ticks.inline;
    report.inlined_nodes = facts.values().map(|f| f.nodes as usize).sum();
    report.inline_model = match opts.inline_model {
        InlineModel::Threshold => "threshold",
        InlineModel::Benefit => "benefit",
    };
    report.benefit = benefit;
    let mut units: Vec<String> = facts.keys().cloned().collect();
    units.sort();
    report.inlined_units = units;
    program.inline = facts.into_iter().collect();
}


// ---------------------------------------------------------------- inlining
//
// The inliner is the pass's fifth rule (`inline`) and the only one that
// rewrites across units: it replaces a saturated direct call with the
// callee's body, so an allocation a callee would have performed happens at
// the call site instead. That is what makes a small function's fresh block
// the caller's own value, which is the shape `crate::stack` needs: a block
// that is born at the call site and dies in the same activation can live in
// the frame.
//
// The rule is GHC's decision, reduced to the inputs this pass has
// (design/inline-unfolding-policy.md section 9):
//
// - **Certain.** A unit in the unfoldable class is copied at every site with
//   no size test: its body is cheap by construction (a primop, a projection,
//   a literal, a shallow decision over a known constructor, a delegation to
//   another such unit). A zero-field constructor is certain too, because its
//   body is a static block, so a copy is a constant. A unit in a recursive
//   group, one that carries a quotation or a join, and one whose parameter
//   classes are still open are refused before any of this.
// - **Risky.** Any other body is admitted to the table only when its
//   weighted size ([`weighted`]) is under
//   [`SimplifyOptions::inline_creation_threshold`], and a site copies it
//   only when its weighted size minus the call's discount is under
//   [`SimplifyOptions::inline_use_threshold`]. The discount is the call
//   itself plus every argument that already costs something, plus more for
//   an interesting argument ([`interesting_arg`]) because the copy will fold
//   it. The discount is the whole of GHC's `computeDiscount` that our cost
//   model can express.
// - A decision-bearing copy is admitted at a site only where the scrutinee
//   argument is a constructor application (unless the body is small enough
//   that duplicating the branch costs no more than the call), so the copy
//   folds in the pass that follows instead of duplicating the branch.
// - `CINCH_INLINE_TRACE` prints a line per candidate with its node count,
//   sites and free locals, and per copy with the caller and the callee.
// - Arguments that are not a local are bound to fresh locals in order, so
//   strict evaluation order is unchanged, and the callee's body sees a
//   `Var` in every parameter position. A local argument is substituted
//   directly.
// - Every binder of the copied body gets a fresh local id and every use of
//   it is renamed with it, so the copy can never capture or shadow the
//   caller's binders. The parameters are then renamed to fresh ids too, and
//   the substitution replaces those: a caller's local that lands in the body
//   can never be mistaken for a parameter still to be substituted.
// - A node budget (two percent of the program by default) caps how much the
//   pass may grow the program; a site that would pass it is left alone.
//
// Nothing is removed: the callee keeps its body, because other artifacts
// and the program's interfaces name it.

// ---------------------------------------------- benefit-and-size inlining
//
// The second model ([`InlineModel::Benefit`]) is Flambda 2's: the site test
// compares an estimate of the work the copy removes against the code size
// the copy adds, rather than a body's size against a fixed threshold
// (`middle_end/flambda2/simplify/inlining/call_site_inlining_decision.ml`,
// `terms/code_size_v1.ml`, `terms/removed_operations.ml`). Two things follow
// from that shape and neither is available to a threshold rule:
//
// - An allocation is worth more than it costs. Flambda 2 prices a removed
//   allocation at 7 and an added one at 5 plus a field, so a copy that
//   deletes an allocation and a call is admitted even when it is *larger*
//   than the call it replaces. A threshold rule has no way to say that,
//   which is why ours prices an allocation at six times a call and refuses
//   the copy outright.
// - A copy that removes nothing is refused however small it is. A body of
//   five primops at a site with no argument discount removes one call and
//   adds five instructions, so the size grows and the benefit is one call.
//
// The model here, on Flambda 2's integer scale:
//
// **Code size** ([`code_size`]) is the instruction count the code generator
// emits for a node, in Flambda 2's units: an allocation is 5 plus one per
// field, a direct call 4, an indirect call 6, a C call 10, a primitive 1, a
// switch 5 per arm, a projection 1, a closure 5 plus its environment. A
// literal or a variable is 0.
//
// **Benefit** ([`removed_benefit`]) is the weighted count of operations the
// copy removes from the program, on Flambda 2's `removed_operations` scale
// divided by 8 (so the comparison against size is in size units): a removed
// call 5/8, a removed allocation 7/8, a removed primitive 3/8, a removed
// branch 5/8, a removed indirect call 4/8. The one addition the earlier
// survey recorded and this tree can use: a projection whose scrutinee the
// copy knows to be a constructor application is removed too, because
// `known-constructor` folds it in the same pass.
//
// **The site test** is `size * sites - benefit <= threshold`, where `sites`
// is the number of duplicated call sites: the body's code exists once per
// site after the copies and once now, so the growth is `size * (sites - 1)`
// and the call it removes is already inside `benefit`. A body at or below
// `small_function_size` skips the test entirely (Flambda 2's
// `Always_inline`), and a body above `large_function_size` is not considered
// at all.
//
// **Loops and recursion.** A call site whose enclosing unit is a tail worker
// (`<unit>#tail`, the shape every source loop lowers to) or a member of a
// recursive group is entered on every iteration, so its copy's cost is paid
// per iteration and its benefit is paid once: the site test's allowance is
// divided by `inline_loop_discount`. A callee in a recursive group is not
// admitted at all, which is the same refusal the threshold model makes and
// the reason a self-recursive worker never reaches the stack rule.
//
// Every site decision is tallied in [`BenefitTally`], so the artifact
// manifest says how many sites the rule inlined and how many it refused,
// per reason, rather than leaving the difference between two IR line counts
// to be inferred.

/// Flambda 2's `removed_operations` weights, divided by 8 so a benefit is
/// comparable with a code size. The asymmetry is the point: an allocation
/// is worth 7/8 of a size unit and a primitive 3/8, so deleting an
/// allocation buys more growth than deleting a primitive does.
const BENEFIT_CALL: usize = 5;
const BENEFIT_ALLOCATION: usize = 7;
const BENEFIT_PRIMITIVE: usize = 3;
const BENEFIT_BRANCH: usize = 5;
const BENEFIT_INDIRECT: usize = 4;

/// The site decisions of the benefit model, one bucket per outcome. Every
/// call site the sweep considers lands in exactly one bucket, so `inlined`
/// plus the refusals is the site count the rule looked at.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BenefitTally {
    /// Sites whose copy the rule admitted.
    pub inlined: usize,
    /// Sites refused because the body is larger than
    /// [`SimplifyOptions::inline_large_size`], so no site test ran.
    pub refused_large: usize,
    /// Sites refused because the copy grows the program by more than the
    /// site's allowance: the size test, which is the rule's ordinary
    /// refusal.
    pub refused_size: usize,
    /// Sites refused because the callee is in a recursive group.
    pub refused_recursive: usize,
    /// Sites refused because the callee is a value somewhere, so a copy is
    /// not a replacement for every reference.
    pub refused_value: usize,
    /// Sites refused because the callee is the program's entry unit, or its
    /// parameter classes are still open, or it carries a quotation or a
    /// join: the structural refusals the two models share.
    pub refused_scope: usize,
    /// Sites refused because a decision-bearing body's scrutinee argument is
    /// not a known constructor, so the copy would duplicate the branch
    /// instead of folding it.
    pub refused_fold: usize,
    /// Sites refused because the unfolding chain is already
    /// [`SimplifyOptions::inline_depth`] links long, or the callee is
    /// already on the stack.
    pub refused_chain: usize,
    /// Sites refused because the copy would pass the program-wide growth
    /// budget.
    pub refused_budget: usize,
}

impl BenefitTally {
    /// Every refusal, summed: the sites the rule looked at that it did not
    /// inline.
    pub fn refused(&self) -> usize {
        self.refused_large
            + self.refused_size
            + self.refused_recursive
            + self.refused_value
            + self.refused_scope
            + self.refused_fold
            + self.refused_chain
            + self.refused_budget
    }
}

/// Why the benefit model left a site alone. One variant per
/// [`BenefitTally`] bucket, so the tally and the trace cannot disagree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refusal {
    Large,
    Size,
    Recursive,
    Value,
    Scope,
    Fold,
    Chain,
    Budget,
}

impl Refusal {
    fn name(self) -> &'static str {
        match self {
            Refusal::Large => "large",
            Refusal::Size => "size",
            Refusal::Recursive => "recursive",
            Refusal::Value => "value",
            Refusal::Scope => "scope",
            Refusal::Fold => "fold",
            Refusal::Chain => "chain",
            Refusal::Budget => "budget",
        }
    }

    fn count(self, tally: &mut BenefitTally) {
        match self {
            Refusal::Large => tally.refused_large += 1,
            Refusal::Size => tally.refused_size += 1,
            Refusal::Recursive => tally.refused_recursive += 1,
            Refusal::Value => tally.refused_value += 1,
            Refusal::Scope => tally.refused_scope += 1,
            Refusal::Fold => tally.refused_fold += 1,
            Refusal::Chain => tally.refused_chain += 1,
            Refusal::Budget => tally.refused_budget += 1,
        }
    }
}

/// The instruction count a node's code costs, in Flambda 2's units
/// (`terms/code_size_v1.ml`): an allocation 5 plus one per field, a direct
/// call 4, an indirect call 6, a C call 10, a primitive 1, a switch 5 per
/// arm, a projection 1, a closure 5 plus its environment. A literal, a
/// variable and a value binder cost nothing, because the code generator
/// emits no instruction for them.
fn code_size(e: &Core, tables: &Tables) -> usize {
    match e {
        Core::Int(_)
        | Core::Bool(_)
        | Core::Unit
        | Core::ConstBig { .. }
        | Core::ConstBytes(_)
        | Core::ConstString { .. }
        | Core::ConstSym(_)
        | Core::StrLit(_)
        | Core::Var(_)
        | Core::Global(..)
        | Core::Ctor(..)
        | Core::Prim(_)
        | Core::Foreign(_)
        | Core::SpliceVal(_)
        | Core::SyntaxLit(..)
        | Core::LazyAlloc
        | Core::BoxAlloc
        | Core::Fault => 0,
        // A `let` of a value is the binder's slot, which the occurrence
        // rules eliminate; what it costs is its right hand side.
        Core::Let(_, _, value, body) => code_size(value, tables) + code_size(body, tables),
        Core::App(head, args) => match &**head {
            // A primop's instruction sequence.
            Core::Prim(_) => 1 + args.iter().map(|a| code_size(a, tables)).sum::<usize>(),
            // A real call: the wrapper plus its arguments.
            Core::Foreign(_) => 10 + args.iter().map(|a| code_size(a, tables)).sum::<usize>(),
            // A constructor application: one block, five units plus a field
            // each (a nullary constructor is a static block, so nothing).
            Core::Ctor(id, _) => {
                let fields = tables.ctors.get(&id.0).map_or(args.len(), |(_, f)| *f);
                if fields == 0 {
                    args.iter().map(|a| code_size(a, tables)).sum()
                } else {
                    5 + fields + args.iter().map(|a| code_size(a, tables)).sum::<usize>()
                }
            }
            Core::Global(g, _) if tables.ctors.contains_key(&g.0) => {
                let fields = tables.ctors.get(&g.0).map_or(args.len(), |(_, f)| *f);
                if fields == 0 {
                    args.iter().map(|a| code_size(a, tables)).sum()
                } else {
                    5 + fields + args.iter().map(|a| code_size(a, tables)).sum::<usize>()
                }
            }
            // A direct call: four units, which is Flambda 2's price for
            // `Apply` with a `Direct` call kind.
            Core::Global(..) => 4 + args.iter().map(|a| code_size(a, tables)).sum::<usize>(),
            // Anything else is an indirect call through a closure: six
            // units, plus the closure the head evaluates.
            other => 6 + code_size(other, tables) + args.iter().map(|a| code_size(a, tables)).sum::<usize>(),
        },
        // A closure: five units for the block, plus what its environment
        // evaluates.
        Core::Lam { env, .. } => 5 + env.iter().map(|a| code_size(a, tables)).sum::<usize>(),
        // A switch: five units per arm, plus the scrutinee and the arms.
        Core::Case { scrut, alts } => {
            5 * alts.len()
                + code_size(scrut, tables)
                + alts.iter().map(|a| code_size(&a.body, tables)).sum::<usize>()
        }
        Core::If(condition, then, otherwise) => {
            5 + code_size(condition, tables) + code_size(&then.body, tables) + code_size(&otherwise.body, tables)
        }
        // A load.
        Core::Proj(inner, _) | Core::DictProj(inner, ..) | Core::Demand(inner, _) => {
            1 + code_size(inner, tables)
        }
        Core::InstDict(..) | Core::Reuse { .. } => 5 + child_slots_imm(e).into_iter().map(|c| code_size(c, tables)).sum::<usize>(),
        // The remaining nodes are effects whose own instruction sequence is
        // one unit plus what they carry: a count operation is an atomic on
        // the header, a store is one store.
        _ => 1 + child_slots_imm(e).into_iter().map(|c| code_size(c, tables)).sum::<usize>(),
    }
}

/// The operations one call site's copy removes from the program, in
/// eighths of a [`code_size`] unit. The one unconditional term is the call
/// the copy replaces; the rest is credited only when the site makes it
/// certain.
///
/// This is a lower bound, not Flambda 2's measured benefit. Flambda 2 builds
/// the inlined body in a scratch accumulator, simplifies it, and takes the
/// difference of two `Cost_metrics`; this pass has no scratch simplifier, so
/// the benefit is read off the body and the site instead:
///
/// - The call: every site removes one.
/// - A branch: a body that decides over a parameter is copied only where the
///   site supplies a constructor for that parameter (the `fold` test), so
///   `known-constructor` deletes the decision in the pass that follows. Only
///   the alternatives the fold drops are credited.
/// - A projection: credited where the body projects a local the fold has
///   turned into a field.
/// - An allocation: not credited. A copy moves the allocation to the site,
///   and nothing in this pass deletes it, so crediting it would admit a copy
///   that duplicates the allocation at every site. This is exactly the term
///   Flambda 2 credits (7/8 of a size unit, above its own 5+fields price)
///   and this model cannot, and it is the one place the two disagree.
fn site_benefit(body: &Core, tables: &Tables, folds: bool) -> usize {
    let call = BENEFIT_CALL;
    if !folds {
        return call;
    }
    call + fold_credit(body, tables)
}

/// The operations `known-constructor` deletes from `body` once the
/// parameters it decides over arrive as constructor applications, in
/// eighths of a size unit: each such decision loses every alternative but
/// the one its tag selects, and a projection of a local bound to a
/// constructor application becomes the field itself.
fn fold_credit(body: &Core, tables: &Tables) -> usize {
    fn go(e: &Core, bound: &mut HashMap<LocalId, usize>, tables: &Tables, credit: &mut usize) {
        match e {
            Core::Let(l, _, value, rest) => {
                go(value, bound, tables, credit);
                let inserted = ctor_app(value, tables).map(|tag| {
                    bound.insert(*l, tag);
                });
                go(rest, bound, tables, credit);
                if inserted.is_some() {
                    bound.remove(l);
                }
            }
            Core::Case { scrut, alts } => {
                go(scrut, bound, tables, credit);
                if let Core::Var(v) = &**scrut {
                    if bound.contains_key(v) {
                        *credit += BENEFIT_BRANCH * alts.len().saturating_sub(1);
                    }
                }
                for a in alts {
                    go(&a.body, bound, tables, credit);
                }
            }
            Core::Proj(inner, _) => {
                go(inner, bound, tables, credit);
                if let Core::Var(v) = &**inner {
                    if bound.contains_key(v) {
                        *credit += BENEFIT_PRIMITIVE;
                    }
                }
            }
            _ => {
                for child in child_slots_imm(e) {
                    go(child, bound, tables, credit);
                }
            }
        }
    }
    let mut bound: HashMap<LocalId, usize> = HashMap::new();
    let mut credit = 0;
    go(body, &mut bound, tables, &mut credit);
    credit
}

thread_local! {
    /// The unit the sweep is rewriting, for `CINCH_INLINE_TRACE`.
    static CURRENT: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

fn current_unit() -> String {
    CURRENT.with(|c| c.borrow().clone())
}

/// One unit's inlining facts, for the Core dump: the sites its body was
/// copied to and the callees it took in.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InlineFacts {
    /// Call sites replaced in this unit.
    pub sites: u32,
    /// Core nodes the copies added here.
    pub nodes: u32,
    /// The callees inlined here, in site order.
    pub callees: Vec<String>,
}

/// The bodies a sweep may inline, with their parameter classes.
#[derive(Default)]
struct InlineTable {
    /// Bodies inlined at their single call site, by unit name.
    bodies: HashMap<String, InlineBody>,
    /// The units whose bodies are loop bodies: a tail worker (`<unit>#tail`,
    /// the shape every source loop lowers to) or a member of a recursive
    /// group. A call site inside one of these is entered once per iteration,
    /// so the benefit model divides its allowance there.
    loops: HashSet<String>,
}

impl InlineTable {
    /// Whether `unit`'s body is a loop body.
    fn is_loop(&self, unit: &str) -> bool {
        self.loops.contains(unit)
    }
}

struct InlineBody {
    body: Core,
    /// The callee's parameter classes (`UnitRepr::params`), so a bound
    /// argument's `let` carries the class the count pass needs.
    params: Vec<Class>,
    /// The parameters a copy's `Case` scrutinizes, and whether each must be
    /// a known constructor at the site so the decision folds instead of
    /// being duplicated.
    cases: Vec<usize>,
    fold: bool,
    /// Whether the copy is certain: no site test.
    certain: bool,
    /// The copy's cost on the model's scale.
    weight: usize,
    /// Whether the copy charges the growth budget. A pure class body (no
    /// decision and no link to another unit) replaces a call and its frame
    /// with the instruction itself, so it shrinks the module and spends
    /// nothing; a decision or a chain link can grow, and is bounded by the
    /// budget like every other copy.
    charge: bool,
    /// The benefit model's code size for this body ([`code_size`]).
    size: usize,
    /// The benefit model's estimate of the work a copy removes
    /// ([`removed_benefit`]).
    benefit: usize,
    /// The number of saturated direct call sites the program has for this
    /// body, which is the `sites` factor in the benefit model's growth
    /// term. Zero for a body only the foreign table provides.
    sites: usize,
    /// Whether the callee is in a recursive group, so the benefit model
    /// refuses it before any site test. The threshold model refuses it at
    /// admission; this field exists because the benefit model admits a
    /// recursive body to the table (its size is what a site test needs) and
    /// refuses it at the site.
    recursive: bool,
    /// Whether the callee is used as a value somewhere in the program, so a
    /// copy is not a replacement for every reference.
    valued: bool,
}

impl InlineTable {
    fn get(&self, unit: &str) -> Option<&InlineBody> {
        self.bodies.get(unit)
    }

    fn insert(&mut self, unit: String, body: InlineBody) {
        self.bodies.insert(unit, body);
    }

    fn is_empty(&self) -> bool {
        self.bodies.is_empty()
    }
}

/// The units of `program` whose bodies are in the unfoldable class, with no
/// foreign table. The interface optimisation writer uses this to put exactly
/// the bodies the admission rule would copy into an artifact; computing it
/// once here keeps the producer and the consumer on one rule.
pub fn unfoldable_names(program: &Program) -> std::collections::BTreeSet<String> {
    let tables = Tables::new(program);
    let mut calls: HashMap<String, usize> = HashMap::new();
    let mut values: HashSet<String> = HashSet::new();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for f in &program.fns {
        let mut out = Vec::new();
        collect_refs(&f.body, &tables.arity, false, &mut calls, &mut values, &mut out);
        edges.insert(f.id.0.clone(), out);
    }
    let recursive = recursive_units(&edges);
    unfoldable_units(program, &[], &tables, &recursive).into_keys().collect()
}

/// The inlining facts of one sweep: the call count and value uses of every
/// unit, and the bodies the rules admit. `foreign` are units the program
/// calls but does not define (their unfoldings came from an interface
/// optimisation artifact): they join the class and the table exactly as the
/// program's own units do, and the call sites they answer are the program's.
fn inline_table(
    program: &Program,
    foreign: &[FnDef],
    tables: &Tables,
    opts: &SimplifyOptions,
) -> InlineTable {
    let mut calls: HashMap<String, usize> = HashMap::new();
    let mut values: HashSet<String> = HashSet::new();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for f in &program.fns {
        let mut out = Vec::new();
        collect_refs(&f.body, &tables.arity, false, &mut calls, &mut values, &mut out);
        edges.insert(f.id.0.clone(), out);
    }
    for f in foreign {
        // A foreign body's own calls are not sites of this program: only a
        // caller defined here can be replaced. They are collected for the
        // call graph, so a foreign cycle is still refused entry to the class.
        let mut scratch_calls = HashMap::new();
        let mut scratch_values = HashSet::new();
        let mut out = Vec::new();
        collect_refs(
            &f.body,
            &tables.arity,
            false,
            &mut scratch_calls,
            &mut scratch_values,
            &mut out,
        );
        edges.insert(f.id.0.clone(), out);
    }
    let recursive = recursive_units(&edges);
    let unfold = unfoldable_units(program, foreign, tables, &recursive);
    let trace = opts_trace();
    let mut table = InlineTable::default();
    // The loop bodies: a tail worker, and every member of a recursive group.
    // A call site inside one is entered once per iteration of the loop it
    // sits in, which is the site fact the benefit model discounts for.
    table.loops = program
        .fns
        .iter()
        .map(|f| f.id.0.clone())
        .filter(|name| name.contains("#tail") || recursive.contains(name))
        .collect();
    for f in program.fns.iter().chain(foreign.iter()) {
        // A `prim` unit's body is inline target IR, not Core, and a ctor unit
        // is admitted only when its body is a static block (a nullary
        // constructor's value), which `candidate` decides.
        if f.prim.is_some() || f.meta.prim_body {
            continue;
        }
        let name = &f.id.0;
        let sites = calls.get(name).copied().unwrap_or(0);
        let n = nodes(&f.body);
        let admit = candidate(f, program, &tables, &values, &recursive, &unfold, sites, n, opts);
        if trace {
            // Every candidate the sweep considered, with the numbers the
            // rules decide on: the body's node count and code size, its
            // sites, and the locals the body reads free (exactly its
            // parameters for a body the copy can be closed over).
            let free: Vec<usize> = copy_free_locals(&f.body).into_iter().map(|l| l.0).collect();
            let detail = match &admit {
                Ok(a) => format!(
                    "admitted (size {}, benefit {}, certain {}, folds {})",
                    a.size, a.benefit, a.certain, a.folds
                ),
                Err(why) => format!("refused ({})", why.name()),
            };
            eprintln!(
                "cinch-inline-candidate: {name}: {n} nodes, {} code size, {sites} site(s), {} parameter(s), free {free:?}: {detail}",
                code_size(&f.body, &tables),
                f.arity(),
            );
        }
        if let Ok(admitted) = admit {
            table.insert(
                name.clone(),
                InlineBody {
                    body: f.body.clone(),
                    params: admitted.params,
                    cases: admitted.cases,
                    fold: admitted.fold,
                    charge: admitted.charge,
                    certain: admitted.certain,
                    weight: admitted.weight,
                    size: admitted.size,
                    benefit: admitted.benefit,
                    sites,
                    recursive: admitted.recursive,
                    valued: admitted.valued,
                },
            );
        }
    }
    table
}

/// Why a unit's body is not admitted, in the order [`candidate`] checks
/// them. Spelled out so `CINCH_INLINE_TRACE` says which rule refused
/// rather than leaving a candidate's absence unexplained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refused {
    /// The entry unit, called from generated glue that is not Core.
    Entry,
    /// A value use: the unit is named somewhere that is not a saturated
    /// call, so a copy would not replace it everywhere.
    Value,
    /// In a recursive group, or a loop breaker of one.
    Recursive,
    /// An open parameter class the collector did not resolve.
    Open,
    /// Carries a quotation or a join, whose binding scope is its own.
    Scope,
    /// Too many nodes for the rule that would apply at its site count.
    Large,
    /// Called at no site, or at more than one with a body the multi-site
    /// rule does not admit.
    Sites,
    /// The multi-site rule's body test: it calls, allocates or quotes.
    Costly,
}

impl Refused {
    fn name(self) -> &'static str {
        match self {
            Refused::Entry => "entry",
            Refused::Value => "value",
            Refused::Recursive => "recursive",
            Refused::Open => "open",
            Refused::Scope => "scope",
            Refused::Large => "large",
            Refused::Sites => "sites",
            Refused::Costly => "costly",
        }
    }
}

/// One unit's admission: what the copy needs when a rule takes it, or the
/// check that refused it. `sites` is its saturated direct call count, `n`
/// its body's node count, and `unfold` the class the body was classified
/// into.
struct Admitted {
    /// The callee's parameter classes.
    params: Vec<Class>,
    /// The parameters a copy's `Case` scrutinizes.
    cases: Vec<usize>,
    /// Whether every one of those arguments must be a constructor at the
    /// site, so the copy folds. False for a body small enough that a
    /// duplicated decision costs no more than the call.
    fold: bool,
    /// Whether the copy charges the growth budget.
    charge: bool,
    /// Whether the copy is certain: no site test at all (the class, and a
    /// zero-field constructor, whose body is a static block).
    certain: bool,
    /// The copy's cost on the model's scale (`weighted`).
    weight: usize,
    /// The benefit model's code size for the body ([`code_size`]).
    size: usize,
    /// The benefit model's per-site benefit ([`site_benefit`]).
    benefit: usize,
    /// Whether the benefit model may credit a fold at the site: the same
    /// condition as [`Admitted::fold`], restated because the two models
    /// treat a non-folding decision differently (the threshold model copies
    /// it anyway when it is small, the benefit model never does).
    folds: bool,
    /// The callee is in a recursive group, so no copy may be made. The
    /// threshold model refuses it at admission; the benefit model admits the
    /// body (its size is what the site test reads) and refuses every site.
    recursive: bool,
    /// The callee is used as a value somewhere in the program.
    valued: bool,
}

fn candidate(
    f: &FnDef,
    program: &Program,
    tables: &Tables,
    values: &HashSet<String>,
    recursive: &HashSet<String>,
    unfold: &HashMap<String, Shape>,
    sites: usize,
    n: usize,
    opts: &SimplifyOptions,
) -> Result<Admitted, Refused> {
    let name = &f.id.0;
    // The entry unit is called from the generated exit glue, which is not
    // Core: inlining it anywhere is pointless, and keeping it out keeps the
    // entry visible.
    if program.main.as_ref().is_some_and(|m| &m.0 == name) {
        return Err(Refused::Entry);
    }
    // An unresolved parameter class (`Param`, which the collector left in a
    // body no concrete vector reached) would give a bound argument a `Raw`
    // binder while the body reads it as a borrowed case scrutinee, which
    // the ownership verifier refuses. A callee the inliner cannot type is
    // not inlined.
    if f.repr.params.iter().any(|c| c.is_param()) {
        return Err(Refused::Open);
    }
    if carries_quote(&f.body) || carries_join(&f.body) {
        return Err(Refused::Scope);
    }
    if sites == 0 {
        return Err(Refused::Sites);
    }
    // A nullary constructor's unit returns a static block, so a copy of it
    // is a constant, not work: it is certain, with no size test. A
    // constructor with fields allocates per copy; whether that is worth it
    // is the site test's question, so it is admitted with its own size and
    // benefit and refused there if it does not pay.
    if let Some((_, fields)) = f.ctor {
        let size = if fields == 0 { 0 } else { 5 + fields };
        return Ok(Admitted {
            params: Vec::new(),
            cases: Vec::new(),
            fold: false,
            charge: fields != 0,
            certain: fields == 0,
            weight: weighted(&f.body, tables),
            size,
            benefit: BENEFIT_CALL,
            folds: false,
            recursive: false,
            valued: false,
        });
    }
    let params = || {
        (0..f.arity())
            .map(|i| f.repr.params.get(i).copied().map_or(Class::Raw, concrete))
            .collect()
    };
    let size = code_size(&f.body, tables);
    let is_recursive = recursive.contains(name);
    let valued = values.contains(name);
    // The class rule is the threshold model's certain copy: a body cheap by
    // construction, under a per-shape node cap, is copied to every site with
    // no size test. The benefit model reaches the same conclusion from its
    // own numbers ([`SimplifyOptions::inline_small_size`]), so this arm is
    // only taken under the threshold model.
    if opts.inline_model == InlineModel::Threshold {
        if let Some(shape) = unfold.get(name) {
            if shape.cases == 0 {
                let cap = if shape.chain { opts.inline_unfold } else { opts.inline_small };
                if n <= cap {
                    return Ok(Admitted {
                        params: params(),
                        cases: Vec::new(),
                        fold: false,
                        charge: shape.chain,
                        certain: true,
                        weight: weighted(&f.body, tables),
                        size,
                        benefit: BENEFIT_CALL,
                        folds: false,
                        recursive: is_recursive,
                        valued,
                    });
                }
            } else if n <= opts.inline_unfold_case {
                // A decision whose scrutinee is a parameter: small copies go
                // everywhere (a compare and a branch cost no more than the
                // call), larger ones only where the scrutinee argument is a
                // constructor so the copy folds.
                let mut cases = Vec::new();
                case_scrutinee_params(&f.body, &mut cases);
                let fold = n > opts.inline_unfold_small;
                return Ok(Admitted {
                    params: params(),
                    cases,
                    fold,
                    charge: true,
                    certain: true,
                    weight: weighted(&f.body, tables),
                    size,
                    benefit: BENEFIT_CALL,
                    folds: fold,
                    recursive: is_recursive,
                    valued,
                });
            }
        }
    }
    // The risky path, shared by both models: a body that is not certain is
    // admitted only when it is small enough to be worth considering at all,
    // and each site then makes its own decision. A value use is refused
    // because the copy is no longer a replacement for every reference,
    // which is the size argument the class does not need.
    let weight = weighted(&f.body, tables);
    match opts.inline_model {
        InlineModel::Threshold => {
            if weight > opts.inline_creation_threshold {
                return Err(Refused::Large);
            }
            if valued {
                return Err(Refused::Value);
            }
            Ok(Admitted {
                params: params(),
                cases: Vec::new(),
                fold: false,
                charge: true,
                certain: false,
                weight,
                size,
                benefit: BENEFIT_CALL,
                folds: false,
                recursive: is_recursive,
                valued,
            })
        }
        // Flambda 2's definition-level classes
        // (`function_decl_inlining_decision_type.ml`): at or below
        // `small_function_size` the body is always inlined, above
        // `large_function_size` it is never considered, and between them a
        // site test decides.
        InlineModel::Benefit => {
            if size > opts.inline_large_size {
                return Err(Refused::Large);
            }
            if valued {
                return Err(Refused::Value);
            }
            let mut cases = Vec::new();
            case_scrutinee_params(&f.body, &mut cases);
            let folds = !cases.is_empty();
            Ok(Admitted {
                params: params(),
                cases,
                // The benefit model never copies a decision it cannot fold:
                // a duplicated branch is exactly what the benefit term
                // cannot pay for, because the copy would add arms and remove
                // no call from inside them.
                fold: folds,
                charge: true,
                certain: size <= opts.inline_small_size,
                weight,
                size,
                benefit: site_benefit(&f.body, tables, folds),
                folds,
                recursive: is_recursive,
                valued,
            })
        }
    }
}


/// Whether the copy diagnostics run: always in a test build, and under
/// `CINCH_INLINE_CHECK` in any build, so a release compiler can be made to
/// check the invariant it is suspected of breaking.
fn check_copies() -> bool {
    use std::sync::LazyLock;
    static ON: LazyLock<bool> = LazyLock::new(|| {
        cfg!(debug_assertions) || std::env::var_os("CINCH_INLINE_CHECK").is_some()
    });
    *ON
}

/// Whether the candidate trace is on (`CINCH_INLINE_TRACE`).
fn opts_trace() -> bool {
    use std::sync::LazyLock;
    static ON: LazyLock<bool> = LazyLock::new(|| std::env::var_os("CINCH_INLINE_TRACE").is_some());
    *ON
}

/// The locals a copy of a callee's body reads free, sorted by id. A copy
/// before its substitution must have exactly its parameter ids free: any
/// other id is a local of the copy's original unit, which the copy does not
/// own, and the ownership verifier then reports it as a read of a dead
/// binder.
fn copy_free_locals(e: &Core) -> Vec<LocalId> {
    let mut out: Vec<LocalId> = Vec::new();
    free_locals_in(e, &mut out, true);
    out.sort_by_key(|l| l.0);
    out
}

/// Count the references to every unit in `e`: a saturated direct call of a
/// unit with its arity, or a value use (a bare reference, an unsaturated
/// application, or anything inside a quotation's retained body, where the
/// evaluator applies closures). `edges` collects every unit reference for
/// the recursion test.
fn collect_refs(
    e: &Core,
    arity: &HashMap<String, usize>,
    quoted: bool,
    calls: &mut HashMap<String, usize>,
    values: &mut HashSet<String>,
    edges: &mut Vec<String>,
) {
    match e {
        Core::Global(g, _) => {
            values.insert(g.0.clone());
            edges.push(g.0.clone());
        }
        Core::App(head, args) => {
            match &**head {
                Core::Global(g, _) if !quoted && arity.get(&g.0) == Some(&args.len()) => {
                    *calls.entry(g.0.clone()).or_insert(0) += 1;
                    edges.push(g.0.clone());
                }
                other => collect_refs(other, arity, quoted, calls, values, edges),
            }
            for a in args {
                collect_refs(a, arity, quoted, calls, values, edges);
            }
            return;
        }
        Core::Quote { holes, body, .. } => {
            for h in holes {
                collect_refs(h, arity, quoted, calls, values, edges);
            }
            if let Some(body) = body {
                collect_refs(body, arity, true, calls, values, edges);
            }
            return;
        }
        Core::DeclsQuote { holes, .. } => {
            for h in holes {
                collect_refs(h, arity, quoted, calls, values, edges);
            }
            return;
        }
        _ => {}
    }
    for k in child_slots_imm(e) {
        collect_refs(k, arity, quoted, calls, values, edges);
    }
}



// ------------------------------------------------- the unfoldable class
//
// "Cheap by construction" is the class of bodies whose every node is work
// the code generator performs at the site of the node: a literal, a read of
// a binder the copy renames, a primop's instruction sequence, a projection,
// a value binder, a shallow decision, or a call to another unit in the same
// class. The class is what makes a dictionary method disappear at its call
// site, and it is transitive so a method that delegates to a method closes
// in one unfolding.
//
// The class deliberately excludes everything whose copy creates a new
// scope, a new allocation site, or an effect whose time matters: a closure
// (`Lam`), a quotation (a retained body with its own binding scope), a join
// (a per-unit identity), a lazy thunk, a storage effect, a dictionary
// construction, and a constructor application in value position (one block
// per copy instead of one per definition). A constructor application in
// scrutinee position is admitted because the decision over it folds, which
// is the "shallow case on a known constructor" the policy is about.
//
// Units in a recursive group never reach the class. That keeps the
// transitive walk acyclic, so it can be memoized, and it keeps a
// self-recursive worker out of the stack rule's way.

/// The facts one body accumulates while it is classified: whether it calls
/// a unit (a link the recursive rule may expand) and how many decisions it
/// carries (at most one).
#[derive(Clone, Copy, Default)]
struct Shape {
    /// A call whose head is a unit: an unfoldable link, or a constructor.
    chain: bool,
    /// `Case` nodes in the body.
    cases: usize,
}

/// Whether a body is in the unfoldable class, given `admit` for its
/// callees. `scrutinee` marks a position the decision over it folds.
fn unfold_body(
    e: &Core,
    tables: &Tables,
    admit: &dyn Fn(&str) -> bool,
    scrutinee: bool,
    shape: &mut Shape,
) -> bool {
    let value = |child: &Core, shape: &mut Shape| unfold_body(child, tables, admit, false, shape);
    match e {
        Core::Int(_)
        | Core::Bool(_)
        | Core::Unit
        | Core::ConstBig { .. }
        | Core::ConstBytes(_)
        | Core::ConstString { .. }
        | Core::ConstSym(_)
        | Core::StrLit(_)
        | Core::Var(_) => true,
        // A unit, an intrinsic or a foreign symbol used as a value is a
        // constant the module already carries, not an allocation.
        Core::Global(..) | Core::Prim(_) | Core::Foreign(_) => true,
        // A constructor with no fields is a static block, so a copy of it is
        // a constant. One with fields allocates per copy.
        Core::Ctor(id, _) => tables.ctors.get(&id.0).is_some_and(|(_, fields)| *fields == 0),
        Core::Proj(inner, _) | Core::DictProj(inner, ..) => value(inner, shape),
        Core::Let(_, _, bound, body) => value(bound, shape) && value(body, shape),
        Core::App(head, args) => match &**head {
            Core::Prim(_) | Core::Foreign(_) => args.iter().all(|a| value(a, shape)),
            Core::Global(g, _) => {
                // A constructor unit is an allocation, admitted only where
                // the decision over it folds.
                if tables.ctors.contains_key(&g.0) {
                    return scrutinee && args.iter().all(|a| value(a, shape));
                }
                shape.chain = true;
                admit(&g.0) && args.iter().all(|a| value(a, shape))
            }
            Core::Ctor(..) => scrutinee && args.iter().all(|a| value(a, shape)),
            // A call through a local, a projection or a closure is not a
            // unit call the class can follow.
            _ => false,
        },
        Core::Case { scrut, alts } => {
            shape.cases += 1;
            if shape.cases > 1 {
                return false;
            }
            if !unfold_body(scrut, tables, admit, true, shape) {
                return false;
            }
            alts.iter().all(|a| value(&a.body, shape))
        }
        Core::If(condition, then, otherwise) => {
            value(condition, shape) && value(&then.body, shape) && value(&otherwise.body, shape)
        }
        Core::Dup(_, _, body) | Core::Drop(_, _, body) => value(body, shape),
        _ => false,
    }
}

/// Whether `e` is a constructor application, which is the only scrutinee
/// shape `known_constructor` folds: a copy that carries a decision is
/// admitted at a site without a fold test only when it is small enough that
/// the duplicated decision costs no more than the call it replaces.
fn known_constructor_value(e: &Core, tables: &Tables) -> bool {
    ctor_app(e, tables).is_some()
}

/// The parameters a body scrutinizes in a `Case`, in order. A copy of a
/// decision is admitted at a site only when each of these arguments is a
/// known constructor, so the copy folds in the same pass.
fn case_scrutinee_params(e: &Core, out: &mut Vec<usize>) {
    if let Core::Case { scrut, alts } = e {
        if let Core::Var(v) = &**scrut {
            if !out.contains(&v.0) {
                out.push(v.0);
            }
        }
        for a in alts {
            case_scrutinee_params(&a.body, out);
        }
        return;
    }
    if let Core::App(head, args) = e {
        case_scrutinee_params(head, out);
        for a in args {
            case_scrutinee_params(a, out);
        }
        return;
    }
    for child in child_slots_imm(e) {
        case_scrutinee_params(child, out);
    }
}

/// The unfoldable units of a program and of its foreign body table: names
/// whose bodies are in the class, with the shape each one has. Computed once
/// per program, before any copy, because the class reads declarations and
/// not sites.
fn unfoldable_units(
    program: &Program,
    foreign: &[FnDef],
    tables: &Tables,
    recursive: &HashSet<String>,
) -> HashMap<String, Shape> {
    let bodies: HashMap<&str, &Core> = program
        .fns
        .iter()
        .chain(foreign.iter())
        .filter(|f| f.prim.is_none() && f.ctor.is_none() && !f.meta.prim_body)
        .map(|f| (f.id.0.as_str(), &f.body))
        .collect();
    let mut known: HashMap<String, Option<Shape>> = HashMap::new();
    for f in program.fns.iter().chain(foreign.iter()) {
        let name = f.id.0.clone();
        if known.contains_key(&name) {
            continue;
        }
        classify(&name, &bodies, tables, recursive, &mut known);
    }
    known
        .into_iter()
        .filter_map(|(name, shape)| shape.map(|s| (name, s)))
        .collect()
}

/// Every unit a body applies at the head of a saturated call, in order. The
/// class walk follows these and no other node, so this is the dependency
/// list it needs.
fn body_callees(e: &Core, out: &mut Vec<String>) {
    if let Core::App(head, args) = e {
        if let Core::Global(g, _) = &**head {
            if !out.contains(&g.0) {
                out.push(g.0.clone());
            }
        }
        body_callees(head, out);
        for a in args {
            body_callees(a, out);
        }
        return;
    }
    for child in child_slots_imm(e) {
        body_callees(child, out);
    }
}

/// Classify one unit, memoizing the answer. The graph is acyclic because
/// every recursive unit is excluded before this runs, but the provisional
/// `None` a name gets while it is being classified makes the walk total if
/// that ever stops being true: a cycle reads as not admitted instead of
/// recursing forever.
fn classify(
    name: &str,
    bodies: &HashMap<&str, &Core>,
    tables: &Tables,
    recursive: &HashSet<String>,
    known: &mut HashMap<String, Option<Shape>>,
) -> Option<Shape> {
    if recursive.contains(name) {
        known.insert(name.to_string(), None);
        return None;
    }
    if let Some(answer) = known.get(name) {
        return answer.clone();
    }
    let Some(body) = bodies.get(name) else {
        known.insert(name.to_string(), None);
        return None;
    };
    known.insert(name.to_string(), None);
    let mut callees: Vec<String> = Vec::new();
    body_callees(body, &mut callees);
    for callee in &callees {
        classify(callee, bodies, tables, recursive, known);
    }
    let admitted = |callee: &str| known.get(callee).is_some_and(Option::is_some);
    let mut shape = Shape::default();
    let answer = unfold_body(body, tables, &admitted, false, &mut shape).then_some(shape);
    known.insert(name.to_string(), answer.clone());
    answer
}


/// The units that are recursive, directly or through a cycle: the members of
/// every strongly connected component with more than one node, and every
/// unit with an edge to itself. Tarjan's algorithm, iterative so a deep call
/// graph cannot overflow the native stack.
fn recursive_units(edges: &HashMap<String, Vec<String>>) -> HashSet<String> {
    struct Node<'a> {
        name: &'a str,
        next: usize,
    }
    let mut index: HashMap<&str, usize> = HashMap::new();
    let mut low: HashMap<&str, usize> = HashMap::new();
    let mut on_stack: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = Vec::new();
    let mut out: HashSet<String> = HashSet::new();
    let mut counter = 0usize;
    for root in edges.keys() {
        if index.contains_key(root.as_str()) {
            continue;
        }
        let mut frames: Vec<Node> = vec![Node { name: root.as_str(), next: 0 }];
        index.insert(root.as_str(), counter);
        low.insert(root.as_str(), counter);
        counter += 1;
        stack.push(root.as_str());
        on_stack.insert(root.as_str());
        while let Some(frame) = frames.last_mut() {
            let name = frame.name;
            let children = edges.get(name).map(Vec::as_slice).unwrap_or(&[]);
            if frame.next < children.len() {
                let child = children[frame.next].as_str();
                frame.next += 1;
                let Some(child_edges) = edges.get(child) else { continue };
                let _ = child_edges;
                if !index.contains_key(child) {
                    index.insert(child, counter);
                    low.insert(child, counter);
                    counter += 1;
                    stack.push(child);
                    on_stack.insert(child);
                    frames.push(Node { name: child, next: 0 });
                } else if on_stack.contains(child) {
                    let i = index[child];
                    let l = low[name];
                    low.insert(name, l.min(i));
                }
                continue;
            }
            // Every child is done: close this node.
            let me = low[name];
            let parent = frames.len().checked_sub(2).map(|i| frames[i].name);
            frames.pop();
            if let Some(parent) = parent {
                let p = low[parent];
                low.insert(parent, p.min(me));
            }
            if Some(me) == low.get(name).copied() && index[name] == me {
                // `name` roots a component: pop it off the stack.
                let mut component: Vec<&str> = Vec::new();
                while let Some(top) = stack.pop() {
                    on_stack.remove(top);
                    component.push(top);
                    if top == name {
                        break;
                    }
                }
                let self_edge = edges
                    .get(name)
                    .is_some_and(|cs| cs.iter().any(|c| c == name));
                if component.len() > 1 || self_edge {
                    out.extend(component.into_iter().map(str::to_string));
                }
            }
        }
    }
    out
}

/// Inline every call site `table` admits in one unit's body. Returns the
/// nodes the copies added.
///
/// `stack` is the unfolding chain: a call to a unit already on it is
/// refused, which is what makes the recursion terminate when every bound
/// would otherwise be removed. `depth` is the number of links already
/// closed in this chain, bounded by `opts.inline_depth`.
#[allow(clippy::too_many_arguments)]
fn inline_unit(
    e: &mut Core,
    table: &InlineTable,
    tables: &Tables,
    fresh: &mut Fresh,
    budget: &mut usize,
    facts: &mut InlineFacts,
    t: &mut Ticks,
    stack: &mut Vec<String>,
    depth: usize,
    // Whether this position is inside a loop body (the enclosing unit is a
    // tail worker or a member of a recursive group), which divides the
    // benefit model's allowance at every site below it.
    loop_body: bool,
    tally: &mut BenefitTally,
    opts: &SimplifyOptions,
) {
    // Children first, so a call nested in an argument is inlined before the
    // call whose argument it is.
    match e {
        Core::App(head, args) => {
            inline_unit(head, table, tables, fresh, budget, facts, t, stack, depth, loop_body, tally, opts);
            for a in args {
                inline_unit(a, table, tables, fresh, budget, facts, t, stack, depth, loop_body, tally, opts);
            }
        }
        Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
            // A quotation's retained body is a code unit of its own.
            for h in holes {
                inline_unit(h, table, tables, fresh, budget, facts, t, stack, depth, loop_body, tally, opts);
            }
            return;
        }
        _ => {
            for k in child_slots(e) {
                inline_unit(k, table, tables, fresh, budget, facts, t, stack, depth, loop_body, tally, opts);
            }
        }
    }
    let Core::App(head, args) = e else { return };
    let Core::Global(g, _) = &**head else { return };
    let Some(callee) = table.get(&g.0) else { return };
    if tables.arity.get(&g.0) != Some(&args.len()) {
        return;
    }
    // The stack rule: a unit already being unfolded in this chain is never
    // unfolded again, so a cycle of admitted units cannot recurse without
    // end. Independent of it, the depth cap bounds a long acyclic chain and
    // the node budget bounds the program-wide growth.
    if depth >= opts.inline_depth || stack.iter().any(|name| name == &g.0) {
        tally.refused_chain += 1;
        return;
    }
    // A callee in a recursive group is refused at every site: its body
    // reaches itself, so a copy would be a copy of a function that calls
    // itself, and the stack rule alone would let the chain grow to the depth
    // cap. The threshold model also refuses it at admission; this test is
    // what makes the benefit model's admission safe.
    if callee.recursive {
        tally.refused_recursive += 1;
        return;
    }
    // A decision-bearing body is copied only where the decision folds: a
    // copy with an unknown scrutinee would duplicate the branch instead of
    // removing it. The test is about *duplication*, so it does not apply at
    // a body's only site: there the branch moves into the caller and the
    // callee is pruned, which is one copy of the decision either way.
    // Under the benefit model the fold is also what the benefit term
    // credits, so the two cannot disagree.
    let duplicated = callee.sites > 1;
    if callee.fold
        && duplicated
        && !callee.cases.iter().all(|i| {
            args.get(*i).is_some_and(|a| known_constructor_value(a, tables))
        })
    {
        tally.refused_fold += 1;
        return;
    }
    let added = nodes(&callee.body);
    match opts.inline_model {
        // The site test (design/inline-unfolding-policy.md section 10): the
        // copy pays for the call it removes and for every argument that
        // already costs something, and an interesting argument pays more
        // because the copy will fold it. What is left is compared with the
        // use threshold.
        InlineModel::Threshold => {
            if !callee.certain {
                let mut discount = 10usize;
                for a in args.iter() {
                    discount += if trivial_arg(a, tables) {
                        10 * usize::from(interesting_arg(a, tables))
                    } else {
                        10 + 10 * usize::from(interesting_arg(a, tables))
                    };
                }
                if callee.weight.saturating_sub(discount) > opts.inline_use_threshold {
                    tally.refused_size += 1;
                    return;
                }
            }
        }
        // Flambda 2's site test
        // (`simplify/inlining/call_site_inlining_decision.ml:211-310`):
        // `size * sites - benefit <= threshold`. The body's code exists once
        // per site after the copies and once now, so the growth is
        // `size * sites` against the `size` already there; the call the copy
        // removes is inside the benefit, so no separate subtraction is
        // needed. A body at or below `small_function_size` skips the test
        // (`Always_inline`), and a site inside a loop body divides its
        // allowance because the copy is entered on every iteration while the
        // benefit is paid once.
        InlineModel::Benefit => {
            if !callee.certain {
                let growth = callee.size.saturating_mul(callee.sites.max(1));
                let removed = callee.benefit;
                let allowance = if loop_body {
                    opts.inline_size_threshold / opts.inline_loop_discount.max(1)
                } else {
                    opts.inline_size_threshold
                };
                if growth.saturating_sub(removed) > allowance {
                    tally.refused_size += 1;
                    return;
                }
            }
        }
    }
    if callee.charge {
        if added > *budget {
            tally.refused_budget += 1;
            return;
        }
        *budget -= added;
    }
    tally.inlined += 1;
    facts.sites += 1;
    facts.nodes += added as u32;
    if opts_trace() {
        eprintln!(
            "cinch-inline: {} <= {} ({} nodes, {} site(s))",
            current_unit(),
            g.0,
            added,
            facts.sites
        );
    }
    facts.callees.push(g.0.clone());
    t.fire(Rule::Inline);
    let name = g.0.clone();
    let args = args.clone();
    let mut copy = callee.body.clone();
    let mut renames: HashMap<LocalId, LocalId> = HashMap::new();
    rename_binders(&mut copy, &mut renames, fresh);
    // The parameters are renamed to fresh ids before any substitution. The
    // copy is then closed over ids no caller expression can mention: the
    // parameters are `LocalId(0..arity)`, and a caller's own locals use the
    // same numbers, so substituting a parameter directly would leave a
    // `Var(k)` that a later parameter's substitution could rewrite.
    let params: Vec<LocalId> = (0..args.len()).map(|_| fresh.local()).collect();
    for (i, p) in params.iter().enumerate() {
        subst(&mut copy, LocalId(i), &Core::Var(*p));
    }
    if check_copies() {
        // The invariant the substitution relies on: after the binder and
        // parameter renaming, every local the copy reads free is one of its
        // parameters (a body need not read all of them). A local outside
        // that set is one the copy does not own: the original unit's binder,
        // which the caller's body cannot supply, and which the ownership
        // verifier then refuses.
        let free = copy_free_locals(&copy);
        let mut want = params.clone();
        want.sort_by_key(|l| l.0);
        assert!(
            free.iter().all(|l| want.contains(l)),
            "inline: the copy of {} at {} reads free locals {:?} outside its parameters {:?}: {:?}",
            name,
            current_unit(),
            free,
            want,
            copy
        );
    }
    // A local argument is substituted where the parameter is read. Any other
    // argument is bound to a fresh local in the order the language
    // evaluates it, so nothing moves past an effect and a parameter read
    // twice reads the same value.
    let mut bound: Vec<(LocalId, Class, Core)> = Vec::new();
    for (i, a) in args.iter().enumerate() {
        let p = params[i];
        if matches!(a, Core::Var(_)) {
            subst(&mut copy, p, a);
            continue;
        }
        let t = fresh.local();
        let class = callee.params.get(i).copied().unwrap_or(Class::Raw);
        subst(&mut copy, p, &Core::Var(t));
        bound.push((t, class, a.clone()));
    }
    for (t, class, value) in bound.into_iter().rev() {
        copy = Core::Let(t, class, Box::new(value), Box::new(copy));
    }
    // The recursive half: the copy's own calls are offered the same rule
    // with the callee on the stack. Without this a delegation stays a call
    // (`Eq_Nat.==` would still name `natCmp` through a second wrapper), and
    // the chain is what the depth cap and the stack bound.
    stack.push(name);
    inline_unit(&mut copy, table, tables, fresh, budget, facts, t, stack, depth + 1, loop_body, tally, opts);
    stack.pop();
    *e = copy;
}

/// Give one binder a fresh id, checking the copy's own invariant: the body
/// binds each id once. Two binders of one body sharing an id means one of
/// them was already a copy, and renaming it here would make the two into one
/// binder (a capture the ownership verifier then reports as a read of a dead
/// local).
fn fresh_binder(x: &mut LocalId, renames: &mut HashMap<LocalId, LocalId>, fresh: &mut Fresh) {
    let id = fresh.local();
    let old = *x;
    if renames.insert(old, id).is_some() && check_copies() {
        panic!(
            "inline: two binders of one body share id {} in {}, so a copy would capture it",
            old.0,
            current_unit()
        );
    }
    *x = id;
}

/// Give every binder of `e` a fresh local id: a copy of a callee's body must
/// not share a binder with its original or capture one of the caller's.
/// Parameter ids are not binders here, so they stay for the substitution
/// that follows.
fn rename_binders(e: &mut Core, renames: &mut HashMap<LocalId, LocalId>, fresh: &mut Fresh) {
    let rename = |x: &mut LocalId, renames: &HashMap<LocalId, LocalId>| {
        if let Some(n) = renames.get(x) {
            *x = *n;
        }
    };
    match e {
        // A use of a binder: the whole point of the pass. Every `Var` the
        // copy reads through a renamed binder is renamed with it, which is
        // what makes the copy closed over its parameters alone. Dropping
        // this arm leaves the copy reading the original's ids, which the
        // caller's body does not bind.
        Core::Var(x) => rename(x, renames),
        Core::Global(..) | Core::Ctor(..) | Core::Prim(_) | Core::Foreign(_) => {}
        Core::Int(_)
        | Core::ConstBig { .. }
        | Core::ConstBytes(_)
        | Core::ConstString { .. }
        | Core::ConstSym(_)
        | Core::Bool(_)
        | Core::Unit
        | Core::SpliceVal(_)
        | Core::Fault
        | Core::LazyAlloc
        | Core::BoxAlloc
        | Core::StrLit(_)
        | Core::SyntaxLit(..) => {}
        Core::Dup(x, _, body) | Core::Drop(x, _, body) => {
            rename(x, renames);
            rename_binders(body, renames, fresh);
        }
        Core::Release { block, moved, rest, token, body } => {
            rename(block, renames);
            for (x, _) in moved.iter_mut().chain(rest.iter_mut()) {
                rename(x, renames);
            }
            if let Some(t) = token {
                rename(t, renames);
            }
            rename_binders(body, renames, fresh);
        }
        Core::Reuse { token, args, .. } => {
            rename(token, renames);
            for a in args {
                rename_binders(a, renames, fresh);
            }
        }
        Core::BoxInit(x, v, _) | Core::RefSet(x, v, _) | Core::LazyInit(x, v) => {
            rename(x, renames);
            rename_binders(v, renames, fresh);
        }
        Core::BoxGet(d, _) | Core::BoxClear(d, _) | Core::Demand(d, _) | Core::Proj(d, _) => {
            rename_binders(d, renames, fresh);
        }
        Core::DictProj(d, ..) => rename_binders(d, renames, fresh),
        Core::App(head, args) => {
            rename_binders(head, renames, fresh);
            for a in args {
                rename_binders(a, renames, fresh);
            }
        }
        Core::InstDict(_, args, _) => {
            for a in args {
                rename_binders(a, renames, fresh);
            }
        }
        Core::Lam { env, .. } => {
            for a in env {
                rename_binders(a, renames, fresh);
            }
        }
        Core::Let(x, _, v, body) => {
            rename_binders(v, renames, fresh);
            fresh_binder(x, renames, fresh);
            rename_binders(body, renames, fresh);
        }
        Core::Case { scrut, alts } => {
            rename_binders(scrut, renames, fresh);
            for a in alts {
                match &mut a.pat {
                    CPat::Var(x, _) => fresh_binder(x, renames, fresh),
                    CPat::Con { fields, .. } => {
                        for x in fields.iter_mut() {
                            fresh_binder(x, renames, fresh);
                        }
                    }
                    CPat::Wild => {}
                }
                rename_binders(&mut a.body, renames, fresh);
            }
        }
        Core::Join { params, body, rest, .. } => {
            for (x, _) in params.iter_mut() {
                fresh_binder(x, renames, fresh);
            }
            rename_binders(body, renames, fresh);
            rename_binders(rest, renames, fresh);
        }
        Core::Jump { args, .. } => {
            for a in args {
                rename_binders(a, renames, fresh);
            }
        }
        Core::If(c, th, el) => {
            rename_binders(c, renames, fresh);
            rename_binders(&mut th.body, renames, fresh);
            rename_binders(&mut el.body, renames, fresh);
        }
        Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
            for h in holes {
                rename_binders(h, renames, fresh);
            }
        }
    }
}

/// One past the largest local any unit mentions: every fresh local the pass
/// makes exceeds it.
fn max_local(program: &Program) -> usize {
    let mut next = 0;
    for f in &program.fns {
        crate::core::walk_locals(&f.body, &mut |l| next = next.max(l.0 + 1));
    }
    next
}

/// One past the largest join id any unit mentions.
fn max_join(program: &Program) -> usize {
    fn go(e: &Core, next: &mut usize) {
        if let Core::Join { id, .. } | Core::Jump { id, .. } = e {
            *next = (*next).max(id.0 + 1);
        }
        for child in child_slots_imm(e) {
            go(child, next);
        }
    }
    let mut next = 0;
    for f in &program.fns {
        go(&f.body, &mut next);
    }
    next
}

// ------------------------------------------------- dead-unit elimination
//
// Inlining is the pass that makes the frame allocation pass matter (it moves
// an allocation from a callee into its caller's activation), but a copy
// leaves the callee's body behind: a unit whose every call site was inlined
// is never called again, and one that a rewrite emptied is never called at
// all. Those units are code the artifact still pays for, and they are what
// keeps the inliner's node budget (two percent of the program) the binding
// constraint rather than the size of the program.
//
// `prune` is a reachability sweep over the call graph with the program's
// real roots, run after inlining and before reference counting:
//
// - `program.main`, the synthesized entry adapter;
// - every `foreign export` target, and every Wasm world binding's
//   implementation unit (the C or component side calls those, not Core);
// - every instance's dictionary unit and its methods (a dictionary's words
//   are code pointers, so a method is reached through a `DictProj` the scan
//   cannot see through);
// - every constructor unit, because the layout table's ids and the tags a
//   `case` compares against name them, and a block built by another module
//   carries one of those ids;
// - every `prim` declaration's unit, whose content is inline target IR
//   rather than a Core body;
// - every boundary layout plan's unit (`repr(C)` and friends);
// - every unit of a module an object or package build emits
//   (`emit_modules`), because an interface value of that module may be
//   called from outside the artifact.
//
// A unit is then live when a live unit mentions it, directly or as the
// `Lam` of a closure, and everything else is removed. "Mentions" is the
// conservative relation: any `Core::Global`, `Core::Ctor` or `Lam`'s unit in
// any position, including a quotation's retained body, which code
// generation emits as part of the unit that writes it.
//
// The relations this relies on: a dictionary's method units are referenced
// by the `InstanceDef` (rooted), a constructor's unit by the layout table
// (rooted), and no `Core::Global` names a unit outside `program.fns` for a
// unit this program must emit (`Prim` and `Foreign` name runtime symbols,
// not units).

/// The units removed by [`prune`], for the artifact manifest.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PruneReport {
    /// The units left after the sweep.
    pub kept: usize,
    /// The units removed.
    pub removed: usize,
    /// The Core nodes those units held (the IR the artifact no longer
    /// carries).
    pub removed_nodes: usize,
}

/// Whether `unit` is the root `name` itself or a unit derived from it. A
/// derivation continues with a separator: `@` for a representation variant,
/// `.` for a member or companion symbol, `#` for a tail worker.
fn derived_from(unit: &str, name: &str) -> bool {
    unit.len() >= name.len()
        && unit.starts_with(name)
        && (unit.len() == name.len()
            || matches!(unit.as_bytes()[name.len()], b'@' | b'.' | b'#'))
}

/// Remove every unit no live unit can reach, with the program's real roots
/// (`keep`, in addition to the ones the program itself names): an object or
/// package build's emitted modules and its object roots.
pub fn prune(program: &mut Program, keep: &[String]) -> PruneReport {
    prune_excluding(program, keep, &HashSet::new())
}

/// [`prune`], with the blanket roots (every constructor, primitive,
/// boundary plan and instance) withheld from the units of `excluded`
/// modules: a library object built for a target leaves out the modules
/// that target never selects (a platform backend imported only `when os
/// darwin`, on Linux), and their bindings stay only if a live unit really
/// reaches them.
pub fn prune_excluding(program: &mut Program, keep: &[String], excluded: &HashSet<String>) -> PruneReport {
    let in_excluded = |name: &str| {
        excluded
            .iter()
            .any(|m| name.len() > m.len() && name.starts_with(m.as_str()) && name.as_bytes()[m.len()] == b'.')
    };
    let present: HashSet<&str> = program.fns.iter().map(|f| f.id.0.as_str()).collect();
    let mut live: HashSet<String> = HashSet::new();
    let mut work: Vec<String> = Vec::new();
    let mut root = |name: &str, live: &mut HashSet<String>, work: &mut Vec<String>| {
        if present.contains(name) && live.insert(name.to_string()) {
            work.push(name.to_string());
        }
    };
    if let Some(main) = &program.main {
        root(&main.0, &mut live, &mut work);
    }
    for name in program.plans.keys() {
        if !in_excluded(name) {
            root(name, &mut live, &mut work);
        }
    }
    for plan in program.foreigns.values() {
        if plan.export && !in_excluded(&plan.unit) {
            root(&plan.unit, &mut live, &mut work);
        }
    }
    for inst in &program.instances {
        if in_excluded(&inst.id.0) {
            continue;
        }
        root(&inst.id.0, &mut live, &mut work);
        for m in &inst.methods {
            root(&m.0, &mut live, &mut work);
        }
    }
    for f in &program.fns {
        if (f.ctor.is_some() || f.prim.is_some() || f.meta.prim_body) && !in_excluded(&f.id.0) {
            root(&f.id.0, &mut live, &mut work);
        }
    }
    // A root also covers every unit derived from it: a representation
    // variant (`name@RT`, the copy the object roots' class vectors name),
    // a member or companion symbol (`name.clo`, `name.app`,
    // `name.object-closure`), and a tail worker (`name#tail`). Missing the
    // `@` variants is what left a demanded unit out of the object.
    let mut names: Vec<&str> = program.fns.iter().map(|f| f.id.0.as_str()).collect();
    names.sort_unstable();
    for name in keep {
        let start = names.partition_point(|unit| *unit < name.as_str());
        for unit in &names[start..] {
            if !unit.starts_with(name.as_str()) {
                break;
            }
            if derived_from(unit, name) {
                root(unit, &mut live, &mut work);
            }
        }
    }
    // The call graph's edges: every unit a body names, in any position.
    let bodies: HashMap<&str, &Core> = program
        .fns
        .iter()
        .filter(|f| f.prim.is_none() && f.ctor.is_none() && !f.meta.prim_body)
        .map(|f| (f.id.0.as_str(), &f.body))
        .collect();
    let mut mentioned: Vec<String> = Vec::new();
    // Every name a live body mentions, units or not: the foreign wrappers
    // of an excluded module stay only if one of these calls them.
    let mut reached: HashSet<String> = HashSet::new();
    while let Some(unit) = work.pop() {
        let Some(body) = bodies.get(unit.as_str()) else { continue };
        mentioned.clear();
        unit_mentions(body, &mut mentioned);
        if !excluded.is_empty() {
            reached.extend(mentioned.iter().cloned());
        }
        for name in &mentioned {
            if present.contains(name.as_str()) && live.insert(name.clone()) {
                work.push(name.clone());
            }
        }
    }
    let before: usize = program.fns.len();
    let mut removed_nodes = 0;
    program.fns.retain(|f| {
        let keep = live.contains(&f.id.0);
        if !keep {
            removed_nodes += nodes(&f.body);
        }
        keep
    });
    // The per-unit facts name units too; a removed unit's entry is stale.
    program.inline.retain(|unit, _| live.contains(unit));
    program.frame.retain(|unit, _| live.contains(unit));
    if !excluded.is_empty() {
        program.foreigns.retain(|name, plan| plan.export || !in_excluded(name) || reached.contains(name));
    }
    PruneReport { kept: program.fns.len(), removed: before - program.fns.len(), removed_nodes }
}

/// Every unit the body `e` names anywhere: a direct call or a `Global` in
/// value position, a constructor unit, and the unit behind a `Lam`'s code
/// pointer. Recurses through every child, including a quotation's retained
/// body, which code generation emits inline.
fn unit_mentions(e: &Core, out: &mut Vec<String>) {
    match e {
        Core::Global(g, _) | Core::Ctor(g, _) => out.push(g.0.clone()),
        Core::Foreign(name) => out.push(name.clone()),
        Core::Lam { fn_id, .. } => out.push(fn_id.0.clone()),
        Core::App(h, args) => {
            unit_mentions(h, out);
            for a in args {
                unit_mentions(a, out);
            }
            return;
        }
        Core::Quote { holes, body, .. } => {
            for h in holes {
                unit_mentions(h, out);
            }
            if let Some(b) = body {
                unit_mentions(b, out);
            }
            return;
        }
        _ => {}
    }
    for k in child_slots_imm(e) {
        unit_mentions(k, out);
    }
}

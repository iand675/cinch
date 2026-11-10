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
//! Four rules, in this slice:
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

/// The simplifier's budget and switches.
#[derive(Clone, Debug)]
pub struct SimplifyOptions {
    /// The most passes over the program (`--max-simplifier-iterations`,
    /// default 4). Zero disables the simplifier.
    pub max_iterations: u32,
    /// Resolve a projection from a known static dictionary to a direct
    /// call (`direct-dict-call`).
    pub direct_dict_calls: bool,
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
}

impl Default for SimplifyOptions {
    fn default() -> Self {
        SimplifyOptions {
            max_iterations: 4,
            direct_dict_calls: true,
            // Off by default: measured on the compiler's own bundle it grows
            // Core by about a tenth (3.22M nodes to 3.54M) even with the
            // constructor fold that collapses the decisions it produces, and
            // two units reach tens of thousands of lines from duplicated
            // alternatives. Turn it on with `--case-of-case` to measure it.
            case_of_case: false,
            join_threshold: 20,
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
    CaseOfCase,
}

/// The accumulator every pass shares: the tick total, the per-rule
/// counts, and whether the pass changed anything.
#[derive(Default)]
struct Ticks {
    ticks: usize,
    case_merge: usize,
    known_ctor: usize,
    dict_call: usize,
    case_of_case: usize,
    joins: usize,
    duplicated: usize,
    changed: bool,
}

impl Ticks {
    fn fire(&mut self, rule: Rule) {
        self.ticks += 1;
        self.changed = true;
        match rule {
            Rule::CaseMerge => self.case_merge += 1,
            Rule::KnownConstructor => self.known_ctor += 1,
            Rule::DirectDictCall => self.dict_call += 1,
            Rule::CaseOfCase => self.case_of_case += 1,
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
    opts: &SimplifyOptions,
    fresh: &mut Fresh,
    t: &mut Ticks,
) {
    // Children first: a rewrite below can expose a rewrite above it in the
    // same pass, and the reverse order would need another pass for the
    // same result.
    match e {
        Core::Let(l, _, value, body) => {
            pass(value, scope, tables, classes, opts, fresh, t);
            let alias = match &**value {
                Core::Var(w) => Some(scope.root_of(*w)),
                _ => None,
            };
            let bound = match alias {
                Some(_) => None,
                None => ctor_app(value, tables).map(|_| (**value).clone()),
            };
            if let Some(w) = alias {
                scope.alias.insert(*l, w);
            }
            if let Some(v) = bound.clone() {
                scope.ctor.insert(*l, v);
            }
            pass(body, scope, tables, classes, opts, fresh, t);
            if alias.is_some() {
                scope.alias.remove(&*l);
            }
            if bound.is_some() {
                scope.ctor.remove(&*l);
            }
        }
        _ => {
            for slot in child_slots(e) {
                pass(slot, scope, tables, classes, opts, fresh, t);
            }
        }
    }
    // A node may admit more than one rewrite; re-examine it after each
    // firing so a chain resolves without waiting for the next pass, up to
    // a small bound in case two rules undo each other.
    for _ in 0..8 {
        let fired = if known_constructor(e, scope, tables) {
            Some(Rule::KnownConstructor)
        } else if case_merge(e, scope) {
            Some(Rule::CaseMerge)
        } else if opts.direct_dict_calls && direct_dict_call(e, tables) {
            Some(Rule::DirectDictCall)
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
    // After the node's own rewrites, a `let` whose only reader was the
    // decision that just left has no reader: dropping it removes the
    // allocation. An alias let over a local is a value too.
    if let Core::Let(l, _, value, body) = &*e {
        if !mentions(body, *l) && is_value(value, tables) {
            let body = (**body).clone();
            *e = body;
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
    let mut bound: HashSet<LocalId> = HashSet::new();
    fn go(e: &Core, bound: &mut HashSet<LocalId>, out: &mut Vec<LocalId>) {
        let use_local = |x: LocalId, bound: &HashSet<LocalId>, out: &mut Vec<LocalId>| {
            if !bound.contains(&x) && !out.contains(&x) {
                out.push(x);
            }
        };
        match e {
            Core::Var(x) => use_local(*x, bound, out),
            Core::Let(x, _, v, body) => {
                go(v, bound, out);
                bound.insert(*x);
                go(body, bound, out);
                bound.remove(x);
            }
            Core::Case { scrut, alts } => {
                go(scrut, bound, out);
                for a in alts {
                    let binders: Vec<LocalId> = pat_binders(&a.pat).iter().map(|(l, _)| *l).collect();
                    for b in &binders {
                        bound.insert(*b);
                    }
                    go(&a.body, bound, out);
                    for b in &binders {
                        bound.remove(b);
                    }
                }
            }
            Core::Join { params, body, rest, .. } => {
                for (p, _) in params {
                    bound.insert(*p);
                }
                go(body, bound, out);
                for (p, _) in params {
                    bound.remove(p);
                }
                go(rest, bound, out);
            }
            Core::Dup(x, _, body) | Core::Drop(x, _, body) => {
                use_local(*x, bound, out);
                go(body, bound, out);
            }
            Core::Release { block, moved, rest, body } => {
                use_local(*block, bound, out);
                for (l, _) in moved.iter().chain(rest) {
                    use_local(*l, bound, out);
                }
                go(body, bound, out);
            }
            Core::BoxInit(l, v, _) | Core::LazyInit(l, v) | Core::RefSet(l, v, _) => {
                go(v, bound, out);
                use_local(*l, bound, out);
            }
            _ => {
                for k in child_slots_imm(e) {
                    go(k, bound, out);
                }
            }
        }
    }
    go(e, &mut bound, out);
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
    report.max_iterations = opts.max_iterations as usize;
    if opts.max_iterations == 0 {
        return;
    }
    let tables = Tables::new(program);
    let mut ticks = Ticks::default();
    let mut fresh = Fresh { local: max_local(program), join: max_join(program) };
    for _ in 0..opts.max_iterations {
        ticks.changed = false;
        for f in program.fns.iter_mut() {
            if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
                continue;
            }
            let mut scope = Scope::default();
            let classes = unit_classes(f);
            let mut body = std::mem::replace(&mut f.body, Core::Unit);
            pass(&mut body, &mut scope, &tables, &classes, opts, &mut fresh, &mut ticks);
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
    report.dict_call = ticks.dict_call;
    report.case_of_case = ticks.case_of_case;
    report.joins = ticks.joins;
    report.duplicated = ticks.duplicated;
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

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
//! Three rules, in this slice:
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
//! Case-of-case and join points are deliberately absent: they need a
//! join node in Core and a printer for it, and they are the next slice
//! once this one is measured.
use crate::core::{
    child_slots, child_slots_imm, alpha_eq, CAlt, CPat, Core, GlobalId, LocalId, Program, SimplifyReport,
};
use std::collections::HashMap;

/// The simplifier's budget and switches.
#[derive(Clone, Debug)]
pub struct SimplifyOptions {
    /// The most passes over the program (`--max-simplifier-iterations`,
    /// default 4). Zero disables the simplifier.
    pub max_iterations: u32,
    /// Resolve a projection from a known static dictionary to a direct
    /// call (`direct-dict-call`).
    pub direct_dict_calls: bool,
}

impl Default for SimplifyOptions {
    fn default() -> Self {
        SimplifyOptions { max_iterations: 4, direct_dict_calls: true }
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
}

/// The accumulator every pass shares: the tick total, the per-rule
/// counts, and whether the pass changed anything.
#[derive(Default)]
struct Ticks {
    ticks: usize,
    case_merge: usize,
    known_ctor: usize,
    dict_call: usize,
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
        }
    }
}

/// One pass over a unit: children first, then the node's own rewrites.
/// `scope` maps a local to the constructor application a surrounding
/// `let` bound it to, which is the only form of scrutinee knowledge the
/// pass uses (the analysis-free version of GHC's occurrence information).
fn pass(e: &mut Core, scope: &mut Scope, tables: &Tables, opts: &SimplifyOptions, t: &mut Ticks) {
    // Children first: a rewrite below can expose a rewrite above it in the
    // same pass, and the reverse order would need another pass for the
    // same result.
    match e {
        Core::Let(l, _, value, body) => {
            pass(value, scope, tables, opts, t);
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
            pass(body, scope, tables, opts, t);
            if alias.is_some() {
                scope.alias.remove(&*l);
            }
            if bound.is_some() {
                scope.ctor.remove(&*l);
            }
        }
        _ => {
            for slot in child_slots(e) {
                pass(slot, scope, tables, opts, t);
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
    let Core::Var(v) = &**scrut else { return false };
    let Some(value) = scope.ctor_of(*v) else { return false };
    let Some(tag) = ctor_app(value, tables) else { return false };
    let args = ctor_args(value);
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
    for _ in 0..opts.max_iterations {
        ticks.changed = false;
        for f in program.fns.iter_mut() {
            if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
                continue;
            }
            let mut scope = Scope::default();
            let mut body = std::mem::replace(&mut f.body, Core::Unit);
            pass(&mut body, &mut scope, &tables, opts, &mut ticks);
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
}

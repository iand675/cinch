//! Reference counting insertion over `Core` (design/reclamation.md
//! section 4; Perceus, Reinking et al., PLDI 2021). The pass runs on the
//! residual program after the specialization collector (every class is
//! concrete) and before code generation, at every driver site the
//! collector runs.
//!
//! # Ownership (slice S6a)
//!
//! Every word of class `Obj` or `TObj` is a counted reference; `Raw`
//! words (and the open `Param` classes of generic bodies no copy reached)
//! are never counted. Every counted binder is OWNED: function and closure
//! parameters, `let` binders, and the pattern binders a `case`
//! alternative dups. Borrow inference is S6b.
//!
//! Each owned binder is consumed exactly once on every path: by an owned
//! use (an argument, a constructor field, a closure environment slot, a
//! dictionary argument, a quotation hole, a stored value, a returned
//! value, the closure of a dynamic application) or by a `Drop`. A use
//! that is not the binder's last on its path is preceded by a `Dup`. A
//! binder dead on a path is dropped at the start of that path: right
//! after its `let`, at the entry of each branch and alternative that does
//! not use it, and so always before a tail call, which is why no count
//! operation ever follows a `musttail` call.
//!
//! Borrowed positions read a binder without consuming it: a `case`
//! scrutinee, the container of a projection (`Proj`, `DictProj`,
//! `Demand`, `BoxGet`), and the cell a `BoxInit`, `LazyInit` or `RefSet`
//! writes. When such a read is the binder's last use, the binder is
//! dropped right after it. A projection's result is an owned reference:
//! code generation dups the loaded word by the node's class before the
//! container can be released.
//!
//! A `case` alternative's constructor fields are borrowed from the
//! scrutinee: the fields the body uses are dupped at entry, and the
//! scrutinee is then dropped if the body does not use it (Perceus's basic
//! rule; drop specialization is S6b). A variable alternative's binder is
//! the scrutinee word itself: it takes over the scrutinee's reference when
//! the body does not use the scrutinee, and dups it otherwise.
//!
//! `RefSet` replaces a binder's value: the old value dies during the
//! stored expression (its last use there consumes it, or it is dropped
//! before), and the binder owns the new one.
//!
//! A quotation body is a closed body of its own, transformed with no
//! owned parameters (its holes produce owned values); the runtime
//! evaluator (`runtime/src/eval.rs`) runs the count operations placed
//! there with the node conventions code generation uses.
//!
//! # Verification
//!
//! In debug builds (and so in every test) [`verify`] re-derives the
//! ownership of the transformed body and refuses any path on which an
//! owned binder is not consumed exactly once: a compiler-bug trap at
//! compile time (design/no-faults.md).

use crate::core::{CAlt, CBranch, CPat, Core, FnDef, GlobalId, LocalId, Program};
use crate::repr::Class;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;

type Set = BTreeSet<usize>;

/// Whether a word of class `c` is counted.
fn counted(c: Class) -> bool {
    matches!(c, Class::Obj | Class::TObj)
}

/// A parameter's class with any open parameter resolved to `Raw` (never
/// counted, which can only leak): the convention of code generation's
/// closure shapes.
fn concrete(c: Class) -> Class {
    if c.is_param() {
        Class::Raw
    } else {
        c
    }
}

/// Insert the count operations into every unit with a `Core` body, when
/// the profile reclaims ([`crate::core::TargetProfile::reclaim`]), with
/// every parameter owned. This is the form for an artifact whose units
/// other artifacts call (library objects, package objects, incremental
/// unit objects): owned parameters are the convention every separately
/// compiled caller assumes.
/// Repeated subterms move into shared units first
/// ([`crate::core::share_repeated_subterms`]): that needs the final
/// specialized program and must precede counting, which balances each
/// unit separately.
pub fn insert(program: Program, profile: &crate::core::TargetProfile) -> crate::Result<Program> {
    insert_with(program, profile, false)
}

/// [`insert`] for a closed program, one whose every caller of every unit
/// is in `program` (an executable emitted whole): parameters that are only
/// read are inferred borrowed ([`borrow_modes`]), so neither the caller's
/// dup nor the callee's drop is emitted for them.
pub fn insert_closed(program: Program, profile: &crate::core::TargetProfile) -> crate::Result<Program> {
    insert_with(program, profile, true)
}

fn insert_with(mut program: Program, profile: &crate::core::TargetProfile, closed: bool) -> crate::Result<Program> {
    crate::core::share_repeated_subterms(&mut program);
    if !profile.reclaim {
        return Ok(program);
    }
    name_call_arguments(&mut program);
    let ctors = Ctors::of(&program);
    let (modes, prims) = if closed { borrow_modes(&program, &ctors) } else { Default::default() };
    program.lent = modes.iter().filter(|(k, _)| prims.contains(*k)).map(|(k, m)| (k.clone(), m.clone())).collect();
    // The whole mode map reaches the stack allocation pass, which reads a
    // lent reference as one the callee cannot store or return
    // (`crate::stack::Options`).
    program.borrowed = modes.iter().map(|(k, m)| (k.clone(), m.clone())).collect();
    for f in &mut program.fns {
        if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
            continue;
        }
        let borrowed = borrowed_params(&modes, f);
        let body = transform(f, &modes, &borrowed, &ctors);
        if cfg!(debug_assertions) {
            verify(f, &body, &modes, &borrowed).map_err(|m| {
                if std::env::var_os("CINCH_RC_TRACE").is_some() {
                    eprintln!("cinch-rc: {}\n  input: {:?}\n  output: {:?}", f.id, f.body, body);
                }
                crate::Error::plain(format!("compiler bug: ownership verifier rejected {}: {m}", f.id))
            })?;
        }
        f.body = body;
    }
    Ok(program)
}

/// Borrowed parameter modes by unit name (`true` borrowed), for the units
/// that have at least one borrowed parameter.
type Modes = HashMap<String, Vec<bool>>;

fn borrowed_params(modes: &Modes, f: &FnDef) -> Set {
    modes.get(&f.id.0).map_or_else(Set::new, |m| (0..m.len()).filter(|i| m[*i]).collect())
}

/// Borrowed parameter inference (design/reclamation.md section 4, Lean 4
/// and Perceus). Only a unit every one of whose callers is a saturated
/// direct call in this program takes part: not the entry, not a `foreign
/// export` target, not an instance method, and not a unit used as a value
/// (a closure, a hoisted lambda, a quotation's reference), because those
/// callers pass owned arguments. Every counted parameter of such a unit
/// starts borrowed, and becomes owned until a fixed point when:
///
/// - the unit uses it anywhere but a borrowed position: a `case`
///   scrutinee, a projection's container, or a borrowed parameter of a
///   direct call;
/// - a call passes it a computed counted value (which the caller would
///   have to bind and drop after the call);
/// - a call in tail position passes it anything but a borrowed parameter
///   of the caller or an uncounted value, since nothing may follow a tail
///   call (the constant-stack rule) and a drop after it would be needed.
///
/// A primitive unit (a `prim`-bodied definition or a `Core::Prim`
/// symbol) takes part the same way, starting from the parameters its
/// runtime operation only reads: its wrapper releases the others on
/// return. Returns the modes and the primitive units among them.
/// The parameters that must be owned whatever [`borrow_modes`] would infer:
/// a parameter a `case` matches on whose arm could take a cell reuse. Lean
/// reaches the same rule from the other side (`OwnReason.isForced` for a
/// reset target, `PropagateBorrow`), and without it the two passes cancel:
/// a borrowed scrutinee never reaches the unique path a reuse needs.
fn reuse_scrutinees(body: &Core, ctors: &Ctors, out: &mut Set) {
    if let Core::Case { scrut, alts } = body {
        if let Core::Var(p) = &**scrut {
            for a in alts {
                if let CPat::Con { tag, fields, .. } = &a.pat {
                    if reuse_allocation_anywhere(&a.body, fields.len(), *tag, ctors) {
                        out.insert(p.0);
                    }
                }
            }
        }
    }
    for k in crate::core::child_slots_imm(body) {
        reuse_scrutinees(k, ctors, out);
    }
}

fn borrow_modes(program: &Program, ctors: &Ctors) -> (Modes, std::collections::HashSet<String>) {
    let mut excluded: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Some(main) = &program.main {
        excluded.insert(main.0.clone());
    }
    for plan in program.foreigns.values() {
        if plan.export {
            excluded.insert(plan.unit.clone());
        }
    }
    let mut prim_ops: HashMap<String, crate::core::PrimOp> = HashMap::new();
    for f in &program.fns {
        value_uses(&f.body, false, &mut excluded, &mut prim_ops);
    }
    let mut modes: Modes = HashMap::new();
    let mut prims: std::collections::HashSet<String> = std::collections::HashSet::new();
    for f in &program.fns {
        if f.ctor.is_some() || f.meta.prim_body || excluded.contains(&f.id.0) {
            continue;
        }
        let m: Vec<bool> = param_classes(f)
            .into_iter()
            .enumerate()
            .map(|(i, c)| counted(c) && f.prim.is_none_or(|op| !crate::codegen::prim_consumes(op, i)))
            .collect();
        if m.iter().any(|b| *b) {
            modes.insert(f.id.0.clone(), m);
            if f.prim.is_some() {
                prims.insert(f.id.0.clone());
            }
        }
    }
    for (symbol, op) in prim_ops {
        if excluded.contains(&symbol) {
            continue;
        }
        let m: Vec<bool> = (0..op.arity()).map(|i| !crate::codegen::prim_consumes(op, i)).collect();
        if m.iter().any(|b| *b) {
            modes.insert(symbol.clone(), m);
            prims.insert(symbol);
        }
    }
    let units: Vec<(&FnDef, UnitFacts)> = program
        .fns
        .iter()
        .filter(|f| f.prim.is_none() && f.ctor.is_none() && !f.meta.prim_body)
        .map(|f| (f, UnitFacts::of(&param_classes(f), &f.body)))
        .collect();
    let index: HashMap<&str, usize> = units.iter().enumerate().map(|(i, (f, _))| (f.id.0.as_str(), i)).collect();
    // Who calls each unit directly: a change to a unit's modes revisits
    // its callers (their parameters may now flow into an owned position)
    // and the unit itself (its tail calls see its own parameters).
    let mut callers: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, (f, _)) in units.iter().enumerate() {
        direct_callees(&f.body, &mut |k| {
            let list = callers.entry(k).or_default();
            if list.last() != Some(&i) {
                list.push(i);
            }
        });
    }
    let mut queue: Vec<usize> = (0..units.len()).rev().collect();
    let mut queued = vec![true; units.len()];
    let mut killed: Vec<String> = Vec::new();
    while let Some(u) = queue.pop() {
        queued[u] = false;
        let (f, cx) = &units[u];
        let mut mine = borrowed_params(&modes, f);
        // The unit's own parameters.
        if !mine.is_empty() {
            let names: HashMap<usize, usize> = mine.iter().map(|p| (*p, *p)).collect();
            let mut owned = Set::new();
            owned_uses(&f.body, &names, &modes, cx, &mut owned);
            // A reuse target is owned by construction (Lean's forced
            // ownership rule for a reset target).
            reuse_scrutinees(&f.body, ctors, &mut owned);
            owned.retain(|p| mine.contains(p));
            if !owned.is_empty() {
                let m = modes.get_mut(&f.id.0).expect("a unit with borrowed parameters");
                for p in &owned {
                    m[*p] = false;
                    mine.remove(p);
                }
                killed.push(f.id.0.clone());
            }
        }
        // What its calls pass to borrowed parameters.
        call_sites(&f.body, true, &mut mine, cx, &mut modes, &prims, &mut killed);
        for k in killed.drain(..) {
            for &c in callers.get(&k).into_iter().flatten().chain(index.get(k.as_str())) {
                if !queued[c] {
                    queued[c] = true;
                    queue.push(c);
                }
            }
        }
    }
    modes.retain(|_, m| m.iter().any(|b| *b));
    prims.retain(|p| modes.contains_key(p));
    (modes, prims)
}

/// The modes of a saturated call's head, when it is a unit that has
/// borrowed parameters.
fn callee_modes<'m>(modes: &'m Modes, head: &Core, args: usize) -> Option<&'m Vec<bool>> {
    match head {
        Core::Global(g, _) => modes.get(&g.0),
        Core::Prim(op) => modes.get(&op.symbol()),
        _ => None,
    }
    .filter(|m| m.len() == args)
}

/// Mark the units whose calling convention must stay owned: every unit a
/// quotation body names (`quoted`; the runtime evaluator calls it with
/// the owned convention) and every primitive used other than saturated.
/// Record the primitive operations `e` calls saturated by symbol.
fn value_uses(
    e: &Core,
    quoted: bool,
    out: &mut std::collections::HashSet<String>,
    prims: &mut HashMap<String, crate::core::PrimOp>,
) {
    match e {
        // A unit used as a value (a global word, a closure, a dictionary
        // method, a partial or over-application) is entered through its
        // apply stub, which owns the frame words and drops the ones the
        // unit lends (`codegen::stub_saturate`), so the unit keeps its
        // inferred modes. A quotation is different: the runtime evaluator
        // calls what it names with the owned convention.
        Core::Global(g, _) => {
            if quoted {
                out.insert(g.0.clone());
            }
        }
        Core::Prim(op) => {
            out.insert(op.symbol());
        }
        Core::Lam { fn_id, .. } => {
            if quoted {
                out.insert(fn_id.0.clone());
            }
        }
        Core::App(h, args) => match &**h {
            Core::Global(g, _) => {
                if quoted {
                    out.insert(g.0.clone());
                }
            }
            Core::Prim(op) => {
                if quoted || op.arity() != args.len() {
                    out.insert(op.symbol());
                } else {
                    prims.insert(op.symbol(), *op);
                }
            }
            other => value_uses(other, quoted, out, prims),
        },
        Core::Quote { body: Some(b), .. } => value_uses(b, true, out, prims),
        _ => {}
    }
    for k in crate::core::child_slots_imm(e) {
        value_uses(k, quoted, out, prims);
    }
}

/// Every direct call head of `e` that may carry modes: a unit or a
/// primitive symbol.
fn direct_callees(e: &Core, out: &mut impl FnMut(String)) {
    if let Core::App(h, _) = e {
        match &**h {
            Core::Global(g, _) => out(g.0.clone()),
            Core::Prim(op) => out(op.symbol()),
            other => direct_callees(other, out),
        }
    }
    for k in crate::core::child_slots_imm(e) {
        direct_callees(k, out);
    }
}

/// Add to `out` the parameters that `e` uses other than in a borrowed
/// position, for the parameters `names` maps from their names (the
/// parameter and its [`let_alias`] renamings).
fn owned_uses(e: &Core, names: &HashMap<usize, usize>, modes: &Modes, cx: &UnitFacts, out: &mut Set) {
    let named = |c: &Core| match c {
        Core::Var(x) => names.get(&x.0).copied(),
        _ => None,
    };
    match e {
        Core::Var(x) => {
            if let Some(p) = names.get(&x.0) {
                out.insert(*p);
            }
        }
        Core::Let(x, c, v, b) if let_alias(*x, *c, v, &cx.classes, &cx.written).is_some_and(|y| names.contains_key(&y)) => {
            let p = named(v).expect("an alias of a tracked name");
            let mut names = names.clone();
            names.insert(x.0, p);
            owned_uses(b, &names, modes, cx, out);
        }
        Core::Case { scrut, alts } if named(scrut).is_some() => {
            for a in alts {
                owned_uses(&a.body, names, modes, cx, out);
            }
        }
        Core::Proj(d, _) | Core::DictProj(d, ..) | Core::Demand(d, _) | Core::BoxGet(d, _) | Core::BoxClear(d, _)
            if named(d).is_some() => {}
        Core::LazyInit(l, v) | Core::BoxInit(l, v, _) | Core::RefSet(l, v, _) if names.contains_key(&l.0) => {
            out.insert(names[&l.0]);
            owned_uses(v, names, modes, cx, out);
        }
        Core::App(h, args) => {
            let callee = callee_modes(modes, h, args.len());
            if callee.is_none() {
                owned_uses(h, names, modes, cx, out);
            }
            for (i, a) in args.iter().enumerate() {
                if !(callee.is_some_and(|m| m[i]) && named(a).is_some()) {
                    owned_uses(a, names, modes, cx, out);
                }
            }
        }
        other => {
            for k in crate::core::child_slots_imm(other) {
                owned_uses(k, names, modes, cx, out);
            }
        }
    }
}

/// Apply the call-site rules of [`borrow_modes`] to the calls in `e`
/// (`tail` when `e` is in tail position), for a caller whose borrowed
/// names are `mine`: its borrowed parameters, their [`let_alias`]
/// renamings and the fields of a `case` on any of them. Each callee
/// whose modes change is pushed to `killed`.
fn call_sites(e: &Core, tail: bool, mine: &mut Set, cx: &UnitFacts, modes: &mut Modes, prims: &std::collections::HashSet<String>, killed: &mut Vec<String>) {
    match e {
        Core::App(h, args) => {
            let key = match &**h {
                Core::Global(g, _) => Some(g.0.as_str().into()),
                Core::Prim(op) => Some(std::borrow::Cow::Owned(op.symbol())),
                other => {
                    call_sites(other, false, mine, cx, modes, prims, killed);
                    None
                }
            };
            if let Some(k) = key {
                // A primitive is a leaf: a call to one in tail position gains
                // nothing from staying a tail call, so an owned local it is
                // lent is simply released after it returns, and the lent mode
                // stays (a primitive that owned its arguments paid a dup in
                // every caller and a drop in its own body, which also made a
                // `natCmp` too large for clang to inline).
                let leaf = prims.contains(k.as_ref());
                if let Some(m) = modes.get_mut(k.as_ref()).filter(|m| m.len() == args.len()) {
                    let mut changed = false;
                    for (i, a) in args.iter().enumerate() {
                        let fine = !m[i]
                            || match a {
                                Core::Var(x) => !tail || leaf || mine.contains(&x.0) || !cx.classes.get(&x.0).copied().is_some_and(counted),
                                // A literal owns nothing a callee could keep or a caller
                                // must release (an immediate, or an immortal static
                                // block), so lending one costs nothing.
                                Core::ConstString { .. } | Core::ConstBytes(_) | Core::ConstSym(_) => true,
                                // A `Nat`/`Int` literal is static only while it is an
                                // immediate; a larger one is parsed into a fresh bignum
                                // at run time, an owned temporary like any call result.
                                Core::ConstBig { .. } => small_literal(a),
                                other => expr_class_in(other, &cx.classes).is_some_and(|c| !counted(c)),
                            };
                        if !fine {
                            m[i] = false;
                            changed = true;
                        }
                    }
                    if changed {
                        killed.push(k.into_owned());
                    }
                }
            }
            for a in args {
                call_sites(a, false, mine, cx, modes, prims, killed);
            }
        }
        Core::Let(x, c, v, b) => {
            if let_alias(*x, *c, v, &cx.classes, &cx.written).is_some_and(|y| mine.contains(&y)) {
                mine.insert(x.0);
                call_sites(b, tail, mine, cx, modes, prims, killed);
                mine.remove(&x.0);
                return;
            }
            call_sites(v, false, mine, cx, modes, prims, killed);
            call_sites(b, tail, mine, cx, modes, prims, killed);
        }
        Core::If(c, t, f) => {
            call_sites(c, false, mine, cx, modes, prims, killed);
            call_sites(&t.body, tail, mine, cx, modes, prims, killed);
            call_sites(&f.body, tail, mine, cx, modes, prims, killed);
        }
        Core::Case { scrut, alts } => {
            call_sites(scrut, false, mine, cx, modes, prims, killed);
            let lent = matches!(&**scrut, Core::Var(s) if mine.contains(&s.0));
            for a in alts {
                match &a.pat {
                    // The fields of a borrowed scrutinee are borrowed too.
                    CPat::Con { fields, .. } if lent => {
                        let added: Vec<usize> = fields
                            .iter()
                            .map(|x| x.0)
                            .filter(|x| cx.classes.get(x).copied().is_some_and(counted) && mine.insert(*x))
                            .collect();
                        call_sites(&a.body, tail, mine, cx, modes, prims, killed);
                        for x in added {
                            mine.remove(&x);
                        }
                    }
                    _ => call_sites(&a.body, tail, mine, cx, modes, prims, killed),
                }
            }
        }
        // A quotation body's calls are the evaluator's (no unit there is
        // a candidate, see `value_uses`).
        other => {
            for k in crate::core::child_slots_imm(other) {
                call_sites(k, false, mine, cx, modes, prims, killed);
            }
        }
    }
}

/// A unit's binder classes and the binders it writes through
/// (`LazyInit`, `BoxInit`, `RefSet` targets).
struct UnitFacts {
    classes: HashMap<usize, Class>,
    written: Set,
}

impl UnitFacts {
    fn of(params: &[Class], body: &Core) -> UnitFacts {
        fn walk(e: &Core, out: &mut Set) {
            if let Core::LazyInit(l, _) | Core::BoxInit(l, ..) | Core::RefSet(l, ..) = e {
                out.insert(l.0);
            }
            for k in crate::core::child_slots_imm(e) {
                walk(k, out);
            }
        }
        let mut written = Set::new();
        walk(body, &mut written);
        UnitFacts { classes: binder_classes(params, body), written }
    }
}

/// The variable a `let x = y` renames: the pass binds no new reference
/// for `x` but reads `y` wherever `x` is used, so a borrowed `y` stays
/// borrowed through it. Only for a counted binder of `y`'s class that is
/// never written through (a `RefSet` replaces its binder's own value).
fn let_alias(x: LocalId, c: Class, v: &Core, classes: &HashMap<usize, Class>, written: &Set) -> Option<usize> {
    match v {
        Core::Var(y)
            if counted(c)
                && classes.get(&y.0) == Some(&c)
                && classes.get(&x.0) == Some(&c)
                && !written.contains(&x.0) =>
        {
            Some(y.0)
        }
        _ => None,
    }
}

fn param_classes(f: &FnDef) -> Vec<Class> {
    (0..f.arity()).map(|i| f.repr.params.get(i).copied().map_or(Class::Raw, concrete)).collect()
}

/// Every binder's class: the parameters', and the `let` and pattern
/// binders' recorded classes.
fn binder_classes(params: &[Class], body: &Core) -> HashMap<usize, Class> {
    fn walk(e: &Core, out: &mut HashMap<usize, Class>) {
        match e {
            Core::Let(x, c, ..) => {
                out.insert(x.0, concrete(*c));
            }
            Core::Join { params, .. } => {
                for (x, c) in params {
                    out.insert(x.0, concrete(*c));
                }
            }
            Core::Release { token, .. } => {
                if let Some(t) = token {
                    out.insert(t.0, Class::Raw);
                }
            }
            Core::Case { alts, .. } => {
                for a in alts {
                    match &a.pat {
                        CPat::Var(x, c) => {
                            out.insert(x.0, concrete(*c));
                        }
                        CPat::Con { fields, classes, .. } => {
                            for (i, x) in fields.iter().enumerate() {
                                out.insert(x.0, classes.get(i).copied().map_or(Class::Raw, concrete));
                            }
                        }
                        CPat::Wild => {}
                    }
                }
            }
            _ => {}
        }
        // A quotation body is closed runtime-side code; its binders are
        // the evaluator's.
        if let Core::Quote { holes, .. } = e {
            for h in holes {
                walk(h, out);
            }
            return;
        }
        if let Core::App(h, _) = e {
            walk(h, out);
        }
        for k in crate::core::child_slots_imm(e) {
            walk(k, out);
        }
    }
    let mut out: HashMap<usize, Class> = HashMap::new();
    for (i, c) in params.iter().enumerate() {
        out.insert(i, *c);
    }
    walk(body, &mut out);
    out
}

/// What the reuse insertion reads about one constructor unit.
#[derive(Clone)]
struct CtorFact {
    /// The runtime field count. A constructor without a boundary layout
    /// plan allocates exactly one payload word per field, so this is also
    /// its payload word count.
    fields: usize,
    /// The constructor has a boundary layout plan (attribute annotations
    /// specification section 4.3): its payload words are its recorded C
    /// layout, not its fields, so its cells are not interchangeable with
    /// a uniform constructor's.
    planned: bool,
    /// The qualified data type whose constructors share a tag space.
    data: Option<String>,
    /// The class of each runtime field (`Cons` is `[Param(0), Obj]`): the
    /// reference discipline of each payload word, which is what the
    /// family-crossing proof compares.
    classes: Vec<Class>,
}

/// A program's constructors: by unit name, and by data type and tag.
///
/// A `case` arm names its block by tag alone, so the block's payload word
/// count is only knowable through the data type of the constructor the
/// arm is being compared with. Requiring that lookup to succeed is what
/// makes the reuse's word-for-word overwrite a proof rather than a guess.
struct Ctors {
    by_unit: HashMap<String, CtorFact>,
    by_data: HashMap<String, Vec<(usize, CtorFact)>>,
}

impl Ctors {
    fn of(program: &Program) -> Ctors {
        let mut by_unit: HashMap<String, CtorFact> = HashMap::new();
        let mut by_data: HashMap<String, Vec<(usize, CtorFact)>> = HashMap::new();
        for f in &program.fns {
            let Some((tag, fields)) = f.ctor else {
                continue;
            };
            let planned = program.plans.contains_key(&f.id.0)
                || program.plans.contains_key(crate::repr::base_unit(&f.id.0).as_ref());
            let fact = CtorFact { fields, planned, data: f.repr.data.clone(), classes: f.repr.fields.clone() };
            if let Some(d) = &f.repr.data {
                by_data.entry(d.clone()).or_default().push((tag, fact.clone()));
            }
            by_unit.insert(f.id.0.clone(), fact);
        }
        Ctors { by_unit, by_data }
    }

    /// The constructor a `case` arm's `tag` names inside `data`, which is
    /// what the dying block really is: a tag alone does not say, and the
    /// arm's scrutinee type does.
    fn at_tag(&self, data: &str, tag: usize) -> Option<&CtorFact> {
        self.by_data.get(data)?.iter().find(|(t, _)| *t == tag).map(|(_, f)| f)
    }

    /// The dying block's payload word count, when the arm's tag provably
    /// names a uniform constructor of `data` with `fields` fields. Only
    /// then does the block have exactly `fields` payload words, which is
    /// what the reuse overwrites.
    fn payload_words(&self, data: &str, tag: usize, fields: usize) -> Option<usize> {
        let ctor = self.at_tag(data, tag)?;
        (!ctor.planned && ctor.fields == fields).then_some(fields)
    }
}

/// Rewrite the first constructor allocation on a spine into a reuse of the
/// cell the dying `block` leaves behind, or `None`.
///
/// The spine is what a `Release` body evaluates without branching: `let`
/// values, the count operations around them, and the arguments of a call
/// or a jump, which are evaluated in order before the transfer and never
/// skipped. A reuse under a branch could be skipped on some path, and the
/// cell the release kept alive would leak, so nothing else is searched.
/// One reuse per release: the token is read exactly once.
///
/// A call argument is on the spine and used to be missed. The
/// pattern-match-rebuild shape of a recursive accumulator
/// (`reverse-into ys (Cons y acc)`) puts the fresh constructor in an
/// argument of the recursive call, so the inliner's unrolled copies reused
/// their cells while the residual iteration, the one that carries the loop
/// for the whole tail, allocated one per element. Every argument is
/// evaluated after the `Release` bound the token and before the call, so
/// the reuse is reached on the same path that reached the `Release`.
fn insert_reuse(
    e: &Core,
    token: usize,
    words: usize,
    tag: usize,
    fields: &[LocalId],
    relaxed: bool,
    ctors: &Ctors,
) -> Option<Core> {
    if let Some(r) = reuse_allocation(e, token, words, tag, fields, relaxed, ctors) {
        return Some(r);
    }
    match e {
        Core::Let(x, c, v, b) => {
            // The value is evaluated first, so its allocation is the
            // earlier one: reuse it and leave the continuation alone.
            if let Some(v2) = insert_reuse(v, token, words, tag, fields, relaxed, ctors) {
                return Some(Core::Let(*x, *c, Box::new(v2), b.clone()));
            }
            let b2 = insert_reuse(b, token, words, tag, fields, relaxed, ctors)?;
            Some(Core::Let(*x, *c, v.clone(), Box::new(b2)))
        }
        Core::Dup(x, c, b) => insert_reuse(b, token, words, tag, fields, relaxed, ctors)
            .map(|b2| Core::Dup(*x, *c, Box::new(b2))),
        Core::Drop(x, c, b) => insert_reuse(b, token, words, tag, fields, relaxed, ctors)
            .map(|b2| Core::Drop(*x, *c, Box::new(b2))),
        // A call's arguments are evaluated in order before the transfer,
        // with no branch between them and the `Release`, so the first
        // compatible allocation among them is on the spine. The head is
        // left alone: it is a unit, a primitive or a variable in every
        // shape that reaches here, and a constructor there would not be an
        // allocation on this path.
        Core::App(f, args) => {
            let (i, a2) = insert_reuse_in_args(args, token, words, tag, fields, relaxed, ctors)?;
            let mut args = args.clone();
            args[i] = a2;
            Some(Core::App(f.clone(), args))
        }
        // A jump hands its arguments to a join point's parameters, in the
        // same order and with the same "evaluated before the transfer"
        // rule, so its arguments are on the spine for the same reason.
        Core::Jump { id, args } => {
            let (i, a2) = insert_reuse_in_args(args, token, words, tag, fields, relaxed, ctors)?;
            let mut args = args.clone();
            args[i] = a2;
            Some(Core::Jump { id: *id, args })
        }
        _ => None,
    }
}

/// The first argument of `args` that a reuse can be written into, with the
/// rewritten argument.
fn insert_reuse_in_args(
    args: &[Core],
    token: usize,
    words: usize,
    tag: usize,
    fields: &[LocalId],
    relaxed: bool,
    ctors: &Ctors,
) -> Option<(usize, Core)> {
    for (i, a) in args.iter().enumerate() {
        if let Some(a2) = insert_reuse(a, token, words, tag, fields, relaxed, ctors) {
            return Some((i, a2));
        }
    }
    None
}

/// Write the `Reuse` of a reset/reuse pair into the arms of a branch,
/// leaving the `Release` at the head of the branch: the branch with a
/// `Reuse` on every arm that allocates a compatible constructor, or
/// `None` when no arm allocates one.
///
/// Lean's `expandResetReuse` (`ExpandResetReuse.lean`) pushes a copy of
/// the whole pair into every arm, so each path owns its own token and the
/// one-token-one-reuse rule holds per path. Copying the *release* is what
/// this pass must not do, and the bootstrap compiler proved it: the branch
/// head (an `if`'s condition, a `case`'s scrutinee) is evaluated after the
/// release and before the arms, so a field the head consumes is already
/// dead at arm entry. A release copied into the arm accounts for that
/// field there and drops it a second time, and the allocator reads the
/// second drop as a corrupt free list rather than as a double free.
///
/// The release therefore stays where the straight-line rule puts it, at
/// the head of the branch, carrying the moved-field set the whole arm body
/// computed (the union over the one body it had). Only the `Reuse` moves
/// into the arms, and every arm's reuse names that one token. Exactly one
/// arm runs per path, so the token is read at most once per path:
///
/// - an arm that allocates a compatible constructor writes into the kept
///   cell;
/// - an arm that allocates nothing compatible would leave the token
///   unread, and that is a leak, not a free: on the unique path a release
///   that binds a token KEEPS the cell for the reuse (the `Release` node's
///   contract, `core.rs`), so an arm with no reuse never frees it. The
///   expansion is therefore only applied when EVERY arm reuses; a branch
///   with a non-reusing arm keeps the plain release, which frees the cell
///   on its unique path. (A mixed branch could free the unread token in
///   its other arms instead; that needs a token-freeing node Core does not
///   have, and losing reuse in a mixed branch costs an allocation, while
///   the kept cell cost the whole block.)
///
/// `Some` only when every arm took a pair, because otherwise the release
/// would bind a token some path never reads, and that path would keep the
/// cell forever.
fn branch_reuse(
    e: &Core,
    token: usize,
    words: usize,
    tag: usize,
    fields: &[LocalId],
    ctors: &Ctors,
) -> Option<Core> {
    // The arm's body with the reuse on its spine, when it allocates a
    // compatible constructor there.
    let arm = |body: &Core| -> (Core, bool) {
        let reused = insert_reuse(body, token, words, tag, fields, false, ctors)
            .or_else(|| insert_reuse(body, token, words, tag, fields, true, ctors));
        match reused {
            Some(rewritten) => (rewritten, true),
            None => (body.clone(), false),
        }
    };
    match e {
        Core::Case { scrut, alts } if !alts.is_empty() => {
            let mut bodies = Vec::with_capacity(alts.len());
            let mut every = true;
            for a in alts {
                let (body, reused) = arm(&a.body);
                every &= reused;
                bodies.push(body);
            }
            // A token some arm does not read keeps the cell on that path
            // with nothing to free it: only a branch whose every arm
            // reuses binds one.
            if !every {
                return None;
            }
            let mut bodies = bodies.into_iter();
            let alts = alts
                .into_iter()
                .map(|a| CAlt {
                    pat: a.pat.clone(),
                    weight: a.weight,
                    body: Box::new(bodies.next().expect("one body per arm")),
                })
                .collect();
            Some(Core::Case { scrut: scrut.clone(), alts })
        }
        Core::If(c, t, f) => {
            let (then, then_reused) = arm(&t.body);
            let (otherwise, else_reused) = arm(&f.body);
            if !(then_reused && else_reused) {
                return None;
            }
            Some(Core::If(
                c.clone(),
                CBranch { weight: t.weight, body: Box::new(then) },
                CBranch { weight: f.weight, body: Box::new(otherwise) },
            ))
        }
        _ => None,
    }
}

/// The payload words the dying block already holds, given the arm's field
/// binders: `i` is present when the reuse's argument for slot `i` is that
/// slot's own binder. The cell was never emptied of those words (the
/// `Release` moved them out to the binders and did not release them), so
/// writing them back is a store of the word that is already there. Lean
/// reaches the same conclusion from `oproj` chains (`partitionSelfSets`,
/// `ExpandResetReuse.lean`); the arm's binders are the same fact in a
/// cheaper form, because a `case` arm names its fields directly.
fn already_set(args: &[Core], fields: &[LocalId]) -> Vec<u32> {
    let mut out = Vec::new();
    for (i, a) in args.iter().enumerate() {
        let Core::Var(x) = a else {
            continue;
        };
        if fields.get(i) == Some(x) {
            out.push(i as u32);
        }
    }
    out
}

/// Whether any constructor allocation reachable in `e` could be a reuse of a
/// block of `words` payload words with tag `tag` (the borrow inference's
/// ownership question, which ignores whether the spine makes the reuse
/// legal).
fn reuse_allocation_anywhere(e: &Core, words: usize, tag: usize, ctors: &Ctors) -> bool {
    // Both passes are asked, so the forced-ownership rule covers exactly
    // what either pass could reuse.
    if compatible_ctor(e, words, tag, false, ctors).is_some()
        || compatible_ctor(e, words, tag, true, ctors).is_some()
    {
        return true;
    }
    crate::core::child_slots_imm(e)
        .into_iter()
        .any(|k| reuse_allocation_anywhere(k, words, tag, ctors))
}

/// The constructor and fields of a saturated allocation whose cell is
/// interchangeable with a dying block of `words` payload words matched at
/// `tag`, if any.
///
/// Two proofs, one per pass. The first (`relaxed` off) requires the
/// block's tag to name, inside the *same* data type, an unplanned
/// constructor of the same field count: both payloads are then exactly
/// `words` words with the same class per word, which is what makes the
/// word-for-word overwrite safe and the block's later drop see what is
/// really there. The second (`relaxed` on) is Lean's `relaxedReuse`
/// (`ResetReuse.lean`, `mayReuse`): the constructors may come from
/// different data types, and the proof is carried by the word classes
/// instead. A word class is the reference discipline, so two unplanned
/// constructors of the same field count whose words agree class for class
/// leave a cell that drops exactly as it did; `Raw` words are not
/// references at all, and `Obj`/`TObj` are the counted ones, so a
/// disagreement there would be a live-reference bug and is refused.
fn compatible_ctor<'a>(
    e: &'a Core,
    words: usize,
    tag: usize,
    relaxed: bool,
    ctors: &Ctors,
) -> Option<(&'a GlobalId, &'a Vec<Core>)> {
    let Core::App(head, args) = e else {
        return None;
    };
    let Core::Ctor(id, _) = &**head else {
        return None;
    };
    // A mutable cell's address is observable through the borrow that
    // names it, so it is never built in another value's storage.
    if id.0 == crate::core::MUTABLE_CELL {
        return None;
    }
    let fact = ctors.by_unit.get(&id.0)?;
    if fact.planned || fact.fields != words || args.len() != words {
        return None;
    }
    let data = fact.data.as_deref()?;
    let dying = ctors.at_tag(data, tag)?;
    if !relaxed {
        ctors.payload_words(data, tag, words)?;
        return Some((id, args));
    }
    // The family-crossing proof: both sides unplanned, the same payload
    // word count, and a word class agreeing at every slot.
    if dying.planned || dying.fields != words {
        return None;
    }
    if dying.classes.len() != fact.classes.len() || dying.classes.is_empty() {
        return None;
    }
    if dying.classes.iter().zip(&fact.classes).any(|(a, b)| counted(*a) != counted(*b)) {
        return None;
    }
    Some((id, args))
}

/// The reuse of one compatible allocation, reading the cell `token` names.
fn reuse_allocation(
    e: &Core,
    token: usize,
    words: usize,
    tag: usize,
    fields: &[LocalId],
    relaxed: bool,
    ctors: &Ctors,
) -> Option<Core> {
    let (ctor, args) = compatible_ctor(e, words, tag, relaxed, ctors)?;
    Some(Core::Reuse {
        token: LocalId(token),
        ctor: ctor.clone(),
        already_set: already_set(args, fields),
        args: args.clone(),
    })
}

fn transform(f: &FnDef, modes: &Modes, borrowed: &Set, ctors: &Ctors) -> Core {
    transform_body(&param_classes(f), &f.body, true, modes, borrowed, ctors)
}

/// The count operations of a body whose parameters have the classes
/// `params`; those in `borrowed` are lent by the caller for the whole
/// call, the rest owned. A quotation body is a closed body with none: the
/// runtime evaluator runs the operations the pass places there exactly
/// as code generation runs them in a unit, and it has no form for drop
/// specialisation, so `specialize` is off there.
fn transform_body(
    params: &[Class],
    body: &Core,
    specialize: bool,
    modes: &Modes,
    borrowed: &Set,
    ctors: &Ctors,
) -> Core {
    let UnitFacts { classes: class, written } = UnitFacts::of(params, body);
    let mut next = params.len();
    crate::core::walk_locals(body, &mut |l| next = next.max(l.0 + 1));
    for k in class.keys() {
        next = next.max(k + 1);
    }
    let mut pass = Pass {
        class,
        written,
        fv: HashMap::new(),
        alias: HashMap::new(),
        next,
        specialize,
        modes,
        ctors,
    };
    let owned: Set = (0..params.len()).filter(|i| pass.counted(*i) && !borrowed.contains(i)).collect();
    pass.go(body, &owned, &Set::new())
}

struct Pass<'m> {
    class: HashMap<usize, Class>,
    /// Free counted binders of the original body's nodes, by address.
    /// Only nodes of the input body (alive for the whole pass) are keyed.
    fv: HashMap<*const Core, Rc<Set>>,
    /// Variable alternatives whose binder took over the scrutinee's
    /// reference: their uses are the scrutinee's.
    alias: HashMap<usize, usize>,
    next: usize,
    /// Place `Release` (drop specialisation) where an alternative keeps
    /// fields of a scrutinee it leaves dead.
    specialize: bool,
    /// The callees' borrowed parameters ([`borrow_modes`]).
    modes: &'m Modes,
    /// The program's constructors ([`Ctors`]), for reuse insertion.
    ctors: &'m Ctors,
    /// Binders written through ([`UnitFacts`]).
    written: Set,
}

impl Pass<'_> {
    fn class_of(&self, x: usize) -> Class {
        self.class.get(&x).copied().unwrap_or(Class::Raw)
    }

    fn counted(&self, x: usize) -> bool {
        counted(self.class_of(x))
    }

    fn resolve(&self, x: usize) -> usize {
        let mut x = x;
        while let Some(s) = self.alias.get(&x) {
            x = *s;
        }
        x
    }

    fn fresh(&mut self, c: Class) -> usize {
        let t = self.next;
        self.next += 1;
        self.class.insert(t, c);
        t
    }

    /// The free counted binders of an input node, before aliasing.
    fn fv(&mut self, e: &Core) -> Rc<Set> {
        let key = e as *const Core;
        if let Some(s) = self.fv.get(&key) {
            return s.clone();
        }
        let mut s = Set::new();
        let mut add = |s: &mut Set, x: &LocalId, this: &Pass| {
            if this.counted(x.0) {
                s.insert(x.0);
            }
        };
        match e {
            Core::Var(x) => add(&mut s, x, self),
            Core::Let(x, _, v, b) => {
                s.extend(self.fv(v).iter());
                s.extend(self.fv(b).iter().filter(|y| **y != x.0));
            }
            // A jump's arguments are uses in the enclosing scope; the join's
            // parameters are bound in its body only, and the join id is not a
            // value.
            Core::Join { params, body, rest, .. } => {
                s.extend(self.fv(body).iter().filter(|y| !params.iter().any(|(x, _)| x.0 == **y)));
                s.extend(self.fv(rest).iter());
            }
            Core::Jump { args, .. } => {
                for a in args {
                    s.extend(self.fv(a).iter());
                }
            }
            Core::Case { scrut, alts } => {
                s.extend(self.fv(scrut).iter());
                for a in alts {
                    let bound: Vec<usize> = match &a.pat {
                        CPat::Var(x, _) => vec![x.0],
                        CPat::Con { fields, .. } => fields.iter().map(|x| x.0).collect(),
                        CPat::Wild => Vec::new(),
                    };
                    s.extend(self.fv(&a.body).iter().filter(|y| !bound.contains(y)));
                }
            }
            Core::LazyInit(l, v) | Core::BoxInit(l, v, _) | Core::RefSet(l, v, _) => {
                add(&mut s, l, self);
                s.extend(self.fv(v).iter());
            }
            Core::Dup(l, _, b) | Core::Drop(l, _, b) => {
                add(&mut s, l, self);
                s.extend(self.fv(b).iter());
            }
            Core::Quote { holes, .. } => {
                for h in holes {
                    s.extend(self.fv(h).iter());
                }
            }
            Core::App(h, args) => {
                s.extend(self.fv(h).iter());
                for a in args {
                    s.extend(self.fv(a).iter());
                }
            }
            other => {
                for k in crate::core::child_slots_imm(other) {
                    s.extend(self.fv(k).iter());
                }
            }
        }
        let s = Rc::new(s);
        self.fv.insert(key, s.clone());
        s
    }

    /// The owned binders an input node needs, through the aliases.
    fn need(&mut self, e: &Core) -> Set {
        let fv = self.fv(e);
        fv.iter().map(|x| self.resolve(*x)).collect()
    }

    fn rc(&self, drop: bool, x: usize, body: Core) -> Core {
        let c = self.class_of(x);
        if drop {
            Core::Drop(LocalId(x), c, Box::new(body))
        } else {
            Core::Dup(LocalId(x), c, Box::new(body))
        }
    }

    /// `e` evaluated while owning `owned`, of which `live` must still be
    /// owned afterwards: everything else is consumed by `e` or dropped.
    fn go(&mut self, e: &Core, owned: &Set, live: &Set) -> Core {
        // The fault path never returns: nothing after it runs.
        if matches!(e, Core::Fault) {
            return Core::Fault;
        }
        let mut need = self.need(e);
        need.extend(live.iter().copied());
        let kept: Set = owned.intersection(&need).copied().collect();
        let dead: Vec<usize> = owned.difference(&need).copied().collect();
        let mut core = self.node(e, &kept, live);
        for x in dead.into_iter().rev() {
            core = self.rc(true, x, core);
        }
        core
    }

    /// Subexpressions evaluated in order, each owning what the previous
    /// left, with `live` owned after the last.
    fn seq(&mut self, es: &[&Core], owned: &Set, live: &Set) -> Vec<Core> {
        let mut after: Vec<Set> = vec![live.clone(); es.len()];
        for i in (0..es.len().saturating_sub(1)).rev() {
            let mut s = after[i + 1].clone();
            s.extend(self.need(es[i + 1]));
            after[i] = s;
        }
        let mut cur = owned.clone();
        let mut out = Vec::with_capacity(es.len());
        for (i, e) in es.iter().enumerate() {
            let next: Set = cur.intersection(&after[i]).copied().collect();
            out.push(self.go(e, &cur, &next));
            cur = next;
        }
        out
    }

    fn node(&mut self, e: &Core, owned: &Set, live: &Set) -> Core {
        match e {
            Core::Var(x) => {
                let x = self.resolve(x.0);
                if self.counted(x) && (live.contains(&x) || !owned.contains(&x)) {
                    self.rc(false, x, Core::Var(LocalId(x)))
                } else {
                    Core::Var(LocalId(x))
                }
            }
            Core::App(f, args) => {
                if let Some(m) = callee_modes(self.modes, f, args.len()) {
                    return self.borrowing_call(f, args, &m.clone(), owned, live);
                }
                let mut es: Vec<&Core> = vec![&**f];
                es.extend(args.iter());
                let mut v = self.seq(&es, owned, live);
                let f = v.remove(0);
                Core::App(Box::new(f), v)
            }
            Core::Lam { fn_id, env, classes } => {
                let es: Vec<&Core> = env.iter().collect();
                Core::Lam { fn_id: fn_id.clone(), env: self.seq(&es, owned, live), classes: classes.clone() }
            }
            Core::InstDict(g, args, v) => {
                let es: Vec<&Core> = args.iter().collect();
                Core::InstDict(g.clone(), self.seq(&es, owned, live), v.clone())
            }
            Core::Quote { template, holes, body } => {
                let es: Vec<&Core> = holes.iter().collect();
                Core::Quote {
                    template: template.clone(),
                    holes: self.seq(&es, owned, live),
                    body: body.as_ref().map(|b| Box::new(transform_body(&[], b, false, self.modes, &Set::new(), self.ctors))),
                }
            }
            Core::Let(x, c, v, b) if let_alias(*x, *c, v, &self.class, &self.written).is_some() => {
                // A renaming binds nothing: `x`'s uses are `y`'s.
                let Core::Var(y) = &**v else { unreachable!("a let alias binds a variable") };
                let y = self.resolve(y.0);
                self.alias.insert(x.0, y);
                self.go(b, owned, live)
            }
            Core::Let(x, c, v, b) => {
                let mut fb = self.need(b);
                fb.remove(&x.0);
                fb.extend(live.iter().copied());
                let lv: Set = owned.intersection(&fb).copied().collect();
                let v2 = self.go(v, owned, &lv);
                let mut ob = lv;
                if self.counted(x.0) {
                    ob.insert(x.0);
                }
                let b2 = self.go(b, &ob, live);
                Core::Let(*x, *c, Box::new(v2), Box::new(b2))
            }
            Core::If(c, t, f) => {
                let mut nb = self.need(&t.body);
                nb.extend(self.need(&f.body));
                nb.extend(live.iter().copied());
                let after: Set = owned.intersection(&nb).copied().collect();
                let c2 = self.go(c, owned, &after);
                let t2 = self.go(&t.body, &after, live);
                let f2 = self.go(&f.body, &after, live);
                Core::If(
                    Box::new(c2),
                    CBranch { weight: t.weight, body: Box::new(t2) },
                    CBranch { weight: f.weight, body: Box::new(f2) },
                )
            }
            Core::Case { scrut, alts } => {
                if let Core::Var(s) = &**scrut {
                    let s = self.resolve(s.0);
                    return self.case_on(s, alts, owned, live);
                }
                // A computed scrutinee becomes an owned temporary.
                let mut na = live.clone();
                for a in alts {
                    let bound = pat_binders(&a.pat);
                    na.extend(self.need(&a.body).into_iter().filter(|y| !bound.contains(y)));
                }
                let after: Set = owned.intersection(&na).copied().collect();
                let s2 = self.go(scrut, owned, &after);
                let tc = self.scrut_class(scrut, alts);
                let t = self.fresh(tc);
                let mut o2 = after;
                if counted(tc) {
                    o2.insert(t);
                }
                let case = self.case_on(t, alts, &o2, live);
                Core::Let(LocalId(t), tc, Box::new(s2), Box::new(case))
            }
            // A join's parameters are OWNED by its body, exactly as a
            // function's are: every jump hands one reference over per
            // parameter, and the body consumes each exactly once on every
            // path (a jump may enter it more than once, so a body that reads
            // a value it does not own dups it). The continuation runs in the
            // enclosing scope, with the join's binders out of it.
            //
            // A parameter the body only LENDS is a bug, not a fallback: no
            // enclosing code runs after a jump, so there is no place to
            // release it, and the verifier refuses it. `crate::simplify`
            // keeps such a body out of a join (its parameters are the
            // body's own pattern binders, which the jump consumes).
            Core::Join { id, params, body, rest } => {
                let rest = self.go(rest, owned, live);
                let powned: Set = params
                    .iter()
                    .filter(|(x, _)| self.counted(x.0))
                    .map(|(x, _)| x.0)
                    .collect();
                let body = self.go(body, &powned, &Set::new());
                Core::Join {
                    id: *id,
                    params: params.clone(),
                    body: Box::new(body),
                    rest: Box::new(rest),
                }
            }
            // A jump is an owned use of every argument: the join's parameter
            // takes the reference over, and nothing follows the jump.
            Core::Jump { id, args } => {
                let es: Vec<&Core> = args.iter().collect();
                Core::Jump { id: *id, args: self.seq(&es, owned, live) }
            }
            Core::Proj(d, f) => {
                let f = f.clone();
                let class = f.class;
                self.borrowed(d, owned, live, class, move |d| Core::Proj(Box::new(d), f))
            }
            Core::DictProj(d, i, site, class) => {
                let (i, site, class) = (*i, site.clone(), *class);
                self.borrowed(d, owned, live, class, move |d| Core::DictProj(Box::new(d), i, site, class))
            }
            Core::Demand(d, class) => {
                let class = *class;
                self.borrowed(d, owned, live, class, move |d| Core::Demand(Box::new(d), class))
            }
            Core::BoxGet(d, class) => {
                let class = *class;
                self.borrowed(d, owned, live, class, move |d| Core::BoxGet(Box::new(d), class))
            }
            Core::BoxClear(d, class) => {
                let class = *class;
                self.borrowed(d, owned, live, Class::Raw, move |d| Core::BoxClear(Box::new(d), class))
            }
            Core::BoxInit(l, v, class) => {
                let (l, class) = (self.resolve(l.0), *class);
                self.write_through(l, v, owned, live, class, move |v| {
                    Core::BoxInit(LocalId(l), Box::new(v), class)
                })
            }
            Core::LazyInit(l, v) => {
                let l = self.resolve(l.0);
                self.write_through(l, v, owned, live, Class::Raw, move |v| {
                    Core::LazyInit(LocalId(l), Box::new(v))
                })
            }
            Core::RefSet(l, v, payload) => {
                let l = self.resolve(l.0);
                let payload = *payload;
                self.write_through(l, v, owned, live, Class::Raw, move |v| {
                    Core::RefSet(LocalId(l), Box::new(v), payload)
                })
            }
            // Leaves: constants, unit values, allocations, compile-time
            // forms (an input body has no count operations).
            other => other.clone(),
        }
    }

    /// The class of a computed scrutinee's temporary: a constructor
    /// pattern makes it a block; a variable alternative's binder records
    /// it; otherwise the expression's own class where it shows one, else
    /// `Raw` (uncounted, which can only leak).
    fn scrut_class(&self, scrut: &Core, alts: &[CAlt<Core>]) -> Class {
        if alts.iter().any(|a| matches!(a.pat, CPat::Con { .. })) {
            return Class::Obj;
        }
        for a in alts {
            if let CPat::Var(x, _) = &a.pat {
                return self.class_of(x.0);
            }
        }
        self.expr_class(scrut).unwrap_or(Class::Raw)
    }

    fn expr_class(&self, e: &Core) -> Option<Class> {
        match e {
            Core::Var(x) => Some(self.class_of(self.resolve(x.0))),
            Core::Let(_, _, _, b) => self.expr_class(b),
            other => expr_class_in(other, &self.class),
        }
    }

    /// A saturated direct call of a unit with borrowed parameters `modes`
    /// ([`borrow_modes`]). A variable passed to a borrowed parameter is
    /// only read: it stays owned (or lent) across the evaluation of the
    /// other arguments and the call, and one the caller owns and needs no
    /// more is dropped after the call returns. A borrowed parameter's
    /// computed argument is uncounted by the inference's call-site rule,
    /// so it is evaluated as any other argument.
    fn borrowing_call(&mut self, head: &Core, args: &[Core], modes: &[bool], owned: &Set, live: &Set) -> Core {
        let lent = |i: usize| modes[i] && matches!(args[i], Core::Var(_));
        let reads: Set = (0..args.len())
            .filter(|i| lent(*i))
            .filter_map(|i| match &args[i] {
                Core::Var(x) => Some(self.resolve(x.0)),
                _ => None,
            })
            .filter(|x| self.counted(*x))
            .collect();
        let mut keep = live.clone();
        keep.extend(reads.iter().copied());
        let mut es: Vec<&Core> = vec![head];
        es.extend((0..args.len()).filter(|i| !lent(*i)).map(|i| &args[i]));
        let mut vals = self.seq(&es, owned, &keep).into_iter();
        let head = vals.next().expect("the call's head");
        let args: Vec<Core> = (0..args.len())
            .map(|i| match &args[i] {
                Core::Var(x) if lent(i) => Core::Var(LocalId(self.resolve(x.0))),
                _ => vals.next().expect("an owned argument"),
            })
            .collect();
        let mut call = Core::App(Box::new(head), args);
        let drops: Vec<usize> = reads.iter().copied().filter(|x| owned.contains(x) && !live.contains(x)).collect();
        if !drops.is_empty() {
            let r = self.fresh(Class::Raw);
            let mut after = Core::Var(LocalId(r));
            for x in drops.into_iter().rev() {
                after = self.rc(true, x, after);
            }
            call = Core::Let(LocalId(r), Class::Raw, Box::new(call), Box::new(after));
        }
        call
    }

    /// The alternatives of a `case` on the owned-or-raw binder `s`.
    fn case_on(&mut self, s: usize, alts: &[CAlt<Core>], owned: &Set, live: &Set) -> Core {
        let mut out = Vec::with_capacity(alts.len());
        for a in alts {
            let body = match &a.pat {
                CPat::Con { tag, fields, .. } => {
                    let nb = self.need(&a.body);
                    // The arm reads the block whenever it uses any of its
                    // fields, counted or not: an `I64` field is read
                    // through the block just as an `Obj` one is, and an arm
                    // that reads only scalars is exactly the shape whose
                    // rebuild can take the cell. `moved` still carries only
                    // the counted fields, because only those hand a
                    // reference over.
                    let mut bound_fields = Set::new();
                    crate::core::walk_locals(&a.body, &mut |l| {
                        bound_fields.insert(l.0);
                    });
                    let reads_fields = fields.iter().any(|x| bound_fields.contains(&x.0));
                    let used: Vec<usize> =
                        fields.iter().map(|x| x.0).filter(|x| self.counted(*x) && nb.contains(x)).collect();
                    if self.specialize
                        && reads_fields
                        && self.class_of(s) == Class::Obj
                        && owned.contains(&s)
                        && !live.contains(&s)
                        && !nb.contains(&s)
                    {
                        // The alternative keeps fields and leaves the
                        // scrutinee dead: a uniquely held scrutinee hands
                        // its fields over instead of dupping them and
                        // dropping itself (design/reclamation.md section 4).
                        let mut o = owned.clone();
                        o.remove(&s);
                        o.extend(used.iter().copied());
                        let body = self.go(&a.body, &o, live);
                        let with_class = |x: usize| (LocalId(x), self.class_of(x));
                        let moved = used.iter().map(|x| with_class(*x)).collect();
                        let rest = fields
                            .iter()
                            .map(|x| x.0)
                            .filter(|x| self.counted(*x) && !used.contains(x))
                            .map(with_class)
                            .collect();
                        // A constructor allocation on the body's spine can
                        // be built in this block's cell instead of a fresh
                        // block: the block is uniquely held here (that is
                        // this arm's condition), and the reuse hands it
                        // the new constructor's layout (Lean 4's
                        // reset/reuse pair).
                        // Two passes, in Lean's order (`ResetReuse.lean`
                        // runs `insertResetReuseCore` with `relaxedReuse`
                        // off and then on): the same-family reuse first,
                        // and only when that finds nothing the
                        // family-crossing one, so a compatible successor
                        // of the scrutinee's own type is never passed over
                        // for one of another type.
                        let words = fields.len();
                        let found = insert_reuse(&body, self.next, words, *tag, fields, false, self.ctors)
                            .or_else(|| {
                                insert_reuse(&body, self.next, words, *tag, fields, true, self.ctors)
                            });
                        match found {
                            Some(body) => {
                                let token = self.fresh(Class::Raw);
                                Core::Release {
                                    block: LocalId(s),
                                    moved,
                                    rest,
                                    token: Some(LocalId(token)),
                                    body: Box::new(body),
                                }
                            }
                            None => {
                                // No allocation on the straight-line
                                // spine. The release keeps its place at the
                                // head of the branch and binds the token;
                                // only the reuse moves into the arms (Lean's
                                // `expandResetReuse`), because the head runs
                                // before every arm and a field it consumes
                                // is dead at arm entry.
                                let token = self.fresh(Class::Raw);
                                match branch_reuse(&body, token, words, *tag, fields, self.ctors) {
                                    Some(body) => Core::Release {
                                        block: LocalId(s),
                                        moved,
                                        rest,
                                        token: Some(LocalId(token)),
                                        body: Box::new(body),
                                    },
                                    None => Core::Release {
                                        block: LocalId(s),
                                        moved,
                                        rest,
                                        token: None,
                                        body: Box::new(body),
                                    },
                                }
                            }
                        }
                    } else if self.counted(s) && !owned.contains(&s) {
                        // A borrowed scrutinee (a borrowed parameter, or a
                        // field of one) lives for the whole call, so its
                        // fields stay borrowed: an owned use dups one there.
                        self.go(&a.body, owned, live)
                    } else {
                        let mut o = owned.clone();
                        o.extend(used.iter().copied());
                        // The scrutinee's drop (when the body leaves it dead)
                        // sits inside, after the field dups.
                        let mut body = self.go(&a.body, &o, live);
                        for x in used.into_iter().rev() {
                            body = self.rc(false, x, body);
                        }
                        body
                    }
                }
                CPat::Var(x, _) => {
                    let x = x.0;
                    let nb = self.need(&a.body);
                    if !self.counted(x) || !nb.contains(&x) {
                        self.go(&a.body, owned, live)
                    } else if self.counted(s)
                        && self.class_of(s) == self.class_of(x)
                        && owned.contains(&s)
                        && !live.contains(&s)
                        && !nb.contains(&s)
                    {
                        // The body never reads the scrutinee: its
                        // reference becomes the binder's.
                        self.alias.insert(x, s);
                        let body = self.go(&a.body, owned, live);
                        self.alias.remove(&x);
                        body
                    } else {
                        let mut o = owned.clone();
                        o.insert(x);
                        let body = self.go(&a.body, &o, live);
                        self.rc(false, x, body)
                    }
                }
                CPat::Wild => self.go(&a.body, owned, live),
            };
            out.push(CAlt { pat: a.pat.clone(), weight: a.weight, body: Box::new(body) });
        }
        Core::Case { scrut: Box::new(Core::Var(LocalId(s))), alts: out }
    }

    /// A node that reads its container `d` (borrowed) and yields an owned
    /// result of class `class` (code generation dups the loaded word).
    fn borrowed(
        &mut self,
        d: &Core,
        owned: &Set,
        live: &Set,
        class: Class,
        make: impl FnOnce(Core) -> Core,
    ) -> Core {
        let class = concrete(class);
        match d {
            Core::Var(r) => {
                let r = self.resolve(r.0);
                let node = make(Core::Var(LocalId(r)));
                if self.counted(r) && owned.contains(&r) && !live.contains(&r) {
                    // The container's last use: release it once the
                    // result holds its own reference.
                    let t = self.fresh(class);
                    Core::Let(LocalId(t), class, Box::new(node), Box::new(self.rc(true, r, Core::Var(LocalId(t)))))
                } else {
                    node
                }
            }
            _ => {
                // A computed container: an owned temporary (of its own
                // class; a block unless the expression says otherwise),
                // released after the read.
                let cc = self.expr_class(d).unwrap_or(Class::Obj);
                let d2 = self.go(d, owned, live);
                let tmp = self.fresh(cc);
                let node = make(Core::Var(LocalId(tmp)));
                let inner = if counted(cc) {
                    let t = self.fresh(class);
                    Core::Let(LocalId(t), class, Box::new(node), Box::new(self.rc(true, tmp, Core::Var(LocalId(t)))))
                } else {
                    node
                };
                Core::Let(LocalId(tmp), cc, Box::new(d2), Box::new(inner))
            }
        }
    }

    /// A node that writes through the binder `l` (borrowed) after
    /// evaluating `v` (owned). Keep the cell live while `v` runs, even if
    /// the write is the last use of the cell.
    #[allow(clippy::too_many_arguments)]
    fn write_through(
        &mut self,
        l: usize,
        v: &Core,
        owned: &Set,
        live: &Set,
        class: Class,
        make: impl FnOnce(Core) -> Core,
    ) -> Core {
        let mut keep = live.clone();
        keep.insert(l);
        let lv: Set = owned.intersection(&keep).copied().collect();
        let v2 = self.go(v, owned, &lv);
        let node = make(v2);
        let holds = self.counted(l) && owned.contains(&l);
        if holds && !live.contains(&l) {
            let class = concrete(class);
            let t = self.fresh(class);
            Core::Let(LocalId(t), class, Box::new(node), Box::new(self.rc(true, l, Core::Var(LocalId(t)))))
        } else {
            node
        }
    }
}

fn pat_binders(p: &CPat) -> Vec<usize> {
    match p {
        CPat::Var(x, _) => vec![x.0],
        CPat::Con { fields, .. } => fields.iter().map(|x| x.0).collect(),
        CPat::Wild => Vec::new(),
    }
}

// ------------------------------------------------------------ the verifier

/// Re-derive the ownership of a transformed body: every counted binder
/// starts with the references its binding gives it (one for a parameter,
/// a `let`; none for a pattern binder, which borrows from
/// its live scrutinee until a `Dup`), an owned use or `Drop` takes one, a
/// `Dup` adds one, a borrowed read needs one, and every path that returns
/// ends with none held.
pub fn verify(f: &FnDef, body: &Core, modes: &Modes, borrowed: &Set) -> Result<(), String> {
    verify_body(&param_classes(f), body, modes, borrowed)
}

/// [`verify`] for a body whose parameters have the classes `params`, of
/// which those in `lent` are borrowed: alive throughout, holding no
/// reference of their own.
fn verify_body(params: &[Class], body: &Core, modes: &Modes, lent: &Set) -> Result<(), String> {
    let class = binder_classes(params, body);
    let mut st = State { cnt: HashMap::new(), parent: HashMap::new() };
    for i in 0..params.len() {
        if class.get(&i).copied().is_some_and(counted) && !lent.contains(&i) {
            st.cnt.insert(i, 1);
        }
    }
    let mut v = Verifier { class, modes, lent: lent.clone() };
    if v.value(body, &mut st)? {
        v.ended(&st)?;
    }
    Ok(())
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    cnt: HashMap<usize, i64>,
    /// A pattern binder's scrutinee: it may be dupped while that lives.
    parent: HashMap<usize, usize>,
}

struct Verifier<'m> {
    class: HashMap<usize, Class>,
    modes: &'m Modes,
    /// Borrowed parameters.
    lent: Set,
}

impl Verifier<'_> {
    fn counted(&self, x: usize) -> bool {
        self.class.get(&x).copied().is_some_and(counted)
    }

    fn alive(&self, st: &State, x: usize) -> bool {
        if self.lent.contains(&x) || st.cnt.get(&x).copied().unwrap_or(0) > 0 {
            return true;
        }
        match st.parent.get(&x) {
            Some(p) => self.alive(st, *p),
            None => false,
        }
    }

    fn take(&self, st: &mut State, x: usize, what: &str) -> Result<(), String> {
        if !self.counted(x) {
            return Ok(());
        }
        let n = st.cnt.entry(x).or_insert(0);
        if *n < 1 {
            return Err(format!("{what} of v{x}, which holds no reference here"));
        }
        *n -= 1;
        Ok(())
    }

    fn read(&self, st: &State, x: usize) -> Result<(), String> {
        if self.counted(x) && !self.alive(st, x) {
            return Err(format!("borrowed read of v{x}, which is not alive here"));
        }
        Ok(())
    }

    fn ended(&self, st: &State) -> Result<(), String> {
        let mut held: Vec<(usize, i64)> = st.cnt.iter().filter(|(_, n)| **n != 0).map(|(x, n)| (*x, *n)).collect();
        held.sort();
        match held.first() {
            None => Ok(()),
            Some((x, n)) => Err(format!("a returning path still holds {n} reference(s) to v{x}")),
        }
    }

    /// Branches from one state: every branch that returns normally must
    /// leave the same holdings. `Ok(false)` when none returns.
    fn join(&self, st: &mut State, outs: Vec<State>) -> Result<bool, String> {
        let mut it = outs.into_iter();
        let Some(first) = it.next() else {
            return Ok(false);
        };
        let norm = |s: &State| -> Vec<(usize, i64)> {
            let mut v: Vec<(usize, i64)> = s.cnt.iter().filter(|(_, n)| **n != 0).map(|(x, n)| (*x, *n)).collect();
            v.sort();
            v
        };
        let want = norm(&first);
        for s in it {
            if norm(&s) != want {
                return Err(format!("branches disagree on held references: {want:?} versus {:?}", norm(&s)));
            }
        }
        st.cnt = first.cnt;
        Ok(true)
    }

    /// Evaluate `e` for its owned value; `Ok(false)` when it never
    /// returns (the fault path).
    fn value(&mut self, e: &Core, st: &mut State) -> Result<bool, String> {
        match e {
            Core::Fault => Ok(false),
            Core::Var(x) => {
                self.take(st, x.0, "owned use")?;
                Ok(true)
            }
            Core::Dup(x, _, b) => {
                if self.counted(x.0) {
                    if !self.alive(st, x.0) {
                        return Err(format!("dup of v{}, which is not alive here", x.0));
                    }
                    *st.cnt.entry(x.0).or_insert(0) += 1;
                }
                self.value(b, st)
            }
            Core::Drop(x, _, b) => {
                self.take(st, x.0, "drop")?;
                self.value(b, st)
            }
            Core::Release { block, moved, body, .. } => {
                // The moved fields are read through the block before it
                // is released, and each ends with its own reference.
                for (m, _) in moved {
                    if self.counted(m.0) && !self.alive(st, m.0) {
                        return Err(format!("release moves v{}, which is not alive here", m.0));
                    }
                }
                self.take(st, block.0, "release")?;
                for (m, _) in moved {
                    if self.counted(m.0) {
                        *st.cnt.entry(m.0).or_insert(0) += 1;
                    }
                }
                self.value(body, st)
            }
            Core::Let(x, _, v, b) => {
                if !self.value(v, st)? {
                    return Ok(false);
                }
                if self.counted(x.0) {
                    st.cnt.insert(x.0, 1);
                    st.parent.remove(&x.0);
                }
                self.value(b, st)
            }
            Core::App(f, args) => {
                if !self.value(f, st)? {
                    return Ok(false);
                }
                let modes = callee_modes(self.modes, f, args.len());
                let Some(modes) = modes else {
                    return self.all(args, st);
                };
                // A variable passed to a borrowed parameter is read when
                // the call is made, after every argument.
                let mut reads = Vec::new();
                for (i, a) in args.iter().enumerate() {
                    match a {
                        Core::Var(x) if modes[i] => reads.push(x.0),
                        _ => {
                            if !self.value(a, st)? {
                                return Ok(false);
                            }
                        }
                    }
                }
                for x in reads {
                    self.read(st, x)?;
                }
                Ok(true)
            }
            Core::Lam { env: args, .. } | Core::InstDict(_, args, _) => self.all(args, st),
            // A reuse stores one owned reference per argument into the
            // cell, exactly as a constructor allocation does. Its token is
            // a `Raw` word the enclosing `Release` bound, so there is no
            // reference to account for.
            Core::Reuse { args, .. } => self.all(args, st),
            Core::Quote { holes, body, .. } => {
                if let Some(b) = body {
                    verify_body(&[], b, self.modes, &Set::new()).map_err(|m| format!("in a quotation body: {m}"))?;
                }
                self.all(holes, st)
            }
            Core::If(c, t, f) => {
                if !self.value(c, st)? {
                    return Ok(false);
                }
                let mut outs = Vec::new();
                for b in [&t.body, &f.body] {
                    let mut s = st.clone();
                    if self.value(b, &mut s)? {
                        outs.push(s);
                    }
                }
                self.join(st, outs)
            }
            Core::Case { scrut, alts } => {
                let s = match &**scrut {
                    Core::Var(s) => {
                        self.read(st, s.0)?;
                        s.0
                    }
                    _ => return Err("a case on a computed scrutinee (it must be bound first)".into()),
                };
                let mut outs = Vec::new();
                for a in alts {
                    let mut st2 = st.clone();
                    for x in pat_binders(&a.pat) {
                        if self.counted(x) {
                            st2.cnt.insert(x, 0);
                            st2.parent.insert(x, s);
                        }
                    }
                    if self.value(&a.body, &mut st2)? {
                        for x in pat_binders(&a.pat) {
                            st2.parent.remove(&x);
                        }
                        outs.push(st2);
                    }
                }
                self.join(st, outs)
            }
            Core::Proj(d, _) | Core::DictProj(d, ..) | Core::Demand(d, _) | Core::BoxGet(d, _) | Core::BoxClear(d, _) => match &**d {
                Core::Var(r) => {
                    self.read(st, r.0)?;
                    Ok(true)
                }
                other => {
                    // A container read twice over (`Proj (Proj r i) j`)
                    // is never produced: each computed container is
                    // bound.
                    Err(format!("a projection from an unbound container {}", short(other)))
                }
            },
            Core::BoxInit(l, v, _) | Core::LazyInit(l, v) | Core::RefSet(l, v, _) => {
                if !self.value(v, st)? {
                    return Ok(false);
                }
                self.read(st, l.0)?;
                Ok(true)
            }
            Core::Join { id, params, body, rest } => {
                // The body is a code block of its own entry: one reference
                // per counted parameter, and it must consume each on every
                // path while leaving the enclosing holdings as it found them
                // (it may dup them, never take them).
                let mut s = st.clone();
                for (x, _) in params {
                    if self.counted(x.0) {
                        s.cnt.insert(x.0, 1);
                        s.parent.remove(&x.0);
                    }
                }
                if self.value(body, &mut s)? {
                    for (x, _) in params {
                        if self.counted(x.0) && s.cnt.get(&x.0).copied().unwrap_or(0) != 0 {
                            return Err(format!(
                                "join j{} leaves its parameter v{} unconsumed on a path",
                                id.0, x.0
                            ));
                        }
                    }
                    let norm = |state: &State, skip: &[(LocalId, Class)]| -> Vec<(usize, i64)> {
                        let mut v: Vec<(usize, i64)> = state
                            .cnt
                            .iter()
                            .filter(|(x, n)| **n != 0 && !skip.iter().any(|(p, _)| p.0 == **x))
                            .map(|(x, n)| (*x, *n))
                            .collect();
                        v.sort();
                        v
                    };
                    let after = norm(&s, params);
                    let before = norm(st, &[]);
                    if after != before {
                        return Err(format!(
                            "join j{}'s body changes the enclosing holdings: {before:?} to {after:?}",
                            id.0
                        ));
                    }
                }
                self.value(rest, st)
            }
            Core::Jump { args, .. } => {
                // A jump hands one owned reference per argument over and
                // never returns to this block.
                if !self.all(args, st)? {
                    return Ok(false);
                }
                Ok(false)
            }
            _ => Ok(true),
        }
    }

    fn all(&mut self, es: &[Core], st: &mut State) -> Result<bool, String> {
        for a in es {
            if !self.value(a, st)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

fn short(e: &Core) -> String {
    let s = format!("{e:?}");
    s.chars().take(80).collect()
}

/// The class of `e`'s value when its form fixes it, binders by `classes`.
/// Bind the computed arguments of every saturated direct call to locals,
/// left to right, before borrow inference runs.
///
/// A callee's parameter is lent only while every caller passes it a local
/// (`call_sites`): a computed argument (`f (g x)`) is an owned temporary the
/// callee would have to release, so one such call site withdrew the lent
/// mode for every caller of the unit. That made the common shapes owned
/// everywhere: `natCmp (slots-read s i) x` cost a dup before and a drop
/// inside every comparison, and the drops in turn made `natCmp` too large
/// for clang to inline. Named first, the temporary is an ordinary local:
/// the callee borrows it, and the caller releases it after the call.
///
/// Evaluation order is kept: an argument is named only when every computed
/// argument before it is named too, so the arguments still run left to
/// right. An argument whose class is not known is left in place, and with
/// it every argument after it. Values (locals, constants, top-level units)
/// need no name.
fn name_call_arguments(program: &mut Program) {
    let results: HashMap<String, Class> = program
        .fns
        .iter()
        .filter_map(|f| f.repr.result.map(|c| (f.id.0.clone(), c)))
        .filter(|(_, c)| !matches!(c, Class::Param(_)))
        .collect();
    let arity: HashMap<String, usize> = program.fns.iter().map(|f| (f.id.0.clone(), f.params.len())).collect();
    for f in program.fns.iter_mut() {
        if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
            continue;
        }
        let mut next = max_local_in(&f.body).max(f.params.len()) + 1;
        let mut classes: HashMap<usize, Class> = HashMap::new();
        name_args_in(&mut f.body, &results, &arity, &mut classes, &mut next);
    }
}

fn max_local_in(e: &Core) -> usize {
    let mut most = 0;
    crate::core::walk_locals(e, &mut |l| most = most.max(l.0));
    most
}

/// Whether an argument needs no name: evaluating it is free and repeatable.
fn trivial_argument(e: &Core) -> bool {
    matches!(
        e,
        Core::Var(_) | Core::Int(_) | Core::Bool(_) | Core::Unit | Core::ConstSym(_)
            | Core::Global(..) | Core::Prim(_) | Core::Ctor(..)
    ) || small_literal(e)
}

/// A `Nat`/`Int` literal that is an immediate word on every target (code
/// generation's `immediate_word` keeps 30 bits plus the tag even on a
/// 32-bit word), so it needs no allocation and no count.
fn small_literal(e: &Core) -> bool {
    matches!(e, Core::ConstBig { digits, .. } if digits.parse::<i64>().is_ok_and(|v| v.unsigned_abs() < 1 << 29))
}

/// The class of a computed argument, when it is known.
fn argument_class(e: &Core, results: &HashMap<String, Class>, classes: &HashMap<usize, Class>) -> Option<Class> {
    match e {
        Core::App(head, _) => match &**head {
            Core::Global(g, _) => results.get(&g.0).copied(),
            Core::Prim(op) => crate::codegen::prim_result_class(*op).filter(|c| !matches!(c, Class::Param(_))),
            Core::Ctor(..) => Some(Class::Obj),
            _ => None,
        },
        other => expr_class_in(other, classes).filter(|c| !matches!(c, Class::Param(_))),
    }
}

fn name_args_in(
    e: &mut Core,
    results: &HashMap<String, Class>,
    arity: &HashMap<String, usize>,
    classes: &mut HashMap<usize, Class>,
    next: &mut usize,
) {
    if let Core::Let(l, c, _, _) = e {
        classes.insert(l.0, *c);
    }
    for k in crate::core::child_slots(e) {
        name_args_in(k, results, arity, classes, next);
    }
    let Core::App(head, args) = e else { return };
    let saturated = match &**head {
        Core::Global(g, _) => arity.get(&g.0) == Some(&args.len()),
        Core::Prim(op) => op.arity() == args.len(),
        _ => false,
    };
    if !saturated {
        return;
    }
    // The longest prefix of the arguments in which every computed
    // argument has a known class: those are named; the rest stay.
    let mut plan: Vec<Option<Class>> = Vec::with_capacity(args.len());
    for a in args.iter() {
        if trivial_argument(a) {
            plan.push(None);
            continue;
        }
        match argument_class(a, results, classes) {
            Some(c) => plan.push(Some(c)),
            None => break,
        }
    }
    // Only worth it when a named argument is counted (a raw temporary
    // owns nothing to lend).
    if !plan.iter().any(|c| c.is_some_and(|c| matches!(c, Class::Obj | Class::TObj))) {
        return;
    }
    let mut bindings: Vec<(LocalId, Class, Core)> = Vec::new();
    for (i, slot) in plan.iter().enumerate() {
        if let Some(c) = slot {
            let l = LocalId(*next);
            *next += 1;
            classes.insert(l.0, *c);
            let value = std::mem::replace(&mut args[i], Core::Var(l));
            bindings.push((l, *c, value));
        }
    }
    let mut out = std::mem::replace(e, Core::Unit);
    for (l, c, value) in bindings.into_iter().rev() {
        out = Core::Let(l, c, Box::new(value), Box::new(out));
    }
    *e = out;
}

fn expr_class_in(e: &Core, classes: &HashMap<usize, Class>) -> Option<Class> {
    match e {
        Core::Var(x) => Some(classes.get(&x.0).copied().unwrap_or(Class::Raw)),
        Core::Let(_, _, _, b) => expr_class_in(b, classes),
        Core::ConstBig { .. } => Some(Class::TObj),
        Core::ConstBytes(_) | Core::ConstString { .. } | Core::Lam { .. } | Core::InstDict(..)
        | Core::LazyAlloc | Core::BoxAlloc => Some(Class::Obj),
        Core::Int(_) | Core::Bool(_) | Core::Unit | Core::ConstSym(_) | Core::LazyInit(..)
        | Core::RefSet(..) | Core::BoxClear(..) => Some(Class::Raw),
        Core::Proj(_, f) => Some(concrete(f.class)),
        Core::DictProj(_, _, _, c) | Core::Demand(_, c) | Core::BoxGet(_, c) | Core::BoxInit(_, _, c) => {
            Some(concrete(*c))
        }
        Core::App(f, _) if matches!(**f, Core::Ctor(..)) => Some(Class::Obj),
        _ => None,
    }
}

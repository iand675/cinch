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
//! Quotation bodies are reified for the runtime evaluator and are left
//! untouched.
//!
//! # Verification
//!
//! In debug builds (and so in every test) [`verify`] re-derives the
//! ownership of the transformed body and refuses any path on which an
//! owned binder is not consumed exactly once: a compiler-bug trap at
//! compile time (design/no-faults.md).

use crate::core::{CAlt, CBranch, CPat, Core, FnDef, LocalId, Program};
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
/// the profile reclaims ([`crate::core::TargetProfile::reclaim`]).
pub fn insert(mut program: Program, profile: &crate::core::TargetProfile) -> crate::Result<Program> {
    if !profile.reclaim {
        return Ok(program);
    }
    for f in &mut program.fns {
        if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
            continue;
        }
        let body = transform(f);
        if cfg!(debug_assertions) {
            verify(f, &body).map_err(|m| {
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

fn param_classes(f: &FnDef) -> Vec<Class> {
    (0..f.arity()).map(|i| f.repr.params.get(i).copied().map_or(Class::Raw, concrete)).collect()
}

/// Every binder's class: the parameters', and the `let` and pattern
/// binders' recorded classes.
fn binder_classes(f: &FnDef, body: &Core) -> HashMap<usize, Class> {
    fn walk(e: &Core, out: &mut HashMap<usize, Class>) {
        match e {
            Core::Let(x, c, ..) => {
                out.insert(x.0, concrete(*c));
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
    for (i, c) in param_classes(f).into_iter().enumerate() {
        out.insert(i, c);
    }
    walk(body, &mut out);
    out
}

fn transform(f: &FnDef) -> Core {
    let class = binder_classes(f, &f.body);
    let mut next = f.arity();
    crate::core::walk_locals(&f.body, &mut |l| next = next.max(l.0 + 1));
    for k in class.keys() {
        next = next.max(k + 1);
    }
    let mut pass = Pass { class, fv: HashMap::new(), alias: HashMap::new(), next };
    let owned: Set = (0..f.arity()).filter(|i| pass.counted(*i)).collect();
    pass.go(&f.body, &owned, &Set::new())
}

struct Pass {
    class: HashMap<usize, Class>,
    /// Free counted binders of the original body's nodes, by address.
    /// Only nodes of the input body (alive for the whole pass) are keyed.
    fv: HashMap<*const Core, Rc<Set>>,
    /// Variable alternatives whose binder took over the scrutinee's
    /// reference: their uses are the scrutinee's.
    alias: HashMap<usize, usize>,
    next: usize,
}

impl Pass {
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
            Core::LazyInit(l, v) | Core::BoxInit(l, v, _) | Core::RefSet(l, v) => {
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
                Core::Quote { template: template.clone(), holes: self.seq(&es, owned, live), body: body.clone() }
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
            Core::BoxInit(l, v, class) => {
                let (l, class) = (self.resolve(l.0), *class);
                self.write_through(l, v, owned, live, false, class, move |v| {
                    Core::BoxInit(LocalId(l), Box::new(v), class)
                })
            }
            Core::LazyInit(l, v) => {
                let l = self.resolve(l.0);
                self.write_through(l, v, owned, live, false, Class::Raw, move |v| {
                    Core::LazyInit(LocalId(l), Box::new(v))
                })
            }
            Core::RefSet(l, v) => {
                let l = self.resolve(l.0);
                self.write_through(l, v, owned, live, true, Class::Raw, move |v| {
                    Core::RefSet(LocalId(l), Box::new(v))
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
            Core::ConstBig { .. } => Some(Class::TObj),
            Core::ConstBytes(_) | Core::ConstString { .. } | Core::Lam { .. } | Core::InstDict(..)
            | Core::LazyAlloc | Core::BoxAlloc => Some(Class::Obj),
            Core::Int(_) | Core::Bool(_) | Core::Unit | Core::ConstSym(_) | Core::LazyInit(..)
            | Core::RefSet(..) => Some(Class::Raw),
            Core::Proj(_, f) => Some(concrete(f.class)),
            Core::DictProj(_, _, _, c) | Core::Demand(_, c) | Core::BoxGet(_, c) | Core::BoxInit(_, _, c) => {
                Some(concrete(*c))
            }
            Core::App(f, _) if matches!(**f, Core::Ctor(..)) => Some(Class::Obj),
            _ => None,
        }
    }

    /// The alternatives of a `case` on the owned-or-raw binder `s`.
    fn case_on(&mut self, s: usize, alts: &[CAlt<Core>], owned: &Set, live: &Set) -> Core {
        let mut out = Vec::with_capacity(alts.len());
        for a in alts {
            let body = match &a.pat {
                CPat::Con { fields, .. } => {
                    let nb = self.need(&a.body);
                    let used: Vec<usize> =
                        fields.iter().map(|x| x.0).filter(|x| self.counted(*x) && nb.contains(x)).collect();
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
    /// evaluating `v` (owned). `replace` marks `RefSet`, whose old value
    /// dies during `v` and whose binder owns the new one.
    #[allow(clippy::too_many_arguments)]
    fn write_through(
        &mut self,
        l: usize,
        v: &Core,
        owned: &Set,
        live: &Set,
        replace: bool,
        class: Class,
        make: impl FnOnce(Core) -> Core,
    ) -> Core {
        let mut keep = live.clone();
        if replace {
            keep.remove(&l);
        } else {
            keep.insert(l);
        }
        let lv: Set = owned.intersection(&keep).copied().collect();
        let v2 = self.go(v, owned, &lv);
        let node = make(v2);
        let holds = if replace { self.counted(l) } else { self.counted(l) && owned.contains(&l) };
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
/// a `let` and a `RefSet`; none for a pattern binder, which borrows from
/// its live scrutinee until a `Dup`), an owned use or `Drop` takes one, a
/// `Dup` adds one, a borrowed read needs one, and every path that returns
/// ends with none held.
pub fn verify(f: &FnDef, body: &Core) -> Result<(), String> {
    let class = binder_classes(f, body);
    let mut st = State { cnt: HashMap::new(), parent: HashMap::new() };
    for i in 0..f.arity() {
        if class.get(&i).copied().is_some_and(counted) {
            st.cnt.insert(i, 1);
        }
    }
    let mut v = Verifier { class };
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

struct Verifier {
    class: HashMap<usize, Class>,
}

impl Verifier {
    fn counted(&self, x: usize) -> bool {
        self.class.get(&x).copied().is_some_and(counted)
    }

    fn alive(&self, st: &State, x: usize) -> bool {
        if st.cnt.get(&x).copied().unwrap_or(0) > 0 {
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
                self.all(args, st)
            }
            Core::Lam { env: args, .. } | Core::InstDict(_, args, _) | Core::Quote { holes: args, .. } => {
                self.all(args, st)
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
            Core::Proj(d, _) | Core::DictProj(d, ..) | Core::Demand(d, _) | Core::BoxGet(d, _) => match &**d {
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
            Core::BoxInit(l, v, _) | Core::LazyInit(l, v) => {
                if !self.value(v, st)? {
                    return Ok(false);
                }
                self.read(st, l.0)?;
                Ok(true)
            }
            Core::RefSet(l, v) => {
                if !self.value(v, st)? {
                    return Ok(false);
                }
                if self.counted(l.0) {
                    if st.cnt.get(&l.0).copied().unwrap_or(0) != 0 {
                        return Err(format!("v{}'s old value survives its replacement", l.0));
                    }
                    st.cnt.insert(l.0, 1);
                }
                Ok(true)
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

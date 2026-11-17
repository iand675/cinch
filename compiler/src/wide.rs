//! The wide edge pass (design/wide-scalars.md section 3).
//!
//! A value of class `Wide` (an `I128`/`U128`) is an `i128` in every wide
//! position the emitter types: a local, a unit's parameter or result, a
//! direct call's argument, a constructor field. Some positions hold one
//! word whatever the value's class, because the code reading them is
//! shared: an indirect call's arguments and result (the apply frame), a
//! closure's saved words, a dictionary's value slots, lazy and mutable
//! cells, parallel payloads. There a wide value is *boxed*: an owned word
//! pointing at a `WideBox`, a one-field constructor whose field is wide.
//!
//! This pass makes every such edge explicit in `Core`, so the emitter has
//! two total rules and no convention to remember. It walks each unit with
//! one fact per value position, the representation the consumer wants
//! (`Rep::Wide` or `Rep::Word`), compares it with what the producer
//! gives, and where they disagree inserts one of the two coercions:
//!
//! - **box**: `WideBox e`, a constructor application, so allocation,
//!   layout and ownership are the constructor machinery's;
//! - **unbox**: `let b = e in b.0`, a projection of the box's wide field,
//!   so `crate::rc` sees an ordinary owned `Obj` binder read once and
//!   drops it.
//!
//! A word position's own class site (a `DictProj`, `Demand`, cell or
//! `Par` class) that says `Wide` is rewritten to `Obj`: the position holds
//! the box, which is counted. The apply stub is the one edge outside
//! `Core`; `codegen` unboxes and boxes there with the same layout.
//!
//! The pass runs after the collector (classes are concrete) and the
//! simplifier (joins exist), and before reference counting.

use crate::core::{CPat, Core, FnDef, GlobalId, LocalId, PrimOp, Program, ProjField};
use crate::repr::Class;
use std::collections::{HashMap, HashSet};

/// The builtin box of a wide value in a word position.
pub const WIDE_BOX: &str = "<builtin>.WideBox";

/// What a value position holds: an `i128`, or one word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rep {
    Wide,
    Word,
}

fn rep(c: Class) -> Rep {
    if c == Class::Wide {
        Rep::Wide
    } else {
        Rep::Word
    }
}

/// A callee's boundary: each parameter's representation and the result's.
#[derive(Clone)]
struct Sig {
    params: Vec<Rep>,
    ret: Rep,
}

/// `program` with its edges inserted, borrowed unchanged when it holds no
/// wide value. The pass is idempotent (an inserted edge's two sides then
/// agree), so code generation applies it to a program reference counting
/// already saw, and to one that never reaches reference counting (an
/// inspection emit of a checked program).
pub fn with_edges(program: &Program) -> std::borrow::Cow<'_, Program> {
    if !program.fns.iter().any(mentions_wide) {
        return std::borrow::Cow::Borrowed(program);
    }
    let mut p = program.clone();
    insert_edges(&mut p);
    std::borrow::Cow::Owned(p)
}

/// Insert the box and unbox edges of every unit, and the `WideBox`
/// constructor when an edge needs it. A program with no wide value is
/// returned unchanged.
pub fn insert_edges(program: &mut Program) {
    if !program.fns.iter().any(mentions_wide) {
        return;
    }
    let sigs = signatures(program);
    let mut boxed = false;
    for f in &mut program.fns {
        if f.prim.is_some() || f.ctor.is_some() {
            continue;
        }
        let mut wide: HashSet<LocalId> = f
            .repr
            .params
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == Class::Wide)
            .map(|(i, _)| LocalId(i))
            .collect();
        let mut next = next_local(f);
        let want = f.repr.result.map_or(Rep::Word, rep);
        let body = std::mem::replace(&mut f.body, Core::Unit);
        let mut cx = Cx { sigs: &sigs, wide: &mut wide, joins: HashMap::new(), next: &mut next, boxed: &mut boxed };
        f.body = cx.go(body, want);
    }
    if boxed && !program.fns.iter().any(|f| f.id.0 == WIDE_BOX) {
        program.fns.push(FnDef {
            id: GlobalId(WIDE_BOX.into()),
            params: vec!["v".into()],
            body: Core::Unit,
            prim: None,
            ctor: Some((0, 1)),
            hints: Default::default(),
            meta: Default::default(),
            repr: crate::core::UnitRepr {
                tparams: 0,
                params: vec![Class::Wide],
                fields: vec![Class::Wide],
                data: None,
                result: Some(Class::Obj),
                dictionary_values: Vec::new(),
            },
        });
    }
}

/// Whether a unit's boundary or body mentions a wide class or constant.
fn mentions_wide(f: &FnDef) -> bool {
    if f.repr.result == Some(Class::Wide) || f.repr.params.contains(&Class::Wide) || f.repr.fields.contains(&Class::Wide) {
        return true;
    }
    let mut found = false;
    crate::core::for_each_class(&f.body, &mut |c| found |= c == Class::Wide);
    found
}

/// Every callee's boundary, by unit name: functions and constructors from
/// their classes, primitives over wide scalars from their signatures.
fn signatures(program: &Program) -> HashMap<String, Sig> {
    let mut out = HashMap::new();
    for f in &program.fns {
        let params = if f.ctor.is_some() { &f.repr.fields } else { &f.repr.params };
        out.insert(
            f.id.0.clone(),
            Sig { params: params.iter().map(|c| rep(*c)).collect(), ret: f.repr.result.map_or(Rep::Word, rep) },
        );
    }
    // A foreign import's wrapper: a wide scalar crosses as itself.
    for (name, plan) in &program.foreigns {
        if plan.export {
            continue;
        }
        let r = |a: &crate::core::AbiParam| match a.class {
            crate::core::AbiClass::Scalar(s) if s.is_wide() => Rep::Wide,
            _ => Rep::Word,
        };
        out.insert(name.clone(), Sig { params: plan.params.iter().map(r).collect(), ret: r(&plan.result) });
    }
    out
}

/// A primitive's boundary: its wide scalar positions are wide.
fn prim_sig(op: PrimOp) -> Option<Sig> {
    if !op.touches_wide() {
        return None;
    }
    let (params, ret) = op.sig();
    let r = |t: &crate::types::Ty| match t {
        crate::types::Ty::Prim(s) if s.is_wide() => Rep::Wide,
        crate::types::Ty::Param(_) if matches!(op,
            PrimOp::MVar { classes, .. } | PrimOp::Slots { classes, .. }
                if classes[0] == Class::Wide) => Rep::Wide,
        _ => Rep::Word,
    };
    Some(Sig { params: params.iter().map(r).collect(), ret: r(&ret) })
}

/// The first local id no binder or use in `f` takes.
fn next_local(f: &FnDef) -> u32 {
    fn walk(e: &Core, m: &mut u32) {
        let mut see = |l: LocalId| *m = (*m).max(l.0 as u32 + 1);
        match e {
            Core::Var(l) | Core::Let(l, ..) | Core::LazyInit(l, _) => see(*l),
            Core::Join { params, .. } => params.iter().for_each(|(l, _)| see(*l)),
            Core::Case { alts, .. } => {
                for a in alts {
                    match &a.pat {
                        CPat::Var(l, _) => see(*l),
                        CPat::Con { fields, .. } => fields.iter().for_each(|l| see(*l)),
                        CPat::Wild => {}
                    }
                }
            }
            _ => {}
        }
        if let Core::App(h, _) = e {
            walk(h, m);
        }
        for k in crate::core::child_slots_imm(e) {
            walk(k, m);
        }
    }
    let mut m = f.arity() as u32;
    walk(&f.body, &mut m);
    m
}

struct Cx<'a> {
    sigs: &'a HashMap<String, Sig>,
    wide: &'a mut HashSet<LocalId>,
    /// Each join in scope's parameter representations.
    joins: HashMap<crate::core::JoinId, Vec<Rep>>,
    next: &'a mut u32,
    /// Whether an edge boxed, so the `WideBox` constructor is needed.
    boxed: &'a mut bool,
}

impl Cx<'_> {
    fn fresh(&mut self) -> LocalId {
        let l = LocalId(*self.next as usize);
        *self.next += 1;
        l
    }

    /// `e`, producing `have`, delivered where `want` is expected.
    fn coerce(&mut self, e: Core, have: Rep, want: Rep) -> Core {
        match (have, want) {
            (Rep::Wide, Rep::Word) => {
                *self.boxed = true;
                Core::App(Box::new(Core::Ctor(GlobalId(WIDE_BOX.into()), Vec::new())), vec![e])
            }
            (Rep::Word, Rep::Wide) => {
                let b = self.fresh();
                let field = ProjField { class: Class::Wide, ..ProjField::slot(0, Class::Wide) };
                Core::Let(b, Class::Obj, Box::new(e), Box::new(Core::Proj(Box::new(Core::Var(b)), field)))
            }
            _ => e,
        }
    }

    /// The saturated boundary of a call head, if the head is a known unit.
    fn direct(&self, f: &Core, n: usize) -> Option<Sig> {
        let sig = match f {
            Core::Global(id, _) | Core::Ctor(id, _) => self.sigs.get(&id.0).cloned()?,
            Core::Foreign(name) => self.sigs.get(name).cloned()?,
            Core::Prim(op) => match prim_sig(*op) {
                Some(s) => s,
                // A word primitive: every position is a word.
                None => Sig { params: vec![Rep::Word; op.arity()], ret: Rep::Word },
            },
            _ => return None,
        };
        (sig.params.len() == n).then_some(sig)
    }

    /// Rewrite `e` for a consumer wanting `want`.
    fn go(&mut self, e: Core, want: Rep) -> Core {
        match e {
            Core::Var(x) => {
                let have = if self.wide.contains(&x) { Rep::Wide } else { Rep::Word };
                self.coerce(Core::Var(x), have, want)
            }
            Core::Wide(v) => self.coerce(Core::Wide(v), Rep::Wide, want),
            Core::Let(x, c, v, body) => {
                if c == Class::Wide {
                    self.wide.insert(x);
                }
                let v = self.go(*v, rep(c));
                let body = self.go(*body, want);
                Core::Let(x, c, Box::new(v), Box::new(body))
            }
            Core::App(f, args) => match self.direct(&f, args.len()) {
                Some(sig) => {
                    let args = args.into_iter().zip(&sig.params).map(|(a, r)| self.go(a, *r)).collect();
                    self.coerce(Core::App(f, args), sig.ret, want)
                }
                // An indirect, partial or over-application goes through
                // the apply frame: its arguments and its result are words.
                None => {
                    let f = self.go(*f, Rep::Word);
                    let args = args.into_iter().map(|a| self.go(a, Rep::Word)).collect();
                    self.coerce(Core::App(Box::new(f), args), Rep::Word, want)
                }
            },
            // A zero-arity unit as a value is its call; any other unit as a
            // value is its closure, a word.
            Core::Global(..) | Core::Prim(_) => {
                let have = match self.direct(&e, 0) {
                    Some(sig) => sig.ret,
                    None => Rep::Word,
                };
                self.coerce(e, have, want)
            }
            Core::If(c, mut t, mut f) => {
                let c = self.go(*c, Rep::Word);
                t.body = Box::new(self.go(*t.body, want));
                f.body = Box::new(self.go(*f.body, want));
                Core::If(Box::new(c), t, f)
            }
            Core::Case { scrut, mut alts } => {
                let binds_wide = alts.iter().any(|a| matches!(a.pat, CPat::Var(_, Class::Wide)));
                let binds_word = alts.iter().any(|a| matches!(a.pat, CPat::Var(..) | CPat::Con { .. }));
                for a in &alts {
                    match &a.pat {
                        CPat::Var(x, Class::Wide) => {
                            self.wide.insert(*x);
                        }
                        CPat::Con { fields, classes, .. } => {
                            for (x, c) in fields.iter().zip(classes) {
                                if *c == Class::Wide {
                                    self.wide.insert(*x);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                // A wildcard-only case keeps its scrutinee as produced.
                let scrut_want = if binds_wide {
                    Rep::Wide
                } else if binds_word {
                    Rep::Word
                } else {
                    self.produces(&scrut)
                };
                let scrut = self.go(*scrut, scrut_want);
                for a in &mut alts {
                    let body = std::mem::replace(&mut *a.body, Core::Unit);
                    *a.body = self.go(body, want);
                }
                Core::Case { scrut: Box::new(scrut), alts }
            }
            Core::Join { id, params, body, rest } => {
                for (x, c) in &params {
                    if *c == Class::Wide {
                        self.wide.insert(*x);
                    }
                }
                self.joins.insert(id, params.iter().map(|(_, c)| rep(*c)).collect());
                let rest = self.go(*rest, want);
                let body = self.go(*body, want);
                Core::Join { id, params, body: Box::new(body), rest: Box::new(rest) }
            }
            Core::Jump { id, args } => {
                let reps = self.joins.get(&id).cloned().unwrap_or_default();
                let args = args
                    .into_iter()
                    .enumerate()
                    .map(|(i, a)| self.go(a, reps.get(i).copied().unwrap_or(Rep::Word)))
                    .collect();
                Core::Jump { id, args }
            }
            // A closure saves words.
            Core::Lam { fn_id, env, classes } => {
                let env = env.into_iter().map(|a| self.go(a, Rep::Word)).collect();
                self.coerce(Core::Lam { fn_id, env, classes }, Rep::Word, want)
            }
            Core::Proj(d, f) => {
                let have = rep(f.class);
                let d = self.go(*d, Rep::Word);
                self.coerce(Core::Proj(Box::new(d), f), have, want)
            }
            // Word positions whose own class says what they hold: a wide
            // value there is its box, a counted word.
            Core::DictProj(d, i, site, c) => {
                let d = self.go(*d, Rep::Word);
                let c = if c == Class::Wide { Class::Obj } else { c };
                self.coerce(Core::DictProj(Box::new(d), i, site, c), Rep::Word, want)
            }
            Core::Demand(d, c) => {
                let d = self.go(*d, Rep::Word);
                let c = if c == Class::Wide { Class::Obj } else { c };
                self.coerce(Core::Demand(Box::new(d), c), Rep::Word, want)
            }
            Core::BoxGet(d, c) => {
                let d = self.go(*d, Rep::Word);
                let c = if c == Class::Wide { Class::Obj } else { c };
                self.coerce(Core::BoxGet(Box::new(d), c), Rep::Word, want)
            }
            Core::BoxInit(cell, v, c) => {
                let v = self.go(*v, Rep::Word);
                let c = if c == Class::Wide { Class::Obj } else { c };
                Core::BoxInit(cell, Box::new(v), c)
            }
            Core::RefSet(l, v, c) => {
                let v = self.go(*v, Rep::Word);
                let c = if c == Class::Wide { Class::Obj } else { c };
                Core::RefSet(l, Box::new(v), c)
            }
            // Count operations (present when the pass runs again after
            // reference counting) guard what they precede: their body is
            // the value, so it keeps the consumer's representation.
            Core::Dup(x, c, body) => {
                let body = self.go(*body, want);
                Core::Dup(x, c, Box::new(body))
            }
            Core::Drop(x, c, body) => {
                let body = self.go(*body, want);
                Core::Drop(x, c, Box::new(body))
            }
            Core::Release { block, moved, rest, token, body } => {
                let body = self.go(*body, want);
                Core::Release { block, moved, rest, token, body: Box::new(body) }
            }
            // A quotation's body is reified, not emitted; code generation
            // refuses a wide value in one.
            e @ Core::Quote { .. } => e,
            // Everything else: its children are word positions, and it
            // produces a word (a constant, a closure, a block, a cell).
            mut e => {
                if let Core::Prim(PrimOp::Par { classes, .. }) = &mut e {
                    for c in classes.iter_mut() {
                        if *c == Class::Wide {
                            *c = Class::Obj;
                        }
                    }
                }
                for k in crate::core::child_slots(&mut e) {
                    let child = std::mem::replace(k, Core::Unit);
                    *k = self.go(child, Rep::Word);
                }
                // A fault produces whatever its position wants.
                if matches!(e, Core::Fault) {
                    return e;
                }
                self.coerce(e, Rep::Word, want)
            }
        }
    }

    /// What an expression produces, without rewriting it.
    fn produces(&self, e: &Core) -> Rep {
        match e {
            Core::Wide(_) => Rep::Wide,
            Core::Var(x) => {
                if self.wide.contains(x) {
                    Rep::Wide
                } else {
                    Rep::Word
                }
            }
            Core::App(f, args) => self.direct(f, args.len()).map_or(Rep::Word, |s| s.ret),
            Core::Global(..) | Core::Prim(_) => self.direct(e, 0).map_or(Rep::Word, |s| s.ret),
            Core::Proj(_, f) => rep(f.class),
            Core::Let(_, _, _, b) => self.produces(b),
            _ => Rep::Word,
        }
    }
}

//! The representation collector (option C, `design/layouts-and-specialization.md`
//! section 2): every polymorphic unit is copied once per class vector of
//! its type parameters that the program reaches, and every reference is
//! renamed to the copy it reaches.
//!
//! Roots are every unit that is not a generic body: monomorphic
//! definitions (which subsume `main`'s exit adapter, exports, tests and
//! task bodies), constructors, primitives and foreign boundary units.
//! The worklist only ever holds pairs of a unit and a concrete vector
//! over `Raw`/`Obj`/`TObj`, and a unit with `k` parameters has at most
//! `3^k` of them, so it terminates under polymorphic recursion: a type a
//! constructor is applied to is `Obj` whatever its arguments
//! (`crate::repr::repr_of`), so recursing at `Pair a a` reaches `[Obj]`.
//!
//! A constructor unit whose field classes mention its type's parameters
//! (`Cons` is `[Param(0), Obj]`) is copied per vector its uses reach as
//! well, with those fields resolved, so each copy's blocks carry the
//! pointer map of that vector (design/reclamation.md section 3, slice
//! S5). It has no body, so reaching it reaches nothing further. A
//! constructor whose fields do not depend on the vector stays one unit.
//!
//! Dictionaries are copied per vector of the instance's own parameters.
//! A method with type parameters of its own (beyond the class's) gets one
//! dictionary slot per own vector some projection of that class reaches;
//! every dictionary of the class shares the slot table, so a projection
//! through a passed dictionary finds its copy without knowing the
//! instance. The first such vector keeps the method's declared slot and
//! the others are appended after the declared methods.
use crate::core::{child_slots, child_slots_imm, subst_classes, Class, Core, FnDef, GlobalId, InstanceDef, OptReport, Program};
use crate::repr::spec_name;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

type Vector = Vec<Class>;

/// A vector fitted to `n` parameters: concrete classes kept, a missing or
/// non-concrete entry `Raw`. The flag says whether anything was filled.
fn fit(v: &[Class], n: usize) -> (Vector, bool) {
    let mut filled = v.len() != n;
    let out = (0..n)
        .map(|i| match v.get(i) {
            Some(c) if !c.is_param() => *c,
            _ => {
                filled = true;
                Class::Raw
            }
        })
        .collect();
    (out, filled)
}

enum Item {
    Unit(String, Vector),
    Inst(String, Vector),
}

struct Cx<'a> {
    fns: HashMap<&'a str, &'a FnDef>,
    insts: HashMap<&'a str, &'a InstanceDef>,
    /// Generic units a foreign boundary names: kept under their own name.
    pinned: HashSet<&'a str>,
    units: BTreeMap<String, BTreeSet<Vector>>,
    inst_copies: BTreeMap<String, BTreeSet<Vector>>,
    /// Reached dictionaries per class.
    by_class: BTreeMap<String, BTreeSet<(String, Vector)>>,
    /// Own vectors reached per (class, method index) through projections.
    own: BTreeMap<(String, usize), BTreeSet<Vector>>,
    /// Reached vectors per vector-dependent constructor unit.
    ctors: BTreeMap<String, BTreeSet<Vector>>,
    work: Vec<Item>,
    unrecorded: usize,
}

impl<'a> Cx<'a> {
    fn generic(&self, id: &str) -> Option<&'a FnDef> {
        let f = *self.fns.get(id)?;
        let body = f.prim.is_none() && f.ctor.is_none() && !f.meta.prim_body;
        (body && f.repr.tparams > 0 && !self.pinned.contains(id)).then_some(f)
    }

    /// A constructor unit whose field classes depend on its class vector.
    fn ctor_generic(&self, id: &str) -> Option<&'a FnDef> {
        let f = *self.fns.get(id)?;
        let dependent = f.repr.fields.iter().any(|c| c.is_param());
        (f.ctor.is_some() && f.repr.tparams > 0 && dependent && !self.pinned.contains(id)).then_some(f)
    }

    fn reach_unit(&mut self, id: &str, v: Vector) {
        if self.units.entry(id.to_string()).or_default().insert(v.clone()) {
            self.work.push(Item::Unit(id.to_string(), v));
        }
    }

    /// Reach constructor unit `id` at the class vector of its use.
    fn reach_ctor(&mut self, id: &str, v: &[Class]) {
        if let Some(f) = self.ctor_generic(id) {
            let (v, filled) = fit(v, f.repr.tparams);
            self.unrecorded += filled as usize;
            self.ctors.entry(id.to_string()).or_default().insert(v);
        }
    }

    fn reach_method(&mut self, method: &GlobalId, v: Vector) {
        if let Some(f) = self.generic(&method.0) {
            let (v, _) = fit(&v, f.repr.tparams);
            self.reach_unit(&method.0, v);
        }
    }

    /// The vectors of method `i` in a dictionary at `iv`: the instance's,
    /// followed by each own vector the class's projections reached.
    fn method_vectors(&self, inst: &InstanceDef, i: usize, iv: &[Class]) -> Vec<Vector> {
        let k = inst.own.get(i).copied().unwrap_or(0);
        if k == 0 {
            return vec![iv.to_vec()];
        }
        match self.own.get(&(inst.class.clone(), i)) {
            Some(set) if !set.is_empty() => set
                .iter()
                .map(|o| {
                    let mut v = iv.to_vec();
                    v.extend(o.iter().copied());
                    v
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn scan(&mut self, e: &Core) {
        match e {
            Core::Global(g, v) | Core::Lam { fn_id: g, classes: v, .. } => {
                if let Some(f) = self.generic(&g.0) {
                    let (v, filled) = fit(v, f.repr.tparams);
                    self.unrecorded += filled as usize;
                    self.reach_unit(&g.0, v);
                } else {
                    self.reach_ctor(&g.0, v);
                }
            }
            Core::Ctor(g, v) => self.reach_ctor(&g.0, v),
            Core::InstDict(g, _, iv) => {
                if let Some(inst) = self.insts.get(g.0.as_str()).copied() {
                    let (iv, filled) = fit(iv, inst.tparams);
                    self.unrecorded += filled as usize;
                    if self.inst_copies.entry(g.0.clone()).or_default().insert(iv.clone()) {
                        self.work.push(Item::Inst(g.0.clone(), iv));
                    }
                }
            }
            Core::DictProj(_, i, Some(site)) if !site.own.is_empty() => {
                let (o, filled) = fit(&site.own, site.own.len());
                self.unrecorded += filled as usize;
                let key = (site.class.clone(), *i);
                if self.own.entry(key).or_default().insert(o.clone()) {
                    let dicts: Vec<(String, Vector)> =
                        self.by_class.get(&site.class).cloned().unwrap_or_default().into_iter().collect();
                    for (id, iv) in dicts {
                        let inst = self.insts[id.as_str()];
                        if inst.own.get(*i).copied().unwrap_or(0) > 0 {
                            let mut v = iv.clone();
                            v.extend(o.iter().copied());
                            self.reach_method(&inst.methods[*i], v);
                        }
                    }
                }
            }
            Core::Quote { body: Some(b), .. } => self.scan(b),
            // `child_slots` leaves a spine's head out (the optimizer
            // treats it apart), and the head is where most calls are.
            Core::App(f, _) => self.scan(f),
            _ => {}
        }
        for k in child_slots_imm(e) {
            self.scan(k);
        }
    }

    fn drain(&mut self) {
        while let Some(item) = self.work.pop() {
            match item {
                Item::Unit(id, v) => {
                    let mut body = self.fns[id.as_str()].body.clone();
                    subst_classes(&mut body, &v);
                    self.scan(&body);
                }
                Item::Inst(id, iv) => {
                    let inst = self.insts[id.as_str()];
                    self.by_class.entry(inst.class.clone()).or_default().insert((id.clone(), iv.clone()));
                    for i in 0..inst.methods.len() {
                        for v in self.method_vectors(inst, i, &iv) {
                            self.reach_method(&inst.methods[i], v);
                        }
                    }
                }
            }
        }
    }

    /// A method no projection reached at any own vector still needs a
    /// unit in its declared slot: the all-`Obj` copy. Reaching it may
    /// reach more, so this runs to a fixpoint.
    fn fill_defaults(&mut self) {
        loop {
            let mut pending: Vec<(GlobalId, Vector)> = Vec::new();
            for (id, ivs) in &self.inst_copies {
                let inst = self.insts[id.as_str()];
                for (i, k) in inst.own.iter().enumerate() {
                    let reached = self.own.get(&(inst.class.clone(), i)).map(|s| !s.is_empty()).unwrap_or(false);
                    if *k == 0 || reached {
                        continue;
                    }
                    for iv in ivs {
                        let mut v = iv.clone();
                        v.extend(std::iter::repeat(Class::Obj).take(*k));
                        pending.push((inst.methods[i].clone(), v));
                    }
                }
            }
            let before = self.work.len();
            for (m, v) in pending {
                self.reach_method(&m, v);
            }
            if self.work.len() == before {
                return;
            }
            self.drain();
        }
    }

    /// The own vector a method slot holds when it is the declared slot.
    fn declared_own(&self, inst: &InstanceDef, i: usize) -> Vector {
        let k = inst.own.get(i).copied().unwrap_or(0);
        match self.own.get(&(inst.class.clone(), i)).and_then(|s| s.iter().next()) {
            Some(o) if k > 0 => o.clone(),
            _ => vec![Class::Obj; k],
        }
    }

    /// The appended slots of a class, in order: (method, own vector).
    fn extras(&self, class: &str) -> Vec<(usize, Vector)> {
        let mut out = Vec::new();
        for ((c, i), set) in &self.own {
            if c == class {
                out.extend(set.iter().skip(1).map(|o| (*i, o.clone())));
            }
        }
        out
    }

    fn method_name(&self, m: &GlobalId, v: &[Class]) -> GlobalId {
        match self.generic(&m.0) {
            Some(f) => GlobalId(spec_name(&m.0, &fit(v, f.repr.tparams).0)),
            None => m.clone(),
        }
    }

    /// Rename every reference to the copy it reaches.
    fn rename(&self, e: &mut Core, slots: &HashMap<String, (usize, Vec<(usize, Vector)>)>) {
        match e {
            Core::Global(g, v) | Core::Lam { fn_id: g, classes: v, .. } | Core::Ctor(g, v) => {
                if let Some(f) = self.generic(&g.0).or_else(|| self.ctor_generic(&g.0)) {
                    *v = fit(v, f.repr.tparams).0;
                    *g = GlobalId(spec_name(&g.0, v));
                }
            }
            Core::InstDict(g, _, iv) => {
                if let Some(inst) = self.insts.get(g.0.as_str()) {
                    *iv = fit(iv, inst.tparams).0;
                    *g = GlobalId(inst_name(&g.0, iv));
                }
            }
            Core::DictProj(_, i, site) => {
                if let Some(s) = site.take() {
                    if !s.own.is_empty() {
                        let o = fit(&s.own, s.own.len()).0;
                        let first = self.own.get(&(s.class.clone(), *i)).and_then(|set| set.iter().next());
                        if first != Some(&o) {
                            if let Some((mc, extras)) = slots.get(&s.class) {
                                if let Some(p) = extras.iter().position(|(m, v)| m == i && *v == o) {
                                    *i = mc + p;
                                }
                            }
                        }
                    }
                }
            }
            Core::Quote { body: Some(b), .. } => self.rename(b, slots),
            Core::App(f, _) => self.rename(f, slots),
            _ => {}
        }
        for k in child_slots(e) {
            self.rename(k, slots);
        }
    }
}

/// A dictionary copy's construction unit: the instance itself when it
/// has no parameters of its own.
fn inst_name(id: &str, iv: &[Class]) -> String {
    if iv.is_empty() {
        id.to_string()
    } else {
        spec_name(id, iv)
    }
}

/// Collect and emit the reached copies; `report` records them.
pub fn specialize(program: &Program, report: &mut OptReport) -> Program {
    let mut cx = Cx {
        fns: program.fns.iter().map(|f| (f.id.0.as_str(), f)).collect(),
        insts: program.instances.iter().map(|i| (i.id.0.as_str(), i)).collect(),
        pinned: program.foreigns.values().map(|p| p.unit.as_str()).collect(),
        units: BTreeMap::new(),
        inst_copies: BTreeMap::new(),
        by_class: BTreeMap::new(),
        own: BTreeMap::new(),
        ctors: BTreeMap::new(),
        work: Vec::new(),
        unrecorded: 0,
    };
    for f in &program.fns {
        if cx.generic(&f.id.0).is_none() {
            cx.scan(&f.body);
        }
    }
    cx.drain();
    cx.fill_defaults();

    let mut slots: HashMap<String, (usize, Vec<(usize, Vector)>)> = HashMap::new();
    for (class, dicts) in &cx.by_class {
        if let Some((id, _)) = dicts.iter().next() {
            slots.insert(class.clone(), (cx.insts[id.as_str()].method_count, cx.extras(class)));
        }
    }

    let mut out = program.clone();
    out.fns.clear();
    let empty = BTreeSet::new();
    for f in &program.fns {
        if cx.generic(&f.id.0).is_some() {
            for v in cx.units.get(&f.id.0).unwrap_or(&empty) {
                let mut copy = f.clone();
                copy.id = GlobalId(spec_name(&f.id.0, v));
                subst_classes(&mut copy.body, v);
                cx.rename(&mut copy.body, &slots);
                copy.repr.params = crate::repr::subst_all(&copy.repr.params, v);
                copy.repr.tparams = 0;
                out.fns.push(copy);
                report.versions.push(spec_name(&f.id.0, v));
            }
        } else if cx.ctor_generic(&f.id.0).is_some() {
            // A constructor unit has no body to rename: its copies differ
            // in their field classes alone.
            for v in cx.ctors.get(&f.id.0).unwrap_or(&empty) {
                let mut copy = f.clone();
                copy.id = GlobalId(spec_name(&f.id.0, v));
                copy.repr.params = crate::repr::subst_all(&copy.repr.params, v);
                copy.repr.fields = crate::repr::subst_all(&copy.repr.fields, v);
                copy.repr.tparams = 0;
                out.fns.push(copy);
                report.versions.push(spec_name(&f.id.0, v));
            }
        } else {
            let mut kept = f.clone();
            cx.rename(&mut kept.body, &slots);
            out.fns.push(kept);
        }
    }

    out.instances.clear();
    for inst in &program.instances {
        let Some(ivs) = cx.inst_copies.get(&inst.id.0) else {
            // An unreached dictionary whose methods are all monomorphic
            // roots stays emitted as it was; any other would name generic
            // bodies that are no longer emitted.
            if inst.tparams == 0 && inst.own.iter().all(|k| *k == 0) {
                out.instances.push(inst.clone());
            }
            continue;
        };
        for iv in ivs {
            let mut methods: Vec<GlobalId> = (0..inst.methods.len())
                .map(|i| {
                    let mut v = iv.clone();
                    v.extend(cx.declared_own(inst, i));
                    cx.method_name(&inst.methods[i], &v)
                })
                .collect();
            if let Some((_, extras)) = slots.get(&inst.class) {
                for (i, o) in extras {
                    let mut v = iv.clone();
                    v.extend(o.iter().copied());
                    methods.push(cx.method_name(&inst.methods[*i], &v));
                }
            }
            let mut copy = inst.clone();
            copy.id = GlobalId(inst_name(&inst.id.0, iv));
            copy.method_count = methods.len();
            copy.methods = methods;
            if !iv.is_empty() {
                report.versions.push(copy.id.0.clone());
            }
            out.instances.push(copy);
        }
    }
    report.versions.sort();
    report.unrecorded = cx.unrecorded;
    out
}

/// The driver's optimization step: driving and rewriting
/// (`crate::core::optimize`), then the collector over the residual.
pub fn optimize(
    program: &Program,
    profile: &crate::core::TargetProfile,
) -> crate::Result<(Program, OptReport)> {
    let (residual, mut report) = crate::core::optimize(program, profile)?;
    let out = specialize(&residual, &mut report);
    Ok((out, report))
}

//! Stack allocation of non-escaping blocks (design/stack-allocation.md).
//!
//! The count pass leaves a program in which every block is heap allocated
//! and every reference is an explicit `Dup`, `Drop` or `Release`. This pass
//! runs over that program and decides, per unit, which allocations can live
//! in the unit's own frame instead: a block whose every reference is
//! consumed inside the allocating activation needs no heap, no header
//! count, no free and no collector visit.
//!
//! # The rule
//!
//! Only a `let`-bound allocation is a candidate: a saturated constructor
//! call (`(app (ctor C) args)` with the constructor's whole arity) or a
//! closure (`(lam ...)`). The value's binder is the `let`'s binder, which
//! the count pass already treats as one owned reference. The allocation is
//! frame-eligible when:
//!
//! 1. every reference to the binder is either the scrutinee of a `case`
//!    (a borrowed read) or a `Drop`/`Release` that ends it. Anything else
//!    escapes the activation: a returned value, a constructor field, a
//!    call or `spawn` argument, a jump argument, a projection's container,
//!    a closure environment slot, or a `RefSet`/`BoxInit`/`LazyInit`
//!    store;
//! 2. the binder is `Dup`ped only when its payload holds no counted word. A
//!    second reference would need a second release of the payload, and a
//!    frame block has no count to make one of them a no-op; with nothing
//!    counted in the payload there is nothing for either release to do, so
//!    the two references cannot disagree;
//! 3. the constructor's layout is a regular one (a `repr(C)`,
//!    `repr(packed)` or `repr(transparent)` block stores its fields at
//!    recorded offsets, which this lowering does not reproduce);
//! 4. its payload is at most [`DEFAULT_BLOCK_LIMIT`] bytes and the unit's
//!    slots together at most the caller's frame limit
//!    ([`DEFAULT_FRAME_LIMIT`] by default);
//! 5. the binder's class is counted (`Obj`/`TObj`), because an uncounted
//!    binder has no `Drop` for the payload release to hang on.
//!
//! Rule 1 is what the ownership lattice gives us for free: after
//! `crate::rc::insert` every owned binder is consumed exactly once on every
//! path, so "all references are a case scrutinee or a death" is also "the
//! reference never leaves the activation". No separate escape analysis over
//! callees is needed, and a value stored into any block at all is refused
//! (the pass does not yet know whether that block is itself frame-bound).
//!
//! # What the lowering relies on
//!
//! A frame block keeps an ordinary header with the count
//! [`crate::codegen`]'s shared `block_kinds.rs` calls `RC_FRAME`, so a
//! `case` still reads its tag and a stale count operation is a no-op. The
//! payload is released by the code generator at the binder's death, in
//! reverse payload order, with an ordinary `Drop` per counted field: the
//! block itself is not freed, and its payload fields are ordinary heap
//! references. A frame block is therefore never a payload field of another
//! frame block (rule 1 refuses the store), which is what keeps every
//! frame block's release a flat list of classes.
//!
//! # Inspectability
//!
//! [`allocate`] fills [`crate::core::Program::frame`] (the decision, by
//! unit) and [`crate::core::Program::stack`] (slot and refusal counts for
//! the artifact manifest). `--dump-core` prints `@frame(slots, bytes)` on a
//! unit and marks each frame allocation's node with `@frame(f<slot>)`.

use crate::core::{Core, FnDef, GlobalId, Program};
use crate::repr::Class;
use std::collections::{BTreeMap, HashMap};

/// The largest payload a frame block may have: a very large object
/// belongs on the heap, where the frame is not grown for every call.
pub const DEFAULT_BLOCK_LIMIT: u64 = 4096;

/// The most frame bytes one unit may reserve before the rest of its
/// allocations stay on the heap.
pub const DEFAULT_FRAME_LIMIT: u64 = 16 * 1024;

/// The pass's knobs: `--no-stack-alloc` and `--stack-frame-limit`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    /// Whether the pass runs at all. With it off the program keeps every
    /// allocation on the heap and the emitted IR is the one the compiler
    /// produced before this pass existed.
    pub enabled: bool,
    /// The most frame bytes one unit may reserve (`--stack-frame-limit`).
    pub frame_limit: u64,
}

impl Default for Options {
    fn default() -> Options {
        Options { enabled: true, frame_limit: DEFAULT_FRAME_LIMIT }
    }
}

/// What a frame slot holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    /// A saturated constructor call of `ctor`.
    Ctor { ctor: GlobalId, arity: u32 },
    /// A closure over `fn_id` with `env` saved words.
    Closure { fn_id: GlobalId, env: u32 },
}

/// One allocation the pass lowered into the frame: the kind of block, the
/// slot it uses, and every counted payload word with the byte offset the
/// code generator releases it at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Site {
    /// The slot's index in [`Plan::slots`].
    pub slot: u32,
    /// Payload words (the block's own size, header aside).
    pub words: u32,
    pub shape: Shape,
    /// The counted payload words, in payload order: `(byte offset, class)`.
    /// The code generator loads and drops each of them, in reverse, when
    /// the reference dies.
    pub fields: Vec<(u32, Class)>,
}

impl Site {
    /// The bytes the slot reserves: the header, the payload, and rounding
    /// to a 16-byte stack alignment.
    pub fn bytes(&self) -> u64 {
        (((self.words as u64) + 1) * 8).div_ceil(16) * 16
    }
}

/// One unit's frame allocation decision.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Plan {
    /// The frame allocation of each binder, by local id: the binder of the
    /// `let` whose value the allocation is.
    pub sites: BTreeMap<usize, Site>,
    /// The payload words of each slot, by slot id: the entry block's
    /// allocas.
    pub slots: Vec<u32>,
    /// The bytes the unit's frame reserves, alignment included.
    pub bytes: u64,
}

impl Plan {
    /// The slot of the binder `x`, when `x` names a frame allocation.
    pub fn site(&self, x: usize) -> Option<&Site> {
        self.sites.get(&x)
    }

    /// Whether the unit reserves no frame slot at all.
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// The slot count, as the Core dump and the manifest report it.
    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }
}

/// Why a candidate allocation stayed on the heap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Refusal {
    /// A boundary-layout (`repr(C)`, `repr(packed)`, `repr(transparent)`)
    /// constructor, whose fields do not sit at their payload index.
    Layout,
    /// A `let`-bound constructor call the emitter would not lower as one
    /// block (an unknown unit, a boundary offset, or a closure over a unit
    /// that does not exist).
    Form,
    /// Above [`DEFAULT_BLOCK_LIMIT`] bytes.
    Size,
    /// The binder is `Dup`ped and its payload holds a counted word: a
    /// second reference needs a second release of that word, which a frame
    /// block has no count to make a no-op.
    Duplicated,
    /// A reference leaves the activation (returned, stored, passed,
    /// captured, projected, or written through).
    Escapes,
    /// The unit's frame would pass its limit.
    FrameLimit,
}

impl Refusal {
    /// The name the manifest and the dump use.
    pub fn name(self) -> &'static str {
        match self {
            Refusal::Layout => "layout",
            Refusal::Form => "form",
            Refusal::Size => "size",
            Refusal::Duplicated => "duplicated",
            Refusal::Escapes => "escapes",
            Refusal::FrameLimit => "frame-limit",
        }
    }
}

/// The pass's totals over one program: what the artifact manifest reports.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Whether the pass ran (the profile reclaims and the caller enabled
    /// it).
    pub enabled: bool,
    /// The units that reserve at least one frame slot.
    pub units: u64,
    /// The frame slots of the whole program.
    pub slots: u64,
    /// The bytes those slots reserve, alignment included.
    pub bytes: u64,
    /// Every `let`-bound allocation the pass considered.
    pub candidates: u64,
    /// The candidates that stayed on the heap, by reason.
    pub refused: BTreeMap<&'static str, u64>,
}

impl Report {
    fn refuse(&mut self, why: Refusal) {
        *self.refused.entry(why.name()).or_insert(0) += 1;
    }
}

/// Decide every unit's frame slots and install the plans in `program`.
/// Disabled, or under a profile that does not reclaim, the program keeps
/// every allocation on the heap and the report says the pass ran nothing.
pub fn allocate(program: &mut Program, profile: &crate::core::TargetProfile, opts: &Options) {
    program.frame.clear();
    program.stack = Report::default();
    if !opts.enabled || !profile.reclaim {
        return;
    }
    let fns = Facts::of(program);
    let mut plans: BTreeMap<String, Plan> = BTreeMap::new();
    let mut report = Report { enabled: true, ..Report::default() };
    // The analysis only reads the units, so collect the plans and write
    // them back once.
    let decided: Vec<(String, Plan)> = program
        .fns
        .iter()
        .filter(|f| f.prim.is_none() && f.ctor.is_none() && !f.meta.prim_body)
        .filter_map(|f| {
            let plan = unit(f, &fns, opts, &mut report);
            (!plan.is_empty()).then(|| (f.id.0.clone(), plan))
        })
        .collect();
    for (unit, plan) in decided {
        report.units += 1;
        report.slots += plan.slot_count() as u64;
        report.bytes += plan.bytes;
        plans.insert(unit, plan);
    }
    program.stack = report;
    program.frame = plans;
    if cfg!(debug_assertions) {
        verify(program);
    }
}

/// The program facts the analysis reads: each unit's arity, and for the
/// constructor units their declared field classes (the same data the code
/// generator's shape map uses, so the two cannot disagree) and whether a
/// boundary layout governs the payload.
struct Facts {
    arity: HashMap<String, usize>,
    /// Constructor units' field classes, their arity, and whether a
    /// boundary layout plan governs the payload.
    ctors: HashMap<String, (Vec<Class>, usize, bool)>,
    /// Each unit's runtime parameter classes (`UnitRepr::params`).
    params: HashMap<String, Vec<Class>>,
    /// The borrow inference's modes (`Program::borrowed`): `true` where the
    /// caller only lends the argument.
    borrowed: BTreeMap<String, Vec<bool>>,
    /// The primitive units' borrowed parameters (`Program::lent`).
    lent: BTreeMap<String, Vec<bool>>,
}

impl Facts {
    fn of(p: &Program) -> Facts {
        let mut arity = HashMap::new();
        let mut ctors = HashMap::new();
        let mut params = HashMap::new();
        for f in &p.fns {
            arity.insert(f.id.0.clone(), f.arity());
            params.insert(f.id.0.clone(), f.repr.params.clone());
            if let Some((_, fields)) = f.ctor {
                let mut cs = f.repr.fields.clone();
                if cs.len() != fields {
                    // A constructor the lowering gave no classes: never
                    // counted, exactly as the shape map reads it.
                    cs = vec![Class::Raw; fields];
                }
                ctors.insert(f.id.0.clone(), (cs, f.arity(), p.plans.contains_key(&f.id.0)));
            }
        }
        Facts {
            arity,
            ctors,
            params,
            borrowed: p.borrowed.clone(),
            lent: p.lent.clone(),
        }
    }
}

/// Whether a word of class `c` is counted (the class pass's rule).
fn counted(c: Class) -> bool {
    matches!(c, Class::Obj | Class::TObj)
}

/// One unit's plan.
fn unit(f: &FnDef, facts: &Facts, opts: &Options, report: &mut Report) -> Plan {
    let mut cands: BTreeMap<usize, Result<Site, Refusal>> = BTreeMap::new();
    candidates(&f.body, facts, &mut cands);
    let mut uses: BTreeMap<usize, Uses> = BTreeMap::new();
    scan(&f.body, Role::Value, facts, &mut uses);
    let trace = std::env::var_os("CINCH_STACK_TRACE").is_some();
    let mut plan = Plan::default();
    for (x, cand) in cands {
        report.candidates += 1;
        // The counted payload words, for the trace's column: what a dup or a
        // drop of the block has to act on, and so what the duplicated rule
        // turns on.
        let counted_fields = match &cand {
            Ok(site) => site.fields.len(),
            Err(_) => 0,
        };
        let why = match &cand {
            Ok(site) => match eligible(x, site, &uses, &plan, opts) {
                Ok(()) => {
                    let slot = plan.slots.len() as u32;
                    plan.bytes += site.bytes();
                    plan.slots.push(site.words);
                    plan.sites.insert(x, Site { slot, ..site.clone() });
                    if trace {
                        eprintln!(
                            "cinch-stack: {} local {x}: slot {slot}, {} bytes, {counted_fields} counted field(s)",
                            f.id,
                            site.bytes()
                        );
                    }
                    continue;
                }
                Err(why) => why,
            },
            Err(why) => *why,
        };
        report.refuse(why);
        if trace {
            let detail = uses.get(&x).map(|u| u.why).unwrap_or("");
            let dups = uses.get(&x).map_or(0, |u| u.duplicated);
            eprintln!(
                "cinch-stack: {} local {x}: refused ({}{detail}, {counted_fields} counted field(s), {dups} dup(s))",
                f.id,
                why.name()
            );
        }
    }
    plan
}

/// Why a candidate cannot take a frame slot: the rule's checks in the
/// order the report names them.
fn eligible(
    x: usize,
    site: &Site,
    uses: &BTreeMap<usize, Uses>,
    plan: &Plan,
    opts: &Options,
) -> Result<(), Refusal> {
    let _ = x;
    let u = uses.get(&x).copied().unwrap_or_default();
    // A second reference is harmless when the payload holds nothing to
    // release: a frame block's dup writes no count (the `RC_FRAME` header
    // is immortal) and its drop frees nothing, so two references cannot
    // disagree about a payload that has no owned word in it.
    if u.duplicated > 0 && !site.fields.is_empty() {
        return Err(Refusal::Duplicated);
    }
    if u.escapes > 0 {
        return Err(Refusal::Escapes);
    }
    if u.death == 0 {
        // No reference's death: the binder is dead on every path, but
        // nothing releases its payload. Refuse rather than leak.
        return Err(Refusal::Escapes);
    }
    if plan.bytes + site.bytes() > opts.frame_limit {
        return Err(Refusal::FrameLimit);
    }
    Ok(())
}

/// Every `let`-bound allocation in `e` that the pass could lower, keyed by
/// the binder, with the site or the reason it cannot be one.
fn candidates(e: &Core, facts: &Facts, out: &mut BTreeMap<usize, Result<Site, Refusal>>) {
    match e {
        Core::Let(x, c, v, body) => {
            if counted(*c) {
                if let Some(site) = site_of(v, facts) {
                    out.insert(x.0, site);
                }
            }
            candidates(v, facts, out);
            candidates(body, facts, out);
            return;
        }
        // A quotation's retained body is a code unit of its own, emitted as
        // its own function: it has its own frame, and its local ids are its
        // own.
        Core::Quote { holes, .. } => {
            for h in holes {
                candidates(h, facts, out);
            }
            return;
        }
        // A spine's head is not a child slot.
        Core::App(h, args) => {
            candidates(h, facts, out);
            for a in args {
                candidates(a, facts, out);
            }
            return;
        }
        _ => {}
    }
    for k in crate::core::child_slots_imm(e) {
        candidates(k, facts, out);
    }
}

/// The frame site of an allocation expression, or the reason it cannot be
/// one. `None` means the expression is not an allocation this pass handles
/// at all (a dictionary, a memo cell, a knot, a boxed number, a string).
fn site_of(e: &Core, facts: &Facts) -> Option<Result<Site, Refusal>> {
    match e {
        Core::App(h, args) => {
            let Core::Ctor(g, _) = &**h else { return None };
            let Some((fields, arity, boundary)) = facts.ctors.get(&g.0) else {
                return Some(Err(Refusal::Form));
            };
            // A partial application builds a closure in the apply stub, and
            // a nullary constructor is a static block: neither allocates
            // this constructor's block here.
            if args.len() != *arity || args.len() != fields.len() || args.is_empty() {
                return None;
            }
            if *boundary {
                return Some(Err(Refusal::Layout));
            }
            let words = args.len() as u32;
            Some(payload(words, 0, fields, Shape::Ctor { ctor: g.clone(), arity: words }))
        }
        Core::Lam { fn_id, env, .. } => {
            let params = facts.params.get(&fn_id.0)?;
            if env.len() > *facts.arity.get(&fn_id.0).unwrap_or(&0) {
                return Some(Err(Refusal::Form));
            }
            // The closure's first three payload words are its code, arity
            // and filled count, all uncounted; the saved words follow.
            let words = 3 + env.len() as u32;
            let saved = &params[..env.len().min(params.len())];
            Some(payload(words, 3, saved, Shape::Closure { fn_id: fn_id.clone(), env: env.len() as u32 }))
        }
        _ => None,
    }
}

/// The site of a block of `words` payload words whose counted payload
/// words start at word `base` and have the classes in `classes`.
fn payload(words: u32, base: u32, classes: &[Class], shape: Shape) -> Result<Site, Refusal> {
    if u64::from(words) * 8 > DEFAULT_BLOCK_LIMIT {
        return Err(Refusal::Size);
    }
    let fields = classes
        .iter()
        .enumerate()
        .filter(|(_, c)| counted(**c))
        .map(|(i, c)| ((base + i as u32) * 8, *c))
        .collect();
    Ok(Site { slot: 0, words, shape, fields })
}

/// How a binder's references are used, accumulated over the whole unit.
#[derive(Clone, Copy, Debug, Default)]
struct Uses {
    /// References that end the value (`Drop`, `Release`).
    death: u32,
    /// References read without consuming (a `case` scrutinee, or a
    /// projection's container reading one field).
    scrutinee: u32,
    /// References lent to a borrowed parameter: the callee only reads them.
    lent: u32,
    /// `Dup` operations: a second reference exists.
    duplicated: u32,
    /// References that leave the activation or are stored inside a block.
    escapes: u32,
    /// The first escape's role, for `CINCH_STACK_TRACE`.
    why: &'static str,
}

/// Where a subexpression's value goes, so a binder's reference can be
/// classified: the ownership lattice's positions, spelled out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    /// A value position of the ordinary kind (a unit's result, a `let`
    /// value, an argument whose callee the pass cannot see).
    Value,
    /// A field of the constructor block `(app (ctor C) ... args)`.
    Field,
    /// An argument of a call, or of a join's jump.
    Arg,
    /// The callee of an application: the closure is consumed by the call.
    Head,
    /// A closure's saved word.
    Capture,
    /// A `case` scrutinee.
    Scrutinee,
    /// The container of a projection or a memo demand: one field is read.
    Project,
    /// A value stored through a cell (`RefSet`, `BoxInit`, `LazyInit`).
    Store,
    /// A cell a value is written through.
    Write,
}

/// Accumulate every local's reference uses in `e`. `role` is the position
/// `e`'s own value sits in, which is what makes a bare `Var` readable as a
/// scrutinee, a projection's container or a lent argument instead of an
/// escape; a compound subexpression is scanned as an ordinary value, since
/// the role describes the result, not the parts.
fn scan(e: &Core, role: Role, facts: &Facts, out: &mut BTreeMap<usize, Uses>) {
    match e {
        Core::Var(x) => note(out, x.0, role),
        Core::Dup(x, ..) => uses_of(out, x.0).duplicated += 1,
        Core::Drop(x, ..) => uses_of(out, x.0).death += 1,
        Core::Release { block, moved, rest, body } => {
            uses_of(out, block.0).death += 1;
            // A `Release` field is a pattern binder; if a `let` binder ever
            // names one, it was stored in a block: not this pass's value.
            for (x, _) in moved.iter().chain(rest) {
                note(out, x.0, Role::Store);
            }
            scan(body, role, facts, out);
            return;
        }
        Core::Case { scrut, alts } => {
            scan(scrut, Role::Scrutinee, facts, out);
            for a in alts {
                // An alternative's body is the case's value.
                scan(&a.body, role, facts, out);
            }
            return;
        }
        // A projection reads one field and hands out an owned reference to
        // it; the block keeps its own. A memo cell or a knot is written
        // rather than read (`Demand`, `BoxGet`, `BoxClear`), and no frame
        // block is one: those stay escapes.
        Core::Proj(d, _) | Core::DictProj(d, ..) => {
            scan(d, Role::Project, facts, out);
        }
        Core::Demand(d, _) | Core::BoxGet(d, _) | Core::BoxClear(d, _) => {
            scan(d, Role::Value, facts, out);
        }
        Core::RefSet(x, v, _) | Core::BoxInit(x, v, _) | Core::LazyInit(x, v) => {
            note(out, x.0, Role::Write);
            scan(v, Role::Store, facts, out);
        }
        Core::App(h, args) => {
            let ctor = matches!(&**h, Core::Ctor(..));
            let modes = callee_modes(facts, h).filter(|m| m.len() == args.len());
            scan(h, Role::Head, facts, out);
            for (i, a) in args.iter().enumerate() {
                if ctor {
                    scan(a, Role::Field, facts, out);
                    continue;
                }
                if modes.is_some_and(|m| m[i]) {
                    // A borrowed parameter is only read by the callee: the
                    // reference stays the caller's own, which is where its
                    // release is.
                    match a {
                        Core::Var(x) => borrow_note(out, x.0),
                        other => scan(other, Role::Arg, facts, out),
                    }
                    continue;
                }
                scan(a, Role::Arg, facts, out);
            }
            return;
        }
        Core::InstDict(_, args, _) => {
            for a in args {
                scan(a, Role::Arg, facts, out);
            }
            return;
        }
        Core::Lam { env, .. } => {
            for a in env {
                scan(a, Role::Capture, facts, out);
            }
            return;
        }
        Core::Jump { args, .. } => {
            for a in args {
                scan(a, Role::Arg, facts, out);
            }
            return;
        }
        // A quotation's retained body is a code unit of its own.
        Core::Quote { holes, .. } => {
            for h in holes {
                scan(h, Role::Value, facts, out);
            }
            return;
        }
        _ => {}
    }
    // A part of a compound value is an ordinary value position: the role
    // belongs to the whole.
    for k in crate::core::child_slots_imm(e) {
        scan(k, Role::Value, facts, out);
    }
}

/// Record one reference of local `x` in `role`.
fn note(out: &mut BTreeMap<usize, Uses>, x: usize, role: Role) {
    if matches!(role, Role::Scrutinee | Role::Project) {
        // A read: the block stays the binder's own.
        uses_of(out, x).scrutinee += 1;
        return;
    }
    let u = uses_of(out, x);
    u.escapes += 1;
    if u.why.is_empty() {
        u.why = role_name(role);
    }
}

/// A borrowed argument of a call: the callee reads it and does not own it.
fn borrow_note(out: &mut BTreeMap<usize, Uses>, x: usize) {
    uses_of(out, x).lent += 1;
}

/// The trace's name for a role an escape sits in.
fn role_name(role: Role) -> &'static str {
    match role {
        Role::Value => "value",
        Role::Field => "field",
        Role::Arg => "call",
        Role::Head => "head",
        Role::Capture => "capture",
        Role::Scrutinee => "scrutinee",
        Role::Project => "project",
        Role::Store => "store",
        Role::Write => "write",
    }
}

/// The parameter modes of a saturated call's head, when the program's
/// borrow inference gave it any (`true` borrowed).
fn callee_modes<'f>(facts: &'f Facts, head: &Core) -> Option<&'f Vec<bool>> {
    match head {
        Core::Global(g, _) => facts.borrowed.get(&g.0),
        Core::Prim(op) => facts.lent.get(&op.symbol()),
        _ => None,
    }
}

/// Re-derive every frame decision in debug builds and refuse a plan that
/// breaks the rule: a compiler bug at compile time (design/no-faults.md).
fn verify(program: &Program) {
    for (unit, plan) in &program.frame {
        let Some(f) = program.fns.iter().find(|f| f.id.0 == *unit) else { continue };
        let mut uses: BTreeMap<usize, Uses> = BTreeMap::new();
        let facts = Facts::of(program);
        scan(&f.body, Role::Value, &facts, &mut uses);
        for (x, site) in &plan.sites {
            let u = uses.get(x).copied().unwrap_or_default();
            // A second reference is admissible exactly when the payload has
            // no counted word to release twice (the rule `eligible` states).
            assert!(
                (u.duplicated == 0 || site.fields.is_empty()) && u.escapes == 0,
                "compiler bug: {unit}: frame binder {x} escapes ({} dups, {} escapes, {} counted fields)",
                u.duplicated,
                u.escapes,
                site.fields.len()
            );
            assert!(u.death > 0, "compiler bug: {unit}: frame binder {x} has no release");
        }
    }
}

/// The use record of local `x`.
fn uses_of(out: &mut BTreeMap<usize, Uses>, x: usize) -> &mut Uses {
    out.entry(x).or_default()
}

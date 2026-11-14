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
//! 1. every reference to the binder is one of the four a frame block
//!    admits: the scrutinee of a `case` (a borrowed read), a `Drop` or
//!    `Release` that ends it, a pass to a callee's owned parameter the
//!    callee consumes, or a store into a field of a block that is itself
//!    frame-bound. Anything else escapes the activation: a returned value,
//!    a field of a heap block, a call argument whose callee may keep it, an
//!    argument of a call in tail position, a `spawn` argument, a jump
//!    argument, a projection's container, a closure environment slot, or a
//!    `RefSet`/`BoxInit`/`LazyInit` store;
//! 2. the binder is `Dup`ped only when its payload holds no counted word. A
//!    second reference would outlive the payload's release, and a frame
//!    block has no count to make one reference's drop a no-op; with
//!    nothing counted in the payload there is nothing for either release to
//!    do, so the two references cannot disagree;
//! 3. the constructor's layout is a regular one (a `repr(C)`,
//!    `repr(packed)` or `repr(transparent)` block stores its fields at
//!    recorded offsets, which this lowering does not reproduce);
//! 4. its payload is at most [`DEFAULT_BLOCK_LIMIT`] bytes and the unit's
//!    slots together at most the caller's frame limit
//!    ([`DEFAULT_FRAME_LIMIT`] by default);
//! 5. the binder's class is counted (`Obj`/`TObj`), because an uncounted
//!    binder has no `Drop` for the payload release to hang on.
//!
//! Tail position is excluded from every transfer, and that is not a
//! conservative nicety. Code generation emits a saturated call in tail
//! position as a `musttail` call when the callee's signature matches the
//! caller's (mutual tail recursion) and as a jump to the loop header when
//! the callee is the unit itself ([`crate::codegen`], decision D104). Both
//! forms hand the callee *this* frame: the callee's spills and its own
//! frame slots occupy the caller's slot memory. An argument that points
//! into the caller's frame is therefore overwritten by the callee as soon
//! as it runs, before it can read or release the block, which is a slot
//! whose header the callee has already rewritten. A call in any other
//! position gets a frame of its own, below the caller's, and is safe.
//!
//! Rule 1 is what the ownership lattice gives us for free: after
//! `crate::rc::insert` every owned binder is consumed exactly once on every
//! path, so "every reference is a read or one of the transfers above" is
//! also "the reference never leaves the activation". The two transfers that
//! are not a read widen the rule to a callee's frame and to a nested frame
//! block, and each needs one fact about a place the pass cannot see:
//!
//! - A pass to a callee's owned parameter is admissible only when the
//!   callee consumes the parameter ([`Facts::consumed`]: every reference to
//!   it there is a read or a death, and nothing stores it, returns it or
//!   passes it on). The callee then releases the payload exactly once, on
//!   the same path its own drop takes. A parameter the callee forwards to
//!   another unit is not consumed by this rule, so forwarding is refused
//!   (one pass over the program, no fixpoint over summaries).
//! - A store into a field of another frame-bound block is admissible only
//!   when that block is itself frame-bound, which means the decision is a
//!   fixpoint: each round's frame binders make the stores into them
//!   admissible in the next. The result grows monotonically, so it
//!   terminates, and the frame limit is applied as a final shrink so it
//!   cannot oscillate ([`decide`]).
//!
//! # What the lowering relies on
//!
//! A frame block keeps an ordinary header with the count
//! [`crate::codegen`]'s shared `block_kinds.rs` calls `RC_FRAME`, so a
//! `case` still reads its tag and a stale count operation is a no-op. The
//! payload is released exactly once (design/stack-allocation.md
//! section 7.3): the code generator drops the counted fields at the
//! binder's own death, in reverse payload order, and the runtime's
//! `cinch_frame_release` walks the block's map when a drop arrives from a
//! place the unit cannot see (a callee's owned parameter, or a parent
//! frame block's release). Either way the header is marked released first,
//! so a second path into the same block does nothing: that marker is what
//! lets a frame block be a payload field of another frame block and what
//! makes a callee's drop of a frame argument safe.
//!
//! # Inspectability
//!
//! [`allocate`] fills [`crate::core::Program::frame`] (the decision, by
//! unit) and [`crate::core::Program::stack`] (slot and refusal counts for
//! the artifact manifest). `--dump-core` prints `@frame(slots, bytes)` on a
//! unit and marks each frame allocation's node with `@frame(f<slot>)`.

use crate::core::{Core, FnDef, GlobalId, LocalId, Program};
use crate::repr::Class;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// The largest payload a frame block may have: a very large object
/// belongs on the heap, where the frame is not grown for every call.
pub const DEFAULT_BLOCK_LIMIT: u64 = 4096;

/// The most frame bytes one unit may reserve before the rest of its
/// allocations stay on the heap.
pub const DEFAULT_FRAME_LIMIT: u64 = 16 * 1024;

/// The pass's knobs: `--no-stack-alloc`, `--stack-frame-limit`, and the
/// measurement switch for the rule's two extra transfers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    /// Whether the pass runs at all. With it off the program keeps every
    /// allocation on the heap and the emitted IR is the one the compiler
    /// produced before this pass existed.
    pub enabled: bool,
    /// The most frame bytes one unit may reserve (`--stack-frame-limit`).
    pub frame_limit: u64,
    /// Which transfers beyond a read or a death the rule admits
    /// (`CINCH_STACK_WIDEN`): the switch that measures one widening against
    /// the other.
    pub widen: Widen,
}

impl Default for Options {
    fn default() -> Options {
        Options { enabled: true, frame_limit: DEFAULT_FRAME_LIMIT, widen: Widen::All }
    }
}

/// The two transfers [`Options::widen`] selects, named for the environment
/// variable that sets them (`CINCH_STACK_WIDEN`): `all` (default),
/// `consume` (a callee's consumed owned parameter), `store` (a store into
/// a frame-bound block), `none` (neither, the rule before
/// design/stack-allocation.md section 7.3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Widen {
    #[default]
    All,
    Consume,
    Store,
    None,
}

impl Widen {
    /// The switch's value, from `CINCH_STACK_WIDEN` (anything else is
    /// `All`, so an unset variable is the shipped rule).
    pub fn from_env() -> Widen {
        match std::env::var("CINCH_STACK_WIDEN").ok().as_deref() {
            Some("none") => Widen::None,
            Some("consume") => Widen::Consume,
            Some("store") => Widen::Store,
            _ => Widen::All,
        }
    }

    /// Whether a pass to a callee's consumed owned parameter is admissible.
    fn consumes(self) -> bool {
        matches!(self, Widen::All | Widen::Consume)
    }

    /// Whether a store into a frame-bound block is admissible.
    fn stores(self) -> bool {
        matches!(self, Widen::All | Widen::Store)
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
    /// The units that were handed a frame block as an owned argument: each
    /// callee whose parameter the pass let a frame allocation flow into.
    /// Code generation gives such a unit the runtime's frame arms even
    /// when it builds no frame block of its own (`codegen::Registry`).
    pub receives_frame: std::collections::BTreeSet<String>,
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
    let fns = Facts::of(program, opts.widen);
    if opts.widen.consumes() {
        for f in program.fns.iter_mut() {
            if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
                continue;
            }
            let mut next = crate::core::max_local(&f.body).max(f.arity()) + 1;
            name_closure_args(&mut f.body, &fns, true, &mut next);
        }
    }
    let mut plans: BTreeMap<String, Plan> = BTreeMap::new();
    let mut report = Report { enabled: true, ..Report::default() };
    // The analysis only reads the units, so collect the plans and write
    // them back once.
    let decided: Vec<(String, Plan, BTreeSet<String>)> = program
        .fns
        .iter()
        .filter(|f| f.prim.is_none() && f.ctor.is_none() && !f.meta.prim_body)
        .map(|f| {
            let (plan, receivers) = unit(f, &fns, opts, &mut report);
            (f.id.0.clone(), plan, receivers)
        })
        .collect();
    for (unit, plan, receivers) in decided {
        report.receives_frame.extend(receivers);
        if plan.is_empty() {
            continue;
        }
        report.units += 1;
        report.slots += plan.slot_count() as u64;
        report.bytes += plan.bytes;
        plans.insert(unit, plan);
    }
    program.stack = report;
    program.frame = plans;
    if cfg!(debug_assertions) {
        verify(program, opts);
    }
}

/// Name the closures a non-tail call hands to a parameter its callee
/// consumes (section 7.9): `f (lam ...)` becomes `let k = lam ... in f k`, so
/// the closure is a `let`-bound allocation the frame rule can consider. Only
/// a closure whose saved words are locals, literals or dups of a local is
/// named: building it first then performs nothing but count increments, so
/// moving it ahead of its sibling arguments changes no observable order.
/// `tail` follows the same forms `Scan::go` threads it through; a call in
/// tail position is left alone, since its arguments never take a slot.
fn name_closure_args(e: &mut Core, facts: &Facts, tail: bool, next: &mut usize) {
    fn simple(e: &Core) -> bool {
        match e {
            Core::Var(_) | Core::Int(_) | Core::Bool(_) | Core::Unit => true,
            Core::Dup(_, _, body) => simple(body),
            _ => false,
        }
    }
    match e {
        Core::Let(_, _, v, body) => {
            name_closure_args(v, facts, false, next);
            name_closure_args(body, facts, tail, next);
        }
        Core::Dup(_, _, body) | Core::Drop(_, _, body) | Core::Release { body, .. } => name_closure_args(body, facts, tail, next),
        Core::If(c, t, f) => {
            name_closure_args(c, facts, false, next);
            name_closure_args(&mut t.body, facts, tail, next);
            name_closure_args(&mut f.body, facts, tail, next);
        }
        Core::Case { scrut, alts } => {
            name_closure_args(scrut, facts, false, next);
            for a in alts {
                name_closure_args(&mut a.body, facts, tail, next);
            }
        }
        Core::Join { body, rest, .. } => {
            name_closure_args(body, facts, tail, next);
            name_closure_args(rest, facts, tail, next);
        }
        Core::Quote { holes, .. } => {
            for h in holes {
                name_closure_args(h, facts, false, next);
            }
        }
        Core::App(h, args) => {
            name_closure_args(h, facts, false, next);
            for a in args.iter_mut() {
                name_closure_args(a, facts, false, next);
            }
            if tail {
                return;
            }
            let Some(consumed) = callee_consumes(facts, h).filter(|c| c.len() == args.len()) else { return };
            let mut named: Vec<(LocalId, Core)> = Vec::new();
            for (i, a) in args.iter_mut().enumerate() {
                if consumed[i] && matches!(a, Core::Lam { env, .. } if !env.is_empty() && env.iter().all(simple)) {
                    let x = LocalId(*next);
                    *next += 1;
                    named.push((x, std::mem::replace(a, Core::Var(x))));
                }
            }
            if named.is_empty() {
                return;
            }
            let mut out = std::mem::replace(e, Core::Unit);
            for (x, lam) in named.into_iter().rev() {
                out = Core::Let(x, Class::Obj, Box::new(lam), Box::new(out));
            }
            *e = out;
        }
        _ => {
            for k in crate::core::child_slots(e) {
                name_closure_args(k, facts, false, next);
            }
        }
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
    /// Per unit, which of its parameters it consumes: every reference to
    /// the parameter inside the unit is a read or a death, none is
    /// `Dup`ped, and nothing stores it, returns it or passes it on. The
    /// caller may hand such a parameter a frame block, because the
    /// callee's own drop of it is the payload's release (the frame release
    /// marker makes a second path a no-op).
    consumed: HashMap<String, Vec<bool>>,
}

impl Facts {
    fn of(p: &Program, widen: Widen) -> Facts {
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
            // The parameter summaries are computed against the facts
            // without them: a unit consumes a parameter it drops itself,
            // not one it forwards, so the summary is one pass, not a
            // fixpoint over the call graph (module docs).
            consumed: HashMap::new(),
        }
        .with_summaries(p, widen)
    }

    /// [`Facts::of`]'s second step: fill [`Facts::consumed`] from every
    /// unit's body.
    fn with_summaries(mut self, p: &Program, widen: Widen) -> Facts {
        let mut consumed = HashMap::with_capacity(p.fns.len());
        for f in &p.fns {
            if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
                // A unit whose body is not Core has no references to read.
                continue;
            }
            consumed.insert(f.id.0.clone(), self.summary(f, widen));
        }
        self.consumed = consumed;
        self
    }

    /// Which of `f`'s parameters `f` consumes: counted, every reference a
    /// read or a death, no `Dup`, and at least one death. Scanned with no
    /// frame binder, so a parameter stored anywhere at all is an escape.
    fn summary(&self, f: &FnDef, widen: Widen) -> Vec<bool> {
        let mut s = Scan::with_frames(self, BTreeSet::new(), widen);
        s.summary = Some((&f.id.0, f.arity()));
        s.unit(&f.body);
        (0..f.arity())
            .map(|i| {
                counted(f.repr.params.get(i).copied().unwrap_or(Class::Raw))
                    && s.uses.get(&i).is_some_and(|u| u.escapes == 0 && u.duplicated == 0 && u.death + u.applied > 0)
            })
            .collect()
    }
}

/// Whether a word of class `c` is counted (the class pass's rule).
fn counted(c: Class) -> bool {
    matches!(c, Class::Obj | Class::TObj)
}

/// One unit's plan, and the units it hands a frame block to: those need
/// the runtime's frame arms even when they build no frame block
/// themselves (`codegen::Registry::any_frame`).
fn unit(f: &FnDef, facts: &Facts, opts: &Options, report: &mut Report) -> (Plan, BTreeSet<String>) {
    let mut cands: BTreeMap<usize, Result<Site, Refusal>> = BTreeMap::new();
    candidates(&f.body, facts, &mut cands);
    let trace = std::env::var_os("CINCH_STACK_TRACE").is_some();
    let mut plan = Plan::default();
    let mut receivers: BTreeSet<String> = BTreeSet::new();
    if cands.is_empty() {
        return (plan, receivers);
    }
    let (frames, scan) = decide(f, &cands, facts, opts);
    for (x, cand) in &cands {
        report.candidates += 1;
        // The counted payload words, for the trace's column: what a dup or a
        // drop of the block has to act on, and so what the duplicated rule
        // turns on.
        let counted_fields = match cand {
            Ok(site) => site.fields.len(),
            Err(_) => 0,
        };
        let site = match cand {
            Ok(site) if frames.contains(x) => site,
            Ok(site) => {
                // Refused at the fixpoint: either the frame limit's shrink
                // took it (the rule itself says it is eligible) or the last
                // round's scan did.
                let why = eligible(*x, site, &scan).err().unwrap_or(Refusal::FrameLimit);
                report.refuse(why);
                if trace {
                    let detail = scan.uses.get(x).map(|u| u.why).unwrap_or("");
                    let dups = scan.uses.get(x).map_or(0, |u| u.duplicated);
                    eprintln!(
                        "cinch-stack: {} local {x}: refused ({}{detail}, {counted_fields} counted field(s), {dups} dup(s))",
                        f.id,
                        why.name()
                    );
                }
                continue;
            }
            Err(why) => {
                report.refuse(*why);
                if trace {
                    let detail = scan.uses.get(x).map(|u| u.why).unwrap_or("");
                    let dups = scan.uses.get(x).map_or(0, |u| u.duplicated);
                    eprintln!(
                        "cinch-stack: {} local {x}: refused ({}{detail}, {counted_fields} counted field(s), {dups} dup(s))",
                        f.id,
                        why.name()
                    );
                }
                continue;
            }
        };
        let slot = plan.slots.len() as u32;
        plan.bytes += site.bytes();
        plan.slots.push(site.words);
        plan.sites.insert(*x, Site { slot, ..site.clone() });
        if let Some(units) = scan.consumers.get(x) {
            receivers.extend(units.iter().cloned());
        }
        // A frame closure's apply stub releases it when it is called, so
        // the closure's own unit takes the frame arms too.
        if let Shape::Closure { fn_id, .. } = &site.shape {
            receivers.insert(fn_id.0.clone());
        }
        if trace {
            let u = scan.uses.get(x).copied().unwrap_or_default();
            eprintln!(
                "cinch-stack: {} local {x}: slot {slot}, {} bytes, {counted_fields} counted field(s), \
                 {} consumptions ({}, {} stored, {} in the callee's frame, {} applied)",
                f.id,
                site.bytes(),
                u.death + u.consumed + u.stored + u.applied,
                u.lent,
                u.stored,
                u.consumed,
                u.applied
            );
        }
    }
    (plan, receivers)
}

/// The rule's fixpoint for one unit: the binders that take a frame slot,
/// with the scan they were decided under.
///
/// A store into a block is admissible only when that block is itself
/// frame-bound, so the store rule makes the decision a fixpoint: start with
/// no frame binder and repeat, each round taking the candidates the last
/// round's frame set makes eligible. Adding a frame binder can only turn a
/// store from an escape into a consumption, so the set grows and the
/// iteration terminates.
///
/// The frame limit is *not* part of that iteration: it would make the
/// decision non-monotone (an earlier slot can push a later one over the
/// limit, and dropping it can re-admit the earlier one). It is applied as a
/// shrink of the fixpoint instead, and the fixpoint is re-run under the
/// shrunk set, which only removes further binders, so that loop terminates
/// too.
fn decide<'f>(f: &FnDef, cands: &BTreeMap<usize, Result<Site, Refusal>>, facts: &'f Facts, opts: &Options) -> (BTreeSet<usize>, Scan<'f>) {
    let (mut frames, _) = fixpoint(f, cands, facts, opts, None);
    loop {
        let cap = budget(cands, &frames, opts);
        let (next, scan) = fixpoint(f, cands, facts, opts, Some(&cap));
        if next == frames {
            return (frames, scan);
        }
        frames = next;
    }
}

/// [`decide`]'s structural iteration under an optional set the result may
/// not leave (`cap`).
fn fixpoint<'f>(
    f: &FnDef,
    cands: &BTreeMap<usize, Result<Site, Refusal>>,
    facts: &'f Facts,
    opts: &Options,
    cap: Option<&BTreeSet<usize>>,
) -> (BTreeSet<usize>, Scan<'f>) {
    let mut frames: BTreeSet<usize> = cap.cloned().unwrap_or_default();
    loop {
        let mut scan = Scan::with_frames(facts, frames.clone(), opts.widen);
        scan.unit(&f.body);
        let mut next: BTreeSet<usize> = BTreeSet::new();
        for (x, cand) in cands {
            let Ok(site) = cand else { continue };
            if eligible(*x, site, &scan).is_ok() {
                next.insert(*x);
            }
        }
        if let Some(cap) = cap {
            next.retain(|x| cap.contains(x));
        }
        if next == frames {
            return (next, scan);
        }
        frames = next;
    }
}

/// The frame bytes a set of binders reserves, in binder order, and the
/// prefix of it that fits the limit: the candidates the frame-limit rule
/// keeps (`decide`'s shrink).
fn budget(
    cands: &BTreeMap<usize, Result<Site, Refusal>>,
    frames: &BTreeSet<usize>,
    opts: &Options,
) -> BTreeSet<usize> {
    let mut bytes = 0;
    let mut kept = BTreeSet::new();
    for x in frames {
        let Some(Ok(site)) = cands.get(x) else { continue };
        bytes += site.bytes();
        if bytes > opts.frame_limit {
            break;
        }
        kept.insert(*x);
    }
    kept
}

/// Why a candidate cannot take a frame slot, given the round's scan: the
/// rule's checks in the order the report names them. The frame limit is
/// [`budget`]'s, not this function's.
fn eligible(x: usize, site: &Site, scan: &Scan<'_>) -> Result<(), Refusal> {
    let u = scan.uses.get(&x).copied().unwrap_or_default();
    // A second reference is harmless only when the payload holds nothing to
    // release: a frame block's dup writes no count (the `RC_FRAME` header
    // is never counted) and its drop frees nothing, so two references
    // cannot disagree about a payload that has no owned word in it. With a
    // counted word, the release of one reference would leave the other
    // reading dropped children, and no count can tell them apart.
    if u.duplicated > 0 && !site.fields.is_empty() {
        return Err(Refusal::Duplicated);
    }
    if u.escapes > 0 {
        return Err(Refusal::Escapes);
    }
    if u.death + u.consumed + u.stored + u.applied == 0 {
        // No path releases the payload: the binder is dead on every path,
        // or handed to places that cannot release it. Refuse rather than
        // leak the payload.
        return Err(Refusal::Escapes);
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
    /// References handed to a callee's owned parameter that the callee
    /// consumes: the callee's drop of it is the payload's release.
    consumed: u32,
    /// References stored in a field of a block that is itself frame-bound:
    /// that block's release drops them.
    stored: u32,
    /// References applied as a call's closure, or (in a parameter summary)
    /// forwarded unchanged to the same position of a self-call: the apply
    /// stub's frame arm releases the payload, once (section 7.9).
    applied: u32,
    /// `Dup` operations: a second reference exists.
    duplicated: u32,
    /// References that leave the activation or are stored inside a block
    /// that is not frame-bound.
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
    /// An argument of a call the pass cannot see into, or of a join's jump.
    Arg,
    /// An argument of a call whose parameter is owned and consumed.
    Consumed,
    /// An argument of a call whose parameter is lent: the callee only reads
    /// it.
    Lent,
    /// The callee of an application not in tail position: the apply stub
    /// consumes the closure, and its frame arm releases a frame block's
    /// payload.
    Head,
    /// The callee of an application in tail position. The stub runs in the
    /// frame that replaced this one, so a closure in this activation's slot
    /// is gone by then: an escape for the allocating unit, a consumption in
    /// a parameter summary (the parameter's block sits in a caller's frame,
    /// which outlives the tail call).
    TailHead,
    /// A parameter passed unchanged to its own position of a self-call, in
    /// a parameter summary: consumed exactly when the parameter is, which
    /// is the summary being computed.
    SelfForward,
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
    /// An argument of a call in tail position. The callee's frame replaces
    /// the caller's (`musttail`, or a self tail call's jump to the loop
    /// header), so the slot holding this value is about to be overwritten
    /// by the callee's own spills: never a frame block, whatever the
    /// parameter's ownership says.
    TailCall,
}

/// The reference scan of one unit: every local's uses under one round's
/// frame decision, and the units each local was handed to as a consumed
/// owned argument.
struct Scan<'a> {
    /// The facts the classification reads: each unit's borrowed and
    /// consumed parameters, and the constructor field classes.
    facts: &'a Facts,
    /// The binders this round lowers into the frame. A store into one of
    /// them is a consumption (the block's own release drops it); a store
    /// into any other block is an escape.
    frames: BTreeSet<usize>,
    /// The transfers this run admits beyond a read or a death
    /// ([`Options::widen`]).
    widen: Widen,
    uses: BTreeMap<usize, Uses>,
    /// The callees each binder was handed to as a consumed owned argument:
    /// the units that must carry the runtime's frame arms.
    consumers: BTreeMap<usize, BTreeSet<String>>,
    /// The callee of the consumed argument being scanned, so a reference
    /// inside it records which unit receives the block.
    callee: Option<String>,
    /// When the scan is a parameter summary, the unit it summarises: a tail
    /// application of a parameter is then a consumption, and so is a
    /// self-call forwarding a parameter to its own position.
    summary: Option<(&'a str, usize)>,
}

impl<'a> Scan<'a> {
    fn with_frames(facts: &'a Facts, frames: BTreeSet<usize>, widen: Widen) -> Scan<'a> {
        Scan { facts, frames, widen, uses: BTreeMap::new(), consumers: BTreeMap::new(), callee: None, summary: None }
    }

    /// Scan the unit's whole body. The body is the unit's result, so it
    /// sits in tail position.
    fn unit(&mut self, body: &Core) {
        self.go(body, Role::Value, None, true);
    }

    /// Accumulate every local's reference uses in `e`. `role` is the
    /// position `e`'s own value sits in, which is what makes a bare `Var`
    /// readable as a scrutinee, a projection's container or a consumed
    /// argument instead of an escape. `owner` names the binder of the block
    /// this value is stored into, when the scan knows it: a `let` whose
    /// value is a constructor allocation passes its own binder to that
    /// allocation's fields. `tail` says whether `e` is the value the unit
    /// returns on this path: only the forms code generation's tail position
    /// passes through keep it (`Let`, `Dup`, `Drop`, `Release`, an `If`
    /// branch, a `case` alternative body and a `Join`'s two bodies:
    /// `CoreEmitter::emit_to` settles all of them into the same
    /// destination).
    fn go(&mut self, e: &Core, role: Role, owner: Option<usize>, tail: bool) {
        match e {
            Core::Var(x) => self.note(x.0, role, owner),
            // A count operation's body is the value the whole expression
            // has, so it keeps the position, tail included: code generation
            // emits the count and then settles the body into the same
            // destination (`CoreEmitter::emit_to`).
            Core::Dup(x, _, body) => {
                self.uses_of(x.0).duplicated += 1;
                self.go(body, role, owner, tail);
                return;
            }
            Core::Drop(x, _, body) => {
                self.uses_of(x.0).death += 1;
                self.go(body, role, owner, tail);
                return;
            }
            Core::If(c, t, f) => {
                self.go(c, Role::Value, None, false);
                self.go(&t.body, role, owner, tail);
                self.go(&f.body, role, owner, tail);
                return;
            }
            Core::Release { block, moved, rest, body, .. } => {
                self.uses_of(block.0).death += 1;
                // A `Release` field is a pattern binder; if a `let` binder
                // ever names one, it was stored in a block: not this pass's
                // value.
                for (x, _) in moved.iter().chain(rest) {
                    self.note(x.0, Role::Store, None);
                }
                self.go(body, role, owner, tail);
                return;
            }
            Core::Case { scrut, alts } => {
                self.go(scrut, Role::Scrutinee, None, false);
                for a in alts {
                    // An alternative's body is the case's value.
                    self.go(&a.body, role, owner, tail);
                }
                return;
            }
            // A projection reads one field and hands out an owned reference
            // to it; the block keeps its own. A memo cell or a knot is
            // written rather than read (`Demand`, `BoxGet`, `BoxClear`), and
            // no frame block is one: those stay escapes.
            Core::Proj(d, _) | Core::DictProj(d, ..) => {
                self.go(d, Role::Project, None, false);
            }
            Core::Demand(d, _) | Core::BoxGet(d, _) | Core::BoxClear(d, _) => {
                self.go(d, Role::Value, None, false);
            }
            Core::RefSet(x, v, _) | Core::BoxInit(x, v, _) | Core::LazyInit(x, v) => {
                self.note(x.0, Role::Write, None);
                self.go(v, Role::Store, None, false);
            }
            Core::Let(x, c, v, body) => {
                // A constructor allocation's arguments are its block's
                // fields, so a reference stored in one is a store into the
                // binder `x` (`eligible` admits it when `x` is frame-bound).
                let alloc = matches!(&**v, Core::App(h, _) if matches!(&**h, Core::Ctor(..)));
                self.go(v, Role::Value, alloc.then_some(x.0), false);
                self.go(body, role, owner, tail);
                return;
            }
            Core::App(h, args) => {
                let ctor = matches!(&**h, Core::Ctor(..));
                // A call in tail position hands its frame to the callee:
                // code generation emits a `musttail` call for a saturated
                // call whose signature matches the caller's, and a self
                // tail call is a jump to the loop header, which re-executes
                // this unit's own slot writes. Either way the argument the
                // callee receives would be overwritten by the callee's
                // spills as soon as it runs, so no argument of a tail call
                // is a transfer: the block stays on the heap. A call in any
                // other position gets a frame of its own, below this one,
                // and keeps the two transfers.
                let (modes, consumed) = if tail || ctor {
                    (None, None)
                } else {
                    (
                        callee_modes(self.facts, h).filter(|m| m.len() == args.len()),
                        self.widen
                            .consumes()
                            .then(|| callee_consumes(self.facts, h))
                            .flatten()
                            .filter(|c| c.len() == args.len()),
                    )
                };
                self.go(h, if tail { Role::TailHead } else { Role::Head }, None, false);
                let self_call = match (&**h, self.summary) {
                    (Core::Global(g, _), Some((unit, arity))) => g.0 == unit && args.len() == arity,
                    _ => false,
                };
                for (i, a) in args.iter().enumerate() {
                    if self_call && matches!(a, Core::Var(x) if x.0 == i) {
                        self.go(a, Role::SelfForward, None, false);
                        continue;
                    }
                    if ctor {
                        // The fields of the allocation this value is: the
                        // binder the last `let` passed down.
                        self.go(a, Role::Field, owner, false);
                        continue;
                    }
                    if tail {
                        self.go(a, Role::TailCall, None, false);
                    } else if modes.is_some_and(|m| m[i]) {
                        self.go(a, Role::Lent, None, false);
                    } else if consumed.is_some_and(|c| c[i]) {
                        if let Core::Global(g, _) = &**h {
                            self.callee = Some(g.0.clone());
                        }
                        self.go(a, Role::Consumed, None, false);
                        self.callee = None;
                    } else {
                        self.go(a, Role::Arg, None, false);
                    }
                }
                return;
            }
            Core::InstDict(_, args, _) => {
                for a in args {
                    self.go(a, Role::Arg, None, false);
                }
                return;
            }
            // A join is a block, not a call: its body and its continuation
            // both settle into this expression's destination, so a tail
            // call in either is a tail call in the emitted code.
            Core::Join { body, rest, .. } => {
                self.go(body, role, owner, tail);
                self.go(rest, role, owner, tail);
                return;
            }
            Core::Lam { env, .. } => {
                for a in env {
                    self.go(a, Role::Capture, None, false);
                }
                return;
            }
            Core::Jump { args, .. } => {
                for a in args {
                    self.go(a, Role::Arg, None, false);
                }
                return;
            }
            // A quotation's retained body is a code unit of its own.
            Core::Quote { holes, .. } => {
                for h in holes {
                    self.go(h, Role::Value, None, false);
                }
                return;
            }
            _ => {}
        }
        // A part of a compound value is an ordinary value position: the
        // role belongs to the whole.
        for k in crate::core::child_slots_imm(e) {
            self.go(k, Role::Value, None, false);
        }
    }

    /// Record one reference of local `x` in `role`.
    fn note(&mut self, x: usize, role: Role, owner: Option<usize>) {
        match role {
            // A read: the block stays the binder's own.
            Role::Scrutinee | Role::Project => self.uses_of(x).scrutinee += 1,
            Role::Lent => self.uses_of(x).lent += 1,
            Role::Head | Role::SelfForward => self.uses_of(x).applied += 1,
            Role::TailHead if self.summary.is_some() => self.uses_of(x).applied += 1,
            Role::Consumed => {
                self.uses_of(x).consumed += 1;
                if let Some(unit) = self.callee.clone() {
                    self.consumers.entry(x).or_default().insert(unit);
                }
            }
            Role::Field => {

                // A store into a block whose binder this round frame-binds is
                // a consumption: that block's release drops the field. A
                // store into any other block keeps the child on the heap,
                // because nothing proves the holder dies with the
                // activation.
                if self.widen.stores() && owner.is_some_and(|p| self.frames.contains(&p)) {
                    self.uses_of(x).stored += 1;
                } else {
                    self.escape(x, "field");
                }
            }
            _ => {
                self.escape(x, role_name(role))
            }
        }
    }

    fn escape(&mut self, x: usize, why: &'static str) {
        let u = self.uses_of(x);
        u.escapes += 1;
        if u.why.is_empty() {
            u.why = why;
        }
    }

    fn uses_of(&mut self, x: usize) -> &mut Uses {
        self.uses.entry(x).or_default()
    }
}

/// The trace's name for a role an escape sits in.
fn role_name(role: Role) -> &'static str {
    match role {
        Role::Value => "value",
        Role::Field => "field",
        Role::Arg => "call",
        Role::Consumed => "consumed",
        Role::Lent => "lent",
        Role::Head => "head",
        Role::TailHead => "tail-head",
        Role::SelfForward => "self-forward",
        Role::Capture => "capture",
        Role::Scrutinee => "scrutinee",
        Role::Project => "project",
        Role::Store => "store",
        Role::Write => "write",
        Role::TailCall => "tail-call",
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

/// The parameters of a saturated call's head that the callee consumes
/// ([`Facts::consumed`]). Only a Cinch unit has one: a primitive's and a
/// foreign wrapper's body is not Core, so a frame block is never handed to
/// them.
fn callee_consumes<'f>(facts: &'f Facts, head: &Core) -> Option<&'f Vec<bool>> {
    match head {
        Core::Global(g, _) => facts.consumed.get(&g.0),
        _ => None,
    }
}

/// Re-derive every frame decision in debug builds and refuse a plan that
/// breaks the rule: a compiler bug at compile time (design/no-faults.md).
fn verify(program: &Program, opts: &Options) {
    let facts = Facts::of(program, opts.widen);
    for (unit, plan) in &program.frame {
        let Some(f) = program.fns.iter().find(|f| f.id.0 == *unit) else { continue };
        let mut cands: BTreeMap<usize, Result<Site, Refusal>> = BTreeMap::new();
        candidates(&f.body, &facts, &mut cands);
        let (frames, scan) = decide(f, &cands, &facts, opts);
        // The plan is exactly the rule's fixpoint: a slot the rule does not
        // admit, or a candidate the plan left out that the rule admits, is
        // a compiler bug.
        let sites: BTreeSet<usize> = plan.sites.keys().copied().collect();
        assert_eq!(
            sites, frames,
            "compiler bug: {unit}: the frame slots are not the rule's fixpoint"
        );
        let words: Vec<u32> = plan.sites.values().map(|s| s.words).collect();
        assert_eq!(words, plan.slots, "compiler bug: {unit}: a frame slot is not its binder's payload");
        let bytes: u64 = plan.sites.values().map(|s| s.bytes()).sum();
        assert_eq!(bytes, plan.bytes, "compiler bug: {unit}: the frame bytes are not the slots'");
        for (x, site) in &plan.sites {
            let u = scan.uses.get(x).copied().unwrap_or_default();
            // A second reference is admissible exactly when the payload has
            // no counted word to release twice (the rule `eligible` states),
            // and some path must release the payload.
            assert!(
                (u.duplicated == 0 || site.fields.is_empty()) && u.escapes == 0,
                "compiler bug: {unit}: frame binder {x} escapes ({} dups, {} escapes, {} counted fields)",
                u.duplicated,
                u.escapes,
                site.fields.len()
            );
            assert!(
                u.death + u.consumed + u.stored + u.applied > 0,
                "compiler bug: {unit}: frame binder {x} has no release"
            );
        }
    }
}

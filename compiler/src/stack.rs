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

use crate::core::{Core, FnDef, GlobalId, Program};
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
    /// The payload bytes the slots hold, header and alignment aside: the
    /// column the census's refused classes are measured in.
    pub payload: u64,
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

/// Why a candidate allocation stayed on the heap: the escape census's
/// classification (design/stack-allocation.md section 8).
///
/// The first six variants are the rule's own checks, in the order
/// [`eligible`] applies them: a candidate that fails one of those would be
/// frame-allocated if that check alone were relaxed. The rest name the
/// position of a reference that leaves the allocating activation, which is
/// the population a wider rule (or a partial-escape split) would have to
/// reach.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Refusal {
    /// A boundary-layout (`repr(C)`, `repr(packed)`, `repr(transparent)`)
    /// constructor, whose fields do not sit at their payload index.
    Layout,
    /// A `let`-bound allocation the emitter would not lower as one block (a
    /// partial application, a nullary constructor, an unknown unit, or a
    /// closure over a unit that does not exist).
    Form,
    /// The binder's class is not counted (`Class::Raw`, or a `Class::Param`
    /// the collector left), so there is no `Drop` for the payload release
    /// to hang on.
    Uncounted,
    /// The binder is `Dup`ped and its payload holds a counted word: a
    /// second reference needs a second release of that word, which a frame
    /// block has no count to make a no-op.
    Duplicated,
    /// The payload is above the block limit and nothing else refuses it.
    Size,
    /// The unit's frame would pass its limit and nothing else refuses it.
    FrameLimit,
    /// A reference is the unit's own result.
    Return,
    /// A reference is the value of a `let`, or another value position that
    /// is not the unit's result.
    Value,
    /// A reference is stored into a field of a block that is not
    /// frame-bound.
    Field,
    /// A reference is an argument of a call the pass cannot see into.
    Call,
    /// A reference is an argument of a call in tail position.
    TailCall,
    /// A reference is an argument of `spawn`.
    Spawn,
    /// A reference is an argument of a join's `Jump`.
    Jump,
    /// A reference is a closure's saved word.
    Capture,
    /// A reference is written through a cell (`RefSet`, `BoxInit`,
    /// `LazyInit`), or names the cell being written.
    Write,
    /// A reference is the callee of an application.
    Head,
}

impl Refusal {
    /// Every class, in report order: the rule's checks, then the escape
    /// positions.
    pub const ALL: [Refusal; 16] = [
        Refusal::Layout,
        Refusal::Form,
        Refusal::Uncounted,
        Refusal::Duplicated,
        Refusal::Size,
        Refusal::FrameLimit,
        Refusal::Return,
        Refusal::Value,
        Refusal::Field,
        Refusal::Call,
        Refusal::TailCall,
        Refusal::Spawn,
        Refusal::Jump,
        Refusal::Capture,
        Refusal::Write,
        Refusal::Head,
    ];

    /// The name the census table, the manifest and the dump use.
    pub fn name(self) -> &'static str {
        match self {
            Refusal::Layout => "layout",
            Refusal::Form => "form",
            Refusal::Uncounted => "uncounted",
            Refusal::Duplicated => "duplicated",
            Refusal::Size => "size",
            Refusal::FrameLimit => "frame-limit",
            Refusal::Return => "return",
            Refusal::Value => "value",
            Refusal::Field => "field",
            Refusal::Call => "call",
            Refusal::TailCall => "tail-call",
            Refusal::Spawn => "spawn",
            Refusal::Jump => "jump",
            Refusal::Capture => "capture",
            Refusal::Write => "write",
            Refusal::Head => "head",
        }
    }

    /// The rule-level roll-up the manifest's `refused` row and the design
    /// note before section 8 use: every escape position is one `escapes`.
    pub fn coarse(self) -> &'static str {
        match self {
            Refusal::Layout
            | Refusal::Form
            | Refusal::Uncounted
            | Refusal::Duplicated
            | Refusal::Size
            | Refusal::FrameLimit => self.name(),
            _ => "escapes",
        }
    }

    /// Whether this class is a reference that leaves the activation.
    pub fn is_escape(self) -> bool {
        !matches!(
            self,
            Refusal::Layout
                | Refusal::Form
                | Refusal::Uncounted
                | Refusal::Duplicated
                | Refusal::Size
                | Refusal::FrameLimit
        )
    }
}

/// One census row: how many candidates a class holds and the payload bytes
/// they would have carried.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub count: u64,
    /// The payload bytes (`words * 8`) of the allocations in the class, the
    /// header and alignment aside. Zero for a class whose allocation the
    /// pass could not size (`layout` is sized, `form` and `uncounted` are
    /// not: a form the pass cannot lower has no known arity).
    pub bytes: u64,
}

impl Tally {
    fn add(&mut self, bytes: u64) {
        self.count += 1;
        self.bytes += bytes;
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
    /// The payload bytes those slots hold, header and alignment aside: the
    /// column the census's refused classes are measured in.
    pub payload: u64,
    /// Every `let`-bound allocation the pass considered.
    pub candidates: u64,
    /// The candidates that stayed on the heap, rolled up to the rule's own
    /// names (every escape position is one `escapes`).
    pub refused: BTreeMap<&'static str, u64>,
    /// The census: every class of [`Refusal::ALL`], with the count and the
    /// payload bytes. `frame` is the class of the candidates the pass
    /// lowered, so the row counts sum to [`Report::candidates`].
    ///
    /// A class here is the *binding* reason: the first check that failed,
    /// in [`eligible`]'s order. A candidate that fails two checks appears
    /// once, under the one a change would have to relax first, which is
    /// what "why did this stay on the heap" means.
    pub classes: BTreeMap<&'static str, Tally>,
    /// Every escape position, counted independently: a candidate with an
    /// escape in two positions appears under both. This is the population
    /// each widening would reach on its own, which the binding-reason
    /// column cannot show when the binding reason is a rule-level check
    /// (a duplicated counted payload that also escapes).
    pub positions: BTreeMap<&'static str, Tally>,
    /// The candidates whose references are consumed on one path of a
    /// branch and escape on another: the population a partial-escape split
    /// (materialise a heap copy only on the escaping path) would reach.
    pub partial: Tally,
    /// The largest candidate payload seen (`words * 8`), and the class it
    /// was refused by: the fact that says how much a larger block limit
    /// would buy. When [`Report::largest`] is below `DEFAULT_BLOCK_LIMIT`,
    /// the `size` class is empty because nothing is that big, not because
    /// the check is unreachable.
    pub largest: u64,
    pub largest_class: &'static str,
    /// The units that were handed a frame block as an owned argument: each
    /// callee whose parameter the pass let a frame allocation flow into.
    /// Code generation gives such a unit the runtime's frame arms even
    /// when it builds no frame block of its own (`codegen::Registry`).
    pub receives_frame: std::collections::BTreeSet<String>,
}

/// The census row for the candidates the pass lowered.
pub const FRAME_CLASS: &str = "frame";

impl Report {
    /// Record every escape a candidate has, by position: the independent
    /// column.
    fn positions(&mut self, u: &Uses, bytes: u64) {
        for (i, why) in Refusal::ALL.iter().enumerate() {
            if u.escaped[i] > 0 {
                self.positions.entry(why.name()).or_default().add(bytes);
            }
        }
    }

    fn refuse(&mut self, why: Refusal, bytes: u64) {
        *self.refused.entry(why.coarse()).or_insert(0) += 1;
        self.classes.entry(why.name()).or_default().add(bytes);
        if bytes == self.largest {
            self.largest_class = why.name();
        }
    }

    fn frame(&mut self, bytes: u64) {
        self.classes.entry(FRAME_CLASS).or_default().add(bytes);
        if bytes == self.largest {
            self.largest_class = FRAME_CLASS;
        }
    }

    /// The census as a table, for `--escape-report` and
    /// `CINCH_ESCAPE_REPORT=1`. `what` names the workload the numbers are
    /// over.
    pub fn table(&self, what: &str) -> String {
        let mut out = String::new();
        let total = self.candidates;
        let frame = self.classes.get(FRAME_CLASS).copied().unwrap_or_default();
        let pct = |n: u64| {
            if total == 0 {
                0.0
            } else {
                100.0 * n as f64 / total as f64
            }
        };
        out.push_str(&format!(
            "cinch escape census: {what}: {total} let-bound allocation(s), {} in a frame ({:.1}%), {} refused\n",
            frame.count,
            pct(frame.count),
            total - frame.count
        ));
        out.push_str("  class                  count     %      bytes\n");
        out.push_str(&format!(
            "  {:<20} {:>6} {:>5.1} {:>10}\n",
            FRAME_CLASS,
            frame.count,
            pct(frame.count),
            frame.bytes
        ));
        for why in Refusal::ALL {
            let t = self.classes.get(why.name()).copied().unwrap_or_default();
            if t.count == 0 {
                continue;
            }
            out.push_str(&format!(
                "  {:<20} {:>6} {:>5.1} {:>10}\n",
                why.name(),
                t.count,
                pct(t.count),
                t.bytes
            ));
        }
        if !self.positions.is_empty() {
            out.push_str("\n  escape positions (independent: a candidate may hold several)\n");
            out.push_str("  position               count     %      bytes\n");
            for why in Refusal::ALL.iter().filter(|r| r.is_escape()) {
                let t = self.positions.get(why.name()).copied().unwrap_or_default();
                if t.count == 0 {
                    continue;
                }
                out.push_str(&format!(
                    "  {:<20} {:>6} {:>5.1} {:>10}\n",
                    why.name(),
                    t.count,
                    pct(t.count),
                    t.bytes
                ));
            }
        }
        out.push_str(&format!(
            "  largest candidate: {} bytes ({}, block limit {DEFAULT_BLOCK_LIMIT}, frame limit applies per unit)\n",
            self.largest,
            if self.largest_class.is_empty() { FRAME_CLASS } else { self.largest_class }
        ));
        if self.partial.count > 0 {
            out.push_str(&format!(
                "  {:<20} {:>6} {:>5.1} {:>10}\n",
                "of which partial", self.partial.count, pct(self.partial.count), self.partial.bytes
            ));
        }
        out
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
        report.payload += plan.payload;
        plans.insert(unit, plan);
    }
    program.stack = report;
    program.frame = plans;
    if cfg!(debug_assertions) {
        verify(program, opts);
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
        s.unit(&f.body);
        (0..f.arity())
            .map(|i| {
                counted(f.repr.params.get(i).copied().unwrap_or(Class::Raw))
                    && s.uses.get(&i).is_some_and(|u| u.escapes() == 0 && u.duplicated == 0 && u.death > 0)
            })
            .collect()
    }
}

/// Whether a word of class `c` is counted (the class pass's rule).
fn counted(c: Class) -> bool {
    matches!(c, Class::Obj | Class::TObj)
}

/// One allocation site the pass considers, before the rule's scan.
///
/// The census needs three facts about a site the old `Result<Site,
/// Refusal>` could not carry together: what the block would cost (for the
/// `size` and `frame-limit` columns), the refusal decided without looking
/// at any reference (a boundary layout, a form the emitter will not lower,
/// an uncounted binder), and the site itself when there is one.
#[derive(Clone, Debug)]
struct Cand {
    /// The site, when the pass can size and lower this allocation.
    site: Option<Site>,
    /// The refusal the site's own shape decides, before any reference is
    /// classified.
    blocked: Option<Refusal>,
    /// The payload bytes (`words * 8`), header and alignment aside: the
    /// unit the census's refused columns are measured in.
    bytes: u64,
}

impl Cand {
    /// The site the rule needs, or the refusal that stands in for it.
    fn site(&self) -> Result<&Site, Refusal> {
        match (&self.site, self.blocked) {
            (Some(site), None) => Ok(site),
            (_, Some(why)) => Err(why),
            (None, None) => Err(Refusal::Form),
        }
    }
}

/// One unit's plan, and the units it hands a frame block to: those need
/// the runtime's frame arms even when they build no frame block
/// themselves (`codegen::Registry::any_frame`).
fn unit(f: &FnDef, facts: &Facts, opts: &Options, report: &mut Report) -> (Plan, BTreeSet<String>) {
    let mut cands: BTreeMap<usize, Cand> = BTreeMap::new();
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
        if cand.bytes > report.largest {
            report.largest = cand.bytes;
        }
        // The counted payload words, for the trace's column: what a dup or a
        // drop of the block has to act on, and so what the duplicated rule
        // turns on.
        let counted_fields = cand.site.as_ref().map_or(0, |s| s.fields.len());
        let u = scan.uses.get(x).cloned().unwrap_or_default();
        if !frames.contains(x) {
            // The refusal, from the last round's scan: the rule's own check
            // that failed, or the frame limit's shrink, which is what is
            // left when every check passes and the slot did not fit.
            let why = eligible(*x, cand, &scan).err().unwrap_or(Refusal::FrameLimit);
            report.refuse(why, cand.bytes);
            report.positions(&u, cand.bytes);
            // The partial-escape population: a candidate refused *because*
            // it escapes, whose escape sits on one branch while another
            // branch consumes it. A candidate refused by a rule-level check
            // (a duplicated counted payload, a boundary layout) is not this
            // population: its escape is not what stops it.
            if why.is_escape() && u.partial() {
                report.partial.add(cand.bytes);
            }
            if trace {
                eprintln!(
                    "cinch-stack: {} local {x}: refused ({}), {} byte(s), {counted_fields} counted field(s), {} dup(s), {} escape(s){}",
                    f.id,
                    why.name(),
                    cand.bytes,
                    u.duplicated,
                    u.escapes(),
                    if u.partial() { ", partial" } else { "" }
                );
            }
            continue;
        }
        let site = match cand.site() {
            Ok(site) => site,
            Err(why) => {
                report.refuse(why, cand.bytes);
                continue;
            }
        };
        let slot = plan.slots.len() as u32;
        plan.bytes += site.bytes();
        plan.payload += u64::from(site.words) * 8;
        plan.slots.push(site.words);
        plan.sites.insert(*x, Site { slot, ..site.clone() });
        report.frame(cand.bytes);
        if let Some(units) = scan.consumers.get(x) {
            receivers.extend(units.iter().cloned());
        }
        if trace {
            eprintln!(
                "cinch-stack: {} local {x}: slot {slot}, {} bytes, {counted_fields} counted field(s), \
                 {} consumptions ({}, {} stored, {} in the callee's frame)",
                f.id,
                site.bytes(),
                u.death + u.consumed + u.stored,
                u.lent,
                u.stored,
                u.consumed
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
fn decide<'f>(f: &FnDef, cands: &BTreeMap<usize, Cand>, facts: &'f Facts, opts: &Options) -> (BTreeSet<usize>, Scan<'f>) {
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
    cands: &BTreeMap<usize, Cand>,
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
            if eligible(*x, cand, &scan).is_ok() {
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
    cands: &BTreeMap<usize, Cand>,
    frames: &BTreeSet<usize>,
    opts: &Options,
) -> BTreeSet<usize> {
    let mut bytes = 0;
    let mut kept = BTreeSet::new();
    for x in frames {
        let Some(site) = cands.get(x).and_then(|c| c.site.as_ref()) else { continue };
        bytes += site.bytes();
        if bytes > opts.frame_limit {
            break;
        }
        kept.insert(*x);
    }
    kept
}

/// Why a candidate cannot take a frame slot, given the round's scan: the
/// rule's checks in the order [`Refusal::ALL`] names them. The frame limit
/// is [`budget`]'s, not this function's.
///
/// The checks are ordered so that each column of the census is the
/// population *that check alone* refuses: a site whose shape is refused
/// (`layout`, `form`, `uncounted`, `size`) is never counted as an escape,
/// and an escape is reported by its position. The order the rule itself
/// applies them is the same: a block that will not fit its limit is not
/// frame-allocated whatever its references do, and a duplicated payload
/// with a counted word is refused before its escapes matter.
fn eligible(x: usize, cand: &Cand, scan: &Scan<'_>) -> Result<(), Refusal> {
    if let Some(why) = cand.blocked {
        return Err(why);
    }
    let site = cand.site.as_ref().ok_or(Refusal::Form)?;
    // A block above the block limit is never a frame block. Reported before
    // any reference, because the size is the reason and the bytes are the
    // question the census asks of this class.
    if u64::from(site.words) * 8 > DEFAULT_BLOCK_LIMIT {
        return Err(Refusal::Size);
    }
    let u = scan.uses.get(&x).cloned().unwrap_or_default();
    // A second reference is harmless only when the payload holds nothing to
    // release: a frame block's dup writes no count (the `RC_FRAME` header
    // is never counted) and its drop frees nothing, so two references
    // cannot disagree about a payload that has no owned word in it. With a
    // counted word, the release of one reference would leave the other
    // reading dropped children, and no count can tell them apart.
    if u.duplicated > 0 && !site.fields.is_empty() {
        return Err(Refusal::Duplicated);
    }
    if let Some(why) = u.first() {
        return Err(why);
    }
    if u.death + u.consumed + u.stored == 0 {
        // No path releases the payload: the binder is dead on every path,
        // or handed to places that cannot release it. Refuse rather than
        // leak the payload.
        return Err(Refusal::Return);
    }
    Ok(())
}

/// Every `let`-bound allocation in `e` the pass considers, keyed by the
/// binder, with the site or the reason it cannot be one.
///
/// A `let` whose value is an allocation expression is a candidate whether
/// or not the pass can lower it: the census's `uncounted`, `form` and
/// `layout` columns are exactly the candidates the old rule dropped before
/// looking at any reference, so they have to reach the report to be
/// counted at all. A `let` whose value is not an allocation (a `Var`, a
/// primitive, a string literal) is not a candidate: nothing here allocates
/// a block.
fn candidates(e: &Core, facts: &Facts, out: &mut BTreeMap<usize, Cand>) {
    match e {
        Core::Let(x, c, v, body) => {
            if let Some(mut cand) = cand_of(v, facts) {
                if !counted(*c) {
                    // No `Drop` hangs on an uncounted binder, so the
                    // payload has no release to give it: the class is the
                    // refusal, and it is reported as its own row rather
                    // than folded into `form`.
                    cand.blocked = Some(Refusal::Uncounted);
                }
                out.insert(x.0, cand);
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

/// The candidate an allocation expression names, or `None` when the
/// expression is not an allocation this pass considers at all (a
/// dictionary, a memo cell, a knot, a boxed number, a string, or a
/// primitive call).
fn cand_of(e: &Core, facts: &Facts) -> Option<Cand> {
    match e {
        Core::App(h, args) => {
            let Core::Ctor(g, _) = &**h else { return None };
            let Some((fields, arity, boundary)) = facts.ctors.get(&g.0) else {
                // A constructor the program does not declare as a unit: the
                // emitter will not lower it as one block.
                return Some(Cand { site: None, blocked: Some(Refusal::Form), bytes: 0 });
            };
            // A partial application builds a closure in the apply stub, and
            // a nullary constructor is a static block: neither allocates
            // this constructor's block here. They are not candidates: there
            // is no block at this site to move into a frame.
            if args.len() != *arity || args.len() != fields.len() || args.is_empty() {
                return None;
            }
            let words = args.len() as u32;
            let bytes = u64::from(words) * 8;
            if *boundary {
                // A boundary layout records its fields at offsets this
                // lowering does not reproduce, so the site is sized and
                // refused.
                let site = payload(words, 0, fields, Shape::Ctor { ctor: g.clone(), arity: words });
                return Some(Cand { site: Some(site), blocked: Some(Refusal::Layout), bytes });
            }
            let site = payload(words, 0, fields, Shape::Ctor { ctor: g.clone(), arity: words });
            Some(Cand { site: Some(site), blocked: None, bytes })
        }
        Core::Lam { fn_id, env, .. } => {
            // The closure's size is its three uncounted header words plus
            // the saved ones, which is known even when the unit the closure
            // calls does not exist: the census's `form` column still
            // carries the bytes such a site would have had.
            let words = 3 + env.len() as u32;
            let bytes = u64::from(words) * 8;
            let Some(params) = facts.params.get(&fn_id.0) else {
                return Some(Cand { site: None, blocked: Some(Refusal::Form), bytes });
            };
            if env.len() > *facts.arity.get(&fn_id.0).unwrap_or(&0) {
                return Some(Cand { site: None, blocked: Some(Refusal::Form), bytes });
            }
            // The closure's first three payload words are its code, arity
            // and filled count, all uncounted; the saved words follow.
            let words = 3 + env.len() as u32;
            let saved = &params[..env.len().min(params.len())];
            let site = payload(words, 3, saved, Shape::Closure { fn_id: fn_id.clone(), env: env.len() as u32 });
            Some(Cand { site: Some(site), blocked: None, bytes: u64::from(words) * 8 })
        }
        _ => None,
    }
}

/// The site of a block of `words` payload words whose counted payload
/// words start at word `base` and have the classes in `classes`.
fn payload(words: u32, base: u32, classes: &[Class], shape: Shape) -> Site {
    let fields = classes
        .iter()
        .enumerate()
        .filter(|(_, c)| counted(**c))
        .map(|(i, c)| ((base + i as u32) * 8, *c))
        .collect();
    Site { slot: 0, words, shape, fields }
}

/// How a binder's references are used, accumulated over the whole unit.
#[derive(Clone, Debug, Default)]
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
    /// `Dup` operations: a second reference exists.
    duplicated: u32,
    /// References that leave the activation, by position: the census's
    /// escape classes. A class's count is its [`Refusal`]'s, and
    /// [`Uses::escapes`] sums them.
    escaped: [u32; Refusal::ALL.len()],
    /// The first escape's class, for `CINCH_STACK_TRACE` and the census.
    why: Option<Refusal>,
    /// The branch choices the first escape sits under: empty at the unit's
    /// top level, `[1, 0]` inside the second branch's first branch, and so
    /// on ([`Scan::path`]).
    escape_path: Option<Vec<u32>>,
    /// The branch choices of the first few references that end the value
    /// (`Drop`, `Release`, a consuming owned parameter, a store into a
    /// frame-bound block). The partial-escape census compares these with
    /// [`Uses::escape_path`].
    consume_paths: Vec<Vec<u32>>,
}

/// How many consumption paths [`Uses`] remembers: the partial-escape test
/// is a comparison against the first escape's path, and a binder with more
/// branches than this is reported on the ones it kept.
const CONSUME_PATHS: usize = 8;

impl Uses {
    /// Every reference that leaves the activation.
    fn escapes(&self) -> u32 {
        self.escaped.iter().sum()
    }

    /// The first escape's class: the one the census reports.
    fn first(&self) -> Option<Refusal> {
        self.why
    }

    /// Whether this value escapes on one branch and is consumed on
    /// another: the population a partial-escape split would reach
    /// (design/stack-allocation.md section 8.5).
    ///
    /// The test is the branch paths. The value escapes at some path `E`,
    /// and there is a path `C` where it is consumed that is not below `E`
    /// (`C` does not start with `E`): on `C` the escape did not happen, so
    /// the value could have stayed in the frame there and been copied to
    /// the heap only on `E`. An unconditional escape has `E` empty, and
    /// every path starts with the empty path, so it is never partial: that
    /// is the population no split can help.
    fn partial(&self) -> bool {
        let Some(escape) = &self.escape_path else { return false };
        self.consume_paths.iter().any(|death| !death.starts_with(escape))
    }
}

/// Where a subexpression's value goes, so a binder's reference can be
/// classified: the ownership lattice's positions, spelled out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    /// A value position of the ordinary kind (a `let` value, an `If`
    /// condition, a memo demand's cell).
    Value,
    /// The unit's own result: the value the activation hands back.
    Return,
    /// A field of the constructor block `(app (ctor C) ... args)`.
    Field,
    /// An argument of a call the pass cannot see into.
    Arg,
    /// An argument of `spawn`: the task runs on its own thread, past the
    /// end of the allocating activation.
    Spawn,
    /// An argument of a join's `Jump`: a block, not a call, but the join's
    /// parameters are owned exactly as a unit's, and a jump can leave a
    /// loop the block was built in.
    Jump,
    /// An argument of a call whose parameter is owned and consumed.
    Consumed,
    /// An argument of a call whose parameter is lent: the callee only reads
    /// it.
    Lent,
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
    /// The branch choices from the unit's body to the expression being
    /// scanned: `0`/`1` for an `If`'s two branches, the alternative's index
    /// for a `case`. The partial-escape census compares an escape's path
    /// with a consumption's ([`Uses::partial`]).
    path: Vec<u32>,
}

impl<'a> Scan<'a> {
    fn with_frames(facts: &'a Facts, frames: BTreeSet<usize>, widen: Widen) -> Scan<'a> {
        Scan {
            facts,
            frames,
            widen,
            uses: BTreeMap::new(),
            consumers: BTreeMap::new(),
            callee: None,
            path: Vec::new(),
        }
    }

    /// Scan the unit's whole body. The body is the unit's result, so it
    /// sits in tail position and is the value the activation hands back.
    fn unit(&mut self, body: &Core) {
        self.go(body, Role::Return, None, true);
    }

    /// Scan one branch of a decision point: the path records which side.
    fn branch(&mut self, e: &Core, side: u32, role: Role, owner: Option<usize>, tail: bool) {
        self.path.push(side);
        self.go(e, role, owner, tail);
        self.path.pop();
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
                self.death(x.0);
                self.go(body, role, owner, tail);
                return;
            }
            Core::If(c, t, f) => {
                self.go(c, Role::Value, None, false);
                self.branch(&t.body, 0, role, owner, tail);
                self.branch(&f.body, 1, role, owner, tail);
                return;
            }
            Core::Release { block, moved, rest, body, .. } => {
                self.death(block.0);
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
                for (i, a) in alts.iter().enumerate() {
                    // An alternative's body is the case's value.
                    self.branch(&a.body, i as u32, role, owner, tail);
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
                self.go(h, Role::Head, None, false);
                let spawn = matches!(&**h, Core::Prim(crate::core::PrimOp::Conc(crate::core::ConcOp::Spawn)));
                for (i, a) in args.iter().enumerate() {
                    if ctor {
                        // The fields of the allocation this value is: the
                        // binder the last `let` passed down.
                        self.go(a, Role::Field, owner, false);
                        continue;
                    }
                    if spawn {
                        // The task's body runs on another thread of the
                        // scope, after this activation has returned: no
                        // argument of `spawn` is a transfer.
                        self.go(a, Role::Spawn, None, false);
                    } else if tail {
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
                    self.go(a, Role::Jump, None, false);
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
            Role::Consumed => {
                let path = self.path.clone();
                let u = self.uses_of(x);
                u.consumed += 1;
                if u.consume_paths.len() < CONSUME_PATHS {
                    u.consume_paths.push(path);
                }
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
                    let path = self.path.clone();
                    let u = self.uses_of(x);
                    u.stored += 1;
                    if u.consume_paths.len() < CONSUME_PATHS {
                        u.consume_paths.push(path);
                    }
                } else {
                    self.escape(x, Refusal::Field);
                }
            }
            Role::Return => self.escape(x, Refusal::Return),
            Role::Value => self.escape(x, Refusal::Value),
            Role::Arg => self.escape(x, Refusal::Call),
            Role::TailCall => self.escape(x, Refusal::TailCall),
            Role::Spawn => self.escape(x, Refusal::Spawn),
            Role::Jump => self.escape(x, Refusal::Jump),
            Role::Capture => self.escape(x, Refusal::Capture),
            Role::Store => self.escape(x, Refusal::Write),
            Role::Write => self.escape(x, Refusal::Write),
            Role::Head => self.escape(x, Refusal::Head),
        }
    }

    /// Record that a reference of `x` ends the value, on the path the scan
    /// is on.
    fn death(&mut self, x: usize) {
        let path = self.path.clone();
        let u = self.uses_of(x);
        u.death += 1;
        if u.consume_paths.len() < CONSUME_PATHS {
            u.consume_paths.push(path);
        }
    }

    fn escape(&mut self, x: usize, why: Refusal) {
        let path = self.path.clone();
        let u = self.uses_of(x);
        u.escaped[class_index(why)] += 1;
        if u.why.is_none() {
            u.why = Some(why);
            u.escape_path = Some(path);
        }
    }

    fn uses_of(&mut self, x: usize) -> &mut Uses {
        self.uses.entry(x).or_default()
    }
}

/// The index of a class in [`Refusal::ALL`]: the slot its count occupies
/// in [`Uses::escaped`].
fn class_index(why: Refusal) -> usize {
    Refusal::ALL.iter().position(|r| *r == why).expect("every class is in ALL")
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
        let mut cands: BTreeMap<usize, Cand> = BTreeMap::new();
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
        let payload: u64 = plan.sites.values().map(|s| u64::from(s.words) * 8).sum();
        assert_eq!(payload, plan.payload, "compiler bug: {unit}: the payload is not the slots'");
        for (x, site) in &plan.sites {
            let u = scan.uses.get(x).cloned().unwrap_or_default();
            // A second reference is admissible exactly when the payload has
            // no counted word to release twice (the rule `eligible` states),
            // and some path must release the payload.
            assert!(
                (u.duplicated == 0 || site.fields.is_empty()) && u.escapes() == 0,
                "compiler bug: {unit}: frame binder {x} escapes ({} dups, {} escapes, {} counted fields)",
                u.duplicated,
                u.escapes(),
                site.fields.len()
            );
            assert!(
                u.death + u.consumed + u.stored > 0,
                "compiler bug: {unit}: frame binder {x} has no release"
            );
        }
    }
}

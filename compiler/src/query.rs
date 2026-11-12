//! The query graph: demand-driven queries with recorded dependencies,
//! early cutoff by fingerprint, negative lookups as dependencies, and
//! interface-first pipelining (design/compilation-speed.md section 3,
//! design/incremental-recompilation.md).
//!
//! Shape, in Salsa's and Shake's terms and in this compiler's:
//!
//! * A [`Query`] is a key, an [`Engine`] is the memo table plus a revision
//!   counter, and a [`Subject`] computes values. A subject reads its
//!   dependencies through [`get`], so the engine records what the
//!   evaluation actually read rather than what a schema says it should
//!   have read.
//! * Verification is by fingerprint. A memo entry whose every recorded
//!   dependency still carries the fingerprint it had when the entry was
//!   written is valid for the current revision and is not re-executed.
//!   That is Shake's early cutoff, and it is what makes a recomputed
//!   query whose output fingerprint held leave its dependents alone.
//! * A negative lookup is an ordinary cached value ([`Value::Instances`]
//!   with no ids). Recording the searched bucket as a dependency is what
//!   makes "no instance for `C T`" invalidate when a matching instance
//!   appears, and what keeps it valid when one appears elsewhere.
//! * Inputs ([`Query::Input`]) are the leaves a client sets with
//!   [`Engine::set_input`]. Only a set that changes a value's fingerprint
//!   bumps the revision, and the revision is the only thing that makes
//!   anything stale, so setting an input to what it already was costs
//!   nothing.
//!
//! Correctness discipline: a cycle is an error naming the chain, never a
//! guess, and a caller that hits one falls back to the coarser module
//! closure it already had. A bug here costs time, never a stale build.
//!
//! Every query this module defines is answered from state the compiler
//! already produced. `erasedCore` and `ownedCore` from the design note are
//! deliberately absent: erasure and reference counting are whole-program
//! passes over the merged program, not functions of one declaration's
//! inputs, so memoizing them per declaration would record a dependency
//! set that does not describe the work. They become queries when those
//! passes take declaration granularity.

use crate::core::{FnDef, Program};
use crate::interface::{self, Iface};
use crate::{Error, Result};
use std::collections::{BTreeMap, BTreeSet};

// ============================================================ keys and values

/// One build revision. A revision is bumped only when an input's
/// fingerprint changes, so "valid at the current revision" means "verified
/// against the current inputs".
pub type Revision = u64;

/// A query key.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Query {
    /// A named leaf: a module's source hash, a class's visible instance
    /// set. Nothing computes an input; a client sets it.
    Input(String),
    /// One item's contract projection (its `.cinchif` rows: scheme,
    /// contract facts, layouts, laws).
    Signature(String),
    /// One item's checked body (its units' `FnDef` state, span free).
    ElaboratedBody(String),
    /// One item's total-body closure (the primitive bodies its contract
    /// projection carries).
    ProofClosure(String),
    /// The instances that can serve `class` at `head`; `"*"` asks for
    /// every instance of the class. An empty answer is the negative
    /// lookup.
    InstanceBucket(String, String),
    /// One item's optimization summary: what the optimizer decided about
    /// the units it owns.
    OptSummary(String),
    /// One codegen unit's emitted IR text.
    Object(String),
}

impl Query {
    /// The input naming one module's source.
    pub fn source(module: &str) -> Query {
        Query::Input(format!("source:{module}"))
    }

    /// The input naming one class's visible instance set.
    pub fn instances(class: &str) -> Query {
        Query::Input(format!("instances:{class}"))
    }

    /// The item a query is about, when it is about one.
    pub fn item(&self) -> Option<&str> {
        match self {
            Query::Signature(item)
            | Query::ElaboratedBody(item)
            | Query::ProofClosure(item)
            | Query::OptSummary(item)
            | Query::Object(item) => Some(item),
            _ => None,
        }
    }
}

/// A query value. Two variants cover every query here: text (signatures,
/// bodies, IR, summaries) and an instance answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Text(String),
    Instances(Vec<String>),
}

impl Value {
    pub fn text(&self) -> Option<&str> {
        match self {
            Value::Text(t) => Some(t),
            Value::Instances(_) => None,
        }
    }

    pub fn instances(&self) -> Option<&[String]> {
        match self {
            Value::Instances(ids) => Some(ids),
            Value::Text(_) => None,
        }
    }
}

/// What a query's value is worth to its dependents. Equal fingerprints
/// mean downstream work is reusable; that is the whole contract. The
/// fingerprint carries its value's kind, because an empty text and an
/// empty instance list are different answers.
pub trait QueryValue: Clone {
    fn fingerprint(&self) -> String;
}

impl QueryValue for Value {
    fn fingerprint(&self) -> String {
        match self {
            Value::Text(t) => format!("t:{}", interface::content_hash(t)),
            Value::Instances(ids) => {
                format!("i:{}", interface::content_hash(&ids.join("\u{1}")))
            }
        }
    }
}

// ============================================================ the engine

/// A subject computes one query's value, reading its dependencies through
/// [`get`]. `evaluate` is called only when the memo has no entry valid for
/// the current revision, so it may be as expensive as the real work is.
pub trait Subject<V: QueryValue> {
    fn evaluate(&mut self, engine: &mut Engine<V>, key: &Query) -> Result<V>;
}

/// What one round of demand actually did (the honesty discipline: report
/// the counts, do not assume them).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Queries evaluated from scratch.
    pub executed: usize,
    /// Stale entries whose dependency fingerprints all held: verified
    /// without re-executing.
    pub verified: usize,
    /// Entries already valid at the current revision: no walk at all.
    pub hits: usize,
    /// Queries that re-executed and produced the fingerprint they had
    /// before. Their dependents were reused, which is early cutoff.
    pub cutoff: Vec<Query>,
    /// Queries that re-executed and produced a different fingerprint:
    /// what the change actually reached.
    pub changed: Vec<Query>,
    /// The revision this round ran at.
    pub revision: Revision,
}

impl Report {
    /// Merge another report into this one.
    pub fn absorb(&mut self, other: &Report) {
        self.executed += other.executed;
        self.verified += other.verified;
        self.hits += other.hits;
        self.cutoff.extend(other.cutoff.iter().cloned());
        self.changed.extend(other.changed.iter().cloned());
        self.revision = other.revision;
    }
}

struct Memo<V> {
    value: V,
    fingerprint: String,
    /// The revision this entry was last proven valid at.
    verified_at: Revision,
    /// The revision the value last actually changed at. Early cutoff keeps
    /// the old revision when a re-execution reproduces the value.
    changed_at: Revision,
    /// The dependencies the evaluation read, with each one's fingerprint
    /// at the moment it was read.
    deps: Vec<(Query, String)>,
    /// Set by [`Engine::force`]: the value must be recomputed whatever the
    /// dependencies say, because the state it reads was replaced.
    forced: bool,
}

/// One evaluation in progress: the key being evaluated and the
/// dependencies it has read so far.
struct Frame {
    key: Query,
    deps: Vec<(Query, String)>,
}

/// The memo table and the revision counter.
pub struct Engine<V: QueryValue> {
    revision: Revision,
    memo: BTreeMap<Query, Memo<V>>,
    frames: Vec<Frame>,
    report: Report,
    /// Evaluations in the current round, so a runaway graph fails with a
    /// number instead of hanging a build.
    executed_this_round: usize,
    budget: usize,
}

impl<V: QueryValue> Default for Engine<V> {
    fn default() -> Self {
        Engine::new()
    }
}

impl<V: QueryValue> Engine<V> {
    pub fn new() -> Engine<V> {
        Engine {
            revision: 1,
            memo: BTreeMap::new(),
            frames: Vec::new(),
            report: Report::default(),
            executed_this_round: 0,
            budget: 8_000_000,
        }
    }

    pub fn revision(&self) -> Revision {
        self.revision
    }

    pub fn report(&self) -> &Report {
        &self.report
    }

    /// Take the report, leaving a fresh one. A round's counters and its
    /// cutoff and changed lists are what one rebuild did.
    pub fn take_report(&mut self) -> Report {
        self.executed_this_round = 0;
        let mut out = std::mem::take(&mut self.report);
        out.revision = self.revision;
        out
    }

    pub fn memo_len(&self) -> usize {
        self.memo.len()
    }

    /// The dependencies the last evaluation of `key` recorded.
    pub fn dependencies_of(&self, key: &Query) -> Option<&[(Query, String)]> {
        self.memo.get(key).map(|m| m.deps.as_slice())
    }

    pub fn is_memoized(&self, key: &Query) -> bool {
        self.memo.contains_key(key)
    }

    /// A memo entry's value, fingerprint, and the revision it last changed
    /// at (a probe and the store's writer read these).
    pub fn memo(&self, key: &Query) -> Option<(&V, &str, Revision)> {
        self.memo
            .get(key)
            .map(|m| (&m.value, m.fingerprint.as_str(), m.changed_at))
    }

    /// Every memo entry, for a writer that files them all.
    pub fn entries(&self) -> impl Iterator<Item = (&Query, &V, Revision, &[(Query, String)])> {
        self.memo
            .iter()
            .map(|(k, m)| (k, &m.value, m.changed_at, m.deps.as_slice()))
    }

    /// Set a named input. Returns whether it bumped the revision, which
    /// happens exactly when the value's fingerprint changed: setting an
    /// input to what it already was invalidates nothing.
    pub fn set_input(&mut self, name: &str, value: V) -> bool {
        let key = Query::Input(name.to_string());
        let fingerprint = value.fingerprint();
        if let Some(m) = self.memo.get_mut(&key) {
            if m.fingerprint == fingerprint {
                m.verified_at = self.revision;
                return false;
            }
        }
        self.revision += 1;
        let changed_at = self.revision;
        self.memo.insert(
            key,
            Memo {
                value,
                fingerprint,
                verified_at: changed_at,
                changed_at,
                deps: Vec::new(),
                forced: false,
            },
        );
        true
    }

    /// Every input's `(name, fingerprint)` the memo holds, sorted: the
    /// record of what the memo was computed against.
    pub fn inputs(&self) -> Vec<(String, String)> {
        self.memo
            .iter()
            .filter_map(|(k, m)| match k {
                Query::Input(name) => Some((name.clone(), m.fingerprint.clone())),
                _ => None,
            })
            .collect()
    }

    /// Install a memo entry read back from the store. The entry is stale
    /// by construction (`verified_at` is zero, and revisions start at one),
    /// so the next demand verifies it against the current inputs instead
    /// of trusting it. The store is advisory, and this is where that is
    /// enforced.
    pub fn restore(
        &mut self,
        key: Query,
        value: V,
        changed_at: Revision,
        deps: Vec<(Query, String)>,
    ) {
        if self.memo.contains_key(&key) {
            return;
        }
        let fingerprint = value.fingerprint();
        self.memo.insert(
            key,
            Memo {
                value,
                fingerprint,
                verified_at: 0,
                changed_at,
                deps,
                forced: false,
            },
        );
    }

    /// Mark these entries as needing a recomputation even if every
    /// recorded dependency still holds. A client calls this when the
    /// state a query reads has been replaced by a fresh computation (a
    /// re-checked module), where the inputs may legitimately be identical
    /// while the value is new. The prior fingerprint is kept, so the
    /// recomputation still reports a cutoff when it reproduces the value.
    pub fn force(&mut self, keys: impl IntoIterator<Item = Query>) {
        for key in keys {
            if let Some(m) = self.memo.get_mut(&key) {
                m.verified_at = 0;
                m.forced = true;
            }
        }
    }

    /// Every memo entry that recorded a dependency on something in
    /// `seeds`, transitively, over the dependencies the evaluations
    /// actually read. This is the recorded-reads dirty set, and it is
    /// what makes the graph precise: an item that mentions a changed
    /// declaration in a comment, or references it only from a body whose
    /// fingerprint the change cannot move, is not in it, while an item
    /// whose instance search consulted a bucket whose answer moved is.
    pub fn stale_from(&self, seeds: &BTreeSet<Query>) -> BTreeSet<Query> {
        // The reverse of the recorded dependency edges.
        let mut dependents: BTreeMap<&Query, Vec<&Query>> = BTreeMap::new();
        for (key, memo) in &self.memo {
            for (dep, _) in &memo.deps {
                dependents.entry(dep).or_default().push(key);
            }
        }
        let mut stale: BTreeSet<Query> = BTreeSet::new();
        let mut work: Vec<Query> = seeds.iter().cloned().collect();
        while let Some(q) = work.pop() {
            for next in dependents.get(&q).into_iter().flatten() {
                if stale.insert((*next).clone()) {
                    work.push((*next).clone());
                }
            }
        }
        stale
    }

    fn record(&mut self, key: &Query, fingerprint: String) {
        if let Some(frame) = self.frames.last_mut() {
            if frame.deps.iter().any(|(k, _)| k == key) {
                return;
            }
            frame.deps.push((key.clone(), fingerprint));
        }
    }

    fn cycle(&self, at: usize, key: &Query) -> Error {
        let mut chain: Vec<String> = self.frames[at..]
            .iter()
            .map(|f| format!("{:?}", f.key))
            .collect();
        chain.push(format!("{key:?}"));
        Error::plain(format!("query cycle: {}", chain.join(" -> ")))
    }
}

/// Demand one query. The subject is asked only when the memo has no entry
/// valid for the current revision, and every read the evaluation performs
/// is recorded against the enclosing evaluation.
pub fn get<S: Subject<V>, V: QueryValue>(
    engine: &mut Engine<V>,
    subject: &mut S,
    key: &Query,
) -> Result<V> {
    if let Some(at) = engine.frames.iter().position(|f| &f.key == key) {
        return Err(engine.cycle(at, key));
    }
    // Inputs are leaves: no subject computes them.
    if let Query::Input(_) = key {
        let Some(m) = engine.memo.get(key) else {
            return Err(Error::plain(format!(
                "query graph: the input {key:?} was never set, so nothing can depend on it"
            )));
        };
        let (value, fingerprint) = (m.value.clone(), m.fingerprint.clone());
        engine.record(key, fingerprint);
        return Ok(value);
    }
    // A live entry needs no walk.
    if let Some(m) = engine.memo.get(key) {
        if m.verified_at == engine.revision {
            let (value, fingerprint) = (m.value.clone(), m.fingerprint.clone());
            engine.report.hits += 1;
            engine.record(key, fingerprint);
            return Ok(value);
        }
    }
    // A stale entry is verified by re-reading its dependencies: each one is
    // demanded (which may itself verify or re-execute), and the entry holds
    // only when every fingerprint is the one it was written with.
    // Fingerprint equality is exactly as precise as the values themselves,
    // so this needs no `maybe_changed_after` heuristic.
    let forced = engine.memo.get(key).map(|m| m.forced).unwrap_or(false);
    if let Some(deps) = engine.memo.get(key).map(|m| m.deps.clone()).filter(|_| !forced) {
        let mut holds = true;
        for (dep, was) in &deps {
            if get(engine, subject, dep)?.fingerprint() != *was {
                holds = false;
                break;
            }
        }
        if holds {
            let mut out = None;
            if let Some(m) = engine.memo.get_mut(key) {
                m.verified_at = engine.revision;
                out = Some((m.value.clone(), m.fingerprint.clone()));
            }
            if let Some((value, fingerprint)) = out {
                engine.report.verified += 1;
                engine.record(key, fingerprint);
                return Ok(value);
            }
        }
    }
    // Execute.
    if engine.executed_this_round >= engine.budget {
        return Err(Error::plain(format!(
            "query graph: {} evaluations in one round, which is the budget; \
             the graph is not converging",
            engine.budget
        )));
    }
    engine.executed_this_round += 1;
    engine.frames.push(Frame {
        key: key.clone(),
        deps: Vec::new(),
    });
    let evaluated = subject.evaluate(engine, key);
    let frame = engine.frames.pop().expect("the frame just pushed");
    let value = evaluated?;
    let fingerprint = value.fingerprint();
    let prior = engine
        .memo
        .get(key)
        .map(|m| (m.fingerprint.clone(), m.changed_at));
    let (changed_at, cut) = match &prior {
        Some((old, at)) if *old == fingerprint => (*at, true),
        _ => (engine.revision, false),
    };
    match &prior {
        Some(_) if cut => engine.report.cutoff.push(key.clone()),
        Some(_) => engine.report.changed.push(key.clone()),
        None => {}
    }
    engine.report.executed += 1;
    engine.memo.insert(
        key.clone(),
        Memo {
            value: value.clone(),
            fingerprint: fingerprint.clone(),
            verified_at: engine.revision,
            changed_at,
            deps: frame.deps,
            forced: false,
        },
    );
    engine.record(key, fingerprint);
    Ok(value)
}

// ============================================================ the compiler's queries

/// One module as the queries see it: its source identity, its checked
/// contract, and its checked units. Borrowed, so analysis costs no copy.
#[derive(Clone, Copy)]
pub struct ModuleView<'a> {
    pub source_hash: &'a str,
    pub iface: &'a Iface,
    pub fns: &'a [FnDef],
}

/// The reference dependencies of one item: what its contract mentions
/// (`surface`) and what its body references (`body`).
#[derive(Clone, Debug, Default)]
pub struct Deps {
    pub surface: Vec<String>,
    pub body: Vec<String>,
}

/// One instance-search row the checker recorded: the declaration whose
/// check searched the bucket, the class, and the head symbol (`"*"` when
/// the head was not determined). A row is recorded whether or not the
/// search found an instance, which is what makes the negative case a
/// dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchRow {
    pub decl: String,
    pub class: String,
    pub head: String,
}

/// The subject: the checked state and the emission, in the shape the
/// queries read.
pub struct QueryState<'a> {
    pub modules: BTreeMap<String, ModuleView<'a>>,
    /// Item id to its reference dependencies.
    pub deps: BTreeMap<String, Deps>,
    /// Item id to the instance-search buckets its check consulted.
    pub searches: BTreeMap<String, Vec<(String, String)>>,
    /// Class to its implicit instances: `(item id, normalized head)`.
    pub instances: BTreeMap<String, Vec<(String, String)>>,
    /// Module to the item ids it owns (including its module node).
    pub items: BTreeMap<String, Vec<String>>,
    /// Contract fact node id to the fact it states. A fact node's
    /// signature is the fact, and it reads nothing back into its owner:
    /// the owner's own signature already reads the fact, so reading back
    /// would be a cycle with no information in it.
    pub facts: BTreeMap<String, String>,
    /// The emitted IR text of each codegen unit, by symbol.
    pub units: BTreeMap<String, String>,
    /// The optimizer's per-unit decision facts, rendered.
    pub inline: BTreeMap<String, String>,
}

impl<'a> QueryState<'a> {
    /// Build the state from a checked program and its emission. `deps` and
    /// `searches` come from the graph builder and the checker's recorder;
    /// the rest is derived here so one call site assembles it.
    pub fn of(
        modules: BTreeMap<String, ModuleView<'a>>,
        program: &Program,
        deps: BTreeMap<String, Deps>,
        searches: BTreeMap<String, Vec<(String, String)>>,
    ) -> QueryState<'a> {
        let mut instances: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
        for inst in &program.instances {
            let head = crate::driver::incremental::instance_head(&inst.id.0, &modules);
            instances
                .entry(inst.class.clone())
                .or_default()
                .push((format!("i:{}", inst.id.0), head));
        }
        for rows in instances.values_mut() {
            rows.sort();
            rows.dedup();
        }
        let mut items: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (module, view) in &modules {
            let mut ids = vec![format!("m:{module}.")];
            ids.extend(crate::driver::incremental::item_ids_from(view.iface, module));
            items.insert(module.clone(), ids);
        }
        QueryState {
            modules,
            deps,
            searches,
            instances,
            items,
            facts: BTreeMap::new(),
            units: BTreeMap::new(),
            inline: BTreeMap::new(),
        }
    }

    /// Record the emission: the per-unit IR text and the optimizer's
    /// per-unit facts. Both are what `Object` and `OptSummary` answer
    /// from, so a state without them answers those two queries with empty
    /// text, which is honest (nothing was emitted) and not a stub.
    pub fn set_emission(
        &mut self,
        units: BTreeMap<String, String>,
        inline: BTreeMap<String, String>,
    ) {
        self.units = units;
        self.inline = inline;
    }

    /// The module an item id belongs to (the longest module prefix).
    fn module_of(&self, item: &str) -> Option<String> {
        let name = crate::driver::incremental::name_of_id(item);
        let mut best: Option<&String> = None;
        for m in self.modules.keys() {
            if name == m.as_str() || name.starts_with(&format!("{m}.")) {
                if best.map(|b: &String| m.len() > b.len()).unwrap_or(true) {
                    best = Some(m);
                }
            }
        }
        best.cloned()
    }

    /// The view of the module that owns `item`, copied out with its own
    /// `'a` lifetime: reading a dependency needs `&mut self`, so the view
    /// cannot keep borrowing it.
    fn view(&self, item: &str) -> Result<(ModuleView<'a>, String)> {
        let module = self
            .module_of(item)
            .ok_or_else(|| Error::plain(format!("query graph: no module owns the item {item}")))?;
        let view = self
            .modules
            .get(&module)
            .ok_or_else(|| Error::plain(format!("query graph: no view for module {module}")))?;
        Ok((*view, module))
    }

    /// The unit symbols a declaration owns: its own symbol, the units
    /// under it (methods, hoisted closures), and its dictionary factory.
    fn owned_units(&self, module: &str, item: &str, fns: &[FnDef]) -> Vec<String> {
        let name = crate::driver::incremental::name_of_id(item);
        let prefix = format!("{name}.");
        let mut out: Vec<String> = Vec::new();
        for f in fns {
            if f.id.0 == name || f.id.0.starts_with(&prefix) {
                out.push(f.id.0.clone());
            }
        }
        // An instance's dictionary factory and methods are spelled
        // `<Module>.<instance id>` with the instance's own identity, which
        // the `name` above already is; the factory adds `.mk`.
        if crate::driver::incremental::kind_of_id(item)
            == crate::driver::incremental::ItemKind::Instance
        {
            let mk = format!("{name}.mk");
            if !out.contains(&mk) {
                out.push(mk);
            }
        }
        let _ = module;
        out.sort();
        out.dedup();
        out
    }
}

/// The head symbol of a rendered instance target: its first identifier,
/// normalized (see [`normalize_head`]). `None` when the target does not
/// start with one (a metavariable, a product).
pub fn head_symbol(target: &str) -> Option<String> {
    let head: String = target
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphanumeric() || matches!(c, '_' | '-' | '\'' | '.'))
        .collect();
    if head.is_empty() || head.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(true) {
        return None;
    }
    Some(normalize_head(&head))
}

/// The comparison form of a head symbol. A checked type identity is
/// spelled `Module.Count` and an interface target `Count`; comparing the
/// last segment makes the two agree. Two modules' same-named types then
/// share a bucket, which only makes the dependency coarser: a change
/// re-executes more, never less.
pub fn normalize_head(head: &str) -> String {
    head.rsplit('.').next().unwrap_or(head).to_string()
}

impl<'a> Subject<Value> for QueryState<'a> {
    fn evaluate(&mut self, engine: &mut Engine<Value>, key: &Query) -> Result<Value> {
        match key {
            Query::Input(_) => Err(Error::plain(format!(
                "query graph: the input {key:?} has no evaluation"
            ))),
            Query::Signature(item) => {
                // A contract fact node's contract is the fact itself.
                if crate::driver::incremental::kind_of_id(item)
                    == crate::driver::incremental::ItemKind::Contract
                {
                    let fact = self.facts.get(item).cloned().ok_or_else(|| {
                        Error::plain(format!("query graph: no fact recorded for {item}"))
                    })?;
                    let module = self
                        .module_of(item)
                        .ok_or_else(|| Error::plain(format!("query graph: no module owns {item}")))?;
                    get(engine, self, &Query::source(&module))?;
                    return Ok(Value::Text(fact));
                }
                let (view, module) = self.view(item)?;
                let iface = view.iface;
                get(engine, self, &Query::source(&module))?;
                // A module node's contract is the whole module's surface,
                // so it reads every item it owns and every module it
                // imports. That is what makes a change reach an importer
                // that names none of the changed declarations itself, which
                // is the `.cinchif` chain's own rule.
                if crate::driver::incremental::kind_of_id(item)
                    == crate::driver::incremental::ItemKind::Module
                {
                    for own in self.items.get(&module).cloned().unwrap_or_default() {
                        if &own != item {
                            get(engine, self, &Query::Signature(own))?;
                        }
                    }
                    for import in &iface.imports {
                        if self.modules.contains_key(&import.module) {
                            get(
                                engine,
                                self,
                                &Query::Signature(format!("m:{}.", import.module)),
                            )?;
                        }
                    }
                }
                let surface = self
                    .deps
                    .get(item)
                    .map(|d| d.surface.clone())
                    .unwrap_or_default();
                for dep in &surface {
                    if dep != item {
                        get(engine, self, &Query::Signature(dep.clone()))?;
                    }
                }
                // A contract projection carries the item's total-body
                // rows, so it reads the closure that produced them.
                get(engine, self, &Query::ProofClosure(item.clone()))?;
                Ok(Value::Text(crate::driver::incremental::surface_text(
                    iface, item,
                )))
            }
            Query::ElaboratedBody(item) => {
                let (view, module) = self.view(item)?;
                let fns = view.fns;
                get(engine, self, &Query::Signature(item.clone()))?;
                get(engine, self, &Query::source(&module))?;
                let body = self.deps.get(item).map(|d| d.body.clone()).unwrap_or_default();
                for dep in &body {
                    if dep != item {
                        // A body reference reads the callee's CONTRACT and
                        // not its body: the checker elaborates a call from
                        // the callee's scheme, and a dependency is an
                        // interface (D3). This is the split that makes
                        // interface-first pipelining possible: the body
                        // layer never gates the signature layer, so a
                        // module's contract is usable the moment it exists
                        // and the optimizer is the only layer that reads
                        // bodies.
                        get(engine, self, &Query::Signature(dep.clone()))?;
                    }
                }
                for (class, head) in self.searches.get(item).cloned().unwrap_or_default() {
                    get(engine, self, &Query::InstanceBucket(class, head))?;
                }
                Ok(Value::Text(crate::driver::incremental::body_text(
                    &module, item, fns,
                )))
            }
            Query::ProofClosure(item) => {
                let (view, module) = self.view(item)?;
                let iface = view.iface;
                get(engine, self, &Query::source(&module))?;
                Ok(Value::Text(crate::driver::incremental::proof_text(
                    iface, item,
                )))
            }
            Query::InstanceBucket(class, head) => {
                get(engine, self, &Query::instances(class))?;
                let mut found: Vec<String> = Vec::new();
                for (item, inst_head) in self.instances.get(class).cloned().unwrap_or_default() {
                    if head == "*" || &inst_head == head {
                        found.push(item);
                    }
                }
                found.sort();
                // The candidates' contracts are read: an instance whose
                // signature changes can change which candidate serves the
                // head (its context, its implicitness), so the answer
                // depends on them and not only on the input's id list.
                for item in &found {
                    get(engine, self, &Query::Signature(item.clone()))?;
                }
                Ok(Value::Instances(found))
            }
            Query::OptSummary(item) => {
                let (view, module) = self.view(item)?;
                let fns = view.fns;
                get(engine, self, &Query::ElaboratedBody(item.clone()))?;
                // The optimizer is the layer that reads bodies: an
                // inlined callee's body is copied into this unit, so the
                // summary depends on the bodies this unit references.
                for dep in self.deps.get(item).map(|d| d.body.clone()).unwrap_or_default() {
                    if dep != *item {
                        get(engine, self, &Query::ElaboratedBody(dep))?;
                    }
                }
                let mut rows: Vec<String> = Vec::new();
                for symbol in self.owned_units(&module, item, fns) {
                    if let Some(facts) = self.inline.get(&symbol) {
                        rows.push(format!("{symbol}={facts}"));
                    }
                }
                Ok(Value::Text(rows.join("\n")))
            }
            Query::Object(symbol) => {
                // A unit's IR is a function of the unit's own body and of
                // every body the optimizer copied into it, which is the
                // `OptSummary`'s dependency set. Reading the summary here
                // rather than the bodies keeps that set in one place.
                let owner = self
                    .owner_of_unit(symbol)
                    .ok_or_else(|| Error::plain(format!("query graph: no declaration owns the unit {symbol}")))?;
                get(engine, self, &Query::OptSummary(owner))?;
                Ok(Value::Text(
                    self.units.get(symbol).cloned().unwrap_or_default(),
                ))
            }
        }
    }
}

impl<'a> QueryState<'a> {
    /// The declaration a unit symbol belongs to: its exact item, else the
    /// longest item symbol it is a strict extension of (hoisted closures
    /// spell out `Unit.name`).
    fn owner_of_unit(&self, symbol: &str) -> Option<String> {
        let mut best: Option<(usize, String)> = None;
        for ids in self.items.values() {
            for id in ids {
                if matches!(
                    crate::driver::incremental::kind_of_id(id),
                    crate::driver::incremental::ItemKind::Module
                        | crate::driver::incremental::ItemKind::Contract
                ) {
                    continue;
                }
                let name = crate::driver::incremental::name_of_id(id);
                if symbol == name {
                    return Some(id.clone());
                }
                if symbol.starts_with(&format!("{name}.")) {
                    if best.as_ref().map(|(s, _)| name.len() > *s).unwrap_or(true) {
                        best = Some((name.len(), id.clone()));
                    }
                }
            }
        }
        best.map(|(_, id)| id)
    }
}

// ============================================================ the analysis

/// What one round of demand over the whole graph found.
#[derive(Clone, Debug, Default)]
pub struct Analysis {
    /// Items whose contract, body, or proof closure fingerprint changed:
    /// the change's real reach, which is what has to propagate.
    pub changed: BTreeSet<String>,
    /// Items that re-executed and produced the fingerprint they had
    /// before. Their dependents were reused, so these are the early
    /// cutoffs: work the change did not have to reach.
    pub cutoff: BTreeSet<String>,
    /// Declarations whose checks consulted a bucket whose answer moved:
    /// the searches that must run again, including the ones that found
    /// nothing before.
    pub researched: BTreeSet<String>,
    pub report: Report,
}

/// Set every input the graph reads, then demand every item's contract and
/// body. A memo entry that holds is not re-executed; one whose fingerprint
/// moves lands in `changed`.
///
/// The demand is the whole project on purpose. It is a traversal over memo
/// entries, not a re-check: the cost is one fingerprint comparison per
/// dependency edge, which is the "unchanged rebuild cost does not grow
/// with program size except for graph traversal" bound the design note
/// sets.
pub fn analyze(engine: &mut Engine<Value>, state: &mut QueryState<'_>) -> Result<Analysis> {
    engine.take_report();
    for (module, view) in &state.modules {
        engine.set_input(
            &format!("source:{module}"),
            Value::Text(view.source_hash.to_string()),
        );
    }
    for (class, instances) in &state.instances {
        let mut ids: Vec<String> = instances.iter().map(|(id, _)| id.clone()).collect();
        ids.sort();
        engine.set_input(&format!("instances:{class}"), Value::Instances(ids));
    }
    let mut items: Vec<String> = Vec::new();
    for ids in state.items.values() {
        items.extend(ids.iter().cloned());
    }
    items.sort();
    items.dedup();
    for item in &items {
        get(engine, state, &Query::Signature(item.clone()))?;
    }
    for item in &items {
        // A contract fact node is a stated fact, not code: it has no body
        // query, and its owner's body already reads it.
        if crate::driver::incremental::kind_of_id(item)
            == crate::driver::incremental::ItemKind::Contract
        {
            continue;
        }
        get(engine, state, &Query::ElaboratedBody(item.clone()))?;
    }
    let report = engine.take_report();
    let mut analysis = Analysis::default();
    for q in &report.changed {
        if let Some(item) = q.item() {
            analysis.changed.insert(item.to_string());
        }
        if let Query::InstanceBucket(class, head) = q {
            for (decl, rows) in &state.searches {
                if rows.iter().any(|(c, h)| c == class && h == head) {
                    analysis.researched.insert(decl.clone());
                }
            }
        }
    }
    for q in &report.cutoff {
        if let Some(item) = q.item() {
            analysis.cutoff.insert(item.to_string());
        }
    }
    analysis.report = report;
    Ok(analysis)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A subject with the shapes the engine has to get right: `a` reads the
    /// input `x` and returns a fixed text, `b` reads `a`, `c` reads a
    /// bucket, and `d` reads `x` directly.
    #[derive(Default)]
    struct Probe {
        bucket: Vec<String>,
        executed: Vec<String>,
    }

    impl Subject<Value> for Probe {
        fn evaluate(&mut self, engine: &mut Engine<Value>, key: &Query) -> Result<Value> {
            self.executed.push(format!("{key:?}"));
            match key {
                Query::Signature(name) if name == "a" => {
                    get(engine, self, &Query::Input("x".into()))?;
                    Ok(Value::Text("a".into()))
                }
                Query::Signature(name) if name == "b" => {
                    let a = get(engine, self, &Query::Signature("a".into()))?;
                    Ok(Value::Text(format!("b({})", a.text().unwrap_or(""))))
                }
                Query::Signature(name) if name == "c" => {
                    let bucket = get(engine, self, &Query::InstanceBucket("K".into(), "T".into()))?;
                    Ok(Value::Text(format!(
                        "c({})",
                        bucket.instances().unwrap_or(&[]).join(",")
                    )))
                }
                Query::Signature(name) if name == "d" => {
                    let x = get(engine, self, &Query::Input("x".into()))?;
                    Ok(Value::Text(format!("d({})", x.text().unwrap_or(""))))
                }
                Query::InstanceBucket(class, head) => {
                    get(engine, self, &Query::instances(class))?;
                    let found: Vec<String> = self
                        .bucket
                        .iter()
                        .filter(|id| head == "*" || id.ends_with(head))
                        .cloned()
                        .collect();
                    Ok(Value::Instances(found))
                }
                other => Err(Error::plain(format!("probe: no query {other:?}"))),
            }
        }
    }

    fn demand(engine: &mut Engine<Value>, probe: &mut Probe, key: &Query) -> String {
        let v = get(engine, probe, key).expect("query");
        v.text().unwrap_or_default().to_string()
    }

    fn fresh() -> Engine<Value> {
        let mut engine = Engine::new();
        engine.set_input("x", Value::Text("1".into()));
        engine.set_input("instances:K", Value::Instances(vec![]));
        engine
    }

    #[test]
    fn an_unchanged_input_reuses_everything() {
        let mut engine = fresh();
        let mut probe = Probe::default();
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("b".into())), "b(a)");
        let first = probe.executed.len();
        engine.take_report();
        assert!(
            !engine.set_input("x", Value::Text("1".into())),
            "the same fingerprint bumps no revision"
        );
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("b".into())), "b(a)");
        assert_eq!(probe.executed.len(), first, "nothing re-executed: {:?}", probe.executed);
        let report = engine.take_report();
        assert_eq!(report.executed, 0, "{report:?}");
        assert_eq!(report.hits, 1, "b is live at the revision: {report:?}");
    }

    #[test]
    fn a_cutoff_reuses_the_dependent() {
        let mut engine = fresh();
        let mut probe = Probe::default();
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("b".into())), "b(a)");
        engine.take_report();
        probe.executed.clear();
        assert!(engine.set_input("x", Value::Text("2".into())));
        // `a` re-executes (its dependency moved) and returns the same text,
        // so `b` is never re-executed. That is early cutoff.
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("b".into())), "b(a)");
        assert_eq!(probe.executed.len(), 1, "only `a`: {:?}", probe.executed);
        assert!(probe.executed[0].contains("\"a\""), "{:?}", probe.executed);
        let report = engine.take_report();
        assert_eq!(report.executed, 1, "{report:?}");
        assert_eq!(report.changed.len(), 0, "{report:?}");
        assert_eq!(report.cutoff.len(), 1, "{report:?}");
        assert_eq!(report.cutoff[0], Query::Signature("a".into()));
    }

    #[test]
    fn a_changed_fingerprint_reaches_the_dependent() {
        let mut engine = fresh();
        let mut probe = Probe::default();
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("b".into())), "b(a)");
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("d".into())), "d(1)");
        engine.take_report();
        probe.executed.clear();
        assert!(engine.set_input("x", Value::Text("2".into())));
        // `d` reads `x` into its value, so its fingerprint moves.
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("d".into())), "d(2)");
        let report = engine.take_report();
        assert_eq!(report.changed, vec![Query::Signature("d".into())], "{report:?}");
        assert!(report.cutoff.is_empty(), "{report:?}");
    }

    #[test]
    fn a_negative_lookup_is_a_dependency() {
        let mut engine = fresh();
        let mut probe = Probe::default();
        // No instance: the negative answer is cached and `c` records it.
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("c".into())), "c()");
        engine.take_report();
        probe.executed.clear();
        // Nothing changed: the miss holds and `c` is not re-executed.
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("c".into())), "c()");
        assert!(probe.executed.is_empty(), "{:?}", probe.executed);
        // An instance appears: the miss is stale, so `c` must re-execute.
        probe.bucket = vec!["Lib.InstT".into()];
        assert!(engine.set_input(
            "instances:K",
            Value::Instances(vec!["Lib.InstT".into()])
        ));
        assert_eq!(
            demand(&mut engine, &mut probe, &Query::Signature("c".into())),
            "c(Lib.InstT)"
        );
        let report = engine.take_report();
        assert!(
            report
                .changed
                .contains(&Query::InstanceBucket("K".into(), "T".into())),
            "{report:?}"
        );
        assert!(report.changed.contains(&Query::Signature("c".into())), "{report:?}");
    }

    #[test]
    fn an_instance_elsewhere_keeps_the_miss_valid() {
        let mut engine = fresh();
        let mut probe = Probe::default();
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("c".into())), "c()");
        engine.take_report();
        probe.executed.clear();
        probe.bucket = vec!["Lib.InstOther".into()];
        assert!(engine.set_input(
            "instances:K",
            Value::Instances(vec!["Lib.InstOther".into()])
        ));
        // The class input moved, so the bucket for head `T` re-executes,
        // but its answer did not: the miss still holds and `c`, which
        // reads the bucket, is not re-executed. That is the cutoff doing
        // its job one level in.
        assert_eq!(demand(&mut engine, &mut probe, &Query::Signature("c".into())), "c()");
        assert_eq!(
            probe.executed,
            vec![format!("{:?}", Query::InstanceBucket("K".into(), "T".into()))],
            "the bucket re-executes and `c` does not"
        );
        let report = engine.take_report();
        assert_eq!(
            report.cutoff,
            vec![Query::InstanceBucket("K".into(), "T".into())],
            "{report:?}"
        );
        assert!(report.changed.is_empty(), "{report:?}");
    }

    #[test]
    fn a_cycle_is_an_error_naming_the_chain() {
        #[derive(Default)]
        struct Loop;
        impl Subject<Value> for Loop {
            fn evaluate(&mut self, engine: &mut Engine<Value>, key: &Query) -> Result<Value> {
                let next = match key {
                    Query::Signature(n) if n == "a" => Query::Signature("b".into()),
                    _ => Query::Signature("a".into()),
                };
                get(engine, self, &next)
            }
        }
        let mut engine = Engine::new();
        let mut subject = Loop;
        let error = get(&mut engine, &mut subject, &Query::Signature("a".into()))
            .expect_err("a cycle");
        assert!(error.message.contains("query cycle"), "{}", error.message);
        assert!(
            error.message.contains("Signature(\"a\")"),
            "{}",
            error.message
        );
    }

    #[test]
    fn a_restored_entry_is_verified_not_trusted() {
        // The recorded dependency still holds: the stored value is used.
        let mut engine = fresh();
        let mut probe = Probe::default();
        engine.restore(
            Query::Signature("a".into()),
            Value::Text("from the store".into()),
            1,
            vec![(
                Query::Input("x".into()),
                Value::Text("1".into()).fingerprint(),
            )],
        );
        assert_eq!(
            demand(&mut engine, &mut probe, &Query::Signature("a".into())),
            "from the store"
        );
        assert!(probe.executed.is_empty(), "{:?}", probe.executed);
        // The recorded dependency moved: the stored value is recomputed.
        let mut engine = fresh();
        engine.set_input("x", Value::Text("2".into()));
        let mut probe = Probe::default();
        engine.restore(
            Query::Signature("a".into()),
            Value::Text("from the store".into()),
            1,
            vec![(
                Query::Input("x".into()),
                Value::Text("1".into()).fingerprint(),
            )],
        );
        assert_eq!(
            demand(&mut engine, &mut probe, &Query::Signature("a".into())),
            "a"
        );
        assert_eq!(probe.executed.len(), 1, "{:?}", probe.executed);
    }

    #[test]
    fn staleness_walks_the_reads_and_not_the_names() {
        let mut engine = fresh();
        let mut probe = Probe::default();
        demand(&mut engine, &mut probe, &Query::Signature("b".into()));
        demand(&mut engine, &mut probe, &Query::Signature("c".into()));
        // `x` moved: `a` read it and `b` read `a`. `c` read a bucket and
        // no input, so it is untouched.
        let mut seeds = BTreeSet::new();
        seeds.insert(Query::Input("x".into()));
        let stale = engine.stale_from(&seeds);
        assert!(stale.contains(&Query::Signature("a".into())), "{stale:?}");
        assert!(stale.contains(&Query::Signature("b".into())), "{stale:?}");
        assert!(!stale.contains(&Query::Signature("c".into())), "{stale:?}");
        // A bucket whose answer moved reaches only its readers.
        let mut seeds = BTreeSet::new();
        seeds.insert(Query::instances("K"));
        let stale = engine.stale_from(&seeds);
        assert!(
            stale.contains(&Query::InstanceBucket("K".into(), "T".into())),
            "{stale:?}"
        );
        assert!(stale.contains(&Query::Signature("c".into())), "{stale:?}");
        assert!(!stale.contains(&Query::Signature("b".into())), "{stale:?}");
    }

    #[test]
    fn head_symbols_normalize_to_their_last_segment() {
        assert_eq!(head_symbol("Count"), Some("Count".into()));
        assert_eq!(head_symbol("Data.Count"), Some("Count".into()));
        assert_eq!(head_symbol("List Int"), Some("List".into()));
        assert_eq!(head_symbol("?m1"), None);
        assert_eq!(head_symbol("(Int, Int)"), None);
        assert_eq!(normalize_head("Base.Count"), "Count");
    }
}

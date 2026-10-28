//! P29: Blaze-style incremental recompilation (design/incremental-recompilation.md).
//!
//! Three cooperating layers, mirroring REAPER, INCR, and LINK:
//!
//! * REAPER equivalent: [`ItemGraph`], an on-demand whole-program item
//!   graph (modules, values, types, classes, instances, contract fact
//!   nodes) with edges labeled by kind ([`EdgeKind::Surface`] for
//!   type/contract dependences, [`EdgeKind::Body`] for body/call
//!   dependences) and direction (each edge indexes both its dependents
//!   and its dependencies). Nodes and surface edges come from the
//!   `.cinchif` interface metadata and its contract facts (assertion,
//!   temporal, and law rows); body edges come from the checked
//!   declarations' name references, resolved against the same metadata.
//! * INCR equivalent: [`Session`], a persistent recompilation session
//!   with declaration-level dirty sets. A body-only change rechecks the
//!   changed declaration alone; a signature or contract change rechecks
//!   the dependents' closure ([`ItemGraph::dirty_closure`]). The recheck
//!   runs the shared whole-program checker over the minimal module slice
//!   (dirty modules plus their dependency closure, the name-resolution
//!   context) and the report states what was actually touched:
//!   `rechecked` (the dirty declarations), `retyped` (declarations the
//!   slice run re-typed as context), and `kept`.
//! * LINK equivalent: [`Session::link_units`], a declaration-hash-keyed
//!   object cache over the emitted codegen units. Each unit object is
//!   keyed on its owning declaration's [`Item::decl_hash`]; a rebuild
//!   re-places only the units whose key or bytes changed and reuses the
//!   rest. The linked artifact is assembled from the cache objects and
//!   verified against a fresh emission, so deleting the cache costs
//!   time, never correctness (the advisory rule; byte-identity is
//!   pinned by `tests/incremental.rs`).
//!
//! Honesty discipline: latency claims are measured, never assumed.
//! [`SessionReport::timings`] carries real numbers per rebuild. This
//! bootstrap's codegen emits one LLVM module per program
//! (`codegen::emit_for` is whole-program), so unit extraction for the
//! object cache follows a whole-program emission; the cache still keys,
//! diffs, and assembles at declaration granularity and the assembly is
//! verified against the emission. Per-unit emission is a codegen
//! follow-up outside this module's fence.

use crate::ast::{self, Decl, RefKind, RefSite};
use crate::check::{
    DataImport, LitFold, SafetyAssertion, SafetyReport, StageInput, StageMode, StageReport,
    StagingCx, TemporalRow,
};
use crate::core::{
    CtorLayout, FnDef, ForeignPlan, InstanceDef, LawRow, PrimPlan, Program, RewriteRule,
    TargetProfile,
};
use crate::interface::{self, Iface};
use crate::parser::FixityTable;
use crate::{Error, Result};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use super::{
    load_project, parse_prelude, parse_source, wit_for_source, write_artifacts, PreludeSources,
    Project,
};

// ============================================================ REAPER: the item graph

/// The namespace an item lives in (values and types resolve apart).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemKind {
    /// A module's own surface (its `.cinchif` contract projection).
    Module,
    /// An exported value, constructor, or method unit's declaration.
    Value,
    /// A declared type (data, record, codata, opaque, refinement).
    Type,
    /// A class (`signature`) declaration.
    Class,
    /// An instance declaration.
    Instance,
    /// A contract fact node: one assertion, temporal contract, or law
    /// row of a definition (the P20/P28 contract facts).
    Contract,
}

impl ItemKind {
    fn tag(self) -> &'static str {
        match self {
            ItemKind::Module => "m",
            ItemKind::Value => "v",
            ItemKind::Type => "t",
            ItemKind::Class => "c",
            ItemKind::Instance => "i",
            ItemKind::Contract => "k",
        }
    }

    fn from_tag(tag: &str) -> ItemKind {
        match tag {
            "m" => ItemKind::Module,
            "t" => ItemKind::Type,
            "c" => ItemKind::Class,
            "i" => ItemKind::Instance,
            "k" => ItemKind::Contract,
            _ => ItemKind::Value,
        }
    }
}

/// What part of the dependent a change in the dependency invalidates
/// (the edge kind the design note asks for).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeKind {
    /// The dependent's type/contract surface depends on the
    /// dependency's surface (schemes, instance heads, contract facts).
    Surface,
    /// The dependent's body references the dependency (calls,
    /// constructor uses, value references): code may embed code.
    Body,
}

/// How one item changed between two checked states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeKind {
    /// Body identity changed, surface identity held: recheck the
    /// declaration alone (the `.cinchif` chain does not stale).
    Body,
    /// Signature or contract facts changed: recheck the dependents'
    /// closure.
    Surface,
    /// The item is new to the program.
    Added,
    /// The item left the program.
    Removed,
}

/// A graph item's identity and hashes. `id` is `tag:Module.name`
/// (`v:Lib.double`, `t:Lib.Count`, `k:Lib.count-value#f0`, ...).
#[derive(Clone, Debug)]
pub struct Item {
    pub id: String,
    pub kind: ItemKind,
    pub module: String,
    pub name: String,
    /// Content hash of the item's contract projection (its `.cinchif`
    /// rows: scheme, contract facts, layouts, laws).
    pub surface_hash: String,
    /// Content hash of the item's checked body identity (its units'
    /// `FnDef` state, span-free).
    pub body_hash: String,
    /// Declaration identity: the content hash of `surface_hash` and
    /// `body_hash`. The object cache keys on this.
    pub decl_hash: String,
    /// The contract facts this item carries (normalized spellings): the
    /// labels the graph records.
    pub facts: Vec<String>,
    /// For a contract node: the item the fact is stated for.
    pub owner: Option<String>,
}

/// One labeled dependency edge: `from` depends on `to`.
#[derive(Clone, Debug)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub kind: EdgeKind,
}

/// The whole-program item graph (REAPER equivalent), built on demand
/// from the `.cinchif` metadata, its contract facts, and the checked
/// declarations' references.
#[derive(Clone, Debug, Default)]
pub struct ItemGraph {
    items: BTreeMap<String, Item>,
    edges: Vec<Edge>,
    /// Edge indices whose `to` is the key (the key's dependents).
    dependents: BTreeMap<String, Vec<usize>>,
    /// Edge indices whose `from` is the key (the key's dependencies).
    dependencies: BTreeMap<String, Vec<usize>>,
}

impl ItemGraph {
    pub fn item(&self, id: &str) -> Option<&Item> {
        self.items.get(id)
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.items.values()
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// The edges pointing at `id`: who depends on `id`.
    pub fn dependents_of(&self, id: &str) -> Vec<&Edge> {
        match self.dependents.get(id) {
            Some(ix) => ix.iter().map(|i| &self.edges[*i]).collect(),
            None => Vec::new(),
        }
    }

    /// The edges leaving `id`: what `id` depends on.
    pub fn dependencies_of(&self, id: &str) -> Vec<&Edge> {
        match self.dependencies.get(id) {
            Some(ix) => ix.iter().map(|i| &self.edges[*i]).collect(),
            None => Vec::new(),
        }
    }

    /// Every edge between two items (either direction), for probes.
    pub fn edges_between(&self, a: &str, b: &str) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|e| (e.from == a && e.to == b) || (e.from == b && e.to == a))
            .collect()
    }

    /// The dirty closure of a change set (INCR's rule): a body-only
    /// change marks its own declaration; a surface/contract change marks
    /// the dependents' closure (transitive over both edge kinds, since a
    /// changed signature can break any dependent's check and a changed
    /// contract fact can break any dependent's fact verification).
    /// Removed items seed their recorded dependents.
    pub fn dirty_closure(&self, changes: &[(String, ChangeKind)]) -> BTreeSet<String> {
        let mut dirty: BTreeSet<String> = BTreeSet::new();
        let mut work: Vec<String> = Vec::new();
        for (id, kind) in changes {
            match kind {
                ChangeKind::Body => {
                    dirty.insert(id.clone());
                }
                ChangeKind::Surface | ChangeKind::Added => {
                    if dirty.insert(id.clone()) {
                        work.push(id.clone());
                    }
                }
                ChangeKind::Removed => {
                    for e in self.dependents_of(id) {
                        if dirty.insert(e.from.clone()) {
                            work.push(e.from.clone());
                        }
                    }
                }
            }
        }
        while let Some(id) = work.pop() {
            for e in self.dependents_of(&id) {
                if dirty.insert(e.from.clone()) {
                    work.push(e.from.clone());
                }
            }
        }
        dirty
    }

    fn insert_item(&mut self, item: Item) {
        self.items.insert(item.id.clone(), item);
    }

    fn insert_edge(&mut self, from: &str, to: &str, kind: EdgeKind) {
        let idx = self.edges.len();
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
            kind,
        });
        self.dependents.entry(to.to_string()).or_default().push(idx);
        self.dependencies.entry(from.to_string()).or_default().push(idx);
    }
}

/// Render an item id from its parts.
pub fn item_id(kind: ItemKind, module: &str, name: &str) -> String {
    format!("{}:{module}.{name}", kind.tag())
}

/// Render a contract fact node's id (the owner is already
/// module-qualified).
pub fn contract_id(owner: &str, fact: &str) -> String {
    format!("k:{owner}#{fact}")
}

/// The kind encoded in a rendered item id.
pub fn kind_of_id(id: &str) -> ItemKind {
    ItemKind::from_tag(id.split(':').next().unwrap_or_default())
}

/// The module-qualified name inside a rendered item id (without the
/// kind tag or any `#fact` suffix).
pub fn name_of_id(id: &str) -> &str {
    let rest = id.split(':').nth(1).unwrap_or_default();
    rest.split('#').next().unwrap_or(rest)
}

// ============================================================ declaration identity

/// The canonical surface projection of one item (the `.cinchif` rows it
/// owns): span-free, so a moved declaration keeps its identity.
fn surface_rows(iface: &Iface, id: &str) -> String {
    let mut out = String::new();
    let module = iface.module.as_str();
    if kind_of_id(id) == ItemKind::Module {
        // The module node's surface is the whole contract projection.
        out.push_str(&iface.iface_hash());
        return out;
    }
    let bare = name_of_id(id);
    let local = bare
        .strip_prefix(&format!("{module}."))
        .unwrap_or(bare)
        .to_string();
    for v in &iface.exports.values {
        if local == v.name {
            out.push_str(&format!("v|{}|{}|{}|{}", v.name, v.scheme, v.symbol, v.arity));
        }
    }
    for t in &iface.exports.types {
        if local == t.name {
            out.push_str(&format!("t|{}|{:?}|{}|{:?}", t.name, t.params, t.kind, t.ctors));
            out.push_str(&format!("|fields={:?}", t.fields));
            for l in &iface.layouts {
                if l.name == t.name {
                    out.push_str(&format!("|layout={l:?}"));
                }
            }
        }
        for c in &t.ctors {
            if local == c.name {
                out.push_str(&format!("ctor|{}|{}|{:?}|{:?}", c.name, c.tag, c.fields, c.erased));
            }
        }
    }
    for s in &iface.exports.signatures {
        if local == s.name {
            out.push_str(&format!("c|{}|{:?}|{:?}", s.name, s.params, s.methods));
        }
    }
    for i in &iface.exports.instances {
        if local == i.id {
            out.push_str(&format!(
                "i|{}|{}|{}|{:?}|{:?}|{}",
                i.id, i.class, i.target, i.ctx, i.methods, i.implicit
            ));
        }
    }
    // Contract facts keyed by owner name.
    for a in &iface.assertions {
        if a.name == local {
            out.push_str(&format!("|assert={:?}", a.facts));
        }
    }
    for c in &iface.contracts {
        if c.name == local {
            out.push_str(&format!("|contract={}|{}|{:?}", c.kind, c.formula, c.assumptions));
        }
    }
    for l in &iface.laws {
        if l.instance == local {
            out.push_str(&format!("|law={}|{}|{}", l.class, l.law, l.status));
        }
    }
    if iface.must_use.iter().any(|n| *n == local) {
        out.push_str("|must-use");
    }
    out
}

/// The span-free body identity of one item: the checked `FnDef` state of
/// the units it owns (elaborated core plus hints; no source positions).
fn body_rows(fns: &[&FnDef]) -> String {
    let mut owned: Vec<&FnDef> = fns.to_vec();
    owned.sort_by(|a, b| a.id.0.cmp(&b.id.0));
    let mut out = String::new();
    for f in owned {
        out.push_str(&format!("{}={:?}\n", f.id.0, f));
    }
    out
}

fn hash_of(text: &str) -> String {
    interface::content_hash(text)
}

// ============================================================ reference extraction

/// The declared surface of one declaration: the type/contract references
/// a change there propagates through.
fn surface_refs(d: &Decl, out: &mut Vec<RefSite>) {
    match d {
        Decl::ValueSig { ty, .. } => ast::collect_type_refs(ty, out),
        Decl::Equation { .. } => {}
        Decl::Data(data) => {
            for c in &data.ctors {
                ast::collect_type_refs(&c.ty, out);
            }
        }
        Decl::Record(r) | Decl::Codata(r) => {
            for f in &r.fields {
                ast::collect_type_refs(&f.ty, out);
            }
        }
        Decl::Signature(s) => {
            for (_, ty) in &s.methods {
                ast::collect_type_refs(ty, out);
            }
            for (_, ty) in &s.laws {
                ast::collect_type_refs(ty, out);
            }
        }
        Decl::Instance(i) => {
            ast::collect_type_refs(&i.target, out);
            for c in &i.ctx {
                match c {
                    ast::Oblig::Class(c) => {
                        for a in &c.args {
                            ast::collect_type_refs(a, out);
                        }
                    }
                    ast::Oblig::Pred(t) => ast::collect_type_refs(t, out),
                    ast::Oblig::Preserves { pred, .. } => ast::collect_type_refs(pred, out),
                }
            }
        }
        Decl::Refine(r) => {
            for b in &r.params {
                if let Some(ann) = &b.ann {
                    ast::collect_type_refs(ann, out);
                }
            }
            ast::collect_type_refs(&r.base, out);
            ast::collect_type_refs(&r.pred, out);
        }
        Decl::Prim(p) => {
            ast::collect_type_refs(&p.ty, out);
            for b in &p.bodies {
                for u in &b.uses {
                    ast::collect_type_refs(&u.ty, out);
                }
            }
        }
        Decl::Upgrade(u) => {
            out.push(path_ref(RefKind::Type, &u.old.segments, u.old.span()));
            out.push(path_ref(RefKind::Type, &u.new.segments, u.new.span()));
        }
        Decl::Effect(e) => {
            for (_, ty) in &e.ops {
                ast::collect_type_refs(ty, out);
            }
        }
        Decl::Rewrite(r) => {
            for b in &r.binders {
                if let Some(ann) = &b.ann {
                    ast::collect_type_refs(ann, out);
                }
            }
            for c in &r.ctx {
                match c {
                    ast::Oblig::Class(c) => {
                        for a in &c.args {
                            ast::collect_type_refs(a, out);
                        }
                    }
                    ast::Oblig::Pred(t) => ast::collect_type_refs(t, out),
                    ast::Oblig::Preserves { pred, .. } => ast::collect_type_refs(pred, out),
                }
            }
        }
        Decl::Attributed { inner, .. } => surface_refs(inner, out),
        Decl::NameSpliced { inner, .. } => surface_refs(inner, out),
        Decl::Foreign(_)
        | Decl::OpaqueType(_)
        | Decl::Contract(_)
        | Decl::Import(_)
        | Decl::ImportData(_)
        | Decl::Export(_)
        | Decl::Fixity(_)
        | Decl::Generate { .. } => {}
    }
}

/// The body of one declaration: the term references whose change the
/// body depends on (calls, constructor uses, value references).
fn body_refs(d: &Decl, out: &mut Vec<RefSite>) {
    match d {
        Decl::Equation { eq, .. } => {
            ast::collect_term_refs(&eq.body, out);
            local_refs(&eq.locals, out);
        }
        Decl::Instance(i) => {
            for m in &i.methods {
                ast::collect_term_refs(&m.body, out);
                local_refs(&m.locals, out);
            }
        }
        Decl::Rewrite(r) => {
            ast::collect_term_refs(&r.match_tm, out);
            ast::collect_term_refs(&r.replace_tm, out);
            if let Some(req) = &r.require {
                ast::collect_term_refs(req, out);
            }
            ast::collect_term_refs(&r.justify, out);
        }
        Decl::Prim(p) => {
            for b in &p.bodies {
                if let Some(w) = &b.when {
                    ast::collect_term_refs(w, out);
                }
                if let Some(j) = &b.justify {
                    ast::collect_term_refs(j, out);
                }
            }
        }
        Decl::Upgrade(u) => {
            if let Some(m) = &u.migrate {
                ast::collect_term_refs(m, out);
            }
            if let Some(r) = &u.related {
                ast::collect_term_refs(r, out);
            }
        }
        Decl::Generate { stmts, .. } => {
            for s in stmts {
                match s {
                    ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                        ast::collect_term_refs(term, out)
                    }
                    ast::DoStmt::Let { decls, .. } => local_refs(decls, out),
                }
            }
        }
        Decl::NameSpliced { name_tm, inner } => {
            ast::collect_term_refs(name_tm, out);
            body_refs(inner, out);
        }
        Decl::Attributed { inner, .. } => body_refs(inner, out),
        Decl::ValueSig { .. }
        | Decl::Data(_)
        | Decl::Record(_)
        | Decl::Codata(_)
        | Decl::Signature(_)
        | Decl::Refine(_)
        | Decl::Foreign(_)
        | Decl::OpaqueType(_)
        | Decl::Effect(_)
        | Decl::Contract(_)
        | Decl::Import(_)
        | Decl::ImportData(_)
        | Decl::Export(_)
        | Decl::Fixity(_) => {}
    }
}

fn local_refs(locals: &[ast::LocalDecl], out: &mut Vec<RefSite>) {
    for l in locals {
        match l {
            ast::LocalDecl::Sig { ty, .. } => ast::collect_type_refs(ty, out),
            ast::LocalDecl::Eq { body, .. }
            | ast::LocalDecl::Bind { body, .. }
            | ast::LocalDecl::Lazy { body, .. } => ast::collect_term_refs(body, out),
        }
    }
}

fn path_ref(kind: RefKind, segments: &[ast::Name], span: crate::lexer::Span) -> RefSite {
    RefSite {
        kind,
        quals: segments[..segments.len().saturating_sub(1)]
            .iter()
            .map(|n| n.text.clone())
            .collect(),
        name: segments.last().map(|n| n.text.clone()).unwrap_or_default(),
        span,
    }
}

// ============================================================ the session

/// One module's cached checked state (user and prelude modules alike).
#[derive(Clone, Debug, Default)]
struct ModuleState {
    imports: Vec<String>,
    source_hash: String,
    iface: Option<Iface>,
    /// Checked function units (pre-optimization), in check order.
    fns: Vec<FnDef>,
    instances: Vec<InstanceDef>,
    laws: Vec<LawRow>,
    rewrites: Vec<RewriteRule>,
    prims: Vec<PrimPlan>,
    plans: BTreeMap<String, CtorLayout>,
    foreigns: BTreeMap<String, ForeignPlan>,
    /// Report rows this module contributed (merged into the manifest).
    generated: Vec<String>,
    literal_folds: Vec<LitFold>,
    data_imports: Vec<DataImport>,
    safety: Vec<SafetyAssertion>,
    temporal: Vec<TemporalRow>,
}

/// A recorded layout segment: the ids, in list order, the checker
/// emitted contiguously for one owner (module name, or empty for the
/// owner-less builtins the checker interleaves).
#[derive(Clone, Debug)]
struct Segment {
    owner: String,
    ids: Vec<String>,
}

/// Group an id list into maximal same-owner segments (the checker's
/// emission shape: one owner's units in check order, interleaved with
/// other owners only at segment boundaries).
fn segment_ids(ids: &[String], modules: &[String]) -> Vec<Segment> {
    let owner_of = |id: &str| -> String {
        let mut best = String::new();
        for m in modules {
            if id.starts_with(&format!("{m}.")) && m.len() > best.len() {
                best = m.clone();
            }
        }
        best
    };
    let mut segs: Vec<Segment> = Vec::new();
    for id in ids {
        let owner = owner_of(id);
        match segs.last_mut() {
            Some(s) if s.owner == owner => s.ids.push(id.clone()),
            _ => segs.push(Segment {
                owner,
                ids: vec![id.clone()],
            }),
        }
    }
    segs
}

/// One cache index row (the object cache keys on `decl_hash`).
#[derive(Clone, Debug)]
pub struct CacheRow {
    /// The owning declaration's item id.
    pub decl: String,
    /// The codegen unit's symbol.
    pub symbol: String,
    /// The declaration identity the object is keyed on.
    pub decl_hash: String,
    /// The object bytes' content hash.
    pub object_hash: String,
}

/// The session's link report: what the object cache did this rebuild.
#[derive(Clone, Debug, Default)]
pub struct LinkReport {
    /// Unit symbols re-placed into the cache (new, key changed, or byte
    /// drift).
    pub placed: Vec<String>,
    /// Unit symbols reused from the cache (key and bytes matched).
    pub reused: Vec<String>,
    /// Cache entries dropped (their unit left the program).
    pub dropped: Vec<String>,
    /// The assembled artifact matched a fresh emission byte for byte.
    pub verified: bool,
    /// The cache disagreed with the fresh emission and was repaired
    /// (every unit re-placed); the artifact still matches the emission.
    pub repaired: bool,
    /// The program has no `main` and no foreign exports (a library):
    /// there is no linked artifact to cache, and the artifact set is the
    /// interfaces plus the manifest.
    pub skipped: bool,
}

/// Real timings for one rebuild (milliseconds, wall clock).
#[derive(Clone, Debug, Default)]
pub struct Timings {
    pub parse_ms: f64,
    pub check_ms: f64,
    pub optimize_ms: f64,
    pub emit_ms: f64,
    pub link_ms: f64,
    pub total_ms: f64,
}

/// What one build or rebuild actually did (the driver's own report;
/// `tests/incremental.rs` asserts on these fields, not on timings).
#[derive(Clone, Debug, Default)]
pub struct SessionReport {
    /// Declarations whose identity changed and whose check was redone
    /// (the dirty closure).
    pub rechecked: Vec<String>,
    /// Declarations the slice run re-typed as name-resolution context
    /// but whose identity held (honest disclosure of the checker's
    /// whole-slice entry).
    pub retyped: Vec<String>,
    /// Declarations not touched at all.
    pub kept: Vec<String>,
    /// The user modules the checker ran over. The embedded prelude
    /// family is the implicit base layer of every slice run and is
    /// never listed here (it is hash-stable and never a change source).
    pub typed_modules: Vec<String>,
    /// The classified changes this rebuild responded to.
    pub changes: Vec<(String, ChangeKind)>,
    /// The merged `.cinchif` "checked against" chain verifies.
    pub chain_ok: bool,
    /// The session fell back to a full recheck (the cache fallback
    /// valve: staging inputs, `jpl` profile, instantiation units, new
    /// modules, or layout drift). Correctness first: the fast path
    /// never guesses.
    pub full_rebuild: bool,
    pub link: LinkReport,
    pub timings: Timings,
}

/// The persistent recompilation session (INCR equivalent).
pub struct Session {
    entry: PathBuf,
    root: PathBuf,
    entry_stem: String,
    out_dir: PathBuf,
    cache_dir: PathBuf,
    profile: TargetProfile,
    wit: Vec<crate::interface::WitAdapter>,
    /// Module name to declaring file key (user modules).
    module_file: BTreeMap<String, String>,
    /// Module name to import names (user modules).
    imports: BTreeMap<String, Vec<String>>,
    /// User modules in dependency order (the driver's project order:
    /// entry modules last).
    order: Vec<String>,
    entry_modules: Vec<String>,
    /// Prelude modules in check order (the embedded family).
    prelude_order: Vec<String>,
    /// File key to current source text.
    texts: BTreeMap<String, String>,
    /// File key to the content hash of the text last seen.
    text_hashes: BTreeMap<String, String>,
    /// Checked state for every module (user and prelude).
    states: BTreeMap<String, ModuleState>,
    /// Every module name (user plus prelude) for prefix attribution.
    all_modules: Vec<String>,
    /// The recorded emission layout (segment structure) per list, from
    /// the last full check or merge: the merge reproduces this order.
    layouts: Layouts,
    /// The last full run's raw lists (a full run is its own canonical
    /// order; no merge needed).
    run: Option<RunLists>,
    /// The entry's synthesized exit adapter id, when the entry module
    /// defines `main`.
    main_exit: Option<String>,
    /// Staging inputs recorded by the last check (non-empty forces full
    /// rebuilds: per-module input merging is ambiguous).
    staging_inputs: Vec<StageInput>,
    stage_mode: StageMode,
    graph: Option<ItemGraph>,
    /// The assembled program IR of the last artifact build (the LINK
    /// artifact, also written to `build/<entry>.ll`).
    ir: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct Layouts {
    fns: Vec<Segment>,
    instances: Vec<Segment>,
    laws: Vec<Segment>,
    rewrites: Vec<Segment>,
    prims: Vec<Segment>,
}

/// A checked run's raw global lists (pre-optimization).
#[derive(Clone, Debug, Default)]
struct RunLists {
    fns: Vec<FnDef>,
    instances: Vec<InstanceDef>,
    laws: Vec<LawRow>,
    rewrites: Vec<RewriteRule>,
    prims: Vec<PrimPlan>,
    plans: BTreeMap<String, CtorLayout>,
    foreigns: BTreeMap<String, ForeignPlan>,
    full: bool,
}

fn since_ms(t: std::time::Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

impl Session {
    /// Open the project at `entry` and run a full cold build.
    pub fn open(entry: &Path) -> Result<(Session, SessionReport)> {
        let root = entry
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        let entry_stem = entry
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let mut session = Session {
            entry: entry.to_path_buf(),
            root,
            entry_stem,
            out_dir: PathBuf::new(),
            cache_dir: PathBuf::new(),
            profile: TargetProfile::native(),
            wit: Vec::new(),
            module_file: BTreeMap::new(),
            imports: BTreeMap::new(),
            order: Vec::new(),
            entry_modules: Vec::new(),
            prelude_order: Vec::new(),
            texts: BTreeMap::new(),
            text_hashes: BTreeMap::new(),
            states: BTreeMap::new(),
            all_modules: Vec::new(),
            layouts: Layouts::default(),
            run: None,
            main_exit: None,
            staging_inputs: Vec::new(),
            stage_mode: StageMode::Refresh,
            graph: None,
            ir: None,
        };
        session.out_dir = session.root.join("build");
        session.cache_dir = session.out_dir.join("incr");
        session.reload_project()?;
        let report = session.rebuild_impl(true)?;
        Ok((session, report))
    }

    /// Change one file's text in memory (the session's edit surface; the
    /// tests edit without racing the disk).
    pub fn set_source(&mut self, file: &str, text: &str) {
        self.texts.insert(file.to_string(), text.to_string());
    }

    /// Re-read every project file from disk and rebuild incrementally.
    pub fn rebuild(&mut self) -> Result<SessionReport> {
        self.reload_texts_from_disk()?;
        self.rebuild_impl(false)
    }

    /// Rebuild after in-memory [`Session::set_source`] edits.
    pub fn rebuild_edits(&mut self) -> Result<SessionReport> {
        self.rebuild_impl(false)
    }

    /// The item graph of the current checked state (built on demand).
    pub fn graph(&mut self) -> Result<&ItemGraph> {
        if self.graph.is_none() {
            let g = self.build_graph()?;
            self.graph = Some(g);
        }
        Ok(self.graph.as_ref().expect("graph just built"))
    }

    /// The assembled program IR of the last artifact build.
    pub fn ir_artifact(&self) -> Option<&str> {
        self.ir.as_deref()
    }

    /// The build directory (artifact home).
    pub fn out_dir(&self) -> &Path {
        &self.out_dir
    }

    /// The object cache directory.
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    // -------------------------------------------------------- project shape

    fn reload_project(&mut self) -> Result<()> {
        let project: Project = load_project(&self.entry)?;
        self.module_file = project
            .module_file
            .iter()
            .map(|(m, p)| (m.clone(), p.display().to_string()))
            .collect();
        self.imports = project
            .imports
            .iter()
            .map(|(m, i)| (m.clone(), i.clone()))
            .collect();
        self.order = project.order.clone();
        self.entry_modules = project.entry_modules.clone();
        self.texts = project
            .sources
            .iter()
            .map(|(f, t)| (f.clone(), t.clone()))
            .collect();
        self.all_modules = super::prelude_module_names();
        for m in self.module_file.keys() {
            if !self.all_modules.contains(m) {
                self.all_modules.push(m.clone());
            }
        }
        self.wit = wit_for_source(&self.entry)?;
        Ok(())
    }

    fn reload_texts_from_disk(&mut self) -> Result<()> {
        for file in self.module_file.values().cloned().collect::<Vec<_>>() {
            let path = self.root.join(&file);
            let text = std::fs::read_to_string(&path)
                .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
            self.texts.insert(file, text);
        }
        Ok(())
    }

    // -------------------------------------------------------- the rebuild

    fn rebuild_impl(&mut self, cold: bool) -> Result<SessionReport> {
        let t_total = std::time::Instant::now();
        let mut timings = Timings::default();
        let mut report = SessionReport::default();

        // 1. Which files changed since the last check?
        let mut changed_files: Vec<String> = Vec::new();
        for (file, text) in self.texts.clone() {
            let h = interface::content_hash(&text);
            match self.text_hashes.get(&file) {
                Some(prev) if *prev == h => {}
                _ => {
                    changed_files.push(file.clone());
                    self.text_hashes.insert(file, h);
                }
            }
        }
        changed_files.sort();
        let changed_modules: Vec<String> = self
            .order
            .iter()
            .filter(|m| {
                self.module_file
                    .get(*m)
                    .map(|f| changed_files.contains(f))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        if !cold && changed_modules.is_empty() {
            // Unchanged rebuild: hash comparison and graph bookkeeping
            // only. Nothing is rechecked and nothing is re-emitted.
            for m in &self.order {
                for id in self.item_ids_of(m) {
                    report.kept.push(id);
                }
            }
            report.chain_ok = self.chain_ok();
            report.timings.total_ms = since_ms(t_total);
            self.write_rebuild_manifest(&report)?;
            return Ok(report);
        }

        // 2. The fallback valve: cases where a faithful merge of cached
        //    and fresh state is not provable. Correctness first.
        let new_module_edit = changed_modules.iter().any(|m| {
            self.states
                .get(m)
                .map(|s| s.iface.is_none())
                .unwrap_or(true)
        });
        let unit_installs = self.states.values().any(|s| {
            s.iface
                .as_ref()
                .map(|i| !i.units.is_empty())
                .unwrap_or(false)
        });
        let mut full = cold
            || self.profile.jpl
            || !self.staging_inputs.is_empty()
            || unit_installs
            || new_module_edit;

        // 3. The recheck rounds: the changed modules (round 1) classify
        //    the change kinds; the dirty closure then widens the slice
        //    (round 2 and beyond) until the classification is stable.
        let mut checked: BTreeSet<String> = BTreeSet::new();
        let mut changes: Vec<(String, ChangeKind)> = Vec::new();
        let mut dirty: BTreeSet<String> = BTreeSet::new();
        let mut t_parse = 0.0;
        let mut t_check = 0.0;
        let mut rounds = 0;
        loop {
            rounds += 1;
            if rounds > 8 {
                return Err(Error::plain(
                    "incremental recompilation did not reach a stable dirty set in 8 rounds",
                ));
            }
            let want: BTreeSet<String> = if full {
                self.order.iter().cloned().collect()
            } else if rounds == 1 {
                changed_modules.iter().cloned().collect()
            } else {
                dirty
                    .iter()
                    .filter_map(|id| self.module_of_item(id))
                    .collect()
            };
            let slice = if full {
                self.order.clone()
            } else {
                self.slice_with_deps(&want)
            };
            if slice.iter().all(|m| checked.contains(m)) {
                break;
            }
            let (parse_ms, check_ms, drift) = self.check_slice(&slice, &mut checked, &mut changes)?;
            t_parse += parse_ms;
            t_check += check_ms;
            if drift && !full {
                // The merge cannot be proven faithful (a fresh
                // owner-less unit, or captured staging inputs): full
                // recheck.
                full = true;
                continue;
            }
            if full {
                break;
            }
            let closure = self.graph_dirty(&changes)?;
            if closure == dirty {
                break;
            }
            dirty = closure;
        }
        timings.parse_ms = t_parse;
        timings.check_ms = t_check;
        report.changes = changes.clone();
        report.full_rebuild = full;

        // 4. The dirty set is the report's `rechecked`.
        dirty = self.graph_dirty(&changes)?;
        report.rechecked = dirty.iter().cloned().collect();

        // 5. Merge the checked state into the canonical program, then
        //    run the whole-program passes (optimize, emit) exactly as a
        //    cold build would. A library program (no `main`, no foreign
        //    exports) has no linked artifact: its artifact set is the
        //    interfaces plus the manifest (the driver's `check` mode).
        let t_opt = std::time::Instant::now();
        let merged = self.merged_program()?;
        let can_emit =
            merged.main.is_some() || merged.foreigns.values().any(|p| p.export);
        let (opt_program, opt) = crate::core::optimize(&merged, &self.profile)?;
        timings.optimize_ms = since_ms(t_opt);
        if can_emit {
            let t_emit = std::time::Instant::now();
            let ir = crate::codegen::emit_for(&self.profile, &opt_program)?;
            timings.emit_ms = since_ms(t_emit);

            // 6. LINK: the declaration-keyed object cache assembles the
            //    artifact and re-places only changed units.
            let t_link = std::time::Instant::now();
            let link = self.link_units(&ir)?;
            timings.link_ms = since_ms(t_link);
            report.link = link;
            let ir_path = self.out_dir.join(format!("{}.ll", self.entry_stem));
            std::fs::write(&ir_path, &ir)
                .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
            self.ir = Some(ir);
        } else {
            report.link.skipped = true;
            self.ir = None;
        }

        // 7. Artifacts: interfaces, manifest, and the linked IR.
        let ifaces = self.merged_ifaces();
        let stage = self.merged_report();
        let manifest_path = self.out_dir.join(format!("{}.manifest.json", self.entry_stem));
        write_artifacts(
            &self.out_dir,
            &self.entry_stem,
            &ifaces,
            &manifest_path,
            &stage,
            &opt_program,
            &opt,
            &self.profile,
            &self.wit,
        )?;
        report.chain_ok = self.chain_ok();

        // 8. Kept and retyped: everything else, split by whether the
        //    checker re-typed it in a slice run.
        let retyped: BTreeSet<String> = checked
            .iter()
            .flat_map(|m| self.item_ids_of(m))
            .collect();
        for m in &self.order {
            for id in self.item_ids_of(m) {
                if report.rechecked.contains(&id) {
                    continue;
                }
                if retyped.contains(&id) {
                    report.retyped.push(id);
                } else {
                    report.kept.push(id);
                }
            }
        }
        report.typed_modules = checked
            .iter()
            .filter(|m| self.order.contains(*m))
            .cloned()
            .collect();
        timings.total_ms = since_ms(t_total);
        report.timings = timings;
        self.write_rebuild_manifest(&report)?;
        Ok(report)
    }

    /// The dirty closure over the current graph for an accumulated
    /// change set (the graph is built on demand).
    fn graph_dirty(&mut self, changes: &[(String, ChangeKind)]) -> Result<BTreeSet<String>> {
        let g = self.build_graph()?;
        Ok(g.dirty_closure(changes))
    }

    /// The module a rendered item id belongs to.
    fn module_of_item(&self, id: &str) -> Option<String> {
        let name = name_of_id(id);
        let mut best: Option<String> = None;
        for m in &self.all_modules {
            if name.starts_with(&format!("{m}.")) || name == m.as_str() {
                if best.as_ref().map(|b: &String| m.len() > b.len()).unwrap_or(true) {
                    best = Some(m.clone());
                }
            }
        }
        best
    }

    /// Every item id a module owns in the current checked state.
    fn item_ids_of(&self, module: &str) -> Vec<String> {
        let mut out = vec![item_id(ItemKind::Module, module, "")];
        let Some(state) = self.states.get(module) else {
            return out;
        };
        let Some(iface) = &state.iface else {
            return out;
        };
        out.extend(self.item_ids_from_iface(iface, module));
        out
    }

    fn item_ids_from_iface(&self, iface: &Iface, module: &str) -> Vec<String> {
        let mut out = Vec::new();
        for v in &iface.exports.values {
            out.push(item_id(ItemKind::Value, module, &v.name));
        }
        for t in &iface.exports.types {
            out.push(item_id(ItemKind::Type, module, &t.name));
            for c in &t.ctors {
                out.push(item_id(ItemKind::Value, module, &c.name));
            }
        }
        for s in &iface.exports.signatures {
            out.push(item_id(ItemKind::Class, module, &s.name));
        }
        for i in &iface.exports.instances {
            out.push(item_id(ItemKind::Instance, module, &i.id));
        }
        out
    }

    /// The dirty modules plus the dependency closure the whole-program
    /// checker needs to resolve their names, in project order (entry
    /// modules last, exactly as the cold driver checks).
    fn slice_with_deps(&self, want: &BTreeSet<String>) -> Vec<String> {
        let mut slice: BTreeSet<String> = want.clone();
        let mut work: Vec<String> = want.iter().cloned().collect();
        while let Some(m) = work.pop() {
            for dep in self.imports.get(&m).into_iter().flatten() {
                if self.module_file.contains_key(dep) && slice.insert(dep.clone()) {
                    work.push(dep.clone());
                }
            }
        }
        self.order
            .iter()
            .filter(|m| slice.contains(*m))
            .cloned()
            .collect()
    }

    /// Check one slice: parse what is needed, run the shared checker,
    /// absorb the results into the per-module state, and classify
    /// per-item changes against the prior identity. Returns (parse ms,
    /// check ms, loose-unit drift).
    fn check_slice(
        &mut self,
        slice: &[String],
        checked: &mut BTreeSet<String>,
        changes: &mut Vec<(String, ChangeKind)>,
    ) -> Result<(f64, f64, bool)> {
        let t0 = std::time::Instant::now();
        let adapters = self.wit.clone();
        let prelude: PreludeSources = parse_prelude(&adapters)?;
        self.prelude_order = prelude.modules.iter().map(|m| m.name.text.clone()).collect();
        let prelude_fixities = prelude.base.clone();
        let mut module_fixities: HashMap<String, FixityTable> = prelude.fixities.clone();
        let mut parsed_by_file: HashMap<String, crate::parser::ParsedModules> = HashMap::new();
        let mut modules: Vec<crate::ast::Module> = Vec::new();
        for name in slice {
            let file = self.module_file[name].clone();
            if !parsed_by_file.contains_key(&file) {
                let src = self.texts[&file].clone();
                let parsed = parse_source(&src, &file, &prelude_fixities, &module_fixities)?;
                for (m, f) in &parsed.fixities {
                    module_fixities.insert(m.clone(), f.clone());
                }
                parsed_by_file.insert(file.clone(), parsed);
            }
            let parsed = &parsed_by_file[&file];
            let m = parsed
                .modules
                .iter()
                .find(|m| m.name.text == *name)
                .cloned()
                .ok_or_else(|| Error::plain(format!("{file}: no module {name}")))?;
            modules.push(m);
        }
        let parse_ms = since_ms(t0);

        let t1 = std::time::Instant::now();
        let mut hashes: HashMap<String, String> = HashMap::new();
        let mut files: HashMap<String, String> = HashMap::new();
        for name in slice {
            let file = self.module_file[name].clone();
            hashes.insert(name.clone(), interface::content_hash(&self.texts[&file]));
            files.insert(name.clone(), file);
        }
        let scx = StagingCx {
            fixities: module_fixities,
            files,
            prior_inputs: Vec::new(),
            mode: self.stage_mode,
            workdir: self.root.clone(),
            entry_modules: self.entry_modules.clone(),
            schema_tool: std::env::var("CINCH_SCHEMA_TOOL").ok(),
        };
        let (program, ifaces, report, _diags) = crate::check::check_program_multi_full(
            &prelude.modules,
            &modules,
            &hashes,
            &scx,
            &adapters,
            &self.profile,
        )?;
        let check_ms = since_ms(t1);

        // Group the run's rows per module (longest module prefix; the
        // checker's units spell out `Module.name`).
        let slice_set: BTreeSet<String> = slice.iter().cloned().collect();
        let all = self.all_modules.clone();
        let mut fns_by: HashMap<String, Vec<FnDef>> = HashMap::new();
        let mut loose: Vec<FnDef> = Vec::new();
        for f in &program.fns {
            match owner_module(&f.id.0, &all) {
                Some(m) => fns_by.entry(m).or_default().push(f.clone()),
                None => loose.push(f.clone()),
            }
        }
        let mut inst_by: HashMap<String, Vec<InstanceDef>> = HashMap::new();
        for i in &program.instances {
            if let Some(m) = owner_module(&i.id.0, &all) {
                inst_by.entry(m).or_default().push(i.clone());
            }
        }
        let mut laws_by: HashMap<String, Vec<LawRow>> = HashMap::new();
        for l in &program.laws {
            if let Some(m) = owner_module(&l.instance, &all) {
                laws_by.entry(m).or_default().push(l.clone());
            }
        }
        let mut rew_by: HashMap<String, Vec<RewriteRule>> = HashMap::new();
        for r in &program.rewrites {
            rew_by.entry(r.module.clone()).or_default().push(r.clone());
        }
        let mut prims_by: HashMap<String, Vec<PrimPlan>> = HashMap::new();
        for p in &program.prims {
            prims_by.entry(p.module.clone()).or_default().push(p.clone());
        }
        let mut plans_by: HashMap<String, BTreeMap<String, CtorLayout>> = HashMap::new();
        for (k, v) in &program.plans {
            let m = owner_module(k, &all).unwrap_or_default();
            plans_by.entry(m).or_default().insert(k.clone(), v.clone());
        }
        let mut foreigns_by: HashMap<String, BTreeMap<String, ForeignPlan>> = HashMap::new();
        for (k, v) in &program.foreigns {
            let m = owner_module(k, &all).unwrap_or_default();
            foreigns_by.entry(m).or_default().insert(k.clone(), v.clone());
        }
        let mut gen_by: HashMap<String, Vec<String>> = HashMap::new();
        for (m, names) in &report.generated {
            gen_by.entry(m.clone()).or_default().extend(names.clone());
        }
        let mut folds_by: HashMap<String, Vec<LitFold>> = HashMap::new();
        for f in &report.literal_folds {
            if let Some(m) = owner_module(&f.unit, &all) {
                folds_by.entry(m).or_default().push(f.clone());
            }
        }
        let mut di_by: HashMap<String, Vec<DataImport>> = HashMap::new();
        for d in &report.data_imports {
            di_by.entry(d.module.clone()).or_default().push(d.clone());
        }
        let mut safety_by: HashMap<String, Vec<SafetyAssertion>> = HashMap::new();
        for a in &report.safety.assertions {
            if let Some(m) = owner_module(&a.definition, &all) {
                safety_by.entry(m).or_default().push(a.clone());
            }
        }
        let mut temp_by: HashMap<String, Vec<TemporalRow>> = HashMap::new();
        for t in &report.temporal {
            if let Some(m) = owner_module(&t.definition, &all) {
                temp_by.entry(m).or_default().push(t.clone());
            }
        }
        // Staging inputs have no per-module merge story: any captured
        // input forces the caller back to a full rebuild (drift below).
        let mut drift = !report.inputs.is_empty();
        if !report.inputs.is_empty() {
            self.staging_inputs = report.inputs.clone();
        }

        // Absorb every module the run covered (prelude and slice), and
        // classify the slice's changes against the prior identity. An
        // already-checked module is absorbed but never re-classified
        // (its change kind was decided when it first changed).
        let mut covered: BTreeSet<String> = self.prelude_order.iter().cloned().collect();
        covered.extend(slice.iter().cloned());
        for m in &covered {
            let Some(iface) = ifaces.iter().find(|i| i.module == *m) else {
                continue;
            };
            let fresh_fns = fns_by.remove(m).unwrap_or_default();
            let prior_iface = self.states.get(m).and_then(|s| s.iface.clone());
            if !checked.contains(m) && slice_set.contains(m) {
                self.classify_module(
                    m,
                    prior_iface.as_ref(),
                    iface,
                    &fresh_fns,
                    changes,
                );
            }
            let st = self.states.entry(m.clone()).or_default();
            st.imports = self.imports.get(m).cloned().unwrap_or_default();
            st.source_hash = hashes.get(m).cloned().unwrap_or_default();
            st.iface = Some(iface.clone());
            st.fns = fresh_fns;
            st.instances = inst_by.remove(m).unwrap_or_default();
            st.laws = laws_by.remove(m).unwrap_or_default();
            st.rewrites = rew_by.remove(m).unwrap_or_default();
            st.prims = prims_by.remove(m).unwrap_or_default();
            st.plans = plans_by.remove(m).unwrap_or_default();
            st.foreigns = foreigns_by.remove(m).unwrap_or_default();
            st.generated = gen_by.remove(m).unwrap_or_default();
            st.literal_folds = folds_by.remove(m).unwrap_or_default();
            st.data_imports = di_by.remove(m).unwrap_or_default();
            st.safety = safety_by.remove(m).unwrap_or_default();
            st.temporal = temp_by.remove(m).unwrap_or_default();
            checked.insert(m.clone());
        }
        if slice_set
            .iter()
            .any(|m| self.entry_modules.contains(m))
        {
            self.main_exit = program.main.as_ref().map(|g| g.0.clone());
        }

        // The run's raw lists: a full run is its own canonical order.
        let full = slice_set.len() == self.order.len();
        self.run = Some(RunLists {
            fns: program.fns.clone(),
            instances: program.instances.clone(),
            laws: program.laws.clone(),
            rewrites: program.rewrites.clone(),
            prims: program.prims.clone(),
            plans: program.plans.clone(),
            foreigns: program.foreigns.clone(),
            full,
        });
        if full {
            self.layouts.fns = segment_ids(
                &program.fns.iter().map(|f| f.id.0.clone()).collect::<Vec<_>>(),
                &self.all_modules,
            );
            self.layouts.instances = segment_ids(
                &program
                    .instances
                    .iter()
                    .map(|i| i.id.0.clone())
                    .collect::<Vec<_>>(),
                &self.all_modules,
            );
            self.layouts.laws = segment_ids(
                &program
                    .laws
                    .iter()
                    .map(|l| format!("{}#{}", l.instance, l.law))
                    .collect::<Vec<_>>(),
                &self.all_modules,
            );
            self.layouts.rewrites = segment_ids(
                &program
                    .rewrites
                    .iter()
                    .map(|r| format!("{}.{}", r.module, r.name))
                    .collect::<Vec<_>>(),
                &self.all_modules,
            );
            self.layouts.prims = segment_ids(
                &program
                    .prims
                    .iter()
                    .map(|p| format!("{}.{}", p.module, p.name))
                    .collect::<Vec<_>>(),
                &self.all_modules,
            );
        }
        // A loose (owner-less) unit the layout has never seen cannot be
        // placed by the merge: report drift so the caller falls back.
        for f in &loose {
            if !self.layouts.fns.iter().any(|s| s.ids.contains(&f.id.0)) {
                drift = true;
            }
        }
        if !loose.is_empty() {
            // Keep loose units visible to the merge through the run's
            // raw lists (they are in `run.fns`); their state home is the
            // last module checked, so `item_ids_of` stays complete.
            if let Some(m) = slice.last() {
                let st = self.states.entry(m.clone()).or_default();
                for f in &loose {
                    if !st.fns.iter().any(|x| x.id == f.id) {
                        st.fns.push(f.clone());
                    }
                }
            }
        }
        Ok((parse_ms, check_ms, drift))
    }

    /// Classify one module's per-item changes: added, removed, surface
    /// (signature or contract facts), or body.
    fn classify_module(
        &self,
        module: &str,
        prior: Option<&Iface>,
        fresh: &Iface,
        fresh_fns: &[FnDef],
        changes: &mut Vec<(String, ChangeKind)>,
    ) {
        let prior = match prior {
            Some(p) => p,
            None => {
                for id in self.item_ids_from_iface(fresh, module) {
                    changes.push((id, ChangeKind::Added));
                }
                changes.push((
                    item_id(ItemKind::Module, module, ""),
                    ChangeKind::Added,
                ));
                return;
            }
        };
        let old_ids = self.item_ids_from_iface(prior, module);
        let new_ids = self.item_ids_from_iface(fresh, module);
        for id in &new_ids {
            if !old_ids.contains(id) {
                changes.push((id.clone(), ChangeKind::Added));
            }
        }
        for id in &old_ids {
            if !new_ids.contains(id) {
                changes.push((id.clone(), ChangeKind::Removed));
            }
        }
        let mut all_ids = new_ids.clone();
        all_ids.push(item_id(ItemKind::Module, module, ""));
        for id in &all_ids {
            if !old_ids.contains(id) && id != &item_id(ItemKind::Module, module, "") {
                continue;
            }
            let old_surface = surface_rows(prior, id);
            let new_surface = surface_rows(fresh, id);
            let old_body = self.body_rows_for(module, id, &self.states.get(module).map(|s| s.fns.clone()).unwrap_or_default());
            let new_body = self.body_rows_for(module, id, fresh_fns);
            if old_surface != new_surface {
                changes.push((id.clone(), ChangeKind::Surface));
            } else if old_body != new_body {
                changes.push((id.clone(), ChangeKind::Body));
            }
        }
    }

    /// The body identity of an item over a function list: the units the
    /// item owns (its own symbol, its methods under it, and its hoisted
    /// closures).
    fn body_rows_for(&self, module: &str, id: &str, fns: &[FnDef]) -> String {
        let name = name_of_id(id);
        if !name.starts_with(&format!("{module}.")) {
            return String::new();
        }
        let prefix = format!("{name}.");
        let owned: Vec<&FnDef> = fns
            .iter()
            .filter(|f| f.id.0 == name || f.id.0.starts_with(&prefix))
            .collect();
        body_rows(&owned)
    }

    // -------------------------------------------------------- merge and artifacts

    /// The canonical checked program: the per-module state assembled in
    /// the checker's own order (the recorded layout), so the merged
    /// output is byte-identical to a cold check of the same sources. A
    /// full run needs no merge: its raw lists are the canonical order.
    fn merged_program(&mut self) -> Result<Program> {
        if let Some(run) = self.run.clone() {
            if run.full {
                let main_exit = self.main_exit.clone().map(crate::core::GlobalId);
                return Ok(Program {
                    fns: run.fns,
                    instances: run.instances,
                    main: main_exit,
                    rewrites: run.rewrites,
                    plans: run.plans,
                    foreigns: run.foreigns,
                    prims: run.prims,
                    laws: run.laws,
                });
            }
        }
        let fns: Vec<FnDef> = self.merge_list(ListKind::Fns, |s| s.fns.clone())?;
        let instances: Vec<InstanceDef> =
            self.merge_list(ListKind::Instances, |s| s.instances.clone())?;
        let laws: Vec<LawRow> = self.merge_list(ListKind::Laws, |s| s.laws.clone())?;
        let rewrites: Vec<RewriteRule> =
            self.merge_list(ListKind::Rewrites, |s| s.rewrites.clone())?;
        let prims: Vec<PrimPlan> = self.merge_list(ListKind::Prims, |s| s.prims.clone())?;
        let mut plans: BTreeMap<String, CtorLayout> = BTreeMap::new();
        let mut foreigns: BTreeMap<String, ForeignPlan> = BTreeMap::new();
        for s in self.states.values() {
            plans.extend(s.plans.clone());
            foreigns.extend(s.foreigns.clone());
        }
        let main_exit = self.main_exit.clone().map(crate::core::GlobalId);
        // Refresh the layout from the merged order so the next merge
        // tracks the current structure.
        let fns_ids: Vec<String> = fns.iter().map(|f| f.id.0.clone()).collect();
        self.layouts.fns = segment_ids(&fns_ids, &self.all_modules);
        let inst_ids: Vec<String> = instances.iter().map(|i| i.id.0.clone()).collect();
        self.layouts.instances = segment_ids(&inst_ids, &self.all_modules);
        let law_ids: Vec<String> = laws
            .iter()
            .map(|l| format!("{}#{}", l.instance, l.law))
            .collect();
        self.layouts.laws = segment_ids(&law_ids, &self.all_modules);
        let rew_ids: Vec<String> = rewrites
            .iter()
            .map(|r| format!("{}.{}", r.module, r.name))
            .collect();
        self.layouts.rewrites = segment_ids(&rew_ids, &self.all_modules);
        let prim_ids: Vec<String> = prims
            .iter()
            .map(|p| format!("{}.{}", p.module, p.name))
            .collect();
        self.layouts.prims = segment_ids(&prim_ids, &self.all_modules);
        Ok(Program {
            fns,
            instances,
            main: main_exit,
            rewrites,
            plans,
            foreigns,
            prims,
            laws,
        })
    }

    /// Assemble one list from the recorded segment layout and the
    /// per-module state. Each recorded segment takes the ids assigned to
    /// it: its recorded ids that still exist (in the owner's current
    /// order, so edits that move declarations inside a segment stay
    /// correct) plus new ids placed after their nearest surviving
    /// predecessor. A declaration that moves across a segment boundary
    /// of its own module keeps its recorded slot (the artifact stays
    /// semantics-identical; the pinned cases never move decls).
    fn merge_list<T: Identifiable + Clone>(
        &self,
        kind: ListKind,
        collect: impl Fn(&ModuleState) -> Vec<T>,
    ) -> Result<Vec<T>> {
        let layout: &[Segment] = match kind {
            ListKind::Fns => &self.layouts.fns,
            ListKind::Instances => &self.layouts.instances,
            ListKind::Laws => &self.layouts.laws,
            ListKind::Rewrites => &self.layouts.rewrites,
            ListKind::Prims => &self.layouts.prims,
        };
        // The current pool: every item the states hold, grouped by its
        // owner in current (check) order.
        let mut pool: BTreeMap<String, T> = BTreeMap::new();
        let mut current: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for st in self.states.values() {
            for it in collect(st) {
                let id = it.ident();
                if pool.contains_key(&id) {
                    continue;
                }
                let owner = owner_module(&id, &self.all_modules).unwrap_or_default();
                current.entry(owner).or_default().push(id.clone());
                pool.insert(id, it);
            }
        }
        // Assign each current id to a recorded segment: its own recorded
        // slot when it has one, else the slot of its nearest surviving
        // predecessor (a new declaration lands inside its module's
        // segment at the right place).
        let mut assigned: BTreeMap<String, usize> = BTreeMap::new();
        for (owner, ids) in &current {
            let mut last: Option<usize> = None;
            for id in ids {
                let recorded = layout
                    .iter()
                    .position(|s| s.owner == *owner && s.ids.contains(id));
                let seg = recorded.or(last);
                match seg {
                    Some(i) => {
                        assigned.insert(id.clone(), i);
                        last = Some(i);
                    }
                    None => {
                        // No recorded position (a new owner): append.
                        assigned.insert(id.clone(), usize::MAX);
                    }
                }
            }
        }
        let mut out: Vec<T> = Vec::new();
        let mut emitted: BTreeSet<String> = BTreeSet::new();
        for (i, s) in layout.iter().enumerate() {
            if s.owner.is_empty() {
                // The owner-less interleaved units keep their recorded
                // order (their current grouping carries no order).
                for id in &s.ids {
                    if let Some(t) = pool.get(id) {
                        out.push(t.clone());
                        emitted.insert(id.clone());
                    }
                }
                continue;
            }
            if let Some(ids) = current.get(&s.owner) {
                for id in ids {
                    if assigned.get(id) == Some(&i) {
                        if let Some(t) = pool.get(id) {
                            out.push(t.clone());
                            emitted.insert(id.clone());
                        }
                    }
                }
            }
        }
        let mut rest: Vec<&T> = pool
            .values()
            .filter(|t| !emitted.contains(&t.ident()))
            .collect();
        rest.sort_by_key(|t| t.ident());
        for t in rest {
            out.push(t.clone());
        }
        Ok(out)
    }

    /// The merged interfaces of every module (prelude in family order,
    /// then user modules in project order).
    fn merged_ifaces(&self) -> Vec<Iface> {
        let mut out = Vec::new();
        let mut order = self.prelude_order.clone();
        order.extend(self.order.iter().cloned());
        for m in &order {
            if let Some(Some(i)) = self.states.get(m).map(|s| s.iface.clone()) {
                out.push(i);
            }
        }
        out
    }

    /// The merged stage report: the rows every module contributed, in
    /// the checker's order.
    fn merged_report(&self) -> StageReport {
        let mut generated: Vec<(String, Vec<String>)> = Vec::new();
        let mut folds: Vec<LitFold> = Vec::new();
        let mut imports: Vec<DataImport> = Vec::new();
        let mut safety: Vec<SafetyAssertion> = Vec::new();
        let mut temporal: Vec<TemporalRow> = Vec::new();
        let mut order = self.prelude_order.clone();
        order.extend(self.order.iter().cloned());
        for m in &order {
            let Some(s) = self.states.get(m) else { continue };
            if !s.generated.is_empty() {
                generated.push((m.clone(), s.generated.clone()));
            }
            folds.extend(s.literal_folds.clone());
            imports.extend(s.data_imports.clone());
            safety.extend(s.safety.clone());
            temporal.extend(s.temporal.clone());
        }
        StageReport {
            mode: self.stage_mode,
            inputs: self.staging_inputs.clone(),
            generated,
            literal_folds: folds,
            data_imports: imports,
            safety: SafetyReport {
                assertions: safety,
                jpl: None,
            },
            temporal,
        }
    }

    /// Does the merged `.cinchif` "checked against" chain verify?
    fn chain_ok(&self) -> bool {
        let ifaces: Vec<Iface> = self.merged_ifaces();
        interface::verify_chain(&ifaces).is_ok()
    }

    // -------------------------------------------------------- LINK

    /// The declaration-keyed object cache: split the fresh emission into
    /// its codegen units, key each on its owning declaration's
    /// `decl_hash`, re-place only the units whose key or bytes changed,
    /// and assemble the linked artifact from the cache objects. The
    /// assembly is verified against the fresh emission; any disagreement
    /// repairs the cache from the emission (the cache is advisory).
    fn link_units(&mut self, ir: &str) -> Result<LinkReport> {
        let split = split_ir(ir)?;
        let index_path = self.cache_dir.join("index.tsv");
        let objects_dir = self.cache_dir.join("objects");
        std::fs::create_dir_all(&objects_dir)
            .map_err(|e| Error::plain(format!("{}: {e}", objects_dir.display())))?;
        let prior: BTreeMap<String, CacheEntry> = read_index(&index_path);

        let decl_of_symbol = self.symbol_owners();
        let mut report = LinkReport {
            verified: false,
            repaired: false,
            ..LinkReport::default()
        };
        let mut next_index: BTreeMap<String, CacheEntry> = BTreeMap::new();
        let mut unit_texts: Vec<(String, String)> = Vec::new();
        for (symbol, text) in &split.units {
            let owner = decl_of_symbol.get(symbol).cloned();
            let decl_hash = match &owner {
                Some(o) => self.decl_hash_of(o),
                None => hash_of(symbol),
            };
            let obj_hash = hash_of(text);
            let entry = CacheEntry {
                decl: owner.clone().unwrap_or_else(|| symbol.clone()),
                decl_hash: decl_hash.clone(),
                object_hash: obj_hash.clone(),
            };
            let reuse = match prior.get(symbol) {
                Some(p) => {
                    p.decl_hash == decl_hash
                        && p.object_hash == obj_hash
                        && read_object(&objects_dir, symbol).as_deref() == Some(text.as_str())
                }
                None => false,
            };
            if reuse {
                report.reused.push(symbol.clone());
            } else {
                report.placed.push(symbol.clone());
                write_object(&objects_dir, symbol, text)?;
            }
            next_index.insert(symbol.clone(), entry);
            unit_texts.push((symbol.clone(), text.clone()));
        }
        for key in prior.keys() {
            if !next_index.contains_key(key) {
                report.dropped.push(key.clone());
            }
        }
        let assembled = assemble_ir(&split, &unit_texts);
        report.verified = assembled == ir;
        if !report.verified {
            // The cache disagreed with the emission (byte drift at an
            // unchanged key): repair from the emission and report it.
            report.repaired = true;
            report.placed = split.units.iter().map(|(s, _)| s.clone()).collect();
            report.reused.clear();
            for (symbol, text) in &split.units {
                write_object(&objects_dir, symbol, text)?;
            }
            let assembled = assemble_ir(&split, &unit_texts);
            report.verified = assembled == ir;
        }
        write_index(&index_path, &next_index)?;
        Ok(report)
    }

    /// Unit symbol to owning item id: the declaration whose code the
    /// unit is. Values and ctors key on their own name, instance
    /// dictionary and method units on the instance, hoisted closures on
    /// their parent declaration, and the synthesized exit adapter on
    /// `main`.
    fn symbol_owners(&self) -> BTreeMap<String, String> {
        let mut out: BTreeMap<String, String> = BTreeMap::new();
        let mut all: Vec<&str> = self.prelude_order.iter().map(|s| s.as_str()).collect();
        all.extend(self.order.iter().map(|s| s.as_str()));
        for m in all {
            let Some(state) = self.states.get(m) else { continue };
            let Some(iface) = &state.iface else { continue };
            for v in &iface.exports.values {
                out.insert(v.symbol.clone(), item_id(ItemKind::Value, m, &v.name));
            }
            for t in &iface.exports.types {
                for c in &t.ctors {
                    out.insert(format!("{m}.{}", c.name), item_id(ItemKind::Value, m, &c.name));
                }
            }
            for i in &iface.exports.instances {
                out.insert(format!("{}.mk", i.id), item_id(ItemKind::Instance, m, &i.id));
            }
            for f in &state.fns {
                let sym = f.id.0.clone();
                if out.contains_key(&sym) {
                    continue;
                }
                let owner = self
                    .decl_owner_of(&sym)
                    .unwrap_or_else(|| item_id(ItemKind::Value, m, &sym));
                out.insert(sym, owner);
            }
        }
        if let Some(exit) = &self.main_exit {
            let module = self.module_of_item(exit).unwrap_or_default();
            let local = name_of_id(exit);
            let parent = local.strip_suffix(".exit").unwrap_or(local);
            let parent = parent.rsplit('.').next().unwrap_or(parent);
            out.insert(
                exit.clone(),
                item_id(ItemKind::Value, &module, parent),
            );
        }
        out
    }

    /// The declaration a unit symbol belongs to: its exact item, else
    /// the longest declaration symbol it is a strict extension of
    /// (hoisted closures spell out `Unit.name`).
    fn decl_owner_of(&self, symbol: &str) -> Option<String> {
        let module = owner_module(symbol, &self.all_modules)?;
        let state = self.states.get(&module)?;
        let iface = state.iface.as_ref()?;
        let mut best: Option<(usize, String)> = None;
        let mut consider = |sym: &str, id: String| {
            if symbol == sym {
                return Some(id);
            }
            if symbol.starts_with(&format!("{sym}.")) {
                let score = sym.len();
                if best.as_ref().map(|(s, _)| score > *s).unwrap_or(true) {
                    best = Some((score, id));
                }
            }
            None
        };
        for v in &iface.exports.values {
            if let Some(id) = consider(&v.symbol, item_id(ItemKind::Value, &module, &v.name)) {
                return Some(id);
            }
        }
        for t in &iface.exports.types {
            for c in &t.ctors {
                let sym = format!("{module}.{}", c.name);
                if let Some(id) = consider(&sym, item_id(ItemKind::Value, &module, &c.name)) {
                    return Some(id);
                }
            }
        }
        for i in &iface.exports.instances {
            let id = item_id(ItemKind::Instance, &module, &i.id);
            if symbol == i.id || symbol.starts_with(&format!("{}.", i.id)) {
                let score = i.id.len();
                if best.as_ref().map(|(s, _)| score > *s).unwrap_or(true) {
                    best = Some((score, id));
                }
            }
        }
        best.map(|(_, id)| id)
    }

    /// The declaration identity the object cache keys on.
    fn decl_hash_of(&self, item: &str) -> String {
        if let Some(g) = &self.graph {
            if let Some(it) = g.item(item) {
                return it.decl_hash.clone();
            }
        }
        let module = match self.module_of_item(item) {
            Some(m) => m,
            None => return hash_of(item),
        };
        let Some(state) = self.states.get(&module) else {
            return hash_of(item);
        };
        let Some(iface) = &state.iface else {
            return hash_of(item);
        };
        let surface = surface_rows(iface, item);
        let body = self.body_rows_for(&module, item, &state.fns);
        hash_of(&format!("{surface}\n{body}"))
    }

    // -------------------------------------------------------- graph construction

    /// Build the item graph from the current checked state (REAPER:
    /// on-demand, whole program; nodes and surface edges from the
    /// `.cinchif` metadata and contract facts, body edges from the
    /// checked declarations' references resolved against it).
    fn build_graph(&mut self) -> Result<ItemGraph> {
        let mut g = ItemGraph::default();
        let mut order = self.prelude_order.clone();
        order.extend(self.order.iter().cloned());
        // Nodes: modules and their items, with contract fact nodes.
        for m in &order {
            let Some(state) = self.states.get(m) else { continue };
            let Some(iface) = &state.iface else { continue };
            let iface = iface.clone();
            let source_hash = state.source_hash.clone();
            let module_id = item_id(ItemKind::Module, m, "");
            let surface = surface_rows(&iface, &module_id);
            let body = hash_of(&source_hash);
            g.insert_item(Item {
                id: module_id.clone(),
                kind: ItemKind::Module,
                module: m.clone(),
                name: String::new(),
                surface_hash: surface.clone(),
                body_hash: body.clone(),
                decl_hash: hash_of(&format!("{surface}\n{body}")),
                facts: Vec::new(),
                owner: None,
            });
            for id in self.item_ids_from_iface(&iface, m) {
                let kind = kind_of_id(&id);
                let surface = surface_rows(&iface, &id);
                let body = self.body_rows_for(m, &id, &self.states.get(m).map(|s| s.fns.clone()).unwrap_or_default());
                let facts = contract_facts_of(&iface, &id);
                for (ix, fact) in facts.iter().enumerate() {
                    let cid = contract_id(name_of_id(&id), &format!("f{ix}"));
                    let s = fact.clone();
                    g.insert_item(Item {
                        id: cid.clone(),
                        kind: ItemKind::Contract,
                        module: m.clone(),
                        name: fact.clone(),
                        surface_hash: hash_of(&s),
                        body_hash: String::new(),
                        decl_hash: hash_of(&s),
                        facts: vec![fact.clone()],
                        owner: Some(id.clone()),
                    });
                    g.insert_edge(&cid, &id, EdgeKind::Surface);
                    g.insert_edge(&id, &cid, EdgeKind::Surface);
                }
                g.insert_item(Item {
                    id: id.clone(),
                    kind,
                    module: m.clone(),
                    name: name_of_id(&id).to_string(),
                    surface_hash: surface.clone(),
                    body_hash: body.clone(),
                    decl_hash: hash_of(&format!("{surface}\n{body}")),
                    facts,
                    owner: None,
                });
                // The item's surface is part of its module's surface.
                g.insert_edge(&module_id, &id, EdgeKind::Surface);
                // A constructor unit's code and identity come from its
                // type declaration: a type change reaches the ctor and
                // through it every body that constructs it.
                if kind == ItemKind::Type {
                    for t in &iface.exports.types {
                        if item_id(ItemKind::Type, m, &t.name) != id {
                            continue;
                        }
                        for c in &t.ctors {
                            let ctor_id = item_id(ItemKind::Value, m, &c.name);
                            g.insert_edge(&ctor_id, &id, EdgeKind::Surface);
                        }
                    }
                }
            }
            // Module import edges (the `.cinchif` chain's spine).
            let imports = state.imports.clone();
            for imp in &imports {
                let dep = item_id(ItemKind::Module, imp, "");
                if g.item(&dep).is_some() {
                    g.insert_edge(&module_id, &dep, EdgeKind::Surface);
                }
            }
        }
        // Edges from the declarations' references, resolved against the
        // interface metadata.
        let exports = self.export_table();
        let files: Vec<(String, String, String)> = self
            .order
            .iter()
            .filter_map(|m| {
                self.states.get(m)?;
                let file = self.module_file.get(m)?.clone();
                let text = self.texts.get(&file)?.clone();
                Some((m.clone(), file, text))
            })
            .collect();
        for (m, file, text) in files {
            let parsed = self.parse_for_graph(&file, &text)?;
            for module_ast in parsed.modules.iter().filter(|x| x.name.text == m) {
                for d in &module_ast.decls {
                    let owner = decl_item_of(d, &m);
                    let Some(owner) = owner else { continue };
                    if g.item(&owner).is_none() {
                        continue;
                    }
                    let mut srefs = Vec::new();
                    let mut brefs = Vec::new();
                    surface_refs(d, &mut srefs);
                    body_refs(d, &mut brefs);
                    for r in &srefs {
                        for target in resolve_ref(&exports, &m, r) {
                            if target != owner {
                                g.insert_edge(&owner, &target, EdgeKind::Surface);
                            }
                        }
                    }
                    for r in &brefs {
                        for target in resolve_ref(&exports, &m, r) {
                            if target != owner {
                                g.insert_edge(&owner, &target, EdgeKind::Body);
                            }
                        }
                    }
                }
            }
        }
        Ok(g)
    }

    /// The export table for reference resolution: bare name to the items
    /// that could own it (values, ctors, types, classes, instances).
    fn export_table(&self) -> BTreeMap<String, Vec<(String, ItemKind)>> {
        let mut out: BTreeMap<String, Vec<(String, ItemKind)>> = BTreeMap::new();
        let mut order = self.prelude_order.clone();
        order.extend(self.order.iter().cloned());
        for m in &order {
            let Some(state) = self.states.get(m) else { continue };
            let Some(iface) = &state.iface else { continue };
            for v in &iface.exports.values {
                out.entry(v.name.clone()).or_default().push((
                    item_id(ItemKind::Value, m, &v.name),
                    ItemKind::Value,
                ));
            }
            for t in &iface.exports.types {
                out.entry(t.name.clone()).or_default().push((
                    item_id(ItemKind::Type, m, &t.name),
                    ItemKind::Type,
                ));
                for c in &t.ctors {
                    out.entry(c.name.clone()).or_default().push((
                        item_id(ItemKind::Value, m, &c.name),
                        ItemKind::Value,
                    ));
                }
            }
            for s in &iface.exports.signatures {
                out.entry(s.name.clone()).or_default().push((
                    item_id(ItemKind::Class, m, &s.name),
                    ItemKind::Class,
                ));
                // A method call is a reference to the class's method and
                // to every instance that could discharge it (over-
                // approximate: resolution is the checker's job, the
                // graph only bounds the dirty set from above).
                for meth in &s.methods {
                    out.entry(meth.name.clone()).or_default().push((
                        item_id(ItemKind::Class, m, &s.name),
                        ItemKind::Class,
                    ));
                    for i in &iface.exports.instances {
                        if i.class == s.name {
                            out.entry(meth.name.clone()).or_default().push((
                                item_id(ItemKind::Instance, m, &i.id),
                                ItemKind::Instance,
                            ));
                        }
                    }
                }
            }
            for i in &iface.exports.instances {
                out.entry(i.id.clone()).or_default().push((
                    item_id(ItemKind::Instance, m, &i.id),
                    ItemKind::Instance,
                ));
            }
        }
        out
    }

    /// Parse one file for graph construction, threading the prelude and
    /// already-parsed siblings' fixities (an operator-heavy module must
    /// parse the same way the checker parsed it, or edges would be
    /// missed).
    fn parse_for_graph(
        &mut self,
        file: &str,
        text: &str,
    ) -> Result<crate::parser::ParsedModules> {
        let adapters = self.wit.clone();
        let prelude = parse_prelude(&adapters)?;
        let mut tables: HashMap<String, FixityTable> = prelude.fixities.clone();
        // Thread the fixities of every file that precedes this one in
        // project order (dependency order: imports first).
        let order_files: Vec<String> = self
            .order
            .iter()
            .filter_map(|m| self.module_file.get(m).cloned())
            .collect();
        for f in &order_files {
            if f == file {
                break;
            }
            let Some(t) = self.texts.get(f) else { continue };
            if let Ok(pm) = parse_source(t, f, &prelude.base, &tables) {
                for (m, fix) in &pm.fixities {
                    tables.insert(m.clone(), fix.clone());
                }
            }
        }
        parse_source(text, file, &prelude.base, &tables)
    }

    // -------------------------------------------------------- rebuild manifest

    /// Write the rebuild report (the driver's own manifest of what this
    /// rebuild touched; deterministic content, no timings: it is build
    /// metadata, not artifact identity).
    fn write_rebuild_manifest(&self, report: &SessionReport) -> Result<()> {
        std::fs::create_dir_all(&self.out_dir)
            .map_err(|e| Error::plain(format!("{}: {e}", self.out_dir.display())))?;
        let path = self.out_dir.join(format!("{}.rebuild.json", self.entry_stem));
        let list = |xs: &[String]| -> String {
            let items: Vec<String> = xs.iter().map(|x| format!("{x:?}")).collect();
            format!("[{}]", items.join(", "))
        };
        let mut text = String::new();
        text.push_str("{\n");
        text.push_str(&format!("  \"rechecked\": {},\n", list(&report.rechecked)));
        text.push_str(&format!("  \"retyped\": {},\n", list(&report.retyped)));
        text.push_str(&format!("  \"kept\": {},\n", list(&report.kept)));
        text.push_str(&format!(
            "  \"typed_modules\": {},\n",
            list(&report.typed_modules)
        ));
        text.push_str(&format!(
            "  \"changes\": [{}],\n",
            report
                .changes
                .iter()
                .map(|(id, k)| format!("{{\"item\": {id:?}, \"kind\": {k:?}}}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        text.push_str(&format!(
            "  \"chain_ok\": {},\n  \"full_rebuild\": {},\n",
            report.chain_ok, report.full_rebuild
        ));
        text.push_str("  \"link\": {\n");
        text.push_str(&format!("    \"placed\": {},\n", list(&report.link.placed)));
        text.push_str(&format!("    \"reused\": {},\n", list(&report.link.reused)));
        text.push_str(&format!("    \"dropped\": {},\n", list(&report.link.dropped)));
        text.push_str(&format!(
            "    \"verified\": {},\n    \"repaired\": {}\n  }}\n}}\n",
            report.link.verified, report.link.repaired
        ));
        std::fs::write(&path, text)
            .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
        Ok(())
    }
}

/// The longest module prefix an id belongs to (`Module.name` ids).
fn owner_module(id: &str, modules: &[String]) -> Option<String> {
    let mut best: Option<String> = None;
    for m in modules {
        if id.starts_with(&format!("{m}.")) {
            if best.as_ref().map(|b: &String| m.len() > b.len()).unwrap_or(true) {
                best = Some(m.clone());
            }
        }
    }
    best
}

// ============================================================ helpers

/// The contract facts an item carries (the labels the graph records).
fn contract_facts_of(iface: &Iface, id: &str) -> Vec<String> {
    let name = name_of_id(id);
    let name = name
        .strip_prefix(&format!("{}.", iface.module))
        .unwrap_or(name);
    let mut out = Vec::new();
    for a in &iface.assertions {
        if a.name == name {
            for f in &a.facts {
                out.push(format!("assertion:{f}"));
            }
        }
    }
    for c in &iface.contracts {
        if c.name == name {
            out.push(format!("contract:{}:{}", c.kind, c.formula));
        }
    }
    for l in &iface.laws {
        if l.instance == name {
            out.push(format!("law:{}:{}", l.law, l.status));
        }
    }
    out
}

/// The graph item a declaration owns (the value or instance its
/// equations elaborate into).
fn decl_item_of(d: &Decl, module: &str) -> Option<String> {
    match d {
        Decl::ValueSig { name, .. } => Some(item_id(ItemKind::Value, module, &name.text)),
        Decl::Equation { eq, .. } => Some(item_id(ItemKind::Value, module, &eq.name.text)),
        Decl::Instance(i) => Some(item_id(ItemKind::Instance, module, &i.name.text)),
        Decl::Data(data) => Some(item_id(ItemKind::Type, module, &data.name.text)),
        Decl::Record(r) | Decl::Codata(r) => Some(item_id(ItemKind::Type, module, &r.name.text)),
        Decl::Signature(s) => Some(item_id(ItemKind::Class, module, &s.name.text)),
        Decl::Refine(r) => Some(item_id(ItemKind::Type, module, &r.name.text)),
        Decl::Prim(p) => Some(item_id(ItemKind::Value, module, &p.name.text)),
        Decl::OpaqueType(o) => Some(item_id(ItemKind::Type, module, &o.name.text)),
        Decl::Contract(c) => Some(item_id(ItemKind::Value, module, &c.target.text)),
        Decl::Attributed { inner, .. } => decl_item_of(inner, module),
        Decl::NameSpliced { inner, .. } => decl_item_of(inner, module),
        _ => None,
    }
}

/// Resolve one reference to the items it could name. A qualified name
/// narrows to its own module when that resolves; every other case
/// matches by name across the exports. Over-approximate on purpose: an
/// ambiguous or unresolved name adds edges to every candidate, so the
/// dirty set can only overshoot, never miss.
fn resolve_ref(
    exports: &BTreeMap<String, Vec<(String, ItemKind)>>,
    module: &str,
    r: &RefSite,
) -> Vec<String> {
    let want_type = r.kind == RefKind::Type;
    let fits = |k: &ItemKind| -> bool {
        match (want_type, k) {
            (true, ItemKind::Type) | (true, ItemKind::Class) => true,
            (true, _) => false,
            (false, ItemKind::Type) => false,
            // A value reference may name a class method (the class and
            // its instances) as well as a value or instance.
            (false, _) => true,
        }
    };
    if !r.quals.is_empty() {
        let qual = r.quals.join(".");
        if qual == module {
            if let Some(items) = exports.get(&r.name) {
                let own: Vec<String> = items
                    .iter()
                    .filter(|(id, k)| id.contains(&format!("{module}.")) && fits(k))
                    .map(|(id, _)| id.clone())
                    .collect();
                if !own.is_empty() {
                    return own;
                }
            }
        }
    }
    match exports.get(&r.name) {
        Some(items) => items
            .iter()
            .filter(|(_, k)| fits(k))
            .map(|(id, _)| id.clone())
            .collect(),
        None => Vec::new(),
    }
}

// ============================================================ the typed merge

/// Which recorded list the merge assembles.
#[derive(Clone, Copy, Debug)]
enum ListKind {
    Fns,
    Instances,
    Laws,
    Rewrites,
    Prims,
}

/// The identity one merged list orders by (the checker's unit name).
trait Identifiable {
    fn ident(&self) -> String;
}

impl Identifiable for FnDef {
    fn ident(&self) -> String {
        self.id.0.clone()
    }
}

impl Identifiable for InstanceDef {
    fn ident(&self) -> String {
        self.id.0.clone()
    }
}

impl Identifiable for LawRow {
    fn ident(&self) -> String {
        // Several rows share an instance (one per law): the row's
        // identity is the pair.
        format!("{}#{}", self.instance, self.law)
    }
}

impl Identifiable for RewriteRule {
    fn ident(&self) -> String {
        format!("{}.{}", self.module, self.name)
    }
}

impl Identifiable for PrimPlan {
    fn ident(&self) -> String {
        format!("{}.{}", self.module, self.name)
    }
}

/// One cached unit object's identity (the index row).
#[derive(Clone, Debug, Default)]
struct CacheEntry {
    decl: String,
    decl_hash: String,
    object_hash: String,
}

// ============================================================ LINK: IR units

/// A split program emission: the shared preamble and tail, and the
/// per-unit code blocks in emission order. Every codegen unit `U`
/// contributes `@"U"`, `@"U.app"` and `@"U.clo"` (codegen's module doc),
/// so a unit's block ends at its `.clo` global and every symbol in the
/// block is `U` or `U.<suffix>`.
struct SplitIr {
    preamble: String,
    units: Vec<(String, String)>,
    tail: String,
}

/// The name in one top-level IR item: the quoted or bare identifier
/// after `@`, with `quote()`'s `\XX` hex escapes decoded. Quoted names
/// may contain operator characters (`Data.Nat.Eq_Nat.==`), so the name
/// ends at the closing quote, never at the first `=` or `(`.
fn ir_symbol(chunk: &str) -> (u8, String) {
    if chunk.starts_with("define ") {
        let Some(at) = chunk.find('@') else {
            return (1, String::new());
        };
        (1, decode_ir_name(&chunk[at + 1..]))
    } else if chunk.starts_with('@') {
        (2, decode_ir_name(&chunk[1..]))
    } else {
        (0, String::new())
    }
}

fn decode_ir_name(rest: &str) -> String {
    let bytes = rest.as_bytes();
    if bytes.first() != Some(&b'"') {
        return rest
            .split(['(', ' ', '='])
            .next()
            .unwrap_or_default()
            .to_string();
    }
    let mut out: Vec<u8> = Vec::new();
    let mut i = 1;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => break,
            b'\\' => {
                let hex = |b: u8| -> u8 {
                    (b as char).to_digit(16).unwrap_or(0) as u8
                };
                if i + 2 < bytes.len() {
                    out.push((hex(bytes[i + 1]) << 4) | hex(bytes[i + 2]));
                    i += 3;
                } else {
                    i += 1;
                }
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

/// Split one emission into its units. The split is validated by exact
/// reassembly; any surprise falls back to a single blob unit (the cache
/// stays advisory).
fn split_ir(ir: &str) -> Result<SplitIr> {
    struct Chunk {
        start: usize,
        end: usize,
        kind: u8, // 0 other, 1 define, 2 global
        symbol: String,
    }
    let mut starts: Vec<usize> = Vec::new();
    let mut line_start = 0usize;
    for (i, ch) in ir.char_indices() {
        if ch == '\n' {
            line_start = i + 1;
            continue;
        }
        if i != line_start {
            continue;
        }
        let head = &ir[line_start..];
        let is_top = head.starts_with("define ")
            || head.starts_with('@')
            || head.starts_with('%')
            || head.starts_with("declare ")
            || head.starts_with("attributes ")
            || head.starts_with('!')
            || head.starts_with("target ")
            || head.starts_with(';');
        if is_top {
            starts.push(line_start);
        }
    }
    let mut chunks: Vec<Chunk> = Vec::new();
    for (k, &s) in starts.iter().enumerate() {
        let e = starts.get(k + 1).copied().unwrap_or(ir.len());
        let text = &ir[s..e];
        let (kind, symbol) = ir_symbol(text);
        chunks.push(Chunk {
            start: s,
            end: e,
            kind,
            symbol,
        });
    }
    // Unit names: the `.clo` global names them (every unit emits one).
    let mut clo_units: Vec<String> = Vec::new();
    for c in &chunks {
        if c.kind == 2 {
            if let Some(u) = c.symbol.strip_suffix(".clo") {
                clo_units.push(u.to_string());
            }
        }
    }
    clo_units.sort();
    clo_units.dedup();
    let owner_of = |symbol: &str, kind: u8| -> Option<String> {
        if kind == 0 {
            return None;
        }
        let mut best: Option<String> = None;
        for u in &clo_units {
            if symbol == u.as_str() || symbol.starts_with(&format!("{u}.")) {
                if best.as_ref().map(|b: &String| u.len() > b.len()).unwrap_or(true) {
                    best = Some(u.clone());
                }
            }
        }
        best
    };
    // Group chunks into contiguous unit blocks.
    let mut blocks: Vec<(String, Vec<usize>)> = Vec::new();
    for (i, c) in chunks.iter().enumerate() {
        let owner = owner_of(&c.symbol, c.kind);
        match (owner, blocks.last_mut()) {
            (Some(o), Some((bo, ix))) if *bo == o => ix.push(i),
            (Some(o), _) => blocks.push((o, vec![i])),
            (None, _) => {}
        }
    }
    // Each block must cover exactly one unit's chunks, including its
    // define and its closing clo; otherwise fall back to a blob.
    let mut ok = true;
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for (u, ix) in &blocks {
        if !seen.insert(u.clone()) {
            ok = false;
        }
        let mut has_define = false;
        let mut has_clo = false;
        for &i in ix {
            let c = &chunks[i];
            if c.kind == 1 && c.symbol == *u {
                has_define = true;
            }
            if c.kind == 2 && c.symbol == format!("{u}.clo") {
                has_clo = true;
            }
            if owner_of(&c.symbol, c.kind).as_deref() != Some(u.as_str()) {
                ok = false;
            }
        }
        if !has_define || !has_clo {
            ok = false;
        }
    }
    if blocks.len() != clo_units.len() {
        ok = false;
    }
    if !ok {
        return Ok(SplitIr {
            preamble: String::new(),
            units: vec![(String::new(), ir.to_string())],
            tail: String::new(),
        });
    }
    let first = blocks.first().map(|(_, ix)| ix[0]).unwrap_or(0);
    let last = blocks
        .last()
        .and_then(|(_, ix)| ix.last().copied())
        .unwrap_or(0);
    let preamble_end = chunks.get(first).map(|c| c.start).unwrap_or(ir.len());
    let tail_start = chunks.get(last).map(|c| c.end).unwrap_or(ir.len());
    let mut units = Vec::new();
    for (u, ix) in &blocks {
        let s = chunks[ix[0]].start;
        let e = chunks[*ix.last().expect("non-empty block")].end;
        units.push((u.clone(), ir[s..e].to_string()));
    }
    Ok(SplitIr {
        preamble: ir[..preamble_end].to_string(),
        units,
        tail: ir[tail_start..].to_string(),
    })
}

/// Reassemble an emission from its unit texts (the link step; unit order
/// is the emission order the split recorded).
fn assemble_ir(split: &SplitIr, units: &[(String, String)]) -> String {
    let mut out = String::new();
    out.push_str(&split.preamble);
    for (i, (_, text)) in split.units.iter().enumerate() {
        let text = units
            .get(i)
            .map(|(_, t)| t.as_str())
            .unwrap_or(text.as_str());
        out.push_str(text);
    }
    out.push_str(&split.tail);
    out
}

// ---------------------------------------------------------- the cache store

fn object_name(symbol: &str) -> String {
    if symbol.is_empty() {
        "unit.blob".to_string()
    } else {
        format!("{}.ll", interface::content_hash(symbol))
    }
}

fn read_object(dir: &Path, symbol: &str) -> Option<String> {
    std::fs::read_to_string(dir.join(object_name(symbol))).ok()
}

fn write_object(dir: &Path, symbol: &str, text: &str) -> Result<()> {
    let path = dir.join(object_name(symbol));
    std::fs::write(&path, text).map_err(|e| Error::plain(format!("{}: {e}", path.display())))
}

/// Read the object cache index (keyed on declaration hashes: the first
/// column is the owning declaration's `decl_hash`).
pub fn read_cache_index(cache_dir: &Path) -> Vec<CacheRow> {
    let mut out = Vec::new();
    let Ok(text) = std::fs::read_to_string(cache_dir.join("index.tsv")) else {
        return out;
    };
    for line in text.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() != 4 {
            continue;
        }
        out.push(CacheRow {
            decl_hash: cols[0].to_string(),
            object_hash: cols[1].to_string(),
            symbol: cols[2].to_string(),
            decl: cols[3].to_string(),
        });
    }
    out
}

fn read_index(path: &Path) -> BTreeMap<String, CacheEntry> {
    let mut out = BTreeMap::new();
    for row in read_cache_index(path.parent().unwrap_or(Path::new("."))) {
        out.insert(
            row.symbol.clone(),
            CacheEntry {
                decl: row.decl,
                decl_hash: row.decl_hash,
                object_hash: row.object_hash,
            },
        );
    }
    out
}

fn write_index(path: &Path, index: &BTreeMap<String, CacheEntry>) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)
            .map_err(|e| Error::plain(format!("{}: {e}", dir.display())))?;
    }
    let mut text = String::new();
    for (symbol, e) in index {
        text.push_str(&format!(
            "{}\t{}\t{}\t{}\n",
            e.decl_hash, e.object_hash, symbol, e.decl
        ));
    }
    std::fs::write(path, text).map_err(|e| Error::plain(format!("{}: {e}", path.display())))
}

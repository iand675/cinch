//! Core text: the printer, reader, verifier and differential tool for the
//! annotated dump code generation consumes (`design/core-output.md`).
//!
//! The printer walks this compiler's own Core ([`crate::core::Program`]) and
//! prints what a stage actually carries. A fact the program does not record
//! prints as `-` (a type this compiler erased) rather than as a guess, and an
//! annotation the stage does not carry is omitted.
//!
//! # Stages
//!
//! The design note names four stages. This compiler fuses elaboration and
//! dictionary solving in one checking pass: `crate::check` infers, elaborates
//! to Core and resolves instances and evidence in the same walk, so there is
//! no program between `elaborated` and `solved`. Both names are accepted as
//! aliases of [`Stage::Checked`], which is what `cargo` sees after checking:
//!
//! | Stage | Pipeline point |
//! | --- | --- |
//! | `checked` | the `Program` a check returns, before `crate::rc` |
//! | `owned` | after `crate::rc::insert` over the checked program |
//! | `lowered` | after `crate::specialize::optimize` and the reference-count pass: exactly the code generation input |
//!
//! The pipeline runs specialization before reference counting, so `owned` is
//! not a prefix of `lowered` in this compiler: it is the count pass applied to
//! the checked program, which is what makes it target independent. `checked`
//! and `owned` must print byte identically for every target; only `lowered`
//! prints the target line and target-selected prims.
//!
//! # Determinism
//!
//! Nothing here iterates a hash table, prints an address or reads the clock.
//! Modules print sorted, units sorted by canonical name, annotations in the
//! documented order, alternatives in program order (lowering already sorts by
//! tag), and dictionary slots by slot number. Two runs over the same program
//! produce identical bytes.

use crate::core::{CPat, Core, FnDef, ForeignPlan, InstanceDef, LocalId, PrimTier, Program, TargetProfile};
use crate::interface::Iface;
use crate::repr::Class;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// ---------------------------------------------------------------- stages

/// A textual Core stage (design note section 2). This compiler fuses
/// `elaborated` and `solved` into one checking pass, so both names parse to
/// [`Stage::Checked`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    /// The program a check returns: instance constraints already resolved by
    /// the same pass that elaborates, no ownership, no target facts.
    Checked,
    /// After the reference-count pass: `dup`, `drop`, `release` and the
    /// borrowed parameters are explicit.
    Owned,
    /// After specialization and reference counting: exactly what code
    /// generation reads.
    Lowered,
}

impl Stage {
    /// Parse a `--stage`/`--dump-core` value. `elaborated` and `solved` are
    /// aliases of `checked` because this compiler's checker fuses the two.
    pub fn parse(text: &str) -> Option<Stage> {
        match text {
            "checked" | "elaborated" | "solved" => Some(Stage::Checked),
            "owned" => Some(Stage::Owned),
            "lowered" => Some(Stage::Lowered),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Stage::Checked => "checked",
            Stage::Owned => "owned",
            Stage::Lowered => "lowered",
        }
    }

    /// Every accepted spelling, in the order the help lists them.
    pub fn spellings() -> &'static [&'static str] {
        &["checked", "solved", "elaborated", "owned", "lowered"]
    }

    /// Whether the stage prints the target line (only `lowered` is target
    /// specific; an earlier stage must be byte identical across targets).
    pub fn is_target_specific(self) -> bool {
        matches!(self, Stage::Lowered)
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// ------------------------------------------------------------ signatures

/// Unit signature text, keyed by canonical unit name, taken from the checked
/// interfaces. The seed's `Core` carries no unit schemes: the checker records
/// them in the module interface, which is where [`crate::driver`] reads them
/// from. A unit with no entry prints `-` in the type slot.
#[derive(Clone, Debug, Default)]
pub struct Signatures {
    units: BTreeMap<String, String>,
    /// Every module the program was checked from: a unit's module is the
    /// longest dotted prefix that is one of them (`Main.main.lam22` belongs
    /// to `Main`, not to a module `Main.main`).
    modules: BTreeSet<String>,
}

impl Signatures {
    pub fn empty() -> Signatures {
        Signatures { units: BTreeMap::new(), modules: BTreeSet::new() }
    }

    /// Record a module the program was checked from.
    pub fn declare_module(&mut self, module: impl Into<String>) {
        self.modules.insert(module.into());
    }

    /// The module a unit belongs to: the longest dotted prefix that is a
    /// declared module, falling back to everything before the last dot.
    pub fn module_of(&self, unit: &str) -> String {
        module_of(unit, &self.modules)
    }

    pub fn insert(&mut self, unit: impl Into<String>, ty: impl Into<String>) {
        self.units.insert(unit.into(), canonical_type(&ty.into()));
    }

    pub fn get(&self, unit: &str) -> Option<&str> {
        self.units.get(unit).map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.units.len()
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    /// Collect every exported value's and constructor's canonical scheme.
    /// Instance and synthesized units are not exported and keep `-`.
    pub fn from_ifaces(ifaces: &[Iface]) -> Signatures {
        let mut signatures = Signatures::empty();
        for iface in ifaces {
            signatures.declare_module(iface.module.clone());
            for value in &iface.exports.values {
                signatures.insert(format!("{}.{}", iface.module, value.name), &value.scheme);
            }
            for ty in &iface.exports.types {
                for ctor in &ty.ctors {
                    signatures.insert(format!("{}.{}", iface.module, ctor.name), &ctor.scheme);
                }
            }
        }
        signatures
    }
}

/// Canonicalize a checked scheme for the type slot: one space before an arrow
/// on neither side, a single space after `forall`'s quantifier list before its
/// dot. The checker's own renderer writes `forall (a :: Type) . T`; the
/// printed form is `forall (a :: Type). T`.
fn canonical_type(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ' ' && i + 1 < chars.len() && chars[i + 1] == '.' {
            // Drop a space directly before a dot.
            i += 1;
            continue;
        }
        if chars[i] == '.' && out.ends_with('.') {
            i += 1;
            continue;
        }
        if chars[i] == ' ' && out.ends_with(' ') {
            i += 1;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    let mut out = out.trim().to_string();
    while let Some(inner) = outermost(&out) {
        out = inner.trim().to_string();
    }
    if out.is_empty() {
        "-".to_string()
    } else {
        out
    }
}

// -------------------------------------------------------------- contexts

/// Everything a dump needs beyond the program: the stage, the target, the
/// source identity and the facts the checker records outside [`Program`].
pub struct Context<'a> {
    pub stage: Stage,
    pub profile: &'a TargetProfile,
    pub source_digest: &'a str,
    pub opt_level: u8,
    pub signatures: &'a Signatures,
    /// The modules to dump; `None` dumps every unit of the program.
    pub modules: Option<&'a BTreeSet<String>>,
    /// Rewrite rules the optimizer applied, by unit (`OptSite::kind` is
    /// `rewrite:<rule>`).
    pub rewrites: &'a BTreeMap<String, Vec<String>>,
    /// The Core simplifier's tick count, when the dump is of a program the
    /// simplifier ran over (`--dump-core=lowered` after specialization).
    /// The header carries it as `@ticks <n>`: the count says how much work
    /// the pass did, so a change in it is visible beside the bodies it
    /// rewrote (design/compilation-speed.md section 10).
    pub ticks: Option<usize>,
}

impl<'a> Context<'a> {
    pub fn new(stage: Stage, profile: &'a TargetProfile, source_digest: &'a str) -> Context<'a> {
        Context {
            stage,
            profile,
            source_digest,
            opt_level: 0,
            signatures: empty_signatures(),
            modules: None,
            rewrites: empty_rewrites(),
            ticks: None,
        }
    }

    fn in_scope(&self, unit: &str) -> bool {
        match self.modules {
            None => true,
            Some(modules) => modules.contains(&self.signatures.module_of(unit)),
        }
    }
}

fn empty_signatures() -> &'static Signatures {
    static EMPTY: std::sync::LazyLock<Signatures> = std::sync::LazyLock::new(Signatures::empty);
    &EMPTY
}

fn empty_rewrites() -> &'static BTreeMap<String, Vec<String>> {
    static EMPTY: std::sync::LazyLock<BTreeMap<String, Vec<String>>> = std::sync::LazyLock::new(BTreeMap::new);
    &EMPTY
}

/// The module a canonical unit name belongs to: the longest dotted prefix
/// that is a declared module (`Prim.Sys.Darwin.load-i32` is module
/// `Prim.Sys.Darwin`, `Main.main.lam22` is module `Main`), else everything
/// before the last dot.
pub fn module_of(unit: &str, modules: &BTreeSet<String>) -> String {
    let mut best: Option<&str> = None;
    let mut end = 0;
    for (index, c) in unit.char_indices() {
        if c == '.' {
            let prefix = &unit[..index];
            if modules.contains(prefix) && best.map_or(true, |b| b.len() < prefix.len()) {
                best = Some(prefix);
                end = index;
            }
        }
    }
    let _ = end;
    match best {
        Some(prefix) => prefix.to_string(),
        None => match unit.rsplit_once('.') {
            Some((module, _)) => module.to_string(),
            None => String::new(),
        },
    }
}

// ---------------------------------------------------------------- printing

/// Print every unit of `program` at `stage` (design note sections 4 and 9).
pub fn print(program: &Program, stage: Stage, profile: &TargetProfile, source_digest: &str) -> String {
    print_with(program, &Context::new(stage, profile, source_digest))
}

/// The source identity the `source` line prints: the checked sources of the
/// dumped modules, sorted, hashed. A module the dump does not cover does not
/// move it, so two dumps of the same modules agree whatever else the build
/// compiled.
pub fn source_digest(ifaces: &[Iface], modules: &BTreeSet<String>) -> String {
    let mut identity = String::new();
    for iface in ifaces {
        if modules.contains(&iface.module) {
            identity.push_str(&iface.module);
            identity.push('\u{0}');
            identity.push_str(&iface.source_hash);
            identity.push('\n');
        }
    }
    format!("{:x}", Sha256::digest(identity.as_bytes()))
}

/// The composition every dump point uses, so the CLI, the build and the
/// tests print the same bytes for the same program: signatures from the
/// checked interfaces, the source identity over the dumped modules, the
/// module filter, and the rewrite applications specialization recorded.
pub fn dump(
    program: &Program,
    stage: Stage,
    profile: &TargetProfile,
    ifaces: &[Iface],
    modules: &BTreeSet<String>,
    rewrites: &BTreeMap<String, Vec<String>>,
    opt_level: u8,
) -> String {
    dump_with_ticks(program, stage, profile, ifaces, modules, rewrites, opt_level, None)
}

/// [`dump`] with the simplifier's tick count in the header (`@ticks <n>`).
#[allow(clippy::too_many_arguments)]
pub fn dump_with_ticks(
    program: &Program,
    stage: Stage,
    profile: &TargetProfile,
    ifaces: &[Iface],
    modules: &BTreeSet<String>,
    rewrites: &BTreeMap<String, Vec<String>>,
    opt_level: u8,
    ticks: Option<usize>,
) -> String {
    let signatures = Signatures::from_ifaces(ifaces);
    let digest = source_digest(ifaces, modules);
    let mut ctx = Context::new(stage, profile, &digest);
    ctx.signatures = &signatures;
    ctx.modules = Some(modules);
    ctx.rewrites = rewrites;
    ctx.opt_level = opt_level;
    ctx.ticks = ticks;
    print_with(program, &ctx)
}

/// Print the units in `ctx.modules` (all units when it is `None`).
pub fn print_with(program: &Program, ctx: &Context) -> String {
    let mut printer = Printer::new(program, ctx);
    printer.run()
}

struct Printer<'a> {
    program: &'a Program,
    ctx: &'a Context<'a>,
    fns: BTreeMap<String, &'a FnDef>,
    /// The printed name of every binder in the unit being printed: its
    /// parameters' own names where the artifact records one, and `t<n>` for
    /// every other binder, numbered in first-appearance order.
    names: BTreeMap<usize, String>,
    /// The names already taken in this unit, so a number is never reused.
    used: BTreeSet<String>,
    next_name: usize,
    /// The unit being printed, for the facts keyed by unit name (its frame
    /// allocation decision).
    unit: String,
}

/// One record of the module body: a function, a dictionary, or a foreign
/// binding. Records print sorted by canonical name.
enum Record<'a> {
    Fn(&'a FnDef),
    Instance(&'a InstanceDef),
    Foreign(&'a ForeignPlan),
}

impl<'a> Printer<'a> {
    fn new(program: &'a Program, ctx: &'a Context<'a>) -> Self {
        let mut fns = BTreeMap::new();
        for f in &program.fns {
            fns.entry(f.id.0.clone()).or_insert(f);
        }
        Printer {
            program,
            ctx,
            fns,
            names: BTreeMap::new(),
            used: BTreeSet::new(),
            next_name: 0,
            unit: String::new(),
        }
    }

    fn run(&mut self) -> String {
        let mut records: Vec<(String, Record<'a>)> = Vec::new();
        for f in &self.program.fns {
            if self.ctx.in_scope(&f.id.0) {
                records.push((f.id.0.clone(), Record::Fn(f)));
            }
        }
        for instance in &self.program.instances {
            if self.ctx.in_scope(&instance.id.0) {
                records.push((instance.id.0.clone(), Record::Instance(instance)));
            }
        }
        for plan in self.program.foreigns.values() {
            if self.ctx.in_scope(&plan.unit) {
                records.push((plan.unit.clone(), Record::Foreign(plan)));
            }
        }
        records.sort_by(|a, b| a.0.cmp(&b.0));
        records.dedup_by(|a, b| a.0 == b.0);

        let printed: BTreeSet<String> = records.iter().map(|(name, _)| name.clone()).collect();
        let mut modules: BTreeSet<String> = BTreeSet::new();
        for name in &printed {
            let module = self.ctx.signatures.module_of(name);
            if !module.is_empty() {
                modules.insert(module);
            }
        }
        let referenced = self.referenced(&records, &printed);

        let mut out = String::new();
        out.push_str("core 1\n");
        for module in &modules {
            out.push_str("module ");
            out.push_str(module);
            out.push('\n');
        }
        if self.ctx.stage.is_target_specific() {
            out.push_str(&format!(
                "target {} opt-level {}\n",
                self.ctx.profile.triple, self.ctx.opt_level
            ));
        }
        out.push_str(&format!("source sha256:{}\n", self.ctx.source_digest));
        if let Some(ticks) = self.ctx.ticks {
            out.push_str(&format!("@ticks {ticks}\n"));
        }
        for name in &referenced {
            out.push_str("global ");
            out.push_str(&ident(name));
            out.push('\n');
        }
        for (_, record) in &records {
            match record {
                Record::Fn(f) => self.fn_unit(f, &mut out),
                Record::Instance(instance) => self.instance_unit(instance, &mut out),
                Record::Foreign(plan) => self.foreign_unit(plan, &mut out),
            }
        }
        out
    }

    /// Names the printed bodies reference but the dump does not define: a
    /// filtered dump declares them so every reference still resolves.
    fn referenced(&mut self, records: &[(String, Record<'a>)], printed: &BTreeSet<String>) -> BTreeSet<String> {
        let mut referenced: BTreeSet<String> = BTreeSet::new();
        for (_, record) in records {
            match record {
                Record::Fn(f) => {
                    walk_units(&f.body, &mut |name| {
                        referenced.insert(name.to_string());
                    });
                }
                Record::Instance(instance) => {
                    for method in &instance.methods {
                        referenced.insert(method.0.clone());
                    }
                }
                Record::Foreign(_) => {}
            }
        }
        referenced.retain(|name| !printed.contains(name));
        referenced
    }

    /// The frame site of local `id` in the unit being printed, when the
    /// stack allocation pass gave it one.
    fn frame_site(&self, id: LocalId) -> Option<&crate::stack::Site> {
        self.program.frame.get(&self.unit).and_then(|plan| plan.site(id.0))
    }

    fn fn_unit(&mut self, f: &FnDef, out: &mut String) {
        self.reset_names(f);
        let ty = self.ctx.signatures.get(&f.id.0).unwrap_or("-").to_string();
        out.push_str(&format!("unit {} :: {}\n", ident(&f.id.0), ty));
        let mut lines = self.fn_annotations(f);
        if !f.params.is_empty() {
            lines.push((rank("params"), format!("@params {}", self.params_line(f))));
        }
        lines.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        for (_, line) in lines {
            out.push_str("  ");
            out.push_str(&line);
            out.push('\n');
        }
        out.push_str("  body ");
        let node = self.body(f);
        out.push_str(&render_body(&node));
        out.push_str("end\n");
    }

    fn body(&mut self, f: &FnDef) -> Node {
        if f.meta.prim_body {
            if let Some(plan) = self.program.prims.iter().find(|p| p.unit == f.id.0) {
                if let Some(body) = plan.bodies.get(plan.selected) {
                    return Node::list("prim-body")
                        .child(Node::atom(ident(&body.family)))
                        .child(Node::atom(format!("\"{}\"", escape_text(&body.fragment))));
                }
            }
            return Node::list("prim-body");
        }
        self.expr(&f.body)
    }

    fn fn_annotations(&self, f: &FnDef) -> Vec<(u8, String)> {
        let mut anns: Vec<(u8, String)> = Vec::new();
        if let Some((tag, arity)) = f.ctor {
            let data = match &f.repr.data {
                Some(data) => ident(data),
                None => "-".to_string(),
            };
            anns.push((rank("ctor"), format!("@ctor({tag}, {arity}, {data})")));
        }
        if f.meta.prim_body {
            if let Some(plan) = self.program.prims.iter().find(|p| p.unit == f.id.0) {
                let tier = match plan.tier {
                    PrimTier::Justified => "justified",
                    PrimTier::Assumed => "assumed",
                };
                anns.push((rank("tier"), format!("@tier({tier})")));
                anns.push((rank("prim"), format!("@prim({} for {})", plan.name, plan.selected_why)));
            }
        }
        if let Some(lent) = self.program.lent.get(&f.id.0) {
            let borrowed: Vec<String> = lent
                .iter()
                .enumerate()
                .filter(|(_, b)| **b)
                .map(|(i, _)| (i + 1).to_string())
                .collect();
            if !borrowed.is_empty() {
                anns.push((rank("borrowed"), format!("@borrowed({})", borrowed.join(","))));
            }
        }
        if let Some(plan) = self.program.frame.get(&f.id.0).filter(|p| !p.is_empty()) {
            // Frame allocation (design/stack-allocation.md): the slots this
            // unit reserves and the bytes they add to its frame.
            anns.push((
                rank("frame"),
                format!("@frame({} slot{}, {} bytes)", plan.slot_count(), plural(plan.slot_count()), plan.bytes),
            ));
        }
        if let Some(facts) = self.program.inline.get(&f.id.0) {
            // The inliner's copies in this unit (`inline` rule): the sites
            // and the callees their bodies came from.
            let mut callees = facts.callees.clone();
            callees.sort();
            callees.dedup();
            anns.push((
                rank("inline"),
                format!(
                    "@inline({} site{}, {} nodes: {})",
                    facts.sites,
                    plural(facts.sites as usize),
                    facts.nodes,
                    callees.join(", ")
                ),
            ));
        }
        if let Some((base, letters)) = specialisation(&f.id.0) {
            anns.push((rank("specialised"), format!("@specialised({letters}) from {}", ident(&base))));
        }
        if let Some(rules) = self.ctx.rewrites.get(&f.id.0) {
            let mut unique = rules.clone();
            unique.sort();
            unique.dedup();
            for rule in unique {
                anns.push((rank("rewrite"), format!("@rewrite({rule})")));
            }
        }
        anns.push((rank("stage"), format!("@stage({})", self.ctx.stage.name())));
        anns
    }

    fn params_line(&mut self, f: &FnDef) -> String {
        let types = self.param_types(f);
        let mut parts = Vec::new();
        for index in 0..f.params.len() {
            let name = self.local(LocalId(index));
            let class = match f.repr.params.get(index) {
                Some(class) => class_text(*class),
                None => "-".to_string(),
            };
            let ty = match types.get(index) {
                Some(ty) => ty.clone(),
                None => "-".to_string(),
            };
            parts.push(format!("{name} :: {ty} @{class}"));
        }
        parts.join(", ")
    }

    /// Parameter types, where the unit's scheme decomposes into the runtime
    /// arrows exactly. A dictionary parameter prints the class it carries; a
    /// count that does not line up (an erased evidence binder, a scheme the
    /// checker did not record) prints `-` for every parameter rather than a
    /// guess.
    fn param_types(&self, f: &FnDef) -> Vec<String> {
        let unknown = vec!["-".to_string(); f.params.len()];
        let Some(scheme) = self.ctx.signatures.get(&f.id.0) else {
            return unknown;
        };
        let Some(domains) = arrow_domains(scheme) else {
            return unknown;
        };
        let dicts: BTreeMap<usize, &str> =
            f.meta.dicts.iter().map(|(index, class)| (*index, class.as_str())).collect();
        let mut types = Vec::with_capacity(f.params.len());
        let mut next = 0;
        for index in 0..f.params.len() {
            if let Some(class) = dicts.get(&index) {
                types.push((*class).to_string());
                continue;
            }
            match domains.get(next) {
                Some(domain) => {
                    types.push(domain.clone());
                    next += 1;
                }
                None => return unknown,
            }
        }
        if next != domains.len() {
            return unknown;
        }
        types
    }

    fn instance_unit(&self, instance: &InstanceDef, out: &mut String) {
        out.push_str(&format!("unit {} :: {}\n", ident(&instance.id.0), instance.class));
        out.push_str(&format!(
            "  @instance(tparams {}, subdicts {})\n",
            instance.tparams, instance.subdict_count
        ));
        out.push_str(&format!("  @stage({})\n", self.ctx.stage.name()));
        let slots: Vec<String> = instance
            .methods
            .iter()
            .enumerate()
            .map(|(slot, method)| format!("{slot} = {}", ident(&method.0)))
            .collect();
        out.push_str(&format!("  static {{{}}}\n", slots.join(", ")));
        out.push_str("end\n");
    }

    fn foreign_unit(&self, plan: &ForeignPlan, out: &mut String) {
        out.push_str(&format!("unit {} :: -\n", ident(&plan.unit)));
        out.push_str(&format!("  @ffi(\"{}\")\n", escape_text(&plan.symbol)));
        out.push_str(&format!("  @stage({})\n", self.ctx.stage.name()));
        out.push_str("  body ");
        out.push_str(&render_body(&Node::list("foreign").child(Node::atom(format!("\"{}\"", escape_text(&plan.symbol))))));
        out.push_str("end\n");
    }

    fn expr(&mut self, e: &Core) -> Node {
        match e {
            Core::Int(value) => Node::list("int").child(Node::atom(value.to_string())),
            Core::ConstBig { nat: true, digits } => {
                Node::list("nat").child(Node::atom(format!("\"{}\"", escape_text(digits))))
            }
            Core::ConstBig { nat: false, digits } => match digits.parse::<i64>() {
                Ok(value) => Node::list("int").child(Node::atom(value.to_string())),
                Err(_) => Node::list("bigint").child(Node::atom(format!("\"{}\"", escape_text(digits)))),
            },
            Core::ConstBytes(bytes) => bytes_node("bytes", bytes),
            Core::ConstString { bytes, .. } => bytes_node("string", bytes),
            Core::ConstSym(name) => Node::list("sym").child(Node::atom(ident(name))),
            Core::Bool(value) => Node::list("bool").child(Node::atom(if *value { "true" } else { "false" })),
            Core::Unit => Node::list("unit"),
            Core::Var(id) => Node::list("local").child(Node::atom(self.local(*id))),
            Core::SpliceVal(index) => Node::list("splice").child(Node::atom(index.to_string())),
            Core::Global(id, classes) => classes_mark(Node::list("global").child(Node::atom(ident(&id.0))), classes),
            Core::Ctor(id, classes) => {
                let (tag, arity) = match self.fns.get(&id.0).and_then(|f| f.ctor) {
                    Some((tag, arity)) => (tag.to_string(), arity.to_string()),
                    None => ("-".to_string(), "-".to_string()),
                };
                classes_mark(
                    Node::list("ctor")
                        .child(Node::atom(ident(&id.0)))
                        .child(Node::atom(tag))
                        .child(Node::atom(arity)),
                    classes,
                )
            }
            Core::Prim(op) => Node::list("prim").child(Node::atom(ident(&op.symbol()))),
            Core::Foreign(unit) => Node::list("foreign").child(Node::atom(format!("\"{}\"", escape_text(unit)))),
            Core::DictProj(dict, slot, site, _) => {
                let class = match site {
                    Some(site) => ident(&site.class),
                    None => "-".to_string(),
                };
                Node::list("dictproj")
                    .child(self.expr(dict))
                    .child(Node::atom(slot.to_string()))
                    .child(Node::atom(class))
            }
            Core::Proj(record, field) => Node::list("proj")
                .child(self.expr(record))
                .child(Node::atom(field.slot.to_string()))
                .mark(format!("@class({})", class_text(field.class))),
            Core::InstDict(id, args, classes) => {
                let mut node = Node::list("instdict").child(Node::atom(ident(&id.0)));
                for arg in args {
                    node = node.child(self.expr(arg));
                }
                classes_mark(node, classes)
            }
            Core::App(callee, args) => {
                let mut node = Node::list("app").child(self.expr(callee));
                for arg in args {
                    node = node.child(self.expr(arg));
                }
                node
            }
            Core::Lam { fn_id, env, classes } => {
                // The binder list sits in this unit, so its formals take this
                // unit's next `t<n>`: a closure's parameter can never share a
                // name with a binder of the unit that builds it. The hoisted
                // unit's own `@params` line numbers its formals within itself.
                let arity = match self.fns.get(&fn_id.0) {
                    Some(f) => f.params.len().saturating_sub(env.len()),
                    None => 0,
                };
                let own: Vec<String> = (0..arity).map(|_| self.fresh_name()).collect();
                let mut params = Node::list("");
                for name in own {
                    params = params.child(Node::atom(name));
                }
                let mut node = Node::list("lam").child(params);
                if !env.is_empty() {
                    let mut captured = Node::list("env");
                    for value in env {
                        captured = captured.child(self.expr(value));
                    }
                    node = node.child(captured);
                }
                node = node.child(self.expr(&Core::Global(fn_id.clone(), Vec::new())));
                classes_mark(node, classes)
            }
            Core::Let(id, class, value, body) => {
                let slot = self.frame_site(*id).map(|site| site.slot);
                let mut value = self.expr(value);
                if let Some(slot) = slot {
                    // The allocation is built in the frame slot `f<n>`.
                    value = value.mark(format!("@frame(f{slot})"));
                }
                Node::list("let")
                    .child(Node::atom(self.local(*id)))
                    .mark(format!("@class({})", class_text(*class)))
                    .child(value)
                    .child(self.expr(body))
            }
            Core::Join { id, params, body, rest } => {
                let mut plist = Node::list("params");
                for (param, class) in params {
                    plist = plist.child(
                        Node::list(self.local(*param))
                            .mark(format!("@class({})", class_text(*class))),
                    );
                }
                Node::list("join")
                    .child(Node::atom(id.to_string()))
                    .child(plist)
                    .child(self.expr(body))
                    .child(self.expr(rest))
            }
            Core::Jump { id, args } => {
                let mut node = Node::list("jump").child(Node::atom(id.to_string()));
                for arg in args {
                    node = node.child(self.expr(arg));
                }
                node
            }
            Core::If(cond, then, other) => Node::list("if")
                .child(self.expr(cond))
                .child(weighted(self.expr(&then.body), then.weight))
                .child(weighted(self.expr(&other.body), other.weight)),
            Core::Case { scrut, alts } => {
                let mut node = Node::list("case").child(self.expr(scrut));
                for alt in alts {
                    node = node.child(alt_node(&self.pattern(&alt.pat), self.expr(&alt.body), alt.weight));
                }
                node
            }
            Core::Demand(cell, class) => Node::list("demand")
                .child(self.expr(cell))
                .mark(format!("@class({})", class_text(*class))),
            Core::LazyAlloc => Node::list("lazy-alloc"),
            Core::LazyInit(cell, value) => {
                Node::list("lazy-init").child(Node::atom(self.local(*cell))).child(self.expr(value))
            }
            Core::BoxAlloc => Node::list("box-alloc"),
            Core::BoxInit(cell, value, class) => Node::list("boxinit")
                .child(Node::atom(self.local(*cell)))
                .mark(format!("@class({})", class_text(*class)))
                .child(self.expr(value)),
            Core::BoxGet(box_, class) => Node::list("boxget")
                .child(self.expr(box_))
                .mark(format!("@class({})", class_text(*class))),
            Core::BoxClear(box_, class) => Node::list("boxclear")
                .child(self.expr(box_))
                .mark(format!("@class({})", class_text(*class))),
            Core::RefSet(cell, value, class) => Node::list("refset")
                .child(Node::atom(self.local(*cell)))
                .mark(format!("@class({})", class_text(*class)))
                .child(self.expr(value)),
            Core::Fault => Node::list("fault"),
            Core::Dup(id, class, body) => Node::list("dup")
                .child(Node::atom(self.local(*id)))
                .mark(format!("@class({})", class_text(*class)))
                .child(self.expr(body)),
            Core::Drop(id, class, body) => Node::list("drop")
                .child(Node::atom(self.local(*id)))
                .mark(format!("@class({})", class_text(*class)))
                .child(self.expr(body)),
            Core::Release { block, moved, rest, body } => {
                let mut movedlist = Node::list("moved");
                for (id, _) in moved {
                    movedlist = movedlist.child(Node::atom(self.local(*id)));
                }
                let mut restlist = Node::list("rest");
                for (id, _) in rest {
                    restlist = restlist.child(Node::atom(self.local(*id)));
                }
                Node::list("release")
                    .child(Node::atom(self.local(*block)))
                    .child(movedlist)
                    .child(restlist)
                    .child(self.expr(body))
            }
            Core::Quote { holes, body, .. } => {
                let mut holelist = Node::list("holes");
                for hole in holes {
                    holelist = holelist.child(self.expr(hole));
                }
                let body = match body {
                    Some(body) => self.expr(body),
                    None => Node::list("template"),
                };
                Node::list("quote")
                    .child(Node::atom(holes.len().to_string()))
                    .child(holelist)
                    .child(body)
            }
            Core::DeclsQuote { holes, name_holes, .. } => {
                let mut holelist = Node::list("holes");
                for hole in holes {
                    holelist = holelist.child(self.expr(hole));
                }
                let mut names = Node::list("names");
                for index in name_holes {
                    names = names.child(Node::atom(index.to_string()));
                }
                Node::list("decls-quote")
                    .child(Node::atom(holes.len().to_string()))
                    .child(holelist)
                    .child(names)
            }
            Core::SyntaxLit(tokens, _) => {
                Node::list("syntaxlit").child(Node::atom(tokens.len().to_string()))
            }
            Core::StrLit(text) => Node::list("strlit").child(Node::atom(format!("\"{}\"", escape_text(text)))),
        }
    }

    fn pattern(&mut self, pat: &CPat) -> Node {
        match pat {
            CPat::Wild => Node::atom("_"),
            CPat::Var(id, class) => Node::list(self.local(*id)).mark(format!("@class({})", class_text(*class))),
            CPat::Con { tag, fields, classes } => {
                let mut node = Node::list("con").child(Node::atom(tag.to_string()));
                for (index, field) in fields.iter().enumerate() {
                    let mut binder = Node::list(self.local(*field));
                    if let Some(class) = classes.get(index) {
                        binder = binder.mark(format!("@class({})", class_text(*class)));
                    }
                    node = node.child(binder);
                }
                node
            }
        }
    }

    /// Start a unit's naming: its parameters keep their recorded names where
    /// the artifact has one, and every other binder draws from `t<n>` in
    /// first-appearance order. Two binders of one unit never share a name, so
    /// a shadowed source binder prints as two distinct `t<n>`.
    fn reset_names(&mut self, f: &FnDef) {
        self.unit.clone_from(&f.id.0);
        self.names.clear();
        self.used.clear();
        let naming = parameter_naming(f);
        for (index, name) in naming.names.into_iter().enumerate() {
            self.used.insert(name.clone());
            self.names.insert(index, name);
        }
        self.next_name = naming.next;
    }

    /// The printed name of a binder. Assigning here, at first use, is what
    /// makes the numbering first-appearance order and independent of `LocalId`
    /// values, which are per-unit counters that carry no meaning in the text.
    fn local(&mut self, id: LocalId) -> String {
        if let Some(name) = self.names.get(&id.0) {
            return name.clone();
        }
        let name = self.fresh_name();
        self.names.insert(id.0, name.clone());
        name
    }

    /// The next `t<n>` this unit has not used: a binder with no `LocalId` of
    /// this unit (a closure's formals) takes one too.
    fn fresh_name(&mut self) -> String {
        loop {
            let candidate = format!("t{}", self.next_name);
            self.next_name += 1;
            if !self.used.contains(&candidate) {
                self.used.insert(candidate.clone());
                return candidate;
            }
        }
    }
}

/// A unit's parameter names in order, and the next `t<n>` number: a
/// recorded name is kept, and every placeholder takes the next free `t<n>`.
/// This is one function, not two, so a `lam` node names its formals exactly as
/// the hoisted unit's own `@params` line does.
struct Naming {
    names: Vec<String>,
    next: usize,
}

fn parameter_naming(f: &FnDef) -> Naming {
    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut names: Vec<Option<String>> = Vec::with_capacity(f.params.len());
    for name in &f.params {
        if synthesized_name(name) {
            names.push(None);
        } else {
            used.insert(name.clone());
            names.push(Some(name.clone()));
        }
    }
    let mut next = 0;
    for slot in names.iter_mut() {
        if slot.is_some() {
            continue;
        }
        loop {
            let candidate = format!("t{next}");
            next += 1;
            if !used.contains(&candidate) {
                *slot = Some(candidate);
                break;
            }
        }
    }
    Naming { names: names.into_iter().flatten().collect(), next }
}

/// Whether a recorded parameter name is the checker's placeholder
/// (`v4343`) rather than a name worth printing. Constructors record `f0`,
/// primitives `p0` and generated units record their own words, and those
/// print as they are.
fn synthesized_name(name: &str) -> bool {
    match name.strip_prefix('v') {
        Some(digits) => !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()),
        None => false,
    }
}

fn alt_node(pattern: &Node, body: Node, weight: Option<u32>) -> Node {
    weighted(Node::list("").child(pattern.clone()).child(body), weight)
}

fn weighted(node: Node, weight: Option<u32>) -> Node {
    match weight {
        Some(weight) => node.mark(format!("@weight({weight})")),
        None => node,
    }
}

fn classes_mark(node: Node, classes: &[Class]) -> Node {
    if classes.is_empty() {
        node
    } else {
        let letters: String = classes.iter().copied().map(class_letter).collect();
        node.mark(format!("@classes({letters})"))
    }
}

fn class_letter(class: Class) -> char {
    match class {
        Class::Raw => 'R',
        Class::Obj => 'O',
        Class::TObj => 'T',
        Class::Param(index) => char::from(b'0' + (index % 10) as u8),
    }
}

/// The ownership class a representation class prints (design note section
/// 3.4): `Raw` words are never counted, counted pointers are owned by their
/// binder, and an open type parameter keeps its vector position.
fn class_text(class: Class) -> String {
    match class {
        Class::Raw => "uncounted".to_string(),
        Class::Obj | Class::TObj => "owned".to_string(),
        Class::Param(index) => format!("param({index})"),
    }
}

/// The `@<letters>` specialisation suffix of a unit name, with its generic
/// base (`Main.pair@RO` is `(Main.pair, "RO")`).
fn specialisation(unit: &str) -> Option<(String, String)> {
    let base = crate::repr::base_unit(unit);
    if base.as_ref() == unit {
        return None;
    }
    let letters = &unit[base.len() + 1..];
    if letters.is_empty() {
        None
    } else {
        Some((base.into_owned(), letters.to_string()))
    }
}

fn bytes_node(head: &str, bytes: &[u8]) -> Node {
    if bytes.len() > 32 {
        // A long constant prints its digest and length instead of its
        // contents (design note section 4): the bytes are recoverable from
        // the source, the dump stays readable.
        let digest = format!("{:x}", Sha256::digest(bytes));
        Node::list(head).child(Node::atom(digest)).child(Node::atom(bytes.len().to_string()))
    } else {
        Node::list(head)
            .child(Node::atom(format!("\"{}\"", escape_bytes(bytes))))
            .child(Node::atom(bytes.len().to_string()))
    }
}

/// Identifiers print as themselves when they use only `A-Za-z0-9_.-$` and
/// are quoted with `'...'` otherwise; a quote inside is doubled.
fn ident(name: &str) -> String {
    let plain = |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-' | '$');
    if !name.is_empty() && name.chars().all(plain) {
        name.to_string()
    } else {
        format!("'{}'", name.replace('\'', "''"))
    }
}

fn escape_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn escape_bytes(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len());
    for byte in bytes {
        match byte {
            b'\\' => out.push_str("\\\\"),
            b'"' => out.push_str("\\\""),
            b'\n' => out.push_str("\\n"),
            b'\t' => out.push_str("\\t"),
            0x20..=0x7e => out.push(*byte as char),
            byte => out.push_str(&format!("\\x{byte:02x}")),
        }
    }
    out
}

/// Split a canonical scheme's arrow chain: its runtime arrow domains in
/// order. `None` when the text is not a chain of arrows (a bare type
/// parameter, a scheme with an unrecorded binder).
fn arrow_domains(scheme: &str) -> Option<Vec<String>> {
    let mut body = scheme.trim();
    if body.starts_with("forall ") {
        // Drop the quantifier block up to its closing dot at depth zero.
        let mut depth = 0i32;
        let mut end = None;
        for (index, c) in body.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '.' if depth == 0 => {
                    end = Some(index);
                    break;
                }
                _ => {}
            }
        }
        body = body.get(end? + 1..)?.trim();
    }
    if body.contains("=> ") {
        body = match body.rsplit_once("=> ") {
            Some((_, rest)) => rest.trim(),
            None => body,
        };
    }
    let mut current = body.to_string();
    while let Some(inner) = outermost(&current) {
        current = inner.trim().to_string();
    }
    let mut domains = Vec::new();
    loop {
        match split_arrow(&current) {
            Some((domain, result)) => {
                if domain.trim().is_empty() {
                    return None;
                }
                domains.push(domain.trim().to_string());
                let mut next = result.trim().to_string();
                while let Some(inner) = outermost(&next) {
                    next = inner.trim().to_string();
                }
                if next.is_empty() {
                    return None;
                }
                current = next;
            }
            None => break,
        }
    }
    if domains.is_empty() || current.trim().is_empty() {
        None
    } else {
        Some(domains)
    }
}

/// Split at the first arrow outside parentheses.
fn split_arrow(text: &str) -> Option<(String, String)> {
    let chars: Vec<char> = text.chars().collect();
    let mut depth = 0i32;
    let mut index = 0usize;
    while index < chars.len() {
        match chars[index] {
            '(' => depth += 1,
            ')' => depth -= 1,
            '-' if depth == 0 && chars.get(index + 1) == Some(&'>') => {
                let domain: String = chars[..index].iter().collect();
                let result: String = chars[index + 2..].iter().collect();
                return Some((domain, result));
            }
            _ => {}
        }
        index += 1;
    }
    None
}

/// `Some(inner)` when the whole text is one parenthesized group.
fn outermost(text: &str) -> Option<&str> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    if chars.first().map(|(_, c)| *c) != Some('(') {
        return None;
    }
    let mut depth = 0i32;
    for (index, c) in &chars {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    let end = index + 1;
                    if end == text.len() {
                        return Some(text.get(1..text.len() - 1)?);
                    }
                    return None;
                }
            }
            _ => {}
        }
    }
    None
}

// ------------------------------------------------------------- s-expressions

/// A printed node: an atom, or a bracketed form with a head, its `@` marks
/// and its children. Marks print after the head and before the children so a
/// node's annotations never look like arguments.
#[derive(Clone, Debug, PartialEq)]
enum Node {
    Atom(String),
    List { head: String, marks: Vec<String>, tail: Vec<Node> },
}

impl Node {
    fn atom(text: impl Into<String>) -> Node {
        Node::Atom(text.into())
    }

    fn list(head: impl Into<String>) -> Node {
        Node::List { head: head.into(), marks: Vec::new(), tail: Vec::new() }
    }

    fn child(mut self, child: Node) -> Node {
        if let Node::List { tail, .. } = &mut self {
            tail.push(child);
        }
        self
    }

    fn mark(mut self, mark: impl Into<String>) -> Node {
        if let Node::List { marks, .. } = &mut self {
            marks.push(mark.into());
        }
        self
    }

    /// The node on one line. A mark prints after the node's first child
    /// (after the head when it has none), so a form's name and its `@`
    /// annotations never trade places: `(global Data.List.map @classes(TT))`,
    /// `(let n1 @class(owned) value body)`.
    fn flat(&self) -> String {
        match self {
            Node::Atom(text) => text.clone(),
            Node::List { head, marks, tail } => {
                let mut out = String::from("(");
                out.push_str(head);
                for child in tail.iter().take(1) {
                    if !out.ends_with('(') {
                        out.push(' ');
                    }
                    out.push_str(&child.flat());
                }
                for mark in marks {
                    out.push(' ');
                    out.push_str(mark);
                }
                for child in tail.iter().skip(1) {
                    out.push(' ');
                    out.push_str(&child.flat());
                }
                out.push(')');
                out
            }
        }
    }
}

/// The widest a node may print on one line, indentation included.
const LINE_WIDTH: usize = 100;

fn render(node: &Node, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    let flat = node.flat();
    if pad.len() + flat.chars().count() <= LINE_WIDTH {
        out.push_str(&pad);
        out.push_str(&flat);
        out.push('\n');
        return;
    }
    match node {
        Node::Atom(text) => {
            out.push_str(&pad);
            out.push_str(text);
            out.push('\n');
        }
        Node::List { head, marks, tail } if !tail.is_empty() => {
            let first = tail.first().map(Node::flat).unwrap_or_default();
            let mut open = String::from("(");
            open.push_str(head);
            let mut inline = 0usize;
            if pad.len() + open.len() + 1 + first.chars().count() <= LINE_WIDTH {
                if !open.ends_with('(') {
                    open.push(' ');
                }
                open.push_str(&first);
                inline = 1;
            }
            for mark in marks {
                open.push(' ');
                open.push_str(mark);
            }
            out.push_str(&pad);
            out.push_str(&open);
            out.push('\n');
            for child in tail.iter().skip(inline) {
                render(child, depth + 1, out);
            }
            if out.ends_with('\n') {
                out.pop();
            }
            out.push_str(")\n");
        }
        Node::List { .. } => {
            out.push_str(&pad);
            out.push_str(&flat);
            out.push('\n');
        }
    }
}

/// Render a body expression at depth one and drop its first line's indent, so
/// it sits on the `body` line.
fn render_body(node: &Node) -> String {
    let mut text = String::new();
    render(node, 1, &mut text);
    match text.strip_prefix("  ") {
        Some(rest) => rest.to_string(),
        None => text,
    }
}

/// Every top-level unit a body references: globals, constructors, foreign
/// wrappers, hoisted lambdas and dictionary units.
fn walk_units(body: &Core, f: &mut impl FnMut(&str)) {
    match body {
        Core::Global(id, _) | Core::Ctor(id, _) => f(&id.0),
        Core::Foreign(unit) => f(unit),
        Core::InstDict(id, args, _) => {
            f(&id.0);
            for arg in args {
                walk_units(arg, f);
            }
        }
        Core::Lam { fn_id, env, .. } => {
            f(&fn_id.0);
            for value in env {
                walk_units(value, f);
            }
        }
        Core::ConstBig { .. }
        | Core::Int(_)
        | Core::ConstBytes(_)
        | Core::ConstString { .. }
        | Core::ConstSym(_)
        | Core::Bool(_)
        | Core::Unit
        | Core::Var(_)
        | Core::SpliceVal(_)
        | Core::Prim(_)
        | Core::LazyAlloc
        | Core::BoxAlloc
        | Core::Fault
        | Core::SyntaxLit(..)
        | Core::StrLit(_) => {}
        Core::DictProj(dict, _, _, _) | Core::Proj(dict, _) | Core::Demand(dict, _) | Core::BoxGet(dict, _) | Core::BoxClear(dict, _) => {
            walk_units(dict, f)
        }
        Core::App(callee, args) => {
            walk_units(callee, f);
            for arg in args {
                walk_units(arg, f);
            }
        }
        Core::Let(_, _, value, body) => {
            walk_units(value, f);
            walk_units(body, f);
        }
        Core::Join { body, rest, .. } => {
            walk_units(body, f);
            walk_units(rest, f);
        }
        Core::Jump { args, .. } => {
            for arg in args {
                walk_units(arg, f);
            }
        }
        Core::If(cond, then, other) => {
            walk_units(cond, f);
            walk_units(&then.body, f);
            walk_units(&other.body, f);
        }
        Core::Case { scrut, alts } => {
            walk_units(scrut, f);
            for alt in alts {
                walk_units(&alt.body, f);
            }
        }
        Core::LazyInit(_, value) | Core::BoxInit(_, value, _) | Core::Dup(_, _, value) | Core::Drop(_, _, value) => {
            walk_units(value, f)
        }
        Core::RefSet(_, value, _) => walk_units(value, f),
        Core::Release { body, .. } => walk_units(body, f),
        Core::Quote { holes, body, .. } => {
            for hole in holes {
                walk_units(hole, f);
            }
            if let Some(body) = body {
                walk_units(body, f);
            }
        }
        Core::DeclsQuote { holes, .. } => {
            for hole in holes {
                walk_units(hole, f);
            }
        }
    }
}

// ---------------------------------------------------------------- reading

/// One unit the reader reconstructed: its name, type text, annotations in the
/// order they printed, its parameters and its body or static slot table.
#[derive(Clone, Debug)]
struct ParsedUnit {
    name: String,
    ty: String,
    anns: Vec<(String, String)>,
    params: Vec<String>,
    body: Option<Node>,
    statics: Vec<(u64, String)>,
    has_static: bool,
}

impl ParsedUnit {
}

#[derive(Clone, Debug, Default)]
struct Parsed {
    modules: Vec<String>,
    globals: Vec<String>,
    target: Option<(String, u32)>,
    source: Option<String>,
    /// The simplifier's tick count, when the header carries `@ticks <n>`.
    ticks: Option<u64>,
    units: Vec<ParsedUnit>,
}

/// Read a Core file's structure. Every violation the reader finds becomes an
/// [`Error`]; `None` means the file was not readable at all.
fn parse(text: &str, errors: &mut Vec<Error>) -> Option<Parsed> {
    let lines: Vec<&str> = text.lines().collect();
    // The reader reports the line it rejects, so every finding carries the
    // range the language server highlights.
    let src = Source::new(text);
    let mut file = Parsed::default();
    if lines.is_empty() {
        errors.push(Error::header("empty file"));
        return None;
    }
    if lines[0] != "core 1" {
        errors.push(
            Error::header(format!("the first line is `{}`, not `core 1`", lines[0]))
                .at(src.line_rng(0)),
        );
        return None;
    }
    let mut i = 1;
    // Header stage: 0 modules, 1 target, 2 source, 3 globals. A line that
    // moves the stage backwards is a grammar violation.
    let mut stage = 0u8;
    while i < lines.len() {
        let line = lines[i];
        let here = src.line_rng(i);
        if let Some(rest) = line.strip_prefix("module ") {
            if stage > 0 {
                errors.push(
                    Error::header(format!("`module` after the header's target line: `{line}`")).at(here),
                );
            }
            stage = 0;
            if rest.trim().is_empty() || rest.contains(' ') {
                errors.push(Error::header(format!("module name `{rest}` is not one name")).at(here));
            }
            if file.modules.iter().any(|m| m == rest) {
                errors.push(Error::header(format!("module `{rest}` is declared twice")).at(here));
            }
            if file.modules.last().is_some_and(|last| last.as_str() > rest) {
                errors.push(Error::header(format!("module `{rest}` is out of order")).at(here));
            }
            file.modules.push(rest.to_string());
        } else if let Some(rest) = line.strip_prefix("target ") {
            if stage > 1 {
                errors.push(Error::header(format!("`target` after `source`: `{line}`")).at(here));
            }
            stage = 1;
            if file.target.is_some() {
                errors.push(Error::header("`target` is declared twice").at(here));
            }
            match rest.rsplit_once(" opt-level ") {
                Some((triple, level)) => {
                    if triple.trim().is_empty() || triple.contains(' ') {
                        errors.push(
                            Error::header(format!("target triple `{triple}` is not one word")).at(here),
                        );
                    }
                    match level.parse::<u32>() {
                        Ok(level) if level <= 3 => file.target = Some((triple.to_string(), level)),
                        _ => errors.push(
                            Error::header(format!("`{level}` is not an opt-level of 0 to 3")).at(here),
                        ),
                    }
                }
                None => errors.push(
                    Error::header("`target` needs `<triple> opt-level <0-3>`").at(here),
                ),
            }
        } else if let Some(rest) = line.strip_prefix("source ") {
            if stage > 2 {
                errors.push(Error::header(format!("`source` after `global`: `{line}`")).at(here));
            }
            stage = 2;
            if file.source.is_some() {
                errors.push(Error::header("`source` is declared twice").at(here));
            }
            match rest.strip_prefix("sha256:") {
                Some(hex) if hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) => {
                    file.source = Some(hex.to_string());
                }
                _ => errors.push(
                    Error::header("`source` needs `sha256:` and 64 hex digits").at(here),
                ),
            }
        } else if let Some(rest) = line.strip_prefix("@ticks ") {
            if file.source.is_none() {
                errors.push(Error::header("`@ticks` lines come after `source`").at(here));
            }
            if stage > 2 {
                errors.push(Error::header(format!("`@ticks` after `global`: `{line}`")).at(here));
            }
            stage = 2;
            if file.ticks.is_some() {
                errors.push(Error::header("`@ticks` is declared twice").at(here));
            }
            match rest.parse::<u64>() {
                Ok(ticks) => file.ticks = Some(ticks),
                Err(_) => errors.push(Error::header(format!(
                    "`@ticks` needs a non-negative count, not `{rest}`"
                ))
                .at(here)),
            }
        } else if let Some(rest) = line.strip_prefix("global ") {
            if file.source.is_none() {
                errors.push(Error::header("`global` lines come after `source`").at(here));
            }
            stage = 3;
            if rest.trim().is_empty() || rest.contains(' ') {
                errors.push(Error::header(format!("global name `{rest}` is not one name")).at(here));
            }
            file.globals.push(rest.to_string());
        } else {
            break;
        }
        i += 1;
    }
    if file.modules.is_empty() {
        errors.push(Error::header("no `module` line"));
    }
    if file.source.is_none() {
        errors.push(Error::header("no `source` line"));
    }
    while i < lines.len() {
        let line = lines[i];
        let here = src.line_rng(i);
        if line.is_empty() {
            errors.push(
                Error::header(format!("blank line before the record at `{}`", line_near(&lines, i)))
                    .at(here),
            );
            i += 1;
            continue;
        }
        let Some(rest) = line.strip_prefix("unit ") else {
            errors.push(Error::header(format!("expected `unit`, found `{line}`")).at(here));
            i += 1;
            continue;
        };
        match rest.find(" :: ") {
            Some(pos) => {
                let name = rest[..pos].trim().to_string();
                let ty = rest[pos + 4..].trim().to_string();
                let unit_line = here;
                let mut unit = ParsedUnit {
                    name,
                    ty,
                    anns: Vec::new(),
                    params: Vec::new(),
                    body: None,
                    statics: Vec::new(),
                    has_static: false,
                };
                i += 1;
                let mut terminated = false;
                while i < lines.len() {
                    let inner = lines[i];
                    let here = src.line_rng(i);
                    if inner == "end" {
                        terminated = true;
                        i += 1;
                        break;
                    }
                    // The parameter and slot helpers report through the
                    // shared vector, so their findings take this line's range.
                    let before = errors.len();
                    if let Some(rest) = inner.strip_prefix("  @params ") {
                        unit.params = parse_params(rest, &unit.name, errors);
                        unit.anns.push(("params".to_string(), rest.to_string()));
                    } else if let Some(rest) = inner.strip_prefix("  @") {
                        let (name, value) = split_annotation(rest);
                        unit.anns.push((name, value));
                    } else if let Some(rest) = inner.strip_prefix("  body ") {
                        if unit.body.is_some() || unit.has_static {
                            errors.push(
                                Error::new(&unit.name, "body", "a second body").coded(code::PARSE),
                            );
                        }
                        let (text, used) = body_text(&lines, i, rest);
                        match parse_node(&text) {
                            Ok(node) => unit.body = Some(node),
                            Err(message) => errors.push(Error::new(&unit.name, "body", message)),
                        }
                        i += used;
                        for error in &mut errors[before..] {
                            error.set_at(here);
                            error.set_code(code::PARSE);
                        }
                        continue;
                    } else if let Some(rest) = inner.strip_prefix("  static ") {
                        if unit.body.is_some() || unit.has_static {
                            errors.push(
                                Error::new(&unit.name, "static", "a second body").coded(code::PARSE),
                            );
                        }
                        unit.has_static = true;
                        unit.statics = parse_statics(rest, &unit.name, errors);
                    } else if !inner.is_empty() {
                        errors.push(
                            Error::new(&unit.name, "record", format!("unexpected line `{inner}`"))
                                .coded(code::PARSE),
                        );
                    }
                    // The parameter, slot and s-expression helpers report
                    // through the shared vector: every finding they raise is a
                    // reader finding on this line.
                    for error in &mut errors[before..] {
                        error.set_at(here);
                        error.set_code(code::PARSE);
                    }
                    i += 1;
                }
                if !terminated {
                    let last = i.min(lines.len().saturating_sub(1));
                    errors.push(
                        Error::new(&unit.name, "record", "the unit has no `end`")
                            .coded(code::PARSE)
                            .at(src.line_rng(last)),
                    );
                }
                if unit.body.is_none() && !unit.has_static {
                    errors.push(
                        Error::new(&unit.name, "record", "the unit has no `body` and no `static` slots")
                            .coded(code::PARSE)
                            .at(unit_line),
                    );
                }
                file.units.push(unit);
            }
            None => {
                errors.push(
                    Error::header(format!("`unit` line has no ` :: `: `{line}`")).at(here),
                );
                i += 1;
            }
        }
    }
    Some(file)
}

fn line_near(lines: &[&str], index: usize) -> String {
    lines.get(index).map(|line| line.to_string()).unwrap_or_default()
}

/// The body's text: its first line plus as many following lines as the
/// expression's parentheses need. Returns the text and the lines consumed.
fn body_text(lines: &[&str], start: usize, first: &str) -> (String, usize) {
    let mut text = first.to_string();
    let mut used = 1;
    while paren_depth(&text) > 0 && start + used < lines.len() {
        text.push('\n');
        text.push_str(lines[start + used]);
        used += 1;
    }
    (text, used)
}

fn paren_depth(text: &str) -> i32 {
    let mut depth = 0;
    let mut in_string = false;
    let mut escaped = false;
    for c in text.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
    }
    depth
}

/// `name(value)`, `name value` (an annotation whose value is not a group,
/// like `@span file:line:col`), or bare `name` (a flag).
fn split_annotation(text: &str) -> (String, String) {
    if let Some(pos) = text.find('(') {
        if text.ends_with(')') {
            return (text[..pos].to_string(), text[pos + 1..text.len() - 1].to_string());
        }
    }
    match text.find(' ') {
        Some(pos) => (text[..pos].to_string(), text[pos + 1..].trim().to_string()),
        None => (text.to_string(), String::new()),
    }
}

fn parse_params(text: &str, unit: &str, errors: &mut Vec<Error>) -> Vec<String> {
    let mut names = Vec::new();
    for part in split_top(text, ',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        match part.find(" :: ") {
            Some(pos) => {
                let name = part[..pos].trim().to_string();
                if name.is_empty() {
                    errors.push(Error::new(unit, "params", format!("a parameter has no name in `{part}`")));
                }
                names.push(name);
            }
            None => errors.push(Error::new(unit, "params", format!("`{part}` is not `name :: type @class`"))),
        }
    }
    names
}

fn parse_statics(text: &str, unit: &str, errors: &mut Vec<Error>) -> Vec<(u64, String)> {
    let text = text.trim();
    let Some(inner) = text.strip_prefix('{').and_then(|rest| rest.strip_suffix('}')) else {
        errors.push(Error::new(unit, "static", "`static` needs `{ slot = unit, ... }`"));
        return Vec::new();
    };
    let mut slots = Vec::new();
    for part in split_top(inner, ',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        match part.split_once('=') {
            Some((slot, method)) => match slot.trim().parse::<u64>() {
                Ok(slot) => slots.push((slot, method.trim().to_string())),
                Err(_) => errors.push(Error::new(unit, "static", format!("`{}` is not a slot number", slot.trim()))),
            },
            None => errors.push(Error::new(unit, "static", format!("`{part}` is not `slot = unit`"))),
        }
    }
    slots
}

/// Split `text` at `separator` outside parentheses, brackets and strings.
fn split_top(text: &str, separator: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;
    for c in text.chars() {
        if in_string {
            current.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            '"' => {
                in_string = true;
                current.push(c);
            }
            '(' | '[' | '{' => {
                depth += 1;
                current.push(c);
            }
            ')' | ']' | '}' => {
                depth -= 1;
                current.push(c);
            }
            c if c == separator && depth == 0 => {
                parts.push(current.clone());
                current.clear();
            }
            c => current.push(c),
        }
    }
    parts.push(current);
    parts
}

/// Read one s-expression. Marks (`@class(owned)`) are tokens that begin with
/// `@` and are attached to the enclosing node, so a form's children and its
/// annotations never share a position.
fn parse_node(text: &str) -> Result<Node, String> {
    let chars: Vec<char> = text.chars().collect();
    let mut sx = Sx { chars, pos: 0 };
    let node = sx.node()?;
    sx.skip();
    if sx.pos != sx.chars.len() {
        return Err(format!("trailing text after the expression: `{}`", sx.rest()));
    }
    Ok(node)
}

struct Sx {
    chars: Vec<char>,
    pos: usize,
}

impl Sx {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn skip(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn rest(&self) -> String {
        self.chars[self.pos.min(self.chars.len())..].iter().collect()
    }

    fn node(&mut self) -> Result<Node, String> {
        self.skip();
        match self.peek() {
            Some('(') => {
                self.pos += 1;
                self.skip();
                let head = self.token_or_empty();
                let mut marks = Vec::new();
                let mut tail = Vec::new();
                loop {
                    self.skip();
                    match self.peek() {
                        None => return Err("unclosed parenthesis".to_string()),
                        Some(')') => {
                            self.pos += 1;
                            break;
                        }
                        Some('(') => tail.push(self.node()?),
                        Some('"') => tail.push(Node::atom(self.string_token()?)),
                        Some('@') => marks.push(self.mark()?),
                        Some(_) => tail.push(Node::atom(self.token())),
                    }
                }
                Ok(Node::List { head, marks, tail })
            }
            Some(')') => Err("unexpected `)`".to_string()),
            None => Err("empty expression".to_string()),
            Some('"') => Ok(Node::atom(self.string_token()?)),
            Some(_) => Ok(Node::atom(self.token())),
        }
    }

    fn token_or_empty(&mut self) -> String {
        match self.peek() {
            Some(c) if !c.is_whitespace() && c != '(' && c != ')' && c != '"' && c != '@' => self.token(),
            _ => String::new(),
        }
    }

    fn token(&mut self) -> String {
        let start = self.pos;
        while matches!(self.peek(), Some(c) if !c.is_whitespace() && c != '(' && c != ')') {
            self.pos += 1;
        }
        if self.pos == start {
            self.pos += 1;
        }
        self.chars[start..self.pos].iter().collect()
    }

    /// A quoted atom, kept with its quotes so `flat` reproduces it.
    fn string_token(&mut self) -> Result<String, String> {
        let start = self.pos;
        self.pos += 1;
        let mut escaped = false;
        loop {
            match self.peek() {
                None => return Err("unclosed string".to_string()),
                Some('\\') if !escaped => {
                    escaped = true;
                    self.pos += 1;
                }
                Some('"') if !escaped => {
                    self.pos += 1;
                    break;
                }
                Some(_) => {
                    escaped = false;
                    self.pos += 1;
                }
            }
        }
        Ok(self.chars[start..self.pos].iter().collect())
    }

    /// A mark: `@name`, or `@name(...)` with balanced parentheses.
    fn mark(&mut self) -> Result<String, String> {
        let start = self.pos;
        self.pos += 1;
        while matches!(self.peek(), Some(c) if !c.is_whitespace() && c != '(' && c != ')') {
            self.pos += 1;
        }
        if self.peek() == Some('(') {
            let mut depth = 0i32;
            let mut in_string = false;
            let mut escaped = false;
            while let Some(c) = self.peek() {
                if in_string {
                    if escaped {
                        escaped = false;
                    } else if c == '\\' {
                        escaped = true;
                    } else if c == '"' {
                        in_string = false;
                    }
                } else {
                    match c {
                        '"' => in_string = true,
                        '(' => depth += 1,
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                self.pos += 1;
                                return Ok(self.chars[start..self.pos].iter().collect());
                            }
                        }
                        _ => {}
                    }
                }
                self.pos += 1;
            }
            return Err("unclosed mark".to_string());
        }
        Ok(self.chars[start..self.pos].iter().collect())
    }
}

// ------------------------------------------------------------- annotations

/// The documented annotation order (design note section 9, extended by the
/// node and record annotations this compiler's stages carry). A record's
/// annotations print in increasing rank, and the verifier rejects any other
/// order and every name not listed here.
const ANNOTATION_ORDER: &[&str] = &[
    "span",
    "params",
    "class",
    "captures",
    "ctor",
    "tier",
    "ffi",
    "borrowed",
    "frame",
    "specialised",
    "rewrite",
    "inline",
    "prim",
    "resolved",
    "tailrec",
    "join",
    "instance",
    "stage",
];

fn annotation_rank(name: &str) -> Option<usize> {
    ANNOTATION_ORDER.iter().position(|known| *known == name)
}

/// `s` when `n` is not one: the annotation lines read as prose.
fn plural(n: usize) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}

/// The printer's sort key for an annotation: the same table the verifier
/// checks, so one list orders and validates.
fn rank(name: &str) -> u8 {
    annotation_rank(name).map(|rank| rank as u8).unwrap_or(u8::MAX)
}

/// The `@` marks a node may carry: its ownership class, its instantiation
/// vector, and an alternative's weight.
const NODE_MARKS: &[&str] = &["class", "classes", "weight", "frame"];

// -------------------------------------------------------------- verification

/// The diagnostic codes the language server publishes (design note section
/// 8.4): one per check the reader and verifier perform.
pub mod code {
    /// A reader finding: the file does not follow the text grammar.
    pub const PARSE: &str = "core/parse";
    /// A `local` reference the enclosing scope does not bind.
    pub const UNBOUND_LOCAL: &str = "core/unbound-local";
    /// Two units of one name, which makes resolution ambiguous.
    pub const DUPLICATE_UNIT: &str = "core/duplicate-unit";
    /// A dictionary slot that is not in the instance's printed table.
    pub const SLOT_MISSING: &str = "core/slot-missing";
    /// An annotation out of the documented order.
    pub const ANNOTATION_ORDER: &str = "core/annotation-order";
    /// An annotation name the grammar does not define.
    pub const UNKNOWN_ANNOTATION: &str = "core/unknown-annotation";
    /// Any other verifier finding.
    pub const VERIFY: &str = "core/verify";
    /// A unit with no `@span`, which the server warns about rather than
    /// refusing: a dump's records carry no source position.
    pub const SPAN_MISSING: &str = "core/span-missing";
}

/// One verifier finding, located by unit and by structural path rather than
/// by line (`Main.map:body:alt 2`), and by the source range the language
/// server highlights. `start` and `end` are 0-based line and 0-based column
/// in UTF-16 code units, the LSP convention: the reader fills them for what
/// it rejects, and [`verify`] fills them for the rest from the printed
/// structure.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    pub unit: String,
    pub path: String,
    pub message: String,
    /// The first position of the offending syntax.
    pub start: (u32, u32),
    /// One past its last position.
    pub end: (u32, u32),
    /// The check that failed, one of [`code`].
    pub code: &'static str,
}

impl Error {
    fn new(unit: &str, path: &str, message: impl Into<String>) -> Error {
        Error {
            unit: unit.to_string(),
            path: path.to_string(),
            message: message.into(),
            start: (0, 0),
            end: (0, 0),
            code: code::VERIFY,
        }
    }

    fn header(message: impl Into<String>) -> Error {
        Error::new("", "header", message).coded(code::PARSE)
    }

    /// The same finding with a source range.
    fn at(mut self, range: Rng) -> Error {
        self.set_at(range);
        self
    }

    /// Name the check that failed.
    fn coded(mut self, code: &'static str) -> Error {
        self.code = code;
        self
    }

    fn set_at(&mut self, range: Rng) {
        self.start = range.start;
        self.end = range.end;
    }

    /// Name the check that failed, in place.
    fn set_code(&mut self, check: &'static str) {
        self.code = check;
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}: {}", self.unit, self.message)
        } else if self.unit.is_empty() {
            write!(f, "{}: {}", self.path, self.message)
        } else {
            write!(f, "{}:{}: {}", self.unit, self.path, self.message)
        }
    }
}

/// What a clean [`verify`] counted.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    pub stage: Option<String>,
    pub modules: Vec<String>,
    pub units: usize,
    pub expressions: usize,
    pub dictionaries: usize,
    pub projections: usize,
    pub dictionary_bindings: usize,
    pub annotations: BTreeMap<String, usize>,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = self.stage.as_deref().unwrap_or("-");
        write!(
            f,
            "core {stage}: {} modules, {} units, {} expressions, {} dictionaries, {} projections, {} dictionary bindings",
            self.modules.len(),
            self.units,
            self.expressions,
            self.dictionaries,
            self.projections,
            self.dictionary_bindings
        )?;
        if !self.annotations.is_empty() {
            let rows: Vec<String> =
                self.annotations.iter().map(|(name, count)| format!("@{name} {count}")).collect();
            write!(f, "; annotations: {}", rows.join(", "))?;
        }
        Ok(())
    }
}

/// Parse `text` and re-check it without re-elaborating source (design note
/// section 2's `core verify`): the header and grammar, unit names, annotation
/// names and order, reference resolution, dictionary slot agreement and count
/// operations over in-scope locals.
pub fn verify(text: &str) -> Result<Report, Vec<Error>> {
    let mut errors = Vec::new();
    let Some(file) = parse(text, &mut errors) else {
        return Err(errors);
    };
    let report = check_file(&file, &mut errors);
    if errors.is_empty() {
        Ok(report)
    } else {
        // The verifier names a finding by unit and structural path; the
        // printed structure is what turns that into the range a client
        // highlights. Reader findings already carry their line.
        let syntax = syntax(text);
        for error in &mut errors {
            if error.end != (0, 0) {
                continue;
            }
            if let Some(range) = syntax.locate(error) {
                error.set_at(range);
            }
        }
        errors.sort();
        Err(errors)
    }
}

fn check_file(file: &Parsed, errors: &mut Vec<Error>) -> Report {
    let mut report = Report {
        modules: file.modules.clone(),
        units: file.units.len(),
        ..Report::default()
    };
    let mut units: BTreeMap<&str, &ParsedUnit> = BTreeMap::new();
    let mut declared: BTreeSet<&str> = BTreeSet::new();
    for global in &file.globals {
        declared.insert(global.as_str());
    }
    for unit in &file.units {
        if units.insert(unit.name.as_str(), unit).is_some() {
            errors.push(
                Error::new(&unit.name, "record", "the unit name is not unique")
                    .coded(code::DUPLICATE_UNIT),
            );
        }
        declared.insert(unit.name.as_str());
    }
    let modules_owned: BTreeSet<String> = file.modules.iter().cloned().collect();
    for unit in &file.units {
        let module = module_of(&unit.name, &modules_owned);
        if !module.is_empty() && !modules_owned.contains(&module) {
            errors.push(Error::new(&unit.name, "record", format!("module `{module}` is not declared in the header")));
        }
        // Annotation order and names.
        let mut last = None;
        for (name, value) in &unit.anns {
            match annotation_rank(name) {
                Some(rank) => {
                    if let Some(previous) = last {
                        if rank <= previous {
                            errors.push(
                                Error::new(&unit.name, "@", format!("annotation `@{name}` is out of order"))
                                    .coded(code::ANNOTATION_ORDER),
                            );
                        }
                    }
                    last = Some(rank);
                }
                None => errors.push(
                    Error::new(&unit.name, "@", format!("unknown annotation `@{name}`"))
                        .coded(code::UNKNOWN_ANNOTATION),
                ),
            }
            *report.annotations.entry(name.clone()).or_insert(0) += 1;
            if name == "span" {
                check_span(value, &unit.name, errors);
            }
            if name == "stage" {
                if Stage::parse(value).is_none() {
                    errors.push(Error::new(&unit.name, "@stage", format!("`{value}` is not a stage")));
                }
                report.stage = Some(value.clone());
            }
        }
        if unit.ty.is_empty() {
            errors.push(Error::new(&unit.name, "record", "the unit has no type"));
        }
        report.dictionary_bindings += unit.statics.len();
        for (slot, method) in &unit.statics {
            if !declared.contains(method.as_str()) {
                errors.push(
                    Error::new(
                        &unit.name,
                        &format!("static:{slot}"),
                        format!("slot {slot} names the undefined unit `{method}`"),
                    )
                    .coded(code::SLOT_MISSING),
                );
            }
        }
        if let Some(body) = &unit.body {
            let mut scope: Vec<String> = unit.params.clone();
            let mut checker = Checker { file, units: &units, declared: &declared, report: &mut report };
            checker.expr(body, &mut scope, &unit.name, "body", errors);
            let mut joins: Vec<JoinScope> = Vec::new();
            let mut next = 0usize;
            check_joins(body, &mut joins, 0, &mut next, &unit.name, "body", errors);
        }
    }
    report
}

/// A join point in scope at a jump: its printed id, its parameter count,
/// and the destination (see below) its value is delivered to.
struct JoinScope {
    id: String,
    arity: usize,
    dest: usize,
}

/// The join legality rule of [`crate::core::Core::Join`], over the printed
/// grammar: a `jump` names exactly one enclosing `join`, passes one
/// argument per parameter, and delivers to that join's continuation, which
/// is the destination of the position the `Join` sits in. Destinations are
/// tracked as tokens: the body starts at token 0 (the function's return),
/// the forms that pass a value on unchanged (`let`, the count operations,
/// `release`, both branches of an `if`, the alternatives of a `case`, and
/// both parts of a `join`) keep the token, and every other child position
/// gets a fresh one, because it is a value of its own. So a jump in a tail
/// position of the function is legal, and so is a jump in an alternative of
/// a decision whose value is the join's value; a jump whose join delivers
/// somewhere else is refused. A jump never crosses a lambda or a quotation's
/// retained body: both are separate code units, and their joins are not in
/// scope.
fn check_joins(
    node: &Node,
    joins: &mut Vec<JoinScope>,
    dest: usize,
    next: &mut usize,
    unit: &str,
    path: &str,
    errors: &mut Vec<Error>,
) {
    let Node::List { head, tail: kids, .. } = node else {
        return;
    };
    let fresh = |next: &mut usize| {
        let token = *next;
        *next += 1;
        token
    };
    match head.as_str() {
        "join" => {
            let id = match kids.first() {
                Some(Node::Atom(name)) => name.clone(),
                _ => {
                    errors.push(Error::new(unit, path, "a join has no identifier"));
                    return;
                }
            };
            let arity = match kids.get(1) {
                Some(Node::List { tail: params, .. }) if params.iter().all(|p| matches!(p, Node::List { .. })) => {
                    params.len()
                }
                _ => 0,
            };
            if joins.iter().any(|scope| scope.id == id) {
                errors.push(Error::new(
                    unit,
                    path,
                    format!("`{id}` names two joins in scope, so a jump cannot name one of them"),
                ));
            }
            joins.push(JoinScope { id, arity, dest });
            for (index, child) in kids.iter().enumerate().skip(2) {
                check_joins(child, joins, dest, next, unit, &child_path(head, index, path), errors);
            }
            joins.pop();
            return;
        }
        "jump" => {
            let id = match kids.first() {
                Some(Node::Atom(name)) => name.clone(),
                _ => {
                    errors.push(Error::new(unit, path, "a jump has no identifier"));
                    return;
                }
            };
            let args = kids.len().saturating_sub(1);
            match joins.iter().rev().find(|scope| scope.id == id) {
                None => errors.push(Error::new(
                    unit,
                    path,
                    format!("the jump's join `{id}` is not in scope here"),
                )),
                Some(scope) if scope.arity != args => errors.push(Error::new(
                    unit,
                    path,
                    format!("the jump passes {args} arguments to the join's {} parameters", scope.arity),
                )),
                Some(scope) if scope.dest != dest => errors.push(Error::new(
                    unit,
                    path,
                    format!("the jump's value is not the result of `{id}` here: it is in a position of its own, so it never reaches the join's continuation"),
                )),
                Some(_) => {}
            }
            for (index, child) in kids.iter().enumerate().skip(1) {
                let token = fresh(next);
                check_joins(child, joins, token, next, unit, &child_path(head, index, path), errors);
            }
            return;
        }
        "lam" | "quote" => {
            // A separate code unit: no join of this body is in scope, and
            // no position inside it shares this body's destination.
            let mut inner: Vec<JoinScope> = Vec::new();
            for (index, child) in kids.iter().enumerate() {
                let token = fresh(next);
                check_joins(child, &mut inner, token, next, unit, &child_path(head, index, path), errors);
            }
            return;
        }
        "case" => {
            for (index, child) in kids.iter().enumerate() {
                let child_path = child_path(head, index, path);
                if index == 0 {
                    let token = fresh(next);
                    check_joins(child, joins, token, next, unit, &child_path, errors);
                    continue;
                }
                let Node::List { tail: alt, .. } = child else {
                    continue;
                };
                for (field, part) in alt.iter().enumerate().skip(1) {
                    check_joins(part, joins, dest, next, unit, &format!("{child_path}:field {field}"), errors);
                }
            }
            return;
        }
        _ => {}
    }
    // The forms that pass a value on: the body of a `let`, of a count
    // operation, of a `release`, and both branches of an `if`.
    for (index, child) in kids.iter().enumerate() {
        let child_path = child_path(head, index, path);
        let same = match head.as_str() {
            "if" => index > 0,
            "let" | "dup" | "drop" => index + 1 == kids.len(),
            "release" => index == 3,
            _ => false,
        };
        let token = if same { dest } else { fresh(next) };
        check_joins(child, joins, token, next, unit, &child_path, errors);
    }
}

fn check_span(text: &str, unit: &str, errors: &mut Vec<Error>) {
    let parts: Vec<&str> = text.rsplitn(3, ':').collect();
    if parts.len() != 3 {
        errors.push(Error::new(unit, "@span", format!("`{text}` is not `file:line:col`")));
        return;
    }
    let (col, line, file) = (parts[0], parts[1], parts[2]);
    if file.trim().is_empty() || file.contains(' ') {
        errors.push(Error::new(unit, "@span", format!("`{file}` is not a file name")));
    }
    if line.parse::<u32>().map_or(true, |line| line == 0) {
        errors.push(Error::new(unit, "@span", format!("`{line}` is not a line number")));
    }
    if col.parse::<u32>().map_or(true, |col| col == 0) {
        errors.push(Error::new(unit, "@span", format!("`{col}` is not a column number")));
    }
}

struct Checker<'a> {
    file: &'a Parsed,
    units: &'a BTreeMap<&'a str, &'a ParsedUnit>,
    declared: &'a BTreeSet<&'a str>,
    report: &'a mut Report,
}

impl<'a> Checker<'a> {
    fn expr(&mut self, node: &Node, scope: &mut Vec<String>, unit: &str, path: &str, errors: &mut Vec<Error>) {
        self.report.expressions += 1;
        let Node::List { head, marks, tail } = node else {
            return;
        };
        for mark in marks {
            let name = mark_name(mark);
            if !NODE_MARKS.contains(&name.as_str()) {
                errors.push(Error::new(unit, path, format!("unknown mark `{mark}`")));
            }
        }
        match head.as_str() {
            "local" => {
                let name = atom_at(tail, 0, unit, path, errors);
                if let Some(name) = name {
                    if !scope.iter().any(|bound| bound == name) {
                        errors.push(
                            Error::new(unit, path, format!("`{name}` is out of scope"))
                                .coded(code::UNBOUND_LOCAL),
                        );
                    }
                }
                return;
            }
            "global" | "ctor" => {
                let name = atom_at(tail, 0, unit, path, errors);
                if let Some(name) = name {
                    if !self.declared.contains(name) {
                        errors.push(Error::new(unit, path, format!("`{}` is not a unit of this file", unquote(name))));
                    }
                }
                for (index, child) in tail.iter().enumerate().skip(1) {
                    self.expr(child, scope, unit, &child_path(head, index, path), errors);
                }
                return;
            }
            "dictproj" => {
                self.dictproj(tail, scope, unit, path, errors);
                return;
            }
            "let" => {
                // The binder is the first child; the value and the body are the
                // last two, so a printer that also writes `:: <type>` between
                // them reads the same way.
                let binder = atom_at(tail, 0, unit, path, errors).map(str::to_string);
                let body_index = tail.len().saturating_sub(1);
                for (index, child) in tail.iter().enumerate().skip(1) {
                    if index == body_index {
                        let before = scope.len();
                        if let Some(name) = &binder {
                            scope.push(name.clone());
                        }
                        self.expr(child, scope, unit, &child_path(head, index, path), errors);
                        scope.truncate(before);
                    } else {
                        self.expr(child, scope, unit, &child_path(head, index, path), errors);
                    }
                }
                return;
            }
            "join" => {
                // The join's parameters are bound in its body and not in
                // its continuation; the identifier is not a variable.
                let mut bound: Vec<String> = Vec::new();
                if let Some(Node::List { tail: params, .. }) = tail.get(1) {
                    for param in params {
                        if let Node::List { head: name, .. } = param {
                            bound.push(name.clone());
                        }
                    }
                }
                for (index, child) in tail.iter().enumerate().skip(2) {
                    let child_path = child_path(head, index, path);
                    let before = scope.len();
                    if index == 2 {
                        scope.extend(bound.iter().cloned());
                    }
                    self.expr(child, scope, unit, &child_path, errors);
                    scope.truncate(before);
                }
                return;
            }
            "lam" => {
                let mut bound: Vec<String> = Vec::new();
                if let Some(Node::List { tail: params, .. }) = tail.first() {
                    for param in params {
                        if let Node::Atom(name) = param {
                            bound.push(name.clone());
                        }
                    }
                }
                for (index, child) in tail.iter().enumerate().skip(1) {
                    let child_path = child_path(head, index, path);
                    let before = scope.len();
                    scope.extend(bound.iter().cloned());
                    self.expr(child, scope, unit, &child_path, errors);
                    scope.truncate(before);
                }
                return;
            }
            "case" => {
                for (index, child) in tail.iter().enumerate() {
                    let child_path = child_path(head, index, path);
                    if index == 0 {
                        self.expr(child, scope, unit, &child_path, errors);
                        continue;
                    }
                    let before = scope.len();
                    if let Node::List { tail: alt, .. } = child {
                        if let Some(pattern) = alt.first() {
                            pattern_binders(pattern, scope);
                        }
                        for (field, part) in alt.iter().enumerate().skip(1) {
                            self.expr(part, scope, unit, &format!("{child_path}:field {field}"), errors);
                        }
                    } else {
                        errors.push(Error::new(unit, &child_path, "an alternative is not a list"));
                    }
                    scope.truncate(before);
                }
                return;
            }
            "dup" | "drop" => {
                self.binding(tail, 0, scope, unit, path, errors);
                if let Some(child) = tail.get(2) {
                    self.expr(child, scope, unit, &child_path(head, 2, path), errors);
                }
                return;
            }
            "boxinit" | "refset" => {
                self.binding(tail, 0, scope, unit, path, errors);
                if let Some(child) = tail.get(1) {
                    self.expr(child, scope, unit, &child_path(head, 1, path), errors);
                }
                return;
            }
            "lazy-init" => {
                self.binding(tail, 0, scope, unit, path, errors);
                if let Some(child) = tail.get(1) {
                    self.expr(child, scope, unit, &child_path(head, 1, path), errors);
                }
                return;
            }
            "release" => {
                self.binding(tail, 0, scope, unit, path, errors);
                for (index, child) in tail.iter().enumerate().skip(1) {
                    let child_path = child_path(head, index, path);
                    if index == 3 {
                        self.expr(child, scope, unit, &child_path, errors);
                        continue;
                    }
                    if let Node::List { tail: names, .. } = child {
                        for name in names {
                            if let Node::Atom(name) = name {
                                if !scope.iter().any(|bound| bound == name) {
                                    errors.push(
                                        Error::new(unit, &child_path, format!("`{name}` is out of scope"))
                                            .coded(code::UNBOUND_LOCAL),
                                    );
                                }
                            }
                        }
                    }
                }
                return;
            }
            "instdict" => {
                self.report.dictionaries += 1;
                let name = atom_at(tail, 0, unit, path, errors);
                if let Some(name) = name {
                    if !self.declared.contains(name) {
                        errors.push(Error::new(unit, path, format!("`{}` is not a unit of this file", unquote(name))));
                    }
                }
            }
            _ => {}
        }
        for (index, child) in tail.iter().enumerate() {
            self.expr(child, scope, unit, &child_path(head, index, path), errors);
        }
    }

    fn binding(
        &self,
        tail: &[Node],
        index: usize,
        scope: &[String],
        unit: &str,
        path: &str,
        errors: &mut Vec<Error>,
    ) {
        if let Some(Node::Atom(name)) = tail.get(index) {
            if !scope.iter().any(|bound| bound == name) {
                errors.push(
                    Error::new(unit, path, format!("`{name}` is out of scope"))
                        .coded(code::UNBOUND_LOCAL),
                );
            }
        } else if tail.get(index).is_some() {
            errors.push(Error::new(unit, path, "the count operation's name is not an atom"));
        }
    }

    /// A dictionary projection: its dictionary operand, slot and class. When
    /// the operand is a static `instdict`, the slot must exist in that unit's
    /// printed slot table and the class must be the one that unit implements.
    fn dictproj(&mut self, tail: &[Node], scope: &mut Vec<String>, unit: &str, path: &str, errors: &mut Vec<Error>) {
        self.report.projections += 1;
        let Some(Node::List { head, tail: dict, .. }) = tail.first() else {
            for (index, child) in tail.iter().enumerate() {
                self.expr(child, scope, unit, &child_path("dictproj", index, path), errors);
            }
            return;
        };
        if head == "instdict" {
            self.report.dictionaries += 1;
            let mut inner = scope.clone();
            for (index, child) in dict.iter().enumerate().skip(1) {
                self.expr(child, &mut inner, unit, &child_path("instdict", index, path), errors);
            }
            let slot = tail.get(1).and_then(|node| match node {
                Node::Atom(text) => text.parse::<u64>().ok(),
                _ => None,
            });
            let class = match tail.get(2) {
                Some(Node::Atom(text)) => Some(text.clone()),
                _ => None,
            };
            let name = match dict.first() {
                Some(Node::Atom(name)) => Some(name.clone()),
                _ => None,
            };
            if let (Some(slot), Some(name)) = (slot, name) {
                // A dictionary the dump does not define is one of its own
                // `global` declarations: its slots are not printed here, so
                // there is nothing to check them against.
                if self.file.globals.iter().any(|global| global == &name) {
                    return;
                }
                match self.units.get(name.as_str()) {
                    Some(instance) => {
                        if !instance.has_static {
                            errors.push(
                                Error::new(unit, path, format!("`{name}` has no static slots"))
                                    .coded(code::SLOT_MISSING),
                            );
                        } else if !instance.statics.iter().any(|(printed, _)| *printed == slot) {
                            errors.push(
                                Error::new(
                                    unit,
                                    path,
                                    format!("slot {slot} is not in `{name}`'s static slot table"),
                                )
                                .coded(code::SLOT_MISSING),
                            );
                        }
                        if let Some(class) = class {
                            if class != "-" && class != unquote(&instance.ty) {
                                errors.push(Error::new(
                                    unit,
                                    path,
                                    format!("class `{class}` is not `{}`'s class `{}`", name, instance.ty),
                                ));
                            }
                        }
                    }
                    None => errors.push(
                        Error::new(unit, path, format!("unknown dictionary `{name}`"))
                            .coded(code::SLOT_MISSING),
                    ),
                }
            }
            return;
        }
        for (index, child) in tail.iter().enumerate() {
            self.expr(child, scope, unit, &child_path("dictproj", index, path), errors);
        }
    }
}

fn atom_at<'a>(
    tail: &'a [Node],
    index: usize,
    unit: &str,
    path: &str,
    errors: &mut Vec<Error>,
) -> Option<&'a str> {
    match tail.get(index) {
        Some(Node::Atom(text)) => Some(text.as_str()),
        Some(_) => {
            errors.push(Error::new(unit, path, "expected a name, found a list"));
            None
        }
        None => {
            errors.push(Error::new(unit, path, "the form is missing an operand"));
            None
        }
    }
}

fn pattern_binders(pattern: &Node, scope: &mut Vec<String>) {
    match pattern {
        Node::Atom(text) if text == "_" => {}
        Node::Atom(text) => scope.push(text.clone()),
        Node::List { head, tail, .. } => match head.as_str() {
            "con" => {
                for field in tail.iter().skip(1) {
                    pattern_binders(field, scope);
                }
            }
            "int" | "nat" => {}
            "" => {}
            name => scope.push(name.to_string()),
        },
    }
}

fn mark_name(mark: &str) -> String {
    let rest = mark.strip_prefix('@').unwrap_or(mark);
    match rest.find('(') {
        Some(pos) => rest[..pos].to_string(),
        None => rest.to_string(),
    }
}

fn unquote(name: &str) -> String {
    if name.len() >= 2 && name.starts_with('\'') && name.ends_with('\'') {
        name[1..name.len() - 1].replace("''", "'")
    } else {
        name.to_string()
    }
}

/// The structural label of a form's `index`th child. Cases and applications
/// name their parts; everything else numbers them, which is what a `diff`
/// reports (`Main.map:body:alt 2:field 1`).
fn child_path(head: &str, index: usize, path: &str) -> String {
    let label = match (head, index) {
        ("app", 0) => "callee".to_string(),
        ("app", index) => format!("arg {index}"),
        ("case", 0) => "scrut".to_string(),
        ("case", index) => format!("alt {index}"),
        ("if", 0) => "cond".to_string(),
        ("if", 1) => "then".to_string(),
        ("if", 2) => "else".to_string(),
        ("let", 0) => "binder".to_string(),
        ("let", 1) => "value".to_string(),
        // The body is the last child whatever the printer writes between the
        // binder and it.
        ("let", _) => "body".to_string(),
        ("lam", 0) => "params".to_string(),
        ("lam", 1) => "env".to_string(),
        ("lam", 2) => "body".to_string(),
        ("dup", 0) | ("drop", 0) => "name".to_string(),
        ("dup", 1) | ("drop", 1) => "body".to_string(),
        ("release", 0) => "block".to_string(),
        ("release", 1) => "moved".to_string(),
        ("release", 2) => "rest".to_string(),
        ("release", 3) => "body".to_string(),
        ("refset", 0) | ("boxinit", 0) | ("lazy-init", 0) => "cell".to_string(),
        ("refset", 1) | ("boxinit", 1) | ("lazy-init", 1) => "value".to_string(),
        ("dictproj", 0) => "dict".to_string(),
        ("dictproj", 1) => "slot".to_string(),
        ("dictproj", 2) => "class".to_string(),
        ("proj", 0) => "record".to_string(),
        ("proj", 1) => "field".to_string(),
        ("quote", 0) => "count".to_string(),
        ("quote", 1) => "holes".to_string(),
        ("quote", 2) => "body".to_string(),
        (_, index) => format!("field {}", index + 1),
    };
    format!("{path}:{label}")
}

// ------------------------------------------------------------------- diff

/// Report the first divergence between two Core files as a structural path
/// (design note section 2): `Main.map:body:alt 2:field 1: left (int 1) right
/// (int 2)`. Units compare in canonical order, so a dump of any size is
/// comparable. `None` when the two parse to the same structure, annotations
/// included. The header is not compared: two stages of one build legitimately
/// differ in their target line.
pub fn diff(a: &str, b: &str) -> Option<String> {
    let mut left_errors = Vec::new();
    let mut right_errors = Vec::new();
    let left = parse(a, &mut left_errors);
    let right = parse(b, &mut right_errors);
    let (Some(left), Some(right)) = (left, right) else {
        let message = left_errors
            .first()
            .or_else(|| right_errors.first())
            .map(|error| error.to_string())
            .unwrap_or_else(|| "unreadable file".to_string());
        return Some(format!("unreadable: {message}"));
    };
    let left_units: BTreeMap<&str, &ParsedUnit> =
        left.units.iter().map(|unit| (unit.name.as_str(), unit)).collect();
    let right_units: BTreeMap<&str, &ParsedUnit> =
        right.units.iter().map(|unit| (unit.name.as_str(), unit)).collect();
    let mut names: BTreeSet<&str> = left_units.keys().copied().collect();
    names.extend(right_units.keys().copied());
    for name in names {
        let unit_path = name.to_string();
        match (left_units.get(name), right_units.get(name)) {
            (None, Some(_)) => return Some(format!("{unit_path}: only in the right file")),
            (Some(_), None) => return Some(format!("{unit_path}: only in the left file")),
            (None, None) => {}
            (Some(left), Some(right)) => {
                if left.ty != right.ty {
                    return Some(format!(
                        "{unit_path}:type: left {} right {}",
                        left.ty, right.ty
                    ));
                }
                if let Some(divergence) = diff_annotations(&unit_path, &left.anns, &right.anns) {
                    return Some(divergence);
                }
                if left.params != right.params {
                    return Some(format!(
                        "{unit_path}:params: left [{}] right [{}]",
                        left.params.join(", "),
                        right.params.join(", ")
                    ));
                }
                if left.statics != right.statics {
                    return Some(format!(
                        "{unit_path}:static: left [{}] right [{}]",
                        render_statics(&left.statics),
                        render_statics(&right.statics)
                    ));
                }
                if let Some(divergence) = diff_body(&unit_path, left.body.as_ref(), right.body.as_ref()) {
                    return Some(divergence);
                }
            }
        }
    }
    None
}

fn render_statics(stats: &[(u64, String)]) -> String {
    stats.iter().map(|(slot, unit)| format!("{slot} = {unit}")).collect::<Vec<_>>().join(", ")
}

fn diff_annotations(path: &str, left: &[(String, String)], right: &[(String, String)]) -> Option<String> {
    let left_map: BTreeMap<&str, &str> =
        left.iter().map(|(name, value)| (name.as_str(), value.as_str())).collect();
    let right_map: BTreeMap<&str, &str> =
        right.iter().map(|(name, value)| (name.as_str(), value.as_str())).collect();
    let mut names: BTreeSet<&str> = left_map.keys().copied().collect();
    names.extend(right_map.keys().copied());
    for name in names {
        match (left_map.get(name), right_map.get(name)) {
            (Some(left), Some(right)) if left == right => continue,
            (Some(left), Some(right)) => {
                return Some(format!("{path}:@{name}: left ({left}) right ({right})"))
            }
            (Some(left), None) => return Some(format!("{path}:@{name}: left ({left}) right (-)")),
            (None, Some(right)) => return Some(format!("{path}:@{name}: left (-) right ({right})")),
            (None, None) => {}
        }
    }
    None
}

fn diff_body(path: &str, left: Option<&Node>, right: Option<&Node>) -> Option<String> {
    match (left, right) {
        (None, None) => None,
        (Some(left), None) => Some(format!("{path}:body: left ({}) right (-)", left.flat())),
        (None, Some(right)) => Some(format!("{path}:body: left (-) right ({})", right.flat())),
        (Some(left), Some(right)) => diff_node(left, right, format!("{path}:body")),
    }
}

fn diff_node(left: &Node, right: &Node, path: String) -> Option<String> {
    match (left, right) {
        (Node::Atom(a), Node::Atom(b)) => {
            if a == b {
                None
            } else {
                Some(format!("{path}: left {a} right {b}"))
            }
        }
        (
            Node::List { head: lh, marks: lm, tail: lt },
            Node::List { head: rh, marks: rm, tail: rt },
        ) => {
            if lh != rh {
                return Some(format!("{path}:head: left {lh} right {rh}"));
            }
            if lm != rm {
                return Some(format!(
                    "{path}:marks: left [{}] right [{}]",
                    lm.join(" "),
                    rm.join(" ")
                ));
            }
            if lt.len() != rt.len() {
                let (side, index) = if lt.len() < rt.len() { ("left", lt.len()) } else { ("right", rt.len()) };
                let (left_text, right_text) = match (lt.get(index), rt.get(index)) {
                    (Some(node), None) => (node.flat(), "-".to_string()),
                    (None, Some(node)) => ("-".to_string(), node.flat()),
                    _ => ("-".to_string(), "-".to_string()),
                };
                return Some(format!(
                    "{path}:field {}: the {side} file has no child here: left {left_text} right {right_text}",
                    index + 1
                ));
            }
            // A form whose children are all atoms is one value: report it as
            // the node it is (`left (int 1) right (int 2)`), not as the atom
            // inside it.
            if lt.iter().chain(rt.iter()).all(|node| matches!(node, Node::Atom(_))) {
                if left.flat() != right.flat() {
                    return Some(format!("{path}: left {} right {}", left.flat(), right.flat()));
                }
                return None;
            }
            for (index, (a, b)) in lt.iter().zip(rt.iter()).enumerate() {
                let child = child_path(lh, index, &path);
                if let Some(divergence) = diff_node(a, b, child) {
                    return Some(divergence);
                }
            }
            None
        }
        (a, b) => Some(format!("{path}: left {} right {}", a.flat(), b.flat())),
    }
}

// ----------------------------------------------------------------- syntax
//
// The language server reads a Core file as text, so it needs a position
// carrying view of what the reader above already recognises: units,
// annotations, forms, references and literals, with the range of each.
// Nothing here re-derives a type; the printed text is the only source. Every
// range is in LSP coordinates: 0-based line, 0-based column in UTF-16 code
// units.

/// A position in LSP coordinates.
pub type Pos = (u32, u32);

/// A half-open range in LSP coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rng {
    pub start: Pos,
    pub end: Pos,
}

impl Rng {
    pub fn new(start: Pos, end: Pos) -> Rng {
        Rng { start, end }
    }

    /// Whether `pos` lies inside the range. A cursor sits between characters,
    /// so an atom's start position is inside it and its end position outside.
    pub fn contains(&self, pos: Pos) -> bool {
        self.start <= pos && pos < self.end
    }

    /// The four numbers in the order [`definition`] returns them.
    pub fn tuple(&self) -> (u32, u32, u32, u32) {
        (self.start.0, self.start.1, self.end.0, self.end.1)
    }

    /// Whether `self` covers `other` entirely.
    pub fn covers(&self, other: &Rng) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

/// A header line's kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderKind {
    /// `core 1`.
    Version,
    Module,
    Target,
    Source,
    Ticks,
    Global,
}

impl HeaderKind {
    /// The word that introduces the line.
    pub fn word(self) -> &'static str {
        match self {
            HeaderKind::Version => "core",
            HeaderKind::Module => "module",
            HeaderKind::Target => "target",
            HeaderKind::Source => "source",
            HeaderKind::Ticks => "@ticks",
            HeaderKind::Global => "global",
        }
    }
}

/// One header line and the value it declares.
#[derive(Clone, Debug)]
pub struct HeaderLine {
    pub kind: HeaderKind,
    pub value: String,
    pub range: Rng,
}

/// A `@name(value)` or `@name value` annotation on a unit record.
#[derive(Clone, Debug)]
pub struct Annotation {
    pub name: String,
    /// The whole annotation, `@` included.
    pub range: Rng,
    /// The name without its `@`.
    pub name_range: Rng,
    pub value: String,
    pub value_range: Rng,
}

/// A form's `@` mark: `@class(owned)`, `@classes(TT)`, `@weight(1)`.
#[derive(Clone, Debug)]
pub struct Mark {
    pub name: String,
    pub range: Rng,
    pub name_range: Rng,
    pub value: String,
    pub value_range: Rng,
}

/// An atom inside a form: a name, a slot number, a literal's text.
#[derive(Clone, Debug)]
pub struct Atom {
    pub text: String,
    pub range: Rng,
}

/// A child of a form: a nested form, or an atom.
#[derive(Clone, Debug)]
pub enum Item {
    Atom(Atom),
    Form(Box<Form>),
}

/// One expression node: a form's keyword, its marks and its children.
#[derive(Clone, Debug)]
pub struct Form {
    /// The form's keyword (`app`, `let`, `local`, ...); for an atom node, the
    /// atom's text.
    pub head: String,
    pub head_range: Rng,
    /// The whole node, parentheses included.
    pub range: Rng,
    /// The structural path the verifier reports (`body:alt 2:field 1`), so a
    /// finding's range is a lookup and not a second traversal rule.
    pub path: String,
    /// Whether the node is an atom rather than a list.
    pub atom: bool,
    pub marks: Vec<Mark>,
    /// The printed tail in printed order, marks excluded, which is the order
    /// the verifier numbers them in.
    pub children: Vec<Item>,
}

impl Form {
    /// The atom child at `index`.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        match self.children.get(index) {
            Some(Item::Atom(atom)) => Some(atom),
            _ => None,
        }
    }

    /// The nested form child at `index`.
    pub fn form(&self, index: usize) -> Option<&Form> {
        match self.children.get(index) {
            Some(Item::Form(form)) => Some(form),
            _ => None,
        }
    }

    /// This node and every nested form, in source order.
    pub fn walk<'a>(&'a self, out: &mut Vec<&'a Form>) {
        out.push(self);
        for child in &self.children {
            if let Item::Form(child) = child {
                child.walk(out);
            }
        }
    }
}

/// What a name in a body refers to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefKind {
    Local,
    Global,
    Ctor,
    Prim,
    Foreign,
}

impl RefKind {
    /// The form keyword that carries the reference.
    pub fn head(self) -> &'static str {
        match self {
            RefKind::Local => "local",
            RefKind::Global => "global",
            RefKind::Ctor => "ctor",
            RefKind::Prim => "prim",
            RefKind::Foreign => "foreign",
        }
    }

    /// The kind's name in prose.
    pub fn name(self) -> &'static str {
        match self {
            RefKind::Local => "local",
            RefKind::Global => "unit",
            RefKind::Ctor => "constructor",
            RefKind::Prim => "prim",
            RefKind::Foreign => "foreign unit",
        }
    }

    /// The semantic token type a reference reports as.
    pub fn token(self) -> &'static str {
        match self {
            RefKind::Local => "variable",
            RefKind::Global | RefKind::Ctor => "namespace",
            RefKind::Prim | RefKind::Foreign => "function",
        }
    }
}

/// A `(local x)`, `(global X)`, `(ctor X ...)`, `(prim p)` or
/// `(foreign "s")` reference.
#[derive(Clone, Debug)]
pub struct Ref {
    pub kind: RefKind,
    /// The name the reference prints, quoted atoms unquoted.
    pub name: String,
    pub range: Rng,
    /// The index of the unit whose body holds the reference.
    pub unit: usize,
}

/// A literal's kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,
    Nat,
    Big,
    Bytes,
    String,
    Sym,
    Bool,
    Unit,
}

impl LiteralKind {
    /// The kind's name in prose.
    pub fn word(self) -> &'static str {
        match self {
            LiteralKind::Int => "integer",
            LiteralKind::Nat => "natural",
            LiteralKind::Big => "big integer",
            LiteralKind::Bytes => "bytes",
            LiteralKind::String => "string",
            LiteralKind::Sym => "symbol",
            LiteralKind::Bool => "boolean",
            LiteralKind::Unit => "unit",
        }
    }

    /// The semantic token type a literal reports as.
    pub fn token(self) -> &'static str {
        match self {
            LiteralKind::Int | LiteralKind::Nat | LiteralKind::Big => "number",
            LiteralKind::Bytes | LiteralKind::String | LiteralKind::Sym => "string",
            LiteralKind::Bool | LiteralKind::Unit => "keyword",
        }
    }
}

/// A printed literal and its text.
#[derive(Clone, Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    /// The printed payload (`1`, `"abc"`, `true`), quotes included.
    pub text: String,
    pub range: Rng,
    pub unit: usize,
}

/// A `(dictproj dict slot class)` projection.
#[derive(Clone, Debug)]
pub struct Projection {
    pub range: Rng,
    pub slot: Option<u64>,
    pub slot_range: Option<Rng>,
    pub class: Option<String>,
    pub class_range: Option<Rng>,
}

/// A class name in a class position: an `@class(...)` mark, a `@params` class
/// token or a projection's class.
#[derive(Clone, Debug)]
pub struct ClassOccurrence {
    pub name: String,
    pub range: Rng,
}

/// A `@params` entry: `name :: type @class`.
#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub name_range: Rng,
    pub ty: String,
    /// The entry as printed, which is what a hover shows.
    pub text: String,
}

/// A static slot table entry: `slot = unit`.
#[derive(Clone, Debug)]
pub struct StaticSlot {
    pub slot: u64,
    pub method: String,
    pub range: Rng,
}

/// One `unit NAME :: TYPE` record and the body it prints.
#[derive(Clone, Debug)]
pub struct Unit {
    pub name: String,
    pub name_range: Rng,
    pub ty: String,
    pub ty_range: Rng,
    /// The whole `unit ... end` block.
    pub block: Rng,
    pub anns: Vec<Annotation>,
    pub params: Vec<Param>,
    pub statics: Vec<StaticSlot>,
    pub body: Option<Form>,
}

impl Unit {
    /// The annotation of `name`, when the record carries one.
    pub fn annotation(&self, name: &str) -> Option<&Annotation> {
        self.anns.iter().find(|ann| ann.name == name)
    }

    /// The `@stage` value.
    pub fn stage(&self) -> Option<&str> {
        self.annotation("stage").map(|ann| ann.value.as_str())
    }

    /// The `@span` value.
    pub fn span(&self) -> Option<&str> {
        self.annotation("span").map(|ann| ann.value.as_str())
    }

    /// The `@class` value.
    pub fn class(&self) -> Option<&str> {
        self.annotation("class").map(|ann| ann.value.as_str())
    }

    /// The `@params` entry of `name`.
    pub fn param(&self, name: &str) -> Option<&Param> {
        self.params.iter().find(|param| param.name == name)
    }
}

/// Everything the server reads from one Core file.
#[derive(Clone, Debug, Default)]
pub struct Syntax {
    pub header: Vec<HeaderLine>,
    pub units: Vec<Unit>,
    /// References in source order.
    pub refs: Vec<Ref>,
    /// Literals in source order.
    pub literals: Vec<Literal>,
    /// Projections in source order.
    pub projections: Vec<Projection>,
    /// Class names in class positions, in source order.
    pub classes: Vec<ClassOccurrence>,
}

impl Syntax {
    /// Every expression node in the file, in source order.
    pub fn forms(&self) -> Vec<&Form> {
        let mut out = Vec::new();
        for unit in &self.units {
            if let Some(body) = &unit.body {
                body.walk(&mut out);
            }
        }
        out
    }

    /// The unit of `name`. `last` picks the later record when a file prints
    /// the same name twice, which is what a duplicate unit's finding means.
    pub fn unit(&self, name: &str, last: bool) -> Option<&Unit> {
        let mut found = None;
        for unit in &self.units {
            if unit.name == name {
                found = Some(unit);
                if !last {
                    break;
                }
            }
        }
        found
    }

    /// The range of the syntax a verifier finding names, when the file still
    /// shows it. `None` for a header finding, which the reader located.
    fn locate(&self, error: &Error) -> Option<Rng> {
        if error.unit.is_empty() || error.path == "header" {
            return None;
        }
        let duplicate = error.message.contains("not unique");
        let unit = self.unit(&error.unit, duplicate)?;
        match error.path.as_str() {
            "" => return Some(unit.name_range),
            "record" => return Some(unit.block),
            "@" => {
                let name = annotation_in(&error.message)?;
                return Some(unit.annotation(&name).map(|ann| ann.range).unwrap_or(unit.block));
            }
            "@stage" | "@span" | "params" => {
                let name = error.path.trim_start_matches('@');
                return Some(unit.annotation(name).map(|ann| ann.range).unwrap_or(unit.block));
            }
            _ => {}
        }
        if let Some(slot) = error.path.strip_prefix("static:") {
            let found = unit.statics.iter().find(|entry| entry.slot.to_string() == slot);
            return Some(found.map(|entry| entry.range).unwrap_or(unit.block));
        }
        let found = unit.body.as_ref().and_then(|body| find_path(body, &error.path));
        Some(found.unwrap_or(unit.block))
    }
}

/// The `@name` an annotation finding names, if its message carries one.
fn annotation_in(message: &str) -> Option<String> {
    let at = message.find('@')?;
    let rest = &message[at + 1..];
    let end = rest
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

/// The range of the form whose structural path is `path`.
fn find_path(form: &Form, path: &str) -> Option<Rng> {
    if form.path == path {
        return Some(form.range);
    }
    for child in &form.children {
        if let Item::Form(child) = child {
            if let Some(range) = find_path(child, path) {
                return Some(range);
            }
        }
    }
    None
}

// ------------------------------------------------------------ the scanner

/// The file as characters plus each line's start offset, so a character
/// offset converts to an LSP position in one step.
struct Source {
    chars: Vec<char>,
    /// The character offset each line starts at; `starts[0]` is zero.
    starts: Vec<usize>,
}

impl Source {
    fn new(text: &str) -> Source {
        let chars: Vec<char> = text.chars().collect();
        let mut starts = vec![0usize];
        for (offset, c) in chars.iter().enumerate() {
            if *c == '\n' {
                starts.push(offset + 1);
            }
        }
        Source { chars, starts }
    }

    fn lines(&self) -> usize {
        self.starts.len()
    }

    /// Line `index` without its terminator.
    fn line(&self, index: usize) -> &[char] {
        let start = self.starts[index];
        let mut end = match self.starts.get(index + 1) {
            Some(next) => next - 1,
            None => self.chars.len(),
        };
        while end > start && self.chars[end - 1] == '\r' {
            end -= 1;
        }
        &self.chars[start..end]
    }

    /// Line `index` as text.
    fn text(&self, index: usize) -> String {
        self.line(index).iter().collect()
    }

    /// The LSP position of a character offset.
    fn pos(&self, offset: usize) -> Pos {
        let offset = offset.min(self.chars.len());
        let line = self.starts.partition_point(|start| *start <= offset).saturating_sub(1);
        let col: usize = self.chars[self.starts[line]..offset]
            .iter()
            .map(|c| c.len_utf16())
            .sum();
        (line as u32, col as u32)
    }

    fn rng(&self, start: usize, end: usize) -> Rng {
        Rng { start: self.pos(start), end: self.pos(end) }
    }

    /// A whole line, terminator excluded.
    fn line_rng(&self, index: usize) -> Rng {
        let start = self.starts[index];
        self.rng(start, start + self.line(index).len())
    }

    /// The character offset of column `col` on `line`.
    fn offset(&self, line: usize, col: usize) -> usize {
        self.starts[line] + col
    }

    fn slice(&self, start: usize, end: usize) -> String {
        let start = start.min(self.chars.len());
        let end = end.min(self.chars.len()).max(start);
        self.chars[start..end].iter().collect()
    }
}

/// The offset of `needle` in `line` from `from`, as a character index.
fn find_chars(line: &[char], from: usize, needle: &str) -> Option<usize> {
    let needle: Vec<char> = needle.chars().collect();
    if needle.is_empty() {
        return Some(from);
    }
    let mut at = from;
    while at + needle.len() <= line.len() {
        if line[at..at + needle.len()] == needle[..] {
            return Some(at);
        }
        at += 1;
    }
    None
}

/// `line[from..to]` trimmed of whitespace, as a character range.
fn word_span(line: &[char], from: usize, to: usize) -> (usize, usize) {
    let to = to.min(line.len());
    let from = from.min(to);
    let mut start = from;
    let mut end = to;
    while start < end && line[start].is_whitespace() {
        start += 1;
    }
    while end > start && line[end - 1].is_whitespace() {
        end -= 1;
    }
    (start, end)
}

/// A character cursor over the file. Bodies run over several lines, so the
/// cursor crosses a line end as whitespace.
struct Cursor<'a> {
    src: &'a Source,
    line: usize,
    col: usize,
}

impl<'a> Cursor<'a> {
    fn new(src: &'a Source, line: usize, col: usize) -> Cursor<'a> {
        Cursor { src, line, col }
    }

    fn peek(&self) -> Option<char> {
        let line = self.src.line(self.line);
        if self.col < line.len() {
            Some(line[self.col])
        } else if self.line + 1 < self.src.lines() {
            Some('\n')
        } else {
            None
        }
    }

    fn bump(&mut self) {
        let len = self.src.line(self.line).len();
        if self.col < len {
            self.col += 1;
        } else if self.line + 1 < self.src.lines() {
            self.line += 1;
            self.col = 0;
        }
    }

    fn offset(&self) -> usize {
        self.src.offset(self.line, self.col)
    }

    fn skip_ws(&mut self) {
        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.bump();
        }
    }

    /// A token up to whitespace or a parenthesis.
    fn token(&mut self) -> String {
        let mut out = String::new();
        while let Some(c) = self.peek() {
            if c.is_whitespace() || c == '(' || c == ')' {
                break;
            }
            out.push(c);
            self.bump();
        }
        out
    }

    /// A form's keyword: a token that also stops at a nested form, a quoted
    /// atom and a mark.
    fn head_token(&mut self) -> String {
        let mut out = String::new();
        while let Some(c) = self.peek() {
            if c.is_whitespace() || matches!(c, '(' | ')' | '"' | '@') {
                break;
            }
            out.push(c);
            self.bump();
        }
        out
    }

    /// A quoted atom, quotes included.
    fn string_token(&mut self) -> String {
        let mut out = String::new();
        if self.peek() != Some('"') {
            return out;
        }
        out.push('"');
        self.bump();
        let mut escaped = false;
        while let Some(c) = self.peek() {
            out.push(c);
            self.bump();
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                break;
            }
        }
        out
    }
}

/// Read a mark: `@name`, `@name(value)`.
fn parse_mark(src: &Source, cursor: &mut Cursor) -> Mark {
    let start = cursor.offset();
    cursor.bump(); // `@`
    let name_start = cursor.offset();
    let mut name = String::new();
    while let Some(c) = cursor.peek() {
        if c.is_whitespace() || matches!(c, '(' | ')' | '@' | '"') {
            break;
        }
        name.push(c);
        cursor.bump();
    }
    let name_range = src.rng(name_start, cursor.offset());
    let mut value = String::new();
    let mut value_range = Rng::default();
    if cursor.peek() == Some('(') {
        cursor.bump();
        let value_start = cursor.offset();
        let mut depth = 1;
        while let Some(c) = cursor.peek() {
            match c {
                '(' => {
                    depth += 1;
                    cursor.bump();
                }
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    cursor.bump();
                }
                '"' => {
                    cursor.string_token();
                }
                _ => cursor.bump(),
            }
        }
        let value_end = cursor.offset();
        if cursor.peek() == Some(')') {
            cursor.bump();
        }
        value = src.slice(value_start, value_end);
        value_range = src.rng(value_start, value_end);
    }
    Mark { name, range: src.rng(start, cursor.offset()), name_range, value, value_range }
}

/// Read one expression node. `path` is the structural path the verifier gives
/// the node's position.
fn parse_form(
    src: &Source,
    syn: &mut Syntax,
    cursor: &mut Cursor,
    unit: usize,
    path: String,
) -> Option<Form> {
    cursor.skip_ws();
    let start = cursor.offset();
    let ch = cursor.peek()?;
    if ch != '(' {
        let text = if ch == '"' { cursor.string_token() } else { cursor.token() };
        if text.is_empty() {
            return None;
        }
        let range = src.rng(start, cursor.offset());
        return Some(Form {
            head: text,
            head_range: range,
            range,
            path,
            atom: true,
            marks: Vec::new(),
            children: Vec::new(),
        });
    }
    cursor.bump();
    cursor.skip_ws();
    let head_start = cursor.offset();
    let head = cursor.head_token();
    let head_range = src.rng(head_start, cursor.offset());
    let mut marks = Vec::new();
    let mut children = Vec::new();
    loop {
        cursor.skip_ws();
        let Some(ch) = cursor.peek() else { break };
        match ch {
            ')' => {
                cursor.bump();
                break;
            }
            '(' => {
                // An alternative prints as `((pattern) body)`, and the
                // verifier names its fields from one, so the path rule
                // differs from the numbered default.
                let child = if head.is_empty() {
                    if children.is_empty() {
                        format!("{path}:pattern")
                    } else {
                        format!("{path}:field {}", children.len())
                    }
                } else {
                    child_path(&head, children.len(), &path)
                };
                match parse_form(src, syn, cursor, unit, child) {
                    Some(child) => children.push(Item::Form(Box::new(child))),
                    None => break,
                }
            }
            '@' => marks.push(parse_mark(src, cursor)),
            _ => {
                let atom_start = cursor.offset();
                let text = if ch == '"' { cursor.string_token() } else { cursor.token() };
                let range = src.rng(atom_start, cursor.offset());
                if text.is_empty() {
                    cursor.bump();
                } else {
                    children.push(Item::Atom(Atom { text, range }));
                }
            }
        }
    }
    let range = src.rng(start, cursor.offset());
    let form = Form { head, head_range, range, path, atom: false, marks, children };
    record(syn, unit, &form);
    Some(form)
}

/// Record what a finished node means: a reference, a literal, a projection,
/// and the classes its marks name.
fn record(syn: &mut Syntax, unit: usize, form: &Form) {
    for mark in &form.marks {
        if mark.name == "class" && !mark.value.is_empty() {
            syn.classes
                .push(ClassOccurrence { name: mark.value.clone(), range: mark.value_range });
        }
    }
    let kind = match form.head.as_str() {
        "local" => Some(RefKind::Local),
        "global" => Some(RefKind::Global),
        "ctor" => Some(RefKind::Ctor),
        "prim" => Some(RefKind::Prim),
        "foreign" => Some(RefKind::Foreign),
        _ => None,
    };
    if let Some(kind) = kind {
        if let Some(atom) = form.atom(0) {
            let name = match kind {
                RefKind::Foreign => strip_quotes(&atom.text),
                _ => unquote(&atom.text),
            };
            syn.refs.push(Ref { kind, name, range: atom.range, unit });
        }
        return;
    }
    let literal = match form.head.as_str() {
        "int" => Some(LiteralKind::Int),
        "nat" => Some(LiteralKind::Nat),
        "bigint" => Some(LiteralKind::Big),
        "bytes" => Some(LiteralKind::Bytes),
        "string" => Some(LiteralKind::String),
        "sym" => Some(LiteralKind::Sym),
        "bool" => Some(LiteralKind::Bool),
        "unit" => Some(LiteralKind::Unit),
        _ => None,
    };
    if let Some(kind) = literal {
        let (text, range) = match form.atom(0) {
            Some(atom) => (atom.text.clone(), atom.range),
            None => (String::new(), form.head_range),
        };
        syn.literals.push(Literal { kind, text, range, unit });
        return;
    }
    if form.head == "dictproj" {
        let slot_atom = form.atom(1);
        let slot = slot_atom.and_then(|atom| atom.text.parse::<u64>().ok());
        let slot_range = slot_atom.map(|atom| atom.range);
        let class_atom = form.atom(2);
        let class = class_atom
            .map(|atom| unquote(&atom.text))
            .filter(|name| !name.is_empty() && name != "-");
        let class_range = class_atom.map(|atom| atom.range);
        if let (Some(name), Some(range)) = (class.clone(), class_range) {
            syn.classes.push(ClassOccurrence { name, range });
        }
        syn.projections
            .push(Projection { range: form.range, slot, slot_range, class, class_range });
    }
}

/// A `"name"` atom without its quotes.
fn strip_quotes(text: &str) -> String {
    text.strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .unwrap_or(text)
        .to_string()
}

/// Walk a Core file and describe it. The reader is deliberately shallow: it
/// re-reads the printed shape, never the program the printer started from.
struct Builder<'a> {
    src: &'a Source,
    syn: Syntax,
}

impl<'a> Builder<'a> {
    fn build(mut self) -> Syntax {
        let lines = self.src.lines();
        let mut i = 0;
        if lines > 0 && self.src.text(0) == "core 1" {
            self.syn.header.push(HeaderLine {
                kind: HeaderKind::Version,
                value: "1".to_string(),
                range: self.src.line_rng(0),
            });
            i = 1;
        }
        while i < lines {
            let Some(kind) = self.header_kind(i) else { break };
            let text = self.src.text(i);
            let value = text
                .strip_prefix(kind.word())
                .map(|rest| rest.trim_start().to_string())
                .unwrap_or_default();
            self.syn.header.push(HeaderLine { kind, value, range: self.src.line_rng(i) });
            i += 1;
        }
        while i < lines {
            if self.src.text(i).starts_with("unit ") {
                i = self.unit(i);
            } else {
                i += 1;
            }
        }
        self.syn
    }

    fn header_kind(&self, line: usize) -> Option<HeaderKind> {
        let text = self.src.text(line);
        if text.starts_with("module ") {
            Some(HeaderKind::Module)
        } else if text.starts_with("target ") {
            Some(HeaderKind::Target)
        } else if text.starts_with("source ") {
            Some(HeaderKind::Source)
        } else if text.starts_with("@ticks ") {
            Some(HeaderKind::Ticks)
        } else if text.starts_with("global ") {
            Some(HeaderKind::Global)
        } else {
            None
        }
    }

    /// Read one unit record and return the line after its block.
    fn unit(&mut self, start: usize) -> usize {
        let chars = self.src.line(start).to_vec();
        let prefix = "unit ".chars().count();
        let sep = find_chars(&chars, prefix, " :: ");
        let (name_start, name_end) = word_span(&chars, prefix, sep.unwrap_or(chars.len()));
        let (ty_start, ty_end) = match sep {
            Some(at) => word_span(&chars, at + 4, chars.len()),
            None => (chars.len(), chars.len()),
        };
        let index = self.syn.units.len();
        self.syn.units.push(Unit {
            name: chars[name_start..name_end].iter().collect(),
            name_range: self
                .src
                .rng(self.src.offset(start, name_start), self.src.offset(start, name_end)),
            ty: chars[ty_start..ty_end].iter().collect(),
            ty_range: self.src.rng(self.src.offset(start, ty_start), self.src.offset(start, ty_end)),
            block: Rng::default(),
            anns: Vec::new(),
            params: Vec::new(),
            statics: Vec::new(),
            body: None,
        });
        let mut i = start + 1;
        let mut end_line = None;
        while i < self.src.lines() {
            let text = self.src.text(i);
            if text == "end" {
                end_line = Some(i);
                break;
            }
            if text.starts_with("  @") {
                self.annotation(index, i);
            } else if text.starts_with("  body ") {
                let col = "  body ".chars().count();
                let mut cursor = Cursor::new(self.src, i, col);
                let body =
                    parse_form(self.src, &mut self.syn, &mut cursor, index, "body".to_string());
                self.syn.units[index].body = body;
                // A body runs until its form closes, which may be past this
                // line; the line that closes it is the last one it occupies.
                i = (cursor.line + 1).max(i + 1);
                continue;
            } else if let Some(rest) = text.strip_prefix("  static ") {
                self.statics(index, i, rest);
            }
            i += 1;
        }
        // The block ends at `end`, or at the last line the record occupied.
        let last = end_line.unwrap_or_else(|| i.saturating_sub(1).max(start));
        let next = end_line.map(|end| end + 1).unwrap_or(i);
        self.syn.units[index].block = Rng::new(
            self.src.line_rng(start).start,
            self.src.line_rng(last.min(self.src.lines().saturating_sub(1)).max(start)).end,
        );
        next
    }

    /// Read an annotation line: `@name(value)` or `@name value`.
    fn annotation(&mut self, unit: usize, line: usize) {
        let chars = self.src.line(line).to_vec();
        let at = "  ".chars().count();
        let mut name_end = at + 1;
        while name_end < chars.len() && !matches!(chars[name_end], ' ' | '(' | ')') {
            name_end += 1;
        }
        let name: String = chars[at + 1..name_end].iter().collect();
        let (value_start, value_end) = if name_end < chars.len() && chars[name_end] == '(' {
            let mut close = chars.len();
            while close > name_end && chars[close - 1] != ')' {
                close -= 1;
            }
            let close = if close > name_end { close - 1 } else { name_end + 1 };
            (name_end + 1, close.max(name_end + 1))
        } else {
            word_span(&chars, name_end, chars.len())
        };
        let value: String = chars[value_start..value_end].iter().collect();
        let mut end = chars.len();
        while end > at && chars[end - 1].is_whitespace() {
            end -= 1;
        }
        let ann = Annotation {
            name: name.clone(),
            range: self.src.rng(self.src.offset(line, at), self.src.offset(line, end)),
            name_range: self
                .src
                .rng(self.src.offset(line, at + 1), self.src.offset(line, name_end)),
            value_range: self
                .src
                .rng(self.src.offset(line, value_start), self.src.offset(line, value_end)),
            value,
        };
        if name == "params" {
            let params = self.params(line, value_start, value_end);
            self.syn.units[unit].params = params;
        }
        if name == "class" && !ann.value.is_empty() {
            self.syn
                .classes
                .push(ClassOccurrence { name: ann.value.clone(), range: ann.value_range });
        }
        self.syn.units[unit].anns.push(ann);
    }

    /// Read a `@params` value: `name :: type @class`, comma separated.
    fn params(&mut self, line: usize, from: usize, to: usize) -> Vec<Param> {
        let chars = self.src.line(line);
        let to = to.min(chars.len());
        let from = from.min(to);
        let mut spans = Vec::new();
        let mut start = from;
        let mut depth = 0i32;
        let mut at = from;
        while at < to {
            match chars[at] {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                ',' if depth == 0 => {
                    spans.push((start, at));
                    start = at + 1;
                }
                _ => {}
            }
            at += 1;
        }
        spans.push((start, to));
        let mut out = Vec::new();
        for (part_start, part_end) in spans {
            let (start, end) = word_span(chars, part_start, part_end);
            if start == end {
                continue;
            }
            let (name_start, name_end, ty) = match find_chars(chars, start, " :: ") {
                Some(sep) if sep < end => {
                    let (name_start, name_end) = word_span(chars, start, sep);
                    let (ty_start, ty_end) = word_span(chars, sep + 4, end);
                    (
                        name_start,
                        name_end,
                        chars[ty_start.min(ty_end)..ty_end.max(ty_start)].iter().collect(),
                    )
                }
                _ => (start, end, String::new()),
            };
            // The class token follows the type, as `@owned`.
            let mut class_at = name_end;
            while class_at < end && chars[class_at] != '@' {
                class_at += 1;
            }
            if class_at < end {
                let mut class_end = class_at + 1;
                while class_end < end && !chars[class_end].is_whitespace() {
                    class_end += 1;
                }
                let name: String = chars[(class_at + 1).min(class_end)..class_end.max(class_at + 1)]
                    .iter()
                    .collect();
                if !name.is_empty() {
                    self.syn.classes.push(ClassOccurrence {
                        name,
                        range: self.src.rng(
                            self.src.offset(line, class_at + 1),
                            self.src.offset(line, class_end),
                        ),
                    });
                }
            }
            out.push(Param {
                name: chars[name_start..name_end].iter().collect(),
                name_range: self
                    .src
                    .rng(self.src.offset(line, name_start), self.src.offset(line, name_end)),
                ty,
                text: chars[start..end].iter().collect(),
            });
        }
        out
    }

    /// Read a `static { slot = unit, ... }` line.
    fn statics(&mut self, unit: usize, line: usize, rest: &str) {
        let chars = self.src.line(line).to_vec();
        let offset = chars.len().saturating_sub(rest.chars().count());
        let Some(open) = find_chars(&chars, offset, "{") else { return };
        let Some(close) = find_chars(&chars, open, "}") else { return };
        let mut spans = Vec::new();
        let mut start = open + 1;
        let mut depth = 0i32;
        let mut at = open + 1;
        while at < close {
            match chars[at] {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                ',' if depth == 0 => {
                    spans.push((start, at));
                    start = at + 1;
                }
                _ => {}
            }
            at += 1;
        }
        spans.push((start, close));
        let mut slots = Vec::new();
        for (part_start, part_end) in spans {
            let (start, end) = word_span(&chars, part_start, part_end);
            let Some(equals) = find_chars(&chars, start, "=") else { continue };
            if equals >= end {
                continue;
            }
            let slot: String = chars[start..equals].iter().collect();
            let Ok(slot) = slot.trim().parse::<u64>() else { continue };
            let (method_start, method_end) = word_span(&chars, equals + 1, end);
            slots.push(StaticSlot {
                slot,
                method: chars[method_start.min(method_end)..method_end.max(method_start)]
                    .iter()
                    .collect(),
                range: self.src.rng(self.src.offset(line, start), self.src.offset(line, end)),
            });
        }
        self.syn.units[unit].statics = slots;
    }
}

/// Describe a Core file's structure with source ranges.
pub fn syntax(text: &str) -> Syntax {
    let src = Source::new(text);
    let mut syn = Builder { src: &src, syn: Syntax::default() }.build();
    // The scanner records a node when it closes, so the flat lists are in
    // post-order; sort them into the source order the API promises.
    syn.refs.sort_by_key(|item| (item.range.start, item.range.end));
    syn.literals.sort_by_key(|item| (item.range.start, item.range.end));
    syn.projections.sort_by_key(|item| (item.range.start, item.range.end));
    syn.classes.sort_by_key(|item| (item.range.start, item.range.end));
    syn
}

// ------------------------------------------------------------- hover

/// What a hover over a Core position reports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hover {
    pub range: Rng,
    pub markdown: String,
}

/// A local a body binds, with the annotation the printer gave it.
#[derive(Clone, Debug)]
struct Binder {
    name: String,
    detail: String,
}

/// The hover for a position (0-based line, 0-based UTF-16 column), if the text
/// says anything about it.
pub fn hover(text: &str, line: u32, column: u32) -> Option<Hover> {
    let syn = syntax(text);
    let pos = (line, column);
    let unit = syn.units.iter().find(|unit| unit.block.contains(pos))?;
    if unit.name_range.contains(pos) {
        return Some(Hover { range: unit.name_range, markdown: unit_markdown(unit) });
    }
    if unit.ty_range.contains(pos) {
        return Some(Hover { range: unit.ty_range, markdown: format!("`:: {}`", unit.ty) });
    }
    for ann in &unit.anns {
        if ann.range.contains(pos) {
            return Some(Hover { range: ann.range, markdown: annotation_markdown(&syn, ann) });
        }
    }
    for param in &unit.params {
        if param.name_range.contains(pos) {
            return Some(param_hover(param));
        }
    }
    if let Some(body) = &unit.body {
        let mut scope: Vec<Binder> = unit
            .params
            .iter()
            .map(|param| Binder { name: param.name.clone(), detail: param.text.clone() })
            .collect();
        if let Some(hover) = hover_form(&syn, unit, body, pos, &mut scope) {
            return Some(hover);
        }
    }
    for class in &syn.classes {
        if class.range.contains(pos) {
            return Some(class_hover(&syn, &class.name, class.range));
        }
    }
    for literal in &syn.literals {
        if literal.range.contains(pos) {
            return Some(Hover { range: literal.range, markdown: literal_markdown(literal) });
        }
    }
    None
}

/// The hover for a node, given the locals its enclosing forms bind.
fn hover_form(
    syn: &Syntax,
    unit: &Unit,
    form: &Form,
    pos: Pos,
    scope: &mut Vec<Binder>,
) -> Option<Hover> {
    if !form.range.contains(pos) {
        return None;
    }
    for mark in &form.marks {
        if mark.range.contains(pos) {
            return Some(Hover { range: mark.range, markdown: mark_markdown(syn, mark) });
        }
    }
    if let Some(hover) = operand_hover(syn, form, pos, scope) {
        return Some(hover);
    }
    if form.head_range.contains(pos) && !form.head.is_empty() {
        if let Some(param) = unit.param(&form.head) {
            return Some(param_hover(param));
        }
        return Some(Hover { range: form.head_range, markdown: format!("`{}`", form.head) });
    }
    // An alternative: its first child is the pattern, which binds in the rest
    // of the alternative.
    if form.head.is_empty() && !form.children.is_empty() {
        let mut binders = Vec::new();
        if let Item::Form(pattern) = &form.children[0] {
            form_pattern_binders(pattern, unit, &mut binders);
        }
        for (index, child) in form.children.iter().enumerate() {
            let before = scope.len();
            if index > 0 {
                scope.extend(binders.iter().cloned());
            }
            let hit = match child {
                Item::Atom(atom) => atom_hover(unit, atom, pos),
                Item::Form(child) => hover_form(syn, unit, child, pos, scope),
            };
            scope.truncate(before);
            if hit.is_some() {
                return hit;
            }
        }
        return None;
    }
    if form.head == "let" {
        let binder = form.atom(0).map(|atom| binder_for(unit, form, &atom.text));
        let body_index = form.children.len().saturating_sub(1);
        for (index, child) in form.children.iter().enumerate() {
            let before = scope.len();
            let hit = match child {
                Item::Atom(atom) => {
                    if index == 0 && atom.range.contains(pos) {
                        Some(Hover {
                            range: atom.range,
                            markdown: binder_markdown(&atom.text, binder.as_ref()),
                        })
                    } else {
                        None
                    }
                }
                Item::Form(child) => {
                    if index == body_index {
                        if let Some(binder) = &binder {
                            scope.push(binder.clone());
                        }
                    }
                    hover_form(syn, unit, child, pos, scope)
                }
            };
            scope.truncate(before);
            if hit.is_some() {
                return hit;
            }
        }
        return None;
    }
    if form.head == "lam" {
        // The parameter list is the first child; its names bind in the rest of
        // the form.
        let mut binders = Vec::new();
        if let Some(params) = form.form(0) {
            if !params.head.is_empty() {
                binders.push(binder_for(unit, params, &params.head));
            }
            for child in &params.children {
                if let Item::Atom(atom) = child {
                    binders.push(binder_for(unit, params, &atom.text));
                }
            }
        }
        if let Some(params) = form.form(0) {
            if let Some(hover) = hover_form(syn, unit, params, pos, scope) {
                return Some(hover);
            }
        }
        for (index, child) in form.children.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let Item::Form(child) = child else { continue };
            let before = scope.len();
            scope.extend(binders.iter().cloned());
            let hit = hover_form(syn, unit, child, pos, scope);
            scope.truncate(before);
            if hit.is_some() {
                return hit;
            }
        }
        return None;
    }
    for child in &form.children {
        let hit = match child {
            Item::Atom(atom) => atom_hover(unit, atom, pos),
            Item::Form(child) => hover_form(syn, unit, child, pos, scope),
        };
        if hit.is_some() {
            return hit;
        }
    }
    None
}

/// The hover for a form's operand: a reference, a projection's slot or class.
fn operand_hover(syn: &Syntax, form: &Form, pos: Pos, scope: &[Binder]) -> Option<Hover> {
    match form.head.as_str() {
        "local" => {
            let atom = form.atom(0)?;
            if !atom.range.contains(pos) {
                return None;
            }
            let name = &atom.text;
            let binder = scope.iter().rev().find(|binder| &binder.name == name);
            let markdown = match binder {
                Some(binder) => format!("local `{name}`\n\nbound by `{}`", binder.detail),
                None => format!("local `{name}`\n\nnot bound by an enclosing form"),
            };
            Some(Hover { range: atom.range, markdown })
        }
        "global" | "ctor" => {
            let atom = form.atom(0)?;
            if !atom.range.contains(pos) {
                return None;
            }
            let name = unquote(&atom.text);
            let markdown = match syn.unit(&name, false) {
                Some(unit) => ref_markdown(unit),
                None => format!("imported unit: no definition in this file\n\n`{name}`"),
            };
            Some(Hover { range: atom.range, markdown })
        }
        "prim" => {
            let atom = form.atom(0)?;
            if !atom.range.contains(pos) {
                return None;
            }
            Some(Hover {
                range: atom.range,
                markdown: format!("runtime prim `{}`", unquote(&atom.text)),
            })
        }
        "foreign" => {
            let atom = form.atom(0)?;
            if !atom.range.contains(pos) {
                return None;
            }
            Some(Hover {
                range: atom.range,
                markdown: format!("foreign unit `{}`", strip_quotes(&atom.text)),
            })
        }
        "instdict" => {
            let atom = form.atom(0)?;
            if !atom.range.contains(pos) {
                return None;
            }
            let name = unquote(&atom.text);
            let markdown = match syn.unit(&name, false) {
                Some(unit) => ref_markdown(unit),
                None => format!("dictionary `{name}`"),
            };
            Some(Hover { range: atom.range, markdown })
        }
        "dictproj" => {
            if let Some(atom) = form.atom(1) {
                if atom.range.contains(pos) {
                    return Some(Hover {
                        range: atom.range,
                        markdown: format!(
                            "dictionary slot {}\n\nthe slot the dictionary's class index names",
                            atom.text
                        ),
                    });
                }
            }
            let atom = form.atom(2)?;
            if !atom.range.contains(pos) {
                return None;
            }
            Some(class_hover(syn, &unquote(&atom.text), atom.range))
        }
        _ => None,
    }
}

/// The hover for an atom child, which is a parameter's name when `@params`
/// prints it.
fn atom_hover(unit: &Unit, atom: &Atom, pos: Pos) -> Option<Hover> {
    if !atom.range.contains(pos) {
        return None;
    }
    match unit.param(&atom.text) {
        Some(param) => Some(param_hover(param)),
        None => Some(Hover { range: atom.range, markdown: format!("`{}`", atom.text) }),
    }
}

fn param_hover(param: &Param) -> Hover {
    Hover {
        range: param.name_range,
        markdown: format!("parameter `{}`\n\n`{}`", param.name, param.text),
    }
}

/// The binder a form introduces under `index`, with the annotation the printer
/// gave it.
fn binder_for(unit: &Unit, form: &Form, name: &str) -> Binder {
    if let Some(param) = unit.param(name) {
        return Binder { name: name.to_string(), detail: param.text.clone() };
    }
    // A printer that writes `:: type` between a binder and its value prints it
    // as a `::`-prefixed atom; this compiler prints only the class.
    let typed = form.children.iter().find_map(|child| match child {
        Item::Atom(atom) if atom.text.starts_with(":: ") => Some(atom.text.clone()),
        _ => None,
    });
    Binder {
        name: name.to_string(),
        detail: typed.unwrap_or_else(|| "local binder".to_string()),
    }
}

fn binder_markdown(name: &str, binder: Option<&Binder>) -> String {
    match binder {
        Some(binder) => format!("local `{name}`\n\nbound by `{}`", binder.detail),
        None => format!("local `{name}`"),
    }
}

/// The locals a pattern binds, by the rule the verifier uses: a `con` pattern
/// binds its fields, an `int`/`nat` pattern binds nothing, and any other list
/// head is a variable.
fn form_pattern_binders(pattern: &Form, unit: &Unit, out: &mut Vec<Binder>) {
    if pattern.atom {
        if pattern.head != "_" {
            out.push(binder_for(unit, pattern, &pattern.head));
        }
        return;
    }
    match pattern.head.as_str() {
        "con" => {
            for child in pattern.children.iter().skip(1) {
                if let Item::Form(child) = child {
                    form_pattern_binders(child, unit, out);
                }
            }
        }
        "int" | "nat" | "" => {}
        name => out.push(binder_for(unit, pattern, name)),
    }
}

/// A unit's type, class, span and stage.
fn unit_markdown(unit: &Unit) -> String {
    let mut out = format!("`{}`\n\ntype: `{}`", unit.name, dash(&unit.ty));
    if let Some(class) = unit.class() {
        out.push_str(&format!("\nclass: `{class}`"));
    }
    if let Some(span) = unit.span() {
        out.push_str(&format!("\nspan: `{span}`"));
        if let Some((file, line, column)) = span_position(span) {
            out.push_str(&format!("\nsource `{file}` line {line}, column {column}"));
        }
    }
    if let Some(stage) = unit.stage() {
        out.push_str(&format!("\nstage: `{stage}`"));
    }
    out
}

/// A referenced unit's type and location.
fn ref_markdown(unit: &Unit) -> String {
    let mut out = format!("`{} :: {}`", unit.name, dash(&unit.ty));
    if let Some(span) = unit.span() {
        out.push_str(&format!("\n\nspan: `{span}`"));
    }
    if let Some(stage) = unit.stage() {
        out.push_str(&format!("\n\nstage: `{stage}`"));
    }
    out
}

fn dash(text: &str) -> &str {
    if text.is_empty() {
        "-"
    } else {
        text
    }
}

/// A class name and the dictionary bindings of this file that provide it.
fn class_hover(syn: &Syntax, name: &str, range: Rng) -> Hover {
    let mut markdown = format!("class `{name}`");
    let providers: Vec<&Unit> = syn
        .units
        .iter()
        .filter(|unit| !unit.statics.is_empty() && unquote(&unit.ty) == name)
        .collect();
    if providers.is_empty() {
        markdown.push_str("\n\nno dictionary bindings in this file");
    } else {
        markdown.push_str("\n\nprovided by:");
        for unit in providers {
            let slots: Vec<String> = unit
                .statics
                .iter()
                .map(|slot| format!("{} = `{}`", slot.slot, slot.method))
                .collect();
            markdown.push_str(&format!("\n- `{}`, slots {}", unit.name, slots.join(", ")));
        }
    }
    Hover { range, markdown }
}

fn annotation_markdown(syn: &Syntax, ann: &Annotation) -> String {
    match ann.name.as_str() {
        "span" => match span_position(&ann.value) {
            Some((file, line, column)) => {
                format!("`@span`: source `{file}` line {line}, column {column}")
            }
            None => format!("`@span`\n\n`{}`", ann.value),
        },
        "stage" => format!("stage `{}`", ann.value),
        "class" => class_hover(syn, &ann.value, ann.range).markdown,
        "params" => format!("the unit's parameters\n\n`@params {}`", ann.value),
        "instance" => format!("dictionary instance\n\n`@instance({})`", ann.value),
        _ => {
            if ann.value.is_empty() {
                format!("`@{}`", ann.name)
            } else {
                format!("`@{}`\n\n`{}`", ann.name, ann.value)
            }
        }
    }
}

fn mark_markdown(syn: &Syntax, mark: &Mark) -> String {
    match mark.name.as_str() {
        "class" => class_hover(syn, &mark.value, mark.value_range).markdown,
        "classes" => format!("instantiation classes `{}`", mark.value),
        "weight" => format!("alternative weight `{}`", mark.value),
        _ => format!("`@{}`", mark.name),
    }
}

fn literal_markdown(literal: &Literal) -> String {
    if literal.text.is_empty() {
        format!("{} literal", literal.kind.word())
    } else {
        format!("{} literal `{}`", literal.kind.word(), literal.text)
    }
}

/// A `@span` value as its file, line and column.
fn span_position(value: &str) -> Option<(&str, u32, u32)> {
    let mut parts = value.rsplitn(3, ':');
    let column = parts.next()?.parse::<u32>().ok()?;
    let line = parts.next()?.parse::<u32>().ok()?;
    let file = parts.next()?.trim();
    if file.is_empty() {
        return None;
    }
    Some((file, line, column))
}

/// The range of the unit a `(global X)` names, when `X` is a unit of this
/// file. `None` for an imported name: a jump to a guess is worse than none.
pub fn definition(text: &str, line: u32, column: u32) -> Option<(u32, u32, u32, u32)> {
    let syn = syntax(text);
    let pos = (line, column);
    for reference in &syn.refs {
        if reference.kind != RefKind::Global || !reference.range.contains(pos) {
            continue;
        }
        let unit = syn.unit(&reference.name, false)?;
        return Some(unit.name_range.tuple());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stages_parse_and_alias() {
        assert_eq!(Stage::parse("checked"), Some(Stage::Checked));
        assert_eq!(Stage::parse("elaborated"), Some(Stage::Checked));
        assert_eq!(Stage::parse("solved"), Some(Stage::Checked));
        assert_eq!(Stage::parse("owned"), Some(Stage::Owned));
        assert_eq!(Stage::parse("lowered"), Some(Stage::Lowered));
        assert_eq!(Stage::parse("Checked"), None);
        assert_eq!(Stage::parse(""), None);
        for spelling in Stage::spellings() {
            assert!(Stage::parse(spelling).is_some(), "{spelling}");
        }
        assert!(!Stage::Checked.is_target_specific());
        assert!(!Stage::Owned.is_target_specific());
        assert!(Stage::Lowered.is_target_specific());
    }

    #[test]
    fn ownership_classes_print_their_reclaim_kind() {
        assert_eq!(class_text(Class::Raw), "uncounted");
        assert_eq!(class_text(Class::Obj), "owned");
        assert_eq!(class_text(Class::TObj), "owned");
        assert_eq!(class_text(Class::Param(3)), "param(3)");
    }

    #[test]
    fn identifiers_quote_when_they_leave_the_plain_set() {
        assert_eq!(ident("Data.Core.Nil"), "Data.Core.Nil");
        assert_eq!(ident("Main.main.lam22"), "Main.main.lam22");
        assert_eq!(ident("n1"), "n1");
        assert_eq!(ident("+"), "'+'");
        // A quote inside doubles: the output is open quote, `x`, two quotes,
        // close quote.
        assert_eq!(ident("x'"), concat!("'x", "'''"));
        assert_eq!(ident(""), "''");
        assert_eq!(unquote("'x''"), "x'");
    }

    #[test]
    fn text_and_bytes_escape_the_documented_set() {
        assert_eq!(escape_text("a\"b\\c\nd\te"), "a\\\"b\\\\c\\nd\\te");
        assert_eq!(escape_text("\u{1}"), "\\x01");
        assert_eq!(escape_bytes(&[0, b'a', 0xff]), "\\x00a\\xff");
        // A long constant prints its digest and length, not its contents.
        let long = bytes_node("bytes", &[7u8; 40]);
        let text = text_of(&long);
        assert!(text.starts_with("(bytes "), "{text}");
        assert!(text.ends_with(" 40)"), "{text}");
        assert_eq!(text.split(' ').count(), 3, "digest, then length: {text}");
    }

    fn text_of(node: &Node) -> String {
        node.flat()
    }

    #[test]
    fn types_print_canonically() {
        assert_eq!(canonical_type("Int"), "Int");
        // The quantifier's dot loses its space; the type text is otherwise
        // the checker's own canonical rendering.
        assert_eq!(
            canonical_type("forall (a :: Type) . (a -> (a -> a))"),
            "forall (a :: Type). (a -> (a -> a))"
        );
        assert_eq!(canonical_type("  ((Data.Core.List  Int) -> Int) "), "(Data.Core.List Int) -> Int");
        assert_eq!(canonical_type(""), "-");
    }

    #[test]
    fn arrow_chains_decompose_through_a_using_context() {
        let scheme = "forall (a :: Type). using (oe :: Relation.Ord.Ord a) => (a -> (a -> a))";
        assert_eq!(
            arrow_domains(scheme),
            Some(vec!["a".to_string(), "a".to_string()])
        );
        assert_eq!(
            arrow_domains("(Data.Core.List Int) -> Int"),
            Some(vec!["(Data.Core.List Int)".to_string()])
        );
        // A bare index is not a function, and neither is a parameter.
        assert_eq!(arrow_domains("Int"), None);
        assert_eq!(arrow_domains("a"), None);
    }

    #[test]
    fn parameter_names_keep_meaningful_names_and_number_the_rest() {
        let unit = |params: &[&str]| FnDef {
            id: crate::core::GlobalId("Main.f".to_string()),
            params: params.iter().map(|name| name.to_string()).collect(),
            body: Core::Unit,
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
            meta: crate::core::FnMeta::default(),
            repr: crate::core::UnitRepr::default(),
        };
        // The checker's `v<n>` placeholders are not names: they number in
        // parameter order from zero.
        let naming = parameter_naming(&unit(&["v4341", "v4342", "v4343"]));
        assert_eq!(naming.names, vec!["t0", "t1", "t2"]);
        assert_eq!(naming.next, 3);
        // A constructor's field and a primitive's parameter keep their names.
        assert_eq!(parameter_naming(&unit(&["f0", "f1"])).names, vec!["f0", "f1"]);
        assert_eq!(parameter_naming(&unit(&["p0"])).names, vec!["p0"]);
        // A kept name is never handed out twice: `t0` is taken, so the
        // placeholder becomes `t1`.
        let naming = parameter_naming(&unit(&["t0", "v4342"]));
        assert_eq!(naming.names, vec!["t0", "t1"]);
        assert_eq!(naming.next, 2);
    }

    #[test]
    fn shadowed_binders_print_as_distinct_dense_names() {
        let (text, _) = checked_dump(
            "module Main where\n  shadow :: Int -> Int\n  shadow x = let x = x + 1 in case x of\n    n -> let n = n + 1 in n\n  main :: Int\n  main = shadow 1\n",
        );
        // The parameter is `t0`, and the two shadowing binders are `t1` and
        // `t2`: one name per binder, numbered by first appearance.
        assert!(text.contains("@params t0 :: Int @owned"), "{text}");
        assert!(text.contains("(let t1 @class(owned)"), "{text}");
        assert!(text.contains("(let t2 @class(owned)"), "{text}");
        // No placeholder or `LocalId` reaches the text.
        assert!(!text.contains("$v"), "{text}");
        let placeholder = text
            .split(|c: char| !c.is_ascii_alphanumeric())
            .any(|word| synthesized_name(word));
        assert!(!placeholder, "a `v<n>` placeholder survived: {text}");
        let report = verify(&text).expect("the dump verifies");
        assert!(report.expressions > 3);
    }

    #[test]
    fn nodes_print_marks_after_their_first_child() {
        let node = Node::list("global")
            .child(Node::atom("Data.List.map"))
            .mark("@classes(TT)");
        assert_eq!(node.flat(), "(global Data.List.map @classes(TT))");
        let let_node = Node::list("let")
            .child(Node::atom("n1"))
            .mark("@class(owned)")
            .child(Node::atom("(int 1)"))
            .child(Node::atom("(local n1)"));
        assert_eq!(
            let_node.flat(),
            "(let n1 @class(owned) (int 1) (local n1))"
        );
        assert_eq!(Node::list("unit").flat(), "(unit)");
        let params = Node::list("").child(Node::atom("t0"));
        assert_eq!(params.flat(), "(t0)", "an empty head takes no space");
    }

    #[test]
    fn long_nodes_break_one_child_per_line_under_the_width() {
        let mut node = Node::list("app").child(Node::atom("(global Main.f)"));
        for index in 0..12 {
            node = node.child(Node::atom(format!("(local t{index})")));
        }
        let mut out = String::new();
        render(&node, 1, &mut out);
        let lines: Vec<&str> = out.lines().collect();
        assert!(lines.len() > 1, "a node over the width breaks:\n{out}");
        assert!(lines[0].starts_with("  (app (global Main.f)"), "{out}");
        for line in &lines {
            assert!(line.chars().count() <= LINE_WIDTH, "{line}");
            let indent = line.len() - line.trim_start().len();
            assert_eq!(indent % 2, 0, "indentation is two spaces per level: {line}");
        }
        assert!(lines.last().unwrap().ends_with("))"), "{out}");
    }

    /// Check a source and print it, the way a dump point does.
    fn checked_dump(src: &str) -> (String, Vec<Iface>) {
        let src = src.to_string();
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let (program, ifaces) =
                    crate::driver::check_sources_ifaces(&src, "test.cinch").expect("check");
                let modules: BTreeSet<String> = ["Main".to_string()].into_iter().collect();
                let profile = TargetProfile::native();
                let text = dump(
                    &program,
                    Stage::Checked,
                    &profile,
                    &ifaces,
                    &modules,
                    &BTreeMap::new(),
                    0,
                );
                (text, ifaces)
            })
            .expect("spawn")
            .join()
            .expect("check thread")
    }

    #[test]
    fn a_checked_dump_prints_the_header_units_and_sorted_records() {
        let (text, _ifaces) = checked_dump(
            "module Main where\n  zed :: Int\n  zed = 1\n  alpha :: Int\n  alpha = zed\n  main :: Int\n  main = alpha\n",
        );
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[0], "core 1");
        assert_eq!(lines[1], "module Main");
        assert!(lines[2].starts_with("source sha256:"), "{}", lines[2]);
        assert_eq!(lines[2].len(), "source sha256:".len() + 64);
        // Sorted by canonical name: Main.alpha before Main.main before Main.zed.
        let units: Vec<&str> = lines
            .iter()
            .filter_map(|line| line.strip_prefix("unit "))
            .map(|rest| rest.split(" :: ").next().unwrap_or(rest))
            .collect();
        let mut sorted = units.clone();
        sorted.sort();
        assert_eq!(units, sorted);
        assert!(units.contains(&"Main.alpha"));
        // Determinism: printing the same program twice is the same text.
        let (again, _) = checked_dump(
            "module Main where\n  zed :: Int\n  zed = 1\n  alpha :: Int\n  alpha = zed\n  main :: Int\n  main = alpha\n",
        );
        assert_eq!(text, again);
        // And the reader accepts it.
        let report = verify(&text).expect("the dump verifies");
        assert!(report.units >= 3);
        assert_eq!(report.stage.as_deref(), Some("checked"));
    }

    #[test]
    fn print_without_signatures_leaves_the_type_slot_empty() {
        let (program, ifaces) = {
            let src = "module Main where\n  main :: Int\n  main = 0\n";
            std::thread::Builder::new()
                .stack_size(64 * 1024 * 1024)
                .spawn(move || crate::driver::check_sources_ifaces(src, "test.cinch").expect("check"))
                .expect("spawn")
                .join()
                .expect("thread")
        };
        let profile = TargetProfile::native();
        let modules: BTreeSet<String> = ["Main".to_string()].into_iter().collect();
        let mut ctx = Context::new(Stage::Checked, &profile, "digest");
        ctx.modules = Some(&modules);
        let text = print_with(&program, &ctx);
        assert!(text.contains("unit Main.main :: -\n"), "{text}");
        let digest = source_digest(&ifaces, &modules);
        assert_eq!(digest.len(), 64);
        assert_ne!(digest, source_digest(&ifaces, &BTreeSet::new()));
    }

    #[test]
    fn the_reader_checks_scope_cones_and_slot_tables() {
        let text = "\
core 1
module Main
source sha256:0000000000000000000000000000000000000000000000000000000000000000
unit Main.dict :: Main.Class
  @instance(tparams 0, subdicts 0)
  @stage(checked)
  static {0 = Main.method, 1 = Main.method}
end
unit Main.method :: Int -> Int
  @params t0 :: Int @owned
  @stage(checked)
  body (local t0)
end
unit Main.use :: Int -> Int
  @params t0 :: Int @owned
  @stage(checked)
  body (app (dictproj (instdict Main.dict) 1 Main.Class) (local t0))
end
";
        let report = verify(text).expect("the dictionary use resolves");
        assert_eq!(report.dictionary_bindings, 2);
        assert_eq!(report.projections, 1);
        assert_eq!(report.dictionaries, 1);
        let broken = text.replace("(instdict Main.dict) 1", "(instdict Main.dict) 2");
        let errors = verify(&broken).expect_err("slot 2 is not printed");
        assert_eq!(errors[0].unit, "Main.use");
        assert!(
            errors[0].message.contains("slot 2"),
            "{}",
            errors[0].message
        );
        // A local bound by a `case` alternative's pattern is in scope in that
        // alternative only.
        let scoped = text.replace(
            "  body (local t0)\nend\nunit Main.use",
            "  body (case (local t0) ((con 0 (n1)) (local n1)))\nend\nunit Main.use",
        );
        verify(&scoped).expect("a pattern binder is in scope in its alternative");
        let leak = scoped.replace(
            "body (app (dictproj (instdict Main.dict) 1 Main.Class) (local t0))",
            "body (local n1)",
        );
        let errors = verify(&leak).expect_err("a pattern binder is not in scope after the case");
        assert!(errors.iter().any(|error| error.message.contains("n1")), "{errors:?}");
    }

    #[test]
    fn diff_walks_units_in_canonical_order() {
        let left = "\
core 1
module Main
source sha256:0000000000000000000000000000000000000000000000000000000000000000
unit Main.a :: Int
  @stage(checked)
  body (int 1)
end
unit Main.b :: Int
  @stage(checked)
  body (int 1)
end
";
        let right = left.replace("Main.b", "Main.c");
        assert_eq!(
            diff(left, &right).as_deref(),
            Some("Main.b: only in the left file")
        );
        let changed = left.replace("  body (int 1)\nend\n", "  body (int 2)\nend\n");
        assert_eq!(
            diff(left, &changed).as_deref(),
            Some("Main.a:body: left (int 1) right (int 2)")
        );
        assert_eq!(diff(left, left), None);
    }
    // -------------------------------------------------- the syntax view

    const SAMPLE: &str = "\
core 1
module Main
source sha256:0000000000000000000000000000000000000000000000000000000000000000
unit Main.EchoInt :: Main.Echo
  @span tests/core/dictionaries.cinch:2:3
  @instance(tparams 0, subdicts 0)
  @stage(checked)
  static {0 = Main.EchoInt.echo}
end
unit Main.EchoInt.echo :: -
  @params v4343 :: - @owned
  @stage(checked)
  body (app (dictproj (instdict Main.EchoInt) 0 Main.Echo) (local v4343) (int 1))
end
unit Main.main :: Int
  @span tests/core/dictionaries.cinch:8:1
  @stage(checked)
  body (let $v1 @class(owned) (app (global Main.EchoInt.echo) (int 1)) (local $v1))
end
";

    /// The 0-based position of `needle` in `text`.
    fn at(text: &str, needle: &str) -> Pos {
        let byte = text.find(needle).expect("the needle is in the text");
        let before = &text[..byte];
        let line = before.matches('\n').count() as u32;
        let col = before
            .rsplit('\n')
            .next()
            .unwrap_or("")
            .chars()
            .map(|c| c.len_utf16() as u32)
            .sum();
        (line, col)
    }

    #[test]
    fn syntax_locates_units_annotations_refs_and_literals() {
        let syn = syntax(SAMPLE);
        let names: Vec<&str> = syn.units.iter().map(|unit| unit.name.as_str()).collect();
        assert_eq!(names, ["Main.EchoInt", "Main.EchoInt.echo", "Main.main"]);
        assert_eq!(syn.units[0].ty, "Main.Echo");
        assert_eq!(
            syn.units[0].name_range.start,
            at(SAMPLE, "Main.EchoInt ::"),
            "the name range starts at the name"
        );
        assert_eq!(syn.units[0].ty_range.start, at(SAMPLE, "Main.Echo\n"));
        // The header is `core 1`, `module Main`, `source ...`.
        let kinds: Vec<HeaderKind> = syn.header.iter().map(|line| line.kind).collect();
        assert_eq!(
            kinds,
            [HeaderKind::Version, HeaderKind::Module, HeaderKind::Source]
        );
        assert_eq!(syn.header[1].value, "Main");
        // Annotations carry their own name range, which is inside the range.
        let span = syn.units[0].annotation("span").expect("the echo unit has no span");
        assert_eq!(span.value, "tests/core/dictionaries.cinch:2:3");
        assert_eq!(span.name_range.start, at(SAMPLE, "span tests/core"));
        assert!(span.range.covers(&span.name_range));
        assert_eq!(syn.units[1].stage(), Some("checked"));
        assert_eq!(syn.units[0].statics.len(), 1);
        assert_eq!(syn.units[0].statics[0].slot, 0);
        assert_eq!(syn.units[0].statics[0].method, "Main.EchoInt.echo");
        // References in source order: the echo body's local, then main's
        // global and its `let` binder's use.
        let refs: Vec<(&str, &str)> = syn
            .refs
            .iter()
            .map(|r| (r.kind.head(), r.name.as_str()))
            .collect();
        assert_eq!(
            refs,
            [
                ("local", "v4343"),
                ("global", "Main.EchoInt.echo"),
                ("local", "$v1")
            ]
        );
        assert_eq!(syn.refs[1].range.start, at(SAMPLE, "Main.EchoInt.echo)"));
        // Literals, classes and the one projection.
        let literals: Vec<&str> = syn.literals.iter().map(|l| l.text.as_str()).collect();
        assert_eq!(literals, ["1", "1"]);
        assert_eq!(syn.projections.len(), 1);
        assert_eq!(syn.projections[0].slot, Some(0));
        assert_eq!(syn.projections[0].class.as_deref(), Some("Main.Echo"));
        let classes: Vec<&str> = syn.classes.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(classes, ["owned", "Main.Echo", "owned"]);
        // Every form is reachable from the unit body with its structural path.
        let paths: Vec<&str> = syn.forms().iter().map(|f| f.path.as_str()).collect();
        assert!(paths.contains(&"body"), "{paths:?}");
        assert!(paths.contains(&"body:callee"), "{paths:?}");
        assert!(paths.contains(&"body:body"), "{paths:?}");
    }

    #[test]
    fn the_sample_verifies() {
        let report = verify(SAMPLE).expect("the sample follows the grammar");
        assert_eq!(report.units, 3);
        assert_eq!(report.projections, 1);
    }

    #[test]
    fn verifier_findings_are_located_at_their_form() {
        let unbound = SAMPLE.replace("(local v4343)", "(local v9)");
        let errors = verify(&unbound).expect_err("v9 is not bound");
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert_eq!(errors[0].code, code::UNBOUND_LOCAL);
        // The finding is the `local` form itself, not the whole body.
        assert_eq!(errors[0].path, "body:arg 1");
        assert_eq!(errors[0].start, at(&unbound, "(local v9)"));
        assert_eq!(errors[0].start.0, at(&unbound, "body (app (dictproj").0);
        let order = SAMPLE.replace("  @instance(tparams 0, subdicts 0)\n", "");
        let order = order.replace("  @stage(checked)\n", "  @stage(checked)\n  @instance(tparams 0, subdicts 0)\n");
        let errors = verify(&order).expect_err("instance after stage is out of order");
        assert!(errors.iter().any(|error| error.code == code::ANNOTATION_ORDER), "{errors:?}");
        let spaced = errors
            .iter()
            .find(|error| error.code == code::ANNOTATION_ORDER)
            .expect("an order finding");
        assert_eq!(spaced.start.0, at(&order, "@instance(tparams").0);
        let unknown = SAMPLE.replace("@instance(tparams 0, subdicts 0)", "@inlined(yes)");
        let errors = verify(&unknown).expect_err("an unknown annotation is refused");
        assert_eq!(errors[0].code, code::UNKNOWN_ANNOTATION);
        assert_eq!(errors[0].start.0, at(&unknown, "@inlined").0);
        let duplicate = format!("{SAMPLE}unit Main.main :: Int\n  @stage(checked)\n  body (int 0)\nend\n");
        let errors = verify(&duplicate).expect_err("the name repeats");
        assert!(errors.iter().any(|error| error.code == code::DUPLICATE_UNIT), "{errors:?}");
        let missing = SAMPLE.replace(") 0 Main.Echo)", ") 7 Main.Echo)");
        let errors = verify(&missing).expect_err("slot 7 is not printed");
        assert_eq!(errors[0].code, code::SLOT_MISSING);
        assert_eq!(errors[0].start, at(&missing, "(dictproj"));
    }

    #[test]
    fn hover_describes_a_unit_a_reference_a_local_and_a_class() {
        let found = hover(SAMPLE, at(SAMPLE, "Main.main ::").0, at(SAMPLE, "Main.main ::").1)
            .expect("a unit name hovers");
        assert!(found.markdown.contains("type: `Int`"), "{}", found.markdown);
        assert!(found.markdown.contains("stage: `checked`"), "{}", found.markdown);
        assert!(
            found.markdown.contains("tests/core/dictionaries.cinch:8:1"),
            "{}",
            found.markdown
        );
        let (line, col) = at(SAMPLE, "Main.EchoInt.echo)");
        let found = hover(SAMPLE, line, col + 2).expect("a global reference hovers");
        assert!(
            found.markdown.contains("`Main.EchoInt.echo :: -`"),
            "{}",
            found.markdown
        );
        let (line, col) = at(SAMPLE, "(local $v1))");
        let found = hover(SAMPLE, line, col + 8).expect("a local reference hovers");
        assert!(found.markdown.contains("bound by `local binder`"), "{}", found.markdown);
        let (line, col) = at(SAMPLE, "(local v4343)");
        let found = hover(SAMPLE, line, col + 8).expect("a parameter hovers");
        assert!(
            found.markdown.contains("v4343 :: - @owned"),
            "{}",
            found.markdown
        );
        let (line, col) = at(SAMPLE, "0 Main.Echo)");
        let found = hover(SAMPLE, line, col + 3).expect("a class token hovers");
        assert!(found.markdown.contains("class `Main.Echo`"), "{}", found.markdown);
        assert!(
            found.markdown.contains("`Main.EchoInt`, slots 0 = `Main.EchoInt.echo`"),
            "{}",
            found.markdown
        );
        let imported = SAMPLE.replace("(global Main.EchoInt.echo)", "(global Other.thing)");
        let (line, col) = at(&imported, "Other.thing");
        let found = hover(&imported, line, col).expect("an imported reference hovers");
        assert!(
            found.markdown.contains("imported unit: no definition in this file"),
            "{}",
            found.markdown
        );
    }

    #[test]
    fn definition_jumps_only_to_a_unit_of_the_same_file() {
        let (line, col) = at(SAMPLE, "Main.EchoInt.echo)");
        let (sl, sc, el, ec) =
            definition(SAMPLE, line, col).expect("the referenced unit is in the file");
        assert_eq!((sl, sc), at(SAMPLE, "Main.EchoInt.echo ::"));
        assert_eq!((el, ec), (sl, sc + "Main.EchoInt.echo".len() as u32));
        let imported = SAMPLE.replace("(global Main.EchoInt.echo)", "(global Other.thing)");
        let (line, col) = at(&imported, "Other.thing");
        assert_eq!(definition(&imported, line, col), None, "an imported name has no jump");
        // On a local there is no definition to report.
        let (line, col) = at(SAMPLE, "(local $v1))");
        assert_eq!(definition(SAMPLE, line, col + 8), None);
    }

    #[test]
    fn the_syntax_view_reads_every_golden_file() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/core");
        let mut files = 0;
        for entry in std::fs::read_dir(&dir).expect("the golden directory") {
            let path = entry.expect("a directory entry").path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("core") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("read the golden file");
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            // The golden files are ASCII, so a UTF-16 column is a character
            // index and the assertion can index the line directly.
            let line_at = |pos: Pos| -> Vec<char> {
                text.lines().nth(pos.0 as usize).expect("the line exists").chars().collect()
            };
            let syn = syntax(&text);
            let listed = text.lines().filter(|line| line.starts_with("unit ")).count();
            assert_eq!(syn.units.len(), listed, "{name}");
            assert!(!syn.header.is_empty(), "{name}: no header");
            for unit in &syn.units {
                assert!(!unit.name.is_empty(), "{name}: an unnamed unit");
                assert!(
                    unit.body.is_some() || !unit.statics.is_empty(),
                    "{name}: {} has no body and no slots",
                    unit.name
                );
                let line = line_at(unit.name_range.start);
                let width = unit.name.chars().count();
                let start = unit.name_range.start.1 as usize;
                let printed: String = line[start..start + width].iter().collect();
                assert_eq!(printed, unit.name, "{name}");
                // Every annotation's name range sits on its own `@name`.
                for ann in &unit.anns {
                    let line = line_at(ann.name_range.start);
                    let start = ann.name_range.start.1 as usize;
                    let end = ann.name_range.end.1 as usize;
                    let printed: String = line[start..end].iter().collect();
                    assert_eq!(printed, ann.name, "{name}: {}", unit.name);
                }
            }
            for reference in &syn.refs {
                let line = line_at(reference.range.start);
                let start = reference.range.start.1 as usize;
                let end = reference.range.end.1 as usize;
                let printed: String = line[start..end].iter().collect();
                let printed = if reference.kind == RefKind::Foreign {
                    strip_quotes(&printed)
                } else {
                    unquote(&printed)
                };
                assert_eq!(printed, reference.name, "{name}");
            }
            for literal in &syn.literals {
                assert!(
                    literal.range.start <= literal.range.end,
                    "{name}: a literal range runs backwards"
                );
            }
            files += 1;
        }
        assert!(files >= 7, "the corpus is present: {files} files");
    }

}

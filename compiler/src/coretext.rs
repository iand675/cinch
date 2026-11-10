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

use crate::core::{CAlt, CPat, Core, FnDef, ForeignPlan, InstanceDef, LocalId, PrimTier, Program, TargetProfile};
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
    let signatures = Signatures::from_ifaces(ifaces);
    let digest = source_digest(ifaces, modules);
    let mut ctx = Context::new(stage, profile, &digest);
    ctx.signatures = &signatures;
    ctx.modules = Some(modules);
    ctx.rewrites = rewrites;
    ctx.opt_level = opt_level;
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
    params: BTreeMap<usize, String>,
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
        Printer { program, ctx, fns, params: BTreeMap::new() }
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
                    self.params = param_names(f);
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

    fn fn_unit(&mut self, f: &FnDef, out: &mut String) {
        self.params = param_names(f);
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

    fn params_line(&self, f: &FnDef) -> String {
        let types = self.param_types(f);
        let mut parts = Vec::new();
        for (index, name) in f.params.iter().enumerate() {
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

    fn expr(&self, e: &Core) -> Node {
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
                let own: Vec<String> = match self.fns.get(&fn_id.0) {
                    Some(f) => f.params.iter().skip(env.len()).cloned().collect(),
                    None => Vec::new(),
                };
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
            Core::Let(id, class, value, body) => Node::list("let")
                .child(Node::atom(self.local(*id)))
                .mark(format!("@class({})", class_text(*class)))
                .child(self.expr(value))
                .child(self.expr(body)),
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

    fn pattern(&self, pat: &CPat) -> Node {
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

    fn local(&self, id: LocalId) -> String {
        match self.params.get(&id.0) {
            Some(name) => name.clone(),
            None => format!("$v{}", id.0),
        }
    }
}

fn param_names(f: &FnDef) -> BTreeMap<usize, String> {
    f.params.iter().enumerate().map(|(index, name)| (index, name.clone())).collect()
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
    /// `(let $v1 @class(owned) value body)`.
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
    fn annotation(&self, name: &str) -> Option<&str> {
        self.anns.iter().find(|(n, _)| n == name).map(|(_, v)| v.as_str())
    }

    fn stage(&self) -> Option<&str> {
        self.annotation("stage")
    }
}

#[derive(Clone, Debug, Default)]
struct Parsed {
    modules: Vec<String>,
    globals: Vec<String>,
    target: Option<(String, u32)>,
    source: Option<String>,
    units: Vec<ParsedUnit>,
}

/// Read a Core file's structure. Every violation the reader finds becomes an
/// [`Error`]; `None` means the file was not readable at all.
fn parse(text: &str, errors: &mut Vec<Error>) -> Option<Parsed> {
    let lines: Vec<&str> = text.lines().collect();
    let mut file = Parsed::default();
    if lines.is_empty() {
        errors.push(Error::header("empty file"));
        return None;
    }
    if lines[0] != "core 1" {
        errors.push(Error::header(format!("the first line is `{}`, not `core 1`", lines[0])));
        return None;
    }
    let mut i = 1;
    // Header stage: 0 modules, 1 target, 2 source, 3 globals. A line that
    // moves the stage backwards is a grammar violation.
    let mut stage = 0u8;
    while i < lines.len() {
        let line = lines[i];
        if let Some(rest) = line.strip_prefix("module ") {
            if stage > 0 {
                errors.push(Error::header(format!("`module` after the header's target line: `{line}`")));
            }
            stage = 0;
            if rest.trim().is_empty() || rest.contains(' ') {
                errors.push(Error::header(format!("module name `{rest}` is not one name")));
            }
            if file.modules.iter().any(|m| m == rest) {
                errors.push(Error::header(format!("module `{rest}` is declared twice")));
            }
            if file.modules.last().is_some_and(|last| last.as_str() > rest) {
                errors.push(Error::header(format!("module `{rest}` is out of order")));
            }
            file.modules.push(rest.to_string());
        } else if let Some(rest) = line.strip_prefix("target ") {
            if stage > 1 {
                errors.push(Error::header(format!("`target` after `source`: `{line}`")));
            }
            stage = 1;
            if file.target.is_some() {
                errors.push(Error::header("`target` is declared twice"));
            }
            match rest.rsplit_once(" opt-level ") {
                Some((triple, level)) => {
                    if triple.trim().is_empty() || triple.contains(' ') {
                        errors.push(Error::header(format!("target triple `{triple}` is not one word")));
                    }
                    match level.parse::<u32>() {
                        Ok(level) if level <= 3 => file.target = Some((triple.to_string(), level)),
                        _ => errors.push(Error::header(format!("`{level}` is not an opt-level of 0 to 3"))),
                    }
                }
                None => errors.push(Error::header("`target` needs `<triple> opt-level <0-3>`")),
            }
        } else if let Some(rest) = line.strip_prefix("source ") {
            if stage > 2 {
                errors.push(Error::header(format!("`source` after `global`: `{line}`")));
            }
            stage = 2;
            if file.source.is_some() {
                errors.push(Error::header("`source` is declared twice"));
            }
            match rest.strip_prefix("sha256:") {
                Some(hex) if hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) => {
                    file.source = Some(hex.to_string());
                }
                _ => errors.push(Error::header("`source` needs `sha256:` and 64 hex digits")),
            }
        } else if let Some(rest) = line.strip_prefix("global ") {
            if file.source.is_none() {
                errors.push(Error::header("`global` lines come after `source`"));
            }
            stage = 3;
            if rest.trim().is_empty() || rest.contains(' ') {
                errors.push(Error::header(format!("global name `{rest}` is not one name")));
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
        if line.is_empty() {
            errors.push(Error::header(format!("blank line before the record at `{}`", line_near(&lines, i))));
            i += 1;
            continue;
        }
        let Some(rest) = line.strip_prefix("unit ") else {
            errors.push(Error::header(format!("expected `unit`, found `{line}`")));
            i += 1;
            continue;
        };
        match rest.find(" :: ") {
            Some(pos) => {
                let name = rest[..pos].trim().to_string();
                let ty = rest[pos + 4..].trim().to_string();
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
                    if inner == "end" {
                        terminated = true;
                        i += 1;
                        break;
                    }
                    if let Some(rest) = inner.strip_prefix("  @params ") {
                        unit.params = parse_params(rest, &unit.name, errors);
                        unit.anns.push(("params".to_string(), rest.to_string()));
                    } else if let Some(rest) = inner.strip_prefix("  @") {
                        let (name, value) = split_annotation(rest);
                        unit.anns.push((name, value));
                    } else if let Some(rest) = inner.strip_prefix("  body ") {
                        if unit.body.is_some() || unit.has_static {
                            errors.push(Error::new(&unit.name, "body", "a second body"));
                        }
                        let (text, used) = body_text(&lines, i, rest);
                        match parse_node(&text) {
                            Ok(node) => unit.body = Some(node),
                            Err(message) => errors.push(Error::new(&unit.name, "body", message)),
                        }
                        i += used;
                        continue;
                    } else if let Some(rest) = inner.strip_prefix("  static ") {
                        if unit.body.is_some() || unit.has_static {
                            errors.push(Error::new(&unit.name, "static", "a second body"));
                        }
                        unit.has_static = true;
                        unit.statics = parse_statics(rest, &unit.name, errors);
                    } else if !inner.is_empty() {
                        errors.push(Error::new(&unit.name, "record", format!("unexpected line `{inner}`")));
                    }
                    i += 1;
                }
                if !terminated {
                    errors.push(Error::new(&unit.name, "record", "the unit has no `end`"));
                }
                if unit.body.is_none() && !unit.has_static {
                    errors.push(Error::new(&unit.name, "record", "the unit has no `body` and no `static` slots"));
                }
                file.units.push(unit);
            }
            None => {
                errors.push(Error::header(format!("`unit` line has no ` :: `: `{line}`")));
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
    "specialised",
    "rewrite",
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

/// The printer's sort key for an annotation: the same table the verifier
/// checks, so one list orders and validates.
fn rank(name: &str) -> u8 {
    annotation_rank(name).map(|rank| rank as u8).unwrap_or(u8::MAX)
}

/// The `@` marks a node may carry: its ownership class, its instantiation
/// vector, and an alternative's weight.
const NODE_MARKS: &[&str] = &["class", "classes", "weight"];

// -------------------------------------------------------------- verification

/// One verifier finding, located by unit and by structural path rather than
/// by line (`Main.map:body:alt 2`).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    pub unit: String,
    pub path: String,
    pub message: String,
}

impl Error {
    fn new(unit: &str, path: &str, message: impl Into<String>) -> Error {
        Error { unit: unit.to_string(), path: path.to_string(), message: message.into() }
    }

    fn header(message: impl Into<String>) -> Error {
        Error { unit: String::new(), path: "header".to_string(), message: message.into() }
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
            errors.push(Error::new(&unit.name, "record", "the unit name is not unique"));
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
                            errors.push(Error::new(&unit.name, "@", format!("annotation `@{name}` is out of order")));
                        }
                    }
                    last = Some(rank);
                }
                None => errors.push(Error::new(&unit.name, "@", format!("unknown annotation `@{name}`"))),
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
                errors.push(Error::new(
                    &unit.name,
                    &format!("static:{slot}"),
                    format!("slot {slot} names the undefined unit `{method}`"),
                ));
            }
        }
        if let Some(body) = &unit.body {
            let mut scope: Vec<String> = unit.params.clone();
            let mut checker = Checker { file, units: &units, declared: &declared, report: &mut report };
            checker.expr(body, &mut scope, &unit.name, "body", errors);
        }
    }
    report
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
                        errors.push(Error::new(unit, path, format!("`{name}` is out of scope")));
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
                let binder = atom_at(tail, 0, unit, path, errors).map(str::to_string);
                for (index, child) in tail.iter().enumerate().skip(1) {
                    match (binder.as_deref(), index) {
                        (Some(_), 1) => self.expr(child, scope, unit, &child_path(head, index, path), errors),
                        (Some(name), 2) => {
                            scope.push(name.to_string());
                            self.expr(child, scope, unit, &child_path(head, index, path), errors);
                            scope.pop();
                        }
                        _ => self.expr(child, scope, unit, &child_path(head, index, path), errors),
                    }
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
                                    errors.push(Error::new(unit, &child_path, format!("`{name}` is out of scope")));
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
                errors.push(Error::new(unit, path, format!("`{name}` is out of scope")));
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
                            errors.push(Error::new(unit, path, format!("`{name}` has no static slots")));
                        } else if !instance.statics.iter().any(|(printed, _)| *printed == slot) {
                            errors.push(Error::new(
                                unit,
                                path,
                                format!("slot {slot} is not in `{name}`'s static slot table"),
                            ));
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
                    None => errors.push(Error::new(unit, path, format!("unknown dictionary `{name}`"))),
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
        ("let", 2) => "body".to_string(),
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

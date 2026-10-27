//! The shared analysis layer (P21 + P22): one incremental checker front
//! end for the REPL and the language server.
//!
//! Both front ends drive [`Analysis`]: it holds the workspace's
//! documents, keeps the per-module `.sliface` chain (the "checked
//! against" chain of `interface.rs`), rechecks a changed file and its
//! dependents only, and answers the editor questions (hover, definition,
//! completion, semantic tokens) from the checked and parsed modules.
//!
//! Source spans: the checker reports spans without file names, so every
//! file's text is parsed behind a fixed line band (`span.line` lands in
//! exactly one file's band) and every span is translated back to
//! workspace coordinates on the way out. All file names in this layer
//! are workspace-relative; nothing here ever renders an absolute path
//! (the standing manifest rule).

use crate::ast::{self, Decl, RefKind};
use crate::core::Value;
use crate::interface::{self, Iface};
use crate::lexer::{self, Span, Tok};
use crate::parser::{self, FixityTable, ParsedModules};
use crate::types::{Scalar, Ty};
use crate::{check, driver, Error};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// The module the REPL keeps its session in (P21).
pub const SESSION_MODULE: &str = "ReplSession";
/// The session's pseudo-file name: never a disk path.
pub const SESSION_FILE: &str = "<session>";

/// How bad a diagnostic is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Information,
}

/// One diagnostic in workspace coordinates: `file` is workspace
/// relative, `line` and `col` are 1-based.
#[derive(Clone, Debug)]
pub struct Diag {
    pub severity: Severity,
    pub file: String,
    pub line: u32,
    pub col: u32,
    pub message: String,
}

/// What kind of symbol a name defines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymKind {
    Value,
    Type,
    Ctor,
    Field,
    Class,
    Method,
    Law,
    Instance,
    Effect,
    EffectOp,
}

impl SymKind {
    /// A short label for hover and completion detail.
    pub fn label(self) -> &'static str {
        match self {
            SymKind::Value => "value",
            SymKind::Type => "type",
            SymKind::Ctor => "constructor",
            SymKind::Field => "field",
            SymKind::Class => "class",
            SymKind::Method => "method",
            SymKind::Law => "law",
            SymKind::Instance => "instance",
            SymKind::Effect => "effect",
            SymKind::EffectOp => "effect operation",
        }
    }
}

/// One completion candidate.
#[derive(Clone, Debug)]
pub struct Completion {
    pub label: String,
    pub kind: SymKind,
    /// The rendered scheme or type, when one is known.
    pub detail: String,
}

/// One semantic token in workspace coordinates (1-based line and column,
/// `len` in characters).
#[derive(Clone, Debug)]
pub struct SemToken {
    pub line: u32,
    pub col: u32,
    pub len: u32,
    /// The legend name (`"keyword"`, `"function"`, ...).
    pub kind: &'static str,
}

/// What one [`Analysis::update`] recomputed: the incremental recheck
/// report (P22).
#[derive(Clone, Debug, Default)]
pub struct UpdateReport {
    /// User modules re-elaborated: the changed modules, their dependents
    /// (the `.sliface` chain), and the unchanged dependencies the
    /// whole-program checker needs to resolve their names.
    pub rechecked: Vec<String>,
    /// User modules whose cached `.sliface` and diagnostics were reused;
    /// their sources were not rechecked in this update.
    pub kept: Vec<String>,
    /// The diagnostics this update produced (whole workspace).
    pub diagnostics: Vec<Diag>,
    /// Whether the merged `.sliface` "checked against" chain verifies.
    pub chain_ok: bool,
}

/// The REPL probe's outcome (P21): the inferred scheme and, when the D92
/// evaluator runs the elaborated code, its value.
#[derive(Clone, Debug, Default)]
pub struct ProbeOutcome {
    /// The inferred scheme (`render_scheme` form).
    pub scheme: Option<String>,
    /// The evaluated value, rendered.
    pub value: Option<String>,
    /// Why the value is absent (the evaluator cannot run this code).
    pub eval_error: Option<String>,
    /// The elaboration failure, with its span in the probe source.
    pub error: Option<Error>,
}

/// A probe expression for [`Analysis::update_probe`].
pub enum ProbeInput<'a> {
    /// The expression's source (the REPL's `:type` and evaluation).
    Src(&'a str),
    /// An already-parsed expression (definition-signature synthesis).
    Term(&'a ast::Term),
}

/// A definition site in workspace coordinates.
#[derive(Clone, Debug)]
struct Def {
    name: String,
    kind: SymKind,
    line: u32,
    col: u32,
}

/// A reference site in workspace coordinates.
#[derive(Clone, Debug)]
struct Ref {
    kind: RefKind,
    quals: Vec<String>,
    name: String,
    line: u32,
    col: u32,
}

/// One module's editor-level index.
#[derive(Clone, Debug, Default)]
struct ModuleState {
    file: String,
    imports: Vec<String>,
    defs: Vec<Def>,
    refs: Vec<Ref>,
    /// Rendered signature text per declared name, the hover fallback
    /// when no checked interface exists.
    sig_text: HashMap<String, String>,
    fixities: FixityTable,
}

/// One workspace file.
struct FileState {
    text: String,
    /// The modules it declares, in declaration order.
    modules: Vec<String>,
    /// `scan_file`'s view, for the import graph.
    imports: Vec<(String, Vec<String>)>,
    /// The parse of this file, when the last parse succeeded.
    parsed: Option<ParsedModules>,
}

/// The shared incremental analysis (see the module docs).
pub struct Analysis {
    /// The workspace root: used to relativize disk paths and as the
    /// staging work directory. Never rendered.
    root: std::path::PathBuf,
    files: BTreeMap<String, FileState>,
    modules: BTreeMap<String, ModuleState>,
    /// The `.sliface` chain: one checked interface per module.
    ifaces: BTreeMap<String, Iface>,
    diags: BTreeMap<String, Vec<Diag>>,
    /// File to line band base; spans of the file carry `base - 1` extra
    /// leading lines.
    band: BTreeMap<String, u64>,
    next_band: u64,
    /// Files whose text changed since the last update.
    pending: BTreeSet<String>,
    prelude: Option<driver::PreludeSources>,
    /// The embedded prelude family's editor indices (built on first use).
    prelude_index: Option<BTreeMap<String, ModuleState>>,
}

/// The band width per file: a file larger than this cannot be analyzed.
const BAND_WIDTH: u64 = 300_000;
/// The first band base: below it, a span belongs to the embedded prelude.
const FIRST_BAND: u64 = 1_000_000;

impl Analysis {
    /// An analysis over `root` (a directory the workspace's paths are
    /// relative to).
    pub fn new(root: impl AsRef<std::path::Path>) -> Analysis {
        Analysis {
            root: root.as_ref().to_path_buf(),
            files: BTreeMap::new(),
            modules: BTreeMap::new(),
            ifaces: BTreeMap::new(),
            diags: BTreeMap::new(),
            band: BTreeMap::new(),
            next_band: FIRST_BAND,
            pending: BTreeSet::new(),
            prelude: None,
            prelude_index: None,
        }
    }

    /// The workspace-relative key of a disk path.
    pub fn rel_path(&self, path: &std::path::Path) -> String {
        let abs = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        };
        match abs.strip_prefix(&self.root) {
            Ok(rel) => rel.display().to_string(),
            Err(_) => path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string()),
        }
    }

    /// Open or change a document (in-memory text wins over disk).
    pub fn set_document(&mut self, file: &str, text: &str) {
        let key = file.to_string();
        let changed = self.files.get(&key).map(|f| f.text != text).unwrap_or(true);
        self.files.entry(key.clone()).or_insert_with(|| FileState {
            text: String::new(),
            modules: Vec::new(),
            imports: Vec::new(),
            parsed: None,
        });
        let f = self.files.get_mut(&key).expect("just inserted");
        f.text = text.to_string();
        if changed {
            self.pending.insert(key);
        }
    }

    /// Close a document: its text falls back to disk on the next update.
    pub fn remove_document(&mut self, file: &str) {
        if self.files.remove(file).is_some() {
            self.pending.insert(file.to_string());
        }
    }

    /// A document's current text, when the analysis holds one.
    pub fn document(&self, file: &str) -> Option<&str> {
        self.files.get(file).map(|f| f.text.as_str())
    }

    /// Every file the analysis knows.
    pub fn files(&self) -> Vec<String> {
        self.files.keys().cloned().collect()
    }

    /// The user modules (never the embedded prelude family).
    pub fn modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }

    /// Load a project from disk (the REPL's `:load`): the entry file and
    /// every module it imports, transitively, become documents. Returns
    /// the loaded module names in dependency order. The workspace root
    /// becomes the entry's directory, so every path stays relative.
    pub fn load_project(&mut self, entry: &str) -> Result<Vec<String>, String> {
        let entry_path = self.root.join(entry);
        let dir = entry_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| self.root.clone());
        self.root = dir.clone();
        let mut queue = vec![entry_path.clone()];
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut loaded: Vec<String> = Vec::new();
        let prelude_names = driver::prelude_module_names();
        while let Some(path) = queue.pop() {
            let key = self.rel_path(&path);
            if !seen.insert(key.clone()) {
                continue;
            }
            let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", key))?;
            self.set_document(&key, &text);
            let scanned = self.scan(&key, &text).map_err(|e| e.to_string())?;
            for (m, imps) in &scanned {
                if !loaded.contains(m) {
                    loaded.push(m.clone());
                }
                for imp in imps {
                    if self.module_file(imp).is_some() {
                        continue;
                    }
                    let direct = dir.join(format!("{imp}.lang"));
                    if direct.exists() {
                        queue.push(direct);
                        continue;
                    }
                    if prelude_names.iter().any(|n| n == imp) {
                        continue;
                    }
                    // A unique module declaration beside the entry.
                    let mut candidates = Vec::new();
                    if let Ok(entries) = std::fs::read_dir(&dir) {
                        for e in entries.flatten() {
                            let p = e.path();
                            if p.extension().map(|x| x == "lang").unwrap_or(false) {
                                if let Ok(t) = std::fs::read_to_string(&p) {
                                    if let Ok(ms) = self.scan(&self.rel_path(&p), &t) {
                                        if ms.iter().any(|(m, _)| m == imp) {
                                            candidates.push(p.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    candidates.sort();
                    match candidates.len() {
                        1 => queue.push(candidates.into_iter().next().expect("one")),
                        0 => {
                            return Err(format!(
                                "module {imp} is imported, but no source file beside the entry declares it"
                            ))
                        }
                        _ => {
                            return Err(format!(
                                "module {imp} is imported, but several source files declare it"
                            ))
                        }
                    }
                }
            }
        }
        self.pending.extend(self.files.keys().cloned());
        Ok(loaded)
    }

    /// Re-read every document from disk (the REPL's `:reload`) and mark
    /// it for recheck: reload is an explicit recheck request.
    pub fn reload_from_disk(&mut self) -> Result<(), String> {
        let keys: Vec<String> = self.files.keys().cloned().collect();
        for key in keys {
            if key == SESSION_FILE {
                continue;
            }
            let path = self.root.join(&key);
            if let Ok(text) = std::fs::read_to_string(&path) {
                self.set_document(&key, &text);
                self.pending.insert(key);
            }
        }
        Ok(())
    }

    // ------------------------------------------------------- coordinates

    /// The file's band base, assigned once so spans stay stable.
    fn band_of(&mut self, file: &str) -> u64 {
        if let Some(&b) = self.band.get(file) {
            return b;
        }
        let b = self.next_band;
        self.next_band += BAND_WIDTH;
        self.band.insert(file.to_string(), b);
        b
    }

    /// The file's parse text: its source with the band's leading lines.
    fn banded_text(&mut self, file: &str, text: &str) -> String {
        let base = self.band_of(file);
        if text.lines().count() as u64 >= BAND_WIDTH {
            // Impossible for real sources; fail loudly rather than
            // attribute a span to the wrong file.
            return String::new();
        }
        let mut out = "\n".repeat((base - 1) as usize);
        out.push_str(text);
        out
    }

    /// A banded span back to workspace coordinates, when it lands in a
    /// known band. Spans below the first band are prelude spans (they
    /// carry their own coordinates).
    fn unband(&self, span: Span) -> (String, u32, u32) {
        for (file, &base) in &self.band {
            if span.line as u64 >= base && (span.line as u64) < base + BAND_WIDTH {
                return (file.clone(), span.line - (base as u32 - 1), span.col);
            }
        }
        (String::new(), span.line, span.col)
    }

    // ------------------------------------------------------- scanning

    /// A file's module headers and imports (`(module, imports)`).
    fn scan(&mut self, file: &str, text: &str) -> Result<Vec<(String, Vec<String>)>, Error> {
        let banded = self.banded_text(file, text);
        let scanned = driver::scan_file(&banded, file)?;
        Ok(scanned
            .modules
            .into_iter()
            .map(|(m, imps)| (m, imps.into_iter().map(|(n, _)| n).collect()))
            .collect())
    }

    /// The file that declares `module`, when one is loaded.
    fn module_file(&self, module: &str) -> Option<String> {
        self.modules.get(module).map(|m| m.file.clone())
    }

    // ------------------------------------------------------- update

    /// Recompute after document changes: recheck the changed files and
    /// their dependents via the `.sliface` chain, keep everything else.
    pub fn update(&mut self) -> UpdateReport {
        self.update_impl(None).0
    }

    /// [`update`] plus a probe expression evaluated in `probe_module`'s
    /// scope through the D92 evaluator (P21).
    pub fn update_probe(
        &mut self,
        probe_module: &str,
        probe_src: &str,
    ) -> (UpdateReport, ProbeOutcome) {
        self.update_impl(Some((probe_module, ProbeInput::Src(probe_src))))
    }

    /// [`update_probe`] with an already-parsed expression (the REPL's
    /// definition-signature synthesis).
    pub fn update_probe_term(
        &mut self,
        probe_module: &str,
        term: &ast::Term,
    ) -> (UpdateReport, ProbeOutcome) {
        self.update_impl(Some((probe_module, ProbeInput::Term(term))))
    }

    fn update_impl(
        &mut self,
        probe: Option<(&str, ProbeInput<'_>)>,
    ) -> (UpdateReport, ProbeOutcome) {
        let mut outcome = ProbeOutcome::default();
        // 1. Rescan the changed files: their module names and imports
        //    feed the graph the dirty set is computed over.
        let changed: Vec<String> = self.pending.iter().cloned().collect();
        self.pending.clear();
        for file in &changed {
            let Some(text) = self.files.get(file).map(|f| f.text.clone()) else {
                // Closed and gone from disk: drop its modules.
                let gone: Vec<String> = self
                    .modules
                    .iter()
                    .filter(|(_, m)| &m.file == file)
                    .map(|(n, _)| n.clone())
                    .collect();
                for m in gone {
                    self.modules.remove(&m);
                    self.ifaces.remove(&m);
                    self.diags.remove(&m);
                }
                self.files.remove(file);
                continue;
            };
            let scanned = match self.scan(file, &text) {
                Ok(s) => s,
                Err(e) => {
                    self.diags
                        .insert(file.clone(), vec![self.error_diag(file, &e)]);
                    self.scan_state(file, &text, Vec::new());
                    continue;
                }
            };
            self.scan_state(file, &text, scanned);
        }
        // 2. The dirty set: the changed modules and their dependents,
        //    walking the import edges the `.sliface` chain records.
        let mut dirty: BTreeSet<String> = BTreeSet::new();
        for file in &changed {
            if let Some(f) = self.files.get(file) {
                for (m, _) in &f.imports {
                    dirty.insert(m.clone());
                }
            }
        }
        if let Some((m, _)) = probe {
            dirty.insert(m.to_string());
        }
        let mut work: Vec<String> = dirty.iter().cloned().collect();
        while let Some(m) = work.pop() {
            for (name, st) in &self.modules {
                if st.imports.iter().any(|i| *i == m) && dirty.insert(name.clone()) {
                    work.push(name.clone());
                }
            }
        }
        // 3. The check slice: the dirty modules plus the dependencies
        //    the whole-program checker needs to resolve their names.
        let mut slice: BTreeSet<String> = dirty.clone();
        let mut work: Vec<String> = dirty.iter().cloned().collect();
        while let Some(m) = work.pop() {
            if let Some(st) = self.modules.get(&m) {
                for i in &st.imports {
                    if self.modules.contains_key(i) && slice.insert(i.clone()) {
                        work.push(i.clone());
                    }
                }
            }
        }
        let mut report = UpdateReport {
            rechecked: slice.iter().cloned().collect(),
            kept: self
                .modules
                .keys()
                .filter(|m| !slice.contains(*m) && !is_prelude_module(m))
                .cloned()
                .collect(),
            ..UpdateReport::default()
        };
        if slice.is_empty() {
            // Nothing changed and no probe: the checked state stands.
            report.chain_ok = self.chain_ok();
            report.diagnostics = self.diagnostics();
            return (report, outcome);
        }
        // 4. Parse the slice's files in dependency order, threading the
        //    fixity tables (imports first).
        let mut parse_failed = false;
        let mut parsed: HashMap<String, ast::Module> = HashMap::new();
        let mut tables: HashMap<String, FixityTable> = self
            .prelude_fixities()
            .into_iter()
            .collect();
        for m in topo_slice(self, &slice) {
            let file = match self.module_file(&m) {
                Some(f) => f,
                None => continue,
            };
            let Some(fstate) = self.files.get(&file) else {
                continue;
            };
            if let Some(pm) = &fstate.parsed {
                // Reuse only when this file parsed cleanly before and no
                // fresh parse is pending for it.
                if !changed.contains(&file) {
                    for (name, fix) in &pm.fixities {
                        tables.insert(name.clone(), fix.clone());
                    }
                    for modu in &pm.modules {
                        parsed.insert(modu.name.text.clone(), modu.clone());
                    }
                    continue;
                }
            }
            let text = fstate.text.clone();
            let banded = self.banded_text(&file, &text);
            match parse_file(&banded, &file, &self.prelude_base(), &tables) {
                Ok(pm) => {
                    for (name, fix) in &pm.fixities {
                        tables.insert(name.clone(), fix.clone());
                    }
                    for modu in &pm.modules {
                        parsed.insert(modu.name.text.clone(), modu.clone());
                    }
                    let prev = self.files.get_mut(&file).map(|f| f.parsed.replace(pm));
                    drop(prev);
                    self.diags.remove(&file);
                }
                Err(e) => {
                    parse_failed = true;
                    self.diags
                        .insert(file.clone(), vec![self.error_diag(&file, &e)]);
                    if let Some(f) = self.files.get_mut(&file) {
                        f.parsed = None;
                    }
                }
            }
        }
        // 5. Refresh the parsed-module indices of the slice.
        for m in &slice {
            if let Some(modu) = parsed.get(m) {
                let file = self.module_file(m).unwrap_or_default();
                let base = self.band_of(&file);
                let mut st = index_module(&file, base, modu);
                st.fixities = tables.get(m).cloned().unwrap_or_default();
                self.modules.insert(m.clone(), st);
            }
        }
        if parse_failed {
            for m in &slice {
                self.ifaces.remove(m);
            }
            report.chain_ok = self.chain_ok();
            report.diagnostics = self.diagnostics();
            return (report, outcome);
        }
        // 6. Check the slice (the prelude family is the implicit base
        //    layer) with the probe in scope, through the one shared
        //    checker entry the LSP's rechecks use.
        let modules: Vec<ast::Module> = topo_slice(self, &slice)
            .into_iter()
            .filter_map(|m| parsed.get(&m).cloned())
            .collect();
        let mut hashes = HashMap::new();
        let mut files = HashMap::new();
        for m in &modules {
            let name = m.name.text.clone();
            let file = self.module_file(&name).unwrap_or_default();
            let text = self.files.get(&file).map(|f| f.text.clone()).unwrap_or_default();
            hashes.insert(name.clone(), interface::content_hash(&text));
            files.insert(name.clone(), file);
        }
        let scx = check::StagingCx {
            fixities: tables.clone(),
            files,
            prior_inputs: Vec::new(),
            mode: check::StageMode::default(),
            workdir: self.root.clone(),
            entry_modules: modules.iter().map(|m| m.name.text.clone()).collect(),
            schema_tool: None,
        };
        let adapters = driver::wit_for_source(&self.root.join("_.lang")).unwrap_or_default();
        // The probe expression, parsed in its module's fixity context.
        let had_probe = probe.is_some();
        let probe_term = probe.and_then(|(module, input)| {
            let fix = self
                .modules
                .get(module)
                .map(|m| m.fixities.clone())
                .unwrap_or_else(|| self.prelude_base());
            match input {
                ProbeInput::Src(src) => match parse_term(src, &fix) {
                    Ok(t) => Some((module.to_string(), t)),
                    Err(e) => {
                        outcome.error = Some(e);
                        None
                    }
                },
                ProbeInput::Term(t) => Some((module.to_string(), t.clone())),
            }
        });
        let probe_arg = probe_term
            .as_ref()
            .map(|(module, term)| check::ProbeTerm {
                module: module.clone(),
                term,
            });
        let prelude = self.ensure_prelude();
        let res = check::check_program_multi_full_probe(
            &prelude.modules,
            &modules,
            &hashes,
            &scx,
            &adapters,
            &crate::core::TargetProfile::native(),
            probe_arg.as_ref(),
        );
        match res {
            Ok((_, ifaces, _stage, diags, probe_out)) => {
                for iface in ifaces {
                    self.ifaces.insert(iface.module.clone(), iface);
                }
                for d in diags {
                    let (file, line, col) = self.unband(d.span);
                    let file = self.attr_file(file, &modules);
                    let msg = format!("{}: {}", d.code, d.message);
                    self.diags.entry(file.clone()).or_default().push(Diag {
                        severity: if d.informational {
                            Severity::Information
                        } else {
                            Severity::Warning
                        },
                        file,
                        line,
                        col,
                        message: msg,
                    });
                }
                for m in &slice {
                    let file = self.module_file(m).unwrap_or_default();
                    self.diags.entry(file).or_default();
                }
                if let Some(p) = probe_out {
                    if let Some((_, text)) = &p.scheme {
                        outcome.scheme = Some(text.clone());
                    }
                    if let Some(v) = &p.value {
                        let ty = p
                            .scheme
                            .as_ref()
                            .map(|(s, _)| s.ty.clone())
                            .unwrap_or(Ty::Unit);
                        outcome.value = Some(render_value(&ty, v));
                    }
                    outcome.eval_error = p.eval_error.clone();
                    outcome.error = p.error.clone();
                }
            }
            Err(e) => {
                for m in &slice {
                    self.ifaces.remove(m);
                }
                let (file, line, col) = match e.span {
                    Some(s) => self.unband(s),
                    None => (String::new(), 1, 1),
                };
                let file = self.attr_file(file, &modules);
                for m in &slice {
                    let f = self.module_file(m).unwrap_or_default();
                    self.diags.remove(&f);
                }
                self.diags.insert(
                    file.clone(),
                    vec![Diag {
                        severity: Severity::Error,
                        file,
                        line,
                        col,
                        message: e.message.clone(),
                    }],
                );
                if had_probe {
                    outcome.error = Some(e);
                }
            }
        }
        report.chain_ok = self.chain_ok();
        report.diagnostics = self.diagnostics();
        (report, outcome)
    }

    /// The file to attribute a spanless file to: the slice's entry, the
    /// module the check reached last.
    fn attr_file(&self, file: String, modules: &[ast::Module]) -> String {
        if !file.is_empty() {
            return file;
        }
        modules
            .last()
            .and_then(|m| self.module_file(&m.name.text))
            .or_else(|| self.files.keys().next().cloned())
            .unwrap_or_default()
    }

    /// Update the scan-level state of one file.
    fn scan_state(&mut self, file: &str, text: &str, scanned: Vec<(String, Vec<String>)>) {
        let old: Vec<String> = self
            .modules
            .iter()
            .filter(|(_, m)| m.file == file)
            .map(|(n, _)| n.clone())
            .collect();
        for m in old {
            self.modules.remove(&m);
            self.ifaces.remove(&m);
        }
        let mut mods = Vec::new();
        for (name, imps) in scanned {
            mods.push(name.clone());
            let st = self.modules.entry(name.clone()).or_default();
            st.file = file.to_string();
            st.imports = imps;
        }
        if let Some(f) = self.files.get_mut(file) {
            f.modules = mods;
            f.imports = self
                .modules
                .iter()
                .filter(|(_, m)| m.file == file)
                .map(|(n, m)| (n.clone(), m.imports.clone()))
                .collect();
            let _ = text;
        }
    }

    /// Does the merged `.sliface` "checked against" chain verify?
    fn chain_ok(&self) -> bool {
        let ifaces: Vec<&Iface> = self.ifaces.values().collect();
        let mut by_module: BTreeMap<&str, &Iface> = BTreeMap::new();
        for f in &ifaces {
            by_module.entry(f.module.as_str()).or_insert(f);
        }
        for f in &ifaces {
            for imp in &f.imports {
                match by_module.get(imp.module.as_str()) {
                    None => return false,
                    Some(dep) => {
                        if dep.iface_hash() != imp.iface_hash {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    /// Every diagnostic, sorted by file and position.
    pub fn diagnostics(&self) -> Vec<Diag> {
        let mut out: Vec<Diag> = Vec::new();
        for ds in self.diags.values() {
            out.extend(ds.iter().cloned());
        }
        out.sort_by(|a, b| {
            (&a.file, a.line, a.col).cmp(&(&b.file, b.line, b.col))
        });
        out
    }

    /// The diagnostics of one file.
    pub fn file_diagnostics(&self, file: &str) -> Vec<Diag> {
        self.diags.get(file).cloned().unwrap_or_default()
    }

    fn error_diag(&self, file: &str, e: &Error) -> Diag {
        let (f, line, col) = match e.span {
            Some(s) => self.unband(s),
            None => (String::new(), 1, 1),
        };
        let f = if f.is_empty() { file.to_string() } else { f };
        Diag {
            severity: Severity::Error,
            file: f,
            line,
            col,
            message: e.message.clone(),
        }
    }

    // ------------------------------------------------------- prelude

    fn prelude_fixities(&mut self) -> Vec<(String, FixityTable)> {
        let p = self.ensure_prelude();
        p.fixities.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    fn prelude_base(&mut self) -> FixityTable {
        self.ensure_prelude().base.clone()
    }

    fn ensure_prelude(&mut self) -> &driver::PreludeSources {
        if self.prelude.is_none() {
            let adapters = driver::wit_for_source(&self.root.join("_.lang")).unwrap_or_default();
            let p = driver::parse_prelude(&adapters).expect("the embedded prelude parses");
            let mut index: BTreeMap<String, ModuleState> = BTreeMap::new();
            for m in &p.modules {
                let file = prelude_file_of(&m.name.text);
                let st = index_module(&file, 1, m);
                index.insert(m.name.text.clone(), st);
            }
            self.prelude = Some(p);
            self.prelude_index = Some(index);
        }
        self.prelude.as_ref().expect("just parsed")
    }

    // ------------------------------------------------------- indexing

    // ------------------------------------------------------- answers

    /// The reference or definition at a position, in any module of the
    /// file.
    fn site_at(&self, file: &str, line: u32, col: u32) -> Option<(String, Site)> {
        for (name, st) in &self.modules {
            if st.file != file {
                continue;
            }
            for d in &st.defs {
                if d.line == line && d.col == col {
                    return Some((
                        name.clone(),
                        Site::Def(d.name.clone(), d.kind),
                    ));
                }
            }
            for r in &st.refs {
                if r.line == line && r.col == col {
                    return Some((
                        name.clone(),
                        Site::Ref(r.kind, r.quals.clone(), r.name.clone()),
                    ));
                }
            }
        }
        None
    }

    /// Resolve a name in `module`'s scope to its owning module and kind:
    /// the module's own declarations, then its imports, then the prelude
    /// facade (the implicit base layer).
    fn resolve(
        &self,
        module: &str,
        kind: RefKind,
        quals: &[String],
        name: &str,
    ) -> Option<(String, SymKind, String)> {
        let mut candidates: Vec<String> = Vec::new();
        if !quals.is_empty() {
            let q = quals.join(".");
            if self.modules.contains_key(&q) {
                candidates.push(q);
            } else {
                // `import Data.List` is also seen as `List.foo`.
                for (m, st) in &self.modules {
                    if m == &q
                        || m.ends_with(&format!(".{q}"))
                        || st.imports.iter().any(|i| i == &q || i.ends_with(&format!(".{q}")))
                    {
                        if m != module && !candidates.contains(m) {
                            candidates.push(m.clone());
                        }
                    }
                }
            }
        } else {
            candidates.push(module.to_string());
            if let Some(st) = self.modules.get(module) {
                for i in &st.imports {
                    candidates.push(i.clone());
                }
            }
            // The facade is the implicit base layer: its surface is the
            // family it re-exports.
            candidates.push("Prelude".to_string());
            if let Some(facade) = self.prelude_module_state("Prelude") {
                for i in &facade.imports {
                    candidates.push(i.clone());
                }
            }
        }
        for cand in &candidates {
            if let Some(found) = self.def_in(cand, kind, name) {
                return Some(found);
            }
        }
        None
    }

    /// The definition of `name` in `module`'s namespace, if it has one.
    fn def_in(&self, module: &str, kind: RefKind, name: &str) -> Option<(String, SymKind, String)> {
        let st = self.prelude_module_state(module).or_else(|| self.modules.get(module))?;
        let want = |k: SymKind| match kind {
            RefKind::Value => matches!(
                k,
                SymKind::Value
                    | SymKind::Ctor
                    | SymKind::Field
                    | SymKind::Method
                    | SymKind::Law
                    | SymKind::EffectOp
            ),
            RefKind::Type => matches!(k, SymKind::Type | SymKind::Class | SymKind::Effect),
        };
        st.defs
            .iter()
            .find(|d| d.name == name && want(d.kind))
            .map(|d| (module.to_string(), d.kind, d.name.clone()))
    }

    /// The prelude family's index of `module` (built on first use).
    fn prelude_module_state(&self, module: &str) -> Option<&ModuleState> {
        if self.modules.contains_key(module) {
            return None;
        }
        self.prelude_index
            .as_ref()
            .and_then(|idx| idx.get(module))
    }

    /// The rendered scheme (or type) of a resolved name.
    pub fn scheme_of(&self, module: &str, name: &str, kind: SymKind) -> Option<String> {
        if let Some(iface) = self.ifaces.get(module) {
            let text = match kind {
                SymKind::Value | SymKind::Method | SymKind::EffectOp | SymKind::Law => {
                    iface.exports
                        .values
                        .iter()
                        .find(|v| v.name == name)
                        .map(|v| v.scheme.clone())
                        .or_else(|| {
                            iface.exports.signatures.iter().find_map(|s| {
                                s.methods
                                    .iter()
                                    .find(|m| m.name == name)
                                    .map(|m| m.scheme.clone())
                            })
                        })
                }
                SymKind::Ctor => iface.exports.types.iter().find_map(|t| {
                    t.ctors.iter().find(|c| c.name == name).map(|c| {
                        let mut out = c
                            .fields
                            .iter()
                            .cloned()
                            .chain(std::iter::once(render_type_head(t)))
                            .collect::<Vec<_>>()
                            .join(" -> ");
                        if c.fields.is_empty() {
                            out = render_type_head(t);
                        }
                        out
                    })
                }),
                SymKind::Type => iface.exports.types.iter().find(|t| t.name == name).map(|t| {
                    if t.params.is_empty() {
                        format!("type {} :: {}", t.name, t.kind)
                    } else {
                        format!(
                            "type {} {} :: {}",
                            t.name,
                            t.params.join(" "),
                            (0..t.params.len())
                                .map(|_| "Type")
                                .collect::<Vec<_>>()
                                .join(" -> ")
                        )
                    }
                }),
                SymKind::Class => iface
                    .exports
                    .signatures
                    .iter()
                    .find(|s| s.name == name)
                    .map(|s| format!("class {} {}", s.name, s.params.join(" "))),
                SymKind::Field => None,
                SymKind::Instance => iface
                    .exports
                    .instances
                    .iter()
                    .find(|i| i.id == name)
                    .map(|i| format!("instance {} :: {}", i.id, i.class)),
                SymKind::Effect => None,
            };
            if text.is_some() {
                return text;
            }
        }
        // Fallback: the rendered signature text of the declaration.
        let st = self
            .modules
            .get(module)
            .or_else(|| self.prelude_module_state(module))?;
        st.sig_text.get(name).cloned()
    }

    /// Hover at a position: the name's scheme in a code block.
    pub fn hover(&self, file: &str, line: u32, col: u32) -> Option<String> {
        let (module, site) = self.site_at(file, line, col)?;
        let (rkind, quals, name) = match &site {
            Site::Def(name, kind) => {
                let text = self.scheme_of(&module, name, *kind)?;
                return Some(format!("```safelang\n{name} :: {text}\n```"));
            }
            Site::Ref(kind, quals, name) => (*kind, quals.clone(), name.clone()),
        };
        let (owner, kind, _) = self.resolve(&module, rkind, &quals, &name)?;
        let text = self.scheme_of(&owner, &name, kind)?;
        Some(format!("```safelang\n{name} :: {text}\n```"))
    }

    /// Go to a name's definition: `(file, line, col)` in workspace
    /// coordinates.
    pub fn definition(&self, file: &str, line: u32, col: u32) -> Option<(String, u32, u32)> {
        let (module, site) = self.site_at(file, line, col)?;
        let (owner, _, name) = match &site {
            Site::Def(name, kind) => (module.clone(), *kind, name.clone()),
            Site::Ref(kind, quals, name) => self.resolve(&module, *kind, quals, name)?,
        };
        let st = self
            .modules
            .get(&owner)
            .or_else(|| self.prelude_module_state(&owner))?;
        let d = st.defs.iter().find(|d| d.name == name)?;
        Some((st.file.clone(), d.line, d.col))
    }

    /// The completion candidates visible at a position: the file's
    /// module, its imports, and the prelude facade.
    pub fn completions(&self, file: &str, line: u32, col: u32) -> Vec<Completion> {
        let mut prefix = String::new();
        if let Some(text) = self.document(file) {
            if let Some(src) = text.lines().nth(line.saturating_sub(1) as usize) {
                let chars: Vec<char> = src.chars().collect();
                let mut i = col.saturating_sub(1) as usize;
                while i > 0
                    && (chars[i - 1].is_ascii_alphanumeric()
                        || chars[i - 1] == '_'
                        || chars[i - 1] == '\''
                        || chars[i - 1] == '.')
                {
                    i -= 1;
                }
                prefix = chars[i..(col.saturating_sub(1) as usize).min(chars.len())]
                    .iter()
                    .collect();
            }
        }
        self.completions_matching(file, &prefix)
    }

    /// The completion candidates visible in `file`'s scope whose name
    /// starts with `prefix` (the REPL's completion over session and
    /// prelude names).
    pub fn completions_matching(&self, file: &str, prefix: &str) -> Vec<Completion> {
        let bare = prefix.rsplit('.').next().unwrap_or("");
        let mut seen: BTreeSet<(String, SymKind)> = BTreeSet::new();
        let mut out: Vec<Completion> = Vec::new();
        let push = |module: &str,
                    st: &ModuleState,
                    seen: &mut BTreeSet<(String, SymKind)>,
                    out: &mut Vec<Completion>| {
            for d in &st.defs {
                if !bare.is_empty() && !d.name.starts_with(bare) {
                    continue;
                }
                if !seen.insert((d.name.clone(), d.kind)) {
                    continue;
                }
                let detail = self.scheme_of(module, &d.name, d.kind).unwrap_or_default();
                out.push(Completion {
                    label: d.name.clone(),
                    kind: d.kind,
                    detail,
                });
            }
        };
        let mut order: Vec<String> = Vec::new();
        for (name, st) in &self.modules {
            if st.file == file && !order.contains(name) {
                order.push(name.clone());
            }
        }
        if let Some(st) = self.modules.get(&self.first_module_of(file)) {
            for i in &st.imports {
                order.push(i.clone());
            }
        }
        order.push("Prelude".to_string());
        if let Some(facade) = self.prelude_module_state("Prelude") {
            for i in &facade.imports {
                order.push(i.clone());
            }
        }
        for m in &order {
            let owned;
            let st = match self.modules.get(m) {
                Some(st) => st,
                None => match self.prelude_module_state(m) {
                    Some(st) => st,
                    None => {
                        owned = ModuleState::default();
                        &owned
                    }
                },
            };
            push(m, st, &mut seen, &mut out);
        }
        out.sort_by(|a, b| a.label.cmp(&b.label));
        out
    }

    /// Try to parse `text` as declarations in `module`'s context (the
    /// REPL's definition/expression split). The text's spans are its own
    /// (the caller renders them).
    pub fn parse_decls(&mut self, module: &str, text: &str) -> crate::Result<Vec<ast::Decl>> {
        let fix = self.module_fixity(module);
        let mut src = format!("module {module} where\n");
        for line in text.lines() {
            src.push_str("    ");
            src.push_str(line);
            src.push('\n');
        }
        let tokens = lexer::lex(&src)?;
        let tokens = crate::layout::layout(tokens)?;
        let parsed = parser::parse_modules(&tokens, &fix, &HashMap::new())?;
        let mut decls = Vec::new();
        for m in parsed.modules {
            decls.extend(m.decls);
        }
        Ok(decls)
    }

    /// Parse `text` as one expression in `module`'s context.
    pub fn parse_expr(&mut self, module: &str, text: &str) -> crate::Result<ast::Term> {
        let fix = self.module_fixity(module);
        parse_term(text, &fix)
    }

    /// Everything `module` parses against: its imports' fixities plus
    /// its own (the prelude base when it is not loaded yet).
    fn module_fixity(&mut self, module: &str) -> FixityTable {
        if let Some(st) = self.modules.get(module) {
            return st.fixities.clone();
        }
        self.prelude_base()
    }

    fn first_module_of(&self, file: &str) -> String {
        self.modules
            .iter()
            .find(|(_, m)| m.file == file)
            .map(|(n, _)| n.clone())
            .unwrap_or_default()
    }

    /// The semantic tokens of a file (P22): the lexer's tokens,
    /// classified with the module indices.
    pub fn semantic_tokens(&self, file: &str) -> Vec<SemToken> {
        let Some(text) = self.document(file) else {
            return Vec::new();
        };
        let Ok(tokens) = lexer::lex(text).and_then(crate::layout::layout) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for t in tokens {
            if t.virtual_ {
                continue;
            }
            let (kind, len) = match &t.tok {
                Tok::VarId(name) => {
                    let k = if self
                        .modules
                        .values()
                        .filter(|m| m.file == file)
                        .any(|m| {
                            m.defs
                                .iter()
                                .any(|d| d.line == t.span.line && d.col == t.span.col)
                        }) {
                        token_kind_of_def(self.def_kind_at(file, t.span))
                    } else {
                        match self.ref_at(file, t.span) {
                            Some((module, kind, quals, name)) => {
                                match self.resolve(&module, kind, &quals, &name) {
                                    Some((_, k, _)) => token_kind_of_def(Some(k)),
                                    None => "variable",
                                }
                            }
                            None => "variable",
                        }
                    };
                    (k, name.chars().count() as u32)
                }
                Tok::ConId(name) => {
                    let k = match self.ref_at(file, t.span) {
                        Some((module, kind, quals, name)) => {
                            match self.resolve(&module, kind, &quals, &name) {
                                Some((_, SymKind::Ctor, _)) => "enumMember",
                                Some((_, SymKind::Class, _)) => "class",
                                Some((_, SymKind::Type, _)) => "type",
                                Some((_, SymKind::Effect, _)) => "type",
                                _ => "type",
                            }
                        }
                        None => "type",
                    };
                    (k, name.chars().count() as u32)
                }
                Tok::Op(name) => ("operator", name.chars().count() as u32),
                Tok::Int(s) | Tok::FloatLit(s) => ("number", s.chars().count() as u32),
                Tok::StrLit(s) => ("string", s.chars().count() as u32 + 2),
                Tok::CharLit(s) => ("string", s.chars().count() as u32 + 2),
                other => {
                    if let Some(k) = keyword_of(other) {
                        (k, keyword_len(other))
                    } else {
                        continue;
                    }
                }
            };
            out.push(SemToken {
                line: t.span.line,
                col: t.span.col,
                len,
                kind,
            });
        }
        out
    }

    fn def_kind_at(&self, file: &str, span: Span) -> Option<SymKind> {
        self.modules
            .values()
            .filter(|m| m.file == file)
            .flat_map(|m| m.defs.iter())
            .find(|d| d.line == span.line && d.col == span.col)
            .map(|d| d.kind)
    }

    fn ref_at(&self, file: &str, span: Span) -> Option<(String, RefKind, Vec<String>, String)> {
        self.modules
            .iter()
            .filter(|(_, m)| m.file == file)
            .find_map(|(module, m)| {
                m.refs
                    .iter()
                    .find(|r| r.line == span.line && r.col == span.col)
                    .map(|r| {
                        (
                            module.clone(),
                            r.kind,
                            r.quals.clone(),
                            r.name.clone(),
                        )
                    })
            })
    }

    /// The names one module defines, with their rendered schemes
    /// (`:browse`).
    pub fn browse(&self, module: &str) -> Vec<(String, String)> {
        let Some(st) = self
            .modules
            .get(module)
            .or_else(|| self.prelude_module_state(module))
        else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for d in &st.defs {
            let text = self
                .scheme_of(module, &d.name, d.kind)
                .unwrap_or_else(|| d.kind.label().to_string());
            out.push((d.name.clone(), text));
        }
        out.sort();
        out
    }

    /// The kind of a type or class name (P21's `:kind`): each
    /// parameter's kind, defaulting to `Type`, then `Type` (classes end
    /// in `Constraint`).
    pub fn kind_of(&self, name: &str) -> Option<String> {
        for st in self.modules.values() {
            if let Some(k) = st.sig_text.get(&format!("kind:{name}")) {
                return Some(k.clone());
            }
        }
        if let Some(st) = self.prelude_index.as_ref().and_then(|idx| idx.get("Prelude")) {
            if let Some(k) = st.sig_text.get(&format!("kind:{name}")) {
                return Some(k.clone());
            }
        }
        // A name only reachable through an import: search every index.
        for st in self.prelude_index.as_ref().into_iter().flat_map(|i| i.values()) {
            if let Some(k) = st.sig_text.get(&format!("kind:{name}")) {
                return Some(k.clone());
            }
        }
        None
    }
}

/// The thing found at a source position.
enum Site {
    Def(String, SymKind),
    Ref(RefKind, Vec<String>, String),
}

/// Is `module` one of the embedded prelude family?
fn is_prelude_module(module: &str) -> bool {
    driver::prelude_module_names().iter().any(|n| n == module)
}

/// The slice's modules in dependency order (imports first).
fn topo_slice(analysis: &Analysis, slice: &BTreeSet<String>) -> Vec<String> {
    let mut graph: BTreeMap<String, Vec<interface::IfaceRef>> = BTreeMap::new();
    for m in slice {
        let imports = analysis
            .modules
            .get(m)
            .map(|st| {
                st.imports
                    .iter()
                    .filter(|i| slice.contains(*i))
                    .map(|i| interface::IfaceRef {
                        module: i.clone(),
                        iface_hash: String::new(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        graph.insert(m.clone(), imports);
    }
    let mut order = interface::topo_order(&graph).unwrap_or_default();
    order.retain(|m| slice.contains(m));
    order
}

/// Parse one banded file.
fn parse_file(
    banded: &str,
    file: &str,
    base: &FixityTable,
    tables: &HashMap<String, FixityTable>,
) -> Result<ParsedModules, Error> {
    let tokens = lexer::lex(banded)?;
    let tokens = crate::layout::layout(tokens)?;
    let parsed = parser::parse_modules(&tokens, base, tables)?;
    let _ = file;
    Ok(parsed)
}

/// Parse one expression (the REPL probe source) in `fix`'s context.
pub fn parse_term(src: &str, fix: &FixityTable) -> crate::Result<ast::Term> {
    let tokens = lexer::lex(src)?;
    let tokens = crate::layout::layout(tokens)?;
    parser::parse_term_tokens(&tokens, fix)
}

/// Index one parsed module: definition and reference sites in workspace
/// coordinates, plus rendered signature text.
fn index_module(file: &str, base: u64, modu: &ast::Module) -> ModuleState {
    let conv = |s: Span| -> (u32, u32) {
        (
            if s.line as u64 >= base {
                s.line - (base as u32 - 1)
            } else {
                s.line
            },
            s.col,
        )
    };
    let mut st = ModuleState {
        file: file.to_string(),
        imports: Vec::new(),
        defs: Vec::new(),
        refs: Vec::new(),
        sig_text: HashMap::new(),
        fixities: FixityTable::default(),
    };
    for imp in &modu.imports {
        st.imports.push(imp.path.parts().join("."));
    }
    for d in &modu.decls {
        index_defs(d, &mut st.defs, &mut st.sig_text, &conv);
        let mut refs = Vec::new();
        ast::collect_decl_refs(d, &mut refs);
        for r in refs {
            let (line, col) = conv(r.span);
            st.refs.push(Ref {
                kind: r.kind,
                quals: r.quals,
                name: r.name,
                line,
                col,
            });
        }
    }
    st
}

/// The shipped source name of an embedded prelude module (go-to-
/// definition lands in the family's real files, still relative).
fn prelude_file_of(module: &str) -> String {
    if module == "Prelude" {
        return "prelude/Prelude.lang".to_string();
    }
    for (path, _) in driver::PRELUDE_EARLY.iter().chain(driver::PRELUDE_LATE.iter()) {
        if path.trim_end_matches(".lang").replace('/', ".") == module {
            return format!("prelude/{path}");
        }
    }
    format!("prelude/{}.lang", module.replace('.', "/"))
}

/// Record the definition sites of one declaration.
fn index_defs(
    d: &Decl,
    defs: &mut Vec<Def>,
    sig_text: &mut HashMap<String, String>,
    conv: &dyn Fn(Span) -> (u32, u32),
) {
    let add = |name: &ast::Name, kind: SymKind, defs: &mut Vec<Def>| {
        let (line, col) = conv(name.span);
        if !defs.iter().any(|d| d.name == name.text && d.kind == kind) {
            defs.push(Def { name: name.text.clone(), kind, line, col });
        }
    };
    match d {
        Decl::ValueSig { name, ty, .. } => {
            add(name, SymKind::Value, defs);
            sig_text.insert(name.text.clone(), render_ast_type(ty));
        }
        Decl::Equation { eq, .. } => {
            add(&eq.name, SymKind::Value, defs);
        }
        Decl::Data(t) => {
            add(&t.name, SymKind::Type, defs);
            sig_text.insert(
                t.name.text.clone(),
                kind_shape(&t.params, false),
            );
            sig_text.insert(
                format!("kind:{}", t.name.text),
                kind_shape(&t.params, false),
            );
            for c in &t.ctors {
                add(&c.name, SymKind::Ctor, defs);
                sig_text
                    .entry(c.name.text.clone())
                    .or_insert_with(|| render_ast_type(&c.ty));
            }
        }
        Decl::Record(r) | Decl::Codata(r) => {
            add(&r.name, SymKind::Type, defs);
            add(&r.name, SymKind::Value, defs);
            sig_text.insert(format!("kind:{}", r.name.text), kind_shape(&r.params, false));
            for f in &r.fields {
                add(&f.name, SymKind::Field, defs);
                sig_text.insert(f.name.text.clone(), render_ast_type(&f.ty));
            }
        }
        Decl::Signature(s) => {
            add(&s.name, SymKind::Class, defs);
            sig_text.insert(format!("kind:{}", s.name.text), kind_shape(&s.params, true));
            for (m, ty) in &s.methods {
                add(m, SymKind::Method, defs);
                sig_text.insert(m.text.clone(), render_ast_type(ty));
            }
            for (l, ty) in &s.laws {
                add(l, SymKind::Law, defs);
                sig_text.insert(l.text.clone(), render_ast_type(ty));
            }
        }
        Decl::Instance(i) => {
            add(&i.name, SymKind::Instance, defs);
            for m in &i.methods {
                add(&m.name, SymKind::Value, defs);
            }
        }
        Decl::Foreign(f) => {
            add(&f.name, SymKind::Value, defs);
            sig_text.insert(f.name.text.clone(), render_ast_type(&f.ty));
        }
        Decl::Prim(p) => {
            add(&p.name, SymKind::Value, defs);
            sig_text.insert(p.name.text.clone(), render_ast_type(&p.ty));
        }
        Decl::OpaqueType(o) => {
            add(&o.name, SymKind::Type, defs);
            sig_text.insert(format!("kind:{}", o.name.text), render_ast_type(&o.kind));
        }
        Decl::Effect(e) => {
            add(&e.name, SymKind::Effect, defs);
            for (op, ty) in &e.ops {
                add(op, SymKind::EffectOp, defs);
                sig_text.insert(op.text.clone(), render_ast_type(ty));
            }
        }
        Decl::Attributed { inner, .. } => index_defs(inner, defs, sig_text, conv),
        _ => {}
    }
}

/// The kind of a parameterized declaration: each parameter's annotated
/// kind (default `Type`), then the result kind.
fn kind_shape(params: &[ast::TypeBinder], class: bool) -> String {
    let mut parts: Vec<String> = params
        .iter()
        .map(|p| match &p.ann {
            Some(ann) => format!("({})", render_ast_type(ann)),
            None => "Type".to_string(),
        })
        .collect();
    parts.push(if class {
        "Constraint".to_string()
    } else {
        "Type".to_string()
    });
    parts.join(" -> ")
}

/// The result type name of a type export (`T a b`).
fn render_type_head(t: &interface::TypeExport) -> String {
    if t.params.is_empty() {
        t.name.clone()
    } else {
        format!("{} {}", t.name, t.params.join(" "))
    }
}

/// Render an `ast::Type` as written (hover and `:kind` text).
pub fn render_ast_type(t: &ast::Type) -> String {
    use ast::Type;
    match t {
        Type::Con(p, _) => p.parts().join("."),
        Type::Var(n, _) => n.text.clone(),
        Type::App(f, a, _) => {
            let f = render_ast_type(f);
            let a = render_ast_type(a);
            format!("{f} {a}")
        }
        Type::AppRow(f, row, _) => {
            format!("{} {{{}}}", render_ast_type(f), render_effect_row(row))
        }
        Type::Fun { arg, mult, eff, res, .. } => {
            let m = match mult {
                Some(n) => format!("{} ", n.text),
                None => String::new(),
            };
            let e = match eff {
                Some(row) => format!("{{{}}} ", render_effect_row(row)),
                None => String::new(),
            };
            format!(
                "{} ->{m}{e} {}",
                render_ast_type(arg),
                render_ast_type(res)
            )
        }
        Type::Forall(binders, body, _) => format!(
            "forall {}. {}",
            binders
                .iter()
                .map(|b| b.name.text.clone())
                .collect::<Vec<_>>()
                .join(" "),
            render_ast_type(body)
        ),
        Type::Using(ctx, body, _) => format!(
            "using ({}) => {}",
            ctx.iter()
                .map(|c| format!("{} :: {}", c.name.text, c.class.parts().join(".")))
                .collect::<Vec<_>>()
                .join(", "),
            render_ast_type(body)
        ),
        Type::DepFun(n, dom, mult, eff, cod, _) => {
            let m = match mult {
                Some(n) => format!("{} ", n.text),
                None => String::new(),
            };
            let e = match eff {
                Some(row) => format!("{{{}}} ", render_effect_row(row)),
                None => String::new(),
            };
            format!(
                "({} :: {}) ->{m}{e} {}",
                n.text,
                render_ast_type(dom),
                render_ast_type(cod)
            )
        }
        Type::Sigma(n, dom, cod, _) => format!(
            "Sigma ({} :: {}). {}",
            n.text,
            render_ast_type(dom),
            render_ast_type(cod)
        ),
        Type::Unit(_) => "()".to_string(),
        Type::Tuple(items, _) => format!(
            "({})",
            items.iter().map(render_ast_type).collect::<Vec<_>>().join(", ")
        ),
        Type::Infix(items, ops, _) => {
            let mut out = String::new();
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                    out.push_str(&ops[i - 1].text);
                    out.push(' ');
                }
                out.push_str(&render_ast_type(item));
            }
            out
        }
        Type::Num(n, _) => n.clone(),
    }
}

fn render_effect_row(row: &ast::EffectRow) -> String {
    let mut parts: Vec<String> = row
        .labels
        .iter()
        .map(|(p, args)| {
            if args.is_empty() {
                p.parts().join(".")
            } else {
                format!(
                    "{} {}",
                    p.parts().join("."),
                    args.iter().map(|a| a.text.clone()).collect::<Vec<_>>().join(" ")
                )
            }
        })
        .collect();
    if let Some(tail) = &row.tail {
        parts.push(tail.text.clone());
    }
    parts.join(", ")
}

/// The D92 evaluator's values, rendered (P21): type-directed where the
/// value's machine words need their type to be read (`Char`).
pub fn render_value(ty: &Ty, v: &Value) -> String {
    match v {
        Value::Word(w) => match ty {
            Ty::Prim(Scalar::Char) => {
                let c = char::from_u32(*w as u32).unwrap_or('\u{fffd}');
                match c {
                    '\'' => "'\\''".to_string(),
                    '\\' => "'\\\\'".to_string(),
                    c if (c as u32) < 0x20 => format!("'\\u{{{:x}}}'", c as u32),
                    c => format!("'{c}'"),
                }
            }
            _ => w.to_string(),
        },
        Value::Big { digits, .. } => digits.clone(),
        Value::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        Value::Unit => "()".to_string(),
        Value::Bytes(b) => format!("<bytes {}>", b.borrow().len()),
        Value::Str(s) => format!("{:?}", s),
        Value::Adt { ctor, fields, .. } => {
            let name = ctor.rsplit('.').next().unwrap_or(ctor).to_string();
            if fields.is_empty() {
                name
            } else {
                let args: Vec<String> = fields
                    .iter()
                    .map(|f| {
                        let s = render_value(&Ty::Unit, f);
                        if s.contains(' ') {
                            format!("({s})")
                        } else {
                            s
                        }
                    })
                    .collect();
                format!("{name} {}", args.join(" "))
            }
        }
        Value::Clos { .. } | Value::CtorFn { .. } | Value::PrimFn { .. } | Value::StageFn { .. } => {
            "<function>".to_string()
        }
        Value::Dict { .. } => "<dictionary>".to_string(),
        Value::Code(_) => "<code>".to_string(),
        Value::Decls(_) => "<decls>".to_string(),
        Value::Syntax { .. } => "<syntax>".to_string(),
        Value::Secret { name, .. } => format!("<secret {name}>"),
        Value::Db(_) => "<db>".to_string(),
        Value::Schema(_) => "<schema>".to_string(),
        Value::Cap(_) => "<capability>".to_string(),
    }
}

/// The token class of a definition kind.
fn token_kind_of_def(k: Option<SymKind>) -> &'static str {
    match k {
        Some(SymKind::Value) | Some(SymKind::EffectOp) => "function",
        Some(SymKind::Method) => "method",
        Some(SymKind::Field) => "property",
        Some(SymKind::Ctor) => "enumMember",
        Some(SymKind::Type) => "type",
        Some(SymKind::Class) => "class",
        Some(SymKind::Effect) => "type",
        Some(SymKind::Law) => "function",
        Some(SymKind::Instance) => "class",
        None => "variable",
    }
}

/// Reserved words are semantic keywords; punctuation is not semantic.
fn keyword_of(tok: &Tok) -> Option<&'static str> {
    use Tok::*;
    let kind = match tok {
        Module | Where | Data | Record | Codata | Effect | Signature | Instance | Implicit
        | Import | As | TypeKw | Opaque | Forall | Sigma | Using | In | Let | Letrec | If
        | Then | Else | Case | Of | Do | Handle | With | Rewrite | Target | Justify | MatchKw
        | Replace | Require | Upgrade | Prim | Law | Contract | To | Generate | QuoteDecls
        | Foreign | Export | Ccall | Component | Proof | Spec | Stage | Corec | Macro | Infix
        | Infixl | Infixr | Erased | Own | Return | Resume => "keyword",
        _ => return None,
    };
    Some(kind)
}

/// The rendered length of a keyword token.
fn keyword_len(tok: &Tok) -> u32 {
    let s = match tok {
        Tok::Module => "module",
        Tok::Where => "where",
        Tok::Data => "data",
        Tok::Record => "record",
        Tok::Codata => "codata",
        Tok::Effect => "effect",
        Tok::Signature => "signature",
        Tok::Instance => "instance",
        Tok::Implicit => "implicit",
        Tok::Import => "import",
        Tok::As => "as",
        Tok::TypeKw => "type",
        Tok::Opaque => "opaque",
        Tok::Forall => "forall",
        Tok::Sigma => "Sigma",
        Tok::Using => "using",
        Tok::In => "in",
        Tok::Let => "let",
        Tok::Letrec => "letrec",
        Tok::If => "if",
        Tok::Then => "then",
        Tok::Else => "else",
        Tok::Case => "case",
        Tok::Of => "of",
        Tok::Do => "do",
        Tok::Handle => "handle",
        Tok::With => "with",
        Tok::Rewrite => "rewrite",
        Tok::Target => "target",
        Tok::Justify => "justify",
        Tok::MatchKw => "match",
        Tok::Replace => "replace",
        Tok::Require => "require",
        Tok::Upgrade => "upgrade",
        Tok::Prim => "prim",
        Tok::Law => "law",
        Tok::Contract => "contract",
        Tok::To => "to",
        Tok::Generate => "generate",
        Tok::QuoteDecls => "decls",
        Tok::Foreign => "foreign",
        Tok::Export => "export",
        Tok::Ccall => "ccall",
        Tok::Component => "component",
        Tok::Proof => "proof",
        Tok::Spec => "spec",
        Tok::Stage => "stage",
        Tok::Corec => "corec",
        Tok::Macro => "macro",
        Tok::Infix => "infix",
        Tok::Infixl => "infixl",
        Tok::Infixr => "infixr",
        Tok::Erased => "erased",
        Tok::Own => "own",
        Tok::Return => "return",
        Tok::Resume => "resume",
        _ => "?",
    };
    s.chars().count() as u32
}

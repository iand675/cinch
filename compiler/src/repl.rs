//! P21: the REPL, backed by the shared analysis layer.
//!
//! The REPL drives [`Analysis`] exactly as the language server does: one
//! incremental checker, two front ends. Each input is either a session
//! command (`:type`, `:kind`, `:browse`, `:load`, `:reload`), a block of
//! declarations, or an expression; session state persists across inputs
//! and expression evaluation runs through the D92 evaluator. Incomplete
//! input (an open block, an unfinished operator or literal) continues on
//! the next line. Errors show their source span. Completion covers the
//! session's and the prelude's names.

use crate::analysis::{self, Analysis, ProbeOutcome, SESSION_FILE, SESSION_MODULE};
use crate::ast;
use crate::lexer::{self, Span, Tok};

/// The outcome of feeding one input line to the REPL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplOutcome {
    /// The input completed a block; this is what the REPL prints.
    Output(String),
    /// The input is incomplete: the REPL waits for more lines.
    Incomplete,
    /// `:quit`.
    Quit,
}

/// The REPL session: accumulated declarations and their analysis.
pub struct Repl {
    analysis: Analysis,
    /// The accepted declaration blocks, in order (the session source).
    entries: Vec<String>,
    /// Lines each entry carries before its block's own (synthesized
    /// signatures and joined pending ones).
    entry_skips: Vec<u32>,
    /// Signatures whose defining equations have not arrived yet (a
    /// signature alone is not a checked state).
    pending_sigs: Vec<(String, String)>,
    /// The lines of the block being continued.
    pending: Vec<String>,
    history: Vec<String>,
}

impl Default for Repl {
    fn default() -> Repl {
        Repl::new()
    }
}

impl Repl {
    /// A REPL over the current directory.
    pub fn new() -> Repl {
        Repl::with_root(std::env::current_dir().unwrap_or_default())
    }

    /// A REPL whose paths resolve against `root`.
    pub fn with_root(root: impl AsRef<std::path::Path>) -> Repl {
        let mut repl = Repl {
            analysis: Analysis::new(root),
            entries: Vec::new(),
            entry_skips: Vec::new(),
            pending_sigs: Vec::new(),
            pending: Vec::new(),
            history: Vec::new(),
        };
        repl.sync_session();
        repl
    }

    /// The shared analysis layer (diagnostics, recheck reports).
    pub fn analysis(&self) -> &Analysis {
        &self.analysis
    }

    /// Every accepted input line, in order.
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// The session source the analysis sees (for tests and debugging).
    pub fn session_source(&self) -> String {
        session_source(&self.entries)
    }

    /// Feed one input line. Lines of a multi-line block are fed one at a
    /// time; [`ReplOutcome::Incomplete`] says the block is not closed.
    pub fn eval(&mut self, input: &str) -> ReplOutcome {
        self.history.push(input.to_string());
        if self.pending.is_empty() {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                return ReplOutcome::Output(String::new());
            }
            if trimmed.starts_with(':') {
                return self.command(trimmed);
            }
        }
        self.pending.push(input.to_string());
        let block = self.pending.join("\n");
        if is_incomplete(&block) {
            return ReplOutcome::Incomplete;
        }
        self.pending.clear();
        self.eval_block(&block)
    }

    /// Completion over the session's and the prelude's names (and the
    /// command names after `:`).
    pub fn complete(&self, partial: &str) -> Vec<String> {
        if let Some(cmd) = partial.strip_prefix(':') {
            return [":type", ":kind", ":browse", ":load", ":reload", ":history", ":help", ":quit"]
                .iter()
                .filter(|c| c.starts_with(&format!(":{cmd}")) || **c == partial)
                .map(|c| c.to_string())
                .collect();
        }
        self.analysis
            .completions_matching(SESSION_FILE, partial)
            .into_iter()
            .map(|c| c.label)
            .collect()
    }

    // ------------------------------------------------------- blocks

    /// The session source: the module header and every entry, each
    /// entry's lines indented into the module body.
    fn sync_session(&mut self) {
        let src = self.session_source();
        self.analysis.set_document(SESSION_FILE, &src);
    }

    /// Evaluate one complete block: declarations or an expression.
    fn eval_block(&mut self, block: &str) -> ReplOutcome {
        let decls = match self.analysis.parse_decls(SESSION_MODULE, block) {
            Ok(d) if !d.is_empty() => d,
            _ => return self.eval_expr(block),
        };
        self.eval_decls(block, &decls)
    }

    /// A block of declarations: signatures pair with their equations,
    /// bare equations get their signature synthesized by the checker
    /// (this draft's rule is that definitions state their types), and
    /// everything else enters the session directly.
    fn eval_decls(&mut self, block: &str, decls: &[ast::Decl]) -> ReplOutcome {
        let session_sigs: Vec<String> = self
            .analysis
            .parse_decls(SESSION_MODULE, &self.session_source())
            .map(|ds| {
                ds.iter()
                    .filter_map(|d| match d {
                        ast::Decl::ValueSig { name, .. } => Some(name.text.clone()),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut sig_names: Vec<String> = Vec::new();
        let mut eq_names: Vec<String> = Vec::new();
        let mut other_names: Vec<String> = Vec::new();
        for d in decls {
            match d {
                ast::Decl::ValueSig { name, .. } => sig_names.push(name.text.clone()),
                ast::Decl::Equation { eq, .. } => eq_names.push(eq.name.text.clone()),
                ast::Decl::Data(t) => {
                    other_names.push(t.name.text.clone());
                    for c in &t.ctors {
                        other_names.push(c.name.text.clone());
                    }
                }
                ast::Decl::Record(r) | ast::Decl::Codata(r) => {
                    other_names.push(r.name.text.clone());
                }
                ast::Decl::Signature(s) => {
                    other_names.push(s.name.text.clone());
                    for (m, _) in &s.methods {
                        other_names.push(m.text.clone());
                    }
                }
                ast::Decl::Instance(i) => other_names.push(i.name.text.clone()),
                ast::Decl::Foreign(f) => other_names.push(f.name.text.clone()),
                ast::Decl::Prim(p) => other_names.push(p.name.text.clone()),
                ast::Decl::OpaqueType(o) => other_names.push(o.name.text.clone()),
                ast::Decl::Effect(e) => {
                    other_names.push(e.name.text.clone());
                    for (op, _) in &e.ops {
                        other_names.push(op.text.clone());
                    }
                }
                _ => {}
            }
        }
        // Equations without a signature: synthesize it through the
        // checker (only where the scheme re-states as surface syntax).
        let mut synth: Vec<String> = Vec::new();
        for (name, eq) in equations_of(decls) {
            let has_sig = sig_names.iter().any(|n| n == name)
                || self.pending_sigs.iter().any(|(n, _)| n == name)
                || session_sigs.iter().any(|n| n == name);
            if has_sig {
                continue;
            }
            match self.synthesize(name, eq) {
                Ok(sig) => synth.push(sig),
                Err(e) => {
                    return ReplOutcome::Output(render_error(&e, block, (1, 1)));
                }
            }
        }
        // Every declaration's source text, by its start line (the parse
        // wrapped the block one line down: the module header).
        let mut starts: Vec<usize> = decls.iter().map(|d| decl_span(d).line as usize).collect();
        starts.sort_unstable();
        let mut texts: Vec<(&ast::Decl, String)> = Vec::new();
        for d in decls.iter() {
            let start = decl_span(d).line as usize;
            let end = starts
                .iter()
                .copied()
                .filter(|s| *s > start)
                .min()
                .unwrap_or(usize::MAX);
            let lines: Vec<&str> = block
                .lines()
                .skip(start.saturating_sub(2))
                .take(end.saturating_sub(start))
                .collect();
            texts.push((d, lines.join("\n")));
        }
        // Signatures with no equation here: hold them until the defining
        // equation arrives (a signature alone does not check).
        let mut held: Vec<(String, String)> = Vec::new();
        let mut entry_lines: Vec<String> = synth.clone();
        for (d, text) in &texts {
            if let ast::Decl::ValueSig { name, .. } = d {
                let defined = eq_names.contains(&name.text)
                    || self.pending_sigs.iter().any(|(n, _)| n == &name.text)
                    || self.session_defines(&name.text);
                if !defined {
                    held.push((name.text.clone(), text.clone()));
                    continue;
                }
            }
            entry_lines.push(text.clone());
        }
        // A pending signature joins its equation's entry.
        let mut joined_pending: Vec<String> = Vec::new();
        self.pending_sigs.retain(|(n, text)| {
            if eq_names.contains(n) {
                joined_pending.push(text.clone());
                false
            } else {
                !held.iter().any(|(h, _)| h == n)
            }
        });
        entry_lines.splice(0..0, joined_pending.iter().cloned());
        self.pending_sigs.extend(held.iter().cloned());
        let skip_lines = synth.len() + joined_pending.len();
        let joined = entry_lines.join("\n");
        if joined.trim().is_empty() {
            let noted: Vec<String> = held
                .iter()
                .map(|(n, _)| format!("noted {n} (add its definition)"))
                .collect();
            return ReplOutcome::Output(noted.join("\n"));
        }
        self.entries.push(joined);
        self.entry_skips.push(skip_lines as u32);
        self.sync_session();
        let report = self.analysis.update();
        if let Some(diag) = report
            .diagnostics
            .iter()
            .find(|d| d.severity == analysis::Severity::Error)
        {
            let rendered = render_session_diag(diag, &self.entries, &self.entry_skips);
            // Roll the failed entry back; the session stays checkable.
            self.entries.pop();
            self.entry_skips.pop();
            self.pending_sigs.retain(|(n, _)| !eq_names.contains(n) || held.iter().any(|(h, _)| h == n));
            self.sync_session();
            self.analysis.update();
            return ReplOutcome::Output(rendered);
        }
        let mut out: Vec<String> = Vec::new();
        for (name, _) in &held {
            out.push(format!("noted {name} (add its definition)"));
        }
        let mut names = other_names.clone();
        names.extend(eq_names.iter().cloned());
        names.sort();
        names.dedup();
        for name in names {
            let scheme = self
                .analysis
                .scheme_of(SESSION_MODULE, &name, analysis::SymKind::Value)
                .or_else(|| {
                    self.analysis
                        .scheme_of(SESSION_MODULE, &name, analysis::SymKind::Type)
                })
                .unwrap_or_default();
            if scheme.is_empty() {
                out.push(format!("defined {name}"));
            } else {
                out.push(format!("defined {name} :: {scheme}"));
            }
        }
        if out.is_empty() {
            out.push("ok".to_string());
        }
        ReplOutcome::Output(out.join("\n"))
    }

    /// Is `name` defined (equation-bearing) in the session already?
    fn session_defines(&mut self, name: &str) -> bool {
        let entries = self.entries.clone();
        for e in entries {
            let found = self
                .analysis
                .parse_decls(SESSION_MODULE, &e)
                .map(|ds| {
                    ds.iter().any(|d| {
                        matches!(d, ast::Decl::Equation { eq, .. } if eq.name.text == name)
                    })
                })
                .unwrap_or(false);
            if found {
                return true;
            }
        }
        false
    }

    /// The signature of `name` from the equation alone: infer it with
    /// the checker (the D92 evaluator's elaboration) and re-state it as
    /// surface syntax.
    fn synthesize(&mut self, name: &str, eq: &ast::Equation) -> Result<String, String> {
        if !eq.locals.is_empty() {
            return Err(format!(
                "cannot infer the type of `{name}` (it binds `where` locals); state its signature"
            ));
        }
        let term = if eq.pats.is_empty() {
            eq.body.clone()
        } else {
            ast::Term::Lam(eq.pats.clone(), Box::new(eq.body.clone()), eq.span)
        };
        let (_report, outcome) = self.analysis.update_probe_term(SESSION_MODULE, &term);
        let scheme = match outcome {
            ProbeOutcome {
                scheme: Some(s),
                error: None,
                ..
            } => s,
            ProbeOutcome { error: Some(e), .. } => {
                return Err(format!(
                    "cannot infer the type of `{name}`: {}",
                    e.message
                ))
            }
            _ => {
                return Err(format!(
                    "cannot infer the type of `{name}`; state its signature (`{name} :: ...`)"
                ))
            }
        };
        if scheme.contains('$') || scheme.contains('?') {
            return Err(format!(
                "cannot infer the type of `{name}`; state its signature (`{name} :: ...`)"
            ));
        }
        Ok(format!("{name} :: {scheme}"))
    }

    /// An expression block: evaluate it through the D92 evaluator.
    fn eval_expr(&mut self, block: &str) -> ReplOutcome {
        if let Err(e) = self.analysis.parse_expr(SESSION_MODULE, block) {
            let at = e.span.map(|s| (s.line, s.col)).unwrap_or((1, 1));
            return ReplOutcome::Output(render_error(&e.message, block, at));
        }
        let (report, outcome) = self.analysis.update_probe(SESSION_MODULE, block);
        if let Some(e) = &outcome.error {
            return ReplOutcome::Output(render_error(
                &e.message,
                block,
                e.span.map(|s| (s.line, s.col)).unwrap_or((1, 1)),
            ));
        }
        if let Some(diag) = report
            .diagnostics
            .iter()
            .find(|d| d.severity == analysis::Severity::Error)
        {
            return ReplOutcome::Output(render_session_diag(diag, &self.entries, &self.entry_skips));
        }
        match outcome {
            ProbeOutcome {
                value: Some(v),
                scheme: Some(s),
                ..
            } => ReplOutcome::Output(format!("{v} :: {s}")),
            ProbeOutcome {
                value: None,
                scheme: Some(s),
                eval_error,
                ..
            } => {
                let mut out = format!("it :: {s}");
                if let Some(err) = eval_error {
                    out.push('\n');
                    out.push_str(&format!("error: {err}"));
                }
                ReplOutcome::Output(out)
            }
            _ => ReplOutcome::Output("error: the expression does not elaborate".to_string()),
        }
    }

    // ------------------------------------------------------- commands

    fn command(&mut self, cmd: &str) -> ReplOutcome {
        let (name, arg) = match cmd.split_once(' ') {
            Some((n, a)) => (n, a.trim()),
            None => (cmd, ""),
        };
        match name {
            ":type" | ":t" => {
                if arg.is_empty() {
                    return ReplOutcome::Output("usage: :type <expression>".to_string());
                }
                let (report, outcome) = self.analysis.update_probe(SESSION_MODULE, arg);
                if let Some(e) = &outcome.error {
                    return ReplOutcome::Output(render_error(
                        &e.message,
                        arg,
                        e.span.map(|s| (s.line, s.col)).unwrap_or((1, 1)),
                    ));
                }
                if let Some(diag) = report
                    .diagnostics
                    .iter()
                    .find(|d| d.severity == analysis::Severity::Error)
                {
                    return ReplOutcome::Output(render_session_diag(
                        diag,
                        &self.entries,
                        &self.entry_skips,
                    ));
                }
                match outcome.scheme {
                    Some(s) => ReplOutcome::Output(s),
                    None => ReplOutcome::Output("error: the expression does not elaborate".to_string()),
                }
            }
            ":kind" | ":k" => {
                if arg.is_empty() {
                    return ReplOutcome::Output("usage: :kind <type>".to_string());
                }
                match self.analysis.kind_of(arg) {
                    Some(k) => ReplOutcome::Output(format!("{arg} :: {k}")),
                    None => ReplOutcome::Output(format!("error: no type or class named {arg}")),
                }
            }
            ":browse" | ":b" => {
                let mut modules: Vec<String> = Vec::new();
                if arg.is_empty() {
                    modules.push(SESSION_MODULE.to_string());
                    for m in self.analysis.modules() {
                        if m != SESSION_MODULE {
                            modules.push(m);
                        }
                    }
                } else {
                    modules.push(arg.to_string());
                }
                let mut out: Vec<String> = Vec::new();
                for m in modules {
                    for (n, t) in self.analysis.browse(&m) {
                        out.push(format!("{n} :: {t}"));
                    }
                }
                ReplOutcome::Output(out.join("\n"))
            }
            ":load" | ":l" => {
                if arg.is_empty() {
                    return ReplOutcome::Output("usage: :load <file>".to_string());
                }
                match self.analysis.load_project(arg) {
                    Ok(loaded) => {
                        for m in &loaded {
                            let imp = format!("import {m}");
                            if !self.entries.iter().any(|e| e.trim() == imp) {
                                self.entries.push(imp);
                            }
                        }
                        self.sync_session();
                        let report = self.analysis.update();
                        let errors: Vec<String> = report
                            .diagnostics
                            .iter()
                            .filter(|d| d.severity == analysis::Severity::Error)
                            .map(|d| format!("{}:{}: {}", d.file, d.line, d.message))
                            .collect();
                        if !errors.is_empty() {
                            return ReplOutcome::Output(format!("error: {}", errors.join("\n")));
                        }
                        let mut out = vec![format!(
                            "loaded {}",
                            self.analysis.rel_path(std::path::Path::new(arg))
                        )];
                        for m in &loaded {
                            out.push(format!("rechecked {m}"));
                        }
                        ReplOutcome::Output(out.join("\n"))
                    }
                    Err(e) => ReplOutcome::Output(format!("error: {e}")),
                }
            }
            ":reload" | ":r" => {
                if let Err(e) = self.analysis.reload_from_disk() {
                    return ReplOutcome::Output(format!("error: {e}"));
                }
                let report = self.analysis.update();
                let mut out: Vec<String> = Vec::new();
                for m in &report.rechecked {
                    out.push(format!("rechecked {m}"));
                }
                for d in report.diagnostics.iter() {
                    if d.severity == analysis::Severity::Error {
                        out.push(format!("error: {}:{}: {}", d.file, d.line, d.message));
                    }
                }
                if out.is_empty() {
                    out.push("ok".to_string());
                }
                ReplOutcome::Output(out.join("\n"))
            }
            ":history" | ":hist" => {
                let out: Vec<String> = self
                    .history
                    .iter()
                    .enumerate()
                    .map(|(i, h)| format!("{}: {h}", i + 1))
                    .collect();
                ReplOutcome::Output(out.join("\n"))
            }
            ":help" | ":h" | "?" => ReplOutcome::Output(
                ":type <expr>, :kind <type>, :browse [module], :load <file>, :reload, :history, :quit"
                    .to_string(),
            ),
            ":quit" | ":q" => ReplOutcome::Quit,
            other => ReplOutcome::Output(format!("error: unknown command {other}")),
        }
    }
}

// ------------------------------------------------------- helpers

/// The session source: one module whose body is every entry.
fn session_source(entries: &[String]) -> String {
    let mut out = format!("module {SESSION_MODULE} where\n");
    for e in entries {
        for line in e.lines() {
            out.push_str("  ");
            out.push_str(line);
            out.push('\n');
        }
        if e.lines().next().is_none() {
            out.push('\n');
        }
    }
    out
}

/// The equations of a declaration block: `(name, equation)`.
fn equations_of<'a>(decls: &'a [ast::Decl]) -> Vec<(&'a String, &'a ast::Equation)> {
    decls
        .iter()
        .filter_map(|d| match d {
            ast::Decl::Equation { eq, .. } => Some((&eq.name.text, eq)),
            _ => None,
        })
        .collect()
}

/// A session diagnostic, mapped back through the entries to the input
/// that produced it (line and column in that input's coordinates).
fn render_session_diag(
    diag: &analysis::Diag,
    entries: &[String],
    skips: &[u32],
) -> String {
    // The session source is the header line plus every indented entry
    // line; walk the entries to find the input the span belongs to.
    let mut base = 2u32; // first entry line
    let mut which = 0usize;
    let mut within = diag.line;
    for (i, e) in entries.iter().enumerate() {
        let lines = e.lines().count().max(1) as u32;
        if diag.line < base + lines || i + 1 == entries.len() {
            which = i;
            within = diag.line - base + 1;
            break;
        }
        base += lines;
    }
    let entry = entries.get(which).map(|s| s.as_str()).unwrap_or("");
    let skip = skips.get(which).copied().unwrap_or(0);
    let within = within.saturating_sub(skip);
    let col = diag.col.saturating_sub(2);
    let src = entry
        .lines()
        .nth(within.saturating_sub(1) as usize)
        .unwrap_or("");
    format!(
        "error: {}\n  at {}:{}\n  {}\n  {}^",
        diag.message,
        within,
        col,
        src,
        " ".repeat(col.saturating_sub(1) as usize),
    )
}

/// A declaration's start span (its first line).
fn decl_span(d: &ast::Decl) -> Span {
    match d {
        ast::Decl::ValueSig { span, .. } => *span,
        ast::Decl::Equation { eq, .. } => eq.span,
        ast::Decl::Data(t) => t.span,
        ast::Decl::Record(r) | ast::Decl::Codata(r) => r.span,
        ast::Decl::Signature(s) => s.span,
        ast::Decl::Instance(i) => i.span,
        ast::Decl::Foreign(f) => f.span,
        ast::Decl::Prim(p) => p.span,
        ast::Decl::OpaqueType(o) => o.span,
        ast::Decl::Effect(e) => e.span,
        ast::Decl::Import(i) => i.span,
        ast::Decl::Attributed { inner, .. } => decl_span(inner),
        _ => Span { line: 1, col: 1 },
    }
}

/// Is the block unfinished: an open delimiter, a trailing operator or
/// keyword, or a literal cut off at the end?
fn is_incomplete(block: &str) -> bool {
    let tokens = match lexer::lex(block) {
        Ok(t) => t,
        Err(e) => {
            // A lexical error at the block's last line is an unfinished
            // literal; anywhere else is a real error.
            let last = block.lines().count() as u32;
            return e.span.map(|s| s.line >= last.max(1)).unwrap_or(false);
        }
    };
    let tokens = match crate::layout::layout(tokens) {
        Ok(t) => t,
        Err(_) => return false,
    };
    let mut depth = 0i64;
    for t in &tokens {
        if t.virtual_ {
            continue;
        }
        match t.tok {
            Tok::LParen | Tok::LBracket | Tok::LBrace | Tok::LQuote | Tok::DollarParen => depth += 1,
            Tok::RParen | Tok::RBracket | Tok::RBrace | Tok::RQuote => depth -= 1,
            _ => {}
        }
    }
    if depth > 0 {
        return true;
    }
    let last = tokens
        .iter()
        .rev()
        .find(|t| t.tok != Tok::Eof && !t.virtual_)
        .map(|t| &t.tok);
    match last {
        None => false,
        Some(t) => matches!(
            t,
            Tok::Where
                | Tok::Of
                | Tok::Do
                | Tok::Then
                | Tok::Else
                | Tok::In
                | Tok::Equals
                | Tok::Arrow
                | Tok::Darrow
                | Tok::ColonColon
                | Tok::LArrow
                | Tok::Comma
                | Tok::Pipe
                | Tok::Op(_)
                | Tok::LParen
                | Tok::LBracket
                | Tok::LBrace
                | Tok::LQuote
                | Tok::DollarParen
                | Tok::Forall
                | Tok::Using
                | Tok::Let
                | Tok::Letrec
                | Tok::If
                | Tok::Case
                | Tok::Handle
                | Tok::With
                | Tok::Signature
                | Tok::Data
                | Tok::Record
                | Tok::Codata
                | Tok::Effect
                | Tok::Instance
                | Tok::Import
                | Tok::Export
                | Tok::TypeKw
                | Tok::Opaque
        ),
    }
}

/// An error with its source span, in block coordinates: the offending
/// line and a caret under the column.
fn render_error(message: &str, block: &str, at: (u32, u32)) -> String {
    let (line, col) = at;
    let mut out = format!("error: {message}\n  at {line}:{col}");
    if let Some(src) = block.lines().nth(line.saturating_sub(1) as usize) {
        out.push('\n');
        out.push_str("  ");
        out.push_str(src);
        out.push('\n');
        out.push_str("  ");
        for _ in 1..col {
            out.push(' ');
        }
        out.push('^');
    }
    out
}

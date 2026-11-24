//! Standalone Hackage + Hoogle static HTML generator for `cinch doc html`.
//!
//! Produces a self-contained directory of HTML pages (`index.html`, per-module
//! API pages, line-numbered highlighted source pages, `search-index.json`,
//! `style.css`, and `search.js`) styled in Cinch's warm paper/ink/olive
//! manual palette with built-in name and type-signature search.

use crate::cinchdoc::{flatten, iter_sources, parse_source, Decl, Doc, ModuleDoc};
use crate::doc_search::{pretty_signature, short_name};
use crate::highlight;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

/// Escape HTML text and attribute values.
fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Readable, collision-free Haddock-style anchor for a declaration.
pub fn decl_anchor(kind: &str, name: &str) -> String {
    let prefix = match kind {
        "data" | "record" | "codata" | "type" => "t",
        "signature" => "c",
        "constructor" => "k",
        "field" => "f",
        "method" => "m",
        "instance" => "i",
        _ => "v",
    };
    if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return format!("{prefix}-{name}");
    }
    if let Some((head, tail)) = name.split_once('-') {
        if !head.is_empty()
            && head.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            && !tail.is_empty()
            && !tail.chars().any(|c| c.is_ascii_alphanumeric())
        {
            let hex: String = tail.as_bytes().iter().map(|b| format!("{b:02x}")).collect();
            return format!("{prefix}-{head}-op-{hex}");
        }
    }
    if !name.chars().any(|c| c.is_ascii_alphanumeric()) {
        let hex: String = name.as_bytes().iter().map(|b| format!("{b:02x}")).collect();
        return format!("{prefix}-op-{hex}");
    }
    let mut safe = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            safe.push(ch);
        } else {
            let mut buf = [0u8; 4];
            let bytes = ch.encode_utf8(&mut buf).as_bytes();
            safe.push('_');
            for b in bytes {
                safe.push_str(&format!("{b:02x}"));
            }
            safe.push('_');
        }
    }
    format!("{prefix}-{}", safe.trim_matches('_'))
}

/// Convert a highlighted `<pre class="cinch-highlight">...</pre>` or fragment
/// by linking type/constructor tokens to known definitions in `type_targets`.
fn link_highlighted_code(code_text: &str, type_targets: &HashMap<String, (String, String)>) -> String {
    let spans = highlight::token_spans(code_text);
    let mut out = String::new();
    let mut cursor = 0usize;
    let bytes = code_text.as_bytes();
    let mut idx = 0usize;

    while idx < spans.len() {
        let (start, end, class) = spans[idx];
        if start > cursor {
            out.push_str(&esc(&code_text[cursor..start]));
        }
        let raw = &code_text[start..end];
        if matches!(class, "constructor" | "type") {
            let short = short_name(raw);
            if let Some((href, full_id)) = type_targets.get(raw).or_else(|| type_targets.get(short)) {
                out.push_str(&format!(
                    "<a class=\"doc-type-link tok-{}\" href=\"{}\" title=\"{}\">{}</a>",
                    class,
                    esc(href),
                    esc(full_id),
                    esc(raw)
                ));
            } else {
                out.push_str(&format!(
                    "<span class=\"tok-{}\">{}</span>",
                    class,
                    esc(raw)
                ));
            }
        } else {
            out.push_str(&format!(
                "<span class=\"tok-{}\">{}</span>",
                class,
                esc(raw)
            ));
        }
        cursor = end;
        idx += 1;
    }
    if cursor < bytes.len() {
        out.push_str(&esc(&code_text[cursor..]));
    }
    out
}

/// Minimal, safe Markdown-to-HTML renderer for doc comment bodies.
fn render_doc_markdown(
    markdown: &str,
    symbol_targets: &HashMap<String, String>,
) -> String {
    let mut html = String::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut i = 0usize;
    let mut para: Vec<String> = Vec::new();
    let mut in_list = false;

    let flush_para = |para: &mut Vec<String>, html: &mut String, symbol_targets: &HashMap<String, String>| {
        if !para.is_empty() {
            let joined = para.join(" ");
            html.push_str("<p>");
            html.push_str(&render_inline(&joined, symbol_targets));
            html.push_str("</p>\n");
            para.clear();
        }
    };

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Fenced code block
        if let Some(rest) = trimmed.strip_prefix("```") {
            flush_para(&mut para, &mut html, symbol_targets);
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            let lang = rest.trim();
            let mut body = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                body.push(lines[i]);
                i += 1;
            }
            let code_str = body.join("\n");
            if lang == "cinch" {
                html.push_str(&highlight::html(&code_str));
            } else {
                html.push_str(&format!(
                    "<pre class=\"doc-code\"><code>{}</code></pre>\n",
                    esc(&code_str)
                ));
            }
            i += 1;
            continue;
        }

        if trimmed.is_empty() {
            flush_para(&mut para, &mut html, symbol_targets);
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            i += 1;
            continue;
        }

        if let Some(heading) = trimmed.strip_prefix("### ") {
            flush_para(&mut para, &mut html, symbol_targets);
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            html.push_str(&format!(
                "<h5>{}</h5>\n",
                render_inline(heading, symbol_targets)
            ));
            i += 1;
            continue;
        }
        if let Some(heading) = trimmed.strip_prefix("## ").or_else(|| trimmed.strip_prefix("# ")) {
            flush_para(&mut para, &mut html, symbol_targets);
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            html.push_str(&format!(
                "<h4>{}</h4>\n",
                render_inline(heading, symbol_targets)
            ));
            i += 1;
            continue;
        }

        if let Some(item) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            flush_para(&mut para, &mut html, symbol_targets);
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            html.push_str(&format!(
                "<li>{}</li>\n",
                render_inline(item, symbol_targets)
            ));
            i += 1;
            continue;
        }

        if in_list {
            html.push_str("</ul>\n");
            in_list = false;
        }
        para.push(trimmed.to_string());
        i += 1;
    }

    flush_para(&mut para, &mut html, symbol_targets);
    if in_list {
        html.push_str("</ul>\n");
    }
    html
}

fn render_inline(text: &str, symbol_targets: &HashMap<String, String>) -> String {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0usize;
    let mut out = String::new();

    while i < len {
        // Inline code: `...`
        if chars[i] == '`' {
            let start = i + 1;
            let mut end = start;
            while end < len && chars[end] != '`' {
                end += 1;
            }
            if end < len {
                let code: String = chars[start..end].iter().collect();
                out.push_str("<code>");
                out.push_str(&esc(&code));
                out.push_str("</code>");
                i = end + 1;
                continue;
            }
        }
        // Markdown link: [label](target)
        if chars[i] == '[' {
            let mut close_bracket = i + 1;
            while close_bracket < len && chars[close_bracket] != ']' {
                close_bracket += 1;
            }
            if close_bracket + 1 < len && chars[close_bracket + 1] == '(' {
                let mut close_paren = close_bracket + 2;
                while close_paren < len && chars[close_paren] != ')' {
                    close_paren += 1;
                }
                if close_paren < len {
                    let label: String = chars[i + 1..close_bracket].iter().collect();
                    let target: String = chars[close_bracket + 2..close_paren].iter().collect();
                    let label_html = render_inline(&label, symbol_targets);
                    if let Some(sym) = target.strip_prefix("symbol:") {
                        if let Some(href) = symbol_targets.get(sym) {
                            out.push_str(&format!("<a href=\"{}\">{}</a>", esc(href), label_html));
                        } else {
                            out.push_str(&label_html);
                        }
                    } else if target.starts_with("http://")
                        || target.starts_with("https://")
                        || target.starts_with('#')
                        || target.ends_with(".html")
                    {
                        out.push_str(&format!("<a href=\"{}\">{}</a>", esc(&target), label_html));
                    } else {
                        out.push_str(&label_html);
                    }
                    i = close_paren + 1;
                    continue;
                }
            }
        }
        // Bold: **...**
        if i + 1 < len && chars[i] == '*' && chars[i + 1] == '*' {
            let start = i + 2;
            let mut end = start;
            while end + 1 < len && !(chars[end] == '*' && chars[end + 1] == '*') {
                end += 1;
            }
            if end + 1 < len {
                let inner: String = chars[start..end].iter().collect();
                out.push_str("<strong>");
                out.push_str(&esc(&inner));
                out.push_str("</strong>");
                i = end + 2;
                continue;
            }
        }
        out.push_str(&esc(&chars[i].to_string()));
        i += 1;
    }
    out
}

fn render_doc_tags(doc: &Doc, symbol_targets: &HashMap<String, String>) -> String {
    let mut out = String::new();
    for (key, label) in [
        ("params", "Parameters"),
        ("typeParams", "Type parameters"),
        ("fields", "Fields"),
    ] {
        if let Some(items) = doc.tags.get(key).and_then(Value::as_array) {
            if !items.is_empty() {
                out.push_str(&format!(
                    "<div class=\"doc-meta-block\"><h4>{label}</h4><dl class=\"doc-params\">"
                ));
                for item in items {
                    let name = item.get("name").and_then(Value::as_str).unwrap_or("");
                    let text = item.get("text").and_then(Value::as_str).unwrap_or("");
                    out.push_str(&format!(
                        "<dt><code>{}</code></dt><dd>{}</dd>",
                        esc(name),
                        render_inline(text, symbol_targets)
                    ));
                }
                out.push_str("</dl></div>\n");
            }
        }
    }
    for (key, label) in [
        ("returns", "Returns"),
        ("safety", "Safety"),
        ("errors", "Errors"),
    ] {
        if let Some(v) = doc.tags.get(key).and_then(Value::as_str) {
            out.push_str(&format!(
                "<p class=\"doc-tag-line\"><strong>{label}:</strong> {}</p>\n",
                render_inline(v, symbol_targets)
            ));
        }
    }
    for (key, label) in [("laws", "Laws"), ("effects", "Effects")] {
        if let Some(items) = doc.tags.get(key).and_then(Value::as_array) {
            if !items.is_empty() {
                out.push_str(&format!("<div class=\"doc-meta-block\"><h4>{label}</h4><ul>"));
                for item in items {
                    if let Some(text) = item.as_str() {
                        out.push_str(&format!(
                            "<li>{}</li>",
                            render_inline(text, symbol_targets)
                        ));
                    }
                }
                out.push_str("</ul></div>\n");
            }
        }
    }
    let mut pills = Vec::new();
    if let Some(v) = doc.tags.get("since").and_then(Value::as_str) {
        pills.push(format!("<span class=\"doc-pill\">Since {}</span>", esc(v)));
    }
    if let Some(v) = doc.tags.get("deprecated").and_then(Value::as_str) {
        pills.push(format!(
            "<span class=\"doc-pill doc-pill-warn\">Deprecated: {}</span>",
            esc(v)
        ));
    }
    if let Some(v) = doc.tags.get("experimental").and_then(Value::as_str) {
        pills.push(format!(
            "<span class=\"doc-pill doc-pill-warn\">Experimental: {}</span>",
            esc(v)
        ));
    }
    if !pills.is_empty() {
        out.push_str(&format!(
            "<div class=\"doc-pills\">{}</div>\n",
            pills.join("")
        ));
    }
    out
}

fn page_shell(title: &str, root_rel: &str, body: &str, nav_links: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
  <link rel="stylesheet" href="{root_rel}style.css" />
  <script>
    (function() {{
      var t = localStorage.getItem('cinchdoc-theme');
      if (t === 'light' || t === 'dark') document.documentElement.dataset.theme = t;
    }})();
  </script>
</head>
<body>
  <header class="topbar">
    <div class="topbar-inner">
      <a class="brand" href="{root_rel}index.html">Cinch Docs</a>
      <nav class="topnav">{nav_links}</nav>
      <div class="topbar-actions">
        <a class="search-jump" href="{root_rel}index.html#search" title="Press / to search">Search <kbd>/</kbd></a>
        <button type="button" class="theme-toggle" id="theme-toggle" aria-label="Toggle theme">Theme</button>
      </div>
    </div>
  </header>
  <main class="container">
    {body}
  </main>
  <footer class="footer">
    <span>Generated by <code>cinch doc html</code></span>
  </footer>
  <script>
    (function() {{
      var btn = document.getElementById('theme-toggle');
      if (btn) btn.addEventListener('click', function() {{
        var cur = document.documentElement.dataset.theme || (matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark');
        var next = cur === 'light' ? 'dark' : 'light';
        document.documentElement.dataset.theme = next;
        localStorage.setItem('cinchdoc-theme', next);
      }});
      document.addEventListener('keydown', function(e) {{
        if (e.key === '/' && document.activeElement && !/INPUT|TEXTAREA/.test(document.activeElement.tagName)) {{
          var input = document.getElementById('hoogle-input');
          if (input) {{ e.preventDefault(); input.focus(); input.select(); }}
        }}
      }});
    }})();
  </script>
</body>
</html>
"#,
        title = esc(title),
        root_rel = root_rel,
        nav_links = nav_links,
        body = body,
    )
}

fn render_source_page(module: &ModuleDoc, source_text: &str) -> String {
    let mut rows = String::new();
    for (idx, line) in source_text.lines().enumerate() {
        let num = idx + 1;
        let highlighted = if line.is_empty() {
            " ".to_string()
        } else {
            highlight::fragment(line)
        };
        rows.push_str(&format!(
            "<div class=\"src-line\" id=\"L{num}\"><a class=\"src-num\" href=\"#L{num}\">{num}</a><span class=\"src-code\">{highlighted}</span></div>\n"
        ));
    }
    let body = format!(
        r#"<div class="crumbs"><a href="../index.html">Modules</a> / <a href="../{mod_name}.html">{mod_name}</a> / <span>Source</span></div>
<div class="module-head">
  <h1>{mod_name} <span class="muted">source</span></h1>
  <p class="meta"><code>{src_path}</code></p>
</div>
<div class="source-well">{rows}</div>"#,
        mod_name = esc(&module.module),
        src_path = esc(&module.source),
        rows = rows,
    );
    let nav = format!(
        "<a href=\"../index.html\">Modules</a><a href=\"../{m}.html\">{m}</a>",
        m = esc(&module.module)
    );
    page_shell(&format!("{} · Source", module.module), "../", &body, &nav)
}

fn render_module_page(
    module: &ModuleDoc,
    all_modules: &[ModuleDoc],
    type_targets: &HashMap<String, (String, String)>,
    symbol_targets: &HashMap<String, String>,
    instances_by_symbol: &HashMap<String, Vec<(String, String, String)>>,
) -> String {
    let mut types = Vec::new();
    let mut signatures = Vec::new();
    let mut values = Vec::new();
    let mut instances = Vec::new();
    let mut fixities = Vec::new();

    for decl in &module.declarations {
        match decl.kind.as_str() {
            "data" | "record" | "codata" | "type" => types.push(decl),
            "signature" => signatures.push(decl),
            "instance" => instances.push(decl),
            "fixity" => fixities.push(decl),
            _ => values.push(decl),
        }
    }

    // Build collapsible Haddock-style Synopsis
    let mut synopsis_rows = String::new();
    for decl in module.declarations.iter() {
        if decl.kind == "instance" || decl.kind == "fixity" {
            continue;
        }
        let anchor = decl_anchor(&decl.kind, &decl.name);
        let pretty = pretty_signature(&decl.signature, true);
        let linked = link_highlighted_code(&pretty, type_targets);
        synopsis_rows.push_str(&format!(
            "<li><a class=\"syn-jump\" href=\"#{anchor}\">{}</a> <span class=\"syn-sig\">{linked}</span></li>\n",
            esc(&decl.name)
        ));
    }

    let mut sections = String::new();

    // Module doc
    if let Some(doc) = &module.doc {
        sections.push_str("<section class=\"module-doc\">");
        sections.push_str(&render_doc_markdown(&doc.markdown, symbol_targets));
        sections.push_str(&render_doc_tags(doc, symbol_targets));
        sections.push_str("</section>\n");
    }

    if !synopsis_rows.is_empty() {
        sections.push_str(&format!(
            "<details class=\"synopsis\" open><summary>Synopsis ({} declarations)</summary><ul class=\"synopsis-list\">{synopsis_rows}</ul></details>\n",
            types.len() + signatures.len() + values.len()
        ));
    }

    let render_entry = |decl: &Decl, extra_children_html: &str, inst_key: Option<&str>| -> String {
        let anchor = decl_anchor(&decl.kind, &decl.name);
        let pretty = pretty_signature(&decl.signature, true);
        let sig_html = link_highlighted_code(&pretty, type_targets);
        let src_href = format!("source/{}.html#L{}", esc(&module.module), decl.line);
        let mut body_html = String::new();
        if let Some(doc) = &decl.doc {
            body_html.push_str(&render_doc_markdown(&doc.markdown, symbol_targets));
            body_html.push_str(&render_doc_tags(doc, symbol_targets));
        }
        let mut inst_html = String::new();
        if let Some(key) = inst_key {
            if let Some(insts) = instances_by_symbol.get(key) {
                if !insts.is_empty() {
                    let mut rows = String::new();
                    for (inst_mod, inst_sig, inst_href) in insts {
                        let linked = link_highlighted_code(inst_sig, type_targets);
                        rows.push_str(&format!(
                            "<li><code class=\"inst-sig\">{linked}</code> <a class=\"inst-mod\" href=\"{}\">{}</a></li>\n",
                            esc(inst_href),
                            esc(inst_mod)
                        ));
                    }
                    inst_html = format!(
                        "<details class=\"doc-instances\"><summary>Instances ({})</summary><ul>{rows}</ul></details>\n",
                        insts.len()
                    );
                }
            }
        }
        format!(
            r##"<article class="doc-entry" id="{anchor}">
  <div class="doc-sig-bar">
    <code class="doc-sig">{sig_html}</code>
    <span class="doc-sig-links"><a href="#{anchor}" title="Permalink">#</a><a href="{src_href}" title="View source at line {line}">Source</a></span>
  </div>
  <div class="doc-body">{body_html}{extra_children_html}{inst_html}</div>
</article>
"##,
            anchor = anchor,
            sig_html = sig_html,
            src_href = src_href,
            line = decl.line,
            body_html = body_html,
            extra_children_html = extra_children_html,
            inst_html = inst_html,
        )
    };

    if !types.is_empty() {
        sections.push_str("<section class=\"doc-section\" id=\"sec-types\"><h2>Types</h2>\n");
        for ty in &types {
            let mut children_html = String::new();
            if !ty.children.is_empty() {
                let heading = if ty.kind == "record" { "Fields" } else { "Constructors" };
                children_html.push_str(&format!("<div class=\"doc-children\"><h4>{heading}</h4><ul>"));
                for child in &ty.children {
                    let c_anchor = decl_anchor(&child.kind, &child.name);
                    let c_sig = link_highlighted_code(&pretty_signature(&child.signature, true), type_targets);
                    let c_doc = child
                        .doc
                        .as_ref()
                        .map(|d| render_doc_markdown(&d.markdown, symbol_targets))
                        .unwrap_or_default();
                    children_html.push_str(&format!(
                        "<li id=\"{c_anchor}\"><code class=\"child-sig\">{c_sig}</code><div class=\"child-doc\">{c_doc}</div></li>"
                    ));
                }
                children_html.push_str("</ul></div>\n");
            }
            sections.push_str(&render_entry(ty, &children_html, Some(&ty.name)));
        }
        sections.push_str("</section>\n");
    }

    if !signatures.is_empty() {
        sections.push_str("<section class=\"doc-section\" id=\"sec-signatures\"><h2>Signatures</h2>\n");
        for sig in &signatures {
            let mut children_html = String::new();
            if !sig.children.is_empty() {
                children_html.push_str("<div class=\"doc-children\"><h4>Methods</h4><ul>");
                for child in &sig.children {
                    let c_anchor = decl_anchor("method", &format!("{}-{}", sig.name, child.name));
                    let c_sig = link_highlighted_code(&pretty_signature(&child.signature, true), type_targets);
                    let c_doc = child
                        .doc
                        .as_ref()
                        .map(|d| render_doc_markdown(&d.markdown, symbol_targets))
                        .unwrap_or_default();
                    children_html.push_str(&format!(
                        "<li id=\"{c_anchor}\"><code class=\"child-sig\">{c_sig}</code><div class=\"child-doc\">{c_doc}</div></li>"
                    ));
                }
                children_html.push_str("</ul></div>\n");
            }
            sections.push_str(&render_entry(sig, &children_html, Some(&sig.name)));
        }
        sections.push_str("</section>\n");
    }

    if !values.is_empty() {
        sections.push_str("<section class=\"doc-section\" id=\"sec-values\"><h2>Values</h2>\n");
        for val in &values {
            sections.push_str(&render_entry(val, "", None));
        }
        sections.push_str("</section>\n");
    }

    if !fixities.is_empty() {
        sections.push_str("<section class=\"doc-section\" id=\"sec-fixities\"><h2>Operator fixities</h2><pre class=\"cinch-highlight\"><code class=\"language-cinch\">");
        let lines: Vec<&str> = fixities.iter().map(|f| f.signature.as_str()).collect();
        sections.push_str(&highlight::fragment(&lines.join("\n")));
        sections.push_str("</code></pre></section>\n");
    }

    if !instances.is_empty() {
        sections.push_str("<section class=\"doc-section\" id=\"sec-instances\"><h2>Instances</h2>\n");
        for inst in &instances {
            sections.push_str(&render_entry(inst, "", None));
        }
        sections.push_str("</section>\n");
    }

    // Sidebar list of sibling modules
    let mut sidebar = String::from("<aside class=\"mod-sidebar\"><h3>Modules</h3><ul>");
    for m in all_modules {
        let current = if m.module == module.module {
            " aria-current=\"page\""
        } else {
            ""
        };
        sidebar.push_str(&format!(
            "<li><a href=\"{m}.html\"{current}>{m}</a></li>",
            m = esc(&m.module)
        ));
    }
    sidebar.push_str("</ul></aside>");

    let body = format!(
        r#"<div class="doc-layout">
  {sidebar}
  <div class="doc-main">
    <div class="crumbs"><a href="index.html">Modules</a> / <span>{mod_name}</span></div>
    <div class="module-head">
      <h1>{mod_name}</h1>
      <div class="module-meta">
        <code>import {mod_name}</code>
        <a href="source/{mod_name}.html">View source ↗</a>
      </div>
    </div>
    {sections}
  </div>
</div>"#,
        sidebar = sidebar,
        mod_name = esc(&module.module),
        sections = sections,
    );

    let nav = "<a href=\"index.html\">Modules &amp; Search</a>";
    page_shell(&format!("{} · Cinch Docs", module.module), "", &body, nav)
}

fn render_index_page(title: &str, modules: &[ModuleDoc], total_symbols: usize) -> String {
    let mut groups: BTreeMap<String, Vec<&ModuleDoc>> = BTreeMap::new();
    for m in modules {
        let family = m.module.split('.').next().unwrap_or(&m.module).to_string();
        groups.entry(family).or_default().push(m);
    }

    let mut dir_html = String::new();
    for (family, mods) in &groups {
        dir_html.push_str(&format!("<section class=\"family-group\"><h3>{}</h3><ul class=\"module-list\">", esc(family)));
        for m in mods {
            let summary = m
                .doc
                .as_ref()
                .map(|d| d.summary.as_str())
                .unwrap_or("");
            dir_html.push_str(&format!(
                "<li><a href=\"{name}.html\"><code>{name}</code></a><span>{}</span></li>\n",
                esc(summary),
                name = esc(&m.module)
            ));
        }
        dir_html.push_str("</ul></section>\n");
    }

    let body = format!(
        r#"<section class="hero-search" id="search">
  <h1>{title}</h1>
  <p class="hero-sub">{mod_count} modules · {sym_count} indexed declarations. Search by name, type signature (<code>(a -&gt; b) -&gt; List a -&gt; List b</code>), module (<code>+Data.List</code>), or kind (<code>is:type</code>).</p>
  <div class="search-box">
    <input id="hoogle-input" type="search" placeholder="Search by name or type signature, e.g. Result e a -> Option a" autocomplete="off" />
    <button type="button" id="hoogle-clear" aria-label="Clear">×</button>
  </div>
  <div class="search-status" id="hoogle-status"></div>
  <div class="search-results" id="hoogle-results"></div>
</section>
<section class="module-directory">
  <h2>Module directory</h2>
  {dir_html}
</section>
<script src="search.js"></script>"#,
        title = esc(title),
        mod_count = modules.len(),
        sym_count = total_symbols,
        dir_html = dir_html,
    );

    page_shell(title, "", &body, "<a href=\"index.html\" aria-current=\"page\">Modules &amp; Search</a>")
}

/// Generate a standalone Hackage + Hoogle HTML site from `root` into `out_dir`.
pub fn generate_html_site(
    root: &Path,
    out_dir: &Path,
    iface_dir: Option<&Path>,
    legacy: bool,
    title: &str,
) -> Result<(usize, usize), String> {
    let source_paths = iter_sources(root);
    if source_paths.is_empty() {
        return Err(format!("no .cinch sources found under {}", root.display()));
    }

    let mut modules = Vec::new();
    let mut raw_sources: HashMap<String, String> = HashMap::new();

    for path in source_paths {
        let text = fs::read_to_string(&path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        // Load matching `.cinchif` if `--iface-dir` was given
        let provisional = parse_source(&text, None, &path.to_string_lossy(), None, legacy)?;
        let iface_val: Option<Value> = iface_dir.and_then(|dir| {
            let candidate = dir.join(format!("{}.cinchif", provisional.module));
            fs::read_to_string(candidate)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
        });
        let parsed = if let Some(ref iface) = iface_val {
            parse_source(&text, None, &path.to_string_lossy(), Some(iface), legacy)?
        } else {
            provisional
        };
        raw_sources.insert(parsed.module.clone(), text);
        modules.push(parsed);
    }

    modules.sort_by(|a, b| a.module.cmp(&b.module));

    // Build global type/symbol link tables and cross-module instance index
    let mut type_targets: HashMap<String, (String, String)> = HashMap::new();
    let mut symbol_targets: HashMap<String, String> = HashMap::new();
    let mut instances_by_symbol: HashMap<String, Vec<(String, String, String)>> = HashMap::new();
    let mut search_records = Vec::new();

    for m in &modules {
        let mut flat = Vec::new();
        flatten(&m.declarations, &mut flat);
        for decl in flat {
            let anchor = decl_anchor(&decl.kind, &decl.name);
            let href = format!("{}.html#{anchor}", m.module);
            symbol_targets.insert(decl.id.clone(), href.clone());
            symbol_targets
                .entry(decl.name.clone())
                .or_insert_with(|| href.clone());

            if matches!(decl.kind.as_str(), "data" | "record" | "codata" | "type" | "signature") {
                type_targets.insert(decl.id.clone(), (href.clone(), decl.id.clone()));
                type_targets
                    .entry(decl.name.clone())
                    .or_insert_with(|| (href.clone(), decl.id.clone()));
            }

            if decl.kind == "instance" {
                let pretty = pretty_signature(&decl.signature, true);
                for word in pretty.split(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '.' && c != '-') {
                    if word.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                        let s = short_name(word).to_string();
                        instances_by_symbol.entry(s).or_default().push((
                            m.module.clone(),
                            pretty.clone(),
                            href.clone(),
                        ));
                    }
                }
            }

            let pretty = pretty_signature(&decl.signature, true);
            search_records.push(json!({
                "id": decl.id,
                "module": m.module,
                "kind": decl.kind,
                "name": decl.name,
                "signature": decl.signature,
                "pretty": pretty,
                "summary": decl.doc.as_ref().map(|d| d.summary.as_str()).unwrap_or(""),
                "href": href,
                "sourceHref": format!("source/{}.html#L{}", m.module, decl.line),
                "line": decl.line,
            }));
        }
    }

    let source_out = out_dir.join("source");
    fs::create_dir_all(&source_out)
        .map_err(|e| format!("{}: {e}", source_out.display()))?;

    fs::write(out_dir.join("style.css"), STYLE_CSS)
        .map_err(|e| format!("write style.css: {e}"))?;
    fs::write(out_dir.join("search.js"), SEARCH_JS)
        .map_err(|e| format!("write search.js: {e}"))?;
    fs::write(
        out_dir.join("search-index.json"),
        serde_json::to_string(&json!({"schema": 1, "entries": search_records})).unwrap(),
    )
    .map_err(|e| format!("write search-index.json: {e}"))?;

    for m in &modules {
        let html = render_module_page(
            m,
            &modules,
            &type_targets,
            &symbol_targets,
            &instances_by_symbol,
        );
        let mod_path: PathBuf = out_dir.join(format!("{}.html", m.module));
        fs::write(&mod_path, html)
            .map_err(|e| format!("{}: {e}", mod_path.display()))?;

        if let Some(src_text) = raw_sources.get(&m.module) {
            let src_html = render_source_page(m, src_text);
            let src_path = source_out.join(format!("{}.html", m.module));
            fs::write(&src_path, src_html)
                .map_err(|e| format!("{}: {e}", src_path.display()))?;
        }
    }

    let index_html = render_index_page(title, &modules, search_records.len());
    fs::write(out_dir.join("index.html"), index_html)
        .map_err(|e| format!("write index.html: {e}"))?;

    Ok((modules.len(), search_records.len()))
}

const STYLE_CSS: &str = r#"
:root {
  --paper: #20251c;
  --panel: #2a3023;
  --ink: #eee6d5;
  --muted: #b1b29b;
  --rule: #565d47;
  --accent: #efad78;
  --olive: #c0c995;
  --tok-kw: #f0ac78;
  --tok-type: #e4c58f;
  --tok-str: #bacb9b;
  --tok-num: #f1ce94;
  --tok-com: #9ca58b;
  --font-sans: 'DM Sans', system-ui, -apple-system, sans-serif;
  --font-mono: 'IBM Plex Mono', ui-monospace, SFMono-Regular, Menlo, monospace;
}
:root[data-theme="light"] {
  --paper: #f2ecde;
  --panel: #e7deca;
  --ink: #272b21;
  --muted: #6e6450;
  --rule: #c4baa1;
  --accent: #98532d;
  --olive: #4b5c2b;
  --tok-kw: #924218;
  --tok-type: #79551e;
  --tok-str: #4b5f2a;
  --tok-num: #83471c;
  --tok-com: #626750;
}
* { box-sizing: border-box; }
body { margin: 0; background: var(--paper); color: var(--ink); font: 15px/1.65 var(--font-sans); }
a { color: var(--accent); text-decoration: none; }
a:hover { text-decoration: underline; }
code, pre { font-family: var(--font-mono); }
.topbar { background: #191c17; color: #eee6d5; border-bottom: 3px double #6f7257; position: sticky; top: 0; z-index: 20; }
.topbar-inner { max-width: 1320px; margin: 0 auto; padding: 0.75rem 1.5rem; display: flex; align-items: center; gap: 1.5rem; justify-content: space-between; }
.brand { font-weight: 700; color: #ff9a5a; font-size: 1.05rem; letter-spacing: -0.01em; }
.topnav { display: flex; gap: 1rem; font-size: 0.88rem; }
.topnav a { color: #d5ceb7; }
.topbar-actions { display: flex; align-items: center; gap: 0.75rem; }
.search-jump { font: 0.8rem var(--font-mono); color: #dfd7bf; border: 1px solid #686b51; border-radius: 999px; padding: 0.25rem 0.75rem; }
.search-jump kbd { background: #2d3224; padding: 0.05rem 0.35rem; border-radius: 3px; font-size: 0.75rem; }
.theme-toggle { background: #2a3023; color: #eee6d5; border: 1px solid #686b51; border-radius: 999px; padding: 0.28rem 0.7rem; cursor: pointer; font: 0.78rem var(--font-mono); }
.container { max-width: 1320px; margin: 0 auto; padding: 1.75rem 1.5rem 3.5rem; }
.doc-layout { display: grid; grid-template-columns: 220px minmax(0, 1fr); gap: 2rem; align-items: start; }
.mod-sidebar { position: sticky; top: 4.5rem; max-height: calc(100vh - 6rem); overflow-y: auto; border-right: 1px solid var(--rule); padding-right: 1rem; }
.mod-sidebar h3 { font: 600 0.75rem var(--font-mono); text-transform: uppercase; letter-spacing: 0.06em; color: var(--muted); margin: 0 0 0.75rem; }
.mod-sidebar ul { list-style: none; padding: 0; margin: 0; }
.mod-sidebar li a { display: block; padding: 0.25rem 0.5rem; font: 0.8rem var(--font-mono); color: var(--muted); border-radius: 0 14px 14px 0; }
.mod-sidebar li a[aria-current="page"] { background: var(--panel); color: var(--accent); font-weight: 600; border-left: 3px solid var(--accent); }
.crumbs { font: 0.78rem var(--font-mono); color: var(--muted); margin-bottom: 0.75rem; }
.module-head { border-bottom: 1px solid var(--rule); padding-bottom: 1rem; margin-bottom: 1.5rem; }
.module-head h1 { margin: 0 0 0.5rem; font-size: 2rem; }
.module-meta { display: flex; gap: 1.25rem; align-items: center; font: 0.82rem var(--font-mono); color: var(--muted); }
.synopsis { border: 1px solid var(--rule); border-radius: 4px 18px 4px 4px; background: var(--panel); padding: 0.85rem 1.1rem; margin-bottom: 1.75rem; }
.synopsis summary { cursor: pointer; font: 600 0.82rem var(--font-mono); text-transform: uppercase; letter-spacing: 0.04em; }
.synopsis-list { list-style: none; padding: 0; margin: 0.75rem 0 0; display: grid; gap: 0.3rem; font-size: 0.84rem; }
.synopsis-list li { display: flex; gap: 0.65rem; align-items: baseline; overflow-x: auto; }
.syn-jump { font-family: var(--font-mono); font-weight: 600; white-space: nowrap; }
.syn-sig { font-family: var(--font-mono); color: var(--muted); white-space: nowrap; }
.doc-section { margin-top: 2.25rem; }
.doc-section h2 { font-size: 1.25rem; border-bottom: 1px solid var(--rule); padding-bottom: 0.4rem; margin-bottom: 1rem; }
.doc-entry { margin-bottom: 1.35rem; scroll-margin-top: 4.5rem; }
.doc-sig-bar { display: flex; justify-content: space-between; align-items: baseline; gap: 1rem; background: var(--panel); border: 1px solid var(--rule); border-left: 3px solid var(--accent); border-radius: 3px 12px 3px 3px; padding: 0.55rem 0.9rem; overflow-x: auto; }
.doc-sig { font-size: 0.87rem; line-height: 1.5; }
.doc-sig-links { display: flex; gap: 0.75rem; font: 0.74rem var(--font-mono); white-space: nowrap; }
.doc-body { padding: 0.6rem 0.4rem 0.2rem 1rem; }
.doc-body p { margin: 0.35rem 0 0.65rem; max-width: 78ch; }
.doc-children { margin-top: 0.6rem; }
.doc-children h4, .doc-meta-block h4 { font: 600 0.74rem var(--font-mono); text-transform: uppercase; letter-spacing: 0.05em; color: var(--muted); margin: 0.5rem 0 0.35rem; }
.doc-children ul { list-style: none; padding: 0; margin: 0; border-left: 2px solid var(--rule); padding-left: 0.85rem; }
.doc-children li { margin: 0.45rem 0; }
.child-sig { font-size: 0.85rem; }
.doc-instances { margin-top: 0.65rem; border: 1px solid var(--rule); border-radius: 4px; padding: 0.45rem 0.8rem; background: var(--panel); }
.doc-instances summary { cursor: pointer; font: 600 0.78rem var(--font-mono); color: var(--muted); }
.doc-instances ul { list-style: none; padding: 0; margin: 0.5rem 0 0; display: grid; gap: 0.25rem; font-size: 0.82rem; }
.inst-mod { font: 0.76rem var(--font-mono); margin-left: 0.6rem; color: var(--muted); }
.doc-params { display: grid; grid-template-columns: auto 1fr; gap: 0.25rem 1rem; margin: 0.25rem 0; }
.doc-params dd { margin: 0; }
.doc-pills { display: flex; gap: 0.5rem; flex-wrap: wrap; margin-top: 0.4rem; }
.doc-pill { font: 0.72rem var(--font-mono); padding: 0.12rem 0.5rem; border: 1px solid var(--rule); border-radius: 999px; color: var(--muted); }
.doc-pill-warn { color: var(--accent); border-color: var(--accent); }
.doc-type-link { text-decoration: none; border-bottom: 1px dotted currentColor; }
.doc-type-link:hover { border-bottom-style: solid; text-decoration: none; }
.tok-keyword { color: var(--tok-kw); }
.tok-type, .tok-constructor { color: var(--tok-type); }
.tok-string, .tok-char, .tok-symbol { color: var(--tok-str); }
.tok-number { color: var(--tok-num); }
.tok-comment { color: var(--tok-com); }
.cinch-highlight, .doc-code { background: var(--panel); border: 1px solid var(--rule); border-radius: 4px 14px 4px 4px; padding: 0.85rem 1rem; overflow-x: auto; font-size: 0.84rem; }
.hero-search { background: var(--panel); border: 1px solid var(--rule); border-radius: 6px 36px 6px 6px; padding: 1.75rem; margin-bottom: 2rem; }
.hero-search h1 { margin: 0 0 0.5rem; font-size: 2rem; }
.hero-sub { color: var(--muted); margin: 0 0 1.1rem; }
.search-box { display: flex; align-items: center; border: 1px solid var(--rule); border-radius: 4px 24px 24px 4px; background: var(--paper); padding: 0 0.9rem; }
.search-box input { width: 100%; border: 0; background: transparent; color: var(--ink); font: 1rem var(--font-mono); padding: 0.75rem 0; outline: 0; }
.search-box button { border: 0; background: transparent; color: var(--muted); font-size: 1.3rem; cursor: pointer; }
.search-status { font: 0.78rem var(--font-mono); color: var(--muted); margin-top: 0.65rem; }
.search-results { display: grid; gap: 0.5rem; margin-top: 0.8rem; }
.hit { background: var(--paper); border: 1px solid var(--rule); border-radius: 4px; padding: 0.65rem 0.9rem; }
.hit-head { display: flex; justify-content: space-between; gap: 1rem; align-items: baseline; flex-wrap: wrap; }
.hit-sig { font: 0.86rem var(--font-mono); }
.hit-mod { font: 0.76rem var(--font-mono); color: var(--muted); }
.hit-sum { margin: 0.25rem 0 0; font-size: 0.88rem; color: var(--muted); }
.family-group { margin-bottom: 1.5rem; }
.family-group h3 { font: 600 0.82rem var(--font-mono); text-transform: uppercase; letter-spacing: 0.06em; color: var(--muted); border-bottom: 1px solid var(--rule); padding-bottom: 0.3rem; }
.module-list { list-style: none; padding: 0; margin: 0.5rem 0 0; display: grid; gap: 0.35rem; }
.module-list li { display: grid; grid-template-columns: 240px 1fr; gap: 1rem; align-items: baseline; }
.module-list span { color: var(--muted); font-size: 0.9rem; }
.source-well { background: var(--panel); border: 1px solid var(--rule); border-radius: 4px 16px 4px 4px; padding: 0.75rem 0; overflow-x: auto; font: 0.83rem/1.55 var(--font-mono); }
.src-line { display: grid; grid-template-columns: 56px 1fr; padding: 0 1rem; }
.src-line:target { background: rgba(239, 173, 120, 0.22); }
.src-num { color: var(--muted); user-select: none; text-align: right; padding-right: 1rem; }
.src-code { white-space: pre; }
.footer { border-top: 1px solid var(--rule); padding: 1.25rem 1.5rem; text-align: center; font: 0.78rem var(--font-mono); color: var(--muted); }
@media (max-width: 820px) {
  .doc-layout { grid-template-columns: 1fr; }
  .mod-sidebar { position: static; max-height: none; border-right: 0; border-bottom: 1px solid var(--rule); padding: 0 0 1rem; }
  .module-list li { grid-template-columns: 1fr; gap: 0.2rem; }
}
"#;

const SEARCH_JS: &str = r#"
(async function() {
  const input = document.getElementById('hoogle-input');
  const clearBtn = document.getElementById('hoogle-clear');
  const statusEl = document.getElementById('hoogle-status');
  const resultsEl = document.getElementById('hoogle-results');
  if (!input || !resultsEl) return;

  let entries = [];
  try {
    const res = await fetch('search-index.json');
    const data = await res.json();
    entries = data.entries || [];
  } catch (e) {
    statusEl.textContent = 'Could not load search-index.json (serve via HTTP or open module links below).';
    return;
  }

  function escHtml(s) {
    return String(s).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
  }

  function splitArrows(sig) {
    const s = sig.replace(/^forall\b[^.]*\.\s*/, '').replace(/^using\s*\([^)]*\)\s*=>\s*/, '').replace(/^[^-]*=>\s*/, '');
    const parts = [];
    let depth = 0, start = 0;
    for (let i = 0; i < s.length; i++) {
      if (s[i] === '(' || s[i] === '{') depth++;
      else if (s[i] === ')' || s[i] === '}') depth = Math.max(0, depth - 1);
      else if (depth === 0 && s[i] === '-' && s[i+1] === '>') {
        parts.push(s.slice(start, i).trim().replace(/^\((.*)\)$/, '$1'));
        i += 2;
        if (s[i] === '{') { while (i < s.length && s[i] !== '}') i++; i++; }
        start = i;
      }
    }
    parts.push(s.slice(start).trim().replace(/^\((.*)\)$/, '$1'));
    return parts.filter(Boolean);
  }

  function normType(t) {
    return t.replace(/\b([A-Z][A-Za-z0-9_]*\.)+([A-Z][A-Za-z0-9_]*)/g, '$2').replace(/\s+/g, ' ').trim();
  }

  function matchType(qParts, cParts) {
    if (cParts.length < qParts.length || cParts.length > qParts.length + 2) return null;
    const qRet = normType(qParts[qParts.length - 1]);
    const cRet = normType(cParts[cParts.length - 1]);
    let score = 0;
    if (qRet === cRet) score += 500;
    else if (qRet.length === 1 || cRet.length === 1) score += 320;
    else if (cRet.toLowerCase().includes(qRet.toLowerCase())) score += 260;
    else return null;
    const qArgs = qParts.slice(0, -1).map(normType);
    const cArgs = cParts.slice(0, -1).map(normType);
    const used = new Set();
    for (const qa of qArgs) {
      let best = -1, bestS = 0;
      for (let i = 0; i < cArgs.length; i++) {
        if (used.has(i)) continue;
        const ca = cArgs[i];
        let s = 0;
        if (qa === ca) s = 220;
        else if (qa.length === 1 || ca.length === 1) s = 140;
        else if (ca.toLowerCase().includes(qa.toLowerCase())) s = 110;
        if (s > bestS) { bestS = s; best = i; }
      }
      if (best < 0) return null;
      used.add(best);
      score += bestS;
    }
    return score - (cArgs.length - qArgs.length) * 40;
  }

  function search(raw) {
    const words = raw.trim().split(/\s+/).filter(Boolean);
    if (!words.length) { statusEl.textContent = ''; resultsEl.innerHTML = ''; return; }
    const inc = [], exc = [], kinds = [], rest = [];
    for (const w of words) {
      if (w.startsWith('+')) inc.push(w.slice(1).toLowerCase());
      else if (w.startsWith('-') && w.length > 1) exc.push(w.slice(1).toLowerCase());
      else if (w.startsWith('is:') || w.startsWith('kind:')) kinds.push(w.split(':')[1].toLowerCase());
      else rest.push(w);
    }
    const q = rest.join(' ');
    const isType = q.includes('->') || q.includes('=>') || q.startsWith('::');
    const qParts = isType ? splitArrows(q.replace(/^::\s*/, '')) : [];
    const qlc = q.toLowerCase();

    const hits = [];
    for (const e of entries) {
      const mlc = e.module.toLowerCase();
      if (inc.length && !inc.some(m => mlc.includes(m))) continue;
      if (exc.some(m => mlc.startsWith(m))) continue;
      if (kinds.length && !kinds.some(k => e.kind.toLowerCase() === k || (k === 'type' && /data|record|codata|type/.test(e.kind)))) continue;

      let score = null;
      if (!q) score = 500;
      else if (isType) {
        const sig = (e.pretty || e.signature || '').replace(/^[^:]+::\s*/, '');
        score = matchType(qParts, splitArrows(sig));
      } else {
        const nlc = e.name.toLowerCase();
        if (nlc === qlc) score = 1000;
        else if (nlc.startsWith(qlc)) score = 850 - (nlc.length - qlc.length);
        else if (nlc.includes(qlc)) score = 700;
        else if ((e.summary || '').toLowerCase().includes(qlc)) score = 420;
      }
      if (score !== null) hits.push({ e, score });
    }
    hits.sort((a, b) => b.score - a.score || a.e.module.localeCompare(b.e.module) || a.e.name.localeCompare(b.e.name));
    statusEl.textContent = `${hits.length} result${hits.length === 1 ? '' : 's'}${hits.length > 60 ? ' (showing top 60)' : ''}`;
    resultsEl.innerHTML = hits.slice(0, 60).map(({ e }) => `
      <div class="hit">
        <div class="hit-head">
          <a class="hit-sig" href="${escHtml(e.href)}">${escHtml(e.pretty || e.signature || e.name)}</a>
          <span class="hit-mod">${escHtml(e.module)} · ${escHtml(e.kind)} · <a href="${escHtml(e.sourceHref)}">Source</a></span>
        </div>
        ${e.summary ? `<p class="hit-sum">${escHtml(e.summary)}</p>` : ''}
      </div>
    `).join('');
  }

  input.addEventListener('input', () => search(input.value));
  clearBtn.addEventListener('click', () => { input.value = ''; search(''); input.focus(); });
  const initial = new URLSearchParams(location.search).get('q');
  if (initial) { input.value = initial; search(initial); }
})();
"#;

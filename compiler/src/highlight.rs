//! Cinch syntax highlighting from the repository's tree-sitter grammar.
//!
//! The grammar is compiled into the compiler by `build.rs`. The generated
//! parser is the same one the editor integration tests, so documentation
//! output and editor output agree about what a token is.

use std::sync::LazyLock;
use regex::Regex;
use tree_sitter::{Language, Node, Parser};
use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_cinch() -> *const ();
}

static LANGUAGE: LazyLock<Language> =
    LazyLock::new(|| unsafe { Language::new(LanguageFn::from_raw(tree_sitter_cinch)) });

fn language() -> &'static Language {
    &LANGUAGE
}

#[derive(Clone, Copy, Debug)]
struct Span {
    start: usize,
    end: usize,
    class: &'static str,
}

#[derive(Clone, Copy, Debug)]
struct CharSpan {
    start: usize,
    end: usize,
}

static FENCE_OPEN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\s*)(`{3,})(.*)$").expect("fence regex"));
static FENCE_CLOSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(`{3,})\s*$").expect("fence close regex"));

fn classify(kind: &str) -> Option<&'static str> {
    if kind.starts_with("kw_") {
        return Some("keyword");
    }
    Some(match kind {
        "identifier" | "wildcard" => "variable",
        "constructor" | "constructor_path" => "constructor",
        "type_variable" | "type_name_literal" => "type",
        "int_literal" | "float_literal" => "number",
        "string_literal" => "string",
        "char_literal" => "char",
        "symbol_literal" => "symbol",
        "attr_name" => "attribute",
        "effect_label" => "label",
        "dollar" => "operator",
        other if other.starts_with("op_") || other == "operator" => "operator",
        _ => return None,
    })
}

fn collect(node: Node<'_>, spans: &mut Vec<Span>) {
    if node.child_count() == 0 {
        if node.end_byte() > node.start_byte() {
            if let Some(class) = classify(node.kind()) {
                spans.push(Span { start: node.start_byte(), end: node.end_byte(), class });
            }
        }
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect(child, spans);
    }
}

/// Comment spans are not present in the parse tree: the scanner emits them as
/// a hidden token in `extras`. Scan them separately so front ends can colour
/// prose the grammar intentionally skips.
fn comment_spans(source: &str) -> Vec<CharSpan> {
    let bytes = source.as_bytes();
    let mut spans = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'"' {
            index += 1;
            while index < bytes.len() {
                match bytes[index] {
                    b'\\' => index += 2,
                    b'"' => {
                        index += 1;
                        break;
                    }
                    _ => index += 1,
                }
            }
            continue;
        }
        if bytes[index] == b'{' && bytes.get(index + 1) == Some(&b'-') {
            let start = index;
            let mut depth = 1;
            index += 2;
            while index < bytes.len() && depth > 0 {
                if bytes[index] == b'{' && bytes.get(index + 1) == Some(&b'-') {
                    depth += 1;
                    index += 2;
                } else if bytes[index] == b'-' && bytes.get(index + 1) == Some(&b'}') {
                    depth -= 1;
                    index += 2;
                } else {
                    index += 1;
                }
            }
            spans.push(CharSpan { start, end: index });
            continue;
        }
        if bytes[index] == b'-' && bytes.get(index + 1) == Some(&b'-') {
            let start = index;
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            spans.push(CharSpan { start, end: index });
            continue;
        }
        index += 1;
    }
    spans
}

/// Byte ranges of comments, in source order. Comments are lexical trivia
/// (not parse-tree nodes), so anything that reports them to a front end has
/// to scan for them.
pub fn comment_ranges(source: &str) -> Vec<(usize, usize)> {
    comment_spans(source)
        .into_iter()
        .map(|span| (span.start, span.end))
        .collect()
}

/// Token spans with class names, for front ends that paint their own cells.
pub fn token_spans(source: &str) -> Vec<(usize, usize, &'static str)> {
    spans(source)
        .into_iter()
        .map(|span| (span.start, span.end, span.class))
        .collect()
}

fn spans(source: &str) -> Vec<Span> {
    let mut parser = Parser::new();
    parser.set_language(language()).expect("cinch grammar ABI");
    let tree = parser.parse(source, None).expect("tree-sitter parse");
    let mut found = Vec::new();
    collect(tree.root_node(), &mut found);
    found.extend(
        comment_spans(source)
            .into_iter()
            .map(|span| Span { start: span.start, end: span.end, class: "comment" }),
    );
    found.sort_by_key(|span| (span.start, span.end));

    let mut merged: Vec<Span> = Vec::with_capacity(found.len());
    let mut last_end = 0;
    for span in found {
        if span.start < last_end || span.start == span.end {
            continue;
        }
        merged.push(span);
        last_end = span.end;
    }
    merged
}

fn escape(text: &str, out: &mut String) {
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
}

/// The highlighted body of a code block, without a wrapper element.
pub fn fragment(source: &str) -> String {
    let spans = spans(source);
    let mut out = String::with_capacity(source.len() + spans.len() * 24);
    let mut last = 0;
    for span in spans {
        if span.start < last || span.end > source.len() {
            continue;
        }
        escape(&source[last..span.start], &mut out);
        out.push_str("<span class=\"tok-");
        out.push_str(span.class);
        out.push_str("\">");
        escape(&source[span.start..span.end], &mut out);
        out.push_str("</span>");
        last = span.end;
    }
    escape(&source[last..], &mut out);
    out
}

/// A standalone highlighted Cinch code block.
pub fn html(source: &str) -> String {
    format!(
        "<pre class=\"cinch-highlight\"><code class=\"language-cinch\">{}</code></pre>\n",
        fragment(source)
    )
}

/// Replace top-level ```cinch fences with tree-sitter-highlighted HTML.
/// `cinch test` fences stay fenced: their heredoc sections are not Cinch.
pub fn markdown(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut out = String::new();
    let mut index = 0;
    while index < lines.len() {
        let Some(open) = FENCE_OPEN.captures(lines[index]) else {
            out.push_str(lines[index]);
            out.push('\n');
            index += 1;
            continue;
        };
        let indent = open.get(1).map(|m| m.as_str()).unwrap_or("");
        let fence_len = open.get(2).map(|m| m.as_str().len()).unwrap_or(3);
        let info = open.get(3).map(|m| m.as_str().trim()).unwrap_or("");
        let mut end = index + 1;
        while end < lines.len() {
            if let Some(close) = FENCE_CLOSE.captures(lines[end]) {
                if close.get(1).map(|m| m.as_str().len()).unwrap_or(0) >= fence_len {
                    break;
                }
            }
            end += 1;
        }
        let is_cinch = info.split_whitespace().next() == Some("cinch")
            && !info.split_whitespace().any(|word| word == "test");
        if is_cinch {
            let body = lines[index + 1..end].join("\n");
            if !indent.is_empty() {
                out.push_str(indent);
            }
            out.push_str(&html(&body));
        } else {
            for line in &lines[index..=end.min(lines.len().saturating_sub(1))] {
                out.push_str(line);
                out.push('\n');
            }
        }
        index = end + 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_keywords_and_constructors() {
        let html = html("data Box where\n  Box :: Int -> Box");
        assert!(html.contains("<span class=\"tok-keyword\">data</span>"), "{html}");
        assert!(html.contains("<span class=\"tok-keyword\">where</span>"), "{html}");
        assert!(html.contains("<span class=\"tok-constructor\">Box</span>"), "{html}");
        assert!(html.contains("<span class=\"tok-constructor\">Int</span>"), "{html}");
    }

    #[test]
    fn highlights_comments_skipped_by_the_grammar() {
        let html = html("-- a note\nvalue :: Int\nvalue = 1");
        assert!(html.contains("<span class=\"tok-comment\">-- a note</span>"), "{html}");
        assert!(html.contains("<span class=\"tok-number\">1</span>"), "{html}");
    }

    #[test]
    fn replaces_only_plain_cinch_fences() {
        let source = "```cinch\nvalue = 1\n```\n\n```cinch test name=x\n--- input\nvalue\n```\n";
        let out = markdown(source);
        assert!(out.contains("class=\"cinch-highlight\""), "{out}");
        assert!(out.contains("cinch test name=x"), "{out}");
    }
}

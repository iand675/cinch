"""Experimental, data-only syntax libraries for Cinch.

The compiler remains the authority on the expanded program. The grammar engine
knows no HTML, queries, protocols or other particular syntax library.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
import hashlib
import html
import json
import re


class SyntaxFailure(Exception):
    def __init__(self, position: int, message: str):
        self.position, self.message = position, message
        super().__init__(message)


@dataclass
class Fragment:
    text: str
    origins: list[int]
    copied: list[bool]

    @classmethod
    def source(cls, text, start):
        return cls(text, list(range(start, start + len(text))), [True] * len(text))

    @classmethod
    def generated(cls, text, origin):
        return cls(text, [origin] * len(text), [False] * len(text))

    def __add__(self, other):
        return Fragment(self.text + other.text, self.origins + other.origins,
                        self.copied + other.copied)


def concat(parts):
    return Fragment("".join(p.text for p in parts),
                    sum((p.origins for p in parts), []),
                    sum((p.copied for p in parts), []))


@dataclass
class Match:
    start: int
    end: int
    value: Fragment
    captures: dict[str, "Match"] = field(default_factory=dict)
    items: list["Match"] = field(default_factory=list)
    outputs: dict[str, Fragment] = field(default_factory=dict)

    def output(self, name="code"):
        return self.outputs.get(name, self.value) if name == "code" else self.outputs[name]


def skip_host_token(text, pos):
    """Skip a host string/character/comment; return None for ordinary code.

    Cinch tick names are identifiers, not unterminated character literals.
    """
    if text.startswith("--", pos):
        end = text.find("\n", pos)
        return len(text) if end < 0 else end
    if text.startswith("{-", pos):
        depth, end = 1, pos + 2
        while end < len(text) and depth:
            if text.startswith("{-", end): depth += 1; end += 2
            elif text.startswith("-}", end): depth -= 1; end += 2
            else: end += 1
        if depth: raise SyntaxFailure(pos, "unterminated Cinch block comment")
        return end
    if text[pos:pos + 1] == '"':
        end = pos + 1
        while end < len(text):
            if text[end] == "\\": end += 2
            elif text[end] == '"': return end + 1
            else: end += 1
        raise SyntaxFailure(pos, "unterminated Cinch string")
    if text[pos:pos + 1] == "'":
        char = re.match(r"'(?:\\(?:u\{[0-9a-fA-F]+\}|.)|[^'\\\n])'", text[pos:])
        if char: return pos + len(char.group())
    return None


def host_end(text, start):
    """Find a host hole's closing brace, respecting all nested delimiters."""
    stack, pos = ["}"], start + 1
    pairs = {"(": ")", "[": "]", "{": "}"}
    while pos < len(text):
        skipped = skip_host_token(text, pos)
        if skipped is not None: pos = skipped; continue
        c = text[pos]
        if c in pairs: stack.append(pairs[c])
        elif c in ")]}":
            if not stack or c != stack[-1]:
                raise SyntaxFailure(pos, "unbalanced delimiter in embedded Cinch")
            stack.pop()
            if not stack: return pos
        pos += 1
    raise SyntaxFailure(start, "unfinished embedded Cinch expression: expected `}`")


class Grammar:
    def __init__(self, path):
        self.path = Path(path)
        self.data = json.loads(self.path.read_text())
        if self.data.get("version") != 1: raise ValueError("unsupported syntax version")
        self.rules = self.data["rules"]
        self.entry = self.data["entry"]
        self.helpers = self.data.get("helpers", {})
        self._validate()

    def _validate(self):
        def visit(pattern):
            if isinstance(pattern, str):
                if not pattern: raise ValueError("empty literals are not allowed")
            elif isinstance(pattern, list):
                for child in pattern: visit(child)
            elif isinstance(pattern, dict):
                if "ref" in pattern and pattern["ref"] not in self.rules:
                    raise ValueError(f"unknown syntax rule {pattern['ref']}")
                if "regex" in pattern:
                    # Keep lexical regexes in a deliberately small regular
                    # subset: no executable callbacks, backreferences,
                    # lookarounds or nested quantified groups. Each alternative
                    # has at most one unbounded repeat, so a library cannot
                    # smuggle catastrophic regex backtracking past the budget.
                    classes_removed = re.sub(r"\[(?:\\.|[^\]\\])*\]", "C", pattern["regex"])
                    if re.search(r"[(){}?]|\\[1-9]", classes_removed):
                        raise ValueError("lexical regex uses an unsupported operation")
                    if any(len(re.findall(r"(?<!\\)[*+]", alternative)) > 1
                           for alternative in classes_removed.split("|")):
                        raise ValueError("lexical regex has multiple unbounded repeats")
                    if re.compile(pattern["regex"]).match(""):
                        raise ValueError("lexical patterns must consume input")
                for key in ("of", "many", "quote"):
                    if key in pattern: visit(pattern[key])
                if "choice" in pattern:
                    for child in pattern["choice"]: visit(child)
            else: raise ValueError("invalid syntax pattern")
        if self.entry not in self.rules: raise ValueError("missing entry rule")
        for rule in self.rules.values(): visit(rule["pattern"])
        for helper in self.helpers.values():
            if not re.fullmatch(r"[A-Z][\w]*(?:\.[\w][\w-]*)+", helper):
                raise ValueError("template helpers must have definition-site module qualification")


class Parser:
    def __init__(self, engine, grammar, text, base, invocation):
        self.engine, self.grammar, self.text = engine, grammar, text
        self.base, self.invocation = base, invocation
        self.steps, self.depth = 0, 0
        self.farthest = SyntaxFailure(0, "expected syntax")
        self.active = set()
        self.fresh = {}

    def error(self, pos, message):
        error = SyntaxFailure(pos, message)
        if pos >= self.farthest.position: self.farthest = error
        raise error

    def ws(self, pos):
        while pos < len(self.text) and self.text[pos].isspace(): pos += 1
        return pos

    def rule(self, name, pos):
        key = (name, pos)
        if key in self.active: self.error(pos, f"left recursion in syntax rule `{name}`")
        if self.depth >= 96: self.error(pos, "syntax nesting budget exhausted")
        self.active.add(key); self.depth += 1
        try:
            rule = self.grammar.rules[name]
            result = self.match(rule["pattern"], pos)
            for left, right in rule.get("equal", []):
                a, b = self.lookup(result, left), self.lookup(result, right)
                if a.text != b.text:
                    self.error(result.end, f"{left} `{a.text}` does not match {right} `{b.text}`")
            emission = rule.get("emit")
            if isinstance(emission, str): result.outputs["code"] = self.template(emission, result)
            elif isinstance(emission, dict):
                result.outputs.update({k: self.template(v, result) for k, v in emission.items()})
            self.engine.trace.append({"rule": name, "start": self.base + result.start,
                                      "end": self.base + result.end,
                                      "library": str(self.grammar.path)})
            return result
        finally:
            self.active.remove(key); self.depth -= 1

    def lookup(self, match, path):
        bits = path.split(".")
        if bits[0] == "self": node = match
        else:
            try: node = match.captures[bits[0]]
            except KeyError: raise ValueError(f"unknown capture `{path}` in {self.grammar.path}")
        if len(bits) == 1: return node.output()
        if bits[1] == "raw": return Fragment.source(self.text[node.start:node.end], self.base + node.start)
        return node.output(bits[1])

    def template(self, template, match):
        parts, end = [], 0
        for token in re.finditer(r"\$\{([^}]+)\}", template):
            parts.append(Fragment.generated(template[end:token.start()], self.invocation))
            key = token.group(1)
            if key.startswith("@"):
                part = Fragment.generated(self.grammar.helpers[key[1:]], self.invocation)
            elif key.startswith("!"):
                if key not in self.fresh: self.fresh[key] = self.engine.fresh(key[1:])
                part = Fragment.generated(self.fresh[key], self.invocation)
            else: part = self.lookup(match, key)
            parts.append(part); end = token.end()
        parts.append(Fragment.generated(template[end:], self.invocation))
        result = concat(parts)
        if len(result.text) > 1_000_000: self.error(match.start, "syntax output budget exhausted")
        return result

    def match(self, pattern, pos):
        self.steps += 1
        if self.steps > 100_000: self.error(pos, "syntax matching budget exhausted")
        start = pos
        if isinstance(pattern, str):
            pos = self.ws(pos)
            if not self.text.startswith(pattern, pos): self.error(pos, f"expected `{pattern}`")
            end = pos + len(pattern)
            return Match(pos, end, Fragment.source(pattern, self.base + pos))
        if isinstance(pattern, list):
            items, captures = [], {}
            for child in pattern:
                item = self.match(child, pos); items.append(item); pos = item.end
                for key, value in item.captures.items():
                    if key in captures: raise ValueError(f"duplicate capture `{key}`")
                    captures[key] = value
            return Match(start, pos, concat([n.output() for n in items]), captures, items)
        if "capture" in pattern:
            item = self.match(pattern["of"], pos)
            return Match(item.start, item.end, item.output(), {pattern["capture"]: item})
        if "ref" in pattern: return self.rule(pattern["ref"], pos)
        if "regex" in pattern:
            if pattern.get("skip", True): pos = self.ws(pos)
            found = re.compile(pattern["regex"]).match(self.text, pos)
            if found is None: self.error(pos, "expected " + pattern.get("label", pattern["regex"]))
            return Match(pos, found.end(), Fragment.source(found.group(), self.base + pos))
        if "choice" in pattern:
            for child in pattern["choice"]:
                try: return self.match(child, pos)
                except SyntaxFailure: pass
            raise self.farthest
        if "many" in pattern:
            items = []
            while True:
                try: item = self.match(pattern["many"], pos)
                except SyntaxFailure: break
                if item.end == pos: self.error(pos, "repeated syntax must consume input")
                items.append(item); pos = item.end
            if len(items) < pattern.get("min", 0): raise self.farthest
            joiner = Fragment.generated(pattern.get("join", ", "), self.invocation)
            parts = []
            for i, item in enumerate(items):
                if i: parts.append(joiner)
                parts.append(item.output())
            return Match(start, pos, concat(parts), items=items)
        if "quote" in pattern:
            item = self.match(pattern["quote"], pos)
            text = item.output().text
            if pattern.get("strip"): text = text[1:-1]
            if pattern.get("decode") == "html": text = html.unescape(text)
            quoted = json.dumps(text, ensure_ascii=False)
            return Match(item.start, item.end, Fragment.generated(quoted, self.base + item.start))
        if "host" in pattern:
            pos = self.ws(pos)
            if self.text[pos:pos + 1] != "{": self.error(pos, "expected `{` opening embedded Cinch")
            end = host_end(self.text, pos)
            fragment = self.engine.expand_region(self.text[pos + 1:end], self.base + pos + 1)
            return Match(pos, end + 1, fragment)
        raise ValueError(f"unknown grammar pattern {pattern}")

    def parse(self):
        result = self.rule(self.grammar.entry, 0)
        if self.ws(result.end) != len(self.text): raise self.farthest
        return result.output()


IMPORT = re.compile(r'(?m)^(\s*)import syntax "([^"\n]+)" as ([a-z][\w-]*)\s*$')


class Engine:
    def __init__(self, source, path):
        if len(source) > 1_000_000: raise SyntaxFailure(0, "syntax input budget exhausted")
        self.source, self.path = source, Path(path).resolve()
        self.grammars, self.imports, self.trace = {}, [], []
        self.counter, self.expansions = 0, 0
        self.identifiers = set(re.findall(r"[A-Za-z_][\w-]*", source))
        self.masked = source
        # Imports are recognized only outside host strings and comments.
        candidates = {m.start(): m for m in IMPORT.finditer(source)}
        pos, edits, modules = 0, [], 0
        while pos < len(source):
            candidate = candidates.get(pos)
            if candidate:
                relative, alias = candidate.group(2, 3)
                if alias in self.grammars: raise SyntaxFailure(pos, f"duplicate syntax import `{alias}`")
                grammar_path = (self.path.parent / relative).resolve()
                grammar = Grammar(grammar_path)
                self.grammars[alias] = grammar
                self.imports.append({"alias": alias, "path": str(grammar_path),
                                     "hash": hashlib.sha256(grammar_path.read_bytes()).hexdigest()})
                edits.append((candidate.start(), candidate.end()))
                pos = candidate.end(); continue
            if (pos == 0 or source[pos - 1] == "\n") and re.match(r"module\s+[A-Z]", source[pos:]):
                modules += 1
            invocation = re.match(r"([a-z][\w-]*)!\{", source[pos:])
            if invocation and invocation.group(1) in self.grammars and (pos == 0 or not re.match(r"[\w.]", source[pos - 1])):
                # Imported syntax owns its raw text. Do not mistake an HTML
                # text node for a compile-time import or a module header.
                start = pos + len(invocation.group())
                state = (self.counter, self.expansions, self.identifiers.copy(), len(self.trace))
                parser = Parser(self, self.grammars[invocation.group(1)], source[start:], start, pos)
                try:
                    matched = parser.rule(parser.grammar.entry, 0)
                except SyntaxFailure as error:
                    raise SyntaxFailure(start + error.position, f"{invocation.group(1)}: {error.message}")
                self.counter, self.expansions, self.identifiers, trace_len = state
                del self.trace[trace_len:]
                pos = start + matched.end
                continue
            skipped = skip_host_token(source, pos)
            pos = skipped if skipped is not None else pos + 1
        if modules > 1 and self.imports:
            raise ValueError("syntax imports currently require one module per source file; module scope must not leak")
        for start, end in reversed(edits):
            self.masked = self.masked[:start] + "".join("\n" if c == "\n" else " " for c in source[start:end]) + self.masked[end:]

    def fresh(self, label):
        while True:
            self.counter += 1
            name = f"__syntax_{self.counter}_{label}"
            if name not in self.identifiers:
                self.identifiers.add(name)
                return name

    def expand_region(self, text, base=0):
        pos, copied_start, parts = 0, 0, []
        while pos < len(text):
            skipped = skip_host_token(text, pos)
            if skipped is not None: pos = skipped; continue
            name = re.match(r"([a-z][\w-]*)!\{", text[pos:])
            if name and (pos == 0 or not re.match(r"[\w.]", text[pos - 1])):
                alias = name.group(1)
                if alias in self.grammars:
                    if "#[no-macros]" in self.source or '#[profile("jpl")]' in self.source:
                        raise SyntaxFailure(base + pos, "external syntax expansion is refused under no-macros policy")
                    self.expansions += 1
                    if self.expansions > 256: raise SyntaxFailure(base + pos, "syntax expansion budget exhausted")
                    # Grammar entry includes the island's closing `}`. Its own
                    # lexer decides which braces belong to text versus code.
                    body_start = pos + len(name.group())
                    parser = Parser(self, self.grammars[alias], text[body_start:], base + body_start, base + pos)
                    try: matched = parser.rule(parser.grammar.entry, 0)
                    except SyntaxFailure as error:
                        raise SyntaxFailure(base + body_start + error.position, f"{alias}: {error.message}")
                    parts.append(Fragment.source(text[copied_start:pos], base + copied_start))
                    parts.append(matched.output())
                    pos = body_start + matched.end
                    copied_start = pos
                    continue
            pos += 1
        parts.append(Fragment.source(text[copied_start:], base + copied_start))
        return concat(parts)

    def expand(self):
        if len(self.source) > 1_000_000: raise SyntaxFailure(0, "syntax input budget exhausted")
        fragment = self.expand_region(self.masked)
        if len(fragment.text) > 1_000_000: raise SyntaxFailure(0, "syntax output budget exhausted")
        return Expansion(self.source, fragment, self.imports, self.trace)


@dataclass
class Expansion:
    source: str
    fragment: Fragment
    imports: list
    trace: list

    def original_position(self, line, character):
        """Map zero-based LSP UTF-16 coordinates in generated text to source."""
        offset = utf16_offset(self.fragment.text, line, character)
        if not self.fragment.origins: return (0, 0)
        origin = self.fragment.origins[min(offset, len(self.fragment.origins) - 1)]
        return utf16_position(self.source, origin)

    def generated_position(self, line, character):
        origin = utf16_offset(self.source, line, character)
        for offset, (mapped, copied) in enumerate(zip(self.fragment.origins, self.fragment.copied)):
            if mapped == origin and copied: return utf16_position(self.fragment.text, offset)
        return None

    def manifest(self):
        return {"version": 1, "imports": self.imports, "trace": self.trace,
                "origins": self.fragment.origins, "copied": self.fragment.copied}


def utf16_position(text, offset):
    before = text[:offset]
    return before.count("\n"), len(before.rsplit("\n", 1)[-1].encode("utf-16-le")) // 2


def utf16_offset(text, line, character):
    lines = text.splitlines(keepends=True)
    prefix = sum(len(s) for s in lines[:line])
    units, count = 0, 0
    for c in lines[line] if line < len(lines) else "":
        if units >= character: break
        units += len(c.encode("utf-16-le")) // 2; count += 1
    return prefix + count

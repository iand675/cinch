#!/usr/bin/env python3
"""Structural checks for CinchCore.sublime-syntax.

Sublime Text cannot be driven from this repository, so the syntax definition
is validated as data: the YAML must parse, the top-level keys must be the
ones Sublime requires, every `include` must name a context that exists, and
the categories in design/core-output.md section 8.3 must be present (unit
names as `entity.name.function`, dictionary bindings, ownership classes,
annotations, numerals, string and byte literals, the header, `end`, and the
`--` comment).

    python3 tooling/editors/sublime/test/check.py

Exits nonzero on the first failing group of checks.
"""

import pathlib
import plistlib
import re
import sys

import yaml

HERE = pathlib.Path(__file__).resolve().parent
SYNTAX = HERE.parent / "CinchCore.sublime-syntax"
SURFACE = HERE.parent / "Cinch.sublime-syntax"

failures = []


def check(name, actual, expected):
    ok = actual == expected
    print(f"{name:<28} {'ok  ' if ok else 'FAIL'} {actual!r}")
    if not ok:
        failures.append(name)


def scopes_of(rules):
    """Every scope string a context's rules assign."""
    found = []
    for rule in rules:
        if "scope" in rule:
            found.append(rule["scope"])
        for scope in rule.get("captures", {}).values():
            found.append(scope)
        for inner in rule.get("push", []):
            if isinstance(inner, dict) and "scope" in inner:
                found.append(inner["scope"])
            if isinstance(inner, dict) and "meta_scope" in inner:
                found.append(inner["meta_scope"])
    return found


def main():
    text = SYNTAX.read_text(encoding="utf-8")
    doc = yaml.safe_load(text)

    check("yaml parses", isinstance(doc, dict), True)
    check("scope", doc.get("scope"), "source.cinch-core")
    check("file_extensions", doc.get("file_extensions"), ["core"])
    check("name", doc.get("name"), "Cinch Core")
    check("first_line_match", doc.get("first_line_match"), r"^core \d+$")

    contexts = doc.get("contexts", {})
    check("has main", "main" in contexts, True)

    # Every include names an existing context.
    missing = []
    for name, rules in contexts.items():
        for rule in rules:
            target = rule.get("include")
            if target is not None and target not in contexts:
                missing.append(f"{name} -> {target}")
    check("includes resolve", missing, [])

    all_scopes = []
    for rules in contexts.values():
        all_scopes.extend(scopes_of(rules))
    joined = "\n".join(all_scopes)

    # Section 8.3 categories.
    check(
        "unit names are symbols",
        bool(re.search(r"^entity\.name\.function", joined, re.M)),
        True,
    )
    for category, scope in [
        ("dictionary binding", r"^variable\.other\.dictionary"),
        ("ownership class", r"^storage\.modifier\.ownership"),
        ("annotation", r"^storage\.type\.annotation"),
        ("numeral", r"^constant\.numeric"),
        ("string literal", r"^string\.quoted\.double"),
        ("byte literal", r"^string\.quoted\.double\.bytes"),
        ("header", r"^keyword\.other\.header"),
        ("end", r"^keyword\.other\.unit"),
        ("comment", r"^comment\.line\.double-dash"),
    ]:
        check(category, bool(re.search(scope, joined, re.M)), True)

    # Comment toggling: a `.sublime-syntax` file has no line-comment key, so
    # the `--` marker is declared the TextMate way, in a tmPreferences file
    # scoped to the grammar (this is what Toggle Comment reads).
    prefs = HERE.parent / "Comments.tmPreferences"
    check("tmPreferences exists", prefs.is_file(), True)
    if prefs.is_file():
        with prefs.open("rb") as handle:
            plist = plistlib.load(handle)
        check("tmPreferences scope", plist.get("scope"), "source.cinch-core")
        variables = {
            entry["name"]: entry["value"]
            for entry in plist["settings"]["shellVariables"]
        }
        check("TM_COMMENT_START", variables.get("TM_COMMENT_START"), "-- ")

    # Every node form in section 8.2 is matched by the forms context.
    forms = " ".join(
        rule["match"] for rule in contexts.get("forms", []) if "match" in rule
    )
    expected = [
        "int", "nat", "bytes", "string", "sym", "bool", "unit", "local",
        "global", "ctor", "prim", "foreign", "app", "lam", "dictproj", "proj",
        "instdict", "let", "if", "case", "alt", "pat", "dup", "drop",
        "release", "boxinit", "boxget", "boxclear", "refset", "quote",
        "splice", "syntaxlit", "demand",
    ]
    check("all node forms", [f for f in expected if f not in forms], [])

    if failures:
        print(f"\n{len(failures)} check(s) failed")
        return 1
    print("\nall checks passed")
    return 0


PLACEHOLDER = re.compile(r"\{\{(\w+)\}\}")


def expand(pattern, variables):
    """`{{name}}` variables replaced, the way Sublime expands them."""
    missing = [name for name in PLACEHOLDER.findall(pattern) if name not in variables]
    if missing:
        failures.append(f"undefined variable {missing}")
    return PLACEHOLDER.sub(lambda m: variables.get(m.group(1), ""), pattern)


def first_rule(doc, text, block="main"):
    """The scope Sublime assigns at the start of `text`.

    A context's rules are tried in order and an `include` is inlined where
    it appears, so the first rule (in that flattened order) whose `match`
    or `begin` pattern matches at the position wins.
    """
    variables = doc.get("variables", {})
    contexts = doc.get("contexts", {})
    for rule in contexts.get(block, []):
        if "include" in rule:
            hit = first_rule(doc, text, rule["include"])
            if hit is not None:
                return hit
            continue
        pattern = rule.get("begin") or rule.get("match")
        if pattern is None:
            continue
        if re.match(expand(pattern, variables), text):
            return rule.get("scope") or rule.get("name")
    return None


def surface():
    """The surface syntax's sigil names (amendment section 8).

    `!` and `?` continue a name after its first character and never start
    one, so `empty?` and `print!` highlight as names, `if?` is a name while
    `if` stays a keyword, and a spaced `!=` is an operator run.
    """
    doc = yaml.safe_load(SURFACE.read_text(encoding="utf-8"))
    check("surface scope", doc.get("scope"), "source.cinch")
    check("surface file_extensions", doc.get("file_extensions"), ["cinch"])

    for text, scope in [
        ("empty? x", "variable.other.cinch"),
        ("print! x", "variable.other.cinch"),
        ("read!? x", "variable.other.cinch"),
        ("int-digits-into x", "variable.other.cinch"),
        ("if? x", "variable.other.cinch"),
        ("when? x", "variable.other.cinch"),
        ("if x", "keyword.control.conditional.cinch"),
        ("when x", "variable.other.contextual.cinch"),
        ("!= y", "keyword.operator.cinch"),
        ("= y", "keyword.operator.cinch"),
        ("'empty?", "constant.other.symbol.cinch"),
        ("''Empty?", "entity.name.type.cinch"),
        ("Empty? x", "entity.name.type.cinch"),
    ]:
        check(f"surface {text!r}", first_rule(doc, text), scope)


if __name__ == "__main__":
    status = main()
    surface()
    if failures:
        print(f"\n{len(failures)} check(s) failed")
        sys.exit(1)
    print("\nsurface checks passed")
    sys.exit(status)

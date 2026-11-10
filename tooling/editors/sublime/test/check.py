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


if __name__ == "__main__":
    sys.exit(main())

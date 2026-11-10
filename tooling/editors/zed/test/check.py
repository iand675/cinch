#!/usr/bin/env python3
"""Structural checks for the Zed extension's cinch-core support.

Zed cannot be driven from this repository and the Core grammar lives in
another slice, so the extension is validated as data:

- `extension.toml` parses, lists both language directories, gives every
  language a `[grammars.*]` entry, and gives the language server both
  language names and both LSP language ids;
- each `languages/<name>/config.toml` parses, names a grammar that exists,
  uses `--` comments, and has a distinct path suffix;
- every query file exists and is paren-balanced;
- every node name and anonymous token the queries mention exists in the
  grammar contract recorded in tooling/README.md, so a query cannot name a
  node the grammar does not have (which would make Zed drop the query).

    python3.11 tooling/editors/zed/test/check.py

Needs `tomllib`, so Python 3.11 or newer (or `tomli` installed). The system
`python3` on macOS is 3.9, so run it as

    nix shell nixpkgs#python311 --command python3 tooling/editors/zed/test/check.py

or with any other 3.11+ interpreter.

Exits nonzero when a check fails.
"""

import pathlib
import re
import sys

try:
    import tomllib
except ModuleNotFoundError:
    try:
        import tomli as tomllib
    except ModuleNotFoundError:
        sys.exit(
            "check.py needs a TOML parser: run it with Python 3.11+ "
            "(tomllib) or install tomli"
        )

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent

# Named nodes in tooling/tree-sitter-cinch-core/grammar.js, from the grammar
# owner's contract (design/core-output.md sections 8.2 and 10).
NAMED = {
    "source_file",
    "header",
    "module_line",
    "target_line",
    "source_line",
    "target_triple",
    "unit",
    "unit_name",
    "name",
    "annotation",
    "annotation_name",
    "annotation_args",
    "ownership_counts",
    "class",
    "span",
    "params",
    "param",
    "type",
    "rigid_var",
    "meta_var",
    "refinement_type",
    "row_type",
    "comment",
    "alt",
    "pat",
    "int_lit",
    "nat_lit",
    "bytes_lit",
    "string_lit",
    "sym_lit",
    "bool_lit",
    "unit_lit",
    "local_ref",
    "global_ref",
    "ctor_ref",
    "prim_ref",
    "foreign_ref",
    "app",
    "lam",
    "dict_proj",
    "proj",
    "inst_dict",
    "let",
    "if",
    "case",
    "dup",
    "drop",
    "release",
    "box_init",
    "box_get",
    "box_clear",
    "ref_set",
    "quote",
    "splice",
    "syntax_lit",
    "demand",
}

# Anonymous string tokens the grammar has (the owner's list).
ANONYMOUS = {
    "(",
    ")",
    "{",
    "}",
    ",",
    "::",
    "->",
    ":",
    "@",
    "_",
    ".",
    "-",
    "unit",
    "end",
    "body",
    "forall",
    "refine",
    "with",
    "moved",
    "rest",
    "from",
    "true",
    "false",
    "owned",
    "borrowed",
    "lent",
    "uncounted",
    "immortal",
    "int",
    "nat",
    "bytes",
    "string",
    "sym",
    "bool",
    "local",
    "global",
    "ctor",
    "prim",
    "foreign",
    "app",
    "lam",
    "dictproj",
    "proj",
    "instdict",
    "let",
    "if",
    "case",
    "alt",
    "pat",
    "dup",
    "drop",
    "release",
    "boxinit",
    "boxget",
    "boxclear",
    "refset",
    "quote",
    "splice",
    "syntaxlit",
    "demand",
    "opt-level",
    "sha256",
}

QUERY_FILES = ("highlights.scm", "outline.scm", "brackets.scm", "indents.scm")

failures = []


def check(name, actual, expected):
    ok = actual == expected
    print(f"{name:<34} {'ok  ' if ok else 'FAIL'} {actual!r}")
    if not ok:
        failures.append(name)


def strip_comments(text):
    return "\n".join(line.split(";", 1)[0] for line in text.splitlines())


def balanced(text):
    depth = 0
    in_string = False
    escaped = False
    for char in text:
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth < 0:
                return False
    return depth == 0 and not in_string


def query_names(text):
    """Node names and string tokens a query mentions."""
    body = strip_comments(text)
    names = set(re.findall(r"\(([a-z_][a-z0-9_]*)", body))
    strings = set(re.findall(r'"((?:[^"\\]|\\.)*)"', body))
    # A `#match?` predicate's argument is a regex, not a grammar token.
    strings = {s for s in strings if not s.startswith("^") and "\\\\" not in s}
    return names, strings


def main():
    manifest = tomllib.loads((ROOT / "extension.toml").read_text(encoding="utf-8"))

    check(
        "language directories",
        manifest.get("languages"),
        ["languages/cinch", "languages/cinch-core"],
    )

    grammar_entries = manifest.get("grammars", {})
    check("grammar entries", sorted(grammar_entries), ["cinch", "cinch-core"])
    for grammar, entry in grammar_entries.items():
        check(f"{grammar} repository", "repository" in entry, True)
        check(f"{grammar} rev", "rev" in entry, True)

    suffixes = []
    language_names = []
    for directory in manifest["languages"]:
        path = ROOT / directory / "config.toml"
        check(f"{directory} config exists", path.is_file(), True)
        if not path.is_file():
            continue
        config = tomllib.loads(path.read_text(encoding="utf-8"))
        language_names.append(config.get("name"))
        check(
            f"{directory} grammar exists",
            config.get("grammar") in grammar_entries,
            True,
        )
        check(f"{directory} comment", config.get("line_comments"), ["-- "])
        suffixes.extend(config.get("path_suffixes", []))
        for query in QUERY_FILES:
            qpath = ROOT / directory / query
            check(f"{directory}/{query}", qpath.is_file(), True)
            if not qpath.is_file():
                continue
            text = qpath.read_text(encoding="utf-8")
            check(f"{directory}/{query} balanced", balanced(strip_comments(text)), True)
            # Only the Core grammar's node names are known here; the surface
            # grammar's queries are checked for balance only.
            if config.get("grammar") != "cinch-core":
                continue
            names, strings = query_names(text)
            unknown_names = sorted(names - NAMED)
            unknown_strings = sorted(strings - ANONYMOUS)
            check(f"{directory}/{query} nodes", unknown_names, [])
            check(f"{directory}/{query} tokens", unknown_strings, [])

    # The nvim-treesitter query set (editors/vim/queries/cinch-core) targets
    # the same grammar, so it is validated against the same vocabulary.
    nvim = ROOT.parent / "vim/queries/cinch-core"
    check("nvim query directory", nvim.is_dir(), True)
    for qpath in sorted(nvim.glob("*.scm")):
        text = qpath.read_text(encoding="utf-8")
        check(f"nvim/{qpath.name} balanced", balanced(strip_comments(text)), True)
        names, strings = query_names(text)
        check(f"nvim/{qpath.name} nodes", sorted(names - NAMED), [])
        check(f"nvim/{qpath.name} tokens", sorted(strings - ANONYMOUS), [])

    check("distinct path suffixes", len(set(suffixes)), len(suffixes))
    check("core suffix", "core" in suffixes, True)

    server = manifest.get("language_servers", {}).get("cinch-lsp", {})
    check("server languages", sorted(server.get("languages", [])), ["Cinch", "Cinch Core"])
    ids = server.get("language_ids", {})
    check("server language ids", ids, {"Cinch": "cinch", "Cinch Core": "cinch-core"})
    check(
        "server names match configs",
        sorted(ids) == sorted(language_names),
        True,
    )

    # The Core highlights must cover every form node, so a new form cannot
    # silently render as plain text.
    highlights = (ROOT / "languages/cinch-core/highlights.scm").read_text(
        encoding="utf-8"
    )
    colored = [
        "int_lit", "nat_lit", "bytes_lit", "string_lit", "sym_lit", "bool_lit",
        "unit_lit", "local_ref", "global_ref", "ctor_ref", "prim_ref",
        "foreign_ref", "pat", "unit_name", "annotation_name", "class",
        "ownership_counts", "span", "param", "type", "rigid_var", "meta_var",
        "comment", "header", "module_line", "target_line", "source_line",
    ]
    missing = [node for node in colored if f"({node}" not in highlights]
    check("highlight coverage", missing, [])

    if failures:
        print(f"\n{len(failures)} check(s) failed")
        return 1
    print("\nall checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())

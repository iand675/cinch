#!/usr/bin/env python3
"""One generator per scaling axis, each returning a whole program at size n.

Every generator is a pure function of its size, so a point is reproducible from
its number alone and the sweep has no fixture corpus to rot. The programs are
deliberately boring: one shape each, nothing but the axis varying, so the
measured growth is the axis and not an interaction.

Shared conventions:

  * `main` is the entry point and has type `Int`, which the bootstrap slice
    requires.
  * sums are BALANCED, not left-nested. A left-nested `a + b + c + ...` is a
    different axis (expression depth); balancing it here keeps each generator
    from accidentally measuring its own parenthesis chain.
  * every generated name is reachable from `main`, because the emitter prunes
    unreachable units and a pruned workload measures nothing.
"""

import textwrap


def balanced_sum(terms):
    """`t0 + t1 + ... + tn` as a balanced tree: depth log n, not n."""
    level = list(terms)
    if not level:
        return "0"
    while len(level) > 1:
        nxt = [f"({level[i]} + {level[i + 1]})" for i in range(0, len(level) - 1, 2)]
        if len(level) % 2:
            nxt.append(level[-1])
        level = nxt
    return level[0]


def program(body, imports=()):
    head = "module Main where\n"
    if imports:
        head += "".join(f"  import {line} as {alias}\n" for line, alias in imports)
    return head + textwrap.indent(body.strip("\n") + "\n", "  ")


# --------------------------------------------------------------------- axes


def definitions(n):
    """n independent `Int -> Int` definitions in one module, all called.

    This is the axis the `total-core` table is built over, and the one the
    earlier accident found: every definition contributes one row to a sorted
    list, and the row carries the identities it references.
    """
    defs = "".join(f"def{i} :: Int -> Int\ndef{i} x = x + {i}\n" for i in range(n))
    call = balanced_sum([f"def{i} 1" for i in range(n)])
    return program(defs + f"main :: Int\nmain = {call}\n")


def modules(n):
    """n modules in a chain, each importing the one before it.

    Depth without breadth: each module resolves against exactly one imported
    contract, so the per-module cost is isolated from any fan-in cost.
    """
    parts = ["module Chain0 where\n  export chain0\n  chain0 :: Int\n  chain0 = 0\n"]
    for i in range(1, n):
        parts.append(f"module Chain{i} where\n  import Chain{i - 1} as Prior\n"
                     f"  export chain{i}\n  chain{i} :: Int\n"
                     f"  chain{i} = Prior.chain{i - 1} + 1\n")
    parts.append("module Main where\n" f"  import Chain{n - 1} as Top\n"
                 "  main :: Int\n" f"  main = Top.chain{n - 1}\n")
    return "\n".join(parts)


def constructors(n):
    """One data type with n nullary constructors, and a case over all of them."""
    decl = "data Wide where\n" + "".join(f"  C{i} :: Wide\n" for i in range(n))
    arms = "".join(f"  C{i} -> {i}\n" for i in range(n))
    return program(decl + f"pick :: Wide -> Int\npick w = case w of\n{arms}"
                   "main :: Int\nmain = pick C0\n")


def type_variables(n):
    """One signature quantifying n type variables, applied at all of them.

    Every variable is used in an argument position, so none is dropped by the
    generality check, and `main` instantiates all of them at `Int`.
    """
    vars_ = [f"a{i}" for i in range(n)]
    sig = f"wide :: forall {' '.join(vars_)}. " + " -> ".join(vars_) + " -> Int"
    params = " ".join("_" for _ in vars_)
    # The arguments are one typed `Int` binding rather than literals: a
    # literal's type must be determined by its context, and the context here
    # is an unconstrained type variable.
    args = " ".join("z" for _ in vars_)
    return program("z :: Int\nz = 0\n" + sig + f"\nwide {params} = 7\n"
                   f"main :: Int\nmain = wide {args}\n")


def expression_depth(n):
    """`main` is one left-nested arithmetic chain n operators deep.

    Left nesting is the point: it makes the expression tree a spine, which is
    the shape a recursive traversal over the AST is worst at.
    """
    expr = "0"
    for _ in range(n):
        expr = f"({expr} + 1)"
    return program(f"main :: Int\nmain = {expr}\n")


def case_patterns(n):
    """One `case` over an `Int` with n literal alternatives plus a wildcard."""
    arms = "".join(f"  {i} -> {i}\n" for i in range(n))
    return program(f"pick :: Int -> Int\npick x = case x of\n{arms}  _ -> 0\n"
                   "main :: Int\nmain = pick 1\n")


def interface_entries(n):
    """n modules, each exporting four values, all imported and all used.

    Total interface entries grow as 4n while the module count grows as n, so
    this axis differs from `imports` (which holds exports at one per module)
    by exactly the per-entry term.
    """
    parts = []
    for m in range(n):
        exports = ", ".join(f"v{m}_{k}" for k in range(4))
        values = "".join(f"  v{m}_{k} :: Int\n  v{m}_{k} = {m + k}\n" for k in range(4))
        parts.append(f"module Entries{m} where\n  export {exports}\n{values}")
    imports = [(f"Entries{m}", f"E{m}") for m in range(n)]
    terms = [f"E{m}.v{m}_{k}" for m in range(n) for k in range(4)]
    parts.append(program(f"main :: Int\nmain = {balanced_sum(terms)}\n", imports))
    return "\n\n".join(parts)


def imports(n):
    """One module importing n leaves, each exporting exactly one value.

    Fan-in breadth with a fixed one entry per module, which is the term
    `interface_entries` differs from.
    """
    parts = [f"module Leaf{i} where\n  export leaf{i}\n  leaf{i} :: Int\n  leaf{i} = {i}\n"
             for i in range(n)]
    leaves = [(f"Leaf{i}", f"L{i}") for i in range(n)]
    parts.append(program(f"main :: Int\nmain = {balanced_sum([f'L{i}.leaf{i}' for i in range(n)])}\n",
                         leaves))
    return "\n\n".join(parts)


def string_literal(n):
    """One string literal n bytes long, compared against itself."""
    return program(f's :: String\ns = "{"a" * n}"\n'
                   "main :: Int\nmain = if string-compare s s == 0 then 0 else 1\n")


def identifier(n):
    """One identifier n characters long, defined once and called once."""
    name = "x" * n
    return program(f"{name} :: Int -> Int\n{name} v = v\nmain :: Int\nmain = {name} 0\n")


# --------------------------------------------------------------------- table


AXES = {
    "definitions": (definitions, "definition count in one module", (25, 50, 100, 200)),
    "modules": (modules, "module count, chained imports", (8, 16, 32, 64)),
    "constructors": (constructors, "constructor count in one data type", (8, 16, 32, 64)),
    "typevars": (type_variables, "type variables in one signature", (8, 16, 32, 64)),
    "depth": (expression_depth, "expression depth of one term", (32, 64, 128, 256)),
    "patterns": (case_patterns, "pattern count in one case", (16, 32, 64, 128)),
    "entries": (interface_entries, "interface entries across many modules", (8, 16, 32, 64)),
    "imports": (imports, "imports per module (fan-in)", (8, 16, 32, 64)),
    "string": (string_literal, "length of one string literal", (256, 512, 1024, 2048)),
    "identifier": (identifier, "length of one identifier", (64, 128, 256, 512)),
}

# The control: a known linear law the harness must reproduce. `--check` over a
# program whose manifest carries the provider's interfaces padded by n bytes
# costs a fixed number of blocks per added byte. Two values are published
# (design/compilation-speed.md 2.17 and 2.18): 10.0 before the JSON string
# scan's fast path, 6.0 after it. Reproducing either proves the harness is
# reading interface bytes and nothing else, which is what the control is for.
CONTROL = "interface-bytes"
CONTROL_SIZES = (1024, 2048, 4096, 8192)
CONTROL_LAWS = (6.0, 10.0)


def generate(axis, size):
    """The source text for one point, or `None` for the control axis."""
    if axis == CONTROL:
        return None
    return AXES[axis][0](size)


def description(axis):
    if axis == CONTROL:
        return "size of one interface document (control, known 6.0 blocks/byte)"
    return AXES[axis][1]

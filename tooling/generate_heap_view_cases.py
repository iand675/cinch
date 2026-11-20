#!/usr/bin/env python3
"""Project offline indexed-heap callback observations into bounded Cinch consumers."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "seed/tests/fixtures/heap-view-vectors.json"
HELPERS = ROOT / "seed/tests/fixtures/heap-view-consumer.cinch"
KINDS = {"init": 0, "push": 1, "fix": 2, "remove": 3, "pop": 4, "reverse": 5, "invalid": 6}


def linked(values: list[str]) -> str:
    """Materialize an ordered literal list without requiring an extra public constructor."""
    result = "Nil"
    for value in reversed(values):
        result = f"(Cons ({value}) {result})"
    return result


def item(value: dict[str, int]) -> str:
    """Render an independently recorded priority and stable identity."""
    return f"Item {value['priority']} {value['id']}"


def step(value: dict[str, object]) -> str:
    """Render one operation with exact word checksums and optional removed value."""
    result = "None" if value["result"] is None else f"(Some ({item(value['result'])}))"
    valid = "True" if value["valid"] else "False"
    return (
        f"Step {KINDS[value['kind']]} {value['index']} {value['priority']} {value['id']} "
        f"{value['length']} {value['storage']} {value['positions']} {value['trace']} {result} {valid}"
    )


def consumer(index: int, mode: str) -> str:
    """Build one finite trace or owned-storage test without invoking the reference SDK."""
    case = json.loads(FIXTURE.read_text())[index]
    packed = b"".join(value["priority"].to_bytes(2, "little") + value["id"].to_bytes(2, "little") for value in case["initial"])
    initial = f'(initial-items "{packed.hex()}")'
    if mode == "trace":
        operations = linked([step(value) for value in case["operations"]])
        action = f"trace-case {initial} {case['slots']} {operations}"
    elif mode == "owned":
        action = (
            f"owned-case {initial} {len(case['initial']) // 2} {case['replace_hash']} "
            f"{case['fix_hash']} {case['remove_hash']} {case['reverse_hash']}"
        )
    else:
        action = "control-case ()"
    return HELPERS.read_text() + (
        "\n  -- | Return a failing status when any independently recorded observation differs.\n"
        "  main :: Unit ->{Mut r} Int\n"
        f"  main unit = if {action} then 0 else 1\n"
    )


def main() -> None:
    """Emit an explicitly selected offline fixture consumer."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--case", type=int, required=True)
    parser.add_argument("--mode", choices=["trace", "owned", "control"], required=True)
    args = parser.parse_args()
    print(consumer(args.case, args.mode), end="")


if __name__ == "__main__":
    main()

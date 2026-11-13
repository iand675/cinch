#!/usr/bin/env python3
"""The JSON differential harness: one document set, two scanners, one answer.

The scanner lane of `design/compilation-speed.md` section 2.18 checked 29
documents against an ad hoc probe that was never committed, so this file is
that harness written down. It compares two *implementations* of the same
JSON reader over the same documents and requires byte-identical output:
the canonical print of every document that parses, and the rendered error
bundle of every document that does not.

  reference   `Text.Json` as it stands, through `parse-json`
  candidate   the scanner under test, through `parse-json`

Both sides run as real compiled programs against the same runtime, so the
comparison covers the scanner, the node build, the canonical printer and the
diagnostic renderer, not just the scanner.

  python3 experiments/iface-scan/differential.py --compiler <cinch> \
      --out /tmp/json-differential

The document set is built here rather than checked in, so a case is added by
adding a row to `DOCUMENTS` and no fixture can drift from its description.
Every row is `(name, bytes)`. The cases cover what the representation
introduces (slices, empty and one-byte strings, a slice at the buffer's last
byte, one-entry containers, adjacent identical strings) as well as every
refusal the scanner already produced.

Exit 0 when every document's output matches, 1 otherwise, and the mismatches
are written to `<out>/mismatches/`.
"""

import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# ---------------------------------------------------------------- documents
#
# `bytes` rather than `str` where the case is deliberately not valid UTF-8,
# so a truncated sequence is written as the bytes it is.

DOCUMENTS = [
    # --- scalars
    ("null", b"null"),
    ("true", b"true"),
    ("false", b"false"),
    ("zero", b"0"),
    ("minus-zero", b"-0"),
    ("one", b"1"),
    ("negative", b"-17"),
    ("big-integer", b"123456789012345678901234567890"),
    ("fraction", b"2.5"),
    ("exponent", b"1e10"),
    ("exponent-negative", b"5e-1"),
    ("exponent-upper", b"1E+3"),
    ("float-overflow", b"1e400"),

    # --- strings: the slice cases
    ("empty-string", b'""'),
    ("one-byte-string", b'"a"'),
    ("two-adjacent-strings", b'["a","b"]'),
    ("identical-key-and-value", b'{"a":"a"}'),
    ("long-string", b'"' + b"x" * 4096 + b'"'),
    ("string-at-last-byte", b'{"k":"' + b"y" * 100 + b'"}'),
    ("utf8-two-byte", '"é"'.encode()),
    ("utf8-three-byte", '"€"'.encode()),
    ("utf8-four-byte", '"😀"'.encode()),
    ("utf8-max-scalar", '"' .encode() + b"\xf4\x8f\xbf\xbf" + b'"'),
    ("utf8-in-a-key", '{"é":"€"}'.encode()),

    # --- escapes
    ("escape-quote", b'"a\\"b"'),
    ("escape-backslash", b'"a\\\\b"'),
    ("escape-solidus", b'"a\\/b"'),
    ("escape-backspace", b'"a\\bb"'),
    ("escape-formfeed", b'"a\\fb"'),
    ("escape-newline", b'"a\\nb"'),
    ("escape-return", b'"a\\rb"'),
    ("escape-tab", b'"a\\tb"'),
    ("escape-u-ascii", b'"\\u0041"'),
    ("escape-u-nul", b'"\\u0000"'),
    ("escape-u-pair", b'"\\ud83d\\ude00"'),
    ("escape-u-high-alone", b'"\\ud83d"'),
    ("escape-u-low-alone", b'"\\ude00"'),
    ("escape-u-pair-reversed", b'"\\ude00\\ud83d"'),
    ("escape-u-truncated", b'"\\u00"'),
    ("escape-unknown", b'"a\\qb"'),
    ("escape-at-end", b'"a\\'),

    # --- refusals the scanner already produced
    ("raw-control-byte", b'"a\x01b"'),
    ("raw-newline-in-string", b'"a\nb"'),
    ("raw-tab-in-string", b'"a\tb"'),
    ("unterminated-string", b'"abc'),
    ("truncated-utf8-two", b'"' + b"\xc3"),
    ("truncated-utf8-three", b'"' + b"\xe2\x82"),
    ("truncated-utf8-four", b'"' + b"\xf0\x9f\x98"),
    ("overlong-two", b'"' + b"\xc0\xaf" + b'"'),
    ("overlong-three", b'"' + b"\xe0\x80\xaf" + b'"'),
    ("continuation-first", b'"' + b"\x80" + b'"'),
    ("surrogate-in-utf8", b'"' + b"\xed\xa0\x80" + b'"'),
    ("above-max-scalar", b'"' + b"\xf5\x80\x80\x80" + b'"'),

    # --- containers: the child-index-range boundaries
    ("empty-object", b"{}"),
    ("empty-array", b"[]"),
    ("one-entry-object", b'{"a":1}'),
    ("one-element-array", b"[1]"),
    ("nested-empty", b'{"a":{},"b":[]}'),
    ("array-of-objects", b'[{"a":1},{"a":2}]'),
    ("deep-nesting", b"[" * 40 + b"]" * 40),
    ("duplicate-key", b'{"a":1,"a":2}'),
    ("unterminated-object", b'{"a":1'),
    ("unterminated-array", b"[1,2"),
    ("missing-colon", b'{"a" 1}'),
    ("trailing-comma-object", b'{"a":1,}'),
    ("trailing-comma-array", b"[1,]"),
    ("leading-zero", b"007"),
    ("bare-word", b"nul"),
    ("trailing-junk", b"null x"),
    ("empty-input", b""),
    ("whitespace-only", b" \t\r\n "),
    ("leading-whitespace", b'  \n\t {"a" : 1}  '),
]

# The program each side runs: read one document named on the command line,
# then print either the canonical form or the rendered bundle. One document
# per process, because the output of a run is then exactly one answer and a
# diff names the case without a line-number convention.
PROGRAM = r'''module Main where
  import Text.Json (parse-json, canonical-json)
  import Text.Parse (ParseOutcome, Parsed, Failed, render-bundle)
  import Data.Bytes
  import Data.Nat
  import Data.Path
  import Effect.Fs
  import Effect.Process

  answer :: forall r. using (io :: Io r) => String ->{IO r} Int
  answer path = case fs-read (path-from-string path) of
    Err e -> 1
    Ok src -> case parse-json path src of
      Parsed v soft -> case ioWrite (string-to-bytes (canonical-json v)) of
        w -> 0
      Failed bundle -> case ioWrite (string-to-bytes (render-bundle bundle)) of
        w -> 0

  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    case process-args () of
      Err e -> 2
      Ok paths -> case paths of
        Nil -> 3
        Cons path rest -> answer path
'''


def toolkit(directory):
    """The P30/P31 toolkit the project ships beside its entry."""
    for name in ("Lex", "Scan", "Parse", "Json", "Builder"):
        shutil.copyfile(ROOT / "compiler" / "prelude" / "Text" / f"{name}.cinch",
                        directory / f"Text.{name}.cinch")


def write_cases(directory):
    """Write every document to its own file; the paths are the arguments."""
    cases = directory / "cases"
    cases.mkdir(parents=True, exist_ok=True)
    paths = []
    for name, data in DOCUMENTS:
        path = cases / (name + ".json")
        path.write_bytes(data)
        paths.append(path)
    return paths


def build(compiler, directory, tag):
    """Compile the program and return the executable."""
    project = directory / tag
    project.mkdir(parents=True, exist_ok=True)
    toolkit(project)
    (project / "Main.cinch").write_text(PROGRAM)
    exe = project / "program"
    env = dict(os.environ)
    env.pop("CINCH_ALLOC_STATS", None)
    completed = subprocess.run([str(compiler), "build", "Main.cinch", "-o", "program"],
                               cwd=project, env=env, capture_output=True, text=True)
    if completed.returncode != 0 or not exe.is_file():
        raise SystemExit(f"{tag}: the differential program did not build\n"
                         f"{completed.stdout}{completed.stderr}")
    return exe


def run(exe, path, work):
    """One document, one answer: the process's whole stdout."""
    completed = subprocess.run([str(exe), str(path)], cwd=work, capture_output=True)
    return completed.returncode, completed.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--compiler", default=None,
                        help="the cinch to build with (default: the stage-3 build)")
    parser.add_argument("--candidate", default=None,
                        help="a second cinch; the reference is --compiler. "
                             "Omit to compare the reference against itself, "
                             "which is the harness's own control.")
    parser.add_argument("--out", default="/tmp/json-differential")
    args = parser.parse_args()

    compiler = Path(args.compiler) if args.compiler else ROOT / "build" / "bootstrap" / "stage3" / "cinch"
    if not compiler.is_file():
        sys.exit(f"no compiler at {compiler}")
    candidate = Path(args.candidate) if args.candidate else compiler
    if not candidate.is_file():
        sys.exit(f"no candidate at {candidate}")

    out = Path(args.out)
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    paths = write_cases(out)

    reference = build(compiler, out, "reference")
    other = build(candidate, out, "candidate") if candidate != compiler else reference

    work = out / "work"
    work.mkdir()
    mismatches, answers = [], {}
    for name, path in zip([n for n, _ in DOCUMENTS], paths):
        ref_code, ref_out = run(reference, path, work)
        cand_code, cand_out = run(other, path, work)
        answers[name] = ref_out
        # The exit code is part of the answer: a case that parses on one side
        # and is refused on the other differs even if both print nothing.
        if ref_out != cand_out or ref_code != cand_code:
            mismatches.append((name, ref_code, ref_out, cand_code, cand_out))

    (out / "answers.json").write_text(
        __import__("json").dumps({k: v.decode("utf-8", "backslashreplace")
                                  for k, v in answers.items()},
                                 indent=1, sort_keys=True))
    if mismatches:
        directory = out / "mismatches"
        directory.mkdir()
        for name, rc, a, cc, b in mismatches:
            (directory / (name + ".txt")).write_bytes(
                f"reference exit {rc}: ".encode() + a + b"\n"
                + f"candidate exit {cc}: ".encode() + b + b"\n")

    print(f"{len(DOCUMENTS)} documents, {len(mismatches)} mismatches")
    for name, rc, a, cc, b in mismatches:
        print(f"  {name}\n    reference {rc} {a!r}\n    candidate {cc} {b!r}")
    print(f"wrote {out}")
    sys.exit(0 if not mismatches else 1)


if __name__ == "__main__":
    main()

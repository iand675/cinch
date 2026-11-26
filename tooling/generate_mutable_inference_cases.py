#!/usr/bin/env python3
"""Project scoped mutation, phantom-kind and strict value-restriction source cases."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "seed/tests/fixtures/mutable-inference-vectors.json"

HELPERS = r'''-- | Recheck source-level mutation regions, definition-site kinds and strict value restrictions.
module Main where
  import Cinch.Ast
  import Cinch.Parse (parse-program)
  import Cinch.Resolve (resolve-program)
  import Cinch.Infer
  import Cinch.Core (CoreData, core-data-name, core-data-params)
  import Cinch.Type (Quant, quant-kind, kind-text)
  import Cinch.Report (Diagnostic, diagnostic-message, render-diagnostics)
  import Text.Parse (ParseOutcome, Parsed, Failed)
  import Data.Json
  import Data.List
  import Data.Option
  import Data.String

  -- | Compare a diagnostic prefix without relying on a locale-specific string search.
  starts :: List Char -> List Char -> Bool
  starts needle haystack = case needle of
    Nil -> True
    Cons first rest -> case haystack of
      Nil -> False
      Cons actual more -> char-code first == char-code actual && starts rest more

  -- | Search a finite diagnostic for a required semantic refusal category.
  contains-chars :: List Char -> List Char -> Bool
  contains-chars needle haystack = if starts needle haystack then True else case haystack of
    Nil -> False
    Cons first rest -> contains-chars needle rest

  -- | Require a named refusal rather than treating an unrelated parse failure as a successful negative test.
  diagnosed :: String -> List Diagnostic -> Bool
  diagnosed expected errors = case errors of
    Nil -> False
    Cons first rest -> contains-chars (unpack expected) (unpack (diagnostic-message first)) || diagnosed expected rest

  -- | Find the exact definition-site nominal identity in checked Core data declarations.
  find-data :: String -> List CoreData -> Option CoreData
  find-data identity datas = case datas of
    Nil -> None
    Cons first rest -> if core-data-name first == identity then Some first else find-data identity rest

  -- | Observe a list without relying on a precompiled element-representation variant.
  empty-list :: forall a. List a -> Bool
  empty-list values = case values of Nil -> True; Cons first rest -> False

  -- | Inspect every actual checked parameter kind, including ordinary phantom Type parameters.
  parameter-kinds :: List Quant -> List String -> Bool
  parameter-kinds params expected = case params of
    Nil -> empty-list expected
    Cons first rest -> case expected of
      Nil -> False
      Cons wanted more -> kind-text (quant-kind first) == wanted && parameter-kinds rest more

  -- | Verify producer kinds rather than inferring success from consumer acceptance alone.
  checked-kinds :: List CoreData -> List (String, List String) -> Bool
  checked-kinds datas expected = case expected of
    Nil -> True
    Cons first rest -> case first of
      (identity, kinds) -> case find-data identity datas of
        None -> False
        Some declaration -> parameter-kinds (core-data-params declaration) kinds && checked-kinds datas rest

  -- | Distinguish a valid producer, a required type refusal and unrelated diagnostics.
  result-ok :: Bool -> String -> List (String, List String) -> Elaboration -> Bool
  result-ok accepted expected kinds result =
    let errors = elaboration-errors result in
    if accepted then empty-list errors && checked-kinds (elaboration-data result) kinds
    else diagnosed expected errors

  -- | Parse and resolve a real source program before checking its typing and nominal contracts.
  evaluate :: String -> Bool -> String -> List (String, List String) -> (Bool, String)
  evaluate source accepted expected kinds = case parse-program "mutable-inference" (string-to-bytes source) of
    Parsed program Nil -> if program-unmodeled-count program == 0 then
      case resolve-program program of
        (resolved, Nil) ->
          let result = elaborate-program "mutable-inference" resolved in
          let errors = elaboration-errors result in
          let ok = result-ok accepted expected kinds result in
          (ok, render-diagnostics errors)
        other -> (False, "name resolution failed")
      else (False, "unmodeled source")
    other -> (False, "source did not parse")

  -- | Retain the first source-case failure as a small deterministic process exit status.
  first-failure :: List Bool -> Nat -> Int
  first-failure values index = case values of
    Nil -> 0
    Cons first rest -> if first then first-failure rest (index + 1) else nat-to-int index

  -- | Observe one checked verdict independently of its optional diagnostic payload.
  verdict :: (Bool, String) -> Bool
  verdict result = case result of (ok, details) -> ok
'''


def linked(values: list[str]) -> str:
    """Render a finite literal list in its original observation order."""
    result = "Nil"
    for value in reversed(values):
        result = f"(Cons ({value}) {result})"
    return result


def source() -> str:
    """Use exact offline source bytes and expected kinds without requiring an SDK or compiler at projection time."""
    rows = json.loads(FIXTURE.read_text())
    declarations = []
    calls = []
    for index, row in enumerate(rows):
        kinds = linked([
            f"({json.dumps(name)}, {linked([json.dumps(kind) for kind in params])})"
            for name, params in row["kinds"].items()
        ])
        declarations += [
            f"  -- | Check the {row['name']} source and its exact nominal-kind obligations.",
            f"  check-case{index} :: Unit -> Bool",
            f"  check-case{index} unit = verdict (evaluate {json.dumps(row['source'])} "
            f"{'True' if row['accepted'] else 'False'} {json.dumps(row['diagnostic'])} {kinds})",
            "",
        ]
        calls.append(f"check-case{index} ()")
    return HELPERS + "\n" + "\n".join(declarations) + (
        "\n  -- | Run every source and authority check, preserving its original case number on failure.\n"
        "  main :: Int\n"
        f"  main = first-failure {linked(calls)} 1\n"
    )


if __name__ == "__main__":
    print(source(), end="")

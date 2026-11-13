# `conditional_surface` expectations

What each of the 20 tests in `compiler/tests/conditional_surface.rs` needs, and
whether the staged CLI can meet it. Written before the layer compiles, so the
list is a prediction to be checked against the first green build rather than a
description written after the fact. Every claim marked `[INFERENCE]` is a
prediction; everything unmarked is read off the test source, the pinned
interface set, or `bootstrap/*.cinch`.

State at the time of writing: 2 passed, 19 failed (the 2 are
`a_multi_clause_definition_is_one_definition` and
`the_condition_context_is_the_one_place_a_position_picks_its_profile`, both of
which need no `--target` flag and no `when` clause).

## The three capabilities the tests actually need

The 19 failures are not one gap. They are three, and only the first is in
`Bootstrap.Target`:

**A. In-source `when`.** A `when <atom>` clause on an import or a declaration,
selected against the target profile, with the disjointness rule for a name
defined twice. Delivered by `Bootstrap.Target` (selection, atom evaluation,
disjointness, target view) plus `MacroCall`'s parse layer (`DConditional`,
`parse-conditional-decl`, `import-when-clauses`).

**B. Package manifests.** A `.cinchpkg` whose `select` entries name modules
per `when: {os: [...]}` condition. The staged pipeline never opens a
`.cinchpkg`: `grep -c cinchpkg bootstrap/*.cinch` is zero across every module.
Module availability from a manifest is therefore absent, not merely
target-blind, and no amount of `when` support substitutes for it.

**C. Two profiles, not one.** A build carries a host (where compile-time code
runs) and a target (which the artifact is emitted for), and the tests assert
refusals that name both, plus artifact-manifest rows recording each module's
availability under each. The staged pipeline has one profile and its
`build-manifest` emits `target { triple, word_size, endian }` with no `cross`
flag and no per-module availability rows.

**D. Prelude re-derivation.** The facade `Prim.Sys` is a source module with
three conditional imports. In the staged CLI it arrives as a pinned
`.cinchif` built on this host, so its recorded surface and imports are the
host's (Darwin), whatever target is requested. Re-deriving it per target needs
the prelude *sources* in the compiler, which `collect_compiler_sources` does
not bundle: it takes `bootstrap/*.cinch` and five `Text.*` modules only.

## Per test

| # | Test | Source shape | Needs | Expectation |
|---|---|---|---|---|
| 1 | `a_false_conditional_import_leaves_its_names_unknown` | `import Prim.Sys.Linux when not (os darwin)`, uses `sys-open-read`, Darwin | A | **pass** |
| 2 | `the_complementary_condition_resolves_the_name` | same import as `os darwin` on Darwin; `not (os darwin)` on Linux | A | **pass** |
| 3 | `two_clauses_on_one_import_are_a_conjunction` | `import Prim.Sys.Darwin when os darwin when arch aarch64`, true on Darwin, false on Linux | A | **pass** |
| 4 | `a_false_conditional_import_is_not_a_module_dependency` | `import Data.List when os linux` on Darwin | A | **pass** |
| 5 | `a_false_conditional_declaration_is_absent_from_the_interface` | `when os linux p :: I64` / `= 1` and `when os darwin d :: I64` / `= 2`; reads `Main`'s interface values per target | A | **pass** `[INFERENCE]`: depends on the interface writer recording only the pruned view, which I have not traced end to end |
| 6 | `a_false_conditional_declaration_is_absent_from_code_generation` | same declarations, then `probe = platform-word`; error on Darwin, checks on Linux | A | **pass** |
| 7 | `two_definitions_with_disjoint_conditions_select_one` | `when os darwin word` + `when os linux word`, each sig and body, `export word` | A | **pass** |
| 8 | `two_definitions_whose_conditions_overlap_are_refused` | `when os darwin word` and `when os darwin when arch aarch64 word`; exact message with both conditions and the shared atom | A | **pass** |
| 9 | `a_conditional_definition_may_not_overlap_a_plain_one` | plain `word` plus `when os darwin word`; message names `unconditional` | A | **pass** |
| 10 | `the_interface_records_only_the_selected_import` | two mutually exclusive backend imports in `Main`; reads `Main`'s recorded imports | A | **pass** |
| 11 | `the_filesystem_home_reaches_the_platform_layer_through_the_facade` | no source condition; inspects `Effect.Fs`'s imports from the pinned set | A (profile accepted) | **pass**, measured: the pinned `Effect.Fs.cinchif` imports `Prim.Sys` and neither backend, and that list does not depend on the target |
| 12 | `the_facade_interface_names_one_backend` | no source condition; reads `Prim.Sys`'s imports on Darwin and on Linux | D | **Darwin half passes, Linux half cannot**, measured: the pinned `Prim.Sys.cinchif` imports exactly `Data.Core`, `Data.String`, `Prim.Sys.Darwin`, so it names Darwin on both targets |
| 13 | `the_facade_surface_is_the_selected_backends_surface` | compares `Prim.Sys`'s values with `Prim.Sys.Darwin`'s and with `Prim.Sys.Linux`'s | D | **Darwin half passes, Linux half cannot**, same measured reason: 18 values in the pinned facade, matching the Darwin backend, never the Linux one |
| 14 | `a_target_with_neither_backend_reaches_the_portable_floor` | hand-built Windows profile (not a triple), expects `Prim.Sys` to select `Prim.Sys.Portable` | D, plus a profile channel that accepts a non-triple profile | **cannot pass**. The triple is unsupported, so `profile_of_triple` refuses it by name; and even with a channel, `Prim.Sys` is not re-derived |
| 15 | `compile_time_code_that_reaches_a_host_only_module_is_refused_naming_both_profiles` | `.cinchpkg` `select` on the host OS; a `stage` definition reaches the module | B, C | **cannot pass** |
| 16 | `emitted_code_that_reaches_a_host_only_module_is_refused_as_unavailable_for_the_target` | same manifest; ordinary code reaches the module | B | **cannot pass** |
| 17 | `compile_time_code_that_reaches_a_target_only_module_names_the_host_context` | `.cinchpkg` `select` on the other arch; a `stage` definition reaches it | B, C | **cannot pass** |
| 18 | `the_same_staged_source_builds_for_the_hosts_own_target` | `.cinchpkg` `select` on the host OS; asserts the artifact manifest carries `"cross":false` and a `{"host":...,"target":...}` row | B, C | **cannot pass** |
| 19 | `one_build_selects_different_modules_for_the_host_and_the_target` | `.cinchpkg` `select` for both OSes plus in-source `when`; asserts `cross:true` and both per-module rows | B, C | **cannot pass**. The in-source half would work; the manifest half cannot |

Predicted total once this layer builds: **13 passed, 7 failed** (tests 12, 13
partially and 14 through 19 wholly). Baseline is 1 of 20 and the harness fix
alone reaches 2 of 20, so the target is strictly better either way.

## What would move the remaining 7

- **Tests 15 to 19** need the staged pipeline to read a `.cinchpkg` and apply
  its `select` conditions, and the artifact manifest to record a host and a
  target availability per module with a `cross` flag. That is a manifest and
  artifact-schema change, not a `when` change.
- **Tests 12 and 13** need the prelude sources in the compiler's own source
  set so `Prim.Sys` is checked against the requested target instead of
  arriving as the host's pinned interface. That is `collect_compiler_sources`
  plus the `embedded-homes` list, and it is the same prerequisite as test 14.
- **Test 14** additionally needs a way to name a profile that is not one of
  the five supported triples. The test builds the Windows profile directly
  rather than through `--target`, so this is a harness-to-CLI channel
  question, not a grammar question.

## One thing this list corrects

I earlier assumed the 19 failures were "`--target` plus `when`". They are not:
11 are, 3 are the prelude facade, and 6 are package manifests and two-profile
artifacts. Reporting "19 of 20 blocked on `when`" would have been wrong, and
the build would have shown it.

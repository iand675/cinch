# Self-hosted Cinch compiler (`compiler/`)

`compiler/` contains the Cinch compiler written in Cinch: 65 `Cinch.*.cinch` modules implementing lexing, layout, parsing, macro expansion, module and interface loading, type/effect/refinement/proof/ownership/temporal checking, Core verification, and typed LLVM IR + native object emission.

Both `compiler/` and the Rust seed (`seed/`) speak **interface schema 10** and **runtime ABI 10** (`Cinch.RuntimeAbi.cinch`), including memory-mapped packed interfaces (`.cpk`).

## Two compiler trees

- **`compiler/` (this directory)** is the self-hosted Cinch compiler. All 65 compiler modules use the `Cinch.` namespace and can import any module from `seed/prelude/` via automatic stdlib closure bundling.
- **`seed/` (beside this directory)** is the Rust bootstrap seed: it builds Stage 1 from scratch, owns the `cinchruntime` archive, and holds the `seed/prelude/` standard library sources.

## Directory layout

`tooling/check-compiler-layout.py` enforces a clean top level:

| Path | Contents |
| --- | --- |
| `Cinch.*.cinch` | The 65 compiler implementation modules (one module per file matching its dotted name) |
| `programs/` | Standalone rung drivers (`Run.<Rung>.cinch`) and counterexample fixtures |
| `pins/` | Lockfiles (`stage0.lock.json`, `library-provider.lock.json`) and `--object-roots` tables |
| `evidence/` | Recorded verification receipts grouped by subject (`current/`, `stdlib/`, `native/`, `runtime/`, `other/`) |
| `notes/` | Detailed working notes and implementation reviews |
| `patches/`, `proposals/`, `repros/` | Review patches, proposals, and minimized reproducers |
| `seed/`, `status-ui/` | Stage 0 seed patch (`seed/stage0.patch`) and bootstrap status dashboard |
| `CHECKPOINT.md`, `CONFORMANCE.md` | Latest bootstrap checkpoint log and specification conformance matrix |

### Module architecture

| Pipeline stage | Modules |
| --- | --- |
| **Syntax & AST** | `Cinch.Ast`, `Cinch.Parse`, `Cinch.Layout`, `Cinch.Report`, `Cinch.Document`, `Cinch.Target`, `Cinch.TargetLiterals`, `Cinch.Expand` |
| **Resolution & interfaces** | `Cinch.Resolve`, `Cinch.Nominal`, `Cinch.Load`, `Cinch.Interface`, `Cinch.Manifest`, `Cinch.Package`, `Cinch.PackageHash`, `Cinch.PackageWrite`, `Cinch.PackedBody`, `Cinch.Export`, `Cinch.ExportNominal`, `Cinch.ExportType`, `Cinch.Inputs`, `Cinch.Index` |
| **Static checking & Core** | `Cinch.Builtin`, `Cinch.Contracts`, `Cinch.BodyContract`, `Cinch.BodyScope`, `Cinch.Type`, `Cinch.Promote`, `Cinch.Infer`, `Cinch.Instance`, `Cinch.Dictionary`, `Cinch.Check`, `Cinch.Core`, `Cinch.CoreBody`, `Cinch.Global`, `Cinch.Kinds`, `Cinch.Typing`, `Cinch.Foreign`, `Cinch.Logical`, `Cinch.Proof`, `Cinch.Eval`, `Cinch.Total`, `Cinch.TotalRules`, `Cinch.TotalSemantics`, `Cinch.Guarded`, `Cinch.Eager`, `Cinch.Usage`, `Cinch.Borrow`, `Cinch.Ownership`, `Cinch.Temporal`, `Cinch.TemporalCheck`, `Cinch.Unpack`, `Cinch.UnpackMatch`, `Cinch.Stage`, `Cinch.StageEval`, `Cinch.Ad` |
| **Parallel, incremental & codegen** | `Cinch.Parallel`, `Cinch.Incremental`, `Cinch.RuntimeAbi`, `Cinch.Ir`, `Cinch.RuntimeIr`, `Cinch.Object`, `Cinch.Driver`, `Cinch.Pipeline`, `Cinch.Test` |

`Cinch.Ast` and `Cinch.Report` form the shared frontend/backend contract. Explicit `*Unmodeled` AST nodes ensure any parsed grammar form not yet modeled is counted (`program-unmodeled-count`) and refused by name rather than silently dropped.

## The four-stage bootstrap ladder

Self-hosting builds progress through four isolated stages under `build/bootstrap/stageN/`:

1. **Stage 0 (pinned Rust seed):** built from the revision, patch, and hashes pinned in `pins/stage0.lock.json`. Local edits to `seed/` do not silently alter Stage 0.
2. **Stage 1 (Cinch built by Stage 0):** compiles the frozen `Cinch.*.cinch` snapshot with Stage 0 and links against the pinned `--release` `cinchruntime`.
3. **Stage 2 (Cinch built by Stage 1):** compiles the same frozen snapshot using Stage 1 as the sole frontend (`--emit-ir-with-interfaces`), linking with the prebuilt base library object from `pins/library-provider.lock.json` (which provides base library units only, never `Cinch.*`).
4. **Stage 3 (fixed-point verification):** compiles the frozen snapshot again with Stage 2 and asserts byte-identical LLVM IR, compiled interfaces, and linked executables between Stage 2 and Stage 3, followed by a changed-source diagnostic probe.

Run the ladder via `tooling/bootstrap.sh`:

```sh
tooling/bootstrap.sh build --stage 0   # build pinned Rust seed
tooling/bootstrap.sh build --stage 1   # build Stage 1 Cinch compiler
tooling/bootstrap.sh build --stage 2   # build Stage 2 with Stage 1
tooling/bootstrap.sh smoke --stage 1   # run positive + refusal smoke programs
tooling/bootstrap.sh check             # build Stage 3, verify fixed point and changed-source probe
tooling/bootstrap.sh status            # print current stage build status
```

Each invocation freezes its compiler source snapshot up front so concurrent edits cannot mix versions mid-chain. Missing base-library specialization variants are reported in `requested-roots.json`.

## Self-hosted driver CLI modes

```sh
cinch --check [--interfaces MANIFEST] SOURCE DIAGNOSTICS_BASE
cinch --emit-ir [--interfaces MANIFEST] SOURCE IR
cinch --emit-interfaces [--interfaces PACKAGE] SOURCE PREFIX
cinch --emit-ir-only --interfaces MANIFEST --objects CONFIG SOURCE IR
cinch --serve-ir --interfaces PACKAGE.cpk --objects CONFIG SOURCE IR
cinch --emit-ir-with-interfaces --interfaces MANIFEST --objects CONFIG SOURCE IR
cinch --emit-object --interfaces MANIFEST --objects CONFIG SOURCE IR OBJECT PACKAGE.cpk
cinch --compile-only [--interfaces MANIFEST] SOURCE IR EXECUTABLE REPORT
cinch --interfaces PACKAGE.cpk --objects CONFIG build [--clang PATH] RUNTIME PREFIX SOURCE
cinch --interfaces PACKAGE.cpk --objects CONFIG run [--clang PATH] RUNTIME PREFIX SOURCE [-- ARGS...]
```

- `--interfaces` accepts either a JSON manifest (`{"schema":1,"interfaces":[["Module","hash","path"]]}`) or a memory-mapped `.cpk` package. Transitive dependencies are verified before parsing so fixities come from checked interfaces.
- Packed `build` and `run` link against the authenticated base object, write `PREFIX.exe`, and reuse `PREFIX.ll` when `PREFIX.ll.receipt` matches compiler, source, target, flags, and IR.
- `--serve-ir` keeps decoded contracts and object metadata resident in memory across edits: wait for `ready` on stdout, send a newline on stdin after each save, and read the status line back. Close stdin to exit.
- `--emit-object` emits `.o`, `.ll`, and `.cpk` outputs (pass `-` as `OBJECT` to emit IR plus the contract/ABI delta package for external linking).

## Verification

Full end-to-end verification (frozen snapshot, private caches, serial compiler/runtime/tooling/bootstrap gates through Stage 3):

```sh
python3 tooling/verify_bootstrap.py --evidence /tmp/cinch-verification-NAME
```

Focused checks:

```sh
tooling/run-bootstrap-ladder.sh                           # type-check all compiler modules
tooling/run-bootstrap-ladder.sh --all-programs            # build and run every rung program
tooling/run-bootstrap-ladder.sh programs/Run.Parse.cinch  # run a single rung program
python3 tooling/check-compiler-layout.py                  # enforce directory layout rules
python3 -m unittest discover -s tooling/tests -v          # bootstrap orchestration tests
cargo test --manifest-path seed/Cargo.toml --test bootstrap_contract
```

### Routing Rust integration tests through a staged compiler

`seed/tests/common/mod.rs` routes portable integration tests through an explicit staged binary when both variables are set:

- `CINCH_TEST_COMPILER`: path to the staged Cinch compiler binary
- `CINCH_TEST_INTERFACES`: path to the checked base-library interface manifest or `.cpk`
- `CINCH_TEST_OBJECTS` (optional, for `--emit-object` tests): object provider configuration

Failures report directly without falling back to the Rust frontend, and test artifacts land in `.cinch-self-tests/`.

## Coding discipline for `Cinch.*.cinch`

Modules in `compiler/` follow the bootstrap subset rules:

- **Full equation arity:** bind the exact number of parameters matching the signature's arrow count (eta-expand partial applications).
- **Top-level polymorphism only:** lift polymorphic or `using`-constrained helpers to top-level declarations.
- **Flat acyclic modules:** one module per file (`Cinch.Foo.cinch` declares `module Cinch.Foo where`) with no import cycles.
- **Declaring-module fixity:** declare operator fixities only in the defining module.
- **Case patterns:** match constructors, tuples, lists, and integer literals; compare strings/chars with `string-compare`, `eq-char`/`char-code`, or intern to `Sym`.
- **Escaped string literals:** standard escapes only (`\n`, `\r`, `\t`, `\\`, `\"`, `\'`, `\u{HEX}`).
- **Reserved keywords:** never use keywords (`as`, `implicit`, `target`, `data`, `type`, `to`, `stage`, `spec`, `macro`, `law`, `prim`, `using`, `forall`, `import`, `module`, `if`, `then`, `else`, `where`, `in`, `of`, `let`, `case`) as binders.
- **Recursive ADT spines:** represent recursive sequences inside mutually recursive groups with explicit spine constructors (see `*-spine` types in `Cinch.Ast`).
- **Guarded recursion:** recurse through top-level functions, lambdas, or `letrec`; eager cyclic value bindings are rejected.
- **ASCII dashes only:** avoid em dashes and en dashes in Cinch source and comments.

## Status and tracking

- **Latest checkpoint and Stage 3 grid results:** [`CHECKPOINT.md`](CHECKPOINT.md)
- **Specification conformance matrix:** [`CONFORMANCE.md`](CONFORMANCE.md)
- **Self-hosting architecture and acceptance criteria:** [`design/self-hosting.md`](../design/self-hosting.md)
- **Machine-readable verification receipts:** `compiler/evidence/current/`

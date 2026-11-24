# Self-hosted Cinch compiler (`compiler/`)

`compiler/` contains the Cinch compiler written in Cinch itself: 59 `Bootstrap.*.cinch` modules implementing lexing/parsing, module and interface loading, type/effect/refinement/proof/usage checking, Core verification, and typed LLVM IR + native object emission.

Both `compiler/` and the Rust seed (`seed/`) currently target **interface schema 10** and **runtime ABI 10** (`Bootstrap.RuntimeAbi.cinch`), with support for memory-mapped packed interfaces (`.cpk`; see [notes/package-interfaces.md](notes/package-interfaces.md)).

## Two compiler trees

- **`compiler/` (this directory)** is the language's compiler: `Bootstrap.*.cinch` compiles Cinch source to LLVM IR, compiled interfaces, and native objects. Modules retain the `Bootstrap.` namespace prefix because renaming them changes compiled interface identities.
- **`seed/` (beside this directory)** is the Rust bootstrap seed: it builds Stage 1 on hosts that cannot yet run Cinch natively, owns the `cinchruntime` archive, and embeds the `prelude/` library sources.

## Directory and module map

Top-level files in `compiler/` are restricted to the `Bootstrap.*.cinch` sources and top-level docs; `tooling/check-compiler-layout.py` enforces this layout:

| Path | Contents |
| --- | --- |
| `Bootstrap.*.cinch` | The 59 compiler implementation modules (flat files matching dotted module names) |
| `programs/` | Standalone rung drivers (`Run.<Rung>.cinch`) and counterexample fixtures |
| `pins/` | Lockfiles (`stage0.lock.json`, `library-provider.lock.json`) and `--object-roots` tables |
| `evidence/` | Recorded verification receipts grouped by subject (`current/`, `stdlib/`, `native/`, `runtime/`, `other/`) |
| `notes/` | Architecture notes, implementation plans, and design reviews |
| `patches/`, `proposals/`, `repros/` | Review patches, proposals, and minimized reproducers |
| `seed/`, `status-ui/` | Stage 0 seed patch (`seed/stage0.patch`) and bootstrap status UI |
| `CHECKPOINT.md`, `CONFORMANCE.md` | Current bootstrap checkpoint log and full specification conformance matrix |

### Module architecture

| Pipeline stage | Modules |
| --- | --- |
| **Syntax & AST** | `Bootstrap.Ast`, `Bootstrap.Parse`, `Bootstrap.Layout`, `Bootstrap.Report`, `Bootstrap.Document`, `Bootstrap.Target`, `Bootstrap.TargetLiterals`, `Bootstrap.Expand` |
| **Resolution & interfaces** | `Bootstrap.Resolve`, `Bootstrap.Nominal`, `Bootstrap.Load`, `Bootstrap.Interface`, `Bootstrap.Manifest`, `Bootstrap.Package`, `Bootstrap.PackageHash`, `Bootstrap.PackageWrite`, `Bootstrap.PackedBody`, `Bootstrap.Export`, `Bootstrap.ExportNominal`, `Bootstrap.ExportType`, `Bootstrap.Inputs`, `Bootstrap.Index` |
| **Static checking & Core** | `Bootstrap.Builtin`, `Bootstrap.Contracts`, `Bootstrap.BodyContract`, `Bootstrap.BodyScope`, `Bootstrap.Type`, `Bootstrap.Infer`, `Bootstrap.Instance`, `Bootstrap.Dictionary`, `Bootstrap.Check`, `Bootstrap.Core`, `Bootstrap.CoreBody`, `Bootstrap.Global`, `Bootstrap.Kinds`, `Bootstrap.Typing`, `Bootstrap.Foreign`, `Bootstrap.Logical`, `Bootstrap.Proof`, `Bootstrap.Eval`, `Bootstrap.Total`, `Bootstrap.TotalRules`, `Bootstrap.TotalSemantics`, `Bootstrap.Guarded`, `Bootstrap.Eager`, `Bootstrap.Usage`, `Bootstrap.Temporal`, `Bootstrap.TemporalCheck`, `Bootstrap.Unpack`, `Bootstrap.UnpackMatch`, `Bootstrap.Stage`, `Bootstrap.StageEval` |
| **Codegen & driver** | `Bootstrap.RuntimeAbi`, `Bootstrap.Ir`, `Bootstrap.RuntimeIr`, `Bootstrap.Object`, `Bootstrap.Driver`, `Bootstrap.Pipeline`, `Bootstrap.Test` |

`Bootstrap.Ast` and `Bootstrap.Report` form the shared frontend/backend contract. `Bootstrap.Ast` includes explicit `*Unmodeled` nodes so any grammar form recognized by the parser but not yet modeled is counted (`program-unmodeled-count`) and rejected by name in the checker rather than silently dropped.

## The bootstrap stage ladder

Self-hosting builds progress through four isolated stages under `build/bootstrap/stageN/`:

1. **Stage 0 (pinned Rust seed):** built from the Git revision, patch, and source hashes pinned in `pins/stage0.lock.json`. Local edits to `seed/` do not silently mutate Stage 0.
2. **Stage 1 (Cinch built by Stage 0):** compiles `Bootstrap.*.cinch` with Stage 0 and links against the pinned `--release` `cinchruntime` archive.
3. **Stage 2 (Cinch built by Stage 1):** compiles the same frozen `Bootstrap.*.cinch` bundle using Stage 1 as the sole frontend, emitting typed LLVM IR and checked interfaces (`--emit-ir-with-interfaces`) linked with the prebuilt base library object from `pins/library-provider.lock.json` (which builds base library modules only, never `Bootstrap.*`).
4. **Stage 3 (fixed-point verification):** compiles the frozen bundle again with Stage 2 and asserts byte-identical LLVM IR and compiled interfaces between Stage 2 and Stage 3, plus a changed-source diagnostic probe confirming that edits to the compiler change the resulting executable's behaviour.

Run the ladder with `tooling/bootstrap.sh`:

```sh
tooling/bootstrap.sh build --stage 0   # build pinned Rust seed
tooling/bootstrap.sh build --stage 1   # build Stage 1 Cinch compiler
tooling/bootstrap.sh build --stage 2   # build Stage 2 with Stage 1
tooling/bootstrap.sh smoke --stage 1   # run positive + refusal smoke programs
tooling/bootstrap.sh check             # build Stage 3, verify fixed point and changed-source probe
tooling/bootstrap.sh status            # print current stage build status
```

Each invocation freezes its compiler source snapshot once so concurrent edits cannot mix versions mid-chain. Missing concrete dependency variants from the base library object are reported in `requested-roots.json`.

## Self-hosted driver CLI modes

The compiled Cinch driver supports standalone frontend, packed-interface, and persistent-server modes:

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

- `--interfaces` accepts either a JSON manifest (`{"schema":1,"interfaces":[["Module","hash","path"]]}`) or a memory-mapped `.cpk` package. The loader verifies the full transitive dependency closure before parsing source so operator fixities come from checked interfaces.
- Packed `build` and `run` link against the authenticated base object, write `PREFIX.exe`, and reuse `PREFIX.ll` when `PREFIX.ll.receipt` matches the compiler, source, target, flags, and IR.
- `--serve-ir` keeps decoded contracts and validated object metadata resident in memory across edits: wait for `ready` on stdout, send one newline on stdin after each save of `SOURCE`, and read the status line back. Close stdin to exit.
- `--emit-object` emits `.o`, `.ll`, and `.cpk` outputs (pass `-` as `OBJECT` to emit IR plus the contract/ABI delta package for external linking).

## Building and running verification

Full end-to-end verification (fresh source snapshot, isolated caches, sequential compiler/runtime/tooling/bootstrap gates through Stage 3 fixed point):

```sh
python3 tooling/verify_bootstrap.py --evidence /tmp/cinch-verification-NAME
```

Run individual rung programs (`compiler/programs/Run.<Rung>.cinch`), tooling tests, layout checks, and the Rust contract test:

```sh
tooling/run-bootstrap-ladder.sh                           # type-check all compiler modules
tooling/run-bootstrap-ladder.sh --all-programs            # build and run every rung program
tooling/run-bootstrap-ladder.sh programs/Run.Parse.cinch  # run a single rung program
python3 tooling/check-compiler-layout.py                  # enforce directory layout rules
python3 -m unittest discover -s tooling/tests -v          # bootstrap orchestration tests
cargo test --manifest-path seed/Cargo.toml --test bootstrap_contract
```

### Running portable Rust tests through a staged Cinch compiler

The shared test harness in `seed/tests/common/mod.rs` can route portable integration tests through an explicit Cinch compiler binary instead of the Rust seed by setting:

- `CINCH_TEST_COMPILER`: path to the staged Cinch compiler binary
- `CINCH_TEST_INTERFACES`: path to the checked base-library interface manifest or `.cpk`
- `CINCH_TEST_OBJECTS` (for `--emit-object` tests): object provider configuration

Both `CINCH_TEST_COMPILER` and `CINCH_TEST_INTERFACES` must be set together. Failures are reported directly without falling back to the Rust frontend, and artifacts are recorded under `.cinch-self-tests/`. `seed/examples/bootstrap_external_probe.rs` exercises `--check`, `--compile-only`, and `--emit-ir` through an external Cinch binary.

## Coding discipline for `Bootstrap.*.cinch`

When editing modules in `compiler/`, follow the subset rules enforced across the bootstrap chain:

- **Full equation arity:** every equation must bind the exact number of parameters matching its signature's arrow count (eta-expand partial applications).
- **Top-level polymorphism only:** do not write local signatures with `forall` or `using`; lift polymorphic/constrained helpers to top-level definitions.
- **Flat acyclic modules:** one module per file (`Bootstrap.Foo.cinch` declares `module Bootstrap.Foo where`) with no import cycles. Place shared types in an upstream module and re-export as needed.
- **Declaring-module fixity:** declare operator fixities only in the module that defines the operator.
- **Case patterns:** `case` matches constructors, tuples, lists, and integer literals, not string, character, or float literals (compare with `string-compare`, `eq-char`/`char-code`, or intern to `Sym`).
- **Escaped string literals:** no raw strings; use standard escapes (`\n`, `\r`, `\t`, `\\`, `\"`, `\'`, `\u{HEX}`).
- **Reserved keywords:** never use keywords (`as`, `implicit`, `target`, `data`, `type`, `to`, `stage`, `spec`, `macro`, `law`, `prim`, `using`, `forall`, `import`, `module`, `if`, `then`, `else`, `where`, `in`, `of`, `let`, `case`) as binders.
- **Recursive ADT spines:** a `data` declaration cannot reference a type declared later or nest itself inside another type constructor; represent recursive sequences with explicit spine constructors (see `*-spine` builders in `Bootstrap.Ast`).
- **Guarded recursion:** eager value reference cycles are rejected; recurse through top-level functions, lambdas, or `letrec`.
- **ASCII dashes only:** do not use em dashes or en dashes in Cinch source or comments.

## Current status and tracking

The self-hosted compiler reaches a verified four-stage fixed point on **schema 10 / ABI 10** (including rank-N polymorphism and packed `.cpk` interfaces), with pinned Stage 0 and library-provider revisions in `compiler/pins/`.

- **Latest checkpoint, open issues, and Stage 3 grid results:** [`compiler/CHECKPOINT.md`](CHECKPOINT.md)
- **Specification-to-implementation conformance matrix:** [`compiler/CONFORMANCE.md`](CONFORMANCE.md)
- **Self-hosting architecture and acceptance criteria:** [`design/self-hosting.md`](../design/self-hosting.md)
- **Machine-readable verification receipts:** `compiler/evidence/current/`

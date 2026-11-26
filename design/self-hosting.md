# Self-hosting and bootstrap architecture

Cinch is self-hosting: the compiler in `compiler/` (65 `Bootstrap.*.cinch` modules) compiles itself to native executables through a four-stage ladder and reaches a verified byte-for-byte fixed point between Stage 2 and Stage 3 on **interface schema 10** and **runtime ABI 10**.

Operational commands and module maps live in [`compiler/README.md`](../compiler/README.md). Current checkpoint notes live in [`compiler/CHECKPOINT.md`](../compiler/CHECKPOINT.md).

---

## 1. Two compiler trees, distinct roles

- **`compiler/`** is the Cinch compiler written in Cinch: lexer, layout, parser, macro expansion, module/interface loader, type/effect/refinement/proof/ownership/temporal checker, Core validator, and typed LLVM IR + object emitter.
- **`seed/`** is the Rust bootstrap compiler: it builds Stage 1 from scratch on hosts without an existing Cinch binary, owns the `cinchruntime` library (`seed/runtime/`), embeds the `cinch-base` prelude sources (`seed/prelude/`), and hosts interactive tooling (`repl`, `lsp`, `doc`, `pkg`, `jit`, `cxx`).

---

## 2. The four-stage bootstrap ladder

Orchestrated by `tooling/bootstrap.py` (wrapped by `tooling/bootstrap.sh`) under `build/bootstrap/stageN/`:

1. **Stage 0 (pinned Rust seed):** built from the exact Git revision, patch, and source hashes pinned in `compiler/pins/stage0.lock.json`. Local edits to `seed/` never silently alter Stage 0.
2. **Stage 1 (Cinch built by Stage 0):** freezes a snapshot of `compiler/Bootstrap.*.cinch`, compiles it with Stage 0, and links against the pinned `--release` `cinchruntime`.
3. **Stage 2 (Cinch built by Stage 1):** compiles the same frozen `Bootstrap.*.cinch` snapshot using Stage 1 as the sole frontend (`--emit-ir-with-interfaces`), linking against the prebuilt base library object pinned in `compiler/pins/library-provider.lock.json` (which provides base library units only, never `Bootstrap.*`).
4. **Stage 3 (fixed-point verification):** compiles the frozen snapshot again with Stage 2 and verifies byte-for-byte identity between Stage 2 and Stage 3 on:
   - emitted LLVM IR (`compiler.ll`),
   - all compiled compiler interfaces (`.cinchif`),
   - build manifests and linked executables (using identical deterministic IR shard counts and content-addressed PGO paths).

A **changed-source probe** then mutates a diagnostic label in the checker, rebuilds through the chain, and verifies that the new compiler emits the mutated diagnostic—proving the chain genuinely compiles its inputs rather than replaying a cached binary.

---

## 3. Verification and Nix caching

- **End-to-end verification:** `python3 tooling/verify_bootstrap.py --evidence /tmp/cinch-verify` captures a frozen copy of the repository with private Cargo caches and runs all compiler, runtime, malleable-syntax, tooling, smoke, fixed-point, and Stage 3 frontend gates serially.
- **Stage test grid:** `tooling/run_stage_targets.py --stage 3` routes portable integration suites in `seed/tests/` through a staged Cinch binary via `CINCH_TEST_COMPILER` + `CINCH_TEST_INTERFACES` without falling back to the Rust frontend.
- **Nix binary cache:** [`tooling/nix/README.md`](../tooling/nix/README.md) splits `cinch-seed`, `cinch-provider`, `cinch-pins`, and `cinch-ladder` into independently cacheable derivations pushed to Cachix by `.github/workflows/ladder-cache.yml` and promoted to GitHub nightlies by `.github/workflows/publish-nightly.yml`.

---

## Acceptance

1. The Rust seed compiles the Cinch compiler sources into Stage 1.
2. Stage 1 reads and compiles those same sources into Stage 2, rebuilding every compiler module. It may use `clang`, the runtime, and checked base-library artifacts as the ABI design permits, but must never invoke the Rust compiler frontend or substitute prebuilt compiler code.
3. Stage 2 repeats the build into Stage 3. The compiler executables, canonical interfaces, and emitted IR agree byte for byte between Stages 2 and 3, and both executables run.
4. A changed compiler source is exercised through the chain so reusing an old artifact or emitting a fixed copy of the compiler cannot pass.
5. The complete compiler and runtime test suites pass, including previously ignored tests. External compiler behavior tests must exercise the self-built frontend too; passing the Rust library's unit tests alone does not establish frontend parity.

### Operational invariants for verification
- Always run compiled Cinch programs with a bounded `CINCH_HEAP_LIMIT` (at most `2G` during verification).
- Run verification gates on frozen snapshots (`tooling/verify_bootstrap.py`), never on a concurrently mutated working tree, and never share Cargo target directories across checkouts.
- Keep parallel heavy build/test lanes bounded to avoid machine memory exhaustion.
- Acceptance criteria 1–4 pass on `main`; closing the remaining frontend parity items (foreign/intrinsic elaboration, refinement/precondition edge cases, staging/quotations, and remaining unmodeled declarations) and Stage 3 compiler-process leak checks is the remaining work for Acceptance 5.

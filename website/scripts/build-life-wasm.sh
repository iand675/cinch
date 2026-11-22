#!/usr/bin/env bash
# Build the two assets using the seed compiler and the freestanding runtime.
set -euo pipefail
cd "$(dirname "$0")/../.."
root="$PWD"
: "${WASI_SDK_PATH:?Set WASI_SDK_PATH to a WASI SDK with a wasm32-wasip1 sysroot}"
clang="${CINCH_CLANG:-clang}"
linker="${CINCH_WASM_LD:-wasm-ld}"
assets="$root/website/src/data/examples"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

if [[ -z "${CINCH:-}" ]]; then
  cargo build --locked --release --manifest-path seed/Cargo.toml --bin cinch
fi
cinch="${CINCH:-$root/seed/target/release/cinch}"
cargo build --locked --release --manifest-path seed/runtime/Cargo.toml --target wasm32-unknown-unknown
"$cinch" emit-ir --target wasm32-unknown-unknown "$assets/life.cinch" -o "$scratch/life.ll"
"$clang" --target=wasm32-unknown-unknown -O2 -c "$scratch/life.ll" -o "$scratch/life.o"
exports=(
  life_initial_rule life_step_row cinch_alloc
  cinch_evo_module_new cinch_evo_state_seed cinch_evo_state_payload
  cinch_evo_current_version cinch_evo_version_new cinch_evo_update_begin
  cinch_evo_add_replacement cinch_evo_mark_complete cinch_evo_activate
  cinch_evo_version_try_drop
)
args=()
for symbol in "${exports[@]}"; do args+=("--export=$symbol"); done
"$linker" --no-entry --export-memory --strip-all "${args[@]}" \
  "$scratch/life.o" "$root/seed/runtime/target/wasm32-unknown-unknown/release/libcinchruntime.a" \
  -o "$scratch/life-board.wasm"

CC_wasm32_wasip1="$WASI_SDK_PATH/bin/clang" \
AR_wasm32_wasip1="$WASI_SDK_PATH/bin/llvm-ar" \
CFLAGS_wasm32_wasip1="--sysroot=$WASI_SDK_PATH/share/wasi-sysroot" \
RUSTFLAGS="${RUSTFLAGS:-} -C link-arg=-z -C link-arg=stack-size=8388608 -C link-arg=--max-memory=536870912 -C strip=symbols" \
  cargo build --locked --release --manifest-path seed/Cargo.toml --target wasm32-wasip1 --example life-compiler

install -m 644 "$scratch/life-board.wasm" "$assets/life-board.wasm"
install -m 644 "$root/seed/target/wasm32-wasip1/release/examples/life_compiler.wasm" "$assets/life-compiler.wasm"

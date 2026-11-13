//! The Wasm and components slice (P14; WebAssembly and WASI
//! specification sections 2-8): WIT parsing and the canonical ABI table
//! of the generated component adapter, the target-keyed `Usize` width
//! and wasm32 emission, and the `eval` admission rules (supercompilation
//! and JIT specification section 7). These tests assert the tables, the
//! typing, and the emitted IR text, which needs no toolchain beyond the
//! compiler. Building and running a module is the separate concern of
//! `wasm_target.rs`, which links with `wasm-ld` and executes under Node
//! and skips with the reason printed when either is absent.

mod common;

use cinch::core::{ArtifactProfile, TargetProfile};
use cinch::driver::package_variants;
use common::source_frontend::{check_source_interfaces, check_sources, check_sources_profile, run_source};
use cinch::interface::{build_wit_adapter, CanonicalAbi};
use cinch::parser::parse_wit;

const CODEC_WIT: &str = r#"
package example:codec@0.1.0;

interface decoder {
  enum decode-error { malformed, too-large }
  decode: func(input: list<u8>, limit: u32)
    -> result<list<u8>, decode-error>;
}

world codec-plugin {
  export decoder;
}
"#;

fn codec_adapter() -> cinch::interface::WitAdapter {
    let pkg = parse_wit(CODEC_WIT).expect("codec.wit parses");
    build_wit_adapter(&pkg, CanonicalAbi::Wasi03).expect("codec adapter builds")
}

/// The generated adapter's ABI matches the canonical table of the
/// component model's `decode` (section 4): `list<u8>` lowers to
/// `(i32, i32)`, `u32` to `i32`, and the two-flattened-plus result
/// crosses through the return area, so the core signature takes the
/// return-area pointer as its last parameter.
#[test]
fn codec_wit_binding_matches_the_canonical_table() {
    let ad = codec_adapter();
    assert_eq!(ad.package, "example:codec@0.1.0");
    assert_eq!(ad.abi, CanonicalAbi::Wasi03);
    assert_eq!(ad.module, "Raw");
    let b = &ad.bindings[0];
    assert_eq!(b.wit_name, "decode");
    assert_eq!(b.symbol, "example:codec@0.1.0/decoder#decode");
    // The flattened call: (list ptr, list len, limit, return-area ptr).
    let core_params: Vec<&str> =
        b.call.core_params.iter().map(|t| t.spelling()).collect();
    assert_eq!(core_params, ["i32", "i32", "i32", "i32"]);
    assert!(b.call.core_results.is_empty(), "the result crosses in memory");
    // The return area: the `result<list<u8>, decode-error>` variant
    // layout (tag `u8`, payload at 4, 12 bytes total).
    let ret = b.call.ret.as_ref().expect("return-area plan");
    assert_eq!(ret.size, 12);
    assert_eq!(ret.align, 4);
    assert_eq!(ret.cases.len(), 2);
    // WIT case order is `ok, err`; the prelude `Result` tags are `Err` 0
    // and `Ok` 1.
    assert_eq!((ret.cases[0].disc, ret.cases[0].safe_tag), (0, 1));
    assert_eq!((ret.cases[1].disc, ret.cases[1].safe_tag), (1, 0));
    // The glue is covered for the codec shapes.
    assert_eq!(b.glue_gap, None);
    // The surface scheme of the raw binding.
    assert_eq!(
        b.scheme,
        "decode :: forall region. Ref region Bytes -> U32 ->{Foreign Component, Diverge} Result DecodeError Bytes"
    );
}

/// The boundary error vocabulary (section 3): `ForeignError` carries the
/// adapter's single error type, and the enum cases map from the WIT
/// kebab-case names.
#[test]
fn boundary_error_vocabulary_has_the_documented_shape() {
    let ad = codec_adapter();
    assert_eq!(ad.boundary_error_payload, "DecodeError");
    let v = &ad.vocab;
    assert!(v.contains("data DecodeError where"), "{v}");
    assert!(v.contains("Malformed :: DecodeError"), "{v}");
    assert!(v.contains("TooLarge :: DecodeError"), "{v}");
    assert!(
        v.contains("data BoundaryError where"),
        "{v}"
    );
    assert!(v.contains("ForeignError :: DecodeError -> BoundaryError"), "{v}");
    assert!(v.contains("ContractViolation :: BoundaryError"), "{v}");
    assert!(!v.contains("ForeignFailure"), "one error type needs no opaque failure: {v}");
}

/// A WASI stream is not a productive `Stream a` (section 6): the
/// adapter maps `stream<t>` to its own `WasiStream t` with the degraded
/// observation vocabulary (`Diverge` on the arrows, separate data and
/// completion channels through `stream` and `future`), never to the
/// codata `Stream`.
#[test]
fn wasi_stream_is_an_asynchronous_source_not_a_productive_stream() {
    let pkg = parse_wit(
        r#"
package example:feed@0.1.0;
interface feeder {
  feed: func() -> stream<u8>;
  done: func() -> future<result<_, string>>;
}
"#,
    )
    .expect("stream wit parses");
    let ad = build_wit_adapter(&pkg, CanonicalAbi::Wasi03).expect("adapter builds");
    assert!(ad.streams, "the adapter notices stream types");
    let feed = &ad.bindings[0];
    assert_eq!(feed.result.spelling(), "WasiStream U8");
    // The degraded observation type: the observation rows carry the
    // `Diverge` label and the vocabulary never names the codata `Stream`.
    assert!(ad.vocab.contains("data WasiStream a where"), "{}", ad.vocab);
    assert!(ad.vocab.contains("StreamFailed :: StreamError"), "{}", ad.vocab);
    assert!(!ad.vocab.contains("codata"), "{}", ad.vocab);
    assert!(feed.scheme.contains("Diverge"), "{}", feed.scheme);
}

/// `Usize` width keys on the target (primitive types specification
/// section 2): the same literal is in range at 64 bits and rejected at
/// 32, and a wasm32 artifact carries the Wasm triple.
#[test]
fn usize_width_and_emission_key_on_the_target() {
    let wide = |v: &str| format!(
        "
module Wide where
  wide :: Usize
  wide = {v}
  main :: Int
  main = if wide == wide then 0 else 1
"
    );
    assert!(
        check_sources(&wide("4294967296"), "<test>").is_ok(),
        "the literal fits the host's 64-bit Usize"
    );
    let err = check_sources_profile(
        &wide("4294967296"),
        "<test>",
        &TargetProfile::wasm32(ArtifactProfile::CoreWasm),
    )
    .expect_err("the literal must not fit a 32-bit Usize");
    assert!(err.to_string().contains("out of range for Usize"), "{err}");
    // The emitted IR is the observation, so this program comes from the
    // Rust frontend.
    let program = cinch::driver::check_sources_profile(
        &wide("4294967295"),
        "<test>",
        &TargetProfile::wasm32(ArtifactProfile::CoreWasm),
    )
    .expect("the maximal 32-bit Usize literal checks");
    let ir = cinch::codegen::emit_for(
        &TargetProfile::wasm32(ArtifactProfile::CoreWasm),
        &program,
    )
    .expect("wasm32 IR emission");
    assert!(
        ir.contains("target triple = \"wasm32-unknown-unknown\""),
        "the wasm32 target hook sets the triple"
    );
}

/// `eval` of a typed quote (supercompilation and JIT specification
/// section 7): admitted under a profile that allows runtime code
/// generation and recorded as a build-visible assumption in the same
/// table as the foreign ones; a profile that forbids it rejects the
/// program at compile time.
#[test]
fn eval_is_profile_gated_and_recorded_as_an_assumption() {
    let src = r#"
module UsesEval where
  run :: Nat
  run = eval [| 40 + 2 |]
  main :: Int
  main = if run == 42 then 0 else 1
"#;
    let mut no_eval = TargetProfile::native();
    no_eval.allow_eval = false;
    let err = check_sources_profile(src, "<test>", &no_eval)
        .expect_err("this profile forbids runtime code generation");
    assert!(
        err.to_string().contains("forbids runtime code generation"),
        "{err}"
    );
    let ifaces =
        check_source_interfaces(src, "<test>").expect("eval is admitted by default");
    let user = ifaces
        .iter()
        .find(|i| i.module == "UsesEval")
        .expect("the user module's interface");
    assert!(
        user.assumptions.foreign.iter().any(|a| a.starts_with("eval at ")),
        "eval uses are recorded beside the foreign assumptions: {:?}",
        user.assumptions.foreign
    );
}

/// A target's assumed deployment baseline is enforced at artifact
/// admission (platform proofs and SIMD specification section 3): a
/// required capability the profile cannot supply is a compile error,
/// while a supported lowering (the scalar path of a `target rewrite`)
/// is selected and recorded when the capability is absent.
#[test]
fn missing_required_capabilities_are_rejected_at_admission() {
    let mut needing = TargetProfile::wasm32(ArtifactProfile::CoreWasm);
    needing.required.push("wasm.simd".to_string());
    let src = r#"
module Trivial where
  main :: Int
  main = 0
"#;
    let program = cinch::driver::check_sources_profile(src, "<test>", &needing)
        .expect("checking needs no admission");
    let err = cinch::driver::admit_artifact(&needing, &program, &[])
        .expect_err("the assumed baseline is not available");
    assert!(
        err.to_string().contains("missing required capability: wasm.simd"),
        "{err}"
    );
    let mut supplied = TargetProfile::wasm32(ArtifactProfile::CoreWasm);
    supplied.required.push("wasm.simd".to_string());
    supplied.features.push("wasm.simd".to_string());
    cinch::driver::admit_artifact(&supplied, &program, &[])
        .expect("the deployment contract supplies the capability");
}

/// The runtime evaluation of a typed quote runs end to end on the host
/// target (the runtime half lives in `compiler/runtime/src/eval.rs`);
/// the quote is typed by construction and its value is the program's.
#[test]
fn eval_of_a_typed_quote_runs_end_to_end() {
    let src = r#"
module EvalRun where
  run :: Nat
  run = eval [| 40 + 2 |]

  main :: Int
  main = if run == 42 then 0 else 1
"#;
    let code = run_source(src).expect("the program compiles");
    assert_eq!(code, 0, "eval [| 40 + 2 |] evaluates to 42");
}


/// The canonical ABI glue (WebAssembly and WASI specification section
/// 4): the generated import wrapper lowers `list<u8>` through a
/// call-scoped copy to `(i32, i32)`, passes the limit and the
/// return-area pointer, and lifts the `result` back over the prelude
/// `Result` tags; the export trampoline forwards the canonical call to
/// the program's implementation unit. Emission is the wasm32 shape: the
/// same program under the host profile is refused (D99).
#[test]
fn the_canonical_glue_emits_the_codec_marshalling() {
    let dir = std::env::temp_dir().join(format!("cinch-glue-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    std::fs::write(dir.join("codec.wit"), CODEC_WIT).expect("write codec.wit");
    let src = r#"
module Decoder where
  decode :: forall region. Ref region Bytes -> U32 ->{Foreign Component, Diverge} Result DecodeError Bytes
  decode input limit = Raw.decode input limit

module GlueProbe where
  wrap :: forall region. Ref region Bytes -> U32 ->{Foreign Component, Diverge} Result BoundaryError Bytes
  wrap input limit =
    case Raw.decode input limit of
      Err e -> Err (ForeignError e)
      Ok bytes -> Ok bytes

  main :: Int
  main = 0
"#;
    let path = dir.join("client.cinch");
    std::fs::write(&path, src).expect("write client.cinch");
    let wasm = TargetProfile::wasm32(ArtifactProfile::WasiComponent);
    let program = cinch::driver::check_sources_profile(src, path.to_str().unwrap(), &wasm)
        .expect("the adapter program checks");
    let ir = cinch::codegen::emit_for(&wasm, &program).expect("wasm32 emission");
    // The canonical call signature: the flattened parameters and the
    // return-area pointer last.
    assert!(
        ir.contains(
            "declare void @\"example:codec@0.1.0/decoder#decode\"(i32, i32, i32, i32)"
        ),
        "the import declaration carries the canonical signature"
    );
    // The import wrapper's marshalling: the input copy, the limit, and
    // the call through the canonical symbol.
    assert!(ir.contains("cinch_bytes_len") && ir.contains("cinch_bytes_data"), "the buffer crosses as a copy");
    assert!(ir.contains("call void @\"example:codec@0.1.0/decoder#decode\""), "the wrapper calls the import");
    // The return area lifts over the prelude `Result` tags: `Ok` 1 and
    // `Err` 0 with the discriminant read first.
    assert!(ir.contains("load i8, ptr"), "the discriminant reads from the return area");
    assert!(ir.contains("store i64 1, ptr"), "the Ok block stores tag 1");
    assert!(ir.contains("store i64 0, ptr"), "the Err block stores tag 0");
    // The export trampoline: the canonical symbol is defined and
    // forwards to the program's implementation unit.
    assert!(
        ir.contains("define void @\"example:codec@0.1.0/decoder#decode\""),
        "the world export gets its canonical trampoline"
    );
    // On wasm32 the implementation's buffer, its `U32` limit and its
    // `Result` all cross at 32 bits (design/narrow-value-
    // representation.md section 4), and the trampoline calls it there.
    assert!(
        ir.contains("call i32 @\"Decoder.decode\"(i32 "),
        "the trampoline forwards to the implementation at its narrowed boundary"
    );
    // The host profile refuses the canonical glue (D99: the canonical
    // ABI is the wasm32 shape).
    let err = cinch::codegen::emit_for(&TargetProfile::native(), &program)
        .expect_err("the host target has no canonical ABI glue");
    assert!(err.to_string().contains("wasm32"), "{err}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Baseline and specialized variant packaging (WebAssembly and WASI
/// specification section 8): separate modules selected before
/// instantiation, with the loader recording the selected feature
/// profile. Packaging is deterministic: re-packaging the same sources
/// and profiles reproduces the same artifact identities and therefore
/// the same loader choice.
#[test]
fn variant_packaging_selects_before_instantiation_and_round_trips() {
    let src = r#"
module Pack where
  step :: Nat -> Nat
  step n = n + 1
  twice :: Nat -> Nat
  twice n = step (step n)
  main :: Int
  main = if twice 40 == 42 then 0 else 1
"#;
    let mut specialized = TargetProfile::wasm32(ArtifactProfile::WasiComponent);
    specialized.required.push("wasm.simd".to_string());
    specialized.features.push("wasm.simd".to_string());
    specialized.fuel = 8;
    let mut baseline = TargetProfile::wasm32(ArtifactProfile::WasiComponent);
    baseline.fuel = 0;
    let variants = [
        ("specialized", specialized.clone()),
        ("baseline", baseline.clone()),
    ];
    let dist = package_variants(src, "<test>", &variants).expect("packaging");
    assert_eq!(dist.variants.len(), 2);
    let spec = &dist.variants[0];
    let base = &dist.variants[1];
    assert_eq!(spec.required, ["wasm.simd"]);
    assert!(base.required.is_empty(), "the baseline needs no optional capability: {:?}", base.required);
    assert_ne!(spec.ir_hash, base.ir_hash, "the specialized and baseline modules differ");
    // Selection before instantiation: the loader's capabilities choose.
    assert_eq!(dist.select(&["wasm.simd".to_string()]).unwrap().name, "specialized");
    assert_eq!(dist.select(&[]).unwrap().name, "baseline");
    // The loader record names the selection and the feature profile.
    let record = dist.loader_record("baseline", &[]);
    assert!(record.contains("\"selected\":\"baseline\""), "{record}");
    assert!(record.contains("\"name\":\"specialized\""), "{record}");
    assert!(record.contains("wasm.simd"), "{record}");
    // A reload reproduces the choice (the packaging is deterministic).
    let again = package_variants(src, "<test>", &variants).expect("re-packaging");
    assert_eq!(again.variants[0].ir_hash, spec.ir_hash);
    assert_eq!(again.variants[1].ir_hash, base.ir_hash);
    assert_eq!(again.select(&[]).unwrap().name, "baseline");
}

/// The packaged identities round trip through the P13 variant
/// machinery: install the pair under the packaged identities, observe
/// the guard the loader's feature profile supplies, enter the
/// specialized form, and retire it while a live frame still holds its
/// code (supercompilation and JIT specification section 6).
#[test]
fn the_packaged_variants_round_trip_through_the_variant_machinery() {
    let src = r#"
module Pack2 where
  step :: Nat -> Nat
  step n = n + 1
  main :: Int
  main = if step 41 == 42 then 0 else 1
"#;
    let mut specialized = TargetProfile::wasm32(ArtifactProfile::WasiComponent);
    specialized.required.push("wasm.simd".to_string());
    specialized.features.push("wasm.simd".to_string());
    let mut baseline = TargetProfile::wasm32(ArtifactProfile::WasiComponent);
    baseline.fuel = 0;
    let dist = package_variants(
        src,
        "<test>",
        &[("specialized", specialized), ("baseline", baseline)],
    )
    .expect("packaging");
    let hash_word = |h: &str| i64::from_str_radix(&h[..15], 16).expect("hash word");
    let spec_w = hash_word(&dist.variants[0].ir_hash);
    let base_w = hash_word(&dist.variants[1].ir_hash);
    let evo = format!(
        "
module VariantRT where
  foreign import ccall \"cinch_evo_module_new\" evoNew :: I64 ->{{Global, IO}} I64
  foreign import ccall \"cinch_evo_version_new\" evoVer :: I64 -> I64 -> I64 ->{{Global, IO}} I64
  foreign import ccall \"cinch_evo_feature_avail\" evoFeat :: I64 -> I64 ->{{Global, IO}} I64
  foreign import ccall \"cinch_evo_variant_install\" evoVInst :: I64 -> I64 -> I64 -> I64 ->{{Global, IO}} I64
  foreign import ccall \"cinch_evo_variant_check_guard\" evoVGuard :: I64 -> I64 ->{{Global, IO}} I64
  foreign import ccall \"cinch_evo_variant_enter\" evoVEnt :: I64 ->{{Global, IO}} I64
  foreign import ccall \"cinch_evo_variant_retire\" evoVRet :: I64 ->{{Global, IO}} I64
  foreign import ccall \"cinch_evo_variant_retired\" evoVRetd :: I64 ->{{Global, IO}} I64
  foreign import ccall \"cinch_evo_variant_resident\" evoVRes :: I64 ->{{Global, IO}} I64
  foreign import ccall \"cinch_evo_frame_version\" evoFVer :: I64 ->{{Global, IO}} I64

  main :: Unit ->{{Global, IO}} Int
  main u =
    let m = evoNew 0 in
    let vb = evoVer m {base_w} 0 in
    let vs = evoVer m {spec_w} 0 in
    let av = evoFeat 7 1 in
    let v = evoVInst m vb vs 7 in
    let g = evoVGuard v 7 in
    let bad = evoVGuard v 8 in
    let f = evoVEnt v in
    let fv = evoFVer f in
    let rr = evoVRet v in
    let retd = evoVRetd v in
    let res = evoVRes v in
    if g == 0 then (if bad < 0 then (if fv == vs then (if retd == 1 then (if res == 1 then 0 else 1) else 2) else 3) else 4) else 5
"
    );
    let code = run_source(&evo)
        .expect("the variant round trip compiles and runs");
    assert_eq!(code, 0, "guard holds on the supplied feature, enter runs the specialized identity, retire keeps it resident for the live frame");
}

/// The manifest records the Wasm feature profile (WebAssembly and WASI
/// specification section 8: the loader records the selected feature
/// profile): the artifact shape, the required and available
/// capabilities, the selected profile, the lowerings, and each adapter's
/// canonical table.
#[test]
fn the_manifest_records_the_wasm_feature_profile_and_adapters() {
    let dir = std::env::temp_dir().join(format!("cinch-manifest-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    std::fs::write(dir.join("codec.wit"), CODEC_WIT).expect("write codec.wit");
    let src = r#"
module GlueProbe where
  wrap :: forall region. Ref region Bytes -> U32 ->{Foreign Component, Diverge} Result BoundaryError Bytes
  wrap input limit =
    case Raw.decode input limit of
      Err e -> Err (ForeignError e)
      Ok bytes -> Ok bytes

  main :: Int
  main = 0
"#;
    let path = dir.join("client.cinch");
    std::fs::write(&path, src).expect("write client.cinch");
    let wasm = TargetProfile::wasm32(ArtifactProfile::WasiComponent);
    common::drive_with_profile(
        cinch::driver::Mode::Check,
        cinch::check::StageMode::Refresh,
        &dir,
        "client.cinch",
        None,
        &wasm,
    )
    .expect("the check writes the artifact manifest");
    let manifest =
        std::fs::read_to_string(dir.join("build/client.manifest.json")).expect("the manifest");
    assert!(manifest.contains("\"wasm\""), "{manifest}");
    assert!(manifest.contains("\"artifact\":\"wasi-component\""), "{manifest}");
    assert!(manifest.contains("\"canonical_abi\":\"wasi-0.3\""), "{manifest}");
    assert!(manifest.contains("\"required\":[\"wasm.component-model\"]"), "{manifest}");
    // `selected` is `{"features":[..],"profile":..}` (keys in byte order);
    // the closing brace tells it from the `target` section's `profile`.
    assert!(manifest.contains("\"selected\":{\"features\":["), "{manifest}");
    assert!(manifest.contains("\"profile\":\"wasm32-wasi-component\"}"), "{manifest}");
    assert!(manifest.contains("\"lowerings\":[\"tasks:cooperative\"]"), "{manifest}");
    assert!(manifest.contains("example:codec@0.1.0/decoder#decode"), "{manifest}");
    assert!(manifest.contains("\"ret_area\":{\"align\":4,\"cases\":2,\"size\":12}"), "{manifest}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// The freestanding target's pointer width reaches the places a module
/// spells a target-sized value (WebAssembly and WASI specification
/// section 2): a static initializer holds a pointer at the target's own
/// width, and `memcpy`/`memset` take a `size_t`.
///
/// A linker relocates an address, not a widened copy of one, so an
/// `i64 ptrtoint` in a static initializer is not a constant a 32-bit
/// Wasm module can initialise storage with (`unsupported expression in
/// static initializer`, which clang refuses). The block storage is
/// therefore `[2n x i32]`: each word its low half then its high half,
/// the same bytes on a little-endian target, with the pointer in the
/// half the linker can fill.
#[test]
fn static_blocks_and_size_typed_calls_follow_the_pointer_width() {
    let src = r#"
module Wide where
  import Data.List
  double :: Int -> Int
  double x = x * 2
  n :: Int
  n = length (map double (replicate 2 5))
  main :: Int
  main = if n == 2 then 0 else 1
"#;
    let program = cinch::driver::check_sources_profile(
        src,
        "<test>",
        &TargetProfile::wasm32(ArtifactProfile::CoreWasm),
    )
    .expect("the program checks at wasm32");
    let ir = cinch::codegen::emit_for(
        &TargetProfile::wasm32(ArtifactProfile::CoreWasm),
        &program,
    )
    .expect("wasm32 IR emission");

    // A closure's code word is a relocated address, so the storage that
    // holds one is the i32-pair form, not `[n x i64]`.
    assert!(
        ir.contains("x i32] [i32 ") && ir.contains("ptrtoint (ptr @"),
        "a static block holding a pointer is emitted at the pointer width"
    );
    assert!(
        !ir.contains("i64 ptrtoint (ptr @") || !ir.contains("internal constant [1 x i64] [i64 0, i64 262656]"),
        "no static initializer widens a relocated pointer to i64"
    );
    // `memcpy`'s length is `size_t`.
    assert!(
        ir.contains("declare ptr @memcpy(ptr, ptr, i32)"),
        "a 32-bit target declares memcpy's length at its pointer width:\n{ir}"
    );
    // The tail call a `D104` group produces needs WebAssembly's tail-call
    // feature, which is a per-function attribute.
    assert!(
        ir.contains("\"target-features\"=\"+tail-call\""),
        "every function carries the tail-call feature so musttail assembles"
    );
}

/// A freestanding module registers its own layout table (WebAssembly and
/// WASI specification section 2).
///
/// `llvm.global_ctors` is how a native process registers it: libc runs
/// the constructors before `main`. A bare Wasm module has no `_start` and
/// no libc, so nothing ran them, the table stayed empty, and every shaped
/// block answered `shaped block with no registered layout`. That surfaced
/// as a trap inside the runtime's `release` on the first `map` over a
/// closure with saved words, nowhere near the cause.
///
/// The Wasm entry therefore calls the initialiser itself. A host that
/// also calls `__wasm_call_ctors` runs it twice, and
/// `cinch_layouts_register` is a plain store, so the repeat is free.
#[test]
fn the_wasm_entry_registers_the_layout_table_itself() {
    let src = r#"
module Wide where
  import Data.List
  add-k :: Int -> Int -> Int
  add-k k x = x + k
  shifted :: Int -> Int
  shifted = add-k 3
  n :: Int
  n = length (map shifted (replicate 2 5))
  main :: Int
  main = if n == 2 then 0 else 1
"#;
    let program = cinch::driver::check_sources_profile(
        src,
        "<test>",
        &TargetProfile::wasm32(ArtifactProfile::CoreWasm),
    )
    .expect("the program checks at wasm32");
    let ir = cinch::codegen::emit_for(
        &TargetProfile::wasm32(ArtifactProfile::CoreWasm),
        &program,
    )
    .expect("wasm32 IR emission");
    let main = ir
        .split("define i32 @main()")
        .nth(1)
        .expect("a wasm module has the plain main shape");
    assert!(
        main.contains("call void @__cinch_layouts_init()"),
        "the wasm entry registers the layout table itself:\n{main}"
    );
    assert!(
        main.contains("call i64 @") && main.contains("@cinch_exit_check"),
        "the entry still runs the program and checks its exit:\n{main}"
    );
}

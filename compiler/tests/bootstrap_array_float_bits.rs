//! Nonempty packed floats preserve their bits through the actual Cinch backend.
mod common;
use std::process::Command;

#[test]
fn cinch_packed_float_arrays_preserve_ieee_payloads() {
    // Native produces only the checked external scalar fixture. The client,
    // array construction, updates, reads and bounds are compiled by Cinch.
    let provider = common::project("array_float_provider", &[("FloatBits.cinch", r#"module FloatBits where
  export float32-from-bits, float32-bits, float64-from-bits, float64-bits
  foreign import ccall "cinch_test_from_f32_bits" make32 :: U32 ->{Global} F32
  foreign import ccall "cinch_test_f32_bits" read32 :: F32 ->{Global} U32
  foreign import ccall "cinch_test_from_f64_bits" make64 :: U64 ->{Global} F64
  foreign import ccall "cinch_test_f64_bits" read64 :: F64 ->{Global} U64
  float32-from-bits :: U32 ->{Global} F32
  float32-from-bits bits = make32 bits
  float32-bits :: F32 ->{Global} U32
  float32-bits value = read32 value
  float64-from-bits :: U64 ->{Global} F64
  float64-from-bits bits = make64 bits
  float64-bits :: F64 ->{Global} U64
  float64-bits value = read64 value
"#)]);
    let object = provider.join("provider.o");
    let profile = cinch::core::TargetProfile { rc_debug: true, ..cinch::core::TargetProfile::native() };
    std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn({
        let input = provider.join("FloatBits.cinch");
        let object = object.clone();
        let profile = profile.clone();
        move || cinch::driver::driver_with_profile_opts(
            cinch::driver::Mode::EmitObject, cinch::driver::StageMode::Refresh,
            &input, Some(&object), &profile, &Default::default(),
        ).expect("produce checked scalar object fixture")
    }).unwrap().join().unwrap();
    let c_source = provider.join("float-bits.c");
    std::fs::write(&c_source, r#"#include <stdint.h>
#include <string.h>
float cinch_test_from_f32_bits(uint32_t bits) { float value; memcpy(&value, &bits, sizeof value); return value; }
uint32_t cinch_test_f32_bits(float value) { uint32_t bits; memcpy(&bits, &value, sizeof bits); return bits; }
double cinch_test_from_f64_bits(uint64_t bits) { double value; memcpy(&value, &bits, sizeof value); return value; }
uint64_t cinch_test_f64_bits(double value) { uint64_t bits; memcpy(&bits, &value, sizeof bits); return bits; }
"#).unwrap();
    let float_object = provider.join("float-bits.o");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into());
    let result = Command::new(&clang).arg("-c").arg(&c_source).arg("-o").arg(&float_object).output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let dir = common::project("array_float_consumer", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../bootstrap/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RuntimeObject.cinch")),
    ]);
    let mut interfaces = Vec::new();
    for entry in std::fs::read_dir(provider.join("build")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|value| value == "cinchif") {
            let interface = cinch::interface::Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            interfaces.push(format!("[{}, {}, {}]", quote(&interface.module), quote(&interface.iface_hash()), quote(&path.display().to_string())));
        }
    }
    interfaces.sort();
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", interfaces.join(","))).unwrap();
    std::fs::copy(&object, dir.join("provider.o")).unwrap();
    std::fs::copy(object.with_extension("abi.json"), dir.join("provider.abi.json")).unwrap();
    std::fs::write(dir.join("target.txt"), &profile.triple).unwrap();
    let compiler = common::build(&dir, "Main.cinch", "array-float-emitter").expect("build actual Cinch object and array emitter");
    let cases = [
        (32, vec![0_u64, 0x80000000, 0x3fa00000, 0x7f800000, 0xff800000, 0x7fc01234, 0x7f801234, 1, 0x007fffff, 0x7f7fffff]),
        (64, vec![0_u64, 0x8000000000000000, 0x3ff4000000000000, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff8123412345678, 0x7ff0123412345678, 1, 0x000fffffffffffff, 0x7fefffffffffffff]),
    ];
    for (index, (width, patterns)) in cases.iter().enumerate() {
        let sentinel = if *width == 32 { 0x80000000_u64 } else { 0x8000000000000000_u64 };
        let checks = patterns.iter().map(|bits| format!("check {bits}")).collect::<Vec<_>>().join(" && ");
        let source = format!(r#"module Main where
  import Data.Core
  import FloatBits (float{width}-from-bits as from-bits, float{width}-bits as bits-of)
  foreign import ccall "cinch.runtime.arrayFromListF{width}" packed :: List F{width} -> Array F{width}
  foreign import ccall "cinch.runtime.arrayGet" get :: forall a. Array a -> Nat -> Option a
  foreign import ccall "cinch.runtime.arraySet" set :: forall a. Array a -> Nat -> a -> Option (Array a)
  foreign import ccall "cinch.runtime.arraySlice" slice :: forall a. Array a -> Nat -> Nat -> Option (Array a)
  foreign import ccall "cinch.runtime.arrayAppend" append :: forall a. Array a -> Array a -> Array a
  foreign import ccall "cinch.runtime.arrayReplicate" repeat :: forall a. Nat -> a -> Array a
  same-at :: U{width} -> Array F{width} -> Nat ->{{Global}} Bool
  same-at bits values index = case get values index of None -> False; Some value -> bits-of value == bits
  check :: U{width} ->{{Global}} Bool
  check bits = let value = from-bits bits in let original = packed (Cons value (Cons (from-bits {sentinel}) Nil)) in
    case set original 0 (from-bits {sentinel}) of
      None -> False
      Some updated -> case slice original 0 1 of
        None -> False
        Some cut -> same-at bits original 0 && same-at {sentinel} original 1 && same-at {sentinel} updated 0 && same-at bits cut 0 && same-at bits (append original (repeat 1 value)) 2
  main :: Unit ->{{Global}} Int
  main unit = if {checks} then 42 else 1
"#);
        std::fs::write(dir.join(format!("input-{index}.source")), source).unwrap();
    }
    std::fs::write(dir.join("batch.count"), vec![b'x'; cases.len()]).unwrap();
    let emitted = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert!(emitted.status.success(), "{}\n{}", common::read_output(&dir, "admission.diagnostics"), String::from_utf8_lossy(&emitted.stderr));
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    for (index, (width, patterns)) in cases.iter().enumerate() {
        assert_eq!(common::read_output(&dir, &format!("output-{index}.status")), "0", "F{width}: {}", common::read_output(&dir, &format!("output-{index}.diagnostics")));
        let executable = dir.join(format!("float-{width}"));
        let linked = Command::new(&clang).arg(dir.join(format!("output-{index}.ll"))).arg(&object).arg(&float_object).arg(&runtime).arg("-o").arg(&executable).output().unwrap();
        assert!(linked.status.success(), "F{width}: {}", String::from_utf8_lossy(&linked.stderr));
        let ran = Command::new(&executable).env("CINCH_HEAP_LIMIT", "32M").env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
        assert_eq!(ran.status.code(), Some(42), "F{width}: {}", String::from_utf8_lossy(&ran.stderr));
        assert!(String::from_utf8_lossy(&ran.stderr).split_whitespace().any(|word| word == "leaked=0"));
        println!("Cinch packed F{width}: {} exact bit patterns, persistent update, slice and mixed-format append; {}", patterns.len(), String::from_utf8_lossy(&ran.stderr).trim());
    }
    common::cleanup(&provider);
    common::cleanup(&dir);
}

fn quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

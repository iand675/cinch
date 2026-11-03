//! The Cinch emitter consumes checked library objects and a shared layout prefix.
mod common;
use std::process::Command;

fn quote(s: &str) -> String {
    let mut text = String::from("\"");
    for c in s.chars() {
        match c { '"' => text.push_str("\\\""), '\\' => text.push_str("\\\\"), c => text.push(c) }
    }
    text.push('"'); text
}

#[test]
fn cinch_calls_actual_library_objects_with_checked_layout_authority() {
    let provider = common::project("object_library", &[("Library.cinch", r#"module Library where
  import Data.String (string-to-bytes)
  import Data.Bytes (bytes-compare)
  export make-string, choose, identity, passthrough, Poly, keep, PolyInt, decimal-bytes, decimal-ok
  make-string :: Unit -> String
  make-string unit = "library-owned string"
  choose :: Int -> Bool -> Int
  choose value flag = if flag then value else 0
  identity :: forall a. a -> a
  identity value = value
  passthrough :: forall a. a -> a
  passthrough value = value
  signature Poly a where
    keep :: forall b. a -> b -> Unit -> b
  implicit instance PolyInt :: Poly Int where
    keep anchor value unit = value
  decimal-bytes :: Bool -> Bytes
  decimal-bytes negative = string-to-bytes (if negative then "-0e0" else "1.25e0")
  decimal-ok :: Bool -> Bytes -> Bool
  decimal-ok negative bytes = bytes-compare bytes (decimal-bytes negative) == 0
  secret :: Bool -> Bool
  secret value = value
"#)]);
    let profile = cinch::core::TargetProfile { rc_debug: true, ..cinch::core::TargetProfile::native() };
    let object = provider.join("provider.o");
    std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn({
        let input = provider.join("Library.cinch"); let object = object.clone(); let profile = profile.clone();
        move || {
            let roots = cinch::specialize::parse_object_roots(r#"{"schema":1,"units":[{"symbol":"Library.identity","classes":"R"},{"symbol":"Library.identity","classes":"O"},{"symbol":"Library.identity","classes":"T"}],"dictionaries":[{"symbol":"Library.PolyInt","classes":""}],"methods":[{"class":"Library.Poly","slot":0,"own":"R"},{"class":"Library.Poly","slot":0,"own":"O"},{"class":"Library.Poly","slot":0,"own":"T"}]}"#).unwrap();
            let opts = cinch::driver::DriverOptions { object_roots: roots, ..Default::default() };
            cinch::driver::driver_with_profile_opts(cinch::driver::Mode::EmitObject, cinch::driver::StageMode::Refresh, &input, Some(&object), &profile, &opts).unwrap()
        }
    }).unwrap().join().unwrap();
    let dir = common::project("runtime_objects", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../bootstrap/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RuntimeObject.cinch")),
    ]);
    let mut rows = Vec::new();
    for e in std::fs::read_dir(provider.join("build")).unwrap() {
        let path = e.unwrap().path();
        if path.extension().is_some_and(|e| e == "cinchif") {
            let iface = cinch::interface::Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
        }
    }
    rows.sort();
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    std::fs::copy(&object, dir.join("provider.o")).unwrap();
    let metadata = std::fs::read_to_string(object.with_extension("abi.json")).unwrap();
    std::fs::write(dir.join("provider.abi.json"), &metadata).unwrap();
    std::fs::write(dir.join("target.txt"), &profile.triple).unwrap();
    let compiler = common::build(&dir, "Main.cinch", "runtime-object-emitter").expect("build actual Cinch object consumer");
    let cases = [
        ("raw", "module Main where\n  import Data.Bool (not)\n  main :: Int\n  main = if not False then 227 else 1\n", 227),
        ("closure", "module Main where\n  import Data.Bool (not)\n  apply :: (Bool -> Bool) -> Bool -> Bool\n  apply f b = f b\n  main :: Int\n  main = if apply not False then 229 else 1\n", 229),
        ("owned-result", "module Main where\n  import Library (make-string)\n  main :: Int\n  main = let text = make-string () in 231\n", 231),
        ("partial-owned", "module Main where\n  import Library (choose)\n  main :: Int\n  main = let f = choose 18446744073709551616 in if f True == 18446744073709551616 then 233 else 1\n", 233),
        ("combined-table", "module Main where\n  import Library (make-string)\n  data Box where\n    Box :: Int -> Box\n  main :: Int\n  main = case (make-string (), Box 18446744073709551616) of\n    (text, Box number) -> if number == 18446744073709551616 then 235 else 1\n", 235),
        ("process-arguments", "module Main where\n  import Effect.Process (process-args)\n  import Data.Core (List, Cons, Nil, Result, Ok, Err)\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok args -> case args of\n      Cons argument rest -> if string-compare argument \"argument with spaces\" == 0 then 239 else 3\n      Nil -> 1\n    Err error -> 2\n", 239),
        ("polymorphic-function", "module Main where\n  import Library (identity, make-string)\n  main :: Int\n  main = let text = identity (make-string ()) in if identity True && identity 18446744073709551616 == 18446744073709551616 then 241 else 1\n", 241),
        ("method-families", "module Main where\n  import Library (Poly, keep)\n  data Box where\n    Box :: Int -> Box\n  main :: Int\n  main = let raw = keep (0 :: Int) True in let tagged = keep (0 :: Int) 18446744073709551616 in let object = keep (0 :: Int) (Box 18446744073709551616) in case (raw (), tagged (), object ()) of\n    (flag, n, Box m) -> if flag && n == m then 243 else 1\n", 243),
        ("string-literal", "module Main where\n  import Data.String (string-length)\n  main :: Int\n  main = if string-length \"hi λ🙂\" == 5 then 245 else 1\n", 245),
        ("empty-string", "module Main where\n  import Data.String (string-length)\n  main :: Int\n  main = if string-length \"\" == 0 then 247 else 1\n", 247),
        ("char-pattern", "module Main where\n  main :: Int\n  main = case 'λ' of\n    'λ' -> 249\n    _ -> 1\n", 249),
        ("machine-pattern", "module Main where\n  echo :: U64 -> U64\n  echo n = n\n  main :: Int\n  main = case echo (18446744073709551615 :: U64) of\n    18446744073709551615 -> 251\n    _ -> 1\n", 251),
        ("string-pattern", "module Main where\n  import Library (make-string)\n  main :: Int\n  main = case make-string () of\n    \"different\" -> 1\n    \"library-owned string\" -> 253\n    _ -> 2\n", 253),
        ("float-decimal", "module Main where\n  import Library (decimal-bytes, decimal-ok)\n  foreign import ccall \"cinch.runtime.textF64FromDecimal\" decode :: Bytes -> F64\n  foreign import ccall \"cinch.runtime.textF64ToDecimal\" encode :: F64 -> Bytes\n  main :: Int\n  main = if decimal-ok False (encode (decode (decimal-bytes False))) then 217 else 1\n", 217),
        ("float-negative-zero", "module Main where\n  import Library (decimal-bytes, decimal-ok)\n  foreign import ccall \"cinch.runtime.textF64FromDecimal\" decode :: Bytes -> F64\n  foreign import ccall \"cinch.runtime.textF64ToDecimal\" encode :: F64 -> Bytes\n  main :: Int\n  main = if decimal-ok True (encode (decode (decimal-bytes True))) then 219 else 1\n", 219),
        ("float-first-class", "module Main where\n  import Library (decimal-bytes, decimal-ok)\n  foreign import ccall \"cinch.runtime.textF64FromDecimal\" decode :: Bytes -> F64\n  foreign import ccall \"cinch.runtime.textF64ToDecimal\" encode :: F64 -> Bytes\n  apply :: forall a b. (a -> b) -> a -> b\n  apply f value = f value\n  main :: Int\n  main = if decimal-ok False (apply encode (apply decode (decimal-bytes False))) then 221 else 1\n", 221),
        ("returned-function", "module Main where\n  import Library (identity)\n  import Data.Bool (not)\n  main :: Int\n  main = if identity not False then 237 else 1\n", 237),
    ];
    for (i, (_, source, _)) in cases.iter().enumerate() { std::fs::write(dir.join(format!("input-{i}.source")), source).unwrap(); }
    std::fs::write(dir.join("batch.count"), vec![b'x'; cases.len()]).unwrap();
    let emitted = Command::new(&compiler).current_dir(&dir).env("CINCH_HEAP_LIMIT", "2G").output().unwrap();
    assert!(emitted.status.success(), "project {}\n{}\n{}", dir.display(), common::read_output(&dir,"admission.diagnostics"), String::from_utf8_lossy(&emitted.stderr));
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    for (i, (name, _, code)) in cases.iter().enumerate() {
        assert_eq!(common::read_output(&dir, &format!("output-{i}.status")), "0", "{name}: {}", common::read_output(&dir, &format!("output-{i}.diagnostics")));
        let ir = dir.join(format!("output-{i}.ll")); let exe = dir.join(format!("program-{i}"));
        let prefix: Vec<u64> = metadata.split("\"layout_words\":[").nth(1).unwrap().split(']').next().unwrap().split(',').filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
        let text = std::fs::read_to_string(&ir).unwrap();
        let table = text.lines().find(|l| l.starts_with("@cinch.typed.layouts =")).unwrap();
        let words: Vec<u64> = table.split("] [").nth(1).unwrap().trim_end_matches(']').split(", ").map(|s| s.strip_prefix("i64 ").unwrap().parse().unwrap()).collect();
        assert_eq!(&words[..prefix.len()], &prefix, "{name}: provider layout prefix changed");
        let link = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into())).arg(&ir).arg(&object).arg(&runtime).arg("-o").arg(&exe).output().unwrap();
        assert!(link.status.success(), "{name}: {}", String::from_utf8_lossy(&link.stderr));
        let mut command = Command::new(&exe);
        if *name == "process-arguments" { command.arg("argument with spaces"); }
        let ran = command.env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
        assert_eq!(ran.status.code(), Some(*code), "{name}: {}", String::from_utf8_lossy(&ran.stderr));
        assert!(String::from_utf8_lossy(&ran.stderr).split_whitespace().any(|s| s == "leaked=0"));
        println!("object {name}: exit {code}; {}", String::from_utf8_lossy(&ran.stderr).trim());
    }
    // Input identity failures stop admission before any source program emits.
    std::fs::remove_file(dir.join("batch.count")).unwrap();
    std::fs::write(dir.join("input.source"), "module Main where\n  import Library (passthrough)\n  main :: Int\n  main = passthrough (1 :: Int)\n").unwrap();
    assert_eq!(Command::new(&compiler).current_dir(&dir).output().unwrap().status.code(), Some(5));
    assert!(common::read_output(&dir,"output.ll").is_empty());
    assert!(common::read_output(&dir,"output.diagnostics").contains("checked external variant is absent"));
    assert!(common::read_output(&dir,"output.diagnostics").contains(" [T]"));
    std::fs::write(dir.join("input.source"), cases[0].1).unwrap();
    let wrong = metadata.replace("\"params\":[\"R\"],\"result\":\"R\",\"symbol\":\"Data.Bool.not\"", "\"params\":[\"O\"],\"result\":\"R\",\"symbol\":\"Data.Bool.not\"");
    assert_ne!(wrong, metadata);
    std::fs::write(dir.join("provider.abi.json"), wrong).unwrap();
    assert_eq!(Command::new(&compiler).current_dir(&dir).output().unwrap().status.code(), Some(5));
    assert!(common::read_output(&dir,"output.ll").is_empty());
    assert!(common::read_output(&dir,"output.diagnostics").contains("word ABI"));
    let wrong = metadata.replace("\"fields\":[\"O\",\"O\"],\"methods\":[\"Data.Int.Eq_Int.==\"", "\"fields\":[\"R\",\"O\"],\"methods\":[\"Data.Int.Eq_Int.==\"");
    assert_ne!(wrong, metadata);
    std::fs::write(dir.join("provider.abi.json"), wrong).unwrap();
    std::fs::write(dir.join("input.source"), cases[3].1).unwrap();
    assert_eq!(Command::new(&compiler).current_dir(&dir).output().unwrap().status.code(), Some(5));
    assert!(common::read_output(&dir,"output.ll").is_empty());
    assert!(common::read_output(&dir,"output.diagnostics").contains("factory word ABI"));
    std::fs::write(dir.join("provider.abi.json"), &metadata).unwrap();
    std::fs::write(dir.join("input.source"), "module Main where\n  import Library (secret)\n  main :: Int\n  main = if secret True then 0 else 1\n").unwrap();
    assert_eq!(Command::new(&compiler).current_dir(&dir).output().unwrap().status.code(), Some(4));
    assert!(common::read_output(&dir,"output.ll").is_empty());
    std::fs::write(dir.join("input.source"), "module Main where\n  import Data.String (StrBuf)\n  main :: Int\n  main = 0\n").unwrap();
    assert_eq!(Command::new(&compiler).current_dir(&dir).output().unwrap().status.code(), Some(4));
    assert!(common::read_output(&dir,"output.ll").is_empty());
    std::fs::write(dir.join("input.source"), cases[0].1).unwrap();
    let changed = metadata.replace(&format!("\"runtime_abi\":{}", cinch::interface::RUNTIME_ABI_VERSION), "\"runtime_abi\":999"); assert_ne!(changed, metadata);
    std::fs::write(dir.join("provider.abi.json"), changed).unwrap();
    assert_eq!(Command::new(&compiler).current_dir(&dir).output().unwrap().status.code(), Some(8));
    assert!(common::read_output(&dir,"admission.diagnostics").contains("ABI mismatch"));
    let changed = metadata.replace(&format!("\"target\":{}", quote(&profile.triple)), "\"target\":\"wrong-target\""); assert_ne!(changed, metadata);
    std::fs::write(dir.join("provider.abi.json"), changed).unwrap();
    assert_eq!(Command::new(&compiler).current_dir(&dir).output().unwrap().status.code(), Some(8));
    assert!(common::read_output(&dir,"admission.diagnostics").contains("ABI mismatch"));
    std::fs::write(dir.join("provider.abi.json"), &metadata).unwrap();
    let mut corrupt = std::fs::read(&object).unwrap(); corrupt.push(0); std::fs::write(dir.join("provider.o"), corrupt).unwrap();
    assert_eq!(Command::new(&compiler).current_dir(&dir).output().unwrap().status.code(), Some(8));
    assert!(common::read_output(&dir,"admission.diagnostics").contains("code identity mismatch"));
    common::cleanup(&provider); common::cleanup(&dir);
}

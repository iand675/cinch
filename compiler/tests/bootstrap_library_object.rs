//! Real Cinch CLI object production, authenticated reload and a linked client.
mod common;
use cinch::lsp::{parse_json, Json};
use std::path::Path;
use std::process::Command;

fn sha256(path: &Path) -> String {
    let output = Command::new("python3").args(["-c", "import hashlib,pathlib,sys;print(hashlib.sha256(pathlib.Path(sys.argv[1]).read_bytes()).hexdigest())"]).arg(path).output().unwrap();
    assert!(output.status.success(), "artifact hash: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap().trim().into()
}
fn configuration(path: &Path, object: &Path) {
    std::fs::write(path, Json::obj(vec![
        ("schema", Json::Num(1.0)), ("target", Json::Str(cinch::core::TargetProfile::native().triple)),
        ("object", Json::Str(object.display().to_string())), ("metadata", Json::Str(object.with_extension("abi.json").display().to_string())),
        ("object_sha256", Json::Str(sha256(object))), ("metadata_sha256", Json::Str(sha256(&object.with_extension("abi.json")))),
    ]).to_text()).unwrap();
}
fn interfaces(path: &Path, dirs: &[&Path]) {
    let mut rows = std::collections::BTreeMap::new();
    for dir in dirs {
        for file in std::fs::read_dir(dir).unwrap() {
            let file = file.unwrap().path();
            if file.extension().is_none_or(|e| e != "cinchif") { continue; }
            let iface = cinch::interface::Iface::from_json(&std::fs::read_to_string(&file).unwrap()).unwrap();
            let hash = iface.iface_hash();
            assert!(rows.insert(iface.module.clone(), Json::Arr(vec![Json::Str(iface.module), Json::Str(hash), Json::Str(file.display().to_string())])).is_none(), "duplicate interface");
        }
    }
    std::fs::write(path, Json::obj(vec![("schema", Json::Num(1.0)), ("interfaces", Json::Arr(rows.into_values().collect()))]).to_text()).unwrap();
}
fn fixture_sources(dir: &Path) -> std::collections::BTreeMap<std::ffi::OsString, Vec<u8>> {
    std::fs::read_dir(dir).unwrap().map(|item| item.unwrap().path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "cinch"))
        .map(|p| (p.file_name().unwrap().to_owned(), std::fs::read(p).unwrap())).collect()
}
fn run(exe: &Path, args: &[&Path], mode: &str, manifest: &Path, objects: &Path) -> std::process::Output {
    // Quarantining every freed parser/metadata node turns a large compiler
    // run into a cumulative allocation budget. Keep normal reclamation and
    // leak checks here; the emitted client below still uses RC poisoning.
    let output = Command::new(exe).args([mode, "--interfaces"]).arg(manifest).arg("--objects").arg(objects).args(args)
        .env("CINCH_HEAP_LIMIT", "256M").env("CINCH_RC_DEBUG", "0").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("live=0 immortal=0 leaked=0"), "CLI leak check: {stderr}");
    println!("object CLI {mode}: {:?}; {}", output.status.code(), stderr.trim());
    output
}
#[test]
fn cinch_cli_emits_a_closed_object_that_another_cinch_invocation_can_consume() {
    // Native supplies library dependencies only. The source library below
    // is parsed, checked and emitted exclusively by the actual Cinch CLI.
    let provider = common::project("object_cli_provider", &[("Provider.cinch", "module Provider where\n  marker :: Unit -> Unit\n  marker unit = unit\n")]);
    let object = provider.join("provider.o");
    std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn({
        let input = provider.join("Provider.cinch"); let object = object.clone();
        move || {
            let roots = vec![cinch::specialize::ObjectRoot::Dictionary { symbol: "Data.List.FromList_List".into(), classes: vec![cinch::core::Class::TObj] }];
            let options = cinch::driver::DriverOptions { object_roots: roots, ..Default::default() };
            let profile = cinch::core::TargetProfile { rc_debug: true, ..cinch::core::TargetProfile::native() };
            cinch::driver::driver_with_profile_opts(cinch::driver::Mode::EmitObject, cinch::driver::StageMode::Refresh, &input, Some(&object), &profile, &options).unwrap()
        }
    }).unwrap().join().unwrap();
    // Match the staged driver's complete compiler input set, including
    // modules the CLI does not call, so this one build checks every module.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut compiler_sources = std::collections::BTreeMap::new();
    for file in std::fs::read_dir(root.join("../bootstrap")).unwrap() {
        let file = file.unwrap().path();
        if file.extension().is_some_and(|e| e == "cinch") {
            compiler_sources.insert(file.file_name().unwrap().to_str().unwrap().to_string(), std::fs::read_to_string(file).unwrap());
        }
    }
    compiler_sources.insert("Bootstrap.RuntimeAbi.cinch".into(), format!("-- One exact ABI per compiler build. The pinned stage builder renders this\n-- module from its verified seed lock before hashing the complete sources.\nmodule Bootstrap.RuntimeAbi where\n  export runtime-abi, runtime-abi-int\n  runtime-abi :: Nat\n  runtime-abi = {}\n  runtime-abi-int :: Int\n  runtime-abi-int = {}\n", cinch::interface::RUNTIME_ABI_VERSION, cinch::interface::RUNTIME_ABI_VERSION));
    for (name, text) in common::TOOLKIT { compiler_sources.insert((*name).into(), (*text).into()); }
    let imports = compiler_sources.keys().enumerate().map(|(i, name)| format!("  import {} as CompilerSource{i}\n", name.strip_suffix(".cinch").unwrap())).collect::<String>();
    compiler_sources.insert("Main.cinch".into(), include_str!("../../bootstrap/programs/Run.Pipeline.cinch").replacen("module Main where\n", &format!("module Main where\n{imports}"), 1));
    let files = compiler_sources.iter().map(|(name, text)| (name.as_str(), text.as_str())).collect::<Vec<_>>();
    let dir = common::project("library_object_cli", &files);
    assert_eq!(fixture_sources(&dir), compiler_sources.iter().map(|(name, text)| (std::ffi::OsString::from(name), text.as_bytes().to_vec())).collect());
    let compiler = if let Some(path) = std::env::var_os("CINCH_TEST_LIBRARY_OBJECT_COMPILER") {
        assert!(common::external_frontend::configured().unwrap().is_none(), "a cached Native fixture cannot replace the selected Cinch frontend");
        let compiler = std::path::PathBuf::from(path).canonicalize().unwrap();
        assert_eq!(fixture_sources(&dir), fixture_sources(compiler.parent().unwrap()), "cached compiler source inputs changed");
        compiler
    } else { common::build(&dir, "Main.cinch", "object-compiler").expect("build actual Cinch CLI") };
    let retained = std::env::var_os("CINCH_TEST_LIBRARY_OBJECT_ARTIFACTS");
    if let Some(path) = &retained {
        std::fs::write(path, Json::obj(vec![("compiler", Json::Str(compiler.display().to_string())), ("compiler_sha256", Json::Str(sha256(&compiler))), ("project", Json::Str(dir.display().to_string())), ("provider", Json::Str(provider.display().to_string()))]).to_text()).unwrap();
    }
    let manifest = dir.join("provider.interfaces.json");
    interfaces(&manifest, &[&provider.join("build")]);
    let objects = dir.join("provider.objects.json"); configuration(&objects, &object);
    let library = dir.join("library.source");
    let source = format!("{}\n{}\n{}\n{}", include_str!("../prelude/Data/Array.cinch"), include_str!("../prelude/Data/Tensor.cinch"), include_str!("../prelude/Data/Columnar.cinch"), r#"module Library where
  import Data.Array
  import Data.Tensor
  import Data.Columnar
  export make, peek, column, Box, wrap, unwrap
  make :: Int -> Array Int
  make x = array-from-list [x]
  peek :: Tensor Int -> Array Nat -> Option Int
  peek tensor indices = tensor-get tensor indices
  column :: Array Int -> Columnar Int
  column values = columnar-column values
  data Box where
    Box :: Int -> Box
  wrap :: Int -> Box
  wrap n = Box n
  unwrap :: Box -> Int
  unwrap value = case value of Box n -> n
  hidden :: Int -> Int
  hidden n = n
"#);
    std::fs::write(&library, &source).unwrap();
    let ir = dir.join("library.ll"); let out = dir.join("library.o"); let abi = out.with_extension("abi.json");
    let build = dir.join("object-interfaces"); std::fs::create_dir(&build).unwrap();
    let prefix = build.join("").display().to_string() + "/";
    let result = run(&compiler, &[&library, &ir, &out, &abi, Path::new(&prefix)], "--emit-object", &manifest, &objects);
    assert!(result.status.success(), "{}\n{}", String::from_utf8_lossy(&result.stderr), common::read_output(&dir, "library.ll.diagnostics"));
    let text = std::fs::read_to_string(&abi).unwrap();
    std::fs::write(dir.join("successful.abi.json"), &text).unwrap();
    std::fs::copy(&ir, dir.join("successful.library.ll")).unwrap();
    std::fs::write(dir.join("successful.library.source"), &source).unwrap();
    let metadata = parse_json(&text).unwrap();
    assert_eq!(metadata.get("runtime_abi").and_then(Json::as_num), Some(6.0));
    let constructor = metadata.get("units").and_then(Json::as_arr).unwrap().iter().find(|u| u.get("symbol").and_then(Json::as_str) == Some("Library.Box")).expect("actual concrete constructor root");
    assert_eq!(constructor.get("result").and_then(Json::as_str), Some("O"));
    assert_eq!(constructor.get("arity").and_then(Json::as_num), Some(1.0));
    let make = metadata.get("units").and_then(Json::as_arr).unwrap().iter().find(|u| u.get("symbol").and_then(Json::as_str) == Some("Library.make")).unwrap();
    assert_eq!(make.get("params").and_then(Json::as_arr).unwrap()[0].as_str(), Some("T"));
    assert_eq!(make.get("result").and_then(Json::as_str), Some("O"));
    let iface = cinch::interface::Iface::from_json(&std::fs::read_to_string(build.join("Library.cinchif")).unwrap()).unwrap();
    assert!(!iface.exports.values.iter().any(|v| v.name == "hidden"));
    for name in ["make", "peek", "column", "wrap", "unwrap"] { assert!(iface.exports.values.iter().any(|v| v.name == name)); }
    let llvm = std::fs::read_to_string(&ir).unwrap();
    assert!(!llvm.contains("@llvm.global_ctors"), "dependency object must leave table registration to final consumer");
    for symbol in ["cinch_array_from_list", "cinch_tensor_get", "cinch_columnar_column"] { assert!(llvm.contains(symbol)); }
    let final_manifest = dir.join("client.interfaces.json"); interfaces(&final_manifest, &[&provider.join("build"), &build]);
    let final_objects = dir.join("client.objects.json"); configuration(&final_objects, &out);
    let client = dir.join("client.source"); let client_ir = dir.join("client.ll");
    std::fs::write(&client, r#"module Main where
  import Library
  import Data.Array
  main :: Int
  main = if array-length (make 42) == 1 && unwrap (wrap 184467440737095516160) == 184467440737095516160 then 0 else 1
"#).unwrap();
    let result = run(&compiler, &[&client, &client_ir], "--emit-ir", &final_manifest, &final_objects);
    assert!(result.status.success(), "{}", common::read_output(&dir, "client.ll.diagnostics"));
    std::fs::copy(&client, dir.join("successful.client.source")).unwrap();
    std::fs::copy(&client_ir, dir.join("successful.client.ll")).unwrap();
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    let executable = dir.join("client");
    let linked = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into())).arg(&client_ir).arg(&out).arg(runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(&executable).output().unwrap();
    assert!(linked.status.success(), "{}", String::from_utf8_lossy(&linked.stderr));
    let ran = Command::new(executable).env("CINCH_HEAP_LIMIT", "32M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert_eq!(ran.status.code(), Some(0), "{}", String::from_utf8_lossy(&ran.stderr));
    assert!(String::from_utf8_lossy(&ran.stderr).contains("live=0 immortal=0 leaked=0"));
    println!("closed object client: {}", String::from_utf8_lossy(&ran.stderr).trim());
    std::fs::write(&client, "module Main where\n  import Library (hidden)\n  main :: Int\n  main = hidden 0\n").unwrap();
    assert_eq!(run(&compiler, &[&client, &client_ir], "--emit-ir", &final_manifest, &final_objects).status.code(), Some(4));
    assert!(std::fs::read_to_string(&client_ir).unwrap().is_empty());
    // A failed rebuild invalidates its previously good manifest.
    std::fs::write(&library, "module Library where\n  broken = (\n").unwrap();
    assert!(!run(&compiler, &[&library, &ir, &out, &abi, Path::new(&prefix)], "--emit-object", &manifest, &objects).status.success());
    assert!(std::fs::read(&abi).unwrap().is_empty());
    if retained.is_none() { common::cleanup(&dir); common::cleanup(&provider); }
}

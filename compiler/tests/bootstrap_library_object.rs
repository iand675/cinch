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
    let mut command = Command::new(exe);
    command.args([mode, "--interfaces"]).arg(manifest);
    if matches!(mode, "--emit-object" | "--emit-ir") { command.arg("--objects").arg(objects); }
    let output = command.args(args)
        .env("CINCH_HEAP_LIMIT", "256M").env("CINCH_RC_DEBUG", "0").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("live=0 immortal=0 leaked=0"), "CLI leak check: {stderr}");
    println!("object CLI {mode}: {:?}; {}", output.status.code(), stderr.trim());
    output
}

fn foreign_roundtrip(compiler: &Path, dir: &Path, provider: &Path, manifest: &Path, objects: &Path, runtime: &Path) {
    let root = dir.join("foreign-clients"); std::fs::create_dir(&root).unwrap();
    let fixture = root.join("fixture.c");
    std::fs::write(&fixture, r#"#include <stdint.h>
#include <stddef.h>
static int64_t counter, value = 42;
int64_t ffi_i64(int64_t x) { return x + 2; }
int64_t ffi_f64(double x) { return x == 1.5 ? 42 : -1; }
double ffi_make_f64(void) { return 1.5; }
int64_t ffi_f32(float x) { return x == 1.5f ? 42 : -1; }
float ffi_make_f32(void) { return 1.5f; }
void ffi_increment(void) { counter++; }
int64_t ffi_take(int64_t x) { return counter == 1 ? x : -1; }
void *ffi_open(void) { return &value; }
int64_t h_read(void *p) { return *(int64_t *)p; }
void *ffi_null(void) { return NULL; }
int64_t ffi_i8(int8_t x) { return x == -7 ? 42 : -1; }
int8_t make_i8(void) { return -7; }
int64_t ffi_u8(uint8_t x) { return x == 249 ? 42 : -1; }
uint8_t make_u8(void) { return 249; }
int64_t ffi_i16(int16_t x) { return x == -1234 ? 42 : -1; }
int16_t make_i16(void) { return -1234; }
int64_t ffi_u16(uint16_t x) { return x == 60000 ? 42 : -1; }
uint16_t make_u16(void) { return 60000; }
int64_t ffi_i32(int32_t x) { return x == -1234567 ? 42 : -1; }
int32_t make_i32(void) { return -1234567; }
int64_t ffi_u32(uint32_t x) { return x == UINT32_C(4000000000) ? 42 : -1; }
uint32_t make_u32(void) { return UINT32_C(4000000000); }
"#).unwrap();
    let c_object = root.join("fixture.o");
    let status = Command::new("clang").args(["-O2", "-c"]).arg(&fixture).arg("-o").arg(&c_object).status().unwrap();
    assert!(status.success());
    let object = cinch::lsp::parse_json(&std::fs::read_to_string(objects).unwrap()).unwrap();
    let object = object.get("object").and_then(Json::as_str).unwrap();
    let eq = "  foreign import ccall \"cinch.prim.eqI64\" eq :: I64 -> I64 -> Bool\n";
    let check_and_run = |name: &str, source: &str, selected_manifest: &Path| {
        let case = root.join(name); std::fs::create_dir(&case).unwrap();
        let input = case.join("Main.cinch"); let ir = case.join("program.ll"); let exe = case.join("program");
        std::fs::write(&input, source).unwrap();
        let output = run(compiler, &[&input, &ir], "--emit-ir", selected_manifest, objects);
        assert!(output.status.success(), "{name}: {}", std::fs::read_to_string(ir.with_extension("ll.diagnostics")).unwrap_or_default());
        let linked = Command::new("clang").args([&ir, &c_object, Path::new(object), runtime]).arg("-o").arg(&exe).output().unwrap();
        assert!(linked.status.success(), "{name}: {}", String::from_utf8_lossy(&linked.stderr));
        let ran = Command::new(&exe).env("CINCH_HEAP_LIMIT", "32M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
        assert_eq!(ran.status.code(), Some(0), "{name}: {}", String::from_utf8_lossy(&ran.stderr));
        assert!(String::from_utf8_lossy(&ran.stderr).contains("live=0 immortal=0 leaked=0"), "{name}");
        println!("foreign client {name}: {}", String::from_utf8_lossy(&ran.stderr).trim());
    };
    for (name, declarations, body) in [
        ("scalar", "  foreign import ccall \"ffi_i64\" probe :: I64 ->{Foreign Test} I64\n", "if eq (probe 40) 42 then 0 else 1"),
        ("alias", "  foreign import ccall \"ffi_i64\" a :: I64 ->{Foreign Test} I64\n  foreign import ccall \"ffi_i64\" b :: I64 ->{Foreign Test} I64\n", "if eq (a 40) (b 40) then 0 else 1"),
        ("double", "  foreign import ccall \"ffi_f64\" probe :: F64 ->{Foreign Test} I64\n  foreign import ccall \"ffi_make_f64\" make :: Unit ->{Foreign Test} F64\n", "if eq (probe (make ())) 42 then 0 else 1"),
        ("float", "  foreign import ccall \"ffi_f32\" probe :: F32 ->{Foreign Test} I64\n  foreign import ccall \"ffi_make_f32\" make :: Unit ->{Foreign Test} F32\n", "if eq (probe (make ())) 42 then 0 else 1"),
        ("strict-unit", "  foreign import ccall \"ffi_increment\" tick :: Unit ->{Foreign Test} Unit\n  foreign import ccall \"ffi_take\" take :: Unit -> I64 ->{Foreign Test} I64\n", "if eq (take (tick ()) 42) 42 then 0 else 1"),
        ("pointer", "  opaque type Handle :: Type\n  foreign import ccall \"ffi_open\" open :: Unit ->{Foreign Test} Borrowed Handle\n  foreign import ccall \"h_read\" read :: Borrowed Handle ->{Foreign Test} I64\n", "if eq (read (open ())) 42 then 0 else 1"),
        ("pointer-branch", "  opaque type Handle :: Type\n  foreign import ccall \"ffi_open\" open :: Unit ->{Foreign Test} Borrowed Handle\n  foreign import ccall \"h_read\" read :: Borrowed Handle ->{Foreign Test} I64\n  pick :: Bool ->{Foreign Test} Borrowed Handle\n  pick flag = if flag then open () else open ()\n", "if eq (read (pick True)) 42 then 0 else 1"),
        ("nullable", "  opaque type Handle :: Type\n  foreign import ccall \"ffi_null\" nothing :: Unit ->{Foreign Test} MaybeNull Handle\n", "let p = nothing () in 0"),
    ] {
        let source = format!("module Main where\n{eq}{declarations}  main :: Unit ->{{Foreign Test}} Int\n  main unit = {body}\n");
        check_and_run(name, &source, manifest);
    }
    for (name, ty, value) in [
        ("i8", "I8", "(-7)"), ("u8", "U8", "249"),
        ("i16", "I16", "(-1234)"), ("u16", "U16", "60000"),
        ("i32", "I32", "(-1234567)"), ("u32", "U32", "4000000000"),
    ] {
        // Direct literals independently test argument extension; a C
        // return can already carry extended bits and mask caller errors.
        let declarations = format!("  foreign import ccall \"ffi_{name}\" probe :: {ty} ->{{Foreign Test}} I64\n  foreign import ccall \"make_{name}\" make :: Unit ->{{Foreign Test}} {ty}\n");
        let source = format!("module Main where\n{eq}{declarations}  main :: Unit ->{{Foreign Test}} Int\n  main unit = if eq (probe {value}) 42 then (if eq (probe (make ())) 42 then 0 else 2) else 1\n");
        check_and_run(name, &source, manifest);
    }

    let host_dir = root.join("host"); std::fs::create_dir(&host_dir).unwrap();
    let host = host_dir.join("Host.cinch"); let prefix = host_dir.join("host");
    let source = "module Host where\n  export Handle, open, read-handle, probe\n  opaque type Handle :: Type\n  foreign import ccall \"ffi_open\" open :: Unit ->{Foreign Host, IO} Borrowed Handle\n  foreign import ccall \"h_read\" read-handle :: Borrowed Handle ->{Foreign Host, IO} I64\n  foreign import ccall \"ffi_i64\" probe :: I64 ->{Foreign Host, IO} I64\n";
    std::fs::write(&host, source).unwrap();
    assert!(run(compiler, &[&host, &prefix], "--emit-interfaces", manifest, objects).status.success(), "{}", std::fs::read_to_string(prefix.with_extension("diagnostics")).unwrap_or_default());
    let iface_path = host_dir.join("host.Host.cinchif");
    let actual = cinch::interface::Iface::from_json(&std::fs::read_to_string(&iface_path).unwrap()).unwrap();
    // Native is an independent metadata oracle, never the client frontend.
    let (_, oracle) = cinch::driver::check_sources_ifaces(source, "foreign-interface-oracle.cinch").unwrap();
    // Compare actual file contracts. Native keeps source order internally,
    // while its canonical serializer sorts the foreign table for hashing.
    let expected = oracle.iter().find(|i| i.module == "Host").unwrap();
    let expected = cinch::interface::Iface::from_json(&expected.to_json()).unwrap();
    assert_eq!(actual.exports.foreign, expected.exports.foreign);
    assert_eq!(actual.exports.values, expected.exports.values);
    assert_eq!(actual.assumptions.foreign, expected.assumptions.foreign);
    let host_manifest = root.join("host.interfaces.json");
    interfaces(&host_manifest, &[&provider.join("build"), &host_dir]);
    let direct = format!("module Main where\n  import Host\n{eq}  main :: Unit ->{{Foreign Boundary, IO}} Int\n  main unit = if eq (read-handle (open ())) 42 then 0 else 1\n");
    check_and_run("interface-consumer", &direct, &host_manifest);
    let facade_dir = root.join("facade"); std::fs::create_dir(&facade_dir).unwrap();
    let facade = facade_dir.join("Facade.cinch");
    std::fs::write(&facade, "module Facade where\n  import Host (Handle as H, open as acquire, read-handle as inspect)\n  export H, acquire, inspect\n").unwrap();
    assert!(run(compiler, &[&facade, &facade_dir.join("facade")], "--emit-interfaces", &host_manifest, objects).status.success());
    let facade_manifest = root.join("facade.interfaces.json");
    interfaces(&facade_manifest, &[&provider.join("build"), &host_dir, &facade_dir]);
    let facade_client = format!("module Main where\n  import Facade\n{eq}  main :: Unit ->{{Foreign Boundary, IO}} Int\n  main unit = if eq (inspect (acquire ())) 42 then 0 else 1\n");
    check_and_run("facade-consumer", &facade_client, &facade_manifest);
    let proof_client = format!("module Main where\n  import Host\n{eq}  main :: Unit ->{{Foreign Boundary, IO}} Int\n  main unit = if eq (probe 40) 42 then 0 else 1\n");
    check_and_run("imported-scalar", &proof_client, &host_manifest);
    // Admitting a C-only interface must not relax the hash of a module
    // whose implementation actually comes from the provider object.
    // Provider is a leaf: no other admitted module imports it. Mutating a
    // base module instead would fail its dependents' import hashes first.
    let represented_path = provider.join("build/Provider.cinchif");
    let represented_bytes = std::fs::read_to_string(&represented_path).unwrap();
    let mut represented = cinch::interface::Iface::from_json(&represented_bytes).unwrap();
    let represented_hash = represented.iface_hash();
    represented.assumptions.foreign.push("ffi_unrecorded".into());
    assert_ne!(represented.iface_hash(), represented_hash);
    std::fs::write(&represented_path, represented.to_json()).unwrap();
    interfaces(&host_manifest, &[&provider.join("build"), &host_dir]);
    let input = root.join("wrong-object.cinch"); let output = root.join("wrong-object.ll");
    std::fs::write(&input, &direct).unwrap();
    assert_eq!(run(compiler, &[&input, &output], "--emit-ir", &host_manifest, objects).status.code(), Some(7));
    assert!(std::fs::read(&output).unwrap().is_empty());
    assert!(std::fs::read_to_string(output.with_extension("ll.diagnostics")).unwrap().contains("checked interface identity mismatch"));
    std::fs::write(&represented_path, represented_bytes).unwrap();
    interfaces(&host_manifest, &[&provider.join("build"), &host_dir]);
    // A parent module cannot manufacture an opaque header belonging to
    // an unadmitted nested module, even with fresh, consistent ABI hashes.
    let mut wrong_owner = actual.clone();
    for ty in &mut wrong_owner.exports.types {
        ty.identity = ty.identity.replace("Host.Handle", "Host.Nested.Handle");
    }
    for value in &mut wrong_owner.exports.values {
        value.scheme = value.scheme.replace("Host.Handle", "Host.Nested.Handle");
    }
    for foreign in &mut wrong_owner.exports.foreign {
        foreign.ty = foreign.ty.replace("Host.Handle", "Host.Nested.Handle");
        foreign.result.ty = foreign.result.ty.replace("Host.Handle", "Host.Nested.Handle");
        for parameter in &mut foreign.params {
            parameter.ty = parameter.ty.replace("Host.Handle", "Host.Nested.Handle");
        }
    }
    let text = wrong_owner.to_json();
    assert_eq!(cinch::interface::Iface::from_json(&text).unwrap(), wrong_owner);
    std::fs::write(&iface_path, text).unwrap();
    interfaces(&host_manifest, &[&provider.join("build"), &host_dir]);
    let input = root.join("wrong-owner.cinch"); let output = root.join("wrong-owner");
    std::fs::write(&input, &direct).unwrap();
    assert_eq!(run(compiler, &[&input, &output], "--check", &host_manifest, objects).status.code(), Some(4));
    assert!(std::fs::read_to_string(output.with_extension("diagnostics")).unwrap().contains("Host.Nested.Handle"));
    std::fs::write(&iface_path, actual.to_json()).unwrap();
    for nullable in [false, true] {
        let mut wrong = actual.clone();
        if nullable { wrong.exports.foreign[0].result.nullable = true; }
        else { wrong.exports.foreign[0].result.width = 4; }
        std::fs::write(&iface_path, wrong.to_json()).unwrap();
        interfaces(&host_manifest, &[&provider.join("build"), &host_dir]);
        let input = root.join("wrong.cinch"); let output = root.join("wrong");
        std::fs::write(&input, &direct).unwrap();
        assert_eq!(run(compiler, &[&input, &output], "--check", &host_manifest, objects).status.code(), Some(4));
        assert!(std::fs::read_to_string(output.with_extension("diagnostics")).unwrap().contains("ABI contract disagree"));
    }
    std::fs::write(&iface_path, actual.to_json()).unwrap();
    // The forged graph is well typed, closed, and has this exact foreign
    // identity and machine carrier types. It still grants no authority to
    // replace a foreign operation with a pure body or use it in proofs.
    let graph = cinch::total::CheckedGraph::check(vec![cinch::total::Definition {
        identity: "Host.probe".into(), parameters: vec![cinch::total::Sort::I64],
        result: cinch::total::Sort::I64, body: cinch::total::Expr::Local(0),
    }]).unwrap();
    let mut wrong = actual.clone();
    let claimed = wrong.exports.values.iter_mut().find(|v| v.name == "probe").unwrap();
    claimed.resolved = Some(cinch::interface::ResolvedScheme {
        scheme: "(I64 -> I64)".into(), nominals: vec![], measures: vec![],
    });
    claimed.total_body = Some(graph);
    let text = wrong.to_json();
    assert_eq!(cinch::interface::Iface::from_json(&text).unwrap(), wrong);
    std::fs::write(&iface_path, text).unwrap();
    interfaces(&host_manifest, &[&provider.join("build"), &host_dir]);
    let input = root.join("wrong-body.cinch"); let output = root.join("wrong-body");
    std::fs::write(&input, &proof_client).unwrap();
    assert_eq!(run(compiler, &[&input, &output], "--check", &host_manifest, objects).status.code(), Some(4));
    assert!(std::fs::read_to_string(output.with_extension("diagnostics")).unwrap().contains("foreign value cannot grant resolved total-body authority"));
    std::fs::write(&iface_path, actual.to_json()).unwrap();
    for body in [
        "foreign import ccall \"ffi_i64\" probe :: I64 -> I64",
        "foreign import ccall \"ffi_i64\" probe :: Int ->{Foreign Test} Int",
        "foreign import ccall \"ffi_i64\" probe :: (I64 -> I64) ->{Foreign Test} I64",
    ] {
        let input = root.join("refuse.cinch"); let output = root.join("refuse");
        std::fs::write(&input, format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n")).unwrap();
        assert_eq!(run(compiler, &[&input, &output], "--check", manifest, objects).status.code(), Some(4));
        assert!(!std::fs::read(output.with_extension("diagnostics")).unwrap().is_empty());
    }
}
fn resolved_application_contracts(compiler: &Path, dir: &Path, manifest: &Path, objects: &Path) {
    let root = dir.join("resolved-application-contracts");
    std::fs::create_dir(&root).unwrap();
    let source = r#"module ContractShapes where
  data Pair a b where
    Pair :: a -> b -> Pair a b
  identity :: forall a b. Pair a b -> Pair a b
  identity value = value
  inspect :: forall r s. Scope r s -> Int
  inspect token = 7
  pending :: forall s e a. Unit -> Task s e a
  pending unit = pending unit
"#;
    let (_, native) = cinch::driver::check_sources_ifaces(source, "native-application-contracts.cinch").unwrap();
    let expected = native.iter().find(|iface| iface.module == "ContractShapes").unwrap();
    let input = root.join("ContractShapes.cinch");
    let prefix = root.join("written");
    std::fs::write(&input, source).unwrap();
    assert!(run(compiler, &[&input, &prefix], "--emit-interfaces", manifest, objects).status.success());
    let actual = cinch::interface::Iface::from_json(&std::fs::read_to_string(root.join("written.ContractShapes.cinchif")).unwrap()).unwrap();
    for name in ["identity", "inspect", "pending"] {
        let value = actual.exports.values.iter().find(|value| value.name == name).unwrap();
        let reference = expected.exports.values.iter().find(|value| value.name == name).unwrap();
        assert!(value.resolved.is_some(), "missing real resolved value contract: {name}");
        assert_eq!(value.resolved, reference.resolved, "actual Cinch writer disagrees with Native: {name}");
    }
    let identity = actual.exports.values.iter().find(|value| value.name == "identity").unwrap();
    assert!(identity.scheme.contains("ContractShapes.Pair a b"), "public spelling: {}", identity.scheme);
    assert!(identity.resolved.as_ref().unwrap().scheme.contains("((ContractShapes.Pair a) b)"), "canonical spelling lost binary application");
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
  export make, peek, column, Box, wrap, unwrap, discard-word, mixed
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
  discard-word :: I64 %0 -> Int
  discard-word _ = 7
  mixed :: Int -> I64 %0 -> Int -> Int
  mixed first _ last = first + last
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
    assert_eq!(metadata.get("runtime_abi").and_then(Json::as_num), Some(f64::from(cinch::interface::RUNTIME_ABI_VERSION)));
    let constructor = metadata.get("units").and_then(Json::as_arr).unwrap().iter().find(|u| u.get("symbol").and_then(Json::as_str) == Some("Library.Box")).expect("actual concrete constructor root");
    assert_eq!(constructor.get("result").and_then(Json::as_str), Some("O"));
    assert_eq!(constructor.get("arity").and_then(Json::as_num), Some(1.0));
    let make = metadata.get("units").and_then(Json::as_arr).unwrap().iter().find(|u| u.get("symbol").and_then(Json::as_str) == Some("Library.make")).unwrap();
    assert_eq!(make.get("params").and_then(Json::as_arr).unwrap()[0].as_str(), Some("T"));
    assert_eq!(make.get("result").and_then(Json::as_str), Some("O"));
    let iface = cinch::interface::Iface::from_json(&std::fs::read_to_string(build.join("Library.cinchif")).unwrap()).unwrap();
    assert!(!iface.exports.values.iter().any(|v| v.name == "hidden"));
    for name in ["make", "peek", "column", "wrap", "unwrap"] { assert!(iface.exports.values.iter().any(|v| v.name == name)); }
    let units = metadata.get("units").and_then(Json::as_arr).unwrap();
    for (name, params) in [("discard-word", &[][..]), ("mixed", &["T"][..])] {
        let exported = iface.exports.values.iter().find(|value| value.name == name).unwrap();
        assert!(exported.scheme.contains("%0"), "{}", exported.scheme);
        let symbol = format!("Library.{name}");
        let unit = units.iter()
            .find(|unit| unit.get("symbol").and_then(Json::as_str) == Some(symbol.as_str()))
            .unwrap();
        assert_eq!(unit.get("arity").and_then(Json::as_num), Some(params.len() as f64));
        let physical: Vec<_> = unit.get("params").and_then(Json::as_arr).unwrap()
            .iter().map(|param| param.as_str().unwrap()).collect();
        assert_eq!(physical.as_slice(), params);
        assert_eq!(unit.get("result").and_then(Json::as_str), Some("O"));
    }
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
  main =
    let erased-ok = discard-word (429865131 :: I64) == 7 && mixed 20 (829054163 :: I64) 22 == 42 in
    if erased-ok && array-length (make 42) == 1 && unwrap (wrap 184467440737095516160) == 184467440737095516160 then 0 else 1
"#).unwrap();
    let result = run(&compiler, &[&client, &client_ir], "--emit-ir", &final_manifest, &final_objects);
    assert!(result.status.success(), "{}", common::read_output(&dir, "client.ll.diagnostics"));
    std::fs::copy(&client, dir.join("successful.client.source")).unwrap();
    std::fs::copy(&client_ir, dir.join("successful.client.ll")).unwrap();
    let client_llvm = std::fs::read_to_string(&client_ir).unwrap();
    assert!(!client_llvm.contains("429865131"));
    assert!(!client_llvm.contains("829054163"));
    assert!(client_llvm.lines().any(|line|
        line.contains("call i64 @__apply(i64 ") && line.contains(", i64 0, ptr ")));
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    let executable = dir.join("client");
    let linked = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into())).arg(&client_ir).arg(&out).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(&executable).output().unwrap();
    assert!(linked.status.success(), "{}", String::from_utf8_lossy(&linked.stderr));
    let ran = Command::new(executable).env("CINCH_HEAP_LIMIT", "32M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert_eq!(ran.status.code(), Some(0), "{}", String::from_utf8_lossy(&ran.stderr));
    assert!(String::from_utf8_lossy(&ran.stderr).contains("live=0 immortal=0 leaked=0"));
    println!("closed object client: {}", String::from_utf8_lossy(&ran.stderr).trim());
    std::fs::write(&client, "module Main where\n  import Library (hidden)\n  main :: Int\n  main = hidden 0\n").unwrap();
    assert_eq!(run(&compiler, &[&client, &client_ir], "--emit-ir", &final_manifest, &final_objects).status.code(), Some(4));
    assert!(std::fs::read_to_string(&client_ir).unwrap().is_empty());
    std::fs::write(&client, "module Library where\n  hidden :: Int -> Int\n  hidden n = n\n").unwrap();
    std::fs::write(&client_ir, "stale IR").unwrap();
    assert_eq!(run(&compiler, &[&client, &client_ir], "--emit-ir", &final_manifest, &final_objects).status.code(), Some(4));
    assert!(common::read_output(&dir, "client.ll.diagnostics").contains("module Library is already supplied by the library object"));
    assert!(std::fs::read(&client_ir).unwrap().is_empty());
    // A failed rebuild invalidates its previously good manifest.
    std::fs::write(&library, "module Library where\n  broken = (\n").unwrap();
    assert!(!run(&compiler, &[&library, &ir, &out, &abi, Path::new(&prefix)], "--emit-object", &manifest, &objects).status.success());
    assert!(std::fs::read(&abi).unwrap().is_empty());
    resolved_application_contracts(&compiler, &dir, &manifest, &objects);
    foreign_roundtrip(&compiler, &dir, &provider, &manifest, &objects, &runtime);
    if retained.is_none() { common::cleanup(&dir); common::cleanup(&provider); }
}

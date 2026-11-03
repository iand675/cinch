//! Checked byte observations retain their input carrier, scope and primitive identity.
mod common;
#[test]
fn byte_observations_recheck_typed_input_scope_and_runtime_identity() {
    let dir=common::project("byte_observations", &[
        ("Bootstrap.Type.cinch",include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch",include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Main.cinch",include_str!("../../bootstrap/programs/Run.ByteObservations.cinch")),
    ]);
    let exe=common::build(&dir,"Main.cinch","byte-observations").expect("build byte observation checker");
    for size in [0,1,17,129,4096] {
        std::fs::write(dir.join("nonce.txt"),vec![b'x';size]).unwrap();
        let result=std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(result.status.code(),Some(0),"observation check {size}; project {}\n{}",dir.display(),String::from_utf8_lossy(&result.stderr));
    }
    common::cleanup(&dir);
}

#[test]
fn source_observations_use_declared_primitive_identity_and_actual_buffer() {
    let dir=common::project("source_byte_observations", &[
        ("Bootstrap.Check.cinch",include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch",include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch",include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch",include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch",include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch",include_str!("../../bootstrap/programs/Run.SourceRefinements.cinch")),
    ]);
    let exe=common::build(&dir,"Main.cinch","source-observations").expect("build Cinch source observation checker");
    let cases=[
        ("nonnegative observation","nonnegative :: Bytes -> refined (out >= 0) Nat\n  nonnegative b = extent b",0),
        ("machine-sized observation","bounded :: Bytes -> refined (out <= 9223372036854775807) Nat\n  bounded b = extent b",0),
        ("refined buffer introduction","keep :: using (extent b > 0) => (b :: Bytes) -> refined (extent out > 0) Bytes\n  keep b = b",0),
        ("named buffer refinement","type NonEmpty = refine (b :: Bytes) with extent b > 0\n  size :: NonEmpty -> refined (out > 0) Nat\n  size b = extent b",0),
        ("wrong buffer refinement","bad :: using (extent b > 0) => (b :: Bytes) -> (c :: Bytes) -> refined (extent out > 0) Bytes\n  bad b c = c",4),
        ("actual result identity","same :: (b :: Bytes) -> refined (out == extent b) Nat\n  same b = extent b",0),
        ("using actual buffer","keep :: using (n < extent b) => (b :: Bytes) -> (n :: Nat) -> refined (out < extent b) Nat\n  keep b n = n",0),
        ("primitive branch fact","positive :: Bytes -> refined (out > 0) Nat\n  positive b = if extent b > 0 then extent b else 1",0),
        ("once-bound observation","same :: (b :: Bytes) -> refined (out == extent b) Nat\n  same b = let size = extent b in size",0),
        ("arithmetic observation","next :: (b :: Bytes) -> refined (out == extent b + 1) Nat\n  next b = extent b + 1",0),
        ("different buffer cannot borrow fact","bad :: using (extent b > 0) => (b :: Bytes) -> (c :: Bytes) -> refined (out > 0) Nat\n  bad b c = extent c",4),
        ("ordinary name cannot lend fact","bytes-len :: Bytes -> Nat\n  bytes-len b = 0\n  bad :: Bytes -> refined (out > 0) Nat\n  bad b = if bytes-len b > 0 then 0 else 1",4),
        ("wrong source index operand","bad :: (n :: Nat) -> refined (out == extent n) Nat\n  bad n = n",4),
        ("ordinary observation type call","reader :: Bytes -> Nat\n  reader b = 0\n  bad :: (b :: Bytes) -> refined (out == reader b) Nat\n  bad b = 0",4),
        ("missing bound buffer","bad :: refined (out == extent unknown) Nat\n  bad = 0",4),
        ("observation cannot strengthen","bad :: (b :: Bytes) -> refined (out > extent b) Nat\n  bad b = extent b",4),
        ("observation scope cannot escape","bad :: Bytes -> refined (out > 0) Nat\n  bad b = let ignored = if extent b > 0 then extent b else 1 in extent b",4),
    ];
    for (name,body,status) in cases {
        let source=format!("module Main where\n  foreign import ccall \"cinch.runtime.bytesLen\" extent :: Bytes -> Nat\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"),&source).unwrap();
        let result=std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report=common::read_output(&dir,"refinement-report.txt");
        assert_eq!(result.status.code(),Some(status),"{name}; project {}\n{source}\n{report}\n{}",dir.display(),String::from_utf8_lossy(&result.stderr));
    }
    common::cleanup(&dir);
}


#[test]
fn imported_observation_identity_survives_a_renamed_facade() {
    use cinch::interface::Iface;
    fn quote(s:&str)->String {
        let mut out=String::from("\"");
        for c in s.chars(){match c{'"'=>out.push_str("\\\""),'\\'=>out.push_str("\\\\"),c if c<='\u{1f}'=>out.push_str(&format!("\\u{:04x}",c as u32)),c=>out.push(c)}}
        out.push('"');out
    }
    let producer=common::project("byte_observation_producer",&[
        ("A.cinch","module A where\n  foreign import ccall \"cinch.runtime.bytesLen\" extent :: Bytes -> Nat\n"),
        ("Facade.cinch","module Facade where\n  import A (extent as measure)\n  import Data.Bytes (bytes-len as bounded-measure)\n  export measure, bounded-measure\n"),
        ("Main.cinch","module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ]);
    common::check_library_fixture_native(&producer,"Main.cinch").expect("emit real primitive/facade interfaces");
    let dir=common::project("imported_byte_observations",&[
        ("Bootstrap.Check.cinch",include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch",include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch",include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch",include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch",include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch",include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch",include_str!("../../bootstrap/programs/Run.ImportedPreconditions.cinch")),
    ]);
    let mut rows=Vec::new();
    for entry in std::fs::read_dir(producer.join("build")).unwrap(){
        let path=entry.unwrap().path();
        if path.extension().is_some_and(|s|s=="cinchif"){
            let iface=Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module!="Main"{rows.push(format!("[{},{},{}]",quote(&iface.module),quote(&iface.iface_hash()),quote(&path.display().to_string())));}
        }
    }
    rows.sort();std::fs::write(dir.join("interfaces.json"),format!("{{\"schema\":1,\"interfaces\":[{}]}}",rows.join(","))).unwrap();
    let exe=common::build(&dir,"Main.cinch","imported-byte-observations").expect("build imported byte observer consumer");
    for (name,body,status) in [
        ("checked base refinement result","same :: (b :: Bytes) -> refined (out == bounded-measure b) Nat\n  same b = bounded-measure b",0),
        ("checked base refinement branch","positive :: Bytes -> refined (out > 0) Nat\n  positive b = if bounded-measure b > 0 then bounded-measure b else 1",0),
        ("once-bound checked base result","same :: (b :: Bytes) -> refined (out == bounded-measure b) Nat\n  same b = let size = bounded-measure b in size",0),
        ("renamed primitive result","same :: (b :: Bytes) -> refined (out == measure b) Nat\n  same b = measure b",0),
        ("renamed primitive branch","positive :: Bytes -> refined (out > 0) Nat\n  positive b = if measure b > 0 then measure b else 1",0),
        ("defining using identity","keep :: using (n < measure b) => (b :: Bytes) -> (n :: Nat) -> refined (out < measure b) Nat\n  keep b n = n",0),
        ("different buffer","bad :: using (measure b > 0) => (b :: Bytes) -> (c :: Bytes) -> refined (out > 0) Nat\n  bad b c = measure c",4),
        ("local shadow","measure :: Bytes -> Nat\n  measure b = 0\n  bad :: (b :: Bytes) -> refined (out == measure b) Nat\n  bad b = 0",4),
    ]{
        let source=format!("module Main where\n  import Facade (measure, bounded-measure)\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"),&source).unwrap();
        let result=std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report=common::read_output(&dir,"imported-report.txt");
        assert_eq!(result.status.code(),Some(status),"{name}; project {}\n{source}\n{report}\n{}",dir.display(),String::from_utf8_lossy(&result.stderr));
    }
    let a_path=producer.join("build/A.cinchif");
    let original=Iface::from_json(&std::fs::read_to_string(&a_path).unwrap()).unwrap();
    for (label,mutated_field,expected_message) in [
        ("carrier declaration disagreement","type","foreign/value schemes disagree"),
        ("wrong intrinsic ABI","abi","ABI contract"),
    ] {
        let mut changed=original.clone();
        let contract=changed.exports.foreign.iter_mut().find(|x|x.external=="cinch.runtime.bytesLen").unwrap();
        if mutated_field=="type" { contract.ty="(Nat -> Nat)".into(); } else { contract.abi="imaginary".into(); }
        std::fs::write(&a_path,changed.to_json()).unwrap();
        let mut input_rows=Vec::new();
        for entry in std::fs::read_dir(producer.join("build")).unwrap(){
            let path=entry.unwrap().path();
            if path.extension().is_some_and(|s|s=="cinchif"){
                let iface=Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
                if iface.module!="Main" && iface.module!="Facade" {input_rows.push(format!("[{},{},{}]",quote(&iface.module),quote(&iface.iface_hash()),quote(&path.display().to_string())));}
            }
        }
        input_rows.sort();std::fs::write(dir.join("interfaces.json"),format!("{{\"schema\":1,\"interfaces\":[{}]}}",input_rows.join(","))).unwrap();
        std::fs::write(dir.join("source.cinch"),"module Main where\n  import A (extent)\n  same :: (b :: Bytes) -> refined (out == extent b) Nat\n  same b = extent b\n  main :: Int\n  main = 0\n").unwrap();
        let result=std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report=common::read_output(&dir,"imported-report.txt");
        assert_eq!(result.status.code(),Some(4),"{label}: {report}");
        assert!(report.contains(expected_message),"{label}: {report}");
    }
    common::cleanup(&producer);common::cleanup(&dir);
}

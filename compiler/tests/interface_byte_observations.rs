//! The real Native producer closes primitive observations across private aliases.
mod common;
use cinch::interface::Iface;
fn quote(text:&str)->String {
    let mut out=String::from("\"");
    for c in text.chars(){match c{'"'=>out.push_str("\\\""),'\\'=>out.push_str("\\\\"),c if c<='\u{1f}'=>out.push_str(&format!("\\u{:04x}",c as u32)),c=>out.push(c)}}
    out.push('"');out
}
#[test]
fn private_observations_close_over_the_kernel_and_actual_buffer_identity() {
    let source=common::project("observation_contract_producer",&[
        ("A.cinch",r#"module A where
  export take-at, copy-size, make-positive, read-positive, not-portable
  foreign import ccall "cinch.runtime.bytesLen" extent :: Bytes -> Nat
  take-at :: using (i < extent b) => (b :: Bytes) -> (i :: Nat) -> Nat
  take-at b i = i
  copy-size :: (b :: Bytes) -> refined (r == extent b) Nat
  copy-size b = extent b
  type PositiveBytes = refine (b :: Bytes) with extent b > 0
  make-positive :: using (extent b > 0) => (b :: Bytes) -> PositiveBytes
  make-positive b = b
  read-positive :: PositiveBytes -> refined (r > 0) Nat
  read-positive b = extent b
  bytes-len :: Nat -> Nat
  bytes-len n = n
  not-portable :: using (bytes-len n > 0) => (n :: Nat) -> Nat
  not-portable n = n
"#),
        ("Facade.cinch","module Facade where\n  import A (take-at as at, copy-size, make-positive, read-positive)\n  export at, copy-size, make-positive, read-positive\n"),
        ("Main.cinch","module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&source,"Main.cinch").expect("check and emit real observation contracts");
    let a=Iface::from_json(&std::fs::read_to_string(source.join("build/A.cinchif" )).unwrap()).unwrap();
    for name in ["take-at","copy-size","make-positive","read-positive"] {
        let export=a.exports.values.iter().find(|v|v.name==name).unwrap();
        assert!(export.resolved.is_some(),"{name}: {export:?}");
    }
    assert!(a.exports.values.iter().find(|v|v.name=="copy-size").unwrap().resolved.as_ref().unwrap().scheme.contains("b.bytesLen"));
    assert!(a.exports.values.iter().find(|v|v.name=="make-positive").unwrap().resolved.as_ref().unwrap().nominals.iter().any(|n|n.identity=="A.PositiveBytes" && n.refinement.as_ref().is_some_and(|r|r.predicate.contains("b.bytesLen"))));
    assert!(a.exports.values.iter().find(|v|v.name=="not-portable").unwrap().resolved.is_none(),"ordinary same-spelled function must not be exported as a primitive observation");
    let dir=common::project("observation_contract_consumer",&[
        ("Bootstrap.Check.cinch",include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch",include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch",include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch",include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch",include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch",include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch",include_str!("../../bootstrap/programs/Run.ImportedPreconditions.cinch")),
    ]);
    let mut rows=Vec::new();
    for entry in std::fs::read_dir(source.join("build")).unwrap(){
        let path=entry.unwrap().path();
        if path.extension().is_some_and(|e|e=="cinchif"){
            let iface=Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module!="Main" {rows.push(format!("[{},{},{}]",quote(&iface.module),quote(&iface.iface_hash()),quote(&path.display().to_string())));}
        }
    }
    rows.sort();std::fs::write(dir.join("interfaces.json"),format!("{{\"schema\":1,\"interfaces\":[{}]}}",rows.join(","))).unwrap();
    let exe=common::build(&dir,"Main.cinch","observation-contracts").expect("build real imported observation contract checker");
    for (name,body,status) in [
        ("checked result closes private primitive","safe :: Bytes -> Nat -> Nat\n  safe b i = let n = copy-size b in if i < n then at b i else 0",0),
        ("wrong buffer cannot borrow size","bad :: Bytes -> Bytes -> Nat -> Nat\n  bad b c i = let n = copy-size c in if i < n then at b i else 0",4),
        ("unchecked bound cannot discharge condition","bad :: Bytes -> Nat\n  bad b = at b 0",4),
        ("private buffer refinement survives facade","safe :: Bytes -> Nat\n  safe b = let n = copy-size b in if n > 0 then read-positive (make-positive b) else 0",0),
        ("private refinement requires its proof","bad :: Bytes -> Nat\n  bad b = read-positive (make-positive b)",4),
        ("private primitive remains hidden","bad :: Bytes -> Nat\n  bad b = extent b",4),
        ("private type remains hidden","bad :: PositiveBytes -> Nat\n  bad b = 0",4),
        ("wire operation is not source binding","bad :: (b :: Bytes) -> refined (out == b.bytesLen b) Nat\n  bad b = 0",4),
    ]{
        let input=format!("module Main where\n  import Facade (at, copy-size, make-positive, read-positive)\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"),&input).unwrap();
        let result=std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report=common::read_output(&dir,"imported-report.txt");
        assert_eq!(result.status.code(),Some(status),"{name}; project {}\n{input}\n{report}\n{}",dir.display(),String::from_utf8_lossy(&result.stderr));
    }
    common::cleanup(&source);common::cleanup(&dir);
}

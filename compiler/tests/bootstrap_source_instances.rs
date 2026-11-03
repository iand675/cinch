//! Real dictionary constructor contracts and capture-avoiding specialization.
mod common;
#[test]
fn instance_contracts_recheck_slots_captures_and_method_specialization() {
    let dir = common::project("source_instance_contracts", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Instance.cinch", include_str!("../../bootstrap/Bootstrap.Instance.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.InstanceContracts.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "instance-contracts").expect("build typed source dictionary contracts");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(result.status.code(), Some(0), "instance contract group at nonce {size}; project {}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
    }
    common::cleanup(&dir);
}

fn quote(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"),
            c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"'); out
}
#[test]
fn source_instances_build_checked_dictionaries_from_defining_class_interfaces() {
    use cinch::interface::Iface;
    let producer = common::project("source_instance_classes", &[
        ("A.cinch", r#"module A where
  export Echo, echo, PairOps, z-first, a-last, Mapper, make-one, Tick, tick, Value, value
  signature Echo a where
    echo :: a -> a
  signature PairOps a where
    z-first :: a -> a
    a-last :: forall b. using (extra :: Echo b) => b -> a -> a
  signature Mapper (f :: Type -> Type) where
    make-one :: forall a. a -> f a
  signature Tick a where
    tick :: (n :: Nat) -> a -> a
  signature Value a where
    value :: a
"#),
        ("Facade.cinch", "module Facade where\n  import A (Echo, echo, PairOps as Pair, z-first, a-last, Mapper, make-one, Tick, tick, Value, value)\n  export Echo, echo, Pair, z-first, a-last, Mapper, make-one, Tick, tick, Value, value\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ]);
    common::check_library_fixture_native(&producer, "Main.cinch").expect("produce actual defining class contracts");
    let dir = common::project("source_instance_consumer", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceInstances.cinch")),
    ]);
    let mut rows = Vec::new();
    for entry in std::fs::read_dir(producer.join("build")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let iface = Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module != "Main" {
                rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
            }
        }
    }
    rows.sort(); std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "source-instances").expect("build actual Cinch source dictionary elaborator");
    for (name, body, wanted) in [
        ("imported class without source instance", "probe :: using (given :: Echo Int) => Int -> Int\n  probe x = echo x", 0),
        ("renamed imported class with method context", "probe :: using (pair :: Pair Int, extra :: Echo Bool) => Bool -> Int -> Int\n  probe b x = a-last b (z-first x)", 0),
        ("higher kind imported class without instance", "probe :: forall (f :: Type -> Type). using (given :: Mapper f) => Int -> f Int\n  probe x = make-one x", 0),
        ("monomorphic dictionary and method call", "implicit instance EchoInt :: Echo Int where\n    echo x = x\n  answer :: Int\n  answer = echo 1", 0),
        ("machine carrier specialization", "implicit instance EchoI64 :: Echo I64 where\n    echo x = x\n  answer :: I64\n  answer = echo (-1 :: I64)", 0),
        ("plain instance explicit value", "instance Explicit :: Echo Int where\n    echo x = x\n  answer :: Int\n  answer = with (chosen = Explicit) in echo 1", 0),
        ("ordinary value cannot supply a dictionary", "implicit instance EchoInt :: Echo Int where\n    echo x = x\n  answer :: Int\n  answer = with (chosen = 1) in echo 1", 4),
        ("plain instance excluded from search", "instance Explicit :: Echo Int where\n    echo x = x\n  answer :: Int\n  answer = echo 1", 4),
        ("slots follow declaration order", "implicit instance PairInt :: Pair Int where\n    a-last ignored x = x\n    z-first x = x", 0),
        ("method local quantifier and context", "implicit instance PairInt :: Pair Int where\n    z-first x = x\n    a-last ignored x = x\n  use :: using (given :: Echo Bool) => Bool -> Int\n  use b = a-last b 1", 0),
        ("method annotation alpha rename", "implicit instance PairInt :: Pair Int where\n    z-first x = x\n    a-last :: forall t. using (extra :: Echo t) => t -> Int -> Int\n    a-last ignored x = x", 0),
        ("higher kind class parameter", "implicit instance ListMaker :: Mapper List where\n    make-one x = Cons x Nil\n  answer :: List Int\n  answer = make-one 1", 0),
        ("factory context captured by field", "implicit instance EchoList a :: using (element :: Echo a) => Echo (List a) where\n    echo xs = case xs of\n      Nil -> Nil\n      Cons x rest -> Cons (echo x) rest", 0),
        ("generic factory selected with context", "implicit instance EchoInt :: Echo Int where\n    echo x = x\n  implicit instance EchoList a :: using (element :: Echo a) => Echo (List a) where\n    echo xs = xs\n  answer :: List Int\n  answer = echo (Cons 1 Nil)", 0),
        ("dependent method argument", "implicit instance TickInt :: Tick Int where\n    tick n x = x\n  answer :: Int\n  answer = tick 1 7", 0),
        ("value method", "implicit instance ValueInt :: Value Int where\n    value = 7\n  answer :: Int\n  answer = value", 0),
        ("missing method", "implicit instance PairInt :: Pair Int where\n    z-first x = x", 4),
        ("extra method", "implicit instance EchoInt :: Echo Int where\n    echo x = x\n    extra x = x", 4),
        ("wrong method body", "implicit instance EchoInt :: Echo Int where\n    echo x = True", 4),
        ("wrong generic method body", "implicit instance EchoList a :: Echo (List a) where\n    echo xs = Cons True Nil", 4),
        ("wrong annotation", "implicit instance EchoInt :: Echo Int where\n    echo :: Bool -> Bool\n    echo x = x", 4),
        ("overlapping implicit instances", "implicit instance First :: Echo Int where\n    echo x = x\n  implicit instance Second :: Echo Int where\n    echo x = x", 4),
        ("nonclass target", "implicit instance Bad :: Int where\n    echo x = x", 4),
        ("dictionary cannot also be defined", "implicit instance EchoInt :: Echo Int where\n    echo x = x\n  EchoInt = 0", 4),
    ] {
        let source = format!("module Main where\n  import Facade\n  import Data.Core (List, Cons, Nil)\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-instance-report.txt");
        assert_eq!(result.status.code(), Some(wanted), "{name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
        println!("source dictionary case passed: {name}");
    }
    let actual_json = "module Text.Json where\n  import Text.Codec (JsonCodec)\n  import Data.Json (Json)\n  import Data.Result (Ok)\n  implicit instance JsonCodec_Json :: JsonCodec Json where\n    encode-json j = j\n    decode-json j = Ok j\nmodule Main where\n  main :: Int\n  main = 0\n";
    std::fs::write(dir.join("source.cinch"), actual_json).unwrap();
    let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    assert_eq!(result.status.code(), Some(0), "actual Text.Json source instance; project {}\n{}\n{}", dir.display(), common::read_output(&dir, "source-instance-report.txt"), String::from_utf8_lossy(&result.stderr));
    println!("actual Text.Json source dictionary passed");
    common::cleanup(&producer); common::cleanup(&dir);
}

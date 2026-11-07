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
        ("inferred higher kind imported class", "probe :: forall f. using (given :: Mapper f) => Int -> f Int\n  probe x = make-one x", 0),
        ("inferred constructor only from dictionary", "probe :: forall f. using (given :: Mapper f) => Int -> Int\n  probe x = x", 0),
        ("inferred binary constructor", "signature PairCtor (f :: Type -> Type -> Type) where\n    pair :: forall a b. a -> b -> f a b\n  probe :: forall f. using (given :: PairCtor f) => Int -> Bool -> f Int Bool\n  probe x y = pair x y", 0),
        ("explicit proper kind is authoritative", "probe :: forall (f :: Type). using (given :: Mapper f) => Int -> f Int\n  probe x = make-one x", 4),
        ("inconsistent inferred constructor arity", "probe :: forall f. f Int -> f Int Bool -> f Int\n  probe x y = x", 4),
        ("dictionary kind contradicts application", "probe :: forall f. using (given :: Mapper f) => f Int Bool -> f Int Bool\n  probe x = x", 4),
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

#[test]
fn source_class_defaults_are_checked_at_the_owner_and_inherited_with_closed_contracts() {
    use cinch::interface::Iface;
    let dir = common::project("source_class_defaults", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Export.cinch", include_str!("../../bootstrap/Bootstrap.Export.cinch")),
        ("Bootstrap.ExportNominal.cinch", include_str!("../../bootstrap/Bootstrap.ExportNominal.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ClassDefaults.cinch")),
    ]);
    std::fs::write(dir.join("interfaces.json"), "{\"schema\":1,\"interfaces\":[]}").unwrap();
    let exe = common::build(&dir, "Main.cinch", "class-default-checker").expect("build actual Cinch class-default checker/writer");
    let owner = "module Defaults where\n  export Describe, size, twice\n  helper :: Int -> Int\n  helper n = n + n\n  signature Describe a where\n    size :: a -> Int\n    twice :: a -> Int\n    twice x = helper (size x)\n";
    let inherited = "module Main where\n  import Defaults\n  implicit instance DescribeInt :: Describe Int where\n    size x = x\n  answer :: Int\n  answer = twice 21\n  main :: Int\n  main = 0\n";
    let overridden = inherited.replace("    size x = x\n", "    size x = x\n    twice x = 7\n");
    let unused = "module Main where\n  import Defaults\n  main :: Int\n  main = 0\n";
    let mut default_id = None;
    for (name, source, wanted) in [
        ("inherited default", format!("{owner}{inherited}"), 0),
        ("explicit override", format!("{owner}{overridden}"), 0),
        ("unused valid default", format!("{owner}{unused}"), 0),
        ("owner private helper wins over caller helper", format!("{owner}{}", inherited.replace("  answer ::", "  helper :: Bool -> Bool\n  helper x = x\n  answer ::")), 0),
        ("changed default", format!("{}{inherited}", owner.replace("helper (size x)", "helper (size x) + 0")), 0),
        ("comment and span changes preserve default", format!("\n-- shifted source\n{}{inherited}", owner.replace("    twice x =", "    -- default implementation\n    twice x =")), 0),
        ("unused bad default", format!("{}{unused}", owner.replace("helper (size x)", "helper x")), 4),
        ("overrides cannot hide a bad default", format!("{}{overridden}", owner.replace("helper (size x)", "helper x")), 4),
        ("undeclared default", format!("{}{unused}", owner.replace("    twice x =", "    thrice x = 3\n    twice x =")), 4),
        ("missing nondefault method", format!("{owner}{}", inherited.replace("    size x = x\n", "    twice x = 7\n")), 4),
        ("bad override", format!("{owner}{}", overridden.replace("    twice x = 7", "    twice x = True")), 4),
        ("default helper stays private", format!("{owner}{}", inherited.replace("  answer = twice 21", "  answer = helper 21")), 4),
        ("class parameter annotation in inherited default", "module Defaults where\n  signature Identity a where\n    seed :: a -> a\n    identity :: a -> a\n    identity x = (x :: a)\nmodule Main where\n  import Defaults\n  implicit instance IdentityInt :: Identity Int where\n    seed x = x\n  main :: Int\n  main = identity 42\n".into(), 0),
        ("method local parameter annotation in inherited default", "module Defaults where\n  signature Pick a where\n    seed :: a -> a\n    pick :: forall b. b -> a -> b\n    pick x ignored = (x :: b)\nmodule Main where\n  import Defaults\n  implicit instance PickInt :: Pick Int where\n    seed x = x\n  main :: Int\n  main = pick 42 0\n".into(), 0),
        ("term and type names stay separate in inherited annotations", "module Defaults where\n  signature Identity a where\n    seed :: a -> a\n    identity :: a -> a\n    identity a = (a :: a)\nmodule Main where\n  import Defaults\n  implicit instance IdentityInt :: Identity Int where\n    seed x = x\n  main :: Int\n  main = identity 42\n".into(), 0),
        ("nullary default field", "module Defaults where\n  signature Value a where\n    value :: a\n    duplicate :: a\n    duplicate = value\nmodule Main where\n  import Defaults\n  implicit instance ValueInt :: Value Int where\n    value = 7\n  main :: Int\n  main = duplicate\n".into(), 0),
    ] {
        let native = cinch::driver::check_sources_ifaces(&source, "class-default-oracle.cinch");
        assert_eq!(native.is_ok(), wanted == 0, "Native oracle {name}: {native:?}\n{source}");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "class-default-report.txt");
        assert_eq!(result.status.code(), Some(wanted), "{name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
        if name == "missing nondefault method" {
            assert!(report.contains("instance DescribeInt does not define method size"), "{report}");
            let line = source.lines().position(|line| line.contains("implicit instance DescribeInt")).unwrap() + 1;
            assert!(report.contains(&format!("bootstrap.check:{line}:")), "instance refusal lost its actual source declaration: {report}");
        }
        if name == "undeclared default" {
            assert!(report.contains("`thrice` is not a method of Describe"), "{report}");
        }
        let text = common::read_output(&dir, "class-default-interfaces.jsonl");
        if wanted == 0 {
            let interfaces: Vec<_> = text.lines().map(|line| Iface::from_json(line).expect("independent public interface reader")).collect();
            cinch::interface::verify_chain(&interfaces).expect("source dependency hashes refer to the emitted owner");
            assert!(interfaces.iter().all(|i| i.exports.values.iter().all(|v| !v.name.contains("#class-default:"))), "a default acquired a source-visible name");
            if !matches!(name, "term and type names stay separate in inherited annotations" | "nullary default field" | "class parameter annotation in inherited default" | "method local parameter annotation in inherited default") {
                let defining = interfaces.iter().find(|i| i.module == "Defaults").unwrap();
                let describe = defining.exports.signatures.iter().find(|s| s.identity == "Defaults.Describe").unwrap();
                assert_eq!(describe.methods[0].name, "size");
                assert_eq!(describe.methods[0].default, None);
                assert_eq!(describe.methods[1].name, "twice");
                let identity = describe.methods[1].default.clone().expect("checked default body identity");
                if name == "inherited default" { default_id = Some(identity); }
                else if name == "changed default" { assert_ne!(Some(identity), default_id); }
                else { assert_eq!(Some(identity), default_id, "{name}"); }
            }
        } else { assert!(text.is_empty(), "failed input retained stale interface output"); }
        println!("source class default case passed: {name}");
    }

    let producer = "module Restore where\n  export Restore, seed, restored\n  signature Restore a where\n    seed :: a -> a\n    restored :: a -> a\n    restored x = seed x\n";
    let consumer = "module Main where\n  import Restore (Restore, restored)\n  implicit instance RestoreInt :: Restore Int where\n    seed x = x\n  main :: Int\n  main = restored 42\n";
    let run = |source: &str| {
        std::fs::write(dir.join("source.cinch"), source).unwrap();
        std::process::Command::new(&exe).current_dir(&dir).output().unwrap()
    };
    let result = run(producer);
    assert_eq!(result.status.code(), Some(0), "produce separate class default: {}", common::read_output(&dir, "class-default-report.txt"));
    let interfaces = common::read_output(&dir, "class-default-interfaces.jsonl");
    let mut restore = interfaces.lines().map(|line| Iface::from_json(line).unwrap())
        .find(|iface| iface.module == "Restore").unwrap();
    let method = &restore.exports.signatures[0].methods[1];
    assert_eq!(method.name, "restored");
    assert!(method.default.is_some(), "the producer must carry the actual clauses");
    let interface = dir.join("Restore.cinchif");
    let write_manifest = |interfaces: &[&Iface]| {
        let mut rows = Vec::new();
        for iface in interfaces {
            let path = dir.join(format!("{}.cinchif", iface.module));
            std::fs::write(&path, iface.to_json()).unwrap();
            rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
        }
        rows.sort();
        std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    };
    write_manifest(&[&restore]);
    assert_eq!(run(consumer).status.code(), Some(0), "inherit from the interface alone: {}", common::read_output(&dir, "class-default-report.txt"));

    let native_source = format!("{producer}\nmodule Main where\n  import Restore\n  main :: Int\n  main = 0\n");
    let native = cinch::driver::check_sources_ifaces(&native_source, "native-default-owner.cinch").unwrap().1;
    cinch::interface::verify_chain(&native).expect("Native owner and prelude dependency chain");
    let native_restore = native.iter().find(|iface| iface.module == "Restore").unwrap();
    assert!(native_restore.exports.signatures[0].methods[1].default.is_some());
    let native_inputs: Vec<_> = native.iter().filter(|iface| iface.module != "Main").collect();
    write_manifest(&native_inputs);
    assert_eq!(run(consumer).status.code(), Some(0), "inherit Native owner clauses in Cinch: {}", common::read_output(&dir, "class-default-report.txt"));

    let body = restore.exports.signatures[0].methods[1].default.take().unwrap();
    write_manifest(&[&restore]);
    assert_eq!(run(consumer).status.code(), Some(4), "the class no longer provides a default");
    assert!(common::read_output(&dir, "class-default-report.txt").contains("does not define method restored"));

    let clauses = body.to_json();
    restore.exports.signatures[0].methods[1].default = Some(body);
    write_manifest(&[&restore]);
    let document = restore.to_json();
    let altered_clauses = clauses.replace("\"x\"", "\"different\"");
    assert_ne!(clauses, altered_clauses);
    let tampered = document.replace(&clauses, &altered_clauses);
    let altered = Iface::from_json(&tampered).expect("the altered clauses still have the canonical shape");
    assert_ne!(restore.iface_hash(), altered.iface_hash());
    std::fs::write(&interface, tampered).unwrap();
    assert_eq!(run(consumer).status.code(), Some(90), "the manifest must authenticate the original default clauses");

    let fingerprint_only = document.replace(&clauses, "\"12345\"");
    assert!(Iface::from_json(&fingerprint_only).is_err(), "a digest is not a default clause array");
    std::fs::write(&interface, fingerprint_only).unwrap();
    assert_eq!(run(consumer).status.code(), Some(90), "the Cinch loader must reject the same malformed contract");

    common::cleanup(&dir);
}

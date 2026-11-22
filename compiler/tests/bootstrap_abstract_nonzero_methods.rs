//! Generic method carriers defer formation without inventing arithmetic facts.
mod common;
use cinch::interface::Iface;
fn quote(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)), c => out.push(c) }
    }
    out.push('"'); out
}
fn manifest(source: &std::path::Path, target: &std::path::Path) {
    let mut rows = Vec::new();
    for file in std::fs::read_dir(source.join("build")).unwrap() {
        let path = file.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let iface = Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module != "Main" { rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string()))); }
        }
    }
    rows.sort();
    std::fs::write(target.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
}


#[test]
fn real_div_and_rem_contracts_keep_abstract_carriers_until_specialization() {
    let producer = common::project("abstract_nonzero_producer", &[
        ("Main.cinch", "module Main where\n  import Algebra.Arithmetic\n  import Data.Nat\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&producer, "Main.cinch").expect("produce actual generic arithmetic contracts");
    let arithmetic = Iface::from_json(&std::fs::read_to_string(producer.join("build/Algebra.Arithmetic.cinchif")).unwrap()).unwrap();
    for name in ["Div", "Rem"] {
        let signature = arithmetic.exports.signatures.iter().find(|s| s.name == name).unwrap();
        let method = signature.methods.first().unwrap().resolved.as_ref().expect("resolved defining method contract");
        assert!(method.scheme.contains("(a :: Type)") && method.scheme.contains("Algebra.Arithmetic.NonZero a"), "{name}: {}", method.scheme);
        assert!(method.nominals.iter().any(|row| row.identity == "Algebra.Arithmetic.NonZero" && row.refinement.is_some()));
    }
    let dir = common::project("abstract_nonzero_consumer", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RefinementClosure.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "abstract-nonzero").expect("compile actual Cinch method consumer");
    manifest(&producer, &dir);
    for (name, body, status, diagnostic) in [
        ("generic refined identity", "keep :: forall a. NonZero a -> NonZero a\n  keep d = d", 0, ""),
        ("generic Div given", "forward :: forall a. using (dict :: Div a) => a -> NonZero a -> a\n  forward n d = n / d", 0, ""),
        ("generic Rem given", "forward :: forall a. using (dict :: Rem a) => a -> NonZero a -> a\n  forward n d = n % d", 0, ""),
        ("expected generic result checks first literal", "keep :: forall a. a -> a\n  keep x = x\n  probe :: Nat\n  probe = keep 7", 0, ""),
        ("expected generic curried result", "first :: forall a. a -> a -> a\n  first x y = x\n  probe :: Nat\n  probe = first 7 3", 0, ""),
        ("expected higher order result", "apply :: forall a. (a -> a) -> a -> a\n  apply f x = f x\n  keep :: forall a. a -> a\n  keep x = x\n  probe :: Nat\n  probe = apply keep 7", 0, ""),
        ("explicit Int argument cannot become Nat", "bad :: Nat\n  bad = 7 / (3 :: Int)", 4, "mismatch"),
        ("expected result retains effects", "keep :: Nat ->{Diverge} Nat\n  keep x = x\n  use :: Unit ->{Diverge} Nat\n  use unit = keep 7", 0, ""),
        ("expected result cannot erase effects", "keep :: Nat ->{Diverge} Nat\n  keep x = x\n  bad :: Nat\n  bad = keep 7", 4, "effect"),
        ("Nat Div specialization", "probe :: Nat\n  probe = 7 / 3", 0, ""),
        ("Nat Rem specialization", "probe :: Nat\n  probe = 7 % 3", 0, ""),
        ("Nat Div rejects zero", "bad :: Nat\n  bad = 7 / 0", 4, "refinement predicate is not established"),
        ("Nat Rem rejects zero", "bad :: Nat\n  bad = 7 % 0", 4, "refinement predicate is not established"),
        ("abstract carrier grants no Nat facts", "bad :: forall a. NonZero a -> refined (r > 0) Nat\n  bad d = 0", 4, "refinement predicate is not established"),
        ("abstract carrier identities stay distinct", "bad :: forall a b. NonZero a -> NonZero b\n  bad d = d", 4, "mismatch"),
        ("float carrier remains refused", "bad :: NonZero F64 -> Nat\n  bad d = 0", 4, "NonZero needs a checked mathematical carrier"),
        ("Bool carrier remains refused", "bad :: NonZero Bool -> Nat\n  bad d = 0", 4, "NonZero needs a checked mathematical carrier"),
    ] {
        std::fs::write(dir.join("source.cinch"), format!("module Main where\n  import Algebra.Arithmetic (NonZero, Div, Rem, (/), (%))\n  import Data.Nat (nat-rem)\n  {body}\n  main :: Int\n  main = 0\n")).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "refinement-report.txt");
        assert_eq!(output.status.code(), Some(status), "{name}: {report}\n{}", String::from_utf8_lossy(&output.stderr));
        if status == 0 { assert!(report.is_empty(), "{name}: {report}"); }
        else { assert!(report.contains(diagnostic), "{name}: {report}"); }
        eprintln!("abstract NonZero method case passed: {name}");
    }
    common::cleanup(&producer); common::cleanup(&dir);
}

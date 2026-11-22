//! Numeric reflection follows actual bodies rather than class identities/slots.
mod common;

fn checked_with_nat_method(source: &str, method_body: &str) -> cinch::core::Program {
    let original = include_str!("../prelude/Data/Nat.cinch");
    let method = "(<) x y = lt-i64 (nat-cmp x y) 0";
    assert_eq!(original.matches(method).count(), 1);
    let mut family = cinch::driver::parse_prelude(&[]).unwrap();
    let replacement = cinch::driver::parse_source(&original.replace(method, method_body), "prelude/Data/Nat.cinch", &family.base, &family.exported_fixities).unwrap();
    *family.modules.iter_mut().find(|m| m.name.text == "Data.Nat").unwrap() = replacement.modules.into_iter().next().unwrap();
    let user = cinch::driver::parse_source(source, "numeric-bodies.cinch", &family.base, &family.exported_fixities).unwrap();
    let mut context = cinch::driver::StagingCx::default();
    context.fixities = family.fixities;
    context.fixities.extend(user.fixities);
    cinch::check::check_program_multi(&family.modules, &user.modules, &std::collections::HashMap::new(), &context).unwrap().0
}

#[test]
fn compiled_cinch_reflects_only_actual_numeric_equations() {
    let dir = common::project("numeric_body_semantics", &[
        ("Bootstrap.TotalSemantics.cinch", include_str!("../../bootstrap/Bootstrap.TotalSemantics.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.TotalSemantics.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "numeric-body-semantics").expect("compile body semantic recognizer");
    let source = r#"module Main where
  add :: Nat -> Nat -> Nat
  add a b = a + b
  multiply :: Nat -> Nat -> Nat
  multiply a b = a * b
  subtract :: Int -> Int -> Int
  subtract a b = a - b
  less :: Nat -> Nat -> Bool
  less a b = a < b
  swapped :: Nat -> Nat -> Bool
  swapped a b = b < a
  constant :: Nat -> Nat -> Bool
  constant a b = False
  altered :: Nat -> Nat -> Nat
  altered a b = a + 2 * b
  alias :: Nat -> Nat -> Nat
  alias a b = let same = a in same + b
  forward :: Nat -> Nat -> Nat
  forward a b = add a b
  main :: Int
  main = 0
"#;
    let roots = ["Main.add", "Main.multiply", "Main.subtract", "Main.less", "Main.swapped", "Main.constant", "Main.altered", "Main.alias", "Main.forward"];
    for (label, method, comparison) in [
        ("actual ordinary Nat order", "(<) x y = lt-i64 (nat-cmp x y) 0", "<"),
        ("same defining class and slot but different actual body", "(<) x y = False", "opaque"),
    ] {
        let checked = checked_with_nat_method(source, method);
        std::fs::write(dir.join("bodies.json"), checked.total_bodies.closure(roots).unwrap().to_json()).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "{label}; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        assert_eq!(common::read_output(&dir, "semantics.txt"), format!("Main.add:+\nMain.multiply:*\nMain.subtract:-\nMain.less:{comparison}\nMain.swapped:opaque\nMain.constant:opaque\nMain.altered:opaque\nMain.alias:+\nMain.forward:+\n"), "{label}");
    }
    common::cleanup(&dir);
}

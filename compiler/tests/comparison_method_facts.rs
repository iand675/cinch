#[path = "common/numeric_provider.rs"]
mod numeric_provider;
// Branch facts follow the actual selected implementation, including its
// transitive checked body. A numeric class argument grants no equation.

use cinch::{core::*, driver::StagingCx};

fn check_with_method(module: &str, original: &str, old: &str, replacement: &str, source: &str) -> cinch::Result<Program> {
    check_with_family(cinch::driver::parse_prelude(&[])?, module, original, old, replacement, source, false)
}

fn check_isolated(module: &str, original: &str, old: &str, replacement: &str, source: &str) -> cinch::Result<Program> {
    check_with_family(numeric_provider::family()?, module, original, old, replacement, source, true)
}

fn check_with_family(mut family: cinch::driver::PreludeSources, module: &str, original: &str, old: &str, replacement: &str, source: &str, isolated: bool) -> cinch::Result<Program> {
    assert_eq!(original.matches(old).count(), 1);
    let (method, declarations) = replacement.split_once("\n\n").unwrap_or((replacement, ""));
    let changed_source = format!("{}\n{declarations}\n", original.replace(old, method));
    let changed = cinch::driver::parse_source(&changed_source, "selected-method.cinch", &family.base, &family.exported_fixities)?;
    let changed = changed.modules.into_iter().next().unwrap();
    let destination = family.modules.iter_mut().find(|m| m.name.text == module).expect("defining module");
    if isolated {
        let method_name = old.split_whitespace().next().unwrap().trim_matches(['(', ')']);
        let (instance_name, equation) = changed.decls.iter().find_map(|declaration| {
            let cinch::ast::Decl::Instance(instance) = declaration else { return None };
            instance.methods.iter().find(|method| method.name.text == method_name).map(|method| (instance.name.text.clone(), method.clone()))
        }).expect("changed defining method");
        let instance = destination.decls.iter_mut().find_map(|declaration| match declaration {
            cinch::ast::Decl::Instance(instance) if instance.name.text == instance_name => Some(instance), _ => None,
        }).expect("same selected instance in isolated checked provider");
        *instance.methods.iter_mut().find(|method| method.name.text == method_name).unwrap() = equation;
    } else { *destination = changed; }
    let user = cinch::driver::parse_source(source, "comparison-client.cinch", &family.base, &family.exported_fixities)?;
    let mut context = StagingCx::default();
    context.fixities = family.fixities;
    context.fixities.extend(user.fixities);
    let (program, _, _) = cinch::check::check_program_multi(&family.modules, &user.modules, &std::collections::HashMap::new(), &context)?;
    Ok(program)
}

const NAT: &str = include_str!("../prelude/Data/Nat.cinch");
const LESS: &str = "(<) x y = lt-i64 (nat-cmp x y) 0";
const NAT_CLIENT: &str = "module Main where\n  keep :: (n :: Nat) -> refined (out == n) Nat\n  keep n = if n < 0 then 0 else n\n  main :: Int\n  main = if unrefine (keep 4) == 4 then 0 else 42\n";

fn reject(result: cinch::Result<Program>) {
    let error = result.expect_err("an arbitrary method must not fabricate a numeric branch fact").to_string();
    assert!(error.contains("obligation") || error.contains("refin"), "unexpected refusal: {error}");
}

fn run(mut program: Program) -> i32 {
    // The direct multi-module library API may omit the entry. Use the same
    // mathematical-Int exit conversion as the ordinary driver's adapter.
    if program.main.is_none() {
        let id = GlobalId("Main.comparison-test-exit".into());
        program.fns.push(FnDef { id: id.clone(), params: vec![], body: Core::App(Box::new(Core::Prim(PrimOp::Int(IntOp::ToI64))), vec![Core::Global(GlobalId("Main.main".into()), vec![])]), prim: None, ctor: None, hints: Default::default(), meta: Default::default(), repr: Default::default() });
        program.main = Some(id);
    }
    cinch::driver::run_program(&program, &TargetProfile::native()).unwrap()
}

#[test]
fn changing_the_selected_nat_order_cannot_prove_a_false_result() {
    assert_eq!(run(check_with_method("Data.Nat", NAT, LESS, LESS, NAT_CLIENT).unwrap()), 0);
    reject(check_with_method("Data.Nat", NAT, LESS, "(<) x y = True", NAT_CLIENT));
}

#[test]
fn int_equality_requires_its_selected_body_too() {
    let original = include_str!("../prelude/Data/Int.cinch");
    let equality = "(==) x y = eq-i64 (int-cmp x y) 0";
    let source = "module Main where\n  keep :: (n :: Int) -> refined (out == n) Int\n  keep n = if n == 0 then 0 else n\n  main :: Int\n  main = 0\n";
    check_with_method("Data.Int", original, equality, equality, source).unwrap();
    reject(check_with_method("Data.Int", original, equality, "(==) x y = True", source));
}

#[test]
fn machine_comparisons_retain_their_exact_carrier() {
    let original = include_str!("../prelude/Data/Machine.cinch");
    let less = "(<) x y = lt-u8 x y";
    let source = "module Main where\n  keep :: (n :: U8) -> refined (out == n) U8\n  keep n = if n < 0 then 0 else n\n  main :: Int\n  main = 0\n";
    check_with_method("Data.Machine", original, less, less, source).unwrap();
    reject(check_with_method("Data.Machine", original, less, "(<) x y = True", source));
}

#[test]
fn checked_aliases_lets_and_operand_orientation_preserve_real_semantics() {
    for body in [
        "(<) x y = let c = nat-cmp x y in lt-i64 c 0",
        "(<) x y = gt-i64 0 (nat-cmp x y)",
        "(<) x y = not (ge-i64 (nat-cmp x y) 0)",
        "(<) x y = helper x y\n\n  helper :: Nat -> Nat -> Bool\n  helper a b = let c = nat-cmp a b in lt-i64 c 0",
    ] {
        check_with_method("Data.Nat", NAT, LESS, body, NAT_CLIENT).unwrap_or_else(|e| panic!("{body}: {e}"));
    }
    reject(check_with_method("Data.Nat", NAT, LESS, "(<) x y = lt-i64 (nat-cmp y x) 0", NAT_CLIENT));
}

#[test]
fn a_lexical_numeric_dictionary_is_not_an_arithmetic_axiom() {
    let source = NAT_CLIENT.replace("keep ::", "keep :: using (order :: Ord Nat) =>");
    reject(check_with_method("Data.Nat", NAT, LESS, LESS, &source));
}

#[test]
fn a_recursive_comparison_cannot_manufacture_a_fact() {
    reject(check_with_method("Data.Nat", NAT, LESS, "(<) x y = helper x y\n\n  helper :: Nat -> Nat -> Bool\n  helper a b = helper a b", NAT_CLIENT));
}

#[test]
fn ordinary_lexical_dispatch_requires_no_numeric_certificate() {
    let source = "module Main where\n  plain :: using (order :: Ord Nat) => Nat -> Bool\n  plain n = if n < 0 then True else False\n  main :: Int\n  main = if plain 4 then 42 else 0\n";
    assert_eq!(run(check_with_method("Data.Nat", NAT, LESS, LESS, source).unwrap()), 0);
}

#[test]
fn an_arbitrary_selected_method_can_still_drive_an_ordinary_runtime_branch() {
    let original = include_str!("../prelude/Data/Char.cinch");
    let less = "(<) x y = lt-char x y";
    let source = "module Main where\n  main :: Int\n  main = if 'x' < 'a' then 0 else 42\n";
    assert_eq!(run(check_with_method("Data.Char", original, less, less, source).unwrap()), 42);
    assert_eq!(run(check_with_method("Data.Char", original, less, "(<) x y = True", source).unwrap()), 0);
}

#[test]
fn a_cited_forward_comparison_is_verified_after_its_body_arrives() {
    // Function.Combinators is checked before Data.Nat. Its checked
    // subtraction cites the false arm of Nat equality to prove n >= 1.
    // A different actual later comparison must not retain the earlier
    // proposed equality equation, even if later ordinary branches typecheck.
    let equality = "(==) x y = eq-i64 (nat-cmp x y) 0";
    let result = check_with_method("Data.Nat", NAT, equality, "(==) x y = ge-i64 (nat-cmp x y) 0", "module Main where\n  main :: Int\n  main = 0\n");
    let error = result.expect_err("a forward declaration cannot be a final numeric axiom").to_string();
    assert!(error.contains("actual selected implementation") && error.contains("Data.Nat.Eq_Nat"), "{error}");
}

const ADD: &str = "(+) x y = nat-add x y";
const ALTERED_ADD: &str = "(+) x y = if lt-i64 (nat-cmp x 100) 0 then nat-add x y else x";

#[test]
fn runtime_arithmetic_operands_require_the_actual_selected_equation() {
    let source = "module Main where\n  keep :: (n :: Nat) -> refined (out == n) Nat\n  keep n = if n + 1 == n then 0 else n\n  main :: Int\n  main = if unrefine (keep 100) == 100 then 0 else 42\n";
    assert_eq!(run(check_isolated("Data.Nat", NAT, ADD, ADD, source).unwrap()), 0);
    reject(check_isolated("Data.Nat", NAT, ADD, ALTERED_ADD, source));
}

#[test]
fn annotated_and_nested_runtime_arithmetic_keep_selected_method_authority() {
    for expression in ["((n + 1) :: Nat)", "(n + 1) + 0", "n + (0 + 1)"] {
        let source = format!("module Main where\n  keep :: (n :: Nat) -> refined (out == n) Nat\n  keep n = if {expression} == n then 0 else n\n  main :: Int\n  main = 0\n");
        check_isolated("Data.Nat", NAT, ADD, ADD, &source).unwrap();
        reject(check_isolated("Data.Nat", NAT, ADD, ALTERED_ADD, &source));
    }
}

#[test]
fn dependent_runtime_arguments_do_not_inherit_mathematical_operator_spelling() {
    let source = "module Main where\n  data At (n :: Nat) where\n    At :: At n\n  take :: (n :: Nat) -> At n -> Nat\n  take n _ = n\n  main :: Int\n  main = if take (100 + 1) (At :: At 101) == 101 then 0 else 42\n";
    assert_eq!(run(check_isolated("Data.Nat", NAT, ADD, ADD, source).unwrap()), 0);
    let error = check_isolated("Data.Nat", NAT, ADD, ALTERED_ADD, source).expect_err("a dependent argument needs the actual runtime value").to_string();
    assert!(error.contains("dependent") || error.contains("index"), "{error}");
}

#[test]
fn arbitrary_runtime_addition_still_runs_when_no_mathematical_fact_is_claimed() {
    let source = "module Main where\n  main :: Int\n  main = if (100 :: Nat) + 1 == 100 then 0 else 42\n";
    assert_eq!(run(check_isolated("Data.Nat", NAT, ADD, ADD, source).unwrap()), 42);
    assert_eq!(run(check_isolated("Data.Nat", NAT, ADD, ALTERED_ADD, source).unwrap()), 0);
}

#[test]
fn lexical_arithmetic_dictionaries_cannot_supply_numeric_equations() {
    let source = "module Main where\n  keep :: using (addition :: Add Nat) => (n :: Nat) -> refined (out == n) Nat\n  keep n = if n + 1 == n then 0 else n\n  main :: Int\n  main = 0\n";
    reject(check_isolated("Data.Nat", NAT, ADD, ADD, source));
}

#[test]
fn mathematical_type_operators_keep_their_independent_meaning() {
    let source = "module Main where\n  data At (n :: Nat) where\n    At :: At n\n  same :: At (100 + 1) -> At 101\n  same x = x\n  main :: Int\n  main = case same At of\n    At -> 0\n";
    assert_eq!(run(check_isolated("Data.Nat", NAT, ADD, ALTERED_ADD, source).unwrap()), 0);
}

#[test]
fn named_helpers_do_not_unfold_runtime_operators_as_mathematical_axioms() {
    let source = "module Main where\n  bump :: Nat -> Nat\n  bump n = n + 1\n  proof equation :: Eq (bump 100) 101\n  proof equation = refl\n  main :: Int\n  main = 0\n";
    check_isolated("Data.Nat", NAT, ADD, ADD, source).unwrap();
    let error = check_isolated("Data.Nat", NAT, ADD, ALTERED_ADD, source).expect_err("an altered runtime helper must not prove its old mathematical equation").to_string();
    assert!(error.contains("index mismatch") && error.contains("refl"), "{error}");
}

#[test]
fn contextual_integer_literals_retain_the_actual_runtime_carrier() {
    let original = include_str!("../prelude/Data/Int.cinch");
    let addition = "(+) x y = int-add x y";
    let changed = "(+) x y = if lt-i64 (int-cmp x 100) 0 then int-add x y else x";
    let source = "module Main where\n  keep :: (n :: Int) -> refined (out == n) Int\n  keep n = if 100 + 1 == n then 101 else n\n  main :: Int\n  main = if unrefine (keep 100) == 100 then 0 else 42\n";
    assert_eq!(run(check_isolated("Data.Int", original, addition, addition, source).unwrap()), 0);
    reject(check_isolated("Data.Int", original, addition, changed, source));
}

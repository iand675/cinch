//! Explicit region quantifiers keep pure factories in the runtime-identity namespace.
mod common;

/// A pure constructor factory still stores callbacks tied to the captured cell owner.
const ACTIONS: &str = r#"-- | Store a mixed-effect action without performing it during construction.
module Actions where
  import Data.Mutable.Var
  export Work, prepare, prepare-eta, run, identity, apply-maker, prepare-extra, apply-maker-flat
  -- | An action belongs to exactly one runtime identity.
  data Work r where
    -- | A private callback preserves both I/O and mutation identity.
    Stored :: (Unit ->{IO r, Mut r} Int) -> Work r
  -- | Reading captures the cell's actual owner.
  action :: forall r. Var r Int -> Unit ->{IO r, Mut r} Int
  action cell unit = var-read cell
  -- | State the identity kind even though construction performs no effect.
  prepare :: forall (r :: Region). Var r Int -> Work r
  prepare cell = Stored (action cell)
  -- | An ordinary eta closure preserves the same pure construction contract.
  prepare-eta :: forall (r :: Region). Var r Int -> Work r
  prepare-eta cell = Stored (\unit -> action cell unit)
  -- | A pure identity need not manufacture a type-level region substitute.
  identity :: forall (r :: Region). Work r -> Work r
  identity work = work
  -- | A nested universal factory can select the caller's identity.
  apply-maker :: forall (s :: Region). (forall (r :: Region). Var r Int -> Work r) -> Var s Int -> Work s
  apply-maker maker cell = maker cell
  -- | A factory may also quantify an ordinary ignored value type.
  prepare-extra :: forall (r :: Region) a. Var r Int -> a -> Work r
  prepare-extra cell ignored = prepare cell
  -- | Consecutive nested quantifiers preserve both type and identity namespaces.
  apply-maker-flat :: forall (s :: Region). (forall a. forall (r :: Region). Var r Int -> a -> Work r) -> Var s Int -> Work s
  apply-maker-flat maker cell = maker cell ()
  -- | Execute under the identity stored in the action's type.
  run :: forall r. Work r -> Unit ->{IO r, Mut r} Int
  run work unit = case work of Stored action -> action unit
"#;

/// Copy actual source dependencies without replacing the embedded base family.
fn project(main: &str) -> std::path::PathBuf {
    common::project("explicit_region_quantifiers", &[
        ("Data.Mutable.Var.cinch", include_str!("../prelude/Data/Mutable/Var.cinch")),
        ("Actions.cinch", ACTIONS),
        ("Main.cinch", main),
    ])
}

/// Pure and nested factories preserve real mutable-cell observations at runtime.
#[test]
fn pure_factories_and_nested_region_quantifiers_execute() {
    let dir = project(r#"-- | Observe source-owned action factories.
module Main where
  import Actions
  import Data.Mutable.Var
  -- | Every factory returns the original cell's value.
  main :: Unit ->{IO r, Mut r} Int
  main unit =
    let cell = var-new (41 :: Int) in
    if run (identity (prepare cell)) () == 41
      && run (prepare-eta cell) () == 41
      && run (apply-maker prepare cell) () == 41
      && run (apply-maker-flat prepare-extra cell) () == 41
    then 0 else 1
"#);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

/// Refusals must come from the intended consumer after its dependencies check.
fn reject(declarations: &str) {
    let source = format!("-- | Refuse conflicting region uses.\nmodule Main where\n  import Actions\n{declarations}\n  -- | A valid entrypoint keeps the refusal local to the conflicting declaration.\n  main :: Int\n  main = 0\n");
    let dir = project(&source);
    let error = common::check(&dir, "Main.cinch").expect_err("region constraints must remain enforced");
    assert!(error.contains("Main.cinch"), "dependency failed before intended refusal: {error}");
    assert!(error.contains("region") || error.contains("Region"), "{error}");
    common::cleanup(&dir);
}

/// Distinct explicitly quantified identities cannot be exchanged by a pure function.
#[test]
fn explicit_regions_cannot_be_relabelled() {
    reject("  -- | This declaration intentionally conflicts with the runtime identity kind.\n  bad :: forall (r :: Region) (s :: Region). Work r -> Work s\n  bad value = value");
}

/// A runtime identity is not a mathematical index value.
#[test]
fn explicit_regions_cannot_be_numeric_indices() {
    reject("  -- | An indexed family accepts mathematical naturals.\n  data Indexed (n :: Nat) where\n    -- | The only test index is zero.\n    Item :: Indexed 0\n  -- | A runtime identity cannot supply a natural index.\n  bad :: forall (r :: Region). Indexed r -> Int\n  bad value = 0");
}

/// Explicit row and region namespaces must remain disjoint.
#[test]
fn explicit_regions_cannot_be_row_tails() {
    reject("  -- | This declaration intentionally conflicts with the runtime identity kind.\n  bad :: forall (r :: Region). (Unit ->{|r} Int) -> Int\n  bad callback = 0");
}

/// Nested region annotations cannot disguise a numeric index.
#[test]
fn nested_explicit_regions_cannot_be_numeric_indices() {
    reject("  -- | An indexed family accepts mathematical naturals.\n  data Indexed (n :: Nat) where\n    -- | The only test index is zero.\n    Item :: Indexed 0\n  -- | A runtime identity cannot supply a natural index.\n  bad :: (forall (r :: Region). Indexed r -> Int) -> Int\n  bad callback = 0");
}

/// A caller cannot turn one concrete identity into an action valid under every identity.
#[test]
fn nested_universal_region_cannot_capture_a_specific_identity() {
    reject("  -- | This declaration intentionally conflicts with the runtime identity kind.\n  bad :: forall (s :: Region). Work s -> (forall (r :: Region). Work r)\n  bad value = value");
}

/// Runtime identities cannot stand for inhabited value types.
#[test]
fn explicit_regions_cannot_be_value_types() {
    reject("  -- | An identity kind cannot supply a value domain.\n  bad :: forall (r :: Region). r -> r\n  bad value = value");
}

/// Nested runtime identities cannot become effect row tails.
#[test]
fn nested_explicit_regions_cannot_be_row_tails() {
    reject("  -- | An identity kind cannot supply a callback row.\n  bad :: (forall (r :: Region). (Unit ->{|r} Int) -> Int) -> Int\n  bad callback = 0");
}

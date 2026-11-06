//! Defining constructor contracts retain the source proof slot and zero payload.
mod common;

#[test]
fn decision_constructors_check_erasure_metadata_and_source_evidence() {
    let dir = common::project("source_constructors", &[
        ("Bootstrap.Parse.cinch", include_str!("../../bootstrap/Bootstrap.Parse.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Infer.cinch", include_str!("../../bootstrap/Bootstrap.Infer.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceConstructors.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-constructors").expect("build source constructor checker");
    let yes = "wrap :: forall (p :: Prop). (h :: p) -> Dec p\n  wrap h = Yes h";
    let cases = [
        ("projection scope infers a unique visible declaring record", "record User where\n    age :: Nat\n  apply :: forall a b. (a -> b) -> a -> b\n  apply f value = f value\n  user :: User\n  user = User { age = 36 }\n  age :: Nat\n  age = apply (\\value -> value.age) user", true),
        ("projection scope retains the original ambiguous lambda refusal", "record Number where\n    value :: Nat\n  record Flag where\n    value :: Bool\n  apply :: forall a b. (a -> b) -> a -> b\n  apply f value = f value\n  flag :: Flag\n  flag = Flag { value = True }\n  read :: Bool\n  read = apply (\\item -> item.value) flag", false),
        ("projection scope accepts an annotated receiver among identical labels", "record Number where\n    value :: Nat\n  record Flag where\n    value :: Bool\n  apply :: forall a b. (a -> b) -> a -> b\n  apply f value = f value\n  flag :: Flag\n  flag = Flag { value = True }\n  read :: Bool\n  read = apply (\\item -> (item :: Flag).value) flag", true),
        ("projection scope does not invent a record for a rigid generic receiver", "record User where\n    age :: Nat\n  bad :: forall a. a -> Nat\n  bad value = value.age", false),
        ("projection scope ignores a private unselected record with the same label", "module Secret where\n  export marker\n  record Hidden where\n    age :: Nat\n  marker :: Int\n  marker = 0\nmodule Main where\n  import Secret (marker)\n  record User where\n    age :: Nat\n  apply :: forall a b. (a -> b) -> a -> b\n  apply f value = f value\n  user :: User\n  user = User { age = 36 }\n  age :: Nat\n  age = apply (\\value -> value.age) user\n  main :: Int\n  main = 0\n", true),
        ("projection scope admits an explicitly selected record alias", "module A where\n  export Person, sample\n  record Person where\n    age :: Nat\n  sample :: Person\n  sample = Person { age = 36 }\nmodule Main where\n  import A (Person as User, sample as user)\n  apply :: forall a b. (a -> b) -> a -> b\n  apply f value = f value\n  age :: Nat\n  age = apply (\\value -> value.age) user\n  main :: Int\n  main = 0\n", true),
        ("projection scope cannot infer a record omitted by selective import", "module A where\n  export Person, sample\n  record Person where\n    age :: Nat\n  sample :: Person\n  sample = Person { age = 36 }\nmodule Main where\n  import A (sample)\n  apply :: forall a b. (a -> b) -> a -> b\n  apply f value = f value\n  age :: Nat\n  age = apply (\\value -> value.age) sample\n  main :: Int\n  main = 0\n", false),
        ("projection scope preserves a known receiver without importing its type", "module A where\n  export sample\n  record Person where\n    age :: Nat\n  sample :: Person\n  sample = Person { age = 36 }\nmodule Main where\n  import A (sample)\n  age :: Nat\n  age = sample.age\n  main :: Int\n  main = 0\n", true),
        ("indexed coverage eliminates the different closed index", "data Signal (channels :: Nat) where\n    Mono :: Int -> Signal 1\n    Stereo :: Int -> Int -> Signal 2\n  observe :: Signal 1 -> Int\n  observe signal = case signal of Mono x -> x", true),
        ("indexed coverage selects the second closed index", "data Signal (channels :: Nat) where\n    Mono :: Int -> Signal 1\n    Stereo :: Int -> Int -> Signal 2\n  observe :: Signal 2 -> Int\n  observe signal = case signal of Stereo x y -> x + y", true),
        ("indexed coverage proves an empty closed index", "data Signal (channels :: Nat) where\n    Mono :: Int -> Signal 1\n    Stereo :: Int -> Int -> Signal 2\n  absurd :: Signal 0 -> Int\n  absurd signal = case signal of {}", true),
        ("indexed coverage does not drop a feasible alternative", "data Signal (channels :: Nat) where\n    Mono :: Int -> Signal 1\n    Other :: Int -> Signal 1\n  observe :: Signal 1 -> Int\n  observe signal = case signal of Mono x -> x", false),
        ("indexed coverage does not treat a symbolic mismatch as impossible", "data Signal (channels :: Nat) where\n    Mono :: Int -> Signal 1\n    Stereo :: Int -> Int -> Signal 2\n  observe :: forall (n :: Nat). Signal n -> Int\n  observe signal = case signal of {}", false),
        ("indexed coverage respects normalized closed arithmetic", "data Signal (channels :: Nat) where\n    Mono :: Int -> Signal (1 + 0)\n    Stereo :: Int -> Int -> Signal (1 + 1)\n  observe :: Signal 1 -> Int\n  observe signal = case signal of Mono x -> x", true),

        ("record declared Nat field controls literal carrier", "record User where\n    age :: Nat\n    name :: String\n  user :: User\n  user = User { name = \"Ada\", age = 36 }", true),
        ("record expected generic result controls literal carrier", "record Box a where\n    value :: a\n  box :: Box Nat\n  box = Box { value = 36 }", true),
        ("record unsigned narrow field permits its upper bound", "record Octet where\n    value :: U8\n  octet :: Octet\n  octet = Octet { value = 255 }", true),
        ("record unsigned narrow field rejects overflow", "record Octet where\n    value :: U8\n  octet :: Octet\n  octet = Octet { value = 256 }", false),
        ("record Nat field rejects a negative literal", "record Box where\n    value :: Nat\n  box :: Box\n  box = Box { value = -1 }", false),
        ("record nested Nat initializer retains field type", "record Inner where\n    value :: Nat\n  record Outer where\n    inner :: Inner\n  outer :: Outer\n  outer = Outer { inner = Inner { value = 42 } }", true),
        ("record function initializer receives its declared domain and result", "record Callback where\n    run :: Nat -> Nat\n  callback :: Callback\n  callback = Callback { run = \\value -> 1 }", true),
        ("constructor result keeps ordinary type parameters", "data Box a where\n    Box :: a -> Box a", true),
        ("constructor result cannot fix a type parameter", "data Box a where\n    Box :: a -> Box Bool", false),
        ("constructor result cannot permute type parameters", "data Pair a b where\n    Pair :: a -> b -> Pair b a", false),
        ("constructor result keeps proposition parameters", "data Witness (p :: Prop) where\n    Witness :: Witness p", true),
        ("constructor result cannot fix a proposition parameter", "data Witness (p :: Prop) where\n    Witness :: Witness False", false),

        ("record declaration retains generic fields", "record Box a where\n    value :: a\n    label :: String", true),
        ("record construction follows labels rather than initializer order", "record Box where\n    count :: Int\n    label :: String\n  make :: Box\n  make = Box { label = \"owned label\", count = 42 }", true),
        ("record generic field projection", "record Box a where\n    value :: a\n    label :: String\n  payload :: forall a. Box a -> a\n  payload box = box.value", true),
        ("record incomplete pattern is rejected", "record Box a where\n    value :: a\n    label :: String\n  payload :: forall a. Box a -> a\n  payload (Box { value = x }) = x", false),
        ("record explicit wildcard discards owned field", "record Box a where\n    value :: a\n    label :: String\n  payload :: forall a. Box a -> a\n  payload (Box { label = _, value = x }) = x", true),
        ("record nested construction and patterns", "record Inner where\n    value :: Int\n  record Outer where\n    value :: Inner\n  make :: Outer\n  make = Outer { value = Inner { value = 42 } }\n  payload :: Outer -> Int\n  payload (Outer { value = Inner { value = n } }) = n", true),
        ("record field names are selected by receiver type", "record Number where\n    value :: Int\n  record Flag where\n    value :: Bool\n  read-number :: Number -> Int\n  read-number value = value.value\n  read-flag :: Flag -> Bool\n  read-flag value = value.value", true),
        ("record function field", "record Callback where\n    run :: Int -> Int\n  call :: Callback -> Int\n  call callback = callback.run 42", true),
        ("record missing construction field", "record Box where\n    value :: Int\n    label :: String\n  bad :: Box\n  bad = Box { value = 42 }", false),
        ("record duplicate construction field", "record Box where\n    value :: Int\n  bad :: Box\n  bad = Box { value = 1, value = 2 }", false),
        ("record unknown construction field", "record Box where\n    value :: Int\n  bad :: Box\n  bad = Box { unknown = 42 }", false),
        ("record duplicate declaration field", "record Box where\n    value :: Int\n    value :: Bool", false),
        ("record wrong field type", "record Box where\n    value :: Int\n  bad :: Box\n  bad = Box { value = True }", false),
        ("record projection keeps its actual field type", "record Box where\n    value :: Bool\n  bad :: Box -> Int\n  bad box = box.value", false),
        ("record unknown projection", "record Box where\n    value :: Int\n  bad :: Box -> Int\n  bad box = box.unknown", false),
        ("record unknown pattern field", "record Box where\n    value :: Int\n  bad :: Box -> Int\n  bad (Box { unknown = n }) = n", false),
        ("record duplicate pattern field", "record Box where\n    value :: Int\n  bad :: Box -> Int\n  bad (Box { value = a, value = b }) = a", false),
        ("nullary constructor does not capture following field", "data Leaf where\n    End :: Leaf\n    Item :: Int -> Leaf\n  data Box where\n    Box :: Leaf -> Int -> Box\n  read :: Box -> Int\n  read (Box End n) = n\n  read (Box (Item value) n) = value + n", true),
        ("two adjacent nullary constructor fields", "data Leaf where\n    End :: Leaf\n  data Pair where\n    Pair :: Leaf -> Leaf -> Pair\n  read :: Pair -> Int\n  read (Pair End End) = 0", true),
        ("nested fields require parentheses", "data Leaf where\n    Item :: Int -> Leaf\n  data Box where\n    Box :: Leaf -> Int -> Box\n  bad :: Box -> Int\n  bad (Box Item value n) = n", false),
        ("equation constructor parameter does not capture next argument", "data Leaf where\n    End :: Leaf\n    Item :: Int -> Leaf\n  read :: Leaf -> Int -> Int\n  read End n = n\n  read (Item value) n = value + n", true),
        ("lambda constructor parameter does not capture next argument", "data Leaf where\n    End :: Leaf\n  read :: Leaf -> Int -> Int\n  read leaf n = (\\End value -> value) leaf n", true),
        ("existential constructor declaration", "data Exists where\n    Pack :: forall a. a -> Exists", true),
        ("existential constructor introduction", "data Exists where\n    Pack :: forall a. a -> Exists\n  make :: Int -> Exists\n  make x = Pack x", true),
        ("family and constructor binders retain distinct kinds", "data Tagged a where\n    Pack :: forall (n :: Nat). a -> Tagged a", true),
        ("fixed indexed constructor result", "data Vec (n :: Nat) a where\n    VNil :: Vec 0 a", true),
        ("computed indexed constructor result", "data Vec (n :: Nat) a where\n    VCons :: forall (k :: Nat). a -> Vec k a -> Vec (k + 1) a", true),
        ("constructor cannot return another family", "data Other where\n    End :: Other\n  data Box a where\n    Wrong :: a -> Other", false),
        ("constructor result must be saturated", "data Box a where\n    Wrong :: Int -> Box", false),
        ("constructor index kind remains checked", "data Vec (n :: Nat) a where\n    Wrong :: Vec Int Int", false),
        ("hidden constructor payload cannot escape as Int", "data Exists where\n    Pack :: forall a. a -> Exists\n  bad :: Exists -> Int\n  bad (Pack x) = x", false),
        ("hidden constructor payload cannot escape as Bool", "data Exists where\n    Pack :: forall a. a -> Exists\n  bad :: Exists -> Bool\n  bad value = case value of Pack x -> x", false),
        ("positive proof constructor", yes, true),
        ("negative proof constructor", "reject :: forall (p :: Prop). (h :: Not p) -> Dec p\n  reject h = No h", true),
        ("both decision alternatives", "extract :: forall (p :: Prop). Dec p -> Int\n  extract value = case value of Yes h -> 1; No h -> 0", true),
        ("vacuous assumption remains scoped", "wrap :: (h :: False) -> Dec False\n  wrap h = Yes h", true),
        ("ordinary source constructor may share spelling", "data Number where\n    Yes :: Int -> Number\n  extract :: Number -> Int\n  extract value = case value of Yes n -> n", true),
        ("literal cannot be positive evidence", "bad :: Dec False\n  bad = Yes 1", false),
        ("positive evidence is not negative evidence", "bad :: forall (p :: Prop). (h :: p) -> Dec p\n  bad h = No h", false),
        ("negative evidence is not positive evidence", "bad :: forall (p :: Prop). (h :: Not p) -> Dec p\n  bad h = Yes h", false),
        ("different propositions remain distinct", "bad :: forall (p :: Prop) (q :: Prop). (h :: p) -> Dec q\n  bad h = Yes h", false),
        ("decision must cover both alternatives", "extract :: forall (p :: Prop). Dec p -> Int\n  extract value = case value of Yes h -> 1", false),
        ("evidence cannot be inspected", "bad :: forall (p :: Prop). Dec p -> Int\n  bad value = case value of Yes True -> 1; No _ -> 0", false),
        ("proof binder is not a runtime result", "bad :: forall (p :: Prop). Dec p -> Int\n  bad value = case value of Yes h -> h; No _ -> 0", false),
    ];
    for (name, body, accepted) in cases {
        let source = if body.starts_with("module ") { body.to_string() } else { format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n") };
        if name.starts_with("record ") || name.starts_with("constructor result ") || name.starts_with("indexed coverage ") || name.starts_with("projection scope ") {
            let oracle = cinch::driver::check_sources(&source, "native-record-source-oracle.cinch");
            assert_eq!(oracle.is_ok(), accepted, "Native record oracle: {name}\n{oracle:?}");
        }
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        std::fs::write(dir.join("mutation.txt"), "").unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-constructors-report.txt");
        assert_eq!(output.status.code(), Some(if accepted {0} else {4}), "case {name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    let source = format!("module Main where\n  {yes}\n  main :: Int\n  main = 0\n");
    std::fs::write(dir.join("source.cinch"), &source).unwrap();
    for field in ["name", "family", "parameter_kind", "runtime_abi", "tag", "arity", "runtime_arity", "scheme", "erased"] {
        std::fs::write(dir.join("mutation.txt"), field).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-constructors-report.txt");
        assert_eq!(output.status.code(), Some(4), "mutated {field}; project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        assert!(report.contains("kernel proof/erasure contract"), "mutated {field}: {report}");
    }
    common::cleanup(&dir);
}

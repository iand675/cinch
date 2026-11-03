//! Runtime-fed Cinch source becomes typed Core and shared-runtime LLVM.
//! Bignums, tuples and source ADTs run with reclamation and leak checks enabled.
mod common;
use std::process::Command;

#[test]
fn typed_runtime_numbers_preserve_exact_values_and_reclaim_owned_words() {
    let dir = common::project("runtime_ir", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../bootstrap/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RuntimeIr.cinch")),
    ]);
    let producer = common::project("runtime_classes", &[
        ("RuntimeClasses.cinch", r#"module RuntimeClasses where
  export Choose, choose, Ordered, z-first, a-last, Value, value, Echo, echo, Ops, first, second, Maker, bump, make, Use, use, Shared, duplicate, stored, observe, escape-value
  signature Choose a where
    choose :: a -> Int
  signature Ordered a where
    z-first :: a -> a
    a-last :: a -> a
  signature Value a where
    value :: a
  signature Echo a where
    echo :: a -> a
  signature Ops a where
    first :: a -> a
    second :: a -> a
  signature Maker a where
    bump :: a -> a
    make :: a -> a -> a
  signature Use a where
    use :: using (extra :: Echo Int) => a -> Int
  signature Shared a where
    duplicate :: a
    stored :: a
    observe :: Unit -> a
    escape-value :: Unit -> Unit -> a
"#),
        ("Main.cinch", "module Main where\n  import RuntimeClasses\n  main :: Int\n  main = 0\n"),
    ]);
    // These are kernel/backend tests: produce genuine checked class
    // contracts against an empty Prelude, keeping executable base methods
    // outside this fixture's scope. The full bootstrap uses the real base.
    let prelude_source = "module Prelude where\n";
    let class_source = std::fs::read_to_string(producer.join("RuntimeClasses.cinch")).unwrap();
    let main_source = std::fs::read_to_string(producer.join("Main.cinch")).unwrap();
    let fixities = Default::default();
    let tables = std::collections::HashMap::new();
    let prelude = cinch::driver::parse_source(prelude_source, "Prelude.cinch", &fixities, &tables).unwrap();
    let classes = cinch::driver::parse_source(&class_source, "RuntimeClasses.cinch", &fixities, &tables).unwrap();
    let entry = cinch::driver::parse_source(&main_source, "Main.cinch", &fixities, &tables).unwrap();
    let modules = [classes.modules, entry.modules].concat();
    let hashes = [("Prelude".into(), cinch::interface::content_hash(prelude_source)),
        ("RuntimeClasses".into(), cinch::interface::content_hash(&class_source)),
        ("Main".into(), cinch::interface::content_hash(&main_source))].into_iter().collect();
    let (_, checked_interfaces, _, _) = cinch::check::check_program_multi_full(
        &prelude.modules, &modules, &hashes, &cinch::check::StagingCx::default(), &[], &cinch::core::TargetProfile::native(),
    ).expect("produce the actual declaring class contracts");
    std::fs::create_dir_all(producer.join("build")).unwrap();
    for interface in checked_interfaces {
        std::fs::write(producer.join(format!("build/{}.cinchif", interface.module)), interface.to_json()).unwrap();
    }
    let mut interfaces = Vec::new();
    for entry in std::fs::read_dir(producer.join("build")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let interface = cinch::interface::Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if interface.module != "Main" {
                interfaces.push(format!("[{},{},{}]", quote(&interface.module), quote(&interface.iface_hash()), quote(&path.display().to_string())));
            }
        }
    }
    interfaces.sort();
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", interfaces.join(","))).unwrap();
    let compiler = common::build(&dir, "Main.cinch", "runtime-ir").expect("build the Cinch runtime emitter");
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    assert!(runtime.is_file(), "the Native-built emitter must have built its shared runtime");
    let tag64 = format!("  data Tag where\n{}  main :: Int\n  main = case C63 of\n    C63 -> 137\n    _ -> 1\n",
        (0..64).map(|i| format!("    C{i} :: Tag\n")).collect::<String>());
    let cases = [
        ("huge", "  main :: Int\n  main = if 340282366920938463463374607431768211456 + 7 == 340282366920938463463374607431768211463 then 37 else 1\n", 37),
        ("alias", "  main :: Int\n  main = let x = 9223372036854775808 in let y = x * x in if y - x * x == 0 then 39 else 1\n", 39),
        ("call", "  identity :: Int -> Int\n  identity x = x\n  main :: Int\n  main = if identity 340282366920938463463374607431768211456 == 340282366920938463463374607431768211456 then 41 else 1\n", 41),
        ("branch", "  pick :: Bool -> Int -> Int -> Int\n  pick flag x y = if flag then x else y\n  main :: Int\n  main = if pick False 9223372036854775808 18446744073709551616 == 18446744073709551616 then 43 else 1\n", 43),
        ("recursive", "  sum :: Int -> Int\n  sum n = if n <= 0 then 9223372036854775808 else n + sum (n - 1)\n  main :: Int\n  main = if sum 20 == 9223372036854776018 then 47 else 1\n", 47),
        ("negative", "  main :: Int\n  main = let x = -9223372036854775809 in if -x == 9223372036854775809 then 53 else 1\n", 53),
        ("natural", "  previous :: Nat -> Nat\n  previous n = if n == 0 then 0 else n - 1\n  main :: Int\n  main = if previous 340282366920938463463374607431768211456 == 340282366920938463463374607431768211455 then 59 else 1\n", 59),
        ("wildcard", "  pick :: Int -> Int -> Int\n  pick _ x = x\n  main :: Int\n  main = if pick 18446744073709551616 9223372036854775808 == 9223372036854775808 then 61 else 1\n", 61),
        ("tuple-field", "  first :: (Int, Int) -> Int\n  first (x, y) = x\n  main :: Int\n  main = if first (18446744073709551616, 9223372036854775808) == 18446744073709551616 then 67 else 1\n", 67),
        ("tuple-case", "  main :: Int\n  main = let pair = (9223372036854775808, 9223372036854775808) in case pair of\n    (x, y) -> if x == y then 71 else 1\n", 71),
        ("nested-tuple", "  pick :: (Bool, (Int, Nat)) -> Int\n  pick (flag, (x, y)) = if flag then x else 0\n  main :: Int\n  main = if pick (True, (18446744073709551616, 340282366920938463463374607431768211456)) == 18446744073709551616 then 73 else 1\n", 73),
        ("dropped-tuple", "  ignore :: (Int, (Nat, Int)) -> Int\n  ignore _ = 79\n  main :: Int\n  main = ignore (18446744073709551616, (340282366920938463463374607431768211456, -9223372036854775809))\n", 79),
        ("full-inline-map", "  ignore :: (Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int) -> Int\n  ignore _ = 83\n  main :: Int\n  main = ignore (9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808, 9223372036854775808)\n", 83),
        ("nullary-constructors", "  data Colour where\n    Red :: Colour\n    Green :: Colour\n    Blue :: Colour\n  main :: Int\n  main = case Blue of\n    Red -> 1\n    Green -> 2\n    Blue -> 89\n", 89),
        ("recursive-tree", "  data Tree where\n    Leaf :: Int -> Tree\n    Node :: Tree -> Tree -> Tree\n  total :: Tree -> Int\n  total (Leaf n) = n\n  total (Node left right) = total left + total right\n  main :: Int\n  main = if total (Node (Leaf 18446744073709551616) (Leaf 9223372036854775808)) == 27670116110564327424 then 97 else 1\n", 97),
        ("failed-nested-pattern", "  data Tree where\n    Leaf :: Int -> Tree\n    Node :: Tree -> Tree -> Tree\n  main :: Int\n  main = case Node (Leaf 340282366920938463463374607431768211457) (Leaf 9223372036854775808) of\n    Node (Leaf 340282366920938463463374607431768211456) (Leaf n) -> 1\n    Node (Leaf x) (Leaf y) -> if x > y then 101 else 2\n    _ -> 3\n", 101),
        ("shared-constructor-root", "  data Tree where\n    Leaf :: Int -> Tree\n    Node :: Tree -> Tree -> Tree\n  total :: Tree -> Int\n  total (Leaf n) = n\n  total (Node left right) = total left + total right\n  main :: Int\n  main = let leaf = Leaf 18446744073709551616 in let tree = Node leaf leaf in if total tree == 36893488147419103232 then 103 else 1\n", 103),
        ("raw-bool-patterns", "  choose :: Bool -> Int -> Int\n  choose True n = n\n  choose False n = 0\n  main :: Int\n  main = if choose False 18446744073709551616 == 0 && choose True 18446744073709551616 == 18446744073709551616 then 107 else 1\n", 107),
        ("numeric-equations", "  classify :: Int -> Int\n  classify 0 = 2\n  classify n = 3\n  main :: Int\n  main = if classify 0 == 2 && classify 340282366920938463463374607431768211456 == 3 then 109 else 1\n", 109),
        ("nested-case-guards", "  data Box where\n    Box :: Int -> Box\n  get :: Box -> Int\n  get (Box n) = case n of\n    value | value < 0 -> 0\n    value -> value\n  main :: Int\n  main = if get (Box (-18446744073709551616)) == 0 && get (Box 18446744073709551616) == 18446744073709551616 then 113 else 1\n", 113),
        ("failed-case-guard", "  data Box where\n    Box :: Int -> Box\n  main :: Int\n  main = case Box 18446744073709551616 of\n    Box n | n < 0 -> 1\n    Box n -> if n == 18446744073709551616 then 127 else 2\n", 127),
        ("natural-literal-pattern", "  classify :: Nat -> Int\n  classify 340282366920938463463374607431768211456 = 131\n  classify _ = 1\n  main :: Int\n  main = classify 340282366920938463463374607431768211456\n", 131),
        ("last-regular-tag", tag64.as_str(), 137),
        ("generic-identity-classes", "  data Token where\n    Token :: Int -> Token\n  identity :: forall a. a -> a\n  identity x = x\n  readToken :: Token -> Int\n  readToken (Token n) = n\n  main :: Int\n  main = if identity True && identity (readToken (identity (Token 18446744073709551616))) == 18446744073709551616 then 151 else 1\n", 151),
        ("generic-shared-tobj", "  identity :: forall a. a -> a\n  identity x = x\n  keepNat :: Nat -> Nat\n  keepNat n = identity n\n  main :: Int\n  main = if keepNat 340282366920938463463374607431768211456 == 340282366920938463463374607431768211456 && identity (-18446744073709551616) == -18446744073709551616 then 157 else 1\n", 157),
        ("generic-shared-obj", "  data LeftBox where\n    LeftBox :: Int -> LeftBox\n  data RightBox where\n    RightBox :: Bool -> RightBox\n  identity :: forall a. a -> a\n  identity x = x\n  main :: Int\n  main = case identity (LeftBox 18446744073709551616) of LeftBox n -> case identity (RightBox True) of RightBox flag -> if flag && n == 18446744073709551616 then 163 else 1\n", 163),
        ("generic-tuple-fields", "  duplicate :: forall a. a -> (a, a)\n  duplicate x = (x, x)\n  main :: Int\n  main = case duplicate 18446744073709551616 of (x, y) -> case duplicate True of (a, b) -> if a && b && x == y then 167 else 1\n", 167),
        ("polymorphic-recursion", "  grow :: forall a. Int -> a -> Int\n  grow n value = if n == 0 then 173 else grow (n - 1) (value, value)\n  main :: Int\n  main = grow 8 True\n", 173),
        ("class-swapping-recursion", "  rotate :: forall a b. Int -> a -> b -> Int\n  rotate n first second = if n == 0 then 179 else rotate (n - 1) second first\n  main :: Int\n  main = rotate 11 True (18446744073709551616 :: Int)\n", 179),
        ("generic-data-classes", "  data Wrap a where\n    Wrap :: a -> Wrap a\n  unwrap :: forall a. Wrap a -> a\n  unwrap (Wrap value) = value\n  main :: Int\n  main = if unwrap (Wrap True) && unwrap (Wrap 18446744073709551616) == 18446744073709551616 then 181 else 1\n", 181),
        ("generic-recursive-data", "  data Seq a where\n    End :: Seq a\n    Link :: a -> Seq a -> Seq a\n  size :: forall a. Seq a -> Int\n  size End = 0\n  size (Link value rest) = 1 + size rest\n  main :: Int\n  main = if size (Link (18446744073709551616 :: Int) (Link 340282366920938463463374607431768211456 End)) == 2 && size (Link True End) == 1 then 191 else 1\n", 191),
        ("generic-data-growing-recursion", "  data Wrap a where\n    Wrap :: a -> Wrap a\n  grow :: forall a. Int -> a -> Int\n  grow n value = if n == 0 then 193 else grow (n - 1) (Wrap value)\n  main :: Int\n  main = grow 7 (18446744073709551616 :: Int)\n", 193),
        ("generic-owned-result", "  data Wrap a where\n    Wrap :: a -> Wrap a\n  duplicate :: forall a. a -> Wrap (a, a)\n  duplicate value = Wrap (value, value)\n  main :: Int\n  main = case duplicate 340282366920938463463374607431768211456 of Wrap (x, y) -> if x == y then 197 else 1\n", 197),
        ("lambda-argument", "  apply :: (Int -> Int) -> Int -> Int\n  apply f x = f x\n  main :: Int\n  main = if apply (\\x -> x + 1) 18446744073709551616 == 18446744073709551617 then 199 else 1\n", 199),
        ("escaping-capture", "  make :: Int -> Int -> Int\n  make x = \\y -> x + y\n  main :: Int\n  main = if make 18446744073709551616 2 == 18446744073709551618 then 201 else 1\n", 201),
        ("shared-closure", "  make :: Int -> Int -> Int\n  make x = \\y -> x + y\n  main :: Int\n  main = let f = make 18446744073709551616 in if f 2 == 18446744073709551618 && f 3 == 18446744073709551619 then 203 else 1\n", 203),
        ("global-function-value", "  apply :: (Int -> Int) -> Int -> Int\n  apply f x = f x\n  bump :: Int -> Int\n  bump x = x + 1\n  main :: Int\n  main = if apply bump 18446744073709551616 == 18446744073709551617 then 205 else 1\n", 205),
        ("partial-function", "  add :: Int -> Int -> Int\n  add x y = x + y\n  main :: Int\n  main = let f = add 18446744073709551616 in if f 2 == 18446744073709551618 then 207 else 1\n", 207),
        ("dynamic-partial-application", "  add :: Int -> Int -> Int\n  add x y = x + y\n  twice :: (Int -> Int -> Int) -> Int -> Int -> Int\n  twice f x y = let partial = f x in partial y\n  main :: Int\n  main = if twice add 18446744073709551616 2 == 18446744073709551618 then 209 else 1\n", 209),
        ("partial-constructor", "  data Pair a where\n    Pair :: a -> a -> Pair a\n  makePair :: Int -> Int -> Pair Int\n  makePair x = Pair x\n  main :: Int\n  main = let huge = 18446744073709551616 in case makePair huge huge of Pair x y -> if x == y then 211 else 1\n", 211),
        ("nested-escaping-closures", "  make :: Int -> Int -> Int -> Int\n  make x = \\y -> \\z -> x + y + z\n  main :: Int\n  main = if make 18446744073709551616 2 3 == 18446744073709551621 then 213 else 1\n", 213),
        ("unused-capture", "  ignore :: Int -> Int -> Int\n  ignore outer = \\inner -> inner\n  main :: Int\n  main = if ignore 18446744073709551616 9 == 9 then 215 else 1\n", 215),
        ("shadowed-capture", "  make :: Int -> Int -> Int -> Int\n  make x = \\x -> let y = x + 1 in \\z -> y + z\n  main :: Int\n  main = if make 18446744073709551616 2 3 == 6 then 217 else 1\n", 217),
        ("generic-capture-classes", "  constant :: forall a b. a -> b -> a\n  constant x = \\y -> x\n  main :: Int\n  main = if constant True (18446744073709551616 :: Int) && constant 18446744073709551616 True == 18446744073709551616 then 219 else 1\n", 219),
        ("higher-order-generic-map", "  data Seq a where\n    End :: Seq a\n    Link :: a -> Seq a -> Seq a\n  mapSeq :: forall a b. (a -> b) -> Seq a -> Seq b\n  mapSeq f End = End\n  mapSeq f (Link x rest) = Link (f x) (mapSeq f rest)\n  total :: Seq Int -> Int\n  total End = 0\n  total (Link x rest) = x + total rest\n  main :: Int\n  main = if total (mapSeq (\\x -> x + 18446744073709551616) (Link (2 :: Int) (Link 3 End))) == 36893488147419103237 then 221 else 1\n", 221),
        ("borrowed-field-capture", "  data Box where\n    Box :: Int -> Box\n  get :: Box -> Int -> Int\n  get box = case box of Box n -> \\x -> n + x\n  main :: Int\n  main = if get (Box 18446744073709551616) 3 == 18446744073709551619 then 223 else 1\n", 223),
        ("closure-constructor-field", "  data Action where\n    Action :: (Int -> Int) -> Action\n  run :: Action -> Int -> Int\n  run (Action f) x = f x\n  main :: Int\n  main = if run (Action (\\x -> x + 18446744073709551616)) 7 == 18446744073709551623 then 225 else 1\n", 225),
        ("branch-returned-closure", "  pick :: Bool -> Int -> Int\n  pick flag = if flag then (\\x -> x + 18446744073709551616) else (\\x -> x - 1)\n  main :: Int\n  main = if pick True 2 == 18446744073709551618 && pick False 2 == 1 then 227 else 1\n", 227),
        ("generic-object-capture", "  data Box where\n    Box :: Int -> Box\n  constant :: forall a b. a -> b -> a\n  constant x = \\y -> x\n  main :: Int\n  main = case constant (Box 18446744073709551616) False of Box x -> if x == 18446744073709551616 then 229 else 1\n", 229),
        ("lambda-tuple-pattern", "  run :: (Int, Int) -> Int\n  run = \\(x, y) -> x + y\n  main :: Int\n  main = if run (18446744073709551616, 2) == 18446744073709551618 then 231 else 1\n", 231),
        ("function-captures", "  compose :: (Int -> Int) -> (Int -> Int) -> Int -> Int\n  compose f g = \\x -> f (g x)\n  bump :: Int -> Int\n  bump x = x + 1\n  main :: Int\n  main = if compose bump bump 18446744073709551616 == 18446744073709551618 then 233 else 1\n", 233),
        ("selected-dictionary", "  import RuntimeClasses\n  instance First :: Choose Int where\n    choose x = x + 1\n  instance Second :: Choose Int where\n    choose x = x + 2\n  run :: using (chosen :: Choose Int) => Int -> Int\n  run x = choose x\n  main :: Int\n  main = if (with (d = First) in run 18446744073709551616) == 18446744073709551617 && (with (d = Second) in run 18446744073709551616) == 18446744073709551618 then 101 else 1\n", 101),
        ("dictionary-slot-order", "  import RuntimeClasses\n  implicit instance OpsInt :: Ordered Int where\n    a-last x = x + 2\n    z-first x = x + 1\n  main :: Int\n  main = if z-first 18446744073709551616 == 18446744073709551617 && a-last 18446744073709551616 == 18446744073709551618 then 103 else 1\n", 103),
        ("dictionary-value", "  import RuntimeClasses\n  implicit instance Big :: Value Int where\n    value = 340282366920938463463374607431768211456\n  main :: Int\n  main = if value == 340282366920938463463374607431768211456 then 107 else 1\n", 107),
        ("dictionary-object-value", "  import RuntimeClasses\n  data Box where\n    Box :: Int -> Box\n  implicit instance Big :: Value Box where\n    value = Box 18446744073709551616\n  main :: Int\n  main = case (value :: Box) of Box x -> if x == 18446744073709551616 then 109 else 1\n", 109),
        ("dictionary-factory-context", "  import RuntimeClasses\n  data Box a where\n    Box :: a -> Box a\n  implicit instance EchoInt :: Echo Int where\n    echo x = x + 1\n  implicit instance EchoBox a :: using (element :: Echo a) => Echo (Box a) where\n    echo xs = case xs of Box x -> Box (echo x)\n  main :: Int\n  main = case echo (Box 18446744073709551616) of Box x -> if x == 18446744073709551617 then 113 else 1\n", 113),
        ("dictionary-self-method", "  import RuntimeClasses\n  implicit instance OpsInt :: Ops Int where\n    first x = second x\n    second x = x + 1\n  main :: Int\n  main = if first 18446744073709551616 == 18446744073709551617 then 127 else 1\n", 127),
        ("dictionary-self-escaping", "  import RuntimeClasses\n  implicit instance OpsInt :: Maker Int where\n    bump x = x + 1\n    make x = \\y -> bump (x + y)\n  main :: Int\n  main = let f = make 18446744073709551616 in if f 2 == 18446744073709551619 && f 3 == 18446744073709551620 then 131 else 1\n", 131),
        ("dictionary-evidence-capture", "  import RuntimeClasses\n  implicit instance EchoInt :: Echo Int where\n    echo x = x + 1\n  maker :: using (e :: Echo Int) => Int -> Int -> Int\n  maker x = \\y -> echo (x + y)\n  main :: Int\n  main = let f = maker 18446744073709551616 in if f 2 == 18446744073709551619 && f 3 == 18446744073709551620 then 137 else 1\n", 137),
        ("dictionary-extra-evidence", "  import RuntimeClasses\n  implicit instance EchoInt :: Echo Int where\n    echo x = x + 1\n  implicit instance UseInt :: Use Int where\n    use x = echo x\n  main :: Int\n  main = if use 18446744073709551616 == 18446744073709551617 then 139 else 1\n", 139),
        ("dictionary-raw-value", "  import RuntimeClasses\n  implicit instance Flag :: Value Bool where\n    value = True\n  main :: Int\n  main = if value then 149 else 1\n", 149),
        ("dictionary-shared-evidence", "  import RuntimeClasses\n  instance EchoInt :: Echo Int where\n    echo x = x + 1\n  run :: using (e :: Echo Int) => Int -> Int\n  run x = echo x + echo x\n  main :: Int\n  main = with (e = EchoInt) in if run 18446744073709551616 == 36893488147419103234 then 151 else 1\n", 151),
        ("erased-region-identity", "  identity :: forall r a. Io r -> a -> a\n  identity token x = x\n  main :: Int\n  main = if identity (newIO ()) 18446744073709551616 == 18446744073709551616 then 157 else 1\n", 157),
        ("io-unit-entry", "  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in 163\n", 163),
        ("io-cancellation", "  main :: Unit ->{IO r, Cancel r} Int\n  main unit = with (io = newIO ()) in case ioSleep 0 of\n    _ -> if pollCancel () then 1 else 167\n", 167),
        ("io-evidence-capture", "  make :: forall r. using (io :: Io r) => Nat -> Unit ->{IO r, Cancel r} Int\n  make n = \\u -> case ioSleep n of _ -> if pollCancel () then 1 else 173\n  main :: Unit ->{IO r, Cancel r} Int\n  main unit = with (io = newIO ()) in let f = make 0 in f ()\n", 173),
        ("io-observable-bytes", "  foreign import ccall \"cinch.runtime.bytesNew\" allocate :: Nat -> Bytes\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in let buffer = allocate 2 in case ioWrite buffer of\n    _ -> case ioWrite buffer of _ -> 179\n", 179),
    ];
    for (index, (_, body, _)) in cases.iter().enumerate() {
        std::fs::write(dir.join(format!("input-{index}.source")), format!("module Main where\n{body}")).unwrap();
    }
    std::fs::write(dir.join(format!("input-{}.source", cases.len())), "module Main where\n  import RuntimeClasses\n  data Box where\n    Box :: Int -> Box\n  instance RawValue :: Value Bool where\n    value = True\n  instance TaggedValue :: Value Int where\n    value = 340282366920938463463374607431768211456\n  instance ObjectValue :: Value Box where\n    value = Box 18446744073709551616\n  instance FunctionValue :: Value (Int -> Int) where\n    value = let n = 18446744073709551616 in \\x -> x + n\n  getRaw :: Value Bool\n  getRaw = RawValue\n  getTagged :: Value Int\n  getTagged = TaggedValue\n  getObject :: Value Box\n  getObject = ObjectValue\n  getFunction :: Value (Int -> Int)\n  getFunction = FunctionValue\n  instance SharedBox :: Shared Box where\n    duplicate = stored\n    stored = Box 18446744073709551616\n    observe u = stored\n    escape-value u = \\v -> observe v\n  getShared :: Shared Box\n  getShared = SharedBox\n  main :: Int\n  main = case (getRaw, getTagged, getObject, getFunction, getShared) of _ -> 0\n").unwrap();
    std::fs::write(dir.join("batch.count"), vec![b'x'; cases.len() + 1]).unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "batch input/output: {}", String::from_utf8_lossy(&output.stderr));
    std::fs::remove_file(dir.join("batch.count")).unwrap();
    // Following ABI/refusal probes need no class interfaces.
    std::fs::write(dir.join("interfaces.json"), "{\"schema\":1,\"interfaces\":[]}").unwrap();
    for (index, (name, _, expected)) in cases.into_iter().enumerate() {
        assert_eq!(common::read_output(&dir, &format!("output-{index}.status")), "0", "{name}: {}", common::read_output(&dir, &format!("output-{index}.diagnostics")));
        assert!(common::read_output(&dir, &format!("output-{index}.diagnostics")).is_empty());
        let ir = common::read_output(&dir, &format!("output-{index}.ll"));
        std::fs::write(dir.join("output.ll"), &ir).unwrap();
        let identity_worker = |line: &&str| line.starts_with("define i64 ") && line.contains("105_100_101_110_116_105_116_121_") && line.contains(".r");
        if name == "generic-identity-classes" {
            assert_eq!(ir.lines().filter(identity_worker).count(), 3, "one worker per reached Raw/Obj/TObj class");
            let raw_header = ir.lines().find(|line| line.starts_with("define i64 ") && line.contains("105_100_101_110_116_105_116_121_.r0")).expect("raw identity worker");
            let raw = ir.split(raw_header).nth(1).unwrap();
            let raw_body = raw.split('}').next().unwrap();
            assert!(!raw_body.contains("@cinch_dup") && !raw_body.contains("@cinch_drop"), "Raw values must never be reference counted: {raw_body}");
        }
        if name == "generic-shared-tobj" || name == "generic-shared-obj" {
            assert_eq!(ir.lines().filter(identity_worker).count(), 1, "{name}: callers with the same class must share one worker");
        }
        if name == "polymorphic-recursion" || name == "generic-data-growing-recursion" {
            let grow_name = "103_114_111_119_";
            assert_eq!(ir.lines().filter(|line| line.starts_with("define i64 ") && line.contains(grow_name) && line.contains(".r")).count(), 2, "{name}: changing source types must reach only two representation versions");
        }
        let exe = dir.join(format!("emitted-{name}"));
        let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .arg(dir.join("output.ll")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(&exe).output().unwrap();
        assert!(output.status.success(), "{name}: {}", String::from_utf8_lossy(&output.stderr));
        let output = Command::new(&exe).env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
        assert_eq!(output.status.code(), Some(expected), "{name}: {}", String::from_utf8_lossy(&output.stderr));
        if name == "io-observable-bytes" {
            assert_eq!(output.stdout, vec![0u8; 4], "IO writes must reach stdout twice");
        } else {
            assert!(output.stdout.is_empty(), "unexpected stdout for {name}");
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        let leak = stderr.lines().find(|line| line.starts_with("cinch-leak:")).expect("runtime leak report");
        assert!(leak.split_whitespace().any(|word| word == "leaked=0"), "{name}: {stderr}");
        println!("{name}: exit {expected}; {leak}");
    }
    let dictionary_index = cases.len();
    assert_eq!(common::read_output(&dir, &format!("output-{dictionary_index}.status")), "0", "dictionary ABI: {}", common::read_output(&dir, &format!("output-{dictionary_index}.diagnostics")));
    let mut dictionary_ir = common::read_output(&dir, &format!("output-{dictionary_index}.ll"))
        .replace("define i32 @main(i32 %argc, ptr %argv)", "define i32 @cinch_program_main(i32 %argc, ptr %argv)");
    let encode = |name: &str| name.chars().map(|ch| format!("{}_", ch as u32)).collect::<String>();
    for (source, bridge) in [("getRaw", "raw"), ("getTagged", "tagged"), ("getObject", "object"), ("getFunction", "function"), ("getShared", "shared")] {
        let symbol = format!("cinch.typed.m.{}.{}", encode("Main"), encode(source));
        dictionary_ir.push_str(&format!("\ndefine i64 @cinch_abi_{bridge}() {{\nentry:\n  %value = call i64 @\"{symbol}\"()\n  ret i64 %value\n}}\n"));
    }
    dictionary_ir.push_str("\ndefine i64 @cinch_abi_dict_shape(i64 %index) {\nentry:\n  %slot = getelementptr i64, ptr @cinch.typed.layouts, i64 %index\n  %value = load i64, ptr %slot\n  ret i64 %value\n}\n");
    std::fs::write(dir.join("dictionaries.ll"), dictionary_ir).unwrap();
    std::fs::write(dir.join("dictionaries.c"), r#"#include <stdint.h>
extern uint64_t cinch_abi_raw(void), cinch_abi_tagged(void), cinch_abi_object(void), cinch_abi_function(void), cinch_abi_shared(void);
extern uint64_t cinch_abi_dict_shape(uint64_t);
extern void cinch_reclaim_enable(void), cinch_dup(uint64_t), cinch_drop(uint64_t);
extern uint64_t cinch_int_from_decimal(const char *, uint64_t), __apply(uint64_t, uint64_t, uint64_t *);
extern int64_t cinch_int_cmp(uint64_t, uint64_t), cinch_exit_check(int64_t);
static int dictionary(uint64_t word, uint64_t map) {
  uint64_t *p = (uint64_t *)(uintptr_t)word, h = p[-1];
  return ((h >> 40) & 255) == 3 && p[0] == 1 && cinch_abi_dict_shape(h >> 48) == (UINT64_C(2) | map << 32);
}
int main(void) {
  cinch_reclaim_enable();
  uint64_t raw = cinch_abi_raw(), tagged = cinch_abi_tagged(), object = cinch_abi_object(), function = cinch_abi_function();
  int good = dictionary(raw, 0) && dictionary(tagged, 8) && dictionary(object, 4) && dictionary(function, 4);
  good = good && ((uint64_t *)(uintptr_t)raw)[1] == 1;
  uint64_t expected = cinch_int_from_decimal("340282366920938463463374607431768211456", 39);
  good = good && cinch_int_cmp(((uint64_t *)(uintptr_t)tagged)[1], expected) == 0;
  cinch_drop(expected);
  uint64_t box = ((uint64_t *)(uintptr_t)object)[1];
  expected = cinch_int_from_decimal("18446744073709551616", 20);
  good = good && cinch_int_cmp(((uint64_t *)(uintptr_t)box)[0], expected) == 0;
  cinch_drop(expected);
  uint64_t f = ((uint64_t *)(uintptr_t)function)[1];
  good = good && ((((uint64_t *)(uintptr_t)f)[-1] >> 40) & 255) == 2;
  cinch_dup(f);
  uint64_t arg = cinch_int_from_decimal("2", 1), result = __apply(f, 1, &arg);
  expected = cinch_int_from_decimal("18446744073709551618", 20);
  good = good && cinch_int_cmp(result, expected) == 0;
  cinch_drop(result); cinch_drop(expected);
  cinch_drop(raw); cinch_drop(tagged); cinch_drop(object); cinch_drop(function);
  uint64_t shared = cinch_abi_shared(), *p = (uint64_t *)(uintptr_t)shared;
  uint64_t header = p[-1], shared_box = p[1];
  good = good && ((header >> 40) & 255) == 3 && p[0] == 4
    && cinch_abi_dict_shape(header >> 48) == (UINT64_C(5) | UINT64_C(340) << 32)
    && p[1] == p[2];
  uint64_t unit = 0, observer = p[3];
  cinch_dup(observer);
  uint64_t observed = __apply(observer, 1, &unit);
  good = good && observed == shared_box;
  cinch_drop(observed);
  uint64_t maker = p[4];
  cinch_dup(maker);
  uint64_t escaped = __apply(maker, 1, &unit);
  cinch_drop(shared);
  observed = __apply(escaped, 1, &unit);
  expected = cinch_int_from_decimal("18446744073709551616", 20);
  good = good && observed == shared_box
    && cinch_int_cmp(((uint64_t *)(uintptr_t)observed)[0], expected) == 0;
  cinch_drop(observed); cinch_drop(expected);
  return (int)cinch_exit_check(good ? 0 : 239);
}
"#).unwrap();
    let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("dictionaries.ll")).arg(dir.join("dictionaries.c")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(dir.join("dictionary-abi")).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new(dir.join("dictionary-abi")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"));
    println!("C dictionary ABI: kind3, exact count/maps for Raw/TObj/Obj/closure values; {}", String::from_utf8_lossy(&output.stderr).trim());
    // Verify the physical ABI through C, including the anonymous tuple's
    // tag0 when its map matches a named tag1 constructor's map.
    std::fs::write(dir.join("input.source"), "module Main where\n  data D where\n    A :: Int -> D\n    B :: D -> D -> D\n  makePair :: (D, D)\n  makePair = let leaf = A 18446744073709551616 in let node = B leaf leaf in (node, leaf)\n  data Wrap a where\n    Wrap :: a -> Wrap a\n  makeGeneric :: (Wrap Bool, Wrap Int, Wrap D)\n  makeGeneric = let leaf = A 18446744073709551616 in (Wrap True, Wrap 340282366920938463463374607431768211456, Wrap leaf)\n  main :: Int\n  main = case (makePair, makeGeneric) of _ -> 0\n").unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "ABI: {} {}", common::read_output(&dir, "output.diagnostics"), String::from_utf8_lossy(&output.stderr));
    let ir = common::read_output(&dir, "output.ll");
    assert!(ir.contains("define i32 @main(i32 %argc, ptr %argv)"));
    let mut ir = ir.replace("define i32 @main(i32 %argc, ptr %argv)", "define i32 @cinch_program_main(i32 %argc, ptr %argv)");
    let encode = |name: &str| name.chars().map(|ch| format!("{}_", ch as u32)).collect::<String>();
    let symbol = format!("cinch.typed.m.{}.{}", encode("Main"), encode("makePair"));
    ir.push_str(&format!("\ndefine i64 @cinch_abi_pair() {{\nentry:\n  %pair = call i64 @\"{symbol}\"()\n  ret i64 %pair\n}}\n"));
    let generic_symbol = format!("cinch.typed.m.{}.{}", encode("Main"), encode("makeGeneric"));
    ir.push_str(&format!("\ndefine i64 @cinch_abi_generic() {{\nentry:\n  %value = call i64 @\"{generic_symbol}\"()\n  ret i64 %value\n}}\n"));
    ir.push_str("\ndefine i64 @cinch_abi_layout(i64 %index) {\nentry:\n  %slot = getelementptr i64, ptr @cinch.typed.layouts, i64 %index\n  %word = load i64, ptr %slot\n  ret i64 %word\n}\n");
    std::fs::write(dir.join("abi.ll"), ir).unwrap();
    std::fs::write(dir.join("abi.c"), r#"
#include <stdint.h>
extern uint64_t cinch_abi_pair(void);
extern uint64_t cinch_abi_generic(void);
extern uint64_t cinch_abi_layout(uint64_t);
extern void cinch_reclaim_enable(void);
extern void cinch_drop(uint64_t);
extern int64_t cinch_exit_check(int64_t);
static uint64_t tag(uint64_t word) { return (((uint64_t *)(uintptr_t)word)[-1] >> 48) & 63; }
int main(void) {
  cinch_reclaim_enable();
  uint64_t pair = cinch_abi_pair();
  uint64_t *fields = (uint64_t *)(uintptr_t)pair;
  uint64_t node = fields[0], leaf = fields[1];
  uint64_t *children = (uint64_t *)(uintptr_t)node;
  int good = tag(pair) == 0 && tag(node) == 1 && tag(leaf) == 0
    && children[0] == leaf && children[1] == leaf;
  uint64_t generic = cinch_abi_generic();
  uint64_t *boxes = (uint64_t *)(uintptr_t)generic;
  uint64_t raw_id = (((uint64_t *)(uintptr_t)boxes[0])[-1] >> 48) & 65535;
  uint64_t tagged_id = (((uint64_t *)(uintptr_t)boxes[1])[-1] >> 48) & 65535;
  uint64_t object_id = (((uint64_t *)(uintptr_t)boxes[2])[-1] >> 48) & 65535;
  good = good && raw_id != tagged_id && tagged_id != object_id && raw_id != object_id
    && cinch_abi_layout(raw_id) == 1
    && cinch_abi_layout(tagged_id) == (UINT64_C(2) << 32 | 1)
    && cinch_abi_layout(object_id) == (UINT64_C(1) << 32 | 1)
    && ((uint64_t *)(uintptr_t)boxes[0])[0] == 1;
  cinch_drop(pair);
  cinch_drop(generic);
  return (int)cinch_exit_check(good ? 0 : 149);
}
"#).unwrap();
    let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("abi.ll")).arg(dir.join("abi.c")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(dir.join("abi-program")).output().unwrap();
    assert!(output.status.success(), "ABI link: {}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new(dir.join("abi-program")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(0), "physical ABI: {}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"));
    println!("C ABI tags and aliases: exit0; {}", String::from_utf8_lossy(&output.stderr).trim());
    // The typed frontend accepts this; unavailable lowering must refuse it
    // explicitly and clear the preceding successful IR artifact.
    std::fs::write(dir.join("input.source"), "module Main where\n  identity :: forall (f :: Type -> Type) a. f a -> f a\n  identity x = x\n  data Wrap a where\n    Wrap :: a -> Wrap a\n  main :: Int\n  main = case identity (Wrap 1) of Wrap n -> n\n").unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(5), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(common::read_output(&dir, "output.diagnostics").contains("representation"));
    assert!(common::read_output(&dir, "output.ll").is_empty());
    std::fs::write(dir.join("input.source"), "module Main where\n  main :: Int\n  main = case (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17) of\n    _ -> 0\n").unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(5), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(common::read_output(&dir, "output.diagnostics").contains("extended"));
    assert!(common::read_output(&dir, "output.ll").is_empty());
    let source = format!("module Main where\n  data Tag where\n{}  main :: Int\n  main = case C64 of\n    C64 -> 1\n    _ -> 2\n", (0..65).map(|i| format!("    C{i} :: Tag\n")).collect::<String>());
    std::fs::write(dir.join("input.source"), source).unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(5), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(common::read_output(&dir, "output.diagnostics").contains("extended"));
    assert!(common::read_output(&dir, "output.ll").is_empty());
    common::cleanup(&producer);
    common::cleanup(&dir);
}

fn quote(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '\"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"),
            c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('\"'); out
}

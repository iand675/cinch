//! Returned functions and lambda patterns use ordinary checked result types
//! and the same exhaustive pattern compiler as case/function equations.
use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::Mode;
use std::process::Command;

mod common;

#[test]
fn returned_functions_and_lambda_patterns_check_execute_and_reclaim() {
    std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn(|| {
        let cases = [
            ("lambda-argument", "module Main where\n  apply :: (Int -> Int) -> Int -> Int\n  apply f x = f x\n  main :: Int\n  main = if apply (\\x -> x + 1) 18446744073709551616 == 18446744073709551617 then 199 else 1\n", 199),
            ("escaping-capture", "module Main where\n  make :: Int -> Int -> Int\n  make x = \\y -> x + y\n  main :: Int\n  main = if make 18446744073709551616 2 == 18446744073709551618 then 201 else 1\n", 201),
            ("shared-closure", "module Main where\n  make :: Int -> Int -> Int\n  make x = \\y -> x + y\n  main :: Int\n  main = let f = make 18446744073709551616 in if f 2 == 18446744073709551618 && f 3 == 18446744073709551619 then 203 else 1\n", 203),
            ("global-function-value", "module Main where\n  apply :: (Int -> Int) -> Int -> Int\n  apply f x = f x\n  bump :: Int -> Int\n  bump x = x + 1\n  main :: Int\n  main = if apply bump 18446744073709551616 == 18446744073709551617 then 205 else 1\n", 205),
            ("partial-function", "module Main where\n  add :: Int -> Int -> Int\n  add x y = x + y\n  main :: Int\n  main = let f = add 18446744073709551616 in if f 2 == 18446744073709551618 then 207 else 1\n", 207),
            ("dynamic-partial-application", "module Main where\n  add :: Int -> Int -> Int\n  add x y = x + y\n  twice :: (Int -> Int -> Int) -> Int -> Int -> Int\n  twice f x y = let partial = f x in partial y\n  main :: Int\n  main = if twice add 18446744073709551616 2 == 18446744073709551618 then 209 else 1\n", 209),
            ("partial-constructor", "module Main where\n  data Pair a where\n    Pair :: a -> a -> Pair a\n  makePair :: Int -> Int -> Pair Int\n  makePair x = Pair x\n  main :: Int\n  main = let huge = 18446744073709551616 in case makePair huge huge of Pair x y -> if x == y then 211 else 1\n", 211),
            ("nested-escaping-closures", "module Main where\n  make :: Int -> Int -> Int -> Int\n  make x = \\y -> \\z -> x + y + z\n  main :: Int\n  main = if make 18446744073709551616 2 3 == 18446744073709551621 then 213 else 1\n", 213),
            ("unused-capture", "module Main where\n  ignore :: Int -> Int -> Int\n  ignore outer = \\inner -> inner\n  main :: Int\n  main = if ignore 18446744073709551616 9 == 9 then 215 else 1\n", 215),
            ("shadowed-capture", "module Main where\n  make :: Int -> Int -> Int -> Int\n  make x = \\x -> let y = x + 1 in \\z -> y + z\n  main :: Int\n  main = if make 18446744073709551616 2 3 == 6 then 217 else 1\n", 217),
            ("generic-capture-classes", "module Main where\n  constant :: forall a b. a -> b -> a\n  constant x = \\y -> x\n  main :: Int\n  main = if constant True (18446744073709551616 :: Int) && constant 18446744073709551616 True == 18446744073709551616 then 219 else 1\n", 219),
            ("higher-order-generic-map", "module Main where\n  data Seq a where\n    End :: Seq a\n    Link :: a -> Seq a -> Seq a\n  mapSeq :: forall a b. (a -> b) -> Seq a -> Seq b\n  mapSeq f End = End\n  mapSeq f (Link x rest) = Link (f x) (mapSeq f rest)\n  total :: Seq Int -> Int\n  total End = 0\n  total (Link x rest) = x + total rest\n  main :: Int\n  main = if total (mapSeq (\\x -> x + 18446744073709551616) (Link (2 :: Int) (Link 3 End))) == 36893488147419103237 then 221 else 1\n", 221),
            ("borrowed-field-capture", "module Main where\n  data Box where\n    Box :: Int -> Box\n  get :: Box -> Int -> Int\n  get box = case box of Box n -> \\x -> n + x\n  main :: Int\n  main = if get (Box 18446744073709551616) 3 == 18446744073709551619 then 223 else 1\n", 223),
            ("closure-constructor-field", "module Main where\n  data Action where\n    Action :: (Int -> Int) -> Action\n  run :: Action -> Int -> Int\n  run (Action f) x = f x\n  main :: Int\n  main = if run (Action (\\x -> x + 18446744073709551616)) 7 == 18446744073709551623 then 225 else 1\n", 225),
            ("branch-returned-closure", "module Main where\n  pick :: Bool -> Int -> Int\n  pick flag = if flag then (\\x -> x + 18446744073709551616) else (\\x -> x - 1)\n  main :: Int\n  main = if pick True 2 == 18446744073709551618 && pick False 2 == 1 then 227 else 1\n", 227),
            ("generic-object-capture", "module Main where\n  data Box where\n    Box :: Int -> Box\n  constant :: forall a b. a -> b -> a\n  constant x = \\y -> x\n  main :: Int\n  main = case constant (Box 18446744073709551616) False of Box x -> if x == 18446744073709551616 then 229 else 1\n", 229),
            ("lambda-tuple-pattern", "module Main where\n  run :: (Int, Int) -> Int\n  run = \\(x, y) -> x + y\n  main :: Int\n  main = if run (18446744073709551616, 2) == 18446744073709551618 then 231 else 1\n", 231),
            ("function-captures", "module Main where\n  compose :: (Int -> Int) -> (Int -> Int) -> Int -> Int\n  compose f g = \\x -> f (g x)\n  bump :: Int -> Int\n  bump x = x + 1\n  main :: Int\n  main = if compose bump bump 18446744073709551616 == 18446744073709551618 then 233 else 1\n", 233),
        ];
        for (index, (name, source, expected)) in cases.iter().enumerate() {
            let dir = std::env::temp_dir().join(format!("cinch_native_closure_{}_{}", std::process::id(), index));
            std::fs::create_dir_all(&dir).unwrap();
            let input = dir.join("Main.cinch");
            std::fs::write(&input, source).unwrap();
            let output = dir.join("artifact");
            let profile = TargetProfile { reclaim: true, rc_debug: true, ..TargetProfile::native() };
            common::drive_with_profile(Mode::Build, StageMode::Refresh, &dir, "Main.cinch", Some(output.clone()), &profile)
                .unwrap_or_else(|error| panic!("{name}: {error}"));
            let ran = Command::new(&output).env("CINCH_HEAP_LIMIT", "32M").env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
            let stderr = String::from_utf8_lossy(&ran.stderr);
            assert_eq!(ran.status.code(), Some(*expected), "{name}: {stderr}");
            assert!(stderr.split_whitespace().any(|word| word == "leaked=0"), "{name}: {stderr}");
            println!("{name}: exit{expected}; {stderr}");
        }
        let negatives = [
            "module Main where\n  bad :: Bool -> Int\n  bad = \\True -> 0\n  main :: Int\n  main = 0\n",
            "module Main where\n  bad :: (Int, Int) -> Int\n  bad = \\(x, x) -> x\n  main :: Int\n  main = 0\n",
            "module Main where\n  bad :: Int -> Int -> Bool\n  bad x = \\y -> x + y\n  main :: Int\n  main = 0\n",
            "module Main where\n  bad :: Int -> Int\n  bad x y = x\n  main :: Int\n  main = 0\n",
        ];
        for (index, source) in negatives.iter().enumerate() {
            let dir = std::env::temp_dir().join(format!("cinch_native_closure_negative_{}_{}", std::process::id(), index));
            std::fs::create_dir_all(&dir).unwrap();
            let input = dir.join("Main.cinch");
            std::fs::write(&input, source).unwrap();
            let profile = TargetProfile::native();
            assert!(common::drive_with_profile(Mode::Check, StageMode::Refresh, &dir, "Main.cinch", None, &profile).is_err(), "negative{index} accepted");
        }
    }).unwrap().join().unwrap();
}

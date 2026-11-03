//! Actual source factories, strict dictionary values, scoped evidence and IO.
use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{driver_with_profile, Mode};
use std::process::Command;
#[test]
fn dictionaries_use_actual_factories_and_reclaim_value_slots() {
    std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn(|| {
        let cases = [
            ("selected-dictionary", "module Main where\n  signature Choose a where\n    choose :: a -> Int\n  instance First :: Choose Int where\n    choose x = x + 1\n  instance Second :: Choose Int where\n    choose x = x + 2\n  run :: using (chosen :: Choose Int) => Int -> Int\n  run x = choose x\n  main :: Int\n  main = if (with (d = First) in run 18446744073709551616) == 18446744073709551617 && (with (d = Second) in run 18446744073709551616) == 18446744073709551618 then 101 else 1\n", 101),
            ("dictionary-slot-order", "module Main where\n  signature Ops a where\n    z-first :: a -> a\n    a-last :: a -> a\n  implicit instance OpsInt :: Ops Int where\n    a-last x = x + 2\n    z-first x = x + 1\n  main :: Int\n  main = if z-first 18446744073709551616 == 18446744073709551617 && a-last 18446744073709551616 == 18446744073709551618 then 103 else 1\n", 103),
            ("dictionary-value", "module Main where\n  signature Value a where\n    value :: a\n  implicit instance Big :: Value Int where\n    value = 340282366920938463463374607431768211456\n  main :: Int\n  main = if value == 340282366920938463463374607431768211456 then 107 else 1\n", 107),
            ("dictionary-object-value", "module Main where\n  data Box where\n    Box :: Int -> Box\n  signature Value a where\n    value :: a\n  implicit instance Big :: Value Box where\n    value = Box 18446744073709551616\n  main :: Int\n  main = case (value :: Box) of Box x -> if x == 18446744073709551616 then 109 else 1\n", 109),
            ("dictionary-factory-context", "module Main where\n  data Box a where\n    Box :: a -> Box a\n  signature Echo a where\n    echo :: a -> a\n  implicit instance EchoInt :: Echo Int where\n    echo x = x + 1\n  implicit instance EchoBox a :: using (element :: Echo a) => Echo (Box a) where\n    echo xs = case xs of Box x -> Box (echo x)\n  main :: Int\n  main = case echo (Box 18446744073709551616) of Box x -> if x == 18446744073709551617 then 113 else 1\n", 113),
            ("dictionary-self-method", "module Main where\n  signature Ops a where\n    first :: a -> a\n    second :: a -> a\n  implicit instance OpsInt :: Ops Int where\n    first x = second x\n    second x = x + 1\n  main :: Int\n  main = if first 18446744073709551616 == 18446744073709551617 then 127 else 1\n", 127),
            ("dictionary-self-escaping", "module Main where\n  signature Ops a where\n    first :: a -> a\n    second :: a -> a -> a\n  implicit instance OpsInt :: Ops Int where\n    first x = x + 1\n    second x = \\y -> first (x + y)\n  main :: Int\n  main = let f = second 18446744073709551616 in if f 2 == 18446744073709551619 && f 3 == 18446744073709551620 then 131 else 1\n", 131),
            ("dictionary-evidence-capture", "module Main where\n  signature Echo a where\n    echo :: a -> a\n  implicit instance EchoInt :: Echo Int where\n    echo x = x + 1\n  maker :: using (e :: Echo Int) => Int -> Int -> Int\n  maker x = \\y -> echo (x + y)\n  main :: Int\n  main = let f = maker 18446744073709551616 in if f 2 == 18446744073709551619 && f 3 == 18446744073709551620 then 137 else 1\n", 137),
            ("dictionary-extra-evidence", "module Main where\n  signature Echo a where\n    echo :: a -> a\n  signature Use a where\n    use :: using (extra :: Echo Int) => a -> Int\n  implicit instance EchoInt :: Echo Int where\n    echo x = x + 1\n  implicit instance UseInt :: Use Int where\n    use x = echo x\n  main :: Int\n  main = if use 18446744073709551616 == 18446744073709551617 then 139 else 1\n", 139),
            ("dictionary-raw-value", "module Main where\n  signature Value a where\n    value :: a\n  implicit instance Flag :: Value Bool where\n    value = True\n  main :: Int\n  main = if value then 149 else 1\n", 149),
            ("dictionary-shared-evidence", "module Main where\n  signature Echo a where\n    echo :: a -> a\n  instance EchoInt :: Echo Int where\n    echo x = x + 1\n  run :: using (e :: Echo Int) => Int -> Int\n  run x = echo x + echo x\n  main :: Int\n  main = with (e = EchoInt) in if run 18446744073709551616 == 36893488147419103234 then 151 else 1\n", 151),
            ("erased-region-identity", "module Main where\n  identity :: forall r a. Io r -> a -> a\n  identity token x = x\n  main :: Int\n  main = if identity (newIO ()) 18446744073709551616 == 18446744073709551616 then 157 else 1\n", 157),
            ("io-unit-entry", "module Main where\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in 163\n", 163),
            ("io-cancellation", "module Main where\n  main :: Unit ->{IO r, Cancel r} Int\n  main unit = with (io = newIO ()) in case ioSleep 0 of\n    _ -> if pollCancel () then 1 else 167\n", 167),
            ("io-evidence-capture", "module Main where\n  make :: forall r. using (io :: Io r) => Nat -> Unit ->{IO r, Cancel r} Int\n  make n = \\u -> case ioSleep n of _ -> if pollCancel () then 1 else 173\n  main :: Unit ->{IO r, Cancel r} Int\n  main unit = with (io = newIO ()) in let f = make 0 in f ()\n", 173),
            ("io-observable-bytes", "module Main where\n  foreign import ccall \"cinch.runtime.bytesNew\" allocate :: Nat -> Bytes\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in let buffer = allocate 2 in case ioWrite buffer of\n    _ -> case ioWrite buffer of _ -> 179\n", 179),
            ("generic-value-maps", "module Main where\n  data Box a where\n    Box :: a -> Box a\n  signature Source a where\n    source :: a\n  signature Value a where\n    value :: a\n  implicit instance SourceInt :: Source Int where\n    source = 340282366920938463463374607431768211456\n  implicit instance SourceBox :: Source (Box Int) where\n    source = Box 18446744073709551616\n  implicit instance CopyValue a :: using (input :: Source a) => Value a where\n    value = source\n  intValue :: Int\n  intValue = value\n  boxValue :: Box Int\n  boxValue = value\n  main :: Int\n  main = case boxValue of\n    Box x -> if x == 18446744073709551616 && intValue == 340282366920938463463374607431768211456 then 181 else 1\n", 181),
        ];
        for (index, (name, source, expected)) in cases.iter().enumerate() {
            let dir = std::env::temp_dir().join(format!("cinch_native_dictionary_{}_{}", std::process::id(), index));
            std::fs::create_dir_all(&dir).unwrap();
            let input = dir.join("Main.cinch");
            std::fs::write(&input, source).unwrap();
            let output = dir.join("artifact");
            let profile = TargetProfile { reclaim: true, rc_debug: true, ..TargetProfile::native() };
            driver_with_profile(Mode::Build, StageMode::Refresh, &input, Some(&output), &profile)
                .unwrap_or_else(|error| panic!("{name}: {error}"));
            let ran = Command::new(&output).env("CINCH_HEAP_LIMIT", "32M").env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
            let stderr = String::from_utf8_lossy(&ran.stderr);
            assert_eq!(ran.status.code(), Some(*expected), "{name}: {stderr}");
            assert!(stderr.split_whitespace().any(|word| word == "leaked=0"), "{name}: {stderr}");
            if *name == "io-observable-bytes" { assert_eq!(ran.stdout, vec![0u8; 4]); }
            else { assert!(ran.stdout.is_empty(), "{name}: unexpected stdout"); }
            println!("{name}: exit{expected}; {stderr}");
        }
        let dir = std::env::temp_dir().join(format!("cinch_native_dictionary_scope_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("A.cinch"), "module A where\n  export Echo, echo, Explicit\n  signature Echo a where\n    echo :: a -> a\n  instance Explicit :: Echo Int where\n    echo x = x + 1\n").unwrap();
        let input = dir.join("Main.cinch");
        let scoped = "module Main where\n  import A (Echo, echo, Explicit as Selected)\n  main :: Int\n  main = with (e = Selected) in if echo 18446744073709551616 == 18446744073709551617 then 191 else 1\n";
        std::fs::write(&input, scoped).unwrap();
        let output = dir.join("artifact");
        let profile = TargetProfile { reclaim: true, rc_debug: true, ..TargetProfile::native() };
        driver_with_profile(Mode::Build, StageMode::Refresh, &input, Some(&output), &profile).expect("renamed exported instance");
        let ran = Command::new(&output).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
        assert_eq!(ran.status.code(), Some(191), "{}", String::from_utf8_lossy(&ran.stderr));
        assert!(String::from_utf8_lossy(&ran.stderr).split_whitespace().any(|w| w == "leaked=0"));
        for unavailable in ["Explicit", "A.Explicit"] {
            std::fs::write(&input, scoped.replace("e = Selected", &format!("e = {unavailable}"))).unwrap();
            assert!(driver_with_profile(Mode::Check, StageMode::Refresh, &input, None, &profile).is_err(), "original instance spelling escaped import rename: {unavailable}");
        }
        let invalid = [
            "module Main where\n  main :: Int\n  main = with (fake = 1) in 0\n",
            "module Main where\n  signature Value a where\n    value :: a\n  instance Flag :: Value Bool where\n    value = True\n  main :: Int\n  main = with (e = Flag) in value + 1\n",
            "module Main where\n  signature Value a where\n    value :: a\n  instance Flag :: Value Int where\n    value = 1\n  main :: Int\n  main = value\n",
        ];
        for (index, source) in invalid.iter().enumerate() {
            let dir = std::env::temp_dir().join(format!("cinch_native_dictionary_invalid_{}_{}", std::process::id(), index));
            std::fs::create_dir_all(&dir).unwrap();
            let input = dir.join("Main.cinch"); std::fs::write(&input, source).unwrap();
            assert!(driver_with_profile(Mode::Check, StageMode::Refresh, &input, None, &TargetProfile::native()).is_err(), "invalid dictionary case {index} accepted");
        }
    }).unwrap().join().unwrap();
}

//! Erased constructor witnesses retain lexical runtime value identity.
#[test]
fn constructor_proofs_keep_derivation_scope_and_strict_aliases() {
    let cases = [
        ("shadowed zero", "bad :: Int -> Option (NonZero Int)\n  bad n = case decide (NonZero n) of Yes w -> let n = (0 :: Int) in Some (NonZero n w); No other -> None", false),
        ("shadowed different value", "bad :: Int -> Int -> Option (NonZero Int)\n  bad n m = case decide (NonZero n) of Yes w -> let n = m in Some (NonZero n w); No other -> None", false),
        ("strict alias", "make :: Int -> Option (NonZero Int)\n  make n = case decide (NonZero n) of Yes w -> let alias = n in Some (NonZero alias w); No other -> None", true),
        ("strict alias chain", "make :: Int -> Option (NonZero Int)\n  make n = case decide (NonZero n) of Yes w -> let first = n in let second = first in Some (NonZero second w); No other -> None", true),
        ("outer value alias survives shadow", "make :: Int -> Option (NonZero Int)\n  make n = case decide (NonZero n) of Yes w -> let original = n in let n = (0 :: Int) in Some (NonZero original w); No other -> None", true),
        ("same value shadow", "make :: Int -> Option (NonZero Int)\n  make n = case decide (NonZero n) of Yes w -> let n = n in Some (NonZero n w); No other -> None", true),
        ("different evidence", "bad :: Int -> Int -> Option (NonZero Int)\n  bad n m = case decide (NonZero n) of Yes w -> Some (NonZero m w); No other -> None", false),
        ("proof name shadows payload name", "make :: Int -> Option (NonZero Int)\n  make n = let original = n in case decide (NonZero n) of Yes n -> Some (NonZero original n); No other -> None", true),
    ];
    for (name, body, accepted) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let actual = cinch::driver::check_sources(&source, "proof-value-identity.cinch");
        assert_eq!(actual.is_ok(), accepted, "{name}: {}", actual.err().map_or("accepted".into(), |e| e.message));
    }
}

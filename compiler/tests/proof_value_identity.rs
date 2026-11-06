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


#[test]
fn quantified_proposition_domains_keep_their_declared_kind() {
    let cases = [
        ("original positive proof constructor", "wrap :: forall (p :: Prop). (h :: p) -> Dec p\n  wrap h = Yes h", true),
        ("unnamed proof domain", "wrap :: forall (p :: Prop). p -> Dec p\n  wrap h = Yes h", true),
        ("negative proof constructor", "reject :: forall (p :: Prop). (h :: Not p) -> Dec p\n  reject h = No h", true),
        ("both decision alternatives", "extract :: forall (p :: Prop). Dec p -> Int\n  extract value = case value of Yes h -> 1; No h -> 0", true),
        ("proof cannot escape into a runtime result", "bad :: forall (p :: Prop). p -> Int\n  bad h = h", false),
        ("different propositions remain distinct", "bad :: forall (p :: Prop) (q :: Prop). (h :: p) -> Dec q\n  bad h = Yes h", false),
        ("positive evidence is not negative evidence", "bad :: forall (p :: Prop). (h :: p) -> Dec p\n  bad h = No h", false),
        ("negative evidence is not positive evidence", "bad :: forall (p :: Prop). (h :: Not p) -> Dec p\n  bad h = Yes h", false),
        ("ordinary type cannot become a proposition", "bad :: forall (p :: Type). p -> Dec p\n  bad h = Yes h", false),
        ("proof is not a numeric index", "bad :: forall (p :: Prop). Vec p Int\n  bad = VNil", false),
        ("later signature cannot inherit an earlier proposition kind", "keep :: forall (p :: Prop). p -> Dec p\n  keep h = Yes h\n  identity :: forall p. p -> p\n  identity value = value", true),
    ];
    for (name, body, accepted) in cases {
        let body = body.replace("\\n", "\n");
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let actual = cinch::driver::check_sources(&source, "proposition-carrier.cinch");
        assert_eq!(actual.is_ok(), accepted, "{name}: {actual:?}\n{source}");
    }
}


#[test]
fn quantified_evidence_constructors_execute_with_erased_fields() {
    let cases = [
        r#"module Main where
  wrap :: forall (p :: Prop). (h :: p) -> Dec p
  wrap h = Yes h
  main :: Int
  main = case decide (1 < 2) of
    Yes h -> case wrap h of Yes _ -> 0; No _ -> 1
    No _ -> 2
"#,
        r#"module Main where
  reject :: forall (p :: Prop). (h :: Not p) -> Dec p
  reject h = No h
  main :: Int
  main = case decide (2 < 1) of
    Yes _ -> 2
    No h -> case reject h of Yes _ -> 1; No _ -> 0
"#,
    ];
    for source in cases {
        assert_eq!(cinch::driver::run_source(source).expect("compile quantified erased evidence"), 0);
    }
}

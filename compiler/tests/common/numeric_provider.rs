// A checked numeric provider deliberately has no byte/string consumers whose
// proofs would depend on a replacement operator. The full embedded-family
// regression cases remain separate and unchanged.
pub fn family() -> cinch::Result<cinch::driver::PreludeSources> {
    let canonical = cinch::driver::parse_prelude(&[])?;
    let parsed = cinch::driver::parse_source(r#"
module Relation.Eq where
  infix 3 ==, /=
  export Eq, (==), (/=)
  signature Eq a where
    (==) :: a -> a -> Bool
    (/=) :: a -> a -> Bool
module Relation.Ord where
  infix 3 <, <=, >, >=
  export Ord, (<), (<=), (>), (>=)
  signature Ord a where
    (<) :: a -> a -> Bool
    (<=) :: a -> a -> Bool
    (>) :: a -> a -> Bool
    (>=) :: a -> a -> Bool
module Algebra.Arithmetic where
  infixl 9 *
  infixl 8 +, -
  export Add, (+), Sub, (-), Mul, (*), InRange
  type InRange (lo :: Int) (hi :: Int) a = refine (x :: a) with (lo <= x) && (x <= hi)
  signature Add a where
    (+) :: a -> a -> a
  signature Sub a where
    (-) :: a -> a -> a
  signature Mul a where
    (*) :: a -> a -> a
module Data.Nat where
  import Relation.Eq
  import Relation.Ord
  import Algebra.Arithmetic
  export nat-add, nat-mul, nat-cmp, lt-i64, eq-i64
  foreign import ccall "cinch.runtime.natAdd" nat-add :: Nat -> Nat -> Nat
  foreign import ccall "cinch.runtime.natMul" nat-mul :: Nat -> Nat -> Nat
  foreign import ccall "cinch.runtime.natCmp" nat-cmp :: Nat -> Nat -> I64
  foreign import ccall "cinch.prim.ltI64" lt-i64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.leI64" le-i64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.gtI64" gt-i64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.geI64" ge-i64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.eqI64" eq-i64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.neI64" ne-i64 :: I64 -> I64 -> Bool
  implicit instance Eq_Nat :: Eq Nat where
    (==) x y = eq-i64 (nat-cmp x y) 0
    (/=) x y = ne-i64 (nat-cmp x y) 0
  implicit instance Ord_Nat :: Ord Nat where
    (<) x y = lt-i64 (nat-cmp x y) 0
    (<=) x y = le-i64 (nat-cmp x y) 0
    (>) x y = gt-i64 (nat-cmp x y) 0
    (>=) x y = ge-i64 (nat-cmp x y) 0
  implicit instance Add_Nat :: Add Nat where
    (+) x y = nat-add x y
  implicit instance Mul_Nat :: Mul Nat where
    (*) x y = nat-mul x y
module Data.Int where
  import Data.Nat as N
  import Relation.Eq
  import Relation.Ord
  import Algebra.Arithmetic
  export int-add, int-sub, int-cmp
  foreign import ccall "cinch.runtime.intAdd" int-add :: Int -> Int -> Int
  foreign import ccall "cinch.runtime.intSub" int-sub :: Int -> Int -> Int
  foreign import ccall "cinch.runtime.intCmp" int-cmp :: Int -> Int -> I64
  implicit instance Eq_Int :: Eq Int where
    (==) x y = N.eq-i64 (int-cmp x y) 0
    (/=) x y = if N.eq-i64 (int-cmp x y) 0 then False else True
  implicit instance Add_Int :: Add Int where
    (+) x y = int-add x y
  implicit instance Sub_Int :: Sub Int where
    (-) x y = int-sub x y
module Data.String where
  import Algebra.Arithmetic
  import Relation.Ord
  utf8-encode-width :: Nat -> InRange 1 4 Nat
  utf8-encode-width cp =
    if cp < 128 then 1
    else if cp < 2048 then 2
    else if cp < 65536 then 3
    else 4
module Prelude where
  import Relation.Eq
  import Relation.Ord
  import Algebra.Arithmetic
  import Data.Nat
  import Data.Int
  export module Relation.Eq
  export module Relation.Ord
  export module Algebra.Arithmetic
  export module Data.Nat
  export module Data.Int
"#, "numeric-provider.cinch", &canonical.base, &canonical.exported_fixities)?;
    Ok(cinch::driver::PreludeSources { modules: parsed.modules, fixities: parsed.fixities,
        exported_fixities: parsed.exported_fixities, base: canonical.base })
}

/// Replace the requested method equation in its actual checked provider.
/// Classes, instance identities, slots and all other definitions remain.
pub fn replace_method(family: &mut cinch::driver::PreludeSources, module: &str, original: &str, old: &str, replacement: &str) -> cinch::Result<()> {
    assert_eq!(original.matches(old).count(), 1);
    let parsed = cinch::driver::parse_source(&original.replace(old, replacement), "selected-provider.cinch", &family.base, &family.exported_fixities)?;
    let changed = parsed.modules.into_iter().next().unwrap();
    let method_name = old.split_whitespace().next().unwrap().trim_matches(['(', ')']);
    let (instance_name, equation) = changed.decls.iter().find_map(|declaration| {
        let cinch::ast::Decl::Instance(instance) = declaration else { return None };
        instance.methods.iter().find(|method| method.name.text == method_name).map(|method| (instance.name.text.clone(), method.clone()))
    }).expect("actual changed method");
    let destination = family.modules.iter_mut().find(|m| m.name.text == module).expect("actual defining module");
    let instance = destination.decls.iter_mut().find_map(|declaration| match declaration {
        cinch::ast::Decl::Instance(instance) if instance.name.text == instance_name => Some(instance), _ => None,
    }).expect("same selected instance");
    *instance.methods.iter_mut().find(|method| method.name.text == method_name).unwrap() = equation;
    Ok(())
}

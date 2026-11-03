# 12. Ownership, borrowing, and regions

An effect says what a computation may do; a multiplicity says how it may use an argument. This matters for resources such as task handles and mutable state, where duplicating or dropping a value can break the protocol.

# Read the usage annotation

| Arrow | Meaning |
| --- | --- |
| `a %0 -> b` | The argument is erased; it cannot supply runtime data. |
| `a %1 -> b` | Consume the argument exactly once on every path. |
| `a %aff -> b` | Consume it at most once. |
| `a %many -> b` | Unrestricted uses, subject to the value's other rules. |

Usage is checked across control-flow paths. Two alternative branches each using a value once do not use it twice: only one branch runs.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  choose :: Bool -> Nat %1 -> Nat
  choose flag value = if flag then value else value

  main :: Int
  main = if choose False 42 == 42 then 0 else 1
```

In contrast, the following definition duplicates its linear argument:

<!-- cinch:reject contains="multiplicity %1" -->
```cinch
module Main where
  duplicate :: Nat %1 -> Nat
  duplicate n = n + n

  main :: Int
  main = 0
```

Changing one branch of `choose` to return a constant would drop `value` on that path and would also be rejected. Do not resolve a resource error by changing `%1` to `%many` without understanding the resource's intended protocol.

# Borrow when you only need to inspect

`Ref r a` is a shared borrow in region `r`; `Mut r a` is a mutable borrow. `copyOut` produces an owned copy when the structural `Copy` capability permits it. The compiler controls `Copy` (also known as `Share`); you cannot make an arbitrary resource copyable by adding a user instance.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  next :: forall region. Ref region Nat -> Nat
  next value = copyOut value + 1

  main :: Int
  main = if next (borrow 41) == 42 then 0 else 1
```

The borrow is used within its valid region. It cannot be returned from a scope that owns the underlying value if doing so would let the reference outlive that value. Mutable borrows also obey uniqueness checks. See [ownership reference](../reference/10-ownership-and-memory.md) for projection places, closure captures, and escape rules.

# Mutation invalidates evidence

A proof about the contents of mutable state is only valid while those contents remain appropriate. A write invalidates facts whose footprints overlap that state. Retaining a proof value is not permission to apply it after the fact it described has changed. This becomes important when proofs justify bounds checks or temporal contracts.

# Exercise and answer

For `consume :: Nat %1 -> Nat`, is `consume x = if True then x else 0` a sound general pattern for linear resources?

<details><summary>Answer</summary>

Write both branches to consume the resource according to its protocol. More generally the usage rule is path-sensitive and requires exactly one use on every admitted path; a branch that discards the resource is the problem. Avoid using a constant condition to disguise an invalid resource protocol.

</details>

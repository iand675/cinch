# 9. Signatures, instances, and reusable operations

A generic function cannot add arbitrary values merely because its type contains a variable. It needs a promise that an addition operation exists for that type. In Cinch, a `signature` declares such a vocabulary, and an `instance` supplies an implementation. Constraints make the required vocabulary explicit.

# Define a small vocabulary

<!-- cinch:run exit=0 -->
```cinch
module Main where
  signature Score a where
    score :: a -> Nat

  data Ticket where
    Ticket :: Nat -> Ticket

  implicit instance Score_Ticket :: Score Ticket where
    score t = case t of
      Ticket n -> n * 10

  points :: forall a. using (s :: Score a) => a -> Nat
  points value = score value

  main :: Int
  main = if points (Ticket 4) == 40 then 0 else 1
```

`Score a` declares `score`. `Score_Ticket` implements it for `Ticket`. The `implicit` keyword allows the compiler to resolve this dictionary when a caller requires `Score Ticket`. It must find a unique applicable implicit instance; competing instances are an ambiguity, not a last-import-wins choice.

The binder `s` names the dictionary required by `points`. Calls to methods elaborate into dictionary projections and applications. This is runtime dictionary passing, not an instruction to guess an implementation based on the representation of the argument.

# Use library constraints

Arithmetic and comparisons are library methods too. A function using `+` needs an `Add` dictionary; a comparison needs the appropriate `Eq` or `Ord` dictionary. A function using both addition and multiplication states both constraints. The current compiler does not infer superclass entailment from one class to another.

**Current language rule — explicit constraints.** A class constraint does not entail another class. A `using` context must name each class the body uses; superclass entailment is not a promised feature. See [current support](../reference/16-support-and-roadmap.md) for the boundary and relevant design sources.

`Category.Functor`, `Category.Applicative`, `Category.Monad`, `Category.Foldable`, and `Category.Traversable` provide higher-kinded abstractions. Import the relevant module when you use its vocabulary. For an unfamiliar operator such as `<$>`, follow the method signature rather than assuming it is compiler syntax.

<span class="support-tag support-tag--planned">Coming soon</span> **Associated types and richer dependent module interfaces.** The specification allows a signature to describe an operation whose result family depends on the implementation and input, such as filtering a vector into an existential-length result while filtering a list into another list. The current dictionary signature system does not implement that complete associated-type surface. Use the explicit concrete result types taught in this book today; [the module guide](../reference/07-modules-and-packages.md) shows the intended declaration fragment and its limits.

# Module parameters

A module can take a signature dictionary as a parameter: `module Tally (S :: Stepper) where`. Importing `Tally (StepUp) as Up` supplies a particular implementation. Another import may instantiate the same module with `StepDown`. Value parameters are also supported. This is useful when a module should depend on a selected policy rather than the globally available implicit instance.

# Laws are claims with a status

Signatures can state laws. Some primitive law checks use finite samples; a successful sample check is not a universal proof. Manifests distinguish checked evidence, assumptions, and unsupported checks. Before relying on a law to justify a rewrite, read [laws and optimization](../reference/12-rewrites-and-optimization.md).

# Exercise and answer

Add a `Badge` type with a `Score` instance returning a fixed 100 points. Does `points` need to change?

<details><summary>Answer</summary>

Declare `data Badge where Badge :: Badge` and an implicit `Score Badge` instance whose `score b = 100`. `points` stays unchanged because it relies only on the signature's operation. `points Badge` selects the new dictionary.

</details>

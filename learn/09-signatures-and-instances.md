# 9. Signatures, instances, and reusable operations

A function cannot add arbitrary values merely because its type contains a type variable. It needs a promise that an addition operation exists for that type, and it must name that promise in its own type so callers know what to supply. In Cinch a `signature` declares the vocabulary, an `instance` supplies an implementation for one type, and a `using` clause states which vocabulary a definition requires. This chapter writes both pieces, resolves one from the library, and states the rule the checker applies when a constraint is missing.

Check and run each complete program on the page with `$C check Main.cinch` and `$C run Main.cinch`, where `$C` is `compiler/target/debug/cinch`.

# Before you start

You need `data` declarations and `case` from [chapter 4](04-data-and-matching.md), function signatures and application from [chapter 3](03-functions.md), and imports from [chapter 8](08-modules.md). Nothing here depends on effects, so a `check` is enough to follow the chapter.

# Declare the vocabulary

A class declaration in Cinch is spelled `signature`. It names the class, gives its type parameters, and lists one type per method, as a fragment of a module body:

```cinch
  signature Score a where
    score :: a -> Nat
```

The indentation under `signature` is the class body; each line is a method with an ordinary type. `Score a` is a class with one type parameter. A signature declares no constructor and no value: you cannot write `Score 3`, because there is no `Score` value to build, and the compiler reports `cinch: unknown name Score (in Main.cinch)`. What the declaration describes is a dictionary, a record whose fields are the methods.

The generated reference shows that record shape directly. In [Algebra.Monoid](../library/Algebra.Monoid.md) the class renders as `signature Monoid a`, and its nullary method as:

```text
mempty :: forall (a :: Type). using (dict :: Algebra.Monoid.Monoid a) => a
```

The `using (dict :: ...)` on a class method is the dictionary the method needs, and `mempty` takes no argument because the dictionary carries the value itself.

# Write an instance

An instance supplies the methods for one type. It has a name, a head that fixes the type parameters, and a body of method definitions, as in this fragment:

```cinch
  data Ticket where
    Ticket :: Nat -> Ticket

  implicit instance Score_Ticket :: Score Ticket where
    score t = case t of
      Ticket n -> n * 10
```

Read the three parts separately. `Score_Ticket` is the instance's name, a declaration in the module that you can refer to. `Score Ticket` is its head, and it fixes `a` to `Ticket`. The body defines `score` for that head.

The `implicit` keyword marks the instance as a candidate for automatic resolution. An instance without it is not a candidate, so every call site that needs `Score Ticket` fails; the diagnostics table below quotes that message. The one other way to choose an instance is to pass it as a module argument, `import Tally (StepUp) as Up`, which the last section of this chapter covers.

# State the requirement in the type

A generic definition lists the dictionaries it needs in a `using` clause, written in the same module as the class:

```cinch
  points :: forall a. using (s :: Score a) => a -> Nat
  points value = score value
```

The three pieces are independent. `forall a.` binds the type variable. `using (s :: Score a)` requires a dictionary for `Score a` and gives it the binder name `s`. The arrow `a -> Nat` is the function's own type. Inside the body, `score value` elaborates to a projection of the `score` method out of `s`, so the definition is a function that takes a dictionary and returns a function from `a` to `Nat`.

Calls to methods elaborate into dictionary projections and applications. This is runtime dictionary passing: the compiler does not guess an implementation from the representation of the argument.

# A complete program

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Algebra.Monoid
  import Algebra.Semigroup

  signature Score a where
    score :: a -> Nat

  data Ticket where
    Ticket :: Nat -> Ticket

  implicit instance Score_Ticket :: Score Ticket where
    score t = case t of
      Ticket n -> n * 10

  points :: forall a. using (s :: Score a) => a -> Nat
  points value = score value

  total :: forall a. using (m :: Monoid a, sg :: Semigroup a) => List a -> a
  total xs = mconcat xs

  main :: Int
  main =
    let earned = points (Ticket 4)
    in case total (Cons (Sum earned) (Cons (Sum 2) Nil)) of
      Sum n -> if n == 42 then 0 else 1
```

The program runs and exits `0` without printing, because `main` here returns the exit status alone. Printing arrives in [chapter 10](10-effects-and-io.md).

`points (Ticket 4)` is where implicit resolution happens. The call requires `Score Ticket`, so the checker looks for an implicit instance whose head matches that class and type, finds exactly one (`Score_Ticket`), and passes it. `score` then evaluates `Ticket 4` and multiplies by ten, so `earned` is `40`.

`total` needs two dictionaries even though the body names one function. `mconcat` has type `forall a. using (dict :: Monoid a, sa :: Semigroup a) => List a -> a` in the generated interface, so a caller of `total` must supply both, and `total` forwards both. `total (Cons (Sum 40) (Cons (Sum 2) Nil))` combines two `Sum` values with `<>`, and `Sum` wraps `Nat` under addition, so the result is `Sum 42`.

# Resolve an instance from the library

`Sum` and `Product` exist to carry arithmetic monoids for `Nat`, because the bare `Nat` carries none: two candidate monoids live on it, so the newtype chooses. The checked instance table in [Algebra.Monoid](../library/Algebra.Monoid.md) is the authority on names and targets:

| Instance | Target | Resolution |
| --- | --- | --- |
| `Algebra.Monoid.Monoid_Sum` | `Algebra.Monoid.Monoid Algebra.Monoid.Sum` | Implicit |
| `Algebra.Monoid.Monoid_Product` | `Algebra.Monoid.Monoid Algebra.Monoid.Product` | Implicit |
| `Algebra.Monoid.Semigroup_Sum` | `Algebra.Semigroup.Semigroup Algebra.Monoid.Sum` | Implicit |
| `Algebra.Monoid.Monoid_All` | `Algebra.Monoid.Monoid Algebra.Monoid.All` | Implicit |
| `Algebra.Monoid.Monoid_Pair` | `Algebra.Monoid.Monoid (t, u)` | Implicit |

None of these lives in your module. You import the module and the dictionary arrives through the same resolution step as `Score_Ticket`, which is why `Sum earned` needs no dictionary argument at the call site.

[Relation.Eq](../library/Relation.Eq.md) follows the same pattern for equality: `signature Eq a` with methods `==` and `/=`, fixity `infix 3`, and instances that live with their types rather than in the class module. The module's note states the reason the class needs no superclass: a class constraint entails nothing, so every function states the constraints it uses.

# The explicit-constraint rule

A class constraint does not entail another class. `Monoid` does not extend `Semigroup`, so a definition that combines values with `<>` must name `Semigroup` even when it also names `Monoid`, as `total` does above. Leaving it out is not a deduction the checker performs. The surface grammar records the rule among its named rejections as `superclass entailment (a class constraint entails nothing)`.

<!-- cinch:reject contains="no instance for Semigroup a" -->
```cinch
module Main where
  import Algebra.Monoid
  import Algebra.Semigroup

  total :: forall a. using (m :: Monoid a) => List a -> a
  total xs = mconcat xs

  main :: Int
  main = 0
```

```text
cinch: 6:14: no instance for Semigroup a#844 (in Main.cinch)
```

The `#844` is a fresh variable identity that changes between runs; the word to read is the class it names. When the body contains a method, the checker must solve that method's own requirement into the constraint list, and this one is not in the list.

The same rule applies to imports. An operator must be imported from the module that declares its fixity, and importing a module that mentions another's class does not import it. Here the `using` clause is right and the import is missing:

<!-- cinch:reject contains="no fixity declared for the operator" -->
```cinch
module Main where
  import Algebra.Monoid

  combine :: forall a. using (m :: Monoid a, sg :: Semigroup a) => a -> a -> a
  combine x y = x <> y

  main :: Int
  main = 0
```

```text
cinch: Main.cinch: 5:19: there is no fixity declared for the operator `<>`, and there is no default for an undeclared operator
```

This diagnostic puts the file before the position, unlike the others on this page. Add `import Algebra.Semigroup` and the same definition checks. The class module note in [Algebra.Monoid](../library/Algebra.Monoid.md) states the reason: the imported operator keeps the fixity declared by `Algebra.Semigroup`, and the importing module cannot redeclare it.

# What the compiler enforces

These are the rules the checker applies to signatures, instances, and constraints.

| Rule | What it rejects |
| --- | --- |
| resolution needs one applicable instance | two `implicit` instances for the same class and type make a call site ambiguous |
| `implicit` marks candidates | an instance without the keyword is invisible to resolution |
| every used class is in the `using` clause | a body that calls a method whose class the type does not name |
| no superclass entailment | inferring `Semigroup a` from `Monoid a` |
| an operator's fixity comes from its module | `<>` used without importing `Algebra.Semigroup` |

Resolution requires a unique applicable implicit instance. Two competing instances are an ambiguity rather than a last-import-wins choice, and the diagnostic names the class and the type.

# Diagnostics you will meet

Each message below is copied from a real run. Variable identities such as `a#844` are fresh per run, so a different program reports a different suffix.

| Symptom | Message |
| --- | --- |
| the type does not state the class its body uses | `cinch: 13:18: no instance for Score a#844 (in Main.cinch)` |
| no instance exists for the concrete type | `cinch: 12:13: no instance for Score Ticket (in Main.cinch)` |
| the instance exists but lacks `implicit` | `cinch: 15:13: no instance for Score Ticket (in Main.cinch)` |
| two implicit instances match | `cinch: 18:13: ambiguous instances for Score Ticket (in Main.cinch)` |
| a required class is missing from `using` | `cinch: 6:14: no instance for Semigroup a#844 (in Main.cinch)` |
| the operator's module was not imported | `cinch: Main.cinch: 5:19: there is no fixity declared for the operator` followed by the operator name |

The first and second rows have the same wording because the checker reaches the same state from both directions: at that position it needs a dictionary and has none. Reading the line and column tells you which requirement was left unmet.

A class is named by its bare name because this message names only one `Score` and one `Ticket`. Two declarations that share a bare name are both written in full in the same message, so a program with its own `Score` beside this one would read `Main.Score Main.Ticket`. The identity itself is always the qualified `Owner.Name`; only the printed spelling depends on what else the message names.

<!-- cinch:reject contains="ambiguous instances for Score Ticket" -->
```cinch
module Main where
  signature Score a where
    score :: a -> Nat

  data Ticket where
    Ticket :: Nat -> Ticket

  implicit instance Score_TicketA :: Score Ticket where
    score t = 10

  implicit instance Score_TicketB :: Score Ticket where
    score t = 20

  points :: forall a. using (s :: Score a) => a -> Nat
  points value = score value

  main :: Int
  main = if points (Ticket 4) == 10 then 0 else 1
```

Resolution finds two candidates and refuses to choose. Delete one instance, or make the implementations apply to different types, and the program checks again.

# Module parameters

A module can take a dictionary as a parameter instead of relying on implicit resolution: `module Tally (S :: Stepper) where`, and an importing module selects the instance by name with `import Tally (StepUp) as Up`. Two imports of the same module with different arguments produce two units with different behaviour, which the `params` suite in `compiler/tests/params.rs` exercises for both signature and value parameters. Use this shape when a module should depend on a selected policy rather than on whatever instance happens to be in scope.

# Laws are claims with a status

A signature can state laws, and the library records them. The `Monoid` laws are the left and right unit laws with `mempty`, plus associativity of `<>` from `Semigroup`. Some primitive law checks run over finite samples, and a successful sample check is not a universal proof. [Laws and optimization](../reference/12-rewrites-and-optimization.md) describes what evidence a rewrite may rely on.

# Exercises

1. Add `data Badge where Badge :: Badge` with an `implicit` instance of `Score Badge` returning a fixed `100`. Does `points` need to change?
2. Write `combine :: forall a. using (m :: Monoid a, sg :: Semigroup a) => List a -> a` for `String` using the library's `Monoid String` instance, and say which constraint you could not drop.

<details><summary>Answer to exercise 1</summary>

No. `points` is polymorphic over the dictionary and mentions only the `score` method, so a new instance for a new type leaves it untouched. You write the instance and call `points Badge`; resolution selects the new dictionary. This is the reason to put the operation in a signature rather than to write a specialised function per type.

</details>

<details><summary>Answer to exercise 2</summary>

The definition is `combine xs = mconcat xs`, and both constraints stay, because `mconcat`'s own scheme requires `Monoid a` and `Semigroup a`. Import `Algebra.Monoid` for `mconcat` and the `Monoid String` instance, and `Algebra.Semigroup` for `<>`'s fixity. Dropping `Semigroup a` fails with `no instance for Semigroup a`. Dropping the `Algebra.Semigroup` import fails with the fixity message, even though the class constraint is present.

</details>

Continue with [chapter 10](10-effects-and-io.md), which adds output to a program by pairing an effect row with a capability.

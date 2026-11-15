# Types, kinds, declarations, and inference

Types, kinds, and values share one term syntax, so `Vec (n + 1) a` and `(x :: a) -> a` are elaborated rather than parsed by a second restricted grammar. This page gives the exact spellings the parser accepts, the declaration forms, and the inference rules whose limits you meet when reading a diagnostic. Rationale is in [surface syntax and grammar](../specifications/surface-syntax-and-grammar.md) and [core language](../specifications/core-language-specification.md).

# Function arrows

An arrow is `->` followed by an optional effect row and preceded by an optional multiplicity.

| Spelling | Meaning |
| --- | --- |
| `A -> B` | Pure arrow. The closed empty row. |
| `A ->{} B` | The same closed empty row, written out. |
| `A ->{Poke, Diverge} B` | Closed row holding exactly the labels between the braces. |
| `A ->{IO r \| e} B` | Labels first, then `\|`, then one row variable. |
| `A %0 -> B` | The argument is never used and is erased. |
| `A %1 -> B` | The argument is used exactly once on every path. |
| `A %aff -> B` | The argument is used at most once. |
| `A %many -> B` | The argument is unrestricted. This is the default when no multiplicity is written. |

Arrows associate right, application associates left, and a curried definition carries each application step's effects on the arrow that produces the next function. `f :: Int -> Int ->{Poke} Int` performs `Poke` when given the second argument and is pure when given only the first. The row grammar, the label rules, and higher-order row variables are in [effects and capabilities](09-effects-and-capabilities.md); a printed arrow shows the multiplicity inline, as in `(?a %many ->{?r} ?b)`.

Multiplicity violations are reported against the binder name and the count:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  keep :: forall a. a %1 -> a
  keep x = x

  drop' :: forall a. a %0 -> Nat
  drop' x = 0

  affine :: forall a. a %aff -> Nat
  affine x = 1

  many :: forall a. a %many -> (a, a)
  many x = (x, x)

  main :: Int
  main = case many (keep 1) of
    (a, b) -> if a == b then 0 else 1
```

<!-- cinch:reject contains="must be used exactly once on every path" -->
```cinch
module Main where
  consume :: forall a. a %1 -> Unit
  consume x = ()

  main :: Int
  main = 0
```

<!-- cinch:reject contains="is erased and must never be used" -->
```cinch
module Main where
  oops :: forall a. a %0 -> Nat
  oops x = case x of
    y -> 0

  main :: Int
  main = 0
```

The three messages are `multiplicity %1: x must be used exactly once on every path, but is used 2 time(s)`, `multiplicity %1: x must be used exactly once on every path, but is never used`, and `multiplicity %0: x is erased and must never be used, but is used 1 time(s)`.

# Type application and type variables

Apply a type constructor by juxtaposition: `List a`, `Result ConfigError Bytes`, `Vec n a`, `Io r`, `Scope r s`. A partially applied constructor is refused at the point where a full type is required, and over-application is refused too:

```text
cinch: 6:10: type Main.Pair must be fully applied (expects 1 arguments)
cinch: 6:10: type Main.Pair expects at most 1 arguments, got 2
```

Tuples are structural: `(a, b)` is a pair type, `()` is `Unit`, and a three-field product needs a record or a data declaration rather than `(a, b, c)`.

Two different things are both called a variable by readers, and the checker keeps them apart.

| Term | What it is |
| --- | --- |
| A parameter, as in `data Pair (a :: Type)` | Declared by the type's own binder list. It is part of the constructor's identity, and `Pair` has arity 1. |
| A type variable, as in `forall a. a -> a` | Quantified at one signature or at one constructor type. Every use instantiates it afresh. |

A signature may not use an unquantified variable. The checker reports it by name:

```text
cinch: 3:8: unknown type variable a
```

Row and region variables are variables of their own kinds and need their own quantifiers; `unknown row variable e` and `unknown region variable region` are the messages when a signature mentions one that no binder introduces.

# Kinds and universe levels

`Type` is stratified: `Type 0` is the universe of ordinary types, `Type 1` holds `Type 0`, and `Type : Type` is never admitted. `Prop` is the separate universe of propositions; its inhabitants are erased at runtime, and [dependent types](08-dependent-types-and-proofs.md) documents the proofs built there. `Nat` is the kind of index values, and `Row` is the kind of effect rows.

The REPL reports the kind of a source declaration:

```text
Pair :: (Type) -> Type
Tagged :: (Nat) -> Type
Wrap :: (Type -> Type) -> (Type) -> Type
Handle :: Type
```

`Wrap`'s first parameter has kind `Type -> Type`, so a higher-kinded type may be passed to it. The checker refuses a universe above the one a parameter expects:

<!-- cinch:reject contains="`Type : Type` is not admitted" -->
```cinch
module Main where
  data Box (a :: Type) where
    Box :: a -> Box a

  f :: Box Type -> Nat
  f b = 0

  main :: Int
  main = 0
```

Kind errors have five shapes you will meet:

| Mistake | Message |
| --- | --- |
| `Type` passed where `Type 0` is expected | ``the universe `Type 0` has kind `Type 1`: `Type : Type` is not admitted`` |
| A numeral written in a type position | ``a numeral is an index value, not a type; index values appear as arguments of an indexed family (as in `Vec 0 a`)`` |
| One variable used at both a type and an index position | `` `a` is used both as a type and as an index value; annotate it `` |
| A value position used as a type, as in `Depth Bool` | `unknown name Bool in index position` |
| A type declared but not applied | `type Main.Pair must be fully applied (expects 1 arguments)` |

# Quantifiers, dependent binders, and contexts

`forall` binds implicit parameters. Each binder may carry a kind annotation, and a binder of kind `Nat` is mentioned by later binders and by index positions:

| Form | Meaning |
| --- | --- |
| `forall a. ...` | Unannotated type variable. |
| `forall (a :: Type). ...` | Type variable with an explicit kind. |
| `forall (n :: Nat). ...` | Index variable, usable in `Vec n` and in dependent binders. |
| `forall (e :: Row). ...` | Row variable. |
| `forall (p :: Prop). ...` | Proposition variable. |
| `(n :: Nat) -> Vec n a` | Dependent function: the first argument is a value the result type mentions. |
| `Sigma (n :: Nat). Vec n a` | Dependent pair. |
| `using (d :: Eq a) => ...` | A class dictionary the caller supplies. |
| `using (i < n) => ...` | A proof obligation the caller discharges. |

Supplying an obligation explicitly is `f @{io = newIO ()}` at a call, or `with (io = newIO ()) in body` for a lexical scope. A class dictionary is resolved from instances instead; naming a non-capability parameter in a supply is refused with `twice has no capability parameter add`.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  pair :: forall (n :: Nat). Vec n Int -> Sigma (m :: Nat). Vec m Int
  pair xs = (n, xs)

  main :: Int
  main = case pair (Cons 1 (Cons 2 Nil)) of
    (m, v) -> if m == 2 then 0 else 1
```

An obligation carried by `using` is checked at each call, and the report names the instantiated proposition:

<!-- cinch:reject contains="of this call is false" -->
```cinch
module Main where
  at :: using (i < n) => (i :: Nat) -> (n :: Nat) -> Nat
  at i n = i

  main :: Int
  main = at 5 3
```

# Polymorphic arguments and results

A `forall` may appear inside a type, not only at the start of a signature. `(forall a. a -> a) -> (I64, String) -> (I64, String)` takes an argument that is itself polymorphic, so the body may use it at more than one type. The callee chooses the types, not the caller:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  both :: (forall a. a -> a) -> (I64, String) -> (I64, String)
  both f p = case p of (n, s) -> (f n, f s)

  main :: Int
  main = case both (\x -> x) (7, "seven") of
    (n, s) -> if n == 7 && string-length s == 5 then 0 else 1
```

A constructor field may hold a polymorphic value, and a function may return one:

| Form | Meaning |
| --- | --- |
| `(forall a. a -> a) -> Int` | A rank-2 argument: the body may apply it at several types. |
| `((forall a. a -> a) -> Int) -> Int` | Rank 3: the argument itself takes a polymorphic argument. |
| `Church :: (forall a. (a -> a) -> a -> a) -> Church` | A polymorphic constructor field. |
| `Bool -> (forall a. a -> a -> a)` | A polymorphic result, instantiated where it is applied. |
| `(forall f. f I64 -> f I64) -> ...` | A nested binder may be a type constructor, an effect row or a region. |

The rules are predicative. A term checked against a polymorphic type is checked at fresh rigid type variables, so `\x -> x` is accepted and a function of `I64 -> I64` is not. Those variables may not escape into a type outside the term. Building the value performs no effects, because it is built once for each representation its uses need. A type variable never stands for a polymorphic type, so `ident both` is refused, and a nested `forall` cannot carry `using` constraints. Two polymorphic types are the same when they differ only in the names of their binders.

<!-- cinch:reject contains="type mismatch: expected I64, got a" -->
```cinch
module Main where
  run :: (forall a. a -> a) -> I64
  run f = f 1

  inc :: I64 -> I64
  inc n = n

  main :: Int
  main = int-from-i64 (run inc)
```

A polymorphic value is a block of closures, one for each word representation its uses reach, so passing one costs a pointer, and a use is one indirect call.

# Declaration forms

## Data declarations

`data Name binders where` lists constructor signatures, one `::` each. Constructors are the only way to build the type, and each constructor records its argument count.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Shape where
    Circle :: Nat -> Shape
    Rect :: Nat -> Nat -> Shape

  area :: Shape -> Nat
  area s = case s of
    Circle r -> 3 * r * r
    Rect w h -> w * h

  main :: Int
  main = case area (Rect 2 3) of
    0 -> 1
    _ -> 0
```

## Indexed families

A parameter of kind `Nat` makes the family indexed. The constructor's result type states the index it builds, and a constructor may bind index variables that its fields mention.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Nat2 (n :: Nat) where
    Z :: Nat2 0
    S :: forall k. Nat2 k -> Nat2 (k + 1)

  two :: Nat2 2
  two = S (S Z)

  depth :: Nat2 2 -> Nat
  depth v = case v of
    S (S Z) -> 2
    _ -> 0

  main :: Int
  main = case depth two of
    2 -> 0
    _ -> 1
```

Index equations are solved on numerals and by conversion on definitionally equal terms. A case over a rigid index does not refine it, and the checker says so:

```text
cinch: 9:5: cannot match constructor Z against Main.Nat2 @value[4056]: the index equation 0 vs @value[4056] has no solution here (this slice does not refine rigid indices; reshape the type or dispatch through `decide`)
```

Building a value at a rigid index is refused the same way, with `constructor Nil cannot build this value: index mismatch: 0 vs @value[4056]`.

## Record declarations

`record Name where` lists named fields. Construction is `Name { field = value, ... }`, projection is `value.field`, and a pattern may name the fields it binds. There is no record-update syntax: rebuilding the record is the way to change one field, and a stray brace after an expression is refused with `record construction must place `R { x = e, ... }` directly on a constructor name`.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Point where
    x :: Int
    y :: Int

  origin :: Point
  origin = Point { x = 0, y = 3 }

  shift :: Int -> Point -> Point
  shift d p = Point { x = p.x + d, y = p.y }

  main :: Int
  main = case shift 2 origin of
    Point { x = 2, y = 3 } -> 0
    _ -> 1
```

## Refinement and opaque type declarations

`type` does not declare a synonym. It declares a refinement, and the only accepted right-hand side is `refine (x :: T) with P`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  type Positive = refine (n :: Nat) with 0 < n

  name :: Positive -> Nat
  name p = unrefine p

  main :: Int
  main = case name 4 of
    4 -> 0
    _ -> 1
```

A value that does not satisfy the predicate is refused when it is constructed: `integer literal 0 is not a Positive: its predicate `0 < 0` is false (a refined value needs its predicate to hold)`. Refinement machinery is in [refinements](06-refinements.md).

Aliasing a type constructor is not available, and the attempt names the two declaration forms that do exist:

<!-- cinch:reject contains="a `type` declaration is a refinement" -->
```cinch
module Main where
  type Pairs a = List (a, a)

  main :: Int
  main = 0
```

The full message is ``expected `refine (x :: T) with p` (a `type` declaration is a refinement; `opaque type T :: Type` declares an abstract foreign resource), found constructor `List` ``. A kind signature on a refinement is likewise refused with `kind signatures on refinement declarations are not supported in the bootstrap slice`.

`opaque type Name :: Type` declares an abstract type whose values come from a foreign boundary. It takes no parameters and no right-hand side.

# Inference and its limits

The checker synthesizes a type where it can and checks the expression against an expected type where it cannot. Unification handles occurs checking and structural rows. These limits are the ones you will read in a diagnostic.

**Top-level definitions need signatures.** `missing signature for f (this draft does not silently infer top-level types)`. A definition with no signature is not inferred and then generalized.

**Local value bindings generalize; computations do not.** A `let`-bound function is polymorphic at each later use, so one local identity is usable at more than one type:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let id' = \x -> x
    in if id' True then
         case id' (5 :: I32) of
           n -> if n == 5 then 0 else 1
       else 2
```

A local binding whose right-hand side is an application is not a value, so its pending unification variables stay unsolved and the next use reports them.

**An unsolved implicit is an error, not a default.** A literal whose type no constraint fixes reports `the type of this integer literal is not determined by context; add a type annotation`. A dictionary no instance satisfies reports the wanted class with its unsolved argument:

<!-- cinch:reject contains="no instance for Eq (List ?m" -->
```cinch
module Main where
  main :: Int
  main = if Nil == Nil then 0 else 1
```

The suffix `(a literal's type is not determined by context; add a type annotation)` says what the unresolved variable is waiting for.

**Occurs checking is fatal.** A self-application is refused rather than accepted at an infinite type: `cannot construct infinite type: ?m2017 in (?m2017 %many ->{?r897} ?m2018)`.

**Coverage is checked.** An omitted constructor is an error, and a guarded row does not cover:

```text
cinch: 4:10: non-exhaustive patterns: a value other than 0, 1 is not covered by an unguarded row (a guarded row covers nothing, since its guard can fail); add an unguarded final row such as `_ -> ...`
```

**Effect rows must agree.** A pure declaration whose body performs a label is refused with `effect rows do not match: {} vs {IO r#845 | ?r894}`; [effects and capabilities](09-effects-and-capabilities.md) documents the row calculus.

**Local recursion is not available.** A `let` group is sequential, so `let f x = f x in ...` reports `unknown name f`; move the recursion to a top-level definition.

# Proposition variables and decisions

A `Prop`-kinded binder may be quantified over, and `decide` produces the `Dec` value that tells you which side holds. `Dec`, `Yes`, and `No` are builtin, and the evidence fields are erased. [Compiler-provided vocabulary](20-compiler-vocabulary.md) lists their schemes.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  pick :: forall (p :: Prop). Dec p -> Bool
  pick d = case d of
    Yes h -> True
    No h -> False

  main :: Int
  main = case pick (decide (1 < 2)) of
    True -> 0
    False -> 1
```

# Interface contracts

A compiled `.cinchif` records each scheme's quantifier kinds, the nominal identity of every declared type, constructor schemes, the private nominal and class closure a caller may not reinterpret, method visibility, and admitted contexts. A caller that declares an unrelated local type with the same short name does not thereby obtain the private type.

Implementation: `compiler/src/types.rs`, `check.rs`, `check_decls.rs`, `check_terms.rs`, `check_interface.rs`; suites: `hk.rs`, `dependent.rs`, `interface_kinds.rs`, `interface_nominals.rs`, `interface_class_contracts.rs`, `interface_region_effects.rs`, `bootstrap_infer.rs`.

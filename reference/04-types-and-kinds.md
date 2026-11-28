# Types, kinds, declarations, and inference

Cinch shares a single term syntax across values, types, and kinds: indexed types such as `Vec (n + 1) a` and dependent arrows such as `(x :: a) -> a` are elaborated from one grammar rather than parsed by a separate type-only language. This chapter specifies function arrows, type application, universes and kinds, quantifiers, higher-rank polymorphism, type declarations, inference boundaries, and module interface contracts.

# Function arrows, effects, and multiplicities

A function arrow `->` carries an optional leading multiplicity and an optional trailing effect row. Omitting the multiplicity defaults to `%many` (unrestricted); omitting the effect row defaults to `{}` (pure).

| Syntax | Meaning |
| --- | --- |
| `A -> B` | Pure function (closed empty effect row `{}`). |
| `A ->{} B` | Explicit closed empty effect row. |
| `A ->{Poke, Diverge} B` | Closed effect row containing exactly the listed effects. |
| `A ->{IO r \| e} B` | Open effect row: explicit labels followed by `\|` and one row variable. |
| `A %0 -> B` | Erased argument: never used computationally; absent at runtime. |
| `A %1 -> B` | Linear argument: used exactly once on every execution path. |
| `A %aff -> B` | Affine argument: used at most once on every execution path. |
| `A %many -> B` | Unrestricted argument (default). |

Function arrows associate to the right (`A -> B -> C` is `A -> (B -> C)`), and curried functions attach effects to the specific arrow whose application performs them. A function of type `Int -> Int ->{Poke} Int` is pure when applied to one argument and performs `Poke` only upon receiving the second. Diagnostics print multiplicities inline, e.g. `(?a %many ->{?r} ?b)`. See [effects and capabilities](09-effects-and-capabilities.md) for row polymorphism and [ownership and memory](10-ownership-and-memory.md) for linearity and borrows.

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

Violating a `%1` or `%0` multiplicity is a compile-time error reporting the binder and exact usage count:

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

# Type application, tuples, and variables

Type constructors are applied by juxtaposition, associating to the left: `List a`, `Result ConfigError Bytes`, `Vec n a`, `Io r`, `Scope r s`. Where a saturated type of kind `Type` is required, under-applying or over-applying a constructor is rejected:

```text
cinch: 6:10: type Main.Pair must be fully applied (expects 1 arguments)
cinch: 6:10: type Main.Pair expects at most 1 arguments, got 2
```

Tuples are structural n-ary products: `()` is `Unit`, `(a, b)` is a 2-tuple, `(a, b, c)` is a 3-tuple, and so on. Tuples unify componentwise and require matching arities. Use a `record` or `data` declaration when you want named fields or nominal identity.

Cinch distinguishes two kinds of type-level names:

| Construct | Role |
| --- | --- |
| Declaration parameter (`data Pair (a :: Type)`) | Fixed part of the type constructor's signature and arity. |
| Quantified type variable (`forall a. a -> a`) | Bound by a signature or constructor quantifier; instantiated fresh at each use. |

Every type, row, and region variable appearing in a signature must be explicitly quantified. Unbound variables are rejected by kind (`unknown type variable a`, `unknown row variable e`, `unknown region variable region`).

# Kinds and universe levels

Cinch uses a stratified universe hierarchy with distinct kinds for propositions, indices, effect rows, and regions:

- `Type 0` (written `Type`) is the universe of ordinary value types; `Type 1` classifies `Type 0`, and so on. `Type : Type` is never admitted.
- `Prop` is the universe of propositions. Its inhabitants are erased before code generation (see [dependent types and proofs](08-dependent-types-and-proofs.md)).
- `Nat` is the kind of compile-time numeric indices (such as `Vec n a`).
- `Row` is the kind of effect rows.
- `Region` is the kind of runtime capability identities. Quantifying `forall (r :: Region). ...` allows a pure factory to capture or return an effectful handle without executing effects while preserving its identity across `using (io :: Io r)` constraints. Regions cannot be used as value types, numeric indices, or effect-row tails, and distinct quantified regions cannot be unified.

Type declarations may also take higher-kinded parameters. For example, a declaration `Wrap` taking a unary type constructor has kind `(Type -> Type) -> Type -> Type`, which can be inspected in the REPL:

```text
Pair :: (Type) -> Type
Tagged :: (Nat) -> Type
Wrap :: (Type -> Type) -> (Type) -> Type
Handle :: Type
```

Passing a universe where a `Type 0` argument is expected violates stratification:

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

Common kind diagnostics:

| Error | Diagnostic |
| --- | --- |
| Universe passed as `Type 0` | ``the universe `Type 0` has kind `Type 1`: `Type : Type` is not admitted`` |
| Numeral in type position | ``a numeral is an index value, not a type; index values appear as arguments of an indexed family (as in `Vec 0 a`)`` |
| Variable used as both type and index | `` `a` is used both as a type and as an index value; annotate it `` |
| Type name in index position | `unknown name Bool in index position` |
| Unapplied constructor in type position | `type Main.Pair must be fully applied (expects 1 arguments)` |

# Quantifiers, dependent binders, and contexts

`forall` introduces implicit parameters; dependent arrows `(x :: T) -> U` introduce explicit parameters that later types may reference; `using (...) => ...` introduces type class dictionaries, capabilities, or proof obligations.

| Form | Meaning |
| --- | --- |
| `forall a. ...` | Implicit type variable. |
| `forall (a :: Type). ...` | Implicit type variable with explicit kind. |
| `forall (n :: Nat). ...` | Implicit index variable usable in types and dependent binders. |
| `forall (e :: Row). ...` | Implicit effect-row variable. |
| `forall (p :: Prop). ...` | Implicit proposition variable. |
| `forall (r :: Region). ...` | Implicit region identity variable. |
| `(n :: Nat) -> Vec n a` | Dependent function whose return type mentions the argument `n`. |
| `Sigma (n :: Nat). Vec n a` | Dependent pair pairing a witness `n` with a `Vec n a`. |
| `using (d :: Eq a) => ...` | Type class constraint resolved from instances. |
| `using (io :: Io r) => ...` | Capability requirement supplied explicitly or lexically. |
| `using (i < n) => ...` | Proof obligation discharged at each call site. |

Capabilities are supplied at a call with `f @{io = newIO ()}` or over a lexical scope with `with (io = newIO ()) in body`. Class dictionaries are resolved automatically from instances; supplying a non-capability parameter via `@{...}` is rejected.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  pair :: forall (n :: Nat). Vec n Int -> Sigma (m :: Nat). Vec m Int
  pair xs = (n, xs)

  main :: Int
  main = case pair (Cons 1 (Cons 2 Nil)) of
    (m, v) -> if m == 2 then 0 else 1
```

A `using` proof obligation is checked at every call site with the instantiated arguments:

<!-- cinch:reject contains="of this call is false" -->
```cinch
module Main where
  at :: using (i < n) => (i :: Nat) -> (n :: Nat) -> Nat
  at i n = i

  main :: Int
  main = at 5 3
```

# Polymorphic arguments and results

`forall` is not restricted to top-level prenex position. Placing `forall` inside a parameter type, constructor field, or return type gives higher-rank polymorphism: the callee chooses the instantiation types rather than the caller.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  both :: (forall a. a -> a) -> (I64, String) -> (I64, String)
  both f p = case p of (n, s) -> (f n, f s)

  main :: Int
  main = case both (\x -> x) (7, "seven") of
    (n, s) -> if n == 7 && string-length s == 5 then 0 else 1
```

| Form | Meaning |
| --- | --- |
| `(forall a. a -> a) -> Int` | Rank-2 argument usable at multiple types inside the body. |
| `((forall a. a -> a) -> Int) -> Int` | Rank-3 argument taking a rank-2 function. |
| `(using (r :: Read a) => Unit ->{\|e} b) ->{\|e} b` | Higher-rank capability-constrained callback (introduced by `with` inside the callee). |
| `Church :: (forall a. (a -> a) -> a -> a) -> Church` | Constructor field holding a polymorphic function. |
| `Bool -> (forall a. a -> a -> a)` | Polymorphic result instantiated where applied. |
| `(forall f. f I64 -> f I64) -> ...` | Higher-kinded, row, or region nested quantifier. |

Higher-rank checking is predicative:

- Checking a term against a `forall` type introduces fresh rigid type variables (skolems), which cannot unify with concrete types or escape into outer scopes.
- Constructing a polymorphic value performs no effects.
- Type variables cannot be instantiated with polymorphic types (impredicative instantiation such as passing `both` to `id` is rejected).
- Higher-rank `using (...) =>` constraints are supported in argument positions for lexical capabilities (such as `Effect.Reader`'s `local` and `with-reader`); nested `forall` blocks themselves do not carry class-dictionary `using` constraints.
- Two polymorphic types are equal up to alpha-renaming of their bound variables.

Passing a monomorphic function where a polymorphic argument is expected is rejected:

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

At runtime, a first-class polymorphic value is represented as a pointer to a block of closures specialized for each word representation its uses reach, so passing one costs a single pointer and calling it costs one indirect call.

# Declaration forms

## Data declarations

Algebraic data types are declared with `data Name params where` and explicit constructor signatures:

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

A parameter of kind `Nat` defines an indexed data family. Each constructor specifies the exact index it constructs and may quantify additional index variables referenced by its arguments:

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

Index equations are solved on numerals and definitionally equal terms. Matching a constructor whose index constrains an abstract (rigid) index variable does not refine that rigid index inside the branch; reshape the type or branch through `decide` (see [dependent types and proofs](08-dependent-types-and-proofs.md)).

## Record declarations

Records are declared with `record Name where` and named `field :: Type` entries. Values are built with `Name { field = value, ... }` and projected with `value.field`. There is no functional record-update syntax; construct a new record explicitly when changing fields.

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

The `type` keyword declares a refinement type rather than a type synonym. Its right-hand side must have the form `refine (x :: T) with P` (see [refinements](06-refinements.md)):

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

Refinement declarations do not accept kind signatures, and bare type aliases are rejected:

<!-- cinch:reject contains="a `type` declaration is a refinement" -->
```cinch
module Main where
  type Pairs a = List (a, a)

  main :: Int
  main = 0
```

`opaque type Name :: Type` declares a parameterless abstract type whose values originate across a foreign boundary (see [FFI, representation, and inline primitives](15-ffi-layout-and-attributes.md)).

# Type inference and its limits

Type checking is bidirectional: the compiler synthesizes types bottom-up where unambiguous and checks against expected types top-down where needed. Unification enforces occurs checks and structural effect-row constraints. The following boundaries are intentional:

- **Top-level signatures are mandatory.** Every top-level binding must carry an explicit `::` signature; the compiler does not silently infer or generalize top-level definitions.
- **Value restriction on local bindings.** Syntactic values bound with `let` (such as lambdas) generalize and may be instantiated at multiple types; non-value applications do not generalize, leaving their unification variables monomorphic.

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

- **No silent defaulting of ambiguous types.** Unconstrained type variables and ambiguous integer literals are errors rather than silently defaulted; add a type annotation to fix the type:

<!-- cinch:reject contains="no instance for Eq (List ?m" -->
```cinch
module Main where
  main :: Int
  main = if Nil == Nil then 0 else 1
```

- **Occurs check rejects infinite types.** Self-application (`\x -> x x`) fails rather than synthesizing a recursive type.
- **Pattern coverage is enforced.** Every reachable constructor or domain requires an unguarded row; guarded rows contribute zero coverage.
- **Effect rows must agree.** Performing an effect not permitted by a function's arrow row is a type error (`effect rows do not match`).
- **Local recursion requires `letrec`.** Plain `let` is strictly sequential and non-recursive (`let f x = f x in ...` fails with `unknown name f`). Use `letrec` for recursive or mutually recursive local bindings (see [expressions and patterns](03-expressions-and-patterns.md)), or lift the binding to top level.

# Proposition variables and decisions

Signatures may quantify over `Prop` variables, and `decide` returns a builtin `Dec p` (`Yes` or `No`) whose evidence field is erased at runtime. See [compiler-provided vocabulary](20-compiler-vocabulary.md) for the builtin schemes.

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

# Module interface contracts

A compiled `.cinchif` interface records every exported scheme's quantifier kinds, nominal type identities, constructor schemes, private type and class closures, method visibility, and admitted contexts. Nominal identity is module-qualified: declaring a local type with the same short name as a private exported type never aliases or exposes it.

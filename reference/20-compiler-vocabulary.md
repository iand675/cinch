# Compiler-provided vocabulary and special forms

Most types and functions in Cinch are ordinary declarations in the standard library (indexed in the [library reference](../library/00-index.md)). A smaller core surface is installed directly by the typechecker rather than declared in a `.cinch` file.

This surface consists of two distinct mechanisms:

- **Installed types and values** are real bindings with known schemes (registered under a bare name, a qualified name, or both). They cannot be redeclared by user code.
- **Special forms** have no first-class value binding; the checker elaborates their applications directly by syntax, so they must be applied to their required arguments rather than referenced bare.

# Installed types

Installed types are in scope before any module source is parsed:

| Type | Kind arity | Role |
| --- | --- | --- |
| `Nat`, `Int`, `Bool`, `Unit`, `Bytes`, `Sym` | 0 | Core value types. `Nat` and `Int` are arbitrary-precision integers with an immediate fast path; `Bytes` and `Sym` are counted handles. |
| `Char`, `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`, `F32`, `F64`, `Usize`, `Isize` | 0 | The thirteen fixed-width machine scalars (see [Chapter 05](05-numbers-and-primitives.md)). |
| `Array`, `Tensor`, `Columnar` | 1 | Opaque runtime containers whose shapes and column layouts cannot be forged by user constructors. |
| `Io` | 1 | Region-indexed I/O capability token (`Io r`). Represented as a uniform word; capability identity lives in the type. |
| `Scope`, `Task` | 2, 3 | Structured concurrency handles: `Scope r s` and linear `Task s x a`. |
| `Code`, `SyntaxBinding` | 1 | Compile-time-only typed quotations and hygienic syntax bindings. |
| `Syntax`, `TypeSyntax`, `PackedCode`, `Declarations`, `Name`, `Secret`, `IoCap`, `FileReadCap`, `FileWriteCap`, `ProcCap`, `SchemaSnapshot`, `Database`, `SchemaRead` | 0 | Compile-time-only staging types. None of these ever appear in runtime code. |
| `Vec` | 2 | Built-in length-indexed vector family: `Nil :: Vec 0 a`, `Cons :: a -> Vec k a -> Vec (k + 1) a`. |
| `Fin` | 1 | Built-in bounded natural family: `Fin :: (i :: Nat) -> (bound :: i < n) -> Fin n` (`bound` is erased). |
| `Dec` | 1 | Built-in decidability family: `Yes :: (h :: p) -> Dec p`, `No :: Not p -> Dec p` (both evidence fields are erased). |

`Vec`, `Fin`, and `Dec` are built-in because their runtime helpers rely on internal traversals and because `Vec`'s `Nil`/`Cons` constructors coexist with `Data.List`. User programs can declare their own indexed families with the same `data` syntax (see [Chapter 04](04-types-and-kinds.md)).

When a `.wit` adapter is generated, the compiler additionally synthesizes `WasiStream`, `WasiFuture`, `StreamError`, and `BoundaryError` in `Prelude.Generated` rather than registering them unconditionally as built-ins.

# Installed values

## Core values

| Name | Arity | Scheme | Effect row | Notes |
| --- | --- | --- | --- | --- |
| `True`, `False` | 0 | `Bool` | pure | Boolean constructors. |
| `newIO` (`Runtime.newIO`) | 1 | `Unit -> Io r` | pure | Mints a fresh `Io r` capability token with a fresh region identity. |
| `eval` | 1 | `forall a. Code a -> a` | pure | Evaluates a typed quotation. Requires a target profile with `allow_eval` (see [targets and components](19-targets-and-components.md)). |

## Index and evidence values

| Name | Arity | Scheme | Notes |
| --- | --- | --- | --- |
| `toNat` | 1 | `U32 -> Nat` | Total, executable conversion usable inside refinement predicates. |
| `length` | 1 | `Bytes -> Nat` | Installed bare, but shadowed by Prelude's `Data.List.length :: forall a. List a -> Int`. |
| `vecStreamStep` | 1 | `Ref region (Vec n a) -> Step (Ref region (Vec n a)) a` | Producer step over a borrowed length-indexed vector. |
| `refl` | 0 | `Eq x x` | Erased reflexivity witness. |
| `congr` | 3 | `Eq a b -> Eq (f a) (f b)` | Erased congruence witness lifting equality across a total function. |
| `Vec.getCopy` | 2 | `forall region n a. using (copy :: Copy a) => Ref region (Vec n a) -> Fin n -> a` | Qualified only; no bare `getCopy` binding exists. |

Referencing `getCopy` unqualified is rejected:

<!-- cinch:reject contains="unknown name getCopy" -->
```cinch
module Main where
  readAt :: forall n region. Ref region (Vec n Nat) -> Fin n -> Nat
  readAt v f = getCopy v f

  main :: Int
  main = 0
```

## Runtime capabilities and concurrency

All `Io r` operations require an `Io r` capability in scope via `using`.

| Name | Arity | Scheme | Effect row |
| --- | --- | --- | --- |
| `withScope` | 1 | `forall a e. using (io :: Io r) => (Scope r s ->{e} a) ->{Diverge \| e} a` | `{Diverge \| e}` |
| `spawn` | 2 | `Scope r s -> (Unit ->{eb} Result x a) ->{Async r \| eb} Task s x a` | `{Async r \| eb}` |
| `joinAllOrCancel` | 3 | `Scope r s -> Task s x v -> Task s x w ->{Async r, Cancel r, Diverge} Result x (v, w)` | `{Async r, Cancel r, Diverge}` |
| `ioWrite` | 1 | `using (io :: Io r) => Bytes ->{IO r} Unit` | `{IO r}` |
| `ioRead` | 1 | `using (io :: Io r) => Nat ->{Diverge, IO r} Option Bytes` | `{Diverge, IO r}` |
| `ioSleep` | 1 | `using (io :: Io r) => Nat ->{Cancel r, IO r} Unit` | `{Cancel r, IO r}` |
| `pollCancel` | 1 | `using (io :: Io r) => Unit ->{Cancel r} Bool` | `{Cancel r}` |

In addition to their type schemes, `spawn` and `joinAllOrCancel` enforce scope-escape, capture, and linear task-handle rules during elaboration. Partially applying `spawn` without its task body is rejected:

<!-- cinch:reject contains="spawn takes a scope and one task body" -->
```cinch
module Main where
  bad :: forall r s. using (io :: Io r) => Scope r s -> Int
  bad s = spawn s

  main :: Int
  main = 0
```

Writing bytes to stdout with `ioWrite`:

<!-- cinch:run exit=0 stdout="hi\n" -->
```cinch
module Main where
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case ioWrite (bytes-from-u8s (Cons 104 (Cons 105 (Cons 10 Nil)))) of
      () -> 0
```

Running two tasks inside a structured concurrency scope and consuming both linear task handles via `joinAllOrCancel`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result Nat (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok 20)
    h2 <- spawn s (\() -> Ok 2)
    joinAllOrCancel s h h2

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> case p of
      (a, b) -> if a * 10 + b == 202 then 0 else 1
    Err e -> 2
```

Dropping a `Task` handle without joining it fails with `consumed exactly once`. See [Chapter 13](13-concurrency-and-runtime.md) for concurrency lifetime and cancellation rules.

## Evidence and indexed bounds

`decide` (indexed under [Special forms](#special-forms)) produces `Yes`/`No` values of the built-in `Dec` family, which feed indexed constructors like `Fin` and bounds-checked accessors like `Vec.getCopy`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  safeAt :: (n :: Nat) -> Nat -> Option (Fin n)
  safeAt n i = case decide (i < n) of
    Yes bound -> Some (Fin i bound)
    No _ -> None

  readAt :: forall n region. Ref region (Vec n Nat) -> Fin n -> Nat
  readAt v f = Vec.getCopy v f

  main :: Int
  main = case safeAt 3 1 of
    Some f -> 0
    None -> 1
```

Combining `toNat`, `refl`, and `eval`:

<!-- cinch:run exit=42 -->
```cinch
module Main where
  proof p :: Eq 3 3
  proof p = refl

  main :: Int
  main = case toNat (3 :: U32) of
    n -> if n == 3 then
           eval [| 41 + 1 |]
         else 1
```

## Compile-time (staging) values

Staging values are registered both bare and under `Stage.*`, and syntax/name builders also have `Syntax.*` and `Name.*` aliases. They are valid only inside `stage`/`macro` definitions, `generate do` blocks, and quotation splices; referencing any staging value in runtime code is rejected:

<!-- cinch:reject contains="snapshot is compile-time only" -->
```cinch
module Main where
  main :: Int
  main = snapshot
```

### Staging capabilities, I/O, and generation

| Name | Arity | Scheme | Effect row |
| --- | --- | --- | --- |
| `snapshot` | 2 | `forall a e. String -> (IoCap ->{e} a) ->{e} a` | `{e}` |
| `withReadFiles` | 2 | `forall a e. IoCap -> (FileReadCap ->{e} a) ->{e} a` | `{e}` |
| `withWriteFiles` | 2 | `forall a e. IoCap -> (FileWriteCap ->{e} a) ->{e} a` | `{e}` |
| `withProcess` | 2 | `forall a e. IoCap -> (ProcCap ->{e} a) ->{e} a` | `{e}` |
| `withDatabase` | 4 | `forall a e. IoCap -> Secret -> SchemaRead -> (Database ->{e} a) ->{e} a` | `{e}` |
| `readFile` | 2 | `FileReadCap -> String ->{StageFile} String` | `{StageFile}` |
| `writeFile` | 3 | `FileWriteCap -> String -> String ->{StageFile} Unit` | `{StageFile}` |
| `run` | 3 | `ProcCap -> String -> List String ->{StageProcess} String` | `{StageProcess}` |
| `secret` | 1 | `String ->{StageSecret} Secret` | `{StageSecret}` |
| `emit` | 1 | `Declarations ->{StageEmit} Unit` | `{StageEmit}` |
| `introspectConsistently` | 1 | `Database -> SchemaSnapshot` | pure |
| `ReadSchema` | 0 | `SchemaRead` | pure |
| `CodecGenerator.derive` | 1 | `SchemaSnapshot -> Declarations` | pure (qualified only) |

### Quotation, string, and name helpers

| Name (and alias) | Arity | Scheme |
| --- | --- | --- |
| `quoteNat`, `quoteInt`, `quoteBool` | 1 | `Nat -> Code Nat`, `Int -> Code Int`, `Bool -> Code Bool` |
| `name-to-sym`, `sym-to-name` | 1 | `Name -> Sym`, `Sym -> Name` |
| `strNat`, `natStr`, `strLen` | 1 | `String -> Nat`, `Nat -> String`, `String -> Nat` |
| `strAppend` | 2 | `String -> String -> String` |
| `name-append` (`Name.append`) | 2 | `Name -> Name -> Name` |
| `name-prefix` (`Name.prefix`) | 2 | `String -> Name -> Name` |
| `name-suffix` (`Name.suffix`) | 2 | `Name -> String -> Name` |
| `name-join` (`Name.join`) | 3 | `String -> Name -> Name -> Name` |

### Syntax, code, type, and declaration builders

| Name (and alias) | Arity | Scheme |
| --- | --- | --- |
| `syntax-new-binding` (`Syntax.newBinding`) | 1 | `forall a. Name -> SyntaxBinding a` |
| `syntax-capture` (`Syntax.capture`) | 1 | `forall a. Syntax -> Code a` |
| `syntax-capture-under` (`Syntax.captureUnder`) | 2 | `forall a b. SyntaxBinding a -> Syntax -> Code b` |
| `syntax-reference` (`Syntax.reference`) | 1 | `forall a. SyntaxBinding a -> Code a` |
| `syntax-let-in` (`Syntax.letIn`) | 3 | `forall a b. SyntaxBinding a -> Code a -> Code b -> Code b` |
| `syntax-apply` (`Syntax.apply`) | 2 | `forall a b e. Code (a %many ->{e} b) -> Code a -> Code b` |
| `syntax-lambda` (`Syntax.lambda`) | 2 | `forall a b e. SyntaxBinding a -> Code b -> Code (a %many ->{e} b)` |
| `syntax-definition` (`Syntax.definition`) | 1 | `forall a. Name -> Code a` |
| `syntax-record` (`Syntax.recordValue`) | 2 | `forall a. Name -> Syntax -> Code a` |
| `syntax-error` (`Syntax.error`) | 1 | `forall a. String -> a` |
| `syntax-items` (`Syntax.items`) | 1 | `Syntax -> List Syntax` |
| `syntax-name` (`Syntax.name`) | 1 | `Syntax -> Name` |
| `syntax-pack-code` (`Syntax.packCode`) | 1 | `forall a. Code a -> PackedCode` |
| `syntax-tuple-code` (`Syntax.tupleCode`) | 1 | `forall a. List PackedCode -> Code a` |
| `syntax-capture-type`, `syntax-declaration-type`, `syntax-signature-type` | 1, 2, 2 | `Syntax -> TypeSyntax`, `Name -> Declarations -> TypeSyntax`, `Name -> Declarations -> TypeSyntax` |
| `syntax-apply-type`, `syntax-function-type` | 2 | `TypeSyntax -> TypeSyntax -> TypeSyntax` |
| `syntax-tuple-type` (`Syntax.tupleType`) | 1 | `List TypeSyntax -> TypeSyntax` |
| `syntax-empty-declarations` (`Syntax.emptyDeclarations`) | 0 | `Declarations` |
| `syntax-append-declarations` (`Syntax.appendDeclarations`) | 2 | `Declarations -> Declarations -> Declarations` |
| `syntax-link-declarations` (`Syntax.linkDeclarations`) | 4 | `Name -> Name -> Declarations -> Declarations -> Declarations` |
| `syntax-rename-declarations` (`Syntax.renameDeclarations`) | 3 | `Name -> Name -> Declarations -> Declarations -> Declarations` |
| `syntax-export-declaration` (`Syntax.exportDeclaration`) | 2 | `Name -> Declarations -> Declarations` |
| `syntax-specialize-declaration-type` (`Syntax.specializeDeclarationType`) | 3 | `Name -> Syntax -> Declarations -> Declarations` |
| `syntax-declaration-reference`, `syntax-function-code` | 2 | `forall a. Name -> Declarations -> Code a` |
| `syntax-specialize-type` (`Syntax.specializeType`) | 3 | `Name -> TypeSyntax -> Declarations -> Declarations` |
| `syntax-add-constructor` (`Syntax.addConstructor`) | 4 | `Name -> Name -> TypeSyntax -> Declarations -> Declarations` |
| `syntax-add-equations` (`Syntax.addEquations`) | 4 | `Name -> Name -> Declarations -> Declarations -> Declarations` |
| `syntax-record-declaration` (`Syntax.recordDeclaration`) | 2 | `Name -> Syntax -> Declarations` |

Using hygienic syntax builders and `emit` to generate `main`:

<!-- cinch:run exit=42 -->
```cinch
module Main where
  stage identity :: forall a. Code (a -> a)
  stage identity =
    let binding = Syntax.newBinding (sym-to-name 'x)
    in Syntax.lambda binding (Syntax.reference binding)

  generate do
    emit (quoteDecls where
      main :: Int
      main = if $(identity) True then $(identity) (42 :: Int) else 0)
```

See [Chapter 14](14-staging-and-macros.md) for staging phases, splicing, and hygiene rules.

## WIT stream and future bindings

A generated WIT adapter installs its raw interface functions under qualified names (`Raw.<fn>`). When the adapter includes streams or futures, three bare operations are also installed with closed effect row `{Foreign Component, Diverge}`:

| Name | Arity | Scheme |
| --- | --- | --- |
| `wasiStreamRead` | 1 | `forall a. WasiStream a ->{Foreign Component, Diverge} Result StreamError (Option a)` |
| `wasiStreamWrite` | 2 | `forall a. WasiStream a -> a ->{Foreign Component, Diverge} Result StreamError Unit` |
| `wasiFutureWait` | 1 | `forall a. WasiFuture a ->{Foreign Component, Diverge} a` |

Without a discovered `.wit` adapter, these bindings are absent:

<!-- cinch:reject contains="unknown name wasiStreamRead" -->
```cinch
module Main where
  main :: Int
  main = wasiStreamRead
```

# Special forms

Special forms have no standalone binding or polymorphic value type. They are matched syntactically at application sites, so referencing a special form unapplied is a type error.

| Form | Arity | Semantics |
| --- | --- | --- |
| `decide p` | 1 | Evaluates proposition `p` to `Dec p` (`Yes` proof or `No` refutation). Static numeral propositions fold at compile time; runtime decisions scope evidence to the examined values. |
| `ignore e` | 1 | Explicitly discards a must-use value (such as `Result`), returning `Unit`. |
| `copyOut e` | 1 | Copies a value out of `Ref`/`Mut`, or duplicates a `Copy` value bound `%many`. |
| `refSet m v` | 2 | Writes `v` into `Mut region a`, returning `Unit` and invalidating prior refinement facts on that cell. |
| `borrow x` | 1 | Creates a shared borrow `Ref region a`; requires a region annotation (via expected type or ascription). |
| `mutBorrow x` | 1 | Creates a mutable borrow `Mut region a`; requires a region annotation. |
| `delay f` | 1 | Wraps `Unit ->{e} a` into `Thunk {e} a` without running `e`. |
| `force t` | 1 | Forces `Thunk {e} a` (performing `e`) or advances a guarded `Later a`. |
| `demand x` | 1 | Forces a memo cell `~x` and returns a borrow of the cached value under `{Eval region}`. |
| `unrefine x` | 1 | Projects a refined value to its underlying carrier type (zero-cost identity at run time). |

## `decide`

`decide` bridges runtime checks and static proof obligations:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  safeDiv :: Nat -> Nat -> Option Nat
  safeDiv n d = case decide (NonZero d) of
    Yes w -> Some (nat-div n (NonZero d w))
    No f -> None

  main :: Int
  main = case safeDiv 10 2 of
    Some q -> case q of
      5 -> 0
      _ -> 1
    None -> 2
```

## `copyOut`

`copyOut` requires `Copy` evidence and a `%many` binder (or a borrow). Copying a `%1` binder is rejected because linear values cannot be duplicated:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  dup :: forall a. using (c :: Copy a) => a -> (a, a)
  dup x = (copyOut x, copyOut x)

  main :: Int
  main = case dup 7 of
    (a, b) -> if a == b then 0 else 1
```

<!-- cinch:reject contains="no Copy evidence for a" -->
```cinch
module Main where
  dup :: forall a. a -> (a, a)
  dup x = (copyOut x, x)

  main :: Int
  main = 0
```

## `borrow`, `mutBorrow`, and `refSet`

`mutBorrow` and `refSet` introduce and mutate cells; writing through a `Mut` invalidates any prior branch facts about that cell:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  bump :: forall region. Mut region Int -> Int
  bump m =
    let u = refSet m 9
    in copyOut m

  update :: forall region. Int -> Int
  update x = bump (mutBorrow x :: Mut region Int)

  main :: Int
  main = case update 3 of
    9 -> 0
    _ -> 1
```

`borrow` requires a known region from its context or an explicit type ascription:

<!-- cinch:reject contains="cannot infer the region of borrow" -->
```cinch
module Main where
  read :: forall region. Ref region Int -> Int
  read r = copyOut r

  main :: Int
  main =
    let v = 41
    in case borrow v of
      r -> case read r of
        41 -> 0
        _ -> 1
```

<!-- cinch:run exit=0 -->
```cinch
module Main where
  read :: forall region. Ref region Int -> Int
  read r = copyOut r

  peek :: forall region. Int -> Int
  peek x = read (borrow x :: Ref region Int)

  main :: Int
  main = case peek 41 of
    41 -> 0
    _ -> 1
```

## `delay`, `force`, and `demand`

`delay`/`force` control explicit thunks; `demand` evaluates a memo binding (`~name`) and returns a borrow of the cached cell:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  thunkOf :: forall (e :: Row). Thunk {e} Int
  thunkOf = delay (\() -> 41 + 1)

  main :: Int
  main = case force thunkOf of
    42 -> 0
    _ -> 1
```

<!-- cinch:run exit=0 -->
```cinch
module Main where
  cell :: Int -> Int
  cell x = copyOut (demand y)
    where ~y = x + 1

  main :: Int
  main = case cell 41 of
    42 -> 0
    _ -> 1
```

## `unrefine` and `ignore`

`unrefine` strips a refinement wrapper at zero runtime cost; `ignore` explicitly acknowledges and discards a must-use value:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  total :: InRange 0 255 Nat -> Nat
  total n = unrefine n

  discard :: (Result String Int) -> Int
  discard r = case ignore r of
    () -> 0

  main :: Int
  main = case total 65 of
    65 -> discard (Ok 1)
    _ -> 1
```

# Qualified aliases and reserved namespaces

## Dotted presentation aliases

The checker installs nine qualified presentation aliases that resolve directly to their standard-library definitions:

| Alias | Target |
| --- | --- |
| `U32.zero` | `zero-u32` |
| `U32.wrapping-add` | `wrapping-add-u32` |
| `Vec.stream` | `vec-stream` |
| `Stream.fold`, `Stream.map` | `prod-fold`, `prod-map` |
| `Scalar.sum-wrap32` | `scalar-sum-wrap32` |
| `Simd.sum-wrap32` | `simd-sum-wrap32` |
| `Target.detect-vec-u32` | `detect-vec-u32` |
| `SumProofs.scalar-to-simd` | `scalar-to-simd` |

## `Proof.search`

`Proof.search` is a qualified-only proof-elaboration form valid only where an expected proposition is known from context. It has no runtime representation and cannot be inferred standalone (`Proof.search needs an expected proposition; it cannot be inferred or used at runtime`).

## Reserved foreign symbol prefixes

`cinch.prim.*` and `cinch.runtime.*` are reserved foreign symbol namespaces owned by the Prelude and C runtime. User code must not bind against them directly; use admitted `prim` declarations or ordinary `foreign import` declarations with explicit ABI and effect contracts (see [Chapter 15](15-ffi-layout-and-attributes.md)).

# Standard library vs. compiler-installed names

Everything outside the tables above comes from standard-library `.cinch` modules (re-exported through `Prelude` or imported explicitly). An `unknown name` diagnostic for a library operation means its module has not been imported; compiler-installed names always resolve.

| Family | Home modules | Representative names |
| --- | --- | --- |
| Arithmetic classes & operators | `Algebra.Arithmetic` | `Add`, `Div`, `NonZero`, `+`, `/` |
| Bitwise classes & operators | `Algebra.Bits` | `Bits`, `&`, `<<` |
| Equality & ordering | `Relation.Eq`, `Relation.Ord` | `Eq`, `Ord`, `==`, `<` |
| Arbitrary-precision integers | `Data.Nat`, `Data.Int` | `nat-add`, `checked-div`, `int-mod-nat` |
| Machine scalars | `Data.Machine` | `checked-add-u8`, `u8-to-i32`, `max-bound-i64` |
| Text, bytes, symbols, chars | `Data.String`, `Data.Bytes`, `Data.Sym`, `Data.Char` | `string-to-bytes`, `bytes-get`, `intern`, `char-code` |
| Containers | `Data.List`, `Data.Option`, `Data.Result`, `Data.Map` | `map-option`, `Ok`, `map-lookup` |
| High-level I/O & concurrency | `Effect.Sequencing`, `Effect.Io`, `Effect.Concurrent`, `Effect.Fs` | `fs-read`, `read-config-bytes`, `fetch` |

Prefer the high-level wrappers in `Effect.Io` and `Effect.Concurrent` over calling low-level installed primitives (`ioRead`, `ioWrite`, `spawn`) directly.

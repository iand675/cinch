# Compiler-provided vocabulary and special forms

Most operations are ordinary library declarations, indexed in the [library reference](../library/00-index.md). A second surface is installed by the Rust checker itself: these names have no source declaration in Prelude, and their schemes live in the checker's registration table rather than in a `.cinch` file. This page is the index of that surface. Every name below was read from a registration site in `compiler/src/check_decls.rs` and then exercised with the compiler; the entry for each name gives its arity, its scheme, its effect row, and either a program that uses it or the message that refuses it.

Two kinds of name appear. An **installed value** is a real binding with a scheme, registered under a bare name, a qualified name, or both. A **special form** has no binding at all: the checker matches its application directly in `check_terms.rs`, so printing its type is not even meaningful.

# Installed types

These types are registered before any source is read, so no program can redeclare them.

| Name | Arity | Role |
| --- | --- | --- |
| `Nat`, `Int`, `Bool`, `Unit`, `Bytes`, `Sym` | 0 | Core types. `Nat` and `Int` are bignums with an immediate fast path; `Bytes` and `Sym` are counted handles. |
| `Char`, `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`, `F32`, `F64`, `Usize`, `Isize` | 0 | The thirteen machine scalars, from the `Scalar` enumeration. [Numbers and primitives](05-numbers-and-primitives.md) gives widths. |
| `Array`, `Tensor`, `Columnar` | 1 | Opaque runtime containers. No source constructor can forge a shape or a column layout. |
| `Io` | 1 | Capability evidence for identity `r`. The value is a uniform word; the identity lives in the type. |
| `Scope`, `Task` | 2, 3 | Structured concurrency carriers: `Scope r s` and `Task s x a`. |
| `Code`, `SyntaxBinding` | 1 | Compile-time-only carriers: a typed quotation, and a binding in a captured syntax tree. |
| `Syntax`, `TypeSyntax`, `PackedCode`, `Declarations`, `Name`, `Secret`, `IoCap`, `FileReadCap`, `FileWriteCap`, `ProcCap`, `SchemaSnapshot`, `Database`, `SchemaRead` | 0 | Compile-time-only staging vocabulary. None of them appears in generated code. |
| `Vec` | 2 | Builtin indexed family: `Nil :: Vec 0 a`, `Cons :: a -> Vec k a -> Vec (k + 1) a`. |
| `Fin` | 1 | Builtin indexed family: `Fin :: (i :: Nat) -> (bound :: i < n) -> Fin n`. The bound is erased. |
| `Dec` | 1 | Builtin indexed family: `Yes :: (h :: p) -> Dec p`, `No :: Not p -> Dec p`. Both fields are erased. |

`Vec`, `Fin`, and `Dec` arrive as builtins because their runtime helpers need walks the surface cannot express, and because `Vec`'s constructor spellings `Nil` and `Cons` would collide with `Data.List`. A user program declares its own indexed families in the same syntax; [types and kinds](04-types-and-kinds.md) shows one.

The generated WIT vocabulary is a different matter: `WasiStream`, `WasiFuture`, `StreamError`, and `BoundaryError` are written into the `Prelude.Generated` module when an adapter is generated, not registered as builtins.

# Installed values

## Core values

| Name | Arity | Scheme | Effect row | Evidence |
| --- | --- | --- | --- | --- |
| `True`, `False` | 0 | `Bool` | none | Used by every `if` and `case` below. |
| `newIO` | 1 | `Unit -> Io r` | pure | `with (io = newIO ()) in ...` in the I/O program below. Also registered as `Runtime.newIO`. |
| `eval` | 1 | `forall a. Code a -> a` | pure | `eval [\| 41 + 1 \|]` below, under a profile with `allow_eval`. |

`newIO` constructs a fresh runtime capability identity; each use instantiates a new region, and the value is a unit marker. `eval` consumes a typed quotation and carries no capability and no effect, because the quotation was typed where it was built. A target profile may forbid runtime code generation, and [supercompilation and JIT](../specifications/supercompilation-and-jit-specification.md) section 7 states that boundary.

## Index and evidence values

| Name | Arity | Scheme | Notes |
| --- | --- | --- | --- |
| `toNat` | 1 | `U32 -> Nat` | Total, executable index function usable inside refinements. |
| `length` | 1 | `Bytes -> Nat` | Installed, but shadowed at the surface: Prelude's `Data.List.length :: forall a. List a -> Int` answers first, which is what `:t length` prints. |
| `vecStreamStep` | 1 | `Ref region (Vec n a) -> Step (Ref region (Vec n a)) a` | The producer step over a borrowed vector. |
| `refl` | 0 | `Eq x x` | Evidence constructor. Erased. |
| `congr` | 3 | `Eq a b -> Eq (f a) (f b)` | Evidence constructor lifting equality across a named total function. Erased. |
| `Vec.getCopy` | 2 | `forall region n a. using (copy :: Copy a) => Ref region (Vec n a) -> Fin n -> a` | Qualified only; the bare name is not installed. |

`congr` takes the function, the two endpoints, and the equality; the scheme the REPL prints is `(Eq ?i550 ?i551 -> Eq (?i549 ?i550) (?i549 ?i551))`. `getCopy` has no bare spelling, and a program referring to `getCopy` unqualified is refused:

<!-- cinch:reject contains="unknown name getCopy" -->
```cinch
module Main where
  readAt :: forall n region. Ref region (Vec n Nat) -> Fin n -> Nat
  readAt v f = getCopy v f

  main :: Int
  main = 0
```

## Runtime capabilities and concurrency

All four `Io r` operations carry an `Io r` constraint the caller must satisfy, which is why their schemes are only printable with the capability in scope.

| Name | Arity | Scheme | Effect row |
| --- | --- | --- | --- |
| `withScope` | 1 | `forall a e. using (io :: Io r) => (Scope r s ->{e} a) ->{Diverge \| e} a` | body row plus `Diverge` |
| `spawn` | 2 | `Scope r s -> (Unit ->{eb} Result x a) ->{Async r \| eb} Task s x a` | `Async r` plus the body row |
| `joinAllOrCancel` | 3 | `Scope r s -> Task s x v -> Task s x w ->{Async r, Cancel r, Diverge} Result x (v, w)` | closed `{Async r, Cancel r, Diverge}` |
| `ioWrite` | 1 | `using (io :: Io r) => Bytes ->{IO r} Unit` | closed `{IO r}` |
| `ioRead` | 1 | `using (io :: Io r) => Nat ->{Diverge, IO r} Option Bytes` | closed `{Diverge, IO r}` |
| `ioSleep` | 1 | `using (io :: Io r) => Nat ->{Cancel r, IO r} Unit` | closed `{Cancel r, IO r}` |
| `pollCancel` | 1 | `using (io :: Io r) => Unit ->{Cancel r} Bool` | closed `{Cancel r}` |

`spawn` and `joinAllOrCancel` get their capture, scope, and one-shot elaboration in `check_terms.rs`, so the registered scheme is the arity contract and the elaborator adds the ownership rules. A `spawn` that is not applied to its body directly is refused before the scheme is consulted:

<!-- cinch:reject contains="spawn takes a scope and one task body" -->
```cinch
module Main where
  bad :: forall r s. using (io :: Io r) => Scope r s -> Int
  bad s = spawn s

  main :: Int
  main = 0
```

`ioWrite` writes bytes and returns `Unit`. `ioRead` returns `Option Bytes` so the end of input is an ordinary value. Here the program writes `hi` and a newline:

<!-- cinch:run exit=0 stdout="hi\n" -->
```cinch
module Main where
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case ioWrite (bytes-from-u8s (Cons 104 (Cons 105 (Cons 10 Nil)))) of
      () -> 0
```

Two tasks run inside one scope, and the join consumes both handles exactly once and returns their results in handle order:

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

Dropping a handle instead of joining it is refused with `consumed exactly once`. [Concurrency](13-concurrency-and-runtime.md) gives the ordering and lifetime rules.

## Evidence and decisions

`decide` is a special form rather than an installed value, so it is indexed under [special forms](#special-forms) below. The two values it produces, `Yes` and `No`, are builtin constructors of the builtin `Dec` family, and `Fin`'s constructor is builtin too:

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

`safeAt` builds a `Fin n` only from the bound `decide` hands back, and `Vec.getCopy` accepts nothing else. `toNat`, `refl`, and `eval` appear together here:

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

Every staging value is registered twice: under its bare name and under the `Stage.` qualification, and the syntax builders also under their namespaced aliases (`Syntax.*`, `Name.*`). All of them are refused in runtime code with the same message:

<!-- cinch:reject contains="snapshot is compile-time only" -->
```cinch
module Main where
  main :: Int
  main = snapshot
```

The rule covers `stage` and `macro` definitions, `generate do` blocks, and quotation splices.

| Name | Arity | Scheme | Effect row |
| --- | --- | --- | --- |
| `snapshot` | 2 | `forall a e. String -> (IoCap ->{e} a) ->{e} a` | callback row |
| `withReadFiles` | 2 | `forall a e. IoCap -> (FileReadCap ->{e} a) ->{e} a` | callback row |
| `withWriteFiles` | 2 | `forall a e. IoCap -> (FileWriteCap ->{e} a) ->{e} a` | callback row |
| `withProcess` | 2 | `forall a e. IoCap -> (ProcCap ->{e} a) ->{e} a` | callback row |
| `withDatabase` | 4 | `forall a e. IoCap -> Secret -> SchemaRead -> (Database ->{e} a) ->{e} a` | callback row |
| `readFile` | 2 | `FileReadCap -> String ->{StageFile} String` | `{StageFile}` |
| `writeFile` | 3 | `FileWriteCap -> String -> String ->{StageFile} Unit` | `{StageFile}` |
| `run` | 3 | `ProcCap -> String -> List String ->{StageProcess} String` | `{StageProcess}` |
| `secret` | 1 | `String ->{StageSecret} Secret` | `{StageSecret}` |
| `emit` | 1 | `Declarations ->{StageEmit} Unit` | `{StageEmit}` |
| `introspectConsistently` | 1 | `Database -> SchemaSnapshot` | pure |
| `ReadSchema` | 0 | `SchemaRead` | pure |
| `quoteNat`, `quoteInt`, `quoteBool` | 1 | `Nat -> Code Nat`, `Int -> Code Int`, `Bool -> Code Bool` | pure |
| `name-to-sym`, `sym-to-name` | 1 | `Name -> Sym`, `Sym -> Name` | pure |
| `strNat`, `natStr`, `strLen` | 1 | `String -> Nat`, `Nat -> String`, `String -> Nat` | pure |
| `strAppend` | 2 | `String -> String -> String` | pure |
| `syntax-new-binding` (`Syntax.newBinding`) | 1 | `forall a. Name -> SyntaxBinding a` | pure |
| `syntax-capture` (`Syntax.capture`) | 1 | `forall a. Syntax -> Code a` | pure |
| `syntax-capture-under` (`Syntax.captureUnder`) | 2 | `forall a b. SyntaxBinding a -> Syntax -> Code b` | pure |
| `syntax-reference` (`Syntax.reference`) | 1 | `forall a. SyntaxBinding a -> Code a` | pure |
| `syntax-let-in` (`Syntax.letIn`) | 3 | `forall a b. SyntaxBinding a -> Code a -> Code b -> Code b` | pure |
| `syntax-apply` (`Syntax.apply`) | 2 | `forall a b e. Code (a %many ->{e} b) -> Code a -> Code b` | row parameter `e` |
| `syntax-lambda` (`Syntax.lambda`) | 2 | `forall a b e. SyntaxBinding a -> Code b -> Code (a %many ->{e} b)` | row parameter `e` |
| `syntax-definition` (`Syntax.definition`) | 1 | `forall a. Name -> Code a` | pure |
| `syntax-record` (`Syntax.recordValue`) | 2 | `forall a. Name -> Syntax -> Code a` | pure |
| `syntax-error` (`Syntax.error`) | 1 | `forall a. String -> a` | pure |
| `syntax-items` (`Syntax.items`) | 1 | `Syntax -> List Syntax` | pure |
| `syntax-name` (`Syntax.name`) | 1 | `Syntax -> Name` | pure |
| `syntax-empty-declarations` (`Syntax.emptyDeclarations`) | 0 | `Declarations` | pure |
| `syntax-append-declarations` (`Syntax.appendDeclarations`) | 2 | `Declarations -> Declarations -> Declarations` | pure |
| `syntax-link-declarations` (`Syntax.linkDeclarations`) | 4 | `Name -> Name -> Declarations -> Declarations -> Declarations` | pure |
| `syntax-rename-declarations` (`Syntax.renameDeclarations`) | 3 | `Name -> Name -> Declarations -> Declarations` | pure |
| `syntax-export-declaration` (`Syntax.exportDeclaration`) | 2 | `Name -> Declarations -> Declarations` | pure |
| `syntax-specialize-declaration-type` (`Syntax.specializeDeclarationType`) | 3 | `Name -> Syntax -> Declarations -> Declarations` | pure |
| `syntax-capture-type`, `syntax-declaration-type`, `syntax-signature-type` | 1, 2, 2 | `Syntax -> TypeSyntax`, `Name -> Declarations -> TypeSyntax`, `Name -> Declarations -> TypeSyntax` | pure |
| `syntax-declaration-reference`, `syntax-function-code` | 2 | `forall a. Name -> Declarations -> Code a` | pure |
| `syntax-apply-type`, `syntax-function-type` | 2 | `TypeSyntax -> TypeSyntax -> TypeSyntax` | pure |
| `syntax-tuple-type` (`Syntax.tupleType`) | 1 | `List TypeSyntax -> TypeSyntax` | pure |
| `syntax-specialize-type` (`Syntax.specializeType`) | 3 | `Name -> TypeSyntax -> Declarations -> Declarations` | pure |
| `syntax-add-constructor` (`Syntax.addConstructor`) | 4 | `Name -> Name -> TypeSyntax -> Declarations -> Declarations` | pure |
| `syntax-add-equations` (`Syntax.addEquations`) | 4 | `Name -> Name -> Declarations -> Declarations -> Declarations` | pure |
| `syntax-record-declaration` (`Syntax.recordDeclaration`) | 2 | `Name -> Syntax -> Declarations` | pure |
| `syntax-pack-code` (`Syntax.packCode`) | 1 | `forall a. Code a -> PackedCode` | pure |
| `syntax-tuple-code` (`Syntax.tupleCode`) | 1 | `forall a. List PackedCode -> Code a` | pure |
| `name-append` (`Name.append`) | 2 | `Name -> Name -> Name` | pure |
| `name-prefix` (`Name.prefix`) | 2 | `String -> Name -> Name` | pure |
| `name-suffix` (`Name.suffix`) | 2 | `Name -> String -> Name` | pure |
| `name-join` (`Name.join`) | 3 | `String -> Name -> Name -> Name` | pure |
| `CodecGenerator.derive` | 1 | `SchemaSnapshot -> Declarations` | pure, qualified only |

`CodecGenerator.derive` has no bare spelling, so a user definition named `derive` stays available.

A quotation, two syntax builders, and `emit` in one program, which prints nothing and exits 42:

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

Staging types never appear in generated code, and a `Code` carrier is checked at its splice. [Staging and macros](14-staging-and-macros.md) gives the ordering and hygiene rules.

## WIT and stream vocabulary

A generated WIT adapter installs its raw bindings under qualified names (`adapter binding`), because the bare function name stays the user's to define. When the adapter declares streams, three word-level observations are also installed bare, each with the closed row `{Foreign Component, Diverge}`:

| Name | Arity | Scheme |
| --- | --- | --- |
| `wasiStreamRead` | 1 | `forall a. WasiStream a ->{Foreign Component, Diverge} Result StreamError (Option a)` |
| `wasiStreamWrite` | 2 | `forall a. WasiStream a -> a ->{Foreign Component, Diverge} Result StreamError Unit` |
| `wasiFutureWait` | 1 | `forall a. WasiFuture a ->{Foreign Component, Diverge} a` |

Without a generated adapter the names do not exist, which is why `Raw` code reaches them through the boundary module instead:

<!-- cinch:reject contains="unknown name wasiStreamRead" -->
```cinch
module Main where
  main :: Int
  main = wasiStreamRead
```

# Special forms

These names have no registry entry. The checker matches the application in `check_terms.rs`, so the arity and the rule below are the whole contract, and a bare reference is refused with `<name> must be applied in the bootstrap slice`.

| Name | Arity | What it does | Evidence |
| --- | --- | --- | --- |
| `decide` | 1 | Takes a proposition and yields `Dec p`: `Yes` carries the proof, `No` the refutation. Static decisions fold on numerals; the evidence is scoped to the values examined. | The `safeDiv` program below. |
| `ignore` | 1 | Explicitly discards a must-use result, including a `Result`. Returns `Unit`. | The `discard` program below. |
| `copyOut` | 1 | Copies out of a `Ref` or `Mut`, or duplicates a `Copy` value. Requires `Copy` evidence and a `%many` binder. | The `copyOut` program below. |
| `refSet` | 2 | Writes through a `Mut`: `Mut region a -> a -> Unit`. Invalidates facts about the cell. | The `bump` program below. |
| `borrow` | 1 | Introduces a `Ref region a` from a place. The region needs an annotation. | The `peek` program below. |
| `mutBorrow` | 1 | Introduces a `Mut region a` from a place. The region needs an annotation. | The `cell` program below. |
| `delay` | 1 | `(Unit ->{e} a) -> Thunk {e} a`. Creating the thunk performs nothing. | The `thunkOf` program below. |
| `force` | 1 | Forces a `Thunk {e} a` or a `Later a`, performing `e` in the first case. | The `thunkOf` program below. |
| `demand` | 1 | Demands a `Lazy` cell and yields a borrow of the cached value, performing `{Eval region}`. | The `cell` program below. |
| `unrefine` | 1 | Projects a refinement to its carrier. The identity at run time. | The `total` program below. |

`decide` is the entry to the obligation world. A numeral proposition folds here, which is what makes `10 / 2` total without a written proof:

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

`copyOut` needs `Copy` evidence and a binder it may duplicate. Both refusals name the reason:

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

A `%1` binder is refused with `multiplicity %1: copyOut would copy x, but a value bound %1 cannot be duplicated (copying needs a %many binder or a borrow)`.

`mutBorrow` and `refSet` are the mutable-place pair. A write between a test and a use invalidates the fact the test established, which is reported as a stale-evidence error.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  cell :: forall region. Int -> Mut region Int
  cell x = mutBorrow x

  bump :: forall region. Mut region Int -> Int
  bump m =
    let u = refSet m 9
    in copyOut m

  main :: Int
  main = case bump (cell 3) of
    9 -> 0
    _ -> 1
```

`borrow` produces the shared analogue, and without an annotation the region is unknowable:

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

With the annotation and a bound region, the same program checks:

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

`delay` and `force` are the explicit thunk pair. `force` on a `Thunk` performs the body's row; on a `Later` it takes the next guarded tick.

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

`demand` yields a borrow of a memo cell, so the cache is never silently duplicated; `copyOut` produces the owned value. The cell comes from a memo binding `~name`:

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

`unrefine` is a refinement's free projection and `ignore` is the explicit discard. Both appear here:

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

# Qualified spellings and reserved namespaces

Beyond the per-name qualifications above, the checker registers nine dotted presentation aliases that resolve to the surface's home definition, so the split module layout stays invisible:

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

`Proof.search` is a qualified-only elaboration form used where an expected proposition exists; it is not a value and cannot be inferred or used at run time. The message is `Proof.search needs an expected proposition; it cannot be inferred or used at runtime`.

`cinch.prim.*` and `cinch.runtime.*` are reserved foreign symbol namespaces, not callable vocabulary. Their declarations are the prelude's `foreign import ccall` lines; the second is the runtime's own C surface. Guessing a reserved symbol does not give you an extension API: define an admitted `prim` body or a real foreign binding with explicit ABI and containment contracts instead. [Primitive operations](../specifications/primitive-operations-and-inline-target-ir-specification.md) is the normative text.

# What is library rather than installed

The names above are the whole installed surface. Everything else that looks built in arrives through Prelude's re-export list, and a name Prelude does not re-export is reachable only by importing its home module. The distinction matters when reading a diagnostic: `unknown name` for a Prelude name means the import is missing, while a registered name always resolves.

| Family | Home module | Examples |
| --- | --- | --- |
| Arithmetic classes and their operators | `Algebra.Arithmetic` | `Add`, `Div`, `NonZero`, `+`, `/` |
| Bit classes and operators | `Algebra.Bits` | `Bits`, `&`, `<<` |
| Equality and order | `Relation.Eq`, `Relation.Ord` | `Eq`, `Ord`, `==`, `<` |
| Mathematical integers | `Data.Nat`, `Data.Int` | `nat-add`, `checked-div`, `int-mod-nat` |
| Machine scalars | `Data.Machine` | `checked-add-u8`, `u8-to-i32`, `max-bound-i64` |
| Text, bytes, symbols | `Data.String`, `Data.Bytes`, `Data.Sym`, `Data.Char` | `string-to-bytes`, `bytes-get`, `intern`, `char-code` |
| Containers | `Data.List`, `Data.Option`, `Data.Result`, `Data.Map` | `map-option`, `Ok`, `map-lookup` |
| Sequencing and I/O wrappers | `Effect.Sequencing`, `Effect.Io`, `Effect.Concurrent`, `Effect.Fs` | `fs-read`, `read-config-bytes`, `fetch` |

`Effect.Concurrent` and `Effect.Io` are worth reading beside this page: they are ordinary Prelude modules whose operations call the installed `ioRead`, `ioWrite`, and `spawn`, which is the intended way to use them.

# Implementation

`compiler/src/check_decls.rs` holds every registration site: `install_builtins`, `install_concurrency_api`, `install_eval_api`, `install_staging_api`, `install_stage_value`, `install_stage_value_row`, `install_index_families`, `register_family`, `install_index_values`, `install_wit_api`, `install_stream_api`, and `install_presentation_aliases`. Special-form application is in `compiler/src/check_terms.rs` (`infer_builtin_app` and the application elaborator); schemes and the `Word` representation are in `compiler/src/core.rs`. Suites: `bootstrap_intrinsics.rs`, `bootstrap_effects.rs`, `concurrency.rs`, `dependent.rs`, `staging.rs`, `stage_code_expectations.rs`, `bootstrap_pipeline.rs`.

# Types, kinds, declarations, and inference

Cinch combines explicit top-level signatures with bidirectional checking and unification. Kinds describe what parameters a type constructor expects; dependent binders allow types to mention values.

# Type grammar in use

| Type | Interpretation |
| --- | --- |
| `Int`, `Nat`, `Bool`, `Unit`, `Bytes`, `Char`, `Sym` | Core vocabulary; library homes provide operations and instances. |
| `List a`, `Result e a` | Type application. |
| `(a, b)` | Structural tuple type; `()` is unit. |
| `a -> b` | Pure function. |
| `a %1 -> b` | Exactly-once argument use. Other multiplicities: `%0`, `%aff`, `%many`. |
| `a ->{IO r, Diverge} b` | Function with a closed effect row. |
| `a ->{IO r | e} b` | Row extension over tail `e`. |
| `forall a. a -> a` | Explicit polymorphism. |
| `forall (f :: Type -> Type) a. f a -> f a` | Higher-kinded binder. |
| `using (eq :: Eq a) => a -> a -> Bool` | Required class dictionary. |
| `using (i < n) => ...` | Proof obligation over named dependent arguments. |
| `(n :: Nat) -> Vec n a` | Dependent function binder. |
| `Sigma (n :: Nat). Vec n a` | Dependent pair. |
| `Thunk {e} a`, `Lazy ...` | Effect-row arguments for deferred/memo types in their admitted forms. |

`Type` is stratified into universes. `Type : Type` is rejected. `Prop` is the proof universe with erasure and proof irrelevance; a proof cannot be used to manufacture arbitrary runtime data. Region binders have region kind and must remain scoped through interfaces and calls.

# Data, records, and signatures

A `data` declaration lists constructor signatures. Type parameters may be explicitly kinded. Constructor results can carry indices; positivity checking prevents invalid negative recursive occurrences. A `record` declares named fields and a product constructor. `codata` declares observations and obeys its own productivity discipline.

A `signature` declares methods and may give checked default implementations and laws. An `instance` supplies a dictionary; an `implicit instance` participates in unique resolution. Methods, constraints, and dictionaries retain definition-site identities through modules. An implicit instance is not selected merely by textual proximity.

`type Name ... = refine (...) with Predicate` declares a refinement. Do not assume arbitrary Haskell type synonyms, type families, or newtype syntax from this use of `type`. An `opaque type` declares a foreign/opaque representation boundary with its admitted rules.

# Inference and generalization

The checker synthesizes types where possible and checks expressions against expected types where necessary. Unification includes occurs checking. Effect rows are structural typed rows with metavariables and tails; performed effects must agree with the declared row. A method call inserts dictionary requirements.

Top-level signatures state their quantifiers explicitly. Eligible local value bindings can be generalized under the value restriction, so one local identity function can be used at different types. Effectful computations and restricted resources cannot gain unsound polymorphism through an unrestricted generalization rule.

The implementation is not an inference engine for arbitrary higher-rank dependent programs. Locally annotated `forall`/`using` signatures are a named restriction. For generic constraints, explicitly list every needed class: there is no current superclass entailment.

Types and byte storage are separate contracts. [Type layout and runtime representation](24-type-layout.md) explains native value words, explicit record layouts, padding, erased fields, and planned alternative representations.

# Interface contracts

Compiled schemes preserve quantifier kinds, defining nominal identities, constructor schemes, relevant private nominal/class closure, method visibility, and admitted contexts. A caller must not reinterpret a private type by giving an unrelated local declaration the same short name. These are checked compiler artifacts, not merely pretty-printed type strings for a documentation viewer.

Implementation: `compiler/src/types.rs`, `check.rs`, `check_decls.rs`, `check_interface.rs`; suites: `hk.rs`, `dependent.rs`, `interface_kinds.rs`, `interface_nominals.rs`, `interface_class_contracts.rs`, `interface_region_effects.rs`.

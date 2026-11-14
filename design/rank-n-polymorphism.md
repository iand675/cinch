# Rank-N polymorphism

Status: design for implementation, 2026-11-14. The Rust compiler
implements sections 1 to 6; section 8 is the Cinch frontend's port.
Companion to `design/layouts-and-specialization.md` (the class-vector
collector this design extends) and `specifications/core-language-specification.md`
(the amendment that makes the rule normative).

## 1 What is admitted

A `forall` may now appear anywhere a type may, not only at the outermost
position of a signature:

```haskell
both :: (forall a. a -> a) -> (I64, String) -> (I64, String)
both f (n, s) = (f n, f s)

data Church = Church (forall a. (a -> a) -> a -> a)

run-church :: Church -> Nat
run-church c = case c of Church k -> k (\n -> n + 1) 0
```

A nested quantifier binds proper types (`Type`), higher-kinded type
constructors, effect rows and regions. It may not carry `using`
constraints, proposition binders, index binders or runtime value
binders; those stay prenex, with a named refusal. Their runtime meaning
needs dictionary or witness arguments inside a polymorphic value, which
this slice does not define.

## 2 The ontology

Three things exist, and each one is a distinct construct in the checker
and in Core.

| Concept | Source | Checker | Core |
| --- | --- | --- | --- |
| **Polytype** `forall as. T` | a nested `forall` | `Ty::Forall(Box<Scheme>)` | a word of class `Obj`: a dictionary block |
| **Generalisation site** | a term checked against a polytype | skolemise, check, escape check, purity check | an anonymous instance of the polytype's family, built by `InstDict` |
| **Instantiation site** | a polytype-typed term used where a monotype is wanted | fresh metavariables for `as` | `DictProj` of slot 0 at the use's own class vector |

The key observation is that the backend already has this concept. A
class method with type parameters of its own (`fmap :: forall a b. ...`
inside a `Functor f` dictionary) is a polymorphic value carried by a
runtime block. The collector gives such a method one dictionary slot per
own class vector that some projection reaches, and every dictionary of
the class shares that slot table
(`specialize.rs`, "Dictionaries are copied per vector of the instance's
own parameters"). A rank-N value is that construct without a class
declaration:

- A polytype `forall as. T` names a **family**, an anonymous single-method
  class. Its name is `#poly.<k>`, where `k` is the number of type binders
  (rows and regions carry no class). The family's one method has `k` own
  parameters.
- A generalisation site is an anonymous **instance** of the family. Its one
  method unit has the term's free locals as its leading parameters (the
  dictionary's captured words, built by `mk` exactly as a parameterised
  instance's sub-dictionaries are), and is generic over the enclosing
  unit's type parameters followed by the skolems.
- An instantiation site is a **projection** of slot 0 with a
  `MethodSite { class: family, own: classes(ts) }`.

This is the class-indexed bundle chosen for the representation. It is
refined by the collector's demand: a family's dictionaries carry a slot
only for each own vector that some projection in the program reaches,
not all `3^k`. When nothing reaches the family, the collector's
existing default gives the all-`Obj` slot. `k` is therefore unbounded,
and no runtime class dispatch is introduced. The design note's
statement that "every instantiation is static" stays true. What changes
is that a projection's dictionary is no longer always statically known,
which was already true of a dictionary passed as an argument.

Family identity must be invariant under substitution of free outer type
variables. Inside `g :: forall b. (forall a. a -> b) -> b` the polytype
is `forall a. a -> b`, while at a call site it is `forall a. a -> Int`.
If those were different families, `g`'s projection would index a slot
table that the caller's dictionary never filled. A digest of the body
cannot be made invariant: `f a` with `f := Either e` changes shape. The
binder count is invariant. Sharing one slot table across all `k`-binder
polytypes only over-generates method copies, at most the reached own
vectors (never more than `3^k`). It never confuses two slots, because
the slot table is a function of the family alone.

## 3 Checking: predicative, bidirectional, simple subsumption

The rules are GHC's simplified subsumption (GHC proposal 287), restricted
to be predicative.

**Check** `e` against `forall as. T`:
1. Rigidify `as` to fresh skolems `ks` (`Checker::rigidify`).
2. Under a fresh performed-effect frame, check `e` against `T[ks/as]`.
3. The frame's performed row must be pure. A polymorphic value is built
   once per reached class vector, so evaluating it twice must be
   unobservable.
4. No skolem may appear in the zonked type of any binding captured from
   the enclosing scope, nor in any enclosing dictionary (escape check).
5. Elaborate to `ECore::PolyIntro`.

If `e` is a bare variable whose type is already alpha-equivalent to the
polytype, it passes through unchanged and nothing is rebuilt.

**Infer** a term whose zonked type is `forall as. T`: instantiate `as`
with fresh metavariables and elaborate to `ECore::PolyElim`. This happens
at variables, at an application head and at projections. It is top-level
instantiation only. A polytype under an arrow is not instantiated
implicitly, and an argument of type `forall a. a -> a` where
`I64 -> I64` is wanted is instantiated at the argument, through the
argument's own inference.

**Unify** `forall as. T` with `forall bs. U`: the binder lists must agree
in length and kind. Both sides are rigidified to the *same* fresh skolems,
and the bodies are unified. While that runs, no metavariable may be bound
to a type that mentions those skolems (the unifier's escape set).
Polytypes are invariant: there is no deep subsumption and no
co/contravariance through arrows. An eta-expansion at the use site
recovers what deep subsumption would admit.

**Predicativity**: a metavariable is never bound to a polytype. Binding
`?m := forall a. ...` is refused with "a type variable cannot stand for a
polymorphic type; annotate the binding". This keeps inference
order-independent. A lambda checked against an arrow binds its
parameter at the arrow's domain directly (no metavariable), so
`\f -> (f 1, f True)` against a rank-2 arrow works. A constructor's
scheme already carries its fields' polytypes.

Usage: a polytype-typed binder is reusable like a function (`%many` by
default). A generalisation body's captures count as a lambda's, as
unbounded uses.

Positivity: a polytype in a constructor field judges its body with the
polarity of its position. The binders introduce no data-type occurrence.

## 4 Elaboration and lowering

```
ECore::PolyIntro { skolems: Vec<String>, body: Box<ECore>, ty: Ty }
```

An instantiation needs no new form. It is the existing
`ECore::DictProj(value, 0, Some(("#poly.<k>", own_types)), body_type)`.

`hoist_lambdas_in` treats a `PolyIntro` as a zero-parameter lambda. It
computes the body's free locals and emits a hoisted method unit
`<def>.poly<N>`, whose parameters are those locals followed by the
parameters of a lambda body, when the body is one. A slot then holds the
method's partial application, not a closure computed at construction.
The node becomes `ECore::InstDict(<def>.inst<N>, captured, enclosing
identity, explicit)`.

Every hoisted unit records `extra`, the skolems of the generalisation
sites that enclose it. `finish_fn` lowers each unit with
`tparams = definition tparams ++ extra`. A word of skolem type then takes
`Class::Param(n + j)`, which the collector substitutes like any other.
For each site, `finish_fn` pushes an `InstanceDef`:

- `class = #poly.<k>`
- `tparams` = the enclosing vector's length
- `own = [k]`
- `subdict_count` = the captured locals

A strict method, one whose body is not a lambda, records its
`UnitRepr::result` class, so the slot's computed value is counted when
the dictionary dies.

The instance and method names differ before their last segment
(`Main.f.inst3`, `Main.f.poly3`), so the symbol scanner that maps a
static `X.dict` block back to `X.mk` attributes it exactly. That scanner
now applies its dictionary rule before its longest-prefix rule.

A rewrite rule or a compile-time probe that builds a polymorphic value is
refused by name; neither path registers instances yet.

Nothing else is new. `mk_body` already builds a dictionary whose slots
close over its arguments, taking their classes from the method unit's
parameter classes. The collector already gives one slot per reached own
vector.

## 5 Interfaces

A polytype in an exported signature is written to the compiled interface
as a new type tag. Readers that predate it refuse an interface containing
it, and existing interfaces are byte-identical. A separately compiled
object that projects a family it does not construct declares the
family's slot demand through the existing `ObjectRoot::Method` protocol.

## 6 Diagnostics

| Situation | Message |
| --- | --- |
| `using` inside a nested `forall` | "a nested forall cannot carry `using` constraints yet; lift them to the signature" |
| `?m := forall ...` | "a type variable cannot stand for a polymorphic type; annotate the binding" |
| skolem escape | "the polymorphic type variable `a` would escape its `forall`" |
| effectful generalisation body | "a polymorphic value must be pure to build; this body performs `...`" |
| arity or kind mismatch of two polytypes | "polymorphic types differ in their quantifiers: ..." |

## 7 What this does not do

- No impredicative instantiation (Quick Look). `id both` is refused, and
  `(id :: (forall a. a -> a) -> ...)` via annotation is the way.
- No deep skolemisation or deep instantiation.
- No polytype with `using` constraints. That is the natural next slice:
  the family's method takes the dictionaries as leading arguments after
  its captures, and the projection site resolves them.

## 8 Cinch frontend (`Bootstrap.*`): the port

Status: not started. The Rust compiler is the reference implementation
(sections 1 to 6). The self-hosted frontend still refuses a nested
`forall` in `Bootstrap.Infer.convert` with "higher-rank forall needs
explicit elaboration". No compiler module uses one, so the fixed point
does not depend on this port.

The Cinch elaborator already has the machinery the rules need. Metas
carry levels (`enter-level`, `restore-level`). `URigid` carries the level
of its scope. `Bootstrap.Type.bind-other` already refuses a solution whose
rigid variables are deeper than the metavariable (`escapes`). It also
already lowers the levels of the metavariables a solution mentions
(`lower`), which is the level adjustment that section 3 needed to add to
the Rust unifier. The port has six slices, each with its own gate.

1. **Types** (`Bootstrap.Type`). Add the constructor
   `UForall :: List Quant -> UType -> UType`, with `UBound` binders
   exactly as `Scheme` uses them. Rename it apart in `map` and
   `substitute`. In `unify-structure`, a `UForall` pair enters a level,
   opens both sides at one shared list of rigid variables
   (`open-scheme True`), unifies the bodies and restores the level.
   `bind-other` refuses a solution that contains a `UForall`. It must also
   be added to `type-text`, `valid-type`, `free-metas`, `same-shape` and
   the positivity walker.
2. **Conversion** (`Bootstrap.Infer.convert`). Convert `TForall s qs body`
   by running the same binder classification as `parameters` and
   producing a `UForall`. Refuse `TUsing` under it, using the Rust
   diagnostic text.
3. **Elaboration** (`Bootstrap.Infer`).
   - When checking against a `UForall`: enter a level, skolemise, check
     the term, require a pure row, and restore the level. Pass through a
     term that already synthesises the polytype.
   - When synthesising a `UForall`, at a variable, an application head or
     a constructor field, instantiate it with `open-scheme False`.
   - Bind a lambda parameter at the expected domain directly (already the
     shape of `bind-arrow-pattern`).
4. **Core** (`Bootstrap.Core`, `Bootstrap.Typing`). Add the two term forms
   `CPolyIntro :: Span -> UType -> List Nat -> CoreTerm -> CoreTerm`
   (polytype, skolem ids, body) and
   `CPolyElim :: Span -> UType -> CoreTerm -> List UType -> CoreTerm`.
   The independent Core checker re-types both by the same rules, so a
   Core producer can never claim a generalisation that its body does not
   support.
5. **Emitter** (`Bootstrap.RuntimeIr`). Lower `CPolyIntro` to an anonymous
   instance of `#poly.<k>`: a dictionary demand whose one method is a
   worker over the enclosing class vector followed by the skolems' classes.
   Lower `CPolyElim` to the slot projection of that vector. This is the
   same `DictionaryDemand` and method-slot path that `Bootstrap.Object`
   already lays out.
6. **Interfaces** (`Bootstrap.ExportType`, `Bootstrap.Load`). Write the
   polytype in its source spelling, as the Rust writer does (section 5),
   and read it back through slice 2.

The acceptance gate is `tests/rank_n.rs` replayed through Stage 3
(`CINCH_TEST_COMPILER`), with the four-stage fixed point unchanged.

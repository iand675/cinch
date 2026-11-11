# Mutual data groups and SCC-wide positivity

Design note for admitting mutually recursive data declarations. The
normative rule is in `specifications/core-language-specification.md`
section 2; this note records the reasoning, what the rule refuses, and how
each compiler implements it. No em dashes.

## 1 Why

The self-hosted compiler could not give `Kind` a direct `UType` field,
because `Kind` and `UType` would then be mutually recursive and the
declaration-order rule refused the pair. It serialised a `UType` into a
tree and re-read it instead: `read-carrier-tree`, `carrier-tree` and
`type-kind` were about 9% of a toolkit check and the top allocation
callers on that workload. A language whose own compiler contorts around a
missing feature is telling you the feature is not optional. Mutual data is
also what a user of an inductive language expects: a term and its
declarations, a statement and its expression, an interface and its
members, are naturally one recursive block.

Two properties are wanted at once, and they pull against each other:

- **Order independence.** A data declaration must be able to name one
  written later in the same module.
- **Soundness.** The inductive reading of a recursive type requires
  positivity, and a cycle makes the occurrence graph circular, so
  positivity can no longer be judged one declaration at a time: a
  declaration's own name is not the only name that may occur in it.

## 2 The rule

Let `G` be a strongly connected component of the module's reference graph
over its data and record declarations (an edge `T -> S` when a constructor
or field type of `T` mentions `S`). A declaration that lies on no cycle is
the one-member group `{T}`. Every constructor field type `t` of every
member must satisfy:

1. An occurrence of a member `S` of `G` is admitted only as `S a1 ... an`
   with no `ai` mentioning any member of `G`.
2. An arrow, dependent function, dependent pair, tuple, or a
   `Thunk`/`Lazy`/`Later` payload is traversed: the codomain (pair
   components, tuple components, payload) is judged at the same polarity,
   and a function or dependent function domain flips the polarity.
3. A member in a negative position is refused.
4. A member nested inside any other type constructor is refused: inside a
   type parameter (`List T`), inside a refinement's carrier, inside a
   handler's input or output, inside a member's own arguments.
5. A borrow or mutable borrow is a pointer, not a component, so a type
   behind one is not a structural occurrence and is not judged.

Clauses 1 to 3 are Dybjer's strict positivity condition for a mutual
inductive block, which is what makes the block well founded: every
recursive occurrence is reachable only through a constructor of a member,
and no member can be reached through a negative position, so no impredicative
or non-well-founded encoding slips in. Clause 1 also keeps the recursion
*regular*: a member may not be applied to an argument that mentions the
group, so a group cannot be used to encode a non-regular tree type whose
positivity argument would need size information.

Clause 4 is the honest boundary. A type former `F` is
positivity-preserving when `F X` is strictly positive in `X`; `List` and
`Option` are, a general type parameter is not known to be. Rather than
assume it, the rule refuses the nesting, exactly as the single-declaration
rule already did. Admitting nesting is the open design point named in
`design/dependent-core-kernel.md` section 5, and it stays open here.

Clause 5 records what the implementation already did: `Ref r a` and
`Mut r a` are indirections, so `record Cell where next :: Ref r Cell` is a
linked structure, not an infinite type. It is written down because the
rule is otherwise "a member may occur nowhere but strictly positively", and
an unstated exception is a bug waiting to be read as an oversight.

### What the rule refuses that a user might want

- `data Expr where Block :: List Stmt -> Expr` in a group with `Stmt`.
  Positivity-preserving nesting would admit it. Until that is decided, the
  spine encoding (`StmtCons`/`StmtEnd`) is the answer, which is what
  `Bootstrap.Ast` already does.
- A group whose cycle runs through a type that is not a data or record
  declaration of the module: a refinement, codata, an opaque type, a
  signature, or an imported type. Those declarations still resolve only
  against names declared before them, so the cycle is refused where the
  forward reference is written. The seed reports `unknown type R`; that is
  the same refusal it gave before this change.
- A cycle through a type synonym. This surface has no type synonyms, so
  the case cannot arise today; when one is added, a synonym MUST be
  expanded before groups are formed, or a cycle through it is refused.

### Why records are members

A record is a data declaration with one constructor. Treating it as a
member is not a convenience: if a record is excluded from the group, a
cycle `A -> R -> A` with a negative occurrence inside `R`'s field
(`record R where f :: A -> A`) is unchecked, because neither `{A}` nor
`{R}` contains the other and neither check looks at the other's fields.
That is unsound, so the group must include the record. Both compilers
therefore group records with data.

Codata is different: its observation discipline is guardedness, not
positivity, and a codata declaration is checked on its own with its
`Later` guards. A cycle through codata is refused by the declaration-order
rule in the seed and by the field polarity of the generated observation
storage in the self-hosted compiler, which agrees.

## 3 The seed

`compiler/src/check_decls.rs`:

- `Checker::predeclare_nominal_heads` walks the module's flattened
  declarations before any type is converted and registers every `data` and
  `record` head: name, parameter names and kinds, and for a record its
  field names and field erasure. A duplicate name is refused there, which
  is where the per-declaration branches used to refuse it.
- The same pass builds the reference graph from the declaration bodies
  (`data_type_references`, a syntactic walk that ignores qualified names
  and captured-scope names) and runs Tarjan (`strongly_connected`) over it.
  Every component with a cycle is recorded in `Checker::data_groups` as its
  members' nominal identities; a declaration on no cycle has no entry and
  `Checker::data_group` returns the singleton.
- `check_group_positivity` is `check_positivity` with the family replaced
  by the group. The diagnostic is unchanged for a one-member group; for a
  cycle it names `the recursive group {M.A, M.B}`.
- The data branch and the record branch both call it. The record branch
  did not check positivity before, so `record R where f :: List R` is now
  refused; the corpus has no such declaration.

Tarjan is iterative because the depth is a declaration count and a
recursive implementation would make the recursion depth a source property.

## 4 The self-hosted compiler

Status: this section is the specified change, not a landed one. The seed
side in section 3 is implemented, tested and green; the self-hosted edits
below are written and reviewed as a patch against
`bootstrap/Bootstrap.Infer.cinch` but are not applied in this lane, so no
stage has been built with them. Section 6 therefore records no numbers.

`bootstrap/Bootstrap.Infer.cinch`:

- `collect-kinds` becomes three passes. `collect-data-heads` registers every
  data name with its arity and `KType` parameter kinds, `collect-data-kinds`
  replaces each with the checked kind (a parameter kind may itself name a
  data declaration, so the arity must exist first), and
  `collect-kinds-rest` is the old walk with the data branch removed. A
  forward reference inside a constructor type therefore resolves, because
  constructor types convert in `register`, after the kind context is
  complete.
- `data-group-of` computes the group by mutual reachability
  (`reachable ds a b && reachable ds b a`), which is the strongly connected
  component by definition. The declaration counts are small, and the
  definition is easier to review than a functional Tarjan.
- `positive` takes a group instead of an owner: `occurs-group` in every
  place `occurs-owner` appeared, `group-member` at the head position, and
  two new clauses that refuse a group occurrence inside a refinement
  (`URefineFormer`, `URefine`) or a required proposition (`URequire`)
  carrier.
- `register-ctors` and `check-positive` take the group; `register`'s data
  branch computes it. `register-imported-codata` passes a one-member group.
- `data-order` is deleted. It refused every forward data reference because
  SCC-wide positivity did not exist; the group-aware `check-positive` now
  decides, and a forward reference that forms no cycle is judged by its
  one-member group exactly as before.
- `needs-inference` gains `recursive-data-needs`: a data declaration whose
  constructor arguments name a data type of the module takes the elaborated
  path, because the fast path has no positivity check at all. This is what
  makes the discipline apply to every data declaration rather than only to
  the ones the elaborated path happened to see. A declaration with no
  constructor arguments naming a data type (an enumeration) keeps the fast
  path.

## 5 The interface

No schema change, and no group marker. The reasoning:

- A group is a property of one module's declarations. It is established
  when that module's source is checked, and the interface is emitted by the
  compiler that checked it. A consumer never re-checks a defining module's
  positivity, so there is nothing for the consumer to reconstruct.
- The reader needs order independence, not the group. An interface's type
  rows are emitted in declaration order, each constructor row carrying a
  scheme as text. The self-hosted reader registers every imported type name
  in `collect-kinds` and parses constructor schemes in later passes
  (`checked-data`, `register-external`), so a constructor scheme naming a
  later row of the same interface resolves. Parameter kinds are the one
  order-sensitive part: a group member whose *parameter kind* names another
  member would need that member's kind first. That is the same limitation
  as any imported type whose parameter kind names a type from a later
  interface, it is not new, and no bootstrap group uses a parameterised
  member. It is recorded here rather than papered over.
- The seed never reads an interface back into its type table; it re-checks
  source. Its `.cinchif` rows are the same shape and the same argument
  applies.

So the schema version is unchanged and the pins are untouched.

## 6 Measurements

None yet: the step that would pay for the feature (giving `Kind` a direct
`UType` field) needs the self-hosted change in section 4 first. The
baseline it will be compared against exists: `build/bootstrap/stage1`
built from the current sources, and `tooling/run_stage_targets.py --stage
1 --out /tmp/mutual-data/run-1` over `dependent hk goal_tactics
bootstrap_refinement_closure bootstrap_load bootstrap_infer
bootstrap_typing prim ownership` (3 of 9 targets pass at that baseline;
the per-test pass/fail list is in that directory's `summary.json`).

## 7 What this does not decide

- Positivity-preserving nesting, so `List T` inside a group stays refused.
- Mutual codata groups and their guardedness interaction.
- Type synonyms, which the surface does not have.

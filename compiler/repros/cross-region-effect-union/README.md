# Independent effect region inference

The original public Var and HashTable probes declare both `Mut r` and `Mut s`
but are rejected by two independent Rust compiler epochs. Their exact sources,
b004 observations and the current 37dc diagnostic are retained here. To run
these standalone files, copy the actual `Data.Mutable.Var.cinch` and
`Data.Mutable.HashTable.cinch` library modules beside them. The Cargo regression
creates that source project automatically.

Effect accumulation used the row-unification matcher, which committed flexible
region identities just because two performed labels had the same head. That
prematurely identified the second call's region with the first call's owner.
An effect union must record both identities until argument/result constraints
say whether they coincide.

The first non-unifying union candidate exposed a second solver limitation:
row matching treated labels as one-to-one occurrences even when unification
made two flexible occurrences the same set member. `Effect.Clock` exposed it
before the intended consumer checked. The exact initial patch, test harness,
source hashes and failure log remain here. Its initially broad negative-test
assertions were also masked by that dependency failure; they are not credited
as exercised refusals. The corrected tests require the error to come from the
consumer module.

The final solver first pairs equal resolved identities, then solves flexible
partners, and canonicalizes any labels that become equal. Distinct rigid owners
remain distinct. Direct solver tests cover both row orders, independent later
bindings, duplicate collapse and missing rigid owners. Actual Var/HashTable
consumers exercise ordinary/local/curried calls, result-selected regions and
open callback rows; negative consumers cover missing/pure/relabelled effects
and wrong storage result identities. Broader parent results are recorded
separately after all required checks finish.

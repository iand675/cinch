# Deferred generic NonZero formation — WIP

This checkpoint extends the preceding anonymous-contract candidate with an
internal pending predicate for generic `NonZero a` contracts. It addresses the
actual generic Div/Rem header-formation blocker in the compiler bootstrap.

An abstract carrier may retain its structural predicate until specialization.
The Core arithmetic reifier gives that pending node no arithmetic meaning.
Normalization checks a concrete supported carrier and keeps machine-integer
promotion explicit. Bool/float carriers and escaping or mismatched identities
remain invalid; a printed type name never establishes a numeric carrier.

The compiled-Cinch type/Core runner passes 51 checks at five runtime identities
above 2^128. These include pending formation, specialization, codecs, scope,
rejection of unsupported carriers and absence of abstract arithmetic facts, plus
all 31 preceding NonZero checks. A separate test with eleven actual Native-produced
Div/Rem interface consumers is still pending. The broader anonymous-contract
validation also remains in progress on its own frozen snapshot.

[abstract-nonzero-wip-results.json](abstract-nonzero-wip-results.json) records this
checkpoint's scope and source identities. This is private development made
reviewable on GitHub, not an integrated compiler or a refreshed seed. The last
installed checkpoint remains `codex/cinch-bootstrap`; stage2/3 acceptance and the
complete suite through a self-built frontend remain unfinished.

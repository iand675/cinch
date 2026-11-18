# Multipart both-effect review experiment

Production remains the published `f0e8fc8c` source checkpoint. `baseline/`
freezes its exact Text/Effect source bytes; `sources/` is a review candidate.
No shared MIME, Stream or Buffered module is changed. This experiment is not a
runtime, ABI-object, performance or package-completion claim.

The candidate adds three factories:

| Factory | Callback effects | Construction effects |
| --- | --- | --- |
| `multipart-source-with-mutation` | `IO r, Mut r` | Pure |
| `multipart-sink-with-mutation` | `IO r, Mut r` | Pure |
| `multipart-storage-with-mutation` | `IO r, Mut r` for all eight capabilities | `Mut r` |

The original IO-only factory types remain unchanged and callable. Explicit
widening clients show how to use their IO-only callbacks with the new richer
factories. They do not erase mutation from a richer callback.

The concrete compatibility cost is **two existing public effect rows**:
`multipart-source-read` and `multipart-sink-write` now require both named
capabilities. A third changed row, `random-read`, is private. Existing raw-read
or raw-write callers with only `IO r` must gain `Mut r`, even when their factory
was IO-only, because the opaque source/sink type can now hold either variant.
Reader/writer/part/form/file operations already carry both effects. All eight
storage dispatch operations also already carry both; their new alternatives
preserve exactly one callback and its advanced state. The complete before/after
signatures and source hashes are in `signature-impact.json`.

The new private constructors retain live callbacks directly. Real Part/File
readers and output-part writers are forwarded without eager copying, fake
cursors, error flattening, effect casts or closing borrowed handles. Their
callback prefixes/statuses and short-write progress remain independent. The
same `r` names IO and mutation, matching the native session API. Arbitrary
storage error/key/handle/state types remain explicit.

`clients/positive.cinch` checks actual live Part/File/output-part forwarding,
the complete new storage dictionary, all original factories and explicit
IO-only widening wrappers. Both qualified Rust855/c4 and admitted source-selected
Native2f/V2 accept it. Four negative clients per frontend refuse the unchanged
IO-only factories when callbacks require mutation. All ten source/type/proof
outcomes and exact diagnostics are retained in `gates.json`. No handles are
fabricated or callbacks executed by these source-only gates. Canonical parsing
checks all 235 candidate Effect declarations, including private constructors,
with zero diagnostics or orphan comments.

The first Native command incorrectly passed `--objects` to `--check`; its early
invocation failure is preserved and is not treated as a compiler rejection.
Removing only that unsupported argument admits the identical positive source.
The last intended Native storage refusal uses an exact IO-versus-IO+Mut type-row
diagnostic; the original harness classification failure is preserved, with only
that named diagnostic added to the expectation. No source or compiler guard was
weakened. Generated cold check caches are recoverably archived with full member
and compressed/decoded SHA/size/mode checks, keeping actual sources and client
inputs plain. An initial transient 5.40 MiB bound violation was stopped and
retained; verified recovery/dedup brings the final check artifacts below 5 MiB.

Root reviews the effect and representation impact before selecting production
source. Selection requires fresh runtime/ownership/short-I/O gates, changed
header/escape checks, source-absent compiler-boundary review and appropriate
performance requalification. Existing full-corpus and benchmark evidence stays
attached to its original production source epoch.

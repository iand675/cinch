# Executable library objects

The compiled-interface design requires executable library bodies alongside
checked source contracts. A pure function scheme cannot supply a body, and
two independently registered runtime layout tables cannot safely share values.

## Implemented producer boundary

The Rust compiler's `emit-object` command emits one closed library dependency
object, LLVM text and a canonical JSON physical manifest. It uses the ordinary
source loader, checker, interface coherence checks, specialization and ownership
pass. A library with a process entry is rejected. This is currently a dependency
bundle; it is not independent per-module incremental object production.

The object defines its actual uniform-word units and exports a closure accessor
for each callable unit. Static closure storage remains private. It leaves
`__apply` to the final consumer, emits coalescible memo helpers, and neither
registers a layout table nor enables reclamation at initialization.

Physical manifest schema 1 records:

- Runtime ABI, target triple, word width, endianness and reclamation mode.
- Actual unit symbols, arities, parameter word classes, known result classes
  and closure accessor symbols. An unknown result remains null. Missing or
  unresolved parameter annotations are rejected. Parameter-independent data
  constructors may share a concrete unit across type arguments.
- Named type placement, actual constructor descriptors and tags, dictionary
  method order and stored field classes. These facts grant no source names.
- The exact final packed layout table, including pointer-map extension words
  and their absolute indices, plus the descriptor-row count and escape flag.
- The complete checked interface closure and its interface identities, plus
  deterministic content hashes of the LLVM text and native object. These use
  the existing interface FNV content hash, not a cryptographic digest.

Source exports and abstraction remain governed by the checked interfaces.
For example the physical manifest can describe private `Data.String.StrBuf`
without exporting its constructor to source programs. String remains a wrapper
owning a byte buffer, not an alias for Bytes.

## Consumer requirements

A consumer must validate target/runtime compatibility, artifact bytes and the
matching checked interface closure before using the physical facts. Artifact
admission should additionally record cryptographic hashes for the object and
interface inputs. A null result class cannot authorize ownership operations.

The recorded table is an immutable prefix of the final process table. New
consumer layouts must start beyond the entire prefix, including extension
words. For regular descriptor allocation, round its word count up to a group
boundary first. Register the combined table once, before executing either
producer or consumer code. Appending layouts must preserve every prefix word
and therefore all absolute extension indices. Separately produced bundles
cannot simply be linked if their descriptor spaces overlap; they need one
shared allocation plan or checked relocation.

All calls must use a representation variant actually emitted by the producer,
with its real context order and checked class method family plan. A source
export does not imply that every polymorphic variant exists. Private layouts
must not be guessed from source names or public constructor subsets.

## Remaining bootstrap integration

The Cinch backend does not yet consume this manifest, preseed its layout
allocator, emit external calls or reconcile imported class method families with
the producer. Explicit demand roots for library specialization remain needed;
the current producer uses the existing reached-variant collector. Ordinary
`build` also does not yet link supplied dependency objects.

Bootstrap integration must admit only the intended library/runtime dependency
closure. No Rust frontend or compiler implementation object may substitute for
compiling `Bootstrap.*` with the preceding Cinch compiler. Genuine Stage2,
Stage3 reproduction, changed-source behavior and the complete test/correctness
gates remain separate requirements.

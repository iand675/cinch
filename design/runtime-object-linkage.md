# Executable library objects

The compiled-interface design requires executable library bodies alongside
checked source contracts. A pure function scheme cannot supply a body, and
two independently registered runtime layout tables cannot safely share values.

## Implemented producer boundary

The Rust compiler's `emit-object` command emits one LLVM object and a canonical
JSON physical manifest. It uses the ordinary source loader, checker, interface
coherence checks, specialization and ownership pass. A library with a process
entry is rejected. The package installer passes the package's own module set, so
its object defines only those modules and generated primitive helpers; every
other module is emitted as an external declaration. Dependency objects are
loaded before the root in JIT. Native `build` and `run` accept package objects and emit only the project modules plus external declarations. The default `emit-object` without a module filter still emits the historical whole-closure dependency bundle, which is not independent per-module incremental object production.

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
- Class method families produced by the specialization collector: declaring
  class identity, declared method count, and the logical slot, physical slot
  and own representation vector of each emitted method version. Every factory
  of the class uses this same table.
- The exact final packed layout table, including pointer-map extension words
  and their absolute indices, plus the descriptor-row count and escape flag.
- The complete checked interface closure and its interface identities, plus
  deterministic content hashes of the LLVM text and native object. These use
  the existing interface FNV content hash, not a cryptographic digest.

## Explicit library demands

`emit-object` accepts `--object-roots roots.json` to retain variants that a
separately compiled consumer needs. The file uses schema 1:

```json
{
  "schema": 1,
  "units": [{"symbol": "Library.choose", "classes": "T"}],
  "dictionaries": [{"symbol": "Library.PolyInt", "classes": ""}],
  "methods": [{"class": "Library.Poly", "slot": 0, "own": "T"}]
}
```

Vectors contain only `R`, `O` and `T`. Units and dictionaries use their defining
symbols and actual type-parameter vector lengths; method requests name the
declaring class, logical slot and method's own vector. Validation checks method
requests against the producer's actual defining instance contracts. Unknown
roots, wrong vector lengths, unresolved classes and unknown fields are rejected.
An object whose transitive collection leaves unrecorded representation arguments
is refused rather than filling missing runtime authority with Raw.

Demands seed the existing finite collector before its normal root scan. They
do not enumerate all possible vectors. Requested roots and their transitive
dependencies are retained; parameter-independent constructors still share one
unit. Method vectors are sorted canonically, the first keeps the declared slot,
and further vectors append after the declared methods. The emitted manifest
records this actual plan. Runtime method sites and factories use the declaring
module's canonical class identity, including through aliases and imports.

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

## Cinch consumer and stage integration

The Cinch command-line consumer accepts one closed bundle:

```text
cinch --emit-ir --interfaces interfaces.json --objects objects.json source.cinch source.ll
```

The object configuration uses schema 1:

```json
{
  "schema": 1,
  "target": "aarch64-apple-darwin",
  "object": "/absolute/path/library.o",
  "metadata": "/absolute/path/library.abi.json",
  "object_sha256": "actual SHA256 of library.o",
  "metadata_sha256": "actual SHA256 of library.abi.json"
}
```

The target must be the producer's actual target. The consumer checks both
cryptographic identities, the configured runtime ABI, ownership mode and word
format, the independently checked interface closure and physical pointer maps.
It seeds the complete layout prefix before allocating its own descriptors.
Rejected admission or lowering clears stale IR.

External values require checked value-export authority; instance factories
require checked instance-export authority. Actual inventory membership alone
does not introduce a source name. Calls validate concrete parameter/result
classes and use actual emitted function variants, closure accessors and class
method families. Physical arity is distinct from arrows exposed by instantiating
a function-valued type parameter. Remaining arguments apply to the returned
function. String literals use the configured ABI's checked private wrapper
descriptor over an owned byte buffer; the constructor stays private.

The stage driver separately pins the library-only producer in
`bootstrap/library-provider.lock.json`. This pin is independent of stage 0.
It builds a generated trivial library root and only its base-package closure,
plus compiler-generated primitive and tuple helpers. It records the producer,
object, metadata, checked interfaces and matching runtime archive identities.
Later stages invoke only the preceding Cinch compiler on the compiler bundle,
then link its IR with this library object and runtime. Source preparation keeps
one explicit ABI: current live builds use ABI6, while the unchanged pinned stage
path prepares ABI5 and selects the matching ABI5 provider.

`bootstrap/library-object-roots.json` supplies concrete dependency demands.
Failed stage emission additionally writes `requested-roots.json`, containing
only actual missing function, factory and method vectors reported by retained
Core lowering. These requests must be checked by the producer before another
attempt; no Cartesian enumeration or unresolved Raw substitution is used.

Ordinary `build` does not yet consume supplied dependency objects. Full
compiler emission, canonical compiler interface output, remaining runtime
forms and the complete suites/correctness obligations remain bootstrap work.

Bootstrap integration must admit only the intended library/runtime dependency
closure. No Rust frontend or compiler implementation object may substitute for
compiling `Bootstrap.*` with the preceding Cinch compiler. Genuine Stage2,
Stage3 reproduction, changed-source behavior and the complete test/correctness
gates remain separate requirements.

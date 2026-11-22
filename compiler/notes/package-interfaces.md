# Packed compiler interfaces

A `.cpk` file carries a complete schema-10 interface set in a read-only
package. It keeps one string table and one copy of each checked body
definition, even when many exports reference the same definitions. The
contract digest, total bodies, typed schemes, layouts, laws, and dependency
identities have the same meaning as in the per-module JSON files.

The Rust compiler writes the package directly from its checked contracts and
body trees while emitting a standalone library object.

```sh
cinch emit-object Library.cinch --interface-package-only -o library.o
```

The fresh-base workflow uses the same writer. `tooling/build_base_artifact.py`
records the producer's `library.cpk` in its receipt. A bootstrap using
`--library-artifact` takes that package as its base artifact after checking
its digest and public module identities.

Use the staged package and linker configuration to compile a client:

```sh
"$stage/cinch" --check-types --interfaces "$stage/base.cpk" Hello.cinch hello
"$stage/cinch" --emit-ir-only --interfaces "$stage/base.cpk" \
  --objects "$stage/base-objects.json" Hello.cinch hello.ll
```

The provider's object ABI lives in a binary section of the same package.
`base-objects.json` records the target, paths, and digests used to authenticate
the package and the compiled library object.

The Rust compiler accepts a packed interface sidecar for package objects
alongside their current execution metadata. For an installed package:

```sh
python3 tooling/pack_interfaces.py "$installed/build/package.cinchif" \
  "$installed/build/package.cpk" --execution
cinch build Main.cinch --package-object "$installed/package.o"
```

The Rust object admission path checks the typed contracts against the package
execution metadata. The seed still checks its source closure. Existing
per-module JSON files and native JSON manifests remain supported.

The readers decode structured values directly from the package sections and
verify the same interface identities and contracts as the JSON readers.
The package carries complete contracts without a JSON compatibility payload.
The writer replaces the file by rename so an existing mapping keeps seeing
its original bytes. Keep published package files immutable while mapped.

## File format

All words are unsigned 32-bit little-endian. The 88-byte header holds the
eight-byte `CINCHPK4` magic, total file length, format flag 4, and nine
offset/length pairs. The section pairs are file-relative and define
contiguous ranges in this order:

| Section | Data |
| --- | --- |
| Strings | Entry count, one offset/length pair per ID, and packed UTF-8. String spans elsewhere are relative to the packed UTF-8. |
| Modules | Sorted 36-byte rows. The words carry name and digest spans, value range, contract span, and an inverse index from original input rank to sorted row. |
| Values | Sorted 32-byte rows within each module. The words carry name, symbol and scheme spans, arity, and root body ID. `0xffffffff` denotes an absent body. |
| Bodies | Sorted 32-byte rows. The words carry identity and result sorts, parameter and edge ranges, and a root node ID. |
| Edges | Body IDs packed as words. |
| Nodes | Checked body nodes, in postorder. |
| Operands | Parameter sort IDs and lists of argument node IDs. |
| Contracts | Typed documents preserving the schema-10 field names and values. |
| ABI | Optional structured metadata. Public interface metadata references its packed contract by name and digest. Private ABI-only contracts appear in this section once. |

Contract values begin with a tag word. Tags 0 through 2 are null, false,
and true. Tags 3 and 8 carry unsigned and negative integers with a low and
high magnitude word. Tag 4 carries a string ID. Tags 5 and 6 carry an item or
field count and their values. Object keys are string IDs in strict byte
order. Tag 9 carries a checked body's root definition ID. Structured
values have a maximum nesting depth of 256.

### Indexed body format

A body row holds its identity string ID, result sort string ID, parameter
start and count, root node ID, call edge start and count, and a reserved
zero word. Parameter ranges index sort string IDs in Operands. Edge
ranges index sorted distinct body IDs in Edges.

Each node consists of a tag and three unsigned words.

| Tag | Node | First word | Second word | Third word |
| --- | --- | --- | --- | --- |
| 0 | Local | Lexical slot | 0 | 0 |
| 1 | Number | Sort string ID | Decimal string ID | 0 |
| 2 | Boolean | 0 or 1 | 0 | 0 |
| 3 | Primitive | Operation string ID | Argument start | Argument count |
| 4 | Call | Target body ID | Argument start | Argument count |
| 5 | Let | Value node ID | Body node ID | 0 |
| 6 | If | Condition node ID | True node ID | False node ID |

Argument ranges index node IDs in Operands. Each definition occupies a
contiguous interval in Nodes. Its subtrees appear in source operand order
before their parent, and each node belongs to exactly one definition tree.
References between definitions use Call nodes. A contract's entire checked
body graph is tag 9 followed by one body ID. Its member definitions follow
from the call edges and are ordered by identity for the schema-10 digest.

Rust package-object admission checks sorts, lexical slots and call edges
directly in the packed records. It walks closures by definition ID and hashes
the same canonical contract without allocating expression trees. An explicit
request for an owned definition materializes that closure on demand. Native
Cinch validates the node sorts, lexical scope, and call graph before
allocating expression trees. It builds a shared compatibility tree only
for a definition selected by a contract's body reference. Its total-body
queries use checked node
IDs and operand spans against the retained package bytes when the
definition matches the admitted row. This includes the refinement checker
and the LLVM inliner for imported primitive bodies. Graphs that match the
checked package rows reuse their signatures and edges. A projected closure
builds a JSON definition array only when a caller requests it. During
package decode, repeated references to one packed root share its closure
and compatibility document. Each reference still hashes its graph at the
current contract position. The reader hashes
plain strings and indexed body graphs from their file records. Escaped text
and legacy shared definitions use the canonical value writer.
For packages that repeatedly hash large checked closures, the native loader
keeps temporary FNV-1a summaries of contiguous body rows. A summary preserves
the same canonical digest for any preceding contract fields. Each row in a
cached range is serialized at most once during package decode.

An execution interface can supply `interface_refs` in place of embedded
`interfaces`. Each row supplies the module name and contract digest. This
form requires the matching packed sidecar.

For native client compilation the object configuration can authenticate
ABI metadata containing `interface_refs` and an optional single
`private_interfaces` entry. The public rows resolve against the checked
contract set in the recorded order. The private entry carries its document
and must satisfy the existing standalone object interface checks.
### Structured ABI format

The optional ABI section starts with the
number of string IDs used by the interface contracts, followed by a
structured value. It shares the contract string table and appends ABI-only
IDs after the contract strings. Interface readers skip that suffix.
Private ABI-only contracts are encoded as ordinary values. The ABI section
never uses the shared definition or body root tags.

An object configuration can name the package itself as its metadata
artifact. Its SHA-256 then authenticates the entire package before the
compiler reads the ABI section. The bootstrap emits this configuration as
`base-objects.json` and uses it with `base.cpk` for native self-builds.
When a native client's interfaces and object configuration name the
same package path, it reads one bounded snapshot and reuses the decoded
contract strings for ABI admission. It verifies the digest of that
snapshot before decoding the ABI. Interface-only loads also read a bounded
snapshot so that checked body references can retain their backing bytes.

## Verification

```sh
python3 tooling/verify_package_interfaces.py --native "$stage" --seed "$seed"
```

The gate checks valid and invalid Hello World programs using the staged and
repacked inputs, compares emitted LLVM IR, requires a clean leak line, corrupts
package sections and identities, and checks a real installed Rust package.
Native timings use fresh processes with a warm filesystem cache.
The native bootstrap writes `base.cpk` and uses it as the base contract
input for self-compilation. Stage3 verification records its digest and
passes it to the compiler for tests that use the standard base library.

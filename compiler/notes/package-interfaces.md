# Packed compiler interfaces

A `.cpk` file carries a complete schema-10 interface set in a read-only
package. It keeps one string table and one copy of each checked body
definition, even when many exports reference the same definitions. The
contract digest, total bodies, typed schemes, layouts, laws, and dependency
identities have the same meaning as in the per-module JSON files.

To generate a package from a native compiler's existing manifest:

```sh
python3 tooling/pack_interfaces.py "$stage/base-interfaces.json" base.cpk \
  --objects "$stage/objects.json"
"$stage/cinch" --check-types --interfaces base.cpk Hello.cinch hello
"$stage/cinch" --emit-ir-only --interfaces base.cpk \
  --objects "$stage/objects.json" Hello.cinch hello.ll
```

The `--objects` argument to the packer is optional. It carries the provider's
ABI metadata in a separate section. The native object linker still verifies
the configured metadata digest, binary digest, and target through
`objects.json`. The metadata in the package does not grant link authority.

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

The readers map the file and decode structured values directly from its
sections. They verify the same interface identities and contracts as the
JSON readers. The current checkers materialize their internal semantic trees,
and native Cinch materializes each interned string once. The package carries
complete contracts without a JSON compatibility payload. The writer replaces
the file by rename so an existing mapping keeps seeing its original bytes.
Keep published package files immutable while mapped.

## File format

All words are unsigned 32-bit little-endian. The 80-byte header holds the
eight-byte `CINCHPK2` magic, total file length, format flag 2, and eight
offset/length pairs. The section pairs are file-relative and define
contiguous ranges in this order:

| Section | Data |
| --- | --- |
| Strings | Entry count, one offset/length pair per ID, and packed UTF-8. String spans elsewhere are relative to the packed UTF-8. |
| Modules | Sorted 36-byte rows. The words carry name and digest spans, value range, contract span, and an inverse index from original input rank to sorted row. |
| Values | Sorted 32-byte rows within each module. The words carry name, symbol and scheme spans, arity, and root body ID. `0xffffffff` denotes an absent body. |
| Bodies | Sorted 32-byte rows. The words carry identity, result sort and recipe spans, plus an edge range. |
| Edges | Body IDs packed as words. |
| Recipes | Typed definition records, with no definition references in a recipe. |
| Contracts | Typed documents preserving the schema-10 field names and values. |
| ABI | Optional JSON metadata. Public interface metadata references its packed contract by name and digest. Private ABI-only contracts appear in this section once. |

Contract values begin with a tag word. Tags 0 through 2 are null, false,
and true. Tags 3 and 8 carry unsigned and negative integers with a low and
high magnitude word. Tag 4 carries a string ID. Tags 5 and 6 carry an item or
field count and their values. Object keys are string IDs in strict byte
order. Tag 7 carries a count of shared definition spans followed by
`(first ID, count)` pairs that expand in the recorded order. A contract or
recipe has a maximum nesting depth of 256 values.

### Indexed body format

The indexed format uses `CINCHPK3`, version 3 and an 88-byte header.
Its directory has nine sections. The Nodes and Operands sections replace
Recipes in the same position. Other section layouts are unchanged.

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

The packer CLI writes version 3 by default. Pass `--legacy-recipes` for
version 2. The readers consume the mapped records and materialize one
shared tree per definition. Native Cinch uses the contract document shape
for its checker and canonical contract digests.
An execution interface can supply `interface_refs` in place of embedded
`interfaces`. Each row supplies the module name and contract digest. This
form requires the matching packed sidecar.

For native client compilation the object configuration can authenticate
ABI metadata containing `interface_refs` and an optional single
`private_interfaces` entry. The public rows resolve against the checked
contract set in the recorded order. The private entry carries its document
and must satisfy the existing standalone object interface checks.
The packer's ABI section already carries metadata in this form.
An object configuration can name the package itself as its metadata
artifact. Its SHA-256 then authenticates the entire package before the
compiler reads the ABI section. The bootstrap emits this configuration as
`base-objects.json` and uses it with `base.cpk` for native self-builds.

## Verification

```sh
python3 tooling/verify_package_interfaces.py --native "$stage" --seed "$seed"
```

The gate checks valid and invalid Hello World programs through both native
input paths, compares emitted LLVM IR, requires a clean leak line, corrupts
package sections and identities, and checks a real installed Rust package.
Native timings use fresh processes with a warm filesystem cache.
The native bootstrap writes `base.cpk` and uses it as the base contract
input for self-compilation. Stage3 verification records its digest and
passes it to the compiler for tests that use the standard base library.

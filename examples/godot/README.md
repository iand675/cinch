# Godot development in Cinch

This project builds a Godot 4 GDExtension containing native Cinch classes and
an editor scripting language. `Mover.cinch` attaches to ordinary `Node2D`
nodes; `Tool.cinch` runs in the editor; `ResourceLogic.cinch` attaches to
reference-counted resources. `GameLogic.cinch` implements a native
`CinchSpinner` class. `ValueLogic.cinch` exercises the value bridge.

GDExtensions run in a normal Godot installation. Changing Godot's engine source
means compiling a custom engine and is a separate integration.

## Build and open

Prerequisites: the Cinch compiler toolchain, a C++17 compiler, Git, SCons
(`python -m pip install --user scons`), and Godot 4.5 or later.

From this directory:

```sh
cinch cxx build bindings.json --out-dir build
godot --editor --path .
```

The first build fetches the pinned godot-cpp 4.5 SDK and builds the selected
engine classes, value bridge, script provider, and library. An existing SDK
can be selected with `--godot-cpp /path/to/godot-cpp`, or `godot.godot_cpp` in
the specification. The SDK must be 4.5 or newer; its API version becomes the
minimum compatible Godot version. Builds target the host's Linux or macOS
architecture and the standard single-precision Godot ABI.

Open `Mover.cinch` in Godot's Script workspace to edit it with Cinch syntax
highlighting. It recognizes keywords, types, numbers, operators, strings,
characters, tick names, line comments, and nested block comments. Attach it to
a `Node2D` through the Inspector, or open `mover.tscn`. Its exported properties
are editable in the Inspector and saved in scenes. Signals appear in Godot's
signal metadata and support ordinary connections.

Cinch scripts are compiled ahead of time. Save source, rerun the build command,
and restart Godot to load the new revision. The editor reports a rebuild is
needed when source differs from the compiled snapshot. Live native reload,
Cinch breakpoints, semantic completion, and script-to-script inheritance are
not implemented. Adding a new attached script also requires a class entry in
`bindings.json`; the editor does not infer that entry from source.

The build writes public headers, generated Cinch export entries, a
`.gdextension` resource, the library, and `native-build.json` with SDK and
input identities. It registers the extension in `.godot/extension_list.cfg`,
preserving other extensions. Libraries are replaced atomically; live reload
is disabled while instances can retain native callbacks.

## Attached scripts and native classes

Both use `export_classes` in [bindings.json](bindings.json). Set
`"script": "Mover.cinch"` to make a class an attached script. This path is
relative to the project containing `bindings.json`, must stay inside it, and
must identify a unique `.cinch` file. It is automatically included in the
compilation. The other `sources` contain additional implementation modules. These also open
with highlighting and validate against their compiled source snapshots; they
do not become attachable script classes.

A class needs a Cinch `name`, a C++ state type `cpp`, and a supported `base`:
`godot::Node`, `Node2D`, `Node3D`, `Control`, `Resource`, or `RefCounted`, each
qualified with `godot::`. Script state does not subclass the engine object;
the provider attaches it to that object's script instance. Omitting `script`
creates a registered native C++ class inheriting the base instead.

Fields become inspector properties and generated Cinch getters/setters.
Methods receive a leading `Borrowed <class>` receiver and their declared
arguments. `_ready`, `_process`, `_physics_process`, and `_notification` can
be exposed as script methods. Native classes use `override: true` for actual
C++ virtual callbacks, checked against the SDK. `native_methods` imports
selected inherited methods for Cinch to call, as demonstrated by `translate`.

At runtime, load an attached script and call `new()`, attach it with
`node.set_script(script)`, or instantiate a scene containing it. Resource
instances use Godot reference counting. Every instance owns independent state;
freeing it invokes its Cinch destructor hook exactly once. Regular scripts
use editable placeholders in the editor without executing their callbacks.
Set `tool: true` to run the constructor, methods, and callbacks in the editor.

## Values and ownership

Godot fields and method signatures accept numeric scalars and these additional
types:

| JSON type | Cinch parameter | Cinch result |
| --- | --- | --- |
| `Bool` | `Bool` | `Bool` |
| `String`, `StringName` | native Cinch `String` | native Cinch `String` |
| `Vector2/3/4`, `Vector2i/3i/4i`, `Color` | generated `GDVector2` etc. records | same records |
| `Variant`, `Array`, `Dictionary` | `Borrowed GDVariant` etc. | `Owned GDHeap GDVariant` etc. |
| `Object` | `MaybeNull (RawPtr GDObject)` | same pointer type |

The C++ adapter handles the Godot value ABI. Cinch strings and vector records
are converted automatically; temporary holders are deleted on return. Opaque
results are linear owners: consume them with `gdVariantDelete`, `gdArrayDelete`,
or `gdDictionaryDelete`. Copy helpers create a new owner; borrowed parameters
must not escape the call. `Owned` is checked by the normal Cinch ownership
checker. An echo of a borrowed Variant returns `gdVariantCopy value`.

Arrays and dictionaries retain Godot reference semantics. `gdArrayCopy` and
`gdDictionaryCopy` copy holders; `gdArrayDuplicate` and `gdDictionaryDuplicate`
make deep collection duplicates. Helpers provide checked indexing, mutation,
keys, sizes, clearing, and removal. Variants can carry every Godot value,
including resources, Callables, transforms, and packed arrays. Typed constructor
and accessor helpers cover the table above, plus integers, floats, booleans,
and objects. `gdValueString` and `gdReadValueString` convert native Cinch strings
to and from Variant owners. A mismatched checked accessor or invalid index
faults through Cinch's runtime instead of dereferencing invalid memory.

`gdObjectCall object method arguments` dynamically invokes any engine method
using a native Cinch method name and borrowed `GDArray`, returning an owned
`GDVariant`. `gdObjectGet`/`gdObjectSet` access engine properties. Godot owns raw
object pointers; use `gdObjectId`/`gdObjectFromId` when retaining an identity
across frames so a freed object can resolve to null. Pointer arguments must
still refer to live objects during a call. Godot integers are signed 64 bit;
use that range for values exposed to Godot.

Field initializers accept JSON booleans, Unicode strings, component arrays for
vectors/colors, and recursive JSON values for Variants/collections. Collection
defaults are constructed separately for each instance.

## Signals

Declare `signals: [{"name":"changed","params":["I64","String"]}]` on a class.
The generator exposes `emit<Class>Changed self count label` to Cinch. Emitting
with no listeners is supported. `get<Class>Object self` accesses the engine
object for signal and dynamic-call helpers. `get<Class>ObjectId self` supplies
the engine identity directly.

`gdSignalConnect source signal receiver method flags` uses native Cinch strings
for the signal and method names and returns Godot's error code. Flags are
Godot's `CONNECT_*` integer flags. `gdSignalDisconnect` and `gdSignalConnected`
take the same arguments (their flags argument is ignored). Connections use
Godot Callables and object IDs, so Godot handles deferred delivery and freed
receivers; Cinch closures are not retained as callbacks. GDScript, scenes, and
the editor can also connect these signals normally.

## Verification

```sh
godot --headless --editor --path . --import
godot --headless --path . --script test.gd --quit-after 120
```

The runtime checks print `CINCH_GODOT_OK`. They cover rich values, collections,
dynamic calls, attached scenes, lifecycle hooks, resources, tool scripts, and
signal delivery. The optional `addons/cinch_verify` plugin checks real editor
behavior, inspector serialization, tool execution, and automatic syntax
highlighting, then exits. The Rust integration test enables it in a temporary
project:

```sh
CINCH_GODOT=/path/to/godot CINCH_GODOT_CPP=/path/to/godot-cpp \
  cargo test --test cxx_exports -- --ignored
```

`Lifecycle.cpp` only instruments destruction counts; gameplay stays in Cinch.
Direct native symbol imports remain available alongside these Godot adapters.

# Godot gameplay implemented in Cinch

This is a runnable Godot 4 GDExtension: the generated `CinchSpinner` class
derives from `Node2D`, exposes an inspector property and callable methods,
handles `_process` in Cinch, and calls Godot's inherited `rotate` method.
`Lifecycle.cpp` is test instrumentation for destruction counts; gameplay is
in `GameLogic.cinch`. `main.tscn` displays a rotating square. `test.gd` checks registration, instance
state, inspector writes, frame callbacks, engine calls, and destruction.

GDExtensions run in a normal Godot installation. Modifying Godot's engine
source is a separate integration requiring a custom engine build. This example
does not install Cinch as an editor scripting language: gameplay is compiled
from `GameLogic.cinch` into the extension's native classes.

## Build and run

Prerequisites: the Cinch compiler toolchain, a C++17 compiler, Git, SCons
(`python -m pip install --user scons`), and Godot 4.5 or later.

From this directory:

```sh
cinch cxx build bindings.json --out-dir build
godot --editor --path .
```

The first build fetches the pinned godot-cpp 4.5 SDK, generates only the required
engine classes, and builds the shared library. An existing compatible SDK can
be selected with `--godot-cpp /path/to/godot-cpp`, or `godot.godot_cpp` in the
specification. The SDK must be 4.5 or newer; its API version is recorded as the
minimum compatible Godot version. Builds currently target the host's Linux or
macOS architecture and use the standard single-precision Godot ABI.

For a headless verification:

```sh
godot --headless --editor --path . --import
godot --headless --path . --script test.gd --quit-after 120
```

The second command prints `CINCH_GODOT_OK` and exits zero. Build output includes
the public headers, generated Cinch export entries, a `.gdextension` resource,
the library, and `native-build.json` recording the SDK and input identities.
The build adds this extension to `.godot/extension_list.cfg`, preserving other
entries, so Godot loads it before its initial editor scan. This also avoids a
Godot 4.6 first-discovery/editor-documentation race observed with C++ extensions.

Rebuild with the same command after editing Cinch or the specification. The
shared library is replaced atomically. Restart the game/editor to load the new
library; live reload is disabled while native instances can retain callbacks.

## Defining classes

See [bindings.json](bindings.json) and [GameLogic.cinch](GameLogic.cinch).
Exported classes use the [ordinary C++ export specification](../cxx/exports/README.md).
Adding `godot: {}` generates registration, reflection bindings, a project
resource, and a complete build. Supported bases are `godot::Node`, `Node2D`,
`Node3D`, `Control`, `Resource`, and `RefCounted`, each qualified with `godot::`.

Numeric fields become inspector properties with getters and setters. Methods
become Godot-callable methods. `override: true` generates actual virtual
overrides, such as `_ready`, `_process`, or `_physics_process`, whose signatures
the SDK compiler checks. A Cinch method receives a leading borrowed receiver;
`_process` then receives `F64` delta and returns `Unit`. `native_methods` makes
selected inherited numeric methods callable from Cinch.

Classes default to Godot runtime classes, so gameplay does not run in the
editor. Set `tool: true` on a class to opt into editor execution. Godot owns
its instances; `free`, `queue_free`, and reference counting drive the generated
destructor hook as appropriate to the base class.

This path currently covers numeric properties/methods, native callbacks, and
per-instance state. Godot `Variant`, strings, vectors, collections, signal
connections, and an attachable `.cinch` script provider are further binding
work. Supported direct native symbol imports remain available alongside this
class adapter. The verified flow is a gameplay extension using the supported numeric API.

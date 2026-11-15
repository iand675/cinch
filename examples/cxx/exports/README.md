# Export Cinch functions and C++ classes

Build the shared library and its public headers in one step:

```sh
cinch cxx build bindings.json --out-dir build
c++ -std=c++17 host.cpp -Ibuild -Lbuild -lHostApi -Wl,-rpath,"$PWD/build" -o build/host
./build/host
cc host.c -Ibuild -Lbuild -lHostApi -Wl,-rpath,"$PWD/build" -o build/c-host
./build/c-host
```

Both hosts exit zero. The C++ host constructs two `demo::Counter` instances,
calls their Cinch implementations, verifies independent instance state, and
observes each Cinch destructor hook once. It also calls a Cinch function that
builds and consumes a heap list, exercising library runtime initialization.
The C host uses the generated allocation, method, and deletion entry points.

`HostApi.h` declares the C ABI. `HostApi.hpp` defines ordinary C++ classes and
convenience functions. `HostApi.cpp` implements the native accessors and class
factories. `HostApiExports.cinch` contains checked export entries forwarding
to the authored implementation modules. `HostApiLibrary.cinch` is the complete
source compilation assembled by the build command. The resulting library
initializes its own runtime layouts when loaded. Linux libraries bind their
own runtime internally; loading another Cinch library does not replace that
table. Shared-library replacement is atomic, including after a failed link.

`cinch cxx bindings.json --out-dir generated` generates the public API and
adapter sources without building. `cinch build Library.cinch --shared -o
libLibrary.so` builds a self-contained native library from ordinary
`foreign export ccall` declarations. Repeat `--native-object file.o` to link
native objects or archives; their content hashes are recorded in the build
manifest. These flags support native Linux and macOS. Use `.dylib` on macOS.

## Export specification

`exports` entries name a C++ convenience function, the qualified Cinch
implementation (`Module.function`), `params`, `result`, and optional `effects`.
An empty `params` list means the Cinch implementation takes `Unit`. Empty
`effects` means pure. Effects belong to the final arrow and must account for
the implementation. C ABI symbols default to `cinch_export_<module>_<name>`. Optional `symbol`
selects an exact C identifier; in that case the public header declares the
chosen symbol directly, without a convenience alias.

`export_classes` entries provide:

- `name`: the opaque Cinch type name.
- `cpp`: the qualified C++ class name.
- `fields`: numeric instance fields with `name`, `type`, and optional numeric
  `initial` (default zero). Initializers must fit the declared type.
- `methods`: the C++ `name`, qualified Cinch `function`, explicit `params`,
  `result`, and `effects`. Cinch receives a leading `Borrowed <class>` receiver.
- Optional `constructor` and `destructor` hooks, each with `function` and
  `effects`. They take `Borrowed <class>` and return `Unit`.
- Optional `base` and `export_headers` at the top level to derive from an
  existing C++ base. A method's `override: true` is checked by the C++ compiler.
- Optional `native_methods` entries (`name`, `cpp`, `params`, `result`) expose
  selected inherited scalar methods to Cinch, including real `this` adjustment.

For `Counter.count`, the Cinch API supplies `getCounterCount` and
`setCounterCount`, both with `Foreign Cxx, IO`. The C++ API supplies `get_count`
and `set_count`. Instance fields are stored in the native object and remain
valid throughout its destructor hook. Copying and moving are disabled to keep
object identity stable. The host must serialize mutation and destruction.

Ordinary exported classes also provide `newCounter` and `deleteCounter` in
Cinch, with a paired linear `Owned` handle, and corresponding C ABI factories.
C++ callers can use normal stack allocation, `new`/`delete`, or `unique_ptr`.
Godot classes instead use Godot's object allocation and lifetime protocol.

`sources` lists the authored Cinch files in the compilation, including any
project modules they import. `native_sources` optionally lists C++ source
files; `libraries` lists additional native libraries. Paths are relative to
`bindings.json`. Build output must use a separate generated directory. The
command preserves authored package manifests, records native input hashes,
and leaves implementation files intact.

Exported parameters/results support fixed-width numeric scalars and native
pointer roles. Class fields and inherited method adapters currently support
numeric scalars. Shared builds compile the full source closure and currently
reject `--package-object` consumption. Ordinary Cinch data can be allocated and used inside an
implementation; its native representation is not implicitly exposed. The C++
compiler supplies class layout, base adjustment, and vtables. Existing classes,
templates, arbitrary by-value C++ objects, and retained callbacks require
additional explicit bindings. Native errors follow Cinch's existing FFI fault
protocol; C++ exceptions cannot unwind through a Cinch export.

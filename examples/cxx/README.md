# Native C++ bindings

From this directory, with `cinch`, a C++17 compiler, and `ar` on PATH:

```sh
cinch cxx bindings.json --out-dir .
c++ -std=c++17 -fPIC -I. -c Counter.cpp -o Counter.o
ar rcs libcounter.a Counter.o
cinch run ./Main.cinch
```

The program constructs a C++ object, calls a mutating method, deletes the
object, and checks that its destructor ran. Exit status is zero. The project
manifest links the archive and GNU's C++ runtime. On macOS, change `stdc++`
to `c++` in `link.libs`. Compile the shim and its dependencies for the
same target, standard library, and ABI as the final executable. Non-inline
implementations must also be included in the archive or linked libraries.

`cinch cxx` reads an explicit JSON selection of a header's API and emits a
normal Cinch module and a C++ translation unit. It does not parse or guess
arbitrary header layouts. `module` chooses the generated module (qualified
names create subdirectories); `header` is an include path interpreted by the
C++ compiler. A function specifies its Cinch `name`, qualified C++ `cpp` name,
`params` (default `[]`), and `result`. A class specifies its opaque Cinch
`name`, qualified C++ `cpp` name, `constructor` and `destructor` binding names,
constructor `params`, and `methods`. Method entries have the same fields as
functions; `cpp` is an unqualified member name. Binding names must be unique
within the module. Unknown JSON fields are errors.

Supported values are `I8/U8`, `I16/U16`, `I32/U32`, `I64/U64`, `F32/F64`, and
`Unit` results. Empty argument lists become a Cinch `Unit` argument.
C++ resolves overloads from the declared scalar arguments. A template
instantiation can be exposed through a qualified C++ type alias in the header.
Constructors allocate with `new`; the paired destructor uses `delete` on the
same concrete type, so C++ handles layout, inheritance, and member destruction.
Methods take a call-scoped `Borrowed` receiver, never a copied C++ object.

The constructor returns `Owned <module-and-class-allocator> <opaque-type>`.
The destructor consumes that owner. Cinch's existing linear ownership checker
requires consumption and rejects double destruction. Call the generated
destructor explicitly after the final method call, including on each branch.
There is no GC finalizer or automatic scope-exit destruction in this slice.
The owner cannot be silently forgotten just because it was borrowed by a method.

All generated calls declare `Foreign Cxx`, `IO`, and `Diverge`. Wrappers are
`noexcept` and catch every C++ exception, including allocation failure, before
calling `std::terminate`. For recoverable errors, expose a nonthrowing scalar
status API in the header. Native contracts remain trusted: bindings do not
prove that the selected implementation respects its declaration.

This initial bridge excludes strings and containers across the boundary,
by-value classes, pointer-returning methods, callbacks, operators, implicit
conversions to owners, automatic header discovery, and library unloading.
Keep those behind an explicitly authored shim for now. It does not provide
scheduler-aware blocking calls or thread transfer guarantees.

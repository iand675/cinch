# Review of the first sealed Color comparison

`control-code-review.json` verifies both measured executables, driver, reference, LLVM and assembly against their successful preflight receipts. The reproducible inspector reads existing artifacts only. Across206 definitions, exactly the four intended numeric LLVM bodies differ: expansion, premultiplication, unpremultiplication and four-channel distance. Main's clocks, repetition, batch dispatch and raw/canonical hashing have identical normalized optimized instructions.

The RGBA control constructs stored RGBA16 inputs. The actual carrier jump table's entry1 returns the stored word directly through the same common return. That complete canonical observation path is identical in the two assembly files; it executes none of the changed numeric kernels. Its measured0.835 scalar/SIMD ratio at256 pixels therefore cannot establish a direct vector-arithmetic cost. It remains a real observed control regression. Code placement, cache/branch state and scheduling are possible explanations, not established causes.

The YCbCr16 workload constructs NYCbCrA values. Canonical observation uses the unchanged y-dependent16-bit YCbCr conversion followed by changed premultiplication. This is a real affected arithmetic path, unlike the RGBA control. Both variants also perform the same typed raw-field and canonical output tuple observations. The first window does not isolate the numeric body's contribution.

Nearest lookup inlines color dispatch and distance arithmetic. The SIMD worker has a24-byte stack subtraction and606 assembly lines; the scalar worker has40 bytes and566 lines. The shared continuation likewise changes register allocation. These facts show why whole-worker inspection is needed, but smaller stack reservation does not predict elapsed performance. Palette model conversion reaches the same nearest worker through `color-convert`; direct palette conversion uses another public dispatch path. The model regression cannot be generalized to the shared distance kernel when direct conversion favors SIMD in the same run.

All raw samples remain intact. For the256-pixel cases below, entries are five complete timed intervals in nanoseconds; nonpalette intervals contain20 batches, palette intervals contain one:

| Mode | SIMD intervals | Scalar intervals |
|---|---|---|
|RGBA control|274944,274303,507234,351830,452692|275254,275204,317317,411250,293742|
|YCbCr16|672473,335845,338179,980025,575486|365060,510799,332671,316506,370819|
|Palette Convert256|267814,197757,197797,298599,197658|235665,222374,262385,294263,558271|
|Palette Model256|198879,371329,199751,309105,271719|224267,224037,223417,224648,225410|

This spread is visible evidence requiring caution, not a reason to delete a sample or reverse any reported median. No pre/post quota throttling occurred in the recorded window, which does not remove other runtime variability. Longer paired controls could test repeatability; they need another exclusive grant and must retain the original results.

Source/IR expose avoidable-cost candidates shared by SIMD and scalar: nearest carries an optional `(Nat,Color)` pair and constructs both a tuple and `Some` for each newly improved best, while typed field observers and canonical observations construct output tuples. The bounded256-entry benchmark uses immediate Nat indices; a call to Nat addition is not evidence of boxed-Nat allocation. A separate counter-enabled correctness run with zero/one/many repetitions can distinguish setup/post-observation allocation from per-batch allocation and test whether numeric variants allocate equally. Allocation counters were off during elapsed timing. No dynamic allocation count has yet been claimed.

Source151b, every measured program, the SDK reference, and all earlier results remain immutable. Any later optimization must be a separate candidate with complete independent functional/ownership validation. Current source-absent and Native scopes remain unclaimed; the separately reported Root C2/d91 maintained target passed2/2.

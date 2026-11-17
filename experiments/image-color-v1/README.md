# Image color capability design

The pinned go1.27.4 `image/color` audit contains78 cumulative records. `sdk-surface.json` retains the actual audit; `operation-map.json` maps each record to planned `Image.Color` operations and records the exact installed executable/source identities. This is an isolated additive experiment in the existing Multipart lane. Historical Multipart/QP sources, benchmarks and receipts stay frozen.

`Color` will be opaque and pure, retaining11 standard source-model identities and their exact8/16-bit raw components. Each constructor has a typed optional component observer; reconstructing a value replaces fields. This avoids mutable object emulation while preserving every mapped field, including the embedded luma/chroma triple. Premultiplied constructors will preserve representable raw components even when a caller supplies RGB greater than alpha. `color-source` retains a caller-supplied pure callback for custom four-channel16-bit observations. `color-rgba` observes premultiplied16-bit RGBA. Standard constants retain their original Gray16/Alpha16 identities.

`ColorModel` will be an explicit pure `Color -> Option Color` capability. Optional outputs preserve empty-palette and custom absent conversions. A total-callback convenience wraps ordinary `Color -> Color` functions; builtin models return `Some`. Conversion to the same concrete model preserves its raw channels, including hidden nonzero RGB at zero alpha. Models are values with explicit builtin-kind observation, without hidden registration/global state or global equality policy.

`ColorPalette` will be an immutable sequence of Color values. Nearest selection uses all four premultiplied16-bit RGBA channels, shifting each squared difference by2 before summing. First ties and exact-match early selection are preserved. Empty index is0 and empty conversion is`None`; successful conversion returns the original selected value, preserving its model identity and custom capability.

The implementation must retain these distinct rounding contracts:

-8-bit channel expansion duplicates the byte; straight-alpha multiplication truncates using255 or65535 as appropriate. Generic straight-model conversion unpremultiplies before reducing precision; same-model conversion bypasses that loss.
-Grayscale uses integer19595/38470/7471 coefficients and the exact16-bit rounding adjustment before shifting24 or16 bits.
-YCbCr uses fixed JFIF coefficients and y-dependent0x10101 rounding. Direct16-bit observation must retain precision beyond conversion through an8-bit RGB intermediate. The alpha-bearing model premultiplies after16-bit conversion; conversion from zero-alpha generic input preserves the specified branch behavior.
-CMYK uses exact maximum-channel/division rules, with the black singularity explicit and16-bit observation kept separate from8-bit RGB output.

Packing and bounded channel extraction use machine words, with no per-channel arbitrary-precision arithmetic in hot kernels. Four-lane palette distance is suitable for inline LLVM vector arithmetic; independent scalar controls must verify per-lane truncation, overflow bounds and lane order. Single-pixel divisions/rounding remain scalar where vectorization adds no benefit. Real host optimized code and AArch64/wasm objects will be inspected before any SIMD claim.

An independent pinned SDK generator will capture concrete model/channel observations, every builtin conversion, custom model callbacks, palette empty/tie/alpha cases, boundary16-bit rounding, deterministic adversarial color cubes and seeded samples. A separate scalar arithmetic implementation will cross-check conversion/kernel results. Runtime gates will use explicit compiler/runtime identities, O0/O2, normal32MiB and poison256MiB heaps,8MiB runtime stacks and zero leaks. Public paired benchmarks will measure complete conversion/palette pipelines with observable output/retained-input checksums against the pinned reference; all samples/regressions and allocation differences remain visible. Root must grant exclusive timing separately.

## Initial source and independent reference checkpoint

The complete source now passes qualified current Rust3dae/d91 source/type/proof checking. Its exact source is `151b5fea2d5e3c899aa72701071b100d62a999d04cc4d07147fe836398ba9707`; the complete genuine defining interface closure remains plain under `build/image-color/source-3dae-v3`. All95 declarations/constructors have canonical documentation, with zero diagnostics/orphans. This is not a runtime or optimized-code claim.

Actual pinned SDK generation produced1,808 concrete source colors with all11 builtin model results,343 direct RGB conversion triples,96 direct CMYK conversions,15 palette cases and standard/custom-model observations. Every one of22,857 exact observations agrees with a separate scalar mathematical implementation. `vectors.json`, `sdk-generation.json` and `scalar-validation.json` retain exact input/output/tool identities; CI consumes committed fixtures without an SDK dependency.

Earlier330 layout,3dae reserved-name and LLVM no-jump diagnostics remain in complete separate failed projects. Zero-divisor handling now uses selected safe divisors and selected results, preserving the zero-alpha/black branches within checked jump-free primitives. No compiler guard was changed.

## Current standalone functional qualification

The full maintained public consumer now passes3dae/d91 O2 and a separate exact-accepted-IR O0 relink, each with normal32MiB and poisoned256MiB heaps,8MiB runtime stacks and zero leaks. It checks every captured source/model/helper/palette observation, all typed component discriminators, embedded luma/chroma, standard constants, custom optional models, retained managed channel lists, live captured-byte callbacks before/after serialized mutation and escaped-owner callbacks after factory frames return. The independent live SDK trace also confirms that a previously selected custom color retains identity and observes later owner mutation.

All three attempted foreign-IO callback erasures reject at the intended effect-row boundary. No source factory, model or palette effect contract was narrowed. Earlier incomplete test FFI effects and the palette-index-to-I64 proof diagnostic remain in exact failed projects; only the fixture was corrected.

All nine actual numeric LLVM bodies pass51,052,544 independent scalar C comparisons, including all16,777,216 byte triples,131,072 packed wide inputs and65,536 ink-cube values. Real AArch64, wasm32 and big-endianPPC64 objects have verified headers; foreign execution is not claimed. `code-inspection.json` proves that the complete public consumer reaches every identical tested body and retains actual optimized host instructions, including vector integer palette distance and premultiplication.

The O2 unit used12,744,523bytes; the O0 relink6,613,280bytes; the actual-kernel unit58,104bytes. Each preserves the350/250MiB guards and30MiB cap. Genuine source/interface producers and all failure artifacts remain plain. Root's proposed SDK-free test initializes the current host runtime through an ordinary compiler build, separately captures its locked archive, then runs the guarded functional unit. It is explicitly Rust-only and retains complete failed projects; no local Cargo gate is claimed.

`performance-plan.json` specifies matched public pipelines for standard/custom models and alpha, all direct conversions, and empty/small/256-entry/alpha/tie palettes. Functional preflights and actual code inspection must precede an explicit Root timing grant. Full Native qualification, source-absent API, measured performance, Root parent gates and production selection remain separate pending scopes. Root owns integration, shared inventory/docs and Cargo.

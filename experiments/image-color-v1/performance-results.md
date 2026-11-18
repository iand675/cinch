# Sealed public color pipeline results

All756 interleaved warmup/sample observations match complete output/model-identity/raw/canonical and retained-input/palette/model hashes. All504 Cinch processes report zero leaks. Root explicitly held all peers and its heavy work for this exclusiveCPU0 window. Actual pre/post cgroup counters record zero throttled periods and zero throttled microseconds. Allocation counters wereOFF.

The same immutable3dae/d91 source151b pair is used throughout.21 modes ×2 input sizes ×3 implementations ×(five samples +one warmup) are retained without outlier removal. Nonpalette conditions use20 complete-batch repetitions; palette conditions useone. Actual optimized code proves vector candidate kernels and scalar lane-by-lane controls under the same backend flags; input setup, original-owner traversal and final teardown stay outside the measured interval.

| Public mode | Pixels | SIMD ns/batch | Scalar / SIMD | SIMD / SDK |
|---|---:|---:|---:|---:|
|rgba-control|16|913.40|1.002|4.118|
|rgba-control|256|17591.50|0.835|3.925|
|premultiply|16|932.90|0.997|4.104|
|premultiply|256|14036.65|1.074|3.162|
|straight-model|16|2023.05|1.001|4.509|
|straight-model|256|33723.80|1.056|3.699|
|gray-model|16|1726.60|1.007|4.641|
|gray-model|256|34331.20|0.905|3.987|
|ycbcr16|16|992.00|0.987|3.596|
|ycbcr16|256|28774.30|0.634|3.387|
|rgb-ycbcr|16|1522.80|1.003|3.346|
|rgb-ycbcr|256|31185.95|1.001|3.308|
|ycbcr-rgb|16|2167.75|0.816|4.870|
|ycbcr-rgb|256|29483.90|0.971|2.709|
|rgb-cmyk|16|2068.10|0.999|4.147|
|rgb-cmyk|256|32836.95|1.004|2.897|
|cmyk-rgb|16|1779.20|1.021|3.979|
|cmyk-rgb|256|32055.25|1.116|2.950|
|custom-source|16|1513.30|0.995|6.979|
|custom-source|256|25501.40|0.957|5.558|
|custom-model|16|1544.80|1.001|3.832|
|custom-model|256|27914.00|0.977|3.177|
|alpha-model|16|1746.65|1.009|4.410|
|alpha-model|256|33872.50|0.843|4.235|
|palette-index-empty|16|1032.00|0.883|3.215|
|palette-index-empty|256|3856.00|1.018|2.048|
|palette-index-small|16|3595.00|0.936|4.433|
|palette-index-small|256|33400.00|1.033|7.187|
|palette-index-256|16|5078.00|0.974|4.333|
|palette-index-256|256|182245.00|1.135|1.080|
|palette-index-ties|16|1572.00|0.892|2.853|
|palette-index-ties|256|11427.00|1.011|5.027|
|palette-convert-empty|16|901.00|0.946|2.567|
|palette-convert-empty|256|2894.00|0.934|2.370|
|palette-convert-small|16|4897.00|1.016|4.611|
|palette-convert-small|256|49725.00|1.008|6.719|
|palette-convert-256|16|6049.00|0.974|4.507|
|palette-convert-256|256|197797.00|1.327|1.546|
|palette-convert-ties|16|3074.00|1.108|3.041|
|palette-convert-ties|256|28092.00|1.002|5.458|
|palette-model-256|16|6370.00|0.986|4.511|
|palette-model-256|256|271719.00|0.825|2.227|

At256 pixels, the large-palette Index and Convert pipelines favor the explicit vector candidate by1.135x/1.327x; premultiplication and straight-model conversion favor it by1.074x/1.056x. Other rows regress, including the RGBA control0.835x, YCbCr16 0.634x, alpha model0.843x and palette-model conversion0.825x. The raw observations are retained; shared dispatch, layout, allocation and sampling explanations remain hypotheses until actual code/control review. No slowdown is discarded as noise.

SDK gaps remain material, including custom-source6.98x at16 pixels and small-palette Index7.19x at256 pixels. Different immutable-value/interface/list/slice/GC/runtime allocation policies are part of these complete public pipelines; no parity or blanket SIMD attribution is claimed. Source/IR/ASM and allocation analysis will identify avoidable costs before any separate optimization candidate. Original source151b and every measured input remain unchanged.

The window took1.92693seconds; report growth340,539bytes, initial497,045,504bytes/final496,701,440bytes free. The350MiB initial/250MiB reserve/5MiB report guards passed, and both input receipts are unchanged. These are actual quota/resource facts, not corrections to the measured samples.

The parent maintained target separately passed2/2 in5.45seconds on Root15a6f22f C2/d91. It is not relabeled as the standalone3dae epoch. Production selection, complete Native qualification and source-absent client scope remain separate.

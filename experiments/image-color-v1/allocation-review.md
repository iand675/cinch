# Allocation census of the sealed public Color programs

The existing3dae/d91 programs ran with ordinary32MiB heap/8MiB stack, allocation and kind counters enabled, at256 pixels and zero/one/twenty batches. All189 complete SDK/candidate/scalar output and retained-owner observations match; all126 Cinch processes report zero leaks. No program or source was rebuilt. Printed clock fields are retained only as raw output, with no elapsed claim.

These are whole-process counts: input/model/palette construction, post-clock retained traversal, argument parsing and decimal formatting remain included. Subtracting count0 removes much common work but does not establish exact per-batch allocation. Formatting the printed elapsed number can itself change small final allocations; full raw kinds/size buckets remain available.

| Mode | SIMD headers20−0 | Scalar headers20−0 | SIMD header bytes20−0 | SIMD−scalar total headers at20 |
|---|---:|---:|---:|---:|
|rgba-control|15374|15374|491800|0|
|premultiply|15374|15374|491800|0|
|straight-model|30734|30734|778520|0|
|gray-model|25612|25612|573680|0|
|ycbcr16|15384|15384|492000|0|
|rgb-ycbcr|25612|25612|737520|0|
|ycbcr-rgb|30734|30736|860440|-2|
|rgb-cmyk|35852|35852|1106160|0|
|cmyk-rgb|30734|30734|901400|0|
|custom-source|20492|20492|819440|0|
|custom-model|24332|24332|727280|0|
|alpha-model|25614|25614|573720|0|
|palette-index-empty|10|10|200|0|
|palette-index-small|24614|24614|492280|0|
|palette-index-256|53736|53736|1074720|0|
|palette-index-ties|10252|10252|205040|0|
|palette-convert-empty|12|12|240|0|
|palette-convert-small|45094|45090|1065720|0|
|palette-convert-256|74214|74214|1648120|0|
|palette-convert-ties|30734|30734|778520|0|
|palette-model-256|74214|74214|1648120|0|

RGBA control, NYCbCrA, Palette Model256 and Palette Index256 have identical SIMD/scalar header counts at every observed count. The original regressions in these modes therefore have no observed difference in whole-process header allocation count. This does not attribute elapsed differences or rule out code layout/runtime state effects.

For the RGBA control, source/IR expose one optional raw component tuple and one canonical component tuple per observed color; nearest additionally constructs optional index/color tuples on improved choices. These actual public representation costs explain concrete investigation targets shared by both numeric variants. The census does not count SDK allocations, and no cross-runtime allocation parity is claimed.

Higher-count correctness preflight also passed the four confirmation modes at256 pixels,5000 batches for RGBA/NYCbCrA and100 for both palettes:20 independent reference/normal/poison observations,16 Cinch zero-leak profiles. Existing plain binary/source identities are unchanged. No longer-count elapsed samples have been collected.

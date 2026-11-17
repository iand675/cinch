# Longer-count confirmation of four public Color paths

Root explicitly granted the exclusive CPU0 window after all peers acknowledged no child. Only the same sealed3dae/d91 programs ran, with counters off and no source/build/input changes. Four256-pixel modes used5000 complete batches for RGBA16/NYCbCrA and100 for Palette Model/Index. Five rotated samples plus warmup produce72 independent observations; all48 Cinch processes report zero leaks and every output/retained-owner fingerprint matches the actual SDK reference.

| Mode | Initial scalar/SIMD | Longer scalar/SIMD | SIMD ns/batch | Scalar ns/batch | SDK ns/batch | SIMD/SDK |
|---|---:|---:|---:|---:|---:|---:|
|rgba-control|0.835|0.940|16928.65|15916.42|4054.76|4.175|
|ycbcr16|0.634|1.010|15889.16|16042.39|4508.53|3.524|
|palette-model-256|0.825|1.162|207775.49|241384.53|93933.79|2.212|
|palette-index-256|1.135|1.151|191749.04|220796.38|90330.84|2.123|

The first0.634 NYCbCrA and0.825 Palette Model ratios do not repeat under longer batching. NYCbCrA now differs by about1%, while Palette Model favors SIMD by1.162x; Palette Index again favors SIMD by1.151x. These facts support the vector distance path on the two large-palette pipelines in this confirmation. They do not erase the first window or establish blanket SIMD gains across all21 modes.

The identical-path RGBA16 control still favors scalar:0.940 versus initial0.835. Actual Main control instructions and the selected stored-RGBA16 canonical observation path are identical, and the separate allocation census records equal header counts for both variants. No direct arithmetic or allocation-count cause has been established. Placement/cache/branch state and scheduling remain hypotheses; this persistent control difference limits attribution of small improvements. No further elapsed measurement or source optimization is claimed.

All five timed intervals are retained below in nanoseconds; these are complete multi-batch intervals, not per-pixel samples:

| Mode | SIMD | Scalar | SDK |
|---|---|---|---|
|rgba-control|105012695,84643244,87568879,78298666,76569402|79615899,93525598,79582098,77888777,77762486|20032943,20273816,24598829,20776457,19552937|
|ycbcr16|79445792,79356657,78795238,80596953,82040141|87662731,80070487,79774258,80211940,81377003|22246230,22599434,22523880,23700140,22542638|
|palette-model-256|20305215,20191703,20777549,21546783,21740386|24138453,23139982,24753883,24401220,23968955|9935238,8706120,8980023,9479729,9393379|
|palette-index-256|18136483,19804648,19174904,18929613,20121507|22079638,22178378,21499882,21692964,22405100|8896557,9221569,8680991,9056800,9033084|

The complete window took3.087003s. Actual quota deltas: usage3198876us,31 periods, zero throttled periods and zero throttled microseconds. Initial free space405778432B/final405737472B; raw report35720B. The350MiB start/250MiB reserve/5MiB report guards passed. Both successful input receipts remain unchanged. No counter adjustment or outlier removal is applied.

Material SDK gaps remain, including RGBA4.18x, NYCbCrA3.52x and the two palette paths about2.1–2.2x in this confirmation. Source/IR and the separate census expose actual public tuple/Option/nearest-state allocation costs. Any attempt to reduce them must use a separate candidate, retain source151b and these sealed inputs, and repeat full independent functional/ownership validation before performance selection.

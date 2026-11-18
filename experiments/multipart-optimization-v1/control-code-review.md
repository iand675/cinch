# Sealed control code review

The raw-read and body-write regressions remain unresolved by this static review. The exact receipt in `control-code-review.json` was produced without executing or rebuilding a program. It verifies both sealed source/code sets against their existing receipts and retains all prior control samples.

Of 715 common LLVM functions, 707 are identical after removing only metadata and attribute reference numbering. The reader body, part reader, output writer, field creator/writer, delimiter/search workers, checksum, repeat loop and clock wrapper also have identical optimized assembly after removing comments and function-local label IDs. Registers, instructions, constants, symbols and CFI remain part of that comparison. This is instruction identity, not a claim of equal address layout or elapsed cost.

The shared `Main.operation` dispatcher differs: its stack subtraction grows from 40 to 56 bytes, and its static call count grows from 89 to 94. Changed disposition code is inlined into that dispatcher. This can change instruction/register/branch layout for modes that share it; the review does not demonstrate that this causes either slowdown. The separately changed POSIX filename helper is retained through the default policy, but the measured reader observes form names and headers, never filenames. Its form-name path reads the media-parameter cache directly without invoking the basename callback.

Existing independent counter controls show essentially unchanged raw-read allocations (about 310/op and 40.3 KiB/op at 4 KiB), while body-write allocations decrease from about 84 to 81/op. Peak live bytes differ by only three bytes in each control. Allocation growth therefore does not explain the recorded slowdowns. Counter-enabled elapsed fields are not timing evidence.

The five 20-iteration raw-read samples overlap substantially across binaries; body-write has an elevated candidate median with a baseline outlier retained. A focused confirmation using the same sealed binaries, 200 iterations, seven samples and a dense-disposition anchor is justified. It uses a separate report and independent SDK checksums; it will not overwrite the original 384-observation report. Root authorized at most 20 seconds of exclusive CPU0, actual pre/post cgroup statistics, and the timing-only 300 MiB start/250 MiB reserve/5 MiB report exception. No compiler or source build is included. Candidate selection still requires confirmation review and parent gates.

## Focused confirmation

The authorized confirmation completed in 1.952 seconds with all peers and Root held. All 72 observations agree on independent complete-output and retained-input/context digests; all 48 Cinch observations report zero leaks. All seven samples and warmups remain in `control-confirmation.json`. Both input receipts are unchanged. Actual pre/post cgroup statistics show zero throttled periods and zero throttled microseconds (1,910,424 microseconds of total cgroup CPU usage). Report growth was 33,491 bytes; free space was 364,187,648 bytes initially and 364,150,784 afterward.

| 4 KiB pipeline | Baseline ns/op | Candidate ns/op | Baseline / candidate |
|---|---:|---:|---:|
| Raw read | 24,004.23 | 23,904.18 | 1.004 |
| Body write | 7,308.72 | 7,340.32 | 0.996 |
| Dense disposition | 757,880.01 | 177,584.19 | 4.268 |

The earlier 8%/13% control slowdowns did not reproduce with ten times as many iterations. This resolves the immediate concern about a repeatable regression in these two controls under the measured protocol; it does not erase the first run or establish a particular layout/noise cause. The targeted dense-disposition improvement remains. Root parent gates and explicit source selection are still required.

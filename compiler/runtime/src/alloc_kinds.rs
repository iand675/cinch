//! Allocation census by block kind and size class: `CINCH_ALLOC_KINDS=1`.
//!
//! The stats line of [`crate::heap`] says how much a program allocates in
//! total. This module says what it allocates: every heap block counts once
//! under its header's kind ([`crate::block::KIND_*`], `other` for a kind
//! the header validation does not know) and once under the size class of
//! its whole allocation, header and escape word included, which is the
//! byte count the stats line's `header-bytes` adds up. Both censuses read
//! the same two counters a block already pays for, so they are one switch:
//!
//! * `CINCH_ALLOC_KINDS=1`: at process exit write one `cinch-alloc-kind:
//!   kind=<name> blocks=<n> bytes=<n>` line per kind that allocated, then
//!   one `cinch-alloc-size: upto=<n> blocks=<n> bytes=<n>` line per size
//!   class that allocated (classes 8, 16, 24, 32, 40, 48, 64, 96, 128,
//!   192, 256, `above`), to stderr, next to the stats line and from the
//!   same `atexit` handler.
//!
//! Unset, every block allocation pays one relaxed load and a branch: the
//! census never changes what a program allocates, frees or prints, and the
//! stats line keeps its existing format. The counters are only ever
//! written by [`note`], behind that branch, so a program that does not ask
//! for the census keeps its allocation rate.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};

use crate::block::{KIND_LAST, KIND_NONE};

/// One bucket per live kind, plus `other` for a kind outside the range.
const KINDS: usize = KIND_LAST as usize + 1;
const BUCKETS: usize = KINDS + 1;
const OTHER: usize = KINDS;

/// The size classes, in bytes; the last bucket holds everything above
/// [`SIZES_LAST`].
const SIZES: [usize; 11] = [8, 16, 24, 32, 40, 48, 64, 96, 128, 192, 256];
const SIZES_LAST: usize = SIZES[10];
const CLASSES: usize = SIZES.len() + 1;
const ABOVE: usize = SIZES.len();

const NAMES: [&[u8]; BUCKETS] = [
    b"none",
    b"ctor",
    b"closure",
    b"dict",
    b"memo",
    b"knot",
    b"nat",
    b"int",
    b"bytes",
    b"bytes-static",
    b"sym",
    b"code",
    b"boxed-scalar",
    b"raw",
    b"array",
    b"tensor",
    b"columnar",
    b"other",
];

const _: () = assert!(KINDS == NAMES.len() - 1);
const _: () = assert!(KIND_NONE == 0, "the census indexes names by kind number");

/// Whether `CINCH_ALLOC_KINDS=1` (set once, by [`crate::heap`] reading the
/// environment on the first allocation).
static ENABLED: AtomicBool = AtomicBool::new(false);

static KIND_BLOCKS: [AtomicU64; BUCKETS] = [const { AtomicU64::new(0) }; BUCKETS];
static KIND_BYTES: [AtomicU64; BUCKETS] = [const { AtomicU64::new(0) }; BUCKETS];
static CLASS_BLOCKS: [AtomicU64; CLASSES] = [const { AtomicU64::new(0) }; CLASSES];
static CLASS_BYTES: [AtomicU64; CLASSES] = [const { AtomicU64::new(0) }; CLASSES];

/// Turn the census on: the environment said so.
pub(crate) fn enable() {
    ENABLED.store(true, Relaxed);
}

/// The size class of a whole `bytes`-byte allocation: the first class that
/// holds it, [`ABOVE`] above the last.
fn class_of(bytes: usize) -> usize {
    if bytes > SIZES_LAST {
        return ABOVE;
    }
    SIZES.iter().position(|&limit| bytes <= limit).unwrap_or(ABOVE)
}

/// Count one fresh header block of `kind` whose whole allocation, header
/// and escape word included, is `bytes` bytes. Called by
/// [`crate::block::allocate_block`], the one allocator of header blocks.
#[inline]
pub(crate) fn note(kind: u8, bytes: usize) {
    if ENABLED.load(Relaxed) {
        record(kind, bytes);
    }
}

/// [`note`] with the census on: never inlined into the allocator, so the
/// disabled path is the load and the branch alone.
#[inline(never)]
fn record(kind: u8, bytes: usize) {
    let b = (kind as usize).min(OTHER);
    KIND_BLOCKS[b].fetch_add(1, Relaxed);
    KIND_BYTES[b].fetch_add(bytes as u64, Relaxed);
    let c = class_of(bytes);
    CLASS_BLOCKS[c].fetch_add(1, Relaxed);
    CLASS_BYTES[c].fetch_add(bytes as u64, Relaxed);
}

/// Write the census, one line per kind that allocated and then one per
/// size class that allocated. The `atexit` handler of `CINCH_ALLOC_KINDS=1`.
pub(crate) extern "C" fn print() {
    use crate::heap::Line;
    let mut kind = 0;
    while kind < BUCKETS {
        let blocks = KIND_BLOCKS[kind].load(Relaxed);
        if blocks != 0 {
            let mut line = Line::new();
            line.push(b"cinch-alloc-kind: kind=");
            line.push(NAMES[kind]);
            line.push(b" blocks=");
            line.num(blocks);
            line.push(b" bytes=");
            line.num(KIND_BYTES[kind].load(Relaxed));
            line.push(b"\n");
            line.flush();
        }
        kind += 1;
    }
    let mut class = 0;
    while class < CLASSES {
        let blocks = CLASS_BLOCKS[class].load(Relaxed);
        if blocks != 0 {
            let mut line = Line::new();
            line.push(b"cinch-alloc-size: upto=");
            if class == ABOVE {
                line.push(b"above");
            } else {
                line.num(SIZES[class] as u64);
            }
            line.push(b" blocks=");
            line.num(blocks);
            line.push(b" bytes=");
            line.num(CLASS_BYTES[class].load(Relaxed));
            line.push(b"\n");
            line.flush();
        }
        class += 1;
    }
}

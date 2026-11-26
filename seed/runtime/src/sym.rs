//! Interned symbol cells backing the language-level `Sym` type.
//!
//! A `Sym` word is a handle to a process-wide interned name cell
//! (primitive types specification section 9): one word, no C
//! counterpart. The table is keyed by the name's bytes, so two equal
//! names are the same cell and `Eq` is identity (an O(1) pointer test).
//! `Ord` is lexicographic over the name's bytes, never over cell
//! addresses, so the order is deterministic across runs and builds.
//!
//! Cells are immutable and never freed (the runtime's block discipline
//! in [`crate::abi`]), so a handle stays valid for the life of the
//! process and sharing one is free. A name must be valid UTF-8 (`intern`
//! takes a `String` at the surface), so [`SymCell::name`] is always
//! sound text and `sym-name` is total.

use crate::abi;
use crate::error::{Error, Result};
use std::sync::Mutex;

/// One interned name cell: the name bytes behind a `Sym` word.
///
/// The boxed slice lives inside an immortal `KIND_SYM` block (`RC_IMMORTAL`),
/// so neither the cell nor its byte buffer is ever moved or freed for the
/// lifetime of the process.
pub struct SymCell {
    name: Box<[u8]>,
}

impl SymCell {
    /// The cell's name (always valid UTF-8).
    #[inline]
    pub fn name(&self) -> &[u8] {
        &self.name
    }
}

/// Lightweight non-owning key pointing directly into an immortal [`SymCell`]'s
/// pinned `Box<[u8]>`, so each interned symbol allocates its byte buffer once.
#[derive(Clone, Copy)]
struct SymKey {
    ptr: *const u8,
    len: usize,
}

// Safety: every `SymKey` stored in the table points to the immutable byte
// buffer owned by an immortal `SymCell` (`RC_IMMORTAL`), never moved or freed.
unsafe impl Send for SymKey {}
unsafe impl Sync for SymKey {}

impl SymKey {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        if self.len == 0 {
            &[]
        } else {
            // Safety: valid for `self.len` immutable bytes for `'static`.
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }
}

#[derive(Clone, Copy)]
struct Slot {
    hash: u64,
    key: SymKey,
    /// Non-zero `SymCell` address, or `0` for an empty slot.
    cell: usize,
}

const EMPTY_SLOT: Slot = Slot {
    hash: 0,
    key: SymKey {
        ptr: std::ptr::null(),
        len: 0,
    },
    cell: 0,
};

/// Open-addressed linear-probing table for one shard, keyed by precomputed
/// 64-bit hash and non-owning [`SymKey`].
struct ShardTable {
    slots: Vec<Slot>,
    len: usize,
}

impl ShardTable {
    const fn new() -> Self {
        Self {
            slots: Vec::new(),
            len: 0,
        }
    }

    #[inline]
    fn lookup(&self, hash: u64, name: &[u8]) -> Option<i64> {
        let cap = self.slots.len();
        if cap == 0 {
            return None;
        }
        let mask = cap - 1;
        let mut idx = (hash as usize) & mask;
        loop {
            // Safety: `idx <= mask < cap`.
            let slot = unsafe { self.slots.get_unchecked(idx) };
            if slot.cell == 0 {
                return None;
            }
            if slot.hash == hash && slot.key.len == name.len() && slot.key.as_bytes() == name {
                return Some(slot.cell as i64);
            }
            idx = (idx + 1) & mask;
        }
    }

    #[cold]
    fn grow(&mut self) {
        let new_cap = if self.slots.is_empty() {
            16
        } else {
            self.slots.len() * 2
        };
        let mut next = vec![EMPTY_SLOT; new_cap];
        let mask = new_cap - 1;
        for &slot in &self.slots {
            if slot.cell == 0 {
                continue;
            }
            let mut idx = (slot.hash as usize) & mask;
            while next[idx].cell != 0 {
                idx = (idx + 1) & mask;
            }
            next[idx] = slot;
        }
        self.slots = next;
    }

    fn insert(&mut self, hash: u64, key: SymKey, cell: usize) {
        if self.slots.is_empty() || (self.len + 1) * 10 >= self.slots.len() * 7 {
            self.grow();
        }
        let mask = self.slots.len() - 1;
        let mut idx = (hash as usize) & mask;
        while self.slots[idx].cell != 0 {
            idx = (idx + 1) & mask;
        }
        self.slots[idx] = Slot { hash, key, cell };
        self.len += 1;
    }
}

/// Number of cache-line-padded stripes in the global intern table.
const NUM_SHARDS: usize = 64;

#[repr(align(128))]
struct Shard {
    table: Mutex<ShardTable>,
}

impl Shard {
    const fn new() -> Self {
        Self {
            table: Mutex::new(ShardTable::new()),
        }
    }
}

/// The process-wide sharded intern table: 64 cache-line-aligned stripes
/// indexed by the top 6 bits of the name's 64-bit hash.
static SHARDS: [Shard; NUM_SHARDS] = [const { Shard::new() }; NUM_SHARDS];

#[inline]
fn wymix(a: u64, b: u64) -> u64 {
    let r = (a as u128).wrapping_mul(b as u128);
    (r as u64) ^ ((r >> 64) as u64)
}

#[inline]
fn mix64(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

/// Fast 64-bit hash computed once before locking. Top 6 bits select the
/// shard and low bits index the shard's slot table.
#[inline]
fn hash_bytes(bytes: &[u8]) -> u64 {
    const P0: u64 = 0xa076_1d64_78bd_642f;
    const P1: u64 = 0xe703_7ed1_a0b4_28db;
    const P2: u64 = 0x8ebc_6af0_9c88_c6e3;
    let len = bytes.len();
    let mut seed = P0 ^ (len as u64).wrapping_mul(P1);
    let mut i = 0;
    while i + 8 <= len {
        let w = u64::from_ne_bytes(bytes[i..i + 8].try_into().unwrap());
        seed = wymix(seed ^ w, P1);
        i += 8;
    }
    let rem = len - i;
    if rem > 0 {
        let tail = if rem >= 4 {
            let lo = u32::from_ne_bytes(bytes[i..i + 4].try_into().unwrap()) as u64;
            let hi = u32::from_ne_bytes(bytes[len - 4..len].try_into().unwrap()) as u64;
            lo | (hi << 32)
        } else {
            ((bytes[i] as u64) << 16)
                | ((bytes[i + (rem >> 1)] as u64) << 8)
                | (bytes[len - 1] as u64)
        };
        seed = wymix(seed ^ tail, P2);
    }
    mix64(seed ^ (len as u64))
}

/// The cell for `name`, interning it on first sight: an equal name
/// resolves to the same cell every time. A name that is not valid UTF-8
/// is [`Error::MalformedName`] (the surface `intern` takes a `String`).
pub fn intern(name: &[u8]) -> Result<i64> {
    let hash = hash_bytes(name);
    let shard_idx = (hash >> 58) as usize;
    let mut table = SHARDS[shard_idx]
        .table
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    // Fast hit path: every entry in the table was validated on insertion, so
    // a hit skips UTF-8 validation and allocates nothing.
    if let Some(cell) = table.lookup(hash, name) {
        return Ok(cell);
    }
    // Miss path: validate UTF-8 once before boxing the immortal SymCell.
    std::str::from_utf8(name).map_err(|_| Error::MalformedName)?;
    let cell = abi::box_sym(SymCell {
        name: Box::from(name),
    });
    // Derive the non-owning key pointer from the final immortal SymCell so
    // the pointer stays valid for the lifetime of the process.
    let pinned = unsafe { abi::as_sym(cell) }.name();
    let key = SymKey {
        ptr: pinned.as_ptr(),
        len: pinned.len(),
    };
    table.insert(hash, key, cell as usize);
    Ok(cell)
}

/// Identity: the same interned cell (an O(1) pointer test).
#[inline]
pub fn eq(a: &SymCell, b: &SymCell) -> bool {
    std::ptr::eq(a, b)
}

/// The lexicographic name order as the raw -1, 0 or 1 (`natCmp`'s
/// convention): never the cell addresses, so the order is deterministic
/// across runs.
#[inline]
pub fn cmp(a: &SymCell, b: &SymCell) -> i64 {
    if std::ptr::eq(a, b) {
        return 0;
    }
    abi::compare_bytes(a.name(), b.name())
}

// ---------------------------------------------------------------------------
// Extern "C" surface
// ---------------------------------------------------------------------------

/// The cell for the `len` bytes at `ptr` (`symIntern :: Bytes -> Sym` at
/// the intrinsic layer). The table is keyed by name, so an equal name
/// resolves to the same cell as any earlier intern (and as the
/// compile-time constant of a tick literal). Generated code passes a
/// `Bytes` value's own data pointer and length, so a negative length or
/// a null pointer with a positive length is a compiler-bug trap
/// ([`Error::CorruptValue`]); a name that is not UTF-8 faults
/// ([`Error::MalformedName`]).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_sym_intern(ptr: *const u8, len: i64) -> i64 {
    let Ok(n) = usize::try_from(len) else {
        abi::raise(Error::CorruptValue);
    };
    let slice: &[u8] = if n == 0 {
        &[]
    } else if ptr.is_null() {
        abi::raise(Error::CorruptValue);
    } else {
        unsafe { std::slice::from_raw_parts(ptr, n) }
    };
    match intern(slice) {
        Ok(cell) => cell,
        Err(e) => abi::raise(e),
    }
}

/// A fresh `Bytes` word holding the cell's name (`symName :: Sym ->
/// Bytes`). The name is copied into the buffer, so the cell stays
/// immutable behind the language's `Bytes` mutation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_sym_name(word: i64) -> i64 {
    let cell = unsafe { abi::as_sym(word) };
    abi::box_bytes(crate::bytes::Bytes::from(cell.name().to_vec()))
}

/// Identity (0 or 1): `symEq`, the O(1) pointer test over interned
/// cells.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_sym_eq(a: i64, b: i64) -> i64 {
    let (ca, cb) = unsafe { (abi::as_sym(a), abi::as_sym(b)) };
    i64::from(eq(ca, cb))
}

/// The raw -1, 0 or 1 of the lexicographic name order (`symCmp`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_sym_cmp(a: i64, b: i64) -> i64 {
    let (ca, cb) = unsafe { (abi::as_sym(a), abi::as_sym(b)) };
    cmp(ca, cb)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(word: i64) -> &'static SymCell {
        // The test words are this module's own interned cells.
        unsafe { abi::as_sym(word) }
    }

    #[test]
    fn equal_names_are_one_cell_and_identity_holds() {
        let a = intern(b"foo").expect("intern");
        let b = intern(b"foo").expect("intern");
        let c = intern(b"bar").expect("intern");
        assert!(eq(cell(a), cell(b)));
        assert!(!eq(cell(a), cell(c)));
    }

    #[test]
    fn names_round_trip() {
        let a = intern(b"int-digits-into").expect("intern");
        assert_eq!(cell(a).name(), b"int-digits-into");
    }

    #[test]
    fn order_is_by_name_not_by_cell() {
        // `bar` interns second but sorts first: the order is the names'
        // order, deterministic across runs (never insertion order).
        let foo = intern(b"foo").expect("intern");
        let bar = intern(b"bar").expect("intern");
        assert_eq!(cmp(cell(bar), cell(foo)), -1);
        assert_eq!(cmp(cell(foo), cell(foo)), 0);
        assert_eq!(cmp(cell(foo), cell(bar)), 1);
    }

    #[test]
    fn non_utf8_names_are_refused() {
        assert_eq!(intern(&[0xff]), Err(Error::MalformedName));
        assert_eq!(intern(&[0xc0, 0xaf]), Err(Error::MalformedName));
        assert_eq!(intern(&[0xe2, 0x82]), Err(Error::MalformedName));
        // Repeated lookup of invalid UTF-8 still misses and returns MalformedName.
        assert_eq!(intern(&[0xff]), Err(Error::MalformedName));
    }

    #[test]
    fn empty_and_varied_lengths_and_shard_growth() {
        let empty1 = intern(b"").expect("empty intern");
        let empty2 = intern(b"").expect("empty intern repeat");
        assert_eq!(empty1, empty2);
        assert_eq!(cell(empty1).name(), b"");

        // Intern enough symbols across lengths 1..=80 to force multiple shards to grow.
        let mut words = Vec::with_capacity(2048);
        for i in 0..2048usize {
            let pad = "x".repeat(i % 64);
            let s = format!("sym_growth_{i}_{pad}");
            let w = intern(s.as_bytes()).expect("intern");
            assert_eq!(cell(w).name(), s.as_bytes());
            words.push((s, w));
        }
        // Re-intern every symbol and verify exact pointer identity and cmp order.
        for (s, w) in &words {
            assert_eq!(intern(s.as_bytes()).expect("hit"), *w);
            assert_eq!(cmp(cell(*w), cell(*w)), 0);
        }
        for pair in words.windows(2) {
            let (ref s0, w0) = pair[0];
            let (ref s1, w1) = pair[1];
            let want = match s0.as_bytes().cmp(s1.as_bytes()) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            assert_eq!(cmp(cell(w0), cell(w1)), want);
        }
    }

    #[test]
    fn concurrent_intern_resolves_to_single_canonical_cell() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let num_threads = 8usize;
        let shared_names: Arc<Vec<Vec<u8>>> = Arc::new(
            (0..256)
                .map(|i| format!("concurrent_shared_symbol_{i:04}_λ").into_bytes())
                .collect(),
        );
        let barrier = Arc::new(Barrier::new(num_threads));
        let mut handles = Vec::with_capacity(num_threads);

        for tid in 0..num_threads {
            let names = Arc::clone(&shared_names);
            let b = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                b.wait();
                let mut results = Vec::with_capacity(names.len());
                for (i, name) in names.iter().enumerate() {
                    let w = intern(name).expect("shared intern");
                    assert_eq!(cell(w).name(), name.as_slice());
                    results.push(w);
                    // Mix in thread-unique names and invalid UTF-8 probes.
                    let uniq = format!("concurrent_uniq_t{tid}_i{i}");
                    let uw = intern(uniq.as_bytes()).expect("unique intern");
                    assert_eq!(intern(uniq.as_bytes()).expect("unique hit"), uw);
                    assert_eq!(intern(&[0xfe, tid as u8, i as u8]), Err(Error::MalformedName));
                }
                results
            }));
        }

        let all: Vec<Vec<i64>> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        for i in 0..shared_names.len() {
            let expected = all[0][i];
            for t in 1..num_threads {
                assert_eq!(all[t][i], expected, "mismatch for shared symbol {i} on thread {t}");
            }
        }
    }
}

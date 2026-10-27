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
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

/// One interned name cell: the name bytes behind a `Sym` word.
pub struct SymCell {
    name: Vec<u8>,
}

impl SymCell {
    /// The cell's name (always valid UTF-8).
    pub fn name(&self) -> &[u8] {
        &self.name
    }
}

/// The process-wide intern table: name bytes to cell word. Cells are
/// boxed once, so the stored word is the name's identity.
static TABLE: LazyLock<Mutex<HashMap<Vec<u8>, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// The cell for `name`, interning it on first sight: an equal name
/// resolves to the same cell every time. A name that is not valid UTF-8
/// is [`Error::MalformedName`] (the surface `intern` takes a `String`).
pub fn intern(name: &[u8]) -> Result<i64> {
    std::str::from_utf8(name).map_err(|_| Error::MalformedName)?;
    let mut t = TABLE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(&cell) = t.get(name) {
        return Ok(cell as i64);
    }
    let cell = abi::box_sym(SymCell {
        name: name.to_vec(),
    });
    t.insert(name.to_vec(), cell as usize);
    Ok(cell)
}

/// Identity: the same interned cell (an O(1) pointer test).
pub fn eq(a: &SymCell, b: &SymCell) -> bool {
    std::ptr::eq(a, b)
}

/// The lexicographic name order as the raw -1, 0 or 1 (`natCmp`'s
/// convention): never the cell addresses, so the order is deterministic
/// across runs.
pub fn cmp(a: &SymCell, b: &SymCell) -> i64 {
    match a.name().cmp(b.name()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

// ---------------------------------------------------------------------------
// Extern "C" surface
// ---------------------------------------------------------------------------

/// The cell for the `len` bytes at `ptr` (`symIntern :: Bytes -> Sym` at
/// the intrinsic layer). The table is keyed by name, so an equal name
/// resolves to the same cell as any earlier intern (and as the
/// compile-time constant of a tick literal). Negative lengths and null
/// pointers with a positive length fault; a name that is not UTF-8
/// faults.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_sym_intern(ptr: *const u8, len: i64) -> i64 {
    if len < 0 {
        abi::raise(Error::OutOfBounds);
    }
    let n = match usize::try_from(len) {
        Ok(n) => n,
        Err(_) => abi::raise(Error::OutOfBounds),
    };
    let slice: &[u8] = if n == 0 {
        &[]
    } else if ptr.is_null() {
        abi::raise(Error::OutOfBounds);
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
pub unsafe extern "C" fn sl_sym_name(word: i64) -> i64 {
    let cell = unsafe { abi::as_sym(word) };
    abi::box_bytes(crate::bytes::Bytes::from(cell.name().to_vec()))
}

/// Identity (0 or 1): `symEq`, the O(1) pointer test over interned
/// cells.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_sym_eq(a: i64, b: i64) -> i64 {
    let (ca, cb) = unsafe { (abi::as_sym(a), abi::as_sym(b)) };
    i64::from(eq(ca, cb))
}

/// The raw -1, 0 or 1 of the lexicographic name order (`symCmp`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_sym_cmp(a: i64, b: i64) -> i64 {
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
    }
}

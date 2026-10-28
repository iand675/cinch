//! Byte buffers backing the language-level `Bytes` type.
//!
//! A buffer is a fixed-length byte region behind an `i64` word: the
//! runtime block is `Boxed<Bytes>` (`compiler/runtime/src/abi.rs`) and
//! this payload is a stable `#[repr(C)]` pair of length and data
//! pointer, so a read-only data-section buffer (a string literal's
//! constant backing) is the same block shape generated code can emit
//! directly. The region is owned (heap buffers are allocated here and
//! never freed, matching the word model) or read-only constant data
//! (the `KIND_BYTES_STATIC` block kind: mutation attempts fault instead
//! of writing). Indexing is bounds-checked; out-of-range indices,
//! negative lengths, and byte values outside `0..=255` are
//! [`Error::OutOfBounds`], never undefined behaviour. This is the
//! foundation the FFI and string slices build on.

use crate::error::{Error, Result};
use std::fmt;

/// An owned, mutable byte buffer (or a read-only constant one behind
/// the same word model).
#[repr(C)]
pub struct Bytes {
    len: i64,
    data: *mut u8,
}

impl Bytes {
    /// A zeroed buffer of `len` bytes; `len` must be non-negative.
    pub fn new(len: i64) -> Result<Bytes> {
        if len < 0 {
            return Err(Error::OutOfBounds);
        }
        let len = usize::try_from(len).map_err(|_| Error::OutOfBounds)?;
        Ok(Bytes::from(vec![0u8; len]))
    }

    /// The buffer length in bytes (always representable in an `i64`).
    pub fn len(&self) -> i64 {
        self.len
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The byte at `index` (0..=255).
    pub fn get(&self, index: i64) -> Result<i64> {
        let i = usize::try_from(index).map_err(|_| Error::OutOfBounds)?;
        if i >= self.len as usize {
            return Err(Error::OutOfBounds);
        }
        Ok(i64::from(unsafe { *self.data.add(i) }))
    }

    /// Write `value` (0..=255) at `index`.
    ///
    /// The caller (`abi::as_bytes_mut`) hands out a mutable borrow only
    /// for heap buffers; a read-only constant buffer (a string
    /// literal's backing) faults before reaching here.
    pub fn set(&mut self, index: i64, value: i64) -> Result<()> {
        let i = usize::try_from(index).map_err(|_| Error::OutOfBounds)?;
        let v = u8::try_from(value).map_err(|_| Error::OutOfBounds)?;
        if i >= self.len as usize {
            return Err(Error::OutOfBounds);
        }
        unsafe { *self.data.add(i) = v };
        Ok(())
    }

    /// Borrow the contents.
    pub fn as_slice(&self) -> &[u8] {
        if self.len == 0 {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
    }
}

impl From<Vec<u8>> for Bytes {
    /// Take ownership of the vector's region (the word model leaks it:
    /// words are never freed). The vector is shrunk to its length so
    /// `len` and the region agree exactly.
    fn from(mut buf: Vec<u8>) -> Bytes {
        buf.shrink_to_fit();
        let len = buf.len() as i64;
        let data = if buf.is_empty() {
            std::ptr::NonNull::<u8>::dangling().as_ptr()
        } else {
            Box::into_raw(buf.into_boxed_slice()) as *mut u8
        };
        Bytes { len, data }
    }
}

impl PartialEq for Bytes {
    /// Byte semantics: equal contents, whatever the backing.
    fn eq(&self, other: &Bytes) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Bytes {}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Bytes").field(&self.as_slice()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_new_zeroes_and_lengths() {
        let b = Bytes::new(4).unwrap();
        assert_eq!(b.len(), 4);
        assert!(!b.is_empty());
        assert_eq!(b.as_slice(), &[0, 0, 0, 0]);
        let empty = Bytes::new(0).unwrap();
        assert_eq!(empty.len(), 0);
        assert!(empty.is_empty());
    }

    #[test]
    fn bytes_rejects_negative_length() {
        assert_eq!(Bytes::new(-1), Err(Error::OutOfBounds));
        assert_eq!(Bytes::new(i64::MIN), Err(Error::OutOfBounds));
    }

    #[test]
    fn bytes_get_and_set_round_trip() {
        let mut b = Bytes::new(3).unwrap();
        assert_eq!(b.set(1, 255), Ok(()));
        assert_eq!(b.get(1), Ok(255));
        assert_eq!(b.get(0), Ok(0));
        assert_eq!(b.as_slice(), &[0, 255, 0]);
    }

    #[test]
    fn bytes_bounds_and_value_checks() {
        let mut b = Bytes::new(2).unwrap();
        assert_eq!(b.get(-1), Err(Error::OutOfBounds));
        assert_eq!(b.get(2), Err(Error::OutOfBounds));
        assert_eq!(b.set(2, 1), Err(Error::OutOfBounds));
        assert_eq!(b.set(0, -1), Err(Error::OutOfBounds));
        assert_eq!(b.set(0, 256), Err(Error::OutOfBounds));
    }
}

//! Byte buffers backing the language-level `Bytes` type.
//!
//! A buffer is an owned, zero-initialized, fixed-length `Vec<u8>` boxed in
//! a runtime block behind an `i64` word, like the bignums. Indexing is
//! bounds-checked; out-of-range indices, negative lengths, and byte values
//! outside `0..=255` are [`Error::OutOfBounds`], never undefined
//! behaviour. This is the foundation the FFI and string slices build on.

use crate::error::{Error, Result};

/// An owned, mutable byte buffer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bytes {
    buf: Vec<u8>,
}

impl Bytes {
    /// A zeroed buffer of `len` bytes; `len` must be non-negative.
    pub fn new(len: i64) -> Result<Bytes> {
        if len < 0 {
            return Err(Error::OutOfBounds);
        }
        let len = usize::try_from(len).map_err(|_| Error::OutOfBounds)?;
        Ok(Bytes {
            buf: vec![0u8; len],
        })
    }

    /// The buffer length in bytes (always representable in an `i64`).
    pub fn len(&self) -> i64 {
        self.buf.len() as i64
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// The byte at `index` (0..=255).
    pub fn get(&self, index: i64) -> Result<i64> {
        let i = usize::try_from(index).map_err(|_| Error::OutOfBounds)?;
        self.buf.get(i).map(|&b| i64::from(b)).ok_or(Error::OutOfBounds)
    }

    /// Write `value` (0..=255) at `index`.
    pub fn set(&mut self, index: i64, value: i64) -> Result<()> {
        let i = usize::try_from(index).map_err(|_| Error::OutOfBounds)?;
        let v = u8::try_from(value).map_err(|_| Error::OutOfBounds)?;
        let slot = self.buf.get_mut(i).ok_or(Error::OutOfBounds)?;
        *slot = v;
        Ok(())
    }

    /// Borrow the contents.
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(buf: Vec<u8>) -> Bytes {
        Bytes { buf }
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

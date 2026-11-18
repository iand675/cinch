//! `cinch_bytes_compare` is byte-lexicographic order: the in-place compare
//! reads the common prefix a word at a time, and the answer must be the one
//! `&[u8]::cmp` gives for every pair, including prefixes, one-byte
//! differences inside and across a word boundary, high bytes, and the empty
//! buffer. Order is observable (canonical interface rows, JSON object keys,
//! diagnostics), so this is the ordering test of the fast path.

use cinchruntime::abi::{
    cinch_bytes_compare, cinch_bytes_from_data, cinch_bytes_len, cinch_free,
};

/// A fresh `Bytes` word holding `bytes` (the canonical ABI copies).
fn bytes(text: &[u8]) -> i64 {
    unsafe { cinch_bytes_from_data(text.as_ptr(), text.len() as i64) }
}

fn order(a: &[u8], b: &[u8]) -> i64 {
    match a.cmp(b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

const CORPUS: [&[u8]; 15] = [
    b"",
    b"a",
    b"ab",
    b"abc",
    b"abd",
    b"abcdefg",
    b"abcdefgh",
    b"abcdefgi",
    b"abcdefghum",
    b"abcdefghijk",
    b"abcdefghijklmnop",
    b"abcdefghijklmnopq",
    b"z",
    b"\xff",
    b"\x01\x02\x03\x04\x05\x06\x07\x08\x09",
];

#[test]
fn every_pair_orders_as_the_slice_comparison() {
    for left in CORPUS {
        for right in CORPUS {
            let a = bytes(left);
            let b = bytes(right);
            let got = unsafe { cinch_bytes_compare(a, b) };
            assert_eq!(got, order(left, right), "{left:?} against {right:?}");
            assert_eq!(unsafe { cinch_bytes_compare(b, a) }, order(right, left));
            unsafe {
                cinch_free(a);
                cinch_free(b);
            }
        }
    }
}

/// Two distinct buffers with equal contents are equal, and each buffer
/// compared with itself is equal: the answer never depends on which region
/// a word names.
#[test]
fn equal_contents_are_equal_from_distinct_buffers() {
    for text in CORPUS {
        let a = bytes(text);
        let b = bytes(text);
        assert_eq!(unsafe { cinch_bytes_compare(a, b) }, 0);
        assert_eq!(unsafe { cinch_bytes_compare(a, a) }, 0);
        assert_eq!(unsafe { cinch_bytes_len(a) }, text.len() as i64);
        unsafe {
            cinch_free(a);
            cinch_free(b);
        }
    }
}

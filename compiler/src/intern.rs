//! Interned names.
//!
//! Compiler names (unit identities, type and class names, binder names,
//! field names, identities decoded from interfaces) repeat across nearly
//! every structure the frontend builds, clones, hashes and compares. As
//! `String` each repetition costs an allocation on construction, a
//! copy on clone and a byte comparison on use. Interning turns all three
//! into one word: an [`Interned`] stores the address of the canonical
//! allocation for that spelling, so equality and hashing are pointer
//! operations and cloning copies two machine words.
//!
//! Two invariants make identity equality sound.
//!
//! - Every `Interned` is produced by [`intern`], which returns the one
//!   allocation registered for that spelling. Nothing else can build one:
//!   the field is private and there is no constructor that takes a
//!   pointer.
//! - A registered allocation is never freed. The set of interned strings
//!   is write-once, append-only, so a stored address names the same bytes
//!   for the rest of the process.
//!
//! Ordering is deliberately *not* the handle order: [`Ord`] compares
//! spellings lexicographically, exactly as `String` did, so any structure
//! sorted by a name (a `BTreeSet` of unit names, a `BTreeMap` keyed by a
//! type name) keeps the order the emitted IR and interface bytes were
//! written in.

use std::collections::HashSet;
use std::fmt;
use std::ops::Deref;
use std::sync::{LazyLock, Mutex};

/// A name, canonicalized to one process-wide allocation.
///
/// Equality and hashing are by address; ordering and display are by
/// spelling. A clone is two machine words and never allocates.
#[derive(Clone, Copy)]
pub struct Interned(&'static str);

/// The canonical allocation for every interned spelling.
///
/// The lock is taken once per distinct spelling: a lookup that hits
/// registers nothing and only reads. Reads of an `Interned` never touch
/// this table, because the handle already holds the address.
static NAMES: LazyLock<Mutex<HashSet<&'static str>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

fn names() -> &'static Mutex<HashSet<&'static str>> {
    &NAMES
}

/// Intern one spelling, returning the canonical handle for it.
pub fn intern(name: &str) -> Interned {
    let mut names = names().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(canonical) = names.get(name) {
        return Interned(canonical);
    }
    // The allocation outlives every handle that names it, by the
    // write-once invariant above.
    let canonical: &'static str = Box::leak(name.to_owned().into_boxed_str());
    names.insert(canonical);
    Interned(canonical)
}

impl Interned {
    /// The spelling.
    pub fn as_str(self) -> &'static str {
        self.0
    }

    /// The number of bytes in the spelling.
    pub fn len(self) -> usize {
        self.0.len()
    }

    /// Whether the spelling is empty.
    pub fn is_empty(self) -> bool {
        self.0.is_empty()
    }

    /// Whether the spelling starts with `prefix`.
    pub fn starts_with(self, prefix: &str) -> bool {
        self.0.starts_with(prefix)
    }

    /// The spelling as raw bytes, for interface encoding.
    pub fn as_bytes(self) -> &'static [u8] {
        self.0.as_bytes()
    }
}

impl Default for Interned {
    fn default() -> Self {
        intern("")
    }
}

impl Deref for Interned {
    type Target = str;

    fn deref(&self) -> &str {
        self.0
    }
}

impl AsRef<str> for Interned {
    fn as_ref(&self) -> &str {
        self.0
    }
}

/// Identity equality: a canonical handle names exactly one spelling.
impl PartialEq for Interned {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.0.as_ptr() == other.0.as_ptr()
    }
}

impl Eq for Interned {}

impl std::hash::Hash for Interned {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.0.as_ptr() as usize);
    }
}

/// Spelling order, which is the order every name-keyed structure was
/// already sorted in, and the order the emitted bytes were written in.
impl PartialOrd for Interned {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interned {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(other.0)
    }
}

impl fmt::Display for Interned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Spelled as a quoted string, so a debug print of a type or an IR node
/// reads as it did when the field was a `String`.
impl fmt::Debug for Interned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.0, f)
    }
}

impl From<&str> for Interned {
    fn from(name: &str) -> Self {
        intern(name)
    }
}

impl From<String> for Interned {
    fn from(name: String) -> Self {
        let mut names = names().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(canonical) = names.get(name.as_str()) {
            return Interned(canonical);
        }
        // Reuse the string's own buffer rather than copying the bytes.
        let canonical: &'static str = Box::leak(name.into_boxed_str());
        names.insert(canonical);
        Interned(canonical)
    }
}

impl From<&String> for Interned {
    fn from(name: &String) -> Self {
        intern(name)
    }
}

/// A spelling as an owned `String`, for an API that takes `Into<String>`.
impl From<Interned> for String {
    fn from(name: Interned) -> String {
        name.0.to_owned()
    }
}

impl From<&Interned> for Interned {
    fn from(name: &Interned) -> Self {
        *name
    }
}

impl PartialEq<str> for Interned {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Interned {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for Interned {
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<Interned> for str {
    fn eq(&self, other: &Interned) -> bool {
        self == other.0
    }
}

impl PartialEq<Interned> for &str {
    fn eq(&self, other: &Interned) -> bool {
        *self == other.0
    }
}

impl PartialEq<Interned> for String {
    fn eq(&self, other: &Interned) -> bool {
        self.as_str() == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_spellings_share_one_handle() {
        let a = intern("Data.List.map");
        let b = intern("Data.List.map");
        let c = Interned::from(String::from("Data.List.map"));
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(a.as_ptr(), c.as_ptr());
    }

    #[test]
    fn distinct_spellings_differ_and_keep_string_order() {
        let mut names = vec![intern("b"), intern("a"), intern("aa"), intern("ab")];
        names.sort();
        let spellings: Vec<&str> = names.iter().map(|n| n.as_str()).collect();
        assert_eq!(spellings, vec!["a", "aa", "ab", "b"]);
        assert_ne!(intern("a"), intern("b"));
    }

    #[test]
    fn a_handle_reads_as_its_spelling() {
        let name = intern("Bootstrap.Check");
        assert_eq!(name.as_str(), "Bootstrap.Check");
        assert_eq!(name.to_string(), "Bootstrap.Check");
        assert_eq!(format!("{name}"), "Bootstrap.Check");
        assert_eq!(format!("{name:?}"), "\"Bootstrap.Check\"");
        assert!(name.starts_with("Boot"));
        assert_eq!(&*name, "Bootstrap.Check");
    }
}

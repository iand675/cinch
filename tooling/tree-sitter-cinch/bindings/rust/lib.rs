use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_cinch() -> *const ();
}

/// The Cinch parser generated from this package's grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_cinch) };

//! Cinch bootstrap compiler.
//!
//! Pipeline: source -> [`lexer`] -> [`layout`] -> [`parser`] -> [`check`] ->
//! [`core`] -> [`codegen`] (LLVM IR text) -> clang (native binary).
//!
//! Scope of this bootstrap slice is documented in `compiler/README.md`.

pub mod analysis;
pub mod ast;
pub mod check;
pub mod codegen;
pub mod core;
pub mod driver;
pub mod interface;
pub mod layout;
pub mod lexer;
pub mod lsp;
pub mod parser;
pub mod rc;
pub mod repl;
pub mod repr;
pub mod specialize;
pub mod store;
pub mod types;

use std::fmt;

/// A compile-time diagnostic with an optional source position.
#[derive(Clone, Debug)]
pub struct Error {
    pub span: Option<lexer::Span>,
    pub message: String,
}

impl Error {
    pub fn at(span: lexer::Span, message: impl Into<String>) -> Self {
        Error { span: Some(span), message: message.into() }
    }

    pub fn plain(message: impl Into<String>) -> Self {
        Error { span: None, message: message.into() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(span) => write!(f, "{}: {}", span, self.message),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;

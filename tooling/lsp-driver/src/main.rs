//! Run the SafeLang language server over stdio.
//!
//! The compiler binary has no `lsp` subcommand today (compiler/src/main.rs
//! offers check/emit-ir/build/run only); the server lives in the library
//! (`safelang::lsp::LspServer`) and speaks LSP over stdio with
//! `Content-Length` framed JSON-RPC, exactly as driven by
//! `compiler/tests/lsp.rs`. This driver is that invocation: when
//! `safelang lsp` lands in the binary, point the editor at it instead
//! (one setting change).

use safelang::lsp::LspServer;
use std::io::{self, BufReader};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut server = LspServer::new();
    if let Err(e) = server.run(BufReader::new(stdin.lock()), stdout.lock()) {
        eprintln!("safelang-lsp: {e}");
        std::process::exit(1);
    }
}

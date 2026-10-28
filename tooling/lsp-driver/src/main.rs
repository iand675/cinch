//! Run the Cinch language server over stdio.
//!
//! The `cinch lsp` subcommand dispatches to the same server
//! (`cinch::lsp::LspServer`) over stdio with `Content-Length` framed
//! JSON-RPC, exactly as driven by `compiler/tests/lsp.rs`, so editors
//! can exec the compiler binary directly. This driver remains for
//! editors that prefer a dedicated server binary: it is the same
//! invocation in a two-file wrapper.

use cinch::lsp::LspServer;
use std::io::{self, BufReader};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut server = LspServer::new();
    if let Err(e) = server.run(BufReader::new(stdin.lock()), stdout.lock()) {
        eprintln!("cinch-lsp: {e}");
        std::process::exit(1);
    }
}

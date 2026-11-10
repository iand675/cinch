//! The Zed extension for Cinch.
//!
//! Grammar, language configuration, and queries are declarative (see
//! `extension.toml` and `languages/cinch/`). This binary exists for the one
//! procedural part: telling Zed how to start the Cinch language server,
//! which is the compiler binary itself (`cinch lsp`, stdio JSON-RPC, see
//! `compiler/src/lsp.rs`).

use zed_extension_api as zed;

struct CinchExtension;

impl zed::Extension for CinchExtension {
    fn new() -> Self {
        CinchExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let env = worktree.shell_env();

        // A dedicated server binary wins, then the compiler's subcommand.
        // Both are resolved through the login-shell environment, so a
        // `cinch` installed by `cargo install` or linked into a PATH
        // directory is found even when Zed was launched from the GUI.
        if let Some(command) = worktree.which("cinch-lsp") {
            return Ok(zed::Command {
                command,
                args: Vec::new(),
                env,
            });
        }
        if let Some(command) = worktree.which("cinch") {
            return Ok(zed::Command {
                command,
                args: vec!["lsp".to_string()],
                env,
            });
        }

        Err(
            "no Cinch language server found: build the compiler \
             (`cargo build --release --manifest-path compiler/Cargo.toml`) and put `cinch` \
             on PATH, or set `lsp.cinch-lsp.binary.path` (and its `arguments` to [\"lsp\"]) \
             in Zed settings"
                .to_string(),
        )
    }
}

zed::register_extension!(CinchExtension);

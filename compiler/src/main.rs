use cinch::driver::{driver_opts, DriverOptions, Mode};
use std::process::exit;

const USAGE: &str = "\
cinch: bootstrap compiler for the safe language design

usage:
  cinch check <file.cinch> [--rebuild]         typecheck only
  cinch emit-ir <file.cinch> [-o <out.ll>] [--rebuild]
  cinch build <file.cinch> [-o <out>] [--rebuild]
  cinch run <file.cinch> [--rebuild]           compile and run; exit code is the program's
  cinch lsp                        run the language server over stdio

flags:
  -o <path>   the output path (emit-ir and build)
  --rebuild   recompute a package-store entry that fails verification
              instead of failing the compile with the reason
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (mode, rest): (Mode, &[String]) = match args.first().map(|s| s.as_str()) {
        Some("check") => (Mode::Check, &args[1..]),
        Some("emit-ir") => (Mode::EmitIr, &args[1..]),
        Some("build") => (Mode::Build, &args[1..]),
        Some("run") => (Mode::Run, &args[1..]),
        Some("lsp") => {
            // The language server over stdio (LSP's standard transport):
            // the same `LspServer` the REPL analysis layer and the
            // tooling driver share, driven by framed JSON-RPC on
            // stdin/stdout.
            let stdin = std::io::stdin();
            let stdout = std::io::stdout();
            let mut server = cinch::lsp::LspServer::new();
            if let Err(e) = server.run(std::io::BufReader::new(stdin.lock()), stdout.lock()) {
                eprintln!("cinch: {e}");
                exit(1);
            }
            exit(0);
        }
        _ => {
            eprint!("{USAGE}");
            exit(2);
        }
    };
    let mut file: Option<&str> = None;
    let mut out: Option<&str> = None;
    let mut opts = DriverOptions::default();
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "-o" => {
                i += 1;
                match rest.get(i) {
                    Some(o) => out = Some(o),
                    None => {
                        eprintln!("cinch: -o needs a path");
                        exit(2);
                    }
                }
            }
            "--rebuild" => opts.rebuild_store = true,
            f if file.is_none() => file = Some(f),
            other => {
                eprintln!("cinch: unexpected argument {other}");
                exit(2);
            }
        }
        i += 1;
    }
    let Some(file) = file else {
        eprint!("{USAGE}");
        exit(2);
    };
    match driver_opts(
        mode,
        std::path::Path::new(file),
        out.map(std::path::Path::new),
        &opts,
    ) {
        Ok(code) => exit(code),
        Err(e) => {
            eprintln!("cinch: {e}");
            exit(1);
        }
    }
}

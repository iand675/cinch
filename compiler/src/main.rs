use cinch::driver::{driver_opts, DriverOptions, Mode};
use std::process::exit;

const USAGE: &str = "\
cinch: bootstrap compiler for the safe language design

usage:
  cinch check <file.cinch> [--rebuild]         typecheck only
  cinch emit-ir <file.cinch> [-o <out.ll>] [--rebuild]
  cinch emit-object <library.cinch> [-o <out.o>] [--object-roots <roots.json>] [--rebuild]
  cinch build <file.cinch> [-o <out>] [--rebuild]
  cinch run <file.cinch> [args...] [--rebuild] compile and run; exit code is the program's
  cinch lsp                        run the language server over stdio
  cinch doc extract <file.cinch> [--iface <cinchif>] [--legacy] [--out <json>]
  cinch doc render <file.cinch> [--iface <cinchif>] [--legacy] [--plain] [--out <markdown>]
  cinch doc lookup <symbol> [--dir <root>] [--legacy] [--json]
  cinch doc index <root> [--legacy] [--out <json>]
  cinch doc test <root...> [--compiler <cinch>] [--legacy] [--match <text>]
  cinch doc markdown [<file.md>] [--out <html>]  highlight cinch fences on stdin or a file
  cinch doc highlight <file.cinch> [--out <html>]
  cinch repl                       interactive session in the current project
  cinch jit <file.cinch>           JIT-compile and run (requires the jit feature)
  cinch pkg add NAME@VERSION       install a registry package into a project

flags:
  -o <path>   the output path (emit-ir, emit-object and build)
  --object-roots <path>  concrete library variants demanded by a consumer
  --rebuild   recompute a package-store entry that fails verification
              instead of failing the compile with the reason
  --          everything after it is a program argument (run), so an
              argument that looks like a flag can still be passed

`run` forwards every word after the entry file to the program's own
command line, which the program reads with `process-args`
(Effect.Process).
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (mode, rest): (Mode, &[String]) = match args.first().map(|s| s.as_str()) {
        Some("check") => (Mode::Check, &args[1..]),
        Some("emit-ir") => (Mode::EmitIr, &args[1..]),
        Some("emit-object") => (Mode::EmitObject, &args[1..]),
        Some("build") => (Mode::Build, &args[1..]),
        Some("run") => (Mode::Run, &args[1..]),
        Some("doc") => exit(cinch::cinchdoc::cli(&args[1..])),
        Some("repl") => {
            if let Err(error) = cinch::repl::run() {
                eprintln!("cinch repl: {error}");
                exit(1);
            }
            exit(0);
        }
        Some("pkg") => exit(cinch::pkg::cli(&args[1..])),
        Some("jit") => {
            #[cfg(feature = "jit")]
            {
                let Some(file) = args.get(1) else {
                    eprintln!("cinch jit: needs a source file");
                    exit(2);
                };
                match cinch::driver::jit_file(std::path::Path::new(file)) {
                    Ok(code) => exit(code),
                    Err(error) => {
                        eprintln!("cinch: {error}");
                        exit(1);
                    }
                }
            }
            #[cfg(not(feature = "jit"))]
            {
                eprintln!("cinch: this build was compiled without the jit feature");
                exit(2);
            }
        }
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
            "--object-roots" => {
                i += 1;
                let Some(path) = rest.get(i) else { eprintln!("cinch: --object-roots needs a path"); exit(2) };
                let text = std::fs::read_to_string(path).unwrap_or_else(|e| { eprintln!("cinch: {path}: {e}"); exit(2) });
                opts.object_roots = cinch::specialize::parse_object_roots(&text).unwrap_or_else(|e| { eprintln!("cinch: {e}"); exit(2) });
            }
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
            // Everything after `--` is the run program's own argument
            // list, verbatim: `cinch run prog.cinch -- --rebuild x`.
            "--" => {
                opts.program_args.extend(rest[i + 1..].iter().cloned());
                break;
            }
            f if file.is_none() => file = Some(f),
            // After the entry file, a word that is not one of the flags
            // above belongs to the program `run` executes. `--` is the
            // escape for a program argument that looks like a flag.
            other => opts.program_args.push(other.to_string()),
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

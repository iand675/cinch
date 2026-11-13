use cinch::driver::{driver_opts, DriverOptions, Mode};
use std::path::PathBuf;
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
  cinch core emit <file.cinch> [--stage=<name>] [--target <triple>] [-o <out.core>]
                                   write the Core dump codegen consumes
  cinch core verify <file.core>    parse and re-check a Core dump
  cinch core diff <a.core> <b.core>
                                   report the first structural divergence
  cinch doc extract <file.cinch> [--iface <cinchif>] [--legacy] [--out <json>]
  cinch doc render <file.cinch> [--iface <cinchif>] [--legacy] [--plain] [--out <markdown>]
  cinch doc lookup <symbol> [--dir <root>] [--legacy] [--json]
  cinch doc index <root> [--legacy] [--out <json>]
  cinch doc test <root...> [--compiler <cinch>] [--legacy] [--match <text>]
  cinch doc markdown [<file.md>] [--out <html>]  highlight cinch fences on stdin or a file
  cinch doc highlight <file.cinch> [--out <html>]
  cinch repl                       interactive session in the current project
  cinch jit <file.cinch>           JIT-compile and run (requires the jit feature)
  cinch jit --package-object <package.o> [--package-dependency <dep.o>]... --call <symbol>
                                   load an installed package object and its
                                   dependency objects, then call one zero-arity
                                   unit (requires the jit feature)
  cinch pkg add NAME@VERSION | PATH_DIR | URL --git-rev REV
                                   install a registry, path, or git package
  cinch pkg add --path DIR | --git URL --git-rev REV
                                   install a local or git source explicitly
  cinch pkg install [--locked]     install the project .cinchpkg dependencies

flags:
  -o <path>   the output path (emit-ir, emit-object and build)
  --object-roots <path>  concrete library variants demanded by a consumer
  --package-object <path>  link an installed package object in build/run
  --opt-level <0|1|2|3>  clang's optimization level for build, run and
              emit-object (default 0 for build and run, 2 for emit-object)
  --max-simplifier-iterations <n>  the Core simplifier's pass cap over the
              specialized program (default 4; 0 disables the simplifier)
  --no-direct-dict-calls  keep a static dictionary projection plus its
              `__apply` call instead of a direct call to the method unit
  --no-saturate-partial  keep a partial application applied once as a
              closure plus an `__apply` instead of one saturated call
  --no-inline  leave every call in place instead of copying a small
              callee's body to its site (the rule is on by default)
  --inline-model <threshold|benefit>  which site test the inliner uses: GHC's
              weighted-size-against-a-threshold, or Flambda 2's benefit
              against size (the default)
  --emit-interface-optimisations  write the .cinchopt unfoldings beside this
              compile's .cinchif files (emit-object does so by default)
  --interface-optimisations <manifest>  read the .cinchopt unfoldings beside
              each manifest row's .cinchif and inline those units at this
              program's call sites; the units are not emitted here, so link
              the object the artifacts describe
  --no-stack-alloc  keep every allocation on the heap instead of building
              a non-escaping block in its activation's frame (the pass is
              on by default; with it off the IR is the pre-pass IR)
  --stack-frame-limit <bytes>  the most frame bytes one unit may reserve
              (default 16384); the rest of its allocations stay on the heap
  --escape-report  print the escape census after frame allocation: one row
              per class (frame-allocated, and every reason an allocation
              stayed on the heap) with its count, percentage and the payload
              bytes those allocations carry (CINCH_ESCAPE_REPORT=1 does the
              same; design/stack-allocation.md section 8)
  --dump-core <stage>  also write build/<entry>.core at that pipeline point:
              checked (solved and elaborated are aliases), owned, or lowered
              (the default stage, and the exact code generation input); a
              non-default stage writes build/<entry>.<stage>.core, and
              `check` dumps the point it reaches, marked in @stage
  --rebuild   recompute a package-store entry that fails verification
              instead of failing the compile with the reason
  --target <triple>  the target to compile for: aarch64-apple-darwin,
              x86_64-apple-darwin, x86_64-unknown-linux-gnu,
              aarch64-unknown-linux-gnu, or wasm32-unknown-unknown (check
              and emit-ir work for all of them here; build needs the
              target's runtime archive, which for wasm32 is
              cargo build --target wasm32-unknown-unknown in compiler/runtime)
  --runtime-archive <path>  the cinchruntime archive built for a foreign
              --target, which a cross build links
  --target-version <version>  the platform version package conditions see
  --print-link-plan  report every contribution to the final link
  --allow-raw-link NAME  consent to one package's raw link words
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
        Some("core") => exit(cinch::driver::core_cli(&args[1..])),
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
                let mut file: Option<&str> = None;
                let mut object: Option<&str> = None;
                let mut dependencies: Vec<PathBuf> = Vec::new();
                let mut symbol: Option<&str> = None;
                let mut i = 1;
                while i < args.len() {
                    match args[i].as_str() {
                        "--package-object" => {
                            i += 1;
                            match args.get(i) {
                                Some(path) => object = Some(path),
                                None => {
                                    eprintln!("cinch jit: --package-object needs a path");
                                    exit(2);
                                }
                            }
                        }
                        "--package-dependency" => {
                            i += 1;
                            match args.get(i) {
                                Some(path) => dependencies.push(PathBuf::from(path)),
                                None => {
                                    eprintln!("cinch jit: --package-dependency needs a path");
                                    exit(2);
                                }
                            }
                        }
                        "--call" => {
                            i += 1;
                            match args.get(i) {
                                Some(name) => symbol = Some(name),
                                None => {
                                    eprintln!("cinch jit: --call needs a unit symbol");
                                    exit(2);
                                }
                            }
                        }
                        other if other.starts_with("--") => {
                            eprintln!("cinch jit: unknown flag {other}");
                            exit(2);
                        }
                        other => {
                            if file.is_some() {
                                eprintln!("cinch jit: takes one source file");
                                exit(2);
                            }
                            file = Some(other);
                        }
                    }
                    i += 1;
                }
                match (file, object, symbol) {
                    (Some(file), None, None) => {
                        match cinch::driver::jit_file(std::path::Path::new(file)) {
                            Ok(code) => exit(code),
                            Err(error) => {
                                eprintln!("cinch: {error}");
                                exit(1);
                            }
                        }
                    }
                    (None, Some(object), Some(symbol)) => {
                        match cinch::driver::jit_package_symbol(
                            std::path::Path::new(object),
                            &dependencies,
                            symbol,
                        ) {
                            Ok(value) => {
                                println!("{value}");
                                exit(0);
                            }
                            Err(error) => {
                                eprintln!("cinch: {error}");
                                exit(1);
                            }
                        }
                    }
                    (Some(_), Some(_), _) => {
                        eprintln!(
                            "cinch jit: a source program and an installed package object cannot share one engine while the program still compiles its dependency sources"
                        );
                        exit(2);
                    }
                    (None, Some(_), None) => {
                        eprintln!("cinch jit: --package-object needs --call SYMBOL");
                        exit(2);
                    }
                    _ => {
                        eprintln!("cinch jit: needs a source file, or --package-object and --call");
                        exit(2);
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
                let Some(path) = rest.get(i) else {
                    eprintln!("cinch: --object-roots needs a path");
                    exit(2)
                };
                let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
                    eprintln!("cinch: {path}: {e}");
                    exit(2)
                });
                opts.object_roots =
                    cinch::specialize::parse_object_roots(&text).unwrap_or_else(|e| {
                        eprintln!("cinch: {e}");
                        exit(2)
                    });
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
            "--dump-core" => {
                i += 1;
                match rest.get(i).map(String::as_str) {
                    Some(name) => match cinch::coretext::Stage::parse(name) {
                        Some(stage) => opts.dump_core = Some(stage),
                        None => {
                            eprintln!(
                                "cinch: --dump-core takes one of {}",
                                cinch::coretext::Stage::spellings().join(", ")
                            );
                            exit(2);
                        }
                    },
                    None => {
                        eprintln!("cinch: --dump-core needs a stage");
                        exit(2);
                    }
                }
            }
            other if other.starts_with("--dump-core=") => {
                let name = &other["--dump-core=".len()..];
                match cinch::coretext::Stage::parse(name) {
                    Some(stage) => opts.dump_core = Some(stage),
                    None => {
                        eprintln!(
                            "cinch: --dump-core takes one of {}",
                            cinch::coretext::Stage::spellings().join(", ")
                        );
                        exit(2);
                    }
                }
            }
            "--max-simplifier-iterations" => {
                i += 1;
                match rest.get(i).and_then(|n| n.parse::<u32>().ok()) {
                    Some(n) => opts.max_simplifier_iterations = Some(n),
                    None => {
                        eprintln!("cinch: --max-simplifier-iterations needs a non-negative count");
                        exit(2);
                    }
                }
            }
            "--no-direct-dict-calls" => opts.no_direct_dict_calls = true,
            "--no-saturate-partial" => opts.no_saturate_partial = true,
            "--no-inline" => opts.no_inline = true,
            "--inline-model" => {
                i += 1;
                match rest.get(i).map(String::as_str) {
                    Some(v @ ("threshold" | "benefit")) => {
                        opts.inline_model = Some(v.to_string());
                    }
                    _ => {
                        eprintln!("cinch: --inline-model needs `threshold` or `benefit`");
                        exit(2);
                    }
                }
            }
            "--interface-optimisations" => {
                i += 1;
                match rest.get(i) {
                    Some(path) => opts.interface_optimisations = Some(PathBuf::from(path)),
                    None => {
                        eprintln!("cinch: --interface-optimisations needs a manifest path");
                        exit(2);
                    }
                }
            }
            "--emit-interface-optimisations" => opts.emit_interface_optimisations = true,
            "--no-stack-alloc" => opts.no_stack_alloc = true,
            "--stack-frame-limit" => {
                i += 1;
                match rest.get(i).and_then(|n| n.parse::<u64>().ok()) {
                    Some(n) => opts.stack_frame_limit = Some(n),
                    None => {
                        eprintln!("cinch: --stack-frame-limit needs a byte count");
                        exit(2);
                    }
                }
            }
            "--escape-report" => opts.escape_report = true,
            "--print-link-plan" => opts.print_link_plan = true,
            "--allow-raw-link" => {
                i += 1;
                match rest.get(i) {
                    Some(name) => opts.allow_raw_link.push(name.to_string()),
                    None => {
                        eprintln!("cinch: --allow-raw-link needs a package name");
                        exit(2);
                    }
                }
            }
            "--target" => {
                i += 1;
                match rest.get(i) {
                    Some(triple) => opts.target = Some(triple.to_string()),
                    None => {
                        eprintln!("cinch: --target needs a triple such as aarch64-apple-darwin");
                        exit(2);
                    }
                }
            }
            "--runtime-archive" => {
                i += 1;
                match rest.get(i) {
                    Some(path) => opts.runtime_archive = Some(PathBuf::from(path)),
                    None => {
                        eprintln!("cinch: --runtime-archive needs a path to libcinchruntime.a");
                        exit(2);
                    }
                }
            }
            "--opt-level" => {
                if !matches!(mode, Mode::Build | Mode::Run | Mode::EmitObject) {
                    eprintln!("cinch: --opt-level applies to build, run and emit-object");
                    exit(2);
                }
                i += 1;
                match rest.get(i).and_then(|level| level.parse::<u8>().ok()).filter(|level| *level <= 3) {
                    Some(level) => opts.opt_level = Some(level),
                    None => {
                        eprintln!("cinch: --opt-level needs 0, 1, 2 or 3");
                        exit(2);
                    }
                }
            }
            "--target-version" => {
                i += 1;
                match rest.get(i) {
                    Some(version) => opts.target_version = Some(version.to_string()),
                    None => {
                        eprintln!("cinch: --target-version needs a version such as 14.0");
                        exit(2);
                    }
                }
            }
            "--package-object" if matches!(mode, Mode::Build | Mode::Run) => {
                i += 1;
                match rest.get(i) {
                    Some(path) => opts.package_objects.push(PathBuf::from(path)),
                    None => {
                        eprintln!("cinch: --package-object needs a path");
                        exit(2);
                    }
                }
            }
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

use safelang::driver::{driver, Mode};
use std::process::exit;

const USAGE: &str = "\
safelang: bootstrap compiler for the safe language design

usage:
  safelang check <file.lang>          typecheck only
  safelang emit-ir <file.lang> [-o <out.ll>]
  safelang build <file.lang> [-o <out>]
  safelang run <file.lang>            compile and run; exit code is the program's
";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (mode, rest): (Mode, &[String]) = match args.first().map(|s| s.as_str()) {
        Some("check") => (Mode::Check, &args[1..]),
        Some("emit-ir") => (Mode::EmitIr, &args[1..]),
        Some("build") => (Mode::Build, &args[1..]),
        Some("run") => (Mode::Run, &args[1..]),
        _ => {
            eprint!("{USAGE}");
            exit(2);
        }
    };
    let mut file: Option<&str> = None;
    let mut out: Option<&str> = None;
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "-o" => {
                i += 1;
                match rest.get(i) {
                    Some(o) => out = Some(o),
                    None => {
                        eprintln!("safelang: -o needs a path");
                        exit(2);
                    }
                }
            }
            f if file.is_none() => file = Some(f),
            other => {
                eprintln!("safelang: unexpected argument {other}");
                exit(2);
            }
        }
        i += 1;
    }
    let Some(file) = file else {
        eprint!("{USAGE}");
        exit(2);
    };
    match driver(mode, std::path::Path::new(file), out.map(std::path::Path::new)) {
        Ok(code) => exit(code),
        Err(e) => {
            eprintln!("safelang: {e}");
            exit(1);
        }
    }
}

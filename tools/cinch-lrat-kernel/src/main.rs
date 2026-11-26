//! Standalone CLI entrypoint for `cinch-lrat-kernel`:
//! `cinch-lrat-kernel <formula.cnf> <proof.lrat>`

#![forbid(unsafe_code)]

use cinch_lrat_kernel::{parse_dimacs, verify_lrat, KERNEL_ID};
use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: cinch-lrat-kernel <formula.cnf> <proof.lrat>");
        return ExitCode::from(2);
    }
    let cnf = match fs::read_to_string(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{KERNEL_ID}: cannot read `{}`: {e}", args[1]);
            return ExitCode::from(2);
        }
    };
    let proof = match fs::read_to_string(&args[2]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{KERNEL_ID}: cannot read `{}`: {e}", args[2]);
            return ExitCode::from(2);
        }
    };
    let (nv, clauses) = match parse_dimacs(&cnf) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{KERNEL_ID}: {e}");
            return ExitCode::from(1);
        }
    };
    match verify_lrat(nv, &clauses, &proof) {
        Ok(steps) => {
            println!("VERIFIED ({KERNEL_ID}): {steps} LRAT RUP step(s) over {nv} vars / {} clauses", clauses.len());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("REJECTED ({KERNEL_ID}): {e}");
            ExitCode::from(1)
        }
    }
}

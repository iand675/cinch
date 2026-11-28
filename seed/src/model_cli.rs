//! CLI subcommand `cinch model` for inspecting, tracing, and visualizing
//! explicit-state temporal machines (`machine`, `contract`, `refines`).
//!
//! Commands:
//!   cinch model inspect <file.cinch> [MachineName] [--json]
//!   cinch model trace <file.cinch> <MachineName> [--state <N> | --deadlock | --where <pred>]
//!   cinch model dot <file.cinch> <MachineName> [-o <out.dot>]

use crate::check::{self, MachineReport, StageMode, StagingCx};
use crate::core::TargetProfile;
use crate::driver;
use crate::lexer;
use crate::parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

const MODEL_USAGE: &str = "\
usage:
  cinch model inspect <file.cinch> [MachineName] [--json] [--model-fail-on-capped]
      explore machines and print state/edge counts, action domains, symmetry/POR stats, and contract verdicts
  cinch model trace <file.cinch> <MachineName> [--state <N> | --deadlock | --where <pred>]
      print the shortest BFS trace from init to state N, a deadlock, or a state matching <pred>
  cinch model dot <file.cinch> <MachineName> [-o <out.dot>]
      export the explored state transition graph in Graphviz DOT format
";

struct LoadedSources {
    prelude: driver::PreludeSources,
    modules: Vec<crate::ast::Module>,
    scx: StagingCx,
    fixities: parser::FixityTable,
}

fn load_entry_sources(entry: &Path) -> crate::Result<LoadedSources> {
    let prelude = driver::parse_prelude(&[])?;
    let root = entry.parent().unwrap_or_else(|| Path::new("."));
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut ordered_files: Vec<(PathBuf, String)> = Vec::new();
    let mut queue: Vec<PathBuf> = vec![entry.to_path_buf()];

    while let Some(path) = queue.pop() {
        let canon = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        if !visited.insert(canon) {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| {
            crate::Error::plain(format!("{}: {e}", path.display()))
        })?;
        // Discover local sibling module files referenced by `import Mod.Name`.
        for line in text.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("import ") {
                let mod_name = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim();
                if mod_name.is_empty() || mod_name == "data" {
                    continue;
                }
                let rel_slash = mod_name.replace('.', "/") + ".cinch";
                let rel_dot = format!("{mod_name}.cinch");
                for candidate in [root.join(&rel_slash), root.join(&rel_dot)] {
                    if candidate.is_file() {
                        queue.push(candidate);
                    }
                }
            }
        }
        ordered_files.push((path, text));
    }
    ordered_files.reverse();

    let mut modules = Vec::new();
    let mut files = HashMap::new();
    let mut fixity_map = prelude.fixities.clone();
    let mut entry_modules = Vec::new();
    for (idx, (path, text)) in ordered_files.iter().enumerate() {
        let label = path.display().to_string();
        let parsed = driver::parse_source(text, &label, &prelude.base, &HashMap::new())?;
        for (m, f) in parsed.fixities {
            fixity_map.insert(m, f);
        }
        for m in parsed.modules {
            files.insert(m.name.text.clone(), label.clone());
            if idx + 1 == ordered_files.len() {
                entry_modules.push(m.name.text.clone());
            }
            modules.push(m);
        }
    }
    let fixities = prelude.base.clone();
    let mut scx = StagingCx::default();
    scx.mode = StageMode::Refresh;
    scx.fixities = fixity_map;
    scx.files = files;
    scx.entry_modules = entry_modules;
    Ok(LoadedSources {
        prelude,
        modules,
        scx,
        fixities,
    })
}

fn find_report<'a>(reports: &'a [MachineReport], name: &str) -> Option<&'a MachineReport> {
    reports
        .iter()
        .find(|r| r.name == name || format!("{}.{}", r.module, r.name) == name)
}

pub fn cli(args: &[String]) -> i32 {
    match args.first().map(|s| s.as_str()) {
        None | Some("-h") | Some("--help") | Some("help") => {
            print!("{MODEL_USAGE}");
            0
        }
        Some("inspect") => cli_inspect(&args[1..]),
        Some("trace") => cli_trace(&args[1..]),
        Some("dot") => cli_dot(&args[1..]),
        Some(other) => {
            eprintln!("cinch model: unknown subcommand `{other}`\n{MODEL_USAGE}");
            2
        }
    }
}

fn cli_inspect(args: &[String]) -> i32 {
    let mut file: Option<&str> = None;
    let mut filter: Option<&str> = None;
    let mut as_json = false;
    let mut fail_on_capped_flag = false;
    for arg in args {
        match arg.as_str() {
            "--json" => as_json = true,
            "--model-fail-on-capped" => fail_on_capped_flag = true,
            "-h" | "--help" => {
                print!("{MODEL_USAGE}");
                return 0;
            }
            other if other.starts_with("--") => {
                eprintln!("cinch model inspect: unknown flag `{other}`");
                return 2;
            }
            other => {
                if file.is_none() {
                    file = Some(other);
                } else if filter.is_none() {
                    filter = Some(other);
                } else {
                    eprintln!("cinch model inspect: unexpected argument `{other}`");
                    return 2;
                }
            }
        }
    }
    let Some(file) = file else {
        eprintln!("cinch model inspect: expected <file.cinch>");
        return 2;
    };
    let loaded = match load_entry_sources(Path::new(file)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("cinch model: {e}");
            return 1;
        }
    };
    let reports = match check::inspect_machines_multi(
        &loaded.prelude.modules,
        &loaded.modules,
        &loaded.scx,
        &TargetProfile::native(),
    ) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("cinch model: {e}");
            return 1;
        }
    };
    let selected: Vec<&MachineReport> = match filter {
        Some(name) => match find_report(&reports, name) {
            Some(r) => vec![r],
            None => {
                eprintln!("cinch model inspect: no machine named `{name}`");
                return 1;
            }
        },
        None => reports.iter().collect(),
    };
    if selected.is_empty() {
        if as_json {
            println!("[]");
        } else {
            println!("no `machine` declarations found in {file}");
        }
        return 0;
    }
    if as_json {
        let items: Vec<String> = selected.iter().map(|r| r.to_json()).collect();
        println!("[{}]", items.join(","));
    } else {
        for (i, r) in selected.iter().enumerate() {
            if i > 0 {
                println!();
            }
            println!("{}", r.format_summary());
        }
    }
    let fail_on_capped = fail_on_capped_flag || driver::env_model_fail_on_capped();
    let capped_machines: Vec<&MachineReport> = selected.iter().copied().filter(|r| r.capped).collect();
    if fail_on_capped && !capped_machines.is_empty() {
        for m in &capped_machines {
            eprintln!(
                "cinch model inspect: machine `{}.{}` hit state exploration cap (`max_states = {}`, explored {} states)",
                m.module,
                m.name,
                m.max_states,
                m.states.len()
            );
        }
    }
    let any_failed = selected
        .iter()
        .any(|r| r.contracts.iter().any(|c| !c.passed) || r.refinements.iter().any(|rf| !rf.passed));
    if any_failed || (fail_on_capped && !capped_machines.is_empty()) { 1 } else { 0 }
}

fn cli_trace(args: &[String]) -> i32 {
    let mut file: Option<&str> = None;
    let mut machine_name: Option<&str> = None;
    let mut state_id: Option<usize> = None;
    let mut want_deadlock = false;
    let mut pred_src: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--state" => {
                i += 1;
                match args.get(i).and_then(|s| s.parse::<usize>().ok()) {
                    Some(n) => state_id = Some(n),
                    None => {
                        eprintln!("cinch model trace: --state expects a non-negative integer");
                        return 2;
                    }
                }
            }
            "--deadlock" => want_deadlock = true,
            "--where" => {
                i += 1;
                match args.get(i) {
                    Some(p) => pred_src = Some(p.clone()),
                    None => {
                        eprintln!("cinch model trace: --where expects a predicate expression");
                        return 2;
                    }
                }
            }
            "-h" | "--help" => {
                print!("{MODEL_USAGE}");
                return 0;
            }
            other if other.starts_with("--") => {
                eprintln!("cinch model trace: unknown flag `{other}`");
                return 2;
            }
            other => {
                if file.is_none() {
                    file = Some(other);
                } else if machine_name.is_none() {
                    machine_name = Some(other);
                } else {
                    eprintln!("cinch model trace: unexpected argument `{other}`");
                    return 2;
                }
            }
        }
        i += 1;
    }
    let (Some(file), Some(machine_name)) = (file, machine_name) else {
        eprintln!("cinch model trace: expected <file.cinch> <MachineName>");
        return 2;
    };
    let loaded = match load_entry_sources(Path::new(file)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("cinch model: {e}");
            return 1;
        }
    };
    if let Some(pred_text) = pred_src {
        let wrapped = format!("module TraceProbe where\n  tracePred = {pred_text}\n");
        let parsed = match lexer::lex(&wrapped)
            .and_then(crate::layout::layout)
            .and_then(|toks| parser::parse_modules(&toks, &loaded.fixities, &HashMap::new()))
        {
            Ok(p) => p,
            Err(e) => {
                eprintln!("cinch model trace: invalid --where predicate: {e}");
                return 1;
            }
        };
        let pred_term = parsed
            .modules
            .first()
            .and_then(|m| m.decls.first())
            .and_then(|d| match d {
                crate::ast::Decl::Equation { eq, .. } => Some(eq.body.clone()),
                _ => None,
            });
        let Some(pred_term) = pred_term else {
            eprintln!("cinch model trace: could not parse --where predicate");
            return 1;
        };
        match check::trace_machine_where(
            &loaded.prelude.modules,
            &loaded.modules,
            &loaded.scx,
            &TargetProfile::native(),
            machine_name,
            &pred_term,
        ) {
            Ok(Some((idx, trace))) => {
                println!("Shortest trace in `{machine_name}` to matching state #{idx}:");
                println!("{trace}");
                return 0;
            }
            Ok(None) => {
                println!("No reachable state in `{machine_name}` satisfies `{pred_text}`.");
                return 1;
            }
            Err(e) => {
                eprintln!("cinch model trace: {e}");
                return 1;
            }
        }
    }

    let reports = match check::inspect_machines_multi(
        &loaded.prelude.modules,
        &loaded.modules,
        &loaded.scx,
        &TargetProfile::native(),
    ) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("cinch model: {e}");
            return 1;
        }
    };
    let Some(report) = find_report(&reports, machine_name) else {
        eprintln!("cinch model trace: no machine named `{machine_name}`");
        return 1;
    };
    let target = if want_deadlock {
        match report.deadlocks.first().copied() {
            Some(d) => d,
            None => {
                println!("Machine `{machine_name}` has no reachable deadlocks.");
                return 0;
            }
        }
    } else {
        state_id.unwrap_or(0)
    };
    match report.format_trace_to(target) {
        Some(trace) => {
            println!("Shortest trace in `{}` to state #{target}:", report.name);
            println!("{trace}");
            0
        }
        None => {
            eprintln!(
                "cinch model trace: state #{target} out of range (machine `{}` has {} state(s))",
                report.name,
                report.states.len()
            );
            1
        }
    }
}

fn cli_dot(args: &[String]) -> i32 {
    let mut file: Option<&str> = None;
    let mut machine_name: Option<&str> = None;
    let mut out_path: Option<&str> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                i += 1;
                match args.get(i) {
                    Some(p) => out_path = Some(p.as_str()),
                    None => {
                        eprintln!("cinch model dot: -o expects a file path");
                        return 2;
                    }
                }
            }
            "-h" | "--help" => {
                print!("{MODEL_USAGE}");
                return 0;
            }
            other if other.starts_with("--") => {
                eprintln!("cinch model dot: unknown flag `{other}`");
                return 2;
            }
            other => {
                if file.is_none() {
                    file = Some(other);
                } else if machine_name.is_none() {
                    machine_name = Some(other);
                } else {
                    eprintln!("cinch model dot: unexpected argument `{other}`");
                    return 2;
                }
            }
        }
        i += 1;
    }
    let (Some(file), Some(machine_name)) = (file, machine_name) else {
        eprintln!("cinch model dot: expected <file.cinch> <MachineName>");
        return 2;
    };
    let loaded = match load_entry_sources(Path::new(file)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("cinch model: {e}");
            return 1;
        }
    };
    let reports = match check::inspect_machines_multi(
        &loaded.prelude.modules,
        &loaded.modules,
        &loaded.scx,
        &TargetProfile::native(),
    ) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("cinch model: {e}");
            return 1;
        }
    };
    let Some(report) = find_report(&reports, machine_name) else {
        eprintln!("cinch model dot: no machine named `{machine_name}`");
        return 1;
    };
    let dot = report.to_dot();
    if let Some(p) = out_path {
        if let Err(e) = fs::write(p, format!("{dot}\n")) {
            eprintln!("cinch model dot: failed to write {p}: {e}");
            return 1;
        }
    } else {
        println!("{dot}");
    }
    0
}

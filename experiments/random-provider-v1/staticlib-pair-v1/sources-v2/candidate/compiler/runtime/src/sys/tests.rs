//! The audit test for the syscall layer.
//!
//! # Why it exists
//!
//! The runtime's promise (design/base-as-a-package.md section 5) is that
//! every entry point into the OS lives in `sys/`, so the whole foreign
//! surface can be audited in one directory and a later rung
//! (`#![no_std]`, our own allocator and `_start`) can replace it in one
//! place. A declaration that drifts back into `platform.rs`, `stack.rs`
//! or `pty.rs` breaks that promise silently: the crate still links,
//! because the C library is on the link line either way, and the only
//! symptom would be a reader finding a kernel entry point somewhere it
//! was told there is none. So this test greps the sources.
//!
//! It is a grep, not a parser. It recognises an `extern "C" {` block at
//! the start of a line (after an optional `unsafe `) and the `fn <name>`
//! declarations inside it, which is the shape every declaration in this
//! crate has. A `#[unsafe(no_mangle)] pub extern "C" fn` DEFINITION,
//! which is what every `cinch_*` export is, is not a declaration and is
//! deliberately invisible to it: `the_scanner_sees_a_declaration` pins
//! that distinction, so the grep cannot pass by failing to see anything.

use std::path::{Path, PathBuf};

/// Foreign symbols that may be declared outside `sys/` because they are
/// genuinely not kernel entry points. Empty today: the two calls with no
/// syscall behind them (`getenv` and `atexit`) moved into `sys/` as
/// well, because the property worth keeping is "one auditable place",
/// not "one auditable place for the interesting calls". An entry here
/// needs a reason a reviewer can check, so add it with one.
const ALLOWED: &[&str] = &[];

#[test]
fn only_the_sys_layer_declares_foreign_functions() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    walk(&root, &mut files);
    assert!(
        files.len() > 10,
        "the walk found {} files under {}, so the walk is broken",
        files.len(),
        root.display()
    );

    let sys = root.join("sys");
    let mut offenders = Vec::new();
    let mut in_sys = 0;
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let declarations = declarations(&text);
        if path.starts_with(&sys) {
            in_sys += declarations.len();
            continue;
        }
        for (line, name) in declarations {
            if !ALLOWED.contains(&name.as_str()) {
                offenders.push(format!("{}:{line}: {name}", path.display()));
            }
        }
    }

    assert!(
        in_sys > 0,
        "no declaration was found inside {} either, so the grep is blind",
        sys.display()
    );
    assert!(
        offenders.is_empty(),
        "these foreign functions are declared outside src/sys/, which is \
         the only directory allowed to name the operating system; move the \
         declaration into src/sys/ and call it through crate::sys:\n  {}",
        offenders.join("\n  ")
    );
}

/// The scanner itself, pinned on a fixture: a declaration is seen, a
/// definition is not, and the line number is the declaration's own.
#[test]
fn the_scanner_sees_a_declaration() {
    let block = "unsafe extern \"C\" {\n    // a comment\n    fn kill(pid: i32) -> i32;\n    pub(super) fn write(fd: i32) -> i32;\n}\n";
    assert_eq!(
        declarations(block),
        vec![(3, "kill".to_string()), (4, "write".to_string())]
    );

    let definition = "pub extern \"C\" fn cinch_ok() -> i64 {\n    0\n}\n";
    assert!(declarations(definition).is_empty());
}

/// `getpid` against an independent answer (`std`'s). It is the one
/// binding with no caller in the runtime yet, and the only call here
/// that can be checked without touching anything.
#[test]
fn getpid_agrees_with_std() {
    assert_eq!(super::getpid(), i64::from(std::process::id()));
}

/// The stack range [`crate::stack`] guards has to be the real one: the
/// low end plus the size must contain a live local of this frame, or the
/// guard band is protecting somebody else's memory.
#[test]
fn the_thread_stack_range_contains_this_frame() {
    let local = std::hint::black_box(0u8);
    let here = &local as *const u8 as usize;
    let (low, size) = super::this_thread_stack().expect("the platform describes this thread's stack");
    assert!(size > 0, "stack size {size}");
    let high = low.checked_add(size).expect("stack range does not wrap");
    assert!(
        (low..high).contains(&here),
        "this frame at {here:#x} is outside the stack range {low:#x}..{high:#x}"
    );
}

/// Every `.rs` file under `dir`, recursively.
fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

/// The `(line, name)` of every foreign function declared in `text`.
fn declarations(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut inside = false;
    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if inside {
            if trimmed.starts_with('}') {
                inside = false;
            } else if let Some(name) = declared_fn(trimmed) {
                out.push((i + 1, name));
            }
        } else if trimmed.contains("extern \"C\" {") {
            inside = true;
        }
    }
    out
}

/// The name of a `fn` declaration line, whatever its visibility.
fn declared_fn(line: &str) -> Option<String> {
    let rest = line
        .strip_prefix("pub(super) ")
        .or_else(|| line.strip_prefix("pub(crate) "))
        .or_else(|| line.strip_prefix("pub "))
        .unwrap_or(line)
        .strip_prefix("fn ")?;
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

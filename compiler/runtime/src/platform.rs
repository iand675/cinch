//! Platform layer runtime support (P34): the `cinch_*` externs behind
//! the prelude's `Effect.Clock`, `Effect.Fs` and `Effect.Process`
//! homes (and the `Data.Path` values they carry).
//!
//! The foreign boundary of these homes is the plain `foreign import
//! ccall` machinery (foreign function interface specification
//! sections 2-4): machine scalars only, never boxed Cinch words. So
//! paths and content bytes cross through WORD BUFFERS, small owned
//! byte vectors behind an `i64` handle word, filled and drained one
//! byte at a time from the language side (`Data.Bytes`' intrinsics
//! supply the bytes). The handle words follow the same trust model as
//! the scope words of `crate::task`: the language side only passes
//! handles its own `cinch_buf_new` calls returned.
//!
//! Every extern returns an `i64` where a NEGATIVE value is the negated
//! error code of the named vocabulary below and a non-negative value is
//! the result (0 for success of an action, a handle, a size, an exit
//! code). The named vocabulary (`NotFound`, `PermissionDenied`, ...)
//! is the prelude's `FsError`/`ProcessError` shape; the codes are
//! shared between the two homes.
//!
//! The clocks are runtime capabilities in the `Io r` mold (decision
//! D45's determinism rule): they are consulted only when a program
//! with the `Io r` capability calls them at runtime, never ambient at
//! compile time, so a program's build output cannot depend on when it
//! compiles.
//!
//! * `cinch_buf_new() -> i64`, `cinch_buf_free(h) -> i64`,
//!   `cinch_buf_push(h, byte) -> i64`, `cinch_buf_len(h) -> i64`,
//!   `cinch_buf_get(h, i) -> i64`: the word-buffer bridge.
//! * `cinch_clock_monotonic_ns() -> i64`, `cinch_clock_wall_ns() ->
//!   i64`: the clocks (nanoseconds; the monotonic one since an
//!   arbitrary origin, the wall one since the Unix epoch).
//! * `cinch_fs_read(path, out) -> i64`, `cinch_fs_write(path, data)
//!   -> i64`, `cinch_fs_rename(from, to) -> i64`,
//!   `cinch_fs_meta_kind(path) -> i64`, `cinch_fs_meta_size(path) ->
//!   i64`, `cinch_fs_dir_open(path) -> i64`, `cinch_fs_dir_next(dir,
//!   out) -> i64`, `cinch_fs_dir_close(dir) -> i64`: the filesystem
//!   operations (`fs-rename` is the atomic one).
//! * `cinch_proc_new(prog) -> i64`, `cinch_proc_arg(proc, arg) ->
//!   i64`, `cinch_proc_env(proc, kv) -> i64`, `cinch_proc_stdin(proc,
//!   data) -> i64`, `cinch_proc_run(proc) -> i64`,
//!   `cinch_proc_read_out(proc, out) -> i64`, `cinch_proc_wait(proc)
//!   -> i64`, `cinch_proc_free(proc) -> i64`: the subprocess
//!   operations over real pipes (the `clang`/`lld` driver shape).
//!
//! The exports are `#[unsafe(no_mangle)]` `cinch_*` externs verified
//! with `nm` by the tests below (the standing runtime rule).

use std::collections::BTreeMap;
use std::sync::LazyLock;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Error vocabulary (shared by Effect.Fs and Effect.Process)
// ---------------------------------------------------------------------------

/// The named error codes a negative result carries, negated. The
/// prelude maps them to its named error constructors one for one.
const NOT_FOUND: i64 = 1;
const PERMISSION_DENIED: i64 = 2;
const ALREADY_EXISTS: i64 = 3;
const NOT_A_DIRECTORY: i64 = 4;
const IS_A_DIRECTORY: i64 = 5;
const INVALID_ARGUMENT: i64 = 6;
const IO_ERROR: i64 = 7;

/// Map a failed `std::io` operation to the named code.
fn io_code(e: &std::io::Error) -> i64 {
    use std::io::ErrorKind::*;
    match e.kind() {
        NotFound => NOT_FOUND,
        PermissionDenied => PERMISSION_DENIED,
        AlreadyExists => ALREADY_EXISTS,
        NotADirectory => NOT_A_DIRECTORY,
        IsADirectory => IS_A_DIRECTORY,
        InvalidInput => INVALID_ARGUMENT,
        _ => IO_ERROR,
    }
}

// ---------------------------------------------------------------------------
// Word buffers
// ---------------------------------------------------------------------------

/// Borrow the vector behind a buffer handle.
unsafe fn buf<'a>(h: i64) -> Option<&'a mut Vec<u8>> {
    if h == 0 {
        return None;
    }
    Some(unsafe { &mut *(h as *mut Vec<u8>) })
}

/// Allocate an empty word buffer; the handle word.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_buf_new() -> i64 {
    Box::into_raw(Box::new(Vec::<u8>::new())) as usize as i64
}

/// Release a word buffer. A null handle is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_buf_free(h: i64) -> i64 {
    if h == 0 {
        return 0;
    }
    drop(unsafe { Box::from_raw(h as *mut Vec<u8>) });
    0
}

/// Append one byte (0..=255) to a word buffer: 0, or the negated
/// `InvalidArgument` code for a bad handle or byte.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_buf_push(h: i64, byte: i64) -> i64 {
    let Ok(b) = u8::try_from(byte) else {
        return -INVALID_ARGUMENT;
    };
    match unsafe { buf(h) } {
        Some(v) => {
            v.push(b);
            0
        }
        None => -INVALID_ARGUMENT,
    }
}

/// A word buffer's length, or the negated `InvalidArgument` code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_buf_len(h: i64) -> i64 {
    match unsafe { buf(h) } {
        Some(v) => v.len() as i64,
        None => -INVALID_ARGUMENT,
    }
}

/// A word buffer's byte at `i` (0..=255), or the negated
/// `InvalidArgument` code for a bad handle or index.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_buf_get(h: i64, i: i64) -> i64 {
    let Some(v) = (unsafe { buf(h) }) else {
        return -INVALID_ARGUMENT;
    };
    let Ok(i) = usize::try_from(i) else {
        return -INVALID_ARGUMENT;
    };
    match v.get(i) {
        Some(&b) => i64::from(b),
        None => -INVALID_ARGUMENT,
    }
}

/// Borrow a handle's bytes for the filesystem and process operations.
unsafe fn buf_slice<'a>(h: i64) -> Option<&'a [u8]> {
    unsafe { buf(h) }.map(|v| v.as_slice())
}

/// A handle's bytes decoded as a path (raw OS bytes; not necessarily
/// UTF-8, though the prelude builds them from text).
unsafe fn buf_path(h: i64) -> Option<std::path::PathBuf> {
    use std::os::unix::ffi::OsStringExt;
    unsafe { buf_slice(h) }.map(|b| std::path::PathBuf::from(std::ffi::OsString::from_vec(b.to_vec())))
}

// ---------------------------------------------------------------------------
// Clocks
// ---------------------------------------------------------------------------

/// The monotonic clock's origin (first use), so the reading is small
/// and monotonic.
static MONO_ORIGIN: LazyLock<Instant> = LazyLock::new(Instant::now);

/// Nanoseconds since an arbitrary origin: non-decreasing across calls.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_clock_monotonic_ns() -> i64 {
    let d = MONO_ORIGIN.elapsed();
    i64::try_from(d.as_nanos()).unwrap_or(i64::MAX)
}

/// Nanoseconds since the Unix epoch (clamped to `0..=i64::MAX`: the
/// wall clock never reports before the epoch or past the year 2262
/// here; those bounds are the reading's documented range).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_clock_wall_ns() -> i64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => i64::try_from(d.as_nanos()).unwrap_or(i64::MAX),
        Err(_) => 0,
    }
}

// ---------------------------------------------------------------------------
// Filesystem
// ---------------------------------------------------------------------------

/// Read the file at `path` into `out` (replacing its contents): 0, or
/// the negated error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_read(path: i64, out: i64) -> i64 {
    let (Some(p), Some(o)) = (unsafe { buf_path(path) }, unsafe { buf(out) }) else {
        return -INVALID_ARGUMENT;
    };
    match std::fs::read(&p) {
        Ok(bytes) => {
            *o = bytes;
            0
        }
        Err(e) => -io_code(&e),
    }
}

/// Write `data` to the file at `path`, creating or truncating it: 0,
/// or the negated error code. (Atomic replacement is `fs-rename`.)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_write(path: i64, data: i64) -> i64 {
    let (Some(p), Some(d)) = (unsafe { buf_path(path) }, unsafe { buf_slice(data) }) else {
        return -INVALID_ARGUMENT;
    };
    match std::fs::write(&p, d) {
        Ok(()) => 0,
        Err(e) => -io_code(&e),
    }
}

/// Atomically rename `from` to `to` (POSIX `rename`: the target sees
/// the whole old or the whole new content, never a partial): 0, or
/// the negated error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_rename(from: i64, to: i64) -> i64 {
    let (Some(f), Some(t)) = (unsafe { buf_path(from) }, unsafe { buf_path(to) }) else {
        return -INVALID_ARGUMENT;
    };
    match std::fs::rename(&f, &t) {
        Ok(()) => 0,
        Err(e) => -io_code(&e),
    }
}

/// The kind of the file at `path`: 0 file, 1 directory, 2 other, or
/// the negated error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_meta_kind(path: i64) -> i64 {
    let Some(p) = (unsafe { buf_path(path) }) else {
        return -INVALID_ARGUMENT;
    };
    match std::fs::metadata(&p) {
        Ok(m) => {
            if m.is_file() {
                0
            } else if m.is_dir() {
                1
            } else {
                2
            }
        }
        Err(e) => -io_code(&e),
    }
}

/// The size of the file at `path` in bytes, or the negated error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_meta_size(path: i64) -> i64 {
    let Some(p) = (unsafe { buf_path(path) }) else {
        return -INVALID_ARGUMENT;
    };
    match std::fs::metadata(&p) {
        Ok(m) => i64::try_from(m.len()).unwrap_or(i64::MAX),
        Err(e) => -io_code(&e),
    }
}

/// Open the directory at `path` for listing: a directory handle word,
/// or the negated error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_dir_open(path: i64) -> i64 {
    let Some(p) = (unsafe { buf_path(path) }) else {
        return -INVALID_ARGUMENT;
    };
    match std::fs::read_dir(&p) {
        Ok(rd) => Box::into_raw(Box::new(rd)) as usize as i64,
        Err(e) => -io_code(&e),
    }
}

/// The next directory entry's file name into `out` (replacing its
/// contents): 1 with an entry, 0 at the end of the listing, or the
/// negated error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_dir_next(dir: i64, out: i64) -> i64 {
    use std::os::unix::ffi::OsStringExt;
    if dir == 0 {
        return -INVALID_ARGUMENT;
    }
    let rd = unsafe { &mut *(dir as *mut std::fs::ReadDir) };
    let Some(o) = (unsafe { buf(out) }) else {
        return -INVALID_ARGUMENT;
    };
    match rd.next() {
        None => 0,
        Some(Err(e)) => -io_code(&e),
        Some(Ok(entry)) => {
            *o = entry.file_name().into_vec();
            1
        }
    }
}

/// Release a directory handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_dir_close(dir: i64) -> i64 {
    if dir == 0 {
        return 0;
    }
    drop(unsafe { Box::from_raw(dir as *mut std::fs::ReadDir) });
    0
}

// ---------------------------------------------------------------------------
// Processes
// ---------------------------------------------------------------------------

/// A subprocess being assembled or run: the program, its arguments,
/// its `KEY=VALUE` environment additions, its stdin payload, and the
/// captured stdout.
struct Proc {
    program: std::path::PathBuf,
    args: Vec<std::ffi::OsString>,
    env: BTreeMap<std::ffi::OsString, std::ffi::OsString>,
    stdin: Vec<u8>,
    child: Option<std::process::Child>,
    stdout: Vec<u8>,
    ran: bool,
}

/// Borrow a process handle.
unsafe fn proc<'a>(h: i64) -> Option<&'a mut Proc> {
    if h == 0 {
        return None;
    }
    Some(unsafe { &mut *(h as *mut Proc) })
}

/// Begin assembling a subprocess of the program at `prog`: a process
/// handle word, or the negated error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_proc_new(prog: i64) -> i64 {
    let Some(p) = (unsafe { buf_path(prog) }) else {
        return -INVALID_ARGUMENT;
    };
    let proc = Proc {
        program: p,
        args: Vec::new(),
        env: BTreeMap::new(),
        stdin: Vec::new(),
        child: None,
        stdout: Vec::new(),
        ran: false,
    };
    Box::into_raw(Box::new(proc)) as usize as i64
}

/// Append one argument: 0, or the negated error code. (Only valid
/// before `cinch_proc_run`.)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_proc_arg(h: i64, arg: i64) -> i64 {
    use std::os::unix::ffi::OsStringExt;
    let (Some(p), Some(a)) = (unsafe { proc(h) }, unsafe { buf_slice(arg) }) else {
        return -INVALID_ARGUMENT;
    };
    if p.ran {
        return -INVALID_ARGUMENT;
    }
    p.args.push(std::ffi::OsString::from_vec(a.to_vec()));
    0
}

/// Add one environment entry from a `KEY=VALUE` buffer (the first `=`
/// separates; a value may contain `=`): 0, or the negated error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_proc_env(h: i64, kv: i64) -> i64 {
    use std::os::unix::ffi::OsStringExt;
    let (Some(p), Some(kv)) = (unsafe { proc(h) }, unsafe { buf_slice(kv) }) else {
        return -INVALID_ARGUMENT;
    };
    if p.ran {
        return -INVALID_ARGUMENT;
    }
    let Some(eq) = kv.iter().position(|&b| b == b'=') else {
        return -INVALID_ARGUMENT;
    };
    if eq == 0 {
        return -INVALID_ARGUMENT;
    }
    p.env.insert(
        std::ffi::OsString::from_vec(kv[..eq].to_vec()),
        std::ffi::OsString::from_vec(kv[eq + 1..].to_vec()),
    );
    0
}

/// Append to the subprocess's stdin payload (written and closed when
/// it runs): 0, or the negated error code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_proc_stdin(h: i64, data: i64) -> i64 {
    let (Some(p), Some(d)) = (unsafe { proc(h) }, unsafe { buf_slice(data) }) else {
        return -INVALID_ARGUMENT;
    };
    if p.ran {
        return -INVALID_ARGUMENT;
    }
    p.stdin.extend_from_slice(d);
    0
}

/// Run the subprocess with real pipes: 0 once it has STARTED (its
/// stdout pipe is drained by `cinch_proc_read_out`), or the negated
/// error code. The child inherits this process's stderr.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_proc_run(h: i64) -> i64 {
    let Some(p) = (unsafe { proc(h) }) else {
        return -INVALID_ARGUMENT;
    };
    if p.ran {
        return -INVALID_ARGUMENT;
    }
    let mut cmd = std::process::Command::new(&p.program);
    cmd.args(&p.args);
    for (k, v) in &p.env {
        cmd.env(k, v);
    }
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    match cmd.spawn() {
        Ok(mut child) => {
            // Write the stdin payload and close the pipe (the child
            // sees end of input). The payload is bounded by the pipe
            // buffer only while the child does not read; the worked
            // drivers (`clang`, `lld`) consume theirs.
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                let _ = stdin.write_all(&p.stdin);
                let _ = stdin.flush();
            }
            p.stdin.clear();
            p.child = Some(child);
            p.ran = true;
            0
        }
        Err(e) => -io_code(&e),
    }
}

/// Drain the subprocess's stdout pipe into `out` (replacing its
/// contents) and return its length: the call returns at the pipe's end
/// (the child closed its stdout or exited), or the negated error code.
/// Repeated calls after the drain report the remaining output (empty
/// once taken).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_proc_read_out(h: i64, out: i64) -> i64 {
    let (Some(p), Some(o)) = (unsafe { proc(h) }, unsafe { buf(out) }) else {
        return -INVALID_ARGUMENT;
    };
    if !p.ran {
        return -INVALID_ARGUMENT;
    }
    if p.stdout.is_empty() {
        if let Some(child) = p.child.as_mut() {
            if let Some(mut so) = child.stdout.take() {
                use std::io::Read;
                let mut buf = Vec::new();
                match so.read_to_end(&mut buf) {
                    Ok(_) => p.stdout = buf,
                    Err(e) => return -io_code(&e),
                }
            }
        }
    }
    *o = std::mem::take(&mut p.stdout);
    o.len() as i64
}

/// Wait for the subprocess and report its exit code (0..=255; a
/// signal death reports 128 + the signal number), or the negated
/// error code when it was never run.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_proc_wait(h: i64) -> i64 {
    let Some(p) = (unsafe { proc(h) }) else {
        return -INVALID_ARGUMENT;
    };
    let Some(mut child) = p.child.take() else {
        return -INVALID_ARGUMENT;
    };
    match child.wait() {
        Ok(status) => match status.code() {
            Some(code) => i64::from(code),
            None => {
                #[cfg(unix)]
                {
                    use std::os::unix::process::ExitStatusExt;
                    128 + i64::from(status.signal().unwrap_or(0))
                }
                #[cfg(not(unix))]
                {
                    128
                }
            }
        },
        Err(e) => -io_code(&e),
    }
}

/// Release a process handle; a still-running child is killed and
/// reaped first (no zombies left behind).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_proc_free(h: i64) -> i64 {
    if h == 0 {
        return 0;
    }
    let mut p = unsafe { Box::from_raw(h as *mut Proc) };
    if let Some(mut child) = p.child.take() {
        let _ = child.kill();
        let _ = child.wait();
    }
    drop(p);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The standing runtime rule: every `cinch_*` export is a global
    /// text symbol in the built runtime artifact (`nm`-verified, as in
    /// `crate::text`).
    #[test]
    fn platform_exports_are_global_text_symbols() {
        let mut candidates: Vec<std::path::PathBuf> = Vec::new();
        let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("target");
        let mut stack = vec![dir.clone()];
        while let Some(d) = stack.pop() {
            if let Ok(entries) = std::fs::read_dir(&d) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else {
                        let name = entry.file_name().to_string_lossy().into_owned();
                        if name.starts_with("libcinchruntime-")
                            && (name.ends_with(".rlib") || name.ends_with(".a"))
                        {
                            candidates.push(path);
                        }
                    }
                }
            }
        }
        for sub in ["debug", "release"] {
            let dir = dir.join(sub);
            for artifact in ["libcinchruntime.a"] {
                let p = dir.join(artifact);
                if p.exists() {
                    candidates.push(p);
                }
            }
            let deps = dir.join("deps");
            if let Ok(entries) = std::fs::read_dir(&deps) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_with("libcinchruntime-")
                        && (name.ends_with(".rlib") || name.ends_with(".a"))
                    {
                        candidates.push(entry.path());
                    }
                }
            }
        }
        let syms = [
            "cinch_buf_new",
            "cinch_buf_free",
            "cinch_buf_push",
            "cinch_buf_len",
            "cinch_buf_get",
            "cinch_clock_monotonic_ns",
            "cinch_clock_wall_ns",
            "cinch_fs_read",
            "cinch_fs_write",
            "cinch_fs_rename",
            "cinch_fs_meta_kind",
            "cinch_fs_meta_size",
            "cinch_fs_dir_open",
            "cinch_fs_dir_next",
            "cinch_fs_dir_close",
            "cinch_proc_new",
            "cinch_proc_arg",
            "cinch_proc_env",
            "cinch_proc_stdin",
            "cinch_proc_run",
            "cinch_proc_read_out",
            "cinch_proc_wait",
            "cinch_proc_free",
        ];
        for archive in &candidates {
            let out = std::process::Command::new("nm")
                .arg(archive)
                .output()
                .expect("run nm on the runtime artifact");
            let listing = String::from_utf8_lossy(&out.stdout);
            let all_present = syms.iter().all(|sym| {
                listing.lines().any(|line| {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    // `nm` prints `address T name` (GNU) or `address T
                    // _name` (Mach-O); `U` marks an undefined
                    // reference, which is not an export.
                    fields.len() >= 3
                        && fields[1] == "T"
                        && (fields[2] == *sym || fields[2] == format!("_{sym}"))
                })
            });
            if all_present {
                return;
            }
        }
        panic!(
            "the cinch_platform exports are not global text symbols in any runtime artifact ({candidates:?})"
        );
    }

    /// The word-buffer bridge round trips bytes.
    #[test]
    fn word_buffers_round_trip() {
        let h = cinch_buf_new();
        assert_eq!(unsafe { cinch_buf_push(h, 65) }, 0);
        assert_eq!(unsafe { cinch_buf_push(h, 66) }, 0);
        assert!(unsafe { cinch_buf_push(h, 256) } < 0);
        assert_eq!(unsafe { cinch_buf_len(h) }, 2);
        assert_eq!(unsafe { cinch_buf_get(h, 0) }, 65);
        assert_eq!(unsafe { cinch_buf_get(h, 1) }, 66);
        assert!(unsafe { cinch_buf_get(h, 2) } < 0);
        assert_eq!(unsafe { cinch_buf_free(h) }, 0);
    }

    /// The clocks answer and the monotonic one does not go backwards.
    #[test]
    fn clock_readings_are_ordered_and_finite() {
        let a = cinch_clock_monotonic_ns();
        let b = cinch_clock_monotonic_ns();
        assert!(a <= b, "monotonic readings never go backwards: {a} > {b}");
        assert!(cinch_clock_wall_ns() > 0);
    }

    /// The filesystem operations over a real scratch directory.
    #[test]
    fn fs_round_trip_rename_and_named_errors() {
        let dir = std::env::temp_dir().join(format!("cinch_platform_fs_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let mk = |s: &str| -> i64 {
            let h = cinch_buf_new();
            for b in s.as_bytes() {
                assert_eq!(unsafe { cinch_buf_push(h, i64::from(*b)) }, 0);
            }
            h
        };
        let text = |h: i64| -> String {
            let n = unsafe { cinch_buf_len(h) };
            let mut out = Vec::new();
            for i in 0..n {
                out.push(unsafe { cinch_buf_get(h, i) } as u8);
            }
            String::from_utf8(out).expect("utf-8")
        };
        let a = mk(&format!("{}/a.txt", dir.display()));
        let b = mk(&format!("{}/b.txt", dir.display()));
        let content = mk("hello");
        let out = cinch_buf_new();
        assert_eq!(unsafe { cinch_fs_write(a, content) }, 0);
        assert_eq!(unsafe { cinch_fs_read(a, out) }, 0);
        assert_eq!(text(out), "hello");
        assert_eq!(unsafe { cinch_fs_meta_kind(a) }, 0);
        assert_eq!(unsafe { cinch_fs_meta_size(a) }, 5);
        assert_eq!(unsafe { cinch_fs_rename(a, b) }, 0);
        assert_eq!(unsafe { cinch_fs_read(b, out) }, 0);
        assert_eq!(text(out), "hello");
        assert_eq!(unsafe { cinch_fs_meta_kind(a) }, -NOT_FOUND);
        let missing = mk(&format!("{}/missing.txt", dir.display()));
        assert_eq!(unsafe { cinch_fs_read(missing, out) }, -NOT_FOUND);
        assert_eq!(unsafe { cinch_fs_meta_kind(missing) }, -NOT_FOUND);
        // The listing sees both scratch names.
        let dirh = mk(&format!("{}", dir.display()));
        let dh = unsafe { cinch_fs_dir_open(dirh) };
        assert!(dh > 0);
        let mut names = Vec::new();
        loop {
            match unsafe { cinch_fs_dir_next(dh, out) } {
                0 => break,
                1 => names.push(text(out)),
                e => panic!("dir_next failed: {e}"),
            }
        }
        assert_eq!(unsafe { cinch_fs_dir_close(dh) }, 0);
        assert!(names.contains(&"b.txt".to_string()), "listing {names:?}");
        for h in [a, b, content, out, missing, dirh] {
            assert_eq!(unsafe { cinch_buf_free(h) }, 0);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A real subprocess: args on the command line, env in its
    /// environment, stdin and stdout over pipes, exit code through the
    /// wait.
    #[test]
    fn process_capture_round_trips() {
        let mk = |s: &str| -> i64 {
            let h = cinch_buf_new();
            for b in s.as_bytes() {
                assert_eq!(unsafe { cinch_buf_push(h, i64::from(*b)) }, 0);
            }
            h
        };
        let text = |h: i64| -> String {
            let n = unsafe { cinch_buf_len(h) };
            let mut out = Vec::new();
            for i in 0..n {
                out.push(unsafe { cinch_buf_get(h, i) } as u8);
            }
            String::from_utf8(out).expect("utf-8")
        };
        let sh = mk("/bin/sh");
        let p = unsafe { cinch_proc_new(sh) };
        assert!(p > 0);
        let out = cinch_buf_new();
        // args + env + stdin through the pipes.
        for a in ["-c", "cat; echo \"arg:$1 env:$PROBE\"", "sh", "A"] {
            let ab = mk(a);
            assert_eq!(unsafe { cinch_proc_arg(p, ab) }, 0);
            unsafe { cinch_buf_free(ab) };
        }
        let kv = mk("PROBE=visible");
        assert_eq!(unsafe { cinch_proc_env(p, kv) }, 0);
        unsafe { cinch_buf_free(kv) };
        let data = mk("payload\n");
        assert_eq!(unsafe { cinch_proc_stdin(p, data) }, 0);
        unsafe { cinch_buf_free(data) };
        assert_eq!(unsafe { cinch_proc_run(p) }, 0);
        assert_eq!(unsafe { cinch_proc_read_out(p, out) }, "payload\narg:A env:visible\n".len() as i64);
        assert_eq!(text(out), "payload\narg:A env:visible\n");
        assert_eq!(unsafe { cinch_proc_wait(p) }, 0);
        assert_eq!(unsafe { cinch_proc_free(p) }, 0);
        // A failing exit code comes through the wait.
        let p = unsafe { cinch_proc_new(sh) };
        for a in ["-c", "exit 7"] {
            let ab = mk(a);
            assert_eq!(unsafe { cinch_proc_arg(p, ab) }, 0);
            unsafe { cinch_buf_free(ab) };
        }
        assert_eq!(unsafe { cinch_proc_run(p) }, 0);
        assert_eq!(unsafe { cinch_proc_wait(p) }, 7);
        assert_eq!(unsafe { cinch_proc_free(p) }, 0);
        unsafe { cinch_buf_free(sh) };
        unsafe { cinch_buf_free(out) };
    }
}

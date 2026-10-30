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
//! * `cinch_fs_mkdir(path, parents) -> i64` (one directory, or the
//!   whole chain when `parents` is nonzero, matching `mkdir -p`),
//!   `cinch_fs_remove(path, recursive) -> i64` (a file, a directory, or
//!   a subtree; a symlink removes the link, never its target) and
//!   `cinch_fs_temp_dir() -> i64` (the platform temp directory as a
//!   `Bytes` path): the mutating filesystem operations the standard
//!   library's `Dir`/`FileUtils` round needs.
//! * `cinch_random_bytes(n) -> i64`: `n` bytes from the OS entropy
//!   source as a fresh `Bytes` (`/dev/urandom`; the runtime carries no
//!   external crate).
//! * `cinch_signal_send(pid, sig) -> i64`: POSIX `kill` (the raw
//!   result: 0, or -1 on failure, which the Cinch side maps to a named
//!   error).
//! * `cinch_argv_init(argc, argv) -> i64`, `cinch_argv_count() -> i64`,
//!   `cinch_argv_get(i) -> i64`: this process's OWN command line. The
//!   generated entry glue stashes `argc`/`argv` through `cinch_argv_init`
//!   before any program code runs (so a compiled program can read its
//!   own arguments); word 0 is the program name, the count includes it,
//!   and an index outside `0..count` takes the defined out-of-bounds
//!   fault. A `main` a C caller supplies (`foreign export`) never
//!   captures, so the count is 0 there.
//! * `cinch_run_main(entry) -> i64`: the generated entry glue's next
//!   step after `cinch_argv_init`: run the program's entry unit on a
//!   thread with a 1 GiB reserved stack (`CINCH_STACK_SIZE=<n>[K|M|G]`
//!   overrides it) inside the stack guard of `crate::stack`, and answer
//!   its result word. Overflowing the stack, or the OS refusing the
//!   thread, is the `Exhausted` exit 73.
//! * `cinch_proc_new(prog) -> i64`, `cinch_proc_arg(proc, arg) ->
//!   i64`, `cinch_proc_env(proc, kv) -> i64`, `cinch_proc_stdin(proc,
//!   data) -> i64`, `cinch_proc_run(proc) -> i64`,
//!   `cinch_proc_read_out(proc, out) -> i64`, `cinch_proc_wait(proc)
//!   -> i64`, `cinch_proc_free(proc) -> i64`: the subprocess
//!   operations over real pipes (the `clang`/`lld` driver shape).
//! * `cinch_process_spawn(command, args, cwd, env) -> i64`,
//!   `cinch_process_write_stdin(child, data) -> i64`,
//!   `cinch_process_close_stdin(child) -> i64`,
//!   `cinch_process_read(child, which, max) -> i64`,
//!   `cinch_process_wait(child) -> i64`, `cinch_process_close(child)
//!   -> i64`: the LIVE child of the piped-process ABI (separately
//!   readable stdout and stderr, writable stdin, a handle table), which
//!   the one-shot set above cannot express. Its text and payload
//!   arguments cross as `Bytes` words (`crate::abi`) rather than
//!   through the word buffers of the one-shot set: a long-lived child
//!   is a language-level value stream, not a byte-at-a-time bridge.
//!
//! The exports are `#[unsafe(no_mangle)]` `cinch_*` externs verified
//! with `nm` by the tests below (the standing runtime rule).

use std::collections::BTreeMap;
use std::sync::{LazyLock, OnceLock};
use std::time::Instant;

// ---------------------------------------------------------------------------
// Error vocabulary (shared by Effect.Fs and Effect.Process)
// ---------------------------------------------------------------------------

/// The named error codes a negative result carries, negated. The
/// prelude maps them to its named error constructors one for one.
/// `crate::net` reads them from here: one vocabulary spans the
/// platform's externs, so a socket failure and a filesystem failure
/// name the same codes.
pub(crate) const NOT_FOUND: i64 = 1;
pub(crate) const PERMISSION_DENIED: i64 = 2;
pub(crate) const ALREADY_EXISTS: i64 = 3;
pub(crate) const NOT_A_DIRECTORY: i64 = 4;
pub(crate) const IS_A_DIRECTORY: i64 = 5;
pub(crate) const INVALID_ARGUMENT: i64 = 6;
pub(crate) const IO_ERROR: i64 = 7;

/// Map a failed `std::io` operation to the named code.
pub(crate) fn io_code(e: &std::io::Error) -> i64 {
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

/// The path a `Bytes` word names (the intrinsic boundary: a boxed
/// `Bytes` block, `textSha256`'s shape, not a word-buffer handle).
unsafe fn bytes_path(word: i64) -> std::path::PathBuf {
    use std::os::unix::ffi::OsStringExt;
    let b = unsafe { crate::abi::as_bytes(word) };
    std::path::PathBuf::from(std::ffi::OsString::from_vec(b.as_slice().to_vec()))
}

/// Create the directory at `path`, or the whole missing chain when
/// `parents` is nonzero (`mkdir -p`: an existing directory is success):
/// 0, or the negated error code. Without `parents` a missing parent or
/// an existing `path` is the error the OS reports (`AlreadyExists`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_mkdir(path: i64, parents: i64) -> i64 {
    let p = unsafe { bytes_path(path) };
    let r = if parents != 0 {
        std::fs::create_dir_all(&p)
    } else {
        std::fs::create_dir(&p)
    };
    match r {
        Ok(()) => 0,
        Err(e) => -io_code(&e),
    }
}

/// Remove what `path` names: a regular file or a symlink, or (when
/// `recursive` is nonzero) a whole directory subtree: 0, or the negated
/// error code. A missing path is the error `NotFound`. The removal
/// resolves the kind through `symlink_metadata` and removes a symlink
/// itself, so a link is NEVER followed to remove its target, and the
/// recursive path (`remove_dir_all`) does not follow the links it walks
/// either.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_fs_remove(path: i64, recursive: i64) -> i64 {
    let p = unsafe { bytes_path(path) };
    let meta = match std::fs::symlink_metadata(&p) {
        Ok(m) => m,
        Err(e) => return -io_code(&e),
    };
    let r = if meta.file_type().is_dir() {
        if recursive != 0 {
            std::fs::remove_dir_all(&p)
        } else {
            std::fs::remove_dir(&p)
        }
    } else {
        std::fs::remove_file(&p)
    };
    match r {
        Ok(()) => 0,
        Err(e) => -io_code(&e),
    }
}

/// The platform temp directory as a fresh `Bytes` path (`std::env::
/// temp_dir`'s choice: `$TMPDIR`, else `/tmp`). The Cinch side composes
/// unique names under it; the path is not created here.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_fs_temp_dir() -> i64 {
    use std::os::unix::ffi::OsStrExt;
    let bytes = std::env::temp_dir().as_os_str().as_bytes().to_vec();
    crate::abi::box_bytes(crate::bytes::Bytes::from(bytes))
}

// ---------------------------------------------------------------------------
// Entropy
// ---------------------------------------------------------------------------

/// `n` bytes from the OS entropy source as a fresh `Bytes` buffer, or
/// the negated error code for a negative `n` or a failed read. The
/// source is `/dev/urandom` (a character device that never blocks once
/// seeded; the runtime carries no external crate).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_random_bytes(n: i64) -> i64 {
    use std::io::Read;
    let Ok(len) = usize::try_from(n) else {
        return -INVALID_ARGUMENT;
    };
    let mut buf = vec![0u8; len];
    match std::fs::File::open("/dev/urandom").and_then(|mut f| f.read_exact(&mut buf)) {
        Ok(()) => crate::abi::box_bytes(crate::bytes::Bytes::from(buf)),
        Err(e) => -io_code(&e),
    }
}

// ---------------------------------------------------------------------------
// Signals
// ---------------------------------------------------------------------------

unsafe extern "C" {
    /// POSIX `kill(2)` (the signal numbers are the platform's).
    fn kill(pid: i32, sig: i32) -> i32;
}

/// Send signal `sig` (a POSIX signal number) to process `pid`: the raw
/// `kill` result (0, or -1 on failure), which the Cinch side maps to its
/// named error. A `pid` outside the `i32` range is `InvalidArgument`.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_signal_send(pid: i64, sig: i64) -> i64 {
    let (Ok(pid), Ok(sig)) = (i32::try_from(pid), i32::try_from(sig)) else {
        return -INVALID_ARGUMENT;
    };
    i64::from(unsafe { kill(pid, sig) })
}

// ---------------------------------------------------------------------------
// This process's own command line (argv)
// ---------------------------------------------------------------------------

/// The command line the generated entry glue captured: one byte string
/// per OS `argv` word, the program name first, exactly as the operating
/// system passed it.
///
/// The glue calls [`cinch_argv_init`] before any program code runs
/// (`compiler/src/codegen.rs` emits the call in the native `@main`), and
/// the words are copied in here because the entry's own `argv` is only
/// guaranteed for the duration of `main`. A first capture wins; a
/// program whose `main` a C caller supplies (`foreign export`) never
/// captures at all, so the count is 0 there.
static ARGV: OnceLock<Vec<Vec<u8>>> = OnceLock::new();

/// Capture the process's `argc`/`argv`: 0. Every word is copied (the
/// caller's array and its strings need not outlive the call). A null
/// `argv`, a negative `argc`, or a repeated call leaves the capture as
/// it is.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_argv_init(argc: i64, argv: *const *const u8) -> i64 {
    if argv.is_null() {
        return 0;
    }
    let Ok(n) = usize::try_from(argc) else {
        return 0;
    };
    let mut words = Vec::with_capacity(n);
    for i in 0..n {
        let p = unsafe { *argv.add(i) };
        if p.is_null() {
            words.push(Vec::new());
            continue;
        }
        let mut len = 0usize;
        while unsafe { *p.add(len) } != 0 {
            len += 1;
        }
        words.push(unsafe { std::slice::from_raw_parts(p, len) }.to_vec());
    }
    let _ = ARGV.set(words);
    0
}

/// How many words the captured command line has (0 when nothing was
/// captured). Word 0 is the program name.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_argv_count() -> i64 {
    ARGV.get().map_or(0, |words| words.len() as i64)
}

/// The `i`-th captured word as a fresh `Bytes` word: word 0 is the
/// program name, the rest are its arguments. An index outside
/// `0..count` takes the defined out-of-bounds fault (exit 71, the
/// `Bytes` indexing convention).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_argv_get(i: i64) -> i64 {
    if let Some(words) = ARGV.get() {
        if let Ok(i) = usize::try_from(i) {
            if let Some(word) = words.get(i) {
                return crate::abi::box_bytes(crate::bytes::Bytes::from(word.clone()));
            }
        }
    }
    crate::abi::raise(crate::error::Error::OutOfBounds)
}

// ---------------------------------------------------------------------------
// The program's own thread
// ---------------------------------------------------------------------------

/// The stack reserved for the program's code when `CINCH_STACK_SIZE` is
/// unset: 1 GiB.
pub const DEFAULT_STACK_SIZE: usize = 1 << 30;

/// Run the program's entry unit `entry` on a thread with a large reserved
/// stack and answer its result word; the generated native `@main` calls
/// this right after [`cinch_argv_init`].
///
/// The process's own main thread has the platform's default stack
/// (8 MiB on macOS, and on Linux under the usual `ulimit -s`), and a
/// non-tail recursion over a long list is an ordinary program: generated
/// code keeps every local in its own stack slot, so frames run to
/// kilobytes and the default stack ends after a few thousand of them. The
/// reservation is address space only; the OS commits a page when the
/// recursion first touches it, so a shallow program pays nothing for it.
///
/// `CINCH_STACK_SIZE=<n>[K|M|G]` sets the size (an unparsable or zero
/// value keeps the default; a size below [`crate::stack::MIN_STACK_BYTES`]
/// is raised to it). `entry` runs inside [`crate::stack::guarded`], so
/// overflowing the reservation is the stack `Exhausted` exit (`cinch:
/// exhausted: stack`, exit 73), never a crash. A refused reservation is
/// the `Exhausted` exit too, before any program code runs: `stack` when
/// the OS had no memory for it, `threads` otherwise. A fault inside
/// `entry` leaves through `exit` from the program's thread as before, so
/// the exit code and the `atexit` handlers (the `CINCH_ALLOC_STATS` line)
/// are unchanged.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_run_main(entry: extern "C" fn() -> i64) -> i64 {
    let size = crate::heap::env_bytes(c"CINCH_STACK_SIZE")
        .and_then(crate::heap::parse_limit)
        .filter(|&n| n > 0)
        .and_then(|n| usize::try_from(n).ok())
        .unwrap_or(DEFAULT_STACK_SIZE)
        .max(crate::stack::MIN_STACK_BYTES);
    let spawned = std::thread::Builder::new()
        .stack_size(size)
        .spawn(move || crate::stack::guarded(|| entry()));
    match spawned {
        // Every runtime extern is `extern "C"`, so a Rust panic aborts in
        // place and never unwinds into the join: a failed join is a
        // runtime invariant broken, not a program outcome.
        Ok(handle) => handle
            .join()
            .unwrap_or_else(|_| crate::abi::panic_exit(b"compiler bug: the program thread unwound")),
        Err(e) => crate::stack::spawn_refused(&e),
    }
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

/// A reaped child's exit code, the shell convention: the child's own
/// exit status, or 128 + the signal number when a signal killed it (the
/// raw signal case: there is no exit status to report then). Both the
/// one-shot `cinch_proc_wait` and the piped ABI's `cinch_process_wait`
/// report through this, so one convention spans the two.
pub(crate) fn exit_code(status: &std::process::ExitStatus) -> i64 {
    match status.code() {
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
    }
}

/// Wait for the subprocess and report its exit code through
/// [`exit_code`] (0..=255; a signal death reports 128 + the signal
/// number), or the negated error code when it was never run.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_proc_wait(h: i64) -> i64 {
    let Some(p) = (unsafe { proc(h) }) else {
        return -INVALID_ARGUMENT;
    };
    let Some(mut child) = p.child.take() else {
        return -INVALID_ARGUMENT;
    };
    match child.wait() {
        Ok(status) => exit_code(&status),
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

// ---------------------------------------------------------------------------
// Piped children (the live-child ABI)
// ---------------------------------------------------------------------------

/// A live child of the piped-process ABI: the child itself, the three
/// stdio pipes the caller reads and writes, and the exit code once a
/// `cinch_process_wait` has reaped it.
struct PipedChild {
    child: std::process::Child,
    stdin: Option<std::process::ChildStdin>,
    stdout: Option<std::process::ChildStdout>,
    stderr: Option<std::process::ChildStderr>,
    status: Option<i64>,
}

/// The live piped children, by handle word.
static PIPES: LazyLock<std::sync::Mutex<BTreeMap<i64, PipedChild>>> =
    LazyLock::new(|| std::sync::Mutex::new(BTreeMap::new()));

/// The next child handle word. Handles are handed out once and never
/// reused, so a stale handle is invalid rather than another child's.
static PIPES_NEXT: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(1);

/// The child table: one lock over the entries (the invariant is
/// per-entry, so a poisoned lock is recovered). A blocking pipe
/// operation of one child holds it, so the operations of one child are
/// sequenced against each other and against another child's; the
/// language-side capability is what sequences a caller's own use.
fn pipes() -> std::sync::MutexGuard<'static, BTreeMap<i64, PipedChild>> {
    PIPES.lock().unwrap_or_else(|e| e.into_inner())
}

/// A `Bytes` word's NUL-separated words as OS strings. `Err` is the
/// negated error code for an entry with no bytes: the encoding cannot
/// express an empty argument, so one is refused rather than dropped. A
/// single trailing separator adds no word (the last word of
/// `-c\0echo hi\0` is `echo hi`). `crate::pty`'s spawn shares it (one
/// NUL-separated argument encoding spans the process and terminal
/// families).
pub(crate) unsafe fn nul_words(word: i64) -> std::result::Result<Vec<std::ffi::OsString>, i64> {
    use std::os::unix::ffi::OsStringExt;
    let bytes = unsafe { crate::abi::as_bytes(word) }.as_slice();
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let mut parts: Vec<&[u8]> = bytes.split(|&b| b == 0).collect();
    if parts.last().is_some_and(|p| p.is_empty()) {
        parts.pop();
    }
    let mut out = Vec::new();
    for p in parts {
        if p.is_empty() {
            return Err(-INVALID_ARGUMENT);
        }
        out.push(std::ffi::OsString::from_vec(p.to_vec()));
    }
    Ok(out)
}

/// A `Bytes` word's NUL-separated `KEY=VALUE` entries (the FIRST `=`
/// separates; a value may contain one): `Err` is the negated error code
/// for an entry without a separator or with an empty key. An empty word
/// adds nothing, so the child inherits this process's environment.
/// `crate::pty`'s spawn shares it.
pub(crate) unsafe fn env_words(
    word: i64,
) -> std::result::Result<Vec<(std::ffi::OsString, std::ffi::OsString)>, i64> {
    use std::os::unix::ffi::OsStringExt;
    let bytes = unsafe { crate::abi::as_bytes(word) }.as_slice();
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let mut parts: Vec<&[u8]> = bytes.split(|&b| b == 0).collect();
    if parts.last().is_some_and(|p| p.is_empty()) {
        parts.pop();
    }
    let mut out = Vec::new();
    for p in parts {
        let Some(eq) = p.iter().position(|&b| b == b'=') else {
            return Err(-INVALID_ARGUMENT);
        };
        if eq == 0 {
            return Err(-INVALID_ARGUMENT);
        }
        out.push((
            std::ffi::OsString::from_vec(p[..eq].to_vec()),
            std::ffi::OsString::from_vec(p[eq + 1..].to_vec()),
        ));
    }
    Ok(out)
}

/// Spawn the program `command` (a `Bytes` word of its path, raw OS
/// bytes) with `args` (a `Bytes` word of NUL-separated argument words,
/// empty for none) running in the directory `cwd` (a `Bytes` path; empty
/// inherits this process's own directory) with the environment
/// additions `env` (a `Bytes` word of NUL-separated `KEY=VALUE` entries;
/// empty inherits): a child handle word, or the negated error code
/// (`NotFound` for a program that does not exist, `InvalidArgument` for
/// an argument list or environment the encoding cannot carry).
///
/// The three stdio pipes belong to the child handle: its stdout and
/// stderr are read separately (`cinch_process_read`) and its stdin is
/// written (`cinch_process_write_stdin`, then
/// `cinch_process_close_stdin` for the end of input). The child does
/// NOT inherit this process's stdout or stderr, which is what the
/// one-shot set above cannot do.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_process_spawn(command: i64, args: i64, cwd: i64, env: i64) -> i64 {
    use std::os::unix::ffi::OsStringExt;
    let command = unsafe { crate::abi::as_bytes(command) }.as_slice();
    if command.is_empty() {
        return -INVALID_ARGUMENT;
    }
    let args = match unsafe { nul_words(args) } {
        Ok(w) => w,
        Err(code) => return code,
    };
    let env = match unsafe { env_words(env) } {
        Ok(w) => w,
        Err(code) => return code,
    };
    let cwd = unsafe { crate::abi::as_bytes(cwd) }.as_slice();
    let mut cmd = std::process::Command::new(std::ffi::OsString::from_vec(command.to_vec()));
    cmd.args(args);
    if !cwd.is_empty() {
        cmd.current_dir(std::path::PathBuf::from(std::ffi::OsString::from_vec(cwd.to_vec())));
    }
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    match cmd.spawn() {
        Ok(mut child) => {
            let stdin = child.stdin.take();
            let stdout = child.stdout.take();
            let stderr = child.stderr.take();
            let h = PIPES_NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            pipes().insert(
                h,
                PipedChild {
                    child,
                    stdin,
                    stdout,
                    stderr,
                    status: None,
                },
            );
            h
        }
        Err(e) => -io_code(&e),
    }
}

/// Write a `Bytes` word to the child's stdin: the byte count (the whole
/// buffer is written, and the call blocks until it is), or the negated
/// error code for a handle that is not live, a stdin the caller already
/// closed, or a pipe failure. The stdin pipe leaves the table for the
/// write (one writer at a time, and the lock is not held across the
/// blocking call); a handle closed underneath the write drops it, which
/// closes the pipe. The peer-closed `SIGPIPE` hazard is `crate::net`'s
/// note.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_process_write_stdin(h: i64, data: i64) -> i64 {
    use std::io::Write;
    let data = unsafe { crate::abi::as_bytes(data) }.as_slice();
    let Some(mut stdin) = pipes().get_mut(&h).and_then(|p| p.stdin.take()) else {
        return -INVALID_ARGUMENT;
    };
    let r = stdin.write_all(data);
    if let Some(p) = pipes().get_mut(&h) {
        p.stdin = Some(stdin);
    }
    match r {
        Ok(()) => i64::try_from(data.len()).unwrap_or(i64::MAX),
        Err(e) => -io_code(&e),
    }
}

/// Close the child's stdin, which is how the child sees the end of its
/// input: 0, or the negated error code for a handle that is not live.
/// Closing twice is 0: the caller's intent (a closed stdin) still holds.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_process_close_stdin(h: i64) -> i64 {
    let mut table = pipes();
    let Some(p) = table.get_mut(&h) else {
        return -INVALID_ARGUMENT;
    };
    p.stdin = None;
    0
}

/// Read the next bytes of one of the child's output pipes into a fresh
/// `Bytes` word: `which` is 0 for the child's stdout and 1 for its
/// stderr, and the result is what one platform call produced (at most
/// `max` bytes, empty at the end of that pipe), or the negated error
/// code for a handle or a `which` that is not live. An output pipe the
/// caller already took to its end keeps answering empty (the pipe is at
/// its end, which is not an error), and `max` is both the allocation
/// and the read cap.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_process_read(h: i64, which: i64, max: i64) -> i64 {
    use std::io::Read;
    let Ok(max) = usize::try_from(max) else {
        return -INVALID_ARGUMENT;
    };
    if max == 0 || !matches!(which, 0 | 1) {
        return -INVALID_ARGUMENT;
    }
    let mut table = pipes();
    let Some(p) = table.get_mut(&h) else {
        return -INVALID_ARGUMENT;
    };
    let mut buf = vec![0u8; max];
    let r = if which == 0 {
        match p.stdout.as_mut() {
            Some(s) => s.read(&mut buf),
            None => return -INVALID_ARGUMENT,
        }
    } else {
        match p.stderr.as_mut() {
            Some(s) => s.read(&mut buf),
            None => return -INVALID_ARGUMENT,
        }
    };
    match r {
        Ok(n) => {
            buf.truncate(n);
            crate::abi::box_bytes(crate::bytes::Bytes::from(buf))
        }
        Err(e) => -io_code(&e),
    }
}

/// Wait for the child and report its exit code through [`exit_code`]
/// (0..=255; a signal death reports 128 + the signal number, the raw
/// signal case): the code is remembered, so a second wait answers the
/// same one, and a handle that is not live is `InvalidArgument`. The
/// wait blocks, so a caller drains the output pipes it needs before it
/// waits (a child writing into a full pipe nobody reads is the caller's
/// deadlock, the refusal `Effect.Process`'s header names).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_process_wait(h: i64) -> i64 {
    let mut table = pipes();
    let Some(p) = table.get_mut(&h) else {
        return -INVALID_ARGUMENT;
    };
    if let Some(code) = p.status {
        return code;
    }
    match p.child.wait() {
        Ok(status) => {
            let code = exit_code(&status);
            p.status = Some(code);
            code
        }
        Err(e) => -io_code(&e),
    }
}

/// Close a child: the handle's entry goes away and a child no wait has
/// reaped is killed and reaped first (no zombies left behind). Returns
/// 0, or the negated error code for a handle that is not live; closing
/// twice is the second case, an error rather than a no-op (the handle
/// word is gone).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_process_close(h: i64) -> i64 {
    match pipes().remove(&h) {
        Some(mut p) => {
            if p.status.is_none() {
                let _ = p.child.kill();
                let _ = p.child.wait();
            }
            0
        }
        None => -INVALID_ARGUMENT,
    }
}

/// A live piped child's pid, for the crate's own probe of the raw
/// signal case (the language side signals through `cinch_signal_send`
/// with a pid it obtained some other way).
#[cfg(test)]
pub(crate) fn piped_child_pid(h: i64) -> Option<i32> {
    pipes().get(&h).map(|p| p.child.id() as i32)
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
            "cinch_fs_mkdir",
            "cinch_fs_remove",
            "cinch_fs_temp_dir",
            "cinch_random_bytes",
            "cinch_signal_send",
            "cinch_argv_init",
            "cinch_argv_count",
            "cinch_argv_get",
            "cinch_run_main",
            "cinch_proc_new",
            "cinch_proc_arg",
            "cinch_proc_env",
            "cinch_proc_stdin",
            "cinch_proc_run",
            "cinch_proc_read_out",
            "cinch_proc_wait",
            "cinch_proc_free",
            "cinch_process_spawn",
            "cinch_process_write_stdin",
            "cinch_process_close_stdin",
            "cinch_process_read",
            "cinch_process_wait",
            "cinch_process_close",
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

    /// The mutating filesystem operations, the entropy source and the
    /// signal bridge over a real scratch directory.
    #[test]
    fn fs_mutations_entropy_and_signal_round_trip() {
        use std::os::unix::ffi::OsStrExt;
        let dir = std::env::temp_dir().join(format!("cinch_platform_mut_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch dir");
        // The intrinsics take `Bytes` words (boxed buffers), the shape
        // `cinch_text_sha256` uses, not word-buffer handles.
        let bytes_word = |s: &str| -> i64 {
            crate::abi::box_bytes(crate::bytes::Bytes::from(s.as_bytes().to_vec()))
        };
        let j = |p: &std::path::Path| p.to_string_lossy().into_owned();
        // mkdir, then the directory exists; a second non-recursive mkdir
        // on it is `AlreadyExists`.
        let one = dir.join("one");
        assert_eq!(unsafe { cinch_fs_mkdir(bytes_word(&j(&one)), 0) }, 0);
        assert!(one.is_dir());
        assert_eq!(unsafe { cinch_fs_mkdir(bytes_word(&j(&one)), 0) }, -ALREADY_EXISTS);
        // `mkdir -p` builds the whole chain and is idempotent.
        let deep = dir.join("a/b/c");
        assert_eq!(unsafe { cinch_fs_mkdir(bytes_word(&j(&deep)), 1) }, 0);
        assert!(deep.is_dir());
        assert_eq!(unsafe { cinch_fs_mkdir(bytes_word(&j(&deep)), 1) }, 0);
        // Remove a file.
        let file = dir.join("a/b/c/f.txt");
        std::fs::write(&file, b"x").expect("write");
        assert_eq!(unsafe { cinch_fs_remove(bytes_word(&j(&file)), 0) }, 0);
        assert!(!file.exists());
        // Remove a subtree.
        let sub = dir.join("a/b");
        assert_eq!(unsafe { cinch_fs_remove(bytes_word(&j(&sub)), 1) }, 0);
        assert!(!sub.exists());
        // A missing path is `NotFound`.
        assert_eq!(unsafe { cinch_fs_remove(bytes_word(&j(&dir.join("nope"))), 0) }, -NOT_FOUND);
        // A symlink removes the link, never its target.
        let target = dir.join("target.txt");
        std::fs::write(&target, b"kept").expect("write target");
        let link = dir.join("link.txt");
        std::os::unix::fs::symlink(&target, &link).expect("symlink");
        assert_eq!(unsafe { cinch_fs_remove(bytes_word(&j(&link)), 0) }, 0);
        assert!(!link.exists());
        assert!(target.exists(), "the symlink target survives");
        // The temp directory is a non-empty existing directory path.
        let td = cinch_fs_temp_dir();
        assert!(td > 0);
        let n = unsafe { crate::abi::cinch_bytes_len(td) };
        assert!(n > 0);
        let mut path = Vec::new();
        for i in 0..n {
            path.push(unsafe { crate::abi::cinch_bytes_get(td, i) } as u8);
        }
        assert!(std::path::Path::new(std::ffi::OsStr::from_bytes(&path)).is_dir());
        // Entropy: the requested length, and two draws differ.
        let r1 = cinch_random_bytes(16);
        let r2 = cinch_random_bytes(16);
        assert!(r1 > 0 && r2 > 0);
        assert_eq!(unsafe { crate::abi::cinch_bytes_len(r1) }, 16);
        let differs = (0..16).any(|i| unsafe { crate::abi::cinch_bytes_get(r1, i) }
            != unsafe { crate::abi::cinch_bytes_get(r2, i) });
        assert!(differs, "two 16-byte entropy reads are equal");
        assert!(cinch_random_bytes(-1) < 0);
        // Signal 0 to this process: the existence check succeeds.
        assert_eq!(cinch_signal_send(i64::from(std::process::id()), 0), 0);
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

    /// The command-line capture: the entry glue's call copies the words,
    /// the count includes the program name, and each word reads back
    /// (empty words and words with a space included).
    #[test]
    fn argv_capture_round_trips() {
        let args: [&[u8]; 4] = [b"/usr/local/bin/tool", b"a b", b"-x", b""];
        // The runtime reads each word to its NUL, so the probe's buffers
        // carry one.
        let owned: Vec<Vec<u8>> = args
            .iter()
            .map(|a| {
                let mut v = a.to_vec();
                v.push(0);
                v
            })
            .collect();
        let ptrs: Vec<*const u8> = owned.iter().map(|w| w.as_ptr()).collect();
        assert_eq!(unsafe { cinch_argv_init(ptrs.len() as i64, ptrs.as_ptr()) }, 0);
        assert_eq!(cinch_argv_count(), 4);
        for (i, want) in args.iter().enumerate() {
            let word = cinch_argv_get(i as i64);
            assert_eq!(
                unsafe { crate::abi::as_bytes(word) }.as_slice(),
                *want,
                "word {i}"
            );
        }
        // A null `argv` and a repeated call leave the capture as it is.
        assert_eq!(unsafe { cinch_argv_init(0, std::ptr::null()) }, 0);
        assert_eq!(cinch_argv_count(), 4);
        // An index outside the capture takes the defined out-of-bounds
        // fault, which exits the process, so it is not exercised here:
        // the compiled-program test (`compiler/tests/argv.rs`) covers
        // the words a real command line delivers.
    }
}

/// The piped-process ABI's own behavior (the standing export rule is
/// the `nm` list in the module above, which names these six externs
/// beside the one-shot ones).
#[cfg(test)]
mod pipe_tests {
    use super::*;

    /// One `Bytes` word holding `s`'s bytes.
    fn bytes_of(s: &str) -> i64 {
        crate::abi::box_bytes(crate::bytes::Bytes::from(s.as_bytes().to_vec()))
    }

    /// A `Bytes` word's content as text.
    fn text_of(word: i64) -> String {
        String::from_utf8(unsafe { crate::abi::as_bytes(word) }.as_slice().to_vec())
            .expect("utf-8")
    }

    /// One output pipe's content, read to its end (the loop is what
    /// makes the probe independent of how the child's writes were
    /// chunked).
    fn read_all(h: i64, which: i64) -> String {
        let mut out = String::new();
        loop {
            let b = unsafe { cinch_process_read(h, which, 4096) };
            assert!(b > 0, "read {which}: {b}");
            let chunk = text_of(b);
            if chunk.is_empty() {
                return out;
            }
            out.push_str(&chunk);
        }
    }

    /// A live child behind three separate pipes: the environment
    /// addition and the working directory arrive (the child writes a
    /// file beside its cwd), stdin feeds it, stdout and stderr are read
    /// apart and each reaches its end as an empty read, the exit code
    /// comes through a repeated wait, and a closed handle is the named
    /// error.
    #[test]
    fn piped_child_streams_round_trip() {
        let dir = std::env::temp_dir().join(format!("cinch_pipe_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let h = unsafe {
            cinch_process_spawn(
                bytes_of("/bin/sh"),
                bytes_of("-c\0echo \"$PROBE\"; cat; echo err >&2; echo here > cwd-check.txt"),
                bytes_of(&dir.display().to_string()),
                bytes_of("PROBE=visible"),
            )
        };
        assert!(h > 0, "spawn: {h}");
        assert_eq!(unsafe { cinch_process_write_stdin(h, bytes_of("payload\n")) }, 8);
        assert_eq!(cinch_process_close_stdin(h), 0);
        // Closing twice keeps the caller's intent (a closed stdin).
        assert_eq!(cinch_process_close_stdin(h), 0);
        assert_eq!(read_all(h, 0), "visible\npayload\n");
        assert_eq!(read_all(h, 1), "err\n");
        // Both pipes are at their end, which is an empty read, not an
        // error.
        let eof = unsafe { cinch_process_read(h, 0, 4096) };
        assert!(eof > 0, "eof read: {eof}");
        assert_eq!(unsafe { crate::abi::cinch_bytes_len(eof) }, 0);
        assert_eq!(unsafe { cinch_process_wait(h) }, 0);
        assert_eq!(unsafe { cinch_process_wait(h) }, 0);
        assert_eq!(
            std::fs::read_to_string(dir.join("cwd-check.txt")).expect("the child's file"),
            "here\n"
        );
        assert_eq!(cinch_process_close(h), 0);
        assert_eq!(unsafe { cinch_process_read(h, 1, 4096) }, -INVALID_ARGUMENT);
        assert_eq!(cinch_process_close(h), -INVALID_ARGUMENT);
        // A program that does not exist is the named error, not a
        // panic.
        assert_eq!(
            unsafe {
                cinch_process_spawn(
                    bytes_of("/nonexistent/cinch-program"),
                    bytes_of(""),
                    bytes_of(""),
                    bytes_of(""),
                )
            },
            -NOT_FOUND
        );
        // An argument list the encoding cannot carry (an empty word
        // between separators) is refused.
        assert_eq!(
            unsafe {
                cinch_process_spawn(
                    bytes_of("/bin/sh"),
                    bytes_of("a\0\0b"),
                    bytes_of(""),
                    bytes_of(""),
                )
            },
            -INVALID_ARGUMENT
        );
        // An environment entry with no separator is refused.
        assert_eq!(
            unsafe {
                cinch_process_spawn(
                    bytes_of("/bin/sh"),
                    bytes_of(""),
                    bytes_of(""),
                    bytes_of("NOVALUE"),
                )
            },
            -INVALID_ARGUMENT
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A killed child's wait reports the raw signal case (128 + the
    /// signal number, the shell convention), and a `which` outside the
    /// two output pipes is the named argument error.
    #[test]
    fn piped_child_signal_death_and_bad_selector() {
        let h = unsafe {
            cinch_process_spawn(bytes_of("/bin/cat"), bytes_of(""), bytes_of(""), bytes_of(""))
        };
        assert!(h > 0, "spawn: {h}");
        assert_eq!(unsafe { cinch_process_read(h, 2, 16) }, -INVALID_ARGUMENT);
        let pid = piped_child_pid(h).expect("the live child's pid");
        assert_eq!(cinch_signal_send(i64::from(pid), 9), 0);
        assert_eq!(unsafe { cinch_process_wait(h) }, 137);
        assert_eq!(cinch_process_close(h), 0);
    }
}

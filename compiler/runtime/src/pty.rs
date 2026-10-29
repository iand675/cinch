//! Pseudo-terminal runtime support (`Effect.Pty`, the PTY shape of
//! `design/stdlib-parity-ruby.md`'s `Open3`/`PTY` entries, the refusal
//! D367 the piped-child ABI left open): the `cinch_pty_*` externs over a
//! POSIX pseudo-terminal pair plus one child, no `forkpty` and no
//! handshake.
//!
//! # Boundary convention
//!
//! The platform layer's (`crate::platform`): text and payload arguments
//! (`Bytes` words) cross through `crate::abi` (`as_bytes` in,
//! `box_bytes` out), handles, ports and counts as machine scalars, and a
//! NEGATIVE result is the negated error code of the shared named
//! vocabulary (a handle that was closed or never existed, and a
//! malformed argument list or window size, are `INVALID_ARGUMENT`; a
//! failed allocation, spawn or system call is the mapped I/O code). The
//! codes are read from `crate::platform` rather than restated, so one
//! vocabulary spans every platform extern.
//!
//! # The handle table
//!
//! A live pseudo-terminal lives in a `Mutex`-guarded map keyed by a
//! counter value handed out once and never reused (the ownership style
//! of `crate::net` and the piped section of `crate::platform`), so a
//! handle the caller invented, or one this module already closed, is
//! always `INVALID_ARGUMENT` rather than somebody else's terminal. One
//! entry holds the pair of descriptors the allocation produced, the
//! child spawned over the slave (if any) and the reaped status: the
//! master is the caller's read/write/control end, and the slave is held
//! only until the child has it, then dropped in the parent so the master
//! observes the end of the stream when the last child end goes away. The
//! lock is held only for the table lookup: a blocking read or write
//! works on a `try_clone`d dup of the master, so one terminal's blocking
//! call never stalls another's.
//!
//! # Why the POSIX calls, not `forkpty`
//!
//! The POSIX pair calls only allocate the descriptors: `posix_openpt`
//! hands back the master, `grantpt` and `unlockpt` prepare the slave,
//! `ptsname_r` names it, and opening that path is the slave. The child
//! is then an ordinary `std::process::Command` whose stdin, stdout and
//! stderr are the slave (`Stdio::from(OwnedFd)`), so the spawn path is
//! the piped child's with a terminal on the far end instead of three
//! separate pipes. `forkpty` would fuse allocation and fork into one C
//! call and take the descriptor plumbing out of the language runtime's
//! hands; this route keeps the two separable and needs no handshake.
//!
//! # Library requirement
//!
//! Every call the pair needs (`posix_openpt`, `grantpt`, `unlockpt`,
//! `ptsname_r` and `open`) is in the C library on both platforms:
//! libSystem on Darwin and the C library itself on Linux, whichever
//! glibc generation provides. No extra library is required on either
//! platform, and no library of the `libutil` kind is needed here, so
//! the extern block carries no link attribute and the driver passes no
//! extra link flag.
//!
//! # Named refusals (boundaries this module does not cross)
//!
//! * no terminal emulation beyond the pseudo-terminal itself: this
//!   module moves bytes over the master and never interprets an escape
//!   sequence or keeps a screen buffer.
//! * no job control: the terminal's line discipline is the platform's;
//!   this module never touches process groups, foreground/background or
//!   `SIGTTOU`/`SIGTTIN`.
//! * no session or detach manipulation: no `setsid`, no `TIOCNOTTY`, no
//!   controlling-terminal adoption the spawn does not already carry.
//! * no Windows: the POSIX pseudo-terminal calls are Unix-only, so this
//!   module is Unix-only (`Effect.Pty`'s header names the refusal too).
//! * a read's end of stream: a platform read of a terminal whose last
//!   peer has gone may answer the raw error (EIO on Linux) rather than an
//!   empty buffer; the caller treats a closed handle as the named error
//!   and an ended child as what its wait reported.
//!
//! The exports are `#[unsafe(no_mangle)]` `cinch_*` externs verified
//! with `nm` by the tests below (the standing runtime rule).

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{LazyLock, Mutex};

use crate::platform::{exit_code, io_code, INVALID_ARGUMENT};

// The POSIX pseudo-terminal calls and the control call this module
// needs. Each of them is in the C library on both platforms (the module
// header's library note), so the block carries no link attribute;
// `ioctl` carries the `TIOCSWINSZ` window-size request.
unsafe extern "C" {
    // Allocate the master end of a pseudo-terminal pair (a descriptor,
    // or -1 with `errno` set) and leave the pair's slave locked.
    fn posix_openpt(flags: i32) -> RawFd;
    // Give the pair's slave to this user with the mode a terminal needs.
    fn grantpt(master: RawFd) -> i32;
    // Clear the slave's lock, so the path below can be opened.
    fn unlockpt(master: RawFd) -> i32;
    // Write the slave's device path into `buf` (`buf_len` counts the
    // NUL): 0, or the error number / -1 with `errno` set per platform.
    fn ptsname_r(master: RawFd, buf: *mut u8, buf_len: usize) -> i32;
    // Open a device node by path (`flags`, and a mode only with
    // `O_CREAT`): a descriptor, or -1 with `errno` set.
    fn open(path: *const std::ffi::c_char, flags: i32, ...) -> RawFd;
    // The terminal control call (`TIOCSWINSZ`).
    fn ioctl(fd: RawFd, request: u64, ...) -> i32;
}

/// The open flags both ends of the pair are opened with: read/write,
/// no controlling terminal (the pair is not this process's own).
const O_RDWR: i32 = 2;
/// `O_NOCTTY` per platform: the value is fixed by the ABI, never
/// derived (`0o400` on Linux, `0x20000` on Darwin).
#[cfg(target_os = "linux")]
const O_NOCTTY: i32 = 0o400;
#[cfg(not(target_os = "linux"))]
const O_NOCTTY: i32 = 0x2_0000;

/// `ERANGE`, what `ptsname_r` answers a buffer too small to hold the
/// slave's path: 34 on both platforms, fixed by the ABI, never derived.
const ERANGE: i32 = 34;

/// `TIOCSWINSZ` per platform: both are `_IOW('t', 103, struct winsize)`
/// and the value is fixed by the ABI, never derived.
#[cfg(target_os = "linux")]
const TIOCSWINSZ: u64 = 0x5414;
#[cfg(not(target_os = "linux"))]
const TIOCSWINSZ: u64 = 0x8008_7467;

/// The `winsize` shape `TIOCSWINSZ` reads (`struct winsize` on both
/// platforms: four `unsigned short`).
#[repr(C)]
#[derive(Clone, Copy)]
struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

/// A live pseudo-terminal: the master descriptor, the slave until the
/// child has it, the child and its reaped status.
struct PtyEntry {
    master: OwnedFd,
    slave: Option<OwnedFd>,
    child: Option<std::process::Child>,
    status: Option<i64>,
}

/// The live pseudo-terminals, by handle word.
static PTYS: LazyLock<Mutex<BTreeMap<i64, PtyEntry>>> =
    LazyLock::new(|| Mutex::new(BTreeMap::new()));

/// The next handle word. Handles are handed out once and never reused,
/// so a stale handle is always invalid rather than live again.
static NEXT_HANDLE: AtomicI64 = AtomicI64::new(1);

/// The handle table (a poisoned lock is recovered: the table's
/// invariant is per-entry, never per-call).
fn table() -> std::sync::MutexGuard<'static, BTreeMap<i64, PtyEntry>> {
    PTYS.lock().unwrap_or_else(|e| e.into_inner())
}

/// A dup of a live handle's master, or `None` when the handle is not
/// live. The dup is what the caller reads and writes through, so the
/// table lock is released before any blocking call.
fn master_dup(h: i64) -> Option<OwnedFd> {
    table().get(&h).and_then(|e| e.master.try_clone().ok())
}

/// The slave's device path for a master, from `ptsname_r`: the slave's
/// name is bounded (`/dev/pts/NNN` on Linux, `/dev/ttysNNN` on Darwin),
/// so a fixed generous buffer holds it, and the second attempt covers
/// the `ERANGE` answer a short buffer would give.
fn slave_path(master: RawFd) -> std::io::Result<std::ffi::CString> {
    let mut buf = vec![0u8; 128];
    loop {
        let r = unsafe { ptsname_r(master, buf.as_mut_ptr(), buf.len()) };
        if r == 0 {
            let len = buf.iter().position(|b| *b == 0).unwrap_or(buf.len());
            return std::ffi::CString::new(&buf[..len]).map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "the slave's path carries a NUL before its end",
                )
            });
        }
        // glibc answers the error number itself; Darwin answers -1 and
        // sets `errno`. Both are read here, and `ERANGE` retries once
        // at a larger size before the failure is reported.
        let err = if r > 0 {
            r
        } else {
            std::io::Error::last_os_error().raw_os_error().unwrap_or(r)
        };
        if err == ERANGE && buf.len() < 1024 {
            buf.resize(buf.len() * 4, 0);
            continue;
        }
        return Err(std::io::Error::from_raw_os_error(err));
    }
}

/// Open a pseudo-terminal pair: a handle word whose entry holds the
/// master and (until a spawn consumes it) the slave, or the negated
/// error code. The master is what `posix_openpt` allocated, and the
/// slave is the device node `ptsname_r` named once `grantpt` and
/// `unlockpt` had prepared the pair; each end is closed on the way out
/// if a step fails or the table insert cannot happen.
///
/// Neither end carries close-on-exec, the inheritance the platform's
/// own pair allocation left, which is the state the
/// `Stdio::from(OwnedFd)` handoff in `cinch_pty_spawn` works from.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_pty_open() -> i64 {
    // The master end. `O_NOCTTY` keeps the pair from becoming this
    // process's controlling terminal. The descriptor is taken into an
    // `OwnedFd` at once, so every failure below closes it.
    let master = unsafe { posix_openpt(O_RDWR | O_NOCTTY) };
    if master < 0 {
        return -io_code(&std::io::Error::last_os_error());
    }
    let master = unsafe { OwnedFd::from_raw_fd(master) };
    // `grantpt` gives the slave to this user with a terminal's mode,
    // and `unlockpt` clears its lock so the path can be opened.
    if unsafe { grantpt(master.as_raw_fd()) } != 0 {
        return -io_code(&std::io::Error::last_os_error());
    }
    if unsafe { unlockpt(master.as_raw_fd()) } != 0 {
        return -io_code(&std::io::Error::last_os_error());
    }
    let path = match slave_path(master.as_raw_fd()) {
        Ok(path) => path,
        Err(e) => return -io_code(&e),
    };
    // The slave end, opened the way the pair's own allocation opens it:
    // the path's device node, read/write, no controlling terminal.
    let slave = unsafe { open(path.as_ptr(), O_RDWR | O_NOCTTY) };
    if slave < 0 {
        return -io_code(&std::io::Error::last_os_error());
    }
    let slave = unsafe { OwnedFd::from_raw_fd(slave) };
    let h = NEXT_HANDLE.fetch_add(1, Ordering::SeqCst);
    table().insert(
        h,
        PtyEntry {
            master,
            slave: Some(slave),
            child: None,
            status: None,
        },
    );
    h
}

/// Spawn `program` (a `Bytes` word of its path, raw OS bytes) as a child
/// of the pseudo-terminal `h`, with `args` (a `Bytes` word of
/// NUL-separated argument words, empty for none), running in the
/// directory `cwd` (a `Bytes` path; empty inherits this process's own
/// directory) and the environment additions `env` (a `Bytes` word of
/// NUL-separated `KEY=VALUE` entries; empty inherits): the handle word,
/// or the negated error code (`NotFound` for a program that does not
/// exist, `InvalidArgument` for an argument list or environment the
/// encoding cannot carry or a handle that is not live, and a handle
/// whose slave was already consumed by an earlier spawn).
///
/// The child's stdin, stdout and stderr are all the slave end, so it is
/// a terminal session: the terminal's line discipline echoes what the
/// caller writes and turns `\n` into `\r\n` on the way out. The parent
/// drops its own slave end once the child holds it, so the master sees
/// the end of the stream when the child is gone.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_pty_spawn(
    h: i64,
    program: i64,
    args: i64,
    cwd: i64,
    env: i64,
) -> i64 {
    use std::os::unix::ffi::OsStringExt;
    let program = unsafe { crate::abi::as_bytes(program) }.as_slice();
    if program.is_empty() {
        return -INVALID_ARGUMENT;
    }
    let args = match unsafe { crate::platform::nul_words(args) } {
        Ok(w) => w,
        Err(code) => return code,
    };
    let env = match unsafe { crate::platform::env_words(env) } {
        Ok(w) => w,
        Err(code) => return code,
    };
    let cwd = unsafe { crate::abi::as_bytes(cwd) }.as_slice();
    // Take the slave out of the table: a second spawn on the same handle
    // has nothing to hand the child, and the descriptor must not stay in
    // the parent past the handoff anyway.
    let Some(slave) = table().get_mut(&h).and_then(|e| e.slave.take()) else {
        return -INVALID_ARGUMENT;
    };
    let Ok(stdin) = slave.try_clone() else {
        return -io_code(&std::io::Error::last_os_error());
    };
    let Ok(stdout) = slave.try_clone() else {
        return -io_code(&std::io::Error::last_os_error());
    };
    let mut cmd = std::process::Command::new(std::ffi::OsString::from_vec(program.to_vec()));
    cmd.args(args);
    if !cwd.is_empty() {
        cmd.current_dir(std::path::PathBuf::from(std::ffi::OsString::from_vec(cwd.to_vec())));
    }
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.stdin(std::process::Stdio::from(stdin));
    cmd.stdout(std::process::Stdio::from(stdout));
    cmd.stderr(std::process::Stdio::from(slave));
    match cmd.spawn() {
        Ok(mut child) => {
            let mut t = table();
            if let Some(e) = t.get_mut(&h) {
                e.child = Some(child);
            } else {
                // The handle was closed underneath the spawn: the child
                // would be unreachable, so reap it now.
                let _ = child.kill();
                let _ = child.wait();
                return -INVALID_ARGUMENT;
            }
            h
        }
        Err(e) => -io_code(&e),
    }
}

/// Read the next bytes of the terminal master into a fresh `Bytes` word:
/// what one platform call produced (at most `max` bytes), or the negated
/// error code for a handle that is not live or a stream failure. `max`
/// is the allocation and the read cap, so a `max` outside `1..=i64::MAX`
/// is `INVALID_ARGUMENT` and the caller's own buffer size bounds the
/// allocation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_pty_read(h: i64, max: i64) -> i64 {
    let Ok(max) = usize::try_from(max) else {
        return -INVALID_ARGUMENT;
    };
    if max == 0 {
        return -INVALID_ARGUMENT;
    }
    let Some(fd) = master_dup(h) else {
        return -INVALID_ARGUMENT;
    };
    let mut f = std::fs::File::from(fd);
    let mut buf = vec![0u8; max];
    match f.read(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            crate::abi::box_bytes(crate::bytes::Bytes::from(buf))
        }
        Err(e) => -io_code(&e),
    }
}

/// Write a `Bytes` word's content to the terminal master: the byte count
/// (the whole buffer is written, and the call blocks until it is), or
/// the negated error code for a handle that is not live or a stream
/// failure. What the master writes is what the child's terminal reads on
/// its stdin.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_pty_write(h: i64, data: i64) -> i64 {
    let data = unsafe { crate::abi::as_bytes(data) }.as_slice();
    let Some(fd) = master_dup(h) else {
        return -INVALID_ARGUMENT;
    };
    let mut f = std::fs::File::from(fd);
    match f.write_all(data) {
        Ok(()) => i64::try_from(data.len()).unwrap_or(i64::MAX),
        Err(e) => -io_code(&e),
    }
}

/// Wait for the terminal's child and report its exit code (0..=255; a
/// signal death reports 128 + the signal number, the shell convention,
/// through `crate::platform::exit_code`): the code is remembered, so a
/// second wait answers the same one, and a handle that is not live or
/// never spawned is `InvalidArgument`. The wait blocks, so a caller
/// drains the output it needs before it waits.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_pty_wait(h: i64) -> i64 {
    let mut t = table();
    let Some(e) = t.get_mut(&h) else {
        return -INVALID_ARGUMENT;
    };
    if let Some(code) = e.status {
        return code;
    }
    let Some(child) = e.child.as_mut() else {
        return -INVALID_ARGUMENT;
    };
    match child.wait() {
        Ok(status) => {
            let code = exit_code(&status);
            e.status = Some(code);
            code
        }
        Err(err) => -io_code(&err),
    }
}

/// Resize the terminal to `cols` columns by `rows` rows (the
/// `TIOCSWINSZ` request): 0, or the negated error code for a handle that
/// is not live, a size outside the `u16` range, or a control-call
/// failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_pty_resize(h: i64, cols: i64, rows: i64) -> i64 {
    let (Ok(cols), Ok(rows)) = (u16::try_from(cols), u16::try_from(rows)) else {
        return -INVALID_ARGUMENT;
    };
    let t = table();
    let Some(e) = t.get(&h) else {
        return -INVALID_ARGUMENT;
    };
    let ws = Winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(e.master.as_raw_fd(), TIOCSWINSZ, &ws as *const Winsize) };
    if r == 0 {
        0
    } else {
        -io_code(&std::io::Error::last_os_error())
    }
}

/// Close a terminal: the handle's entry goes away, and a child no wait
/// has reaped is killed and reaped first (no zombies left behind).
/// Returns 0, or the negated `InvalidArgument` code for a handle that is
/// not live. Closing twice is the second case, an error rather than a
/// no-op: the handle word is gone. Dropping the master also closes the
/// caller's read/write end.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_pty_close(h: i64) -> i64 {
    match table().remove(&h) {
        Some(mut e) => {
            if e.status.is_none() {
                if let Some(child) = e.child.as_mut() {
                    let _ = child.kill();
                    let _ = child.wait();
                }
            }
            // `master`, `slave` and `child` drop here.
            0
        }
        None => -INVALID_ARGUMENT,
    }
}

#[cfg(test)]
mod tests {
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

    /// The standing runtime rule: every `cinch_*` export is a global text
    /// symbol in the built runtime artifact (`nm`-verified, as in
    /// `crate::text`).
    #[test]
    fn pty_exports_are_global_text_symbols() {
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
            "cinch_pty_open",
            "cinch_pty_spawn",
            "cinch_pty_read",
            "cinch_pty_write",
            "cinch_pty_wait",
            "cinch_pty_close",
            "cinch_pty_resize",
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
                    // _name` (Mach-O); `U` marks an undefined reference,
                    // which is not an export.
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
            "the cinch_pty exports are not global text symbols in any runtime artifact ({candidates:?})"
        );
    }

    /// A real terminal round trip: open a pair, spawn `/bin/sh` on the
    /// slave, write a command line, read the echoed line and the
    /// command output, resize the window, then ask the shell to exit,
    /// wait, and get the named error for the closed handle.
    #[test]
    fn pty_round_trip_exit_status_and_named_errors() {
        let h = unsafe { cinch_pty_open() };
        assert!(h > 0, "open: {h}");
        // A terminal with no child yet: read and write are the named
        // error for a handle with no slave child? No: the master is
        // live, so a read would block. Sizing is safe.
        assert_eq!(unsafe { cinch_pty_resize(h, 80, 24) }, 0);
        let r = unsafe {
            cinch_pty_spawn(
                h,
                bytes_of("/bin/sh"),
                bytes_of(""),
                bytes_of(""),
                bytes_of(""),
            )
        };
        assert_eq!(r, h, "spawn: {r}");
        assert_eq!(unsafe { cinch_pty_write(h, bytes_of("echo hi\n")) }, 8);
        // The terminal echoes the input and the command prints `hi`; a
        // read returns whatever one platform call produced, so
        // accumulate until the substring is present (the echo
        // interleaves, so an exact buffer would be brittle).
        let mut seen = String::new();
        for _ in 0..64 {
            if seen.contains("hi") {
                break;
            }
            let b = unsafe { cinch_pty_read(h, 1024) };
            assert!(b > 0, "read: {b}");
            seen.push_str(&text_of(b));
        }
        assert!(seen.contains("hi"), "transcript: {seen:?}");
        assert_eq!(unsafe { cinch_pty_write(h, bytes_of("exit\n")) }, 5);
        assert_eq!(unsafe { cinch_pty_wait(h) }, 0);
        assert_eq!(cinch_pty_close(h), 0);
        // A closed handle is the named error, on a read and on a second
        // close alike.
        assert_eq!(unsafe { cinch_pty_read(h, 16) }, -INVALID_ARGUMENT);
        assert_eq!(unsafe { cinch_pty_write(h, bytes_of("x")) }, -INVALID_ARGUMENT);
        assert_eq!(cinch_pty_close(h), -INVALID_ARGUMENT);
        assert_eq!(unsafe { cinch_pty_read(4096, 16) }, -INVALID_ARGUMENT);
        assert_eq!(unsafe { cinch_pty_wait(4096) }, -INVALID_ARGUMENT);
        // An out-of-range window size is the named argument error.
        assert_eq!(unsafe { cinch_pty_resize(h, 70000, 24) }, -INVALID_ARGUMENT);
        // A program that does not exist is the named error, not a panic.
        let h2 = unsafe { cinch_pty_open() };
        assert!(h2 > 0, "open2: {h2}");
        let bad = unsafe {
            cinch_pty_spawn(
                h2,
                bytes_of("/nonexistent/cinch-program"),
                bytes_of(""),
                bytes_of(""),
                bytes_of(""),
            )
        };
        assert!(bad < 0, "spawn of a missing program: {bad}");
        assert_eq!(cinch_pty_close(h2), 0);
    }
}

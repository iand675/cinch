//! Networking runtime support (P40's networking family): the
//! `cinch_net_*` externs behind the prelude's socket and name-resolution
//! homes (`design/stdlib-parity-ruby.md`'s `Socket`/`Resolv`/`Net::HTTP`
//! entries; the transports P32's OTel export needs).
//!
//! # Boundary convention
//!
//! The platform layer's (`crate::platform`): a `Bytes` argument and a
//! `Bytes` result cross as their word (read through `crate::abi::as_bytes`,
//! freshly boxed through `crate::abi::box_bytes`), ports, handles and
//! byte counts as machine scalars, and a NEGATIVE result is the negated
//! error code of the shared named vocabulary (a refused connection or a
//! broken stream is `IO_ERROR`; a handle that was closed or never
//! existed, and a malformed host or port, are `INVALID_ARGUMENT`).
//! Because platform.rs's codes are the shared vocabulary, this module
//! reads them from there rather than restating them.
//!
//! # The handle table
//!
//! Live sockets live in a `Mutex`-guarded map keyed by a counter value
//! handed out once and never reused (the ownership style of
//! `crate::task`'s scope records), so a handle the caller invented, or
//! one this module already closed, is always `INVALID_ARGUMENT` rather
//! than somebody else's socket. The lock is held only for the lookup: a
//! blocking read or write works on a `try_clone`d dup of the stream, so
//! one socket's blocking call never stalls another's. Closing drops the
//! table's copy, so a dup that is still reading observes the end of the
//! stream (`read` returns 0) rather than an error.
//!
//! # The resolved-address format
//!
//! `cinch_net_resolve` returns a fresh `Bytes` holding one line per
//! resolved address, each spelled `<ip>:<port>` and newline terminated:
//! the `std::net::SocketAddr` text (`127.0.0.1:8080`, and the bracketed
//! `[::1]:8080` for IPv6), UTF-8, in the resolver's order. That is the
//! format the DNS home presents.
//!
//! # Named refusals (boundaries this module does not cross)
//!
//! * plain TCP only: no TLS (a caller that needs it wraps the stream),
//!   no UDP, no listening or accepting (this module is the client side).
//! * no timeouts: connect and read block until the platform call
//!   returns, and neither observes the concurrency cancellation token
//!   (concurrency specification section 4), so a canceled task's socket
//!   call still returns only when the socket does. A cancellation-aware
//!   read needs the poll/timeout shim this slice does not ship.
//! * UTF-8 host names only: a host word that is not UTF-8 text is
//!   `INVALID_ARGUMENT` (the resolver takes text; an arbitrary byte name
//!   is not something the platform resolves).
//! * one read, not a filled length: a read returns what one platform
//!   call produced (at most `max` bytes, and fewer when the kernel says
//!   so), never a loop to fill the requested length.
//! * a write's warning: writing to a socket whose peer has closed its
//!   read side raises `SIGPIPE`, which a program built by this compiler
//!   does not have ignored (the entry point is the generated `main`), so
//!   such a write can terminate the process instead of reporting
//!   `IO_ERROR`. That is the hazard `cinch_proc_stdin`'s pipe write
//!   carries too; it needs the signal discipline this slice does not
//!   ship.
//!
//! The exports are `#[unsafe(no_mangle)]` `cinch_*` externs verified with
//! `nm` by the tests below (the standing runtime rule).

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{LazyLock, Mutex};

use crate::platform::{io_code, INVALID_ARGUMENT};

/// The live sockets, by handle word.
static SOCKETS: LazyLock<Mutex<BTreeMap<i64, TcpStream>>> =
    LazyLock::new(|| Mutex::new(BTreeMap::new()));

/// The next handle word. Handles are handed out once and never reused,
/// so a stale handle is always invalid rather than live again.
static NEXT_HANDLE: AtomicI64 = AtomicI64::new(1);

/// The handle table (a poisoned lock is recovered: the table's
/// invariant is per-entry, never per-call).
fn table() -> std::sync::MutexGuard<'static, BTreeMap<i64, TcpStream>> {
    SOCKETS.lock().unwrap_or_else(|e| e.into_inner())
}

/// Take ownership of a connected stream and answer its handle word.
fn insert(stream: TcpStream) -> i64 {
    let h = NEXT_HANDLE.fetch_add(1, Ordering::SeqCst);
    table().insert(h, stream);
    h
}

/// A dup of a live handle's stream, or `None` when the handle is not
/// live. The dup is what the caller reads and writes through, so the
/// table lock is released before any blocking call.
fn dup(h: i64) -> Option<TcpStream> {
    table().get(&h).and_then(|s| s.try_clone().ok())
}

/// A port word: the `u16` range only.
fn port_of(port: i64) -> Option<u16> {
    u16::try_from(port).ok()
}

/// The UTF-8 host text of a `Bytes` word, or `None` when the word does
/// not hold UTF-8. A word that is not a `Bytes` word is the undefined
/// value fault path, as for every `Bytes` intrinsic.
unsafe fn host_of<'a>(word: i64) -> Option<&'a str> {
    std::str::from_utf8(unsafe { crate::abi::as_bytes(word) }.as_slice()).ok()
}

/// Connect to `host` (a `Bytes` word of UTF-8 host text) at `port`
/// (0..=65535) over TCP: a live socket handle, or the negated error
/// code. The name goes through the platform resolver
/// (`std::net::TcpStream::connect` over `(host, port)`), so a name that
/// does not resolve and a connection the peer refuses are both reported
/// as errors, never a panic and never a hang on a name that cannot
/// resolve.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_net_tcp_connect(host: i64, port: i64) -> i64 {
    let Some(name) = (unsafe { host_of(host) }) else {
        return -INVALID_ARGUMENT;
    };
    let Some(port) = port_of(port) else {
        return -INVALID_ARGUMENT;
    };
    match TcpStream::connect((name, port)) {
        Ok(s) => insert(s),
        Err(e) => -io_code(&e),
    }
}

/// Read the next bytes of a socket into a fresh `Bytes` word: what one
/// platform call produced (at most `max` bytes, empty at the end of the
/// stream), or the negated error code for a handle that is not live or a
/// stream failure. `max` is the allocation and the read cap, so a `max`
/// outside `1..=i64::MAX` is `INVALID_ARGUMENT` and the caller's own
/// buffer size bounds the allocation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_net_read(h: i64, max: i64) -> i64 {
    let Ok(max) = usize::try_from(max) else {
        return -INVALID_ARGUMENT;
    };
    if max == 0 {
        return -INVALID_ARGUMENT;
    }
    let Some(mut s) = dup(h) else {
        return -INVALID_ARGUMENT;
    };
    let mut buf = vec![0u8; max];
    match s.read(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            crate::abi::box_bytes(crate::bytes::Bytes::from(buf))
        }
        Err(e) => -io_code(&e),
    }
}

/// Write a `Bytes` word's content to a socket: the byte count (the whole
/// buffer is written, and the call blocks until it is), or the negated
/// error code for a handle that is not live or a stream failure. The
/// peer-closed `SIGPIPE` hazard is the module header's.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_net_write(h: i64, data: i64) -> i64 {
    let data = unsafe { crate::abi::as_bytes(data) }.as_slice();
    let Some(mut s) = dup(h) else {
        return -INVALID_ARGUMENT;
    };
    match s.write_all(data) {
        Ok(()) => i64::try_from(data.len()).unwrap_or(i64::MAX),
        Err(e) => -io_code(&e),
    }
}

/// Close a socket: 0, or the negated `InvalidArgument` code for a
/// handle that is not live. Closing twice is the second case, an error
/// rather than a no-op: the handle word is gone.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_net_close(h: i64) -> i64 {
    if table().remove(&h).is_some() {
        0
    } else {
        -INVALID_ARGUMENT
    }
}

/// Resolve `host` (a `Bytes` word of UTF-8 host text) at `port`
/// (0..=65535): a fresh `Bytes` holding one `<ip>:<port>` line per
/// resolved address (the module header's format), or the negated error
/// code. A name that resolves to nothing is an empty `Bytes`, not an
/// error: the resolver answered.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_net_resolve(host: i64, port: i64) -> i64 {
    use std::net::ToSocketAddrs;
    let Some(name) = (unsafe { host_of(host) }) else {
        return -INVALID_ARGUMENT;
    };
    let Some(port) = port_of(port) else {
        return -INVALID_ARGUMENT;
    };
    match (name, port).to_socket_addrs() {
        Ok(addrs) => {
            let mut out: Vec<u8> = Vec::new();
            for a in addrs {
                out.extend_from_slice(a.to_string().as_bytes());
                out.push(b'\n');
            }
            crate::abi::box_bytes(crate::bytes::Bytes::from(out))
        }
        Err(e) => -io_code(&e),
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

    /// A listener on a loopback port with a peer thread that echoes one
    /// line and then closes its side (so the client reaches the end of
    /// the stream).
    fn echoing_peer() -> u16 {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let port = listener.local_addr().expect("local addr").port();
        std::thread::spawn(move || {
            let (mut s, _) = listener.accept().expect("accept");
            let mut line = Vec::new();
            let mut b = [0u8; 1];
            while s.read(&mut b).unwrap_or(0) == 1 {
                line.push(b[0]);
                if b[0] == b'\n' {
                    break;
                }
            }
            let mut reply = b"echo:".to_vec();
            reply.extend_from_slice(&line);
            let _ = s.write_all(&reply);
            let _ = s.flush();
            // Dropping the stream closes the peer's side: the client's
            // next read is the end of the stream.
        });
        port
    }

    /// A loopback port with nothing listening on it (bound, then
    /// dropped; the race a repeated bind would lose is acceptable for a
    /// refused-connection probe).
    fn dead_port() -> u16 {
        let l = std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let port = l.local_addr().expect("local addr").port();
        drop(l);
        port
    }

    /// The standing runtime rule: every `cinch_*` export is a global
    /// text symbol in the built runtime artifact (`nm`-verified, as in
    /// `crate::text`).
    #[test]
    fn net_exports_are_global_text_symbols() {
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
            "cinch_net_tcp_connect",
            "cinch_net_read",
            "cinch_net_write",
            "cinch_net_close",
            "cinch_net_resolve",
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
            "the cinch_net exports are not global text symbols in any runtime artifact ({candidates:?})"
        );
    }

    /// A real loopback round trip: connect, write a line, read the
    /// peer's echo back, read the end of the stream as an empty buffer,
    /// and a closed or invented handle as the named error.
    #[test]
    fn socket_round_trip_eof_and_named_errors() {
        let port = echoing_peer();
        let host = bytes_of("127.0.0.1");
        let h = unsafe { cinch_net_tcp_connect(host, i64::from(port)) };
        assert!(h > 0, "connect: {h} (port {port})");
        let line = bytes_of("hello\n");
        assert_eq!(unsafe { cinch_net_write(h, line) }, 6);
        let reply = unsafe { cinch_net_read(h, 1024) };
        assert!(reply > 0, "read: {reply}");
        assert_eq!(text_of(reply), "echo:hello\n");
        // The peer closed its side: the next read is the end of the
        // stream, an empty buffer rather than an error.
        let eof = unsafe { cinch_net_read(h, 1024) };
        assert!(eof > 0, "eof read: {eof}");
        assert_eq!(unsafe { crate::abi::cinch_bytes_len(eof) }, 0);
        assert_eq!(cinch_net_close(h), 0);
        // A closed handle is the named error, on a read and on a second
        // close alike.
        assert_eq!(unsafe { cinch_net_read(h, 1024) }, -INVALID_ARGUMENT);
        assert_eq!(cinch_net_close(h), -INVALID_ARGUMENT);
        assert_eq!(unsafe { cinch_net_read(4096, 1024) }, -INVALID_ARGUMENT);
        // A refused connection is an error, not a hang and not a panic.
        // The probe frees a port before connecting to it, so the OS can
        // re-hand that port to somebody else in between (observed as a
        // rare flake on a loaded machine); each attempt takes a fresh
        // port, and only a port nobody took can answer the refusal.
        let mut dead = 0;
        for _ in 0..8 {
            dead = unsafe { cinch_net_tcp_connect(host, i64::from(dead_port())) };
            if dead < 0 {
                break;
            }
        }
        assert!(dead < 0, "connect to a freed port never reported an error");
        // The resolver answers at least one address line for the local
        // name, in the documented `<ip>:<port>` shape.
        let localhost = bytes_of("localhost");
        let lines = unsafe { cinch_net_resolve(localhost, i64::from(port)) };
        assert!(lines > 0, "resolve: {lines}");
        let text = text_of(lines);
        assert!(text.ends_with('\n'), "resolve did not newline-terminate: {text:?}");
        assert!(
            text.lines().all(|l| l.contains(':') && l.ends_with(&port.to_string())),
            "resolve lines are not <ip>:<port>: {text:?}"
        );
        // A malformed host or port is the named argument error.
        assert_eq!(unsafe { cinch_net_tcp_connect(localhost, 70000) }, -INVALID_ARGUMENT);
        assert_eq!(unsafe { cinch_net_resolve(host, -1) }, -INVALID_ARGUMENT);
    }
}

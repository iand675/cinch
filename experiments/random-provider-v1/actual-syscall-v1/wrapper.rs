//! Standalone qualification of the copied Linux backend and entropy helper.
//!
//! Every draw uses the actual operating-system entropy provider. This
//! executable does not build or qualify the public boxed-Bytes ABI.

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

/// The unchanged syscall layer's Linux invalid-argument error number.
pub(crate) const EINVAL: i64 = 22;

/// Exact copied proposed backend, including its existing inline assembly.
#[allow(dead_code)]
#[path = "../proposed/compiler/runtime/src/sys/linux.rs"]
mod linux_backend;

/// The runtime seam, forwarded to the real copied Linux implementation.
mod sys {
    /// Issue a real blocking kernel entropy draw without substitution.
    pub(crate) fn getrandom(buf: &mut [u8]) -> (i64, i64) {
        crate::linux_backend::getrandom(buf)
    }
}

/// Exact copied proposed exact-fill module, with no provider replacement.
#[path = "../proposed/compiler/runtime/src/entropy.rs"]
mod entropy;

/// Write an identified raw capture record without altering the drawn bytes.
fn capture_record(out: &mut impl Write, bytes: &[u8]) -> io::Result<()> {
    out.write_all(&(bytes.len() as u64).to_be_bytes())?;
    out.write_all(bytes)
}

/// Exercise the backend and exact filler, retaining all actual entropy draws.
fn capture(path: &Path) -> io::Result<()> {
    let mut out = OpenOptions::new().write(true).create_new(true).open(path)?;
    out.write_all(b"CINCH-ENTROPY-CAPTURE-V1\n")?;
    let mut direct = [0; 16];
    let (count, errno) = sys::getrandom(&mut direct);
    assert_eq!((count, errno), (16, 0), "real small backend draw");
    capture_record(&mut out, &direct)?;
    println!("backend bytes=16 count={count} errno={errno}");

    for len in [0, 1, 16, 255, 256, 257, 4096, 1024 * 1024] {
        let mut first = vec![0; len];
        let mut second = vec![0; len];
        entropy::fill(&mut first)?;
        entropy::fill(&mut second)?;
        assert_eq!(first.len(), len);
        assert_eq!(second.len(), len);
        if len >= 16 {
            assert_ne!(first, second, "freshness smoke check at length {len}");
        }
        capture_record(&mut out, &first)?;
        capture_record(&mut out, &second)?;
        println!("exact-fill bytes={len} draws=2 status=ok");
    }
    out.flush()
}

/// Run one explicitly marked draw so an untimed syscall trace is attributable.
fn single(len: usize) -> io::Result<()> {
    let mut bytes = vec![0; len];
    eprintln!("BEGIN ENTROPY DRAW len={len}");
    entropy::fill(&mut bytes)?;
    eprintln!("END ENTROPY DRAW len={len}");
    println!("exact-fill bytes={} status=ok", bytes.len());
    Ok(())
}

/// Select real capture or a single draw; reject accidental output overwrites.
fn main() -> io::Result<()> {
    let mut args = std::env::args_os().skip(1);
    match args.next().as_deref().and_then(|arg| arg.to_str()) {
        Some("capture") => {
            let path = args.next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "capture requires a new output path")
            })?;
            capture(Path::new(&path))
        }
        Some("single16") => single(16),
        Some("single0") => single(0),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "expected capture PATH, single16, or single0",
        )),
    }
}

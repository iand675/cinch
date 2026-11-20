//! Exact filling from the operating system's cryptographic entropy source.
//!
//! Linux uses its blocking `getrandom` interface and retains the device
//! provider only when the kernel reports that interface is unavailable.

use std::io::{self, Read};

/// Fill the whole slice from the operating system or return its failure.
/// Empty requests retain the device-open check of the original provider.
pub(crate) fn fill(buf: &mut [u8]) -> io::Result<()> {
    if buf.is_empty() {
        return device_fill(buf);
    }
    #[cfg(target_os = "linux")]
    {
        fill_with(
            buf,
            |remaining| {
                let (count, errno) = crate::sys::getrandom(remaining);
                if errno != 0 {
                    Err(io::Error::from_raw_os_error(errno as i32))
                } else {
                    usize::try_from(count).map_err(|_| {
                        io::Error::new(io::ErrorKind::InvalidData, "invalid entropy count")
                    })
                }
            },
            device_fill,
        )
    }
    #[cfg(not(target_os = "linux"))]
    {
        device_fill(buf)
    }
}

/// Open the existing cryptographic device and read the requested slice.
/// The descriptor is local to this call and is closed on every exit path.
fn device_fill(buf: &mut [u8]) -> io::Result<()> {
    std::fs::File::open("/dev/urandom")?.read_exact(buf)
}

/// Fill successive suffixes, retrying interruptions without losing progress.
/// Only Linux `ENOSYS` (38) delegates the unfinished suffix to `fallback`;
/// denied access and all other failures remain failures. Injected operations
/// let tests exercise progress and error boundaries without consuming entropy.
#[cfg(any(target_os = "linux", test))]
fn fill_with(
    mut buf: &mut [u8],
    mut draw: impl FnMut(&mut [u8]) -> io::Result<usize>,
    fallback: impl FnOnce(&mut [u8]) -> io::Result<()>,
) -> io::Result<()> {
    while !buf.is_empty() {
        match draw(buf) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "entropy source made no progress",
                ));
            }
            Ok(count) if count <= buf.len() => buf = &mut buf[count..],
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "entropy source exceeded the requested length",
                ));
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) if error.raw_os_error() == Some(38) => return fallback(buf),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Short reads and interruptions preserve the filled prefix and exact suffix.
    #[test]
    fn partial_reads_and_interruptions_preserve_progress() {
        let mut buf = [0; 7];
        let mut call = 0;
        fill_with(
            &mut buf,
            |out| {
                let lengths = [7, 5, 5, 2];
                assert_eq!(out.len(), lengths[call]);
                call += 1;
                if call == 2 {
                    return Err(io::Error::from_raw_os_error(4));
                }
                let count = [2, 0, 3, 2][call - 1];
                out[..count].fill(call as u8);
                Ok(count)
            },
            |_| panic!("fallback must not run"),
        )
        .unwrap();
        assert_eq!(call, 4);
        assert_eq!(buf, [1, 1, 3, 3, 3, 4, 4]);
    }

    /// An unavailable syscall delegates only the unfilled suffix, exactly once.
    #[test]
    fn unavailable_syscall_preserves_prefix() {
        for prefix in [0, 3] {
            let mut buf = [0; 7];
            let mut calls = 0;
            let mut fallbacks = 0;
            fill_with(
                &mut buf,
                |out| {
                    calls += 1;
                    if prefix > 0 && calls == 1 {
                        out[..prefix].fill(5);
                        Ok(prefix)
                    } else {
                        Err(io::Error::from_raw_os_error(38))
                    }
                },
                |out| {
                    fallbacks += 1;
                    assert_eq!(out.len(), 7 - prefix);
                    out.fill(9);
                    Ok(())
                },
            )
            .unwrap();
            assert_eq!(fallbacks, 1);
            assert_eq!(&buf[..prefix], vec![5; prefix]);
            assert_eq!(&buf[prefix..], vec![9; 7 - prefix]);
        }
    }

    /// Access, argument, and device failures propagate without fallback.
    #[test]
    fn ordinary_failures_never_fall_back() {
        for errno in [1, 5, 11, 13, 22] {
            for prefix in [0, 2] {
                let mut buf = [0; 4];
                let mut calls = 0;
                let error = fill_with(
                    &mut buf,
                    |out| {
                        calls += 1;
                        if prefix > 0 && calls == 1 {
                            out[..prefix].fill(6);
                            Ok(prefix)
                        } else {
                            Err(io::Error::from_raw_os_error(errno))
                        }
                    },
                    |_| panic!("fallback must not run"),
                )
                .unwrap_err();
                assert_eq!(error.raw_os_error(), Some(errno));
                assert_eq!(calls, if prefix == 0 { 1 } else { 2 });
                assert_eq!(&buf[..prefix], vec![6; prefix]);
            }
        }
    }

    /// A failed fallback remains an error rather than publishing partial bytes.
    #[test]
    fn fallback_failure_propagates() {
        let error = fill_with(
            &mut [0; 4],
            |_| Err(io::Error::from_raw_os_error(38)),
            |_| Err(io::Error::from_raw_os_error(13)),
        )
        .unwrap_err();
        assert_eq!(error.raw_os_error(), Some(13));
    }

    /// An empty exact-fill loop makes no draws and never invokes fallback.
    #[test]
    fn empty_loop_has_no_calls() {
        fill_with(
            &mut [],
            |_| panic!("draw must not run"),
            |_| panic!("fallback must not run"),
        )
        .unwrap();
    }

    /// Zero and oversized reports fail immediately instead of looping or slicing.
    #[test]
    fn invalid_counts_fail_closed() {
        for (count, kind) in [
            (0, io::ErrorKind::UnexpectedEof),
            (5, io::ErrorKind::InvalidData),
        ] {
            let mut calls = 0;
            let error = fill_with(
                &mut [0; 4],
                |_| {
                    calls += 1;
                    Ok(count)
                },
                |_| panic!("fallback must not run"),
            )
            .unwrap_err();
            assert_eq!(error.kind(), kind);
            assert_eq!(calls, 1);
        }
    }
}

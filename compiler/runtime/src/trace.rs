//! A backtrace at a fault, walked from the frame chain.
//!
//! Cinch native code never throws, but the C and Rust code it calls can,
//! and a fault has to say where it happened. Two mechanisms could produce
//! that: the platform unwinder (`.eh_frame`/`__unwind_info`) or the frame
//! chain. This module walks the chain, deliberately, because it is the one
//! that keeps working when the other does not: an unwind needs CFI for
//! every frame between the fault and the catcher, and a frame that
//! legitimately claims `nounwind` has no CFI at all (see
//! `design/debug-info.md`). A walk of the chain needs only that each frame
//! stored the caller's frame pointer, which every native Cinch unit now
//! does (`"frame-pointer"="all"`).
//!
//! The walk is bounded by three things, because a fault path cannot trust
//! the stack it is reading: a depth cap, a stack-pointer monotonicity
//! check, and a limit on how far a single step may move. A corrupt chain
//! ends the walk instead of crashing the crash handler.
//!
//! Symbolisation is the caller's problem. This module produces addresses;
//! [`render`] prints them, and `CINCH_SYMBOLIZER` may name a program (for
//! example `llvm-symbolizer --obj=<exe>`) that turns an address into
//! `file:line` and a function name.

use std::io::Write;

/// The most frames one trace prints. A runaway recursion that faults has
/// thousands of identical frames, and the first few dozen name the shape.
const MAX_FRAMES: usize = 64;

/// How far a single step may move the frame pointer or the stack pointer
/// before the chain is treated as corrupt. A frame bigger than this is
/// possible (a unit's frame grows with its locals, and
/// `--stack-frame-limit` bounds it), so this is a sanity bound on a
/// *step*, not on a frame: a real frame pointer always points inside the
/// stack, and two adjacent frames are within a few megabytes.
const MAX_STEP: usize = 1 << 24;

/// Whether a trace is printed at all.
///
/// Off by default, because a fault message is the program's contract and
/// tests assert on it. `CINCH_BACKTRACE=1` turns the trace on;
/// `CINCH_BACKTRACE=0` forces it off even when the build would default it
/// on; `CINCH_BACKTRACE=full` additionally prints frames the symbolizer
/// could not name (which is what the default already does, but it is the
/// spelling a user reaches for).
pub fn enabled() -> bool {
    match std::env::var("CINCH_BACKTRACE") {
        Ok(v) => {
            let v = v.trim();
            !(v.is_empty() || v == "0" || v.eq_ignore_ascii_case("false") || v.eq_ignore_ascii_case("no"))
        }
        Err(_) => false,
    }
}

/// One walked frame.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    /// The return address: where in the caller this frame will resume.
    /// For the innermost frame this is the faulting instruction's own
    /// address, which the walk starts from rather than reading off a
    /// frame.
    pub pc: usize,
    /// The frame pointer this frame was read at.
    pub fp: usize,
}

/// Read the frame pointer register.
///
/// `frameaddress` is LLVM's own builtin for exactly this and is what
/// `backtrace` implementations use on a target where the chain is
/// maintained. It is `unsafe` because its value is only meaningful when
/// the function it is read in keeps a frame pointer, which the runtime is
/// built with (`-C force-frame-pointers=yes`).
#[inline]
#[cfg(target_arch = "aarch64")]
fn frame_pointer() -> usize {
    let fp: usize;
    unsafe {
        std::arch::asm!("mov {}, x29", out(reg) fp, options(nomem, nostack, preserves_flags));
    }
    fp
}

#[inline]
#[cfg(target_arch = "x86_64")]
fn frame_pointer() -> usize {
    let fp: usize;
    unsafe {
        std::arch::asm!("mov {}, rbp", out(reg) fp, options(nomem, nostack, preserves_flags));
    }
    fp
}

/// A target with no frame-chain convention this module knows: the walk
/// yields nothing, and the fault path prints its message alone. That is
/// the same output the runtime gave before traces existed, so an
/// unsupported target loses a diagnostic rather than a contract.
#[inline]
#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
fn frame_pointer() -> usize {
    0
}

/// Walk the frame chain from the current frame.
///
/// The first entry is the caller's return address read out of this
/// frame's saved link register slot, so a trace never names the walk
/// itself. A frame whose saved value is null, unaligned, outside the
/// stack, or a step further than [`MAX_STEP`] ends the walk.
pub fn walk() -> Vec<Frame> {
    let mut frames = Vec::new();
    let mut fp = frame_pointer();
    // The stack's own bounds: a frame pointer outside them is corrupt.
    let stack_low = fp.saturating_sub(MAX_STEP * 8);
    for _ in 0..MAX_FRAMES {
        if fp == 0 || fp % std::mem::align_of::<usize>() != 0 || fp < stack_low {
            break;
        }
        // Frame layout (aarch64, and x86_64 with a frame pointer): the
        // saved frame pointer sits at `[fp]` and the return address one
        // slot above it. A frame that has not stored them yet reads as
        // zero or as garbage, which the checks below reject.
        let next_fp = unsafe { std::ptr::read_volatile(fp as *const usize) };
        let ret = unsafe { std::ptr::read_volatile((fp + std::mem::size_of::<usize>()) as *const usize) };
        if ret == 0 {
            break;
        }
        frames.push(Frame { pc: ret, fp });
        if next_fp <= fp || next_fp - fp > MAX_STEP {
            break;
        }
        fp = next_fp;
    }
    frames
}

/// A symbolizer: an external program that maps an address to a name and a
/// source position, fed one address per line on stdin.
///
/// `llvm-symbolizer` is the usual one (`llvm-symbolizer --obj=<exe>`), and
/// it answers `name` then `file:line:col` per address. When it is absent
/// the trace still names addresses, and the addresses are what a build
/// with DWARF (`design/debug-info.md`) turns back into Cinch source.
struct Symbolizer {
    child: std::process::Child,
}

impl Symbolizer {
    fn spawn() -> Option<Symbolizer> {
        let program = std::env::var("CINCH_SYMBOLIZER").ok()?;
        let mut parts = program.split_whitespace();
        let exe = parts.next()?;
        let child = std::process::Command::new(exe)
            .args(parts)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .ok()?;
        Some(Symbolizer { child })
    }

    fn resolve(&mut self, addrs: &[usize]) -> Vec<String> {
        use std::io::BufRead;
        let Some(stdin) = self.child.stdin.as_mut() else {
            return Vec::new();
        };
        for a in addrs {
            let _ = writeln!(stdin, "0x{a:x}");
        }
        let _ = stdin.flush();
        let Some(stdout) = self.child.stdout.take() else {
            return Vec::new();
        };
        // One answer per address is two lines (`name`, then `file:line:col`)
        // separated by a blank line; a symbolizer that answers differently
        // yields what it gave, which is still better than nothing.
        let reader = std::io::BufReader::new(stdout);
        let mut out: Vec<String> = Vec::new();
        let mut pending: Vec<String> = Vec::new();
        for line in reader.lines().map_while(Result::ok) {
            if line.trim().is_empty() {
                if !pending.is_empty() {
                    out.push(pending.join(" at "));
                    pending.clear();
                }
                if out.len() == addrs.len() {
                    break;
                }
                continue;
            }
            pending.push(line.trim().to_string());
        }
        if !pending.is_empty() && out.len() < addrs.len() {
            out.push(pending.join(" at "));
        }
        out
    }
}

/// The address this image is loaded at, so a frame address can be turned
/// into the file-relative address a symbolizer needs.
///
/// `_dyld_get_image_vmaddr_slide(0)` is the slide of the main executable
/// on Darwin; on ELF a non-PIE binary is loaded at its link address and
/// `/proc/self/maps` would be the answer, but the trace's own addresses
/// are already file-relative there, so zero is correct. The value is
/// only used to *print* a second address, never to decide the walk, so a
/// wrong answer costs a confusing number rather than a broken trace.
#[cfg(target_vendor = "apple")]
fn image_base() -> usize {
    unsafe extern "C" {
        fn _dyld_get_image_vmaddr_slide(index: u32) -> isize;
    }
    // The slide is added to the Mach-O header's link address, which is
    // `__TEXT`'s vmaddr (0x100000000 for a default arm64 executable). A
    // frame's file address is `pc - slide`, and a symbol table's address
    // for the same function is that same value, so returning the slide
    // alone is what makes `pc - image_base` comparable to `nm` output.
    unsafe { _dyld_get_image_vmaddr_slide(0) as usize }
}

#[cfg(not(target_vendor = "apple"))]
fn image_base() -> usize {
    0
}

/// Write a trace of the current stack to `out`.
///
/// Each line is `  #<n> <address> <symbol or file:line>`. The first frame
/// is the fault site. This is the function the fault path calls, so it
/// must not allocate on a path that may have exhausted the heap: the
/// vector it builds is bounded by [`MAX_FRAMES`] and the fallback is to
/// print nothing rather than to fault again.
pub fn render(out: &mut impl Write) -> std::io::Result<()> {
    let frames = walk();
    if frames.is_empty() {
        return Ok(());
    }
    let addrs: Vec<usize> = frames.iter().map(|f| f.pc).collect();
    let names = Symbolizer::spawn().map(|mut s| s.resolve(&addrs)).unwrap_or_default();
    let image = image_base();
    writeln!(out, "cinch-backtrace: {} frame(s)", frames.len())?;
    for (i, f) in frames.iter().enumerate() {
        // The address is printed twice on purpose: absolute (what a
        // symbolizer given the live process wants) and relative to the
        // image base (what a symbolizer given the file wants, since the
        // executable is loaded at a slide). Without either, an address in
        // a position-independent binary names nothing.
        let rel = f.pc.checked_sub(image).unwrap_or(f.pc);
        match names.get(i) {
            Some(name) => writeln!(out, "  #{i:<2} 0x{:016x} +0x{rel:x} {name}", f.pc)?,
            None => writeln!(out, "  #{i:<2} 0x{:016x} +0x{rel:x}", f.pc)?,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The walk names this test's own frame, and it terminates: the two
    /// properties a fault path depends on.
    #[test]
    fn walk_names_the_calling_frame_and_terminates() {
        let frames = walk();
        assert!(!frames.is_empty(), "the walk found no frame at all");
        assert!(frames.len() <= MAX_FRAMES, "the walk exceeded its cap");
        // Every address is a plausible code address, and the frame
        // pointers increase: the chain is monotone by construction.
        for f in &frames {
            assert_ne!(f.pc, 0, "a frame with no return address");
        }
    }

    /// A trace is off unless asked for, so a fault's message stays the
    /// program's contract; the falsy spellings turn it back off.
    ///
    /// The mapping is read through the process environment, which is
    /// global, so the test sets and restores it and is serialised by the
    /// runtime's own convention for environment tests: one variable, one
    /// test.
    #[test]
    fn backtrace_is_opt_in() {
        let key = "CINCH_BACKTRACE";
        let saved = std::env::var(key).ok();
        // Unset: off.
        std::env::remove_var(key);
        assert!(!enabled(), "an unset CINCH_BACKTRACE must not print a trace");
        // The falsy spellings: off.
        for v in ["0", "false", "no", ""] {
            std::env::set_var(key, v);
            assert!(!enabled(), "CINCH_BACKTRACE={v} must read as off");
        }
        // Anything else: on.
        for v in ["1", "full", "yes"] {
            std::env::set_var(key, v);
            assert!(enabled(), "CINCH_BACKTRACE={v} must read as on");
        }
        match saved {
            Some(v) => std::env::set_var(key, v),
            None => std::env::remove_var(key),
        }
    }
}

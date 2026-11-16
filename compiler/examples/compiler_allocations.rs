//! Run the ordinary Core CLI with process-wide Rust allocation counters.
//! Counters include startup and the whole compiler call, then stop before
//! printing. Requested bytes are allocation traffic, not live heap size.
//! Foreign C allocations and child processes are outside these counters.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};

struct CountedSystem;
static ACTIVE: AtomicBool = AtomicBool::new(true);
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static REALLOCS: AtomicU64 = AtomicU64::new(0);
static REALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static FREES: AtomicU64 = AtomicU64::new(0);
static FREE_BYTES: AtomicU64 = AtomicU64::new(0);

unsafe impl GlobalAlloc for CountedSystem {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() && ACTIVE.load(Relaxed) {
            ALLOCS.fetch_add(1, Relaxed);
            ALLOC_BYTES.fetch_add(layout.size() as u64, Relaxed);
        }
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc_zeroed(layout) };
        if !p.is_null() && ACTIVE.load(Relaxed) {
            ALLOCS.fetch_add(1, Relaxed);
            ALLOC_BYTES.fetch_add(layout.size() as u64, Relaxed);
        }
        p
    }
    unsafe fn realloc(&self, p: *mut u8, old: Layout, size: usize) -> *mut u8 {
        let next = unsafe { System.realloc(p, old, size) };
        if !next.is_null() && ACTIVE.load(Relaxed) {
            REALLOCS.fetch_add(1, Relaxed);
            REALLOC_BYTES.fetch_add(size as u64, Relaxed);
        }
        next
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        if ACTIVE.load(Relaxed) {
            FREES.fetch_add(1, Relaxed);
            FREE_BYTES.fetch_add(layout.size() as u64, Relaxed);
        }
        unsafe { System.dealloc(p, layout) };
    }
}

#[global_allocator]
static ALLOCATOR: CountedSystem = CountedSystem;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = cinch::driver::core_cli(&args);
    ACTIVE.store(false, Relaxed);
    eprintln!("cinch-rust-alloc: {}", serde_json::json!({
        "allocations": ALLOCS.load(Relaxed), "allocated_bytes": ALLOC_BYTES.load(Relaxed),
        "reallocations": REALLOCS.load(Relaxed), "reallocated_bytes": REALLOC_BYTES.load(Relaxed),
        "deallocations": FREES.load(Relaxed), "deallocated_bytes": FREE_BYTES.load(Relaxed),
        "exit": code,
    }));
    std::process::exit(code);
}

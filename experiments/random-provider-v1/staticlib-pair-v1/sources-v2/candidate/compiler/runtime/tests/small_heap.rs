//! The small-object heap (`compiler/runtime/src/small.rs`) as a program
//! sees it through the allocation ABI: blocks a task's thread allocated
//! and its joiner freed after the thread finished are reused, by the
//! joiner itself and by the next thread to start. One test in a process
//! of its own, so no other test's allocations share the heap's pages.

use std::collections::HashSet;

use cinchruntime::abi::{cinch_alloc_raw, cinch_free};
use cinchruntime::block::KIND_RAW;

const N: usize = 50_000;

/// 128 payload bytes: 136-byte blocks with the header word.
const PAYLOAD: i64 = 128;

fn alloc_many() -> Vec<usize> {
    (0..N)
        .map(|_| unsafe { cinch_alloc_raw(PAYLOAD, i64::from(KIND_RAW)) } as usize)
        .collect()
}

fn free_all(blocks: &[usize]) {
    for &p in blocks {
        unsafe { cinch_free(p as i64) };
    }
}

fn reused(blocks: &[usize], earlier: &HashSet<usize>) -> usize {
    blocks.iter().filter(|p| earlier.contains(p)).count()
}

#[test]
fn blocks_a_finished_thread_left_behind_are_reused() {
    let live = cinchruntime::heap::live_blocks();

    // Freed here after their thread exited; this thread then allocates as
    // much again without starting another thread.
    let theirs = std::thread::spawn(alloc_many).join().unwrap();
    free_all(&theirs);
    let earlier: HashSet<usize> = theirs.into_iter().collect();
    let mine = alloc_many();
    let n = reused(&mine, &earlier);
    assert!(n >= N * 9 / 10, "this thread reused only {n} of {N} blocks freed after their thread exited");
    free_all(&mine);

    // Freed here after their thread exited; the next thread to start
    // allocates as much again.
    let theirs = std::thread::spawn(alloc_many).join().unwrap();
    free_all(&theirs);
    let earlier: HashSet<usize> = theirs.into_iter().collect();
    let n = std::thread::spawn(move || {
        let again = alloc_many();
        let n = reused(&again, &earlier);
        free_all(&again);
        n
    })
    .join()
    .unwrap();
    assert!(n >= N * 9 / 10, "the next thread reused only {n} of {N} blocks freed after their thread exited");

    assert_eq!(cinchruntime::heap::live_blocks(), live, "every block was freed");
}

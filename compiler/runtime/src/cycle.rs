//! Test-safepoint prototype for bounded reclamation of local counted-array cycles.
//!
//! Discovery reads precise maps into shadow metadata without changing any actual
//! reference count. The caller establishes quiescence; this module installs no
//! allocation-time or exit-time collector and is absent from production builds.

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::marker::PhantomData;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::rc::Rc;

use crate::{block, rc};

thread_local! {
    /// Non-owning candidates exist only while a focused test has enabled tracking.
    static TRACKER: RefCell<Option<Tracker>> = const { RefCell::new(None) };
}

/// Local candidate addresses are retired before free or atomic publication.
#[derive(Default)]
struct Tracker {
    /// Initialized counted arrays; membership never contributes a strong reference.
    roots: BTreeSet<usize>,
    /// Last attempted candidate, so small passes do not always select the first root.
    cursor: Option<usize>,
    /// Collection is synchronous and cannot reenter itself through runtime disposal.
    running: bool,
}

/// Work limits apply to the entire pass, including components that are aborted.
#[derive(Clone, Copy)]
pub(crate) struct Limits {
    /// Maximum candidate roots attempted by one pass.
    pub(crate) roots: usize,
    /// Maximum distinct graph nodes discovered across attempted components.
    pub(crate) nodes: usize,
    /// Maximum counted pointer edges recorded across attempted components.
    pub(crate) edges: usize,
    /// Maximum payload words covered by precise scans, including skipped raw slots.
    pub(crate) slots: usize,
}

impl Default for Limits {
    /// Keep metadata and work bounded independently of the program's total heap size.
    fn default() -> Self {
        Self {
            roots: 16,
            nodes: 4096,
            edges: 16384,
            slots: 65536,
        }
    }
}

/// A focused pass reports actual disposal separately from conservative aborts.
#[derive(Default, Debug)]
pub(crate) struct Outcome {
    /// Managed blocks disposed after all their internal edges were detached.
    pub(crate) reclaimed: usize,
    /// Complete candidates from which external owners prevented any reclamation.
    pub(crate) retained: usize,
    /// Incomplete, unsupported or inconsistent components left entirely unchanged.
    pub(crate) aborted: usize,
}

/// Remaining accounting enforces limits across multiple candidate components.
struct Budget {
    /// Nodes still available for discovery.
    nodes: usize,
    /// Pointer edges still available for recording.
    edges: usize,
    /// Payload words still available for precise scanning.
    slots: usize,
}

/// One positive local heap block and its precise outgoing edge range.
struct Node {
    /// Payload address whose lifetime is protected by the quiescent graph.
    address: usize,
    /// Unmodified reference count, kind and layout observed at discovery.
    header: u64,
    /// Incoming edges from nodes in this trial graph, including duplicate slots.
    incoming: usize,
    /// First recorded outgoing pointer edge.
    first: usize,
    /// Exclusive end of the recorded outgoing pointer edges.
    end: usize,
}

/// A counted pointer edge retains its exact payload slot and original value.
struct Edge {
    /// Absolute payload slot index in its source block.
    slot: usize,
    /// Owned reference removed from that slot during reclamation.
    word: i64,
    /// A local graph node, or an opaque atomic/immortal boundary.
    target: Option<usize>,
}

/// Discovery metadata never retains or mutates the actual graph.
struct Graph {
    /// Breadth-first ordered positive heap nodes.
    nodes: Vec<Node>,
    /// Precise outgoing pointer edges, grouped by source node.
    edges: Vec<Edge>,
    /// Address-to-node identity preserves sharing and duplicate incoming edges.
    indices: HashMap<usize, usize>,
}

/// Register a fully initialized counted array without retaining it.
/// This hook is compiled only for tests; an absent tracker performs no allocation.
pub(crate) fn register_array(address: usize, format: i64) {
    if !matches!(format, 1 | 2) {
        return;
    }
    let _ = TRACKER.try_with(|state| {
        if let Some(tracker) = state.borrow_mut().as_mut() {
            tracker.roots.insert(address);
        }
    });
}

/// Retire an address before deallocation or before its owning thread publishes it.
pub(crate) fn retire_array(address: usize) {
    let _ = TRACKER.try_with(|state| {
        if let Some(tracker) = state.borrow_mut().as_mut() {
            tracker.roots.remove(&address);
        }
    });
}

/// A managed pointer is neither a tagged immediate nor the initialized null hole.
fn block_word(word: i64) -> bool {
    word != 0 && word & 1 == 0
}

/// Positive frame markers are excluded in addition to zero and atomic counts.
fn local_heap(header: u64) -> bool {
    let count = block::rc(header);
    count > 0 && count != block::RC_FRAME && count != block::RC_FRAME_RELEASED
}

impl Graph {
    /// Insert a node only when its metadata fits the remaining pass budget.
    fn insert(&mut self, address: usize, header: u64, budget: &mut Budget) -> Option<usize> {
        if budget.nodes == 0 {
            return None;
        }
        budget.nodes -= 1;
        let index = self.nodes.len();
        self.nodes.push(Node {
            address,
            header,
            incoming: 0,
            first: 0,
            end: 0,
        });
        self.indices.insert(address, index);
        Some(index)
    }

    /// Follow only quiescent positive local nodes; opaque boundaries are never scanned.
    ///
    /// # Safety
    /// The root is registered and live, and the caller guarantees stable managed
    /// payloads and owned backing for every live borrowed alias during this walk.
    unsafe fn discover(root: usize, budget: &mut Budget) -> Option<Self> {
        let header = unsafe { block::read_header(root as *const u8) };
        if !local_heap(header) {
            return None;
        }
        let mut graph = Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            indices: HashMap::new(),
        };
        graph.insert(root, header, budget)?;
        let mut index = 0;
        while index < graph.nodes.len() {
            let address = graph.nodes[index].address;
            let header = graph.nodes[index].header;
            let kind = block::kind(header);
            if !(block::KIND_CTOR..=block::KIND_LAST).contains(&kind) || kind == block::KIND_TVAR {
                return None;
            }
            let scan = unsafe { block::Scan::of(address as *const u8, header) };
            // Wide sparse maps may skip many raw words in counted_below. Charge
            // the entire scan up front so that skipped words remain bounded too.
            budget.slots = budget.slots.checked_sub(scan.words())?;
            let first = graph.edges.len();
            let mut below = scan.words();
            while let Some((slot, _code)) = scan.counted_below(below) {
                below = slot;
                let word = unsafe { (address as *const i64).add(slot).read() };
                if !block_word(word) {
                    continue;
                }
                if word & 7 != 0 || budget.edges == 0 {
                    return None;
                }
                budget.edges -= 1;
                let target = if let Some(&existing) = graph.indices.get(&(word as usize)) {
                    Some(existing)
                } else {
                    let child_header = unsafe { block::read_header(word as usize as *const u8) };
                    let count = block::rc(child_header);
                    if matches!(count, block::RC_FRAME | block::RC_FRAME_RELEASED) {
                        return None;
                    }
                    if local_heap(child_header) {
                        Some(graph.insert(word as usize, child_header, budget)?)
                    } else {
                        None
                    }
                };
                if let Some(target) = target {
                    graph.nodes[target].incoming = graph.nodes[target].incoming.checked_add(1)?;
                }
                graph.edges.push(Edge { slot, word, target });
            }
            graph.nodes[index].first = first;
            graph.nodes[index].end = graph.edges.len();
            index += 1;
        }
        Some(graph)
    }

    /// Derive external owners in shadow counters and mark their full forward closure live.
    fn live(&self) -> Option<Vec<bool>> {
        let mut live = vec![false; self.nodes.len()];
        let mut pending = VecDeque::new();
        for (index, node) in self.nodes.iter().enumerate() {
            let count = usize::try_from(block::rc(node.header)).ok()?;
            if node.incoming > count {
                return None;
            }
            if count > node.incoming {
                live[index] = true;
                pending.push_back(index);
            }
        }
        while let Some(index) = pending.pop_front() {
            let node = &self.nodes[index];
            for edge in &self.edges[node.first..node.end] {
                if let Some(target) = edge.target {
                    if !live[target] {
                        live[target] = true;
                        pending.push_back(target);
                    }
                }
            }
        }
        Some(live)
    }

    /// Detach all unreachable edges before dropping outgoing live references and disposing blocks.
    ///
    /// # Safety
    /// Discovery was complete at a quiescent safepoint, all live aliases are
    /// backed by counted owners, and no graph header or payload has changed.
    unsafe fn reclaim(&self, live: &[bool]) -> Option<usize> {
        if self
            .nodes
            .iter()
            .any(|node| unsafe { block::read_header(node.address as *const u8) } != node.header)
        {
            return None;
        }
        for (index, node) in self.nodes.iter().enumerate() {
            if !live[index] {
                for edge in &self.edges[node.first..node.end] {
                    unsafe { (node.address as *mut i64).add(edge.slot).write(0) };
                }
            }
        }
        for (index, node) in self.nodes.iter().enumerate() {
            if !live[index] {
                for edge in &self.edges[node.first..node.end] {
                    if edge.target.is_none_or(|target| live[target]) {
                        unsafe { rc::cinch_drop(edge.word) };
                    }
                }
            }
        }
        let mut reclaimed = 0;
        for (index, node) in self.nodes.iter().enumerate() {
            if !live[index] {
                unsafe { rc::dispose_detached(node.address as *mut u8, node.header) };
                reclaimed += 1;
            }
        }
        Some(reclaimed)
    }
}

/// Analyze registered roots at an explicit quiescent test safepoint without retaining them.
/// Trial analysis leaves counts untouched; reclamation releases only outgoing live references.
///
/// # Safety
/// No participant may mutate or publish a positive graph reachable from a
/// candidate, and all borrowed aliases must still have owned backing. No
/// pointer-reversal release or mark traversal may be active on this thread.
pub(crate) unsafe fn collect_local(limits: Limits) -> Outcome {
    let roots = TRACKER.with(|state| {
        let mut state = state.borrow_mut();
        let Some(tracker) = state.as_mut() else {
            return None;
        };
        if tracker.running {
            return None;
        }
        tracker.running = true;
        Some(match tracker.cursor {
            Some(cursor) => tracker
                .roots
                .range((Excluded(cursor), Unbounded))
                .chain(tracker.roots.range((Unbounded, Included(cursor))))
                .take(limits.roots)
                .copied()
                .collect::<Vec<_>>(),
            None => tracker.roots.iter().take(limits.roots).copied().collect(),
        })
    });
    let Some(roots) = roots else {
        return Outcome::default();
    };
    let mut budget = Budget {
        nodes: limits.nodes,
        edges: limits.edges,
        slots: limits.slots,
    };
    let mut outcome = Outcome::default();
    for root in roots {
        if budget.nodes == 0 || budget.slots == 0 {
            break;
        }
        let active = TRACKER.with(|state| {
            let mut state = state.borrow_mut();
            let tracker = state.as_mut().unwrap();
            tracker.cursor = Some(root);
            tracker.roots.contains(&root)
        });
        if !active {
            continue;
        }
        let result = unsafe { Graph::discover(root, &mut budget) }.and_then(|graph| {
            let live = graph.live()?;
            unsafe { graph.reclaim(&live) }
        });
        match result {
            Some(0) => outcome.retained += 1,
            Some(count) => outcome.reclaimed += count,
            None => outcome.aborted += 1,
        }
    }
    TRACKER.with(|state| {
        if let Some(tracker) = state.borrow_mut().as_mut() {
            tracker.running = false;
        }
    });
    outcome
}

/// A test guard keeps registration on the same thread and refuses leftover candidates.
struct TrackingGuard {
    /// Prevent moving a local registry guard to a different thread.
    _local: PhantomData<Rc<()>>,
}

impl TrackingGuard {
    /// Enable tracking for one isolated test, preserving default runtime-test allocation behavior.
    fn new() -> Self {
        TRACKER.with(|state| {
            assert!(state.borrow().is_none());
            *state.borrow_mut() = Some(Tracker::default());
        });
        Self {
            _local: PhantomData,
        }
    }
}

impl Drop for TrackingGuard {
    /// Remove only host metadata; tests must already have reclaimed their managed candidates.
    fn drop(&mut self) {
        TRACKER.with(|state| {
            let tracker = state.borrow_mut().take().unwrap();
            if !std::thread::panicking() {
                assert!(
                    tracker.roots.is_empty(),
                    "test left live collector candidates"
                );
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arrays;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Barrier};

    /// Read a managed array element as a borrowed word without changing its count.
    unsafe fn element(array: i64, slot: usize) -> i64 {
        unsafe { (array as usize as *const i64).add(slot + 2).read() }
    }

    /// Install a retained pointer, preserving the test's separately owned outside alias.
    unsafe fn retain_into(array: i64, slot: usize, value: i64) {
        unsafe { rc::cinch_dup(value) };
        unsafe { arrays::write(array, slot as i64, value) };
    }

    /// Observe the real reference count rather than a trial or retained-candidate count.
    unsafe fn count(word: i64) -> i64 {
        block::rc(unsafe { block::read_header(word as usize as *const u8) })
    }

    /// Construct a leaf whose single raw payload word is not a managed reference.
    unsafe fn leaf(value: i64) -> i64 {
        let p = unsafe { block::allocate_ctor(8, block::shape_field(0, 0), true) };
        unsafe { p.cast::<i64>().write(value) };
        p as usize as i64
    }

    /// Build two counted arrays whose edge counts are valid while both outside aliases are owned.
    unsafe fn pair() -> (i64, i64) {
        let a = unsafe { arrays::array(1, 2) };
        let b = unsafe { arrays::array(1, 2) };
        unsafe { retain_into(a, 0, b) };
        unsafe { retain_into(b, 0, a) };
        (a, b)
    }

    /// A reachable unique array must stay unique; an unreachable self-edge must reclaim without an extra retained root.
    #[test]
    fn unique_alias_survives_without_count_inflation_and_self_cycle_reclaims() {
        let _tracking = TrackingGuard::new();
        let a = unsafe { arrays::array(1, 2) };
        assert_eq!(unsafe { count(a) }, 1);
        let alive = unsafe { collect_local(Limits::default()) };
        assert_eq!(alive.reclaimed, 0);
        assert_eq!(unsafe { count(a) }, 1);
        assert_eq!(unsafe { element(a, 0) }, 0);
        unsafe { retain_into(a, 0, a) };
        unsafe { rc::cinch_drop(a) };
        let dead = unsafe { collect_local(Limits::default()) };
        assert_eq!(dead.reclaimed, 1);
    }

    /// An external alias protects every reachable member until the alias is actually dropped.
    #[test]
    fn reachable_two_cell_cycle_preserves_payload_and_original_counts() {
        let _tracking = TrackingGuard::new();
        let (a, b) = unsafe { pair() };
        unsafe { rc::cinch_drop(b) };
        let before = (unsafe { count(a) }, unsafe { count(b) });
        let alive = unsafe { collect_local(Limits::default()) };
        assert_eq!(alive.reclaimed, 0);
        assert_eq!(before, (2, 1));
        assert_eq!((unsafe { count(a) }, unsafe { count(b) }), before);
        assert_eq!(unsafe { element(a, 0) }, b);
        assert_eq!(unsafe { element(b, 0) }, a);
        unsafe { rc::cinch_drop(a) };
        let dead = unsafe { collect_local(Limits::default()) };
        assert_eq!(dead.reclaimed, 2);
    }

    /// Multiple references in distinct slots each contribute to the shadow incoming count and detach exactly once.
    #[test]
    fn duplicate_internal_edges_reclaim_exactly_once() {
        let _tracking = TrackingGuard::new();
        let a = unsafe { arrays::array(2, 2) };
        let b = unsafe { arrays::array(1, 2) };
        unsafe {
            retain_into(a, 0, b);
            retain_into(a, 1, b);
            retain_into(b, 0, a);
            rc::cinch_drop(a);
            rc::cinch_drop(b);
        }
        assert_eq!((unsafe { count(a) }, unsafe { count(b) }), (1, 2));
        let dead = unsafe { collect_local(Limits::default()) };
        assert_eq!(dead.reclaimed, 2);
    }

    /// A captured cell uses the closure's precise saved-argument map, never its raw code or arity words.
    #[test]
    fn closure_capture_cycle_reclaims_with_its_cell() {
        let _tracking = TrackingGuard::new();
        let a = unsafe { arrays::array(1, 2) };
        let closure = unsafe {
            block::allocate_shaped(32, block::KIND_CLOSURE, block::shape_field(0, 1), true)
        };
        unsafe {
            closure.cast::<u64>().add(1).write(1);
            closure.cast::<u64>().add(2).write(1);
            rc::cinch_dup(a);
            closure.cast::<i64>().add(3).write(a);
            retain_into(a, 0, closure as usize as i64);
            rc::cinch_drop(closure as usize as i64);
            rc::cinch_drop(a);
        }
        assert_eq!(unsafe { collect_local(Limits::default()) }.reclaimed, 2);
    }

    /// A long ring is traversed and disposed iteratively on a deliberately small native stack.
    #[test]
    fn large_cycle_uses_bounded_native_stack() {
        std::thread::Builder::new()
            .stack_size(128 * 1024)
            .spawn(|| {
                let _tracking = TrackingGuard::new();
                let size = 10_000;
                let nodes = (0..size)
                    .map(|_| unsafe { arrays::array(1, 2) })
                    .collect::<Vec<_>>();
                for index in 0..size {
                    unsafe { retain_into(nodes[index], 0, nodes[(index + 1) % size]) };
                }
                for &node in &nodes {
                    unsafe { rc::cinch_drop(node) };
                }
                let result = unsafe {
                    collect_local(Limits {
                        roots: 1,
                        nodes: size,
                        edges: size,
                        slots: 3 * size,
                    })
                };
                assert_eq!(result.reclaimed, size);
            })
            .unwrap()
            .join()
            .unwrap();
    }

    /// A cycle may die while a shared live tail survives with the exact one remaining external reference.
    #[test]
    fn outgoing_live_tail_is_dropped_once_after_internal_detach() {
        let _tracking = TrackingGuard::new();
        let a = unsafe { arrays::array(2, 2) };
        let b = unsafe { arrays::array(1, 2) };
        let tail = unsafe { leaf(71) };
        unsafe {
            retain_into(a, 0, b);
            retain_into(a, 1, tail);
            retain_into(b, 0, a);
            rc::cinch_drop(a);
            rc::cinch_drop(b);
        }
        let dead = unsafe { collect_local(Limits::default()) };
        assert_eq!(dead.reclaimed, 2);
        assert_eq!(unsafe { count(tail) }, 1);
        assert_eq!(unsafe { (tail as usize as *const i64).read() }, 71);
        unsafe { rc::cinch_drop(tail) };
    }

    /// An incomplete component leaves all actual headers and slots unchanged, then succeeds with a sufficient budget.
    #[test]
    fn budget_abort_does_not_modify_graph_or_counts() {
        let _tracking = TrackingGuard::new();
        let (a, b) = unsafe { pair() };
        unsafe {
            rc::cinch_drop(a);
            rc::cinch_drop(b)
        };
        let ah = unsafe { block::read_header(a as usize as *const u8) };
        let bh = unsafe { block::read_header(b as usize as *const u8) };
        for limits in [
            Limits {
                nodes: 1,
                ..Limits::default()
            },
            Limits {
                edges: 0,
                ..Limits::default()
            },
            Limits {
                slots: 2,
                ..Limits::default()
            },
        ] {
            let stopped = unsafe { collect_local(limits) };
            assert_eq!(stopped.reclaimed, 0);
            assert!(stopped.aborted > 0);
            assert_eq!(unsafe { block::read_header(a as usize as *const u8) }, ah);
            assert_eq!(unsafe { block::read_header(b as usize as *const u8) }, bh);
            assert_eq!(unsafe { element(a, 0) }, b);
            assert_eq!(unsafe { element(b, 0) }, a);
        }
        assert_eq!(unsafe { collect_local(Limits::default()) }.reclaimed, 2);
    }

    /// Array retirement precedes sharing; an atomic child can mutate while only its header is considered by a local pass.
    #[test]
    fn atomic_boundary_is_not_traced_or_kept_in_local_registry() {
        let _tracking = TrackingGuard::new();
        let a = unsafe { arrays::array(2, 2) };
        let b = unsafe { arrays::array(1, 2) };
        let shared = unsafe { arrays::array(1, 2) };
        unsafe { rc::cinch_mark_mt(shared) };
        TRACKER.with(|state| {
            assert!(!state
                .borrow()
                .as_ref()
                .unwrap()
                .roots
                .contains(&(shared as usize)))
        });
        unsafe {
            retain_into(a, 0, b);
            retain_into(b, 0, a);
            retain_into(a, 1, shared);
            rc::cinch_drop(a);
            rc::cinch_drop(b);
        }
        let stop = Arc::new(AtomicBool::new(false));
        let finished = Arc::clone(&stop);
        let started = Arc::new(Barrier::new(2));
        let worker_started = Arc::clone(&started);
        let writer = std::thread::spawn(move || {
            unsafe { arrays::write(shared, 0, 1) };
            worker_started.wait();
            let mut word = 1;
            while !finished.load(Ordering::Acquire) {
                unsafe { arrays::write(shared, 0, word) };
                word ^= 2;
            }
            shared
        });
        started.wait();
        let dead = unsafe { collect_local(Limits::default()) };
        assert_eq!(dead.reclaimed, 2);
        stop.store(true, Ordering::Release);
        let shared = writer.join().unwrap();
        assert_eq!(unsafe { count(shared) }, -1);
        unsafe { rc::cinch_drop(shared) };
    }

    /// Direct free and poisoned free share the same retirement hook, so later address reuse cannot leave stale candidates.
    #[test]
    fn ordinary_free_removes_nonowning_candidate_immediately() {
        let _tracking = TrackingGuard::new();
        for _ in 0..128 {
            let a = unsafe { arrays::array(1, 2) };
            assert_eq!(unsafe { count(a) }, 1);
            unsafe { rc::cinch_drop(a) };
            TRACKER.with(|state| assert!(state.borrow().as_ref().unwrap().roots.is_empty()));
        }
        assert_eq!(unsafe { collect_local(Limits::default()) }.reclaimed, 0);
    }
}

//! Loop metadata for the `#[unroll(N)]` hint (attribute annotations
//! specification section 5).
//!
//! A self tail call runs in constant stack: the saturated call stores its
//! arguments over the parameter slots and branches to the unit's loop
//! header (surface grammar local amendments section 9; decision D104).
//! That branch is a back edge, and LLVM reads a loop's unrolling policy
//! from `!llvm.loop` metadata attached to the back edge's terminator. The
//! hint therefore becomes metadata there and nowhere else: the core is
//! never unrolled, the hint cannot change a program's values, and a
//! definition whose body has no self tail call emits nothing at all,
//! because it has no back edge to carry the metadata.
//!
//! Two verifier rules decide the shape of the nodes. A loop node MUST be
//! `distinct`: loop ids are compared by node identity, so a uniqued node
//! would make two different loops look like the same loop. And a loop node
//! MUST name itself as its first operand, which is how LLVM tells the
//! node's own id apart from a further loop id in the operand list (it skips
//! that first operand when it reads the policy). Both rules are why the
//! loop node is written after its own id is known:
//!
//! ```text
//! !7 = distinct !{!7, !6}          the loop node, self-referential
//! !6 = !{!"llvm.loop.unroll.count", i32 4}
//! ```
//!
//! The property node is a plain (uniqued) node: two loops asking for the
//! same count may share it, which is exactly what uniquing is for.

use crate::core::UnrollHint;
use std::cell::{Cell, RefCell};

/// The metadata one unit's back edges carry, computed on the first back
/// edge and reused by the rest.
///
/// A definition may reach its loop header from several sites (one per
/// branch that tail-calls itself), and every one of those latches belongs
/// to the same loop, so they must carry one loop node rather than one node
/// each. Allocating lazily also keeps the no-loop case free: a definition
/// that never tail-calls itself never allocates a node, so its module
/// carries no loop metadata to be ignored.
#[derive(Clone, Debug, Default)]
pub struct LoopMeta {
    hint: Option<UnrollHint>,
    /// The terminator suffix once a back edge asked for it.
    suffix: Option<String>,
}

impl LoopMeta {
    /// The loop metadata for one definition, from its hint.
    pub fn new(hint: Option<UnrollHint>) -> Self {
        LoopMeta { hint, suffix: None }
    }

    /// The text to append to a back edge's `br` line, allocating the nodes
    /// on first use. Empty when the definition carries no hint.
    ///
    /// `defs` and `next` are the module's metadata section: the definitions
    /// written so far, and the id source.
    pub fn suffix(&mut self, defs: &RefCell<Vec<String>>, next: &Cell<usize>) -> String {
        let hint = self.hint;
        self.suffix
            .get_or_insert_with(|| loop_metadata(hint, defs, next))
            .clone()
    }
}

/// The metadata a back-edge terminator carries for `hint`: `", !llvm.loop
/// !N"`, or empty when there is no hint.
pub fn loop_metadata(
    hint: Option<UnrollHint>,
    defs: &RefCell<Vec<String>>,
    next: &Cell<usize>,
) -> String {
    let Some(hint) = hint else {
        return String::new();
    };
    // The parser maps `#[unroll(0)]` to `Disable`, so a zero count cannot
    // reach the property node (`llvm.loop.unroll.count` with zero is
    // meaningless to LLVM).
    debug_assert!(!matches!(hint, UnrollHint::Count(0)));
    let property = match hint {
        UnrollHint::Count(n) => format!("!{{!\"llvm.loop.unroll.count\", i32 {n}}}"),
        UnrollHint::Disable => "!{!\"llvm.loop.unroll.disable\"}".to_string(),
    };
    // The loop node names the property node, so the property node's id is
    // taken first and the loop node's definition can then name both.
    let property_id = take(next);
    defs.borrow_mut().push(format!("!{property_id} = {property}\n"));
    let loop_id = take(next);
    defs.borrow_mut()
        .push(format!("!{loop_id} = distinct !{{!{loop_id}, !{property_id}}}\n"));
    format!(", !llvm.loop !{loop_id}")
}

/// Take the next module metadata id.
fn take(next: &Cell<usize>) -> usize {
    let id = next.get();
    next.set(id + 1);
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A metadata section in memory: ids from 3.
    struct Section {
        next: Cell<usize>,
        defs: RefCell<Vec<String>>,
    }

    impl Section {
        fn new() -> Self {
            Section { next: Cell::new(3), defs: RefCell::new(Vec::new()) }
        }

        fn defs(&self) -> Vec<String> {
            self.defs.borrow().clone()
        }
    }

    /// The definitions `loop_metadata` writes, and the suffix it returns.
    fn meta(hint: Option<UnrollHint>) -> (Vec<String>, String) {
        let section = Section::new();
        let suffix = loop_metadata(hint, &section.defs, &section.next);
        (section.defs(), suffix)
    }

    #[test]
    fn a_count_is_a_property_node_behind_a_self_referential_loop_node() {
        let (defs, suffix) = meta(Some(UnrollHint::Count(4)));
        assert_eq!(suffix, ", !llvm.loop !4");
        assert_eq!(defs[0], "!3 = !{!\"llvm.loop.unroll.count\", i32 4}\n");
        assert_eq!(defs[1], "!4 = distinct !{!4, !3}\n");
    }

    #[test]
    fn disable_is_the_property_node_with_no_count() {
        let (defs, suffix) = meta(Some(UnrollHint::Disable));
        assert_eq!(suffix, ", !llvm.loop !4");
        assert_eq!(defs[0], "!3 = !{!\"llvm.loop.unroll.disable\"}\n");
        assert_eq!(defs[1], "!4 = distinct !{!4, !3}\n");
    }

    #[test]
    fn no_hint_writes_nothing() {
        let (defs, suffix) = meta(None);
        assert!(defs.is_empty());
        assert_eq!(suffix, "");
    }

    #[test]
    fn the_loop_node_is_allocated_once_however_many_back_edges_ask() {
        let section = Section::new();
        let mut meta = LoopMeta::new(Some(UnrollHint::Count(2)));
        let first = meta.suffix(&section.defs, &section.next);
        let second = meta.suffix(&section.defs, &section.next);
        assert_eq!(first, second);
        assert_eq!(section.defs().len(), 2);
        assert_eq!(section.next.get(), 5);
    }

    #[test]
    fn a_hintless_loop_allocates_nothing() {
        let section = Section::new();
        let mut meta = LoopMeta::new(None);
        assert_eq!(meta.suffix(&section.defs, &section.next), "");
        assert!(section.defs().is_empty());
        assert_eq!(section.next.get(), 3);
    }
}

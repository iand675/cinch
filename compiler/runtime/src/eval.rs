//! The evaluator for reified typed quotations: the runtime half of
//! `eval : Code a -> a` (supercompilation and JIT specification
//! section 7).
//!
//! Quotations are typed at construction: the compiler checks the body
//! and reifies it into the word program below (decision D43: a quotation
//! body is closed runtime-side code whose only inputs are its captured
//! holes), and evaluation is never evidence for a type. What [`run`]
//! adds is only the meaning of a program the compiler already checked. A
//! quote block is still data that can be corrupt (hand-built or
//! damaged), and every such case is an [`Error`], never a value of the
//! quoted type.
//!
//! # The quote block
//!
//! A `Code a` value is one word pointing at a block allocated by
//! generated code (`compiler/src/codegen.rs`, `emit_quote_block`):
//!
//! | byte offset | size | field |
//! |-------------|------|-------|
//! | 0 | 8 | `kind`: `i64`, [`KIND_CODE`] (3) |
//! | 8 | 8 | `code`: the program's first word |
//! | 16 | 8 | `len`: the program's word count |
//! | 24 | 8 | `holes`: the number of captured hole words |
//! | 32 + 8 * i | 8 | `hole[i]`: one quote-block word per hole |
//!
//! Every hole word is itself a quote block (the same header): the `HOLE`
//! opcode evaluates the captured block's program in place, so a `Code t`
//! splice fills a `t` position. [`run`] validates the kind word, a
//! non-null program pointer and sane counts (`len` at least one word and
//! at most [`MAX_CODE_WORDS`], `holes` at most [`MAX_HOLES`]) before
//! use, then decodes and bounds-checks the whole program stream before
//! evaluating any of it.
//!
//! # The opcode table
//!
//! The table mirrors `quote_op` in `compiler/src/codegen.rs`; the
//! encoder and this decoder are one specification and change together. A
//! program is a prefix-encoded expression tree of `u64` words; operand
//! words are either immediates or relocated unit-closure words (in the
//! compiler, LLVM `ptrtoint` constants; in tests, real code pointers).
//!
//! | word | opcode | operands | meaning |
//! |------|--------|----------|---------|
//! | 1 | `INT` | `[v]` | the raw word `v` |
//! | 2 | `BIG` | `[nat, ndigits, digits...]` | a bignum decimal constant: 1 for a `Nat`, 0 for an `Int` (its digits may carry a leading `-`), `ndigits` decimal bytes packed eight per word little endian |
//! | 3 | `BOOL` | `[0|1]` | the boolean word |
//! | 4 | `UNIT` | | the unit word (0) |
//! | 5 | `LOCAL` | `[slot]` | the local's word |
//! | 6 | `HOLE` | `[idx]` | the captured hole's quote block, evaluated in place |
//! | 7 | `LET` | `[slot, value, body]` | bind `slot` to `value` in `body` |
//! | 8 | `IF` | `[cond, then, else]` | evaluate `cond`, then exactly one branch |
//! | 9 | `CASE` | `[scrutinee, ncon, cons..., default]` | select by the scrutinee block's tag word; each `con` is `[tag, nfields, fields..., body]` binding the block's field words; `default` is `[0]` (none), `[1, body]` (wildcard) or `[2, slot, body]` (variable) |
//! | 10 | `APP` | `[fn, nargs, args...]` | apply through the closure apply-stub convention |
//! | 11 | `LAM` | `[stub, arity, nenv, env...]` | allocate the closure block `{ stub; arity; nenv; env... }` of a hoisted lambda |
//! | 12 | `CTOR` | `[tag, nfields, fields...]` | allocate `{ tag; fields... }` |
//! | 13 | `PROJ` | `[value, mode, ...]` | mode 0 `[slot]` loads the block's field word at `8 * (1 + slot)`; mode 1 `[offset, size, signed]` loads a boundary-layout scalar at `8 + offset`; mode 2 `[offset]` loads a boundary-layout word at `8 + offset` |
//! | 14 | `FAULT` | | the program's own fault (exit 70) |
//! | 15 | `LAZY_ALLOC` | | allocate `{ 0; 0; 0 }`: an empty memo cell |
//! | 16 | `LAZY_INIT` | `[cell-local, thunk]` | store the thunk into the cell; the value is the unit word |
//! | 17 | `DEMAND` | `[cell]` | run the memo cell's state machine |
//! | 18 | `UNITREF` | `[clo]` | a unit's closure word as a value |
//!
//! # Value conventions
//!
//! The evaluator shares the crate's word conventions (see the crate
//! docs) and the code generator's block layouts:
//!
//! * Every value is one `i64` word. An ADT or `CTOR` block is
//!   `{ i64 tag; i64 field0; ... }`; field `i` lives at byte offset
//!   `8 * (1 + i)`, and a boundary-layout projection reads at payload
//!   base 8 plus its recorded offset.
//! * A function value or unit closure is `{ code_ptr; arity; nfilled;
//!   saved[0..] }`. Applying one loads word 0 and calls it as
//!   `unsafe extern "C" fn(i64 self, i64 argc, *mut i64 argv) -> i64`
//!   (the `__apply` convention of `crate::task::apply_closure`), so
//!   under- and over-application behave exactly as the generated apply
//!   stubs define.
//! * A memo cell is `{ state; thunk; value }` (24 zeroed bytes, as
//!   `cinch_alloc` allocates). `DEMAND` runs the same state machine as the
//!   generated `__cinch_demand` helper (`MEMO_HELPERS` in
//!   `compiler/src/codegen.rs`): state 0 stores 1, applies the thunk to
//!   one Unit word (0), stores the value and stores state 2; state 2
//!   returns the stored value; any other state (including 1, a cell
//!   demanded while its own evaluation runs) is the `BlackHole` fault.
//!
//! # Faults and exit codes
//!
//! Every failure of [`run`] is an [`Error`]; `cinch_eval` maps it onto the
//! defined fault paths and a failing evaluation therefore produces no
//! value:
//!
//! | `Error` | when | `cinch_eval` exit code |
//! |---------|------|---------------------|
//! | [`Error::BlackHole`] | the program's `FAULT` opcode, or a memo cell demanded mid-evaluation | 70 (the `BlackHole` fault, `cinch_blackhole`) |
//! | [`Error::CorruptValue`] | malformed contents: bad kind word, null or short program, truncated or unknown opcode stream, out-of-range local or hole index, non-block word where a block is required, trailing words | 71 (`cinch_panic`, message naming eval) |
//! | [`Error::MalformedDecimal`] | a `BIG` constant's digit bytes are not a numeral | 71 |
//! | [`Error::NestingTooDeep`] | nesting past [`MAX_NESTING`] | 71 |
//!
//! # Nesting bound
//!
//! Decoding, evaluating and nested `HOLE` runs each consume one unit of
//! a per-thread budget of [`MAX_NESTING`] units; exceeding it is
//! [`Error::NestingTooDeep`] rather than a stack overflow. Applied stub
//! code is native code and recurses on its own terms (as generated code
//! does); the budget bounds everything the evaluator itself nests.
//!
//! # Re-entrant faults
//!
//! An applied stub cannot return an error through the `__apply` ABI (the
//! generated helpers exit the process at the innermost fault instead), so
//! a [`run`] that fails while another evaluation is on the stack records
//! its error in a per-thread pending slot. The enclosing evaluation
//! surfaces the pending error after the next stub application, so a
//! fault raised inside applied code aborts the whole evaluation instead
//! of being lost in a returned word.
//!
//! # Evaluator conventions
//!
//! Decisions where this decoder pins down a detail the encoder leaves
//! open:
//!
//! * `IF` takes any nonzero condition word as true, mirroring the
//!   generated `icmp ne i64 %c, 0`.
//! * `CTOR`'s `fields...` operands are expressions (the field values),
//!   while `CASE`'s `con` `fields...` operands are local slots to bind.
//! * `LAZY_INIT`'s `cell-local` operand is an immediate local slot (as
//!   the encoder emits it), while `DEMAND`'s `cell` operand is an
//!   expression.
//! * `CASE` requires unique tags per `con`; a scrutinee tag that selects
//!   no `con` with no default is corrupt data (a well-typed quotation's
//!   matches are exhaustive and never reach it).
//! * A `BIG` constant's padding bytes (the last word beyond `ndigits`)
//!   must be zero, and the program must be consumed exactly (trailing
//!   words are corrupt).
//! * `PROJ` mode 1 accepts `size` 0 through 8 bytes (0 loads the
//!   constant 0, matching the compiler's zero-width scalar) and sign- or
//!   zero-extends a narrower load to the value word, matching the
//!   generated `load iN` plus `sext`/`zext`. Recorded offsets and sizes
//!   are trusted as in generated code; only the block word itself is
//!   checked (non-null and word-aligned, the crate's extern-word
//!   policy).

use std::cell::Cell;

use crate::error::{Error, Result};

/// The quote block's kind word: the code generator's `KIND_CODE`
/// (`emit_quote_block` in `compiler/src/codegen.rs`). It is checked
/// before any other header word, so a bignum or unit-closure word used
/// where a quote block belongs faults as corrupt data.
pub const KIND_CODE: i64 = 3;

/// The nesting budget (in units of evaluator recursion) for decoder
/// recursion, evaluator recursion and nested `HOLE` runs combined.
/// Generous for any compiler-generated quotation; past it is
/// [`Error::NestingTooDeep`], never a stack overflow. One unit costs a
/// couple of small native frames, so the whole bound fits a 2 MiB
/// thread stack in a debug build and a 512 KiB one in a release build
/// (measured with this crate's tests).
pub const MAX_NESTING: u32 = 1024;

/// Sane upper bound for a quote program's word count (the count is data
/// and is validated before the stream is read).
pub const MAX_CODE_WORDS: u64 = 1 << 26;

/// Sane upper bound for a quote block's hole count.
pub const MAX_HOLES: u64 = 1 << 20;

/// The opcodes of a quotation's reified program. One specification with
/// `quote_op` in `compiler/src/codegen.rs`: change them together.
mod op {
    /// `[v]`: a raw word (machine scalar values).
    pub const INT: u64 = 1;
    /// `[nat, ndigits, digits...]`: a bignum decimal constant, its
    /// digits packed eight bytes per word, little endian.
    pub const BIG: u64 = 2;
    /// `[0|1]`.
    pub const BOOL: u64 = 3;
    pub const UNIT: u64 = 4;
    /// `[slot]`: the local's word.
    pub const LOCAL: u64 = 5;
    /// `[idx]`: the captured hole's quote block, evaluated in place
    /// (a `Code t` splice fills a `t` position).
    pub const HOLE: u64 = 6;
    /// `[slot, value, body]`.
    pub const LET: u64 = 7;
    /// `[cond, then, else]`.
    pub const IF: u64 = 8;
    /// `[scrutinee, ncon, cons..., default]`; each `con` is
    /// `[tag, nfields, fields..., body]` binding the block's field words,
    /// `default` is `[0]` (no default), `[1, body]` (wildcard) or
    /// `[2, slot, body]` (variable).
    pub const CASE: u64 = 9;
    /// `[fn, nargs, args...]`: apply through the closure apply-stub
    /// convention.
    pub const APP: u64 = 10;
    /// `[stub, arity, nenv, env...]`: allocate the closure block
    /// `{ stub; arity; nenv; env... }` of a hoisted lambda.
    pub const LAM: u64 = 11;
    /// `[tag, nfields, fields...]`: allocate `{ tag; fields... }`.
    pub const CTOR: u64 = 12;
    /// `[value, mode, ...]`: mode 0 `[slot]` loads the block's field
    /// word at `8 * (1 + slot)`; mode 1 `[offset, size, signed]` loads a
    /// boundary-layout scalar at `8 + offset`; mode 2 `[offset]` loads a
    /// boundary-layout word at `8 + offset`.
    pub const PROJ: u64 = 13;
    /// The program's own fault (exit 70).
    pub const FAULT: u64 = 14;
    /// Allocate `{ 0; 0; 0 }`: an empty memo cell.
    pub const LAZY_ALLOC: u64 = 15;
    /// `[cell-local, thunk]`: store the thunk into the cell; the value
    /// is the unit word.
    pub const LAZY_INIT: u64 = 16;
    /// `[cell]`: run the memo cell's state machine.
    pub const DEMAND: u64 = 17;
    /// `[clo]`: a unit's closure word as a value.
    pub const UNITREF: u64 = 18;
}

// ------------------------------------------------------------ decoded tree

/// One decoded expression node. Binds are dense environment indices (the
/// decoder resolves local slots to their innermost binding), so
/// evaluation is pure indexing with no slot lookup.
enum Node {
    Int(i64),
    /// A `BIG` constant whose word is built during decoding.
    Big(i64),
    Bool(i64),
    Unit,
    Local(u32),
    Hole(u32),
    Let(u32, Box<Node>, Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Case(Box<Node>, Vec<Arm>, Alt),
    App(Box<Node>, Vec<Node>),
    Lam { stub: i64, arity: u64, env: Vec<Node> },
    Ctor { tag: i64, fields: Vec<Node> },
    Proj { value: Box<Node>, mode: u64, slot: u64, offset: u64, size: u64, signed: bool },
    Fault,
    LazyAlloc,
    LazyInit(u32, Box<Node>),
    Demand(Box<Node>),
    UnitRef(i64),
}

/// One `CASE` constructor alternative: its field binders (dense
/// environment indices, in block field order) and its body.
struct Arm {
    tag: u64,
    fields: Vec<u32>,
    body: Node,
}

/// The `CASE` default alternative.
enum Alt {
    None,
    Wild(Box<Node>),
    Var(u32, Box<Node>),
}

// ------------------------------------------------------------ thread state

/// Per-thread evaluator state: the recursion budget, the `run` nesting
/// counter (for the re-entrant fault channel), and the pending fault
/// raised from applied code.
#[derive(Clone, Copy)]
struct Tls {
    frames: u32,
    runs: u32,
    pending: Option<Error>,
}

thread_local! {
    static TLS: Cell<Tls> = const {
        Cell::new(Tls { frames: 0, runs: 0, pending: None })
    };
}

/// One recursion frame of the [`MAX_NESTING`] budget; dropping it
/// returns the frame.
struct Frame;

impl Frame {
    fn new() -> Result<Frame> {
        TLS.with(|t| {
            let mut s = t.get();
            if s.frames >= MAX_NESTING {
                return Err(Error::NestingTooDeep);
            }
            s.frames += 1;
            t.set(s);
            Ok(Frame)
        })
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        TLS.with(|t| {
            let mut s = t.get();
            s.frames -= 1;
            t.set(s);
        });
    }
}

/// Surface and clear the pending fault, if any.
fn take_pending() -> Result<()> {
    TLS.with(|t| {
        let mut s = t.get();
        let e = s.pending.take();
        t.set(s);
        match e {
            Some(e) => Err(e),
            None => Ok(()),
        }
    })
}

// ------------------------------------------------------------ word access

/// A word used where a block is required: non-null and word-aligned.
/// Any other bogus word is a caller bug and faults like an invalid
/// pointer dereference (the crate's extern-word policy).
fn block_ptr(word: i64) -> Result<*const i64> {
    if word == 0 || word & 7 != 0 {
        return Err(Error::CorruptValue);
    }
    Ok(word as *const i64)
}

/// Read the block word at byte offset `off` (a multiple of 8).
unsafe fn word_at(p: *const i64, off: usize) -> i64 {
    unsafe { p.cast::<u8>().add(off).cast::<i64>().read() }
}

/// Write the block word at byte offset `off` (a multiple of 8).
unsafe fn word_set(p: *const i64, off: usize, v: i64) {
    unsafe { p.cast_mut().cast::<u8>().add(off).cast::<i64>().write(v) }
}

/// Load a `size`-byte little-endian scalar at `off` and extend it into
/// the value word (`size` 0..=8 is validated at decode time; 0 loads the
/// constant 0, the compiler's zero-width scalar).
unsafe fn scalar_at(p: *const i64, off: usize, size: u64, signed: bool) -> i64 {
    let b = p.cast::<u8>();
    unsafe {
        match size {
            0 => 0,
            1 => {
                let v = b.add(off).read();
                if signed {
                    v as i8 as i64
                } else {
                    v as i64
                }
            }
            2 => {
                let v = b.add(off).cast::<u16>().read_unaligned();
                if signed {
                    v as i16 as i64
                } else {
                    v as i64
                }
            }
            4 => {
                let v = b.add(off).cast::<u32>().read_unaligned();
                if signed {
                    v as i32 as i64
                } else {
                    v as i64
                }
            }
            _ => b.add(off).cast::<i64>().read_unaligned(),
        }
    }
}

// ------------------------------------------------------------ the decoder

/// The decoder over one program stream: the operand cursor, the local
/// slots in scope (innermost last), and the dense environment indices to
/// hand out.
struct Dec<'a> {
    words: &'a [u64],
    pos: usize,
    slots: Vec<(u64, u32)>,
    next: u32,
    holes: usize,
}

impl<'a> Dec<'a> {
    fn new(words: &'a [u64], holes: usize) -> Dec<'a> {
        Dec { words, pos: 0, slots: Vec::new(), next: 0, holes }
    }

    /// The next operand word.
    fn word(&mut self) -> Result<u64> {
        let w = self.words.get(self.pos).copied().ok_or(Error::CorruptValue)?;
        self.pos += 1;
        Ok(w)
    }

    /// The next `n` operand words (a `BIG` digit chunk).
    fn tail(&mut self, n: usize) -> Result<&'a [u64]> {
        let end = self.pos.checked_add(n).ok_or(Error::CorruptValue)?;
        if end > self.words.len() {
            return Err(Error::CorruptValue);
        }
        let out = &self.words[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    /// Reject an operand count the remaining stream cannot hold, before
    /// reserving anything for it.
    fn budget(&self, n: usize) -> Result<()> {
        if n > self.words.len() - self.pos {
            return Err(Error::CorruptValue);
        }
        Ok(())
    }

    /// A count operand as `usize`.
    fn count(&self, n: u64) -> Result<usize> {
        let n = usize::try_from(n).map_err(|_| Error::CorruptValue)?;
        self.budget(n)?;
        Ok(n)
    }

    /// The next operand word as a count.
    fn count_word(&mut self) -> Result<usize> {
        let w = self.word()?;
        self.count(w)
    }

    /// Bind `slot` to a fresh dense environment index.
    fn bind(&mut self, slot: u64) -> u32 {
        let idx = self.next;
        self.next += 1;
        self.slots.push((slot, idx));
        idx
    }

    /// The innermost binding of `slot`; unbound is corrupt data.
    fn lookup(&self, slot: u64) -> Result<u32> {
        self.slots
            .iter()
            .rev()
            .find(|(s, _)| *s == slot)
            .map(|(_, i)| *i)
            .ok_or(Error::CorruptValue)
    }

    /// Decode one expression; consumes exactly its own words. The arm
    /// bodies live in small helpers: the nesting bound counts frames,
    /// and small frames keep a debug build's per-frame cost low (see
    /// [`Frame`]).
    fn expr(&mut self) -> Result<Node> {
        let _f = Frame::new()?;
        match self.word()? {
            op::INT => Ok(Node::Int(self.word()? as i64)),
            op::BIG => self.big(),
            op::BOOL => self.expr_bool(),
            op::UNIT => Ok(Node::Unit),
            op::LOCAL => self.expr_local(),
            op::HOLE => self.expr_hole(),
            op::LET => self.expr_let(),
            op::IF => self.expr_if(),
            op::CASE => self.case(),
            op::APP => self.expr_app(),
            op::LAM => self.expr_lam(),
            op::CTOR => self.expr_ctor(),
            op::PROJ => self.expr_proj(),
            op::FAULT => Ok(Node::Fault),
            op::LAZY_ALLOC => Ok(Node::LazyAlloc),
            op::LAZY_INIT => self.expr_lazy_init(),
            op::DEMAND => self.expr_demand(),
            op::UNITREF => Ok(Node::UnitRef(self.word()? as i64)),
            _ => Err(Error::CorruptValue),
        }
    }

    fn expr_bool(&mut self) -> Result<Node> {
        let b = self.word()?;
        if b > 1 {
            return Err(Error::CorruptValue);
        }
        Ok(Node::Bool(b as i64))
    }

    fn expr_local(&mut self) -> Result<Node> {
        let slot = self.word()?;
        Ok(Node::Local(self.lookup(slot)?))
    }

    fn expr_hole(&mut self) -> Result<Node> {
        let idx = usize::try_from(self.word()?).map_err(|_| Error::CorruptValue)?;
        if idx >= self.holes {
            return Err(Error::CorruptValue);
        }
        Ok(Node::Hole(idx as u32))
    }

    fn expr_let(&mut self) -> Result<Node> {
        let slot = self.word()?;
        let value = self.expr()?;
        let saved = self.slots.len();
        let idx = self.bind(slot);
        let body = self.expr();
        self.slots.truncate(saved);
        Ok(Node::Let(idx, Box::new(value), Box::new(body?)))
    }

    fn expr_if(&mut self) -> Result<Node> {
        let c = self.expr()?;
        let t = self.expr()?;
        let f = self.expr()?;
        Ok(Node::If(Box::new(c), Box::new(t), Box::new(f)))
    }

    fn expr_app(&mut self) -> Result<Node> {
        let f = self.expr()?;
        let n = self.count_word()?;
        let mut args = Vec::with_capacity(n);
        for _ in 0..n {
            args.push(self.expr()?);
        }
        Ok(Node::App(Box::new(f), args))
    }

    fn expr_lam(&mut self) -> Result<Node> {
        let stub = self.word()? as i64;
        if stub == 0 {
            return Err(Error::CorruptValue);
        }
        let arity = self.word()?;
        let nenv = self.count_word()?;
        if nenv as u64 > arity {
            return Err(Error::CorruptValue);
        }
        let mut env = Vec::with_capacity(nenv);
        for _ in 0..nenv {
            env.push(self.expr()?);
        }
        Ok(Node::Lam { stub, arity, env })
    }

    fn expr_ctor(&mut self) -> Result<Node> {
        let tag = self.word()? as i64;
        let n = self.count_word()?;
        let mut fields = Vec::with_capacity(n);
        for _ in 0..n {
            fields.push(self.expr()?);
        }
        Ok(Node::Ctor { tag, fields })
    }

    fn expr_proj(&mut self) -> Result<Node> {
        let value = self.expr()?;
        let mode = self.word()?;
        match mode {
            0 => {
                let slot = self.word()?;
                Ok(Node::Proj {
                    value: Box::new(value),
                    mode,
                    slot,
                    offset: 0,
                    size: 0,
                    signed: false,
                })
            }
            1 => {
                let offset = self.word()?;
                let size = self.word()?;
                if size > 8 {
                    return Err(Error::CorruptValue);
                }
                let signed = self.word()?;
                if signed > 1 {
                    return Err(Error::CorruptValue);
                }
                Ok(Node::Proj {
                    value: Box::new(value),
                    mode,
                    slot: 0,
                    offset,
                    size,
                    signed: signed == 1,
                })
            }
            2 => {
                let offset = self.word()?;
                Ok(Node::Proj {
                    value: Box::new(value),
                    mode,
                    slot: 0,
                    offset,
                    size: 0,
                    signed: false,
                })
            }
            _ => Err(Error::CorruptValue),
        }
    }

    fn expr_lazy_init(&mut self) -> Result<Node> {
        let slot = self.word()?;
        let idx = self.lookup(slot)?;
        let thunk = self.expr()?;
        Ok(Node::LazyInit(idx, Box::new(thunk)))
    }

    fn expr_demand(&mut self) -> Result<Node> {
        Ok(Node::Demand(Box::new(self.expr()?)))
    }

    /// `[scrutinee, ncon, cons..., default]`.
    fn case(&mut self) -> Result<Node> {
        let scrut = self.expr()?;
        let ncon = self.count_word()?;
        let mut arms = Vec::with_capacity(ncon);
        let mut tags: Vec<u64> = Vec::with_capacity(ncon);
        for _ in 0..ncon {
            let tag = self.word()?;
            tags.push(tag);
            let nf = self.count_word()?;
            let saved = self.slots.len();
            let mut fields = Vec::with_capacity(nf);
            for _ in 0..nf {
                let slot = self.word()?;
                fields.push(self.bind(slot));
            }
            let body = self.expr();
            self.slots.truncate(saved);
            arms.push(Arm { tag, fields, body: body? });
        }
        // Unique tags, as the encoder guarantees; first-match selection
        // over duplicates would hide corrupt data.
        tags.sort_unstable();
        tags.dedup();
        if tags.len() != arms.len() {
            return Err(Error::CorruptValue);
        }
        let alt = match self.word()? {
            0 => Alt::None,
            1 => Alt::Wild(Box::new(self.expr()?)),
            2 => {
                let slot = self.word()?;
                let saved = self.slots.len();
                let idx = self.bind(slot);
                let body = self.expr();
                self.slots.truncate(saved);
                Alt::Var(idx, Box::new(body?))
            }
            _ => return Err(Error::CorruptValue),
        };
        Ok(Node::Case(Box::new(scrut), arms, alt))
    }

    /// `[nat, ndigits, digits...]`: unpack the digit bytes and build the
    /// bignum word through the crate's own nat/int machinery. The bytes
    /// are parsed once through `from_decimal` so a malformed numeral is
    /// an `Err`, and once more through the `cinch_*_decimal` entry point,
    /// which then cannot fail, to obtain the word in the runtime's own
    /// block layout.
    fn big(&mut self) -> Result<Node> {
        let nat = self.word()?;
        if nat > 1 {
            return Err(Error::CorruptValue);
        }
        let nd = usize::try_from(self.word()?).map_err(|_| Error::CorruptValue)?;
        if nd == 0 {
            return Err(Error::MalformedDecimal);
        }
        let nwords = nd.checked_add(7).ok_or(Error::CorruptValue)? / 8;
        let ws = self.tail(nwords)?;
        let mut digits: Vec<u8> = Vec::with_capacity(nd);
        for w in ws {
            for i in 0..8 {
                let byte = ((w >> (8 * i)) & 0xFF) as u8;
                if digits.len() < nd {
                    digits.push(byte);
                } else if byte != 0 {
                    // The encoder's chunking loop leaves the padding
                    // bytes of the last word zero.
                    return Err(Error::CorruptValue);
                }
            }
        }
        let word = if nat == 1 {
            crate::Nat::from_decimal(&digits)?;
            unsafe { crate::abi::cinch_nat_from_decimal(digits.as_ptr(), digits.len()) }
        } else {
            crate::Int::from_decimal(&digits)?;
            unsafe { crate::abi::cinch_int_from_decimal(digits.as_ptr(), digits.len()) }
        };
        Ok(Node::Big(word))
    }
}

// ------------------------------------------------------------ the evaluator

/// The closure apply-stub ABI (`compiler/src/core.rs`; see
/// `crate::task::apply_closure`): word 0 of a closure block is called
/// with the closure itself, the argument count and the argument vector.
type ApplyStub = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;

/// Apply a closure through its stub. Under- and over-application are
/// the stub's business, exactly as in generated code.
unsafe fn apply(clo: i64, argv: &mut [i64]) -> i64 {
    let code = unsafe { *(clo as *const i64) };
    let stub: ApplyStub = unsafe { std::mem::transmute(code as usize) };
    unsafe { stub(clo, argv.len() as i64, argv.as_mut_ptr()) }
}

/// Apply a closure and surface any fault the applied code raised (the
/// re-entrant channel of the module docs).
fn apply_checked(clo: i64, argv: &mut [i64]) -> Result<i64> {
    let v = unsafe { apply(clo, argv) };
    take_pending()?;
    Ok(v)
}

/// Evaluate one decoded node against the environment (dense indices) of
/// its quote block, with that block's hole words. Like the decoder, the
/// node cases live in small helpers so debug builds keep one small frame
/// per recursion step (see [`Frame`]).
fn eval(node: &Node, env: &mut [i64], holes: &[i64]) -> Result<i64> {
    let _f = Frame::new()?;
    match node {
        Node::Int(v) | Node::Big(v) | Node::Bool(v) | Node::UnitRef(v) => Ok(*v),
        Node::Unit => Ok(0),
        Node::Local(i) => Ok(env[*i as usize]),
        Node::Hole(i) => run(holes[*i as usize]),
        Node::Let(i, value, body) => eval_let(*i, value, body, env, holes),
        Node::If(c, t, e) => eval_if(c, t, e, env, holes),
        Node::Case(scrut, arms, alt) => eval_case(scrut, arms, alt, env, holes),
        Node::App(f, args) => eval_app(f, args, env, holes),
        Node::Lam { stub, arity, env: captured } => {
            eval_lam(*stub, *arity, captured, env, holes)
        }
        Node::Ctor { tag, fields } => eval_ctor(*tag, fields, env, holes),
        Node::Proj { value, mode, slot, offset, size, signed } => {
            eval_proj(value, *mode, *slot, *offset, *size, *signed, env, holes)
        }
        // The program's own fault is the `BlackHole` fault path (exit 70
        // at `cinch_eval`), the same fault the generated
        // `__cinch_blackhole` takes.
        Node::Fault => Err(Error::BlackHole),
        Node::LazyAlloc => Ok(unsafe { crate::abi::cinch_alloc(24) }),
        Node::LazyInit(i, thunk) => eval_lazy_init(*i, thunk, env, holes),
        Node::Demand(cell) => eval_demand(cell, env, holes),
    }
}

fn eval_let(
    slot: u32,
    value: &Node,
    body: &Node,
    env: &mut [i64],
    holes: &[i64],
) -> Result<i64> {
    let v = eval(value, env, holes)?;
    env[slot as usize] = v;
    eval(body, env, holes)
}

fn eval_if(c: &Node, t: &Node, e: &Node, env: &mut [i64], holes: &[i64]) -> Result<i64> {
    let cond = eval(c, env, holes)?;
    // The generated lowering branches on `icmp ne i64 %c, 0`.
    if cond != 0 {
        eval(t, env, holes)
    } else {
        eval(e, env, holes)
    }
}

fn eval_case(
    scrut: &Node,
    arms: &[Arm],
    alt: &Alt,
    env: &mut [i64],
    holes: &[i64],
) -> Result<i64> {
    let w = eval(scrut, env, holes)?;
    let p = block_ptr(w)?;
    let tag = unsafe { word_at(p, 0) } as u64;
    if let Some(arm) = arms.iter().find(|a| a.tag == tag) {
        for (i, slot) in arm.fields.iter().enumerate() {
            let off = 8usize.checked_mul(1 + i).ok_or(Error::CorruptValue)?;
            env[*slot as usize] = unsafe { word_at(p, off) };
        }
        return eval(&arm.body, env, holes);
    }
    match alt {
        // A well-typed quotation's matches are exhaustive; reaching
        // here means corrupt data.
        Alt::None => Err(Error::CorruptValue),
        Alt::Wild(body) => eval(body, env, holes),
        Alt::Var(i, body) => {
            env[*i as usize] = w;
            eval(body, env, holes)
        }
    }
}

fn eval_app(f: &Node, args: &[Node], env: &mut [i64], holes: &[i64]) -> Result<i64> {
    let clo = eval(f, env, holes)?;
    block_ptr(clo)?;
    let mut argv = Vec::with_capacity(args.len().max(1));
    for a in args {
        argv.push(eval(a, env, holes)?);
    }
    apply_checked(clo, &mut argv)
}

fn eval_lam(
    stub: i64,
    arity: u64,
    captured: &[Node],
    env: &mut [i64],
    holes: &[i64],
) -> Result<i64> {
    let mut saved = Vec::with_capacity(captured.len());
    for e in captured {
        saved.push(eval(e, env, holes)?);
    }
    let words = 3 + saved.len();
    let clo = unsafe { crate::abi::cinch_alloc((words * 8) as i64) };
    let p = block_ptr(clo)?;
    unsafe {
        word_set(p, 0, stub);
        word_set(p, 8, arity as i64);
        word_set(p, 16, saved.len() as i64);
        for (i, v) in saved.iter().enumerate() {
            word_set(p, 24 + 8 * i, *v);
        }
    }
    Ok(clo)
}

fn eval_ctor(tag: i64, fields: &[Node], env: &mut [i64], holes: &[i64]) -> Result<i64> {
    let mut vals = Vec::with_capacity(fields.len());
    for f in fields {
        vals.push(eval(f, env, holes)?);
    }
    let words = 1 + vals.len();
    let block = unsafe { crate::abi::cinch_alloc((words * 8) as i64) };
    let p = block_ptr(block)?;
    unsafe {
        word_set(p, 0, tag);
        for (i, v) in vals.iter().enumerate() {
            word_set(p, 8 * (1 + i), *v);
        }
    }
    Ok(block)
}

fn eval_proj(
    value: &Node,
    mode: u64,
    slot: u64,
    offset: u64,
    size: u64,
    signed: bool,
    env: &mut [i64],
    holes: &[i64],
) -> Result<i64> {
    let w = eval(value, env, holes)?;
    let p = block_ptr(w)?;
    match mode {
        0 => {
            let off = slot
                .checked_add(1)
                .and_then(|s| s.checked_mul(8))
                .ok_or(Error::CorruptValue)?;
            let off = usize::try_from(off).map_err(|_| Error::CorruptValue)?;
            Ok(unsafe { word_at(p, off) })
        }
        1 | 2 => {
            let off = offset.checked_add(8).ok_or(Error::CorruptValue)?;
            let off = usize::try_from(off).map_err(|_| Error::CorruptValue)?;
            if mode == 1 {
                Ok(unsafe { scalar_at(p, off, size, signed) })
            } else {
                Ok(unsafe { word_at(p, off) })
            }
        }
        _ => Err(Error::CorruptValue),
    }
}

fn eval_lazy_init(slot: u32, thunk: &Node, env: &mut [i64], holes: &[i64]) -> Result<i64> {
    let tv = eval(thunk, env, holes)?;
    let p = block_ptr(env[slot as usize])?;
    unsafe { word_set(p, 8, tv) };
    Ok(0)
}

fn eval_demand(cell: &Node, env: &mut [i64], holes: &[i64]) -> Result<i64> {
    let w = eval(cell, env, holes)?;
    let p = block_ptr(w)?;
    // The state machine of the generated `__cinch_demand` helper
    // (`MEMO_HELPERS` in `compiler/src/codegen.rs`).
    match unsafe { word_at(p, 0) } {
        0 => {
            unsafe { word_set(p, 0, 1) };
            let thunk = unsafe { word_at(p, 8) };
            block_ptr(thunk)?;
            let mut argv = [0i64];
            let v = apply_checked(thunk, &mut argv)?;
            unsafe {
                word_set(p, 16, v);
                word_set(p, 0, 2);
            }
            Ok(v)
        }
        2 => Ok(unsafe { word_at(p, 16) }),
        // State 1 is a cell demanded while its own evaluation runs (or
        // any other corrupt state): the `BlackHole` fault.
        _ => Err(Error::BlackHole),
    }
}

// ------------------------------------------------------------ the entry points

/// Evaluate the reified quotation in the `Code` block `quote` to its
/// value word (the safe Rust API of [`cinch_eval`]).
///
/// The quote block and its whole program stream are validated and
/// decoded before anything is evaluated; malformed contents are an
/// [`Error`] (never a value of the quoted type), and nesting past
/// [`MAX_NESTING`] is [`Error::NestingTooDeep`]. See the module docs
/// for the block layout, the opcode table and the fault exit codes.
pub fn run(quote: i64) -> Result<i64, Error> {
    let outer = TLS.with(|t| {
        let mut s = t.get();
        let o = s.runs;
        s.runs += 1;
        t.set(s);
        o
    });
    let r = run_body(quote);
    TLS.with(|t| {
        let mut s = t.get();
        s.runs -= 1;
        if let Err(e) = r {
            if outer == 0 {
                // The caller sees the error directly; leave no stale
                // pending fault behind.
                s.pending = None;
            } else if s.pending.is_none() {
                // Re-entered from applied code: record the fault for the
                // enclosing evaluation to surface.
                s.pending = Some(e);
            }
        }
        t.set(s);
    });
    r
}

/// Validate the quote block, decode its program, evaluate it.
fn run_body(quote: i64) -> Result<i64> {
    let _f = Frame::new()?;
    let p = block_ptr(quote)?;
    if unsafe { word_at(p, 0) } != KIND_CODE {
        return Err(Error::CorruptValue);
    }
    let code = unsafe { word_at(p, 8) };
    if code == 0 || code & 7 != 0 {
        return Err(Error::CorruptValue);
    }
    let len = count(unsafe { word_at(p, 16) }, 1, MAX_CODE_WORDS)?;
    let nh = count(unsafe { word_at(p, 24) }, 0, MAX_HOLES)?;
    let words: &[u64] = unsafe { std::slice::from_raw_parts(code as *const u64, len) };
    let holes: &[i64] = unsafe { std::slice::from_raw_parts(p.add(4), nh) };
    let mut dec = Dec::new(words, nh);
    let root = dec.expr()?;
    if dec.pos != words.len() {
        // The encoder emits exactly one expression per program.
        return Err(Error::CorruptValue);
    }
    let mut env = vec![0i64; dec.next as usize];
    eval(&root, &mut env, holes)
}

/// A header count operand: `min..=max` or corrupt data.
fn count(word: i64, min: u64, max: u64) -> Result<usize> {
    let n = word as u64;
    if n < min || n > max {
        return Err(Error::CorruptValue);
    }
    usize::try_from(n).map_err(|_| Error::CorruptValue)
}

/// Evaluate a reified quotation for generated code (`eval : Code a -> a`,
/// supercompilation and JIT specification section 7).
///
/// This is the `extern "C"` face of [`run`], with the crate's word
/// conventions (see the crate docs). A failing evaluation produces no
/// value: the quoted program's own fault and a black-holed memo cell
/// take the `BlackHole` path ([`crate::abi::cinch_blackhole`], exit 70),
/// every other failure takes the `cinch_panic` path (exit 71) with a
/// message naming eval. A bogus non-null `quote` word is a caller bug
/// and faults like any invalid pointer dereference; every corrupt
/// quotation this evaluator can name is a defined fault instead.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_eval(quote: i64) -> i64 {
    match run(quote) {
        Ok(v) => v,
        Err(Error::BlackHole) => crate::abi::cinch_blackhole(),
        Err(e) => {
            let msg = format!("cinch_eval: {e}");
            unsafe { crate::abi::cinch_panic(msg.as_ptr(), msg.len()) }
        }
    }
}

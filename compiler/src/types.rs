//! Type representations, unification and generalization used by the checker.
//!
//! This is our equivalent of Hindley-Milner with row typing:
//!
//! - Types unify with an occurs check; metavariables stand for unknowns.
//! - Effect rows are sets of labels plus a tail, where the tail is closed,
//!   a rigid row variable from a `forall`, or a row metavariable.
//! - Row unification supports extension (one row absorbs the other's extra
//!   labels through its tail) and row *lacks* constraints: a tail may be
//!   required to lack a label, which is how the memo black-hole rule is
//!   enforced (`Eval r` must not appear in a cell body's row).
//! - Schemes quantify type and row variables; instantiation replaces them
//!   with fresh metavariables, generalization quantifies the metavariables
//!   that are not free in the environment.

use crate::intern::Interned;
use crate::spelling::{Spell, Spelling};
use std::collections::HashMap;
use std::fmt;

/// The machine scalar primitives (primitive types specification, section 2).
///
/// Every scalar is one `i64` word at runtime (`core.rs`): signed integers
/// are kept sign-extended to 64 bits, unsigned integers zero-extended,
/// `F32` holds the IEEE binary32 bits zero-extended in the low 32, `F64`
/// the binary64 bits, and `Char` a Unicode scalar value.
///
/// `Usize` and `Isize` take their width from the target profile. This
/// bootstrap compiler targets 64-bit platforms only (the runtime word is
/// `i64` everywhere), so they are 64 bits here; a wasm32 profile would make
/// them 32 bits and is out of scope for this slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Scalar {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Char,
    Usize,
    Isize,
}

impl Scalar {
    /// Every scalar, in declaration order.
    pub const ALL: [Scalar; 13] = [
        Scalar::I8,
        Scalar::I16,
        Scalar::I32,
        Scalar::I64,
        Scalar::U8,
        Scalar::U16,
        Scalar::U32,
        Scalar::U64,
        Scalar::F32,
        Scalar::F64,
        Scalar::Char,
        Scalar::Usize,
        Scalar::Isize,
    ];

    /// The scalars integer arithmetic, bit operations and integer
    /// conversions range over: everything but `F32`, `F64` and `Char`
    /// (primitive types specification, section 4).
    pub const INT_LIKE: [Scalar; 10] = [
        Scalar::I8,
        Scalar::I16,
        Scalar::I32,
        Scalar::I64,
        Scalar::U8,
        Scalar::U16,
        Scalar::U32,
        Scalar::U64,
        Scalar::Usize,
        Scalar::Isize,
    ];

    pub const FLOATS: [Scalar; 2] = [Scalar::F32, Scalar::F64];

    /// The type's name as written in source, e.g. `U32` or `Usize`.
    pub fn name(self) -> &'static str {
        use Scalar::*;
        match self {
            I8 => "I8",
            I16 => "I16",
            I32 => "I32",
            I64 => "I64",
            U8 => "U8",
            U16 => "U16",
            U32 => "U32",
            U64 => "U64",
            F32 => "F32",
            F64 => "F64",
            Char => "Char",
            Usize => "Usize",
            Isize => "Isize",
        }
    }

    /// The lowercase spelling used in total conversion names (`u8ToI32`).
    pub fn lower_name(self) -> &'static str {
        use Scalar::*;
        match self {
            I8 => "i8",
            I16 => "i16",
            I32 => "i32",
            I64 => "i64",
            U8 => "u8",
            U16 => "u16",
            U32 => "u32",
            U64 => "u64",
            F32 => "f32",
            F64 => "f64",
            Char => "char",
            Usize => "usize",
            Isize => "isize",
        }
    }

    pub fn parse(name: &str) -> Option<Scalar> {
        use Scalar::*;
        Some(match name {
            "I8" => I8,
            "I16" => I16,
            "I32" => I32,
            "I64" => I64,
            "U8" => U8,
            "U16" => U16,
            "U32" => U32,
            "U64" => U64,
            "F32" => F32,
            "F64" => F64,
            "Char" => Char,
            "Usize" => Usize,
            "Isize" => Isize,
            _ => return None,
        })
    }

    /// Width in bits (`Char` is 32 like `U32`).
    pub fn width(self) -> u32 {
        use Scalar::*;
        match self {
            I8 | U8 => 8,
            I16 | U16 => 16,
            I32 | U32 | F32 => 32,
            I64 | U64 | F64 | Usize | Isize => 64,
            Char => 32,
        }
    }

    /// Size in bytes (primitive types specification section 2: the
    /// layout table every target must honor).
    pub fn size(self) -> u64 {
        self.width() as u64 / 8
    }

    /// Alignment in bytes (equals the size for every primitive type).
    pub fn align(self) -> u64 {
        self.size()
    }

    pub fn is_float(self) -> bool {
        matches!(self, Scalar::F32 | Scalar::F64)
    }

    pub fn is_int_like(self) -> bool {
        !self.is_float() && self != Scalar::Char
    }

    /// Two's complement signed order for `I*`/`Isize`; unsigned (or code
    /// point) order otherwise.
    pub fn is_signed(self) -> bool {
        use Scalar::*;
        match self {
            I8 | I16 | I32 | I64 | Isize => true,
            U8 | U16 | U32 | U64 | Usize | Char | F32 | F64 => false,
        }
    }

    /// The smallest value as its canonical `i64` word.
    pub fn min_bound_word(self) -> i64 {
        use Scalar::*;
        match self {
            I8 => -128,
            I16 => -32768,
            I32 => -2147483648,
            I64 | Isize => i64::MIN,
            U8 | U16 | U32 | U64 | Usize | Char => 0,
            F32 | F64 => panic!("floats have no bounds"),
        }
    }

    /// The largest value as its canonical `i64` word (`maxBoundU64` is the
    /// all-ones bit pattern, which reads back as `-1` as an `i64`).
    pub fn max_bound_word(self) -> i64 {
        use Scalar::*;
        match self {
            I8 => 127,
            I16 => 32767,
            I32 => 2147483647,
            I64 | Isize => i64::MAX,
            U8 => 255,
            U16 => 65535,
            U32 => 4294967295,
            U64 | Usize => -1,
            Char => 0x10FFFF,
            F32 | F64 => panic!("floats have no bounds"),
        }
    }

    /// Does every value of `self` fit `to` (integer types only)?
    pub fn fits(self, to: Scalar) -> bool {
        debug_assert!(self.is_int_like() && to.is_int_like());
        if self == to {
            return true;
        }
        let sw = self.width();
        let tw = to.width();
        match (self.is_signed(), to.is_signed()) {
            // Same signedness: equal widths are fine too (I64 and Isize
            // are both 64 bits on this target profile).
            (false, false) | (true, true) => sw <= tw,
            (false, true) => sw < tw,
            (true, false) => false,
        }
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MetaId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RowMetaId(pub usize);

/// A usage multiplicity on an arrow's parameter (ownership and usage
/// design, section 2): erased, linear, affine or unrestricted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Mult {
    /// `%0`: never used at runtime.
    Zero,
    /// `%1`: used exactly once on every evaluation path.
    One,
    /// `%aff`: used at most once (dropping runs the destructor).
    Aff,
    /// `%many`: used any number of times.
    Many,
    /// Internal: an inference-created arrow whose parameter multiplicity
    /// is not yet fixed. Unifies with anything; binders facing it take the
    /// type-directed default ([`default_mult`]).
    Flex,
}

impl Mult {
    /// The surface spelling, or `None` for the internal `Flex`.
    pub fn spelling(self) -> Option<&'static str> {
        match self {
            Mult::Zero => Some("%0"),
            Mult::One => Some("%1"),
            Mult::Aff => Some("%aff"),
            Mult::Many => Some("%many"),
            Mult::Flex => None,
        }
    }

    /// Parse a surface multiplicity spelling (`%0`, `%1`, `%aff`, `%many`).
    pub fn parse(name: &str) -> Option<Mult> {
        match name {
            "%0" => Some(Mult::Zero),
            "%1" => Some(Mult::One),
            "%aff" => Some(Mult::Aff),
            "%many" => Some(Mult::Many),
            _ => None,
        }
    }
}

impl fmt::Display for Mult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.spelling() {
            Some(s) => write!(f, "{s}"),
            None => write!(f, "%many"),
        }
    }
}

/// One effect label, e.g. `IO` or `Eval r` (a label with one region argument).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffLabel {
    pub head: Interned,
    pub arg: Option<Interned>,
}

impl EffLabel {
    pub fn plain(head: impl Into<Interned>) -> EffLabel {
        EffLabel { head: head.into(), arg: None }
    }

    pub fn with_arg(head: impl Into<Interned>, arg: impl Into<Interned>) -> EffLabel {
        EffLabel { head: head.into(), arg: Some(arg.into()) }
    }
}

impl fmt::Display for EffLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.arg {
            Some(a) => write!(f, "{} {}", self.head, a),
            None => write!(f, "{}", self.head),
        }
    }
}

/// The tail of an effect row: closed, a rigid row variable from a
/// signature, or a row metavariable.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum RowTail {
    /// No tail: the row is exactly its labels.
    #[default]
    Closed,
    /// A rigid row variable named by a `forall` (`{|e}` in a signature).
    Rigid(Interned),
    /// An inference variable.
    Meta(RowMetaId),
}

/// An effect row: a set of labels plus a tail.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EffRow {
    pub labels: Vec<EffLabel>,
    pub tail: RowTail,
}

impl EffRow {
    pub fn pure() -> EffRow {
        EffRow::default()
    }

    pub fn is_pure(&self) -> bool {
        self.labels.is_empty() && self.tail == RowTail::Closed
    }

    pub fn closed(labels: Vec<EffLabel>) -> EffRow {
        EffRow { labels, tail: RowTail::Closed }
    }

    pub fn contains(&self, label: &EffLabel) -> bool {
        self.labels.contains(label)
    }

    /// Rename rigid region arguments, row variables and tails.
    pub fn rename(&self, map: &HashMap<String, String>) -> EffRow {
        EffRow {
            labels: self
                .labels
                .iter()
                .map(|l| EffLabel {
                    head: l.head.clone(),
                    arg: l.arg.as_ref().map(|a| map.get(a.as_str()).cloned().unwrap_or_else(|| a.to_string()).into()),
                })
                .collect(),
            tail: match &self.tail {
                RowTail::Rigid(t) => {
                    RowTail::Rigid(map.get(t.as_str()).cloned().unwrap_or_else(|| t.to_string()).into())
                }
                other => other.clone(),
            },
        }
    }
}

impl fmt::Display for EffRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, l) in self.labels.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{l}")?;
        }
        match &self.tail {
            RowTail::Closed => {}
            RowTail::Rigid(t) => {
                if !self.labels.is_empty() {
                    write!(f, " | ")?;
                }
                write!(f, "{t}")?;
            }
            RowTail::Meta(RowMetaId(m)) => {
                if !self.labels.is_empty() {
                    write!(f, " | ")?;
                }
                write!(f, "?r{m}")?;
            }
        }
        write!(f, "}}")
    }
}

// ------------------------------------------------- dependent core vocabulary

/// A universe: `Type i` is the universe of computational types at level
/// `i`, `Prop` the universe of erased propositions (dependent core kernel
/// design, section 2). `Type i : Type (i + 1)`, so `Type : Type` is never
/// admitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sort {
    Type(u32),
    Prop,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sort::Type(0) => write!(f, "Type"),
            Sort::Type(n) => write!(f, "Type {n}"),
            Sort::Prop => write!(f, "Prop"),
        }
    }
}

/// Mathematical `Nat`/`Int` arithmetic in type-level positions
/// (`Vec (k + 1) a`): these reduce on numerals only (dependent core kernel
/// design, section 4), never through class dictionaries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdxOp {
    Add,
    Sub,
    Mul,
}

impl fmt::Display for IdxOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdxOp::Add => write!(f, "+"),
            IdxOp::Sub => write!(f, "-"),
            IdxOp::Mul => write!(f, "*"),
        }
    }
}

/// Comparison on index values; the proposition `a < b` and friends
/// (`Eq x y` has its own form because it is type-parametric).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdxCmp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl fmt::Display for IdxCmp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdxCmp::Eq => write!(f, "=="),
            IdxCmp::Ne => write!(f, "/="),
            IdxCmp::Lt => write!(f, "<"),
            IdxCmp::Le => write!(f, "<="),
            IdxCmp::Gt => write!(f, ">"),
            IdxCmp::Ge => write!(f, ">="),
        }
    }
}

/// A semantic primitive key, outside the source module-name grammar.
/// Only checked `BytesOp::Len` declarations may introduce it. Public
/// diagnostic rendering keeps the ordinary base-library spelling.
pub(crate) const BYTE_LENGTH_INDEX: &str = "<intrinsic>.bytes-length";

/// A first-order value embedded in a type (an index): variables shared
/// with term syntax (the surface shares term syntax between kinds, types
/// and values), numerals, mathematical arithmetic, and applications of
/// named total definitions. Inference variables support implicit-argument
/// solving (design section 2: elaboration is insert-and-solve).
#[derive(Clone, Debug, PartialEq)]
pub enum ITerm {
    Var(Interned),
    /// A checked runtime value, independent of spelling and of the dictionary
    /// namespace. Only elaboration can introduce this local identity.
    LocalValue(crate::core::LocalId),
    /// Checked dictionary identity. Descriptive value names cannot construct
    /// this namespace or impersonate a dictionary binder.
    Evidence(IndexEvidence),
    Num(Interned),
    /// A named total definition used as a function value (the head of an
    /// index-level application).
    Fn(Interned),
    /// A checked global instantiation. Type arguments and runtime evidence
    /// belong to the index's identity, even when the visible arguments agree.
    /// The visible source application remains an outer `App`, so dictionary
    /// and implicit-value arguments never change its source arity.
    Inst(Box<IndexInstantiation>),
    /// `\x -> body`, an index-level lambda (a lifting function like
    /// `congr`'s, or a type-position lambda).
    Lam(Interned, Box<ITerm>),
    App(Box<ITerm>, Vec<ITerm>),
    Bin(IdxOp, Box<ITerm>, Box<ITerm>),
    MVar(usize),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IndexEvidence {
    Parameter(String),
    Local(usize),
}

#[derive(Clone, Copy)]
enum IndexSubstitution<'a> {
    Value(&'a str, &'a ITerm),
    Evidence(&'a HashMap<IndexEvidence, ITerm>),
    Inference(&'a HashMap<usize, ITerm>),
    LocalValues(&'a HashMap<crate::core::LocalId, ITerm>),
}

impl IndexSubstitution<'_> {
    fn shadows_value(self, name: &str) -> bool {
        matches!(self, Self::Value(bound, _) if bound == name)
    }
    fn captures_value(self, name: &str) -> bool {
        match self {
            Self::Value(_, value) => value.mentions(name),
            Self::Evidence(values) => values.values().any(|value| value.mentions(name)),
            Self::Inference(values) => values.values().any(|value| value.mentions(name)),
            Self::LocalValues(values) => values.values().any(|value| value.mentions(name)),
        }
    }
    fn replacement(self, term: &ITerm) -> Option<ITerm> {
        match (self, term) {
            (Self::Value(name, value), ITerm::Var(candidate)) if candidate == name => Some(value.clone()),
            (Self::Evidence(values), ITerm::Evidence(binding)) => values.get(binding).cloned(),
            (Self::Inference(values), ITerm::MVar(variable)) => values.get(variable).cloned(),
            (Self::LocalValues(values), ITerm::LocalValue(local)) => values.get(local).cloned(),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IndexHead {
    Function(String),
    Dictionary(String),
    Marker,
    /// A provisional declaration pass has not selected this dictionary.
    /// Strict elaboration must reconstruct the header without this node.
    DeclarationPending(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexInstantiation {
    pub head: IndexHead,
    pub types: Vec<Ty>,
    pub dictionaries: Vec<ITerm>,
    pub arguments: Vec<IndexArgument>,
}

/// The non-type portion of a checked global's quantified telescope. Value
/// carriers select their actual runtime representation; erased arguments still
/// contribute to type identity, substitution and metavariable ownership.
#[derive(Clone, Debug, PartialEq)]
pub enum IndexArgument {
    Value { value: ITerm, carrier: Ty, runtime: bool },
    Proposition(PTerm),
    Row(EffRow),
    Region(Interned),
    Evidence(ITerm),
}

impl IndexArgument {
    pub fn kind(&self) -> u8 {
        match self { Self::Value { runtime: false, .. } => 0, Self::Value { runtime: true, .. } => 1,
            Self::Proposition(_) => 2, Self::Row(_) => 3, Self::Region(_) => 4, Self::Evidence(_) => 5 }
    }

    /// Reuse the native type walkers for every namespace. The effect row's
    /// ordinary function carrier is only a traversal view, never runtime code.
    pub fn as_type(&self) -> Ty {
        match self {
            Self::Value { value, carrier, .. } => Ty::Tuple(vec![carrier.clone(), Ty::Idx(Box::new(value.clone()))]),
            Self::Proposition(prop) => Ty::Prp(Box::new(prop.clone())),
            Self::Row(row) => Ty::fun_eff(Ty::Unit, row.clone(), Ty::Unit),
            Self::Region(region) => Ty::Region(region.clone()),
            Self::Evidence(value) => Ty::Idx(Box::new(value.clone())),
        }
    }

    fn map<T: FnMut(&Ty) -> Ty + ?Sized, U: FnMut(&ITerm) -> ITerm + ?Sized>(&self, ty: &mut T, term: &mut U) -> Self {
        match self {
            Self::Value { value, carrier, runtime } => Self::Value { value: term(value), carrier: ty(carrier), runtime: *runtime },
            Self::Evidence(value) => Self::Evidence(term(value)),
            Self::Proposition(_) => {
                let Ty::Prp(prop) = ty(&self.as_type()) else { unreachable!("type traversal preserves proposition shape") };
                Self::Proposition(*prop)
            }
            Self::Row(_) => {
                let Ty::Fun(_, _, row, _) = ty(&self.as_type()) else { unreachable!("type traversal preserves row carrier shape") };
                Self::Row(row)
            }
            Self::Region(_) => {
                let Ty::Region(region) = ty(&self.as_type()) else { unreachable!("type traversal preserves region shape") };
                Self::Region(region)
            }
        }
    }

    fn map_types(&self, map: &mut impl FnMut(&Ty) -> Ty) -> Self {
        match self {
            Self::Value { value, carrier, runtime } => {
                let carrier = map(carrier);
                Self::Value { value: value.map_types(map), carrier, runtime: *runtime }
            }
            Self::Evidence(value) => Self::Evidence(value.map_types(map)),
            _ => self.map(map, &mut |term| term.clone()),
        }
    }
}

impl IndexInstantiation {
    pub fn runtime_values(&self) -> impl Iterator<Item = (&ITerm, &Ty)> {
        self.arguments.iter().filter_map(|argument| match argument {
            IndexArgument::Value { value, carrier, runtime: true } => Some((value, carrier)), _ => None,
        })
    }
    pub fn map(&self, ty: &mut impl FnMut(&Ty) -> Ty, term: &mut impl FnMut(&ITerm) -> ITerm) -> Self {
        Self {
            head: self.head.clone(),
            types: self.types.iter().map(&mut *ty).collect(),
            dictionaries: self.dictionaries.iter().map(&mut *term).collect(),
            arguments: self.arguments.iter().map(|argument| argument.map(ty, term)).collect(),
        }
    }

    pub fn terms(&self) -> impl Iterator<Item = &ITerm> {
        self.dictionaries.iter().chain(self.arguments.iter().filter_map(|argument| match argument {
            IndexArgument::Value { value, .. } | IndexArgument::Evidence(value) => Some(value), _ => None,
        }))
    }

    pub fn embedded_types(&self) -> impl Iterator<Item = Ty> + '_ {
        self.types.iter().cloned().chain(self.arguments.iter().filter_map(|argument| match argument {
            IndexArgument::Value { carrier, .. } => Some(carrier.clone()),
            IndexArgument::Evidence(_) => None,
            _ => Some(argument.as_type()),
        }))
    }
}

impl ITerm {
    pub fn has_declaration_pending(&self) -> bool {
        match self {
            Self::Inst(call) => matches!(call.head, IndexHead::DeclarationPending(_))
                || call.embedded_types().any(|ty| ty.has_declaration_pending())
                || call.terms().any(Self::has_declaration_pending),
            Self::Lam(_, body) => body.has_declaration_pending(),
            Self::App(head, arguments) => head.has_declaration_pending() || arguments.iter().any(Self::has_declaration_pending),
            Self::Bin(_, left, right) => left.has_declaration_pending() || right.has_declaration_pending(),
            _ => false,
        }
    }
    /// Visit embedded type arguments, including those in nested dictionaries.
    pub fn visit_types(&self, visit: &mut impl FnMut(&Ty)) {
        match self {
            ITerm::Inst(call) => {
                for ty in call.embedded_types() { visit(&ty); }
                for term in call.terms() { term.visit_types(visit); }
            }
            ITerm::App(head, args) => {
                head.visit_types(visit);
                for arg in args { arg.visit_types(visit); }
            }
            ITerm::Lam(_, body) => body.visit_types(visit),
            ITerm::Bin(_, a, b) => { a.visit_types(visit); b.visit_types(visit); }
            _ => {}
        }
    }

    pub fn map_types(&self, map: &mut impl FnMut(&Ty) -> Ty) -> Self {
        match self {
            ITerm::Inst(call) => {
                let types = call.types.iter().map(&mut *map).collect();
                let dictionaries = call.dictionaries.iter().map(|term| term.map_types(map)).collect();
                let arguments = call.arguments.iter().map(|argument| argument.map_types(map)).collect();
                ITerm::Inst(Box::new(IndexInstantiation { head: call.head.clone(), types, dictionaries, arguments }))
            }
            ITerm::App(head, args) => ITerm::App(Box::new(head.map_types(map)), args.iter().map(|arg| arg.map_types(map)).collect()),
            ITerm::Lam(name, body) => ITerm::Lam(name.clone(), Box::new(body.map_types(map))),
            ITerm::Bin(op, a, b) => ITerm::Bin(*op, Box::new(a.map_types(map)), Box::new(b.map_types(map))),
            _ => self.clone(),
        }
    }

    pub fn num(n: i128) -> ITerm {
        ITerm::Num(n.to_string().into())
    }

    /// A named application: `f a b`.
    pub fn app_fn(head: impl Into<String>, args: Vec<ITerm>) -> ITerm {
        // A nullary reference is the bare head: `App(f, [])` and `f`
        // denote the same index value (a `law` equation over a nullary
        // method like `mempty` must unify with a term's plain
        // reference).
        if args.is_empty() {
            ITerm::Fn(head.into().into())
        } else {
            ITerm::App(Box::new(ITerm::Fn(head.into().into())), args)
        }
    }

    /// Free value names, in first-appearance order.
    pub fn free_vars(&self, out: &mut Vec<String>) {
        match self {
            ITerm::Var(n) => {
                if !out.iter().any(|present| present.as_str() == n.as_str()) {
                    out.push(n.to_string());
                }
            }
            ITerm::LocalValue(_) | ITerm::Evidence(_) | ITerm::Num(_) | ITerm::Fn(_) | ITerm::MVar(_) => {}
            ITerm::Inst(call) => {
                for ty in call.embedded_types() { ty.free_index_vars(out); }
                for term in call.terms() { term.free_vars(out); }
            }
            ITerm::Lam(x, body) => {
                let mut inner = Vec::new();
                body.free_vars(&mut inner);
                for v in inner {
                    if v != *x && !out.contains(&v) {
                        out.push(v);
                    }
                }
            }
            ITerm::App(f, args) => {
                f.free_vars(out);
                for a in args {
                    a.free_vars(out);
                }
            }
            ITerm::Bin(_, a, b) => {
                a.free_vars(out);
                b.free_vars(out);
            }
        }
    }

    pub fn mentions(&self, name: &str) -> bool {
        let mut out = Vec::new();
        self.free_vars(&mut out);
        out.iter().any(|v| v == name)
    }

    pub fn free_mvars(&self, out: &mut Vec<usize>) {
        match self {
            ITerm::MVar(m) => {
                if !out.contains(m) {
                    out.push(*m);
                }
            }
            ITerm::Var(_) | ITerm::LocalValue(_) | ITerm::Evidence(_) | ITerm::Num(_) | ITerm::Fn(_) => {}
            ITerm::Inst(call) => {
                for ty in call.embedded_types() { ty.free_idx_metas(out); }
                for term in call.terms() { term.free_mvars(out); }
            }
            ITerm::Lam(_, body) => body.free_mvars(out),
            ITerm::App(f, args) => {
                f.free_mvars(out);
                for a in args {
                    a.free_mvars(out);
                }
            }
            ITerm::Bin(_, a, b) => {
                a.free_mvars(out);
                b.free_mvars(out);
            }
        }
    }

    /// Substitute a value name (beta at the index level): capture
    /// avoiding under lambdas (a shadowed binder stops the walk; a
    /// binder the substitution would capture renames apart).
    pub fn subst(&self, name: &str, by: &ITerm) -> ITerm {
        self.substitute(IndexSubstitution::Value(name, by))
    }

    pub fn subst_evidence(&self, bindings: &HashMap<IndexEvidence, ITerm>) -> ITerm {
        self.substitute(IndexSubstitution::Evidence(bindings))
    }

    pub fn subst_local_values(&self, bindings: &HashMap<crate::core::LocalId, ITerm>) -> ITerm {
        self.substitute(IndexSubstitution::LocalValues(bindings))
    }

    pub fn local_values(&self, out: &mut Vec<crate::core::LocalId>) {
        match self {
            Self::LocalValue(local) => { if !out.contains(local) { out.push(*local); } }
            Self::Inst(call) => {
                for ty in call.embedded_types() { ty.local_values(out); }
                for term in call.terms() { term.local_values(out); }
            }
            Self::Lam(_, body) => body.local_values(out),
            Self::App(head, args) => { head.local_values(out); for arg in args { arg.local_values(out); } }
            Self::Bin(_, left, right) => { left.local_values(out); right.local_values(out); }
            _ => {}
        }
    }

    fn substitute(&self, substitution: IndexSubstitution<'_>) -> ITerm {
        if let Some(value) = substitution.replacement(self) { return value; }
        match self {
            ITerm::Var(_) | ITerm::LocalValue(_) | ITerm::Evidence(_) | ITerm::Num(_) | ITerm::Fn(_) | ITerm::MVar(_) => self.clone(),
            ITerm::Inst(call) => ITerm::Inst(Box::new(call.map(
                &mut |ty| ty.substitute_index(substitution), &mut |term| term.substitute(substitution),
            ))),
            ITerm::Lam(name, _) if substitution.shadows_value(name) => self.clone(),
            ITerm::Lam(name, body) => {
                let mut body = (**body).clone();
                if substitution.captures_value(name) {
                    let mut fresh = format!("{name}$sub");
                    while body.mentions(&fresh) || substitution.captures_value(&fresh) || substitution.shadows_value(&fresh) {
                        fresh.push('$');
                    }
                    body = body.subst(name, &ITerm::Var(fresh.clone().into()));
                    ITerm::Lam(fresh.into(), Box::new(body.substitute(substitution)))
                } else {
                    ITerm::Lam(name.clone(), Box::new(body.substitute(substitution)))
                }
            }
            ITerm::App(head, args) => ITerm::App(Box::new(head.substitute(substitution)), args.iter().map(|arg| arg.substitute(substitution)).collect()),
            ITerm::Bin(op, a, b) => ITerm::Bin(*op, Box::new(a.substitute(substitution)), Box::new(b.substitute(substitution))),
        }
    }

    /// Fold arithmetic on numerals (iota, dependent core kernel design
    /// section 4: mathematical operations reduce on numerals only) and
    /// beta-reduce lambda applications. Bounded by fuel, so reduction
    /// terminates by construction: a redex past the bound stays
    /// unreduced and compares syntactically.
    pub fn normalize(&self) -> ITerm {
        self.normalize_fuel(64)
    }

    fn normalize_fuel(&self, fuel: u32) -> ITerm {
        if fuel == 0 {
            return self.clone();
        }
        match self {
            ITerm::Bin(op, a, b) => {
                let a = a.normalize_fuel(fuel - 1);
                let b = b.normalize_fuel(fuel - 1);
                let (x, y) = match (&a, &b) {
                    (ITerm::Num(x), ITerm::Num(y)) => {
                        let (Ok(x), Ok(y)) = (x.parse::<i128>(), y.parse::<i128>()) else {
                            return ITerm::Bin(*op, Box::new(a), Box::new(b));
                        };
                        let v = match op {
                            IdxOp::Add => x.checked_add(y),
                            IdxOp::Sub => x.checked_sub(y),
                            IdxOp::Mul => x.checked_mul(y),
                        };
                        return match v {
                            Some(v) => ITerm::num(v),
                            None => ITerm::Bin(*op, Box::new(a), Box::new(b)),
                        };
                    }
                    _ => (a, b),
                };
                ITerm::Bin(*op, Box::new(x), Box::new(y))
            }
            ITerm::App(f, args) => {
                let f = f.normalize_fuel(fuel - 1);
                let args: Vec<ITerm> = args.iter().map(|a| a.normalize_fuel(fuel - 1)).collect();
                match f {
                    ITerm::Lam(x, body) if !args.is_empty() => {
                        let (first, rest) = args.split_first().expect("nonempty");
                        let reduced = body.subst(&x, first);
                        if rest.is_empty() {
                            reduced.normalize_fuel(fuel - 1)
                        } else {
                            ITerm::App(Box::new(reduced), rest.to_vec()).normalize_fuel(fuel - 1)
                        }
                    }
                    f => ITerm::App(Box::new(f), args),
                }
            }
            ITerm::Lam(x, body) => ITerm::Lam(x.clone(), Box::new(body.normalize_fuel(fuel - 1))),
            ITerm::Inst(call) => ITerm::Inst(Box::new(call.map(
                &mut |ty| ty.clone(), &mut |term| term.normalize_fuel(fuel - 1),
            ))),
            other => other.clone(),
        }
    }

    /// Definitional equality of index terms: beta (substitution happens at
    /// elaboration), iota on numerals, alpha for lambdas, and syntactic
    /// equality elsewhere. Stuck terms compare syntactically; conversion
    /// never unfolds user definitions (decision D28).
    pub fn eq(&self, other: &ITerm) -> bool {
        match (self.normalize(), other.normalize()) {
            (ITerm::Var(x), ITerm::Var(y)) => x == y,
            (ITerm::LocalValue(x), ITerm::LocalValue(y)) => x == y,
            (ITerm::Evidence(x), ITerm::Evidence(y)) => x == y,
            (ITerm::Num(x), ITerm::Num(y)) => x == y,
            (ITerm::Fn(x), ITerm::Fn(y)) => x == y,
            (ITerm::Inst(x), ITerm::Inst(y)) => {
                x.head == y.head && x.types.len() == y.types.len()
                    && x.embedded_types().zip(y.embedded_types()).all(|(a, b)| ty_eq(&a, &b))
                    && x.dictionaries.len() == y.dictionaries.len()
                    && x.arguments.len() == y.arguments.len() && x.arguments.iter().zip(&y.arguments).all(|(a, b)| a.kind() == b.kind())
                    && x.terms().zip(y.terms()).all(|(a, b)| a.eq(b))
            }
            (ITerm::MVar(x), ITerm::MVar(y)) => x == y,
            (ITerm::Lam(x, b), ITerm::Lam(y, c)) => {
                let mut name = "$alpha".to_string();
                while b.mentions(&name) || c.mentions(&name) || name == x || name == y { name.push('$'); }
                let fresh = ITerm::Var(name.into());
                b.subst(&x, &fresh).eq(&c.subst(&y, &fresh))
            }
            (ITerm::App(f, xs), ITerm::App(g, ys)) => {
                f.eq(&g) && xs.len() == ys.len() && xs.iter().zip(ys.iter()).all(|(x, y)| x.eq(y))
            }
            (ITerm::Bin(op, a, b), ITerm::Bin(op2, c, d)) => {
                op == op2 && a.eq(&c) && b.eq(&d)
            }
            _ => false,
        }
    }
}

impl fmt::Display for ITerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.spell(f, Spelling::Identity)
    }
}

/// Function and constructor heads are written by identity in every
/// spelling; the types embedded in an instantiation follow the spelling.
impl Spell for ITerm {
    fn spell(&self, f: &mut fmt::Formatter<'_>, s: Spelling<'_>) -> fmt::Result {
        match self {
            ITerm::Var(n) => write!(f, "{n}"),
            ITerm::LocalValue(local) => write!(f, "@value[{}]", local.0),
            ITerm::Evidence(binding) => write!(f, "@dictionary[{binding:?}]"),
            ITerm::Num(n) => write!(f, "{n}"),
            ITerm::Fn(n) => {
                // The baked measure vocabulary reads as its spelling
                // (`length xs > 100`, never `<builtin>.length xs > 100`).
                let n = if n == BYTE_LENGTH_INDEX { "Data.Bytes.bytes-len" }
                    else { n.strip_prefix("<builtin>.").unwrap_or(n) };
                write!(f, "{n}")
            }
            ITerm::MVar(m) => write!(f, "?i{m}"),
            ITerm::Inst(call) => {
                match &call.head {
                    IndexHead::Function(name) => write!(f, "{name}")?,
                    IndexHead::Dictionary(name) => write!(f, "dictionary[{name}]")?,
                    IndexHead::Marker => write!(f, "Copy-evidence")?,
                    IndexHead::DeclarationPending(id) => write!(f, "<declaration-evidence:{id}>")?,
                }
                for ty in call.embedded_types() { write!(f, " @({})", s.of(&ty))?; }
                for dictionary in &call.dictionaries { write!(f, " @{{{}}}", s.of(dictionary))?; }
                for argument in &call.arguments { write!(f, " @{}({})", argument.kind(), s.of(&argument.as_type()))?; }
                Ok(())
            }
            ITerm::Lam(x, body) => write!(f, "(\\{x} -> {})", s.of(&**body)),
            ITerm::App(head, args) => {
                if matches!(&**head, ITerm::Fn(n) if n.starts_with("<builtin>.")) {
                    write!(f, "{}", s.of(&**head))?;
                    for a in args {
                        write!(f, " {}", s.of(a))?;
                    }
                    return Ok(());
                }
                write!(f, "({}", s.of(&**head))?;
                for a in args {
                    write!(f, " {}", s.of(a))?;
                }
                write!(f, ")")
            }
            ITerm::Bin(op, a, b) => write!(f, "({} {op} {})", s.of(&**a), s.of(&**b)),
        }
    }
}

/// A proposition in `Prop` (dependent core kernel design, section 7):
/// comparisons and equality over index values, negation, conjunction
/// and disjunction (the refinement lattice: compound predicates), and
/// inference variables for implicit solving.
#[derive(Clone, Debug, PartialEq)]
pub enum PTerm {
    /// Logical constants in Prop, distinct from Bool values.
    Lit(bool),
    Cmp(IdxCmp, Box<ITerm>, Box<ITerm>),
    Eq(Box<ITerm>, Box<ITerm>),
    Not(Box<PTerm>),
    /// `p && q` (refinement types design, "compound predicates"): the
    /// conjunction of two admissible predicates; the lattice meet.
    And(Box<PTerm>, Box<PTerm>),
    /// `p || q` (refinement types design, "compound predicates"): the
    /// disjunction of two admissible predicates; the lattice join.
    Or(Box<PTerm>, Box<PTerm>),
    /// A structural decidable predicate of the refinement vocabulary
    /// (refinement types design, "shape"): `NotNull xs`, `NonZero n`,
    /// `Ascii s`, `Ordered xs`. These decide by structural evaluation
    /// over constructor forms at check time, over sample values in the
    /// law layer, and by their runtime tests under `decide`.
    Pred(PredFn, Box<ITerm>),
    Var(String),
    MVar(usize),
}

/// The closed decidable predicate vocabulary over container values
/// (refinement types design: predicates inhabit the decidable logic;
/// no SMT and no refinement inference). Each predicate is admissible
/// over specific operand shapes only, checked where a refinement or a
/// `decide` names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PredFn {
    /// A list-shaped value is not its empty constructor (`List`).
    NotNull,
    /// A numeric value is not zero (`Nat`/`Int` and machine integers).
    NonZero,
    /// Every byte of a text value is below 128 (`String`).
    Ascii,
    /// A list of numerics is in non-decreasing order (`List Nat`).
    Ordered,
}

impl PredFn {
    /// The surface spelling.
    pub fn name(&self) -> &'static str {
        match self {
            PredFn::NotNull => "NotNull",
            PredFn::NonZero => "NonZero",
            PredFn::Ascii => "Ascii",
            PredFn::Ordered => "Ordered",
        }
    }

    /// Resolve a name in proposition position to the vocabulary.
    pub fn lookup(name: &str) -> Option<PredFn> {
        match name {
            "NotNull" => Some(PredFn::NotNull),
            "NonZero" => Some(PredFn::NonZero),
            "Ascii" => Some(PredFn::Ascii),
            "Ordered" => Some(PredFn::Ordered),
            _ => None,
        }
    }

    /// The operand shapes this predicate decides over, for the error
    /// that refuses a wrong predicate naming it.
    pub fn operand_note(&self) -> &'static str {
        match self {
            PredFn::NotNull => "list-shaped values (`List`)",
            PredFn::NonZero => "numeric values (`Nat`, `Int`, machine integers)",
            PredFn::Ascii => "text values (`String`) and lists of chars (`List Char`)",
            PredFn::Ordered => "lists of numerics (`List Nat`)",
        }
    }
}

impl fmt::Display for PredFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// ------------------------------------------------------------------
// The implication theory (refinement types design, "narrowings and the
// implication theory"): a small table-driven decidable procedure over
// predicates, each row decidable, no SMT. This module is the shared
// bounded-arithmetic home of the interval-containment machinery (the
// design note's single home for `bounded(n)` depth bounds and temporal
// `recur` consumers too; those are not rewired here).
//
// The rows, in the order `pred_implies` consults them:
//   1. syntactic identity (atom-set inclusion over conjunctions:
//      `p && q implies p` is this row over the flattened conjuncts);
//   2. interval containment for comparison/measure atoms over one
//      measure expression (`> 1 && < 9` implies `> 0 && < 10` via
//      `lo >= lo'` and `hi <= hi'` on integer-normalized bounds);
//   3. the documented vocabulary rows: `length x > 0` implies
//      `NotNull x`, a range excluding zero implies `NonZero x`, and a
//      range disjoint from an atom refutes it (`Not` row);
//   4. unknown pairs are NOT implied: the cast needs a `Dec`, or
//      refuses (never claim an implication the table cannot justify).

/// Alpha-canonicalize a predicate: its free variables (the refinement
/// binder and any index parameters) are renamed to positional names in
/// first-appearance order, so predicates over differently named binders
/// compare and imply up to renaming.
pub fn pred_canon(p: &PTerm) -> PTerm {
    let mut order: Vec<String> = Vec::new();
    p.free_vars(&mut order);
    let mut out = p.clone();
    for (i, v) in order.iter().enumerate() {
        out = out.subst(v, &ITerm::Var(format!("$b{i}").into()));
    }
    out
}

/// Alpha-equivalence of two predicates (the same lattice point under
/// different binder spellings).
pub fn pred_alpha_eq(p: &PTerm, q: &PTerm) -> bool {
    pred_canon(p).eq(&pred_canon(q))
}

/// The comparison that holds exactly when `op` fails (integer
/// trichotomy; never applied to float comparisons, which give no facts).
fn neg_cmp(op: IdxCmp) -> IdxCmp {
    match op {
        IdxCmp::Lt => IdxCmp::Ge,
        IdxCmp::Le => IdxCmp::Gt,
        IdxCmp::Gt => IdxCmp::Le,
        IdxCmp::Ge => IdxCmp::Lt,
        IdxCmp::Eq => IdxCmp::Ne,
        IdxCmp::Ne => IdxCmp::Eq,
    }
}

/// The top-level conjuncts of a predicate, with negated comparisons
/// folded into their positive forms (`Not (x < 5)` is `x >= 5`) so the
/// interval rows see every bound the predicate carries.
pub fn pred_conjuncts(p: &PTerm) -> Vec<PTerm> {
    fn go(p: &PTerm, out: &mut Vec<PTerm>) {
        match p {
            PTerm::And(a, b) => {
                go(a, out);
                go(b, out);
            }
            PTerm::Not(q) => match &**q {
                PTerm::Cmp(op, a, b) => {
                    out.push(PTerm::Cmp(neg_cmp(*op), a.clone(), b.clone()))
                }
                PTerm::Not(r) => go(r, out),
                _ => out.push(p.clone()),
            },
            _ => out.push(p.clone()),
        }
    }
    let mut out = Vec::new();
    go(p, &mut out);
    out
}

/// A predicate's binder: the first free name it states its truth over
/// (a named refinement's or the `refined` former's implicit binder
/// keeps `$it`).
pub fn pred_binder(p: &PTerm) -> String {
    let mut fv = Vec::new();
    p.free_vars(&mut fv);
    // A positional argument name (`$a1` in a result refinement naming an
    // argument, `refined (r < $a1) Nat`) is never the refined binder, and
    // a renamed binder (`$it`) is it wherever it appears.
    if fv.iter().any(|v| v == "$it") {
        return "$it".to_string();
    }
    fv.into_iter()
        .find(|v| !is_positional_arg(v))
        .unwrap_or_else(|| "$it".to_string())
}

/// Whether a name is a positional argument binder (`$a{i}`): a
/// signature's `using` obligations and its result refinements name the
/// call's arguments this way.
pub fn is_positional_arg(v: &str) -> bool {
    v.strip_prefix("$a").is_some_and(|i| !i.is_empty() && i.bytes().all(|b| b.is_ascii_digit()))
}

/// The integer interval a predicate states over one measure expression
/// of its binder (`length xs > 100 && length xs <= 200` gives
/// `(101, 200)` over `length xs`; a boundless end is `None`): the
/// preservation check's witness-tier corners.
pub fn pred_measure_bounds(p: &PTerm, measure: &ITerm) -> (Option<i128>, Option<i128>) {
    let conj = pred_conjuncts(p);
    let iv = interval_over(&conj, measure);
    (iv.lo, iv.hi)
}

/// Whether `p` implies `q` under the table: sound and decidable, never
/// complete (an unknown pair is not implied).
pub fn pred_implies(p: &PTerm, q: &PTerm) -> bool {
    pred_implies_canon(&pred_canon(p), &pred_canon(q))
}

/// Whether the fact `p` entails the obligation `q` over the same names
/// (the evidence discharge: both speak about the caller's own values, so
/// no renaming applies; `a < b` must not entail `b < a`). The rows are
/// `pred_implies`'s, which need a numeral on one side of an atom;
/// relations between variables (`i < n` entails `i + 1 <= n`) and the
/// combination of several facts are `linear_entails`, which needs the
/// names' integer sorts.
pub fn pred_entails(p: &PTerm, q: &PTerm) -> bool {
    pred_implies_canon(p, q)
}

fn pred_implies_canon(p: &PTerm, q: &PTerm) -> bool {
    if matches!(q, PTerm::Lit(true)) || matches!(p, PTerm::Lit(false)) || p.eq(q) {
        return true;
    }
    match q {
        PTerm::And(a, b) => {
            pred_implies_canon(p, a) && pred_implies_canon(p, b)
        }
        PTerm::Or(a, b) => pred_implies_canon(p, a) || pred_implies_canon(p, b),
        _ => match p {
            PTerm::Or(a, b) => {
                pred_implies_canon(a, q) && pred_implies_canon(b, q)
            }
            _ => atom_implied_by(&pred_conjuncts(p), q),
        },
    }
}

/// A measure expression and its integer-normalized inclusive bounds
/// (`x > 1` is `x >= 2`; unbounded ends are `None`).
#[derive(Clone, Copy, Debug)]
struct Interval {
    lo: Option<i128>,
    hi: Option<i128>,
}

impl Interval {
    fn top() -> Interval {
        Interval { lo: None, hi: None }
    }

    fn meet(self, other: Interval) -> Interval {
        Interval {
            lo: match (self.lo, other.lo) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (a, None) => a,
                (None, b) => b,
            },
            hi: match (self.hi, other.hi) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, None) => a,
                (None, b) => b,
            },
        }
    }

    /// `self` contained in `other` (`lo >= lo'` and `hi <= hi'`).
    fn contained_in(self, other: Interval) -> bool {
        let lo_ok = match (self.lo, other.lo) {
            (Some(a), Some(b)) => a >= b,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => true,
        };
        let hi_ok = match (self.hi, other.hi) {
            (Some(a), Some(b)) => a <= b,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => true,
        };
        lo_ok && hi_ok
    }

    /// Whether the two ranges are provably disjoint.
    fn disjoint(self, other: Interval) -> bool {
        match (self.hi, other.lo) {
            (Some(a), Some(b)) if a < b => true,
            _ => match (self.lo, other.hi) {
                (Some(a), Some(b)) if a > b => true,
                _ => false,
            },
        }
    }

    /// Whether every point of the range is non-zero.
    fn excludes_zero(self) -> bool {
        match (self.lo, self.hi) {
            (Some(lo), _) if lo >= 1 => true,
            (_, Some(hi)) if hi <= -1 => true,
            _ => false,
        }
    }
}

/// The numeral value of a bound index term (numerals and arithmetic
/// over them).
fn bound_value(t: &ITerm) -> Option<i128> {
    match t.normalize() {
        ITerm::Num(d) => d.parse::<i128>().ok(),
        ITerm::Bin(op, a, b) => {
            let x = bound_value(&a)?;
            let y = bound_value(&b)?;
            match op {
                IdxOp::Add => x.checked_add(y),
                IdxOp::Sub => x.checked_sub(y),
                IdxOp::Mul => x.checked_mul(y),
            }
        }
        _ => None,
    }
}

/// One comparison atom read as `(measure, interval)` when one side is a
/// measure expression and the other an integer bound.
fn interval_atom(a: &PTerm) -> Option<(ITerm, Interval)> {
    let (op, l, r) = match a {
        PTerm::Cmp(op, l, r) => (*op, (**l).clone(), (**r).clone()),
        _ => return None,
    };
    let (measure, bound, op) = match (bound_value(&l), bound_value(&r)) {
        (Some(_), Some(_)) => return None,
        (None, Some(_)) => (l, bound_value(&r)?, op),
        (Some(_), None) => (r, bound_value(&l)?, flip_cmp(op)),
        (None, None) => return None,
    };
    let iv = match op {
        IdxCmp::Lt => Interval { lo: None, hi: Some(bound - 1) },
        IdxCmp::Le => Interval { lo: None, hi: Some(bound) },
        IdxCmp::Gt => Interval { lo: Some(bound + 1), hi: None },
        IdxCmp::Ge => Interval { lo: Some(bound), hi: None },
        IdxCmp::Eq => Interval { lo: Some(bound), hi: Some(bound) },
        IdxCmp::Ne => return None,
    };
    Some((measure, iv))
}

fn flip_cmp(op: IdxCmp) -> IdxCmp {
    match op {
        IdxCmp::Lt => IdxCmp::Gt,
        IdxCmp::Le => IdxCmp::Ge,
        IdxCmp::Gt => IdxCmp::Lt,
        IdxCmp::Ge => IdxCmp::Le,
        IdxCmp::Eq => IdxCmp::Eq,
        IdxCmp::Ne => IdxCmp::Ne,
    }
}

/// The conjuncts' combined interval over one measure expression.
fn interval_over(conj: &[PTerm], measure: &ITerm) -> Interval {
    let mut iv = Interval::top();
    for a in conj {
        if let Some((m, ai)) = interval_atom(a) {
            if m.eq(measure) {
                iv = iv.meet(ai);
            }
        }
    }
    iv
}

/// The table rows that justify one atomic consequent from the
/// hypothesis conjuncts (row 1 is syntactic identity).
fn atom_implied_by(conj: &[PTerm], qa: &PTerm) -> bool {
    if conj.iter().any(|pa| pa.eq(qa)) {
        return true;
    }
    match qa {
        PTerm::Cmp(IdxCmp::Ne, ..) => {
            // A disequality against a bound (row 3): the conjuncts'
            // range misses the bound, or the bound is zero and the
            // measure is `NonZero`.
            let Some((m, b)) = ne_atom(qa) else {
                return false;
            };
            let iv = interval_over(conj, &m);
            iv.disjoint(Interval { lo: Some(b), hi: Some(b) })
                || (b == 0
                    && conj
                        .iter()
                        .any(|pa| matches!(pa, PTerm::Pred(PredFn::NonZero, y) if (**y).eq(&m))))
        }
        PTerm::Cmp(..) | PTerm::Eq(..) => {
            // Interval containment (row 2): the conjuncts' range over
            // the atom's measure sits inside the atom's own range.
            let Some((m, qi)) = interval_atom(qa) else {
                return false;
            };
            interval_over(conj, &m).contained_in(qi)
        }
        PTerm::Not(r) => match &**r {
            PTerm::Not(s) => atom_implied_by(conj, s),
            _ => {
                // Refutation by disjoint ranges (row 3).
                let Some((m, ri)) = interval_atom(r) else {
                    return false;
                };
                interval_over(conj, &m).disjoint(ri)
            }
        },
        PTerm::Pred(PredFn::NotNull, x) => {
            // `length x > 0` implies `NotNull x` (row 3).
            let len_x = ITerm::app_fn("<builtin>.length".to_string(), vec![(**x).clone()]);
            let iv = interval_over(conj, &len_x);
            matches!(iv.lo, Some(lo) if lo >= 1)
        }
        PTerm::Pred(PredFn::NonZero, x) => {
            // A range excluding zero implies `NonZero x` (row 3), and so
            // does `x /= 0` (the predicate's own decision test).
            interval_over(conj, x).excludes_zero()
                || conj.iter().any(|pa| ne_atom(pa).is_some_and(|(m, b)| b == 0 && m.eq(x)))
        }
        _ => false,
    }
}

/// A disequality atom read as `(measure, bound)` (`x /= 5` or `5 /= x`).
fn ne_atom(a: &PTerm) -> Option<(ITerm, i128)> {
    let PTerm::Cmp(IdxCmp::Ne, l, r) = a else {
        return None;
    };
    match (bound_value(l), bound_value(r)) {
        (None, Some(b)) => Some(((**l).clone(), b)),
        (Some(b), None) => Some(((**r).clone(), b)),
        _ => None,
    }
}

// ------------------------------------------------------------------
// Linear facts over variables: difference logic (refinement types
// design, "narrowings and the implication theory"; proof automation
// design section 4 item 2). The interval rows above need a numeral on
// one side of every atom and read one fact at a time. This procedure
// reads every fact in scope together as constraints `x - y <= c` over
// integer terms and refutes them with the obligation's negation: a
// negative cycle in the constraint graph (Bellman-Ford) is the
// refutation, and the cycle itself is the certificate. Difference
// logic is the fragment chosen because it is what index and length
// reasoning needs (`i < n` gives `i + 1 <= n`, `i <= j` and `j < n`
// give `i < n`, a `Nat` is never negative) and it decides in
// polynomial time with a certificate a kernel checks by addition;
// unit-two-variable or full Presburger reasoning (Fourier-Motzkin,
// omega) costs exponential worst-case search for shapes (`i + j < n`,
// `2 * i <= n`) no obligation in the prelude has yet.
//
// What a term means: every term denotes the value it computes at run
// time, read through its sort.
// - `Nat` and `Int` arithmetic is exact (a `Nat` difference only returns
//   when it is exact), so `+`, `-` and multiplication by a numeral
//   linearize, and an opaque `Nat` value is never negative.
// - A machine integer or `Char` is an exact value in its type's range,
//   but its arithmetic is opaque: the prelude gives machine types no
//   `Add`/`Sub`/`Mul` (primitive types specification section 4), so a
//   `+` there is a user instance of an overflow family the solver cannot
//   see (`i + 1` over `I64` may wrap to the minimum). `i + 1` is then
//   one more value of the type, bounded by the range like any other.
// - A term with no known integer sort (a float, an unknown function's
//   result) gives no constraint: an atom over it is skipped as a
//   hypothesis and refused as a goal.
// A `/=` hypothesis is used by case split (`d <= -1` or `d >= 1`), at
// most `DIFF_SPLITS` deep, splitting only a disequality the current
// model violates; a deeper need is refused, never guessed.

/// The integer domain a term's values range over, which fixes what its
/// arithmetic means to the solver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntSort {
    /// `Nat`: exact arithmetic, never negative.
    Nat,
    /// `Int`: exact arithmetic, unbounded.
    Int,
    /// A machine integer or `Char`: an exact value in `[lo, hi]`, with
    /// opaque arithmetic.
    Bounded(i128, i128),
}

impl IntSort {
    /// The sort of a scalar type's values (`None` for floats, whose
    /// comparisons are not integer reasoning). `Usize`/`Isize` take the
    /// widest range any target gives them, which holds everywhere.
    pub fn of_scalar(s: Scalar) -> Option<IntSort> {
        if s.is_float() {
            return None;
        }
        if s == Scalar::Char {
            return Some(IntSort::Bounded(0, 0x10FFFF));
        }
        let w = s.width();
        Some(if s.is_signed() {
            IntSort::Bounded(-(1i128 << (w - 1)), (1i128 << (w - 1)) - 1)
        } else {
            IntSort::Bounded(0, (1i128 << w) - 1)
        })
    }

    fn exact(self) -> bool {
        !matches!(self, IntSort::Bounded(..))
    }
}

/// The integer sorts of the names a set of facts reads: locals by name
/// (the innermost binding last) and full applications of named
/// functions by head and arity (the baked measures `length` and
/// `toNat` are `Nat` without an entry).
#[derive(Clone, Debug, Default)]
pub struct Sorts {
    pub vars: Vec<(String, IntSort)>,
    pub fns: Vec<(String, usize, IntSort)>,
    pub values: Vec<(crate::core::LocalId, IntSort)>,
}

impl Sorts {
    /// A term's sort: a local's, a function result's, or arithmetic's
    /// operands' (both operands of `+` have the result's type).
    pub fn term(&self, t: &ITerm) -> Option<IntSort> {
        match t {
            ITerm::Var(n) => self.vars.iter().rev().find(|(m, _)| m == n).map(|(_, s)| *s),
            ITerm::LocalValue(local) => self.values.iter().rev().find(|(id, _)| id == local).map(|(_, sort)| *sort),
            ITerm::Fn(h) => self.fun(h, 0),
            ITerm::Inst(call) => match &call.head {
                IndexHead::Function(head) => self.fun(head, 0),
                _ => None,
            },
            ITerm::App(f, args) => match &**f {
                ITerm::Fn(h) => self.fun(h, args.len()),
                ITerm::Inst(call) => match &call.head {
                    IndexHead::Function(head) => self.fun(head, args.len()),
                    _ => None,
                },
                _ => None,
            },
            ITerm::Bin(_, a, b) => self.term(a).or_else(|| self.term(b)),
            ITerm::Num(_) | ITerm::Evidence(_) | ITerm::Lam(..) | ITerm::MVar(_) => None,
        }
    }

    fn fun(&self, head: &str, arity: usize) -> Option<IntSort> {
        if arity == 1 && matches!(head, "<builtin>.length" | "<builtin>.toNat") {
            return Some(IntSort::Nat);
        }
        self.fns.iter().find(|(h, k, _)| h == head && *k == arity).map(|(_, _, s)| *s)
    }
}

/// One difference constraint `x - y <= c`; `None` is the constant zero.
#[derive(Clone, Debug, PartialEq)]
pub struct DiffLe {
    pub x: Option<ITerm>,
    pub y: Option<ITerm>,
    pub c: i128,
}

impl DiffLe {
    /// Whether `by` alone establishes `self`: the same difference under
    /// an equal or tighter bound.
    fn follows_from(&self, by: &DiffLe) -> bool {
        self.x == by.x && self.y == by.y && by.c <= self.c
    }
}

/// Why one constraint of a refutation holds.
#[derive(Clone, Debug, PartialEq)]
pub enum DiffWhy {
    /// Conjunct `conj` (in `pred_conjuncts` order) of hypothesis `hyp`.
    Hyp { hyp: usize, conj: usize },
    /// The range axiom of the term's sort (a `Nat` is non-negative, a
    /// machine integer lies in its type's range).
    Domain(ITerm),
    /// The negation of the goal's disjunct `k`.
    Goal(usize),
    /// The alternative that the enclosing case split at this depth
    /// (counted from the root) assumes.
    Case(usize),
}

/// One cited constraint of a cycle.
#[derive(Clone, Debug, PartialEq)]
pub struct DiffStep {
    pub le: DiffLe,
    pub why: DiffWhy,
}

/// A difference-logic refutation of the hypotheses together with the
/// goal's negation, checkable without search: a cycle's constraints sum
/// to `0 <= c` with `c < 0` (every term cancels), and a split refutes
/// both alternatives of a disequality.
#[derive(Clone, Debug, PartialEq)]
pub enum DiffCert {
    Cycle(Vec<DiffStep>),
    /// The disequality cited by `on` (a hypothesis `l /= r`, or the
    /// negation of an `==` goal) read as `l - r <= -1` (`lt`) or
    /// `r - l <= -1` (`gt`); each side's steps cite it as `Case(depth)`.
    Split { on: DiffWhy, lt: Box<DiffCert>, gt: Box<DiffCert> },
    /// A refutation outside the difference fragment (a fact or goal over
    /// three or more terms, `i + k <= bytes-len b`): see [`LinCert`].
    Linear(LinCert),
}

/// One linear constraint `sum(a * key) <= c` over the same opaque keys
/// the difference reading uses, with its terms sorted by key and no
/// zero coefficient, so two readings of one atom compare equal.
#[derive(Clone, Debug, PartialEq)]
pub struct LinLe {
    pub terms: Vec<(ITerm, i128)>,
    pub c: i128,
}

impl LinLe {
    /// Whether `by` alone establishes `self`: the same left side under
    /// an equal or tighter bound.
    fn follows_from(&self, by: &LinLe) -> bool {
        self.terms == by.terms && by.c <= self.c
    }
}

/// One cited constraint of a Farkas combination.
#[derive(Clone, Debug, PartialEq)]
pub struct LinStep {
    pub le: LinLe,
    pub why: DiffWhy,
}

/// A linear refutation: positive integer multipliers whose weighted sum
/// of the cited constraints cancels every term and leaves `0 <= c` with
/// `c < 0` (a Farkas certificate), or a split of a disequality whose
/// two strict alternatives are each refuted.
#[derive(Clone, Debug, PartialEq)]
pub enum LinCert {
    Farkas(Vec<(i128, LinStep)>),
    Split { on: DiffWhy, lt: Box<LinCert>, gt: Box<LinCert> },
}

/// A proof that hypotheses entail one goal clause (an atom, or a
/// disjunction of atoms): the clause and its refutation certificate.
#[derive(Clone, Debug, PartialEq)]
pub struct LinearProof {
    pub goal: PTerm,
    pub cert: DiffCert,
}

impl LinearProof {
    /// The hypotheses the certificate cites (the facts a discharge read,
    /// for the staleness check over mutable places).
    pub fn hyps_used(&self) -> Vec<usize> {
        fn why(w: &DiffWhy, out: &mut Vec<usize>) {
            if let DiffWhy::Hyp { hyp, .. } = w {
                if !out.contains(hyp) {
                    out.push(*hyp);
                }
            }
        }
        fn go(c: &DiffCert, out: &mut Vec<usize>) {
            match c {
                DiffCert::Cycle(steps) => steps.iter().for_each(|s| why(&s.why, out)),
                DiffCert::Split { on, lt, gt } => {
                    why(on, out);
                    go(lt, out);
                    go(gt, out);
                }
                DiffCert::Linear(l) => lin(l, out),
            }
        }
        fn lin(c: &LinCert, out: &mut Vec<usize>) {
            match c {
                LinCert::Farkas(steps) => steps.iter().for_each(|(_, s)| why(&s.why, out)),
                LinCert::Split { on, lt, gt } => {
                    why(on, out);
                    lin(lt, out);
                    lin(gt, out);
                }
            }
        }
        let mut out = Vec::new();
        go(&self.cert, &mut out);
        out
    }
}

/// The deepest case split over `/=` facts the search attempts.
const DIFF_SPLITS: usize = 4;

/// A linear form `sum(coef * key) + c` over opaque keys.
struct Lin {
    terms: Vec<(ITerm, i128)>,
    c: i128,
}

impl Lin {
    fn add(mut self, other: Lin, sign: i128) -> Option<Lin> {
        self.c = self.c.checked_add(other.c.checked_mul(sign)?)?;
        for (k, a) in other.terms {
            let a = a.checked_mul(sign)?;
            match self.terms.iter_mut().find(|(j, _)| *j == k) {
                Some((_, b)) => *b = b.checked_add(a)?,
                None => self.terms.push((k, a)),
            }
        }
        Some(self)
    }

    fn scale(mut self, k: i128) -> Option<Lin> {
        self.c = self.c.checked_mul(k)?;
        for (_, a) in self.terms.iter_mut() {
            *a = a.checked_mul(k)?;
        }
        Some(self)
    }
}

/// A normalized term as a linear form: exact arithmetic linearizes,
/// anything else (and every compound term of an opaque sort) is a key.
fn lin_of(t: &ITerm, exact: bool) -> Option<Lin> {
    if let ITerm::Num(d) = t {
        return Some(Lin { terms: Vec::new(), c: d.parse().ok()? });
    }
    if exact {
        match t {
            ITerm::Bin(IdxOp::Add, a, b) => return lin_of(a, true)?.add(lin_of(b, true)?, 1),
            ITerm::Bin(IdxOp::Sub, a, b) => return lin_of(a, true)?.add(lin_of(b, true)?, -1),
            ITerm::Bin(IdxOp::Mul, a, b) => match (&**a, &**b) {
                (ITerm::Num(k), o) | (o, ITerm::Num(k)) => return lin_of(o, true)?.scale(k.parse().ok()?),
                _ => {}
            },
            _ => {}
        }
    }
    Some(Lin { terms: vec![(t.clone(), 1)], c: 0 })
}

/// What `lin <= 0` says as a difference constraint.
enum Fit {
    Le(DiffLe),
    /// Holds of every value (no information).
    True,
    /// Not a difference constraint.
    No,
}

fn fit(lin: Lin) -> Fit {
    let terms: Vec<(ITerm, i128)> = lin.terms.into_iter().filter(|(_, a)| *a != 0).collect();
    // `a * (x - y) + c <= 0` over integers is `x - y <= floor(-c / a)`.
    let bound = |a: i128| lin.c.checked_neg().map(|n| n.div_euclid(a));
    let le = |x: Option<ITerm>, y: Option<ITerm>, a: i128| match bound(a) {
        Some(c) => Fit::Le(DiffLe { x, y, c }),
        None => Fit::No,
    };
    match terms.as_slice() {
        [] if lin.c <= 0 => Fit::True,
        [] => Fit::Le(DiffLe { x: None, y: None, c: -lin.c }),
        [(k, a)] if *a > 0 => le(Some(k.clone()), None, *a),
        [(k, a)] => le(None, Some(k.clone()), -*a),
        [(k, a), (j, b)] if *a == -*b && *a > 0 => le(Some(k.clone()), Some(j.clone()), *a),
        [(k, a), (j, b)] if *a == -*b => le(Some(j.clone()), Some(k.clone()), *b),
        _ => Fit::No,
    }
}

/// `l - r <= d` as a difference constraint, trying the exact reading of
/// both sides first and then each side as one opaque value (`i + j <=
/// 5` is a bound on the value `i + j`).
fn read_le(l: &ITerm, r: &ITerm, d: i128, sorts: &Sorts) -> Fit {
    let Some(sort) = sorts.term(l).or_else(|| sorts.term(r)) else {
        return Fit::No;
    };
    let exact = sort.exact();
    for (lx, rx) in [(exact, exact), (false, exact), (exact, false), (false, false)] {
        let (Some(a), Some(b)) = (lin_of(l, lx), lin_of(r, rx)) else {
            return Fit::No;
        };
        let Some(diff) = a.add(b, -1).and_then(|x| x.add(Lin { terms: Vec::new(), c: d }, -1)) else {
            return Fit::No;
        };
        match fit(diff) {
            Fit::No => continue,
            f => return f,
        }
    }
    Fit::No
}

/// An atom's difference reading: constraints that all hold, or a
/// disequality whose two strict alternatives are `lt` and `gt`.
enum DiffReading {
    Les(Vec<DiffLe>),
    Ne(DiffLe, DiffLe),
}

fn read_cmp(op: IdxCmp, l: &ITerm, r: &ITerm, sorts: &Sorts) -> DiffReading {
    let les = |fs: Vec<Fit>| {
        DiffReading::Les(
            fs.into_iter()
                .filter_map(|f| match f {
                    Fit::Le(le) => Some(le),
                    _ => None,
                })
                .collect(),
        )
    };
    match op {
        IdxCmp::Lt => les(vec![read_le(l, r, -1, sorts)]),
        IdxCmp::Le => les(vec![read_le(l, r, 0, sorts)]),
        IdxCmp::Gt => les(vec![read_le(r, l, -1, sorts)]),
        IdxCmp::Ge => les(vec![read_le(r, l, 0, sorts)]),
        IdxCmp::Eq => les(vec![read_le(l, r, 0, sorts), read_le(r, l, 0, sorts)]),
        IdxCmp::Ne => match (read_le(l, r, -1, sorts), read_le(r, l, -1, sorts)) {
            (Fit::Le(a), Fit::Le(b)) => DiffReading::Ne(a, b),
            _ => DiffReading::Les(Vec::new()),
        },
    }
}

/// An atom as one comparison over normalized terms: `NonZero x` is `x
/// /= 0`, `NotNull x` is `length x >= 1`, and `Not` flips the operator.
fn diff_lit(a: &PTerm) -> Option<(IdxCmp, ITerm, ITerm)> {
    match a {
        PTerm::Cmp(op, l, r) => Some((*op, l.normalize(), r.normalize())),
        PTerm::Eq(l, r) => Some((IdxCmp::Eq, l.normalize(), r.normalize())),
        PTerm::Pred(PredFn::NonZero, x) => Some((IdxCmp::Ne, x.normalize(), ITerm::num(0))),
        PTerm::Pred(PredFn::NotNull, x) => Some((
            IdxCmp::Ge,
            ITerm::app_fn("<builtin>.length".to_string(), vec![x.normalize()]),
            ITerm::num(1),
        )),
        PTerm::Not(p) => diff_lit(p).map(|(op, l, r)| (neg_cmp(op), l, r)),
        _ => None,
    }
}

fn read_atom(a: &PTerm, sorts: &Sorts) -> DiffReading {
    match diff_lit(a) {
        Some((op, l, r)) => read_cmp(op, &l, &r, sorts),
        None => DiffReading::Les(Vec::new()),
    }
}

/// The negation of goal disjunct `g`.
fn read_negated(g: &PTerm, sorts: &Sorts) -> DiffReading {
    match diff_lit(g) {
        Some((op, l, r)) => read_cmp(neg_cmp(op), &l, &r, sorts),
        None => DiffReading::Les(Vec::new()),
    }
}

/// A term's sort axioms: a `Nat` value is non-negative (unless it is a
/// difference, whose exact value may be negative), a machine integer or
/// `Char` lies in its range.
fn sort_domain(t: &ITerm, sorts: &Sorts) -> Vec<DiffLe> {
    fn has_sub(t: &ITerm) -> bool {
        match t {
            ITerm::Bin(IdxOp::Sub, ..) => true,
            ITerm::Bin(_, a, b) => has_sub(a) || has_sub(b),
            _ => false,
        }
    }
    match sorts.term(t) {
        Some(IntSort::Nat) if !has_sub(t) => vec![DiffLe { x: None, y: Some(t.clone()), c: 0 }],
        Some(IntSort::Bounded(lo, hi)) => vec![
            DiffLe { x: Some(t.clone()), y: None, c: hi },
            DiffLe { x: None, y: Some(t.clone()), c: -lo },
        ],
        _ => Vec::new(),
    }
}

/// A goal's disjuncts when it is a disjunction of atoms.
fn goal_disjuncts(g: &PTerm, out: &mut Vec<PTerm>) -> bool {
    match g {
        PTerm::Or(a, b) => goal_disjuncts(a, out) && goal_disjuncts(b, out),
        PTerm::And(..) => false,
        _ => {
            out.push(g.clone());
            true
        }
    }
}

/// Whether the hypotheses entail `goal` in difference logic, with one
/// checkable proof per goal clause. Sound and, within the fragment and
/// the split bound, complete: a failure means some integer assignment
/// satisfies every readable hypothesis and violates the goal. Of equal
/// hypotheses only the last (innermost) is read, so a fact re-derived
/// after a write is the one a proof cites.
pub fn linear_entails(hyps: &[PTerm], goal: &PTerm, sorts: &Sorts) -> Option<Vec<LinearProof>> {
    if let PTerm::And(a, b) = goal {
        let mut p = linear_entails(hyps, a, sorts)?;
        p.extend(linear_entails(hyps, b, sorts)?);
        return Some(p);
    }
    let mut disjuncts = Vec::new();
    if !goal_disjuncts(goal, &mut disjuncts) {
        let PTerm::Or(a, b) = goal else { return None };
        return linear_entails(hyps, a, sorts).or_else(|| linear_entails(hyps, b, sorts));
    }
    let mut steps: Vec<DiffStep> = Vec::new();
    let mut diseqs: Vec<(DiffWhy, DiffLe, DiffLe)> = Vec::new();
    let mut add = |reading: DiffReading, why: DiffWhy, steps: &mut Vec<DiffStep>| match reading {
        DiffReading::Les(ls) => {
            steps.extend(ls.into_iter().map(|le| DiffStep { le, why: why.clone() }))
        }
        DiffReading::Ne(lt, gt) => diseqs.push((why, lt, gt)),
    };
    for (k, g) in disjuncts.iter().enumerate() {
        add(read_negated(g, sorts), DiffWhy::Goal(k), &mut steps);
    }
    // Innermost first: Bellman-Ford relaxes in edge order, so a cycle
    // through a fresh fact is found before one through an older equal
    // one (equal ones are skipped outright).
    for (i, h) in hyps.iter().enumerate().rev() {
        if hyps[i + 1..].iter().any(|later| later == h) {
            continue;
        }
        for (j, a) in pred_conjuncts(h).iter().enumerate() {
            add(read_atom(a, sorts), DiffWhy::Hyp { hyp: i, conj: j }, &mut steps);
        }
    }
    let mut keys: Vec<ITerm> = Vec::new();
    let mut note = |t: &Option<ITerm>| {
        if let Some(t) = t {
            if !keys.contains(t) {
                keys.push(t.clone());
            }
        }
    };
    for s in &steps {
        note(&s.le.x);
        note(&s.le.y);
    }
    for (_, lt, gt) in &diseqs {
        for le in [lt, gt] {
            note(&le.x);
            note(&le.y);
        }
    }
    for k in &keys {
        for le in sort_domain(k, sorts) {
            steps.push(DiffStep { le, why: DiffWhy::Domain(k.clone()) });
        }
    }
    // Difference logic first (complete within its fragment and cheap);
    // then the linear procedure, for facts and goals over three or more
    // terms that the difference reading cannot hold.
    let cert = match diff_refute(&mut steps, &diseqs, &keys, 0) {
        Some(c) => c,
        None => DiffCert::Linear(lin_prove(hyps, &disjuncts, sorts)?),
    };
    let proof = LinearProof { goal: goal.clone(), cert };
    // The search is untrusted: only a proof the checker accepts counts.
    check_linear_proof(hyps, &proof, sorts).then(|| vec![proof])
}

/// Refute `steps` together with one alternative of every disequality.
fn diff_refute(
    steps: &mut Vec<DiffStep>,
    diseqs: &[(DiffWhy, DiffLe, DiffLe)],
    keys: &[ITerm],
    depth: usize,
) -> Option<DiffCert> {
    let model = match diff_cycle(steps, keys) {
        Err(cycle) => return Some(DiffCert::Cycle(cycle.into_iter().map(|e| steps[e].clone()).collect())),
        Ok(model) => model,
    };
    // The model satisfies every constraint; it is a counterexample
    // unless it makes some disequality's two sides equal. Split only
    // such a disequality.
    let val = |t: &Option<ITerm>| match t {
        None => Some(0),
        Some(t) => keys.iter().position(|k| k == t).map(|i| model[i + 1] - model[0]),
    };
    let holds = |le: &DiffLe| matches!((val(&le.x), val(&le.y)), (Some(x), Some(y)) if x - y <= le.c);
    let k = diseqs.iter().position(|(_, lt, gt)| !holds(lt) && !holds(gt))?;
    if depth >= DIFF_SPLITS {
        return None;
    }
    let (on, lt, gt) = diseqs[k].clone();
    let rest: Vec<(DiffWhy, DiffLe, DiffLe)> =
        diseqs.iter().enumerate().filter(|(i, _)| *i != k).map(|(_, d)| d.clone()).collect();
    let side = |le: DiffLe, steps: &mut Vec<DiffStep>| {
        steps.push(DiffStep { le, why: DiffWhy::Case(depth) });
        let c = diff_refute(steps, &rest, keys, depth + 1);
        steps.pop();
        c
    };
    let lt = side(lt, steps)?;
    let gt = side(gt, steps)?;
    Some(DiffCert::Split { on, lt: Box::new(lt), gt: Box::new(gt) })
}

/// Bellman-Ford from a virtual source over `x - y <= c` as an edge `y ->
/// x` of weight `c` (node 0 is the constant zero, node `i + 1` is key
/// `i`): a negative cycle's edge indices, or a satisfying assignment.
fn diff_cycle(steps: &[DiffStep], keys: &[ITerm]) -> std::result::Result<Vec<i128>, Vec<usize>> {
    let node = |t: &Option<ITerm>| match t {
        None => 0,
        Some(t) => keys.iter().position(|k| k == t).map_or(0, |i| i + 1),
    };
    let edges: Vec<(usize, usize, i128)> =
        steps.iter().map(|s| (node(&s.le.x), node(&s.le.y), s.le.c)).collect();
    let n = keys.len() + 1;
    let mut dist = vec![0i128; n];
    let mut pred = vec![usize::MAX; n];
    let mut last = None;
    for _ in 0..=n {
        last = None;
        for (e, &(x, y, c)) in edges.iter().enumerate() {
            let cand = dist[y].saturating_add(c);
            if cand < dist[x] {
                dist[x] = cand;
                pred[x] = e;
                last = Some(x);
            }
        }
        if last.is_none() {
            return Ok(dist);
        }
    }
    // Still relaxing after `n + 1` rounds: walking predecessors `n`
    // times from the last relaxed node lands on a negative cycle.
    let mut v = last.expect("relaxed this round");
    for _ in 0..n {
        match pred.get(v) {
            Some(&e) if e != usize::MAX => v = edges[e].1,
            _ => return Err(Vec::new()),
        }
    }
    let start = v;
    let mut cycle = Vec::new();
    loop {
        let e = pred[v];
        if e == usize::MAX || cycle.len() > n {
            return Err(Vec::new());
        }
        cycle.push(e);
        v = edges[e].1;
        if v == start {
            return Err(cycle);
        }
    }
}

/// Check a linear proof without search (the trusted half: what the
/// dependent kernel's evidence check would run on the certificate):
/// every step's constraint follows from what it cites, re-read from the
/// hypotheses, the goal and the sort axioms, and each cycle sums to a
/// contradiction.
pub fn check_linear_proof(hyps: &[PTerm], proof: &LinearProof, sorts: &Sorts) -> bool {
    let mut disjuncts = Vec::new();
    if !goal_disjuncts(&proof.goal, &mut disjuncts) {
        return false;
    }
    let cited = |w: &DiffWhy| -> Option<DiffReading> {
        match w {
            DiffWhy::Hyp { hyp, conj } => {
                let conjs = pred_conjuncts(hyps.get(*hyp)?);
                Some(read_atom(conjs.get(*conj)?, sorts))
            }
            DiffWhy::Goal(k) => Some(read_negated(disjuncts.get(*k)?, sorts)),
            DiffWhy::Domain(t) => Some(DiffReading::Les(sort_domain(t, sorts))),
            DiffWhy::Case(_) => None,
        }
    };
    fn go(
        cert: &DiffCert,
        cited: &dyn Fn(&DiffWhy) -> Option<DiffReading>,
        cases: &mut Vec<DiffLe>,
    ) -> bool {
        match cert {
            DiffCert::Cycle(steps) => {
                let justified = steps.iter().all(|s| match &s.why {
                    DiffWhy::Case(d) => cases.get(*d).is_some_and(|c| s.le.follows_from(c)),
                    w => matches!(cited(w), Some(DiffReading::Les(ls)) if ls.iter().any(|l| s.le.follows_from(l))),
                });
                let mut sum: i128 = 0;
                let mut net: Vec<(ITerm, i64)> = Vec::new();
                let mut tally = |t: &Option<ITerm>, d: i64| {
                    if let Some(t) = t {
                        match net.iter_mut().find(|(k, _)| k == t) {
                            Some((_, n)) => *n += d,
                            None => net.push((t.clone(), d)),
                        }
                    }
                };
                for s in steps {
                    tally(&s.le.x, 1);
                    tally(&s.le.y, -1);
                    match sum.checked_add(s.le.c) {
                        Some(v) => sum = v,
                        None => return false,
                    }
                }
                justified && !steps.is_empty() && sum < 0 && net.iter().all(|(_, n)| *n == 0)
            }
            DiffCert::Split { on, lt, gt } => {
                let Some(DiffReading::Ne(a, b)) = cited(on) else {
                    return false;
                };
                for (alt, sub) in [(a, lt), (b, gt)] {
                    cases.push(alt);
                    let ok = go(sub, cited, cases);
                    cases.pop();
                    if !ok {
                        return false;
                    }
                }
                true
            }
            // The linear procedure answers at the top level only.
            DiffCert::Linear(_) => false,
        }
    }
    if let DiffCert::Linear(l) = &proof.cert {
        return check_lin_cert(hyps, &disjuncts, l, sorts);
    }
    go(&proof.cert, &cited, &mut Vec::new())
}

// --------------------------------------------------------------------------
// Linear facts beyond the difference fragment (design/proof-automation.md
// section 5, "three-term facts"). Bellman-Ford above only reads `x - y <=
// c`; a bound such as `di + n <= bytes-len dst` has three terms. This
// procedure reads every comparison as `sum(a * key) <= c` over the same
// opaque keys, eliminates keys by Fourier-Motzkin, and returns positive
// integer multipliers (a Farkas certificate) that `check_lin_cert`
// re-checks by summing. It is sound over the integers and incomplete: it
// refutes the rational relaxation, so a set with a rational solution and
// no integer one (`2 * x == 1`) is not refuted, beyond the rounding each
// single constraint gets when it is read.

/// The most rows elimination may hold at once. Passing it abandons the
/// attempt, which is a refusal of the obligation (the caller then names
/// it as unproved), never a long search: every elimination step is
/// bounded by `LIN_ROWS^2` combinations and there are at most as many
/// steps as keys.
const LIN_ROWS: usize = 256;

/// The deepest disequality split the linear procedure attempts, and how
/// many disequalities it tries at each depth.
const LIN_SPLITS: usize = 2;
const LIN_SPLIT_TRIES: usize = 6;

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// `l - r <= d` as a linear constraint, the left side's coefficients
/// divided by their common factor and the bound rounded down (sound over
/// the integers). `None` when it holds of every value.
fn lin_le(l: &ITerm, r: &ITerm, d: i128, sorts: &Sorts) -> Option<LinLe> {
    let exact = sorts.term(l).or_else(|| sorts.term(r))?.exact();
    let diff = lin_of(l, exact)?.add(lin_of(r, exact)?, -1)?;
    let c = d.checked_sub(diff.c)?;
    let mut terms: Vec<(ITerm, i128)> = diff.terms.into_iter().filter(|(_, a)| *a != 0).collect();
    if terms.is_empty() {
        return (c < 0).then_some(LinLe { terms, c });
    }
    let g = terms.iter().fold(0, |g, (_, a)| gcd(g, *a));
    for (_, a) in terms.iter_mut() {
        *a /= g;
    }
    terms.sort_by_cached_key(|(t, _)| format!("{t:?}"));
    Some(LinLe { terms, c: c.div_euclid(g) })
}

enum LinReading {
    Les(Vec<LinLe>),
    Ne(LinLe, LinLe),
}

fn lin_cmp(op: IdxCmp, l: &ITerm, r: &ITerm, sorts: &Sorts) -> LinReading {
    let les = |v: Vec<Option<LinLe>>| LinReading::Les(v.into_iter().flatten().collect());
    match op {
        IdxCmp::Lt => les(vec![lin_le(l, r, -1, sorts)]),
        IdxCmp::Le => les(vec![lin_le(l, r, 0, sorts)]),
        IdxCmp::Gt => les(vec![lin_le(r, l, -1, sorts)]),
        IdxCmp::Ge => les(vec![lin_le(r, l, 0, sorts)]),
        IdxCmp::Eq => les(vec![lin_le(l, r, 0, sorts), lin_le(r, l, 0, sorts)]),
        IdxCmp::Ne => match (lin_le(l, r, -1, sorts), lin_le(r, l, -1, sorts)) {
            (Some(a), Some(b)) => LinReading::Ne(a, b),
            _ => LinReading::Les(Vec::new()),
        },
    }
}

fn lin_atom(a: &PTerm, sorts: &Sorts) -> LinReading {
    match diff_lit(a) {
        Some((op, l, r)) => lin_cmp(op, &l, &r, sorts),
        None => LinReading::Les(Vec::new()),
    }
}

fn lin_negated(g: &PTerm, sorts: &Sorts) -> LinReading {
    match diff_lit(g) {
        Some((op, l, r)) => lin_cmp(neg_cmp(op), &l, &r, sorts),
        None => LinReading::Les(Vec::new()),
    }
}

/// A key's sort axioms as linear constraints.
fn lin_domain(t: &ITerm, sorts: &Sorts) -> Vec<LinLe> {
    sort_domain(t, sorts)
        .into_iter()
        .map(|d| {
            let mut terms = Vec::new();
            if let Some(x) = d.x {
                terms.push((x, 1));
            }
            if let Some(y) = d.y {
                terms.push((y, -1));
            }
            LinLe { terms, c: d.c }
        })
        .collect()
}

/// The linear procedure over the hypotheses and the goal clause's
/// disjuncts: the constraints connected to the goal's keys, refuted with
/// up to `LIN_SPLITS` disequality splits.
fn lin_prove(hyps: &[PTerm], disjuncts: &[PTerm], sorts: &Sorts) -> Option<LinCert> {
    let mut steps: Vec<LinStep> = Vec::new();
    let mut diseqs: Vec<(DiffWhy, LinLe, LinLe)> = Vec::new();
    let add = |r: LinReading, why: DiffWhy, steps: &mut Vec<LinStep>, diseqs: &mut Vec<(DiffWhy, LinLe, LinLe)>| match r {
        LinReading::Les(ls) => steps.extend(ls.into_iter().map(|le| LinStep { le, why: why.clone() })),
        LinReading::Ne(a, b) => diseqs.push((why, a, b)),
    };
    for (k, g) in disjuncts.iter().enumerate() {
        add(lin_negated(g, sorts), DiffWhy::Goal(k), &mut steps, &mut diseqs);
    }
    let goal_rows = steps.len();
    let goal_diseqs = diseqs.len();
    for (i, h) in hyps.iter().enumerate().rev() {
        if hyps[i + 1..].iter().any(|later| later == h) {
            continue;
        }
        for (j, a) in pred_conjuncts(h).iter().enumerate() {
            add(lin_atom(a, sorts), DiffWhy::Hyp { hyp: i, conj: j }, &mut steps, &mut diseqs);
        }
    }
    // Only the constraints that share keys with the goal, transitively,
    // can take part in its refutation.
    let mut keys: Vec<ITerm> = Vec::new();
    for s in &steps[..goal_rows] {
        for (t, _) in &s.le.terms {
            if !keys.contains(t) {
                keys.push(t.clone());
            }
        }
    }
    // Negating equality produces a disequality, not an ordinary row.
    // Its keys must seed the dependency closure too; otherwise all the
    // facts needed for a three-variable equality are silently discarded.
    for (_, a, _) in &diseqs[..goal_diseqs] {
        for (t, _) in &a.terms { if !keys.contains(t) { keys.push(t.clone()); } }
    }
    let touches = |le: &LinLe, keys: &[ITerm]| le.terms.iter().any(|(t, _)| keys.contains(t));
    let mut used = vec![false; steps.len()];
    let mut used_ne = vec![false; diseqs.len()];
    loop {
        let mut grew = false;
        for (i, s) in steps.iter().enumerate() {
            if !used[i] && (i < goal_rows || touches(&s.le, &keys)) {
                used[i] = true;
                grew = true;
                for (t, _) in &s.le.terms {
                    if !keys.contains(t) {
                        keys.push(t.clone());
                    }
                }
            }
        }
        for (i, (_, a, _)) in diseqs.iter().enumerate() {
            if !used_ne[i] && (i < goal_diseqs || touches(a, &keys)) {
                used_ne[i] = true;
                grew = true;
                for (t, _) in &a.terms {
                    if !keys.contains(t) {
                        keys.push(t.clone());
                    }
                }
            }
        }
        if !grew {
            break;
        }
    }
    let mut rows: Vec<LinStep> =
        steps.into_iter().zip(used).filter(|(_, u)| *u).map(|(s, _)| s).collect();
    for k in &keys {
        for le in lin_domain(k, sorts) {
            rows.push(LinStep { le, why: DiffWhy::Domain(k.clone()) });
        }
    }
    if rows.len() > LIN_ROWS {
        return None;
    }
    let diseqs: Vec<(DiffWhy, LinLe, LinLe)> =
        diseqs.into_iter().zip(used_ne).filter(|(_, u)| *u).map(|(d, _)| d).collect();
    lin_refute(&mut rows, &diseqs, 0)
}

fn lin_refute(rows: &mut Vec<LinStep>, diseqs: &[(DiffWhy, LinLe, LinLe)], depth: usize) -> Option<LinCert> {
    if let Some(m) = fourier_motzkin(rows) {
        let mut cert: Vec<(i128, LinStep)> = Vec::new();
        for (i, k) in m {
            cert.push((k, rows[i].clone()));
        }
        return Some(LinCert::Farkas(cert));
    }
    if depth >= LIN_SPLITS {
        return None;
    }
    for k in 0..diseqs.len().min(LIN_SPLIT_TRIES) {
        let (on, lt, gt) = diseqs[k].clone();
        let rest: Vec<(DiffWhy, LinLe, LinLe)> =
            diseqs.iter().enumerate().filter(|(i, _)| *i != k).map(|(_, d)| d.clone()).collect();
        let mut side = |le: LinLe, rows: &mut Vec<LinStep>| {
            rows.push(LinStep { le, why: DiffWhy::Case(depth) });
            let c = lin_refute(rows, &rest, depth + 1);
            rows.pop();
            c
        };
        let Some(l) = side(lt, rows) else { continue };
        let Some(g) = side(gt, rows) else { continue };
        return Some(LinCert::Split { on, lt: Box::new(l), gt: Box::new(g) });
    }
    None
}

/// Fourier-Motzkin elimination over `rows`: the multipliers (row index,
/// positive integer) of a combination that cancels every key and leaves
/// a negative bound, or `None` when the rows are rationally feasible, a
/// coefficient overflows, or the row cap is passed.
fn fourier_motzkin(rows: &[LinStep]) -> Option<Vec<(usize, i128)>> {
    // Each row: coefficients, bound, and its derivation over `rows`.
    type Row = (Vec<(ITerm, i128)>, i128, Vec<(usize, i128)>);
    let mut cur: Vec<Row> =
        rows.iter().enumerate().map(|(i, s)| (s.le.terms.clone(), s.le.c, vec![(i, 1)])).collect();
    loop {
        if let Some(r) = cur.iter().find(|r| r.0.is_empty() && r.1 < 0) {
            return Some(r.2.clone());
        }
        // The key whose elimination makes the fewest new rows.
        let mut best: Option<(ITerm, usize)> = None;
        let mut seen: Vec<ITerm> = Vec::new();
        for r in &cur {
            for (t, _) in &r.0 {
                if seen.contains(t) {
                    continue;
                }
                seen.push(t.clone());
                let coef = |r: &Row| r.0.iter().find(|(k, _)| k == t).map_or(0, |(_, a)| *a);
                let pos = cur.iter().filter(|r| coef(r) > 0).count();
                let neg = cur.iter().filter(|r| coef(r) < 0).count();
                let cost = pos * neg;
                if best.as_ref().is_none_or(|(_, c)| cost < *c) {
                    best = Some((t.clone(), cost));
                }
            }
        }
        let (t, _) = best?;
        let coef = |r: &Row| r.0.iter().find(|(k, _)| *k == t).map_or(0, |(_, a)| *a);
        let (mut next, mut pos, mut neg): (Vec<Row>, Vec<Row>, Vec<Row>) = (Vec::new(), Vec::new(), Vec::new());
        for r in cur {
            match coef(&r) {
                0 => next.push(r),
                a if a > 0 => pos.push(r),
                _ => neg.push(r),
            }
        }
        for p in &pos {
            for n in &neg {
                let (ap, an) = (coef(p), -coef(n));
                let mut terms: Vec<(ITerm, i128)> = Vec::new();
                for (row, k) in [(p, an), (n, ap)] {
                    for (key, a) in &row.0 {
                        let v = a.checked_mul(k)?;
                        match terms.iter_mut().find(|(j, _)| j == key) {
                            Some((_, b)) => *b = b.checked_add(v)?,
                            None => terms.push((key.clone(), v)),
                        }
                    }
                }
                terms.retain(|(_, a)| *a != 0);
                let c = p.1.checked_mul(an)?.checked_add(n.1.checked_mul(ap)?)?;
                let mut deriv: Vec<(usize, i128)> = Vec::new();
                for (row, k) in [(p, an), (n, ap)] {
                    for (i, m) in &row.2 {
                        let v = m.checked_mul(k)?;
                        match deriv.iter_mut().find(|(j, _)| j == i) {
                            Some((_, b)) => *b = b.checked_add(v)?,
                            None => deriv.push((*i, v)),
                        }
                    }
                }
                // Exact division by a common factor of everything keeps
                // the multipliers integral and the numbers small.
                let g = terms
                    .iter()
                    .map(|(_, a)| *a)
                    .chain(deriv.iter().map(|(_, m)| *m))
                    .chain(std::iter::once(c))
                    .fold(0, gcd);
                let g = if g > 1 { g } else { 1 };
                for (_, a) in terms.iter_mut() {
                    *a /= g;
                }
                for (_, m) in deriv.iter_mut() {
                    *m /= g;
                }
                next.push((terms, c / g, deriv));
                if next.len() > LIN_ROWS {
                    return None;
                }
            }
        }
        cur = next;
    }
}

/// Check a linear certificate without search: every cited constraint
/// follows from its source re-read, every multiplier is positive, and the
/// weighted sum cancels every key and bounds zero by a negative number.
fn check_lin_cert(hyps: &[PTerm], disjuncts: &[PTerm], cert: &LinCert, sorts: &Sorts) -> bool {
    let cited = |w: &DiffWhy| -> Option<LinReading> {
        match w {
            DiffWhy::Hyp { hyp, conj } => {
                let conjs = pred_conjuncts(hyps.get(*hyp)?);
                Some(lin_atom(conjs.get(*conj)?, sorts))
            }
            DiffWhy::Goal(k) => Some(lin_negated(disjuncts.get(*k)?, sorts)),
            DiffWhy::Domain(t) => Some(LinReading::Les(lin_domain(t, sorts))),
            DiffWhy::Case(_) => None,
        }
    };
    fn go(cert: &LinCert, cited: &dyn Fn(&DiffWhy) -> Option<LinReading>, cases: &mut Vec<LinLe>) -> bool {
        match cert {
            LinCert::Farkas(steps) => {
                let justified = steps.iter().all(|(k, s)| {
                    *k > 0
                        && match &s.why {
                            DiffWhy::Case(d) => cases.get(*d).is_some_and(|c| s.le.follows_from(c)),
                            w => matches!(cited(w), Some(LinReading::Les(ls)) if ls.iter().any(|l| s.le.follows_from(l))),
                        }
                });
                let mut sum: i128 = 0;
                let mut net: Vec<(ITerm, i128)> = Vec::new();
                for (k, s) in steps {
                    for (t, a) in &s.le.terms {
                        let Some(v) = a.checked_mul(*k) else { return false };
                        match net.iter_mut().find(|(j, _)| j == t) {
                            Some((_, b)) => match b.checked_add(v) {
                                Some(x) => *b = x,
                                None => return false,
                            },
                            None => net.push((t.clone(), v)),
                        }
                    }
                    match s.le.c.checked_mul(*k).and_then(|v| sum.checked_add(v)) {
                        Some(v) => sum = v,
                        None => return false,
                    }
                }
                justified && !steps.is_empty() && sum < 0 && net.iter().all(|(_, n)| *n == 0)
            }
            LinCert::Split { on, lt, gt } => {
                let Some(LinReading::Ne(a, b)) = cited(on) else {
                    return false;
                };
                for (alt, sub) in [(a, lt), (b, gt)] {
                    cases.push(alt);
                    let ok = go(sub, cited, cases);
                    cases.pop();
                    if !ok {
                        return false;
                    }
                }
                true
            }
        }
    }
    go(cert, &cited, &mut Vec::new())
}

impl PTerm {
    pub fn has_declaration_pending(&self) -> bool {
        match self {
            Self::Eq(left, right) | Self::Cmp(_, left, right) => left.has_declaration_pending() || right.has_declaration_pending(),
            Self::Pred(_, argument) => argument.has_declaration_pending(),
            Self::Not(body) => body.has_declaration_pending(),
            Self::And(left, right) | Self::Or(left, right) => left.has_declaration_pending() || right.has_declaration_pending(),
            _ => false,
        }
    }
    pub fn visit_types(&self, visit: &mut impl FnMut(&Ty)) {
        match self {
            PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => { a.visit_types(visit); b.visit_types(visit); }
            PTerm::Not(prop) => prop.visit_types(visit),
            PTerm::And(a, b) | PTerm::Or(a, b) => { a.visit_types(visit); b.visit_types(visit); }
            PTerm::Pred(_, term) => term.visit_types(visit),
            PTerm::Lit(_) | PTerm::Var(_) | PTerm::MVar(_) => {}
        }
    }

    pub fn map_types(&self, map: &mut impl FnMut(&Ty) -> Ty) -> Self {
        match self {
            PTerm::Cmp(op, a, b) => PTerm::Cmp(*op, Box::new(a.map_types(map)), Box::new(b.map_types(map))),
            PTerm::Eq(a, b) => PTerm::Eq(Box::new(a.map_types(map)), Box::new(b.map_types(map))),
            PTerm::Not(prop) => PTerm::Not(Box::new(prop.map_types(map))),
            PTerm::And(a, b) => PTerm::And(Box::new(a.map_types(map)), Box::new(b.map_types(map))),
            PTerm::Or(a, b) => PTerm::Or(Box::new(a.map_types(map)), Box::new(b.map_types(map))),
            PTerm::Pred(pred, term) => PTerm::Pred(*pred, Box::new(term.map_types(map))),
            _ => self.clone(),
        }
    }

    pub fn free_vars(&self, out: &mut Vec<String>) {
        match self {
            PTerm::Cmp(_, a, b) => {
                a.free_vars(out);
                b.free_vars(out);
            }
            PTerm::Eq(a, b) => {
                a.free_vars(out);
                b.free_vars(out);
            }
            PTerm::Not(p) => p.free_vars(out),
            PTerm::And(a, b) | PTerm::Or(a, b) => {
                a.free_vars(out);
                b.free_vars(out);
            }
            PTerm::Pred(_, a) => {
                a.free_vars(out);
            }
            PTerm::Var(n) => {
                if !out.contains(n) {
                    out.push(n.clone());
                }
            }
            PTerm::Lit(_) | PTerm::MVar(_) => {}
        }
    }

    pub fn mentions(&self, name: &str) -> bool {
        let mut out = Vec::new();
        self.free_vars(&mut out);
        out.iter().any(|v| v == name)
    }

    /// Free index metavariables in the proposition's operands. Proposition
    /// metavariables have a separate namespace and are not index variables.
    pub fn free_mvars(&self, out: &mut Vec<usize>) {
        match self {
            PTerm::Lit(_) | PTerm::MVar(_) | PTerm::Var(_) => {}
            PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
                a.free_mvars(out);
                b.free_mvars(out);
            }
            PTerm::Not(p) => p.free_mvars(out),
            PTerm::And(a, b) | PTerm::Or(a, b) => {
                a.free_mvars(out);
                b.free_mvars(out);
            }
            PTerm::Pred(_, a) => a.free_mvars(out),
        }
    }

    pub fn subst(&self, name: &str, by: &ITerm) -> PTerm {
        self.substitute(IndexSubstitution::Value(name, by))
    }

    pub fn subst_evidence(&self, bindings: &HashMap<IndexEvidence, ITerm>) -> PTerm {
        self.substitute(IndexSubstitution::Evidence(bindings))
    }

    pub fn subst_local_values(&self, bindings: &HashMap<crate::core::LocalId, ITerm>) -> PTerm {
        self.substitute(IndexSubstitution::LocalValues(bindings))
    }

    pub fn local_values(&self, out: &mut Vec<crate::core::LocalId>) {
        match self {
            Self::Cmp(_, left, right) | Self::Eq(left, right) => { left.local_values(out); right.local_values(out); }
            Self::Not(body) => body.local_values(out),
            Self::And(left, right) | Self::Or(left, right) => { left.local_values(out); right.local_values(out); }
            Self::Pred(_, value) => value.local_values(out),
            Self::Lit(_) | Self::Var(_) | Self::MVar(_) => {}
        }
    }

    fn substitute(&self, substitution: IndexSubstitution<'_>) -> PTerm {
        match self {
            PTerm::Cmp(op, a, b) => PTerm::Cmp(
                *op,
                Box::new(a.substitute(substitution)),
                Box::new(b.substitute(substitution)),
            ),
            PTerm::Eq(a, b) => {
                PTerm::Eq(Box::new(a.substitute(substitution)), Box::new(b.substitute(substitution)))
            }
            PTerm::Not(p) => PTerm::Not(Box::new(p.substitute(substitution))),
            PTerm::And(a, b) => PTerm::And(
                Box::new(a.substitute(substitution)),
                Box::new(b.substitute(substitution)),
            ),
            PTerm::Or(a, b) => PTerm::Or(
                Box::new(a.substitute(substitution)),
                Box::new(b.substitute(substitution)),
            ),
            PTerm::Pred(pf, a) => PTerm::Pred(*pf, Box::new(a.substitute(substitution))),
            PTerm::Lit(_) | PTerm::Var(_) | PTerm::MVar(_) => self.clone(),
        }
    }

    pub fn normalize(&self) -> PTerm {
        match self {
            PTerm::Cmp(op, a, b) => PTerm::Cmp(
                *op,
                Box::new(a.normalize()),
                Box::new(b.normalize()),
            ),
            PTerm::Eq(a, b) => PTerm::Eq(Box::new(a.normalize()), Box::new(b.normalize())),
            PTerm::Not(p) => PTerm::Not(Box::new(p.normalize())),
            PTerm::And(a, b) => PTerm::And(Box::new(a.normalize()), Box::new(b.normalize())),
            PTerm::Or(a, b) => PTerm::Or(Box::new(a.normalize()), Box::new(b.normalize())),
            PTerm::Pred(pf, a) => PTerm::Pred(*pf, Box::new(a.normalize())),
            other => other.clone(),
        }
    }

    pub fn eq(&self, other: &PTerm) -> bool {
        match (self.normalize(), other.normalize()) {
            (PTerm::Cmp(op, a, b), PTerm::Cmp(op2, c, d)) => {
                op == op2 && (*a).eq(&*c) && (*b).eq(&*d)
            }
            (PTerm::Eq(a, b), PTerm::Eq(c, d)) => (*a).eq(&*c) && (*b).eq(&*d),
            (PTerm::Not(a), PTerm::Not(b)) => (*a).eq(&*b),
            (PTerm::And(a, b), PTerm::And(c, d)) | (PTerm::Or(a, b), PTerm::Or(c, d)) => {
                (*a).eq(&*c) && (*b).eq(&*d)
            }
            (PTerm::Pred(pf, a), PTerm::Pred(pf2, b)) => pf == pf2 && (*a).eq(&*b),
            (PTerm::Lit(x), PTerm::Lit(y)) => x == y,
            (PTerm::Var(x), PTerm::Var(y)) => x == y,
            (PTerm::MVar(x), PTerm::MVar(y)) => x == y,
            _ => false,
        }
    }
}

impl fmt::Display for PTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.spell(f, Spelling::Identity)
    }
}

impl Spell for PTerm {
    fn spell(&self, f: &mut fmt::Formatter<'_>, s: Spelling<'_>) -> fmt::Result {
        match self {
            PTerm::Cmp(op, a, b) => write!(f, "{} {op} {}", s.of(a), s.of(b)),
            PTerm::Eq(a, b) => write!(f, "Eq {} {}", s.of(a), s.of(b)),
            PTerm::Not(p) => write!(f, "Not ({})", s.of(p)),
            PTerm::And(a, b) => write!(f, "({} && {})", s.of(a), s.of(b)),
            PTerm::Or(a, b) => write!(f, "({} || {})", s.of(a), s.of(b)),
            PTerm::Pred(pf, a) => write!(f, "{pf} {}", s.of(a)),
            PTerm::Lit(value) => f.write_str(if *value { "True" } else { "False" }),
            PTerm::Var(n) => write!(f, "{n}"),
            PTerm::MVar(m) => write!(f, "?p{m}"),
        }
    }
}

/// The role of a boundary pointer (foreign function interface
/// specification section 3): every pointer argument or result carries
/// one. Safe callers never construct these roles from integer casts;
/// roles only arise from foreign results, out-cells, and buffer borrows.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PtrRole {
    /// `RawPtr t`: a raw unchecked pointer (the value is not claimed to
    /// be valid, aligned, or owned).
    Raw,
    /// `Borrowed t`: a call-scoped borrow, valid only for the duration
    /// of the call that produced or consumes it.
    Borrowed,
    /// `Retained t`: a retained borrow, valid until the foreign side
    /// acknowledges release (foreign function interface specification
    /// section 8).
    Retained,
    /// `Owned <allocator> t`: transferred ownership; released only by
    /// the deallocator of the same allocator identity (foreign function
    /// interface specification section 4).
    Owned(String),
    /// `Out t`: a caller-allocated one-word out-cell, initialized before
    /// the call and read afterward.
    Out,
    /// `MaybeNull t`: a nullable raw pointer (`null` is the absence).
    MaybeNull,
}

impl fmt::Display for PtrRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PtrRole::Raw => write!(f, "RawPtr"),
            PtrRole::Borrowed => write!(f, "Borrowed"),
            PtrRole::Retained => write!(f, "Retained"),
            PtrRole::Owned(a) => write!(f, "Owned {a}"),
            PtrRole::Out => write!(f, "Out"),
            PtrRole::MaybeNull => write!(f, "MaybeNull"),
        }
    }
}

impl PtrRole {
    /// Whether values of this role may be absent (the null pointer).
    pub fn nullable(&self) -> bool {
        matches!(self, PtrRole::MaybeNull)
    }

    /// Whether the callee takes ownership (a deallocator must exist).
    pub fn owned(&self) -> bool {
        matches!(self, PtrRole::Owned(_))
    }
}

/// The C ABI scalar spellings accepted at a `foreign` boundary (foreign
/// function interface specification section 2: bindings specify integer
/// widths and signedness). Each maps to its target scalar; the foreign
/// contract records the width and signedness explicitly.
pub fn c_abi_scalar(name: &str) -> Option<Scalar> {
    use Scalar::*;
    Some(match name {
        "CChar" => I8,
        "CUchar" => U8,
        "CShort" => I16,
        "CUshort" => U16,
        "CInt" => I32,
        "CUint" => U32,
        "CLong" | "CLongLong" => I64,
        "CULong" | "CULongLong" => U64,
        "CSize" | "SizeT" => Usize,
        "CSsize" => Isize,
        "CFloat" => F32,
        "CDouble" => F64,
        _ => return None,
    })
}

/// Monomorphic type: rigid parameters, unification variables, primitives,
/// algebraic data types, functions with effect rows and parameter usage
/// multiplicities, affine thunks, memo cells and borrows, plus the
/// dependent core: universes, embedded index values, propositions, and
/// dependent function and pair types.
#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    /// The mathematical `Int`: a boxed bignum word (sign-magnitude),
    /// distinct from the machine `Prim` scalars (primitive types
    /// specification, section 1).
    Int,
    /// The mathematical `Nat`: a boxed bignum word (non-negative),
    /// distinct from the machine `Prim` scalars (primitive types
    /// specification, section 1).
    Nat,
    Bool,
    Unit,
    /// `Bytes`: a boxed byte buffer word.
    Bytes,
    /// `Sym`: a one-word handle to a process-wide interned name cell
    /// (primitive types specification, section 9). Not a machine scalar:
    /// `Eq` is identity, `Ord` is lexicographic by name.
    Sym,
    /// A machine scalar primitive (`I32`, `U8`, `F64`, `Char`, ...).
    Prim(Scalar),
    /// Rigid type parameter bound by a `forall`.
    Param(Interned),
    /// Runtime identity (a region-like name) in argument position, as in
    /// `Io r`: identities unify by the region rules (flexible names take a
    /// rigid identity; distinct rigid identities never merge).
    Region(Interned),
    /// Unification variable.
    Meta(MetaId),
    /// `T a b ...`.
    Adt(Interned, Vec<Ty>),
    /// `(T1, ..., Tn)`, n >= 2: a structural product.
    Tuple(Vec<Ty>),
    /// `a %m ->{|e} b`: the multiplicity is the parameter's usage class.
    Fun(Box<Ty>, Mult, EffRow, Box<Ty>),
    /// `Thunk {e} a`.
    Thunk { eff: EffRow, inner: Box<Ty> },
    /// `Later a`: the guarded observation modality (core language
    /// specification section 9). A logical modality with no effect row and
    /// no reclamation obligation; at runtime it keeps the thunk
    /// representation (codata and guardedness design, section 1). It is
    /// never interchangeable with `Thunk`.
    Later(Box<Ty>),
    /// `Lazy {e} r a`: a memo cell in region `r`.
    Lazy { eff: EffRow, region: Interned, inner: Box<Ty> },
    /// `Ref r a`: a shared borrow of `a` in region `r`.
    Ref(Interned, Box<Ty>),
    /// `Mut r a`: an exclusive borrow of `a` in region `r`.
    Mut(Interned, Box<Ty>),
    /// A handler value for one effect label: operation clauses plus a
    /// return clause. `input` is the handled computation's result, `output`
    /// the result of handling, `eff` the effects the clause bodies perform
    /// (core specification section 6: introduced effects appear in the
    /// handler's type), and `resumption` the clause's promise about its
    /// one-shot continuation: `%aff` at most once, `%1` exactly once.
    Handler {
        label: EffLabel,
        input: Box<Ty>,
        output: Box<Ty>,
        eff: EffRow,
        resumption: Mult,
    },
    /// A universe as a type: `Type i` or `Prop` (the kind of types and of
    /// propositions).
    Sort(Sort),
    /// A value in type position: the `n` of `Vec n a`, the `k + 1` of
    /// `Vec (k + 1) a`.
    Idx(Box<ITerm>),
    /// A proposition: a type whose universe is `Prop`, e.g. `i < n` or
    /// `Eq x y`.
    Prp(Box<PTerm>),
    /// Dependent function `(x :: A) -> B`, where `B` may mention `x`.
    Pi {
        var: Interned,
        dom: Box<Ty>,
        mult: Mult,
        eff: EffRow,
        cod: Box<Ty>,
    },
    /// Dependent pair `Sigma (x :: A). B`, where `B` may mention `x`.
    Sigma {
        var: Interned,
        dom: Box<Ty>,
        cod: Box<Ty>,
    },
    /// A boundary pointer to `pointee` with an ownership role (foreign
    /// function interface specification sections 2 and 3). All pointer
    /// roles cross the C ABI as `ptr`; the role is contract metadata and
    /// a static discipline, never a runtime difference.
    Ptr { role: PtrRole, pointee: Box<Ty> },
    /// `f a b ...` where the head is a type parameter, a unification
    /// variable, or a partially applied type former: a higher-kinded
    /// application spine (modules and compilation specification section
    /// 1). Named type constructors keep the flattened `Adt` form;
    /// [`Ty::apply_spine`] builds these with beta-reduction, so a spine's
    /// head is never itself a saturated application.
    App(Box<Ty>, Vec<Ty>),
    /// A bare type former in prefix form: `(->)` (the pure total arrow
    /// family) or a structural tuple former `(,)`, `(,,)`, ...
    Former(Former),
    /// A nested polymorphic type `forall as. T` (design/rank-n-polymorphism.md).
    /// The scheme binds only `params` (with `con_params` naming the
    /// higher-kinded ones), `row_params` and `region_params`; every other
    /// namespace is empty. Binder names are fresh at conversion, and every
    /// substitution is shadow-aware. At run time the value is a dictionary
    /// block of the polytype's family (class `Obj`).
    Forall(Box<Scheme>),
}

/// A bare type constructor former spellable in prefix position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Former {
    /// The pure total arrow family `(->)`: `((->) r) a` is the pure
    /// total arrow `r -> a` with its default parameter multiplicity and
    /// a pure row. Effectful arrows are a distinct shape and never arise
    /// from this former.
    Arrow,
    /// The structural tuple former `(,,,...)` of the given arity.
    Tuple(usize),
}

impl Former {
    /// The former's total argument count.
    pub fn arity(&self) -> usize {
        match self {
            Former::Arrow => 2,
            Former::Tuple(n) => *n,
        }
    }
}

impl Ty {
    pub fn has_declaration_pending(&self) -> bool {
        match self {
            Self::Idx(index) => index.has_declaration_pending(),
            Self::Prp(proposition) => proposition.has_declaration_pending(),
            Self::Adt(_, arguments) | Self::Tuple(arguments) => arguments.iter().any(Self::has_declaration_pending),
            Self::App(head, arguments) => head.has_declaration_pending() || arguments.iter().any(Self::has_declaration_pending),
            Self::Fun(domain, _, _, result) | Self::Pi { dom: domain, cod: result, .. }
            | Self::Sigma { dom: domain, cod: result, .. } => domain.has_declaration_pending() || result.has_declaration_pending(),
            Self::Ptr { pointee: inner, .. } | Self::Thunk { inner, .. } | Self::Lazy { inner, .. }
            | Self::Later(inner) | Self::Ref(_, inner) | Self::Mut(_, inner) => inner.has_declaration_pending(),
            Self::Handler { input, output, .. } => input.has_declaration_pending() || output.has_declaration_pending(),
            _ => false,
        }
    }
    /// Free value names inside type arguments. Pi/Sigma bind only their
    /// codomain; they must not hide an outer name in their own domain.
    pub fn free_index_vars(&self, out: &mut Vec<String>) {
        match self {
            Ty::Idx(term) => term.free_vars(out),
            Ty::Prp(prop) => prop.free_vars(out),
            Ty::Adt(_, args) | Ty::Tuple(args) => {
                for arg in args { arg.free_index_vars(out); }
            }
            Ty::App(head, args) => {
                head.free_index_vars(out);
                for arg in args { arg.free_index_vars(out); }
            }
            Ty::Fun(dom, _, _, cod) => { dom.free_index_vars(out); cod.free_index_vars(out); }
            Ty::Pi { var, dom, cod, .. } | Ty::Sigma { var, dom, cod } => {
                dom.free_index_vars(out);
                let mut inner = Vec::new();
                cod.free_index_vars(&mut inner);
                for name in inner {
                    if name != *var && !out.contains(&name) { out.push(name); }
                }
            }
            Ty::Thunk { inner, .. } | Ty::Lazy { inner, .. } | Ty::Later(inner)
                | Ty::Ref(_, inner) | Ty::Mut(_, inner) => inner.free_index_vars(out),
            Ty::Forall(scheme) => scheme.ty.free_index_vars(out),
            Ty::Ptr { pointee, .. } => pointee.free_index_vars(out),
            Ty::Handler { input, output, .. } => { input.free_index_vars(out); output.free_index_vars(out); }
            _ => {}
        }
    }

    /// The value range `[lo, hi)` of canonical `i64` words this type's
    /// values occupy, when the type statically bounds them (`Bool` and
    /// the primitive scalars narrower than the word). `None` when the
    /// words span the `i64` space (`Int`, `Nat`, `U64`) or the value is
    /// boxed (`Fin n` included: a `Fin` word is its constructor-block
    /// pointer, not the bounded index, so no range covers it): a range
    /// the analysis cannot state is never guessed (the `!range`/`range`
    /// facts of `design/llvm-contract-attributes.md`).
    pub fn value_range(&self) -> Option<(i64, i64)> {
        match self {
            Ty::Bool => Some((0, 2)),
            Ty::Prim(s) => match s {
                Scalar::U8 => Some((0, 256)),
                Scalar::U16 => Some((0, 65536)),
                Scalar::U32 => Some((0, 4294967296)),
                Scalar::I8 => Some((-128, 128)),
                Scalar::I16 => Some((-32768, 32768)),
                Scalar::I32 => Some((-2147483648, 2147483648)),
                Scalar::Char => Some((0, 0x110000)),
                _ => None,
            },
            _ => None,
        }
    }

    /// A pure arrow `a -> b` with an unrestricted parameter.
    pub fn fun(a: Ty, b: Ty) -> Ty {
        Ty::Fun(Box::new(a), Mult::Many, EffRow::pure(), Box::new(b))
    }

    /// A pure arrow `a -> b` whose parameter multiplicity is still open.
    pub fn fun_gen(a: Ty, b: Ty) -> Ty {
        Ty::Fun(Box::new(a), Mult::Flex, EffRow::pure(), Box::new(b))
    }

    /// An arrow `a ->{|e} b` with an unrestricted parameter.
    pub fn fun_eff(a: Ty, eff: EffRow, b: Ty) -> Ty {
        Ty::Fun(Box::new(a), Mult::Many, eff, Box::new(b))
    }

    /// An arrow `a %m ->{|e} b`.
    pub fn fun_mult(a: Ty, m: Mult, eff: EffRow, b: Ty) -> Ty {
        Ty::Fun(Box::new(a), m, eff, Box::new(b))
    }

    /// Build `a1 -> a2 -> ... -> res` with unrestricted pure arrows.
    pub fn funs(args: &[Ty], res: Ty) -> Ty {
        args.iter().rev().fold(res, |acc, a| Ty::fun(a.clone(), acc))
    }

    /// Apply one type-level argument to a head, with beta-reduction:
    /// named constructors collect the argument into their `Adt` spine, a
    /// saturated arrow or tuple former collapses to `Fun`/`Tuple`, and
    /// anything else (a parameter, a metavariable, an unsaturated former)
    /// becomes an `App` spine. Substituting a constructor application
    /// for a parameter runs through here, so `f a` with `f := Result e`
    /// becomes `Result e a` and `f := (->) r` becomes `r -> a`.
    pub fn apply1(head: Ty, arg: Ty) -> Ty {
        match head {
            Ty::Adt(name, mut args) => {
                args.push(arg);
                Ty::Adt(name, args)
            }
            Ty::App(h, mut args) => {
                if let Ty::Former(f) = &*h {
                    let n = f.arity();
                    args.push(arg);
                    if args.len() == n {
                        return Ty::former_apply(*f, args);
                    }
                    return Ty::App(h, args);
                }
                args.push(arg);
                Ty::App(h, args)
            }
            Ty::Former(f) => {
                if f.arity() == 1 {
                    return Ty::former_apply(f, vec![arg]);
                }
                Ty::App(Box::new(Ty::Former(f)), vec![arg])
            }
            other => Ty::App(Box::new(other), vec![arg]),
        }
    }

    /// The saturated image of a type former.
    fn former_apply(f: Former, args: Vec<Ty>) -> Ty {
        match f {
            Former::Arrow => {
                let mut it = args.into_iter();
                let a = it.next().expect("arrow argument");
                let b = it.next().expect("arrow argument");
                let m = default_mult(&a);
                Ty::fun_mult(a, m, EffRow::pure(), b)
            }
            Former::Tuple(_) => Ty::Tuple(args),
        }
    }

    /// Apply a spine of type-level arguments to a head, left to right.
    pub fn apply_spine(head: Ty, args: impl IntoIterator<Item = Ty>) -> Ty {
        args.into_iter().fold(head, Ty::apply1)
    }

    /// Count the arrows of a function type.
    pub fn arity(&self) -> usize {
        match self {
            Ty::Fun(_, _, _, b) => 1 + b.arity(),
            Ty::Pi { cod, .. } => 1 + cod.arity(),
            _ => 0,
        }
    }

    /// A dependent function type whose binder is `var`.
    pub fn pi(var: impl Into<String>, dom: Ty, mult: Mult, eff: EffRow, cod: Ty) -> Ty {
        Ty::Pi {
            var: var.into().into(),
            dom: Box::new(dom),
            mult,
            eff,
            cod: Box::new(cod),
        }
    }

    /// A dependent pair type.
    pub fn sigma(var: impl Into<String>, dom: Ty, cod: Ty) -> Ty {
        Ty::Sigma {
            var: var.into().into(),
            dom: Box::new(dom),
            cod: Box::new(cod),
        }
    }

    /// View any function type as a (possibly non-dependent) Pi: `Ty::Fun`
    /// is a Pi whose binder nothing mentions.
    pub fn as_pi(&self) -> (String, &Ty, Mult, &EffRow, &Ty) {
        match self {
            Ty::Fun(a, m, e, b) => ("$".to_string(), a, *m, e, b),
            Ty::Pi { var, dom, mult, eff, cod } => (var.clone().to_string(), dom, *mult, eff, cod),
            _ => unreachable!("as_pi on a non-function type"),
        }
    }

    /// Does this type mention the index-level variable `name`?
    pub fn mentions_idx(&self, name: &str) -> bool {
        let mut names = Vec::new();
        self.free_index_vars(&mut names);
        names.iter().any(|candidate| candidate == name)
    }

    /// Alpha-rename a dependent binder before an index substitution, when
    /// needed to prevent capture of a free name in the replacement.
    fn substituted_codomain(var: &str, dom: &Ty, cod: &Ty, substitution: IndexSubstitution<'_>) -> (String, Ty) {
        if substitution.shadows_value(var) { return (var.to_owned(), cod.clone()); }
        if !substitution.captures_value(var) { return (var.to_owned(), cod.substitute_index(substitution)); }
        let mut fresh = format!("{var}$sub");
        while dom.mentions_idx(&fresh) || cod.mentions_idx(&fresh) || substitution.captures_value(&fresh) || substitution.shadows_value(&fresh) {
            fresh.push('$');
        }
        let renamed = cod.subst_idx(var, &ITerm::Var(fresh.clone().into()));
        (fresh, renamed.substitute_index(substitution))
    }

    /// Substitute the index-level variable `name` by the index term `by`
    /// (beta for dependent application and elimination).
    pub fn subst_idx(&self, name: &str, by: &ITerm) -> Ty {
        self.substitute_index(IndexSubstitution::Value(name, by))
    }

    pub fn subst_evidence(&self, bindings: &HashMap<IndexEvidence, ITerm>) -> Ty {
        self.substitute_index(IndexSubstitution::Evidence(bindings))
    }

    pub fn subst_local_values(&self, bindings: &HashMap<crate::core::LocalId, ITerm>) -> Ty {
        self.substitute_index(IndexSubstitution::LocalValues(bindings))
    }

    pub fn local_values(&self, out: &mut Vec<crate::core::LocalId>) {
        match self {
            Self::Idx(value) => value.local_values(out),
            Self::Prp(prop) => prop.local_values(out),
            Self::Adt(_, args) | Self::Tuple(args) => for arg in args { arg.local_values(out); },
            Self::App(head, args) => { head.local_values(out); for arg in args { arg.local_values(out); } }
            Self::Fun(dom, _, _, cod) | Self::Pi { dom, cod, .. } | Self::Sigma { dom, cod, .. } => {
                dom.local_values(out); cod.local_values(out);
            }
            Self::Thunk { inner, .. } | Self::Lazy { inner, .. } | Self::Later(inner)
                | Self::Ref(_, inner) | Self::Mut(_, inner) => inner.local_values(out),
            Self::Ptr { pointee, .. } => pointee.local_values(out),
            Self::Handler { input, output, .. } => { input.local_values(out); output.local_values(out); }
            _ => {}
        }
    }

    /// Freshen deferred dictionary witnesses when copying their owning code.
    /// As with value/evidence substitution, replacements are simultaneous.
    pub fn subst_index_metas(&self, variables: &HashMap<usize, ITerm>) -> Ty {
        self.substitute_index(IndexSubstitution::Inference(variables))
    }

    pub fn mentions_evidence(&self, binding: &IndexEvidence) -> bool {
        *self != self.subst_evidence(&HashMap::from([(binding.clone(), ITerm::num(0))]))
    }

    fn substitute_index(&self, substitution: IndexSubstitution<'_>) -> Ty {
        match self {
            Ty::Idx(t) => Ty::Idx(Box::new(t.substitute(substitution))),
            Ty::Prp(p) => Ty::Prp(Box::new(p.substitute(substitution))),
            Ty::Adt(n, args) => {
                Ty::Adt(n.clone(), args.iter().map(|a| a.substitute_index(substitution)).collect())
            }
            Ty::Tuple(args) => {
                Ty::Tuple(args.iter().map(|a| a.substitute_index(substitution)).collect())
            }
            Ty::Fun(a, m, e, b) => Ty::fun_mult(
                a.substitute_index(substitution),
                *m,
                e.clone(),
                b.substitute_index(substitution),
            ),
            Ty::Pi { var, dom, mult, eff, cod } => {
                let (var, cod) = Self::substituted_codomain(var, dom, cod, substitution);
                Ty::Pi { var: var.into(), dom: Box::new(dom.substitute_index(substitution)), mult: *mult, eff: eff.clone(), cod: Box::new(cod) }
            }
            Ty::Ptr { role, pointee } => Ty::Ptr {
                role: role.clone(),
                pointee: Box::new(pointee.substitute_index(substitution)),
            },
            Ty::Sigma { var, dom, cod } => {
                let (var, cod) = Self::substituted_codomain(var, dom, cod, substitution);
                Ty::Sigma { var: var.into(), dom: Box::new(dom.substitute_index(substitution)), cod: Box::new(cod) }
            }
            Ty::Forall(scheme) => scheme.map_forall_body(|body| body.substitute_index(substitution)),
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: eff.clone(),
                inner: Box::new(inner.substitute_index(substitution)),
            },
            Ty::Later(inner) => Ty::Later(Box::new(inner.substitute_index(substitution))),
            Ty::Lazy { eff, region, inner } => Ty::Lazy {
                eff: eff.clone(),
                region: region.clone(),
                inner: Box::new(inner.substitute_index(substitution)),
            },
            Ty::Ref(region, inner) => {
                Ty::Ref(region.clone(), Box::new(inner.substitute_index(substitution)))
            }
            Ty::Mut(region, inner) => {
                Ty::Mut(region.clone(), Box::new(inner.substitute_index(substitution)))
            }
            Ty::Handler { label, input, output, eff, resumption } => Ty::Handler {
                label: label.clone(),
                input: Box::new(input.substitute_index(substitution)),
                output: Box::new(output.substitute_index(substitution)),
                eff: eff.clone(),
                resumption: *resumption,
            },
            Ty::App(h, args) => Ty::App(
                Box::new(h.substitute_index(substitution)),
                args.iter().map(|a| a.substitute_index(substitution)).collect(),
            ),
            other => other.clone(),
        }
    }

    /// Collect the index metavariables free in this type.
    pub fn free_idx_metas(&self, out: &mut Vec<usize>) {
        match self {
            Ty::Idx(t) => t.free_mvars(out),
            Ty::Prp(p) => p.free_mvars(out),
            Ty::Adt(_, args) | Ty::Tuple(args) => {
                for a in args {
                    a.free_idx_metas(out);
                }
            }
            Ty::Fun(a, _, _, b) | Ty::Pi { dom: a, cod: b, .. } => {
                a.free_idx_metas(out);
                b.free_idx_metas(out);
            }
            Ty::Sigma { dom, cod, .. } => {
                dom.free_idx_metas(out);
                cod.free_idx_metas(out);
            }
            Ty::Forall(scheme) => scheme.ty.free_idx_metas(out),
            Ty::Thunk { inner, .. } | Ty::Later(inner) => inner.free_idx_metas(out),
            Ty::Lazy { inner, .. } => inner.free_idx_metas(out),
            Ty::Ref(_, inner) | Ty::Mut(_, inner) => inner.free_idx_metas(out),
            Ty::Ptr { pointee, .. } => pointee.free_idx_metas(out),
            Ty::Handler { input, output, .. } => {
                input.free_idx_metas(out);
                output.free_idx_metas(out);
            }
            Ty::App(h, args) => {
                h.free_idx_metas(out);
                for a in args {
                    a.free_idx_metas(out);
                }
            }
            _ => {}
        }
    }

    /// Collect the metavariables and row metavariables free in this type.
    pub fn free_metas(&self, out: &mut Vec<EitherMeta>) {
        match self {
            Ty::Meta(m) => out.push(EitherMeta::Type(*m)),
            Ty::Idx(term) => term.visit_types(&mut |ty| ty.free_metas(out)),
            Ty::Prp(prop) => prop.visit_types(&mut |ty| ty.free_metas(out)),
            Ty::Adt(_, args) | Ty::Tuple(args) => {
                for a in args {
                    a.free_metas(out);
                }
            }
            Ty::Fun(a, _, e, b) => {
                a.free_metas(out);
                e.free_metas(out);
                b.free_metas(out);
            }
            Ty::Pi { dom, eff, cod, .. } => {
                dom.free_metas(out);
                eff.free_metas(out);
                cod.free_metas(out);
            }
            Ty::Sigma { dom, cod, .. } => {
                dom.free_metas(out);
                cod.free_metas(out);
            }
            Ty::Forall(scheme) => scheme.ty.free_metas(out),
            Ty::Thunk { eff, inner } => {
                eff.free_metas(out);
                inner.free_metas(out);
            }
            Ty::Later(inner) => {
                inner.free_metas(out);
            }
            Ty::Lazy { eff, inner, .. } => {
                eff.free_metas(out);
                inner.free_metas(out);
            }
            Ty::Ref(_, inner) | Ty::Mut(_, inner) => inner.free_metas(out),
            Ty::Ptr { pointee, .. } => pointee.free_metas(out),
            Ty::Handler { input, output, eff, .. } => {
                input.free_metas(out);
                eff.free_metas(out);
                output.free_metas(out);
            }
            Ty::App(h, args) => {
                h.free_metas(out);
                for a in args {
                    a.free_metas(out);
                }
            }
            _ => {}
        }
    }
}

/// The default multiplicity of an unannotated binder (ownership and usage
/// design, section 2): thunks are affine because `force` consumes and
/// dropping an unforced thunk destroys it (laziness specification,
/// section 3); a value whose type mentions a task handle is linear
/// because dropping a handle would silently detach execution
/// (concurrency specification, section 2); everything else is
/// unrestricted in this slice.
pub fn default_mult(ty: &Ty) -> Mult {
    match ty {
        // A thunk is consumed by `force`; a `Later` observation likewise
        // (the usage-multiplicity rule keeping `Later` from unrestricted
        // thunk semantics).
        Ty::Thunk { .. } | Ty::Later(_) => Mult::Aff,
        _ => {
            if mentions_task(ty) {
                Mult::One
            } else {
                Mult::Many
            }
        }
    }
}

/// Whether a type mentions a `Task` handle anywhere inside it (so its
/// binders default to the handle discipline: consumed exactly once by
/// joining).
pub fn mentions_task(ty: &Ty) -> bool {
    match ty {
        Ty::Adt(n, args) => n == "Task" || args.iter().any(mentions_task),
        Ty::Tuple(args) => args.iter().any(mentions_task),
        Ty::Fun(a, _, _, b) | Ty::Pi { dom: a, cod: b, .. } => {
            mentions_task(a) || mentions_task(b)
        }
        Ty::Sigma { dom, cod, .. } => mentions_task(dom) || mentions_task(cod),
        Ty::Forall(scheme) => mentions_task(&scheme.ty),
        Ty::Thunk { inner, .. } | Ty::Later(inner) | Ty::Lazy { inner, .. } => {
            mentions_task(inner)
        }
        Ty::Ref(_, i) | Ty::Mut(_, i) => mentions_task(i),
        Ty::Ptr { pointee, .. } => mentions_task(pointee),
        Ty::Handler { input, output, .. } => {
            mentions_task(input) || mentions_task(output)
        }
        Ty::App(h, args) => mentions_task(h) || args.iter().any(mentions_task),
        _ => false,
    }
}

/// Whether a type mentions a `Lazy {e} r a` memo cell anywhere inside it.
/// A memo cell is single-threaded by construction and does not cross its
/// region's thread (lazy specification, sections 5 and 9).
pub fn mentions_memo(ty: &Ty) -> bool {
    match ty {
        Ty::Lazy { .. } => true,
        Ty::Adt(_, args) | Ty::Tuple(args) => args.iter().any(mentions_memo),
        Ty::Fun(a, _, _, b) | Ty::Pi { dom: a, cod: b, .. } => {
            mentions_memo(a) || mentions_memo(b)
        }
        Ty::Sigma { dom, cod, .. } => mentions_memo(dom) || mentions_memo(cod),
        Ty::Forall(scheme) => mentions_memo(&scheme.ty),
        Ty::Thunk { inner, .. } | Ty::Later(inner) => mentions_memo(inner),
        Ty::Ref(_, i) | Ty::Mut(_, i) => mentions_memo(i),
        Ty::Ptr { pointee, .. } => mentions_memo(pointee),
        Ty::Handler { input, output, .. } => {
            mentions_memo(input) || mentions_memo(output)
        }
        Ty::App(h, args) => mentions_memo(h) || args.iter().any(mentions_memo),
        _ => false,
    }
}

/// A type or row metavariable, for generalization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EitherMeta {
    Type(MetaId),
    Row(RowMetaId),
}

impl EffRow {
    pub fn free_metas(&self, out: &mut Vec<EitherMeta>) {
        if let RowTail::Meta(m) = self.tail {
            out.push(EitherMeta::Row(m));
        }
    }
}

/// Whether a type needs parentheses when it appears as an ARGUMENT of an
/// applied type constructor (`List (List a)`, `Option (List a)`,
/// `List (a -> b)` in the arrows it emits itself). Types that already
/// print their own delimiters (tuples, arrows, Pis, Sigmas, handlers) do
/// not; a nullary constructor or a variable is atomic. Without this the
/// rendered contract text is not re-parseable, which matters because an
/// interface's rendered scheme is the contract a consumer reads.
fn needs_parens_as_argument(t: &Ty) -> bool {
    match t {
        Ty::Adt(_, args) => !args.is_empty(),
        Ty::App(..)
        | Ty::Thunk { .. }
        | Ty::Later(_)
        | Ty::Lazy { .. }
        | Ty::Ref(..)
        | Ty::Mut(..)
        | Ty::Ptr { .. }
        | Ty::Handler { .. } => true,
        _ => false,
    }
}

/// `Display` writes every nominal by its identity: the rendering an
/// interface, a Core dump and every other machine-read text carries. A
/// diagnostic renders through `diagnostic!` instead (`crate::spelling`).
impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.spell(f, Spelling::Identity)
    }
}

impl Spell for Ty {
    fn spell(&self, f: &mut fmt::Formatter<'_>, s: Spelling<'_>) -> fmt::Result {
        match self {
            Ty::Int => write!(f, "Int"),
            Ty::Nat => write!(f, "Nat"),
            Ty::Bool => write!(f, "Bool"),
            Ty::Unit => write!(f, "Unit"),
            Ty::Bytes => write!(f, "Bytes"),
            Ty::Sym => write!(f, "Sym"),
            Ty::Prim(scalar) => write!(f, "{scalar}"),
            Ty::Param(p) => write!(f, "{p}"),
            Ty::Region(r) => write!(f, "{r}"),
            Ty::Meta(MetaId(m)) => write!(f, "?m{m}"),
            Ty::Adt(t, args) => {
                // The anonymous former prints as it is written: the
                // predicate parenthesized and a renamed binder (`$it`)
                // back to a plain name, so an interface's rendering reads
                // back.
                if let (true, [Ty::Prp(p), base]) = (t == "refined", args.as_slice()) {
                    let mut free = Vec::new();
                    p.free_vars(&mut free);
                    let mut binder = "r".to_string();
                    while free.contains(&binder) { binder.push_str("-r"); }
                    let p = p.subst("$it", &ITerm::Var(binder.into()));
                    return if needs_parens_as_argument(base) {
                        write!(f, "refined ({}) ({})", s.of(&p), s.of(base))
                    } else {
                        write!(f, "refined ({}) {}", s.of(&p), s.of(base))
                    };
                }
                s.nominal(f, t)?;
                for a in args {
                    if needs_parens_as_argument(a) {
                        write!(f, " ({})", s.of(a))?;
                    } else {
                        write!(f, " {}", s.of(a))?;
                    }
                }
                Ok(())
            }
            Ty::App(h, args) => {
                write!(f, "({}", s.of(h))?;
                for a in args {
                    if needs_parens_as_argument(a) {
                        write!(f, " ({})", s.of(a))?;
                    } else {
                        write!(f, " {}", s.of(a))?;
                    }
                }
                write!(f, ")")
            }
            Ty::Former(Former::Arrow) => write!(f, "(->)"),
            Ty::Former(Former::Tuple(n)) => {
                write!(f, "(")?;
                for _ in 0..*n {
                    write!(f, ",")?;
                }
                write!(f, ")")
            }
            Ty::Tuple(items) => {
                write!(f, "(")?;
                for (i, a) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", s.of(a))?;
                }
                write!(f, ")")
            }
            // A result refinement naming arguments by position prints
            // with those arguments named (`(a0 :: I64) -> refined (r ==
            // a0) Nat`), the form it was written in.
            Ty::Fun(..) if mentions_positional_result(self) => write!(f, "{}", s.of(&name_positional(self, 0))),
            Ty::Fun(a, m, e, b) => match (m, e.is_pure()) {
                (Mult::Many, true) => write!(f, "({} -> {})", s.of(a), s.of(b)),
                (Mult::Many, false) => write!(f, "({} ->{e} {})", s.of(a), s.of(b)),
                (_, true) => write!(f, "({} {m} -> {})", s.of(a), s.of(b)),
                (_, false) => write!(f, "({} {m} ->{e} {})", s.of(a), s.of(b)),
            },
            Ty::Thunk { eff, inner } => write!(f, "Thunk {eff} {}", s.of(inner)),
            Ty::Later(inner) => write!(f, "Later {}", s.of(inner)),
            Ty::Lazy { eff, region, inner } => write!(f, "Lazy {eff} {region} {}", s.of(inner)),
            Ty::Ref(region, inner) => write!(f, "Ref {region} {}", s.of(inner)),
            Ty::Mut(region, inner) => write!(f, "Mut {region} {}", s.of(inner)),
            Ty::Ptr { role, pointee } => match pointee.as_ref() {
                Ty::Ptr { .. } | Ty::Fun(..) | Ty::Pi { .. } | Ty::Sigma { .. } => {
                    write!(f, "{role} ({})", s.of(pointee))
                }
                other => write!(f, "{role} {}", s.of(other)),
            },
            Ty::Handler { label, input, output, eff, resumption } => {
                write!(f, "(handle {label} {} => {} adding {eff} resuming {resumption})", s.of(input), s.of(output))
            }
            Ty::Sort(sort) => write!(f, "{sort}"),
            Ty::Idx(t) => write!(f, "{}", s.of(t)),
            Ty::Prp(p) => write!(f, "{}", s.of(p)),
            Ty::Pi { var, dom, mult, eff, cod } => {
                if *mult == Mult::Many && eff.is_pure() {
                    write!(f, "(({var} :: {}) -> {})", s.of(dom), s.of(cod))
                } else {
                    write!(f, "(({var} :: {}) {mult} ->{eff} {})", s.of(dom), s.of(cod))
                }
            }
            Ty::Sigma { var, dom, cod } => {
                write!(f, "(Sigma ({var} :: {}). {})", s.of(dom), s.of(cod))
            }
            Ty::Forall(scheme) => {
                // Binders print by their source spelling, renamed apart from
                // every free name of the body, so the rendering (an
                // interface's scheme text) parses back to the same polytype.
                let mut renaming = VarRenaming::default();
                let mut spelled: Vec<String> = Vec::new();
                let mut write_binder = |f: &mut fmt::Formatter<'_>, name: &String, kind: u8| -> fmt::Result {
                    let base = skolem_spelling(name).to_string();
                    let mut candidate = base.clone();
                    let mut n = 0;
                    while spelled.contains(&candidate) || (candidate != *name && scheme.ty.mentions_param(&candidate)) {
                        n += 1;
                        candidate = format!("{base}{n}");
                    }
                    let target: Interned = candidate.clone().into();
                    match kind {
                        0 => { renaming.types.insert(name.clone(), target); }
                        1 => { renaming.rows.insert(name.clone(), target); }
                        _ => { renaming.regions.insert(name.clone(), target); }
                    }
                    write!(f, " {candidate}")?;
                    spelled.push(candidate);
                    Ok(())
                };
                write!(f, "(forall")?;
                for name in &scheme.params { write_binder(f, name, 0)?; }
                for name in &scheme.row_params { write_binder(f, name, 1)?; }
                for name in &scheme.region_params { write_binder(f, name, 2)?; }
                write!(f, ". {})", s.of(&scheme.ty.rename_vars(&renaming)))
            }
        }
    }
}

/// The source spelling of a skolem or nested binder name (`a` for
/// `a#f3#17` and for a comparison's `$q4.a`).
pub fn skolem_spelling(name: &str) -> &str {
    let name = name.rsplit_once('.').map_or(name, |(_, last)| last);
    name.split('#').next().unwrap_or(name)
}

/// A renaming of free type parameters, rigid row variables and region
/// names, as [`Ty::rename_vars`] applies it.
#[derive(Clone, Debug, Default)]
pub struct VarRenaming {
    pub types: HashMap<String, Interned>,
    pub rows: HashMap<String, Interned>,
    pub regions: HashMap<String, Interned>,
}

impl VarRenaming {
    /// This renaming without the names a nested quantifier binds.
    fn under(&self, scheme: &Scheme) -> VarRenaming {
        let mut inner = self.clone();
        for p in &scheme.params { inner.types.remove(p); }
        for p in &scheme.row_params { inner.rows.remove(p); }
        for p in &scheme.region_params { inner.regions.remove(p); }
        inner
    }

    fn region(&self, r: &Interned) -> Interned {
        self.regions.get(r.as_str()).cloned().unwrap_or_else(|| r.clone())
    }

    fn row(&self, row: &EffRow) -> EffRow {
        EffRow {
            labels: row.labels.iter().map(|l| EffLabel { head: l.head.clone(), arg: l.arg.as_ref().map(|a| self.region(a)) }).collect(),
            tail: match &row.tail {
                RowTail::Rigid(t) => RowTail::Rigid(self.rows.get(t.as_str()).cloned().unwrap_or_else(|| t.clone())),
                other => other.clone(),
            },
        }
    }
}

impl Ty {
    /// Whether a polytype occurs anywhere inside this type (the
    /// predicativity check of design/rank-n-polymorphism.md section 3).
    pub fn contains_forall(&self) -> bool {
        let mut found = false;
        self.visit_shallow(&mut |t| found |= matches!(t, Ty::Forall(_)) || t.contains_forall());
        found
    }

    /// Whether a rigid type parameter of this name occurs free.
    pub fn mentions_param(&self, name: &str) -> bool {
        match self {
            Ty::Param(p) => p.as_str() == name,
            Ty::Forall(scheme) if scheme.params.iter().any(|p| p == name) => false,
            _ => {
                let mut found = false;
                self.visit_shallow(&mut |t| found |= t.mentions_param(name));
                found
            }
        }
    }

    /// Visit the immediate component types (not index or proposition
    /// terms, which carry no polytypes and no rigid type parameters that
    /// a polytype could bind).
    fn visit_shallow(&self, f: &mut impl FnMut(&Ty)) {
        match self {
            Ty::Adt(_, args) | Ty::Tuple(args) => args.iter().for_each(|a| f(a)),
            Ty::App(h, args) => { f(h); args.iter().for_each(|a| f(a)); }
            Ty::Fun(a, _, _, b) | Ty::Pi { dom: a, cod: b, .. } | Ty::Sigma { dom: a, cod: b, .. } => { f(a); f(b); }
            Ty::Thunk { inner, .. } | Ty::Later(inner) | Ty::Lazy { inner, .. } | Ty::Ref(_, inner) | Ty::Mut(_, inner) => f(inner),
            Ty::Ptr { pointee, .. } => f(pointee),
            Ty::Handler { input, output, .. } => { f(input); f(output); }
            Ty::Forall(scheme) => f(&scheme.ty),
            _ => {}
        }
    }

    /// Rename free type parameters, rigid row variables and region names.
    /// A nested quantifier's own binders shadow the renaming. The new
    /// names are always fresh skolems, so no capture can occur.
    pub fn rename_vars(&self, r: &VarRenaming) -> Ty {
        let go = |t: &Ty| t.rename_vars(r);
        match self {
            Ty::Param(p) => match r.types.get(p.as_str()) { Some(n) => Ty::Param(n.clone()), None => self.clone() },
            Ty::Region(x) => Ty::Region(r.region(x)),
            Ty::Adt(n, args) => Ty::Adt(n.clone(), args.iter().map(go).collect()),
            Ty::Tuple(args) => Ty::Tuple(args.iter().map(go).collect()),
            Ty::App(h, args) => Ty::App(Box::new(go(h)), args.iter().map(go).collect()),
            Ty::Fun(a, m, e, b) => Ty::Fun(Box::new(go(a)), *m, r.row(e), Box::new(go(b))),
            Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi { var: var.clone(), dom: Box::new(go(dom)), mult: *mult, eff: r.row(eff), cod: Box::new(go(cod)) },
            Ty::Sigma { var, dom, cod } => Ty::Sigma { var: var.clone(), dom: Box::new(go(dom)), cod: Box::new(go(cod)) },
            Ty::Thunk { eff, inner } => Ty::Thunk { eff: r.row(eff), inner: Box::new(go(inner)) },
            Ty::Later(inner) => Ty::Later(Box::new(go(inner))),
            Ty::Lazy { eff, region, inner } => Ty::Lazy { eff: r.row(eff), region: r.region(region), inner: Box::new(go(inner)) },
            Ty::Ref(x, inner) => Ty::Ref(r.region(x), Box::new(go(inner))),
            Ty::Mut(x, inner) => Ty::Mut(r.region(x), Box::new(go(inner))),
            Ty::Ptr { role, pointee } => Ty::Ptr { role: role.clone(), pointee: Box::new(go(pointee)) },
            Ty::Handler { label, input, output, eff, resumption } => Ty::Handler {
                label: EffLabel { head: label.head.clone(), arg: label.arg.as_ref().map(|a| r.region(a)) },
                input: Box::new(go(input)), output: Box::new(go(output)), eff: r.row(eff), resumption: *resumption,
            },
            Ty::Idx(term) => Ty::Idx(Box::new(term.map_types(&mut |t| t.rename_vars(r)))),
            Ty::Prp(prop) => Ty::Prp(Box::new(prop.map_types(&mut |t| t.rename_vars(r)))),
            Ty::Forall(scheme) => {
                let inner = r.under(scheme);
                scheme.map_forall_body(|body| body.rename_vars(&inner))
            }
            _ => self.clone(),
        }
    }
}

/// Whether an arrow chain's final result mentions a positional argument
/// name (`$a{i}`).
fn mentions_positional_result(t: &Ty) -> bool {
    match t {
        Ty::Fun(_, _, _, c) | Ty::Pi { cod: c, .. } => mentions_positional_result(c),
        Ty::Adt(n, args) if n == "refined" => match args.as_slice() {
            [Ty::Prp(p), _] => {
                let mut fv = Vec::new();
                p.free_vars(&mut fv);
                fv.iter().any(|v| is_positional_arg(v))
            }
            _ => false,
        },
        _ => false,
    }
}

/// The arrow chain with each positionally named argument (`$a{i}`) bound
/// by a dependent arrow named `a{i}`, for display.
fn name_positional(t: &Ty, i: usize) -> Ty {
    match t {
        Ty::Fun(a, m, e, b) => {
            let pos = format!("$a{i}");
            let name = format!("a{i}");
            let cod = name_positional(b, i + 1);
            if cod.mentions_idx(&pos) {
                Ty::Pi {
                    var: name.clone().into(),
                    dom: a.clone(),
                    mult: *m,
                    eff: e.clone(),
                    cod: Box::new(cod.subst_idx(&pos, &ITerm::Var(name.into()))),
                }
            } else {
                Ty::Fun(a.clone(), *m, e.clone(), Box::new(cod))
            }
        }
        Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi {
            var: var.clone(),
            dom: dom.clone(),
            mult: *mult,
            eff: eff.clone(),
            cod: Box::new(name_positional(cod, i + 1)),
        },
        other => other.clone(),
    }
}

/// A class application in a constraint, e.g. `Add a`.
#[derive(Clone, Debug, PartialEq)]
pub struct ClassApp {
    pub class: String,
    pub args: Vec<Ty>,
}

/// `Display` writes the class key and every argument by identity; a
/// diagnostic renders through `diagnostic!` (`crate::spelling`).
impl fmt::Display for ClassApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.spell(f, Spelling::Identity)
    }
}

impl Spell for ClassApp {
    fn spell(&self, f: &mut fmt::Formatter<'_>, s: Spelling<'_>) -> fmt::Result {
        s.nominal(f, &self.class)?;
        for a in &self.args {
            if needs_parens_as_argument(a) {
                write!(f, " ({})", s.of(a))?;
            } else {
                write!(f, " {}", s.of(a))?;
            }
        }
        Ok(())
    }
}

/// Index-level quantification (dependent core kernel design, section 2):
/// `idx_params` are schema-level index and proposition parameters
/// (constructor family parameters and constructor-local `forall` binders);
/// they elaborate to metavariables and take no runtime argument.
#[derive(Clone, Debug, PartialEq)]
pub enum IxParam {
    /// `forall k.` or `forall (n :: Nat).`, with its value type.
    Val(String, Ty),
    /// `forall (p :: Prop).`
    Prop(String),
}

impl IxParam {
    pub fn name(&self) -> &str {
        match self {
            IxParam::Val(n, _) | IxParam::Prop(n) => n,
        }
    }
}

/// A refinement obligation of a `using` context (refinement types
/// design, "instance heads at refinements"): either a decidable
/// predicate over the carrier values (`Ordered xs`, `Sorted xs`) or the
/// generic transfer's preservation obligation (`Preserves (<>) p`).
#[derive(Clone, Debug, PartialEq)]
pub enum PropOblig {
    /// Every carrier-typed argument must establish the predicate (the
    /// binder is the predicate's own: the obligation names no argument).
    Pred(PTerm),
    /// The generic transfer's validity condition: the named method must
    /// preserve the predicate (the closure `post p p` implies `p`).
    Preserves { method: String, pred: PTerm },
}

/// A polymorphic scheme. `params`, `row_params` and `region_params` are the
/// quantified type, row and region variables; `ctx` holds `using`
/// constraints. `val_params` are implicit value arguments (a
/// `forall (n :: Nat)`): inserted at every call and passed at runtime
/// (core specification section 3: witnesses execution needs are retained);
/// `prop_params` are erased proposition binders.
#[derive(Clone, Debug, PartialEq)]
pub struct Scheme {
    pub params: Vec<String>,
    /// Erased dictionary identities quantified by a native Code carrier.
    /// This namespace is separate from source value/index parameter names.
    pub evidence_params: Vec<IndexEvidence>,
    /// Resolved higher-kinded type parameters and their argument counts.
    /// The ordinary type parameters in `params` have kind Type.
    pub con_params: Vec<(String, usize)>,
    /// Schema-level index parameters (no runtime argument).
    pub idx_params: Vec<IxParam>,
    /// Implicit runtime value arguments, name and type.
    pub val_params: Vec<(String, Ty)>,
    /// Implicit proposition binders (`forall (p :: Prop)`): erased.
    pub prop_params: Vec<String>,
    pub row_params: Vec<String>,
    pub region_params: Vec<String>,
    pub ctx: Vec<(String, ClassApp)>,
    /// Refinement obligations of the signature's `using` context (the
    /// caller discharges them through the `Dec` evidence path).
    pub obligs: Vec<PropOblig>,
    pub ty: Ty,
}

impl Scheme {
    /// A polytype with this scheme's binders over a transformed body.
    pub fn map_forall_body(&self, f: impl FnOnce(&Ty) -> Ty) -> Ty {
        Ty::Forall(Box::new(Scheme { ty: f(&self.ty), ..self.clone() }))
    }

    /// Whether this scheme binds only what a nested quantifier may bind
    /// (design/rank-n-polymorphism.md section 1).
    pub fn is_nested_quantifier(&self) -> bool {
        self.evidence_params.is_empty() && self.idx_params.is_empty() && self.val_params.is_empty()
            && self.prop_params.is_empty() && self.ctx.is_empty() && self.obligs.is_empty()
    }

    pub fn mono(ty: Ty) -> Scheme {
        Scheme {
            evidence_params: Vec::new(),
            con_params: Vec::new(),
            params: Vec::new(),
            idx_params: Vec::new(),
            val_params: Vec::new(),
            prop_params: Vec::new(),
            row_params: Vec::new(),
            region_params: Vec::new(),
            ctx: Vec::new(),
            obligs: Vec::new(),
            ty,
        }
    }
}

/// Rewrite the region-classified parameters of a converted type into
/// runtime identities: `Io r`'s argument and effect label arguments are
/// regions, so a parameter used in those positions is a region everywhere
/// it appears in argument position (decision D21).
pub fn regionize(t: &Ty, regions: &[String]) -> Ty {
    match t {
        Ty::Idx(term) => Ty::Idx(Box::new(term.map_types(&mut |ty| regionize(ty, regions)))),
        Ty::Prp(prop) => Ty::Prp(Box::new(prop.map_types(&mut |ty| regionize(ty, regions)))),
        Ty::Param(p) if regions.iter().any(|region| region.as_str() == p.as_str()) => Ty::Region(*p),
        Ty::Adt(n, args) => {
            Ty::Adt(n.clone(), args.iter().map(|a| regionize(a, regions)).collect())
        }
        Ty::Tuple(args) => Ty::Tuple(args.iter().map(|a| regionize(a, regions)).collect()),
        Ty::Fun(a, m, e, b) => Ty::fun_mult(regionize(a, regions), *m, e.clone(), regionize(b, regions)),
        Ty::Thunk { eff, inner } => Ty::Thunk {
            eff: eff.clone(),
            inner: Box::new(regionize(inner, regions)),
        },
        Ty::Later(inner) => Ty::Later(Box::new(regionize(inner, regions))),
        Ty::Lazy { eff, region, inner } => Ty::Lazy {
            eff: eff.clone(),
            region: region.clone(),
            inner: Box::new(regionize(inner, regions)),
        },
        Ty::Ref(region, inner) => {
            Ty::Ref(region.clone(), Box::new(regionize(inner, regions)))
        }
        Ty::Mut(region, inner) => {
            Ty::Mut(region.clone(), Box::new(regionize(inner, regions)))
        }
        Ty::Ptr { role, pointee } => Ty::Ptr {
            role: role.clone(),
            pointee: Box::new(regionize(pointee, regions)),
        },
        Ty::Handler { label, input, output, eff, resumption } => Ty::Handler {
            label: label.clone(),
            input: Box::new(regionize(input, regions)),
            output: Box::new(regionize(output, regions)),
            eff: eff.clone(),
            resumption: *resumption,
        },
        Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi {
            var: var.clone(),
            dom: Box::new(regionize(dom, regions)),
            mult: *mult,
            eff: eff.clone(),
            cod: Box::new(regionize(cod, regions)),
        },
        Ty::Sigma { var, dom, cod } => Ty::Sigma {
            var: var.clone(),
            dom: Box::new(regionize(dom, regions)),
            cod: Box::new(regionize(cod, regions)),
        },
        Ty::Forall(scheme) => scheme.map_forall_body(|body| regionize(body, regions)),
        other => other.clone(),
    }
}

/// [`regionize`] for a class application.
pub fn regionize_class(c: &ClassApp, regions: &[String]) -> ClassApp {
    ClassApp {
        class: c.class.clone(),
        args: c.args.iter().map(|a| regionize(a, regions)).collect(),
    }
}

/// Collect the row-variable and region names a type mentions (used to
/// classify `forall` binders after conversion).
pub fn collect_row_names(t: &Ty, rows: &mut Vec<String>, regions: &mut Vec<String>) {
    let note_row = |r: &EffRow, rows: &mut Vec<String>, regions: &mut Vec<String>| {
        for l in &r.labels {
            if let Some(a) = &l.arg {
                if !regions.iter().any(|r| r.as_str() == a.as_str()) {
                    regions.push(a.clone().to_string());
                }
            }
        }
        if let RowTail::Rigid(t) = &r.tail {
            if !rows.iter().any(|r| r.as_str() == t.as_str()) {
                rows.push(t.clone().to_string());
            }
        }
    };
    match t {
        Ty::Idx(term) => term.visit_types(&mut |ty| collect_row_names(ty, rows, regions)),
        Ty::Prp(prop) => prop.visit_types(&mut |ty| collect_row_names(ty, rows, regions)),
        Ty::Fun(a, _, e, b) => {
            collect_row_names(a, rows, regions);
            note_row(e, rows, regions);
            collect_row_names(b, rows, regions);
        }
        Ty::Thunk { eff, inner } => {
            note_row(eff, rows, regions);
            collect_row_names(inner, rows, regions);
        }
        Ty::Later(inner) => {
            collect_row_names(inner, rows, regions);
        }
        Ty::Lazy { eff, region, inner } => {
            note_row(eff, rows, regions);
            if !regions.iter().any(|present| present.as_str() == region.as_str()) {
                regions.push(region.to_string());
            }
            collect_row_names(inner, rows, regions);
        }
        Ty::Ref(region, inner) | Ty::Mut(region, inner) => {
            if !regions.iter().any(|present| present.as_str() == region.as_str()) {
                regions.push(region.to_string());
            }
            collect_row_names(inner, rows, regions);
        }
        Ty::Ptr { pointee, .. } => {
            collect_row_names(pointee, rows, regions);
        }
        Ty::Adt(_, args) | Ty::Tuple(args) => {
            for a in args {
                collect_row_names(a, rows, regions);
            }
        }
        Ty::Region(r) => {
            if !regions.iter().any(|present| present.as_str() == r.as_str()) {
                regions.push(r.to_string());
            }
        }
        Ty::Handler { label, input, output, eff, .. } => {
            if let Some(a) = &label.arg {
                if !regions.iter().any(|r| r.as_str() == a.as_str()) {
                    regions.push(a.to_string());
                }
            }
            note_row(eff, rows, regions);
            collect_row_names(input, rows, regions);
            collect_row_names(output, rows, regions);
        }
        Ty::Pi { dom, eff, cod, .. } => {
            collect_row_names(dom, rows, regions);
            note_row(eff, rows, regions);
            collect_row_names(cod, rows, regions);
        }
        Ty::Sigma { dom, cod, .. } => {
            collect_row_names(dom, rows, regions);
            collect_row_names(cod, rows, regions);
        }
        // A nested quantifier's own rows and regions are not the
        // signature's: only its free names classify outer binders.
        Ty::Forall(scheme) => {
            let (mut inner_rows, mut inner_regions) = (Vec::new(), Vec::new());
            collect_row_names(&scheme.ty, &mut inner_rows, &mut inner_regions);
            for row in inner_rows {
                if !scheme.row_params.contains(&row) && !rows.contains(&row) { rows.push(row); }
            }
            for region in inner_regions {
                if !scheme.region_params.contains(&region) && !regions.contains(&region) { regions.push(region); }
            }
        }
        _ => {}
    }
}

/// Multiplicity unification: the internal `Flex` takes the other side's
/// class; otherwise two arrows must agree on their parameter's usage.
fn unify_mults(a: Mult, b: Mult) -> std::result::Result<(), String> {
    if a == b || a == Mult::Flex || b == Mult::Flex {
        Ok(())
    } else {
        Err(format!("multiplicity mismatch: {a} vs {b}"))
    }
}

/// Hindley-Milner style substitution with row typing.
///
/// Speculation uses snapshots instead of copies: `snapshot` opens a
/// scope, every mutation made while any scope is open is recorded in
/// `undo`, and `rollback_to` replays those records backwards to restore
/// the exact state the snapshot saw (lengths, bindings, lacks order and
/// region identities). `commit` keeps the mutations. Scopes nest and
/// close in LIFO order. With no open scope nothing is recorded.
#[derive(Clone, Default)]
pub struct Unifier {
    provisional_declarations: bool,
    subst: ChunkVec<Option<Ty>>,
    row_subst: ChunkVec<Option<EffRow>>,
    /// Tail metavariables that must lack a label (memo black-hole rule).
    lacks: Vec<(RowMetaId, EffLabel)>,
    /// Region-name substitution: a flexible name (created when a scheme
    /// is instantiated) may be identified with one rigid region.
    region_subst: HashMap<Interned, Interned>,
    /// Region names that may still be identified with another region.
    flex_regions: Vec<Interned>,
    /// Inference variables for index values (implicit index arguments).
    idx_subst: ChunkVec<Option<ITerm>>,
    /// Inference variables for propositions (implicit evidence types).
    prop_subst: ChunkVec<Option<PTerm>>,
    /// How many bound entries of `subst`, `idx_subst` and `prop_subst`
    /// mention provisional declaration evidence.
    pending_bindings: usize,
    /// Inverse of every mutation made since the outermost open snapshot,
    /// oldest first. Empty whenever `open_snapshots` is zero.
    undo: Vec<Undo>,
    /// Number of snapshots taken and not yet rolled back or committed.
    open_snapshots: usize,
    /// Skolem scopes now open, innermost last: polytype comparisons and
    /// generalisation sites. A metavariable created before a scope opened
    /// (its id is below the scope's watermark) may not be bound to a type
    /// mentioning that scope's skolems (design/rank-n-polymorphism.md
    /// section 3).
    skolem_scopes: Vec<(usize, Vec<String>)>,
    /// Names the next polytype comparison's skolems.
    forall_counter: usize,
    /// Metavariables promoted to an older level: binding an outer
    /// metavariable to a type mentioning an inner one makes the inner one
    /// as old as the outer, so it cannot later carry a skolem out by the
    /// back door (level adjustment, as in OCaml's generalisation).
    level_floor: HashMap<usize, usize>,
}

/// Entries per `ChunkVec` chunk.
const CHUNK: usize = 64;

/// A vector stored as shared fixed-size chunks. Cloning copies one pointer
/// per chunk, not the elements; a write copies only the chunk it touches,
/// and only while another clone still shares it. Every chunk but the last
/// holds exactly `CHUNK` elements, and the last is never empty.
#[derive(Clone)]
struct ChunkVec<T> {
    chunks: Vec<std::sync::Arc<Vec<T>>>,
    len: usize,
}

impl<T> Default for ChunkVec<T> {
    fn default() -> Self {
        Self { chunks: Vec::new(), len: 0 }
    }
}

impl<T: Clone> ChunkVec<T> {
    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len { Some(&self.chunks[index / CHUNK][index % CHUNK]) } else { None }
    }

    fn push(&mut self, value: T) {
        if self.len % CHUNK == 0 {
            self.chunks.push(std::sync::Arc::new(Vec::with_capacity(CHUNK)));
        }
        std::sync::Arc::make_mut(self.chunks.last_mut().expect("a chunk with room")).push(value);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        let last = self.chunks.last_mut()?;
        let value = std::sync::Arc::make_mut(last).pop();
        self.len -= 1;
        if self.len % CHUNK == 0 {
            self.chunks.pop();
        }
        value
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.chunks.iter().flat_map(|chunk| chunk.iter())
    }
}

impl<T> std::ops::Index<usize> for ChunkVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < self.len, "index {index} out of bounds for length {}", self.len);
        &self.chunks[index / CHUNK][index % CHUNK]
    }
}

impl<T: Clone> std::ops::IndexMut<usize> for ChunkVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        assert!(index < self.len, "index {index} out of bounds for length {}", self.len);
        &mut std::sync::Arc::make_mut(&mut self.chunks[index / CHUNK])[index % CHUNK]
    }
}

/// The inverse of one `Unifier` mutation.
#[derive(Clone)]
enum Undo {
    /// A metavariable's level before a skolem-scope promotion.
    Promote(usize, Option<usize>),
    Provisional(bool),
    NewType,
    BindType(usize, Option<Ty>),
    NewRow,
    BindRow(usize, Option<EffRow>),
    NewIndex,
    BindIndex(usize, Option<ITerm>),
    NewProp,
    BindProp(usize, Option<PTerm>),
    PushLacks,
    ReplaceLacks(Vec<(RowMetaId, EffLabel)>),
    BindRegion(Interned, Option<Interned>),
    NewFlexRegion,
}

/// An open speculation scope on a `Unifier`. It must be closed exactly
/// once, by `Unifier::rollback_to` or `Unifier::commit`, after every
/// snapshot taken later has been closed.
#[must_use = "a snapshot must be rolled back or committed"]
pub struct Snapshot {
    undo_len: usize,
    depth: usize,
}

impl Unifier {
    /// Open a speculation scope. Mutations from now on can be undone with
    /// `rollback_to` until the scope is closed.
    pub fn snapshot(&mut self) -> Snapshot {
        self.open_snapshots += 1;
        Snapshot { undo_len: self.undo.len(), depth: self.open_snapshots }
    }

    /// Undo every mutation made since `snapshot` and close it.
    pub fn rollback_to(&mut self, snapshot: Snapshot) {
        assert_eq!(snapshot.depth, self.open_snapshots, "unifier snapshots close in LIFO order");
        while self.undo.len() > snapshot.undo_len {
            match self.undo.pop().expect("length checked") {
                Undo::Provisional(old) => self.provisional_declarations = old,
                Undo::Promote(id, old) => match old {
                    Some(level) => { self.level_floor.insert(id, level); }
                    None => { self.level_floor.remove(&id); }
                },
                Undo::NewType => { self.subst.pop(); }
                Undo::BindType(id, old) => { self.set_type(id, old); }
                Undo::NewRow => { self.row_subst.pop(); }
                Undo::BindRow(id, old) => self.row_subst[id] = old,
                Undo::NewIndex => { self.idx_subst.pop(); }
                Undo::BindIndex(id, old) => { self.set_index(id, old); }
                Undo::NewProp => { self.prop_subst.pop(); }
                Undo::BindProp(id, old) => { self.set_prop(id, old); }
                Undo::PushLacks => { self.lacks.pop(); }
                Undo::ReplaceLacks(old) => self.lacks = old,
                Undo::BindRegion(name, old) => match old {
                    Some(old) => { self.region_subst.insert(name, old); }
                    None => { self.region_subst.remove(&name); }
                },
                Undo::NewFlexRegion => { self.flex_regions.pop(); }
            }
        }
        self.open_snapshots -= 1;
    }

    /// Keep every mutation made since `snapshot` and close it. An
    /// enclosing snapshot can still roll them back.
    pub fn commit(&mut self, snapshot: Snapshot) {
        assert_eq!(snapshot.depth, self.open_snapshots, "unifier snapshots close in LIFO order");
        self.open_snapshots -= 1;
        if self.open_snapshots == 0 {
            self.undo.clear();
        }
    }

    /// Record the inverse of a mutation, if any snapshot is open.
    #[inline]
    fn log(&mut self, undo: impl FnOnce() -> Undo) {
        if self.open_snapshots > 0 {
            self.undo.push(undo());
        }
    }

    /// Replace a type binding, keeping `pending_bindings` exact.
    fn set_type(&mut self, id: usize, value: Option<Ty>) -> Option<Ty> {
        let added = value.as_ref().is_some_and(Ty::has_declaration_pending);
        let old = std::mem::replace(&mut self.subst[id], value);
        let removed = old.as_ref().is_some_and(Ty::has_declaration_pending);
        self.pending_bindings = self.pending_bindings + usize::from(added) - usize::from(removed);
        old
    }

    /// Replace an index binding, keeping `pending_bindings` exact.
    fn set_index(&mut self, id: usize, value: Option<ITerm>) -> Option<ITerm> {
        let added = value.as_ref().is_some_and(ITerm::has_declaration_pending);
        let old = std::mem::replace(&mut self.idx_subst[id], value);
        let removed = old.as_ref().is_some_and(ITerm::has_declaration_pending);
        self.pending_bindings = self.pending_bindings + usize::from(added) - usize::from(removed);
        old
    }

    /// Replace a proposition binding, keeping `pending_bindings` exact.
    fn set_prop(&mut self, id: usize, value: Option<PTerm>) -> Option<PTerm> {
        let added = value.as_ref().is_some_and(PTerm::has_declaration_pending);
        let old = std::mem::replace(&mut self.prop_subst[id], value);
        let removed = old.as_ref().is_some_and(PTerm::has_declaration_pending);
        self.pending_bindings = self.pending_bindings + usize::from(added) - usize::from(removed);
        old
    }

    fn bind_type(&mut self, id: usize, ty: Ty) {
        let old = self.set_type(id, Some(ty));
        self.log(|| Undo::BindType(id, old));
    }

    pub(crate) fn provisional_declarations(&mut self, enabled: bool) {
        let old = std::mem::replace(&mut self.provisional_declarations, enabled);
        self.log(|| Undo::Provisional(old));
    }

    /// Does any binding mention provisional declaration evidence?
    pub(crate) fn has_declaration_pending(&self) -> bool {
        debug_assert_eq!(
            self.pending_bindings > 0,
            self.subst.iter().flatten().any(Ty::has_declaration_pending)
                || self.idx_subst.iter().flatten().any(ITerm::has_declaration_pending)
                || self.prop_subst.iter().flatten().any(PTerm::has_declaration_pending)
        );
        self.pending_bindings > 0
    }
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fresh(&mut self) -> Ty {
        let id = self.subst.len();
        self.subst.push(None);
        self.log(|| Undo::NewType);
        Ty::Meta(MetaId(id))
    }

    /// A fresh index-value metavariable, as a type (`?i` in index
    /// position).
    pub fn fresh_idx(&mut self) -> Ty {
        let id = self.idx_subst.len();
        self.idx_subst.push(None);
        self.log(|| Undo::NewIndex);
        Ty::Idx(Box::new(ITerm::MVar(id)))
    }

    /// A fresh proposition metavariable, as a type.
    pub fn fresh_prop(&mut self) -> Ty {
        let id = self.prop_subst.len();
        self.prop_subst.push(None);
        self.log(|| Undo::NewProp);
        Ty::Prp(Box::new(PTerm::MVar(id)))
    }

    /// A fresh index-value metavariable (the bare term form).
    pub fn fresh_idx_term(&mut self) -> ITerm {
        match self.fresh_idx() {
            Ty::Idx(i) => *i,
            other => panic!("fresh_idx returned {other}"),
        }
    }

    pub fn fresh_row(&mut self) -> EffRow {
        let id = self.row_subst.len();
        self.row_subst.push(None);
        self.log(|| Undo::NewRow);
        EffRow { labels: Vec::new(), tail: RowTail::Meta(RowMetaId(id)) }
    }

    /// Record a region name as flexible: the fresh name an instantiation
    /// gives a `forall`-bound region. Flexible names take the identity of
    /// the rigid region they are used at; rigid names never merge.
    pub fn note_flex_region(&mut self, name: impl Into<Interned>) {
        let name = name.into();
        if !self.flex_regions.contains(&name) {
            self.flex_regions.push(name);
            self.log(|| Undo::NewFlexRegion);
        }
    }

    /// Is this (resolved) region name still flexible: not yet pinned to a
    /// rigid identity?
    pub fn is_flex_region(&self, name: impl Into<Interned>) -> bool {
        let r = self.resolve_region(name.into());
        self.flex_regions.contains(&r)
    }

    /// Follow the region-name substitution.
    pub fn resolve_region(&self, r: impl Into<Interned>) -> Interned {
        let r = r.into();
        match self.region_subst.get(&r) {
            Some(next) => self.resolve_region(*next),
            None => r,
        }
    }

    /// Identify two region names. Equal names agree; a flexible name takes
    /// the other side's identity; two distinct rigid names (scope regions,
    /// memo groups, signature binders) never meet.
    pub fn unify_regions(&mut self, a: impl Into<Interned>, b: impl Into<Interned>) -> std::result::Result<(), String> {
        let ra = self.resolve_region(a);
        let rb = self.resolve_region(b);
        if ra == rb {
            return Ok(());
        }
        let a_flex = self.flex_regions.contains(&ra);
        let b_flex = self.flex_regions.contains(&rb);
        if a_flex {
            self.bind_region(ra, rb);
            Ok(())
        } else if b_flex {
            self.bind_region(rb, ra);
            Ok(())
        } else {
            Err(format!("regions do not match: {ra} vs {rb}"))
        }
    }

    fn bind_region(&mut self, from: Interned, to: Interned) {
        if self.open_snapshots > 0 {
            let old = self.region_subst.insert(from, to);
            self.undo.push(Undo::BindRegion(from, old));
        } else {
            self.region_subst.insert(from, to);
        }
    }

    /// Follow metavar bindings one level.
    fn shallow(&self, ty: &Ty) -> Ty {
        self.shallow_ref(ty).clone()
    }

    /// Follow metavar bindings one level without copying the result.
    fn shallow_ref<'a>(&'a self, mut ty: &'a Ty) -> &'a Ty {
        while let Ty::Meta(MetaId(m)) = ty {
            match self.subst.get(*m).and_then(|s| s.as_ref()) {
                Some(t) => ty = t,
                None => break,
            }
        }
        ty
    }

    /// Follow index metavariable bindings.
    pub fn resolve_idx(&self, t: &ITerm) -> ITerm {
        self.resolve_idx_types(t, &mut |ty| self.zonk(ty))
    }

    fn resolve_idx_types(&self, t: &ITerm, map: &mut dyn FnMut(&Ty) -> Ty) -> ITerm {
        match t {
            ITerm::Inst(call) => {
                let types = call.types.iter().map(&mut *map).collect();
                let dictionaries = call.dictionaries.iter().map(|term| self.resolve_idx_types(term, map)).collect();
                let arguments = call.arguments.iter().map(|argument| match argument {
                    IndexArgument::Value { value, carrier, runtime } => {
                        let carrier = map(carrier);
                        IndexArgument::Value { value: self.resolve_idx_types(value, map), carrier, runtime: *runtime }
                    }
                    IndexArgument::Evidence(value) => IndexArgument::Evidence(self.resolve_idx_types(value, map)),
                    _ => argument.map(map, &mut |term| term.clone()),
                }).collect();
                ITerm::Inst(Box::new(IndexInstantiation { head: call.head.clone(), types, dictionaries, arguments }))
            }
            ITerm::Lam(name, body) => ITerm::Lam(name.clone(), Box::new(self.resolve_idx_types(body, map))),
            ITerm::MVar(m) => match self.idx_subst.get(*m).and_then(|s| s.as_ref()) {
                Some(next) => self.resolve_idx_types(next, map),
                None => t.clone(),
            },
            ITerm::App(f, args) => {
                ITerm::App(
                    Box::new(self.resolve_idx_types(f, map)),
                    args.iter().map(|a| self.resolve_idx_types(a, map)).collect(),
                )
            }
            ITerm::Bin(op, a, b) => ITerm::Bin(
                *op,
                Box::new(self.resolve_idx_types(a, map)),
                Box::new(self.resolve_idx_types(b, map)),
            ),
            other => other.clone(),
        }
    }

    /// Follow proposition metavariable bindings.
    pub fn resolve_prop(&self, p: &PTerm) -> PTerm {
        self.resolve_prop_types(p, &mut |ty| self.zonk(ty))
    }

    fn resolve_prop_types(&self, p: &PTerm, map: &mut dyn FnMut(&Ty) -> Ty) -> PTerm {
        match p {
            PTerm::MVar(m) => match self.prop_subst.get(*m).and_then(|s| s.as_ref()) {
                Some(next) => self.resolve_prop_types(next, map),
                None => p.clone(),
            },
            PTerm::Cmp(op, a, b) => PTerm::Cmp(
                *op,
                Box::new(self.resolve_idx_types(a, map)),
                Box::new(self.resolve_idx_types(b, map)),
            ),
            PTerm::Eq(a, b) => {
                PTerm::Eq(Box::new(self.resolve_idx_types(a, map)), Box::new(self.resolve_idx_types(b, map)))
            }
            PTerm::Not(q) => PTerm::Not(Box::new(self.resolve_prop_types(q, map))),
            PTerm::And(a, b) => PTerm::And(
                Box::new(self.resolve_prop_types(a, map)),
                Box::new(self.resolve_prop_types(b, map)),
            ),
            PTerm::Or(a, b) => PTerm::Or(
                Box::new(self.resolve_prop_types(a, map)),
                Box::new(self.resolve_prop_types(b, map)),
            ),
            PTerm::Pred(pf, a) => PTerm::Pred(*pf, Box::new(self.resolve_idx_types(a, map))),
            other => other.clone(),
        }
    }

    fn idx_occurs(&self, id: usize, t: &ITerm) -> bool {
        match self.resolve_idx(t) {
            ITerm::Inst(call) => {
                let mut metas = Vec::new();
                for ty in call.embedded_types() { ty.free_idx_metas(&mut metas); }
                metas.contains(&id) || call.terms().any(|term| self.idx_occurs(id, term))
            }
            ITerm::Lam(_, body) => self.idx_occurs(id, &body),
            ITerm::MVar(m) => m == id,
            ITerm::App(f, args) => {
                self.idx_occurs(id, &f) || args.iter().any(|a| self.idx_occurs(id, a))
            }
            ITerm::Bin(_, a, b) => self.idx_occurs(id, &a) || self.idx_occurs(id, &b),
            _ => false,
        }
    }

    fn prop_occurs(&self, id: usize, p: &PTerm) -> bool {
        match self.resolve_prop(p) {
            PTerm::MVar(m) => m == id,
            PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
                self.prop_occurs_index(id, &a) || self.prop_occurs_index(id, &b)
            }
            PTerm::Pred(_, term) => self.prop_occurs_index(id, &term),
            PTerm::Not(q) => self.prop_occurs(id, &q),
            PTerm::And(a, b) | PTerm::Or(a, b) => {
                self.prop_occurs(id, &a) || self.prop_occurs(id, &b)
            }
            _ => false,
        }
    }

    fn prop_occurs_index(&self, id: usize, term: &ITerm) -> bool {
        let mut found = false;
        self.resolve_idx(term).visit_types(&mut |ty| found |= self.prop_occurs_type(id, ty));
        found
    }

    fn prop_occurs_type(&self, id: usize, ty: &Ty) -> bool {
        match self.shallow_ref(ty) {
            Ty::Prp(prop) => self.prop_occurs(id, &prop),
            Ty::Idx(term) => self.prop_occurs_index(id, &term),
            Ty::Adt(_, args) | Ty::Tuple(args) => args.iter().any(|ty| self.prop_occurs_type(id, ty)),
            Ty::App(head, args) => self.prop_occurs_type(id, &head) || args.iter().any(|ty| self.prop_occurs_type(id, ty)),
            Ty::Fun(dom, _, _, cod) | Ty::Pi { dom, cod, .. } | Ty::Sigma { dom, cod, .. } => self.prop_occurs_type(id, &dom) || self.prop_occurs_type(id, &cod),
            Ty::Forall(scheme) => self.prop_occurs_type(id, &scheme.ty),
            Ty::Thunk { inner, .. } | Ty::Lazy { inner, .. } | Ty::Later(inner) | Ty::Ref(_, inner) | Ty::Mut(_, inner) => self.prop_occurs_type(id, &inner),
            Ty::Ptr { pointee, .. } => self.prop_occurs_type(id, &pointee),
            Ty::Handler { input, output, .. } => self.prop_occurs_type(id, &input) || self.prop_occurs_type(id, &output),
            _ => false,
        }
    }

    /// Bind an index metavariable.
    pub fn bind_idx(&mut self, id: usize, t: ITerm) -> std::result::Result<(), String> {
        let t = self.resolve_idx(&t);
        if !self.provisional_declarations && t.has_declaration_pending() {
            return Err("provisional declaration evidence cannot bind a checked index".into());
        }
        if matches!(t, ITerm::MVar(other) if other == id) { return Ok(()); }
        if self.idx_occurs(id, &t) {
            return Err(crate::diagnostic!(
                "cannot construct infinite index: ?i{} in {}", id, t
            ));
        }
        let old = self.set_index(id, Some(t));
        self.log(|| Undo::BindIndex(id, old));
        Ok(())
    }

    /// Bind a proposition metavariable.
    pub fn bind_prop(&mut self, id: usize, p: PTerm) -> std::result::Result<(), String> {
        let p = self.resolve_prop(&p);
        if !self.provisional_declarations && p.has_declaration_pending() {
            return Err("provisional declaration evidence cannot bind a checked proposition".into());
        }
        if matches!(p, PTerm::MVar(other) if other == id) { return Ok(()); }
        if self.prop_occurs(id, &p) {
            return Err(crate::diagnostic!(
                "cannot construct infinite proposition: ?p{} in {}", id, p
            ));
        }
        let old = self.set_prop(id, Some(p));
        self.log(|| Undo::BindProp(id, old));
        Ok(())
    }

    /// Unify two index terms: metavariables bind, everything else is
    /// structural with definitional-equality validation. Heuristic
    /// bindings (inverting `?k + 1` against a numeral) are guesses that
    /// the final conversion check must confirm.
    pub fn unify_idx(&mut self, a: &ITerm, b: &ITerm) -> std::result::Result<(), String> {
        let snapshot = self.snapshot();
        if let Err(error) = self.unify_idx_raw(a, b) {
            self.rollback_to(snapshot);
            return Err(error);
        }
        let left = self.resolve_idx(a);
        let right = self.resolve_idx(b);
        if left.eq(&right) || (self.provisional_declarations && (left.has_declaration_pending() || right.has_declaration_pending())) {
            self.commit(snapshot);
            Ok(())
        } else {
            self.rollback_to(snapshot);
            Err(crate::diagnostic!(
                "index mismatch: {} vs {}",
                self.resolve_idx(a),
                self.resolve_idx(b)
            ))
        }
    }

    fn unify_idx_raw(&mut self, a: &ITerm, b: &ITerm) -> std::result::Result<(), String> {
        let a = self.resolve_idx(a);
        let b = self.resolve_idx(b);
        if !self.provisional_declarations && (a.has_declaration_pending() || b.has_declaration_pending()) {
            return Err("provisional declaration evidence is not a checked index".into());
        }
        match (&a, &b) {
            (ITerm::MVar(m), _) => self.bind_idx(*m, b),
            (_, ITerm::MVar(m)) => self.bind_idx(*m, a),
            _ if self.provisional_declarations && (a.has_declaration_pending() || b.has_declaration_pending()) => Ok(()),
            (ITerm::Var(x), ITerm::Var(y)) if x == y => Ok(()),
            (ITerm::LocalValue(x), ITerm::LocalValue(y)) if x == y => Ok(()),
            (ITerm::Evidence(x), ITerm::Evidence(y)) if x == y => Ok(()),
            (ITerm::Num(x), ITerm::Num(y)) if x == y => Ok(()),
            (ITerm::Fn(x), ITerm::Fn(y)) if x == y => Ok(()),
            (ITerm::Inst(x), ITerm::Inst(y)) if x.head == y.head && x.types.len() == y.types.len()
                && x.dictionaries.len() == y.dictionaries.len() && x.arguments.len() == y.arguments.len() && x.arguments.iter().zip(&y.arguments).all(|(a, b)| a.kind() == b.kind()) => {
                for (a, b) in x.embedded_types().zip(y.embedded_types()) { self.unify(&a, &b)?; }
                for (a, b) in x.terms().zip(y.terms()) { self.unify_idx_raw(a, b)?; }
                Ok(())
            }
            (ITerm::App(f, xs), ITerm::App(g, ys)) if xs.len() == ys.len() => {
                self.unify_idx_raw(f, g)?;
                for (x, y) in xs.iter().zip(ys) {
                    self.unify_idx_raw(x, y)?;
                }
                Ok(())
            }
            (ITerm::Bin(op, x, y), ITerm::Bin(op2, u, v)) if op == op2 => {
                self.unify_idx_raw(x, u)?;
                self.unify_idx_raw(y, v)
            }
            _ => {
                // Inversion heuristics: arithmetic with one unknown
                // reduces to the other side (validated by conversion).
                if let Some((m, rhs)) = idx_invert(&a, &b) {
                    return self.bind_idx(m, rhs);
                }
                if let Some((m, rhs)) = idx_invert(&b, &a) {
                    return self.bind_idx(m, rhs);
                }
                if a.eq(&b) {
                    Ok(())
                } else {
                    Err(crate::diagnostic!("index mismatch: {} vs {}", a, b))
                }
            }
        }
    }

    /// Unify two propositions (validated like index terms).
    pub fn unify_prop(&mut self, a: &PTerm, b: &PTerm) -> std::result::Result<(), String> {
        let snapshot = self.snapshot();
        if let Err(error) = self.unify_prop_raw(a, b) {
            self.rollback_to(snapshot);
            return Err(error);
        }
        let left = self.resolve_prop(a);
        let right = self.resolve_prop(b);
        if left.eq(&right) || (self.provisional_declarations && (left.has_declaration_pending() || right.has_declaration_pending())) {
            self.commit(snapshot);
            Ok(())
        } else {
            self.rollback_to(snapshot);
            Err(crate::diagnostic!(
                "propositions do not match: {} vs {}",
                self.resolve_prop(a),
                self.resolve_prop(b)
            ))
        }
    }

    fn unify_prop_raw(&mut self, a: &PTerm, b: &PTerm) -> std::result::Result<(), String> {
        let a = self.resolve_prop(a);
        let b = self.resolve_prop(b);
        if !self.provisional_declarations && (a.has_declaration_pending() || b.has_declaration_pending()) {
            return Err("provisional declaration evidence is not a checked proposition".into());
        }
        match (&a, &b) {
            (PTerm::MVar(m), _) => self.bind_prop(*m, b),
            (_, PTerm::MVar(m)) => self.bind_prop(*m, a),
            (PTerm::Var(x), PTerm::Var(y)) if x == y => Ok(()),
            (PTerm::Cmp(op, x, y), PTerm::Cmp(op2, u, v)) if op == op2 => {
                self.unify_idx(x, u)?;
                self.unify_idx(y, v)
            }
            (PTerm::Eq(x, y), PTerm::Eq(u, v)) => {
                self.unify_idx(x, u)?;
                self.unify_idx(y, v)
            }
            (PTerm::Not(x), PTerm::Not(y)) => self.unify_prop(x, y),
            (PTerm::And(x, y), PTerm::And(u, v)) | (PTerm::Or(x, y), PTerm::Or(u, v)) => {
                self.unify_prop(x, u)?;
                self.unify_prop(y, v)
            }
            (PTerm::Pred(pf, x), PTerm::Pred(pf2, y)) if pf == pf2 => self.unify_idx(x, y),
            _ => {
                if a.eq(&b) {
                    Ok(())
                } else {
                    Err(crate::diagnostic!("propositions do not match: {} vs {}", a, b))
                }
            }
        }
    }

    /// Definitional equality (conversion) on resolved types: beta (at
    /// elaboration), iota on numerals, proof irrelevance for `Prop`
    /// components, and syntactic equality elsewhere. Terminating by
    /// construction: conversion never unfolds user definitions
    /// (decision D28).
    pub fn conv_eq(&self, a: &Ty, b: &Ty) -> bool {
        ty_eq(&self.zonk(a), &self.zonk(b))
    }

    /// Resolve a row's tail through the row substitution, flattening the
    /// labels a bound tail contributes. Label region arguments follow the
    /// region-name substitution, so two labels of one head and one resolved
    /// identity collapse.
    pub fn resolve_row(&self, row: &EffRow) -> EffRow {
        let canon = |l: &EffLabel| EffLabel {
            head: l.head.clone(),
            arg: l.arg.as_ref().map(|a| self.resolve_region(a)),
        };
        let mut labels = Vec::new();
        for l in &row.labels {
            let l = canon(l);
            if !labels.contains(&l) {
                labels.push(l);
            }
        }
        let mut tail = row.tail.clone();
        loop {
            match tail {
                RowTail::Meta(m) => match self.row_subst.get(m.0).and_then(|s| s.as_ref()) {
                    Some(r) => {
                        for l in &r.labels {
                            let l = canon(l);
                            if !labels.contains(&l) {
                                labels.push(l);
                            }
                        }
                        tail = r.tail.clone();
                    }
                    None => break,
                },
                _ => break,
            }
        }
        labels.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
        EffRow { labels, tail }
    }

    /// Whether a type uses only type/row metavariables allocated in this
    /// store. A private inference fork may export a type only after its new
    /// variables have been solved away; this also covers rows created by
    /// unification, rather than just the initially instantiated parameters.
    pub(crate) fn owns_type_metas(&self, ty: &Ty) -> bool {
        let mut metas = Vec::new();
        ty.free_metas(&mut metas);
        metas.into_iter().all(|meta| match meta {
            EitherMeta::Type(id) => id.0 < self.subst.len(),
            EitherMeta::Row(id) => id.0 < self.row_subst.len(),
        })
    }

    /// An exported index telescope must not retain private inference
    /// variables in any namespace, including its dictionary type arguments.
    pub(crate) fn owns_all_metas(&self, ty: &Ty) -> bool {
        self.owns_type_metas(ty) && self.owns_value_metas_in_type(ty)
    }

    fn owns_value_metas_in_type(&self, ty: &Ty) -> bool {
        match ty {
            Ty::Idx(term) => self.owns_index_metas(term),
            Ty::Prp(prop) => self.owns_proposition_metas(prop),
            Ty::Adt(_, args) | Ty::Tuple(args) => args.iter().all(|ty| self.owns_value_metas_in_type(ty)),
            Ty::App(head, args) => self.owns_value_metas_in_type(head) && args.iter().all(|ty| self.owns_value_metas_in_type(ty)),
            Ty::Fun(dom, _, _, cod) | Ty::Pi { dom, cod, .. } | Ty::Sigma { dom, cod, .. } => self.owns_value_metas_in_type(dom) && self.owns_value_metas_in_type(cod),
            Ty::Forall(scheme) => self.owns_value_metas_in_type(&scheme.ty),
            Ty::Thunk { inner, .. } | Ty::Lazy { inner, .. } | Ty::Later(inner) | Ty::Ref(_, inner) | Ty::Mut(_, inner) => self.owns_value_metas_in_type(inner),
            Ty::Ptr { pointee, .. } => self.owns_value_metas_in_type(pointee),
            Ty::Handler { input, output, .. } => self.owns_value_metas_in_type(input) && self.owns_value_metas_in_type(output),
            _ => true,
        }
    }

    fn owns_index_metas(&self, term: &ITerm) -> bool {
        match term {
            ITerm::MVar(id) => *id < self.idx_subst.len(),
            ITerm::Inst(call) => call.embedded_types().all(|ty| self.owns_value_metas_in_type(&ty)) && call.terms().all(|term| self.owns_index_metas(term)),
            ITerm::App(head, args) => self.owns_index_metas(head) && args.iter().all(|term| self.owns_index_metas(term)),
            ITerm::Lam(_, body) => self.owns_index_metas(body),
            ITerm::Bin(_, a, b) => self.owns_index_metas(a) && self.owns_index_metas(b),
            _ => true,
        }
    }

    fn owns_proposition_metas(&self, prop: &PTerm) -> bool {
        match prop {
            PTerm::MVar(id) => *id < self.prop_subst.len(),
            PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => self.owns_index_metas(a) && self.owns_index_metas(b),
            PTerm::Pred(_, term) => self.owns_index_metas(term),
            PTerm::Not(prop) => self.owns_proposition_metas(prop),
            PTerm::And(a, b) | PTerm::Or(a, b) => self.owns_proposition_metas(a) && self.owns_proposition_metas(b),
            PTerm::Lit(_) | PTerm::Var(_) => true,
        }
    }

    /// Resolve all metavar bindings.
    pub fn zonk(&self, ty: &Ty) -> Ty {
        self.zonk_in(ty, None)
    }

    /// Resolve all metavar bindings, then replace the unbound
    /// metavariables `map` names by their types (themselves resolved):
    /// one instantiation of a let-generalized local's quantified
    /// metavariables.
    pub fn subst_metas(&self, ty: &Ty, map: &HashMap<MetaId, Ty>) -> Ty {
        self.zonk_in(ty, Some(map))
    }

    fn zonk_in(&self, ty: &Ty, map: Option<&HashMap<MetaId, Ty>>) -> Ty {
        let z = |t: &Ty| self.zonk_in(t, map);
        match self.shallow_ref(ty) {
            Ty::Meta(m) => match map.and_then(|mp| mp.get(m)) {
                Some(t) => self.zonk(t),
                None => Ty::Meta(*m),
            },
            Ty::Adt(t, args) => Ty::Adt(t.clone(), args.iter().map(z).collect()),
            Ty::App(h, args) => Ty::apply_spine(z(h), args.iter().map(z).collect::<Vec<_>>()),
            Ty::Tuple(args) => Ty::Tuple(args.iter().map(z).collect()),
            Ty::Fun(a, m, e, b) => Ty::fun_mult(z(a), *m, self.resolve_row(e), z(b)),
            Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi {
                var: var.clone(),
                dom: Box::new(z(&dom)),
                mult: *mult,
                eff: self.resolve_row(&eff),
                cod: Box::new(z(&cod)),
            },
            Ty::Sigma { var, dom, cod } => Ty::Sigma {
                var: var.clone(),
                dom: Box::new(z(&dom)),
                cod: Box::new(z(&cod)),
            },
            Ty::Forall(scheme) => scheme.map_forall_body(|body| z(body)),
            Ty::Idx(t) => Ty::Idx(Box::new(self.resolve_idx_types(&t, &mut |ty| self.zonk_in(ty, map)))),
            Ty::Prp(p) => Ty::Prp(Box::new(self.resolve_prop_types(&p, &mut |ty| self.zonk_in(ty, map)))),
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: self.resolve_row(&eff),
                inner: Box::new(z(&inner)),
            },
            Ty::Later(inner) => Ty::Later(Box::new(z(&inner))),
            Ty::Lazy { eff, region, inner } => Ty::Lazy {
                eff: self.resolve_row(&eff),
                region: self.resolve_region(*region),
                inner: Box::new(z(&inner)),
            },
            Ty::Ref(region, inner) => Ty::Ref(self.resolve_region(*region), Box::new(z(&inner))),
            Ty::Mut(region, inner) => Ty::Mut(self.resolve_region(*region), Box::new(z(&inner))),
            Ty::Ptr { role, pointee } => Ty::Ptr { role: role.clone(), pointee: Box::new(z(&pointee)) },
            Ty::Region(r) => Ty::Region(self.resolve_region(*r)),
            Ty::Handler { label, input, output, eff, resumption } => Ty::Handler {
                label: EffLabel {
                    head: label.head.clone(),
                    arg: label.arg.as_ref().map(|a| self.resolve_region(a)),
                },
                input: Box::new(z(&input)),
                output: Box::new(z(&output)),
                eff: self.resolve_row(&eff),
                resumption: *resumption,
            },
            t => t.clone(),
        }
    }

    pub fn zonk_class(&self, c: &ClassApp) -> ClassApp {
        ClassApp {
            class: c.class.clone(),
            args: c.args.iter().map(|a| self.zonk(a)).collect(),
        }
    }

    /// The metavariables free in a type after resolution.
    pub fn free_metas(&self, ty: &Ty) -> Vec<EitherMeta> {
        let mut out = Vec::new();
        self.zonk(ty).free_metas(&mut out);
        out.sort_by_key(|m| format!("{m:?}"));
        out.dedup();
        out
    }

    /// Replace free metavariables with bound variables, producing the body
    /// of a generalized scheme. Returns the quantified type and row names.
    pub fn generalize(&self, ty: &Ty, metas: &[EitherMeta]) -> (Ty, Vec<String>, Vec<String>) {
        let mut names: HashMap<EitherMeta, String> = HashMap::new();
        let mut ty_params = Vec::new();
        let mut row_params = Vec::new();
        for (i, m) in metas.iter().enumerate() {
            match m {
                EitherMeta::Type(_) => {
                    let n = format!("a${i}");
                    names.insert(*m, n.clone());
                    ty_params.push(n);
                }
                EitherMeta::Row(_) => {
                    let n = format!("e${i}");
                    names.insert(*m, n.clone());
                    row_params.push(n);
                }
            }
        }
        (self.gen_ty(ty, &names), ty_params, row_params)
    }

    /// Interpreter-local inference identities. The same metavariable in two
    /// quotations has one name, while unrelated metavariables never collide.
    pub(crate) fn generalize_for_code(&self, ty: &Ty) -> (Ty, Vec<String>, Vec<String>) {
        let mut names = HashMap::new();
        let mut parameters = Vec::new();
        let mut rows = Vec::new();
        for meta in self.free_metas(ty) {
            let name = match meta {
                EitherMeta::Type(MetaId(id)) => {
                    let name = format!("$code-type:{id}");
                    parameters.push(name.clone());
                    name
                }
                EitherMeta::Row(RowMetaId(id)) => {
                    let name = format!("$code-row:{id}");
                    rows.push(name.clone());
                    name
                }
            };
            names.insert(meta, name);
        }
        (self.gen_ty(ty, &names), parameters, rows)
    }

    fn gen_ty(&self, ty: &Ty, names: &HashMap<EitherMeta, String>) -> Ty {
        match self.shallow_ref(ty) {
            Ty::Meta(m) => match names.get(&EitherMeta::Type(*m)) {
                Some(n) => Ty::Param(n.clone().into()),
                None => Ty::Meta(*m),
            },
            Ty::Adt(n, args) => {
                Ty::Adt(n.clone(), args.iter().map(|a| self.gen_ty(a, names)).collect())
            }
            Ty::App(head, args) => Ty::apply_spine(
                self.gen_ty(&head, names),
                args.iter().map(|arg| self.gen_ty(arg, names)),
            ),
            Ty::Tuple(args) => {
                Ty::Tuple(args.iter().map(|a| self.gen_ty(a, names)).collect())
            }
            Ty::Fun(a, m, e, b) => Ty::fun_mult(
                self.gen_ty(&a, names),
                *m,
                self.gen_row(&e, names),
                self.gen_ty(&b, names),
            ),
            Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi {
                var: var.clone(),
                dom: Box::new(self.gen_ty(&dom, names)),
                mult: *mult,
                eff: self.gen_row(&eff, names),
                cod: Box::new(self.gen_ty(&cod, names)),
            },
            Ty::Sigma { var, dom, cod } => Ty::Sigma {
                var: var.clone(),
                dom: Box::new(self.gen_ty(&dom, names)),
                cod: Box::new(self.gen_ty(&cod, names)),
            },
            Ty::Forall(scheme) => scheme.map_forall_body(|body| self.gen_ty(body, names)),
            Ty::Idx(t) => Ty::Idx(Box::new(self.resolve_idx_types(&t, &mut |ty| self.gen_ty(ty, names)))),
            Ty::Prp(p) => Ty::Prp(Box::new(self.resolve_prop_types(&p, &mut |ty| self.gen_ty(ty, names)))),
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: self.gen_row(&eff, names),
                inner: Box::new(self.gen_ty(&inner, names)),
            },
            Ty::Later(inner) => Ty::Later(Box::new(self.gen_ty(&inner, names))),
            Ty::Lazy { eff, region, inner } => Ty::Lazy {
                eff: self.gen_row(&eff, names),
                region: region.clone(),
                inner: Box::new(self.gen_ty(&inner, names)),
            },
            Ty::Ref(region, inner) => {
                Ty::Ref(region.clone(), Box::new(self.gen_ty(&inner, names)))
            }
            Ty::Mut(region, inner) => {
                Ty::Mut(region.clone(), Box::new(self.gen_ty(&inner, names)))
            }
            Ty::Ptr { role, pointee } => Ty::Ptr {
                role: role.clone(),
                pointee: Box::new(self.gen_ty(&pointee, names)),
            },
            Ty::Handler { label, input, output, eff, resumption } => Ty::Handler {
                label: label.clone(),
                input: Box::new(self.gen_ty(&input, names)),
                output: Box::new(self.gen_ty(&output, names)),
                eff: self.gen_row(&eff, names),
                resumption: *resumption,
            },
            t => t.clone(),
        }
    }

    fn gen_row(&self, row: &EffRow, names: &HashMap<EitherMeta, String>) -> EffRow {
        let r = self.resolve_row(row);
        EffRow {
            labels: r.labels,
            tail: match r.tail {
                RowTail::Meta(m) => match names.get(&EitherMeta::Row(m)) {
                    Some(n) => RowTail::Rigid(n.clone().into()),
                    None => RowTail::Meta(m),
                },
                t => t,
            },
        }
    }

    fn occurs(&self, id: MetaId, ty: &Ty) -> bool {
        match self.shallow_ref(ty) {
            Ty::Meta(m) => *m == id,
            Ty::Idx(term) => {
                let mut found = false;
                term.visit_types(&mut |ty| found |= self.occurs(id, ty));
                found
            }
            Ty::Prp(prop) => {
                let mut found = false;
                prop.visit_types(&mut |ty| found |= self.occurs(id, ty));
                found
            }
            Ty::Adt(_, args) | Ty::Tuple(args) => args.iter().any(|a| self.occurs(id, a)),
            Ty::Fun(a, _, _, b) => self.occurs(id, &a) || self.occurs(id, &b),
            Ty::Pi { dom, cod, .. } => self.occurs(id, &dom) || self.occurs(id, &cod),
            Ty::Sigma { dom, cod, .. } => self.occurs(id, &dom) || self.occurs(id, &cod),
            Ty::Forall(scheme) => self.occurs(id, &scheme.ty),
            Ty::Thunk { inner, .. } | Ty::Later(inner) => self.occurs(id, &inner),
            Ty::Lazy { inner, .. } => self.occurs(id, &inner),
            Ty::Ref(_, inner) | Ty::Mut(_, inner) => self.occurs(id, &inner),
            Ty::Ptr { pointee, .. } => self.occurs(id, &pointee),
            Ty::Handler { input, output, .. } => {
                self.occurs(id, &input) || self.occurs(id, &output)
            }
            Ty::App(h, args) => {
                self.occurs(id, &h) || args.iter().any(|a| self.occurs(id, a))
            }
            _ => false,
        }
    }

    fn row_occurs(&self, id: RowMetaId, row: &EffRow) -> bool {
        let r = self.resolve_row(row);
        match r.tail {
            RowTail::Meta(m) => m == id,
            _ => false,
        }
    }

    /// Bind a row metavariable to a row, enforcing lacks constraints.
    fn bind_row(&mut self, id: RowMetaId, row: EffRow) -> std::result::Result<(), String> {
        let row = self.resolve_row(&row);
        if self.row_occurs(id, &row) {
            return Err("cannot construct infinite effect row".to_string());
        }
        // Pushes below append past `existing`, so only the constraints
        // present on entry are visited.
        let existing = self.lacks.len();
        for i in 0..existing {
            let m = self.lacks[i].0;
            if m == id && row.contains(&self.lacks[i].1) {
                return Err(format!(
                    "effect row must not contain {} (a memo cell cannot demand its own group)",
                    self.lacks[i].1
                ));
            }
            if let RowTail::Meta(other) = row.tail {
                if other != m {
                    // Propagate the lacks constraint to the new tail.
                    let label = self.lacks[i].1.clone();
                    self.lacks.push((other, label));
                    self.log(|| Undo::PushLacks);
                }
            }
        }
        if self.lacks.iter().any(|(m, _)| *m == id) {
            if self.open_snapshots > 0 {
                self.undo.push(Undo::ReplaceLacks(self.lacks.clone()));
            }
            self.lacks.retain(|(m, _)| *m != id);
        }
        let old = std::mem::replace(&mut self.row_subst[id.0], Some(row));
        self.log(|| Undo::BindRow(id.0, old));
        Ok(())
    }

    /// Require that a row tail never gains `label`.
    pub fn require_lacks(&mut self, row: &EffRow, label: EffLabel) -> std::result::Result<(), String> {
        let r = self.resolve_row(row);
        if r.contains(&label) {
            return Err(format!(
                "effect row must not contain {label} (a memo cell cannot demand its own group)"
            ));
        }
        match r.tail {
            RowTail::Meta(m) => {
                if !self.lacks.contains(&(m, label.clone())) {
                    self.lacks.push((m, label));
                    self.log(|| Undo::PushLacks);
                }
                Ok(())
            }
            RowTail::Rigid(_) | RowTail::Closed => Ok(()),
        }
    }

    /// Do two labels denote one effect? Heads must agree and the region
    /// identities must be unifiable; a flexible name takes the other side's
    /// identity on success (committed).
    fn label_matches(&mut self, a: &EffLabel, b: &EffLabel) -> bool {
        if a.head != b.head {
            return false;
        }
        match (&a.arg, &b.arg) {
            (None, None) => true,
            // A failed region unification changes nothing, so no trial
            // scope is needed.
            (Some(x), Some(y)) => self.unify_regions(x, y).is_ok(),
            _ => false,
        }
    }

    /// Pair up the labels of two rows by head and region identity, greedily
    /// in row order. Returns each side's unmatched labels.
    fn split_rows(&mut self, ra: &EffRow, rb: &EffRow) -> (Vec<EffLabel>, Vec<EffLabel>) {
        let mut matched_b = vec![false; rb.labels.len()];
        let mut a_only = Vec::new();
        for la in &ra.labels {
            let mut hit = None;
            for (j, lb) in rb.labels.iter().enumerate() {
                if !matched_b[j] && self.label_matches(la, lb) {
                    hit = Some(j);
                    break;
                }
            }
            match hit {
                Some(j) => matched_b[j] = true,
                None => a_only.push(la.clone()),
            }
        }
        let b_only = rb
            .labels
            .iter()
            .enumerate()
            .filter(|(j, _)| !matched_b[*j])
            .map(|(_, l)| l.clone())
            .collect();
        (a_only, b_only)
    }

    /// Row unification with extension: the extra labels of each row must be
    /// absorbed by the other row's tail.
    pub fn unify_rows(
        &mut self,
        a: &EffRow,
        b: &EffRow,
    ) -> std::result::Result<(), String> {
        let ra = self.resolve_row(a);
        let rb = self.resolve_row(b);
        let (mut a_only, mut b_only) = self.split_rows(&ra, &rb);
        a_only.sort_by(|x, y| x.to_string().cmp(&y.to_string()));
        b_only.sort_by(|x, y| x.to_string().cmp(&y.to_string()));
        match (&ra.tail, &rb.tail) {
            (RowTail::Closed, RowTail::Closed) => {
                if a_only.is_empty() && b_only.is_empty() {
                    Ok(())
                } else {
                    Err(format!("effect rows do not match: {ra} vs {rb}"))
                }
            }
            (RowTail::Meta(m), RowTail::Closed) => {
                if !a_only.is_empty() {
                    return Err(format!("effect rows do not match: {ra} vs {rb}"));
                }
                self.bind_row(*m, EffRow::closed(b_only))
            }
            (RowTail::Closed, RowTail::Meta(m)) => {
                if !b_only.is_empty() {
                    return Err(format!("effect rows do not match: {ra} vs {rb}"));
                }
                self.bind_row(*m, EffRow::closed(a_only))
            }
            (RowTail::Meta(m), RowTail::Meta(n)) => {
                if m == n {
                    return if a_only.is_empty() && b_only.is_empty() {
                        Ok(())
                    } else {
                        Err(format!("effect rows do not match: {ra} vs {rb}"))
                    };
                }
                let shared = self.fresh_row();
                self.bind_row(*m, EffRow { labels: b_only, tail: shared.tail.clone() })?;
                self.bind_row(*n, EffRow { labels: a_only, tail: shared.tail })
            }
            (RowTail::Rigid(x), RowTail::Rigid(y)) => {
                if x == y && a_only.is_empty() && b_only.is_empty() {
                    Ok(())
                } else {
                    Err(format!("effect rows do not match: {ra} vs {rb}"))
                }
            }
            (RowTail::Rigid(x), RowTail::Closed) | (RowTail::Closed, RowTail::Rigid(x)) => {
                if a_only.is_empty() && b_only.is_empty() {
                    // A rigid row variable unifies with a closed row only
                    // when they are equal; the caller substituted the rigid
                    // name, so equal tails are handled above. A closed side
                    // against a rigid tail is a mismatch unless the rigid
                    // row was already known equal, which we do not assume.
                    Err(format!(
                        "effect rows do not match: {ra} vs {rb} (row variable {x} is rigid)"
                    ))
                } else {
                    Err(format!("effect rows do not match: {ra} vs {rb}"))
                }
            }
            (RowTail::Rigid(x), RowTail::Meta(m)) | (RowTail::Meta(m), RowTail::Rigid(x)) => {
                // The flexible tail absorbs labels from the rigid side.
                // Labels present only on the flexible side would require
                // changing the universally bound row, which is forbidden.
                let (extra, forbidden) = if matches!(ra.tail, RowTail::Rigid(_)) {
                    (a_only, b_only)
                } else { (b_only, a_only) };
                if !forbidden.is_empty() {
                    return Err(format!("effect rows do not match: {ra} vs {rb}"));
                }
                self.bind_row(
                    *m,
                    EffRow {
                        labels: extra,
                        tail: RowTail::Rigid(x.clone()),
                    },
                )
            }
        }
    }

    /// Flatten a type into its application head and arguments: `Adt`
    /// spines keep their name as an empty-suffix head, `App` spines
    /// concatenate through resolved metavariable heads, `Fun` and
    /// `Tuple` decompose onto their type formers (the arrow's
    /// multiplicity and row are re-checked by unifying the reconstructed
    /// image), and anything else is a head with no arguments.
    fn spine(&self, ty: &Ty) -> (Ty, Vec<Ty>) {
        match self.shallow(ty) {
            Ty::Adt(n, args) => (Ty::Adt(n, Vec::new()), args),
            Ty::App(h, args) => {
                let (head, mut xs) = self.spine(&h);
                xs.extend(args);
                (head, xs)
            }
            Ty::Fun(a, _, _, b) => {
                (Ty::Former(Former::Arrow), vec![*a, *b])
            }
            Ty::Tuple(items) => (Ty::Former(Former::Tuple(items.len())), items),
            other => (other, Vec::new()),
        }
    }

    /// Unify two application spines (`f a ...` against `g b ...`): a
    /// metavariable head solves by eta-prefixing the other side, and two
    /// rigid spines unify head-to-head and argument-to-argument. The
    /// solved image is re-unified with the original side, so a former's
    /// beta image (the pure total arrow) still checks multiplicity and
    /// effect row against the observed shape.
    fn unify_spines(
        &mut self,
        a: &Ty,
        b: &Ty,
    ) -> std::result::Result<(), String> {
        let (h1, xs) = self.spine(a);
        let (h2, ys) = self.spine(b);
        // One shared head (the same metavariable, parameter or former)
        // unifies argument to argument; the eta solves below only apply
        // to distinct heads.
        if h1 == h2 {
            if xs.len() != ys.len() {
                return Err(crate::diagnostic!(
                    "type mismatch: expected {}, got {}",
                    self.zonk(a),
                    self.zonk(b)
                ));
            }
            for (x, y) in xs.iter().zip(ys.iter()) {
                self.unify(x, y)?;
            }
            return Ok(());
        }
        let eta = |h: &Ty, prefix: &[Ty], rest: &[Ty]| {
            Ty::apply_spine(h.clone(), prefix.iter().cloned().chain(rest.iter().cloned()))
        };
        if let Ty::Meta(m) = &h1 {
            if xs.len() <= ys.len() {
                let k = ys.len() - xs.len();
                let image = Ty::apply_spine(h2.clone(), ys[..k].to_vec());
                if self.occurs(*m, &image) {
                    return Err(crate::diagnostic!(
                        "cannot construct infinite type: {} in {}",
                        m_display(*m),
                        self.zonk(&image)
                    ));
                }
                self.check_predicative(*m, &image)?;
                self.bind_type(m.0, image.clone());
                let rebuilt = eta(&image, &[], &xs);
                return self.unify(&rebuilt, b);
            }
        }
        if let Ty::Meta(m) = &h2 {
            if ys.len() <= xs.len() {
                let k = xs.len() - ys.len();
                let image = Ty::apply_spine(h1.clone(), xs[..k].to_vec());
                if self.occurs(*m, &image) {
                    return Err(crate::diagnostic!(
                        "cannot construct infinite type: {} in {}",
                        m_display(*m),
                        self.zonk(&image)
                    ));
                }
                self.check_predicative(*m, &image)?;
                self.bind_type(m.0, image.clone());
                let rebuilt = eta(&image, &[], &ys);
                return self.unify(a, &rebuilt);
            }
        }
        if xs.len() != ys.len() {
            return Err(crate::diagnostic!(
                "type mismatch: expected {}, got {}",
                self.zonk(a),
                self.zonk(b)
            ));
        }
        self.unify(&h1, &h2)?;
        for (x, y) in xs.iter().zip(ys.iter()) {
            self.unify(x, y)?;
        }
        Ok(())
    }

    /// A metavariable never stands for a polytype, and never for a type
    /// that mentions a skolem of a polytype comparison now running.
    fn check_predicative(&mut self, m: MetaId, t: &Ty) -> std::result::Result<(), String> {
        // No metavariable is ever bound to a type containing a polytype
        // (this check is what keeps it so), so the unzonked type answers
        // the predicativity question without a traversal of the bindings.
        if t.contains_forall() || matches!(t, Ty::Forall(_)) {
            return Err(crate::diagnostic!(
                "a type variable cannot stand for a polymorphic type ({}); annotate the binding with its polymorphic type",
                self.zonk(t)
            ));
        }
        // Escape needs the bindings: an inner metavariable may already
        // stand for a skolem. Only scopes opened after `m`'s level
        // constrain it, so outside every skolem scope this costs nothing.
        if self.skolem_scopes.is_empty() {
            return Ok(());
        }
        let level = self.level_floor.get(&m.0).copied().unwrap_or(m.0);
        if !self.skolem_scopes.iter().any(|(watermark, _)| level < *watermark) {
            return Ok(());
        }
        let z = self.zonk(t);
        let outer = self.skolem_scopes.iter().filter(|(watermark, _)| level < *watermark);
        if let Some(skolem) = outer.flat_map(|(_, names)| names).find(|k| z.mentions_param(k)) {
            return Err(crate::diagnostic!(
                "the polymorphic type variable `{}` would escape its `forall` (through {})",
                skolem_spelling(skolem),
                z
            ));
        }
        let mut metas = Vec::new();
        z.free_metas(&mut metas);
        for meta in metas {
            if let EitherMeta::Type(n) = meta {
                let current = self.level_floor.get(&n.0).copied().unwrap_or(n.0);
                if current > level {
                    let old = self.level_floor.insert(n.0, level);
                    self.log(|| Undo::Promote(n.0, old));
                }
            }
        }
        Ok(())
    }

    /// Two polytypes are equal when their binders agree in number and kind
    /// and their bodies unify under one shared set of fresh skolems.
    /// Polytypes are invariant: there is no subsumption between them.
    fn unify_foralls(&mut self, s1: &Scheme, s2: &Scheme, a: &Ty, b: &Ty) -> std::result::Result<(), String> {
        let arity = |s: &Scheme, p: &String| s.con_params.iter().find(|(n, _)| n == p).map_or(0, |(_, k)| *k);
        let same_shape = s1.params.len() == s2.params.len()
            && s1.row_params.len() == s2.row_params.len()
            && s1.region_params.len() == s2.region_params.len()
            && s1.params.iter().zip(&s2.params).all(|(x, y)| arity(s1, x) == arity(s2, y));
        if !same_shape {
            return Err(crate::diagnostic!(
                "polymorphic types differ in their quantifiers: expected {}, got {}",
                self.zonk(a),
                self.zonk(b)
            ));
        }
        let base = self.forall_counter;
        self.forall_counter += 1;
        let (mut r1, mut r2) = (VarRenaming::default(), VarRenaming::default());
        let mut skolems = Vec::new();
        for (i, (x, y)) in s1.params.iter().zip(&s2.params).enumerate() {
            let k = format!("$q{base}.{}", x.split('#').next().unwrap_or(x));
            let k = if i == 0 { k } else { format!("{k}{i}") };
            r1.types.insert(x.clone(), k.clone().into());
            r2.types.insert(y.clone(), k.clone().into());
            skolems.push(k);
        }
        for (i, (x, y)) in s1.row_params.iter().zip(&s2.row_params).enumerate() {
            let k: Interned = format!("$q{base}.row{i}").into();
            r1.rows.insert(x.clone(), k.clone());
            r2.rows.insert(y.clone(), k);
        }
        for (i, (x, y)) in s1.region_params.iter().zip(&s2.region_params).enumerate() {
            let k: Interned = format!("$q{base}.region{i}").into();
            r1.regions.insert(x.clone(), k.clone());
            r2.regions.insert(y.clone(), k);
        }
        let mark = self.enter_skolems(skolems);
        let result = self.unify(&s1.ty.rename_vars(&r1), &s2.ty.rename_vars(&r2));
        self.exit_skolems(mark);
        result
    }

    /// Open a skolem scope over `names`: metavariables that exist now may
    /// not be bound to a type mentioning them until [`Self::exit_skolems`].
    pub fn enter_skolems(&mut self, names: Vec<String>) -> usize {
        let mark = self.skolem_scopes.len();
        self.skolem_scopes.push((self.subst.len(), names));
        mark
    }

    pub fn exit_skolems(&mut self, mark: usize) {
        self.skolem_scopes.truncate(mark);
    }

    pub fn unify(&mut self, a: &Ty, b: &Ty) -> std::result::Result<(), String> {
        let a = self.shallow(a);
        let b = self.shallow(b);
        match (&a, &b) {
            (Ty::Int, Ty::Int) | (Ty::Nat, Ty::Nat) | (Ty::Bytes, Ty::Bytes) | (Ty::Bool, Ty::Bool)
            | (Ty::Sym, Ty::Sym) | (Ty::Unit, Ty::Unit) => Ok(()),
            (Ty::Prim(a), Ty::Prim(b)) if a == b => Ok(()),
            (Ty::Param(x), Ty::Param(y)) if x == y => Ok(()),
            (Ty::Meta(m), t) | (t, Ty::Meta(m)) => {
                if !self.provisional_declarations && self.zonk(t).has_declaration_pending() {
                    return Err("provisional declaration evidence cannot bind a checked type".into());
                }
                if let Ty::Meta(m2) = t {
                    if m == m2 {
                        return Ok(());
                    }
                }
                self.check_predicative(*m, t)?;
                if self.occurs(*m, t) {
                    return Err(crate::diagnostic!(
                        "cannot construct infinite type: {} in {}",
                        m_display(*m),
                        self.zonk(t)
                    ));
                }
                self.bind_type(m.0, t.clone());
                Ok(())
            }
            (Ty::Adt(t1, a1), Ty::Adt(t2, a2)) if t1 == t2 && a1.len() == a2.len() => {
                for (x, y) in a1.iter().zip(a2) {
                    self.unify(x, y)?;
                }
                Ok(())
            }
            (Ty::Forall(s1), Ty::Forall(s2)) => self.unify_foralls(s1, s2, &a, &b),
            (Ty::Tuple(a1), Ty::Tuple(a2)) if a1.len() == a2.len() => {
                for (x, y) in a1.iter().zip(a2) {
                    self.unify(x, y)?;
                }
                Ok(())
            }
            (Ty::Fun(a1, m1, e1, b1), Ty::Fun(a2, m2, e2, b2)) => {
                unify_mults(*m1, *m2)?;
                self.unify_rows(e1, e2)?;
                self.unify(a1, a2)?;
                self.unify(b1, b2)
            }
            (Ty::Thunk { eff: e1, inner: i1 }, Ty::Thunk { eff: e2, inner: i2 }) => {
                self.unify_rows(e1, e2)?;
                self.unify(i1, i2)
            }
            (Ty::Later(i1), Ty::Later(i2)) => self.unify(i1, i2),
            // `Later` and `Thunk` are kept apart by design (laziness and
            // reclamation specification, section 2): a `Later` is a
            // guarded observation modality, a `Thunk` an affine deferred
            // computation, and neither converts to the other.
            (Ty::Later(_), Ty::Thunk { .. }) | (Ty::Thunk { .. }, Ty::Later(_)) => Err(crate::diagnostic!(
                "type mismatch: expected {}, got {} (Later and Thunk are distinct types; a Later is a guarded observation, not a deferred computation)",
                self.zonk(&a),
                self.zonk(&b)
            )),
            (
                Ty::Lazy { eff: e1, region: r1, inner: i1 },
                Ty::Lazy { eff: e2, region: r2, inner: i2 },
            ) => {
                self.unify_regions(r1, r2)?;
                self.unify_rows(e1, e2)?;
                self.unify(i1, i2)
            }
            (Ty::Ref(r1, i1), Ty::Ref(r2, i2)) | (Ty::Mut(r1, i1), Ty::Mut(r2, i2)) => {
                self.unify_regions(r1, r2)?;
                self.unify(i1, i2)
            }
            // Boundary pointers unify only under the same ownership role;
            // `Owned` identities name the allocator, so an allocation from
            // one allocator never unifies with another's (foreign function
            // interface specification section 4: a foreign allocation is
            // released by its matching deallocator).
            (Ty::Ptr { role: r1, pointee: p1 }, Ty::Ptr { role: r2, pointee: p2 }) => {
                if r1 != r2 {
                    return Err(crate::diagnostic!(
                        "pointer role mismatch: expected {}, got {} (a boundary pointer carries exactly one role; raw, borrow, owned, out and nullable roles never convert into each other)",
                        self.zonk(&a),
                        self.zonk(&b)
                    ));
                }
                self.unify(p1, p2)
            }
            (Ty::Region(r1), Ty::Region(r2)) => self.unify_regions(r1, r2),
            (
                Ty::Handler {
                    label: l1,
                    input: i1,
                    output: o1,
                    eff: e1,
                    resumption: m1,
                },
                Ty::Handler {
                    label: l2,
                    input: i2,
                    output: o2,
                    eff: e2,
                    resumption: m2,
                },
            ) => {
                if l1.head != l2.head {
                    return Err(crate::diagnostic!(
                        "type mismatch: expected {}, got {}",
                        self.zonk(&a),
                        self.zonk(&b)
                    ));
                }
                match (&l1.arg, &l2.arg) {
                    (Some(x), Some(y)) => self.unify_regions(x, y)?,
                    (None, None) => {}
                    _ => {
                        return Err(crate::diagnostic!(
                            "type mismatch: expected {}, got {}",
                            self.zonk(&a),
                            self.zonk(&b)
                        ))
                    }
                }
                unify_mults(*m1, *m2)?;
                self.unify_rows(e1, e2)?;
                self.unify(i1, i2)?;
                self.unify(o1, o2)
            }
            (Ty::Sort(s1), Ty::Sort(s2)) if s1 == s2 => Ok(()),
            (Ty::Idx(x), Ty::Idx(y)) => self.unify_idx(x, y),
            (Ty::Prp(x), Ty::Prp(y)) => self.unify_prop(x, y),
            (
                Ty::Fun(a1, m1, e1, b1),
                Ty::Pi { var, dom, mult, eff, cod },
            )
            | (
                Ty::Pi { var, dom, mult, eff, cod },
                Ty::Fun(a1, m1, e1, b1),
            ) => self.unify_fun_shapes(
                "$", a1, *m1, e1, b1, var, dom, *mult, eff, cod,
            ),
            (
                Ty::Pi { var: v1, dom: d1, mult: m1, eff: e1, cod: c1 },
                Ty::Pi { var: v2, dom: d2, mult: m2, eff: e2, cod: c2 },
            ) => self.unify_fun_shapes(v1, d1, *m1, e1, c1, v2, d2, *m2, e2, c2),
            (
                Ty::Sigma { var: v1, dom: d1, cod: c1 },
                Ty::Sigma { var: v2, dom: d2, cod: c2 },
            ) => {
                self.unify(d1, d2)?;
                let mut fresh = format!("$sigma{}", self.subst.len());
                while self.zonk(c1).mentions_idx(&fresh) || self.zonk(c2).mentions_idx(&fresh) || fresh == *v1 || fresh == *v2 { fresh.push('$'); }
                self.unify(&c1.subst_idx(v1, &ITerm::Var(fresh.clone().into())), &c2.subst_idx(v2, &ITerm::Var(fresh.into())))
            }
            // A boundary pointer role is never constructed from an
            // integer or any other non-pointer (foreign function
            // interface specification section 3: safe callers MUST NOT
            // construct these roles by casting integers).
            (Ty::Ptr { .. }, other) | (other, Ty::Ptr { .. })
                if !matches!(other, Ty::Ptr { .. } | Ty::Meta(_)) =>
            {
                Err(crate::diagnostic!(
                    "type mismatch: expected {}, got {} (a boundary pointer role is never constructed from an integer or another non-pointer; foreign function interface specification section 3)",
                    self.zonk(&a),
                    self.zonk(&b)
                ))
            }
            _ => {
                if matches!(a, Ty::App(..)) || matches!(b, Ty::App(..)) {
                    return self.unify_spines(&a, &b);
                }
                if self.conv_eq(&a, &b) {
                    Ok(())
                } else {
                    Err(crate::diagnostic!("type mismatch: expected {}, got {}", self.zonk(&a), self.zonk(&b)))
                }
            }
        }
    }

    /// Unify two function shapes (plain arrows and dependent Pis) up to
    /// the binder name: the codomains are compared under one fresh
    /// binder.
    fn unify_fun_shapes(
        &mut self,
        v1: &str,
        d1: &Ty,
        m1: Mult,
        e1: &EffRow,
        c1: &Ty,
        v2: &str,
        d2: &Ty,
        m2: Mult,
        e2: &EffRow,
        c2: &Ty,
    ) -> std::result::Result<(), String> {
        unify_mults(m1, m2)?;
        self.unify_rows(e1, e2)?;
        self.unify(d1, d2)?;
        let mut fresh = format!("$pi{}", self.subst.len());
        while self.zonk(c1).mentions_idx(&fresh) || self.zonk(c2).mentions_idx(&fresh) || fresh == v1 || fresh == v2 { fresh.push('$'); }
        self.unify(
            &c1.subst_idx(v1, &ITerm::Var(fresh.clone().into())),
            &c2.subst_idx(v2, &ITerm::Var(fresh.into())),
        )
    }

    /// Row union: the accumulation operation for performed effects. The
    /// result contains both rows' labels; tails are joined through the row
    /// substitution. Labels of one head and one region identity merge.
    pub fn union_rows(&mut self, a: &EffRow, b: &EffRow) -> std::result::Result<EffRow, String> {
        let ra = self.resolve_row(a);
        let rb = self.resolve_row(b);
        let (only_a, b_only) = self.split_rows(&ra, &rb);
        let mut labels = ra.labels.clone();
        labels.extend(b_only);
        labels.sort_by(|x, y| x.to_string().cmp(&y.to_string()));
        let _ = only_a;
        let tail = match (&ra.tail, &rb.tail) {
            (RowTail::Closed, RowTail::Closed) => RowTail::Closed,
            (t @ RowTail::Rigid(_), RowTail::Closed)
            | (RowTail::Closed, t @ RowTail::Rigid(_)) => t.clone(),
            (RowTail::Rigid(x), RowTail::Rigid(y)) => {
                if x != y {
                    return Err(format!(
                        "cannot join distinct row variables {x} and {y}"
                    ));
                }
                RowTail::Rigid(x.clone())
            }
            (RowTail::Meta(m), RowTail::Closed) | (RowTail::Closed, RowTail::Meta(m)) => {
                RowTail::Meta(*m)
            }
            (RowTail::Meta(m), RowTail::Meta(n)) => {
                if m == n {
                    RowTail::Meta(*m)
                } else {
                    let other = RowTail::Meta(*n);
                    self.bind_row(*m, EffRow { labels: Vec::new(), tail: other.clone() })?;
                    other
                }
            }
            (RowTail::Meta(m), _) => {
                let other = rb.tail.clone();
                self.bind_row(*m, EffRow { labels: Vec::new(), tail: other.clone() })?;
                other
            }
            (RowTail::Rigid(_), RowTail::Meta(m)) => {
                let other = ra.tail.clone();
                self.bind_row(*m, EffRow { labels: Vec::new(), tail: other.clone() })?;
                other
            }
        };
        Ok(EffRow { labels, tail })
    }

    /// Is `label` definitely absent from this row?
    pub fn lacks_label(&self, row: &EffRow, label: &EffLabel) -> bool {
        let r = self.resolve_row(row);
        if r.contains(label) {
            return false;
        }
        match r.tail {
            RowTail::Closed => true,
            RowTail::Rigid(_) => false,
            RowTail::Meta(m) => self.lacks.contains(&(m, label.clone())),
        }
    }
}

/// Invert a stuck index equation `lhs =?= rhs` when `lhs` is arithmetic
/// with one metavariable and `rhs` is a numeral: `?k + c = n` solves
/// `?k := n - c`. Solutions are guesses; the unifier validates them with
/// conversion before committing.
fn idx_invert(lhs: &ITerm, rhs: &ITerm) -> Option<(usize, ITerm)> {
    let (ITerm::Num(n), ITerm::Bin(op, x, y)) = (rhs.normalize(), lhs.normalize()) else {
        return None;
    };
    let n: i128 = n.parse().ok()?;
    let (m, c, left) = match (&*x, &*y) {
        (ITerm::MVar(m), ITerm::Num(c)) => (*m, c.parse::<i128>().ok()?, false),
        (ITerm::Num(c), ITerm::MVar(m)) => (*m, c.parse::<i128>().ok()?, true),
        _ => return None,
    };
    let v = match (op, left) {
        (IdxOp::Add, _) => n.checked_sub(c),
        (IdxOp::Sub, false) => n.checked_add(c),
        (IdxOp::Sub, true) => c.checked_sub(n),
        (IdxOp::Mul, _) if c != 0 && n % c == 0 => n.checked_div(c),
        (IdxOp::Mul, _) => return None,
    }?;
    Some((m, ITerm::num(v)))
}

fn m_display(MetaId(m): MetaId) -> String {
    format!("?m{m}")
}

/// Definitional equality (conversion) on resolved types: dependent
/// function and pair types compare up to binder names (beta happens at
/// elaboration), index values and propositions compare with numeral
/// reduction (iota), and everything else is structural. This procedure is
/// total by construction: it never unfolds a user definition (decision
/// D28), so it cannot loop.
pub fn ty_eq(a: &Ty, b: &Ty) -> bool {
    match (a, b) {
        (Ty::Idx(x), Ty::Idx(y)) => x.eq(y),
        (Ty::Prp(x), Ty::Prp(y)) => x.eq(y),
        (Ty::Sort(s), Ty::Sort(t)) => s == t,
        (Ty::Adt(n, xs), Ty::Adt(m, ys)) => {
            n == m && xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| ty_eq(x, y))
        }
        (Ty::Tuple(xs), Ty::Tuple(ys)) => {
            xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| ty_eq(x, y))
        }
        (Ty::App(h1, xs), Ty::App(h2, ys)) => {
            ty_eq(h1, h2) && xs.len() == ys.len() && xs.iter().zip(ys).all(|(x, y)| ty_eq(x, y))
        }
        (Ty::Former(f), Ty::Former(g)) => f == g,
        (
            Ty::Fun(..) | Ty::Pi { .. },
            Ty::Fun(..) | Ty::Pi { .. },
        ) => {
            let (v1, d1, m1, e1, c1) = a.as_pi();
            let (v2, d2, m2, e2, c2) = b.as_pi();
            if m1 != m2 && m1 != Mult::Flex && m2 != Mult::Flex {
                return false;
            }
            if e1 != e2 {
                return false;
            }
            if !ty_eq(d1, d2) {
                return false;
            }
            let mut fresh = "$conv".to_string();
            while c1.mentions_idx(&fresh) || c2.mentions_idx(&fresh) || fresh == v1 || fresh == v2 { fresh.push('$'); }
            ty_eq(&c1.subst_idx(&v1, &ITerm::Var(fresh.clone().into())), &c2.subst_idx(&v2, &ITerm::Var(fresh.into())))
        }
        (Ty::Forall(s1), Ty::Forall(s2)) => {
            if s1.params.len() != s2.params.len() || s1.row_params.len() != s2.row_params.len()
                || s1.region_params.len() != s2.region_params.len() || s1.con_params.len() != s2.con_params.len() {
                return false;
            }
            // One fresh name family per comparison: a nested comparison's
            // names never capture an enclosing one's.
            static COMPARISONS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let n = COMPARISONS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let (mut r1, mut r2) = (VarRenaming::default(), VarRenaming::default());
            for (i, (x, y)) in s1.params.iter().zip(&s2.params).enumerate() {
                r1.types.insert(x.clone(), format!("$conv{n}.{i}").into());
                r2.types.insert(y.clone(), format!("$conv{n}.{i}").into());
            }
            for (i, (x, y)) in s1.row_params.iter().zip(&s2.row_params).enumerate() {
                r1.rows.insert(x.clone(), format!("$conv{n}.row{i}").into());
                r2.rows.insert(y.clone(), format!("$conv{n}.row{i}").into());
            }
            for (i, (x, y)) in s1.region_params.iter().zip(&s2.region_params).enumerate() {
                r1.regions.insert(x.clone(), format!("$conv{n}.region{i}").into());
                r2.regions.insert(y.clone(), format!("$conv{n}.region{i}").into());
            }
            ty_eq(&s1.ty.rename_vars(&r1), &s2.ty.rename_vars(&r2))
        }
        (Ty::Sigma { var: v1, dom: d1, cod: c1 }, Ty::Sigma { var: v2, dom: d2, cod: c2 }) => {
            if !ty_eq(d1, d2) {
                return false;
            }
            let mut fresh = "$conv".to_string();
            while c1.mentions_idx(&fresh) || c2.mentions_idx(&fresh) || fresh == *v1 || fresh == *v2 { fresh.push('$'); }
            ty_eq(&c1.subst_idx(v1, &ITerm::Var(fresh.clone().into())), &c2.subst_idx(v2, &ITerm::Var(fresh.into())))
        }
        (
            Ty::Thunk { eff: e1, inner: i1 },
            Ty::Thunk { eff: e2, inner: i2 },
        ) => e1 == e2 && ty_eq(i1, i2),
        (Ty::Later(i1), Ty::Later(i2)) => ty_eq(i1, i2),
        (
            Ty::Lazy { eff: e1, region: r1, inner: i1 },
            Ty::Lazy { eff: e2, region: r2, inner: i2 },
        ) => e1 == e2 && r1 == r2 && ty_eq(i1, i2),
        (Ty::Ref(r1, i1), Ty::Ref(r2, i2)) | (Ty::Mut(r1, i1), Ty::Mut(r2, i2)) => {
            r1 == r2 && ty_eq(i1, i2)
        }
        (
            Ty::Handler { label: l1, input: i1, output: o1, eff: e1, resumption: r1 },
            Ty::Handler { label: l2, input: i2, output: o2, eff: e2, resumption: r2 },
        ) => l1 == l2 && r1 == r2 && e1 == e2 && ty_eq(i1, i2) && ty_eq(o1, o2),
        _ => a == b,
    }
}

#[cfg(test)]
mod scalar_tests {
    use super::*;

    /// Machine scalar bounds and width relationships (primitive types
    /// specification, section 2).
    #[test]
    fn scalar_bounds_and_widths() {
        assert_eq!(Scalar::U8.min_bound_word(), 0);
        assert_eq!(Scalar::U8.max_bound_word(), 255);
        assert_eq!(Scalar::I8.min_bound_word(), -128);
        assert_eq!(Scalar::I8.max_bound_word(), 127);
        // `maxBoundU64` is the all-ones canonical word.
        assert_eq!(Scalar::U64.max_bound_word(), -1);
        assert_eq!(Scalar::Char.max_bound_word(), 0x10FFFF);
        assert_eq!(Scalar::Usize.width(), 64);
        assert_eq!(Scalar::Isize.width(), 64);
        assert!(Scalar::Char.width() >= 32);
    }

    #[test]
    fn scalar_value_preservation() {
        // `fits` models section 7's total conversions.
        assert!(Scalar::U8.fits(Scalar::U16));
        assert!(Scalar::U8.fits(Scalar::I16));
        assert!(Scalar::U16.fits(Scalar::I32));
        assert!(!Scalar::U16.fits(Scalar::I16));
        assert!(!Scalar::I8.fits(Scalar::U8));
        assert!(Scalar::I8.fits(Scalar::I8));
        assert!(Scalar::U32.fits(Scalar::Usize));
        assert!(!Scalar::U64.fits(Scalar::I64));
        assert!(Scalar::I64.fits(Scalar::Isize));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn io() -> EffRow {
        EffRow::closed(vec![EffLabel::plain("IO")])
    }

    #[test]
    fn unify_basics() {
        let mut u = Unifier::new();
        let m = u.fresh();
        u.unify(&m, &Ty::Int).unwrap();
        assert_eq!(u.zonk(&m), Ty::Int);
        u.unify(&Ty::Int, &Ty::Bool).unwrap_err();
    }

    #[test]
    fn unify_functions_and_adts() {
        let mut u = Unifier::new();
        let m = u.fresh();
        let f1 = Ty::fun(m.clone(), Ty::Int);
        let f2 = Ty::fun(Ty::Bool, Ty::Int);
        u.unify(&f1, &f2).unwrap();
        assert_eq!(u.zonk(&m), Ty::Bool);

        let mut u = Unifier::new();
        let m = u.fresh();
        u.unify(&Ty::Adt("List".into(), vec![m.clone()]), &Ty::Adt("List".into(), vec![Ty::Int]))
            .unwrap();
        assert_eq!(u.zonk(&m), Ty::Int);
    }

    #[test]
    fn tuples_unify_componentwise() {
        let mut u = Unifier::new();
        let a = u.fresh();
        let b = u.fresh();
        let t1 = Ty::Tuple(vec![a.clone(), Ty::Int, b.clone()]);
        let t2 = Ty::Tuple(vec![Ty::Bool, Ty::Int, Ty::Unit]);
        u.unify(&t1, &t2).unwrap();
        assert_eq!(u.zonk(&a), Ty::Bool);
        assert_eq!(u.zonk(&b), Ty::Unit);

        // Arity mismatches and component mismatches both fail.
        let mut u = Unifier::new();
        u.unify(&Ty::Tuple(vec![Ty::Int]), &Ty::Tuple(vec![Ty::Int, Ty::Int]))
            .unwrap_err();
        let mut u = Unifier::new();
        u.unify(&Ty::Tuple(vec![Ty::Int, Ty::Bool]), &Ty::Tuple(vec![Ty::Int, Ty::Unit]))
            .unwrap_err();

        // The occurs check runs through tuple components.
        let mut u = Unifier::new();
        let m = u.fresh();
        u.unify(&m, &Ty::Tuple(vec![Ty::Int, m.clone()])).unwrap_err();
    }

    #[test]
    fn occurs_check() {
        let mut u = Unifier::new();
        let m = u.fresh();
        let f = Ty::fun(m.clone(), Ty::Int);
        assert!(u.unify(&m, &f).is_err());
    }

    #[test]
    fn rigid_params_only_unify_with_themselves() {
        let mut u = Unifier::new();
        u.unify(&Ty::Param("a".into()), &Ty::Param("a".into())).unwrap();
        assert!(u.unify(&Ty::Param("a".into()), &Ty::Param("b".into())).is_err());
    }

    #[test]
    fn row_unification_extends_through_tails() {
        let mut u = Unifier::new();
        let pure = EffRow::pure();
        let open = u.fresh_row();
        // A pure row unifies with an open row by binding the tail closed.
        u.unify_rows(&pure, &open.clone()).unwrap();
        assert!(u.resolve_row(&open).is_pure());

        // `{IO}` unifies with `{|e}` by giving `e` the label.
        let mut u = Unifier::new();
        let open = u.fresh_row();
        u.unify_rows(&io(), &open.clone()).unwrap();
        assert_eq!(u.resolve_row(&open).labels, vec![EffLabel::plain("IO")]);

        // `{IO}` and `{IO, Eval r}` do not unify when both are closed.
        let eval_r = EffRow::closed(vec![EffLabel::plain("IO"), EffLabel::with_arg("Eval", "r")]);
        let mut u = Unifier::new();
        assert!(u.unify_rows(&io(), &eval_r).is_err());

        // ... but an open tail absorbs the difference.
        let mut u = Unifier::new();
        let open = u.fresh_row();
        u.unify_rows(&open.clone(), &eval_r).unwrap();
        assert_eq!(u.resolve_row(&open).labels.len(), 2);
    }

    #[test]
    fn rigid_row_unification_preserves_labels_without_inventing_effects() {
        let rigid = EffRow { labels: vec![], tail: RowTail::Rigid("e".into()) };
        let read = EffLabel::with_arg("Read", "r");
        let labelled = EffRow { labels: vec![read.clone()], tail: rigid.tail.clone() };
        for reverse in [false, true] {
            let mut uni = Unifier::new();
            let flexible = uni.fresh_row();
            let result = if reverse { uni.unify_rows(&labelled, &flexible) }
                else { uni.unify_rows(&flexible, &labelled) };
            result.unwrap();
            assert_eq!(uni.resolve_row(&flexible), labelled);

            let mut uni = Unifier::new();
            let flexible = uni.fresh_row();
            let extra = EffRow { labels: vec![read.clone()], tail: flexible.tail };
            let result = if reverse { uni.unify_rows(&rigid, &extra) }
                else { uni.unify_rows(&extra, &rigid) };
            assert!(result.is_err(), "a rigid e cannot acquire Read r");
        }
    }

    #[test]
    fn lacks_constraints_catch_forbidden_labels() {
        let mut u = Unifier::new();
        let mut row = u.fresh_row();
        let label = EffLabel::with_arg("Eval", "r#1");
        u.require_lacks(&row, label.clone()).unwrap();
        // Binding the tail to a row with the label fails.
        assert!(u
            .unify_rows(&row.clone(), &EffRow::closed(vec![label.clone()]))
            .is_err());
        let mut u = Unifier::new();
        row = u.fresh_row();
        u.require_lacks(&row, label.clone()).unwrap();
        u.unify_rows(&row.clone(), &EffRow::closed(vec![EffLabel::plain("IO")]))
            .unwrap();
        assert!(!u.lacks_label(&row, &EffLabel::plain("IO")));
        assert!(u.lacks_label(&row, &label));
    }

    #[test]
    fn snapshots_roll_back_every_store_and_nest() {
        let mut u = Unifier::new();
        let a = u.fresh();
        let row = u.fresh_row();
        let label = EffLabel::with_arg("Eval", "r#1");
        u.require_lacks(&row, label.clone()).unwrap();
        u.note_flex_region("r#2");
        let outer = u.snapshot();
        u.unify(&a, &Ty::Int).unwrap();
        let inner = u.snapshot();
        let b = u.fresh();
        let tail = u.fresh_row();
        u.unify_rows(&row, &EffRow { labels: vec![EffLabel::plain("IO")], tail: tail.tail.clone() }).unwrap();
        u.unify_regions("r#2", "rigid").unwrap();
        u.note_flex_region("r#3");
        let i = u.fresh_idx_term();
        u.unify_idx(&i, &ITerm::num(3)).unwrap();
        assert!(u.lacks_label(&tail, &label), "the constraint moved to the new tail");
        u.rollback_to(inner);
        assert_eq!(u.zonk(&a), Ty::Int, "the outer scope's binding survives the inner rollback");
        assert_eq!(u.fresh(), b, "the inner scope's variables are gone");
        assert_eq!(u.resolve_row(&row), row);
        assert!(u.lacks_label(&row, &label));
        assert_eq!(u.resolve_region("r#2"), "r#2");
        assert!(!u.is_flex_region("r#3"));
        assert!(u.fresh_idx_term().eq(&i));
        u.rollback_to(outer);
        assert_eq!(u.zonk(&a), a);
        let kept = u.snapshot();
        u.unify(&a, &Ty::Bool).unwrap();
        u.commit(kept);
        assert_eq!(u.zonk(&a), Ty::Bool);
        assert!(u.undo.is_empty() && u.open_snapshots == 0);
    }

    #[test]
    fn thunk_and_lazy_unify() {
        let mut u = Unifier::new();
        let e = io();
        let t1 = Ty::Thunk { eff: e.clone(), inner: Box::new(Ty::Int) };
        let t2 = Ty::Thunk { eff: e.clone(), inner: Box::new(Ty::Bool) };
        assert!(u.unify(&t1, &t2).is_err());
        let l1 = Ty::Lazy { eff: e.clone(), region: "r".into(), inner: Box::new(Ty::Int) };
        let l2 = Ty::Lazy { eff: e, region: "s".into(), inner: Box::new(Ty::Int) };
        assert!(u.unify(&l1, &l2).is_err());
    }
}

#[cfg(test)]
mod guarded_equality_tests {
    use super::*;
    #[test]
    fn multi_term_equality_keeps_its_negated_goal_and_supporting_facts() {
        let v = |name: &str| ITerm::Var(name.into());
        let cmp = |a, b| PTerm::Cmp(IdxCmp::Eq, Box::new(a), Box::new(b));
        let facts = [cmp(v("result"), ITerm::Bin(IdxOp::Sub, Box::new(v("total")), Box::new(v("extra"))))];
        let sum = ITerm::Bin(IdxOp::Add, Box::new(v("result")), Box::new(v("extra")));
        let goal = cmp(sum.clone(), v("total"));
        let sorts = Sorts { vars: vec![("total".into(), IntSort::Nat), ("extra".into(), IntSort::Nat), ("result".into(), IntSort::Nat)], fns: vec![], values: vec![] };
        let proofs = linear_entails(&facts, &goal, &sorts).expect("the actual equation entails the rearranged equality");
        assert!(proofs.iter().all(|proof| check_linear_proof(&facts, proof, &sorts)));
        assert!(linear_entails(&[], &goal, &sorts).is_none());
        let wrong = cmp(sum, ITerm::Bin(IdxOp::Add, Box::new(v("total")), Box::new(ITerm::num(1))));
        assert!(linear_entails(&facts, &wrong, &sorts).is_none());
    }
}

#[cfg(test)]
mod index_instantiation_tests {
    use super::*;

    #[test]
    fn declaration_placeholders_are_confined_to_provisional_unification() {
        let pending = ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::DeclarationPending(0), types: vec![], dictionaries: vec![], arguments: vec![],
        }));
        let mut strict = Unifier::new();
        assert!(strict.unify_idx(&pending, &pending).is_err());
        let ty = strict.fresh();
        assert!(strict.unify(&ty, &Ty::Idx(Box::new(pending.clone()))).is_err());
        let mut provisional = Unifier::new();
        provisional.provisional_declarations(true);
        let witness = provisional.fresh_idx_term();
        provisional.unify_idx(&witness, &pending).unwrap();
        assert!(provisional.has_declaration_pending());
        assert!(provisional.resolve_idx(&witness).has_declaration_pending());
        let other = ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::DeclarationPending(1), types: vec![], dictionaries: vec![], arguments: vec![],
        }));
        provisional.unify_idx(&pending, &other).unwrap();
        let left = PTerm::Eq(Box::new(pending), Box::new(ITerm::num(0)));
        let right = PTerm::Eq(Box::new(other), Box::new(ITerm::num(0)));
        provisional.unify_prop(&left, &right).unwrap();
        provisional.provisional_declarations(false);
        assert!(provisional.unify_idx(&witness, &witness).is_err());
        assert!(provisional.unify_prop(&left, &right).is_err());
    }

    #[test]
    fn freshened_index_witnesses_do_not_cascade_or_capture_values() {
        let original = Ty::Tuple(vec![Ty::Idx(Box::new(ITerm::MVar(0))), Ty::Idx(Box::new(ITerm::MVar(1)))]);
        let renamed = original.subst_index_metas(&HashMap::from([(0, ITerm::MVar(1)), (1, ITerm::MVar(2))]));
        assert_eq!(renamed, Ty::Tuple(vec![Ty::Idx(Box::new(ITerm::MVar(1))), Ty::Idx(Box::new(ITerm::MVar(2)))]));
        let lambda = Ty::Idx(Box::new(ITerm::Lam("x".into(), Box::new(ITerm::MVar(0)))));
        let Ty::Idx(mapped) = lambda.subst_index_metas(&HashMap::from([(0, ITerm::Var("x".into()))])) else { panic!("lost index") };
        let ITerm::Lam(binder, body) = *mapped else { panic!("lost lambda") };
        assert_ne!(binder, "x");
        assert_eq!(*body, ITerm::Var("x".into()));
    }

    #[test]
    fn dictionary_identity_is_not_a_value_name() {
        let identity = IndexEvidence::Local(7);
        let evidence = ITerm::Evidence(identity.clone());
        let spelling = ITerm::Var("$dictionary-local:7".into());
        let parameter = ITerm::Evidence(IndexEvidence::Parameter("$dictionary-local:7".into()));
        assert!(!evidence.eq(&spelling));
        assert!(!evidence.eq(&parameter));
        assert_eq!(evidence.subst("$dictionary-local:7", &ITerm::num(0)), evidence);
        let bindings = HashMap::from([(identity, ITerm::num(42))]);
        assert_eq!(spelling.subst_evidence(&bindings), spelling);
        assert_eq!(evidence.subst_evidence(&bindings), ITerm::num(42));
        assert!(Unifier::new().unify_idx(&evidence, &spelling).is_err());
    }

    #[test]
    fn evidence_renaming_is_simultaneous_and_preserves_value_binders() {
        let identity = |local| ITerm::Evidence(IndexEvidence::Local(local));
        let term = ITerm::Lam("provider".into(), Box::new(ITerm::App(Box::new(identity(0)), vec![identity(1), ITerm::Var("provider".into())])));
        let mapped = term.subst_evidence(&HashMap::from([
            (IndexEvidence::Local(0), identity(1)), (IndexEvidence::Local(1), identity(2)),
        ]));
        assert_eq!(mapped, ITerm::Lam("provider".into(), Box::new(ITerm::App(Box::new(identity(1)), vec![identity(2), ITerm::Var("provider".into())]))));
    }

    #[test]
    fn evidence_substitution_cannot_capture_a_free_value() {
        let binding = IndexEvidence::Parameter("provider".into());
        let term = ITerm::Lam("x".into(), Box::new(ITerm::Evidence(binding.clone())));
        let ITerm::Lam(name, body) = term.subst_evidence(&HashMap::from([(binding, ITerm::Var("x".into()))])) else { panic!("lost lambda") };
        assert_ne!(name, "x");
        assert_eq!(*body, ITerm::Var("x".into()));
    }

    #[test]
    fn quantified_index_metadata_checks_every_namespace_and_argument_kind() {
        let mut uni = Unifier::new();
        let value = uni.fresh_idx_term();
        let Ty::Prp(prop) = uni.fresh_prop() else { panic!("proposition expected") };
        let row = uni.fresh_row();
        let call = |runtime, region: &str| ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::Function("Scopes.measure".into()), types: vec![], dictionaries: vec![],
            arguments: vec![
                IndexArgument::Value { value: value.clone(), carrier: Ty::Nat, runtime },
                IndexArgument::Proposition((*prop).clone()), IndexArgument::Row(row.clone()),
                IndexArgument::Region(region.into()),
            ],
        }));
        let original = call(false, "outer");
        assert!(!original.eq(&call(true, "outer")));
        assert!(!original.eq(&call(false, "inner")));
        let ty = Ty::Idx(Box::new(original.clone()));
        assert!(!Unifier::new().owns_all_metas(&ty));
        let mut ids = Vec::new(); original.free_mvars(&mut ids);
        assert_eq!(ids, vec![0], "proposition and index IDs remain separate namespaces");
        uni.unify_idx(&value, &ITerm::num(7)).unwrap();
        uni.unify_prop(&prop, &PTerm::Eq(Box::new(ITerm::num(7)), Box::new(ITerm::num(7)))).unwrap();
        uni.unify_rows(&row, &EffRow::pure()).unwrap();
        let resolved = uni.zonk(&ty);
        assert!(Unifier::new().owns_all_metas(&resolved));
        assert!(uni.resolve_idx(&original).eq(&uni.resolve_idx(&original)));
        let before = uni.clone();
        assert!(uni.unify_idx(&original, &call(false, "inner")).is_err());
        assert_eq!(uni.resolve_idx(&original), before.resolve_idx(&original));
    }

    fn call(types: Vec<Ty>, dictionary: ITerm, values: Vec<ITerm>) -> ITerm {
        ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::Function("Measures.measure".into()), types,
            dictionaries: vec![dictionary], arguments: values.into_iter().map(|value| IndexArgument::Value { value, carrier: Ty::Nat, runtime: true }).collect(),
        }))
    }

    #[test]
    fn reflexive_index_and_proposition_metas_are_not_infinite_types() {
        let mut uni = Unifier::new();
        let index = uni.fresh_idx_term();
        uni.unify_idx(&index, &index).unwrap();
        assert_eq!(uni.resolve_idx(&index), index);
        let Ty::Prp(prop) = uni.fresh_prop() else { panic!("proposition expected") };
        uni.unify_prop(&prop, &prop).unwrap();
        assert_eq!(uni.resolve_prop(&prop), *prop);
        let ITerm::MVar(id) = index else { panic!("index meta expected") };
        assert!(uni.bind_idx(id, ITerm::app_fn("loop", vec![ITerm::MVar(id)])).is_err());
        let PTerm::MVar(id) = *prop else { panic!("proposition meta expected") };
        assert!(uni.bind_prop(id, PTerm::Not(Box::new(PTerm::MVar(id)))).is_err());
    }

    #[test]
    fn provider_and_type_instantiations_are_part_of_index_identity() {
        let first = call(vec![Ty::Int], ITerm::Var("provider#1".into()), vec![]);
        assert!(first.eq(&first.clone()));
        assert!(!first.eq(&call(vec![Ty::Int], ITerm::Var("provider#2".into()), vec![])));
        assert!(!first.eq(&call(vec![Ty::Bool], ITerm::Var("provider#1".into()), vec![])));
        assert!(!first.eq(&ITerm::Fn("Measures.measure".into())));
        let factory = |ty| ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::Dictionary("Instances.factory".into()), types: vec![ty],
            dictionaries: vec![ITerm::Var("parent".into())], arguments: vec![],
        }));
        assert!(!call(vec![], factory(Ty::Int), vec![]).eq(&call(vec![], factory(Ty::Bool), vec![])));
    }

    #[test]
    fn substitution_reaches_types_evidence_and_implicit_values_without_capture() {
        let index_type = |name: &str| Ty::Adt("Sized".into(), vec![Ty::Idx(Box::new(ITerm::Var(name.into())))]);
        let term = call(vec![index_type("size")], ITerm::Var("provider".into()), vec![ITerm::Var("size".into())]);
        let substituted = term.subst("size", &ITerm::num(7)).subst("provider", &ITerm::Var("actual-provider".into()));
        let mut free = Vec::new(); substituted.free_vars(&mut free);
        assert_eq!(free, vec!["actual-provider"]);
        assert!(substituted.eq(&call(vec![Ty::Adt("Sized".into(), vec![Ty::Idx(Box::new(ITerm::num(7)))])], ITerm::Var("actual-provider".into()), vec![ITerm::num(7)])));
        let bound = ITerm::Lam("size".into(), Box::new(term));
        let moved = bound.subst("provider", &ITerm::Var("size".into()));
        let mut free = Vec::new(); moved.free_vars(&mut free);
        assert_eq!(free, vec!["size"]);
        let ITerm::Lam(binder, _) = moved else { panic!("lambda lost") };
        assert_ne!(binder, "size");
    }

    #[test]
    fn unification_checks_evidence_and_rolls_back_failed_type_inference() {
        let mut uni = Unifier::new(); let generic = uni.fresh();
        let a = call(vec![generic.clone()], ITerm::Var("provider#1".into()), vec![]);
        let bad = call(vec![Ty::Int], ITerm::Var("provider#2".into()), vec![]);
        assert!(uni.unify_idx(&a, &bad).is_err());
        assert_eq!(uni.zonk(&generic), generic);
        let good = call(vec![Ty::Bool], ITerm::Var("provider#1".into()), vec![]);
        uni.unify_idx(&a, &good).unwrap();
        assert_eq!(uni.zonk(&generic), Ty::Bool);
        assert!(uni.resolve_idx(&a).eq(&good));
    }

    #[test]
    fn hidden_types_participate_in_occurs_checks_and_generalization() {
        let enclosing = Unifier::new();
        let mut private = enclosing.clone();
        let index = private.fresh_idx(); let proposition = private.fresh_prop();
        assert!(private.owns_all_metas(&index)); assert!(private.owns_all_metas(&proposition));
        assert!(!enclosing.owns_all_metas(&index)); assert!(!enclosing.owns_all_metas(&proposition));
        let hidden = Ty::Idx(Box::new(call(vec![proposition], ITerm::num(0), vec![])));
        assert!(!enclosing.owns_all_metas(&hidden));
        let mut uni = Unifier::new(); let parameter = uni.fresh(); let row = uni.fresh_row();
        let term = call(vec![Ty::fun_eff(parameter.clone(), row, Ty::Unit)], ITerm::Var("dict".into()), vec![]);
        let ty = Ty::Idx(Box::new(term));
        assert!(uni.unify(&parameter, &ty).is_err());
        let metas = uni.free_metas(&ty); assert_eq!(metas.len(), 2);
        let (generalized, types, rows) = uni.generalize(&ty, &metas);
        assert_eq!(types.len(), 1); assert_eq!(rows.len(), 1);
        assert!(uni.free_metas(&generalized).is_empty());
        let fresh = uni.fresh_idx_term();
        let cyclic = call(vec![Ty::Idx(Box::new(fresh.clone()))], ITerm::Var("dict".into()), vec![]);
        assert!(uni.unify_idx(&fresh, &cyclic).is_err());
    }

    #[test]
    fn proposition_occurs_checks_follow_hidden_types_without_confusing_index_ids() {
        let mut uni = Unifier::new();
        let Ty::Prp(prop) = uni.fresh_prop() else { panic!("prop meta expected") };
        let PTerm::MVar(id) = *prop else { panic!("prop meta expected") };
        let index = uni.fresh_idx_term();
        uni.bind_prop(id, PTerm::Eq(Box::new(index), Box::new(ITerm::num(0)))).unwrap();
        let Ty::Prp(prop) = uni.fresh_prop() else { panic!("prop meta expected") };
        let PTerm::MVar(id) = *prop else { panic!("prop meta expected") };
        let hidden = call(vec![Ty::Prp(Box::new(PTerm::MVar(id)))], ITerm::Var("dict".into()), vec![]);
        assert!(uni.bind_prop(id, PTerm::Eq(Box::new(hidden), Box::new(ITerm::num(0)))).is_err());
        let mut uni = Unifier::new();
        let index = uni.fresh_idx_term();
        let proposition = uni.fresh_prop();
        let independent = call(vec![proposition], ITerm::Var("dict".into()), vec![]);
        uni.unify_idx(&index, &independent).unwrap();
    }

    #[test]
    fn source_arity_excludes_dictionaries_and_implicit_values() {
        let head = call(vec![Ty::Int], ITerm::Var("dict".into()), vec![ITerm::num(4), ITerm::num(5)]);
        let term = ITerm::App(Box::new(head), vec![ITerm::num(7)]);
        let sorts = Sorts { vars: vec![], fns: vec![("Measures.measure".into(), 1, IntSort::Nat)], values: vec![] };
        assert_eq!(sorts.term(&term), Some(IntSort::Nat));
        assert!(term.normalize().eq(&term));
    }

    #[test]
    fn alpha_conversion_preserves_free_dictionary_names_and_dependent_type_scope() {
        let bound = ITerm::Lam("dict".into(), Box::new(call(vec![], ITerm::Var("dict".into()), vec![])));
        let free = ITerm::Lam("other".into(), Box::new(call(vec![], ITerm::Var("$alpha".into()), vec![])));
        assert!(!bound.eq(&free));
        let pi = |binder: &str, provider: &str| Ty::pi(binder, Ty::Unit, Mult::Many, EffRow::pure(), Ty::Idx(Box::new(call(vec![], ITerm::Var(provider.into()), vec![]))));
        assert!(!ty_eq(&pi("dict", "dict"), &pi("other", "$conv")));
        assert!(Unifier::new().unify(&pi("dict", "dict"), &pi("other", "$pi0")).is_err());
        let ty = Ty::pi("n", Ty::Nat, Mult::Many, EffRow::pure(), Ty::Idx(Box::new(ITerm::Var("n".into()))));
        assert!(!ty.mentions_idx("n"));
        assert_eq!(ty.subst_idx("n", &ITerm::num(9)), ty);
        let with_outer = Ty::pi("n", Ty::Nat, Mult::Many, EffRow::pure(), Ty::Tuple(vec![Ty::Idx(Box::new(ITerm::Var("n".into()))), Ty::Idx(Box::new(ITerm::Var("outer".into())))]));
        let result = with_outer.subst_idx("outer", &ITerm::Var("n".into()));
        let mut names = Vec::new(); result.free_index_vars(&mut names);
        assert_eq!(names, vec!["n"]);
    }
}

#[cfg(test)]
mod local_value_identity_tests {
    use super::*;
    use crate::core::LocalId;

    #[test]
    fn local_values_cannot_be_selected_by_a_spelling_or_dictionary_identity() {
        let value = ITerm::LocalValue(LocalId(7));
        let spelling = value.to_string();
        assert_eq!(value.subst(&spelling, &ITerm::num(42)), value);
        assert_eq!(value.subst_evidence(&HashMap::from([
            (IndexEvidence::Local(7), ITerm::num(42)),
        ])), value);
        let mut uni = Unifier::new();
        assert!(uni.unify_idx(&value, &ITerm::Var(spelling.as_str().into())).is_err());
        assert!(uni.unify_idx(&value, &ITerm::Evidence(IndexEvidence::Local(7))).is_err());
        assert!(uni.unify_idx(&value, &ITerm::LocalValue(LocalId(8))).is_err());
        uni.unify_idx(&value, &value).unwrap();
    }

    #[test]
    fn local_value_substitution_crosses_instantiation_types_without_capturing_names() {
        let local = ITerm::LocalValue(LocalId(7));
        let ty = Ty::pi("subject", Ty::Nat, Mult::Many, EffRow::pure(), Ty::Idx(Box::new(local.clone())));
        let call = ITerm::Inst(Box::new(IndexInstantiation {
            head: IndexHead::Function("Library.checked".into()),
            types: vec![ty],
            dictionaries: vec![ITerm::Evidence(IndexEvidence::Local(7))],
            arguments: vec![IndexArgument::Value {
                value: local, carrier: Ty::Prp(Box::new(PTerm::Eq(
                    Box::new(ITerm::LocalValue(LocalId(7))), Box::new(ITerm::LocalValue(LocalId(7))),
                ))), runtime: true,
            }],
        }));
        let mapped = call.subst_local_values(&HashMap::from([(LocalId(7), ITerm::Var("subject".into()))]));
        let mut values = Vec::new(); mapped.local_values(&mut values);
        assert!(values.is_empty());
        let mut names = Vec::new(); mapped.free_vars(&mut names);
        assert_eq!(names, vec!["subject"]);
        let ITerm::Inst(call) = mapped else { unreachable!() };
        assert_eq!(call.dictionaries, vec![ITerm::Evidence(IndexEvidence::Local(7))]);
        let Ty::Pi { var, cod, .. } = &call.types[0] else { unreachable!() };
        assert_ne!(var, "subject");
        assert_eq!(**cod, Ty::Idx(Box::new(ITerm::Var("subject".into()))));
    }

    #[test]
    fn logical_literals_keep_identity_under_unification_and_substitution() {
        let mut uni = Unifier::new();
        assert!(PTerm::Lit(true).eq(&PTerm::Lit(true)));
        assert!(!PTerm::Lit(true).eq(&PTerm::Lit(false)));
        assert!(uni.unify_prop(&PTerm::Lit(true), &PTerm::Lit(false)).is_err());
        let Ty::Prp(meta) = uni.fresh_prop() else { unreachable!() };
        uni.unify_prop(&meta, &PTerm::Lit(false)).unwrap();
        assert_eq!(uni.resolve_prop(&meta), PTerm::Lit(false));
        assert_eq!(PTerm::Lit(false).subst("False", &ITerm::num(1)), PTerm::Lit(false));
        assert!(!pred_entails(&PTerm::Lit(true), &PTerm::Lit(false)));
        assert!(pred_entails(&PTerm::Lit(false), &PTerm::Var("p".into())));
        assert!(pred_entails(&PTerm::Var("p".into()), &PTerm::Lit(true)));
        assert!(!uni.conv_eq(&Ty::Prp(Box::new(PTerm::Lit(true))), &Ty::Bool));
        assert_eq!(PTerm::Lit(true).to_string(), "True");
        assert_eq!(PTerm::Lit(false).to_string(), "False");
    }

}

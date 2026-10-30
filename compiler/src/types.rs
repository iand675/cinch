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
    pub head: String,
    pub arg: Option<String>,
}

impl EffLabel {
    pub fn plain(head: impl Into<String>) -> EffLabel {
        EffLabel { head: head.into(), arg: None }
    }

    pub fn with_arg(head: impl Into<String>, arg: impl Into<String>) -> EffLabel {
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
    Rigid(String),
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
                    arg: l.arg.as_ref().map(|a| map.get(a).cloned().unwrap_or_else(|| a.clone())),
                })
                .collect(),
            tail: match &self.tail {
                RowTail::Rigid(t) => {
                    RowTail::Rigid(map.get(t).cloned().unwrap_or_else(|| t.clone()))
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

/// A first-order value embedded in a type (an index): variables shared
/// with term syntax (the surface shares term syntax between kinds, types
/// and values), numerals, mathematical arithmetic, and applications of
/// named total definitions. Inference variables support implicit-argument
/// solving (design section 2: elaboration is insert-and-solve).
#[derive(Clone, Debug, PartialEq)]
pub enum ITerm {
    Var(String),
    Num(String),
    /// A named total definition used as a function value (the head of an
    /// index-level application).
    Fn(String),
    /// `\x -> body`, an index-level lambda (a lifting function like
    /// `congr`'s, or a type-position lambda).
    Lam(String, Box<ITerm>),
    App(Box<ITerm>, Vec<ITerm>),
    Bin(IdxOp, Box<ITerm>, Box<ITerm>),
    MVar(usize),
}

impl ITerm {
    pub fn num(n: i128) -> ITerm {
        ITerm::Num(n.to_string())
    }

    /// A named application: `f a b`.
    pub fn app_fn(head: impl Into<String>, args: Vec<ITerm>) -> ITerm {
        // A nullary reference is the bare head: `App(f, [])` and `f`
        // denote the same index value (a `law` equation over a nullary
        // method like `mempty` must unify with a term's plain
        // reference).
        if args.is_empty() {
            ITerm::Fn(head.into())
        } else {
            ITerm::App(Box::new(ITerm::Fn(head.into())), args)
        }
    }

    /// Free value names, in first-appearance order.
    pub fn free_vars(&self, out: &mut Vec<String>) {
        match self {
            ITerm::Var(n) => {
                if !out.contains(n) {
                    out.push(n.clone());
                }
            }
            ITerm::Num(_) | ITerm::Fn(_) | ITerm::MVar(_) => {}
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
            ITerm::Var(_) | ITerm::Num(_) | ITerm::Fn(_) => {}
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
        match self {
            ITerm::Var(n) if n == name => by.clone(),
            ITerm::Var(_) | ITerm::Num(_) | ITerm::Fn(_) | ITerm::MVar(_) => self.clone(),
            ITerm::Lam(x, body) if x == name => self.clone(),
            ITerm::Lam(x, body) => {
                let mut body = (**body).clone();
                if by.mentions(x) {
                    let mut fresh = format!("{x}$sub");
                    let mut n = 0;
                    while body.mentions(&fresh) || by.mentions(&fresh) {
                        n += 1;
                        fresh = format!("{x}$sub{n}");
                    }
                    body = body.subst(x, &ITerm::Var(fresh.clone()));
                    ITerm::Lam(fresh, Box::new(body.subst(name, by)))
                } else {
                    ITerm::Lam(x.clone(), Box::new(body.subst(name, by)))
                }
            }
            ITerm::App(f, args) => ITerm::App(
                Box::new(f.subst(name, by)),
                args.iter().map(|a| a.subst(name, by)).collect(),
            ),
            ITerm::Bin(op, a, b) => ITerm::Bin(
                *op,
                Box::new(a.subst(name, by)),
                Box::new(b.subst(name, by)),
            ),
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
            (ITerm::Num(x), ITerm::Num(y)) => x == y,
            (ITerm::Fn(x), ITerm::Fn(y)) => x == y,
            (ITerm::MVar(x), ITerm::MVar(y)) => x == y,
            (ITerm::Lam(x, b), ITerm::Lam(y, c)) => {
                let fresh = ITerm::Var("$alpha".to_string());
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
        match self {
            ITerm::Var(n) => write!(f, "{n}"),
            ITerm::Num(n) => write!(f, "{n}"),
            ITerm::Fn(n) => {
                // The baked measure vocabulary reads as its spelling
                // (`length xs > 100`, never `<builtin>.length xs > 100`).
                let n = n.strip_prefix("<builtin>.").unwrap_or(n);
                write!(f, "{n}")
            }
            ITerm::MVar(m) => write!(f, "?i{m}"),
            ITerm::Lam(x, body) => write!(f, "(\\{x} -> {body})"),
            ITerm::App(head, args) => {
                if matches!(&**head, ITerm::Fn(n) if n.starts_with("<builtin>.")) {
                    write!(f, "{head}")?;
                    for a in args {
                        write!(f, " {a}")?;
                    }
                    return Ok(());
                }
                write!(f, "({head}")?;
                for a in args {
                    write!(f, " {a}")?;
                }
                write!(f, ")")
            }
            ITerm::Bin(op, a, b) => write!(f, "({a} {op} {b})"),
        }
    }
}

/// A proposition in `Prop` (dependent core kernel design, section 7):
/// comparisons and equality over index values, negation, conjunction
/// and disjunction (the refinement lattice: compound predicates), and
/// inference variables for implicit solving.
#[derive(Clone, Debug, PartialEq)]
pub enum PTerm {
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
        out = out.subst(v, &ITerm::Var(format!("$b{i}")));
    }
    out
}

/// Alpha-equivalence of two predicates (the same lattice point under
/// different binder spellings).
pub fn pred_alpha_eq(p: &PTerm, q: &PTerm) -> bool {
    pred_canon(p).eq(&pred_canon(q))
}

/// The top-level conjuncts of a predicate, with negated comparisons
/// folded into their positive forms (`Not (x < 5)` is `x >= 5`) so the
/// interval rows see every bound the predicate carries.
pub fn pred_conjuncts(p: &PTerm) -> Vec<PTerm> {
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
    fv.into_iter().next().unwrap_or_else(|| "$it".to_string())
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

fn pred_implies_canon(p: &PTerm, q: &PTerm) -> bool {
    if p.eq(q) {
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
            // A range excluding zero implies `NonZero x` (row 3).
            interval_over(conj, x).excludes_zero()
        }
        _ => false,
    }
}

impl PTerm {
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
            PTerm::MVar(_) => {}
        }
    }

    pub fn mentions(&self, name: &str) -> bool {
        let mut out = Vec::new();
        self.free_vars(&mut out);
        out.iter().any(|v| v == name)
    }

    pub fn free_mvars(&self, out: &mut Vec<usize>) {
        match self {
            PTerm::MVar(m) => {
                if !out.contains(m) {
                    out.push(*m);
                }
            }
            PTerm::Var(_) => {}
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
        match self {
            PTerm::Cmp(op, a, b) => PTerm::Cmp(
                *op,
                Box::new(a.subst(name, by)),
                Box::new(b.subst(name, by)),
            ),
            PTerm::Eq(a, b) => {
                PTerm::Eq(Box::new(a.subst(name, by)), Box::new(b.subst(name, by)))
            }
            PTerm::Not(p) => PTerm::Not(Box::new(p.subst(name, by))),
            PTerm::And(a, b) => PTerm::And(
                Box::new(a.subst(name, by)),
                Box::new(b.subst(name, by)),
            ),
            PTerm::Or(a, b) => PTerm::Or(
                Box::new(a.subst(name, by)),
                Box::new(b.subst(name, by)),
            ),
            PTerm::Pred(pf, a) => PTerm::Pred(*pf, Box::new(a.subst(name, by))),
            PTerm::Var(_) | PTerm::MVar(_) => self.clone(),
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
            (PTerm::Var(x), PTerm::Var(y)) => x == y,
            (PTerm::MVar(x), PTerm::MVar(y)) => x == y,
            _ => false,
        }
    }
}

impl fmt::Display for PTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PTerm::Cmp(op, a, b) => write!(f, "{a} {op} {b}"),
            PTerm::Eq(a, b) => write!(f, "Eq {a} {b}"),
            PTerm::Not(p) => write!(f, "Not ({p})"),
            PTerm::And(a, b) => write!(f, "({a} && {b})"),
            PTerm::Or(a, b) => write!(f, "({a} || {b})"),
            PTerm::Pred(pf, a) => write!(f, "{pf} {a}"),
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
    Param(String),
    /// Runtime identity (a region-like name) in argument position, as in
    /// `Io r`: identities unify by the region rules (flexible names take a
    /// rigid identity; distinct rigid identities never merge).
    Region(String),
    /// Unification variable.
    Meta(MetaId),
    /// `T a b ...`.
    Adt(String, Vec<Ty>),
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
    Lazy { eff: EffRow, region: String, inner: Box<Ty> },
    /// `Ref r a`: a shared borrow of `a` in region `r`.
    Ref(String, Box<Ty>),
    /// `Mut r a`: an exclusive borrow of `a` in region `r`.
    Mut(String, Box<Ty>),
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
        var: String,
        dom: Box<Ty>,
        mult: Mult,
        eff: EffRow,
        cod: Box<Ty>,
    },
    /// Dependent pair `Sigma (x :: A). B`, where `B` may mention `x`.
    Sigma {
        var: String,
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
            var: var.into(),
            dom: Box::new(dom),
            mult,
            eff,
            cod: Box::new(cod),
        }
    }

    /// A dependent pair type.
    pub fn sigma(var: impl Into<String>, dom: Ty, cod: Ty) -> Ty {
        Ty::Sigma {
            var: var.into(),
            dom: Box::new(dom),
            cod: Box::new(cod),
        }
    }

    /// View any function type as a (possibly non-dependent) Pi: `Ty::Fun`
    /// is a Pi whose binder nothing mentions.
    pub fn as_pi(&self) -> (String, &Ty, Mult, &EffRow, &Ty) {
        match self {
            Ty::Fun(a, m, e, b) => ("$".to_string(), a, *m, e, b),
            Ty::Pi { var, dom, mult, eff, cod } => (var.clone(), dom, *mult, eff, cod),
            _ => unreachable!("as_pi on a non-function type"),
        }
    }

    /// Does this type mention the index-level variable `name`?
    pub fn mentions_idx(&self, name: &str) -> bool {
        match self {
            Ty::Idx(t) => t.mentions(name),
            Ty::Prp(p) => p.mentions(name),
            Ty::Adt(_, args) | Ty::Tuple(args) => {
                args.iter().any(|a| a.mentions_idx(name))
            }
            Ty::Fun(a, _, _, b) | Ty::Pi { dom: a, cod: b, .. } => {
                a.mentions_idx(name) || b.mentions_idx(name)
            }
            Ty::Sigma { dom, cod, .. } => dom.mentions_idx(name) || cod.mentions_idx(name),
            Ty::Thunk { inner, .. } | Ty::Later(inner) => inner.mentions_idx(name),
            Ty::Lazy { inner, .. } => inner.mentions_idx(name),
            Ty::Ref(_, inner) | Ty::Mut(_, inner) => inner.mentions_idx(name),
            Ty::Ptr { pointee, .. } => pointee.mentions_idx(name),
            Ty::App(_, args) => args.iter().any(|a| a.mentions_idx(name)),
            Ty::Handler { input, output, .. } => {
                input.mentions_idx(name) || output.mentions_idx(name)
            }
            _ => false,
        }
    }

    /// Substitute the index-level variable `name` by the index term `by`
    /// (beta for dependent application and elimination).
    pub fn subst_idx(&self, name: &str, by: &ITerm) -> Ty {
        match self {
            Ty::Idx(t) => Ty::Idx(Box::new(t.subst(name, by))),
            Ty::Prp(p) => Ty::Prp(Box::new(p.subst(name, by))),
            Ty::Adt(n, args) => {
                Ty::Adt(n.clone(), args.iter().map(|a| a.subst_idx(name, by)).collect())
            }
            Ty::Tuple(args) => {
                Ty::Tuple(args.iter().map(|a| a.subst_idx(name, by)).collect())
            }
            Ty::Fun(a, m, e, b) => Ty::fun_mult(
                a.subst_idx(name, by),
                *m,
                e.clone(),
                b.subst_idx(name, by),
            ),
            Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi {
                var: var.clone(),
                dom: Box::new(dom.subst_idx(name, by)),
                mult: *mult,
                eff: eff.clone(),
                cod: Box::new(cod.subst_idx(name, by)),
            },
            Ty::Ptr { role, pointee } => Ty::Ptr {
                role: role.clone(),
                pointee: Box::new(pointee.subst_idx(name, by)),
            },
            Ty::Sigma { var, dom, cod } => Ty::Sigma {
                var: var.clone(),
                dom: Box::new(dom.subst_idx(name, by)),
                cod: Box::new(cod.subst_idx(name, by)),
            },
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: eff.clone(),
                inner: Box::new(inner.subst_idx(name, by)),
            },
            Ty::Later(inner) => Ty::Later(Box::new(inner.subst_idx(name, by))),
            Ty::Lazy { eff, region, inner } => Ty::Lazy {
                eff: eff.clone(),
                region: region.clone(),
                inner: Box::new(inner.subst_idx(name, by)),
            },
            Ty::Ref(region, inner) => {
                Ty::Ref(region.clone(), Box::new(inner.subst_idx(name, by)))
            }
            Ty::Mut(region, inner) => {
                Ty::Mut(region.clone(), Box::new(inner.subst_idx(name, by)))
            }
            Ty::Handler { label, input, output, eff, resumption } => Ty::Handler {
                label: label.clone(),
                input: Box::new(input.subst_idx(name, by)),
                output: Box::new(output.subst_idx(name, by)),
                eff: eff.clone(),
                resumption: *resumption,
            },
            Ty::App(h, args) => Ty::App(
                Box::new(h.subst_idx(name, by)),
                args.iter().map(|a| a.subst_idx(name, by)).collect(),
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

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Int => write!(f, "Int"),
            Ty::Nat => write!(f, "Nat"),
            Ty::Bool => write!(f, "Bool"),
            Ty::Unit => write!(f, "Unit"),
            Ty::Bytes => write!(f, "Bytes"),
            Ty::Sym => write!(f, "Sym"),
            Ty::Prim(s) => write!(f, "{s}"),
            Ty::Param(p) => write!(f, "{p}"),
            Ty::Region(r) => write!(f, "{r}"),
            Ty::Meta(MetaId(m)) => write!(f, "?m{m}"),
            Ty::Adt(t, args) => {
                write!(f, "{t}")?;
                for a in args {
                    if needs_parens_as_argument(a) {
                        write!(f, " ({a})")?;
                    } else {
                        write!(f, " {a}")?;
                    }
                }
                Ok(())
            }
            Ty::App(h, args) => {
                write!(f, "({h}")?;
                for a in args {
                    if needs_parens_as_argument(a) {
                        write!(f, " ({a})")?;
                    } else {
                        write!(f, " {a}")?;
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
                    write!(f, "{a}")?;
                }
                write!(f, ")")
            }
            Ty::Fun(a, m, e, b) => match (m, e.is_pure()) {
                (Mult::Many, true) => write!(f, "({a} -> {b})"),
                (Mult::Many, false) => write!(f, "({a} ->{e} {b})"),
                (_, true) => write!(f, "({a} {m} -> {b})"),
                (_, false) => write!(f, "({a} {m} ->{e} {b})"),
            },
            Ty::Thunk { eff, inner } => write!(f, "Thunk {eff} {inner}"),
            Ty::Later(inner) => write!(f, "Later {inner}"),
            Ty::Lazy { eff, region, inner } => write!(f, "Lazy {eff} {region} {inner}"),
            Ty::Ref(region, inner) => write!(f, "Ref {region} {inner}"),
            Ty::Mut(region, inner) => write!(f, "Mut {region} {inner}"),
            Ty::Ptr { role, pointee } => match pointee.as_ref() {
                Ty::Ptr { .. } | Ty::Fun(..) | Ty::Pi { .. } | Ty::Sigma { .. } => {
                    write!(f, "{role} ({pointee})")
                }
                other => write!(f, "{role} {other}"),
            },
            Ty::Handler { label, input, output, eff, resumption } => {
                write!(f, "(handle {label} {input} => {output} adding {eff} resuming {resumption})")
            }
            Ty::Sort(s) => write!(f, "{s}"),
            Ty::Idx(t) => write!(f, "{t}"),
            Ty::Prp(p) => write!(f, "{p}"),
            Ty::Pi { var, dom, mult, eff, cod } => {
                if *mult == Mult::Many && eff.is_pure() {
                    write!(f, "(({var} :: {dom}) -> {cod})")
                } else {
                    write!(f, "(({var} :: {dom}) {mult} ->{eff} {cod})")
                }
            }
            Ty::Sigma { var, dom, cod } => {
                write!(f, "(Sigma ({var} :: {dom}). {cod})")
            }
        }
    }
}

/// A class application in a constraint, e.g. `Add a`.
#[derive(Clone, Debug, PartialEq)]
pub struct ClassApp {
    pub class: String,
    pub args: Vec<Ty>,
}

impl fmt::Display for ClassApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.class)?;
        for a in &self.args {
            if needs_parens_as_argument(a) {
                write!(f, " ({a})")?;
            } else {
                write!(f, " {a}")?;
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
#[derive(Clone, Debug)]
pub struct Scheme {
    pub params: Vec<String>,
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
    pub fn mono(ty: Ty) -> Scheme {
        Scheme {
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
        Ty::Param(p) if regions.contains(p) => Ty::Region(p.clone()),
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
                if !regions.contains(a) {
                    regions.push(a.clone());
                }
            }
        }
        if let RowTail::Rigid(t) = &r.tail {
            if !rows.contains(t) {
                rows.push(t.clone());
            }
        }
    };
    match t {
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
            if !regions.contains(region) {
                regions.push(region.clone());
            }
            collect_row_names(inner, rows, regions);
        }
        Ty::Ref(region, inner) | Ty::Mut(region, inner) => {
            if !regions.contains(region) {
                regions.push(region.clone());
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
            if !regions.contains(r) {
                regions.push(r.clone());
            }
        }
        Ty::Handler { label, input, output, eff, .. } => {
            if let Some(a) = &label.arg {
                if !regions.contains(a) {
                    regions.push(a.clone());
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
#[derive(Clone, Default)]
pub struct Unifier {
    subst: Vec<Option<Ty>>,
    row_subst: Vec<Option<EffRow>>,
    /// Tail metavariables that must lack a label (memo black-hole rule).
    lacks: Vec<(RowMetaId, EffLabel)>,
    /// Region-name substitution: a flexible name (created when a scheme
    /// is instantiated) may be identified with one rigid region.
    region_subst: HashMap<String, String>,
    /// Region names that may still be identified with another region.
    flex_regions: Vec<String>,
    /// Inference variables for index values (implicit index arguments).
    idx_subst: Vec<Option<ITerm>>,
    /// Inference variables for propositions (implicit evidence types).
    prop_subst: Vec<Option<PTerm>>,
}

impl Unifier {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fresh(&mut self) -> Ty {
        let id = self.subst.len();
        self.subst.push(None);
        Ty::Meta(MetaId(id))
    }

    /// A fresh index-value metavariable, as a type (`?i` in index
    /// position).
    pub fn fresh_idx(&mut self) -> Ty {
        let id = self.idx_subst.len();
        self.idx_subst.push(None);
        Ty::Idx(Box::new(ITerm::MVar(id)))
    }

    /// A fresh proposition metavariable, as a type.
    pub fn fresh_prop(&mut self) -> Ty {
        let id = self.prop_subst.len();
        self.prop_subst.push(None);
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
        EffRow { labels: Vec::new(), tail: RowTail::Meta(RowMetaId(id)) }
    }

    /// Record a region name as flexible: the fresh name an instantiation
    /// gives a `forall`-bound region. Flexible names take the identity of
    /// the rigid region they are used at; rigid names never merge.
    pub fn note_flex_region(&mut self, name: &str) {
        if !self.flex_regions.iter().any(|x| x == name) {
            self.flex_regions.push(name.to_string());
        }
    }

    /// Is this (resolved) region name still flexible: not yet pinned to a
    /// rigid identity?
    pub fn is_flex_region(&self, name: &str) -> bool {
        let r = self.resolve_region(name);
        self.flex_regions.contains(&r)
    }

    /// Follow the region-name substitution.
    pub fn resolve_region(&self, r: &str) -> String {
        match self.region_subst.get(r) {
            Some(next) => self.resolve_region(next),
            None => r.to_string(),
        }
    }

    /// Identify two region names. Equal names agree; a flexible name takes
    /// the other side's identity; two distinct rigid names (scope regions,
    /// memo groups, signature binders) never meet.
    pub fn unify_regions(&mut self, a: &str, b: &str) -> std::result::Result<(), String> {
        let ra = self.resolve_region(a);
        let rb = self.resolve_region(b);
        if ra == rb {
            return Ok(());
        }
        let a_flex = self.flex_regions.iter().any(|x| *x == ra);
        let b_flex = self.flex_regions.iter().any(|x| *x == rb);
        if a_flex {
            self.region_subst.insert(ra, rb);
            Ok(())
        } else if b_flex {
            self.region_subst.insert(rb, ra);
            Ok(())
        } else {
            Err(format!("regions do not match: {ra} vs {rb}"))
        }
    }

    /// Follow metavar bindings one level.
    fn shallow(&self, ty: &Ty) -> Ty {
        match ty {
            Ty::Meta(MetaId(m)) => match self.subst.get(*m).and_then(|s| s.as_ref()) {
                Some(t) => self.shallow(t),
                None => ty.clone(),
            },
            other => other.clone(),
        }
    }

    /// Follow index metavariable bindings.
    pub fn resolve_idx(&self, t: &ITerm) -> ITerm {
        match t {
            ITerm::MVar(m) => match self.idx_subst.get(*m).and_then(|s| s.as_ref()) {
                Some(next) => self.resolve_idx(next),
                None => t.clone(),
            },
            ITerm::App(f, args) => {
                ITerm::App(
                    Box::new(self.resolve_idx(f)),
                    args.iter().map(|a| self.resolve_idx(a)).collect(),
                )
            }
            ITerm::Bin(op, a, b) => ITerm::Bin(
                *op,
                Box::new(self.resolve_idx(a)),
                Box::new(self.resolve_idx(b)),
            ),
            other => other.clone(),
        }
    }

    /// Follow proposition metavariable bindings.
    pub fn resolve_prop(&self, p: &PTerm) -> PTerm {
        match p {
            PTerm::MVar(m) => match self.prop_subst.get(*m).and_then(|s| s.as_ref()) {
                Some(next) => self.resolve_prop(next),
                None => p.clone(),
            },
            PTerm::Cmp(op, a, b) => PTerm::Cmp(
                *op,
                Box::new(self.resolve_idx(a)),
                Box::new(self.resolve_idx(b)),
            ),
            PTerm::Eq(a, b) => {
                PTerm::Eq(Box::new(self.resolve_idx(a)), Box::new(self.resolve_idx(b)))
            }
            PTerm::Not(q) => PTerm::Not(Box::new(self.resolve_prop(q))),
            PTerm::And(a, b) => PTerm::And(
                Box::new(self.resolve_prop(a)),
                Box::new(self.resolve_prop(b)),
            ),
            PTerm::Or(a, b) => PTerm::Or(
                Box::new(self.resolve_prop(a)),
                Box::new(self.resolve_prop(b)),
            ),
            PTerm::Pred(pf, a) => PTerm::Pred(*pf, Box::new(self.resolve_idx(a))),
            other => other.clone(),
        }
    }

    fn idx_occurs(&self, id: usize, t: &ITerm) -> bool {
        match self.resolve_idx(t) {
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
                self.idx_occurs_any(id, &a) || self.idx_occurs_any(id, &b)
            }
            PTerm::Not(q) => self.prop_occurs(id, &q),
            PTerm::And(a, b) | PTerm::Or(a, b) => {
                self.prop_occurs(id, &a) || self.prop_occurs(id, &b)
            }
            _ => false,
        }
    }

    /// Does the index term mention index metavariable `id`? (the occurs
    /// check for propositions over index terms).
    fn idx_occurs_any(&self, id: usize, t: &ITerm) -> bool {
        self.idx_occurs(id, t)
    }

    /// Bind an index metavariable.
    pub fn bind_idx(&mut self, id: usize, t: ITerm) -> std::result::Result<(), String> {
        let t = self.resolve_idx(&t);
        if self.idx_occurs(id, &t) {
            return Err(format!(
                "cannot construct infinite index: ?i{id} in {t}"
            ));
        }
        self.idx_subst[id] = Some(t);
        Ok(())
    }

    /// Bind a proposition metavariable.
    pub fn bind_prop(&mut self, id: usize, p: PTerm) -> std::result::Result<(), String> {
        let p = self.resolve_prop(&p);
        if self.prop_occurs(id, &p) {
            return Err(format!(
                "cannot construct infinite proposition: ?p{id} in {p}"
            ));
        }
        self.prop_subst[id] = Some(p);
        Ok(())
    }

    /// Unify two index terms: metavariables bind, everything else is
    /// structural with definitional-equality validation. Heuristic
    /// bindings (inverting `?k + 1` against a numeral) are guesses that
    /// the final conversion check must confirm.
    pub fn unify_idx(&mut self, a: &ITerm, b: &ITerm) -> std::result::Result<(), String> {
        let mut trial = self.clone();
        trial.unify_idx_raw(a, b)?;
        if trial.resolve_idx(a).eq(&trial.resolve_idx(b)) {
            *self = trial;
            Ok(())
        } else {
            Err(format!(
                "index mismatch: {} vs {}",
                self.resolve_idx(a),
                self.resolve_idx(b)
            ))
        }
    }

    fn unify_idx_raw(&mut self, a: &ITerm, b: &ITerm) -> std::result::Result<(), String> {
        let a = self.resolve_idx(a);
        let b = self.resolve_idx(b);
        match (&a, &b) {
            (ITerm::MVar(m), _) => self.bind_idx(*m, b),
            (_, ITerm::MVar(m)) => self.bind_idx(*m, a),
            (ITerm::Var(x), ITerm::Var(y)) if x == y => Ok(()),
            (ITerm::Num(x), ITerm::Num(y)) if x == y => Ok(()),
            (ITerm::Fn(x), ITerm::Fn(y)) if x == y => Ok(()),
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
                    Err(format!("index mismatch: {a} vs {b}"))
                }
            }
        }
    }

    /// Unify two propositions (validated like index terms).
    pub fn unify_prop(&mut self, a: &PTerm, b: &PTerm) -> std::result::Result<(), String> {
        let mut trial = self.clone();
        trial.unify_prop_raw(a, b)?;
        if trial.resolve_prop(a).eq(&trial.resolve_prop(b)) {
            *self = trial;
            Ok(())
        } else {
            Err(format!(
                "propositions do not match: {} vs {}",
                self.resolve_prop(a),
                self.resolve_prop(b)
            ))
        }
    }

    fn unify_prop_raw(&mut self, a: &PTerm, b: &PTerm) -> std::result::Result<(), String> {
        let a = self.resolve_prop(a);
        let b = self.resolve_prop(b);
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
                    Err(format!("propositions do not match: {a} vs {b}"))
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
        match self.shallow(ty) {
            Ty::Meta(m) => match map.and_then(|mp| mp.get(&m)) {
                Some(t) => self.zonk(t),
                None => Ty::Meta(m),
            },
            Ty::Adt(t, args) => Ty::Adt(t, args.iter().map(z).collect()),
            Ty::App(h, args) => Ty::apply_spine(z(&h), args.iter().map(z).collect::<Vec<_>>()),
            Ty::Tuple(args) => Ty::Tuple(args.iter().map(z).collect()),
            Ty::Fun(a, m, e, b) => Ty::fun_mult(z(&a), m, self.resolve_row(&e), z(&b)),
            Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi {
                var,
                dom: Box::new(z(&dom)),
                mult,
                eff: self.resolve_row(&eff),
                cod: Box::new(z(&cod)),
            },
            Ty::Sigma { var, dom, cod } => Ty::Sigma {
                var,
                dom: Box::new(z(&dom)),
                cod: Box::new(z(&cod)),
            },
            Ty::Idx(t) => Ty::Idx(Box::new(self.resolve_idx(&t))),
            Ty::Prp(p) => Ty::Prp(Box::new(self.resolve_prop(&p))),
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: self.resolve_row(&eff),
                inner: Box::new(z(&inner)),
            },
            Ty::Later(inner) => Ty::Later(Box::new(z(&inner))),
            Ty::Lazy { eff, region, inner } => Ty::Lazy {
                eff: self.resolve_row(&eff),
                region: self.resolve_region(&region),
                inner: Box::new(z(&inner)),
            },
            Ty::Ref(region, inner) => Ty::Ref(self.resolve_region(&region), Box::new(z(&inner))),
            Ty::Mut(region, inner) => Ty::Mut(self.resolve_region(&region), Box::new(z(&inner))),
            Ty::Ptr { role, pointee } => Ty::Ptr { role: role.clone(), pointee: Box::new(z(&pointee)) },
            Ty::Region(r) => Ty::Region(self.resolve_region(&r)),
            Ty::Handler { label, input, output, eff, resumption } => Ty::Handler {
                label: EffLabel {
                    head: label.head.clone(),
                    arg: label.arg.as_ref().map(|a| self.resolve_region(a)),
                },
                input: Box::new(z(&input)),
                output: Box::new(z(&output)),
                eff: self.resolve_row(&eff),
                resumption,
            },
            t => t,
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

    fn gen_ty(&self, ty: &Ty, names: &HashMap<EitherMeta, String>) -> Ty {
        match self.shallow(ty) {
            Ty::Meta(m) => match names.get(&EitherMeta::Type(m)) {
                Some(n) => Ty::Param(n.clone()),
                None => Ty::Meta(m),
            },
            Ty::Adt(n, args) => {
                Ty::Adt(n, args.iter().map(|a| self.gen_ty(a, names)).collect())
            }
            Ty::Tuple(args) => {
                Ty::Tuple(args.iter().map(|a| self.gen_ty(a, names)).collect())
            }
            Ty::Fun(a, m, e, b) => Ty::fun_mult(
                self.gen_ty(&a, names),
                m,
                self.gen_row(&e, names),
                self.gen_ty(&b, names),
            ),
            Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi {
                var,
                dom: Box::new(self.gen_ty(&dom, names)),
                mult,
                eff: self.gen_row(&eff, names),
                cod: Box::new(self.gen_ty(&cod, names)),
            },
            Ty::Sigma { var, dom, cod } => Ty::Sigma {
                var,
                dom: Box::new(self.gen_ty(&dom, names)),
                cod: Box::new(self.gen_ty(&cod, names)),
            },
            Ty::Idx(t) => Ty::Idx(Box::new(self.resolve_idx(&t))),
            Ty::Prp(p) => Ty::Prp(Box::new(self.resolve_prop(&p))),
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: self.gen_row(&eff, names),
                inner: Box::new(self.gen_ty(&inner, names)),
            },
            Ty::Later(inner) => Ty::Later(Box::new(self.gen_ty(&inner, names))),
            Ty::Lazy { eff, region, inner } => Ty::Lazy {
                eff: self.gen_row(&eff, names),
                region,
                inner: Box::new(self.gen_ty(&inner, names)),
            },
            Ty::Ref(region, inner) => {
                Ty::Ref(region, Box::new(self.gen_ty(&inner, names)))
            }
            Ty::Mut(region, inner) => {
                Ty::Mut(region, Box::new(self.gen_ty(&inner, names)))
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
                resumption,
            },
            t => t,
        }
    }

    fn gen_row(&self, row: &EffRow, names: &HashMap<EitherMeta, String>) -> EffRow {
        let r = self.resolve_row(row);
        EffRow {
            labels: r.labels,
            tail: match r.tail {
                RowTail::Meta(m) => match names.get(&EitherMeta::Row(m)) {
                    Some(n) => RowTail::Rigid(n.clone()),
                    None => RowTail::Meta(m),
                },
                t => t,
            },
        }
    }

    fn occurs(&self, id: MetaId, ty: &Ty) -> bool {
        match self.shallow(ty) {
            Ty::Meta(m) => m == id,
            Ty::Adt(_, args) | Ty::Tuple(args) => args.iter().any(|a| self.occurs(id, a)),
            Ty::Fun(a, _, _, b) => self.occurs(id, &a) || self.occurs(id, &b),
            Ty::Pi { dom, cod, .. } => self.occurs(id, &dom) || self.occurs(id, &cod),
            Ty::Sigma { dom, cod, .. } => self.occurs(id, &dom) || self.occurs(id, &cod),
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
        for (m, label) in self.lacks.clone() {
            if m == id && row.contains(&label) {
                return Err(format!(
                    "effect row must not contain {label} (a memo cell cannot demand its own group)"
                ));
            }
            if let RowTail::Meta(other) = row.tail {
                if other != m {
                    // Propagate the lacks constraint to the new tail.
                    self.lacks.push((other, label));
                }
            }
        }
        self.lacks.retain(|(m, _)| *m != id);
        self.row_subst[id.0] = Some(row);
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
            (Some(x), Some(y)) => {
                let mut trial = self.clone();
                if trial.unify_regions(x, y).is_ok() {
                    *self = trial;
                    true
                } else {
                    false
                }
            }
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
                // The metavariable can take the rigid row's shape.
                let extra = if matches!(ra.tail, RowTail::Rigid(_)) { b_only } else { a_only };
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
                return Err(format!(
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
                    return Err(format!(
                        "cannot construct infinite type: {} in {}",
                        m_display(*m),
                        self.zonk(&image)
                    ));
                }
                self.subst[m.0] = Some(image.clone());
                let rebuilt = eta(&image, &[], &xs);
                return self.unify(&rebuilt, b);
            }
        }
        if let Ty::Meta(m) = &h2 {
            if ys.len() <= xs.len() {
                let k = xs.len() - ys.len();
                let image = Ty::apply_spine(h1.clone(), xs[..k].to_vec());
                if self.occurs(*m, &image) {
                    return Err(format!(
                        "cannot construct infinite type: {} in {}",
                        m_display(*m),
                        self.zonk(&image)
                    ));
                }
                self.subst[m.0] = Some(image.clone());
                let rebuilt = eta(&image, &[], &ys);
                return self.unify(a, &rebuilt);
            }
        }
        if xs.len() != ys.len() {
            return Err(format!(
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

    pub fn unify(&mut self, a: &Ty, b: &Ty) -> std::result::Result<(), String> {
        let a = self.shallow(a);
        let b = self.shallow(b);
        match (&a, &b) {
            (Ty::Int, Ty::Int) | (Ty::Nat, Ty::Nat) | (Ty::Bytes, Ty::Bytes) | (Ty::Bool, Ty::Bool)
            | (Ty::Sym, Ty::Sym) | (Ty::Unit, Ty::Unit) => Ok(()),
            (Ty::Prim(a), Ty::Prim(b)) if a == b => Ok(()),
            (Ty::Param(x), Ty::Param(y)) if x == y => Ok(()),
            (Ty::Meta(m), t) | (t, Ty::Meta(m)) => {
                if let Ty::Meta(m2) = t {
                    if m == m2 {
                        return Ok(());
                    }
                }
                if self.occurs(*m, t) {
                    return Err(format!(
                        "cannot construct infinite type: {} in {}",
                        m_display(*m),
                        self.zonk(t)
                    ));
                }
                self.subst[m.0] = Some(t.clone());
                Ok(())
            }
            (Ty::Adt(t1, a1), Ty::Adt(t2, a2)) if t1 == t2 && a1.len() == a2.len() => {
                for (x, y) in a1.iter().zip(a2) {
                    self.unify(x, y)?;
                }
                Ok(())
            }
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
            (Ty::Later(_), Ty::Thunk { .. }) | (Ty::Thunk { .. }, Ty::Later(_)) => Err(format!(
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
                    return Err(format!(
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
                    return Err(format!(
                        "type mismatch: expected {}, got {}",
                        self.zonk(&a),
                        self.zonk(&b)
                    ));
                }
                match (&l1.arg, &l2.arg) {
                    (Some(x), Some(y)) => self.unify_regions(x, y)?,
                    (None, None) => {}
                    _ => {
                        return Err(format!(
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
                let fresh = format!("$sigma{}", self.subst.len());
                self.unify(&c1.subst_idx(v1, &ITerm::Var(fresh.clone())), &c2.subst_idx(v2, &ITerm::Var(fresh)))
            }
            // A boundary pointer role is never constructed from an
            // integer or any other non-pointer (foreign function
            // interface specification section 3: safe callers MUST NOT
            // construct these roles by casting integers).
            (Ty::Ptr { .. }, other) | (other, Ty::Ptr { .. })
                if !matches!(other, Ty::Ptr { .. } | Ty::Meta(_)) =>
            {
                Err(format!(
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
                    Err(format!("type mismatch: expected {}, got {}", self.zonk(&a), self.zonk(&b)))
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
        let fresh = format!("$pi{}", self.subst.len());
        self.unify(
            &c1.subst_idx(v1, &ITerm::Var(fresh.clone())),
            &c2.subst_idx(v2, &ITerm::Var(fresh)),
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
            let fresh = "$conv".to_string();
            ty_eq(&c1.subst_idx(&v1, &ITerm::Var(fresh.clone())), &c2.subst_idx(&v2, &ITerm::Var(fresh)))
        }
        (Ty::Sigma { var: v1, dom: d1, cod: c1 }, Ty::Sigma { var: v2, dom: d2, cod: c2 }) => {
            if !ty_eq(d1, d2) {
                return false;
            }
            let fresh = "$conv".to_string();
            ty_eq(&c1.subst_idx(v1, &ITerm::Var(fresh.clone())), &c2.subst_idx(v2, &ITerm::Var(fresh)))
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

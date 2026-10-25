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

/// Monomorphic type: rigid parameters, unification variables, primitives,
/// algebraic data types, functions with effect rows and parameter usage
/// multiplicities, affine thunks, memo cells and borrows.
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
    /// A machine scalar primitive (`I32`, `U8`, `F64`, `Char`, ...).
    Prim(Scalar),
    /// Rigid type parameter bound by a `forall`.
    Param(String),
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
    /// `Lazy {e} r a`: a memo cell in region `r`.
    Lazy { eff: EffRow, region: String, inner: Box<Ty> },
    /// `Ref r a`: a shared borrow of `a` in region `r`.
    Ref(String, Box<Ty>),
    /// `Mut r a`: an exclusive borrow of `a` in region `r`.
    Mut(String, Box<Ty>),
}

impl Ty {
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

    /// Count the arrows of a function type.
    pub fn arity(&self) -> usize {
        match self {
            Ty::Fun(_, _, _, b) => 1 + b.arity(),
            _ => 0,
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
            Ty::Thunk { eff, inner } => {
                eff.free_metas(out);
                inner.free_metas(out);
            }
            Ty::Lazy { eff, inner, .. } => {
                eff.free_metas(out);
                inner.free_metas(out);
            }
            Ty::Ref(_, inner) | Ty::Mut(_, inner) => inner.free_metas(out),
            _ => {}
        }
    }
}

/// The default multiplicity of an unannotated binder (ownership and usage
/// design, section 2): thunks are affine because `force` consumes and
/// dropping an unforced thunk destroys it (laziness specification,
/// section 3); everything else is unrestricted in this slice.
pub fn default_mult(ty: &Ty) -> Mult {
    match ty {
        Ty::Thunk { .. } => Mult::Aff,
        _ => Mult::Many,
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

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Int => write!(f, "Int"),
            Ty::Nat => write!(f, "Nat"),
            Ty::Bool => write!(f, "Bool"),
            Ty::Unit => write!(f, "Unit"),
            Ty::Bytes => write!(f, "Bytes"),
            Ty::Prim(s) => write!(f, "{s}"),
            Ty::Param(p) => write!(f, "{p}"),
            Ty::Meta(MetaId(m)) => write!(f, "?m{m}"),
            Ty::Adt(t, args) => {
                write!(f, "{t}")?;
                for a in args {
                    write!(f, " {a}")?;
                }
                Ok(())
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
            Ty::Lazy { eff, region, inner } => write!(f, "Lazy {eff} {region} {inner}"),
            Ty::Ref(region, inner) => write!(f, "Ref {region} {inner}"),
            Ty::Mut(region, inner) => write!(f, "Mut {region} {inner}"),
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
            write!(f, " {a}")?;
        }
        Ok(())
    }
}

/// A polymorphic scheme. `params`, `row_params` and `region_params` are the
/// quantified type, row and region variables; `ctx` holds `using` constraints.
#[derive(Clone, Debug)]
pub struct Scheme {
    pub params: Vec<String>,
    pub row_params: Vec<String>,
    pub region_params: Vec<String>,
    pub ctx: Vec<(String, ClassApp)>,
    pub ty: Ty,
}

impl Scheme {
    pub fn mono(ty: Ty) -> Scheme {
        Scheme {
            params: Vec::new(),
            row_params: Vec::new(),
            region_params: Vec::new(),
            ctx: Vec::new(),
            ty,
        }
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
        Ty::Adt(_, args) | Ty::Tuple(args) => {
            for a in args {
                collect_row_names(a, rows, regions);
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

    /// Resolve a row's tail through the row substitution, flattening the
    /// labels a bound tail contributes.
    pub fn resolve_row(&self, row: &EffRow) -> EffRow {
        let mut labels = row.labels.clone();
        let mut tail = row.tail.clone();
        loop {
            match tail {
                RowTail::Meta(m) => match self.row_subst.get(m.0).and_then(|s| s.as_ref()) {
                    Some(r) => {
                        for l in &r.labels {
                            if !labels.contains(l) {
                                labels.push(l.clone());
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
        match self.shallow(ty) {
            Ty::Adt(t, args) => Ty::Adt(t, args.iter().map(|a| self.zonk(a)).collect()),
            Ty::Tuple(args) => Ty::Tuple(args.iter().map(|a| self.zonk(a)).collect()),
            Ty::Fun(a, m, e, b) => {
                Ty::fun_mult(self.zonk(&a), m, self.resolve_row(&e), self.zonk(&b))
            }
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: self.resolve_row(&eff),
                inner: Box::new(self.zonk(&inner)),
            },
            Ty::Lazy { eff, region, inner } => Ty::Lazy {
                eff: self.resolve_row(&eff),
                region: self.resolve_region(&region),
                inner: Box::new(self.zonk(&inner)),
            },
            Ty::Ref(region, inner) => {
                Ty::Ref(self.resolve_region(&region), Box::new(self.zonk(&inner)))
            }
            Ty::Mut(region, inner) => {
                Ty::Mut(self.resolve_region(&region), Box::new(self.zonk(&inner)))
            }
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
            Ty::Thunk { eff, inner } => Ty::Thunk {
                eff: self.gen_row(&eff, names),
                inner: Box::new(self.gen_ty(&inner, names)),
            },
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
            Ty::Thunk { inner, .. } => self.occurs(id, &inner),
            Ty::Lazy { inner, .. } => self.occurs(id, &inner),
            Ty::Ref(_, inner) | Ty::Mut(_, inner) => self.occurs(id, &inner),
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

    /// Row unification with extension: the extra labels of each row must be
    /// absorbed by the other row's tail.
    pub fn unify_rows(
        &mut self,
        a: &EffRow,
        b: &EffRow,
    ) -> std::result::Result<(), String> {
        let ra = self.resolve_row(a);
        let rb = self.resolve_row(b);
        let mut a_only: Vec<EffLabel> =
            ra.labels.iter().filter(|l| !rb.contains(l)).cloned().collect();
        let mut b_only: Vec<EffLabel> =
            rb.labels.iter().filter(|l| !ra.contains(l)).cloned().collect();
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

    pub fn unify(&mut self, a: &Ty, b: &Ty) -> std::result::Result<(), String> {
        let a = self.shallow(a);
        let b = self.shallow(b);
        match (&a, &b) {
            (Ty::Int, Ty::Int) | (Ty::Nat, Ty::Nat) | (Ty::Bytes, Ty::Bytes) | (Ty::Bool, Ty::Bool)
            | (Ty::Unit, Ty::Unit) => Ok(()),
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
            _ => Err(format!("type mismatch: expected {}, got {}", self.zonk(&a), self.zonk(&b))),
        }
    }

    /// Row union: the accumulation operation for performed effects. The
    /// result contains both rows' labels; tails are joined through the row
    /// substitution.
    pub fn union_rows(&mut self, a: &EffRow, b: &EffRow) -> std::result::Result<EffRow, String> {
        let ra = self.resolve_row(a);
        let rb = self.resolve_row(b);
        let mut labels = ra.labels.clone();
        for l in &rb.labels {
            if !labels.contains(l) {
                labels.push(l.clone());
            }
        }
        labels.sort_by(|x, y| x.to_string().cmp(&y.to_string()));
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

fn m_display(MetaId(m): MetaId) -> String {
    format!("?m{m}")
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

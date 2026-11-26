//! Native compile-time evaluator for promoted pure Cinch functions.
//!
//! Instead of encoding type-level functions as quadratic type families and
//! defunctionalization symbols (as in Haskell's `singletons`), Cinch compiles
//! ordinary pure functions directly into a compact, slot-indexed expression IR
//! (`PExpr`) and evaluates them natively in Rust during type checking.
//!
//! Key properties:
//! - **Single definition**: a pure function works at the value level, at the
//!   type level, and on zero-cost `Sing` values with no boilerplate.
//! - **Fast native execution**: O(1) slot indexing (`Var(usize)`), direct
//!   `u128`/`i128` arithmetic, native closures & partial application (no
//!   defunctionalization symbols), and bounded evaluation fuel.
//! - **Symbolic / partial evaluation**: expressions stuck on a rigid type or
//!   index variable gracefully remain neutral (`PVal::Neutral`), while still
//!   reducing known outer structure (e.g. `append [2, 3] rest -> [2, 3 | rest]`,
//!   `add m n -> m + n`).

use std::collections::HashMap;
use std::sync::Arc;

use crate::types::{ITerm, IdxOp, Ty, PROMOTED_CONS, PROMOTED_NIL};

/// Canonical promoted constructor identities for `Bool` values at the type level.
pub const PROMOTED_TRUE: &str = "ctor:Bool.True";
pub const PROMOTED_FALSE: &str = "ctor:Bool.False";

/// Prefix for promoted function application heads (`Ty::Adt("fn:<id>", args)`).
pub const PROMOTED_FN_PREFIX: &str = "fn:";

/// Prefix for first-class promoted function values (`Ty::Adt("fnref:<id>", [])`).
pub const PROMOTED_FNREF_PREFIX: &str = "fnref:";

/// Default evaluation step budget per top-level promoted reduction.
/// Large enough for substantial compile-time algorithms (sorts, shapes, matrix
/// layouts, arithmetic), small enough that non-terminating functions fail fast.
pub const PROMOTE_EVAL_FUEL: usize = 16_384;

/// Maximum recursive call depth in the native compile-time evaluator so
/// non-terminating functions fail cleanly without overflowing the host stack.
pub const MAX_PROMOTE_CALL_DEPTH: usize = 256;

/// Built-in binary operators supported in promoted functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    StrConcat,
    ListAppend,
}

/// Built-in unary operators supported in promoted functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PUnaryOp {
    Neg,
    Not,
}

/// Native fast-path prelude functions available even when base modules are
/// loaded from a precompiled `.cinchpkg` without source ASTs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NativeBuiltin {
    Not,
    Fst,
    Snd,
    ListAppend,
    ListReverse,
    ListLength,
    ListMap,
    ListFoldl,
    ListFoldr,
    ListFilter,
    ListZipWith,
    ListTake,
    ListDrop,
    ListReplicate,
    ListNull,
    ListSum,
    ListProduct,
    StrConcat,
    NatMin,
    NatMax,
    BinOp(PBinOp),
}

/// Pattern in a promoted `case` expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PPat {
    Wild,
    /// Binds the matched value into the next local stack slot.
    Bind,
    /// `x@p`: binds the matched value AND matches `p`.
    As(Box<PPat>),
    Nat(u128),
    Int(i128),
    Bool(bool),
    Str(String),
    Unit,
    Ctor {
        identity: String,
        args: Vec<PPat>,
    },
    Tuple(Vec<PPat>),
}

/// One arm of a promoted `case` expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PCaseArm {
    pub pat: PPat,
    pub guard: Option<PExpr>,
    pub body: PExpr,
}

/// Compact, slot-indexed compile-time expression IR.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PExpr {
    /// Lexical slot index into the evaluation stack (`0..params`, then lets/patterns).
    Var(usize),
    Nat(u128),
    Int(i128),
    Bool(bool),
    Str(String),
    Unit,
    Ctor {
        identity: String,
        args: Vec<PExpr>,
    },
    Tuple(Vec<PExpr>),
    /// Reference to a promoted function by its canonical identity (`Module.name`).
    FnRef(String),
    /// Reference to a native builtin (including first-class operators like `(+)`)
    Native(NativeBuiltin),
    /// Closure literal capturing current stack values up to `capture_count`.
    Lam {
        arity: usize,
        body: Arc<PExpr>,
    },
    App {
        func: Box<PExpr>,
        args: Vec<PExpr>,
    },
    BinOp {
        op: PBinOp,
        lhs: Box<PExpr>,
        rhs: Box<PExpr>,
    },
    UnaryOp {
        op: PUnaryOp,
        arg: Box<PExpr>,
    },
    If {
        cond: Box<PExpr>,
        then_br: Box<PExpr>,
        else_br: Box<PExpr>,
    },
    Let {
        value: Box<PExpr>,
        body: Box<PExpr>,
    },
    Case {
        scrut: Box<PExpr>,
        arms: Vec<PCaseArm>,
    },
}

/// Implementation backing a `PromotedFn`: either compiled from Cinch source
/// equations or a native Rust fast-path for precompiled prelude functions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PromotedBody {
    Expr(Arc<PExpr>),
    Native(NativeBuiltin),
}

/// A pure Cinch function promoted for native compile-time evaluation and
/// zero-cost singleton lifting.
#[derive(Clone, Debug, PartialEq)]
pub struct PromotedFn {
    /// Canonical identity: `"Module.name"`.
    pub identity: String,
    /// Quantified type parameter names (e.g. `["a"]` for `reverse :: forall a. List a -> List a`).
    pub type_params: Vec<String>,
    /// Parameter types (unrolled from curried arrows).
    pub param_tys: Vec<Ty>,
    /// Return type.
    pub ret_ty: Ty,
    /// True when the return type is an index scalar (`Nat`, `Int`, or machine int).
    pub returns_idx: bool,
    /// Evaluable body.
    pub body: PromotedBody,
}

impl PromotedFn {
    pub fn arity(&self) -> usize {
        self.param_tys.len()
    }
}

/// Value domain of the compile-time evaluator.
#[derive(Clone, Debug, PartialEq)]
pub enum PVal {
    Nat(u128),
    Int(i128),
    Bool(bool),
    Str(String),
    Unit,
    Ctor {
        identity: String,
        args: Vec<PVal>,
    },
    Tuple(Vec<PVal>),
    FnRef(String),
    Native(NativeBuiltin),
    Partial {
        func: Box<PVal>,
        args: Vec<PVal>,
    },
    Closure {
        arity: usize,
        env: Vec<PVal>,
        body: Arc<PExpr>,
    },
    /// Stuck symbolic type or index term (e.g. `Ty::Param("xs")`, `Ty::Idx(ITerm::Var("n"))`).
    Neutral(Ty),
}

impl PVal {
    /// Convert a promoted `Ty` into a `PVal` for evaluation.
    pub fn from_ty(ty: &Ty) -> Self {
        match ty {
            Ty::Unit => PVal::Unit,
            Ty::Idx(it) => match it.as_ref() {
                ITerm::Num(s) => {
                    if let Some(rest) = s.as_str().strip_prefix('-') {
                        if let Ok(v) = rest.parse::<i128>() {
                            return PVal::Int(-v);
                        }
                    } else if let Ok(v) = s.as_str().parse::<u128>() {
                        return PVal::Nat(v);
                    }
                    PVal::Neutral(ty.clone())
                }
                _ => PVal::Neutral(ty.clone()),
            },
            Ty::Tuple(items) => PVal::Tuple(items.iter().map(PVal::from_ty).collect()),
            Ty::Adt(id, args) => {
                let s = id.as_str();
                if s == PROMOTED_TRUE && args.is_empty() {
                    PVal::Bool(true)
                } else if s == PROMOTED_FALSE && args.is_empty() {
                    PVal::Bool(false)
                } else if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 && args.is_empty() {
                    PVal::Str(s[1..s.len() - 1].to_string())
                } else if s.starts_with("ctor:") {
                    PVal::Ctor {
                        identity: s.to_string(),
                        args: args.iter().map(PVal::from_ty).collect(),
                    }
                } else if let Some(fn_id) = s.strip_prefix(PROMOTED_FNREF_PREFIX) {
                    if args.is_empty() {
                        PVal::FnRef(fn_id.to_string())
                    } else {
                        PVal::Partial {
                            func: Box::new(PVal::FnRef(fn_id.to_string())),
                            args: args.iter().map(PVal::from_ty).collect(),
                        }
                    }
                } else if let Some(fn_id) = s.strip_prefix(PROMOTED_FN_PREFIX) {
                    PVal::Partial {
                        func: Box::new(PVal::FnRef(fn_id.to_string())),
                        args: args.iter().map(PVal::from_ty).collect(),
                    }
                } else {
                    PVal::Neutral(ty.clone())
                }
            }
            _ => PVal::Neutral(ty.clone()),
        }
    }

    /// Convert an evaluated `PVal` back into a `Ty`.
    pub fn to_ty(&self) -> Option<Ty> {
        match self {
            PVal::Nat(n) => Some(Ty::Idx(Box::new(ITerm::Num(n.to_string().into())))),
            PVal::Int(n) => Some(Ty::Idx(Box::new(ITerm::Num(n.to_string().into())))),
            PVal::Bool(true) => Some(Ty::Adt(PROMOTED_TRUE.into(), Vec::new())),
            PVal::Bool(false) => Some(Ty::Adt(PROMOTED_FALSE.into(), Vec::new())),
            PVal::Str(s) => Some(Ty::Adt(format!("\"{s}\"").into(), Vec::new())),
            PVal::Unit => Some(Ty::Unit),
            PVal::Ctor { identity, args } => {
                let mut out = Vec::with_capacity(args.len());
                for a in args {
                    out.push(a.to_ty()?);
                }
                Some(Ty::Adt(identity.clone().into(), out))
            }
            PVal::Tuple(items) => {
                let mut out = Vec::with_capacity(items.len());
                for a in items {
                    out.push(a.to_ty()?);
                }
                Some(Ty::Tuple(out))
            }
            PVal::FnRef(id) => Some(Ty::Adt(format!("{PROMOTED_FNREF_PREFIX}{id}").into(), Vec::new())),
            PVal::Partial { func, args } => {
                if let PVal::FnRef(id) = func.as_ref() {
                    let mut out = Vec::with_capacity(args.len());
                    for a in args {
                        out.push(a.to_ty()?);
                    }
                    Some(Ty::Adt(format!("{PROMOTED_FN_PREFIX}{id}").into(), out))
                } else {
                    None
                }
            }
            PVal::Neutral(ty) => Some(ty.clone()),
            PVal::Native(_) | PVal::Closure { .. } => None,
        }
    }

    /// Convert an evaluated `PVal` into an `ITerm` (for index-returning calls).
    pub fn to_iterm(&self) -> Option<ITerm> {
        match self {
            PVal::Nat(n) => Some(ITerm::Num(n.to_string().into())),
            PVal::Int(n) => Some(ITerm::Num(n.to_string().into())),
            PVal::Neutral(Ty::Idx(it)) => Some((**it).clone()),
            PVal::Neutral(Ty::Param(p)) => Some(ITerm::Var(p.clone())),
            PVal::Partial { func, args } => {
                if let PVal::FnRef(id) = func.as_ref() {
                    let mut out = Vec::with_capacity(args.len());
                    for a in args {
                        out.push(a.to_ty()?);
                    }
                    Some(ITerm::TyFamily(format!("{PROMOTED_FN_PREFIX}{id}").into(), out))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// True if this value contains no `Neutral` subterms.
    pub fn is_concrete(&self) -> bool {
        match self {
            PVal::Nat(_) | PVal::Int(_) | PVal::Bool(_) | PVal::Str(_) | PVal::Unit => true,
            PVal::Ctor { args, .. } | PVal::Tuple(args) => args.iter().all(PVal::is_concrete),
            _ => false,
        }
    }

    fn as_list(&self) -> Option<(Vec<PVal>, Option<PVal>)> {
        let mut items = Vec::new();
        let mut cur = self;
        loop {
            match cur {
                PVal::Ctor { identity, args } if identity == PROMOTED_NIL && args.is_empty() => {
                    return Some((items, None));
                }
                PVal::Ctor { identity, args } if identity == PROMOTED_CONS && args.len() == 2 => {
                    items.push(args[0].clone());
                    cur = &args[1];
                }
                PVal::Neutral(_) => {
                    return Some((items, Some(cur.clone())));
                }
                _ => return None,
            }
        }
    }

    fn from_list(items: Vec<PVal>, tail: Option<PVal>) -> PVal {
        let mut acc = tail.unwrap_or_else(|| PVal::Ctor {
            identity: PROMOTED_NIL.to_string(),
            args: Vec::new(),
        });
        for item in items.into_iter().rev() {
            acc = PVal::Ctor {
                identity: PROMOTED_CONS.to_string(),
                args: vec![item, acc],
            };
        }
        acc
    }
}

/// Environment holding all registered promoted functions (cheap `Arc` clone).
#[derive(Clone, Debug)]
pub struct PromotedEnv {
    fns: Arc<HashMap<String, PromotedFn>>,
}

impl Default for PromotedEnv {
    fn default() -> Self {
        Self::with_builtins()
    }
}

impl PromotedEnv {
    /// Create a promoted environment pre-populated with native fast-path
    /// implementations for core prelude functions (`Data.List`, `Data.Bool`,
    /// `Data.Tuple`, `Data.String`, `Data.Nat`).
    pub fn with_builtins() -> Self {
        let mut map = HashMap::new();
        let list_ty = |elem: Ty| Ty::Adt(crate::known::LIST.into(), vec![elem]);
        let str_ty = Ty::Adt(crate::known::STRING.into(), Vec::new());
        let a = Ty::Param("a".into());
        let b = Ty::Param("b".into());
        let c = Ty::Param("c".into());

        let mut add = |id: &str,
                       type_params: Vec<&str>,
                       param_tys: Vec<Ty>,
                       ret_ty: Ty,
                       returns_idx: bool,
                       builtin: NativeBuiltin| {
            map.insert(
                id.to_string(),
                PromotedFn {
                    identity: id.to_string(),
                    type_params: type_params.into_iter().map(String::from).collect(),
                    param_tys,
                    ret_ty,
                    returns_idx,
                    body: PromotedBody::Native(builtin),
                },
            );
        };

        add("Data.Bool.not", vec![], vec![Ty::Bool], Ty::Bool, false, NativeBuiltin::Not);
        add("Data.Tuple.fst", vec!["a", "b"], vec![Ty::Tuple(vec![a.clone(), b.clone()])], a.clone(), false, NativeBuiltin::Fst);
        add("Data.Tuple.snd", vec!["a", "b"], vec![Ty::Tuple(vec![a.clone(), b.clone()])], b.clone(), false, NativeBuiltin::Snd);

        add(
            "Data.List.append",
            vec!["a"],
            vec![list_ty(a.clone()), list_ty(a.clone())],
            list_ty(a.clone()),
            false,
            NativeBuiltin::ListAppend,
        );
        add(
            "Data.List.reverse",
            vec!["a"],
            vec![list_ty(a.clone())],
            list_ty(a.clone()),
            false,
            NativeBuiltin::ListReverse,
        );
        add(
            "Data.List.length",
            vec!["a"],
            vec![list_ty(a.clone())],
            Ty::Nat,
            true,
            NativeBuiltin::ListLength,
        );
        add(
            "Data.List.list-length",
            vec!["a"],
            vec![list_ty(a.clone())],
            Ty::Nat,
            true,
            NativeBuiltin::ListLength,
        );
        add(
            "Category.Foldable.length",
            vec!["a"],
            vec![list_ty(a.clone())],
            Ty::Nat,
            true,
            NativeBuiltin::ListLength,
        );
        add(
            "Data.List.map",
            vec!["a", "b"],
            vec![Ty::fun(a.clone(), b.clone()), list_ty(a.clone())],
            list_ty(b.clone()),
            false,
            NativeBuiltin::ListMap,
        );
        add(
            "Category.Functor.map",
            vec!["a", "b"],
            vec![Ty::fun(a.clone(), b.clone()), list_ty(a.clone())],
            list_ty(b.clone()),
            false,
            NativeBuiltin::ListMap,
        );
        add(
            "Data.List.foldl",
            vec!["a", "b"],
            vec![
                Ty::fun(b.clone(), Ty::fun(a.clone(), b.clone())),
                b.clone(),
                list_ty(a.clone()),
            ],
            b.clone(),
            false,
            NativeBuiltin::ListFoldl,
        );
        add(
            "Data.List.foldr",
            vec!["a", "b"],
            vec![
                Ty::fun(a.clone(), Ty::fun(b.clone(), b.clone())),
                b.clone(),
                list_ty(a.clone()),
            ],
            b.clone(),
            false,
            NativeBuiltin::ListFoldr,
        );
        add(
            "Data.List.filter",
            vec!["a"],
            vec![Ty::fun(a.clone(), Ty::Bool), list_ty(a.clone())],
            list_ty(a.clone()),
            false,
            NativeBuiltin::ListFilter,
        );
        add(
            "Data.List.zip-with",
            vec!["a", "b", "c"],
            vec![
                Ty::fun(a.clone(), Ty::fun(b.clone(), c.clone())),
                list_ty(a.clone()),
                list_ty(b.clone()),
            ],
            list_ty(c.clone()),
            false,
            NativeBuiltin::ListZipWith,
        );
        add(
            "Data.List.take",
            vec!["a"],
            vec![Ty::Nat, list_ty(a.clone())],
            list_ty(a.clone()),
            false,
            NativeBuiltin::ListTake,
        );
        add(
            "Data.List.drop",
            vec!["a"],
            vec![Ty::Nat, list_ty(a.clone())],
            list_ty(a.clone()),
            false,
            NativeBuiltin::ListDrop,
        );
        add(
            "Data.List.replicate",
            vec!["a"],
            vec![Ty::Nat, a.clone()],
            list_ty(a.clone()),
            false,
            NativeBuiltin::ListReplicate,
        );
        add(
            "Data.List.null",
            vec!["a"],
            vec![list_ty(a.clone())],
            Ty::Bool,
            false,
            NativeBuiltin::ListNull,
        );
        add(
            "Data.List.sum",
            vec![],
            vec![list_ty(Ty::Nat)],
            Ty::Nat,
            true,
            NativeBuiltin::ListSum,
        );
        add(
            "Data.List.product",
            vec![],
            vec![list_ty(Ty::Nat)],
            Ty::Nat,
            true,
            NativeBuiltin::ListProduct,
        );
        add(
            "Data.String.string-concat",
            vec![],
            vec![str_ty.clone(), str_ty.clone()],
            str_ty.clone(),
            false,
            NativeBuiltin::StrConcat,
        );
        add(
            "Data.Nat.nat-min",
            vec![],
            vec![Ty::Nat, Ty::Nat],
            Ty::Nat,
            true,
            NativeBuiltin::NatMin,
        );
        add(
            "Data.Nat.nat-max",
            vec![],
            vec![Ty::Nat, Ty::Nat],
            Ty::Nat,
            true,
            NativeBuiltin::NatMax,
        );

        Self { fns: Arc::new(map) }
    }

    pub fn register(&mut self, func: PromotedFn) {
        Arc::make_mut(&mut self.fns).insert(func.identity.clone(), func);
    }

    pub fn get(&self, identity: &str) -> Option<&PromotedFn> {
        self.fns.get(identity)
    }

    pub fn contains(&self, identity: &str) -> bool {
        self.fns.contains_key(identity)
    }

    pub fn is_empty(&self) -> bool {
        self.fns.is_empty()
    }

    /// Evaluate a promoted function call `fn_id(args)` to a `Ty`, returning
    /// `None` if evaluation is stuck on a neutral variable or exhausts fuel.
    pub fn eval_to_ty(&self, fn_id: &str, args: &[Ty]) -> Option<Ty> {
        let pvals: Vec<PVal> = args.iter().map(PVal::from_ty).collect();
        self.eval_pvals_to_ty(fn_id, pvals)
    }

    /// Evaluate a promoted function call on already-constructed `PVal`s
    /// (allowing first-class type-level lambdas without mutating `PromotedEnv`).
    pub fn eval_pvals_to_ty(&self, fn_id: &str, pvals: Vec<PVal>) -> Option<Ty> {
        let mut fuel = PROMOTE_EVAL_FUEL;
        let res = self.apply_fn(fn_id, pvals, &mut fuel, 0)?;
        res.to_ty()
    }

    /// Evaluate a promoted function call `fn_id(args)` to an `ITerm`, returning
    /// `None` if evaluation is stuck or exhausts fuel.
    pub fn eval_to_iterm(&self, fn_id: &str, args: &[Ty]) -> Option<ITerm> {
        let pvals: Vec<PVal> = args.iter().map(PVal::from_ty).collect();
        self.eval_pvals_to_iterm(fn_id, pvals)
    }

    /// Evaluate a promoted function call on `PVal`s to an `ITerm`.
    pub fn eval_pvals_to_iterm(&self, fn_id: &str, pvals: Vec<PVal>) -> Option<ITerm> {
        let mut fuel = PROMOTE_EVAL_FUEL;
        let res = self.apply_fn(fn_id, pvals, &mut fuel, 0)?;
        res.to_iterm()
    }

    fn apply_fn(&self, fn_id: &str, args: Vec<PVal>, fuel: &mut usize, depth: usize) -> Option<PVal> {
        let def = self.fns.get(fn_id)?;
        let arity = def.arity();
        if args.len() < arity {
            return Some(PVal::Partial {
                func: Box::new(PVal::FnRef(fn_id.to_string())),
                args,
            });
        }
        if *fuel == 0 || depth >= MAX_PROMOTE_CALL_DEPTH {
            return None;
        }
        *fuel -= 1;

        let (direct_args, extra_args) = if args.len() == arity {
            (args, Vec::new())
        } else {
            let mut a = args;
            let rest = a.split_off(arity);
            (a, rest)
        };

        let result = match &def.body {
            PromotedBody::Expr(expr) => {
                let mut stack = direct_args;
                self.eval_expr(expr, &mut stack, fuel, depth + 1)?
            }
            PromotedBody::Native(builtin) => {
                self.eval_native(*builtin, direct_args, fuel, depth + 1)?
            }
        };

        if extra_args.is_empty() {
            Some(result)
        } else {
            self.apply_val(result, extra_args, fuel, depth + 1)
        }
    }

    fn apply_val(&self, func: PVal, args: Vec<PVal>, fuel: &mut usize, depth: usize) -> Option<PVal> {
        if args.is_empty() {
            return Some(func);
        }
        if depth >= MAX_PROMOTE_CALL_DEPTH {
            return None;
        }
        match func {
            PVal::FnRef(id) => self.apply_fn(&id, args, fuel, depth + 1),
            PVal::Native(b) => {
                let arity = native_arity(b);
                if args.len() < arity {
                    Some(PVal::Partial {
                        func: Box::new(PVal::Native(b)),
                        args,
                    })
                } else {
                    let mut a = args;
                    let rest = a.split_off(arity);
                    let res = self.eval_native(b, a, fuel, depth + 1)?;
                    if rest.is_empty() {
                        Some(res)
                    } else {
                        self.apply_val(res, rest, fuel, depth + 1)
                    }
                }
            }
            PVal::Partial { func: inner, args: mut prev } => {
                prev.extend(args);
                self.apply_val(*inner, prev, fuel, depth + 1)
            }
            PVal::Closure { arity, mut env, body } => {
                if args.len() < arity {
                    return Some(PVal::Partial {
                        func: Box::new(PVal::Closure { arity, env, body }),
                        args,
                    });
                }
                if *fuel == 0 {
                    return None;
                }
                *fuel -= 1;
                let mut a = args;
                let rest = a.split_off(arity);
                env.extend(a);
                let res = self.eval_expr(&body, &mut env, fuel, depth + 1)?;
                if rest.is_empty() {
                    Some(res)
                } else {
                    self.apply_val(res, rest, fuel, depth + 1)
                }
            }
            PVal::Neutral(_) => None,
            _ => None,
        }
    }

    fn eval_expr(&self, expr: &PExpr, stack: &mut Vec<PVal>, fuel: &mut usize, depth: usize) -> Option<PVal> {
        if *fuel == 0 || depth >= MAX_PROMOTE_CALL_DEPTH {
            return None;
        }
        *fuel -= 1;
        match expr {
            PExpr::Var(slot) => stack.get(*slot).cloned(),
            PExpr::Nat(n) => Some(PVal::Nat(*n)),
            PExpr::Int(n) => Some(PVal::Int(*n)),
            PExpr::Bool(b) => Some(PVal::Bool(*b)),
            PExpr::Str(s) => Some(PVal::Str(s.clone())),
            PExpr::Unit => Some(PVal::Unit),
            PExpr::FnRef(id) => Some(PVal::FnRef(id.clone())),
            PExpr::Native(b) => Some(PVal::Native(*b)),
            PExpr::Lam { arity, body } => Some(PVal::Closure {
                arity: *arity,
                env: stack.clone(),
                body: Arc::clone(body),
            }),
            PExpr::Ctor { identity, args } => {
                let mut vals = Vec::with_capacity(args.len());
                for a in args {
                    vals.push(self.eval_expr(a, stack, fuel, depth)?);
                }
                Some(PVal::Ctor {
                    identity: identity.clone(),
                    args: vals,
                })
            }
            PExpr::Tuple(items) => {
                let mut vals = Vec::with_capacity(items.len());
                for a in items {
                    vals.push(self.eval_expr(a, stack, fuel, depth)?);
                }
                Some(PVal::Tuple(vals))
            }
            PExpr::App { func, args } => {
                let fval = self.eval_expr(func, stack, fuel, depth)?;
                let mut avals = Vec::with_capacity(args.len());
                for a in args {
                    avals.push(self.eval_expr(a, stack, fuel, depth)?);
                }
                if let Some(res) = self.apply_val(fval.clone(), avals.clone(), fuel, depth) {
                    return Some(res);
                }
                // Preserve stuck call as a symbolic neutral term so partial
                // evaluation of outer constructors (`Cons h (append t ys)`) still completes!
                if let PVal::FnRef(id) = fval {
                    let mut tys = Vec::with_capacity(avals.len());
                    for a in &avals {
                        tys.push(a.to_ty()?);
                    }
                    let returns_idx = self.fns.get(&id).is_some_and(|d| d.returns_idx);
                    if returns_idx {
                        return Some(PVal::Neutral(Ty::Idx(Box::new(ITerm::TyFamily(
                            format!("{PROMOTED_FN_PREFIX}{id}").into(),
                            tys,
                        )))));
                    }
                    return Some(PVal::Neutral(Ty::Adt(
                        format!("{PROMOTED_FN_PREFIX}{id}").into(),
                        tys,
                    )));
                }
                None
            }
            PExpr::BinOp { op, lhs, rhs } => {
                let l = self.eval_expr(lhs, stack, fuel, depth)?;
                // Short-circuit booleans before evaluating rhs!
                match (op, &l) {
                    (PBinOp::And, PVal::Bool(false)) => return Some(PVal::Bool(false)),
                    (PBinOp::Or, PVal::Bool(true)) => return Some(PVal::Bool(true)),
                    _ => {}
                }
                let r = self.eval_expr(rhs, stack, fuel, depth)?;
                eval_binop(*op, l, r)
            }
            PExpr::UnaryOp { op, arg } => {
                let v = self.eval_expr(arg, stack, fuel, depth)?;
                match (op, v) {
                    (PUnaryOp::Not, PVal::Bool(b)) => Some(PVal::Bool(!b)),
                    (PUnaryOp::Neg, PVal::Int(n)) => n.checked_neg().map(PVal::Int),
                    (PUnaryOp::Neg, PVal::Nat(n)) => i128::try_from(n).ok().and_then(|x| x.checked_neg()).map(PVal::Int),
                    _ => None,
                }
            }
            PExpr::If { cond, then_br, else_br } => match self.eval_expr(cond, stack, fuel, depth)? {
                PVal::Bool(true) => self.eval_expr(then_br, stack, fuel, depth),
                PVal::Bool(false) => self.eval_expr(else_br, stack, fuel, depth),
                _ => None,
            },
            PExpr::Let { value, body } => {
                let v = self.eval_expr(value, stack, fuel, depth)?;
                stack.push(v);
                let res = self.eval_expr(body, stack, fuel, depth);
                stack.pop();
                res
            }
            PExpr::Case { scrut, arms } => {
                let sv = self.eval_expr(scrut, stack, fuel, depth)?;
                for arm in arms {
                    let saved = stack.len();
                    match match_pat(&arm.pat, &sv, stack) {
                        PatMatch::Matched => {
                            if let Some(g) = &arm.guard {
                                match self.eval_expr(g, stack, fuel, depth) {
                                    Some(PVal::Bool(true)) => {}
                                    Some(PVal::Bool(false)) => {
                                        stack.truncate(saved);
                                        continue;
                                    }
                                    _ => {
                                        stack.truncate(saved);
                                        return None;
                                    }
                                }
                            }
                            let res = self.eval_expr(&arm.body, stack, fuel, depth);
                            stack.truncate(saved);
                            return res;
                        }
                        PatMatch::Mismatch => {
                            stack.truncate(saved);
                        }
                        PatMatch::Stuck => {
                            stack.truncate(saved);
                            return None;
                        }
                    }
                }
                None
            }
        }
    }

    fn eval_native(&self, builtin: NativeBuiltin, args: Vec<PVal>, fuel: &mut usize, depth: usize) -> Option<PVal> {
        match builtin {
            NativeBuiltin::BinOp(op) => {
                let [l, r]: [PVal; 2] = args.try_into().ok()?;
                eval_binop(op, l, r)
            }
            NativeBuiltin::Not => match args.as_slice() {
                [PVal::Bool(b)] => Some(PVal::Bool(!b)),
                _ => None,
            },
            NativeBuiltin::Fst => match args.into_iter().next()? {
                PVal::Tuple(mut items) if items.len() == 2 => Some(items.swap_remove(0)),
                _ => None,
            },
            NativeBuiltin::Snd => match args.into_iter().next()? {
                PVal::Tuple(mut items) if items.len() == 2 => Some(items.swap_remove(1)),
                _ => None,
            },
            NativeBuiltin::ListAppend => {
                let [xs, ys]: [PVal; 2] = args.try_into().ok()?;
                let (mut items, tail) = xs.as_list()?;
                match tail {
                    None => Some(PVal::from_list(items, Some(ys))),
                    Some(t) if items.is_empty() => {
                        let _ = t;
                        None
                    }
                    Some(t) => {
                        // Partially reduce known prefix: `[a, b | t] ++ ys -> [a, b | append t ys]`
                        let stuck = PVal::Neutral(Ty::Adt(
                            format!("{PROMOTED_FN_PREFIX}Data.List.append").into(),
                            vec![t.to_ty()?, ys.to_ty()?],
                        ));
                        items.shrink_to_fit();
                        Some(PVal::from_list(items, Some(stuck)))
                    }
                }
            }
            NativeBuiltin::ListReverse => {
                let [xs]: [PVal; 1] = args.try_into().ok()?;
                let (mut items, tail) = xs.as_list()?;
                if tail.is_some() {
                    return None;
                }
                items.reverse();
                Some(PVal::from_list(items, None))
            }
            NativeBuiltin::ListLength => {
                let [xs]: [PVal; 1] = args.try_into().ok()?;
                let (items, tail) = xs.as_list()?;
                if tail.is_some() {
                    if items.is_empty() {
                        return None;
                    }
                    let rest_ty = tail?.to_ty()?;
                    let rest_len = ITerm::TyFamily(
                        format!("{PROMOTED_FN_PREFIX}Data.List.length").into(),
                        vec![rest_ty],
                    );
                    let sum = ITerm::Bin(
                        IdxOp::Add,
                        Box::new(ITerm::Num(items.len().to_string().into())),
                        Box::new(rest_len),
                    )
                    .normalize();
                    return Some(PVal::Neutral(Ty::Idx(Box::new(sum))));
                }
                Some(PVal::Nat(items.len() as u128))
            }
            NativeBuiltin::ListMap => {
                let [f, xs]: [PVal; 2] = args.try_into().ok()?;
                let (items, tail) = xs.as_list()?;
                if tail.is_some() && items.is_empty() {
                    return None;
                }
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(self.apply_val(f.clone(), vec![item], fuel, depth)?);
                }
                let out_tail = match tail {
                    None => None,
                    Some(t) => Some(PVal::Neutral(Ty::Adt(
                        format!("{PROMOTED_FN_PREFIX}Data.List.map").into(),
                        vec![f.to_ty()?, t.to_ty()?],
                    ))),
                };
                Some(PVal::from_list(out, out_tail))
            }
            NativeBuiltin::ListFoldl => {
                let [f, mut acc, xs]: [PVal; 3] = args.try_into().ok()?;
                let (items, tail) = xs.as_list()?;
                if tail.is_some() {
                    return None;
                }
                for item in items {
                    acc = self.apply_val(f.clone(), vec![acc, item], fuel, depth)?;
                }
                Some(acc)
            }
            NativeBuiltin::ListFoldr => {
                let [f, mut acc, xs]: [PVal; 3] = args.try_into().ok()?;
                let (items, tail) = xs.as_list()?;
                if tail.is_some() {
                    return None;
                }
                for item in items.into_iter().rev() {
                    acc = self.apply_val(f.clone(), vec![item, acc], fuel, depth)?;
                }
                Some(acc)
            }
            NativeBuiltin::ListFilter => {
                let [pred, xs]: [PVal; 2] = args.try_into().ok()?;
                let (items, tail) = xs.as_list()?;
                if tail.is_some() {
                    return None;
                }
                let mut out = Vec::new();
                for item in items {
                    match self.apply_val(pred.clone(), vec![item.clone()], fuel, depth)? {
                        PVal::Bool(true) => out.push(item),
                        PVal::Bool(false) => {}
                        _ => return None,
                    }
                }
                Some(PVal::from_list(out, None))
            }
            NativeBuiltin::ListZipWith => {
                let [f, xs, ys]: [PVal; 3] = args.try_into().ok()?;
                let (xs_items, xs_tail) = xs.as_list()?;
                let (ys_items, ys_tail) = ys.as_list()?;
                if xs_tail.is_some() || ys_tail.is_some() {
                    return None;
                }
                let mut out = Vec::with_capacity(xs_items.len().min(ys_items.len()));
                for (x, y) in xs_items.into_iter().zip(ys_items.into_iter()) {
                    out.push(self.apply_val(f.clone(), vec![x, y], fuel, depth)?);
                }
                Some(PVal::from_list(out, None))
            }
            NativeBuiltin::ListTake => {
                let [n_val, xs]: [PVal; 2] = args.try_into().ok()?;
                let PVal::Nat(n) = n_val else { return None };
                let (items, tail) = xs.as_list()?;
                let take_n = usize::try_from(n).ok()?;
                if tail.is_some() && items.len() < take_n {
                    return None;
                }
                Some(PVal::from_list(items.into_iter().take(take_n).collect(), None))
            }
            NativeBuiltin::ListDrop => {
                let [n_val, xs]: [PVal; 2] = args.try_into().ok()?;
                let PVal::Nat(n) = n_val else { return None };
                let (items, tail) = xs.as_list()?;
                let drop_n = usize::try_from(n).ok()?;
                if tail.is_some() && items.len() < drop_n {
                    return None;
                }
                Some(PVal::from_list(items.into_iter().skip(drop_n).collect(), tail))
            }
            NativeBuiltin::ListReplicate => {
                let [n_val, elem]: [PVal; 2] = args.try_into().ok()?;
                let PVal::Nat(n) = n_val else { return None };
                let count = usize::try_from(n).ok()?;
                if count > 4096 {
                    return None;
                }
                Some(PVal::from_list(vec![elem; count], None))
            }
            NativeBuiltin::ListNull => {
                let [xs]: [PVal; 1] = args.try_into().ok()?;
                let (items, tail) = xs.as_list()?;
                if !items.is_empty() {
                    Some(PVal::Bool(false))
                } else if tail.is_none() {
                    Some(PVal::Bool(true))
                } else {
                    None
                }
            }
            NativeBuiltin::ListSum => {
                let [xs]: [PVal; 1] = args.try_into().ok()?;
                let (items, tail) = xs.as_list()?;
                if tail.is_some() {
                    return None;
                }
                let mut acc = PVal::Nat(0);
                for item in items {
                    acc = eval_binop(PBinOp::Add, acc, item)?;
                }
                Some(acc)
            }
            NativeBuiltin::ListProduct => {
                let [xs]: [PVal; 1] = args.try_into().ok()?;
                let (items, tail) = xs.as_list()?;
                if tail.is_some() {
                    return None;
                }
                let mut acc = PVal::Nat(1);
                for item in items {
                    acc = eval_binop(PBinOp::Mul, acc, item)?;
                }
                Some(acc)
            }
            NativeBuiltin::StrConcat => {
                let [a, b]: [PVal; 2] = args.try_into().ok()?;
                eval_binop(PBinOp::StrConcat, a, b)
            }
            NativeBuiltin::NatMin => match args.as_slice() {
                [PVal::Nat(a), PVal::Nat(b)] => Some(PVal::Nat((*a).min(*b))),
                _ => None,
            },
            NativeBuiltin::NatMax => match args.as_slice() {
                [PVal::Nat(a), PVal::Nat(b)] => Some(PVal::Nat((*a).max(*b))),
                _ => None,
            },
        }
    }
}

fn native_arity(b: NativeBuiltin) -> usize {
    match b {
        NativeBuiltin::Not
        | NativeBuiltin::Fst
        | NativeBuiltin::Snd
        | NativeBuiltin::ListReverse
        | NativeBuiltin::ListLength
        | NativeBuiltin::ListNull
        | NativeBuiltin::ListSum
        | NativeBuiltin::ListProduct => 1,
        NativeBuiltin::BinOp(_)
        | NativeBuiltin::ListAppend
        | NativeBuiltin::ListMap
        | NativeBuiltin::ListFilter
        | NativeBuiltin::ListTake
        | NativeBuiltin::ListDrop
        | NativeBuiltin::ListReplicate
        | NativeBuiltin::StrConcat
        | NativeBuiltin::NatMin
        | NativeBuiltin::NatMax => 2,
        NativeBuiltin::ListFoldl | NativeBuiltin::ListFoldr | NativeBuiltin::ListZipWith => 3,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PatMatch {
    Matched,
    Mismatch,
    Stuck,
}

fn match_pat(pat: &PPat, val: &PVal, stack: &mut Vec<PVal>) -> PatMatch {
    match pat {
        PPat::Wild => PatMatch::Matched,
        PPat::Bind => {
            stack.push(val.clone());
            PatMatch::Matched
        }
        PPat::As(inner) => {
            stack.push(val.clone());
            match_pat(inner, val, stack)
        }
        PPat::Unit => match val {
            PVal::Unit => PatMatch::Matched,
            PVal::Neutral(_) => PatMatch::Stuck,
            _ => PatMatch::Mismatch,
        },
        PPat::Bool(expected) => match val {
            PVal::Bool(actual) => {
                if actual == expected {
                    PatMatch::Matched
                } else {
                    PatMatch::Mismatch
                }
            }
            PVal::Neutral(_) => PatMatch::Stuck,
            _ => PatMatch::Mismatch,
        },
        PPat::Nat(expected) => match val {
            PVal::Nat(actual) => {
                if actual == expected {
                    PatMatch::Matched
                } else {
                    PatMatch::Mismatch
                }
            }
            PVal::Int(actual) => {
                if *actual >= 0 && (*actual as u128) == *expected {
                    PatMatch::Matched
                } else {
                    PatMatch::Mismatch
                }
            }
            PVal::Neutral(_) => PatMatch::Stuck,
            _ => PatMatch::Mismatch,
        },
        PPat::Int(expected) => match val {
            PVal::Int(actual) => {
                if actual == expected {
                    PatMatch::Matched
                } else {
                    PatMatch::Mismatch
                }
            }
            PVal::Nat(actual) => {
                if *expected >= 0 && (*expected as u128) == *actual {
                    PatMatch::Matched
                } else {
                    PatMatch::Mismatch
                }
            }
            PVal::Neutral(_) => PatMatch::Stuck,
            _ => PatMatch::Mismatch,
        },
        PPat::Str(expected) => match val {
            PVal::Str(actual) => {
                if actual == expected {
                    PatMatch::Matched
                } else {
                    PatMatch::Mismatch
                }
            }
            PVal::Neutral(_) => PatMatch::Stuck,
            _ => PatMatch::Mismatch,
        },
        PPat::Ctor { identity, args } => match val {
            PVal::Ctor { identity: actual, args: vargs } => {
                if actual != identity || vargs.len() != args.len() {
                    return PatMatch::Mismatch;
                }
                for (p, v) in args.iter().zip(vargs.iter()) {
                    match match_pat(p, v, stack) {
                        PatMatch::Matched => {}
                        other => return other,
                    }
                }
                PatMatch::Matched
            }
            PVal::Neutral(_) => PatMatch::Stuck,
            _ => PatMatch::Mismatch,
        },
        PPat::Tuple(pats) => match val {
            PVal::Tuple(vals) => {
                if vals.len() != pats.len() {
                    return PatMatch::Mismatch;
                }
                for (p, v) in pats.iter().zip(vals.iter()) {
                    match match_pat(p, v, stack) {
                        PatMatch::Matched => {}
                        other => return other,
                    }
                }
                PatMatch::Matched
            }
            PVal::Neutral(_) => PatMatch::Stuck,
            _ => PatMatch::Mismatch,
        },
    }
}

fn eval_binop(op: PBinOp, lhs: PVal, rhs: PVal) -> Option<PVal> {
    match op {
        PBinOp::Add => match (lhs, rhs) {
            (PVal::Nat(a), PVal::Nat(b)) => a.checked_add(b).map(PVal::Nat),
            (PVal::Int(a), PVal::Int(b)) => a.checked_add(b).map(PVal::Int),
            (PVal::Nat(0), other) | (other, PVal::Nat(0)) => Some(other),
            (PVal::Int(0), other) | (other, PVal::Int(0)) => Some(other),
            (a, b) => {
                let it = ITerm::Bin(IdxOp::Add, Box::new(a.to_iterm()?), Box::new(b.to_iterm()?)).normalize();
                Some(PVal::Neutral(Ty::Idx(Box::new(it))))
            }
        },
        PBinOp::Sub => match (lhs, rhs) {
            (PVal::Nat(a), PVal::Nat(b)) => Some(PVal::Nat(a.saturating_sub(b))),
            (PVal::Int(a), PVal::Int(b)) => a.checked_sub(b).map(PVal::Int),
            (other, PVal::Nat(0)) | (other, PVal::Int(0)) => Some(other),
            (a, b) => {
                let it = ITerm::Bin(IdxOp::Sub, Box::new(a.to_iterm()?), Box::new(b.to_iterm()?)).normalize();
                Some(PVal::Neutral(Ty::Idx(Box::new(it))))
            }
        },
        PBinOp::Mul => match (lhs, rhs) {
            (PVal::Nat(a), PVal::Nat(b)) => a.checked_mul(b).map(PVal::Nat),
            (PVal::Int(a), PVal::Int(b)) => a.checked_mul(b).map(PVal::Int),
            (PVal::Nat(0), _) | (_, PVal::Nat(0)) => Some(PVal::Nat(0)),
            (PVal::Int(0), _) | (_, PVal::Int(0)) => Some(PVal::Int(0)),
            (PVal::Nat(1), other) | (other, PVal::Nat(1)) => Some(other),
            (PVal::Int(1), other) | (other, PVal::Int(1)) => Some(other),
            (a, b) => {
                let it = ITerm::Bin(IdxOp::Mul, Box::new(a.to_iterm()?), Box::new(b.to_iterm()?)).normalize();
                Some(PVal::Neutral(Ty::Idx(Box::new(it))))
            }
        },
        PBinOp::Div => match (lhs, rhs) {
            (PVal::Nat(a), PVal::Nat(b)) if b != 0 => Some(PVal::Nat(a / b)),
            (PVal::Int(a), PVal::Int(b)) if b != 0 => a.checked_div(b).map(PVal::Int),
            _ => None,
        },
        PBinOp::Rem => match (lhs, rhs) {
            (PVal::Nat(a), PVal::Nat(b)) if b != 0 => Some(PVal::Nat(a % b)),
            (PVal::Int(a), PVal::Int(b)) if b != 0 => a.checked_rem(b).map(PVal::Int),
            _ => None,
        },
        PBinOp::Eq => {
            if lhs.is_concrete() && rhs.is_concrete() {
                Some(PVal::Bool(lhs == rhs))
            } else if lhs == rhs {
                Some(PVal::Bool(true))
            } else {
                None
            }
        }
        PBinOp::Ne => {
            if lhs.is_concrete() && rhs.is_concrete() {
                Some(PVal::Bool(lhs != rhs))
            } else if lhs == rhs {
                Some(PVal::Bool(false))
            } else {
                None
            }
        }
        PBinOp::Lt => match (lhs, rhs) {
            (PVal::Nat(a), PVal::Nat(b)) => Some(PVal::Bool(a < b)),
            (PVal::Int(a), PVal::Int(b)) => Some(PVal::Bool(a < b)),
            (PVal::Str(a), PVal::Str(b)) => Some(PVal::Bool(a < b)),
            _ => None,
        },
        PBinOp::Le => match (lhs, rhs) {
            (PVal::Nat(a), PVal::Nat(b)) => Some(PVal::Bool(a <= b)),
            (PVal::Int(a), PVal::Int(b)) => Some(PVal::Bool(a <= b)),
            (PVal::Str(a), PVal::Str(b)) => Some(PVal::Bool(a <= b)),
            _ => None,
        },
        PBinOp::Gt => match (lhs, rhs) {
            (PVal::Nat(a), PVal::Nat(b)) => Some(PVal::Bool(a > b)),
            (PVal::Int(a), PVal::Int(b)) => Some(PVal::Bool(a > b)),
            (PVal::Str(a), PVal::Str(b)) => Some(PVal::Bool(a > b)),
            _ => None,
        },
        PBinOp::Ge => match (lhs, rhs) {
            (PVal::Nat(a), PVal::Nat(b)) => Some(PVal::Bool(a >= b)),
            (PVal::Int(a), PVal::Int(b)) => Some(PVal::Bool(a >= b)),
            (PVal::Str(a), PVal::Str(b)) => Some(PVal::Bool(a >= b)),
            _ => None,
        },
        PBinOp::And => match (lhs, rhs) {
            (PVal::Bool(a), PVal::Bool(b)) => Some(PVal::Bool(a && b)),
            _ => None,
        },
        PBinOp::Or => match (lhs, rhs) {
            (PVal::Bool(a), PVal::Bool(b)) => Some(PVal::Bool(a || b)),
            _ => None,
        },
        PBinOp::StrConcat => match (lhs, rhs) {
            (PVal::Str(mut a), PVal::Str(b)) => {
                a.push_str(&b);
                Some(PVal::Str(a))
            }
            _ => None,
        },
        PBinOp::ListAppend => {
            let (items, tail) = lhs.as_list()?;
            if tail.is_some() {
                return None;
            }
            Some(PVal::from_list(items, Some(rhs)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_list_and_higher_order_evaluation() {
        let env = PromotedEnv::with_builtins();
        let list = PVal::from_list(vec![PVal::Nat(2), PVal::Nat(3), PVal::Nat(4)], None)
            .to_ty()
            .unwrap();
        // `Data.List.reverse [2, 3, 4]` -> `[4, 3, 2]`
        let rev = env.eval_to_ty("Data.List.reverse", &[list.clone()]).unwrap();
        let expected_rev = PVal::from_list(vec![PVal::Nat(4), PVal::Nat(3), PVal::Nat(2)], None)
            .to_ty()
            .unwrap();
        assert_eq!(rev, expected_rev);

        // `Data.List.product [2, 3, 4]` -> `24`
        let prod = env.eval_to_iterm("Data.List.product", &[list.clone()]).unwrap();
        assert_eq!(prod, ITerm::Num("24".into()));

        // Partial symbolic append: `Data.List.append [2, 3] rest` -> `Cons 2 (Cons 3 rest)`
        let prefix = PVal::from_list(vec![PVal::Nat(2), PVal::Nat(3)], None)
            .to_ty()
            .unwrap();
        let rest = Ty::Param("rest".into());
        let app = env
            .eval_to_ty("Data.List.append", &[prefix, rest.clone()])
            .unwrap();
        let expected_app = PVal::from_list(vec![PVal::Nat(2), PVal::Nat(3)], Some(PVal::Neutral(rest)))
            .to_ty()
            .unwrap();
        assert_eq!(app, expected_app);
    }

    #[test]
    fn user_recursive_and_higher_order_promoted_fn() {
        let mut env = PromotedEnv::with_builtins();
        // `double n = n * 2`
        env.register(PromotedFn {
            identity: "Main.double".into(),
            type_params: vec![],
            param_tys: vec![Ty::Nat],
            ret_ty: Ty::Nat,
            returns_idx: true,
            body: PromotedBody::Expr(Arc::new(PExpr::BinOp {
                op: PBinOp::Mul,
                lhs: Box::new(PExpr::Var(0)),
                rhs: Box::new(PExpr::Nat(2)),
            })),
        });
        let xs = PVal::from_list(vec![PVal::Nat(2), PVal::Nat(3), PVal::Nat(4)], None)
            .to_ty()
            .unwrap();
        let dbl_ref = Ty::Adt("fnref:Main.double".into(), vec![]);
        let mapped = env.eval_to_ty("Data.List.map", &[dbl_ref, xs]).unwrap();
        let expected = PVal::from_list(vec![PVal::Nat(4), PVal::Nat(6), PVal::Nat(8)], None)
            .to_ty()
            .unwrap();
        assert_eq!(mapped, expected);
    }
}

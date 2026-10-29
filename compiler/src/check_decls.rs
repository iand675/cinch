
// ------------------------------------------------------------ collection and programs

/// Expected type of a compiler primitive, used to validate `foreign import`
/// declarations of `cinch.prim.*` and `cinch.runtime.*` intrinsics.
fn prim_scheme(op: PrimOp) -> Scheme {
    let (args, res) = op.sig();
    Scheme::mono(Ty::funs(&args, res))
}

/// The positivity discipline (core specification section 2; roadmap gap
/// A1): the type being declared may occur in a constructor field only
/// strictly positively, as the field type itself or in the codomain of an
/// arrow. Negative occurrences and nesting inside other type constructors
/// are rejected: positivity-preserving nesting is an open design point
/// (dependent core kernel design, section 5).
fn check_positivity(
    family: &str,
    field: &str,
    ty: &Ty,
    ctor: &ast::Name,
    span: ast::Span,
) -> Result<()> {
    fn go(family: &str, field: &str, ctor: &ast::Name, ty: &Ty, span: ast::Span, positive: bool) -> Result<()> {
        let bad = |what: &str| {
            Err(Error::at(
                span,
                format!(
                    "{} occurs {} in field `{}` of constructor {}; the positivity discipline only allows direct recursive occurrences ({} is not supported in this slice)",
                    family,
                    what,
                    field,
                    ctor.text,
                    if what == "negatively" { "negative occurrences" } else { "positivity-preserving nesting" }
                ),
            ))
        };
        match ty {
            Ty::Adt(n, args) => {
                if n == family {
                    if !positive {
                        return bad("negatively");
                    }
                    for a in args {
                        if ty_mentions(a, family) {
                            return bad("nested inside its own arguments");
                        }
                    }
                    Ok(())
                } else {
                    for a in args {
                        if ty_mentions(a, family) {
                            return bad("nested inside another type constructor");
                        }
                    }
                    Ok(())
                }
            }
            // An arrow's parameter is a negative position; its result is
            // positive. A dependent pair is a product: both sides are
            // positive.
            Ty::Fun(a, _, _, b) | Ty::Pi { dom: a, cod: b, .. } => {
                go(family, field, ctor, a, span, !positive)?;
                go(family, field, ctor, b, span, positive)
            }
            Ty::Sigma { dom, cod, .. } => {
                go(family, field, ctor, dom, span, positive)?;
                go(family, field, ctor, cod, span, positive)
            }
            Ty::Tuple(items) => {
                for i in items {
                    go(family, field, ctor, i, span, positive)?;
                }
                Ok(())
            }
            Ty::Thunk { inner, .. } | Ty::Later(inner) | Ty::Lazy { inner, .. } => {
                // A thunk's body runs at force time: the same positive
                // reading as a product component. A `Later` payload is a
                // guarded future observation (core language specification
                // section 9), read the same way.
                go(family, field, ctor, inner, span, positive)
            }
            _ => Ok(()),
        }
    }
    go(family, field, ctor, ty, span, true)
}

/// Does this type mention the name (as a type constructor head, a type
/// parameter, or an index-level variable)?
fn ty_mentions(t: &Ty, name: &str) -> bool {
    if t.mentions_idx(name) {
        return true;
    }
    match t {
        Ty::Param(n) => n == name,
        Ty::Adt(n, args) => {
            n == name || args.iter().any(|a| ty_mentions(a, name))
        }
        Ty::Tuple(args) => args.iter().any(|a| ty_mentions(a, name)),
        Ty::Fun(a, _, _, b) | Ty::Pi { dom: a, cod: b, .. } => {
            ty_mentions(a, name) || ty_mentions(b, name)
        }
        Ty::Sigma { dom, cod, .. } => ty_mentions(dom, name) || ty_mentions(cod, name),
        Ty::Thunk { inner, .. } | Ty::Later(inner) | Ty::Lazy { inner, .. } => ty_mentions(inner, name),
        Ty::Ref(_, i) | Ty::Mut(_, i) => ty_mentions(i, name),
        Ty::Handler { input, output, .. } => {
            ty_mentions(input, name) || ty_mentions(output, name)
        }
        _ => false,
    }
}

/// An erased component occupies no runtime representation: propositions
/// are evidence and universes are types (core specification section 3).
fn erased_ty(t: &Ty) -> bool {
    matches!(t, Ty::Prp(_) | Ty::Sort(_))
}

/// Builtin type names: machine scalars, the compile-time-only staging
/// types, the boundary pointer roles (foreign function interface
/// specification section 3), the C ABI spellings (section 2), and the
/// schema introspection types (modules specification section 11). A
/// source declaration cannot redefine them.
fn builtin_type_name(name: &str) -> bool {
    if Scalar::parse(name).is_some() {
        return true;
    }
    matches!(
        name,
        "Nat" | "Int" | "Bool" | "Unit" | "Bytes" | "Sym" | "Type" | "Prop"
            | "Code" | "Syntax" | "Declarations" | "Name" | "Secret"
            | "IoCap" | "FileReadCap" | "FileWriteCap" | "ProcCap"
            | "Scope" | "Task"
            | "RawPtr" | "Borrowed" | "Retained" | "Owned" | "Out" | "MaybeNull"
            | "CChar" | "CUchar" | "CShort" | "CUshort" | "CInt" | "CUint"
            | "CLong" | "CULong" | "CLongLong" | "CULongLong" | "CSize" | "SizeT"
            | "CSsize" | "CFloat" | "CDouble" | "CVoid"
            | "SchemaSnapshot" | "Database" | "SchemaRead"
    )
}

/// The effect labels a foreign binding's row must account for (foreign
/// function interface specification section 4: I/O, global state,
/// blocking, divergence, reentrancy, thread affinity and failure).
const ACCOUNTED_EFFECTS: &str =
    "`Foreign <boundary>`, `IO`, `Global`, `Blocking`, `Diverge`, `Fail`, `Reentrant`, `ThreadAffine`";

fn accounted_effect(head: &str) -> bool {
    matches!(
        head,
        "Foreign" | "IO" | "Global" | "Blocking" | "Diverge" | "Fail" | "Reentrant" | "ThreadAffine"
    )
}

/// The arrow count of a (possibly curried) function type.
fn arity_of_arrows(ty: &Ty) -> usize {
    let mut n = 0;
    let mut cur = ty;
    loop {
        match cur {
            Ty::Fun(_, _, _, b) | Ty::Pi { cod: b, .. } => {
                n += 1;
                cur = b;
            }
            _ => return n,
        }
    }
}

/// One schema table's derived type name (`billing_events` becomes
/// `BillingEvents`).
fn codec_type_name(table: &str) -> Result<String> {
    let mut out = String::new();
    let mut upper = true;
    for ch in table.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(if upper { ch.to_ascii_uppercase() } else { ch });
            upper = false;
        } else if ch == '_' || ch == '-' || ch == ' ' {
            upper = true;
        } else {
            return Err(Error::plain(format!(
                "table name `{table}` is not a type name (the codec generator derives one type per table)"
            )));
        }
    }
    if out.is_empty() {
        return Err(Error::plain(format!(
            "table name `{table}` is not a type name (the codec generator derives one type per table)"
        )));
    }
    if out.as_bytes()[0].is_ascii_digit() {
        out.insert(0, 'T');
    }
    Ok(out)
}

/// Whether a schema column name is a field name.
fn codec_field_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '\'')
}

/// The interface's record of one parameter's binding contract. A
/// by-value aggregate crossing is recorded by its native spelling
/// (`byval` parameters, the `sret` result), part of the contract
/// projection (attribute annotations specification section 1.3).
fn iface_param(p: &crate::core::AbiParam, result: bool) -> crate::interface::ForeignParam {
    crate::interface::ForeignParam {
        ty: p.ty.clone(),
        class: match p.class {
            crate::core::AbiClass::Unit => "unit",
            crate::core::AbiClass::Scalar(_) => "scalar",
            crate::core::AbiClass::Ptr => "ptr",
            crate::core::AbiClass::Aggregate if p.by_value => {
                if result {
                    "sret"
                } else {
                    "byval"
                }
            }
            crate::core::AbiClass::Aggregate => "aggregate",
        }
        .to_string(),
        role: p.role.clone(),
        allocator: p.allocator.clone(),
        nullable: p.nullable,
        width: p.width,
        align: p.align,
        signed: p.signed,
        offsets: p.fields.iter().map(|fa| (fa.offset, fa.size)).collect(),
    }
}

// ------------------------------------------------------------ productivity helpers (P8)

/// The names a pattern binds (for the guard walk's shadowing).
fn pat_names(p: &ast::Pat, out: &mut Vec<String>) {
    match p {
        ast::Pat::Var(n, _) => out.push(n.text.clone()),
        ast::Pat::As(n, inner, _) => {
            out.push(n.text.clone());
            pat_names(inner, out);
        }
        ast::Pat::Con(_, args, _) => {
            for a in args {
                pat_names(a, out);
            }
        }
        ast::Pat::Tuple(items, _) | ast::Pat::List(items, _) => {
            for a in items {
                pat_names(a, out);
            }
        }
        ast::Pat::Record(_, fields, _) => {
            for (_, a) in fields {
                pat_names(a, out);
            }
        }
        ast::Pat::Wild(_) | ast::Pat::Unit(_) | ast::Pat::Int(_, _) | ast::Pat::Sym(_, _) => {}
    }
}

/// Visit every subterm of `t`, `t` included.
fn for_each_subterm(t: &ast::Term, f: &mut dyn FnMut(&ast::Term)) {
    f(t);
    match t {
        ast::Term::Var(_)
        | ast::Term::Con(_)
        | ast::Term::Int(_, _)
        | ast::Term::Str(_, _)
        | ast::Term::Char(_, _) | ast::Term::Sym(_, _)
        | ast::Term::Unit(_)
        | ast::Term::Hole(_)
        | ast::Term::SpliceHole { .. }
        | ast::Term::MacroCall { .. }
        | ast::Term::QuoteDecls { .. }
        | ast::Term::SeqOp(_, _) => {}
        ast::Term::Quote { body, .. } | ast::Term::Splice { body, .. } => for_each_subterm(body, f),
        ast::Term::App(h, args, _) => {
            for_each_subterm(h, f);
            for a in args {
                for_each_subterm(a, f);
            }
        }
        ast::Term::Lam(_, b, _) | ast::Term::Neg(b, _) | ast::Term::CpsWrap(b, _) => {
            for_each_subterm(b, f)
        }
        ast::Term::Let(decls, b, _) | ast::Term::Letrec(decls, b, _) => {
            for d in decls {
                match d {
                    ast::LocalDecl::Sig { .. } => {}
                    ast::LocalDecl::Eq { body, .. }
                    | ast::LocalDecl::Bind { body, .. }
                    | ast::LocalDecl::Lazy { body, .. } => for_each_subterm(body, f),
                }
            }
            for_each_subterm(b, f);
        }
        ast::Term::If { cond: c, then: x, els: y, .. } => {
            for_each_subterm(c, f);
            for_each_subterm(x, f);
            for_each_subterm(y, f);
        }
        ast::Term::Case(s, alts, _) => {
            for_each_subterm(s, f);
            for a in alts {
                if let Some(g) = &a.guard {
                    for_each_subterm(g, f);
                }
                for_each_subterm(&a.body, f);
            }
        }
        ast::Term::Ann(e, _, _) | ast::Term::Proj(e, _, _) => for_each_subterm(e, f),
        ast::Term::Forget { value, .. } => for_each_subterm(value, f),
        ast::Term::Narrow { value, witness, .. } => {
            for_each_subterm(value, f);
            for_each_subterm(witness, f);
        }
        ast::Term::Infix(items, _, _) | ast::Term::Tuple(items, _) | ast::Term::List(items, _) => {
            for x in items {
                for_each_subterm(x, f);
            }
        }
        ast::Term::Map(kvs, _) => {
            for (k, v) in kvs {
                for_each_subterm(k, f);
                for_each_subterm(v, f);
            }
        }
        ast::Term::Range(start, step, end, _) => {
            for_each_subterm(start, f);
            if let Some(x) = step {
                for_each_subterm(x, f);
            }
            for_each_subterm(end, f);
        }
        ast::Term::Record(_, fields, _) => {
            for (_, x) in fields {
                for_each_subterm(x, f);
            }
        }
        ast::Term::Do(stmts, _) => {
            for s in stmts {
                match s {
                    ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                        for_each_subterm(term, f)
                    }
                    ast::DoStmt::Let { decls, .. } => {
                        for decl in decls {
                            match decl {
                                ast::LocalDecl::Sig { .. } => {}
                                ast::LocalDecl::Eq { body, .. }
                                | ast::LocalDecl::Bind { body, .. }
                                | ast::LocalDecl::Lazy { body, .. } => for_each_subterm(body, f),
                            }
                        }
                    }
                }
            }
        }
        ast::Term::Handle(e, h, _) => {
            for_each_subterm(e, f);
            for_each_subterm(h, f);
        }
        ast::Term::Handler(h) => for_each_handler(h, f),
        ast::Term::With { value, body, .. } => {
            for_each_subterm(value, f);
            for_each_subterm(body, f);
        }
        ast::Term::CapApply { head, supplies, .. } => {
            for_each_subterm(head, f);
            for (_, x) in supplies {
                for_each_subterm(x, f);
            }
        }
    }
}

fn for_each_handler(h: &ast::HandlerExpr, f: &mut dyn FnMut(&ast::Term)) {
    for_each_subterm(&h.ret_body, f);
    for c in &h.clauses {
        for_each_subterm(&c.body, f);
    }
}

/// Every unqualified variable reference and infix operator of `t` (the
/// module call graph for the recursion groups).
fn collect_calls(t: &ast::Term, out: &mut Vec<String>) {
    for_each_subterm(t, &mut |x| match x {
        ast::Term::Var(p) if p.quals.is_empty() => out.push(p.name.text.clone()),
        ast::Term::Infix(_, ops, _) => {
            for o in ops {
                out.push(o.text.clone());
            }
        }
        _ => {}
    })
}

/// A subterm's position in the guardedness discipline (core language
/// specification section 9; codata and guardedness design, section 3).
#[derive(Clone, Copy, Default)]
struct GuardPos {
    /// Inside a `delay` argument: a lambda body here runs at force.
    delay_arg: bool,
    /// Inside a `delay` thunk body.
    in_delay: bool,
    /// Inside a codata observation's value.
    in_obs: bool,
    /// The nearest enclosing observation is `Later`-typed.
    obs_later: bool,
    /// Inside a `delay` thunk flowing to a `Later`-typed observation: the
    /// guarded position of the discipline.
    guarded: bool,
}

/// What the guard walk enforces for one definition.
enum GuardMode {
    /// A `corec` claim: every recursive call must be guarded.
    Corec,
    /// An ordinary codata producer: unguarded recursion must be reflected
    /// in the type (`Diverge` on the arrows) or refused.
    Ordinary { diverge: bool },
}

impl Checker {
    fn install_builtins(&mut self) {
        for n in ["Nat", "Int", "Bool", "Unit", "Bytes", "Sym"] {
            self.types.entry(n.to_string()).or_default().push(TypeKind {
                module: "<builtin>".into(),
                name: n.into(),
                arity: 0,
                param_kinds: Vec::new(),
                ctors: Vec::new(),
                fields: None,
                field_erased: Vec::new(), opaque: false, refine: None,
            });
        }
        // `Io r`: runtime capability evidence for identity `r` (design
        // section 4, decision D21). The value is a uniform word; the
        // identity lives in the type.
        self.types.entry("Io".to_string()).or_default().push(TypeKind {
            module: "<builtin>".into(),
            name: "Io".into(),
            arity: 1,
            param_kinds: vec![ParamKind::Type],
            ctors: Vec::new(),
            fields: None,
            field_erased: Vec::new(), opaque: false, refine: None,
        });
        // Machine scalars are primitives of the language, not user types
        // (primitive types specification, section 2).
        for s in Scalar::ALL {
            self.types.entry(s.name().to_string()).or_default().push(TypeKind {
                module: "<builtin>".into(),
                name: s.name().into(),
                arity: 0,
                param_kinds: Vec::new(),
                ctors: Vec::new(),
                fields: None,
                field_erased: Vec::new(), opaque: false, refine: None,
            });
        }
        for (n, b) in [("True", true), ("False", false)] {
            let info = ValueInfo {
                module: "<builtin>".into(),
                name: n.into(),
                scheme: Scheme::mono(Ty::Bool),
                kind: ValueKind::BoolLit(b),
            };
            self.values.entry(n.into()).or_default().push(info.clone());
            self.qualified.insert(("<builtin>".into(), n.into()), info);
        }
        // `newIO`: construct a fresh runtime identity (design section 6,
        // `Runtime.newIO`). Each use instantiates a fresh identity; the
        // uniform word is a unit marker until the runtime grows I/O.
        let new_io = ValueInfo {
            module: "<builtin>".into(),
            name: "newIO".into(),
            scheme: Scheme {
            obligs: Vec::new(),
                params: Vec::new(),
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: vec!["r".into()],
                ctx: Vec::new(),
                ty: Ty::fun(Ty::Unit, Ty::Adt("Io".into(), vec![Ty::Region("r".into())])),
            },
            kind: ValueKind::TopFn,
        };
        self.values.entry("newIO".into()).or_default().push(new_io.clone());
        self.qualified.insert(("<builtin>".into(), "newIO".into()), new_io.clone());
        // The qualified spelling `Runtime.newIO` from the design note.
        self.qualified.insert(("Runtime".into(), "newIO".into()), new_io);
        self.fns.push(FnDef {
            id: GlobalId("<builtin>.newIO".into()),
            params: vec!["_".into()],
            body: crate::core::Core::Unit,
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
        });
        self.install_index_families();
        self.install_staging_api();
        self.install_concurrency_api();
        self.install_eval_api();
    }

    /// Structured concurrency and runtime I/O (concurrency specification
    /// sections 2, 4 and 9): the builtin `Scope`/`Task` types and the
    /// `Io r` operation surface. These values are polymorphic builtin
    /// primitives (`PrimOp::Conc`), so they carry full schemes here
    /// instead of prelude `foreign import` signatures; the applications
    /// of `withScope`, `spawn` and `joinAllOrCancel` get their capture,
    /// scope and one-shot elaboration in `check_terms.rs`.
    fn install_concurrency_api(&mut self) {
        use crate::core::{ConcOp, PrimOp};
        use crate::types::{EffLabel, EffRow, Mult, RowTail};
        for (n, arity) in [("Scope", 2), ("Task", 3)] {
            self.types.entry(n.to_string()).or_default().push(TypeKind {
                module: "<builtin>".into(),
                name: n.into(),
                arity,
                param_kinds: vec![ParamKind::Type; arity],
                ctors: Vec::new(),
                fields: None,
                field_erased: Vec::new(),
                opaque: false,
                refine: None,
            });
        }
        let unit = Ty::Unit;
        let bool_ty = Ty::Bool;
        let nat = Ty::Nat;
        let bytes = Ty::Bytes;
        let a = Ty::Param("a".to_string());
        let x = Ty::Param("x".to_string());
        let v = Ty::Param("v".to_string());
        let w = Ty::Param("w".to_string());
        let open = |tail: &str| EffRow {
            labels: Vec::new(),
            tail: RowTail::Rigid(tail.to_string()),
        };
        let closed = |labels: Vec<EffLabel>| EffRow { labels, tail: RowTail::Closed };
        let plain = EffLabel::plain;
        let arg = EffLabel::with_arg;
        let io_ctx = || {
            vec![(
                "io".to_string(),
                ClassApp {
                    class: "Io".to_string(),
                    args: vec![Ty::Region("r".to_string())],
                },
            )]
        };
        let mut reg = |this: &mut Self, name: &str, scheme: Scheme, op: ConcOp| {
            let info = ValueInfo {
                module: "<builtin>".to_string(),
                name: name.to_string(),
                scheme,
                kind: ValueKind::Prim(PrimOp::Conc(op)),
            };
            this.values.entry(name.to_string()).or_default().push(info.clone());
            this.qualified.insert(("<builtin>".to_string(), name.to_string()), info);
        };
        // `withScope body`: the body sees a fresh structured scope; the
        // scope joins or cancels its children before the result returns,
        // so cleanup may not complete (`Diverge`) and the body's effects
        // flow through the result row.
        reg(
            self,
            "withScope",
            Scheme {
            obligs: Vec::new(),
                params: vec!["a".to_string()],
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: vec!["e".to_string()],
                region_params: vec!["r".to_string(), "s".to_string()],
                ctx: io_ctx(),
                ty: Ty::fun_mult(
                    Ty::fun_mult(
                        crate::check::conc_scope_ty("r", "s"),
                        Mult::Many,
                        open("e"),
                        a.clone(),
                    ),
                    Mult::Many,
                    EffRow {
                        labels: vec![plain("Diverge")],
                        tail: RowTail::Rigid("e".to_string()),
                    },
                    a.clone(),
                ),
            },
            ConcOp::WithScope,
        );
        // `spawn scope body`: the body's effects are the spawning
        // scope's responsibility (`{Async r | eb}`), and the body runs
        // exactly once on a task thread.
        reg(
            self,
            "spawn",
            Scheme {
            obligs: Vec::new(),
                params: vec!["x".to_string(), "a".to_string()],
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: vec!["eb".to_string()],
                region_params: vec!["r".to_string(), "s".to_string()],
                ctx: Vec::new(),
                ty: Ty::fun_mult(
                    crate::check::conc_scope_ty("r", "s"),
                    Mult::Many,
                    EffRow::pure(),
                    Ty::fun_mult(
                        Ty::fun_mult(unit.clone(), Mult::Many, open("eb"), {
                            crate::check::conc_result_ty(&x, &a)
                        }),
                        Mult::Many,
                        EffRow {
                            labels: vec![arg("Async", "r")],
                            tail: RowTail::Rigid("eb".to_string()),
                        },
                        crate::check::conc_task_ty("s", &x, &a),
                    ),
                ),
            },
            ConcOp::Spawn,
        );
        // `joinAllOrCancel scope h1 h2`: the two-handle shape of the
        // sketch (concurrency specification, section 2); wider joins
        // elaborate to a wider `JoinN` unit. Each handle is consumed
        // exactly once here: this is the handle's join.
        reg(
            self,
            "joinAllOrCancel",
            Scheme {
            obligs: Vec::new(),
                params: vec!["x".to_string(), "v".to_string(), "w".to_string()],
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: vec!["r".to_string(), "s".to_string()],
                ctx: Vec::new(),
                ty: Ty::fun_mult(
                    crate::check::conc_scope_ty("r", "s"),
                    Mult::Many,
                    EffRow::pure(),
                    Ty::fun_mult(
                        crate::check::conc_task_ty("s", &x, &v),
                        Mult::One,
                        EffRow::pure(),
                        Ty::fun_mult(
                            crate::check::conc_task_ty("s", &x, &w),
                            Mult::One,
                            closed(vec![arg("Async", "r"), arg("Cancel", "r"), plain("Diverge")]),
                            crate::check::conc_result_ty(
                                &x,
                                &Ty::Tuple(vec![v.clone(), w.clone()]),
                            ),
                        ),
                    ),
                ),
            },
            ConcOp::JoinN(2),
        );
        // The `Io r` operations (runtime capabilities, effects and
        // sequencing spec): effect-labeled per the P6 vocabulary.
        reg(
            self,
            "ioWrite",
            Scheme {
            obligs: Vec::new(),
                params: Vec::new(),
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: vec!["r".to_string()],
                ctx: io_ctx(),
                ty: Ty::fun_mult(
                    bytes.clone(),
                    Mult::Many,
                    closed(vec![arg("IO", "r")]),
                    unit.clone(),
                ),
            },
            ConcOp::IoWrite,
        );
        reg(
            self,
            "ioRead",
            Scheme {
            obligs: Vec::new(),
                params: Vec::new(),
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: vec!["r".to_string()],
                ctx: io_ctx(),
                ty: Ty::fun_mult(
                    nat.clone(),
                    Mult::Many,
                    closed(vec![arg("IO", "r"), plain("Diverge")]),
                    Ty::Adt("Option".to_string(), vec![bytes.clone()]),
                ),
            },
            ConcOp::IoRead,
        );
        reg(
            self,
            "ioSleep",
            Scheme {
            obligs: Vec::new(),
                params: Vec::new(),
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: vec!["r".to_string()],
                ctx: io_ctx(),
                ty: Ty::fun_mult(
                    nat,
                    Mult::Many,
                    closed(vec![arg("IO", "r"), arg("Cancel", "r")]),
                    unit,
                ),
            },
            ConcOp::IoSleep,
        );
        reg(
            self,
            "pollCancel",
            Scheme {
            obligs: Vec::new(),
                params: Vec::new(),
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: vec!["r".to_string()],
                ctx: io_ctx(),
                ty: Ty::fun_mult(
                    Ty::Unit,
                    Mult::Many,
                    closed(vec![arg("Cancel", "r")]),
                    bool_ty,
                ),
            },
            ConcOp::PollCancel,
        );
    }

    /// Staging (P9): the builtin compile-time types and the capability API
    /// (modules specification sections 7 and 10). `Code`, `Syntax`,
    /// `Declarations`, `String`, `Secret` and the capability tokens are
    /// compile-time-only types: they never appear in target code (decision
    /// D43).
    /// `eval : Code a -> a` (supercompilation and JIT specification
    /// section 7): a builtin whose signature carries no capability or
    /// effect (a quotation is typed when it is constructed). It is never
    /// evidence: the application checks refuse `spec`/`proof` and erased
    /// positions, and a target profile may forbid runtime code
    /// generation entirely (`allow_eval`).
    fn install_eval_api(&mut self) {
        let a = crate::types::Ty::Param("a".to_string());
        let info = ValueInfo {
            module: "<builtin>".to_string(),
            name: "eval".to_string(),
            scheme: Scheme {
            obligs: Vec::new(),
                params: vec!["a".to_string()],
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: Vec::new(),
                ctx: Vec::new(),
                ty: crate::types::Ty::fun(
                    crate::types::Ty::Adt("Code".to_string(), vec![a.clone()]),
                    a,
                ),
            },
            kind: ValueKind::Prim(PrimOp::Eval),
        };
        self.values.entry("eval".to_string()).or_default().push(info.clone());
        self.qualified.insert(("<builtin>".to_string(), "eval".to_string()), info);
    }

    /// The generated WIT adapter vocabulary (WebAssembly and WASI
    /// specification sections 3, 4 and 6): each raw binding is a foreign
    /// plan over the canonical ABI glue registered under the `Raw`
    /// module's qualified names (a bare function name stays the user's
    /// to define), and the stream observations are word-level host
    /// imports carrying section 6's degraded observation type.
    fn install_wit_api(&mut self, adapters: &[crate::interface::WitAdapter]) -> Result<()> {
        use crate::core::{AbiClass, AbiParam, ForeignPlan};
        use crate::interface::SurfaceTy;
        use crate::types::{EffLabel, EffRow, Mult, RowTail, Ty};

        fn surface_ty(s: &SurfaceTy) -> Ty {
            match s {
                SurfaceTy::Named(n) => match n.as_str() {
                    "Bool" => Ty::Bool,
                    "Unit" => Ty::Unit,
                    "Bytes" => Ty::Bytes,
                    "Nat" => Ty::Nat,
                    "Int" => Ty::Int,
                    other => {
                        for sc in crate::types::Scalar::ALL {
                            if sc.name() == other {
                                return Ty::Prim(sc);
                            }
                        }
                        Ty::Adt(other.to_string(), Vec::new())
                    }
                },
                SurfaceTy::Borrow(t) => {
                    Ty::Ref("region".to_string(), Box::new(surface_ty(t)))
                }
                SurfaceTy::App(f, args) => {
                    Ty::Adt(f.clone(), args.iter().map(surface_ty).collect())
                }
                SurfaceTy::Tuple(ts) => Ty::Tuple(ts.iter().map(surface_ty).collect()),
            }
        }

        let effects = EffRow {
            labels: vec![
                EffLabel::with_arg("Foreign", "Component"),
                EffLabel::plain("Diverge"),
            ],
            tail: RowTail::Closed,
        };
        for ad in adapters {
            self.wit_adapters.push(ad.clone());
            for b in &ad.bindings {
                let unit = format!("cinch.wit.{}.{}", ad.module, b.name);
                let arity = b.params.len();
                let mut ty = surface_ty(&b.result);
                for p in b.params.iter().rev() {
                    let mult = if p.consume { Mult::One } else { Mult::Many };
                    ty = Ty::fun_mult(surface_ty(&p.ty), mult, effects.clone(), ty);
                }
                let borrows = b.params.iter().any(|p| matches!(p.ty, SurfaceTy::Borrow(_)));
                let mut region_params: Vec<String> = Vec::new();
                if borrows {
                    region_params.push("region".to_string());
                }
                // The `Foreign <boundary>` label's argument renames like
                // a region at instantiation (the boundary name is rigid
                // per signature, flexible per call), matching how user
                // `foreign import` schemes classify it.
                region_params.push("Component".to_string());
                let scheme = Scheme {
            obligs: Vec::new(),
                    params: Vec::new(),
                    idx_params: Vec::new(),
                    val_params: Vec::new(),
                    prop_params: Vec::new(),
                    row_params: Vec::new(),
                    region_params,
                    ctx: Vec::new(),
                    ty,
                };
                let info = ValueInfo {
                    module: "<builtin>".to_string(),
                    name: b.name.clone(),
                    scheme,
                    kind: ValueKind::Foreign { unit: unit.clone(), arity },
                };
                self.qualified.insert((ad.module.clone(), b.name.clone()), info);
                // The canonical-ABI plan: the call-side raw binding, and
                // for a world export the trampoline forwarding to the
                // program's implementation unit.
                let mut params: Vec<AbiParam> = b
                    .params
                    .iter()
                    .map(|p| AbiParam {
                        ty: p.ty.spelling(),
                        class: AbiClass::Ptr,
                        role: if p.consume { "owned" } else { "borrowed" }.to_string(),
                        allocator: String::new(),
                        nullable: false,
                        width: 8,
                        align: 8,
                        signed: false,
                        pointee_align: 0,
                        by_value: false,
                        tag: 0,
                        fields: Vec::new(),
                    })
                    .collect();
                let has_ret_ptr = b.call.ret.is_some();
                if has_ret_ptr {
                    params.push(AbiParam {
                        ty: "(return area)".to_string(),
                        class: AbiClass::Ptr,
                        role: "out".to_string(),
                        allocator: String::new(),
                        nullable: false,
                        width: 8,
                        align: 8,
                        signed: false,
                        pointee_align: 0,
                        by_value: false,
                        tag: 0,
                        fields: Vec::new(),
                    });
                }
                let result = AbiParam {
                    ty: b.result.spelling(),
                    class: if b.call.core_results.is_empty() && has_ret_ptr {
                        AbiClass::Unit
                    } else {
                        AbiClass::Ptr
                    },
                    role: "owned".to_string(),
                    allocator: String::new(),
                    nullable: false,
                    width: 8,
                    align: 8,
                    signed: false,
                    pointee_align: 0,
                    by_value: false,
                    tag: 0,
                    fields: Vec::new(),
                };
                self.foreign_plans.push((
                    unit.clone(),
                    ForeignPlan {
                        symbol: b.symbol.clone(),
                        export: false,
                        unit,
                        convention: "ccall".to_string(),
                        params: params.clone(),
                        result: result.clone(),
                        deallocator: String::new(),
                        canon: Some(b.call.clone()),
                    },
                ));
                if b.export {
                    self.foreign_plans.push((
                        b.impl_unit.clone(),
                        ForeignPlan {
                            symbol: b.symbol.clone(),
                            export: true,
                            unit: b.impl_unit.clone(),
                            convention: "ccall".to_string(),
                            params,
                            result,
                            deallocator: String::new(),
                            canon: Some(b.call.clone()),
                        },
                    ));
                }
            }
            if ad.streams {
                // The degraded observation vocabulary (section 6): a
                // WASI stream is an asynchronous source or sink and
                // never a productive `Stream a`; an observation may end,
                // fail or wait indefinitely (`Diverge`), and data and
                // completion travel separate channels (the `stream` and
                // `future` values). The word-level host imports use the
                // runtime's block convention: the host or embedder shim
                // produces the safe words.
                self.install_stream_api()?;
            }
        }
        Ok(())
    }

    /// The stream observation values of the adapter vocabulary
    /// (WebAssembly and WASI specification section 6).
    fn install_stream_api(&mut self) -> Result<()> {
        use crate::core::{AbiClass, AbiParam, ForeignPlan};
        use crate::types::{EffLabel, EffRow, RowTail, Ty};
        let a = Ty::Param("a".to_string());
        let effects = EffRow {
            labels: vec![
                EffLabel::with_arg("Foreign", "Component"),
                EffLabel::plain("Diverge"),
            ],
            tail: RowTail::Closed,
        };
        let stream_a = Ty::Adt("WasiStream".to_string(), vec![a.clone()]);
        let future_a = Ty::Adt("WasiFuture".to_string(), vec![a.clone()]);
        let stream_err = Ty::Adt("StreamError".to_string(), Vec::new());
        let opt_a = Ty::Adt("Option".to_string(), vec![a.clone()]);
        let result_ty = |e: Ty, o: Ty| Ty::Adt("Result".to_string(), vec![e, o]);
        let entries: Vec<(&str, &str, Ty, usize)> = vec![
            (
                "wasiStreamRead",
                "cinch.wasi.stream-read",
                Ty::fun_eff(stream_a.clone(), effects.clone(), result_ty(stream_err.clone(), opt_a)),
                1,
            ),
            (
                "wasiStreamWrite",
                "cinch.wasi.stream-write",
                Ty::fun_eff(
                    stream_a.clone(),
                    effects.clone(),
                    Ty::fun_eff(a.clone(), effects.clone(), result_ty(stream_err.clone(), Ty::Unit)),
                ),
                2,
            ),
            (
                "wasiFutureWait",
                "cinch.wasi.future-wait",
                Ty::fun_eff(future_a, effects, a),
                1,
            ),
        ];
        for (name, symbol, ty, arity) in entries {
            let scheme = Scheme {
            obligs: Vec::new(),
                params: vec!["a".to_string()],
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                // As in [`Checker::install_wit_api`]: the `Foreign`
                // boundary name renames like a region per call.
                region_params: vec!["Component".to_string()],
                ctx: Vec::new(),
                ty,
            };
            let unit = format!("cinch.wit.{name}");
            let info = ValueInfo {
                module: "<builtin>".to_string(),
                name: name.to_string(),
                scheme,
                kind: ValueKind::Foreign { unit: unit.clone(), arity },
            };
            self.values.entry(name.to_string()).or_default().push(info.clone());
            self.qualified.insert(("<builtin>".to_string(), name.to_string()), info);
            let word_param = AbiParam {
                ty: "handle".to_string(),
                class: AbiClass::Ptr,
                role: "borrowed".to_string(),
                allocator: String::new(),
                nullable: false,
                width: 8,
                align: 8,
                signed: false,
                pointee_align: 0,
                by_value: false,
                tag: 0,
                fields: Vec::new(),
            };
            let mut params = vec![word_param.clone()];
            if arity == 2 {
                params.push(word_param);
            }
            self.foreign_plans.push((
                unit.clone(),
                ForeignPlan {
                    symbol: symbol.to_string(),
                    export: false,
                    unit,
                    convention: "ccall".to_string(),
                    params,
                    result: AbiParam {
                        ty: "result word".to_string(),
                        class: AbiClass::Ptr,
                        role: "owned".to_string(),
                        allocator: String::new(),
                        nullable: false,
                        width: 8,
                        align: 8,
                        signed: false,
                        pointee_align: 0,
                        by_value: false,
                        tag: 0,
                        fields: Vec::new(),
                    },
                    deallocator: String::new(),
                    canon: None,
                },
            ));
        }
        Ok(())
    }

    fn install_staging_api(&mut self) {
        for (n, arity) in [
            ("Code", 1),
            ("Syntax", 0),
            ("Declarations", 0),
            ("Name", 0),
            ("Secret", 0),
            ("IoCap", 0),
            ("FileReadCap", 0),
            ("FileWriteCap", 0),
            ("ProcCap", 0),
            ("SchemaSnapshot", 0),
            ("Database", 0),
            ("SchemaRead", 0),
        ] {
            self.types.entry(n.to_string()).or_default().push(TypeKind {
                module: "<builtin>".into(),
                name: n.into(),
                arity,
                param_kinds: if arity == 1 {
                    vec![ParamKind::Type]
                } else {
                    Vec::new()
                },
                ctors: Vec::new(),
                fields: None,
                field_erased: Vec::new(), opaque: false, refine: None,
            });
        }
        let adt = |name: &str, args: Vec<Ty>| Ty::Adt(name.to_string(), args);
        let str_ty = adt("String", Vec::new());
        let secret_ty = adt("Secret", Vec::new());
        let decls_ty = adt("Declarations", Vec::new());
        let io_ty = adt("IoCap", Vec::new());
        let fr_ty = adt("FileReadCap", Vec::new());
        let fw_ty = adt("FileWriteCap", Vec::new());
        let pc_ty = adt("ProcCap", Vec::new());
        let unit = Ty::Unit;
        let nat = Ty::Nat;
        let a = Ty::Param("a".to_string());
        use crate::core::StageOp;
        use crate::types::{Mult, RowTail};
        // Capability refinements supply their token to a callback whose
        // effects flow through the result: `forall a e. String ->
        // (IoCap ->{e} a) ->{e} a` for the snapshot root, `forall a e.
        // IoCap -> (cap ->{e} a) ->{e} a` for each refinement.
        let row_e = EffRow { labels: Vec::new(), tail: RowTail::Rigid("e".to_string()) };
        self.install_stage_value_row(
            "snapshot",
            StageOp::Snapshot,
            vec!["a".to_string()],
            vec!["e".to_string()],
            Ty::fun_mult(
                str_ty.clone(),
                Mult::Many,
                EffRow::pure(),
                Ty::fun_mult(
                    Ty::fun_mult(io_ty.clone(), Mult::Many, row_e.clone(), a.clone()),
                    Mult::Many,
                    row_e.clone(),
                    a.clone(),
                ),
            ),
        );
        for (name, op, cap) in [
            ("withReadFiles", StageOp::WithReadFiles, fr_ty.clone()),
            ("withWriteFiles", StageOp::WithWriteFiles, fw_ty.clone()),
            ("withProcess", StageOp::WithProcess, pc_ty.clone()),
        ] {
            self.install_stage_value_row(
                name,
                op,
                vec!["a".to_string()],
                vec!["e".to_string()],
                Ty::fun_mult(
                    io_ty.clone(),
                    Mult::Many,
                    EffRow::pure(),
                    Ty::fun_mult(
                        Ty::fun_mult(cap, Mult::Many, row_e.clone(), a.clone()),
                        Mult::Many,
                        row_e.clone(),
                        a.clone(),
                    ),
                ),
            );
        }
        let io_eff = |labels: &[&str]| EffRow {
            labels: labels.iter().map(|l| EffLabel::plain(*l)).collect(),
            tail: RowTail::Closed,
        };
        self.install_stage_value(
            "readFile",
            StageOp::ReadFile,
            Vec::new(),
            Ty::fun_mult(
                fr_ty,
                Mult::Many,
                EffRow::pure(),
                Ty::fun_mult(str_ty.clone(), Mult::Many, io_eff(&["StageFile"]), str_ty.clone()),
            ),
        );
        self.install_stage_value(
            "writeFile",
            StageOp::WriteFile,
            Vec::new(),
            Ty::fun_mult(
                fw_ty,
                Mult::Many,
                EffRow::pure(),
                Ty::fun_mult(
                    str_ty.clone(),
                    Mult::Many,
                    EffRow::pure(),
                    Ty::fun_mult(str_ty.clone(), Mult::Many, io_eff(&["StageFile"]), unit.clone()),
                ),
            ),
        );
        self.install_stage_value(
            "run",
            StageOp::Run,
            Vec::new(),
            Ty::fun_mult(
                pc_ty,
                Mult::Many,
                EffRow::pure(),
                Ty::fun_mult(
                    str_ty.clone(),
                    Mult::Many,
                    EffRow::pure(),
                    Ty::fun_mult(
                        adt("List", vec![str_ty.clone()]),
                        Mult::Many,
                        io_eff(&["StageProcess"]),
                        str_ty.clone(),
                    ),
                ),
            ),
        );
        self.install_stage_value(
            "secret",
            StageOp::Secret,
            Vec::new(),
            Ty::fun_mult(str_ty.clone(), Mult::Many, io_eff(&["StageSecret"]), secret_ty.clone()),
        );
        self.install_stage_value(
            "emit",
            StageOp::Emit,
            Vec::new(),
            Ty::fun_mult(decls_ty.clone(), Mult::Many, io_eff(&["StageEmit"]), unit.clone()),
        );
        // Approved constant transport (modules specification section 9):
        // a supported constant becomes generated code. Secrets and other
        // compile-time-only values cannot be lifted.
        self.install_stage_value(
            "quoteNat",
            StageOp::QuoteNat,
            Vec::new(),
            Ty::fun_mult(
                nat.clone(),
                Mult::Many,
                EffRow::pure(),
                adt("Code", vec![nat.clone()]),
            ),
        );
        self.install_stage_value(
            "quoteInt",
            StageOp::QuoteInt,
            Vec::new(),
            Ty::fun_mult(
                Ty::Int,
                Mult::Many,
                EffRow::pure(),
                adt("Code", vec![Ty::Int]),
            ),
        );
        self.install_stage_value(
            "quoteBool",
            StageOp::QuoteBool,
            Vec::new(),
            Ty::fun_mult(
                Ty::Bool,
                Mult::Many,
                EffRow::pure(),
                adt("Code", vec![Ty::Bool]),
            ),
        );
        // The `Name`/`Sym` reification (D123's name-quote
        // correspondence): a reified name and an interned symbol of the
        // same spelling convert back and forth.
        self.install_stage_value(
            "name-to-sym",
            StageOp::NameToSym,
            Vec::new(),
            Ty::fun_mult(adt("Name", Vec::new()), Mult::Many, EffRow::pure(), Ty::Sym),
        );
        self.install_stage_value(
            "sym-to-name",
            StageOp::SymToName,
            Vec::new(),
            Ty::fun_mult(Ty::Sym, Mult::Many, EffRow::pure(), adt("Name", Vec::new())),
        );
        self.install_stage_value(
            "strNat",
            StageOp::StrNat,
            Vec::new(),
            Ty::fun_mult(str_ty.clone(), Mult::Many, EffRow::pure(), nat.clone()),
        );
        self.install_stage_value(
            "natStr",
            StageOp::NatStr,
            Vec::new(),
            Ty::fun_mult(nat, Mult::Many, EffRow::pure(), str_ty.clone()),
        );
        self.install_stage_value(
            "strAppend",
            StageOp::StrAppend,
            Vec::new(),
            Ty::fun_mult(
                str_ty.clone(),
                Mult::Many,
                EffRow::pure(),
                Ty::fun_mult(str_ty.clone(), Mult::Many, EffRow::pure(), str_ty.clone()),
            ),
        );
        self.install_stage_value(
            "strLen",
            StageOp::StrLen,
            Vec::new(),
            Ty::fun_mult(str_ty.clone(), Mult::Many, EffRow::pure(), Ty::Nat),
        );
        // Schema introspection (modules specification section 11; the
        // A6 vocabulary): `Stage.withDatabase` runs the introspection
        // tool over the subprocess capability (decision D45's
        // delegation), `introspectConsistently` is the consistency check
        // the sketch names, and `CodecGenerator.derive` is the codec
        // generator producing `quoteDecls` output from a snapshot. The
        // snapshot is compile-time-only (decision D43's family).
        let db_ty = adt("Database", Vec::new());
        let snap_ty = adt("SchemaSnapshot", Vec::new());
        self.install_stage_value_row(
            "withDatabase",
            StageOp::WithDatabase,
            vec!["a".to_string()],
            vec!["e".to_string()],
            Ty::fun_mult(
                io_ty.clone(),
                Mult::Many,
                EffRow::pure(),
                Ty::fun_mult(
                    secret_ty.clone(),
                    Mult::Many,
                    EffRow::pure(),
                    Ty::fun_mult(
                        adt("SchemaRead", Vec::new()),
                        Mult::Many,
                        EffRow::pure(),
                        Ty::fun_mult(
                            Ty::fun_mult(db_ty.clone(), Mult::Many, row_e.clone(), a.clone()),
                            Mult::Many,
                            row_e.clone(),
                            a.clone(),
                        ),
                    ),
                ),
            ),
        );
        self.install_stage_value(
            "ReadSchema",
            StageOp::ReadSchema,
            Vec::new(),
            adt("SchemaRead", Vec::new()),
        );
        self.install_stage_value(
            "introspectConsistently",
            StageOp::Introspect,
            Vec::new(),
            Ty::fun_mult(db_ty, Mult::Many, EffRow::pure(), snap_ty.clone()),
        );
        // `CodecGenerator.derive` is qualified (a bare `derive` stays
        // available to user definitions).
        let derive_info = ValueInfo {
            module: "<builtin>".into(),
            name: "derive".into(),
            scheme: Scheme {
            obligs: Vec::new(),
                params: Vec::new(),
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: Vec::new(),
                ctx: Vec::new(),
                ty: Ty::fun_mult(snap_ty, Mult::Many, EffRow::pure(), decls_ty),
            },
            kind: ValueKind::StageBuiltin(StageOp::CodecDerive),
        };
        self.qualified
            .insert(("CodecGenerator".into(), "derive".into()), derive_info);
    }

    /// Register one builtin of the compile-time API under its bare name and
    /// the `Stage.` qualification.
    fn install_stage_value(
        &mut self,
        name: &str,
        op: crate::core::StageOp,
        params: Vec<String>,
        ty: Ty,
    ) {
        self.install_stage_value_row(name, op, params, Vec::new(), ty);
    }

    /// [`install_stage_value`] with quantified row parameters.
    fn install_stage_value_row(
        &mut self,
        name: &str,
        op: crate::core::StageOp,
        params: Vec<String>,
        row_params: Vec<String>,
        ty: Ty,
    ) {
        let scheme = Scheme {
            obligs: Vec::new(),
            params,
            idx_params: Vec::new(),
            val_params: Vec::new(),
            prop_params: Vec::new(),
            row_params,
            region_params: Vec::new(),
            ctx: Vec::new(),
            ty,
        };
        let info = ValueInfo {
            module: "<builtin>".into(),
            name: name.into(),
            scheme,
            kind: ValueKind::StageBuiltin(op),
        };
        self.values.entry(name.into()).or_default().push(info.clone());
        self.qualified.insert(("<builtin>".into(), name.into()), info.clone());
        self.qualified.insert(("Stage".into(), name.into()), info);
    }

    /// The dependent core's builtin families (decision D30): `Vec`, `Fin`
    /// and `Dec` are builtin indexed families. The indexed-declaration
    /// mechanism itself is general (`tests/dependent.rs` declares user
    /// families), but these three arrive as builtins because their runtime
    /// helpers (`Vec.getCopy` walking a borrowed vector by a `Fin`) need
    /// code the surface cannot express before the codata and borrow
    /// recursion slices, and because `Vec`'s constructors are spelled
    /// `Nil`/`Cons`, which the prelude's `List` already owns.
    fn install_index_families(&mut self) {
        // `data Vec (n :: Nat) (a :: Type) where Nil :: Vec 0 a;
        //    Cons :: forall k. a -> Vec k a -> Vec (k + 1) a`.
        self.register_family(
            "Vec",
            vec![
                ("n".to_string(), ParamKind::Val(Ty::Nat)),
                ("a".to_string(), ParamKind::Type),
            ],
            vec![
                (
                    "Nil".to_string(),
                    0,
                    Vec::new(),
                    Vec::new(),
                    vec![Ty::Idx(Box::new(ITerm::num(0))), Ty::Param("a".to_string())],
                ),
                (
                    "Cons".to_string(),
                    1,
                    vec![IxParam::Val("k".to_string(), Ty::Nat)],
                    vec![
                        ("$f0".to_string(), Ty::Param("a".to_string())),
                        (
                            "$f1".to_string(),
                            Ty::Adt(
                                "Vec".to_string(),
                                vec![
                                    Ty::Idx(Box::new(ITerm::Var("k".to_string()))),
                                    Ty::Param("a".to_string()),
                                ],
                            ),
                        ),
                    ],
                    vec![
                        Ty::Idx(Box::new(ITerm::Bin(
                            IdxOp::Add,
                            Box::new(ITerm::Var("k".to_string())),
                            Box::new(ITerm::num(1)),
                        ))),
                        Ty::Param("a".to_string()),
                    ],
                ),
            ],
        );
        // `data Fin (n :: Nat) where Fin :: (i :: Nat) -> (bound :: i < n)
        // -> Fin n`: the bound is erased evidence (core specification
        // section 4); `i` is the runtime index.
        self.register_family(
            "Fin",
            vec![("n".to_string(), ParamKind::Val(Ty::Nat))],
            vec![(
                "Fin".to_string(),
                0,
                Vec::new(),
                vec![
                    ("i".to_string(), Ty::Nat),
                    (
                        "bound".to_string(),
                        Ty::Prp(Box::new(PTerm::Cmp(
                            IdxCmp::Lt,
                            Box::new(ITerm::Var("i".to_string())),
                            Box::new(ITerm::Var("n".to_string())),
                        ))),
                    ),
                ],
                vec![Ty::Idx(Box::new(ITerm::Var("n".to_string())))],
            )],
        );
        // `data Dec (p :: Prop) where Yes :: (h :: p) -> Dec p;
        //    No :: (h :: Not p) -> Dec p`: the result of `decide`, with
        // both evidence fields erased.
        self.register_family(
            "Dec",
            vec![("p".to_string(), ParamKind::Prop)],
            vec![
                (
                    "Yes".to_string(),
                    0,
                    Vec::new(),
                    vec![("h".to_string(), Ty::Prp(Box::new(PTerm::Var("p".to_string()))))],
                    vec![Ty::Prp(Box::new(PTerm::Var("p".to_string())))],
                ),
                (
                    "No".to_string(),
                    1,
                    Vec::new(),
                    vec![(
                        "h".to_string(),
                        Ty::Prp(Box::new(PTerm::Not(Box::new(PTerm::Var("p".to_string()))))),
                    )],
                    vec![Ty::Prp(Box::new(PTerm::Var("p".to_string())))],
                ),
            ],
        );
        self.install_index_values();
    }

    /// Register a builtin indexed family with its constructors. A
    /// constructor is `(name, tag, local index binders, (field binder,
    /// field type) fields, result arguments)`.
    fn register_family(
        &mut self,
        name: &str,
        params: Vec<(String, ParamKind)>,
        ctors: Vec<(String, usize, Vec<IxParam>, Vec<(String, Ty)>, Vec<Ty>)>,
    ) {
        let param_kinds: Vec<ParamKind> = params.iter().map(|(_, k)| k.clone()).collect();
        let mut ctor_names: Vec<(String, usize)> = Vec::new();
        for (cname, tag, local_idx, fields, res_args) in ctors {
            let field_tys: Vec<Ty> = fields.iter().map(|(_, t)| t.clone()).collect();
            let field_erased: Vec<bool> = field_tys.iter().map(|t| erased_ty(t)).collect();
            let runtime_arity = field_erased.iter().filter(|e| !**e).count();
            let mut all: Vec<Ty> = field_tys.clone();
            all.extend(res_args.iter().cloned());
            let mut type_params: Vec<String> = Vec::new();
            let mut idx_params: Vec<IxParam> = Vec::new();
            for (p, k) in &params {
                if !all.iter().any(|t| ty_mentions(t, p)) {
                    continue;
                }
                match k {
                    ParamKind::Type | ParamKind::Con(_) => type_params.push(p.clone()),
                    ParamKind::Val(t) => idx_params.push(IxParam::Val(p.clone(), t.clone())),
                    ParamKind::Prop => idx_params.push(IxParam::Prop(p.clone())),
                }
            }
            idx_params.extend(local_idx.iter().cloned());
            let mut ty = Ty::Adt(name.to_string(), res_args.clone());
            for (b, t) in fields.iter().rev() {
                let rest = ty;
                ty = if rest.mentions_idx(b) {
                    Ty::pi(b.clone(), t.clone(), default_mult(t), EffRow::pure(), rest)
                } else {
                    Ty::fun_mult(t.clone(), default_mult(t), EffRow::pure(), rest)
                };
            }
            let scheme = Scheme {
            obligs: Vec::new(),
                params: type_params,
                idx_params,
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: Vec::new(),
                ctx: Vec::new(),
                ty,
            };
            let gid = GlobalId(format!("<builtin>.{cname}"));
            self.add_value(
                ValueInfo {
                    module: "<builtin>".to_string(),
                    name: cname.clone(),
                    scheme,
                    kind: ValueKind::Ctor {
                        tag,
                        arity: field_tys.len(),
                        runtime_arity,
                        field_tys,
                        field_erased,
                        field_binders: fields.iter().map(|(b, _)| b.clone()).collect(),
                        ty_params: params.iter().map(|(n, _)| n.clone()).collect(),
                        local_idx,
                        res_args,
                        refine: None,
                    },
                },
                ast::Span { line: 0, col: 0 },
            )
            .expect("builtin constructors do not collide");
            self.fns.push(FnDef {
                id: gid,
                params: (0..runtime_arity).map(|i| format!("f{i}")).collect(),
                body: crate::core::Core::Unit,
                prim: None,
                ctor: Some((tag, runtime_arity)),
                hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
            });
            ctor_names.push((cname, tag));
        }
        self.types.entry(name.to_string()).or_default().push(TypeKind {
            module: "<builtin>".to_string(),
            name: name.to_string(),
            arity: params.len(),
            param_kinds,
            ctors: ctor_names,
            fields: None,
            field_erased: Vec::new(), opaque: false, refine: None,
        });
    }

    /// The builtin index vocabulary and evidence constructors.
    fn install_index_values(&mut self) {
        // `toNat :: U32 -> Nat` and `length :: Bytes -> Nat`: total,
        // executable index functions (decision D29) used in refinements
        // and discharged by `decide`.
        let to_nat = ValueInfo {
            module: "<builtin>".to_string(),
            name: "toNat".to_string(),
            scheme: Scheme::mono(Ty::fun(Ty::Prim(Scalar::U32), Ty::Nat)),
            kind: ValueKind::TopFn,
        };
        self.add_value(to_nat.clone(), ast::Span { line: 0, col: 0 })
            .expect("builtin");
        self.fns.push(FnDef {
            id: GlobalId("<builtin>.toNat".to_string()),
            params: vec!["x".to_string()],
            body: crate::core::Core::App(
                Box::new(crate::core::Core::Prim(PrimOp::Nat(NatOp::FromI64))),
                vec![crate::core::Core::Var(LocalId(0))],
            ),
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
        });
        let length = ValueInfo {
            module: "<builtin>".to_string(),
            name: "length".to_string(),
            scheme: Scheme::mono(Ty::fun(Ty::Bytes, Ty::Nat)),
            kind: ValueKind::TopFn,
        };
        self.add_value(length, ast::Span { line: 0, col: 0 })
            .expect("builtin");
        self.fns.push(FnDef {
            id: GlobalId("<builtin>.length".to_string()),
            params: vec!["b".to_string()],
            body: crate::core::Core::App(
                Box::new(crate::core::Core::Prim(PrimOp::Bytes(BytesOp::Len))),
                vec![crate::core::Core::Var(LocalId(0))],
            ),
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
        });
        // `Vec.getCopy`: copy the element at a `Fin` index out of a
        // borrowed vector (the acceptance probe's safe element access).
        // The walk needs a recursive block walk the surface cannot express
        // yet, so it is a builtin; `Copy` evidence is structural identity
        // at runtime (decision D16), so the dictionary is passed and
        // ignored.
        let get_copy = ValueInfo {
            module: "<builtin>".to_string(),
            name: "getCopy".to_string(),
            scheme: Scheme {
            obligs: Vec::new(),
                params: vec!["a".to_string()],
                idx_params: vec![IxParam::Val("n".to_string(), Ty::Nat)],
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: vec!["region".to_string()],
                ctx: vec![(
                    "copy".to_string(),
                    ClassApp {
                        class: "Copy".to_string(),
                        args: vec![Ty::Param("a".to_string())],
                    },
                )],
                ty: Ty::funs(
                    &[
                        Ty::Ref(
                            "region".to_string(),
                            Box::new(Ty::Adt(
                                "Vec".to_string(),
                                vec![
                                    Ty::Idx(Box::new(ITerm::Var("n".to_string()))),
                                    Ty::Param("a".to_string()),
                                ],
                            )),
                        ),
                        Ty::Adt(
                            "Fin".to_string(),
                            vec![Ty::Idx(Box::new(ITerm::Var("n".to_string())))],
                        ),
                    ],
                    Ty::Param("a".to_string()),
                ),
            },
            kind: ValueKind::TopFn,
        };
        // The qualified spelling `Vec.getCopy` of the acceptance probes.
        self.qualified.insert(("Vec".to_string(), "getCopy".to_string()), get_copy);
        let go = GlobalId("<builtin>.getCopy.go".to_string());
        // go (v, i): if i == 0 take the head, else walk the tail with
        // i - 1. A well-typed `Fin n` never walks past the end; the fault
        // path (exit 70) is defense in depth like any fall-through match.
        let zero = crate::core::Core::ConstBig { nat: true, digits: "0".to_string() };
        let one = crate::core::Core::ConstBig { nat: true, digits: "1".to_string() };
        let i_is_zero = crate::core::Core::App(
            Box::new(crate::core::Core::Prim(PrimOp::Cmp {
                ty: Scalar::I64,
                op: CmpOp::Eq,
            })),
            vec![
                crate::core::Core::App(
                    Box::new(crate::core::Core::Prim(PrimOp::Nat(NatOp::Cmp))),
                    vec![crate::core::Core::Var(LocalId(1)), zero],
                ),
                crate::core::Core::Int(0),
            ],
        );
        let walk = crate::core::Core::Case {
            scrut: Box::new(crate::core::Core::Var(LocalId(0))),
            alts: vec![
                CAlt::new(CPat::Con { tag: 0, fields: vec![] }, crate::core::Core::Fault),
                CAlt::new(
                    CPat::Con { tag: 1, fields: vec![LocalId(2), LocalId(3)] },
                    crate::core::Core::If(
                        Box::new(i_is_zero),
                        CBranch::new(crate::core::Core::Var(LocalId(2))),
                        CBranch::new(crate::core::Core::App(
                            Box::new(crate::core::Core::Global(go.clone())),
                            vec![
                                crate::core::Core::Var(LocalId(3)),
                                crate::core::Core::App(
                                    Box::new(crate::core::Core::Prim(PrimOp::Nat(NatOp::Sub))),
                                    vec![crate::core::Core::Var(LocalId(1)), one],
                                ),
                            ],
                        )),
                    ),
                ),
            ],
        };
        self.fns.push(FnDef {
            id: go,
            params: vec!["v".to_string(), "i".to_string()],
            body: walk,
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
        });
        self.fns.push(FnDef {
            id: GlobalId("<builtin>.getCopy".to_string()),
            params: vec!["_d".to_string(), "v".to_string(), "fin".to_string()],
            body: crate::core::Core::Let(
                LocalId(3),
                Box::new(crate::core::Core::Proj(
                    Box::new(crate::core::Core::Var(LocalId(2))),
                    crate::core::ProjField::slot(0),
                )),
                Box::new(crate::core::Core::App(
                    Box::new(crate::core::Core::Global(
                        GlobalId("<builtin>.getCopy.go".to_string()),
                    )),
                    vec![crate::core::Core::Var(LocalId(1)), crate::core::Core::Var(LocalId(3))],
                )),
            ),
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
        });
        // `vecStream`/`vecStreamStep` (P10): a producer over a borrowed
        // `Vec` for the target library's scalar sum. The step is the same
        // block walk as `Vec.getCopy`, producing the step form of
        // modules and compilation specification section 6.
        let step_gid = GlobalId("<builtin>.vecStreamStep".to_string());
        // The step form's constructors live with their type
        // (Data.Producer after the prelude split).
        let done_gid = GlobalId("Data.Producer.Done".to_string());
        let yield_gid = GlobalId("Data.Producer.Yield".to_string());
        let step_walk = crate::core::Core::Case {
            scrut: Box::new(crate::core::Core::Var(LocalId(0))),
            alts: vec![
                CAlt::new(
                    CPat::Con { tag: 0, fields: vec![] },
                    crate::core::Core::App(Box::new(crate::core::Core::Ctor(done_gid)), vec![]),
                ),
                CAlt::new(
                    CPat::Con { tag: 1, fields: vec![LocalId(1), LocalId(2)] },
                    crate::core::Core::App(
                        Box::new(crate::core::Core::Ctor(yield_gid)),
                        vec![
                            crate::core::Core::Var(LocalId(1)),
                            crate::core::Core::Var(LocalId(2)),
                        ],
                    ),
                ),
            ],
        };
        let vec_ty = Ty::Adt(
            "Vec".to_string(),
            vec![
                Ty::Idx(Box::new(ITerm::Var("n".to_string()))),
                Ty::Param("a".to_string()),
            ],
        );
        let ref_ty = Ty::Ref("region".to_string(), Box::new(vec_ty.clone()));
        let step_ty = Ty::Adt(
            "Step".to_string(),
            vec![ref_ty.clone(), Ty::Param("a".to_string())],
        );
        let step_value = ValueInfo {
            module: "<builtin>".to_string(),
            name: "vecStreamStep".to_string(),
            scheme: Scheme {
            obligs: Vec::new(),
                params: vec!["a".to_string()],
                idx_params: vec![IxParam::Val("n".to_string(), Ty::Nat)],
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: vec!["region".to_string()],
                ctx: Vec::new(),
                ty: Ty::fun_mult(ref_ty, Mult::Many, EffRow::pure(), step_ty),
            },
            kind: ValueKind::TopFn,
        };
        self.add_value(step_value, ast::Span { line: 0, col: 0 })
            .expect("builtin");
        self.fns.push(FnDef {
            id: step_gid,
            params: vec!["v".to_string()],
            body: step_walk,
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
        });
        // Evidence constructors (core specification section 3): `refl`
        // proves `Eq x x`, `congr` lifts equality across a named total
        // function. Both are erased: they check as proof terms and never
        // become runtime code.
        let refl = ValueInfo {
            module: "<builtin>".to_string(),
            name: "refl".to_string(),
            scheme: Scheme {
            obligs: Vec::new(),
                params: Vec::new(),
                idx_params: vec![IxParam::Val("x".to_string(), Ty::Nat)],
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: Vec::new(),
                ctx: Vec::new(),
                ty: Ty::Prp(Box::new(PTerm::Eq(
                    Box::new(ITerm::Var("x".to_string())),
                    Box::new(ITerm::Var("x".to_string())),
                ))),
            },
            kind: ValueKind::ProofCtor,
        };
        self.add_value(refl, ast::Span { line: 0, col: 0 })
            .expect("builtin");
        let congr = ValueInfo {
            module: "<builtin>".to_string(),
            name: "congr".to_string(),
            scheme: Scheme {
            obligs: Vec::new(),
                params: Vec::new(),
                idx_params: vec![
                    IxParam::Val("f".to_string(), Ty::Nat),
                    IxParam::Val("a".to_string(), Ty::Nat),
                    IxParam::Val("b".to_string(), Ty::Nat),
                ],
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: Vec::new(),
                ctx: Vec::new(),
                ty: Ty::fun(
                    Ty::Prp(Box::new(PTerm::Eq(
                        Box::new(ITerm::Var("a".to_string())),
                        Box::new(ITerm::Var("b".to_string())),
                    ))),
                    Ty::Prp(Box::new(PTerm::Eq(
                        Box::new(ITerm::App(
                            Box::new(ITerm::Var("f".to_string())),
                            vec![ITerm::Var("a".to_string())],
                        )),
                        Box::new(ITerm::App(
                            Box::new(ITerm::Var("f".to_string())),
                            vec![ITerm::Var("b".to_string())],
                        )),
                    ))),
                ),
            },
            kind: ValueKind::ProofCtor,
        };
        self.add_value(congr, ast::Span { line: 0, col: 0 })
            .expect("builtin");
    }

    fn add_value(&mut self, v: ValueInfo, span: ast::Span) -> Result<()> {
        if self
            .values
            .get(&v.name)
            .map(|l| l.iter().any(|x| x.module == v.module))
            .unwrap_or(false)
        {
            return Err(Error::at(span, format!("duplicate definition of {}", v.name)));
        }
        self.values.entry(v.name.clone()).or_default().push(v.clone());
        self.qualified.insert((v.module.clone(), v.name.clone()), v);
        Ok(())
    }

    /// Decompose a class application like `Add (List a)`, `Functor List`
    /// or `Functor ((->) r)`: arguments check against the signature's
    /// parameter kinds, so a class over type constructors takes a
    /// partially applied type constructor.
    fn class_app_of(&self, ty: &ast::Type, vars: &[String]) -> Result<(String, Vec<Ty>)> {
        let mut args = Vec::new();
        let mut cur = ty;
        let sig = loop {
            match cur {
                ast::Type::App(f, a, _) => {
                    args.push((**a).clone());
                    cur = f;
                }
                ast::Type::Con(p, span) => {
                    let sig = self.lookup_sig(p)?;
                    args.reverse();
                    if args.len() != sig.params.len() {
                        return Err(Error::at(
                            *span,
                            format!(
                                "signature {} expects {} type arguments, got {}",
                                sig.name,
                                sig.params.len(),
                                args.len()
                            ),
                        ));
                    }
                    break sig;
                }
                other => {
                    return Err(Error::at(
                        other.span(),
                        "expected a signature application like `Add a`",
                    ));
                }
            }
        };
        let mut tys = Vec::new();
        for (i, a) in args.iter().enumerate() {
            tys.push(self.conv_kind(a, vars, sig.param_kinds.get(i))?);
        }
        Ok((sig.name, tys))
    }

    fn collect_module(
        &mut self,
        m: &ast::Module,
        prelude_name: &str,
        imports: &[String],
    ) -> Result<()> {
        let module = m.name.text.clone();
        self.current_module = module.clone();
        self.module_names.insert(module.clone());
        // Module header safety state (attribute annotations
        // specification section 10): `#[profile "jpl"]` and
        // `#[deny-warnings]` scope the whole module.
        self.module_safety
            .insert(module.clone(), validate_module_attrs(&m.attrs)?);
        // A parameter block (`module M (x :: T, ...) where`, module
        // parameters as module arguments): the module's values are reached
        // only through instantiation units. The parameter types convert
        // once every name is collected ([`Checker::finish_module_params`]).
        if !m.params.is_empty() {
            self.param_modules.insert(module.clone());
            self.mod_param_asts.insert(module.clone(), m.params.clone());
        }
        // An applied import (`import M (args) as N`) instantiates M as the
        // unit N: M's own names stay out of the unqualified scope (they are
        // reached through `N`, and a parameterized module's values reject
        // direct use).
        let mut applied: Vec<String> = Vec::new();
        let mut views: Vec<ImportView> = Vec::new();
        let mut stage_imports: Vec<String> = Vec::new();
        for d in &m.decls {
            if let ast::Decl::Import(i) = d {
                // Imports key on the full dotted name (`import
                // Category.Functor` resolves `module Category.Functor`);
                // the qualifier defaults to the path's last segment
                // (`Functor.bump`).
                let target = i.path.parts().join(".");
                let short = i
                    .path
                    .parts()
                    .last()
                    .copied()
                    .unwrap_or("")
                    .to_string();
                if !i.args.is_empty() {
                    applied.push(target.clone());
                    views.push(ImportView {
                        target,
                        alias: i.alias
                            .as_ref()
                            .map(|a| a.text.clone())
                            .unwrap_or_default(),
                        stage: i.stage,
                        applied: true,
                        alias_only: false,
                        members: None,
                    });
                    continue;
                }
                if i.stage {
                    stage_imports.push(target.clone());
                }
                views.push(ImportView {
                    alias: i
                        .alias
                        .as_ref()
                        .map(|a| a.text.clone())
                        .unwrap_or_else(|| short.clone()),
                    alias_only: i.alias.is_some() && i.members.is_empty(),
                    members: if i.members.is_empty() {
                        None
                    } else {
                        Some(
                            i.members
                                .iter()
                                .map(|m| {
                                    (
                                        m.as_name
                                            .as_ref()
                                            .map(|a| a.text.clone())
                                            .unwrap_or_else(|| m.name.text.clone()),
                                        m.name.text.clone(),
                                    )
                                })
                                .collect(),
                        )
                    },
                    target,
                    stage: i.stage,
                    applied: false,
                });
            }
        }
        // A module re-collected with generated declarations keeps its
        // import views: the synthetic module fragment has no imports.
        self.module_views.entry(module.clone()).or_insert(views);
        // Visibility: the module's own names, then its imports, then the
        // prelude (design note: compiled interface metadata, section 3).
        self.build_scope(&module, false);
        self.module_stage_imports.insert(module.clone(), stage_imports);
        self.module_imports.insert(
            module.clone(),
            imports.iter().filter(|i| !applied.contains(i)).cloned().collect(),
        );
        let mut schemes: HashMap<String, Scheme> = HashMap::new();
        let mut eqs: Vec<(String, ast::Equation)> = Vec::new();

        // Attributes are validated at their attachment position as the
        // position's owner is collected, so the annotated declaration is
        // collected as ordinary and macro-emitted attributes revalidate
        // through this same path (attribute annotations specification
        // sections 1.2 and 6).
        let mut flat: Vec<(Vec<ast::Attribute>, ast::Decl)> = Vec::new();
        for d in &m.decls {
            self.flatten_attributes(d, &mut flat)?;
        }

        for (decl_attrs, d) in &flat {
            // A parameterized module carries its parameters into every
            // definition, which the compile-time and evidence fragments do
            // not model in the bootstrap slice (module parameters as
            // module arguments).
            if self.param_modules.contains(&module) {
                match d {
                    ast::Decl::Generate { span: s, .. } => {
                        return Err(Error::at(
                            *s,
                            "a `generate do` block in a parameterized module is not supported in the bootstrap slice (compile-time code cannot take module parameters)",
                        ));
                    }
                    ast::Decl::Rewrite(r) => {
                        return Err(Error::at(
                            r.span,
                            "a rewrite rule in a parameterized module is not supported in the bootstrap slice (its evidence terms would need the module's parameters)",
                        ));
                    }
                    ast::Decl::Upgrade(u) => {
                        return Err(Error::at(
                            u.span,
                            "an `upgrade` declaration in a parameterized module is not supported in the bootstrap slice (its migration terms would need the module's parameters)",
                        ));
                    }
                    ast::Decl::Contract(c) => {
                        return Err(Error::at(
                            c.span,
                            "a `contract` declaration in a parameterized module is not supported in the bootstrap slice (its formulas would need the module's parameters)",
                        ));
                    }
                    ast::Decl::Import(i) if !i.args.is_empty() => {
                        return Err(Error::at(
                            i.span,
                            "an instantiation inside a parameterized module is not supported in the bootstrap slice (functor composition)",
                        ));
                    }
                    ast::Decl::ValueSig { modifier: Some(m), span: s, .. }
                    | ast::Decl::Equation {
                        modifier: Some(m),
                        eq: ast::Equation { span: s, .. },
                        ..
                    } if matches!(m.text.as_str(), "stage" | "macro") => {
                        return Err(Error::at(
                            *s,
                            "compile-time definitions (`stage`/`macro`) in a parameterized module are not supported in the bootstrap slice",
                        ));
                    }
                    _ => {}
                }
            }
            // The declarations with per-family effects validate inside
            // their own collection below; the rest take the ordinary
            // declaration position here (attribute annotations
            // specification section 1.1).
            match d {
                ast::Decl::Data(..)
                | ast::Decl::Record(..)
                | ast::Decl::Codata(..)
                | ast::Decl::ValueSig { .. }
                | ast::Decl::Equation { .. }
                | ast::Decl::Foreign(..) => {}
                _ => {
                    self.validate_attr_site(decl_attrs, AttrSite::Decl)?;
                }
            }
            match d {
                ast::Decl::NameSpliced { name_tm, .. } => {
                    return Err(Error::at(
                        name_tm.span(),
                        "a name splice (`$( e )`) names a generated declaration and is only meaningful inside a `quoteDecls` template",
                    ));
                }
                ast::Decl::Import(i) => {
                    let parts = i.path.parts();
                    let name = parts.join(".");
                    if name != prelude_name && !imports.iter().any(|x| x == &name) {
                        return Err(Error::at(
                            i.span,
                            format!(
                                "module {module} imports {name}, which the driver did not resolve"
                            ),
                        ));
                    }
                    // An applied import instantiates the target module as
                    // a unit of this module (module parameters as module
                    // arguments); its arguments check in the body pass.
                    if !i.args.is_empty() {
                        self.unit_decls.push((module.clone(), i.clone()));
                    }
                }
                ast::Decl::ImportData(d) => {
                    self.module_data_imports
                        .entry(module.clone())
                        .or_default()
                        .push(d.clone());
                }
                ast::Decl::Export(e) => {
                    self.module_exports
                        .entry(module.clone())
                        .or_default()
                        .extend(e.items.iter().cloned());
                }
                ast::Decl::Fixity(f) => {
                    let assoc = match f.assoc {
                        ast::Assoc::Left => "left",
                        ast::Assoc::Right => "right",
                        ast::Assoc::None => "none",
                    };
                    for op in &f.ops {
                        self.module_fixities.push((
                            module.clone(),
                            crate::interface::FixityExport {
                                op: op.text.clone(),
                                assoc: assoc.to_string(),
                                prec: f.prec as u64,
                            },
                        ));
                    }
                }
                ast::Decl::ValueSig { name, modifier, owned_attrs, ty, span } => {
                    // Function declarations carry the hint attributes of
                    // attribute annotations specification section 5 and
                    // the safety assertions of section 10.
                    let set = self.validate_attr_site(decl_attrs, AttrSite::FnDecl)?;
                    let fn_key = fn_id(&module, &name.text).0;
                    self.record_fn_hints(
                        &fn_key,
                        set.inline,
                        set.fn_cold,
                        *span,
                    )?;
                    self.record_fn_safety(
                        &fn_key,
                        &set.safety,
                        &format!("{}.{}", module, name.text),
                    )?;
                    let logical = match modifier.as_ref().map(|m| m.text.as_str()) {
                        None => None,
                        Some("spec") => Some("spec"),
                        Some("proof") => Some("proof"),
                        // `corec` marks a definition whose recursion must be
                        // productive (core language specification section 9);
                        // it is not part of the logical fragment.
                        Some("corec") => None,
                        // `stage` and `macro` mark compile-time definitions
                        // (staging, P9): executable generator code.
                        Some("stage") | Some("macro") => None,
                        Some(m) => {
                            return Err(Error::at(
                                modifier.as_ref().unwrap().span,
                                format!("modifier {m} is not supported in the bootstrap slice"),
                            ))
                        }
                    };
                    if schemes.contains_key(&name.text) {
                        return Err(Error::at(*span, format!("duplicate signature for {}", name.text)));
                    }
                    let s = self.conv_scheme(ty)?;
                    if logical == Some("proof") {
                        // A `proof` declaration constructs erased
                        // evidence: its type is a proposition or a chain
                        // of implications into one (core specification
                        // section 3).
                        let mut t = &s.ty;
                        loop {
                            match t {
                                Ty::Prp(_) => break,
                                Ty::Fun(_, _, _, b) | Ty::Pi { cod: b, .. } => t = b,
                                other => {
                                    return Err(Error::at(
                                        *span,
                                        format!(
                                            "a `proof` declaration states a proposition, but this one has type {other}"
                                        ),
                                    ))
                                }
                            }
                        }
                    }
                    if logical.is_some() {
                        self.logical.insert((module.clone(), name.text.clone()));
                    }
                    if modifier.as_ref().map(|m| m.text.as_str()) == Some("corec") {
                        self.corec.insert((module.clone(), name.text.clone()));
                    }
                    match modifier.as_ref().map(|m| m.text.as_str()) {
                        Some("stage") => {
                            self.staging.insert((module.clone(), name.text.clone()));
                        }
                        Some("macro") => {
                            self.staging.insert((module.clone(), name.text.clone()));
                            let key = (module.clone(), name.text.clone());
                            if self.macros.contains_key(&key) {
                                return Err(Error::at(
                                    *span,
                                    format!("duplicate macro signature for {}", name.text),
                                ));
                            }
                            for a in owned_attrs {
                                let attr = a.text.clone();
                                match self.macro_attr_names.get(&attr) {
                                    Some((owner, at)) => {
                                        return Err(Error::at(
                                            *span,
                                            format!(
                                                "attribute name `{attr}` is already declared by macro `{owner}` (declared at {}:{}); macro attribute namespaces cannot collide silently",
                                                at.line, at.col
                                            ),
                                        ))
                                    }
                                    None => {
                                        self.macro_attr_names
                                            .insert(attr, (name.text.clone(), *span));
                                    }
                                }
                            }
                            self.macros.insert(
                                key,
                                MacroInfo {
                                    arity: 0,
                                    owned_attrs: owned_attrs.iter().map(|a| a.text.clone()).collect(),
                                    span: *span,
                                },
                            );
                        }
                        _ => {}
                    }
                    // Register the value now: later declarations'
                    // signatures may reference it in index position (a
                    // proposition's `Eq` arguments name total
                    // definitions).
                    let key = (module.clone(), name.text.clone());
                    let kind = if self.macros.contains_key(&key) {
                        ValueKind::MacroFn
                    } else if self.staging.contains(&key) {
                        ValueKind::StageFn
                    } else {
                        ValueKind::TopFn
                    };
                    self.add_value(
                        ValueInfo {
                            module: module.clone(),
                            name: name.text.clone(),
                            scheme: s.clone(),
                            kind,
                        },
                        *span,
                    )?;
                    schemes.insert(name.text.clone(), s);
                }
                ast::Decl::Equation { modifier, eq } => {
                    // Function declarations carry the hint attributes of
                    // attribute annotations specification section 5 and
                    // the safety assertions of section 10; a weight on a
                    // clause sits between the name and its patterns
                    // (section 2).
                    let set = self.validate_attr_site(decl_attrs, AttrSite::FnDecl)?;
                    let fn_key = fn_id(&module, &eq.name.text).0;
                    self.record_fn_hints(
                        &fn_key,
                        set.inline,
                        set.fn_cold,
                        eq.span,
                    )?;
                    self.record_fn_safety(
                        &fn_key,
                        &set.safety,
                        &format!("{}.{}", module, eq.name.text),
                    )?;
                    match modifier.as_ref().map(|m| m.text.as_str()) {
                        None | Some("spec") | Some("proof") | Some("corec") | Some("stage")
                        | Some("macro") => {}
                        Some(m) => {
                            return Err(Error::at(
                                modifier.as_ref().unwrap().span,
                                format!("modifier {m} is not supported in the bootstrap slice"),
                            ))
                        }
                    }
                    if let Some(mod_) = modifier {
                        let marked = match mod_.text.as_str() {
                            "spec" | "proof" => {
                                self.logical.contains(&(module.clone(), eq.name.text.clone()))
                            }
                            "stage" | "macro" => {
                                self.staging.contains(&(module.clone(), eq.name.text.clone()))
                            }
                            _ => self.corec.contains(&(module.clone(), eq.name.text.clone())),
                        };
                        if !marked && schemes.get(&eq.name.text).is_some() {
                            return Err(Error::at(
                                mod_.span,
                                format!(
                                    "the signature of {} has no `{}` modifier",
                                    eq.name.text, mod_.text
                                ),
                            ));
                        }
                    }
                    if modifier.as_ref().map(|m| m.text.as_str()) == Some("macro") {
                        let key = (module.clone(), eq.name.text.clone());
                        match self.macros.get_mut(&key) {
                            Some(info) => {
                                if info.arity == 0 {
                                    info.arity = eq.pats.len();
                                } else if info.arity != eq.pats.len() {
                                    return Err(Error::at(
                                        eq.span,
                                        format!(
                                            "all equations of macro {} must take the same number of token-tree parameters",
                                            eq.name.text
                                        ),
                                    ));
                                }
                            }
                            None => {
                                return Err(Error::at(
                                    eq.span,
                                    format!(
                                        "the signature of macro {} is missing (a macro declares `macro {} :: ty`)",
                                        eq.name.text, eq.name.text
                                    ),
                                ));
                            }
                        }
                    }
                    eqs.push((eq.name.text.clone(), eq.clone()));
                }
                ast::Decl::Foreign(f) => {
                    let set = self.validate_attr_site(decl_attrs, AttrSite::ForeignDecl)?;
                    if f.abi != "ccall" {
                        return Err(Error::at(
                            f.span,
                            "only the ccall ABI is supported; the component ABI needs the WIT adapter (P14)",
                        ));
                    }
                    if let Some(op) = PrimOp::from_symbol(&f.symbol) {
                        // Compiler intrinsics have known signatures
                        // (decision D6).
                        if set.foreign_abi != ForeignAbi::default() {
                            return Err(Error::at(
                                f.span,
                                "`#[byval]` and `#[sret]` spell an aggregate boundary; a compiler intrinsic marshals no aggregate",
                            ));
                        }
                        if !f.is_import {
                            return Err(Error::at(f.span, "a compiler intrinsic cannot be exported"));
                        }
                        let s = self.conv_scheme(&f.ty)?;
                        if !s.params.is_empty() || !s.ctx.is_empty() {
                            return Err(Error::at(f.span, "primitives cannot be polymorphic or constrained"));
                        }
                        let expected = prim_scheme(op);
                        if s.ty != expected.ty {
                            return Err(Error::at(
                                f.span,
                                format!(
                                    "intrinsic {} has type {}, but is declared {}",
                                    f.symbol, expected.ty, s.ty
                                ),
                            ));
                        }
                        let rendered = render_scheme(&s);
                        self.add_value(
                            ValueInfo {
                                module: module.clone(),
                                name: f.name.text.clone(),
                                scheme: s,
                                kind: ValueKind::Prim(op),
                            },
                            f.span,
                        )?;
                        // The interface records the contract; the primitive is
                        // part of the assumption manifest (core specification,
                        // section 11).
                        self.module_foreign.push((
                            module.clone(),
                            crate::interface::ForeignExport {
                                symbol: f.name.text.clone(),
                                abi: "ccall".to_string(),
                                external: f.symbol.clone(),
                                ty: rendered,
                                effects: String::new(),
                                notes: "cinch intrinsic".to_string(),
                                kind: "intrinsic".to_string(),
                                params: Vec::new(),
                                result: crate::interface::ForeignParam {
                                    ty: String::new(),
                                    class: "unit".to_string(),
                                    role: "value".to_string(),
                                    allocator: String::new(),
                                    nullable: false,
                                    width: 0,
                                    align: 1,
                                    signed: false,
                                    offsets: Vec::new(),
                                },
                                deallocator: String::new(),
                            },
                        ));
                    } else {
                        // The intrinsic namespaces are reserved: a
                        // `cinch.prim.*`/`cinch.runtime.*` symbol
                        // that does not resolve names a retired or
                        // unknown intrinsic, not a C symbol (those names
                        // are not C identifiers anyway).
                        if f.symbol.starts_with("cinch.prim.")
                            || f.symbol.starts_with("cinch.runtime.")
                        {
                            return Err(Error::at(
                                f.span,
                                format!(
                                    "foreign symbol {} is not a cinch intrinsic; only cinch.prim.* and cinch.runtime.* intrinsics are supported in the bootstrap slice",
                                    f.symbol
                                ),
                            ));
                        }
                        // A real C symbol (foreign function interface
                        // specification sections 2-4). The envelope is
                        // checked here; the binding contract is built once
                        // every type's layout is known
                        // (`build_foreign_contracts`).
                        let s = self.conv_scheme(&f.ty)?;
                        if !s.params.is_empty() || !s.ctx.is_empty() {
                            return Err(Error::at(
                                f.span,
                                "a foreign binding cannot be polymorphic or constrained",
                            ));
                        }
                        self.check_foreign_envelope(f, &s.ty)?;
                        let unit = fn_id(&module, &f.name.text).0;
                        let arity = arity_of_arrows(&s.ty);
                        if f.is_import {
                            self.add_value(
                                ValueInfo {
                                    module: module.clone(),
                                    name: f.name.text.clone(),
                                    scheme: s.clone(),
                                    kind: ValueKind::Foreign { unit, arity },
                                },
                                f.span,
                            )?;
                        }
                        // The export's target check and the binding
                        // classification both wait for the deferred
                        // contract build (`build_foreign_contracts`).
                        self.pending_foreign
                            .push((module.clone(), f.clone(), s, set.foreign_abi));
                    }
                }
                ast::Decl::Prim(p) => {
                    // Hint attributes apply to the synthesized unit
                    // (primitive operations and inline target IR
                    // specification section 7); semantic attributes do
                    // not apply (the ABI comes from the declared type
                    // and the family contract).
                    let set = self.validate_attr_site(decl_attrs, AttrSite::FnDecl)?;
                    // The intrinsic namespaces stay reserved (D73): a
                    // `prim` must not claim an intrinsic family's name
                    // (primitive operations and inline target IR
                    // specification section 7).
                    let unit = fn_id(&module, &p.name.text).0;
                    self.record_fn_hints(&unit, set.inline, set.fn_cold, p.span)?;
                    if unit.starts_with("cinch.prim.") || unit.starts_with("cinch.runtime.") {
                        return Err(Error::at(
                            p.name.span,
                            format!(
                                "the intrinsic namespaces are reserved (decision D73): a `prim` must not claim the intrinsic name {} (primitive operations and inline target IR specification section 7)",
                                unit
                            ),
                        ));
                    }
                    let s = self.conv_scheme(&p.ty)?;
                    if !s.params.is_empty() || !s.ctx.is_empty() {
                        return Err(Error::at(
                            p.span,
                            "a prim cannot be polymorphic or constrained (primitive operations and inline target IR specification section 2)",
                        ));
                    }
                    // Signature discipline: the bodies receive exactly
                    // the declared arguments in the declared ABI and
                    // return the declared result shape (foreign function
                    // interface specification section 2: widths,
                    // signedness, convention).
                    let (arg_tys, _) = self.check_prim_envelope(p, &s.ty)?;
                    let mut immarg: Vec<usize> = Vec::new();
                    for b in &p.bodies {
                        let mut seen: Vec<usize> = Vec::new();
                        for (n, sp) in &b.immarg {
                            let n = *n as usize;
                            if n >= arg_tys.len() {
                                return Err(Error::at(
                                    *sp,
                                    format!(
                                        "`immarg` index {} is not a parameter of this prim (it takes {})",
                                        n,
                                        arg_tys.len()
                                    ),
                                ));
                            }
                            if seen.contains(&n) {
                                return Err(Error::at(
                                    *sp,
                                    format!("duplicate `immarg` index {}", n),
                                ));
                            }
                            seen.push(n);
                            if !immarg.contains(&n) {
                                immarg.push(n);
                            }
                        }
                    }
                    let arity = arity_of_arrows(&s.ty);
                    self.add_value(
                        ValueInfo {
                            module: module.clone(),
                            name: p.name.text.clone(),
                            scheme: s.clone(),
                            kind: ValueKind::PrimDecl { unit, arity, immarg },
                        },
                        p.span,
                    )?;
                    self.pending_prims.push((module.clone(), p.clone(), s));
                }
                ast::Decl::OpaqueType(o) => {
                    let _set = self.validate_attr_site(decl_attrs, AttrSite::TypeDecl)?;
                    // The kind is `Type` in this slice (foreign function
                    // interface specification section 2 declares opaque
                    // resources kinded `Type`).
                    match &o.kind {
                        ast::Type::Con(p, _) if p.parts() == ["Type"] => {}
                        other => {
                            return Err(Error::at(
                                other.span(),
                                "an opaque type's kind is `Type` in this slice (`opaque type T :: Type`)",
                            ))
                        }
                    }
                    if builtin_type_name(&o.name.text) {
                        return Err(Error::at(
                            o.name.span,
                            format!("{} is a builtin type and cannot be redefined", o.name.text),
                        ));
                    }
                    if self
                        .types
                        .get(&o.name.text)
                        .map(|l| l.iter().any(|t| t.module == module))
                        .unwrap_or(false)
                    {
                        return Err(Error::at(o.name.span, format!("duplicate type {}", o.name.text)));
                    }
                    let mut params: Vec<String> = Vec::new();
                    let mut param_kinds: Vec<ParamKind> = Vec::new();
                    for b in &o.params {
                        let k = match &b.ann {
                            Some(ann) => self.binder_kind(ann, &params)?,
                            None => ParamKind::Type,
                        };
                        if !matches!(k, ParamKind::Type) {
                            return Err(Error::at(
                                b.name.span,
                                "an opaque type's parameters are type parameters in this slice",
                            ));
                        }
                        params.push(b.name.text.clone());
                        param_kinds.push(k);
                    }
                    // Abstract: no constructors and no fields, so values
                    // arise only from foreign calls and there is no
                    // layout projection (foreign function interface
                    // specification section 2).
                    let tk = TypeKind {
                        module: module.clone(),
                        name: o.name.text.clone(),
                        arity: params.len(),
                        param_kinds: param_kinds.clone(),
                        ctors: Vec::new(),
                        fields: None,
                        field_erased: Vec::new(),
                        opaque: true,
                        refine: None,
                    };
                    self.types.entry(o.name.text.clone()).or_default().push(tk.clone());
                    self.qualified_types
                        .insert((module.clone(), o.name.text.clone()), tk);
                }
                ast::Decl::Data(data) => {
                    let set = self.validate_attr_site(decl_attrs, AttrSite::TypeDecl)?;
                    if Scalar::parse(&data.name.text).is_some()
                        || matches!(
                            data.name.text.as_str(),
                            "Nat"
                                | "Int"
                                | "Bool"
                                | "Unit"
                                | "Bytes"
                                | "Sym"
                                | "Code"
                                | "Syntax"
                                | "Declarations"
                                | "Name"
                                | "Secret"
                                | "IoCap"
                                | "FileReadCap"
                                | "FileWriteCap"
                                | "ProcCap"
                        )
                    {
                        return Err(Error::at(
                            data.name.span,
                            format!("{} is a builtin primitive type and cannot be redefined", data.name.text),
                        ));
                    }
                    if self
                        .types
                        .get(&data.name.text)
                        .map(|l| l.iter().any(|t| t.module == module))
                        .unwrap_or(false)
                    {
                        return Err(Error::at(data.name.span, format!("duplicate type {}", data.name.text)));
                    }
                    // Parameters: a kind annotation classifies each one as
                    // a type parameter or an index value (`data Vec (n ::
                    // Nat) (a :: Type)`); an un-annotated parameter is a
                    // type parameter (dependent core kernel design,
                    // section 5).
                    let mut params: Vec<String> = Vec::new();
                    let mut param_kinds: Vec<ParamKind> = Vec::new();
                    for b in &data.params {
                        let k = match &b.ann {
                            Some(ann) => self.binder_kind(ann, &params)?,
                            None => ParamKind::Type,
                        };
                        params.push(b.name.text.clone());
                        param_kinds.push(k);
                    }
                    let mut ctors: Vec<(String, usize)> = Vec::new();
                    // Register the type before converting constructor types,
                    // which may mention the type being declared.
                    self.types.entry(data.name.text.clone()).or_default().push(TypeKind {
                        module: module.clone(),
                        name: data.name.text.clone(),
                        arity: params.len(),
                        param_kinds: param_kinds.clone(),
                        ctors: Vec::new(),
                        fields: None,
                        field_erased: Vec::new(), opaque: false, refine: None,
                    });
                    for (tag, c) in data.ctors.iter().enumerate() {
                        self.validate_attr_site(&c.attrs, AttrSite::Ctor)?;
                        self.kind_constraints.borrow_mut().clear();
                        // Constructor type: optional extra `forall` binders
                        // (constructor-local index binders, suppressed in
                        // the surface and bound explicitly here), then a
                        // Pi/Fun spine into the declared type applied to
                        // its parameters; index parameters may be computed
                        // (`Vec (k + 1) a`).
                        let mut local_idx: Vec<IxParam> = Vec::new();
                        let mut vars = params.clone();
                        let mut rest = &c.ty;
                        while let ast::Type::Forall(binders, body, _) = rest {
                            for b in binders {
                                if vars.contains(&b.name.text) {
                                    return Err(Error::at(
                                        b.name.span,
                                        format!("duplicate binder {}", b.name.text),
                                    ));
                                }
                                let k = match &b.ann {
                                    Some(ann) => self.binder_kind(ann, &vars)?,
                                    None => ParamKind::Val(Ty::Nat),
                                };
                                vars.push(b.name.text.clone());
                                match k {
                                    ParamKind::Type | ParamKind::Con(_) => {
                                        return Err(Error::at(
                                            b.name.span,
                                            "constructor-local type parameters are not supported in this slice; move the parameter to the family",
                                        ))
                                    }
                                    ParamKind::Val(t) => {
                                        local_idx.push(IxParam::Val(b.name.text.clone(), t))
                                    }
                                    ParamKind::Prop => {
                                        local_idx.push(IxParam::Prop(b.name.text.clone()))
                                    }
                                }
                            }
                            rest = body;
                        }
                        let mut fields: Vec<(String, Ty, Mult)> = Vec::new();
                        let mut cur = rest;
                        let result = loop {
                            match cur {
                                ast::Type::Fun { arg, res, mult, eff, span } => {
                                    if let Some(e) = eff {
                                        return Err(Error::at(
                                            e.span,
                                            "effect rows on constructor arguments are not supported in the bootstrap slice",
                                        ));
                                    }
                                    let t = self.conv_type(arg, &vars)?;
                                    let m = match mult {
                                        Some(n) => Mult::parse(&n.text).ok_or_else(|| {
                                            Error::at(n.span, format!("unknown multiplicity {}", n.text))
                                        })?,
                                        None => default_mult(&t),
                                    };
                                    let _ = span;
                                    let bname = format!("$f{}", fields.len());
                                    fields.push((bname, t, m));
                                    cur = res;
                                }
                                ast::Type::DepFun(name, ann, mult, eff, res, span) => {
                                    if let Some(e) = eff {
                                        return Err(Error::at(
                                            e.span,
                                            "effect rows on constructor arguments are not supported in the bootstrap slice",
                                        ));
                                    }
                                    if vars.contains(&name.text) {
                                        return Err(Error::at(
                                            name.span,
                                            format!("duplicate binder {}", name.text),
                                        ));
                                    }
                                    let t = self.conv_type(ann, &vars)?;
                                    let m = match mult {
                                        Some(n) => Mult::parse(&n.text).ok_or_else(|| {
                                            Error::at(n.span, format!("unknown multiplicity {}", n.text))
                                        })?,
                                        None => default_mult(&t),
                                    };
                                    let _ = span;
                                    fields.push((name.text.clone(), t, m));
                                    vars.push(name.text.clone());
                                    cur = res;
                                }
                                other => break other,
                            }
                        };
                        let res_ty = self.conv_type(result, &vars)?;
                        // The result is the declared family applied to its
                        // parameters: type and proposition parameters must
                        // be the declared ones, index parameters may be
                        // computed.
                        let mut res_args: Vec<Ty> = Vec::new();
                        match &res_ty {
                            Ty::Adt(n, args)
                                if n == &data.name.text && args.len() == params.len() =>
                            {
                                res_args = args.clone();
                                for (i, (a, k)) in args.iter().zip(&param_kinds).enumerate() {
                                    let expected = match k {
                                        ParamKind::Type | ParamKind::Con(_) => {
                                            Ty::Param(params[i].clone())
                                        }
                                        ParamKind::Val(_) => continue,
                                        ParamKind::Prop => {
                                            Ty::Prp(Box::new(PTerm::Var(params[i].clone())))
                                        }
                                    };
                                    if a != &expected {
                                        return Err(Error::at(
                                            c.name.span,
                                            format!(
                                                "constructor {} must return {} applied to its parameters; computed indices are only allowed for index parameters (as in `Vec (k + 1) a`)",
                                                c.name.text, data.name.text
                                            ),
                                        ));
                                    }
                                }
                            }
                            other => {
                                return Err(Error::at(
                                    c.name.span,
                                    format!(
                                        "constructor {} must return {} applied to its parameters, got {other}",
                                        c.name.text, data.name.text
                                    ),
                                ))
                            }
                        }
                        // Positivity (core specification section 2; the
                        // positivity discipline applies to ordinary
                        // `data` too, roadmap gap A1).
                        for (fname, fty, _) in &fields {
                            check_positivity(&data.name.text, fname, fty, &c.name, c.ty.span())?;
                        }
                        let field_tys: Vec<Ty> =
                            fields.iter().map(|(_, t, _)| t.clone()).collect();
                        let field_erased: Vec<bool> =
                            field_tys.iter().map(|t| erased_ty(t)).collect();
                        let runtime_arity =
                            field_erased.iter().filter(|e| !**e).count();
                        let mut all = field_tys.clone();
                        all.extend(res_args.iter().cloned());
                        let mut type_params: Vec<String> = Vec::new();
                        let mut idx_params: Vec<IxParam> = Vec::new();
                        for (p, k) in params.iter().zip(&param_kinds) {
                            if !all.iter().any(|t| ty_mentions(t, p)) {
                                continue;
                            }
                            match k {
                                ParamKind::Type | ParamKind::Con(_) => type_params.push(p.clone()),
                                ParamKind::Val(t) => {
                                    idx_params.push(IxParam::Val(p.clone(), t.clone()))
                                }
                                ParamKind::Prop => {
                                    idx_params.push(IxParam::Prop(p.clone()))
                                }
                            }
                        }
                        idx_params.extend(local_idx.iter().cloned());
                        let mut scheme_ty =
                            Ty::Adt(data.name.text.clone(), res_args.clone());
                        for (b, t, m) in fields.iter().rev() {
                            let rest = scheme_ty;
                            scheme_ty = if rest.mentions_idx(b) {
                                Ty::pi(b.clone(), t.clone(), *m, EffRow::pure(), rest)
                            } else {
                                Ty::fun_mult(t.clone(), *m, EffRow::pure(), rest)
                            };
                        }
                        let scheme = Scheme {
            obligs: Vec::new(),
                            params: type_params,
                            idx_params,
                            val_params: Vec::new(),
                            prop_params: Vec::new(),
                            row_params: Vec::new(),
                            region_params: Vec::new(),
                            ctx: Vec::new(),
                            ty: scheme_ty,
                        };
                        self.add_value(
                            ValueInfo {
                                module: module.clone(),
                                name: c.name.text.clone(),
                                scheme,
                                kind: ValueKind::Ctor {
                                    tag,
                                    arity: fields.len(),
                                    runtime_arity,
                                    field_tys,
                                    field_erased,
                                    field_binders: fields.iter().map(|(b, _, _)| b.clone()).collect(),
                                    ty_params: params.clone(),
                                    local_idx,
                                    res_args,
                                    refine: None,
                                },
                            },
                            c.name.span,
                        )?;
                        ctors.push((c.name.text.clone(), tag));
                    }
                    // A representation attribute compiles into layout
                    // facts once every type name is known (attribute
                    // annotations specification section 4.3).
                    if let Some(repr) = set.repr.clone() {
                        let mut fields: Vec<(String, Ty, bool, ReprSpec)> = Vec::new();
                        if data.ctors.len() == 1 {
                            let cname = data.ctors[0].name.text.clone();
                            if let Some(v) = self.qualified.get(&(module.clone(), cname.clone())) {
                                if let ValueKind::Ctor {
                                    field_tys,
                                    field_erased,
                                    field_binders,
                                    ..
                                } = &v.kind
                                {
                                    for (i, t) in field_tys.iter().enumerate() {
                                        fields.push((
                                            field_binders[i].clone(),
                                            t.clone(),
                                            field_erased[i],
                                            ReprSpec::default(),
                                        ));
                                    }
                                }
                            }
                        }
                        self.layout_recipes.insert(
                            (module.clone(), data.name.text.clone()),
                            LayoutRecipe {
                                module: module.clone(),
                                name: data.name.text.clone(),
                                repr,
                                fields,
                                ctor_count: data.ctors.len(),
                                record: false,
                                span: data.name.span,
                            },
                        );
                    }
                    let entry = self
                        .types
                        .get_mut(&data.name.text)
                        .and_then(|ts| ts.iter_mut().find(|t| t.module == module))
                        .expect("type registered above");
                    entry.ctors = ctors;
                }
                ast::Decl::Refine(r) => {
                    // `type R params = refine (x :: T) with p` (refinement
                    // types design): the runtime representation IS the
                    // container; the witness is `Prop`-erased (D31).
                    let _set = self.validate_attr_site(decl_attrs, AttrSite::TypeDecl)?;
                    if Scalar::parse(&r.name.text).is_some()
                        || matches!(
                            r.name.text.as_str(),
                            "Nat"
                                | "Int"
                                | "Bool"
                                | "Unit"
                                | "Bytes"
                                | "Sym"
                                | "Code"
                                | "Syntax"
                                | "Declarations"
                                | "Name"
                                | "Secret"
                                | "IoCap"
                                | "FileReadCap"
                                | "FileWriteCap"
                                | "ProcCap"
                        )
                    {
                        return Err(Error::at(
                            r.name.span,
                            format!("{} is a builtin primitive type and cannot be redefined", r.name.text),
                        ));
                    }
                    if self
                        .types
                        .get(&r.name.text)
                        .map(|l| l.iter().any(|t| t.module == module))
                        .unwrap_or(false)
                    {
                        return Err(Error::at(r.name.span, format!("duplicate type {}", r.name.text)));
                    }
                    let mut params: Vec<String> = Vec::new();
                    let mut param_kinds: Vec<ParamKind> = Vec::new();
                    for b in &r.params {
                        let k = match &b.ann {
                            Some(ann) => self.binder_kind(ann, &params)?,
                            None => ParamKind::Type,
                        };
                        if !matches!(k, ParamKind::Type | ParamKind::Con(_)) {
                            return Err(Error::at(
                                b.name.span,
                                "a refinement's parameters are type parameters (the container states any index parameters)",
                            ));
                        }
                        params.push(b.name.text.clone());
                        param_kinds.push(k);
                    }
                    self.kind_constraints.borrow_mut().clear();
                    let base = self.conv_type(&r.base, &params)?;
                    if !matches!(
                        self.uni.zonk(&base),
                        Ty::Adt(..)
                            | Ty::Int
                            | Ty::Nat
                            | Ty::Bool
                            | Ty::Unit
                            | Ty::Bytes
                            | Ty::Sym
                            | Ty::Prim(_)
                    ) {
                        return Err(Error::at(
                            r.base.span(),
                            format!(
                                "a refinement's container is a named type (the runtime representation IS the container), not {base}"
                            ),
                        ));
                    }
                    let mut vars = params.clone();
                    if vars.contains(&r.binder.text) {
                        return Err(Error::at(
                            r.binder.span,
                            format!("duplicate binder {}", r.binder.text),
                        ));
                    }
                    vars.push(r.binder.text.clone());
                    let pred = self.conv_prop(&r.pred, &vars)?;
                    if !pred.mentions(&r.binder.text) {
                        return Err(Error::at(
                            r.pred.span(),
                            format!(
                                "a refinement's predicate must mention the refined binder `{}`",
                                r.binder.text
                            ),
                        ));
                    }
                    self.check_pred_admissible(&pred, &r.binder.text, &self.uni.zonk(&base), r.pred.span())?;
                    // Register the type before its transparent
                    // constructor (whose scheme mentions the type).
                    self.types.entry(r.name.text.clone()).or_default().push(TypeKind {
                        module: module.clone(),
                        name: r.name.text.clone(),
                        arity: params.len(),
                        param_kinds: param_kinds.clone(),
                        ctors: Vec::new(),
                        fields: None,
                        field_erased: Vec::new(),
                        opaque: false,
                        refine: Some(RefineInfo {
                            base: base.clone(),
                            binder: r.binder.text.clone(),
                            pred: pred.clone(),
                            params: params.clone(),
                        }),
                    });
                    // The transparent constructor, exactly `Fin`-shaped:
                    // `R x w` takes the refined binder's value and the
                    // erased witness `w :: p`, and its runtime form is the
                    // container value itself (no block, no wrapper).
                    let res_args: Vec<Ty> = params
                        .iter()
                        .zip(&param_kinds)
                        .map(|(p, k)| match k {
                            ParamKind::Type | ParamKind::Con(_) => Ty::Param(p.clone()),
                            _ => unreachable!("refinement parameters are types"),
                        })
                        .collect();
                    let field_tys: Vec<Ty> = vec![
                        base.clone(),
                        Ty::Prp(Box::new(pred.clone())),
                    ];
                    let field_binders: Vec<String> =
                        vec![r.binder.text.clone(), "$w".to_string()];
                    let field_erased: Vec<bool> = vec![false, true];
                    let runtime_arity = 1;
                    let mut type_params: Vec<String> = Vec::new();
                    let mut idx_params: Vec<IxParam> = Vec::new();
                    for (p, k) in params.iter().zip(&param_kinds) {
                        if !field_tys.iter().any(|t| ty_mentions(t, p)) {
                            continue;
                        }
                        match k {
                            ParamKind::Type | ParamKind::Con(_) => type_params.push(p.clone()),
                            ParamKind::Val(t) => {
                                idx_params.push(IxParam::Val(p.clone(), t.clone()))
                            }
                            ParamKind::Prop => idx_params.push(IxParam::Prop(p.clone())),
                        }
                    }
                    let mut scheme_ty =
                        Ty::Adt(r.name.text.clone(), res_args.clone());
                    for (i, (b, t)) in field_binders.iter().zip(&field_tys).enumerate().rev() {
                        let rest = scheme_ty;
                        let m = default_mult(t);
                        scheme_ty = if rest.mentions_idx(b) {
                            let _ = i;
                            Ty::pi(b.clone(), t.clone(), m, EffRow::pure(), rest)
                        } else {
                            Ty::fun_mult(t.clone(), m, EffRow::pure(), rest)
                        };
                    }
                    let scheme = Scheme {
            obligs: Vec::new(),
                        params: type_params,
                        idx_params,
                        val_params: Vec::new(),
                        prop_params: Vec::new(),
                        row_params: Vec::new(),
                        region_params: Vec::new(),
                        ctx: Vec::new(),
                        ty: scheme_ty,
                    };
                    self.add_value(
                        ValueInfo {
                            module: module.clone(),
                            name: r.name.text.clone(),
                            scheme,
                            kind: ValueKind::Ctor {
                                tag: 0,
                                arity: field_tys.len(),
                                runtime_arity,
                                field_tys,
                                field_erased,
                                field_binders,
                                ty_params: params.clone(),
                                local_idx: Vec::new(),
                                res_args,
                                refine: Some(r.name.text.clone()),
                            },
                        },
                        r.name.span,
                    )?;
                    // The constructor's unit is the identity on the
                    // container value (the erased witness keeps it from
                    // being a block builder).
                    self.fns.push(FnDef {
                        id: fn_id(&module, &r.name.text),
                        params: vec![r.binder.text.clone()],
                        body: crate::core::Core::Var(LocalId(0)),
                        prim: None,
                        ctor: None,
                        hints: crate::core::FnHints::default(),
                        meta: crate::core::FnMeta::default(),
                    });
                }
                ast::Decl::Record(rec) => {
                    let set = self.validate_attr_site(decl_attrs, AttrSite::TypeDecl)?;
                    if self
                        .types
                        .get(&rec.name.text)
                        .map(|l| l.iter().any(|t| t.module == module))
                        .unwrap_or(false)
                    {
                        return Err(Error::at(rec.name.span, format!("duplicate type {}", rec.name.text)));
                    }
                    let mut params: Vec<String> = Vec::new();
                    let mut param_kinds: Vec<ParamKind> = Vec::new();
                    for b in &rec.params {
                        let k = match &b.ann {
                            Some(ann) => self.binder_kind(ann, &params)?,
                            None => ParamKind::Type,
                        };
                        params.push(b.name.text.clone());
                        param_kinds.push(k);
                    }
                    let mut field_names: Vec<String> = Vec::new();
                    let mut field_erased: Vec<bool> = Vec::new();
                    let mut field_reprs: Vec<ReprSpec> = Vec::new();
                    for f in &rec.fields {
                        // A record field may carry the layout modifiers
                        // of attribute annotations specification section
                        // 4, on fields of `repr(C)` records.
                        let fset = self.validate_attr_site(&f.attrs, AttrSite::Field)?;
                        if let Some(fr) = fset.repr {
                            if !fr.field_only() {
                                return Err(Error::at(
                                    f.span,
                                    format!(
                                        "a field takes `repr(align(N))` or `repr(packed)`/`repr(packed(N))`, not `{}` (attribute annotations specification section 4.1)",
                                        fr.spelled
                                    ),
                                ));
                            }
                            if !set.repr.as_ref().map(|r| r.c_layout).unwrap_or(false) {
                                return Err(Error::at(
                                    f.span,
                                    "a field's representation applies to fields of a `repr(C)` record (attribute annotations specification section 9's open decision admits them there)",
                                ));
                            }
                            field_reprs.push(fr);
                        } else {
                            field_reprs.push(ReprSpec::default());
                        }
                        if field_names.iter().any(|n| n == &f.name.text) {
                            return Err(Error::at(
                                f.name.span,
                                format!("duplicate record field {}", f.name.text),
                            ));
                        }
                        field_names.push(f.name.text.clone());
                        field_erased.push(f.erased);
                    }
                    // Register the type before converting field types, which
                    // may mention the record being declared.
                    self.types.entry(rec.name.text.clone()).or_default().push(TypeKind {
                        module: module.clone(),
                        name: rec.name.text.clone(),
                        arity: params.len(),
                        param_kinds: param_kinds.clone(),
                        ctors: Vec::new(),
                        fields: Some(field_names.clone()),
                        field_erased: field_erased.clone(), opaque: false, refine: None,
                    });
                    // A record's fields see the parameters and the earlier
                    // fields: `erased bound :: length bytes <= limit`
                    // mentions both (core specification section 5).
                    let mut vars = params.clone();
                    let mut field_tys = Vec::new();
                    for f in &rec.fields {
                        let t = self.conv_type(&f.ty, &vars)?;
                        if f.erased && !matches!(t, Ty::Prp(_)) {
                            return Err(Error::at(
                                f.name.span,
                                format!(
                                    "an `erased` field is evidence: its type must be a proposition, but `{}` has type {t}",
                                    f.name.text
                                ),
                            ));
                        }
                        field_tys.push(t);
                        vars.push(f.name.text.clone());
                    }
                    // Erasure (core specification section 3): a
                    // proposition field is evidence whether or not it is
                    // marked `erased`; it occupies no runtime space.
                    let field_erased: Vec<bool> = field_tys
                        .iter()
                        .enumerate()
                        .map(|(i, t)| field_erased[i] || erased_ty(t))
                        .collect();
                    let runtime_arity = field_erased.iter().filter(|e| !**e).count();
                    // A representation attribute compiles into layout
                    // facts once every type name is known (attribute
                    // annotations specification section 4.3).
                    if let Some(repr) = set.repr.clone() {
                        let fields: Vec<(String, Ty, bool, ReprSpec)> = field_names
                            .iter()
                            .zip(field_tys.iter())
                            .zip(field_erased.iter().zip(field_reprs.iter()))
                            .map(|((n, t), (e, fr))| {
                                (n.clone(), t.clone(), *e, fr.clone())
                            })
                            .collect();
                        self.layout_recipes.insert(
                            (module.clone(), rec.name.text.clone()),
                            LayoutRecipe {
                                module: module.clone(),
                                name: rec.name.text.clone(),
                                repr,
                                fields,
                                ctor_count: 1,
                                record: true,
                                span: rec.name.span,
                            },
                        );
                    }
                    let res_args: Vec<Ty> = params
                        .iter()
                        .zip(&param_kinds)
                        .map(|(p, k)| match k {
                            ParamKind::Type | ParamKind::Con(_) => Ty::Param(p.clone()),
                            ParamKind::Val(_) => Ty::Idx(Box::new(ITerm::Var(p.clone()))),
                            ParamKind::Prop => Ty::Prp(Box::new(PTerm::Var(p.clone()))),
                        })
                        .collect();
                    let mut all = field_tys.clone();
                    all.extend(res_args.iter().cloned());
                    let mut type_params: Vec<String> = Vec::new();
                    let mut idx_params: Vec<IxParam> = Vec::new();
                    for (p, k) in params.iter().zip(&param_kinds) {
                        if !all.iter().any(|t| ty_mentions(t, p)) {
                            continue;
                        }
                        match k {
                            ParamKind::Type | ParamKind::Con(_) => type_params.push(p.clone()),
                            ParamKind::Val(t) => {
                                idx_params.push(IxParam::Val(p.clone(), t.clone()))
                            }
                            ParamKind::Prop => idx_params.push(IxParam::Prop(p.clone())),
                        }
                    }
                    let res_ty = Ty::Adt(rec.name.text.clone(), res_args.clone());
                    let mut scheme_ty = res_ty.clone();
                    for (i, t) in field_tys.iter().enumerate().rev() {
                        let b = field_names[i].clone();
                        let rest = scheme_ty;
                        scheme_ty = if rest.mentions_idx(&b) {
                            Ty::pi(b, t.clone(), default_mult(t), EffRow::pure(), rest)
                        } else {
                            Ty::fun_mult(t.clone(), default_mult(t), EffRow::pure(), rest)
                        };
                    }
                    let scheme = Scheme {
            obligs: Vec::new(),
                        params: type_params,
                        idx_params,
                        val_params: Vec::new(),
                        prop_params: Vec::new(),
                        row_params: Vec::new(),
                        region_params: Vec::new(),
                        ctx: Vec::new(),
                        ty: scheme_ty,
                    };
                    // A record is a single-constructor product whose
                    // constructor carries its own name.
                    self.add_value(
                        ValueInfo {
                            module: module.clone(),
                            name: rec.name.text.clone(),
                            scheme,
                            kind: ValueKind::Ctor {
                                tag: 0,
                                arity: field_tys.len(),
                                runtime_arity,
                                field_binders: field_names.clone(),
                                field_tys,
                                field_erased,
                                ty_params: params.clone(),
                                local_idx: Vec::new(),
                                res_args,
                                refine: None,
                            },
                        },
                        rec.name.span,
                    )?;
                    let entry = self
                        .types
                        .get_mut(&rec.name.text)
                        .and_then(|ts| ts.iter_mut().find(|t| t.module == module))
                        .expect("type registered above");
                    entry.ctors = vec![(rec.name.text.clone(), 0)];
                }
                ast::Decl::Codata(rec) => {
                    // A codata type is not a `data` or `record`
                    // declaration: no representation attribute applies
                    // (attribute annotations specification section 4.1).
                    let set = self.validate_attr_site(decl_attrs, AttrSite::TypeDecl)?;
                    if let Some(fr) = &set.repr {
                        return Err(Error::at(
                            rec.name.span,
                            format!(
                                "`#[repr({})]` attaches to a `data` or `record` declaration (attribute annotations specification section 4.1)",
                                fr.spelled
                            ),
                        ));
                    }
                    // A codata type is a record of delayed observations
                    // (codata and guardedness design, section 2): it
                    // registers like a record, but construction is the
                    // observed form and elimination is observation only.
                    if Scalar::parse(&rec.name.text).is_some()
                        || matches!(
                            rec.name.text.as_str(),
                            "Nat" | "Int" | "Bool" | "Unit" | "Bytes" | "Sym"
                                | "Thunk" | "Lazy" | "Later" | "Ref" | "Mut" | "Io"
                                | "Type" | "Prop"
                        )
                    {
                        return Err(Error::at(
                            rec.name.span,
                            format!("{} is a builtin type former and cannot be redefined", rec.name.text),
                        ));
                    }
                    if self
                        .types
                        .get(&rec.name.text)
                        .map(|l| l.iter().any(|t| t.module == module))
                        .unwrap_or(false)
                    {
                        return Err(Error::at(rec.name.span, format!("duplicate type {}", rec.name.text)));
                    }
                    let mut params: Vec<String> = Vec::new();
                    let mut param_kinds: Vec<ParamKind> = Vec::new();
                    for b in &rec.params {
                        let k = match &b.ann {
                            Some(ann) => self.binder_kind(ann, &params)?,
                            None => ParamKind::Type,
                        };
                        params.push(b.name.text.clone());
                        param_kinds.push(k);
                    }
                    let mut obs_names: Vec<String> = Vec::new();
                    for f in &rec.fields {
                        let fset = self.validate_attr_site(&f.attrs, AttrSite::Field)?;
                        if let Some(fr) = &fset.repr {
                            return Err(Error::at(
                                f.span,
                                format!(
                                    "a codata observation takes no representation (found `#[repr({})]`; attribute annotations specification section 4.1)",
                                    fr.spelled
                                ),
                            ));
                        }
                        if obs_names.iter().any(|n| n == &f.name.text) {
                            return Err(Error::at(
                                f.name.span,
                                format!("duplicate observation {}", f.name.text),
                            ));
                        }
                        obs_names.push(f.name.text.clone());
                    }
                    // Register the type before converting observation
                    // types, which may mention the type being declared
                    // (`tail :: Later (Stream a)`).
                    self.types.entry(rec.name.text.clone()).or_default().push(TypeKind {
                        module: module.clone(),
                        name: rec.name.text.clone(),
                        arity: params.len(),
                        param_kinds: param_kinds.clone(),
                        ctors: Vec::new(),
                        fields: Some(obs_names.clone()),
                        field_erased: Vec::new(), opaque: false, refine: None,
                    });
                    let mut vars = params.clone();
                    let mut obs_tys = Vec::new();
                    for f in &rec.fields {
                        let t = self.conv_type(&f.ty, &vars)?;
                        check_positivity(&rec.name.text, &f.name.text, &t, &rec.name, f.span)?;
                        obs_tys.push(t);
                        vars.push(f.name.text.clone());
                    }
                    let field_erased: Vec<bool> = obs_tys.iter().map(|t| erased_ty(t)).collect();
                    let runtime_arity = field_erased.iter().filter(|e| !**e).count();
                    let res_args: Vec<Ty> = params
                        .iter()
                        .zip(&param_kinds)
                        .map(|(p, k)| match k {
                            ParamKind::Type | ParamKind::Con(_) => Ty::Param(p.clone()),
                            ParamKind::Val(_) => Ty::Idx(Box::new(ITerm::Var(p.clone()))),
                            ParamKind::Prop => Ty::Prp(Box::new(PTerm::Var(p.clone()))),
                        })
                        .collect();
                    let mut all = obs_tys.clone();
                    all.extend(res_args.iter().cloned());
                    let mut type_params: Vec<String> = Vec::new();
                    let mut idx_params: Vec<IxParam> = Vec::new();
                    for (p, k) in params.iter().zip(&param_kinds) {
                        if !all.iter().any(|t| ty_mentions(t, p)) {
                            continue;
                        }
                        match k {
                            ParamKind::Type | ParamKind::Con(_) => type_params.push(p.clone()),
                            ParamKind::Val(t) => {
                                idx_params.push(IxParam::Val(p.clone(), t.clone()))
                            }
                            ParamKind::Prop => idx_params.push(IxParam::Prop(p.clone())),
                        }
                    }
                    let res_ty = Ty::Adt(rec.name.text.clone(), res_args.clone());
                    let mut scheme_ty = res_ty.clone();
                    for (i, t) in obs_tys.iter().enumerate().rev() {
                        let b = obs_names[i].clone();
                        let rest = scheme_ty;
                        scheme_ty = if rest.mentions_idx(&b) {
                            Ty::pi(b, t.clone(), default_mult(t), EffRow::pure(), rest)
                        } else {
                            Ty::fun_mult(t.clone(), default_mult(t), EffRow::pure(), rest)
                        };
                    }
                    let scheme = Scheme {
            obligs: Vec::new(),
                        params: type_params,
                        idx_params,
                        val_params: Vec::new(),
                        prop_params: Vec::new(),
                        row_params: Vec::new(),
                        region_params: Vec::new(),
                        ctx: Vec::new(),
                        ty: scheme_ty,
                    };
                    // The observations' closures occupy one block word
                    // each; the constructor exists for the block
                    // allocation and is never a callable value.
                    self.add_value(
                        ValueInfo {
                            module: module.clone(),
                            name: rec.name.text.clone(),
                            scheme,
                            kind: ValueKind::Ctor {
                                tag: 0,
                                arity: obs_tys.len(),
                                runtime_arity,
                                field_binders: obs_names.clone(),
                                field_tys: obs_tys,
                                field_erased: field_erased.clone(),
                                ty_params: params.clone(),
                                local_idx: Vec::new(),
                                res_args,
                                refine: None,
                            },
                        },
                        rec.name.span,
                    )?;
                    let entry = self
                        .types
                        .get_mut(&rec.name.text)
                        .and_then(|ts| ts.iter_mut().find(|t| t.module == module))
                        .expect("type registered above");
                    entry.ctors = vec![(rec.name.text.clone(), 0)];
                    entry.field_erased = field_erased;
                    self.codata_types.insert((module.clone(), rec.name.text.clone()));
                }
                ast::Decl::Signature(sig) => {
                    if matches!(sig.name.text.as_str(), "Copy" | "Share") {
                        return Err(Error::at(
                            sig.name.span,
                            "Copy is a compiler-known class with structural instances (ownership and usage design, section 3); it cannot be declared",
                        ));
                    }
                    if self
                        .sigs
                        .get(&sig.name.text)
                        .map(|l| l.iter().any(|s| s.module == module))
                        .unwrap_or(false)
                    {
                        return Err(Error::at(sig.name.span, format!("duplicate signature {}", sig.name.text)));
                    }
                    let params: Vec<String> =
                        sig.params.iter().map(|p| p.name.text.clone()).collect();
                    // Parameter kinds come from their annotations (`f ::
                    // Type -> Type`) or are inferred from the method
                    // types' uses (an un-annotated `signature Functor f`
                    // infers the same kind as the annotated form).
                    let mut hints: Vec<Option<ParamKind>> = Vec::new();
                    for b in &sig.params {
                        hints.push(match &b.ann {
                            Some(ann) => Some(self.binder_kind(ann, &params)?),
                            None => None,
                        });
                    }
                    self.kind_constraints.borrow_mut().clear();
                    let mut methods: Vec<(String, Scheme)> = Vec::new();
                    for (idx, (mname, mty)) in sig.methods.iter().enumerate() {
                        // Method schemes quantify over the signature's
                        // parameters plus their own; the class constraint is
                        // always `ctx[0]`.
                        let mut extra: Vec<String> = Vec::new();
                        let mut rest = mty;
                        let mut own_ctx: Vec<(String, ClassApp)> = Vec::new();
                        loop {
                            match rest {
                                ast::Type::Forall(binders, body, _) => {
                                    for b in binders {
                                        extra.push(b.name.text.clone());
                                    }
                                    rest = body;
                                }
                                ast::Type::Using(cs, body, _) => {
                                    // A method's own constraint may name
                                    // the method's quantified binders
                                    // (`traverse`'s `using (app ::
                                    // Applicative f)`), so it converts
                                    // against the class parameters plus
                                    // the binders seen so far.
                                    let mut cvars = params.clone();
                                    cvars.extend(extra.iter().cloned());
                                    for c in cs {
                                        match self.conv_oblig(c, &cvars)? {
                                            ConvOblig::Class(n, app) => own_ctx.push((n, app)),
                                            ConvOblig::Prop(_) => {
                                                return Err(Error::at(
                                                    c.span(),
                                                    "a class method's `using` context takes class constraints; refinement obligations live on the instance (they restrict its applicability) or on a value's signature",
                                                ))
                                            }
                                        }
                                    }
                                    rest = body;
                                }
                                _ => break,
                            }
                        }
                        let mut vars = params.clone();
                        vars.extend(extra.iter().cloned());
                        let ty = self.conv_type(rest, &vars)?;
                        let mut all_params = params.clone();
                        all_params.extend(extra);
                        let mut ctx = vec![(
                            "dict".to_string(),
                            ClassApp {
                                class: sig.name.text.clone(),
                                args: params.iter().map(|p| Ty::Param(p.clone())).collect(),
                            },
                        )];
                        ctx.append(&mut own_ctx);
                        let mut row_params: Vec<String> = Vec::new();
                        let mut region_params: Vec<String> = Vec::new();
                        collect_row_names(&ty, &mut row_params, &mut region_params);
                        let ty = crate::types::regionize(&ty, &region_params);
                        let ctx = ctx
                            .into_iter()
                            .map(|(n, c)| (n, crate::types::regionize_class(&c, &region_params)))
                            .collect();
                        let type_params: Vec<String> = all_params
                            .into_iter()
                            .filter(|p| !row_params.contains(p) && !region_params.contains(p))
                            .collect();
                        methods.push((
                            mname.text.clone(),
                            Scheme {
            obligs: Vec::new(),
                                params: type_params,
                                idx_params: Vec::new(),
                                val_params: Vec::new(),
                                prop_params: Vec::new(),
                                row_params,
                                region_params,
                                ctx,
                                ty,
                            },
                        ));
                        let info = ValueInfo {
                            module: module.clone(),
                            name: mname.text.clone(),
                            scheme: methods.last().unwrap().1.clone(),
                            kind: ValueKind::Method {
                                idx,
                            },
                        };
                        if SEQ_CLASSES.contains(&sig.name.text.as_str()) {
                            // The three sequencing providers all have
                            // `pure`/`bind` methods: they share the value
                            // namespace, and the first registered answers
                            // bare uses (decision D22).
                            self.values
                                .entry(mname.text.clone())
                                .or_default()
                                .push(info.clone());
                            self.qualified
                                .insert((module.clone(), mname.text.clone()), info);
                        } else {
                            self.add_value(info.clone(), mname.span)?;
                            // The `Signature.method` spelling: a law or
                            // operation method resolves by its
                            // signature's name as well as its module's
                            // (modules and compilation specification
                            // section 5).
                            self.qualified
                                .insert((sig.name.text.clone(), mname.text.clone()), info);
                        }
                    }
                    // `law` members (primitive operations and inline
                    // target IR specification section 5): each states
                    // an equation over the class's methods, citable as
                    // `justify` evidence by its `Signature.law` identity
                    // and checked sample-wise against every instance.
                    let mut laws: Vec<(String, Scheme)> = Vec::new();
                    for (lname, lty) in &sig.laws {
                        let mut extra: Vec<String> = Vec::new();
                        let mut rest = lty;
                        loop {
                            match rest {
                                ast::Type::Forall(binders, body, _) => {
                                    for b in binders {
                                        extra.push(b.name.text.clone());
                                    }
                                    rest = body;
                                }
                                ast::Type::Using(_, _, uspan) => {
                                    return Err(Error::at(
                                        *uspan,
                                        "a class law states its equation over the class's methods; `using` constraints are not supported on laws in the bootstrap slice",
                                    ))
                                }
                                _ => break,
                            }
                        }
                        let mut vars = params.clone();
                        vars.extend(extra.iter().cloned());
                        let ty = self.conv_type(rest, &vars)?;
                        // The law's conclusion is a proposition: an `Eq`
                        // equation (a bare proposition type is accepted
                        // too; it states no equation to check).
                        let mut conclusion = self.uni.zonk(&ty);
                        loop {
                            match conclusion {
                                Ty::Fun(_, _, _, c) | Ty::Pi { cod: c, .. } => conclusion = *c,
                                Ty::Prp(_) => break,
                                other => {
                                    return Err(Error::at(
                                        lname.span,
                                        format!(
                                            "a class law states a proposition (an `Eq` equation over the class's methods), but `{lname}` concludes with {other}",
                                            lname = lname.text
                                        ),
                                    ))
                                }
                            }
                        }
                        let mut row_params: Vec<String> = Vec::new();
                        let mut region_params: Vec<String> = Vec::new();
                        collect_row_names(&ty, &mut row_params, &mut region_params);
                        let ty = crate::types::regionize(&ty, &region_params);
                        let mut all_params = params.clone();
                        all_params.extend(extra);
                        let type_params: Vec<String> = all_params
                            .into_iter()
                            .filter(|p| !row_params.contains(p) && !region_params.contains(p))
                            .collect();
                        let scheme = Scheme {
            obligs: Vec::new(),
                            params: type_params,
                            idx_params: Vec::new(),
                            val_params: Vec::new(),
                            prop_params: Vec::new(),
                            row_params,
                            region_params,
                            ctx: Vec::new(),
                            ty,
                        };
                        let info = ValueInfo {
                            module: module.clone(),
                            name: lname.text.clone(),
                            scheme: scheme.clone(),
                            kind: ValueKind::Law,
                        };
                        // A law is cited by identity (`Semigroup.assoc`)
                        // and never by bare name: it registers only
                        // under its `Signature.law` spelling, so law
                        // names stay out of the value namespace
                        // (primitive operations and inline target IR
                        // specification section 5).
                        self.qualified
                            .insert((sig.name.text.clone(), lname.text.clone()), info);
                        laws.push((lname.text.clone(), scheme));
                    }
                    let notes = self.kind_constraints.borrow().clone();
                    let mut param_kinds: Vec<ParamKind> = Vec::new();
                    for (b, hint) in sig.params.iter().zip(&hints) {
                        param_kinds.push(self.resolve_binder_kind(
                            &b.name.text,
                            hint.as_ref(),
                            &notes,
                            b.name.span,
                        )?);
                    }
                    // A default belongs to a declared method; its clauses
                    // group by name in source order like top-level ones.
                    let mut defaults: Vec<(String, Vec<ast::Equation>)> = Vec::new();
                    for eq in &sig.defaults {
                        if !methods.iter().any(|(m, _)| *m == eq.name.text) {
                            return Err(Error::at(
                                eq.name.span,
                                format!(
                                    "`{}` is not a method of {}: a default equation needs the method's signature in the same signature block",
                                    eq.name.text, sig.name.text
                                ),
                            ));
                        }
                        match defaults.iter_mut().find(|(m, _)| *m == eq.name.text) {
                            Some((_, eqs)) => eqs.push(eq.clone()),
                            None => defaults.push((eq.name.text.clone(), vec![eq.clone()])),
                        }
                    }
                    let info = SigInfo {
                        module: module.clone(),
                        name: sig.name.text.clone(),
                        params: params.clone(),
                        param_kinds: param_kinds.clone(),
                        methods,
                        defaults,
                        laws,
                    };
                    // A class application is also the dictionary's type:
                    // `(vectors :: VecU32Target t w)` names the class as
                    // a type former (the capability binder rule of
                    // decision D21 already builds `Ty::Adt(class, args)`).
                    let tk = TypeKind {
                        module: module.clone(),
                        name: sig.name.text.clone(),
                        arity: info.params.len(),
                        param_kinds: info.param_kinds.clone(),
                        ctors: Vec::new(),
                        fields: None,
                        field_erased: Vec::new(), opaque: false, refine: None,
                    };
                    self.types.entry(sig.name.text.clone()).or_default().push(tk.clone());
                    self.qualified_types
                        .insert((module.clone(), sig.name.text.clone()), tk);
                    self.sigs.entry(sig.name.text.clone()).or_default().push(info.clone());
                    self.qualified_sigs.insert((module.clone(), sig.name.text.clone()), info);
                }
                ast::Decl::Effect(eff) => {
                    if self.effects.contains_key(&eff.name.text) {
                        return Err(Error::at(
                            eff.name.span,
                            format!("duplicate effect {}", eff.name.text),
                        ));
                    }
                    let region_param = eff.params.first().map(|p| p.text.clone());
                    let base_vars: Vec<String> =
                        eff.params.iter().map(|p| p.text.clone()).collect();
                    // The effect's label: `L` or `L r`.
                    let label = EffLabel {
                        head: eff.name.text.clone(),
                        arg: region_param.clone(),
                    };
                    let mut ops: Vec<(String, Scheme)> = Vec::new();
                    for (mname, mty) in &eff.ops {
                        let mut extra: Vec<String> = Vec::new();
                        let mut rest = mty;
                        loop {
                            match rest {
                                ast::Type::Forall(binders, body, _) => {
                                    for b in binders {
                                        if base_vars.contains(&b.name.text)
                                            || extra.contains(&b.name.text)
                                        {
                                            return Err(Error::at(
                                                b.name.span,
                                                "duplicate type parameter",
                                            ));
                                        }
                                        extra.push(b.name.text.clone());
                                    }
                                    rest = body;
                                }
                                _ => break,
                            }
                        }
                        let mut all = base_vars.clone();
                        all.extend(extra.iter().cloned());
                        let ty = self.conv_type(rest, &all)?;
                        let mut row_params: Vec<String> = Vec::new();
                        let mut region_params: Vec<String> = Vec::new();
                        collect_row_names(&ty, &mut row_params, &mut region_params);
                        let ty = crate::types::regionize(&ty, &region_params);
                        // The operation performs exactly its effect's
                        // label (bootstrap restriction): the arrows are
                        // pure up to the one carrying the label, which is
                        // the operation's last argument arrow.
                        let mut rest_ty = &ty;
                        let mut found_label = false;
                        loop {
                            match rest_ty {
                                Ty::Fun(a, _, e, b) => {
                                    let _ = a;
                                    let want = EffRow::closed(vec![label.clone()]);
                                    if e == &want {
                                        found_label = true;
                                        break;
                                    }
                                    if !e.is_pure() {
                                        return Err(Error::at(
                                            mname.span,
                                            format!(
                                                "effect operation {} must perform exactly its effect's label {{{label}}}",
                                                mname.text
                                            ),
                                        ));
                                    }
                                    rest_ty = b;
                                }
                                _ => break,
                            }
                        }
                        if !found_label {
                            return Err(Error::at(
                                mname.span,
                                format!(
                                    "effect operation {} must perform its effect's label {{{label}}}",
                                    mname.text
                                ),
                            ));
                        }
                        let type_params: Vec<String> = all
                            .into_iter()
                            .filter(|p| !row_params.contains(p) && !region_params.contains(p))
                            .collect();
                        let scheme = Scheme {
            obligs: Vec::new(),
                            params: type_params,
                            idx_params: Vec::new(),
                            val_params: Vec::new(),
                            prop_params: Vec::new(),
                            row_params,
                            region_params,
                            ctx: Vec::new(),
                            ty,
                        };
                        let slot = ops.len() + 1;
                        let info = ValueInfo {
                            module: module.clone(),
                            name: mname.text.clone(),
                            scheme: scheme.clone(),
                            kind: ValueKind::EffectOp {
                                head: eff.name.text.clone(),
                                slot,
                            },
                        };
                        self.add_value(info, mname.span)?;
                        self.effect_ops.insert((module.clone(), mname.text.clone()), ());
                        ops.push((mname.text.clone(), scheme));
                    }
                    self.effects.insert(
                        eff.name.text.clone(),
                        EffectInfo {
                            region_param,
                            ops,
                        },
                    );
                }
                ast::Decl::Instance(inst) => {
                    // The head converts once every module is collected
                    // (`convert_instance_heads`): a head may name a
                    // class or a target type in a module collected
                    // later (the prelude split's family). The bodies
                    // queue for `check_bodies` as before.
                    self.pending_instance_heads
                        .push((module.clone(), inst.clone()));
                    self.instance_defs_pending.push((module.clone(), inst.clone()));
                }
                ast::Decl::Attributed { .. } => {
                    // Attributes are validated and unwrapped before the
                    // collection loop, so the annotated declaration is
                    // collected as ordinary.
                }
                ast::Decl::Generate { stmts, span } => {
                    self.generate_blocks.push(GenBlock {
                        module: module.clone(),
                        stmts: stmts.clone(),
                        span: *span,
                    });
                }
                ast::Decl::Rewrite(r) => {
                    // Rewrite rules are checked after the module's other
                    // declarations (their `match`, `replace` and
                    // `justify` reference the module's names).
                    self.pending_rewrites.push((module.clone(), r.clone()));
                }
                ast::Decl::Upgrade(u) => {
                    // Upgrade declarations are checked after the module's
                    // records and functions are known (runtime evolution
                    // specification section 3: migration field checking).
                    self.pending_upgrades.push((module.clone(), u.clone()));
                }
                ast::Decl::Contract(c) => {
                    // Temporal contracts (core specification section 10)
                    // attach to a definition of their own module: each is
                    // verified with the named definition's elaboration
                    // (its per-step write log and recursion).
                    self.pending_contracts
                        .entry((module.clone(), c.target.text.clone()))
                        .or_default()
                        .push(c.clone());
                }
            }
        }

        // Top-level values resolve by name from other bodies, including
        // their own (recursion): they are registered as their signature
        // is collected. Every equation needs a signature; every signature
        // needs an equation.
        for (name, eq) in &eqs {
            if !schemes.contains_key(name) {
                return Err(Error::at(
                    eq.span,
                    format!("missing signature for {name} (this draft does not silently infer top-level types)"),
                ));
            }
        }
        for (name, _) in &schemes {
            // An `import data` binding defines its name (surface grammar
            // local amendments section 11).
            let bound_by_import = self
                .module_data_imports
                .get(&module)
                .map(|ds| ds.iter().any(|d| &d.name.text == name))
                .unwrap_or(false);
            if name != "main" && !bound_by_import && !eqs.iter().any(|(n, _)| n == name) {
                return Err(Error::plain(format!("missing defining equation for {name}")));
            }
        }
        for (name, scheme) in schemes {
            self.top_schemes.insert((module.clone(), name), scheme);
        }
        for (name, eq) in eqs {
            self.top_eqs.push((module.clone(), name, eq));
        }
        // A module parameter is an ordinary binding of the module scope:
        // it must not collide with a declaration of the same name (module
        // parameters as module arguments).
        for p in &m.params {
            if self
                .values
                .get(&p.name.text)
                .map(|vs| vs.iter().any(|v| v.module == module))
                .unwrap_or(false)
            {
                return Err(Error::at(
                    p.span,
                    format!(
                        "module parameter {} collides with a declaration of the same name",
                        p.name.text
                    ),
                ));
            }
        }
        // The `import data` converters run once the module's names and
        // types are registered (surface grammar local amendments section
        // 11), before the export surface resolves its items.
        self.collect_data_imports(&module, m)?;
        // The export surface and the import hygiene check run once the
        // module's names are registered (surface grammar local
        // amendments section 11).
        self.finish_module_surface(&module)?;
        Ok(())
    }

    /// Validate one attribute list at its attachment position (attribute
    /// annotations specification sections 1.1-1.3) and return what the
    /// position's owner may use: one weight-family value (the wrappers
    /// are weights), one inline hint, a function-level `#[cold]`, and a
    /// parsed `#[repr(...)]` spec. The family, the argument grammar, the
    /// placement and the duplicate rules are all enforced here.
    fn validate_attr_site(
        &self,
        attrs: &[crate::ast::Attribute],
        site: AttrSite,
    ) -> Result<AttrSet> {
        let mut out = AttrSet::default();
        let mut seen: Vec<String> = Vec::new();
        for a in attrs {
            let name = a.name();
            // Two attributes with the same name on one construct are an
            // error unless a macro declares the multi-attribute form
            // (section 1.3); a macro-owned name's forms are its macro's
            // to validate.
            let macro_owned = self.macro_attr_names.contains_key(&name);
            if seen.contains(&name) && !macro_owned {
                return Err(Error::at(
                    a.span,
                    format!(
                        "two `{name}` attributes on one construct: duplicate attribute names are an error unless a macro declares a multi-attribute form (attribute annotations specification section 1.3)"
                    ),
                ));
            }
            seen.push(name.clone());
            let args = a.args.as_deref().unwrap_or("");
            match name.as_str() {
                "meta" => {}
                "weight" | "likely" | "unlikely" | "cold" => {
                    let family_ok = match site {
                        AttrSite::Clause
                        | AttrSite::Alt
                        | AttrSite::Guard
                        | AttrSite::IfCond
                        | AttrSite::IfBranch => true,
                        AttrSite::FnDecl => name == "cold",
                        _ => false,
                    };
                    if !family_ok {
                        return Err(Error::at(a.span, site.placement_error(&name)));
                    }
                    if site == AttrSite::FnDecl {
                        // `#[cold]` on a function declaration is the
                        // function-level hint of section 5, not the
                        // weight wrapper of section 3.
                        if !args.trim().is_empty() {
                            return Err(Error::at(
                                a.span,
                                "`#[cold]` takes no arguments (attribute annotations specification sections 3 and 5)",
                            ));
                        }
                        out.fn_cold = true;
                        continue;
                    }
                    let w = weight_value(a)?;
                    if out.weight.is_some() {
                        return Err(Error::at(
                            a.span,
                            format!(
                                "one alternative carries one weight: `{name}` and the weight attribute before it both assign one (attribute annotations specification section 2.1)"
                            ),
                        ));
                    }
                    out.weight = Some(w);
                }
                "inline" | "noinline" => {
                    if site != AttrSite::FnDecl {
                        return Err(Error::at(a.span, site.placement_error("inline")));
                    }
                    // `#[noinline]` is the spelling of `#[inline(never)]`
                    // (attribute annotations specification section 5).
                    let hint = if name == "noinline" {
                        if !arg_text(args).is_empty() {
                            return Err(Error::at(
                                a.span,
                                "`#[noinline]` takes no arguments (attribute annotations specification section 5)",
                            ));
                        }
                        InlineHint::Never
                    } else {
                        match arg_text(args) {
                            "" => InlineHint::Inline,
                            "always" => InlineHint::Always,
                            "never" => InlineHint::Never,
                            _ => {
                                return Err(Error::at(
                                    a.span,
                                    "`#[inline]` takes no argument or one of `(always)` and `(never)` (attribute annotations specification section 5)",
                                ))
                            }
                        }
                    };
                    if out.inline.is_some() {
                        return Err(Error::at(
                            a.span,
                            "one function carries one inline hint: `#[inline]`, `#[inline(always)]`, `#[inline(never)]` and `#[noinline]` are one hint family (attribute annotations specification section 5)",
                        ));
                    }
                    out.inline = Some(hint);
                }
                "repr" => {
                    if !matches!(site, AttrSite::TypeDecl | AttrSite::Field) {
                        return Err(Error::at(a.span, site.placement_error("repr")));
                    }
                    let spec = parse_repr_args(a)?;
                    if out.repr.is_some() {
                        return Err(Error::at(
                            a.span,
                            "one declaration carries one `#[repr(...)]` attribute: combine its representations inside one list (attribute annotations specification section 1.3)",
                        ));
                    }
                    out.repr = Some(spec);
                }
                // P20: the safety assertion vocabulary (attribute
                // annotations specification section 10). Every
                // assertion attaches to a function definition and is
                // verified there.
                "recursion" | "stack" | "no-alloc" | "alloc" | "max-complexity" | "max-lines"
                | "assert-min" | "checked-params" | "no-macros" | "no-raw-ptr"
                | "deny-warnings" => {
                    if site != AttrSite::FnDecl {
                        return Err(Error::at(a.span, site.placement_error("safety")));
                    }
                    out.safety.fill_from(a)?;
                }
                // The by-value aggregate ABI spelling at a foreign
                // boundary (design/llvm-contract-attributes.md item 6).
                "byval" | "sret" => {
                    if site != AttrSite::ForeignDecl {
                        return Err(Error::at(a.span, site.placement_error(&name)));
                    }
                    if !arg_text(args).is_empty() {
                        return Err(Error::at(
                            a.span,
                            format!("`#[{name}]` takes no arguments"),
                        ));
                    }
                    let slot = if name == "byval" {
                        &mut out.foreign_abi.byval
                    } else {
                        &mut out.foreign_abi.sret
                    };
                    if *slot {
                        return Err(Error::at(
                            a.span,
                            format!(
                                "one declaration carries one `#[{name}]` attribute (attribute annotations specification section 1.3)"
                            ),
                        ));
                    }
                    *slot = true;
                }
                "profile" => {
                    return Err(Error::at(
                        a.span,
                        "`#[profile \"jpl\"]` annotates a module header (attribute annotations specification section 10)",
                    ));
                }
                _ => {
                    if !macro_owned {
                        return Err(Error::at(
                            a.span,
                            format!(
                                "unknown attribute `{name}`: outside the `meta` namespace, an attribute name must be compiler-known or declared by a macro (`macro name attributes ({name}) :: ...`)"
                            ),
                        ));
                    }
                }
            }
        }
        Ok(out)
    }

    /// A declaration's attributes and the declaration they annotate
    /// (attribute annotations specification section 1.1). Validation is
    /// per attachment position and happens where the position's owner is
    /// collected or checked, so macro-emitted attributes revalidate
    /// under ordinary rules (section 6).
    fn flatten_attributes(
        &self,
        d: &crate::ast::Decl,
        out: &mut Vec<(Vec<crate::ast::Attribute>, crate::ast::Decl)>,
    ) -> Result<()> {
        match d {
            crate::ast::Decl::Attributed { attrs, inner } => {
                let mut rest = Vec::new();
                self.flatten_attributes(inner, &mut rest)?;
                for (mut a, dd) in rest {
                    let mut merged = attrs.clone();
                    merged.append(&mut a);
                    out.push((merged, dd));
                }
                Ok(())
            }
            other => {
                out.push((Vec::new(), other.clone()));
                Ok(())
            }
        }
    }

    /// Convert every parameterized module's parameter types once all
    /// names are collected (module parameters as module arguments,
    /// modules specification section 1). The written type's head decides
    /// the parameter's kind: a `signature` application is a dictionary
    /// binding, supplied at instantiation by an explicit dictionary (the
    /// instantiator's explicit dictionary selection); any other type is an
    /// ordinary value binding, which must carry `Copy` evidence because
    /// the parameter is shared by every definition of the module
    /// (ownership and usage design section 3).
    fn finish_module_params(&mut self) -> Result<()> {
        let mut modules: Vec<String> = self.mod_param_asts.keys().cloned().collect();
        modules.sort();
        for module in modules {
            let decls = self.mod_param_asts.get(&module).cloned().unwrap_or_default();
            self.build_scope(&module, true);
            let mut out = Vec::new();
            for p in &decls {
                let mut head = &p.ty;
                while let ast::Type::App(f, _, _) = head {
                    head = f;
                }
                let class = match head {
                    ast::Type::Con(cp, _) if self.lookup_sig(cp).is_ok() => {
                        let (class, args) = self.class_app_of(&p.ty, &[])?;
                        let app = ClassApp { class, args };
                        if app.class == "Io" {
                            return Err(Error::at(
                                p.span,
                                "a runtime capability is lexical evidence (decision D21) and cannot be a module parameter",
                            ));
                        }
                        if app.class == "Copy" {
                            return Err(Error::at(
                                p.span,
                                "`Copy` is structural evidence (ownership and usage design section 3) and cannot be a module parameter",
                            ));
                        }
                        Some(app)
                    }
                    _ => None,
                };
                let (ty, rendered) = match &class {
                    Some(app) => {
                        let r = app.to_string();
                        (Ty::Adt(app.class.clone(), app.args.clone()), r)
                    }
                    None => {
                        let t = self.conv_type(&p.ty, &[])?;
                        if !self.copy_structural(&t, &[], &mut Vec::new()) {
                            return Err(Error::at(
                                p.span,
                                format!(
                                    "a module parameter is shared by every definition of the module, so {} needs `Copy` evidence (ownership and usage design section 3); a signature-typed parameter is a dictionary",
                                    t
                                ),
                            ));
                        }
                        let r = t.to_string();
                        (t, r)
                    }
                };
                out.push(ModParam {
                    name: p.name.text.clone(),
                    sig: format!("{} :: {}", p.name.text, rendered),
                    ty,
                    class,
                });
            }
            self.module_params.insert(module, out);
        }
        Ok(())
    }

    fn check_bodies(&mut self, module: &str, prelude_name: &str) -> Result<()> {
        self.build_scope(module, true);
        let _ = prelude_name;

        // Instantiation declarations (`import M (args) as N`) check their
        // static arguments and register the unit's values before this
        // module's bodies reference them (module parameters as module
        // arguments).
        self.cur_mod_params.clear();
        self.process_units(module)?;

        // Group this module's equations by name, preserving order.
        let mut names: Vec<String> = Vec::new();
        for (m, name, _) in &self.top_eqs {
            if m == module && !names.contains(name) {
                names.push(name.clone());
            }
        }
        for name in names {
            // Staging (P9): `stage`/`macro` definitions are compile-time
            // only, and code compiled on demand for compile-time evaluation
            // is checked already.
            if self
                .checked_early
                .contains(&(module.to_string(), name.clone()))
                || self.staging.contains(&(module.to_string(), name.clone()))
            {
                continue;
            }
            let eqs: Vec<ast::Equation> = self
                .top_eqs
                .iter()
                .filter(|(m, n, _)| m == module && n == &name)
                .map(|(_, _, e)| e.clone())
                .collect();
            let scheme = self
                .top_schemes
                .get(&(module.to_string(), name.clone()))
                .cloned()
                .unwrap();
            let id = fn_id(module, &name);
            // An ordinary top-level definition is a top-level value
            // binding for the eager-reference-cycle check (D302).
            self.record_top_value = true;
            let r = self.check_equations(&name, &id, &scheme, &eqs, Vec::new());
            self.record_top_value = false;
            r?;
        }

        // Mutually recursive logical definitions would let a looping
        // function inhabit a `Prop`: the per-definition structural check
        // covers self-recursion, and this covers reference cycles among
        // `spec`/`proof` definitions (core specification sections 3
        // and 8).
        self.check_logical_cycles()?;

        // Constructors as code generation units: the block stores the
        // non-erased fields only (core specification section 3). The
        // type order is fixed so the emitted units are reproducible.
        let mut type_ks: Vec<TypeKind> = self
            .types
            .values()
            .flatten()
            .filter(|t| t.module == module)
            .cloned()
            .collect();
        type_ks.sort_by(|a, b| a.name.cmp(&b.name));
        for t in type_ks {
            for (cname, tag) in &t.ctors {
                let v = self
                    .qualified
                    .get(&(module.to_string(), cname.clone()))
                    .cloned()
                    .unwrap();
                let runtime_arity = match &v.kind {
                    ValueKind::Ctor { runtime_arity, .. } => *runtime_arity,
                    _ => unreachable!(),
                };
                self.fns.push(FnDef {
                    id: fn_id(module, cname),
                    params: (0..runtime_arity).map(|i| format!("f{i}")).collect(),
                    body: crate::core::Core::Unit,
                    prim: None,
                    ctor: Some((*tag, runtime_arity)),
                    hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
                });
            }
        }

        // Method defaults of this module's classes, checked once at the
        // method's own scheme (the class dictionary is `ctx[0]`), so a
        // broken default is reported at the class even before any instance
        // takes it. The elaborated unit is span-free, and its hash is the
        // default's identity in the interface: instances inline the
        // default, so a changed default must change the class's contract.
        let mut with_defaults: Vec<SigInfo> = self
            .sigs
            .values()
            .flatten()
            .filter(|s| s.module == module && !s.defaults.is_empty())
            .cloned()
            .collect();
        with_defaults.sort_by(|a, b| a.name.cmp(&b.name));
        for sig in with_defaults {
            for (mname, eqs) in &sig.defaults {
                let scheme = sig
                    .methods
                    .iter()
                    .find(|(m, _)| m == mname)
                    .map(|(_, s)| s.clone())
                    .expect("a default names a declared method (checked at collection)");
                let gid = GlobalId(format!("{module}.{}.{mname}.default", sig.name));
                let before = self.fns.len();
                self.check_equations(mname, &gid, &scheme, eqs, Vec::new())?;
                let rows: String = self.fns[before..].iter().map(|f| format!("{f:?}\n")).collect();
                self.default_ids.insert(
                    (module.to_string(), sig.name.clone(), mname.clone()),
                    crate::interface::content_hash(&rows),
                );
            }
        }

        // Instances of this module.
        let pending: Vec<(String, ast::InstDecl)> = self
            .instance_defs_pending
            .iter()
            .filter(|(m, _)| m == module)
            .cloned()
            .collect();
        for (m, inst) in pending {
            match self.check_instance(&m, &inst) {
                Ok(()) => {}
                Err(e) => {
                    // A family instance whose body the shadowed copy of
                    // a home cannot satisfy drops instead of failing the
                    // program (see `shadowed_modules`); anything else
                    // stays loud.
                    if self.embedded_modules.contains(&m) && !self.shadowed_modules.is_empty() {
                        self.instance_defs_pending
                            .retain(|(mm, i)| !(mm == &m && i.name.text == inst.name.text));
                        continue;
                    }
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    fn check_instance(&mut self, module: &str, inst: &ast::InstDecl) -> Result<()> {
        let params: Vec<String> = inst.params.iter().map(|p| p.text.clone()).collect();
        let (class, args) = self.class_app_of(&inst.target, &params)?;
        let sig = self
            .sigs
            .get(&class)
            .and_then(|ss| ss.iter().find(|s| self.def_in_scope(&s.module, &s.name)))
            .cloned()
            .ok_or_else(|| Error::plain(format!("unknown signature {class}")))?;

        // Make this instance visible to its own method bodies even when it is
        // not `implicit`.
        let self_idx = self
            .instances
            .get(&class)
            .and_then(|idxs| {
                idxs.iter()
                    .copied()
                    .find(|&i| self.instance_list[i].name == inst.name.text && self.instance_list[i].module == module)
            })
            .unwrap();
        self.self_instance = Some(self_idx);

        let subdict_count = inst
            .ctx
            .iter()
            .filter(|c| matches!(c, ast::Oblig::Class(_)))
            .count();
        // Refinement transfer (refinement types design): an instance over
        // a refinement is the container's own methods, elaborated at the
        // container's method types (the representations coincide), valid
        // exactly when the methods preserve the predicate.
        let mut base_args: Vec<Ty> = Vec::new();
        let mut transfer = false;
        for a in &args {
            match self.refine_base(a) {
                Some(b) => {
                    transfer = true;
                    base_args.push(b);
                }
                None => base_args.push(a.clone()),
            }
        }
        let mut method_fns = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        let mut pres_methods: Vec<(String, GlobalId, Ty)> = Vec::new();
        for (mname, mscheme) in sig.methods.clone() {
            let mut eqs: Vec<ast::Equation> = inst
                .methods
                .iter()
                .filter(|e| e.name.text == mname)
                .cloned()
                .collect();
            let from_default = eqs.is_empty();
            if from_default {
                match sig.defaults.iter().find(|(m, _)| *m == mname) {
                    Some((_, d)) => eqs = d.clone(),
                    None => {
                        return Err(Error::at(
                            inst.span,
                            format!("instance {} does not define method {mname}", inst.name.text),
                        ))
                    }
                }
            }
            // Substitute the class parameters with the instance's type
            // arguments; the method's own quantifiers become fresh rigid
            // parameters. A refinement carrier substitutes its container
            // for the body's elaboration (the transfer rule).
            let mut map = HashMap::new();
            let mut extra: Vec<String> = Vec::new();
            for p in &mscheme.params {
                match sig.params.iter().position(|s| s == p) {
                    Some(i) => {
                        map.insert(p.clone(), args[i].clone());
                    }
                    None => {
                        map.insert(p.clone(), Ty::Param(self.fresh_param(p)));
                        extra.push(p.clone());
                    }
                }
            }
            let ty = Self::subst_ty(&mscheme.ty, &map);
            if transfer {
                pres_methods.push((mname.clone(), GlobalId(format!("{module}.{}.{}", inst.name.text, mname)), ty.clone()));
            }
            // The body checks at the container's method type (the
            // transfer rule); the dictionary presents the refined one
            // (the rep-identity is the point: no delegation code, no
            // wrapper). Fresh rigid parameters for the base elaboration,
            // so the refined and base types never share metavariables,
            // and one map for the body's type and context alike. A taken
            // default is the exception: it is generic code over the
            // class's own methods, which exist at the refined type (this
            // instance) and not at the container, and composing them
            // preserves the predicate by construction.
            let body_map: HashMap<String, Ty> = if transfer && !from_default {
                let mut bmap = HashMap::new();
                for p in &mscheme.params {
                    match sig.params.iter().position(|s| s == p) {
                        Some(i) => {
                            bmap.insert(p.clone(), base_args[i].clone());
                        }
                        None => {
                            bmap.insert(p.clone(), Ty::Param(self.fresh_param(p)));
                        }
                    }
                }
                bmap
            } else {
                map.clone()
            };
            let body_ty = Self::subst_ty(&mscheme.ty, &body_map);
            // Method function parameters are the instance's subdicts followed
            // by any extra constraints of the method itself; the class
            // constraint is satisfied by this instance and takes no parameter.
            // The instance's own context is already stated in the instance's
            // parameters, so it enters unchanged: substituting the class
            // parameters into it would capture a same-named instance
            // parameter (`using (sa :: Semigroup a) => Semigroup (Dual a)`
            // must keep `sa :: Semigroup a`, not `Semigroup (Dual a)`).
            let inst_ctx = InstCtx::dicts(&self.instance_list[self_idx].ctx);
            let mut ctx: Vec<(String, ClassApp)> = inst_ctx;
            for (n, c) in mscheme.ctx.iter().skip(1) {
                ctx.push((n.clone(), Self::subst_class(c, &body_map)));
            }
            let gid = GlobalId(format!("{module}.{}.{}", inst.name.text, mname));
            let scheme = Scheme {
            obligs: Vec::new(),
                params: extra,
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: mscheme.row_params.clone(),
                region_params: mscheme.region_params.clone(),
                ctx,
                ty: body_ty,
            };
            // A default is written against the method's general type; at a
            // function carrier (`Monoid (t -> u)`) the instance's method
            // type has more arrows than the default binds, so the taken
            // copy is eta-expanded to the instance's arity: the same
            // function, with the arity rule satisfied.
            if from_default {
                let arrows = arrow_count(&scheme.ty);
                for eq in &mut eqs {
                    let missing = arrows.saturating_sub(eq.pats.len());
                    if missing == 0 {
                        continue;
                    }
                    let span = eq.span;
                    let mut args = Vec::with_capacity(missing);
                    for k in 0..missing {
                        let n = ast::Name::new(format!("$eta{k}"), span);
                        eq.pats.push(ast::Pat::Var(n.clone(), span));
                        args.push(ast::Term::Var(ast::VarPath { quals: Vec::new(), name: n }));
                    }
                    let body = std::mem::replace(&mut eq.body, ast::Term::Unit(span));
                    eq.body = ast::Term::App(Box::new(body), args, span);
                }
            }
            // A default resolves names where it was written: the class's
            // module, whose imports the instance's module need not share.
            // Its spans point into that module, so the error says so.
            if from_default {
                self.build_scope(&sig.module, true);
            }
            let checked = self.check_equations(&mname, &gid, &scheme, &eqs, Vec::new());
            if from_default {
                self.build_scope(module, true);
            }
            checked.map_err(|e| {
                if from_default {
                    Error {
                        span: e.span,
                        message: format!(
                            "{} (in the default of {}.{mname}, module {}, taken by instance {})",
                            e.message, sig.name, sig.module, inst.name.text
                        ),
                    }
                } else {
                    e
                }
            })?;
            method_fns.push(gid);
            seen.push(mname);
        }
        for e in &inst.methods {
            if !seen.contains(&e.name.text) {
                return Err(Error::at(
                    e.span,
                    format!("{} is not a method of {class}", e.name.text),
                ));
            }
        }
        self.self_instance = None;

        // Class laws (primitive operations and inline target IR
        // specification section 5): this instance owes its class's laws,
        // checked sample-wise after every module's bodies are known.
        if !sig.laws.is_empty() {
            let subdicts: Vec<(String, Vec<Ty>)> =
                InstCtx::dicts(&self.instance_list[self_idx].ctx)
                    .into_iter()
                    .map(|(_, c)| (c.class.clone(), c.args.clone()))
                    .collect();
            for (law_name, law_scheme) in &sig.laws {
                self.law_obligations.push(LawObligation {
                    instance: inst.name.text.clone(),
                    module: module.to_string(),
                    class: class.clone(),
                    law: law_name.clone(),
                    scheme: law_scheme.clone(),
                    class_params: sig.params.clone(),
                    class_args: args.clone(),
                    method_units: method_fns.clone(),
                    subdicts: subdicts.clone(),
                    span: inst.span,
                });
            }
        }

        // Preservation (refinement types design): the transfer's
        // validity condition, checked sample-wise beside the class laws.
        if transfer {
            let subdicts: Vec<(String, Vec<Ty>)> =
                InstCtx::dicts(&self.instance_list[self_idx].ctx)
                    .into_iter()
                    .map(|(_, c)| (c.class.clone(), c.args.clone()))
                    .collect();
            for (mname, gid, mty) in pres_methods {
                let mut arg_tys: Vec<Ty> = Vec::new();
                let mut cur = mty;
                let result = loop {
                    match self.uni.zonk(&cur) {
                        Ty::Fun(a, _, _, b) => {
                            arg_tys.push(*a);
                            cur = *b;
                        }
                        Ty::Pi { dom, cod, .. } => {
                            arg_tys.push(*dom);
                            cur = *cod;
                        }
                        other => break other,
                    }
                };
                let Some(refine) = self.ty_refine_mention(&result) else {
                    // The method produces no refined value: nothing to
                    // preserve (inputs are refined by their types).
                    continue;
                };
                let direct = matches!(&result, Ty::Adt(n, _) if *n == refine);
                self.pres_obligations.push(PresObligation {
                    instance: inst.name.text.clone(),
                    module: module.to_string(),
                    class: class.clone(),
                    method: mname,
                    method_gid: gid,
                    refine,
                    arg_tys,
                    subdicts: subdicts.clone(),
                    class_args: args.clone(),
                    method_units: method_fns.clone(),
                    nested: !direct,
                    span: inst.span,
                });
            }
        }

        let base_id = fn_id(module, &inst.name.text);
        // A parameterized module's instance dictionary closes over the
        // module's parameters: they are the construction unit's leading
        // arguments before the instance's sub-dictionaries (module
        // parameters as module arguments; instantiation units forward
        // them at their static arguments).
        let mparams = self.module_params.get(module).map(|p| p.len()).unwrap_or(0);
        let def = InstanceDef {
            id: base_id,
            method_count: sig.methods.len(),
            subdict_count: mparams + subdict_count,
            methods: method_fns,
        };
        if mparams > 0 {
            self.param_inst_defs
                .insert((module.to_string(), inst.name.text.clone()), def.clone());
        }
        self.instance_defs.push(def);
        Ok(())
    }

    // --------------------------------------- instantiation units (A5)

    /// One module argument (module parameters as module arguments): a
    /// dictionary parameter takes a named `instance` dictionary (the
    /// instantiator's explicit dictionary selection, modules
    /// specification section 1); a value parameter takes a static value
    /// or literal of the parameter's type. Returns the elaborated
    /// argument and its source rendering (for the unit's identity).
    fn module_arg(
        &mut self,
        arg: &ast::ModuleArg,
        p: &ModParam,
    ) -> Result<(ECore, String)> {
        let span = arg.span();
        match (&p.class, arg) {
            (Some(app), ast::ModuleArg::Name { quals, name }) => {
                let wanted = Ty::Adt(app.class.clone(), app.args.clone());
                let (core, id) = self.named_instance_arg(quals, name, &wanted, span)?;
                Ok((core, id))
            }
            (Some(_), other) => Err(Error::at(
                other.span(),
                format!(
                    "module parameter {} is a dictionary of signature {}; supply an instance of it by name",
                    p.name,
                    p.class.as_ref().map(|c| c.to_string()).unwrap_or_default()
                ),
            )),
            (None, ast::ModuleArg::Name { quals, name }) => {
                // A dictionary named where a value is wanted gets a
                // precise error rather than an unknown name.
                let mut cands = 0;
                for inst in &self.instance_list {
                    if inst.name != name.text {
                        continue;
                    }
                    let named = match &quals[..] {
                        [] => true,
                        [q] => inst.module == q.text,
                        _ => false,
                    };
                    if named {
                        cands += 1;
                    }
                }
                if cands > 0 {
                    return Err(Error::at(
                        span,
                        format!(
                            "instance {} is a dictionary, but module parameter {} is a value of type {}",
                            name.text, p.name, p.ty
                        ),
                    ));
                }
                let term = module_arg_term(quals, name);
                let core = self.check(&mut Env::default(), &term, &p.ty)?;
                let text = arg_name_text(quals, name);
                Ok((core, text))
            }
            (None, lit) => {
                let term = module_arg_lit(lit)?;
                let core = self.check(&mut Env::default(), &term, &p.ty)?;
                Ok((core, arg_lit_text(lit)))
            }
        }
    }

    /// A named dictionary argument: an `instance` of the parameter's
    /// signature, selected explicitly by name (any instance, implicit or
    /// explicit: a module argument is explicit dictionary selection). Its
    /// target matches the parameter type one-way, exactly as at instance
    /// resolution, so a wrong-typed instantiation is rejected here.
    fn named_instance_arg(
        &mut self,
        quals: &[ast::Name],
        name: &ast::Name,
        wanted: &Ty,
        span: ast::Span,
    ) -> Result<(ECore, String)> {
        let mut cands: Vec<usize> = Vec::new();
        for (i, inst) in self.instance_list.iter().enumerate() {
            if inst.name != name.text {
                continue;
            }
            match quals {
                [] => cands.push(i),
                [q] if inst.module == q.text => cands.push(i),
                _ => {}
            }
        }
        let idx = match cands.as_slice() {
            [] => {
                return Err(Error::at(
                    span,
                    format!(
                        "unknown instance {} for module argument (the argument is an `instance` of the parameter's signature)",
                        arg_name_text(quals, name)
                    ),
                ))
            }
            [one] => *one,
            _ => {
                return Err(Error::at(
                    span,
                    format!(
                        "instance {} is declared by several modules; qualify the module argument with its owner",
                        name.text
                    ),
                ))
            }
        };
        if self.instance_list[idx].base {
            return Err(Error::at(
                span,
                format!(
                    "instance {} belongs to the parameterized module {}; its dictionary needs that module's parameters (functor composition is not supported in the bootstrap slice)",
                    self.instance_list[idx].name, self.instance_list[idx].module
                ),
            ));
        }
        let (params, ctx, base_id, target, iname) = {
            let inst = &self.instance_list[idx];
            (
                inst.params.clone(),
                inst.ctx.clone(),
                inst.base_id.clone(),
                inst.target.clone(),
                inst.name.clone(),
            )
        };
        let mut fresh: HashMap<String, Ty> = HashMap::new();
        for p in &params {
            fresh.insert(p.clone(), self.uni.fresh());
        }
        let target = Self::subst_class(&target, &fresh);
        let target_app = Ty::Adt(target.class.clone(), target.args.clone());
        let mut trial = self.uni.clone();
        if trial.unify(&target_app, wanted).is_err() {
            return Err(Error::at(
                span,
                format!(
                    "instance {iname} implements {}, which does not match module parameter type {}",
                    self.instance_list[idx].target, self.uni.zonk(wanted)
                ),
            ));
        }
        self.uni = trial;
        let mut sub: HashMap<String, Ty> = HashMap::new();
        for p in &params {
            let t = self.uni.zonk(&fresh[p]);
            if t == fresh[p] {
                return Err(Error::at(
                    span,
                    format!(
                        "instance {iname} parameter {p} is not determined by its target (supply a fully applied instance)"
                    ),
                ));
            }
            sub.insert(p.clone(), t);
        }
        let mut subs = Vec::new();
        for (n, c) in &InstCtx::dicts(&ctx) {
            let c = Self::subst_class(c, &sub);
            let id = self.new_want(c.clone(), span, &Env::default());
            let _ = n;
            subs.push(ECore::Hole(id));
        }
        Ok((ECore::InstDict(base_id, subs), String::new()))
    }

    /// The instantiation declarations of one module (`import M (args) as
    /// N`, module parameters as module arguments): check the static
    /// arguments against the target's parameters, register the unit's
    /// names as wrapper units over the target's functions, and publish
    /// the target's declared instances under the unit's name. The unit's
    /// identity is computed from the target's parameter signatures and
    /// the argument identities, so two instantiations at different
    /// dictionaries are distinct units; the target module's own contract
    /// is unchanged (instantiation is representation).
    fn process_units(&mut self, module: &str) -> Result<()> {
        let pending: Vec<ast::Import> = self
            .unit_decls
            .iter()
            .filter(|(m, _)| m == module)
            .map(|(_, i)| i.clone())
            .collect();
        for imp in pending {
            let span = imp.span;
            let mut target = imp.path.parts().join(".");
            if !self.module_names.contains(&target) {
                if let Some(last) = imp.path.parts().last() {
                    target = last.to_string();
                }
            }
            let params = self.module_params.get(&target).cloned().ok_or_else(|| {
                if self.unit_names.contains(&target) {
                    Error::at(
                        span,
                        format!("{target} is an instantiation unit, not a module; units take no parameters"),
                    )
                } else if self.module_names.contains(&target) {
                    Error::at(span, format!("module {target} takes no parameters"))
                } else {
                    Error::at(span, format!("unknown module {target}"))
                }
            })?;
            let unit = imp.alias.clone().expect("an applied import names its unit");
            if self.module_names.contains(&unit.text) || self.unit_names.contains(&unit.text) {
                return Err(Error::at(
                    unit.span,
                    format!(
                        "instantiation unit {} collides with a module or unit of the same name",
                        unit.text
                    ),
                ));
            }
            if imp.args.len() != params.len() {
                return Err(Error::at(
                    span,
                    format!(
                        "module {target} takes {} module arguments, got {}",
                        params.len(),
                        imp.args.len()
                    ),
                ));
            }
            let mut arg_cores: Vec<ECore> = Vec::new();
            let mut arg_ids: Vec<String> = Vec::new();
            let mut arg_texts: Vec<String> = Vec::new();
            for (arg, p) in imp.args.iter().zip(&params) {
                let (core, text) = self.module_arg(arg, p)?;
                arg_cores.push(core);
                arg_texts.push(text);
            }
            let sol = self.solve_wants()?;
            let mut filled: Vec<(ECore, String)> = Vec::new();
            for (c, text) in arg_cores.into_iter().zip(&arg_texts) {
                let c = self.fill_holes(&c, &sol)?;
                filled.push((c, text.clone()));
            }
            // Deferred literal words and index arguments resolve
            // positionally across every argument at once (their indices
            // are shared); module arguments keep their runtime class
            // calls (the collection literal fold covers definition
            // bodies).
            let lits = self.resolved_literals()?;
            let idx_vals = self.resolved_idx_args()?;
            let mut cores: Vec<crate::core::Core> = Vec::new();
            for (c, text) in filled {
                let mut c = c;
                resolve_lit_words(&mut c, &lits);
                resolve_idx_words(&mut c, &idx_vals);
                let low = lower(&c);
                arg_ids.push(arg_identity(&low, &text));
                cores.push(low);
            }
            // The unit's values are wrapper units over the target's
            // functions with the arguments applied; types, constructors
            // and signatures re-export the target's unchanged (a module
            // parameter is never a type).
            let vals: Vec<ValueInfo> = self
                .values
                .values()
                .flatten()
                .filter(|v| v.module == target)
                .cloned()
                .collect();
            for v in vals {
                if matches!(v.kind, ValueKind::TopFn) {
                    let mut u = v.clone();
                    u.module = unit.text.clone();
                    self.add_value(u, span)?;
                } else {
                    self.qualified.insert((unit.text.clone(), v.name.clone()), v);
                }
            }
            let tys: Vec<TypeKind> = self
                .types
                .values()
                .flatten()
                .filter(|t| t.module == target)
                .cloned()
                .collect();
            for t in tys {
                self.qualified_types.insert((unit.text.clone(), t.name.clone()), t);
            }
            let sgs: Vec<SigInfo> = self
                .sigs
                .values()
                .flatten()
                .filter(|s| s.module == target)
                .cloned()
                .collect();
            for s in sgs {
                self.qualified_sigs.insert((unit.text.clone(), s.name.clone()), s);
            }
            // The target's declared instances are published under the
            // unit's name: each instantiation constructs its own
            // dictionary at the unit's arguments. Two instantiations
            // publishing unifiable implicit instances are rejected by the
            // coherence check; nothing merges them (modules specification
            // section 1's coherent resolution).
            let base_insts: Vec<InstanceInfo> = self
                .instance_list
                .iter()
                .filter(|i| i.base && i.module == target)
                .cloned()
                .collect();
            let mut unit_insts = Vec::new();
            for bi in base_insts {
                let uid = GlobalId(format!("{}.{}", unit.text, bi.name));
                let idx = self.instance_list.len();
                self.instance_list.push(InstanceInfo {
                    module: unit.text.clone(),
                    name: bi.name.clone(),
                    params: bi.params.clone(),
                    ctx: bi.ctx.clone(),
                    target: bi.target.clone(),
                    base_id: uid,
                    implicit: bi.implicit,
                    base: false,
                });
                self.instances.entry(bi.target.class.clone()).or_default().push(idx);
                unit_insts.push(UnitInst { inst_name: bi.name.clone() });
            }
            let param_sigs: Vec<String> = params.iter().map(|p| p.sig.clone()).collect();
            let unit_id = crate::interface::content_hash(&format!(
                "{}|{}|{}",
                target,
                param_sigs.join(","),
                arg_ids.join(",")
            ));
            self.units.push(UnitRec {
                owner: module.to_string(),
                name: unit.text.clone(),
                target,
                args: cores,
                arg_ids,
                unit_id,
                insts: unit_insts,
            });
            self.unit_names.insert(unit.text.clone());
        }
        Ok(())
    }

    /// Emit each instantiation unit's wrapper units (module parameters as
    /// module arguments): the target module's functions with the unit's
    /// static arguments applied before the forwarded parameters, the
    /// unit's instance dictionary constructions, and its instance method
    /// wrappers. Instantiation is representation: the wrappers sit beside
    /// the target module's unchanged units.
    fn emit_unit_wrappers(&mut self) -> Result<()> {
        let units = self.units.clone();
        let mut wrappers: Vec<FnDef> = Vec::new();
        for u in &units {
            let k = self.module_params.get(&u.target).map(|p| p.len()).unwrap_or(0);
            let names: Vec<String> = self
                .values
                .values()
                .flatten()
                .filter(|v| v.module == u.target && matches!(v.kind, ValueKind::TopFn))
                .map(|v| v.name.clone())
                .collect();
            for name in names {
                let gid = fn_id(&u.target, &name);
                // A logical definition is erased and emits no unit; its
                // references are erased with it.
                let Some(fd) = self.fns.iter().find(|f| f.id == gid).cloned() else {
                    continue;
                };
                let n = fd.arity() - k;
                let mut args = u.args.clone();
                for i in 0..n {
                    args.push(crate::core::Core::Var(LocalId(i)));
                }
                wrappers.push(FnDef {
                    id: fn_id(&u.name, &name),
                    params: (0..n).map(|i| format!("p{i}")).collect(),
                    body: crate::core::Core::App(Box::new(crate::core::Core::Global(gid)), args),
                    prim: None,
                    ctor: None,
                    hints: crate::core::FnHints::default(),
                    meta: shifted_meta(&fd.meta, k),
                });
            }
            for ui in &u.insts {
                let base = self
                    .param_inst_defs
                    .get(&(u.target.clone(), ui.inst_name.clone()))
                    .cloned()
                    .ok_or_else(|| {
                        Error::plain(format!(
                            "internal error: instance {} of {} has no dictionary unit",
                            ui.inst_name, u.target
                        ))
                    })?;
                let mut methods = Vec::new();
                for mth in &base.methods {
                    let fd = self
                        .fns
                        .iter()
                        .find(|f| f.id == *mth)
                        .cloned()
                        .ok_or_else(|| {
                            Error::plain(format!(
                                "internal error: instance method {} has no unit",
                                mth.0
                            ))
                        })?;
                    let n = fd.arity() - k;
                    let prefix = format!("{}.{}.", u.target, ui.inst_name);
                    let suffix = mth.0.strip_prefix(&prefix).unwrap_or(&mth.0);
                    let wid = GlobalId(format!("{}.{}.{}", u.name, ui.inst_name, suffix));
                    let mut args = u.args.clone();
                    for i in 0..n {
                        args.push(crate::core::Core::Var(LocalId(i)));
                    }
                    wrappers.push(FnDef {
                        id: wid.clone(),
                        params: (0..n).map(|i| format!("p{i}")).collect(),
                        body: crate::core::Core::App(
                            Box::new(crate::core::Core::Global(mth.clone())),
                            args,
                        ),
                        prim: None,
                        ctor: None,
                        hints: crate::core::FnHints::default(),
                        meta: shifted_meta(&fd.meta, k),
                    });
                    methods.push(wid);
                }
                self.instance_defs.push(InstanceDef {
                    id: GlobalId(format!("{}.{}", u.name, ui.inst_name)),
                    method_count: base.method_count,
                    subdict_count: base.subdict_count - k,
                    methods,
                });
            }
        }
        self.fns.extend(wrappers);
        Ok(())
    }

    /// Reject reference cycles among logical definitions (a direct
    /// self-reference is the structural check's business, not a cycle).
    fn check_logical_cycles(&self) -> Result<()> {
        for start in self.logical_refs.keys() {
            let mut seen: Vec<(String, String)> = vec![start.clone()];
            let mut stack: Vec<(String, String)> = vec![start.clone()];
            while let Some(k) = stack.pop() {
                let Some(refs) = self.logical_refs.get(&k) else {
                    continue;
                };
                for r in refs {
                    if r == start {
                        if k != *start {
                            return Err(Error::plain(format!(
                                "logical definitions {} are mutually recursive; a looping function must not inhabit a proposition (obligation status: unsupported)",
                                start.1
                            )));
                        }
                        continue;
                    }
                    if !seen.contains(r) {
                        seen.push(r.clone());
                        stack.push(r.clone());
                    }
                }
            }
        }
        Ok(())
    }

    /// Record one top-level binding for the eager-reference-cycle check
    /// (D302): its body's reference edges, walked with the guarded-letrec
    /// discipline (a function's body sits under its parameter lambda, so
    /// its uses are guarded; a value's body runs eagerly). The walk is
    /// over the elaborated core beside the local rule's own, so both
    /// disciplines count the same suspension positions.
    fn collect_top_bind(
        &mut self,
        id: &GlobalId,
        name: &str,
        span: ast::Span,
        is_value: bool,
        body: &ECore,
    ) {
        let owner = self.top_binds.len();
        self.top_binds.push((id.clone(), name.to_string(), span, is_value));
        top_rec_walk(body, if is_value { 0 } else { 1 }, owner, &mut self.top_edges);
    }

    /// The guarded-letrec rule over the top-level bindings (the compiler
    /// defect recorded at D302): the `letrec` fixpoint discipline does
    /// not reach top-level value bindings, so an eager reference cycle
    /// among them re-enters its own thunk at run time and crashes the
    /// compiled binary (signal-killed). The cycle is refused at check
    /// time instead, naming the referenced value and the fix. An
    /// unguarded edge (owner, referenced) is eager; it closes a cycle
    /// when the referenced binding can reach the owner over the group's
    /// edges, which covers the eager call into a sibling's body exactly
    /// as the local rule's reachability does.
    fn check_top_value_cycles(&self) -> Result<()> {
        /// Whether `to` is reachable from `from` over the edge graph.
        fn reaches(adj: &[Vec<usize>], from: usize, to: usize) -> bool {
            let mut seen = vec![false; adj.len()];
            let mut stack = vec![from];
            seen[from] = true;
            while let Some(k) = stack.pop() {
                if k == to {
                    return true;
                }
                for &n in &adj[k] {
                    if !seen[n] {
                        seen[n] = true;
                        stack.push(n);
                    }
                }
            }
            false
        }
        let mut index: HashMap<&GlobalId, usize> = HashMap::new();
        for (i, (id, _, _, _)) in self.top_binds.iter().enumerate() {
            index.entry(id).or_insert(i);
        }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); self.top_binds.len()];
        for (o, r, _) in &self.top_edges {
            if let Some(&ri) = index.get(r) {
                if !adj[*o].contains(&ri) {
                    adj[*o].push(ri);
                }
            }
        }
        let mut flagged: HashSet<(usize, usize)> = HashSet::new();
        for (o, r, guarded) in &self.top_edges {
            if *guarded {
                continue;
            }
            let (oi, Some(&ri)) = (o, index.get(r)) else {
                continue;
            };
            if !flagged.insert((*oi, ri)) {
                continue;
            }
            if *oi == ri || reaches(&adj, ri, *oi) {
                let span = self.top_binds[*oi].2;
                let name = &self.top_binds[ri].1;
                return Err(Error::at(
                    span,
                    format!(
                        "top-level value {name} is part of an eager reference cycle; wrap its reference in a lambda or use letrec"
                    ),
                ));
            }
        }
        Ok(())
    }

    // ------------------------------------------------------------ productivity (P8)

    /// The mutually recursive group of `name` among this module's
    /// definitions: `name` plus every definition that can call back into
    /// it. Recursive calls (to group members) are what the guardedness
    /// discipline governs (core language specification section 9).
    fn rec_group(&self, name: &str) -> Vec<String> {
        let module = self.current_module.clone();
        let mut names: Vec<String> = Vec::new();
        for (m, n, _) in &self.top_eqs {
            if *m == module && !names.contains(n) {
                names.push(n.clone());
            }
        }
        if !names.iter().any(|n| n == name) {
            return vec![name.to_string()];
        }
        let mut edges: Vec<(String, Vec<String>)> = Vec::new();
        for n in &names {
            let mut calls: Vec<String> = Vec::new();
            for (m, n2, e) in &self.top_eqs {
                if *m == module && n2 == n {
                    collect_calls(&e.body, &mut calls);
                }
            }
            calls.retain(|c| names.contains(c));
            calls.sort();
            calls.dedup();
            edges.push((n.clone(), calls));
        }
        let reach = |from: &str, to: &str| -> bool {
            let mut seen = vec![from.to_string()];
            let mut stack = vec![from.to_string()];
            while let Some(k) = stack.pop() {
                if let Some((_, cs)) = edges.iter().find(|(n, _)| *n == k) {
                    for c in cs {
                        if c == to {
                            return true;
                        }
                        if !seen.contains(c) {
                            seen.push(c.clone());
                            stack.push(c.clone());
                        }
                    }
                }
            }
            false
        };
        names
            .into_iter()
            .filter(|b| reach(name, b) && reach(b, name))
            .collect()
    }

    /// Is this a codata type?
    fn is_codata_path(&self, path: &ast::ConPath) -> bool {
        let parts = path.parts();
        match self.lookup_type(&parts) {
            Ok(info) => self
                .codata_types
                .contains(&(info.module.clone(), info.name.clone())),
            Err(_) => false,
        }
    }

    /// Is the observation `field` of codata type `path` a `Later`-typed
    /// guard? (codata and guardedness design, section 3)
    fn obs_is_later(&self, path: &ast::ConPath, field: &str) -> bool {
        let parts = path.parts();
        let Ok(info) = self.lookup_type(&parts) else {
            return false;
        };
        if !self
            .codata_types
            .contains(&(info.module.clone(), info.name.clone()))
        {
            return false;
        }
        let Some(fields) = &info.fields else {
            return false;
        };
        let Some(i) = fields.iter().position(|f| f == field) else {
            return false;
        };
        match self.ctor_of_type(&info, &info.name) {
            Ok(v) => match &v.kind {
                ValueKind::Ctor { field_tys, .. } => {
                    matches!(field_tys.get(i), Some(Ty::Later(_)))
                }
                _ => false,
            },
            Err(_) => false,
        }
    }

    /// Does this definition produce codata (its result type is codata, or
    /// its body builds a codata value)?
    fn produces_codata(&self, scheme: &Scheme, eqs: &[ast::Equation]) -> bool {
        let mut t = &scheme.ty;
        loop {
            match t {
                Ty::Fun(_, _, _, b) | Ty::Pi { cod: b, .. } => t = b,
                _ => break,
            }
        }
        if let Ty::Adt(n, _) = t {
            if let Ok(info) = self.lookup_type(&[n]) {
                if self
                    .codata_types
                    .contains(&(info.module.clone(), info.name.clone()))
                {
                    return true;
                }
            }
        }
        let mut found = false;
        for e in eqs {
            for_each_subterm(&e.body, &mut |x| {
                if let ast::Term::Record(path, _, _) = x {
                    if self.is_codata_path(path) {
                        found = true;
                    }
                }
            });
        }
        found
    }

    /// The productivity discipline (core language specification section
    /// 9). A `corec` definition claims productivity: its guard check runs
    /// (design section 3). An ordinary definition that produces codata is
    /// not certified, so any unguarded recursion must be reflected in its
    /// type (`Diverge` on the arrows) or refused where no row can carry
    /// the loss of productivity.
    fn check_productivity(&self, name: &str, scheme: &Scheme, eqs: &[ast::Equation]) -> Result<()> {
        let key = (self.current_module.clone(), name.to_string());
        if self.corec.contains(&key) {
            let group = self.rec_group(name);
            for e in eqs {
                let mut bound = Vec::new();
                for p in &e.pats {
                    pat_names(p, &mut bound);
                }
                self.walk_guard(
                    &e.body,
                    GuardPos::default(),
                    &group,
                    &GuardMode::Corec,
                    &mut bound,
                )?;
            }
            for e in eqs {
                self.construction_layer(&e.body)?;
            }
            return Ok(());
        }
        if eqs.is_empty() || !self.produces_codata(scheme, eqs) {
            return Ok(());
        }
        // The body runs at the last application: the loss of productivity,
        // if any, surfaces there (decision D3's effect placement).
        let mut diverge = false;
        let mut t = &scheme.ty;
        loop {
            match t {
                Ty::Fun(_, _, e, b) | Ty::Pi { eff: e, cod: b, .. } => {
                    diverge = e.labels.iter().any(|l| l.head == "Diverge");
                    t = b;
                }
                _ => break,
            }
        }
        let group = self.rec_group(name);
        for e in eqs {
            let mut bound = Vec::new();
            for p in &e.pats {
                pat_names(p, &mut bound);
            }
            self.walk_guard(
                &e.body,
                GuardPos::default(),
                &group,
                &GuardMode::Ordinary { diverge },
                &mut bound,
            )?;
        }
        Ok(())
    }

    /// The guard check's construction layer: a `corec` body must build its
    /// codata type at every step, so each recursion level produces its
    /// observations before the next call (design section 3).
    fn construction_layer(&self, t: &ast::Term) -> Result<()> {
        match t {
            ast::Term::Record(path, _, _) if self.is_codata_path(path) => Ok(()),
            ast::Term::Ann(e, _, _) => self.construction_layer(e),
            ast::Term::If { then: a, els: b, .. } => {
                self.construction_layer(a)?;
                self.construction_layer(b)
            }
            ast::Term::Case(_, alts, _) => {
                for alt in alts {
                    self.construction_layer(&alt.body)?;
                }
                Ok(())
            }
            ast::Term::Let(_, body, _) => self.construction_layer(body),
            other => Err(Error::at(
                other.span(),
                "a `corec` body must build its codata type at every step (an observation layer per recursion level); this body does not construct a codata value (core language specification section 9)",
            )),
        }
    }

    /// The syntactic guard walk (design section 3): recursive calls are
    /// allowed only inside `delay` thunks that flow to a `Later`-typed
    /// observation.
    fn walk_guard(
        &self,
        t: &ast::Term,
        pos: GuardPos,
        group: &[String],
        mode: &GuardMode,
        bound: &mut Vec<String>,
    ) -> Result<()> {
        let call = |p: &ast::Name, pos: GuardPos| -> Result<()> {
            if group.iter().all(|g| g != &p.text) {
                return Ok(());
            }
            match mode {
                GuardMode::Corec => {
                    if !pos.guarded {
                        return Err(Error::at(
                            p.span,
                            format!(
                                "unguarded recursive call to `{}`: a `corec` definition must place every recursive call under `delay` in a `Later`-typed observation (core language specification section 9)",
                                p.text
                            ),
                        ));
                    }
                    Ok(())
                }
                GuardMode::Ordinary { diverge } => {
                    if pos.in_delay {
                        // Under a `delay` thunk: the thunk's own type
                        // records what forcing performs (a `Later` body
                        // must be pure; a `Thunk {e}` carries `e`).
                        return Ok(());
                    }
                    if pos.in_obs {
                        return Err(Error::at(
                            p.span,
                            format!(
                                "unguarded recursive call to `{}` in a codata observation: an observation must produce its value in finite time, so an unproductive search cannot run here; compute it before construction or defer it in a `Thunk {{Diverge}}` observation (core language specification section 9)",
                                p.text
                            ),
                        ));
                    }
                    if !*diverge {
                        return Err(Error::at(
                            p.span,
                            format!(
                                "unguarded recursive call to `{}`: this definition is not productive, so its type must reflect the loss of productivity: declare `Diverge` on its arrows (as in `... ->{{Diverge}} ...`), or guard the call under `delay` in a `Later` observation (core language specification section 9)",
                                p.text
                            ),
                        ));
                    }
                    Ok(())
                }
            }
        };
        match t {
            ast::Term::Var(p) => {
                if p.quals.is_empty() && !bound.contains(&p.name.text) {
                    call(&p.name, pos)?;
                }
            }
            ast::Term::Con(_)
            | ast::Term::Int(_, _)
            | ast::Term::Str(_, _)
            | ast::Term::Char(_, _) | ast::Term::Sym(_, _)
            | ast::Term::Unit(_)
            | ast::Term::Hole(_)
            | ast::Term::SpliceHole { .. }
            | ast::Term::MacroCall { .. }
            | ast::Term::QuoteDecls { .. }
            | ast::Term::Quote { .. }
            | ast::Term::Splice { .. }
            | ast::Term::SeqOp(_, _) => {}
            ast::Term::App(f, args, _) => {
                // `delay (\() -> e)` is the guard form when `delay` is the
                // builtin (unshadowed); its lambda body runs at force.
                if let ast::Term::Var(p) = &**f {
                    if p.quals.is_empty()
                        && p.name.text == "delay"
                        && !bound.contains(&p.name.text)
                        && args.len() == 1
                        && self.lookup_value(None, "delay").is_err()
                    {
                        let arg_pos = GuardPos { delay_arg: true, ..pos };
                        return self.walk_guard(&args[0], arg_pos, group, mode, bound);
                    }
                }
                self.walk_guard(f, pos, group, mode, bound)?;
                for a in args {
                    self.walk_guard(a, pos, group, mode, bound)?;
                }
            }
            ast::Term::Lam(pats, body, _) => {
                let body_pos = if pos.delay_arg {
                    GuardPos {
                        delay_arg: false,
                        in_delay: true,
                        guarded: pos.guarded || (pos.in_obs && pos.obs_later),
                        ..pos
                    }
                } else {
                    GuardPos { delay_arg: false, ..pos }
                };
                let mut bound2 = bound.clone();
                for p in pats {
                    pat_names(p, &mut bound2);
                }
                self.walk_guard(body, body_pos, group, mode, &mut bound2)?;
            }
            ast::Term::Let(decls, body, _) | ast::Term::Letrec(decls, body, _) => {
                for d in decls {
                    let mut bound2 = bound.clone();
                    match d {
                        ast::LocalDecl::Sig { .. } => continue,
                        ast::LocalDecl::Eq { name: n, pats, body: b, .. } => {
                            bound2.push(n.text.clone());
                            for p in pats {
                                pat_names(p, &mut bound2);
                            }
                            self.walk_guard(b, pos, group, mode, &mut bound2)?;
                        }
                        ast::LocalDecl::Bind { pat, body: b, .. }
                        | ast::LocalDecl::Lazy { pat, body: b, .. } => {
                            pat_names(pat, &mut bound2);
                            self.walk_guard(b, pos, group, mode, &mut bound2)?;
                        }
                    }
                }
                let mut bound2 = bound.clone();
                for d in decls {
                    match d {
                        ast::LocalDecl::Sig { .. } => {}
                        ast::LocalDecl::Eq { name: n, .. } => bound2.push(n.text.clone()),
                        ast::LocalDecl::Bind { pat, .. } | ast::LocalDecl::Lazy { pat, .. } => {
                            pat_names(pat, &mut bound2)
                        }
                    }
                }
                self.walk_guard(body, pos, group, mode, &mut bound2)?;
            }
            ast::Term::If { cond: c, then: a, els: b, .. } => {
                self.walk_guard(c, pos, group, mode, bound)?;
                self.walk_guard(a, pos, group, mode, bound)?;
                self.walk_guard(b, pos, group, mode, bound)?;
            }
            ast::Term::Case(scrut, alts, _) => {
                self.walk_guard(scrut, pos, group, mode, bound)?;
                for alt in alts {
                    if let Some(g) = &alt.guard {
                        self.walk_guard(g, pos, group, mode, bound)?;
                    }
                    let mut bound2 = bound.clone();
                    pat_names(&alt.pat, &mut bound2);
                    self.walk_guard(&alt.body, pos, group, mode, &mut bound2)?;
                }
            }
            ast::Term::Ann(e, _, _) | ast::Term::Neg(e, _) | ast::Term::Proj(e, _, _) => {
                self.walk_guard(e, pos, group, mode, bound)?
            }
            ast::Term::Forget { value, .. } => {
                self.walk_guard(value, pos, group, mode, bound)?
            }
            ast::Term::Narrow { value, witness, .. } => {
                self.walk_guard(value, pos, group, mode, bound)?;
                self.walk_guard(witness, pos, group, mode, bound)?
            }
            ast::Term::Infix(terms, ops, _) => {
                for o in ops {
                    if !bound.contains(&o.text) {
                        call(o, pos)?;
                    }
                }
                for x in terms {
                    self.walk_guard(x, pos, group, mode, bound)?;
                }
            }
            ast::Term::Tuple(items, _) | ast::Term::List(items, _) => {
                for x in items {
                    self.walk_guard(x, pos, group, mode, bound)?;
                }
            }
            ast::Term::Map(kvs, _) => {
                for (k, v) in kvs {
                    self.walk_guard(k, pos, group, mode, bound)?;
                    self.walk_guard(v, pos, group, mode, bound)?;
                }
            }
            ast::Term::Range(start, step, end, _) => {
                self.walk_guard(start, pos, group, mode, bound)?;
                if let Some(x) = step {
                    self.walk_guard(x, pos, group, mode, bound)?;
                }
                self.walk_guard(end, pos, group, mode, bound)?;
            }
            ast::Term::Record(path, fields, _) => {
                // A codata observation's value runs at observation time:
                // it is a fresh closure, and a `Later`-typed one is the
                // guarded position of the discipline.
                let codata = self.is_codata_path(path);
                for (fname, fval) in fields {
                    let fpos = if codata {
                        GuardPos {
                            delay_arg: false,
                            in_obs: true,
                            obs_later: self.obs_is_later(path, &fname.text),
                            ..pos
                        }
                    } else {
                        GuardPos { delay_arg: false, ..pos }
                    };
                    self.walk_guard(fval, fpos, group, mode, bound)?;
                }
            }
            ast::Term::Do(stmts, _) => {
                for s in stmts {
                    match s {
                        ast::DoStmt::Bind { pat, term, .. } => {
                            let mut bound2 = bound.clone();
                            pat_names(pat, &mut bound2);
                            self.walk_guard(term, pos, group, mode, &mut bound2)?;
                        }
                        ast::DoStmt::Let { decls, .. } => {
                            for decl in decls {
                                match decl {
                                    ast::LocalDecl::Sig { .. } => {}
                                    ast::LocalDecl::Eq { name: n, body: b, .. } => {
                                        let mut bound2 = bound.clone();
                                        bound2.push(n.text.clone());
                                        self.walk_guard(b, pos, group, mode, &mut bound2)?;
                                    }
                                    ast::LocalDecl::Bind { pat, body: b, .. }
                                    | ast::LocalDecl::Lazy { pat, body: b, .. } => {
                                        let mut bound2 = bound.clone();
                                        pat_names(pat, &mut bound2);
                                        self.walk_guard(b, pos, group, mode, &mut bound2)?;
                                    }
                                }
                            }
                        }
                        ast::DoStmt::Expr { term, .. } => {
                            self.walk_guard(term, pos, group, mode, bound)?
                        }
                    }
                }
            }
            ast::Term::Handle(e, h, _) => {
                self.walk_guard(e, pos, group, mode, bound)?;
                self.walk_guard(h, pos, group, mode, bound)?;
            }
            ast::Term::Handler(h) => self.walk_handler(h, pos, group, mode, bound)?,
            ast::Term::With { name: _, value, body, .. } => {
                self.walk_guard(value, pos, group, mode, bound)?;
                self.walk_guard(body, pos, group, mode, bound)?;
            }
            ast::Term::CapApply { head, supplies, .. } => {
                self.walk_guard(head, pos, group, mode, bound)?;
                for (_, x) in supplies {
                    self.walk_guard(x, pos, group, mode, bound)?;
                }
            }
            ast::Term::CpsWrap(body, _) => self.walk_guard(body, pos, group, mode, bound)?,
        }
        Ok(())
    }

    fn walk_handler(
        &self,
        h: &ast::HandlerExpr,
        pos: GuardPos,
        group: &[String],
        mode: &GuardMode,
        bound: &mut Vec<String>,
    ) -> Result<()> {
        let mut bound2 = bound.clone();
        pat_names(&h.ret_pat, &mut bound2);
        self.walk_guard(&h.ret_body, pos, group, mode, &mut bound2)?;
        for c in &h.clauses {
            let mut bound2 = bound.clone();
            for p in &c.pats {
                pat_names(p, &mut bound2);
            }
            if let Some(k) = &c.resume {
                bound2.push(k.text.clone());
            }
            self.walk_guard(&c.body, pos, group, mode, &mut bound2)?;
        }
        Ok(())
    }

    fn check_equations(
        &mut self,
        name: &str,
        id: &GlobalId,
        scheme: &Scheme,
        eqs: &[ast::Equation],
        extra_dicts: Vec<(LocalId, ClassApp)>,
    ) -> Result<()> {
        // The top-level recording marker belongs to this entry alone:
        // a nested check (a staged unit compiled on demand) is not a
        // top-level value binding of its own.
        let record_top = std::mem::take(&mut self.record_top_value);
        self.check_productivity(name, scheme, eqs)?;
        let logical = self
            .logical
            .contains(&(self.current_module.clone(), name.to_string()));
        self.current_logical = if logical {
            Some((self.current_module.clone(), name.to_string()))
        } else {
            None
        };
        self.pending_refs.clear();
        let (ty, ctx, _map) = self.rigidify(scheme);
        self.reset_usage();
        self.smaller.clear();
        // P20: a fresh safety fact collector per definition (attribute
        // annotations specification section 10).
        self.cur_runtime_decides.clear();
        self.cur_def_deny = self
            .safety_attrs
            .get(&id.0)
            .map(|s| s.deny_warnings.is_some())
            .unwrap_or(false);
        let mut env = Env::default();
        let mut param_ids = Vec::new();
        // Optimizer metadata (supercompilation and JIT specification
        // sections 3-4): which parameters are dictionaries, carry a usage
        // multiplicity, or are honest pure callbacks.
        let mut dict_meta: Vec<(LocalId, String)> = Vec::new();
        // Module parameters are the definition's leading runtime
        // parameters (module parameters as module arguments): each is an
        // ordinary binding of the module scope, a dictionary parameter
        // joins the dictionary scope (a provider-head parameter selects
        // the module's `do` provider, decision D22), and a value
        // parameter is shared evidence (`Copy`, checked at declaration).
        self.cur_mod_params.clear();
        let mparams = self
            .module_params
            .get(&self.current_module)
            .cloned()
            .unwrap_or_default();
        for mp in &mparams {
            let l = self.fresh_local();
            if let Some(c) = &mp.class {
                env.dicts.push((l, c.clone()));
                dict_meta.push((l, c.class.clone()));
            }
            self.bind_local(&mut env, &mp.name, l, Scheme::mono(mp.ty.clone()), None, eqs[0].span)?;
            self.register_obligation(l, &mp.name, Some(Mult::Many), mp.ty.clone(), eqs[0].span);
            self.cur_mod_params.push((mp.name.clone(), l, mp.ty.clone()));
            param_ids.push(l);
        }
        for (l, c) in extra_dicts {
            env.dicts.push((l, c.clone()));
            dict_meta.push((l, c.class.clone()));
            param_ids.push(l);
        }
        for (n, c) in &ctx {
            let l = self.fresh_local();
            env.dicts.push((l, c.clone()));
            dict_meta.push((l, c.class.clone()));
            if is_capability_class(&c.class) {
                // A capability binder is a term-level value too: `f @{io =
                // io}` and `with (io = io) in ...` name it (decision D21).
                let ty = Ty::Adt(c.class.clone(), c.args.clone());
                self.bind_local(&mut env, n, l, Scheme::mono(ty), None, eqs[0].span)?;
            }
            param_ids.push(l);
        }
        // Implicit value arguments (`forall (n :: Nat)`) are runtime
        // parameters bound by name (decision D29); proposition binders are
        // erased and take no runtime slot.
        for (n, t) in &scheme.val_params {
            let l = self.fresh_local();
            self.bind_local(&mut env, n, l, Scheme::mono(t.clone()), None, eqs[0].span)?;
            self.register_obligation(l, n, Some(Mult::Many), t.clone(), eqs[0].span);
            param_ids.push(l);
        }
        for n in &scheme.prop_params {
            let l = self.fresh_local();
            self.bind_local(
                &mut env,
                n,
                l,
                Scheme::mono(Ty::Prp(Box::new(PTerm::Var(n.clone())))),
                None,
                eqs[0].span,
            )?;
            self.register_obligation(
                l,
                n,
                Some(Mult::Zero),
                Ty::Prp(Box::new(PTerm::Var(n.clone()))),
                eqs[0].span,
            );
        }
        self.seq_binders = env
            .dicts
            .iter()
            .filter(|(_, c)| SEQ_CLASSES.contains(&c.class.as_str()))
            .map(|(_, c)| c.clone())
            .collect();
        let n_pats = eqs[0].pats.len();
        for e in eqs {
            if e.pats.len() != n_pats {
                return Err(Error::at(
                    e.span,
                    "all equations of a definition must take the same number of parameters",
                ));
            }
        }
        let mut doms = Vec::new();
        let mut mults = Vec::new();
        let mut effs = Vec::new();
        let mut rest = ty;
        for i in 0..n_pats {
            rest = self.uni.zonk(&rest);
            // A dependent parameter binds its name in the remaining type;
            // the equation's pattern names it at runtime (the surface
            // shares term syntax, so the names link the two).
            match rest {
                Ty::Fun(a, m, eff, b) => {
                    doms.push(*a);
                    mults.push(m);
                    effs.push(eff);
                    rest = *b;
                }
                Ty::Pi { var, dom, mult, eff, cod } => {
                    let pat_name = match &eqs[0].pats[i] {
                        ast::Pat::Var(n, _) => n.text.clone(),
                        _ => format!("$dep{i}"),
                    };
                    let mut later = eqs[1..].iter().filter_map(|e| match &e.pats[i] {
                        ast::Pat::Var(n, _) if n.text == pat_name => Some(()),
                        _ => None,
                    });
                    if eqs.len() > 1 && later.count() != eqs.len() - 1 {
                        return Err(Error::at(
                            eqs[0].pats[i].span(),
                            "a dependent parameter must be a variable pattern with the same name in every equation",
                        ));
                    }
                    doms.push(*dom);
                    mults.push(mult);
                    effs.push(eff);
                    rest = cod.subst_idx(&var, &ITerm::Var(pat_name));
                }
                other => {
                    return Err(Error::at(
                        eqs[0].span,
                        format!("{name} has more parameters than its signature ({other})"),
                    ));
                }
            }
        }
        if n_pats > 0
            && matches!(self.uni.zonk(&rest), Ty::Fun(_, _, _, _) | Ty::Pi { .. })
        {
            return Err(Error::at(
                eqs[0].span,
                format!("{name} has fewer parameters than its signature"),
            ));
        }
        let mut value_params = Vec::new();
        for _ in 0..n_pats {
            value_params.push(self.fresh_local());
        }
        // Erased parameters (evidence, types) take no runtime slot (core
        // specification section 3).
        for (i, l) in value_params.iter().enumerate() {
            if !matches!(self.uni.zonk(&doms[i]), Ty::Prp(_) | Ty::Sort(_)) {
                param_ids.push(*l);
            }
        }
        // Usage obligations for the parameters: the arrow's declared
        // multiplicity, with the parameter's source name for diagnostics
        // (ownership and usage design, section 2).
        for (i, l) in value_params.iter().enumerate() {
            let mut pname = "_".to_string();
            for e in eqs {
                if let ast::Pat::Var(n, _) = &e.pats[i] {
                    pname = n.text.clone();
                    break;
                }
            }
            self.register_obligation(
                *l,
                &pname,
                Some(mults[i]),
                doms[i].clone(),
                eqs[0].pats[i].span(),
            );
        }
        // Clause weights (attribute annotations specification section
        // 2): every clause of the group is one alternative of the
        // function's match, all-or-nothing.
        let mut clause_weights: Vec<Option<u32>> = Vec::new();
        for e in eqs {
            let set = self.validate_attr_site(&e.attrs, AttrSite::Clause)?;
            clause_weights.push(set.weight);
        }
        self.complete_weights(
            &mut clause_weights,
            &format!("the clauses of {name}"),
            eqs[0].span,
        )?;
        let rows: Vec<Row> = eqs
            .iter()
            .enumerate()
            .map(|(i, e)| {
                Ok(Row {
                    pats: e
                        .pats
                        .iter()
                        .map(|p| self.expand_pat(p))
                        .collect::<Result<Vec<_>>>()?,
                    guard: None,
                    body: wrap_eq_body(e),
                    env: env.clone(),
                    idx: i,
                    consumed: Vec::new(),
                    weight: clause_weights[i],
                    guard_weight: None,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let covered = self.matrix_covered(&rows, &doms)?;
        // Optimizer metadata for this code generation unit: usage
        // multiplicities of the runtime parameters, honest pure callback
        // parameters, and capability dictionaries (supercompilation and
        // JIT specification sections 3-4). The unit's own purity is set
        // once the body's performed effects are known.
        let mut meta = crate::core::FnMeta::default();
        for (i, l) in value_params.iter().enumerate() {
            if let Some(pos) = param_ids.iter().position(|p| p == l) {
                if matches!(mults[i], Mult::One | Mult::Aff) {
                    meta.linear.push((pos, mults[i]));
                }
                if pure_arrow_ty(&doms[i]) {
                    meta.pure_arrows.push(pos);
                }
            }
        }
        for (l, c) in &dict_meta {
            if let Some(pos) = param_ids.iter().position(|p| p == l) {
                meta.dicts.push((pos, c.clone()));
            }
        }
        // The body runs at the last application, so its effects belong to
        // the last arrow; intermediate arrows are pure closure construction,
        // and a nullary definition is pure.
        let doms_safety = doms.clone();
        let saved = self.enter_perf();
        let body = if n_pats == 0 {
            self.check(&mut env.clone(), &rows[0].body, &rest)?
        } else {
            self.compile_rows(
                rows,
                value_params.iter().map(|l| ECore::Var(*l)).collect(),
                doms,
                &rest,
                eqs[0].span,
                covered,
            )?
        };
        let performed = self.exit_perf(saved);
        if name.starts_with('$') {
            // A `generate do` block performs compile-time effects under its
            // capabilities (modules specification section 10): its
            // performed effects are recorded in the build identity, not
            // declared on a signature.
        } else {
        match effs.split_last() {
            Some((last, earlier)) => {
                for e in earlier {
                    self.uni
                        .unify_rows(e, &EffRow::pure())
                        .map_err(|m| Error::at(eqs[0].span, m))?;
                }
                self.uni
                    .unify_rows(last, &performed)
                    .map_err(|m| Error::at(eqs[0].span, m))?;
            }
            None => {
                self.uni
                    .unify_rows(&EffRow::pure(), &performed)
                    .map_err(|m| {
                        Error::at(
                            eqs[0].span,
                            format!(
                                "{m} (a nullary definition must be pure in the bootstrap slice)"
                            ),
                        )
                    })?;
            }
        }
        }
        self.check_usage()?;
        meta.pure = self.uni.resolve_row(&performed).is_pure();
        // P20: the safety assertions are verified at the definition
        // (attribute annotations specification section 10; decision
        // D112): a violation is a compile error naming the attribute,
        // the rule, and the offending site. The signature's value facts
        // (the `!range`/`noundef`/`nonnull`/`align` of the LLVM
        // contract surface) come from the types themselves.
        {
            let module = self.current_module.clone();
            let mut sig_tys: Vec<Ty> = doms_safety.iter().map(|t| self.uni.zonk(t)).collect();
            sig_tys.push(self.uni.zonk(&rest));
            meta.contract.result_range = sig_tys.last().and_then(|t| t.value_range());
            let mut pcs = vec![crate::core::ParamContract::default(); param_ids.len()];
            for (i, l) in value_params.iter().enumerate() {
                if let Some(pos) = param_ids.iter().position(|p| p == l) {
                    if let Some(t) = sig_tys.get(i) {
                        pcs[pos] = self.param_contract(t);
                    }
                }
            }
            meta.contract.params = pcs;
            self.verify_definition_safety(
                &id.0,
                &module,
                name,
                eqs,
                &sig_tys,
                logical,
                eqs[0].span,
            )?;
            // Temporal contracts (core specification section 10) verify
            // with the definition: their parameter scope is prepared
            // here, the per-step checks run after constraint solving.
            self.prepare_temporal(&module, name, eqs, &sig_tys)?;
        }
        if logical {
            // The logical fragment is total, pure and erased (core
            // specification sections 3 and 8): a looping function cannot
            // inhabit a `Prop`.
            if !self.uni.resolve_row(&performed).is_pure() {
                return Err(Error::at(
                    eqs[0].span,
                    format!(
                        "{name} is a `spec`/`proof` definition and must be pure: the logical fragment cannot perform effects"
                    ),
                ));
            }
            self.check_termination(id, &value_params, &body, eqs[0].span, name)?;
            self.logical_refs.insert(
                (self.current_module.clone(), name.to_string()),
                std::mem::take(&mut self.pending_refs),
            );
            self.current_logical = None;
            return self.finish_fn(id.clone(), param_ids, body, None, None, true, meta);
        }
        self.pending_refs.clear();
        self.current_logical = None;
        if record_top {
            self.collect_top_bind(
                id,
                name,
                eqs[0].span,
                param_ids.is_empty() && n_pats == 0,
                &body,
            );
        }
        self.finish_fn(id.clone(), param_ids, body, None, None, false, meta)
    }

    /// Structural termination for the logical fragment (core
    /// specification sections 2 and 8): every recursive call must take an
    /// argument that is structurally smaller than a named parameter, with
    /// no argument outside the parameter's subcomponent tree. A call that
    /// does not is an unsupported termination argument, never silently
    /// accepted.
    fn check_termination(
        &self,
        id: &GlobalId,
        params: &[LocalId],
        body: &ECore,
        span: ast::Span,
        name: &str,
    ) -> Result<()> {
        fn chain(
            smaller: &HashMap<LocalId, LocalId>,
            extra: &HashMap<LocalId, LocalId>,
            mut x: LocalId,
        ) -> Vec<LocalId> {
            let mut out = vec![x];
            loop {
                let next = extra.get(&x).or_else(|| smaller.get(&x)).copied();
                match next {
                    Some(p) => {
                        out.push(p);
                        x = p;
                    }
                    None => break,
                }
            }
            out
        }
        fn walk(
            e: &ECore,
            id: &GlobalId,
            smaller: &HashMap<LocalId, LocalId>,
            extra: &mut HashMap<LocalId, LocalId>,
            params: &[LocalId],
            bad: &mut Vec<String>,
        ) {
            match e {
                ECore::Global(g) if g == id => {
                    bad.push("a recursive use with no decreasing argument".to_string());
                }
                ECore::App(f, args) => {
                    if let ECore::Global(g) = &**f {
                        if g == id {
                            let mut decreased = false;
                            for a in args {
                                match a {
                                    // Dictionaries and solved index
                                    // arguments are not termination
                                    // candidates.
                                    ECore::Hole(_) | ECore::QuoteHole(_) | ECore::IdxPending(_) | ECore::Unit => {}
                                    ECore::Var(x) => {
                                        let c = chain(smaller, extra, *x);
                                        let mut ok = false;
                                        for p in params {
                                            if c.contains(p) {
                                                ok = true;
                                                if c[0] != *p {
                                                    decreased = true;
                                                }
                                            }
                                        }
                                        if !ok {
                                            bad.push(
                                                "an argument outside the parameters' subcomponent tree"
                                                    .to_string(),
                                            );
                                        }
                                    }
                                    _ => {
                                        bad.push(
                                            "a recursive call whose argument is not a variable"
                                                .to_string(),
                                        );
                                    }
                                }
                            }
                            if !decreased {
                                bad.push("no argument decreases structurally".to_string());
                            }
                        }
                    }
                    // The head of a recursive call is handled above; a
                    // bare self-reference is only a use outside one.
                    match &**f {
                        ECore::Global(g) if g == id => {}
                        _ => walk(f, id, smaller, extra, params, bad),
                    }
                    for a in args {
                        walk(a, id, smaller, extra, params, bad);
                    }
                }
                ECore::DictProj(d, _) | ECore::Proj(d, _) | ECore::Demand(d) => {
                    walk(d, id, smaller, extra, params, bad)
                }
                ECore::InstDict(_, args) | ECore::MkClos { env: args, .. } => {
                    for a in args {
                        walk(a, id, smaller, extra, params, bad);
                    }
                }
                ECore::Lam { body, .. } => walk(body, id, smaller, extra, params, bad),
                ECore::Let(l, v, b) => {
                    walk(v, id, smaller, extra, params, bad);
                    // `let x = y` (a match scrutinee binding, for example)
                    // extends the subcomponent chain rather than breaking
                    // it.
                    let synonym = match &**v {
                        ECore::Var(src) => {
                            extra.insert(*l, *src);
                            true
                        }
                        _ => false,
                    };
                    walk(b, id, smaller, extra, params, bad);
                    if synonym {
                        extra.remove(l);
                    }
                }
                ECore::If(c, t, f) => {
                    walk(c, id, smaller, extra, params, bad);
                    walk(&t.body, id, smaller, extra, params, bad);
                    walk(&f.body, id, smaller, extra, params, bad);
                }
                ECore::LazyInit(_, t) => walk(t, id, smaller, extra, params, bad),
                ECore::Case { scrut, alts } => {
                    walk(scrut, id, smaller, extra, params, bad);
                    for a in alts {
                        walk(&a.body, id, smaller, extra, params, bad);
                    }
                }
                _ => {}
            }
        }
        let mut bad = Vec::new();
        let mut extra = HashMap::new();
        walk(body, id, &self.smaller, &mut extra, params, &mut bad);
        if !bad.is_empty() {
            return Err(Error::at(
                span,
                format!(
                    "{name} is a `spec`/`proof` definition and must terminate: {} (obligation status: unsupported; supply a structural decrease)",
                    bad[0]
                ),
            ));
        }
        Ok(())
    }

    /// Solve the equation's dictionary wants, reify implicit index
    /// arguments, hoist lambdas and emit the code generation unit plus its
    /// hoisted lambdas. Logical definitions are checked and erased: they
    /// produce no code (core specification section 3).
    fn finish_fn(
        &mut self,
        id: GlobalId,
        param_ids: Vec<LocalId>,
        mut body: ECore,
        prim: Option<PrimOp>,
        ctor: Option<(usize, usize)>,
        erased: bool,
        meta: crate::core::FnMeta,
    ) -> Result<()> {
        let sol = self.solve_wants()?;
        // Evidence invalidation over mutable state: regions unify during
        // constraint solving, so the deferred staleness check runs here.
        self.check_ev_uses()?;
        // Temporal contracts (core specification section 10): the
        // per-step obligations read the same resolved write log.
        self.check_temporal(&id)?;
        body = self.fill_holes(&body, &sol)?;
        let mut recs: Vec<(GlobalId, ECore)> = Vec::new();
        for (mid, core) in std::mem::take(&mut self.pending_rec) {
            recs.push((mid, self.fill_holes(&core, &sol)?));
        }
        // Deferred literal words and implicit index arguments resolve
        // positionally across the body and its `letrec` member units at
        // once (their indices are shared).
        let lits = self.resolved_literals()?;
        let vals = if erased {
            self.idx_pends.clear();
            Vec::new()
        } else {
            self.resolved_idx_args()?
        };
        resolve_lit_words(&mut body, &lits);
        resolve_idx_words(&mut body, &vals);
        for (_, core) in recs.iter_mut() {
            resolve_lit_words(core, &lits);
            resolve_idx_words(core, &vals);
        }
        // The collection literal fold (surface grammar local amendments
        // section 10) runs on every emitted body once its constants are
        // fixed and before lambdas hoist.
        if !erased {
            for (mid, core) in recs.iter_mut() {
                let mid = mid.0.clone();
                self.fold_pass(core, &mid, true);
            }
            self.fold_pass(&mut body, &id.0.clone(), true);
        }
        let mut hoisted = Vec::new();
        hoist_lambdas(&mut body, &id.0, &mut self.lam_counter, &mut hoisted);
        // `letrec` member units (surface grammar local amendments
        // section 9): their bodies arrive with their final unit ids, so
        // only their nested lambdas hoist like any others.
        for (mid, core) in recs {
            let (params, mbody) = match core {
                ECore::Lam { params, body } => (params, *body),
                _ => {
                    return Err(Error::plain(
                        "internal error: a letrec member is not a lambda",
                    ))
                }
            };
            let mut mbody = mbody;
            hoist_lambdas(&mut mbody, &id.0, &mut self.lam_counter, &mut hoisted);
            let mut free = Vec::new();
            free_locals(&mbody, &mut Vec::new(), &mut free);
            let mut env: Vec<LocalId> =
                free.into_iter().filter(|l| !params.contains(l)).collect();
            env.sort_by_key(|l| l.0);
            let mut fn_params = env.clone();
            fn_params.extend(params.iter().copied());
            hoisted.push(HoistedFn {
                id: mid,
                params: fn_params,
                body: mbody,
            });
        }
        if !erased {
            // Hint attributes ride the function's code generation unit
            // (attribute annotations specification section 5).
            let hints = self.fn_hints.get(&id.0).copied().unwrap_or_default();
            self.fns.push(finalize_fn(id, param_ids, body, prim, ctor, hints, meta));
            for h in hoisted {
                self.fns.push(finalize_fn(
                    h.id,
                    h.params,
                    h.body,
                    None,
                    None,
                    crate::core::FnHints::default(),
                    crate::core::FnMeta::default(),
                ));
            }
        }
        Ok(())
    }

    /// The dotted presentation forms of the primitive spec (decision D6)
    /// and of the P10 target library: `U32.wrapping-add` for
    /// `wrapping-add-u32`, `Stream.fold` for the producer library's fold,
    /// and the scalar/SIMD pair's qualified spellings. The method part of
    /// a dotted spelling follows the D106 method spelling (`bit-and`),
    /// so it matches the prelude name it presents.
    fn install_presentation_aliases(&mut self) {
        // The dotted presentation spellings of the primitive types
        // specification (`U32.wrappingAdd` and friends) alias the
        // prelude surface's flat names; the alias targets the surface's
        // home definition, so the split's module layout is invisible
        // here.
        let aliases: [(&str, &str, &str); 9] = [
            ("U32", "zero", "zero-u32"),
            ("U32", "wrapping-add", "wrapping-add-u32"),
            ("Vec", "stream", "vec-stream"),
            ("Stream", "fold", "prod-fold"),
            ("Stream", "map", "prod-map"),
            ("Scalar", "sum-wrap32", "scalar-sum-wrap32"),
            ("Simd", "sum-wrap32", "simd-sum-wrap32"),
            ("Target", "detect-vec-u32", "detect-vec-u32"),
            ("SumProofs", "scalar-to-simd", "scalar-to-simd"),
        ];
        for (qm, qn, n) in aliases {
            let hit = self
                .surface_def(&self.prelude_module.clone(), n)
                .and_then(|d| self.qualified.get(&(d.module.clone(), d.name.clone())).cloned());
            if let Some(v) = hit {
                self.qualified.insert((qm.to_string(), qn.to_string()), v);
            }
        }
    }

    /// Check every rewrite declaration in source order (modules and
    /// compilation specification section 5). The declarations reference
    /// their module's names, so they are checked after the ordinary
    /// declarations.
    fn check_rewrites(&mut self) -> Result<()> {
        let pending = std::mem::take(&mut self.pending_rewrites);
        for (module, r) in pending {
            self.current_module = module.clone();
            self.visible = vec![module.clone()];
            if let Some(imports) = self.module_imports.get(&module).cloned() {
                for i in imports {
                    if !self.visible.contains(&i) {
                        self.visible.push(i);
                    }
                }
            }
            let prelude = self.prelude_module.clone();
            if !self.visible.contains(&prelude) {
                self.visible.push(prelude);
            }
            self.check_rewrite(&r)?;
        }
        Ok(())
    }

    /// Check the pending upgrade declarations (runtime evolution
    /// specification sections 2-3): migration field checking over the two
    /// records' field tables, and the compatibility evidence recorded in
    /// the interface. Checked after every module's records and functions
    /// are known.
    fn check_upgrades(&mut self) -> Result<()> {
        let pending = std::mem::take(&mut self.pending_upgrades);
        for (module, u) in pending {
            self.current_module = module.clone();
            self.visible = vec![module.clone()];
            if let Some(imports) = self.module_imports.get(&module).cloned() {
                for i in imports {
                    if !self.visible.contains(&i) {
                        self.visible.push(i);
                    }
                }
            }
            let prelude = self.prelude_module.clone();
            if !self.visible.contains(&prelude) {
                self.visible.push(prelude);
            }
            self.check_upgrade(&u)?;
        }
        Ok(())
    }

    /// Check every `prim` declaration in source order (primitive
    /// operations and inline target IR specification sections 5-8).
    /// Each body's `when` condition lowers against the target profile's
    /// capability evidence, its `justify` evidence checks against the
    /// prim's own equation, the tier follows (the weakest body governs
    /// the contract), and one body is selected per the artifact
    /// (selection is recorded in the manifest, never in the contract
    /// projection). The synthesized unit joins the program as a leaf.
    fn check_prims(&mut self) -> Result<()> {
        let pending = std::mem::take(&mut self.pending_prims);
        for (module, p, s) in pending {
            self.current_module = module.clone();
            self.visible = vec![module.clone()];
            if let Some(imports) = self.module_imports.get(&module).cloned() {
                for i in imports {
                    if !self.visible.contains(&i) {
                        self.visible.push(i);
                    }
                }
            }
            let prelude = self.prelude_module.clone();
            if !self.visible.contains(&prelude) {
                self.visible.push(prelude);
            }
            let unit = fn_id(&module, &p.name.text).0;
            let arity = arity_of_arrows(&s.ty);
            let (arg_tys, res_ty) = self.check_prim_envelope(&p, &s.ty)?;
            let pure = !prim_effectful(&s.ty);
            let mut bodies: Vec<crate::core::PrimBodyPlan> = Vec::new();
            let mut conds: Vec<Option<WhenCond>> = Vec::new();
            let mut laws: Vec<String> = Vec::new();
            let mut all_justified = true;
            for b in &p.bodies {
                let (validation, uses) =
                    self.validate_prim_body(&module, &p, b, &arg_tys, &res_ty, pure)?;
                let cond = match &b.when {
                    Some(w) => Some(self.lower_when(w)?),
                    None => None,
                };
                let assumptions = match &b.justify {
                    Some(j) => self.check_prim_justify(j, &unit, arity, b.span)?,
                    None => {
                        all_justified = false;
                        Vec::new()
                    }
                };
                for a in &assumptions {
                    if !laws.contains(a) {
                        laws.push(a.clone());
                    }
                }
                // The `unsafe` trust (section 4): a body containment
                // cannot verify has its declared row trusted, not
                // verified, and the trust is a recorded assumption with
                // a per-body identity (body edits stale dependents).
                let trusted = b.unverified;
                let trust_id = if trusted {
                    format!(
                        "prim:{unit}#{}",
                        crate::interface::content_hash(&format!(
                            "{}\n{}\n{}\n{:?}\n{:?}",
                            b.family.text,
                            cond.as_ref().map(|c| c.render()).unwrap_or_default(),
                            b.fragment,
                            b.uses.iter().map(|u| u.name.text.clone()).collect::<Vec<_>>(),
                            b.immarg.iter().map(|(n, _)| *n).collect::<Vec<_>>()
                        ))
                    )
                } else {
                    String::new()
                };
                conds.push(cond);
                bodies.push(crate::core::PrimBodyPlan {
                    family: b.family.text.clone(),
                    fragment: b.fragment.clone(),
                    when: b.when.as_ref().map(|_| {
                        conds.last().unwrap().as_ref().unwrap().render()
                    }),
                    immarg: b.immarg.iter().map(|(n, _)| *n).collect(),
                    uses,
                    assumptions,
                    trusted,
                    trust_id,
                    validation,
                });
            }
            // Selection (section 6): a body applies when its family
            // lowers through this build's emission path and its `when`
            // condition holds in the target profile. The single matching
            // conditional body is the specific one; the baseline (no
            // `when`) is the fallback. Two bodies with overlapping
            // conditions in one family are the first-match ambiguity and
            // an error. Selection is fixed at compile time per artifact.
            // Selection (section 6, with section 13's platform
            // predicates): a body applies when its `when` condition holds
            // in the target profile (capability evidence or the target
            // triple's arch) and its family lowers through this build's
            // emission path. A matching conditional body is the specific
            // one and wins; the baseline (no `when`) is the fallback when
            // no condition matches (the canonical portable baseline
            // beside specific bodies). Two bodies with overlapping
            // conditions in one family are the first-match ambiguity and
            // an error. Selection is fixed at compile time per artifact.
            let mut family_units: Vec<(String, usize)> = Vec::new();
            let mut picks: Vec<(&str, Option<usize>, Option<usize>)> = Vec::new();
            for fam in ["llvm", "asm"] {
                let mut matched: Vec<usize> = Vec::new();
                let mut baseline: Option<usize> = None;
                for (i, bd) in bodies.iter().enumerate() {
                    if bd.family != fam {
                        continue;
                    }
                    match &conds[i] {
                        Some(c) if c.holds(&self.target_caps, &self.target_arch) => {
                            matched.push(i)
                        }
                        None => {
                            if baseline.is_some() {
                                return Err(Error::at(
                                    p.span,
                                    format!(
                                        "two baseline `{fam}` bodies overlap (primitive operations and inline target IR specification section 6: two bodies with overlapping conditions in one family are an error)"
                                    ),
                                ));
                            }
                            baseline = Some(i);
                        }
                        _ => {}
                    }
                }
                if matched.len() > 1 {
                    return Err(Error::at(
                        p.span,
                        format!(
                            "overlapping `when` conditions in family `{fam}`: {} bodies match this target profile (primitive operations and inline target IR specification section 6)",
                            matched.len()
                        ),
                    ));
                }
                let m = matched.first().copied();
                if let Some(i) = m.or(baseline) {
                    family_units.push((fam.to_string(), i));
                }
                picks.push((fam, m, baseline));
            }
            let mut selected: Option<(usize, String)> = None;
            for (fam, m, _) in &picks {
                if let Some(i) = m {
                    let render = conds[*i].as_ref().map(|c| c.render()).unwrap_or_default();
                    selected = Some((
                        *i,
                        format!("the `{fam}` body's `when {render}` condition holds in the target profile"),
                    ));
                    break;
                }
            }
            if selected.is_none() {
                for (fam, _, baseline) in &picks {
                    if let Some(i) = baseline {
                        selected = Some((
                            *i,
                            format!("the baseline `{fam}` body (the fallback when no `when` condition matches)"),
                        ));
                        break;
                    }
                }
            }
            let (selected, selected_why) = match selected {
                Some(x) => x,
                None => {
                    let names: Vec<String> =
                        conds.iter().flatten().map(|c| c.render()).collect();
                    if !names.is_empty() {
                        return Err(Error::at(
                            p.span,
                            format!(
                                "no body applies: the `when` conditions name the capability/platform {} which the target profile does not provide (primitive operations and inline target IR specification sections 6 and 13: reject the missing capability or platform)",
                                names.join(", ")
                            ),
                        ));
                    }
                    return Err(Error::at(
                        p.span,
                        format!(
                            "this build cannot emit a body for `{}`: it declares only {} bodies, and the artifact's emission families are `llvm` and `asm` (primitive operations and inline target IR specification section 3)",
                            p.name.text,
                            bodies.iter().map(|b| b.family.as_str()).collect::<Vec<_>>().join(", ")
                        ),
                    ));
                }
            };
            // The tier and the identity rules (section 5): a justified
            // prim's contract is its signature plus the law identity and
            // bodies are representation; an assumed prim's body content
            // is contract, so editing it stales dependents.
            let tier = if all_justified {
                crate::core::PrimTier::Justified
            } else {
                crate::core::PrimTier::Assumed
            };
            let identity = match tier {
                crate::core::PrimTier::Justified => {
                    format!("prim:{unit}:{}:laws[{}]", render_scheme(&s), laws.join(","))
                }
                crate::core::PrimTier::Assumed => {
                    let mut body_text = String::new();
                    for b in &bodies {
                        body_text.push_str(&b.family);
                        body_text.push('\n');
                        body_text.push_str(b.when.as_deref().unwrap_or(""));
                        body_text.push('\n');
                        body_text.push_str(&b.fragment);
                        body_text.push_str(&format!("{:?}", b.uses));
                        body_text.push_str(&format!("{:?}", b.immarg));
                    }
                    format!("prim:{unit}#{}", crate::interface::content_hash(&body_text))
                }
            };
            let plan = crate::core::PrimPlan {
                unit: unit.clone(),
                module: module.clone(),
                name: p.name.text.clone(),
                ty: render_scheme(&s),
                arity,
                args: arg_tys
                    .iter()
                    .map(|t| prim_abi_shape(t, self.usize_bits))
                    .collect(),
                res: prim_abi_shape(&res_ty, self.usize_bits),
                pure,
                tier,
                laws,
                selected,
                selected_why,
                family_units,
                bodies,
                identity,
            };
            // The synthesized unit is a leaf like a foreign import: its
            // content is the inline target fragment (section 7).
            let mut meta = crate::core::FnMeta::default();
            meta.prim_body = true;
            meta.pure = pure;
            self.fns.push(crate::core::FnDef {
                id: crate::core::GlobalId(unit.clone()),
                params: (0..arity).map(|i| format!("p{i}")).collect(),
                body: crate::core::Core::Unit,
                prim: None,
                ctor: None,
                hints: self.fn_hints.get(&unit).cloned().unwrap_or_default(),
                meta,
            });
            self.prims.push(plan);
        }
        Ok(())
    }

    /// A `prim` body's `justify` evidence (primitive operations and
    /// inline target IR specification section 5): the evidence states
    /// `Eq (<prim> x1 ... xn) meaning` over the prim's own argument
    /// binders, exactly the shape a law method of a `signature` (or a
    /// `law` member) declares (decision D55's checked obligation). The
    /// law identity is the assumption; declared `proof`/`spec` code is
    /// kernel evidence and records nothing. Returns the identities.
    fn check_prim_justify(
        &mut self,
        tm: &ast::Term,
        unit: &str,
        arity: usize,
        span: ast::Span,
    ) -> Result<Vec<String>> {
        let shape_err = |detail: &str| {
            Error::at(
                span,
                format!(
                    "a prim body's justification states `Eq ({unit} x1 ... xn) meaning` over the prim's own arguments (primitive operations and inline target IR specification section 5): {detail}"
                ),
            )
        };
        let head: &ast::Term = match tm {
            ast::Term::App(f, _, _) => f,
            other => other,
        };
        let p = match head {
            ast::Term::Var(p) => p,
            other => {
                return Err(Error::at(
                    other.span(),
                    "a prim body's justification is a law method of a `signature` (or a declared `proof`) stating the prim's equation (primitive operations and inline target IR specification section 5)",
                ))
            }
        };
        let v = self.resolve_var(p, p.span())?;
        let (ty, _, _) = self.instantiate(&v.scheme);
        let mut vars: Vec<String> = Vec::new();
        let mut cur = self.uni.zonk(&ty);
        loop {
            match cur {
                Ty::Pi { var, cod, .. } => {
                    vars.push(var);
                    cur = self.uni.zonk(&cod);
                }
                Ty::Fun(_, _, _, c) => {
                    return Err(shape_err(
                        "the justification quantifies its arguments by name (`(x :: T) -> ...`)",
                    ))
                }
                _ => break,
            }
        }
        if vars.len() != arity {
            return Err(shape_err(&format!(
                "the evidence states {} arguments, but the prim takes {}",
                vars.len(),
                arity
            )));
        }
        let lhs = match self.uni.zonk(&cur) {
            Ty::Prp(p) => match *p {
                PTerm::Eq(l, _) => l,
                _ => return Err(shape_err("the conclusion is an `Eq` equation")),
            },
            other => {
                return Err(shape_err(&format!(
                    "the evidence concludes with a proposition, not {other}"
                )))
            }
        };
        let expected = if arity == 0 {
            ITerm::Fn(unit.to_string())
        } else {
            ITerm::App(
                Box::new(ITerm::Fn(unit.to_string())),
                vars.iter().map(|v| ITerm::Var(v.clone())).collect(),
            )
        };
        if *lhs != expected {
            return Err(shape_err(&format!(
                "the equation is about `{}`, not the prim's own application",
                lhs
            )));
        }
        Ok(match v.kind {
            ValueKind::Method { .. } | ValueKind::Law => {
                vec![format!("{}.{}", v.module, v.name)]
            }
            _ => Vec::new(),
        })
    }

    /// Check every instance against its class's laws, sample-wise where
    /// the machinery can decide it (primitive operations and inline
    /// target IR specification section 5). The law machinery's honest
    /// boundary: a law over samplable carriers whose equations evaluate
    /// is checked at its samples; anything else is reported as not
    /// checked, never claimed as verified.
    fn check_laws(&mut self) -> Result<()> {
        // The law machinery resolves classes and dictionaries across the
        // whole program: every collected module is visible here.
        self.visible = self.module_names.iter().cloned().collect();
        let obligations = std::mem::take(&mut self.law_obligations);
        for ob in obligations {
            self.check_law_samples(&ob)?;
        }
        // Preservation of refinement transfers (refinement types design):
        // the transfer's validity condition, checked sample-wise in the
        // same honest tier as the class laws.
        let pres = std::mem::take(&mut self.pres_obligations);
        for ob in &pres {
            self.check_preservation(ob)?;
        }
        // Refinement transfers resolved at uses (refinement types
        // design, "composability"): the point's preservation and
        // postcondition rows beside the declared instances' rows.
        self.check_transfers()?;
        Ok(())
    }

    /// One instance's sample-wise law check (primitive operations and
    /// inline target IR specification section 5).
    fn check_law_samples(&mut self, ob: &LawObligation) -> Result<()> {
        let scope = format!("{}.{}", ob.module, ob.instance);
        let mut row = |status: String| crate::core::LawRow {
            instance: scope.clone(),
            class: ob.class.clone(),
            law: ob.law.clone(),
            status,
        };
        if self
            .module_params
            .get(&ob.module)
            .map(|p| !p.is_empty())
            .unwrap_or(false)
        {
            self.law_rows
                .push(row("not checked (the instance closes over module parameters)".to_string()));
            return Ok(());
        }
        // The law at the instance's type arguments.
        let mut map: HashMap<String, Ty> = HashMap::new();
        for (i, p) in ob.class_params.iter().enumerate() {
            if let Some(a) = ob.class_args.get(i) {
                map.insert(p.clone(), a.clone());
            }
        }
        let mut cur = Self::subst_ty(&ob.scheme.ty, &map);
        let mut binders: Vec<(String, Ty)> = Vec::new();
        loop {
            match cur {
                Ty::Pi { var, dom, cod, .. } => {
                    binders.push((var, *dom));
                    cur = *cod;
                }
                Ty::Fun(_, _, _, c) => {
                    self.law_rows.push(row(
                        "not checked (the law's binders must be named: `(x :: T) -> ...`)"
                            .to_string(),
                    ));
                    return Ok(());
                }
                _ => break,
            }
        }
        let (lhs, rhs) = match cur {
            Ty::Prp(p) => match *p {
                PTerm::Eq(l, r) => (l, r),
                _ => {
                    self.law_rows
                        .push(row("not checked (the law states no `Eq` equation)".to_string()));
                    return Ok(());
                }
            },
            _ => {
                self.law_rows
                    .push(row("not checked (the law states no `Eq` equation)".to_string()));
                return Ok(());
            }
        };
        // Samples of the law's binders, where each type is samplable.
        let mut sample_sets: Vec<Vec<Value>> = Vec::new();
        for (name, dom) in &binders {
            let samples = self.law_samples(dom, 2);
            if samples.is_empty() {
                self.law_rows.push(row(format!(
                    "not checked (no samples for the law binder `{name} :: {dom}`)"
                )));
                return Ok(());
            }
            sample_sets.push(samples);
        }
        // The instance's method units as an index-term head table, with
        // their sub-dictionary values (the equations mention methods by
        // their declared heads and pass no dictionaries).
        let mut methods: HashMap<String, (crate::core::GlobalId, Vec<Value>)> = HashMap::new();
        if self
            .law_collect_methods(ob, 0, &mut methods, &mut Vec::new())?
            .is_none()
        {
            self.law_rows
                .push(row("not checked (the instance's dictionaries do not resolve)".to_string()));
            return Ok(());
        }
        let scx = self.fold_scx.clone().unwrap_or_default();
        let mut cx = StageCx::new(&scx);
        // Every ordered sample assignment (the cross product, capped).
        let mut assignments: Vec<HashMap<String, Value>> = vec![HashMap::new()];
        for ((name, _), samples) in binders.iter().zip(&sample_sets) {
            let mut next = Vec::new();
            for a in &assignments {
                for s in samples {
                    let mut a2 = a.clone();
                    a2.insert(name.clone(), s.clone());
                    next.push(a2);
                }
            }
            assignments = next;
            if assignments.len() > 256 {
                assignments.truncate(256);
            }
        }
        let mut checked = 0usize;
        for frame in &assignments {
            let l = self.eval_law_idx(&mut cx, &lhs, frame, &methods);
            let r = self.eval_law_idx(&mut cx, &rhs, frame, &methods);
            let (l, r) = match (l, r) {
                (Ok(l), Ok(r)) => (l, r),
                (Err(e), _) | (_, Err(e)) => {
                    self.law_rows.push(row(format!(
                        "not checked (the samples did not evaluate: {e})"
                    )));
                    return Ok(());
                }
            };
            match law_value_eq(&l, &r) {
                LawCmp::Equal => checked += 1,
                LawCmp::Opaque => {
                    self.law_rows.push(row(
                        "not checked (a sample produced a value the law machinery cannot compare)"
                            .to_string(),
                    ));
                    return Ok(());
                }
                LawCmp::NotEqual => {
                    let mut sample_text = String::new();
                    for (name, _) in &binders {
                        if !sample_text.is_empty() {
                            sample_text.push_str(", ");
                        }
                        sample_text.push_str(&format!(
                            "{name} = {}",
                            law_value_render(&frame[name])
                        ));
                    }
                    return Err(Error::at(
                        ob.span,
                        format!(
                            "instance `{}` breaks class law `{}.{}` at sample values ({}): left `{}`, right `{}` (primitive operations and inline target IR specification section 5)",
                            scope,
                            ob.class,
                            ob.law,
                            sample_text,
                            law_value_render(&l),
                            law_value_render(&r)
                        ),
                    ));
                }
            }
        }
        self.law_rows.push(row(format!("checked ({checked} samples)")));
        Ok(())
    }

    /// One instance method's preservation check over refinement samples
    /// (refinement types design: the transfer's validity condition, in
    /// the P18 law layer's honest tier). The method must map refined
    /// sample values to values satisfying the refinement's predicate; a
    /// failure rejects the instance naming the preservation law, and a
    /// case the machinery cannot decide reports `not checked`.
    fn check_preservation(&mut self, ob: &PresObligation) -> Result<()> {
        let scope = format!("{}.{}", ob.module, ob.instance);
        let law_name = format!("{}.preserve-{}", ob.refine, ob.method);
        let mut row = |status: String| crate::core::LawRow {
            instance: scope.clone(),
            class: ob.class.clone(),
            law: law_name.clone(),
            status,
        };
        if ob.nested {
            self.law_rows.push(row(
                "not checked (a refined result inside another type former is not checked)"
                    .to_string(),
            ));
            return Ok(());
        }
        if self
            .module_params
            .get(&ob.module)
            .map(|p| !p.is_empty())
            .unwrap_or(false)
        {
            self.law_rows
                .push(row("not checked (the instance closes over module parameters)".to_string()));
            return Ok(());
        }
        // The refinement's predicate and binder.
        let refine_info = self
            .types
            .get(&ob.refine)
            .and_then(|ts| ts.iter().find(|t| t.refine.is_some()))
            .and_then(|t| t.refine.clone());
        let Some(r) = refine_info else {
            self.law_rows.push(row(
                "not checked (the refinement's predicate is not registered)".to_string(),
            ));
            return Ok(());
        };
        // Samples of each method argument, where each type is samplable.
        let mut sample_sets: Vec<Vec<Value>> = Vec::new();
        for aty in &ob.arg_tys {
            let samples = self.law_samples(aty, 2);
            if samples.is_empty() {
                self.law_rows.push(row(format!(
                    "not checked (no samples for the method argument `{aty}`)"
                )));
                return Ok(());
            }
            sample_sets.push(samples);
        }
        // The method's dictionaries resolve exactly like the law
        // checker's (the same honest boundary).
        let dummy = LawObligation {
            instance: ob.instance.clone(),
            module: ob.module.clone(),
            class: ob.class.clone(),
            law: String::new(),
            scheme: Scheme::mono(Ty::Unit),
            class_params: Vec::new(),
            class_args: ob.class_args.clone(),
            method_units: ob.method_units.clone(),
            subdicts: ob.subdicts.clone(),
            span: ob.span,
        };
        let mut methods: HashMap<String, (crate::core::GlobalId, Vec<Value>)> = HashMap::new();
        if self
            .law_collect_methods(&dummy, 0, &mut methods, &mut Vec::new())?
            .is_none()
        {
            self.law_rows
                .push(row("not checked (the instance's dictionaries do not resolve)".to_string()));
            return Ok(());
        }
        let Some(sig) = self
            .sigs
            .get(&ob.class)
            .and_then(|ss| ss.iter().find(|s| self.def_in_scope(&s.module, &s.name)))
            .cloned()
        else {
            self.law_rows
                .push(row("not checked (the instance's class does not resolve)".to_string()));
            return Ok(());
        };
        let head = fn_id(&sig.module, &ob.method).0;
        // Every ordered sample assignment (the cross product, capped),
        // named positionally for the method application's equation.
        let mut assignments: Vec<HashMap<String, Value>> = vec![HashMap::new()];
        for (i, samples) in sample_sets.iter().enumerate() {
            let mut next = Vec::new();
            for a in &assignments {
                for s in samples {
                    let mut a2 = a.clone();
                    a2.insert(format!("$a{i}"), s.clone());
                    next.push(a2);
                }
            }
            assignments = next;
            if assignments.len() > 256 {
                assignments.truncate(256);
            }
        }
        let scx = self.fold_scx.clone().unwrap_or_default();
        let mut cx = StageCx::new(&scx);
        let mut checked = 0usize;
        for frame in &assignments {
            // The method applied to this sample assignment, evaluated
            // through the staging interpreter over the instance's real
            // method code.
            let mut t = ITerm::Fn(head.clone());
            for i in 0..ob.arg_tys.len() {
                t = ITerm::App(Box::new(t), vec![ITerm::Var(format!("$a{i}"))]);
            }
            let value = match self.eval_law_idx(&mut cx, &t, frame, &methods) {
                Ok(v) => v,
                Err(e) => {
                    self.law_rows.push(row(format!(
                        "not checked (the samples did not evaluate: {e})"
                    )));
                    return Ok(());
                }
            };
            match self.pred_value(&r.pred, &r.binder, &value) {
                Some(true) => checked += 1,
                Some(false) => {
                    // Preservation is per conjunct (compound predicates
                    // design): the failure names the conjunct that does
                    // not survive.
                    let failing = pred_conjuncts(&r.pred)
                        .into_iter()
                        .find(|c| self.pred_value(c, &r.binder, &value) == Some(false))
                        .unwrap_or_else(|| r.pred.clone());
                    let mut sample_text = String::new();
                    for i in 0..ob.arg_tys.len() {
                        if !sample_text.is_empty() {
                            sample_text.push_str(", ");
                        }
                        sample_text.push_str(&format!(
                            "$a{i} = {}",
                            law_value_render(&frame[&format!("$a{i}")])
                        ));
                    }
                    return Err(Error::at(
                        ob.span,
                        format!(
                            "instance `{}` breaks preservation law `{}` at sample values ({}): `{}` produced `{}`, which does not satisfy `{}` (refinement types design: a container's methods transfer exactly when they preserve the refinement)",
                            scope,
                            law_name,
                            sample_text,
                            ob.method,
                            law_value_render(&value),
                            failing
                        ),
                    ));
                }
                None => {
                    self.law_rows.push(row(
                        "not checked (the samples' preservation is not decided)".to_string(),
                    ));
                    return Ok(());
                }
            }
        }
        self.law_rows.push(row(format!("checked ({checked} samples)")));
        Ok(())
    }

    /// Every recorded refinement transfer's checks (refinement types
    /// design, "composability"): the point's preservation and
    /// postcondition rows in the law layer's honest tier, beside the
    /// declared instances' own rows.
    fn check_transfers(&mut self) -> Result<()> {
        let transfers = std::mem::take(&mut self.transfer_obligs);
        for ob in &transfers {
            self.check_transfer(ob)?;
        }
        Ok(())
    }

    /// One manifest row, deduplicated by its identity: one report per
    /// (dictionary, law) however the transfer was reached.
    fn push_row(rows: &mut Vec<crate::core::LawRow>, row: crate::core::LawRow) {
        if rows
            .iter()
            .any(|r| r.instance == row.instance && r.law == row.law)
        {
            return;
        }
        rows.push(row);
    }

    /// One refinement transfer's preservation and postcondition check
    /// (refinement types design, "composability"): the method must map
    /// point values to point values (the closure `post p p = p`), per
    /// conjunct with the failing conjunct named. The small-sample tier
    /// and the bound-corner witnesses decide what they can; the rest is
    /// reported `not checked`, never claimed as verified.
    fn check_transfer(&mut self, ob: &TransferOblig) -> Result<()> {
        let point_name = self.point_name(&ob.point, &ob.pred);
        let law_name = format!("{}.preserve-{}", point_name, ob.method);
        let post_name = format!("{}.post-{}", point_name, ob.method);
        let scope = ob.container.clone();
        let class = ob.class.clone();
        let method = ob.method.clone();
        let pred_text = ob.pred.to_string();
        let row = |law: &str, status: String| crate::core::LawRow {
            instance: scope.clone(),
            class: class.clone(),
            law: law.to_string(),
            status,
        };
        let mut bail = |rows: &mut Vec<crate::core::LawRow>, reason: String| {
            Self::push_row(rows, row(&law_name, format!("not checked ({reason})")));
            Self::push_row(
                rows,
                row(&post_name, format!("derived: {pred_text}; not checked ({reason})")),
            );
        };
        if ob.nested {
            bail(
                &mut self.law_rows,
                "a refined result inside another type former is not checked".to_string(),
            );
            return Ok(());
        }
        if self
            .module_params
            .get(&ob.module)
            .map(|p| !p.is_empty())
            .unwrap_or(false)
        {
            bail(
                &mut self.law_rows,
                "the instance closes over module parameters".to_string(),
            );
            return Ok(());
        }
        let sig = self
            .sigs
            .get(&ob.class)
            .and_then(|ss| ss.iter().find(|s| self.def_in_scope(&s.module, &s.name)))
            .cloned();
        let Some(sig) = sig else {
            bail(
                &mut self.law_rows,
                "the instance's class does not resolve".to_string(),
            );
            return Ok(());
        };
        let def = self
            .instance_defs
            .iter()
            .find(|d| d.id == fn_id(&ob.module, &ob.instance))
            .cloned();
        let Some(def) = def else {
            bail(
                &mut self.law_rows,
                "the instance's dictionaries do not resolve".to_string(),
            );
            return Ok(());
        };
        // The method's dictionaries resolve exactly like the law
        // checker's (the same honest boundary).
        let dummy = LawObligation {
            instance: ob.instance.clone(),
            module: ob.module.clone(),
            class: ob.class.clone(),
            law: String::new(),
            scheme: Scheme::mono(Ty::Unit),
            class_params: ob.class_params.clone(),
            class_args: ob.class_args.clone(),
            method_units: def.methods.clone(),
            subdicts: ob.subdicts.clone(),
            span: ob.span,
        };
        let mut methods: HashMap<String, (crate::core::GlobalId, Vec<Value>)> = HashMap::new();
        if self
            .law_collect_methods(&dummy, 0, &mut methods, &mut Vec::new())?
            .is_none()
        {
            bail(
                &mut self.law_rows,
                "the instance's dictionaries do not resolve".to_string(),
            );
            return Ok(());
        }
        // The method's arguments at the point.
        let mut map: HashMap<String, Ty> = HashMap::new();
        for (i, p) in ob.class_params.iter().enumerate() {
            if let Some(a) = ob.class_args.get(i) {
                map.insert(p.clone(), a.clone());
            }
        }
        let mscheme = sig.methods.iter().find(|(m, _)| *m == ob.method).cloned();
        let Some((_, mscheme)) = mscheme else {
            bail(
                &mut self.law_rows,
                "the instance's method does not resolve".to_string(),
            );
            return Ok(());
        };
        let mut arg_tys: Vec<Ty> = Vec::new();
        let mut cur = Self::subst_ty(&mscheme.ty, &map);
        loop {
            match self.uni.zonk(&cur) {
                Ty::Fun(a, _, _, b) => {
                    arg_tys.push(*a);
                    cur = *b;
                }
                Ty::Pi { dom, cod, .. } => {
                    arg_tys.push(*dom);
                    cur = *cod;
                }
                _ => break,
            }
        }
        // Samples of each argument: the point's own samples plus the
        // bound-corner witnesses.
        let witnesses = self.transfer_witnesses(&ob.pred, &ob.binder, &ob.base);
        let mut sample_sets: Vec<Vec<Value>> = Vec::new();
        for aty in &arg_tys {
            let mut samples = self.law_samples(aty, 2);
            for w in &witnesses {
                samples.push(w.clone());
            }
            if samples.is_empty() {
                bail(
                    &mut self.law_rows,
                    format!("no samples for the method argument `{aty}`"),
                );
                return Ok(());
            }
            sample_sets.push(samples);
        }
        // The method applied to each sample assignment, evaluated
        // through the staging interpreter over the instance's real
        // method code.
        let mut assignments: Vec<HashMap<String, Value>> = vec![HashMap::new()];
        for (i, samples) in sample_sets.iter().enumerate() {
            let mut next = Vec::new();
            for a in &assignments {
                for s in samples {
                    let mut a2 = a.clone();
                    a2.insert(format!("$a{i}"), s.clone());
                    next.push(a2);
                }
            }
            assignments = next;
            if assignments.len() > 256 {
                assignments.truncate(256);
            }
        }
        let head = fn_id(&sig.module, &ob.method).0;
        let scx = self.fold_scx.clone().unwrap_or_default();
        let mut cx = StageCx::new(&scx);
        let conj = pred_conjuncts(&ob.pred);
        let mut checked = 0usize;
        for frame in &assignments {
            let mut t = ITerm::Fn(head.clone());
            for i in 0..arg_tys.len() {
                t = ITerm::App(Box::new(t), vec![ITerm::Var(format!("$a{i}"))]);
            }
            let value = match self.eval_law_idx(&mut cx, &t, frame, &methods) {
                Ok(v) => v,
                Err(e) => {
                    bail(
                        &mut self.law_rows,
                        format!("the samples did not evaluate: {e}"),
                    );
                    return Ok(());
                }
            };
            // Preservation is per conjunct (compound predicates design):
            // a failure names the conjunct that does not survive.
            let mut failing: Option<PTerm> = None;
            let mut undecided = false;
            for c in &conj {
                match self.pred_value(c, &ob.binder, &value) {
                    Some(true) => {}
                    Some(false) => {
                        failing = Some(c.clone());
                        break;
                    }
                    None => {
                        undecided = true;
                        break;
                    }
                }
            }
            if let Some(c) = failing {
                let mut sample_text = String::new();
                for i in 0..arg_tys.len() {
                    if !sample_text.is_empty() {
                        sample_text.push_str(", ");
                    }
                    sample_text.push_str(&format!(
                        "$a{i} = {}",
                        law_value_render(&frame[&format!("$a{i}")])
                    ));
                }
                return Err(Error::at(
                    ob.span,
                    format!(
                        "instance `{scope}` breaks preservation law `{law_name}` at sample values ({sample_text}): `{method}` produced `{}`, which does not satisfy `{c}` (refinement types design: a container's methods transfer exactly when they preserve the refinement)",
                        law_value_render(&value)
                    ),
                ));
            }
            if undecided {
                bail(
                    &mut self.law_rows,
                    "the samples' preservation is not decided".to_string(),
                );
                return Ok(());
            }
            checked += 1;
        }
        Self::push_row(
            &mut self.law_rows,
            row(&law_name, format!("checked ({checked} samples)")),
        );
        Self::push_row(
            &mut self.law_rows,
            row(
                &post_name,
                format!("derived: {pred_text}; checked ({checked} samples)"),
            ),
        );
        Ok(())
    }

    /// The lattice point's name for the law rows (nominal names are
    /// named predicates for readability): a named refinement is its
    /// name, an anonymous point takes the named refinement stating the
    /// same predicate, and a nameless point is its predicate.
    fn point_name(&self, point: &Ty, pred: &PTerm) -> String {
        if let Ty::Adt(name, _) = point {
            if self
                .types
                .get(name)
                .map(|ts| ts.iter().any(|t| t.refine.is_some()))
                .unwrap_or(false)
            {
                return name.clone();
            }
        }
        let mut names: Vec<String> = self.types.keys().cloned().collect();
        names.sort();
        for n in names {
            let Some(tk) = self
                .types
                .get(&n)
                .and_then(|ts| ts.iter().find(|t| t.refine.is_some()))
            else {
                continue;
            };
            let Some(r) = &tk.refine else {
                continue;
            };
            if pred_alpha_eq(&r.pred, pred) {
                return n;
            }
        }
        pred.to_string()
    }

    /// The preservation check's witness inputs (the compound predicates
    /// design's per-conjunct rule): values at the point's bound corners,
    /// where a length-growing method is most likely to break a bounded
    /// conjunct (`length xs <= 200` breaks at 200 + 200). A witness is
    /// kept only when it really inhabits the point.
    fn transfer_witnesses(&self, pred: &PTerm, binder: &str, base: &Ty) -> Vec<Value> {
        let numeric = matches!(base, Ty::Nat | Ty::Int | Ty::Prim(_));
        let (lo, hi) = if numeric {
            crate::types::pred_measure_bounds(pred, &ITerm::Var(binder.to_string()))
        } else {
            let len = ITerm::app_fn(
                "<builtin>.length".to_string(),
                vec![ITerm::Var(binder.to_string())],
            );
            crate::types::pred_measure_bounds(pred, &len)
        };
        let mut mags: Vec<i128> = Vec::new();
        if let Some(l) = lo {
            mags.push(l);
            mags.push(l + 1);
        }
        if let Some(h) = hi {
            mags.push(h);
            if h > 0 {
                mags.push(h - 1);
            }
        }
        if let (Some(l), Some(h)) = (lo, hi) {
            if l < h {
                mags.push((l + h) / 2);
            }
        }
        mags.extend([1, 2, 3]);
        let mut out: Vec<Value> = Vec::new();
        let mut seen: Vec<i128> = Vec::new();
        for n in mags {
            if n < 0 || seen.contains(&n) {
                continue;
            }
            if let Some(l) = lo {
                if n < l {
                    continue;
                }
            }
            if let Some(h) = hi {
                if n > h {
                    continue;
                }
            }
            seen.push(n);
            if numeric {
                let v = Value::Word(n as i64);
                if self.pred_value(pred, binder, &v) == Some(true) {
                    out.push(v);
                }
            } else {
                for v in self.list_of_len(base, n as usize) {
                    if self.pred_value(pred, binder, &v) == Some(true) {
                        out.push(v);
                    }
                }
            }
        }
        out
    }

    /// List-shaped witness values of one length over the base's own
    /// sample constructors (each distinct sample head): uniform
    /// elements stay non-decreasing, and a sub-128 head stays ASCII.
    fn list_of_len(&self, base: &Ty, len: usize) -> Vec<Value> {
        let samples = self.law_samples(base, 2);
        let Some(nil) = samples
            .iter()
            .find(|s| matches!(s, Value::Adt { fields, .. } if fields.is_empty()))
            .cloned()
        else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for s in &samples {
            let Value::Adt { ctor, tag, fields } = s else {
                continue;
            };
            if fields.len() < 2 {
                continue;
            }
            let mut v = nil.clone();
            for _ in 0..len {
                v = Value::Adt {
                    ctor: ctor.clone(),
                    tag: *tag,
                    fields: vec![fields[0].clone(), v],
                };
            }
            out.push(v);
        }
        out
    }

    /// The predicate's truth at a sample value (the preservation check's
    /// and the refinement sampler's decision procedure over values):
    /// `None` where the machinery cannot decide it.
    fn pred_value(&self, pred: &PTerm, binder: &str, v: &Value) -> Option<bool> {
        match pred {
            PTerm::Pred(PredFn::NotNull, _) => match v {
                // The empty form is the nullary constructor; every other
                // constructor carries the elements.
                Value::Adt { fields, .. } => Some(!fields.is_empty()),
                _ => None,
            },
            PTerm::Pred(PredFn::NonZero, it) => {
                let n = self.itern_value(it, binder, v)?;
                Some(n != 0)
            }
            PTerm::Pred(PredFn::Ascii, _) => {
                match v {
                    // A list of chars: every element is below 128 (the
                    // byte-range fact `Ascii` implies).
                    Value::Adt { fields, .. } if fields.is_empty() || fields.len() >= 2 => {
                        let mut cur = v;
                        loop {
                            match cur {
                                Value::Adt { fields, .. } if fields.is_empty() => {
                                    return Some(true)
                                }
                                Value::Adt { fields, .. } if fields.len() >= 2 => {
                                    let n = value_num(&fields[0])?;
                                    if !(0..128).contains(&n) {
                                        return Some(false);
                                    }
                                    cur = &fields[1];
                                }
                                _ => return None,
                            }
                        }
                    }
                    _ => {
                        let bytes = match v {
                            Value::Adt { fields, .. } => match fields.as_slice() {
                                [Value::Bytes(b)] => b.clone(),
                                _ => return None,
                            },
                            Value::Bytes(b) => b.clone(),
                            _ => return None,
                        };
                        let ok = bytes.borrow().iter().all(|&b| b < 128);
                        Some(ok)
                    }
                }
            }
            PTerm::Pred(PredFn::Ordered, _) => {
                // The adjacent-pair walk over the list's constructor
                // spine (elements compare numerically).
                let mut cur = v;
                let mut prev: Option<i128> = None;
                loop {
                    match cur {
                        Value::Adt { fields, .. } => {
                            if fields.is_empty() {
                                return Some(true);
                            }
                            if fields.len() < 2 {
                                return None;
                            }
                            let head = value_num(&fields[0])?;
                            if let Some(p) = prev {
                                if p > head {
                                    return Some(false);
                                }
                            }
                            prev = Some(head);
                            cur = &fields[1];
                        }
                        _ => return None,
                    }
                }
            }
            PTerm::Cmp(op, a, b) => {
                let x = self.itern_value(a, binder, v)?;
                let y = self.itern_value(b, binder, v)?;
                Some(match op {
                    IdxCmp::Eq => x == y,
                    IdxCmp::Ne => x != y,
                    IdxCmp::Lt => x < y,
                    IdxCmp::Le => x <= y,
                    IdxCmp::Gt => x > y,
                    IdxCmp::Ge => x >= y,
                })
            }
            PTerm::Eq(a, b) => {
                let x = self.itern_value(a, binder, v)?;
                let y = self.itern_value(b, binder, v)?;
                Some(x == y)
            }
            PTerm::Not(q) => Some(!self.pred_value(q, binder, v)?),
            // Conjunction and disjunction are decidable where both
            // sides are: `&&` holds when both hold, `||` when either
            // holds; an undecided side leaves the compound undecided.
            PTerm::And(a, b) => match (self.pred_value(a, binder, v), self.pred_value(b, binder, v)) {
                (Some(false), _) | (_, Some(false)) => Some(false),
                (Some(true), Some(true)) => Some(true),
                _ => None,
            },
            PTerm::Or(a, b) => match (self.pred_value(a, binder, v), self.pred_value(b, binder, v)) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => None,
            },
            PTerm::Var(_) | PTerm::MVar(_) => None,
        }
    }

    /// The numeric value of an index term at a sample: the binder's
    /// value, numerals and numeric arithmetic, and the baked measures
    /// (`length` over a list spine or byte buffer, `toNat` over a
    /// machine word) that comparison atoms admit (anything else is not
    /// decided at a sample).
    fn itern_value(&self, it: &ITerm, binder: &str, v: &Value) -> Option<i128> {
        match it {
            ITerm::Var(n) if n == binder => value_num(v),
            ITerm::Num(d) => d.parse().ok(),
            ITerm::Bin(op, a, b) => {
                let x = self.itern_value(a, binder, v)?;
                let y = self.itern_value(b, binder, v)?;
                match op {
                    IdxOp::Add => x.checked_add(y),
                    IdxOp::Sub => x.checked_sub(y),
                    IdxOp::Mul => x.checked_mul(y),
                }
            }
            ITerm::App(f, args) if args.len() == 1 => {
                self.itern_measure(f, &args[0], binder, v)
            }
            _ => None,
        }
    }

    /// One baked measure at a sample: `<builtin>.length` over a list
    /// spine or byte buffer, `<builtin>.toNat` over a numeric value.
    fn itern_measure(&self, f: &ITerm, arg: &ITerm, binder: &str, v: &Value) -> Option<i128> {
        let name = match f.normalize() {
            ITerm::Fn(k) => k,
            _ => return None,
        };
        // The measure's operand must read the binder's sample value.
        match arg.normalize() {
            ITerm::Var(n) if n == binder => {}
            _ => return None,
        }
        match name.as_str() {
            "<builtin>.length" => value_len(v),
            "<builtin>.toNat" => value_num(v),
            _ => None,
        }
    }

    /// Collect the method head table for one law check: the instance's
    /// own class methods (and its sub-dictionaries' methods), mapped
    /// from their declared index-term heads to the instance's method
    /// units and sub-dictionary values. `None` when a dictionary does
    /// not resolve: the honest boundary.
    fn law_collect_methods(
        &self,
        ob: &LawObligation,
        depth: u8,
        out: &mut HashMap<String, (crate::core::GlobalId, Vec<Value>)>,
        seen: &mut Vec<String>,
    ) -> Result<Option<Value>> {
        if depth > 4 {
            return Ok(None);
        }
        let sigs = match self.sigs.get(&ob.class) {
            Some(ss) => ss,
            None => return Ok(None),
        };
        let sig = match sigs
            .iter()
            .find(|s| self.def_in_scope(&s.module, &s.name))
        {
            Some(s) => s.clone(),
            None => return Ok(None),
        };
        let scope_key = format!(
            "{}[{}]",
            ob.class,
            ob.class_args
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        if seen.contains(&scope_key) {
            return Ok(None);
        }
        seen.push(scope_key);
        // The instance's sub-dictionaries resolve to concrete instances
        // and contribute their own methods and dictionary values.
        let mut subs: Vec<Value> = Vec::new();
        for (sc, sargs) in &ob.subdicts {
            let Some(idx) = self.unique_instance_for(sc, sargs) else {
                return Ok(None);
            };
            let info = self.instance_list[idx].clone();
            let def = match self
                .instance_defs
                .iter()
                .find(|d| d.id == fn_id(&info.module, &info.name))
            {
                Some(d) => d.clone(),
                None => return Ok(None),
            };
            let sub_ob = LawObligation {
                instance: info.name.clone(),
                module: info.module.clone(),
                class: sc.clone(),
                law: String::new(),
                scheme: ob.scheme.clone(),
                class_params: Vec::new(),
                class_args: sargs.clone(),
                method_units: def.methods.clone(),
                subdicts: InstCtx::dicts(&info.ctx)
                    .into_iter()
                    .map(|(_, c)| (c.class.clone(), c.args.clone()))
                    .collect(),
                span: ob.span,
            };
            let Some(d) = self.law_collect_methods(&sub_ob, depth + 1, out, seen)? else {
                return Ok(None);
            };
            subs.push(d);
        }
        // The instance's own methods, over its sub-dictionaries.
        for (mi, (mname, _)) in sig.methods.iter().enumerate() {
            let Some(gid) = ob.method_units.get(mi) else {
                return Ok(None);
            };
            out.insert(fn_id(&sig.module, mname).0, (gid.clone(), subs.clone()));
        }
        let closes = ob
            .method_units
            .iter()
            .map(|gid| Value::Clos {
                fn_id: gid.clone(),
                env: Vec::new(),
                args: subs.clone(),
                arity: usize::MAX,
            })
            .collect();
        Ok(Some(Value::Dict { methods: closes }))
    }

    /// The unique instance a class application resolves to (the
    /// coherence rule: no two instances of a class unify, so at most one
    /// matches; a parameter matches any argument).
    fn unique_instance_for(&self, class: &str, args: &[Ty]) -> Option<usize> {
        let idxs = self.instances.get(class)?;
        let mut found = None;
        for &i in idxs {
            let info = &self.instance_list[i];
            if info.base {
                continue;
            }
            if info.target.class != class || info.target.args.len() != args.len() {
                continue;
            }
            if info
                .target
                .args
                .iter()
                .zip(args)
                .all(|(p, a)| ty_match(p, a))
            {
                if found.is_some() {
                    return None;
                }
                found = Some(i);
            }
        }
        found
    }

    /// Evaluate one side of a law equation at its sample assignment
    /// (primitive operations and inline target IR specification section
    /// 5): index values reduce through the staging evaluator over the
    /// instance's real method code.
    fn eval_law_idx(
        &mut self,
        cx: &mut StageCx,
        t: &ITerm,
        frame: &HashMap<String, Value>,
        methods: &HashMap<String, (crate::core::GlobalId, Vec<Value>)>,
    ) -> Result<Value> {
        Ok(match t {
            ITerm::Num(d) => match d.parse::<i64>() {
                Ok(v) => Value::Word(v),
                Err(_) => Value::Big {
                    nat: !d.starts_with('-'),
                    digits: d.clone(),
                },
            },
            ITerm::Var(v) => frame.get(v).cloned().ok_or_else(|| {
                Error::plain(format!("internal error: unbound law binder {v}"))
            })?,
            ITerm::Bin(op, a, b) => {
                let x = self.eval_law_idx(cx, a, frame, methods)?;
                let y = self.eval_law_idx(cx, b, frame, methods)?;
                let (Value::Word(x), Value::Word(y)) = (&x, &y) else {
                    return Err(Error::plain(
                        "a law's index arithmetic sample did not reduce to a word",
                    ));
                };
                let (r, overflow) = match op {
                    IdxOp::Add => x.overflowing_add(*y),
                    IdxOp::Sub => x.overflowing_sub(*y),
                    IdxOp::Mul => x.overflowing_mul(*y),
                };
                if overflow {
                    return Err(Error::plain(
                        "a law's index arithmetic sample overflowed (not decided)",
                    ));
                }
                Value::Word(r)
            }
            ITerm::Fn(k) => {
                if let Some((gid, subs)) = methods.get(k) {
                    self.eval_fn(cx, gid, subs.clone())?
                } else {
                    self.global_value(cx, k)?
                }
            }
            ITerm::App(f, args) => {
                let fv = self.eval_law_idx(cx, f, frame, methods)?;
                let mut vals = Vec::new();
                for a in args {
                    vals.push(self.eval_law_idx(cx, a, frame, methods)?);
                }
                self.apply_value(cx, fv, vals)?
            }
            ITerm::MVar(_) => {
                return Err(Error::plain(
                    "internal error: unsolved variable in a law equation",
                ))
            }
            ITerm::Lam(_, _) => {
                return Err(Error::plain(
                    "an index-level lambda is not a law argument (the law layer samples values, not functions)",
                ))
            }
        })
    }

    /// Sample values of one law binder type (primitive operations and
    /// inline target IR specification section 5's sample-wise rule):
    /// machine scalars at their boundaries, `Bool` exhaustively, `Nat`
    /// and `Int` at small values, data types over their constructors'
    /// sample fields, and an unknown carrier at two default words. A
    /// type the machinery cannot sample yields no samples: the honest
    /// boundary.
    fn law_samples(&self, ty: &Ty, depth: u8) -> Vec<Value> {
        match ty {
            Ty::Prim(s) => scalar_samples(s),
            Ty::Bool => vec![Value::Bool(false), Value::Bool(true)],
            Ty::Nat | Ty::Int => vec![Value::Word(0), Value::Word(1), Value::Word(2)],
            Ty::Param(_) => vec![Value::Word(0), Value::Word(1)],
            Ty::Bytes => vec![
                Value::Bytes(std::rc::Rc::new(std::cell::RefCell::new(Vec::new()))),
                Value::Bytes(std::rc::Rc::new(std::cell::RefCell::new(vec![65, 66]))),
                Value::Bytes(std::rc::Rc::new(std::cell::RefCell::new(vec![200]))),
            ],
            Ty::Adt(name, args) if name == "refined" => {
                // An anonymous lattice point (the `refined P T` former):
                // the container's samples its predicate decides true,
                // the same rule as a named refinement's.
                match args.as_slice() {
                    [Ty::Prp(p), base] => {
                        let binder = pred_binder(p);
                        self.law_samples(base, depth)
                            .into_iter()
                            .filter(|s| self.pred_value(p, &binder, s) == Some(true))
                            .collect()
                    }
                    _ => Vec::new(),
                }
            }
            Ty::Adt(name, args) => {
                if depth == 0 {
                    return Vec::new();
                }
                let Some(tks) = self.types.get(name) else {
                    return Vec::new();
                };
                let Some(tk) = tks.first() else {
                    return Vec::new();
                };
                if let Some(r) = &tk.refine {
                    // A refinement's samples are the container's samples
                    // its predicate decides true (refinement types
                    // design: sample-wise checking runs over refined
                    // values).
                    let mut map: HashMap<String, Ty> = HashMap::new();
                    for (i, p) in r.params.iter().enumerate() {
                        if let Some(a) = args.get(i) {
                            map.insert(p.clone(), a.clone());
                        }
                    }
                    let base = Self::subst_ty(&r.base, &map);
                    return self
                        .law_samples(&base, depth)
                        .into_iter()
                        .filter(|s| self.pred_value(&r.pred, &r.binder, s) == Some(true))
                        .collect();
                }
                let mut out: Vec<Value> = Vec::new();
                for (cname, tag) in &tk.ctors {
                    let Some(vs) = self.values.get(cname) else {
                        continue;
                    };
                    let Some(v) = vs
                        .iter()
                        .find(|v| matches!(v.kind, ValueKind::Ctor { .. }))
                    else {
                        continue;
                    };
                    let ValueKind::Ctor {
                        field_tys,
                        field_erased,
                        ty_params,
                        ..
                    } = &v.kind
                    else {
                        continue;
                    };
                    let mut map: HashMap<String, Ty> = HashMap::new();
                    for (i, p) in ty_params.iter().enumerate() {
                        if let Some(a) = args.get(i) {
                            map.insert(p.clone(), a.clone());
                        }
                    }
                    let mut field_sets: Vec<Vec<Value>> = Vec::new();
                    let mut ok = true;
                    for (ft, er) in field_tys.iter().zip(field_erased.iter()) {
                        if *er {
                            continue;
                        }
                        let ft = Self::subst_ty(ft, &map);
                        let samples = self.law_samples(&ft, depth - 1);
                        if samples.is_empty() {
                            ok = false;
                            break;
                        }
                        field_sets.push(samples.into_iter().take(2).collect());
                    }
                    if !ok {
                        continue;
                    }
                    let mut combos: Vec<Vec<Value>> = vec![Vec::new()];
                    for fs in &field_sets {
                        let mut next = Vec::new();
                        for c in &combos {
                            for f in fs {
                                let mut c2 = c.clone();
                                c2.push(f.clone());
                                next.push(c2);
                            }
                        }
                        combos = next;
                        if combos.len() > 3 {
                            combos.truncate(3);
                        }
                    }
                    for fields in combos {
                        out.push(Value::Adt {
                            ctor: fn_id(&tk.module, cname).0,
                            tag: *tag,
                            fields,
                        });
                    }
                    if out.len() >= 4 {
                        break;
                    }
                }
                out.truncate(4);
                out
            }
            _ => Vec::new(),
        }
    }

    /// The field names, field types and result arguments of a visible
    /// `record` type, from its single constructor (the record type name).
    fn record_fields(&self, name: &str, span: ast::Span) -> Result<(Vec<String>, Vec<Ty>, Vec<Ty>)> {
        let mut found = None;
        for v in self.values.get(name).into_iter().flatten() {
            if !self.visible.contains(&v.module) {
                continue;
            }
            if let ValueKind::Ctor {
                field_binders,
                field_tys,
                res_args,
                ..
            } = &v.kind
            {
                found = Some((field_binders.clone(), field_tys.clone(), res_args.clone()));
                break;
            }
        }
        found.ok_or_else(|| {
            Error::at(span, format!("{name} is not a record type (an upgrade relates two records)"))
        })
    }

    /// The type of a simple `migrate`/`related` term: it names a visible
    /// value, whose scheme is the migration's type (runtime evolution
    /// specification section 3).
    fn component_ty(&self, tm: &ast::Term, span: ast::Span, what: &str) -> Result<Ty> {
        let name = match tm {
            ast::Term::Var(v) => v.name.text.clone(),
            _ => {
                return Err(Error::at(
                    span,
                    format!("the {what} component must name a function"),
                ))
            }
        };
        for v in self.values.get(&name).into_iter().flatten() {
            if self.visible.contains(&v.module) {
                return Ok(v.scheme.ty.clone());
            }
        }
        for module in &self.visible {
            if let Some(s) = self.top_schemes.get(&(module.clone(), name.clone())) {
                return Ok(s.ty.clone());
            }
        }
        Err(Error::at(span, format!("unknown {what} function {name}")))
    }

    /// One `upgrade` declaration (runtime evolution specification
    /// sections 2-3). The migration field checking classifies every field
    /// of the two records (kept, added, removed, retyped) and enforces:
    /// a retyped field is an incompatible change and needs a migration; a
    /// migration's function type is exactly `Old %1 -> New`; a record view
    /// (no migration) admits only compatible (additive) changes and keeps
    /// added optional fields distinguishable from missing ones.
    fn check_upgrade(&mut self, u: &ast::UpgradeDecl) -> Result<()> {
        let old_name = u.old.segments.last().expect("a con path has a name").text.clone();
        let new_name = u.new.segments.last().expect("a con path has a name").text.clone();
        let (old_fields, old_tys, old_res) = self.record_fields(&old_name, u.old.span())?;
        let (new_fields, new_tys, new_res) = self.record_fields(&new_name, u.new.span())?;

        // Classify each field's change across the evolution (the runtime
        // evolution section 2 dependency table: inlined bodies and type
        // equations must remain valid or be replaced).
        let mut retyped: Vec<String> = Vec::new();
        let mut added: Vec<(String, Ty)> = Vec::new();
        for (i, nf) in new_fields.iter().enumerate() {
            match old_fields.iter().position(|of| of == nf) {
                Some(j) => {
                    if old_tys[j] != new_tys[i] {
                        retyped.push(nf.clone());
                    }
                }
                None => added.push((nf.clone(), new_tys[i].clone())),
            }
        }

        match &u.migrate {
            Some(mig) => {
                // The migration function is `Old %1 -> New` (section 3):
                // it establishes the state relation and the checked field
                // mapping, covering kept, added, removed and retyped
                // fields alike.
                let expected = Ty::fun_mult(
                    Ty::Adt(old_name.clone(), old_res),
                    Mult::One,
                    EffRow::pure(),
                    Ty::Adt(new_name.clone(), new_res),
                );
                let actual = self.component_ty(mig, u.span, "migrate")?;
                if actual != expected {
                    return Err(Error::at(
                        u.span,
                        format!(
                            "the migration must have type `{expected}`, but it is declared `{actual}` (runtime evolution specification section 3)"
                        ),
                    ));
                }
                self.record_upgrade_evidence(&old_name, &new_name, true, &added, &retyped);
            }
            None => {
                // A record view: a compatible (additive) evolution
                // admitted without a migration. A retyped field is an
                // incompatible change and needs a migration.
                if let Some(f) = retyped.first() {
                    return Err(Error::at(
                        u.span,
                        format!(
                            "incompatible field change `{f}` in the evolution of {old_name} to {new_name} needs a migration (runtime evolution specification section 2)"
                        ),
                    ));
                }
                self.record_upgrade_evidence(&old_name, &new_name, false, &added, &retyped);
            }
        }
        Ok(())
    }

    /// Record the schema compatibility evidence of one upgrade in the
    /// module's interface (runtime evolution specification section 11;
    /// the `.cinchif` "checked against" chain extends to versioned
    /// identities). `added` optional fields stay distinguishable from
    /// missing fields under the view (core rule).
    fn record_upgrade_evidence(
        &mut self,
        old: &str,
        new: &str,
        migrated: bool,
        added: &[(String, Ty)],
        retyped: &[String],
    ) {
        self.upgrade_evidence.push((
            self.current_module.clone(),
            crate::interface::UpgradeEvidence {
                old: old.to_string(),
                new: new.to_string(),
                migrated,
                added: added.iter().map(|(n, t)| (n.clone(), t.to_string())).collect(),
                retyped: retyped.to_vec(),
            },
        ));
    }

    /// Check one rewrite declaration. The justification mechanism
    /// (decision D55): `justify` must check under the rule's equation,
    /// either through the ordinary proof checker (`refl`, `congr`, and
    /// declared `proof`/`spec` functions) or as a law method of a
    /// `signature`, whose declared type is the checked obligation and
    /// whose identity is recorded in the manifest's `assumptions.declared`
    /// when no instance discharges it. In every case the rule must
    /// preserve types, satisfy the resource side conditions, avoid
    /// effectful instantiations its pure equation does not authorize
    /// (modules section 5), and never extend definitional equality
    /// (conversion stays exactly the D28 rules).
    fn check_rewrite(&mut self, r: &ast::RewriteDecl) -> Result<()> {
        self.reset_usage();
        self.kind_constraints.borrow_mut().clear();
        let span = r.span;
        let mut names: Vec<String> = Vec::new();
        for b in &r.binders {
            if names.contains(&b.name.text) {
                return Err(Error::at(b.name.span, "duplicate rewrite binder"));
            }
            names.push(b.name.text.clone());
        }
        for c in &r.ctx {
            if let ast::Oblig::Class(k) = c {
                if names.contains(&k.name.text) {
                    return Err(Error::at(k.span, "duplicate rewrite binder"));
                }
                names.push(k.name.text.clone());
            }
        }
        // Binder hints from their annotations, and the class constraints
        // of the `using` clause, record how each bare binder is used.
        let mut hints: Vec<(String, Option<ParamKind>)> = Vec::new();
        for b in &r.binders {
            let hint = match &b.ann {
                Some(ann) => Some(self.binder_kind(ann, &names)?),
                None => None,
            };
            hints.push((b.name.text.clone(), hint));
        }
        let mut ctx: Vec<(String, ClassApp)> = Vec::new();
        let mut ctx_tys: Vec<Ty> = Vec::new();
        for c in &r.ctx {
            let (n, app) = match self.conv_oblig(c, &names)? {
                ConvOblig::Class(n, app) => (n, app),
                ConvOblig::Prop(_) => {
                    return Err(Error::at(
                        c.span(),
                        "a rewrite's `using` context states the class constraints its template may use; refinement obligations live on instances and signatures",
                    ))
                }
            };
            ctx_tys.extend(app.args.iter().cloned());
            ctx.push((n, app));
        }
        // Region and row names from the annotations and constraints.
        let mut region_params: Vec<String> = Vec::new();
        let mut row_params: Vec<String> = Vec::new();
        let mut ann_tys: Vec<Ty> = Vec::new();
        for b in &r.binders {
            if let Some(ann) = &b.ann {
                let t = self.conv_type(ann, &names)?;
                ann_tys.push(t);
            }
        }
        for t in ann_tys.iter().chain(ctx_tys.iter()) {
            collect_row_names(t, &mut row_params, &mut region_params);
            let mut regs: Vec<(String, bool)> = Vec::new();
            borrow_regions(t, &mut regs);
            for (rr, _) in regs {
                if !region_params.contains(&rr) {
                    region_params.push(rr);
                }
            }
        }
        // Classify the bare binders by how they are used (decision D29).
        let constraints = self.kind_constraints.borrow().clone();
        let mut kinds: HashMap<String, ParamKind> = HashMap::new();
        for (name, hint) in &hints {
            if row_params.contains(name) || region_params.contains(name) {
                continue;
            }
            let uses: Vec<ParamKind> = constraints
                .iter()
                .filter(|(n, _)| n == name)
                .map(|(_, k)| k.clone())
                .collect();
            let axis = |k: &ParamKind| match k {
                ParamKind::Type => 0u8,
                ParamKind::Val(_) => 1,
                ParamKind::Prop => 2,
                ParamKind::Con(_) => 3,
            };
            let mut distinct_axis: Option<u8> = None;
            let mut first_val: Option<ParamKind> = None;
            for k in &uses {
                match distinct_axis {
                    None => distinct_axis = Some(axis(k)),
                    Some(prev) if prev != axis(k) => distinct_axis = None,
                    _ => {}
                }
                if first_val.is_none() && matches!(k, ParamKind::Val(_)) {
                    first_val = Some(k.clone());
                }
            }
            let kind = match (hint, distinct_axis) {
                (Some(h), _) => h.clone(),
                (None, Some(1)) => first_val.unwrap_or(ParamKind::Type),
                (None, Some(2)) => ParamKind::Prop,
                (None, _) => ParamKind::Type,
            };
            kinds.insert(name.clone(), kind);
        }
        // The rule's local environment: value binders and dictionaries
        // are term-level locals (a rule's template consumes both), type
        // and proposition binders are rigid names in the converted types.
        let mut env = Env::default();
        let mut lids: HashMap<String, LocalId> = HashMap::new();
        for (name, hint) in &hints {
            let l = self.fresh_local();
            lids.insert(name.clone(), l);
            let ty = match kinds.get(name) {
                Some(ParamKind::Val(t)) => t.clone(),
                Some(ParamKind::Prop) => {
                    Ty::Prp(Box::new(PTerm::Var(name.clone())))
                }
                _ => Ty::Param(name.clone()),
            };
            if matches!(kinds.get(name), Some(ParamKind::Val(_)) | Some(ParamKind::Prop)) {
                self.bind_local(&mut env, name, l, Scheme::mono(ty), None, span)?;
            }
            let _ = hint;
        }
        let mut ctx_lids: Vec<(String, LocalId, ClassApp)> = Vec::new();
        for (n, app) in &ctx {
            let l = self.fresh_local();
            let ty = Ty::Adt(app.class.clone(), app.args.clone());
            self.bind_local(&mut env, n, l, Scheme::mono(ty), None, span)?;
            env.dicts.push((l, app.clone()));
            lids.insert(n.clone(), l);
            ctx_lids.push((n.clone(), l, app.clone()));
        }
        // Elaborate the four terms in this context, recording the effects
        // each performs.
        let saved = self.enter_perf();
        let (match_ec, match_ty) = self.infer(&mut env, &r.match_tm)?;
        let match_perf = self.exit_perf(saved);
        let saved = self.enter_perf();
        let (replace_ec, replace_ty) = self.infer(&mut env, &r.replace_tm)?;
        let replace_perf = self.exit_perf(saved);
        let require_pair = match &r.require {
            Some(req) => {
                let saved = self.enter_perf();
                let (ec, ty) = self.infer(&mut env, req)?;
                let perf = self.exit_perf(saved);
                Some((ec, ty, perf, req.clone()))
            }
            None => None,
        };
        // Type preservation: the replacement must have the pattern's
        // type (modules section 5: the substitution must satisfy the
        // theorem's type and effect conditions).
        self.uni
            .unify(&match_ty, &replace_ty)
            .map_err(|m| Error::at(span, format!("the `replace` term must have the `match` term's type: {m}")))?;
        let mty = self.uni.zonk(&match_ty);
        // Definitional equality is exactly the D28 rules: a rule over
        // erased evidence would break proof irrelevance, and a rule over
        // types would make a source type mean something else.
        match &mty {
            Ty::Prp(_) => {
                return Err(Error::at(
                    span,
                    "a rewrite over erased evidence would change definitional equality (proof irrelevance) and resurrect erased components",
                ))
            }
            Ty::Sort(_) => {
                return Err(Error::at(
                    span,
                    "a rewrite over types would change definitional equality: the pattern's type is a universe",
                ))
            }
            _ => {}
        }
        // The rule's own instantiation is pure: the equation is `Eq`, and
        // a theorem proved for pure values must not be reused for
        // effectful computations (modules section 5). Constraining the
        // performed row to empty also rejects row-polymorphic rules,
        // whose callbacks can perform anything.
        for (what, perf) in [
            ("`match`", match_perf),
            ("`replace`", replace_perf),
            ("`require`", require_pair.as_ref().map(|p| p.2.clone()).unwrap_or_else(EffRow::pure)),
        ] {
            self.uni.unify_rows(&perf, &EffRow::pure()).map_err(|_| {
                Error::at(
                    span,
                    format!(
                        "an effectful rewrite rule needs an observational-equivalence justification, which the proof kernel does not express in this slice: the {what} term performs effects, and pure `Eq` evidence does not authorize reordering them"
                    ),
                )
            })?;
        }
        // The rule's equation over index values: the same reification the
        // proposition language uses (decision D29).
        let lhs = self.idx_of_term(&r.match_tm, &env)?;
        let rhs = self.idx_of_term(&r.replace_tm, &env)?;
        let mut lhs_vars: Vec<String> = Vec::new();
        lhs.free_vars(&mut lhs_vars);
        let mut rhs_vars: Vec<String> = Vec::new();
        rhs.free_vars(&mut rhs_vars);
        // The pattern must be headed by a defined unit: a bare variable
        // pattern would rewrite every term of its type and change
        // definitional equality.
        let mut head: &ECore = &match_ec;
        while let ECore::App(f, _) = head {
            head = f;
        }
        match head {
            ECore::Global(_) | ECore::Ctor(_) | ECore::Prim(_) | ECore::DictProj(_, _) => {}
            _ => {
                return Err(Error::at(
                    r.match_tm.span(),
                    "the rule's `match` pattern must be headed by a defined function, constructor, or primitive: a bare variable pattern would change definitional equality",
                ))
            }
        }
        // Resource side conditions: the template's free variables are
        // bound by the pattern or the `using` context, and a linear
        // binder is consumed exactly once by the template (a rule must
        // not duplicate or drop a `%1`/`%aff` value).
        let dict_names: Vec<String> = ctx_lids.iter().map(|(n, _, _)| n.clone()).collect();
        for v in rhs_vars.iter().chain(std::iter::empty()) {
            if !lhs_vars.contains(v) && !dict_names.contains(v) {
                return Err(Error::at(
                    span,
                    format!(
                        "the `replace` term mentions `{v}`, which the `match` pattern does not bind"
                    ),
                ));
            }
        }
        for (name, l) in &lids {
            if dict_names.contains(name) {
                continue;
            }
            let ty = match kinds.get(name) {
                Some(ParamKind::Val(t)) => t.clone(),
                _ => continue,
            };
            if matches!(default_mult(&ty), Mult::One | Mult::Aff) {
                let count = rhs_vars.iter().filter(|v| *v == name).count();
                if count != 1 {
                    return Err(Error::at(
                        span,
                        format!(
                            "the `replace` term uses the linear binder `{name}` {count} times: a rewrite must not duplicate or drop a `%1`/`%aff` value"
                        ),
                    ));
                }
            }
            let _ = l;
        }
        // Every pattern variable the rule fires on must be a binder of
        // the rule; unused binders are fine (they appear only in the
        // condition or the justification).
        for v in &lhs_vars {
            if !lids.contains_key(v) {
                return Err(Error::at(
                    span,
                    format!("the `match` pattern mentions `{v}`, which is not a rewrite binder"),
                ));
            }
        }
        // The justification: checked evidence under the rule's equation.
        let eq = PTerm::Eq(Box::new(lhs), Box::new(rhs));
        let assumptions = self.check_justify(&mut env, &r.justify, &eq, span)?;
        // The `require` clause's operative form (platform section 3).
        let cond = match &require_pair {
            None => crate::core::RequireCond::Always,
            Some((_, ty, _, _)) => {
                if matches!(self.uni.zonk(ty), Ty::Prp(_)) {
                    return Err(Error::at(
                        span,
                        "a `require` condition is operational evidence, not a proof: it must have a runtime type",
                    ));
                }
                crate::core::RequireCond::Unknown
            }
        };
        // Lower the pattern and template (decision D28: beta at
        // elaboration; the holes are the rule's binder locals). Literal
        // words and implicit index arguments resolve positionally across
        // all four terms at once.
        let sol = self.solve_wants()?;
        let mut m = match_ec;
        let mut rp = replace_ec;
        let mut req_ec = require_pair.as_ref().map(|p| p.0.clone());
        m = self.fill_holes(&m, &sol)?;
        rp = self.fill_holes(&rp, &sol)?;
        if let Some(req) = &mut req_ec {
            *req = self.fill_holes(req, &sol)?;
        }
        let lit_pends = std::mem::take(&mut self.literal_pends);
        let mut lits: Vec<ECore> = Vec::with_capacity(lit_pends.len());
        for p in &lit_pends {
            let t = self.uni.zonk(&p.ty);
            if matches!(t, Ty::Meta(_)) {
                return Err(Error::at(
                    p.span,
                    "the type of this integer literal is not determined by context; add a type annotation",
                ));
            }
            lits.push(self.literal_const(&t, &p.value, p.span)?);
        }
        let idx_pends = std::mem::take(&mut self.idx_pends);
        let mut vals: Vec<ECore> = Vec::with_capacity(idx_pends.len());
        for p in &idx_pends {
            vals.push(self.reify_idx(&p.iterm, &p.names, p.nat, p.span)?);
        }
        for body in [&mut m, &mut rp] {
            resolve_lit_words(body, &lits);
            resolve_idx_words(body, &vals);
        }
        if let Some(req) = &mut req_ec {
            resolve_lit_words(req, &lits);
            resolve_idx_words(req, &vals);
        }
        let mut hoisted = Vec::new();
        let rule_unit = GlobalId(format!("{}.rewrite.{}", self.current_module, r.name.text));
        hoist_lambdas(&mut m, &rule_unit.0, &mut self.lam_counter, &mut hoisted);
        hoist_lambdas(&mut rp, &rule_unit.0, &mut self.lam_counter, &mut hoisted);
        for h in hoisted {
            self.fns.push(finalize_fn(
                h.id,
                h.params,
                h.body,
                None,
                None,
                crate::core::FnHints::default(),
                crate::core::FnMeta::default(),
            ));
        }
        let pattern = lower(&m);
        let template = lower(&rp);
        let require = req_ec.as_ref().map(|req| lower(req));
        // The `require` clause's operative condition: `Available v` over a
        // capability binder is valid exactly where that capability is in
        // the firing function's validity scope.
        let cond = match (&require, cond) {
            (Some(req), crate::core::RequireCond::Unknown) => match req {
                crate::core::Core::App(f, args) => match (&**f, args.as_slice()) {
                    (
                        crate::core::Core::Global(_) | crate::core::Core::Ctor(_),
                        [crate::core::Core::Var(l)],
                    ) if ctx_lids.iter().any(|(_, c, _)| c == l) => {
                        let class = ctx_lids
                            .iter()
                            .find(|(_, c, _)| c == l)
                            .map(|(_, _, a)| a.class.clone())
                            .expect("ctx binder");
                        crate::core::RequireCond::CapabilityInScope { hole: *l, class }
                    }
                    _ => crate::core::RequireCond::Unknown,
                },
                crate::core::Core::Bool(true) => crate::core::RequireCond::Always,
                crate::core::Core::Bool(false) => crate::core::RequireCond::Never,
                _ => crate::core::RequireCond::Unknown,
            },
            (_, other) => other,
        };
        // Pattern variables: every binder of the rule (a pattern position
        // may be an implicit index argument the elaboration reified).
        let mut holes: Vec<LocalId> = Vec::new();
        for l in lids.values() {
            holes.push(*l);
        }
        holes.sort_by_key(|l| l.0);
        // Every binder the template or condition mentions must be bound
        // at a match site: by the pattern or by a `using` capability.
        let mut template_holes: Vec<LocalId> = Vec::new();
        crate::core::walk_locals(&template, &mut |l: LocalId| {
            if lids.values().any(|h| *h == l) {
                template_holes.push(l);
            }
        });
        let mut pattern_holes: Vec<LocalId> = Vec::new();
        crate::core::walk_locals(&pattern, &mut |l: LocalId| {
            if lids.values().any(|h| *h == l) {
                pattern_holes.push(l);
            }
        });
        for l in &template_holes {
            if !pattern_holes.contains(l) && !ctx_lids.iter().any(|(_, c, _)| c == l) {
                return Err(Error::at(
                    span,
                    "the `replace` term mentions a binder the `match` pattern does not bind",
                ));
            }
        }
        let pure_holes: Vec<LocalId> = r
            .binders
            .iter()
            .filter(|b| {
                kinds
                    .get(&b.name.text)
                    .map(|k| matches!(k, ParamKind::Val(t) if pure_arrow_ty(t)))
                    .unwrap_or(false)
            })
            .filter_map(|b| lids.get(&b.name.text).copied())
            .collect();
        let dict_holes: Vec<(LocalId, String)> = ctx_lids
            .iter()
            .map(|(_, l, app)| (*l, app.class.clone()))
            .collect();
        self.rewrites.push(crate::core::RewriteRule {
            name: r.name.text.clone(),
            module: self.current_module.clone(),
            target: r.target,
            holes,
            pattern,
            template,
            require,
            cond,
            pure_holes,
            dict_holes,
            assumptions,
            line: r.span.line as u32,
        });
        Ok(())
    }

    /// `justify e`: checked evidence under the rule's equation (decision
    /// D55). A law method of a `signature` is the checked obligation its
    /// declared type states (platform proofs and SIMD specification
    /// section 6: instruction laws); everything else is checked by the
    /// ordinary proof kernel (`refl`, `congr`, `decide` extractions, and
    /// declared `proof`/`spec` functions).
    fn check_justify(
        &mut self,
        env: &mut Env,
        tm: &ast::Term,
        eq: &PTerm,
        span: ast::Span,
    ) -> Result<Vec<String>> {
        let head: &ast::Term = match tm {
            ast::Term::App(f, _, _) => f,
            other => other,
        };
        if let ast::Term::Var(p) = head {
            if let Ok(v) = self.resolve_var(p, p.span()) {
                if matches!(v.kind, ValueKind::Method { .. } | ValueKind::Law) {
                    return self.check_law_justify(env, tm, &v, eq, span);
                }
            }
        }
        self.check_proof(env, tm, eq)?;
        Ok(Vec::new())
    }

    /// A law method as justification: instantiate its declared type,
    /// check the arguments, unify the resulting proposition with the
    /// rule's equation, and record the law as a declared assumption when
    /// no instance discharges it.
    fn check_law_justify(
        &mut self,
        env: &mut Env,
        tm: &ast::Term,
        v: &ValueInfo,
        eq: &PTerm,
        span: ast::Span,
    ) -> Result<Vec<String>> {
        let args: Vec<ast::Term> = match tm {
            ast::Term::App(_, args, _) => args.clone(),
            _ => Vec::new(),
        };
        let (mut ty, _ctx, _) = self.instantiate(&v.scheme);
        for a in &args {
            let zt = self.uni.zonk(&ty);
            let (var, dom, cod) = match zt {
                Ty::Fun(d, _, _, c) => (String::from("$"), *d, *c),
                Ty::Pi { var, dom, cod, .. } => (var, *dom, *cod),
                other => {
                    return Err(Error::at(
                        a.span(),
                        format!("the justification takes no further arguments (remaining type {other})"),
                    ))
                }
            };
            self.check(env, a, &dom)?;
            let subst = self.idx_of_term(a, env)?;
            ty = cod.subst_idx(&var, &subst);
        }
        let result = self.uni.zonk(&ty);
        match result {
            Ty::Prp(p) => {
                self.uni.unify_prop(&p, eq).map_err(|m| {
                    Error::at(
                        span,
                        format!(
                            "the justification does not prove the rule's equation: it proves `p`, and the rule states `Eq` over its `match` and `replace` terms ({m})"
                        ),
                    )
                })?;
            }
            other => {
                return Err(Error::at(
                    span,
                    format!("the justification must state a proposition, but has type {other}"),
                ))
            }
        }
        Ok(vec![format!("{}.{}", v.module, v.name)])
    }

    fn finish(mut self, entry_modules: &[String], prelude_name: &str) -> Result<Program> {
        let _ = prelude_name;
        // Instantiation units emit their wrapper units last (module
        // parameters as module arguments): the target module's units are
        // all present by now.
        self.emit_unit_wrappers()?;
        // `main` may live in any module of the entry file (a source file
        // may declare several modules); a library build has none, and
        // `emit-ir`/`build`/`run` name the requirement at codegen time
        // (decision D20).
        let mut mains: Vec<ValueInfo> = Vec::new();
        if let Some(vs) = self.values.get("main") {
            for v in vs {
                if entry_modules.contains(&v.module) {
                    mains.push(v.clone());
                }
            }
        }
        if mains.len() > 1 {
            let names: Vec<String> = mains.iter().map(|v| v.module.clone()).collect();
            return Err(Error::plain(format!(
                "several modules of the entry file define `main` ({}); the entry file's `main` is unique",
                names.join(", ")
            )));
        }
        let main_info = mains.into_iter().next();
        let mut main_exit = None;
        if let Some(main_info) = main_info {
            if !main_info.scheme.params.is_empty() || !main_info.scheme.ctx.is_empty() {
                return Err(Error::plain("main must have type Int or Unit ->{e} Int in the bootstrap slice"));
            }
            // The entry rule (decision D65): `main :: Int` is the pure
            // entry; `main :: Unit ->{e} Int` is the effectful entry,
            // whose declared row is the program's runtime effect
            // envelope (foreign function interface specification
            // section 4's accounting reaches the artifact boundary).
            let main_ty = self.uni.zonk(&main_info.scheme.ty);
            let effectful = match &main_ty {
                Ty::Int => false,
                Ty::Fun(dom, _, _, cod) => {
                    if **dom != Ty::Unit || **cod != Ty::Int {
                        return Err(Error::plain(format!(
                            "main must have type Int or Unit ->{{e}} Int in the bootstrap slice, got {main_ty}"
                        )));
                    }
                    true
                }
                other => {
                    return Err(Error::plain(format!(
                        "main must have type Int or Unit ->{{e}} Int in the bootstrap slice, got {other}"
                    )))
                }
            };
            // The exit adapter (mathematical `Int` slice): `main`'s
            // result is a boxed bignum while the C `main` truncates its
            // unit's word to `i32`, so the checker synthesizes
            // `"<module>.main.exit"`, whose body converts the `main`
            // unit's result to a machine `i64` through `cinch_int_to_i64`,
            // and `Program.main` points at it. An effectful `main` is
            // applied to its `Unit` argument first.
            let user_module = main_info.module.clone();
            let main_id = fn_id(&user_module, "main");
            let exit_id = GlobalId(format!("{user_module}.main.exit"));
            let main_word = if effectful {
                crate::core::Core::App(
                    Box::new(crate::core::Core::Global(main_id)),
                    vec![crate::core::Core::Unit],
                )
            } else {
                crate::core::Core::Global(main_id)
            };
            self.fns.push(FnDef {
                id: exit_id.clone(),
                params: Vec::new(),
                body: crate::core::Core::App(
                    Box::new(crate::core::Core::Prim(PrimOp::Int(IntOp::ToI64))),
                    vec![main_word],
                ),
                prim: None,
                ctor: None,
                hints: crate::core::FnHints::default(),
                    meta: crate::core::FnMeta::default(),
            });
            main_exit = Some(exit_id);
        }
        let (plans, foreigns) = self.build_abi_maps();
        Ok(Program {
            fns: self.fns,
            instances: self.instance_defs,
            main: main_exit,
            rewrites: self.rewrites,
            plans,
            foreigns,
            prims: self.prims,
            laws: self.law_rows,
        })
    }

    /// The `prim` signature envelope (primitive operations and inline
    /// target IR specification section 2): a monomorphic function type
    /// with closed, explicit effect rows (exactly the foreign chapter's
    /// rule for imports), whose arguments and result are machine
    /// scalars: an inline body receives exactly the declared arguments
    /// in the declared ABI and returns the declared result shape
    /// (foreign function interface specification section 2: widths,
    /// signedness, convention).
    fn check_prim_envelope(&self, p: &ast::PrimDecl, ty: &Ty) -> Result<(Vec<Ty>, Ty)> {
        let mut args: Vec<Ty> = Vec::new();
        let mut rows: Vec<&EffRow> = Vec::new();
        let mut cur = ty;
        loop {
            match cur {
                Ty::Fun(d, _, e, b) | Ty::Pi { dom: d, eff: e, cod: b, .. } => {
                    rows.push(e);
                    args.push((**d).clone());
                    cur = b;
                }
                _ => break,
            }
        }
        for r in &rows {
            if !matches!(r.tail, crate::types::RowTail::Closed) {
                return Err(Error::at(
                    p.span,
                    "a prim's effect rows are closed and explicit (primitive operations and inline target IR specification section 2)",
                ));
            }
        }
        for a in &args {
            if !is_prim_abi_ty(a) {
                return Err(Error::at(
                    p.span,
                    format!(
                        "a prim body receives its arguments as machine scalars in the declared ABI (foreign function interface specification section 2), not {a}"
                    ),
                ));
            }
        }
        let res = cur.clone();
        if !is_prim_abi_ty(&res) {
            return Err(Error::at(
                p.span,
                format!(
                    "a prim body returns the declared result shape: a machine scalar (foreign function interface specification section 2), not {res}"
                ),
            ));
        }
        Ok((args, res))
    }

    /// The LLVM type spelling of a prim ABI scalar (the family's
    /// fragment grammar speaks these widths).
    fn prim_abi_word(&self, ty: &Ty) -> Result<String> {
        use crate::types::Scalar;
        Ok(match ty {
            Ty::Prim(Scalar::I8) | Ty::Prim(Scalar::U8) => "i8".into(),
            Ty::Prim(Scalar::I16) | Ty::Prim(Scalar::U16) => "i16".into(),
            Ty::Prim(Scalar::I32) | Ty::Prim(Scalar::U32) | Ty::Prim(Scalar::Char) => "i32".into(),
            Ty::Prim(Scalar::I64) | Ty::Prim(Scalar::U64) => "i64".into(),
            Ty::Prim(Scalar::F32) => "float".into(),
            Ty::Prim(Scalar::F64) => "double".into(),
            Ty::Prim(Scalar::Usize) | Ty::Prim(Scalar::Isize) => format!("i{}", self.usize_bits),
            Ty::Bool => "i8".into(),
            other => {
                return Err(Error::plain(format!(
                    "internal error: {other} is not a prim ABI type"
                )))
            }
        })
    }

    /// One target-IR body (primitive operations and inline target IR
    /// specification sections 3-4): the family must be registered and
    /// emittable, `uses` entries are full foreign binding contracts, and
    /// the fragment passes its family's containment validation. An
    /// inline body is trusted code, so validation is about containment,
    /// never about proving semantics. Returns the validation result the
    /// manifest records (section 8).
    fn validate_prim_body(
        &mut self,
        module: &str,
        p: &ast::PrimDecl,
        b: &ast::PrimBody,
        arg_tys: &[Ty],
        res_ty: &Ty,
        pure: bool,
    ) -> Result<(String, Vec<(String, String)>)> {
        let family = b.family.text.as_str();
        match family {
            "llvm" | "asm" | "wasm" => {}
            "clr" | "jvm" => {
                return Err(Error::at(
                    b.family.span,
                    format!(
                        "the `{family}` target-IR family is reserved: its fragment grammar lands with its backend (primitive operations and inline target IR specification section 3)"
                    ),
                ))
            }
            other => {
                return Err(Error::at(
                    b.family.span,
                    format!(
                        "unknown target-IR family `{other}`: the registered families are `llvm`, `wasm`, `asm`, `clr` and `jvm` (primitive operations and inline target IR specification section 3)"
                    ),
                ))
            }
        }
        // `uses` entries are full foreign binding contracts (section 4),
        // recorded like foreign imports; the reserved intrinsic
        // namespaces stay reserved for them too (decision D73).
        let mut use_names: Vec<String> = Vec::new();
        let mut use_renders: Vec<(String, String)> = Vec::new();
        for u in &b.uses {
            let us = self.conv_scheme(&u.ty)?;
            if !us.params.is_empty() || !us.ctx.is_empty() {
                return Err(Error::at(
                    u.span,
                    "a `uses` entry is a foreign binding contract: monomorphic and unconstrained (foreign function interface specification section 2)",
                ));
            }
            if u.name.text.starts_with("cinch.prim.")
                || u.name.text.starts_with("cinch.runtime.")
            {
                return Err(Error::at(
                    u.name.span,
                    format!(
                        "the intrinsic namespaces are reserved (decision D73): `uses` must not name the intrinsic {} (primitive operations and inline target IR specification section 7)",
                        u.name.text
                    ),
                ));
            }
            use_names.push(u.name.text.clone());
            use_renders.push((u.name.text.clone(), render_scheme(&us)));
            self.module_foreign.push((
                module.to_string(),
                crate::interface::ForeignExport {
                    symbol: u.name.text.clone(),
                    abi: "ccall".to_string(),
                    external: u.name.text.clone(),
                    ty: render_scheme(&us),
                    effects: String::new(),
                    notes: format!("prim `{}` `uses` contract", p.name.text),
                    kind: "prim-use".to_string(),
                    params: Vec::new(),
                    result: crate::interface::ForeignParam {
                        ty: String::new(),
                        class: "unit".to_string(),
                        role: "value".to_string(),
                        allocator: String::new(),
                        nullable: false,
                        width: 0,
                        align: 1,
                        signed: false,
                        offsets: Vec::new(),
                    },
                    deallocator: String::new(),
                },
            ));
        }
        // The `when` condition is the platform chapter's capability
        // evidence (`Available X`), scoped exactly like `target
        // rewrite`'s `require` (decision D59's rule).
        if let Some(w) = &b.when {
            self.lower_when(w)?;
        }
        // The `unsafe` trust rule (section 12): a body containment
        // cannot verify (an `asm` template, a `wasm` fragment, calls
        // through `uses` into opaque code) declares its row under
        // `unsafe`, and the trust is recorded; nothing is silently
        // trusted.
        if matches!(family, "asm" | "wasm") && !b.unverified {
            return Err(Error::at(
                b.span,
                format!(
                    "containment cannot verify a `{family}` body's effects (primitive operations and inline target IR specification section 12): mark it `unsafe` to trust its declared effect row, and the trust is recorded as an assumption"
                ),
            ));
        }
        let validation = match family {
            "llvm" => {
                let (v, opaque_calls) =
                    self.contain_llvm(p, b, arg_tys, res_ty, &use_names, pure)?;
                if opaque_calls && !b.unverified {
                    return Err(Error::at(
                        b.span,
                        "containment cannot verify the effects of a call through `uses` into opaque code (primitive operations and inline target IR specification section 12): mark the body `unsafe` to trust its declared effect row, and the trust is recorded as an assumption",
                    ));
                }
                v
            }
            "asm" => self.contain_asm(b, &use_names)?,
            _ => {
                "wasm fragment accepted structurally; this build has no wasm body emitter (specification section 3: a `wasm` body is selected only for a wasm artifact)"
                    .to_string()
            }
        };
        Ok((validation, use_renders))
    }

    /// A body's `when` condition (primitive operations and inline target
    /// IR specification sections 2 and 13): `Available X` over a
    /// capability name of the platform chapter's evidence, or the
    /// platform predicate `arch <name>` keyed on the target triple.
    fn lower_when(&self, t: &ast::Term) -> Result<WhenCond> {
        let bad = |span: ast::Span| {
            Error::at(
                span,
                "a `when` condition is `Available <capability>` over the target profile's capability evidence, or the platform predicate `arch <name>` over its triple (primitive operations and inline target IR specification sections 2 and 13; platform proofs and SIMD specification section 3)",
            )
        };
        let (head, arg, span) = match t {
            ast::Term::App(f, args, span) => match (&**f, args.as_slice()) {
                (h, [a]) => (h, a, *span),
                _ => return Err(bad(*span)),
            },
            other => return Err(bad(other.span())),
        };
        let head_last = match head {
            ast::Term::Var(p) => p.name.text.clone(),
            ast::Term::Con(p) => p.parts().last().copied().unwrap_or("").to_string(),
            other => return Err(bad(other.span())),
        };
        let name = match arg {
            ast::Term::Var(p) if p.quals.is_empty() => p.name.text.clone(),
            ast::Term::Con(p) => p.parts().join("."),
            other => return Err(bad(other.span())),
        };
        match head_last.as_str() {
            "Available" => {
                if !self.sigs.contains_key(&name) {
                    return Err(Error::at(
                        span,
                        format!(
                            "unknown capability `{name}`: a `when` condition names a `signature` of the target profile's capability evidence (platform proofs and SIMD specification section 3)"
                        ),
                    ));
                }
                Ok(WhenCond::Capability(name))
            }
            "arch" => Ok(WhenCond::Arch(name)),
            _ => Err(bad(span)),
        }
    }

    /// Containment validation of an `llvm` fragment (primitive
    /// operations and inline target IR specification section 4):
    /// signature discipline (no extra parameters or results), no escape
    /// (no jumps; calls and symbol references only to `uses` entries and
    /// target intrinsics), no unwind, the declared effect row is the
    /// body's entire permitted effect, and the family grammar (one
    /// straight-line function body of real instructions).
    fn contain_llvm(
        &self,
        p: &ast::PrimDecl,
        b: &ast::PrimBody,
        arg_tys: &[Ty],
        res_ty: &Ty,
        uses: &[String],
        pure: bool,
    ) -> Result<(String, bool)> {
        let mut opaque_calls = false;
        let frag = b.fragment.trim();
        if frag.is_empty() {
            return Err(Error::at(
                b.fragment_span,
                "containment (grammar): the body's fragment is empty (primitive operations and inline target IR specification section 4)",
            ));
        }
        let res_word = self.prim_abi_word(res_ty)?;
        let _ = arg_tys;
        for raw in frag.lines() {
            let line = strip_llvm_comment(raw).trim().to_string();
            if line.is_empty() {
                continue;
            }
            let at = |msg: String| Error::at(b.fragment_span, msg);
            // Module-level entities are not function-body IR: an inline
            // body declares no parameters or results (section 4's
            // signature discipline).
            let head_word = line.split_whitespace().next().unwrap_or("");
            match head_word {
                "define" | "declare" => {
                    return Err(at(format!(
                        "containment (signature discipline): the body declares extra parameters or results (`{head_word}` in an inline fragment; specification section 4)"
                    )))
                }
                "module" | "target" | "source_filename" | "attributes" => {
                    return Err(at(format!(
                        "containment (grammar): `{head_word}` is module-level IR, not one function body (specification section 4)"
                    )))
                }
                _ => {}
            }
            if line.starts_with('@') || (line.starts_with('!') && line.contains(" = ")) {
                return Err(at(
                    "containment (grammar): a fragment is one function body, not a global or metadata definition (specification section 4)"
                        .to_string(),
                ));
            }
            if line.ends_with(':') {
                return Err(at(
                    "containment (no-escape): a body is straight-line; a label introduces a jump target (specification section 4: the body contains no jumps)"
                        .to_string(),
                ));
            }
            // The instruction: an optional result binding, then the
            // opcode (the family grammar's structural face).
            let mut stmt = line.as_str();
            if stmt.starts_with('%') {
                match stmt.find('=') {
                    Some(i) => stmt = stmt[i + 1..].trim_start(),
                    None => {
                        return Err(at(
                            "containment (grammar): a result binding needs `=` (specification section 4)"
                                .to_string(),
                        ))
                    }
                }
            }
            let op = stmt
                .split(|c: char| c.is_whitespace() || c == '(')
                .next()
                .unwrap_or("");
            match op {
                "br" | "switch" | "indirectbr" => {
                    return Err(at(format!(
                        "containment (no-escape): `{op}` is a jump (specification section 4: the body contains no jumps)"
                    )))
                }
                "invoke" | "callbr" | "resume" | "catchswitch" | "catchpad" | "cleanuppad"
                | "cleanupret" | "catchret" | "landingpad" => {
                    return Err(at(format!(
                        "containment (no-unwind): `{op}` may raise or unwind out of the body (specification section 4)"
                    )))
                }
                "unreachable" => {
                    return Err(at(
                        "containment (no fault paths): `unreachable` is a fault path beyond a declared failure mode (specification section 4)"
                            .to_string(),
                    ))
                }
                "ret" => {
                    let word = stmt.split_whitespace().nth(1).unwrap_or("");
                    if word != res_word {
                        return Err(at(format!(
                            "containment (signature discipline): `ret {word}` does not return the declared result shape ({res_word}; specification section 4)"
                        )));
                    }
                }
                _ => {}
            }
            if !LLVM_OPCODES.contains(&op) {
                return Err(at(format!(
                    "containment (grammar): `{op}` is not an LLVM instruction (specification section 4: each fragment parses under its family's grammar)"
                )));
            }
            // Determinism of effects: the declared effect row is the
            // body's entire permitted effect (section 4).
            if pure {
                let effectful = matches!(op, "store" | "atomicrmw" | "cmpxchg" | "fence")
                    || stmt.split_whitespace().any(|w| w == "volatile");
                if effectful {
                    return Err(at(format!(
                        "containment (determinism of effects): `{op}` is an effect the declared pure effect row does not account for (specification section 4)"
                    )));
                }
            }
            // Symbol references and calls (section 4's no-escape): only
            // `uses` entries and target intrinsics are callable, and an
            // indirect call is never allowed.
            let is_call = op == "call";
            let mut saw_symbol = false;
            for sym in llvm_symbols(&line) {
                saw_symbol = true;
                let err = |what: &str, rule: &str| {
                    at(format!(
                        "containment ({rule}): `@{sym}` is {what} (specification section 4)"
                    ))
                };
                if sym.starts_with("llvm.") {
                    continue;
                }
                if uses.iter().any(|u| u == &sym) {
                    // A call through `uses` into opaque code: its
                    // effects are outside containment's reach, so the
                    // body's row is declared under `unsafe` and trusted
                    // (section 12). Visible effects are rejected below.
                    if is_call {
                        opaque_calls = true;
                    }
                    continue;
                }
                if sym.starts_with("cinch_") || sym.starts_with("cinch.") {
                    return Err(err(
                        "a runtime call (task operations and runtime calls are not permitted in a body)",
                        "no-escape",
                    ));
                }
                if ALLOCATION_SYMS.contains(&sym.as_str()) {
                    return Err(err("allocation (a body allocates nothing)", "no allocation"));
                }
                if IO_SYMS.contains(&sym.as_str()) {
                    return Err(err("I/O (a body performs no I/O)", "no I/O"));
                }
                return Err(err("outside the body's `uses` list", "no-escape"));
            }
            if is_call && !line.contains(" asm ") && !saw_symbol {
                return Err(at(
                    "containment (no-escape): a call through a local value is an indirect call (a body calls only its `uses` symbols; specification section 4)"
                        .to_string(),
                ));
            }
            if is_call && line.contains(" asm ") {
                // An inline-asm call is opaque code (section 12).
                opaque_calls = true;
            }
        }
        Ok((
            "containment validated: signature discipline, no-escape, no-unwind, declared effects, llvm grammar (clang assembles the fragment at emission)"
                .to_string(),
            opaque_calls,
        ))
    }

    /// Containment validation of an `asm` fragment (primitive operations
    /// and inline target IR specification section 4): structural only,
    /// no round-trip check. Fault paths, syscalls and runtime references
    /// are the violations this family can see; the fragment is attached
    /// with `llvm.sideeffect` at emission so DCE cannot drop it
    /// (`design/llvm-contract-attributes.md`).
    fn contain_asm(&self, b: &ast::PrimBody, uses: &[String]) -> Result<String> {
        let frag = b.fragment.trim();
        if frag.is_empty() {
            return Err(Error::at(
                b.fragment_span,
                "containment (grammar): the body's fragment is empty (primitive operations and inline target IR specification section 4)",
            ));
        }
        for raw in frag.lines() {
            let at = |msg: String| Error::at(b.fragment_span, msg);
            for w in raw.split(|c: char| c.is_whitespace() || c == ',') {
                let w = w.trim_matches(|c: char| c == ':' || c == '(' || c == ')');
                if matches!(
                    w,
                    "syscall" | "svc" | "int" | "udf" | "brk" | "hlt" | "ud2" | "trap"
                ) {
                    return Err(at(format!(
                        "containment (no fault paths): `{w}` is a fault or syscall beyond a declared failure mode (specification section 4)"
                    )));
                }
                if (w.starts_with("cinch_") || w.starts_with("cinch."))
                    && !uses.iter().any(|u| u == w)
                {
                    return Err(at(format!(
                        "containment (no-escape): `{w}` is a runtime call outside the body's `uses` list (specification section 4)"
                    )));
                }
            }
        }
        Ok("containment validated structurally (no round-trip check); the fragment is emitted as `llvm.sideeffect` inline asm".to_string())
    }

    /// The foreign envelope (foreign function interface specification
    /// section 4): a function type with closed, explicit effect rows,
    /// and for an import a result arrow that accounts for I/O, blocking,
    /// divergence or failure. Returns the parameter count.
    fn check_foreign_envelope(&self, f: &ast::ForeignDecl, ty: &Ty) -> Result<usize> {
        let mut n = 0;
        let mut rows: Vec<&EffRow> = Vec::new();
        let mut cur = ty;
        loop {
            match cur {
                Ty::Fun(_, _, e, b) | Ty::Pi { eff: e, cod: b, .. } => {
                    n += 1;
                    rows.push(e);
                    cur = b;
                }
                _ => break,
            }
        }
        if n == 0 {
            return Err(Error::at(
                f.span,
                "a foreign binding has a function type (`arg -> ... ->{e} res`)",
            ));
        }
        for r in &rows {
            if !matches!(r.tail, crate::types::RowTail::Closed) {
                return Err(Error::at(
                    f.span,
                    "a foreign binding's effect rows are closed and explicit (foreign function interface specification section 4)",
                ));
            }
        }
        let result_row = rows.last().unwrap();
        if f.is_import
            && (result_row.is_pure()
                || !result_row.labels.iter().any(|l| accounted_effect(&l.head)))
        {
            return Err(Error::at(
                f.span,
                format!(
                    "a foreign import must declare its effects: the result arrow's row names at least one of {ACCOUNTED_EFFECTS} (foreign function interface specification section 4: an external function is not pure merely because its C signature has no explicit state parameter)"
                ),
            ));
        }
        Ok(n)
    }

    /// Build the pending foreign binding contracts once every type's
    /// layout is known (the classification reads the recorded ABI table).
    fn build_foreign_contracts(&mut self) -> Result<()> {
        let pending = std::mem::take(&mut self.pending_foreign);
        for (module, f, s, foreign_abi) in pending {
            if !f.is_import {
                // A foreign export wraps an existing Cinch function
                // under a C-callable entry; its type is exactly the
                // target's (foreign function interface specification
                // section 2).
                match self.top_schemes.get(&(module.clone(), f.name.text.clone())) {
                    Some(t) => {
                        if t.ty != s.ty {
                            return Err(Error::at(
                                f.span,
                                format!(
                                    "a foreign export's type is exactly the target function's type: `{}` is declared {}",
                                    f.name.text, t.ty
                                ),
                            ));
                        }
                    }
                    None => {
                        return Err(Error::at(
                            f.name.span,
                            format!(
                                "a foreign export wraps a defined function: declare `{} :: ...` before exporting it",
                                f.name.text
                            ),
                        ))
                    }
                }
            }
            // The contract classifies its parameter types against the
            // recorded layouts in the binding's own scope: the names a
            // foreign signature mentions are the module's own and its
            // imports'.
            self.build_scope(&module, true);
            let (plan, entry) = self.foreign_contract(&module, &f, &s.ty, foreign_abi)?;
            self.module_foreign.push((module, entry));
            self.foreign_plans.push((plan.unit.clone(), plan));
        }
        Ok(())
    }

    /// One foreign binding's contract (foreign function interface
    /// specification sections 2-4): every parameter and result is
    /// classified for the C ABI with its width, signedness, offsets,
    /// alignment and ownership role recorded; the declared effects are
    /// checked for honesty; the marshalling plan and the interface entry
    /// come back together.
    fn foreign_contract(
        &self,
        module: &str,
        f: &ast::ForeignDecl,
        ty: &Ty,
        foreign_abi: ForeignAbi,
    ) -> Result<(crate::core::ForeignPlan, crate::interface::ForeignExport)> {
        use crate::core::{AbiClass, ForeignPlan};
        // Peel the arrow spine: parameters, per-arrow effect rows, result.
        let mut arg_tys: Vec<&Ty> = Vec::new();
        let mut rows: Vec<&EffRow> = Vec::new();
        let mut cur = ty;
        loop {
            match cur {
                Ty::Fun(a, _, e, b) => {
                    arg_tys.push(a);
                    rows.push(e);
                    cur = b;
                }
                Ty::Pi { dom, eff, cod, .. } => {
                    arg_tys.push(dom);
                    rows.push(eff);
                    cur = cod;
                }
                _ => break,
            }
        }
        if arg_tys.is_empty() {
            return Err(Error::at(
                f.span,
                "a foreign binding has a function type (`arg -> ... ->{e} res`)",
            ));
        }
        // Effect honesty (section 4): every declared row is closed and
        // explicit, and an import's result arrow accounts for the
        // behavior an external function is never assumed not to have.
        for r in &rows {
            if !matches!(r.tail, crate::types::RowTail::Closed) {
                return Err(Error::at(
                    f.span,
                    "a foreign binding's effect rows are closed and explicit (foreign function interface specification section 4)",
                ));
            }
        }
        let result_row = rows.last().unwrap();
        if f.is_import
            && (result_row.is_pure()
                || !result_row.labels.iter().any(|l| accounted_effect(&l.head)))
        {
            return Err(Error::at(
                f.span,
                format!(
                    "a foreign import must declare its effects: the result arrow's row names at least one of {ACCOUNTED_EFFECTS} (foreign function interface specification section 4: an external function is not pure merely because its C signature has no explicit state parameter)"
                ),
            ));
        }
        let mut params = Vec::new();
        for a in &arg_tys {
            params.push(self.abi_param(a).map_err(|e| Error::at(f.span, e.message))?);
        }
        let mut result = self
            .abi_param(cur)
            .map_err(|e| Error::at(f.span, e.message))?;
        // The by-value aggregate ABI spelling (the plan note
        // `design/llvm-contract-attributes.md` item 6): `#[byval]`
        // spells aggregate parameters as the platform's by-value copy,
        // `#[sret]` the aggregate result as the platform's result-out
        // buffer. Without them D66's spelling applies: aggregate
        // parameters cross as const-pointers to call-scoped copies and
        // an aggregate result is rejected instead of guessed.
        if foreign_abi.byval {
            if !params.iter().any(|p| matches!(p.class, AbiClass::Aggregate)) {
                return Err(Error::at(
                    f.span,
                    "`#[byval]` spells aggregate parameters crossing by value; this binding marshals no aggregate parameter",
                ));
            }
            for p in &mut params {
                if matches!(p.class, AbiClass::Aggregate) {
                    p.by_value = true;
                }
            }
        }
        if matches!(result.class, AbiClass::Aggregate) {
            if !foreign_abi.sret {
                return Err(Error::at(
                    f.span,
                    format!(
                        "a foreign result cannot be an aggregate: `{}` follows the platform's by-value struct ABI, which this slice does not guess (foreign function interface specification section 2: reject unsupported layouts); return through an `Out` cell or an `Owned` pointer, or spell the by-value crossing `#[sret]`",
                        result.ty
                    ),
                ));
            }
            result.by_value = true;
        } else if foreign_abi.sret {
            return Err(Error::at(
                f.span,
                "`#[sret]` spells an aggregate result crossing by value; this binding's result is not an aggregate",
            ));
        }
        let unit = if f.is_import {
            fn_id(module, &f.name.text).0
        } else {
            fn_id(module, &f.name.text).0
        };
        let effects: Vec<String> = rows
            .iter()
            .flat_map(|r| r.labels.iter().map(|l| l.to_string()))
            .collect();
        let plan = ForeignPlan {
            symbol: f.symbol.clone(),
            export: !f.is_import,
            unit: unit.clone(),
            convention: "ccall".to_string(),
            params: params.clone(),
            result: result.clone(),
            deallocator: String::new(),
            canon: None,
        };
        let entry = crate::interface::ForeignExport {
            symbol: f.name.text.clone(),
            abi: "ccall".to_string(),
            external: f.symbol.clone(),
            ty: render_scheme(&Scheme {
            obligs: Vec::new(),
                params: Vec::new(),
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: Vec::new(),
                region_params: Vec::new(),
                ctx: Vec::new(),
                ty: ty.clone(),
            }),
            effects: effects.join(", "),
            notes: if f.is_import {
                "foreign import".to_string()
            } else {
                "foreign export".to_string()
            },
            kind: if f.is_import { "import" } else { "export" }.to_string(),
            params: params.iter().map(|p| iface_param(p, false)).collect(),
            result: iface_param(&result, true),
            deallocator: String::new(),
        };
        Ok((plan, entry))
    }

    /// A pointer role's pointee contract: the alignment and size the
    /// role asserts of the memory it points at (`None` asserts nothing,
    /// as for `RawPtr`, opaque handles and types without a recorded
    /// layout). The facts feed the LLVM contract surface's `align`,
    /// `dereferenceable` and `!align` claims.
    fn pointee_layout(&self, pointee: &Ty) -> Option<(u64, u64)> {
        match pointee {
            Ty::Prim(s) => Some((s.align(), s.size())),
            Ty::Ptr { pointee, .. } => self.pointee_layout(pointee),
            Ty::Adt(n, _) => {
                let info = self.lookup_type(&[n]).ok()?;
                if info.opaque {
                    return None;
                }
                let layout =
                    self.type_layouts.get(&(info.module.clone(), info.name.clone()))?;
                if layout.repr == "uniform-word" {
                    return None;
                }
                Some((layout.align, layout.size))
            }
            _ => None,
        }
    }

    /// One parameter's LLVM contract facts (the plan note's
    /// usage-derived parameter attributes): `noundef` for scalar words,
    /// and for pointer-shaped parameters the `nonnull`, `align` and
    /// `dereferenceable` the role asserts of its pointee. A `RawPtr`
    /// asserts nothing of its pointee; a borrow asserts its pointee's
    /// validity and alignment (attribute annotations specification
    /// section 4.2: a reference asserts alignment).
    fn param_contract(&self, t: &Ty) -> crate::core::ParamContract {
        let mut c = crate::core::ParamContract::default();
        match t {
            Ty::Prim(_) | Ty::Bool => c.noundef = true,
            Ty::Ptr { role, pointee } => {
                c.nonnull = !matches!(role, crate::types::PtrRole::MaybeNull);
                if !matches!(
                    role,
                    crate::types::PtrRole::Raw | crate::types::PtrRole::Out
                ) {
                    if let Some((a, s)) = self.pointee_layout(pointee) {
                        c.pointee_align = a;
                        c.pointee_size = s;
                    }
                }
            }
            Ty::Ref(_, a) | Ty::Mut(_, a) => {
                c.nonnull = true;
                if let Some((al, sz)) = self.pointee_layout(a) {
                    c.pointee_align = al;
                    c.pointee_size = sz;
                }
            }
            _ => {}
        }
        c
    }

    /// One parameter or result's C ABI class and binding contract
    /// (foreign function interface specification sections 2-3).
    fn abi_param(&self, ty: &Ty) -> std::result::Result<crate::core::AbiParam, Error> {
        use crate::core::{AbiClass, AbiParam, FieldAccess};
        let mut p = AbiParam {
            ty: ty.to_string(),
            class: AbiClass::Unit,
            role: "value".to_string(),
            allocator: String::new(),
            nullable: false,
            width: 0,
            align: 1,
            signed: false,
            pointee_align: 0,
            by_value: false,
            tag: 0,
            fields: Vec::new(),
        };
        match ty {
            Ty::Unit => Ok(p),
            Ty::Prim(s) => {
                p.class = AbiClass::Scalar(*s);
                p.width = s.size();
                p.align = s.align();
                p.signed = s.is_signed();
                Ok(p)
            }
            Ty::Ptr { role, pointee } => {
                p.class = AbiClass::Ptr;
                p.width = 8;
                p.align = 8;
                p.role = match role {
                    crate::types::PtrRole::Raw => "raw".to_string(),
                    crate::types::PtrRole::Borrowed => "borrowed".to_string(),
                    crate::types::PtrRole::Retained => "retained".to_string(),
                    crate::types::PtrRole::Owned(a) => {
                        p.allocator = a.clone();
                        "owned".to_string()
                    }
                    crate::types::PtrRole::Out => "out".to_string(),
                    crate::types::PtrRole::MaybeNull => {
                        p.nullable = true;
                        "maybe-null".to_string()
                    }
                };
                // The role's assertion over its pointee (the LLVM
                // contract surface's `align` and `dereferenceable`):
                // a borrow asserts its pointee's alignment and size, a
                // `RawPtr` asserts nothing ("not claimed to be valid,
                // aligned, or owned"), and an opaque handle is opaque.
                if !matches!(
                    role,
                    crate::types::PtrRole::Raw | crate::types::PtrRole::Out
                ) {
                    if let Some((a, _)) = self.pointee_layout(pointee) {
                        p.pointee_align = a;
                    }
                }
                if matches!(role, crate::types::PtrRole::Out) {
                    // The out-parameter rule (section 4): the cell is one
                    // caller-owned word, zeroed before the call and read
                    // after it; aggregates cross by pointer instead.
                    match pointee.as_ref() {
                        Ty::Prim(_) | Ty::Ptr { .. } => {}
                        Ty::Adt(n, _) if self.lookup_type(&[n]).map(|t| t.opaque).unwrap_or(false) => {}
                        other => {
                            return Err(Error::plain(format!(
                                "an `Out` cell is one word: its pointee is a machine scalar, a pointer role, or an opaque handle (`Out {other}` is an out-parameter over an aggregate in this slice)"
                            )))
                        }
                    }
                }
                Ok(p)
            }
            Ty::Adt(n, _) => {
                let info = self
                    .lookup_type(&[n])
                    .map_err(|_| Error::plain(format!("{ty} has no C representation")))?;
                if info.opaque {
                    // The opaque-handle role (section 3).
                    p.class = AbiClass::Ptr;
                    p.role = "opaque-handle".to_string();
                    p.width = 8;
                    p.align = 8;
                    return Ok(p);
                }
                let Some(layout) = self
                    .type_layouts
                    .get(&(info.module.clone(), info.name.clone()))
                    .cloned()
                else {
                    return Err(Error::plain(format!(
                        "the recorded layout of {ty} is `uniform-word` (no representation attribute), which has no C representation at the boundary (foreign function interface specification section 2: only `#[repr(C)]`, `#[repr(packed)]` and `#[repr(transparent)]` aggregates cross; boxed aggregates and fixed-discriminant data are rejected instead of guessed)"
                    )));
                };
                if layout.repr == "uniform-word" || layout.discriminant.is_some() {
                    return Err(Error::plain(format!(
                        "the recorded layout of {ty} is `{}`, which has no C representation at the boundary (foreign function interface specification section 2: a source-language type name is not an ABI specification; this slice rejects fixed-discriminant data and boxed aggregates instead of guessing)",
                        layout.repr
                    )));
                }
                // `repr(transparent)` passes through as its field
                // (attribute annotations specification section 4.2).
                if layout.repr == "transparent" {
                    let v = self
                        .qualified
                        .get(&(info.module.clone(), info.name.clone()))
                        .ok_or_else(|| Error::plain(format!("{ty} has no constructor")))?;
                    let inner = match &v.kind {
                        ValueKind::Ctor { field_tys, .. } if field_tys.len() == 1 => {
                            field_tys[0].clone()
                        }
                        _ => {
                            return Err(Error::plain(format!(
                                "{ty} is `repr(transparent)` over something other than one fixed field"
                            )))
                        }
                    };
                    let mut inner_p = self.abi_param(&inner)?;
                    inner_p.ty = ty.to_string();
                    return Ok(inner_p);
                }
                if layout.fields.is_empty() {
                    return Err(Error::plain(format!(
                        "the recorded layout of {ty} has no fields to marshal"
                    )));
                }
                // An aggregate parameter crosses by const-pointer to a
                // call-scoped C-layout copy at the recorded offsets.
                let v = self
                    .qualified
                    .get(&(info.module.clone(), info.name.clone()))
                    .ok_or_else(|| Error::plain(format!("{ty} has no constructor")))?;
                let (field_tys, field_erased, tag) = match &v.kind {
                    ValueKind::Ctor { field_tys, field_erased, tag, .. } => {
                        (field_tys.clone(), field_erased.clone(), *tag)
                    }
                    _ => return Err(Error::plain(format!("{ty} has no constructor"))),
                };
                p.class = AbiClass::Aggregate;
                p.width = layout.size;
                p.align = layout.align;
                p.tag = tag as u64;
                let mut slot = 0;
                for (fty, er) in field_tys.iter().zip(field_erased.iter()) {
                    if *er {
                        continue;
                    }
                    let f = layout.fields.get(slot).ok_or_else(|| {
                        Error::plain(format!("internal error: no layout fact for {ty} field {slot}"))
                    })?;
                    p.fields.push(FieldAccess {
                        offset: f.offset,
                        size: f.size,
                        shape: self.field_shape(fty, f.size),
                        range: fty.value_range(),
                    });
                    slot += 1;
                }
                Ok(p)
            }
            other => Err(Error::plain(format!(
                "{other} has no C representation: language-native closures, dictionaries, vectors, dependent pairs and proofs have no implicit C representation (foreign function interface specification section 2)"
            ))),
        }
    }

    /// Ownership discipline at the boundary (foreign function interface
    /// specification section 4): every owned foreign result is released
    /// by its matching deallocator, the foreign import consuming the
    /// same allocator identity. The pairing is recorded in the contract.
    fn check_deallocators(&mut self) -> Result<()> {
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (_, plan) in &self.foreign_plans {
            if plan.result.role == "owned" && !plan.result.allocator.is_empty() {
                let alloc = plan.result.allocator.clone();
                let symbol = plan.symbol.clone();
                let deallocator = self
                    .foreign_plans
                    .iter()
                    .find(|(_, q)| {
                        !q.export
                            && q.params
                                .iter()
                                .any(|p| p.role == "owned" && p.allocator == alloc)
                    })
                    .map(|(_, q)| q.symbol.clone());
                match deallocator {
                    Some(d) => pairs.push((symbol, d)),
                    None => {
                        return Err(Error::plain(format!(
                            "an owned foreign result needs its matching deallocator: `{symbol}` allocates from `{alloc}`, but no foreign import consumes `Owned {alloc} t` (foreign function interface specification section 4: a foreign allocation is released by its matching deallocator)"
                        )))
                    }
                }
            }
        }
        for (symbol, d) in pairs {
            for (_, plan) in &mut self.foreign_plans {
                if plan.symbol == symbol && plan.result.role == "owned" {
                    plan.deallocator = d.clone();
                }
            }
            for (_, e) in &mut self.module_foreign {
                if e.external == symbol && e.kind == "import" {
                    e.deallocator = d.clone();
                }
            }
        }
        Ok(())
    }

    /// The boundary ABI maps for [`Program`]: constructor layout plans
    /// for `repr(C)`/`repr(packed)`/`repr(transparent)` types (attribute
    /// annotations specification section 4.3) and the foreign binding
    /// contracts (foreign function interface specification section 2).
    fn build_abi_maps(
        &self,
    ) -> (
        std::collections::BTreeMap<String, crate::core::CtorLayout>,
        std::collections::BTreeMap<String, crate::core::ForeignPlan>,
    ) {
        use crate::core::{CtorLayout, FieldAccess};
        let mut plans = std::collections::BTreeMap::new();
        let mut type_ks: Vec<&TypeKind> = self.types.values().flatten().collect();
        type_ks.sort_by(|a, b| (a.module.as_str(), a.name.as_str()).cmp(&(b.module.as_str(), b.name.as_str())));
        for t in type_ks {
            let Some(layout) = self
                .type_layouts
                .get(&(t.module.clone(), t.name.clone()))
                .cloned()
            else {
                continue;
            };
            if layout.repr == "uniform-word" || layout.fields.is_empty() {
                continue;
            }
            for (cname, tag) in &t.ctors {
                let _ = tag;
                let Some(v) = self.qualified.get(&(t.module.clone(), cname.clone())) else {
                    continue;
                };
                let (field_tys, field_erased) = match &v.kind {
                    ValueKind::Ctor { field_tys, field_erased, .. } => {
                        (field_tys.clone(), field_erased.clone())
                    }
                    _ => continue,
                };
                let mut fields = Vec::new();
                let mut slot = 0;
                for (fty, er) in field_tys.iter().zip(field_erased.iter()) {
                    if *er {
                        continue;
                    }
                    let Some(f) = layout.fields.get(slot) else {
                        fields.clear();
                        break;
                    };
                    fields.push(FieldAccess {
                        offset: f.offset,
                        size: f.size,
                        shape: self.field_shape(fty, f.size),
                        range: fty.value_range(),
                    });
                    slot += 1;
                }
                if fields.is_empty() && !layout.fields.is_empty() {
                    continue;
                }
                plans.insert(
                    fn_id(&t.module, cname).0,
                    CtorLayout { payload: layout.size, fields },
                );
            }
        }
        let foreigns = self.foreign_plans.iter().cloned().collect();
        (plans, foreigns)
    }
}

fn wrap_eq_body(e: &ast::Equation) -> ast::Term {
    if e.locals.is_empty() {
        e.body.clone()
    } else {
        ast::Term::Let(e.locals.clone(), Box::new(e.body.clone()), e.span)
    }
}

// --------------------------------------- attribute annotations (P17)

use crate::core::InlineHint;

/// Where one `#[...]` list sits (attribute annotations specification
/// section 1.1); the position decides which families it may carry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AttrSite {
    /// A `data`, `record` or `codata` declaration.
    TypeDecl,
    /// A function declaration: a value signature or a top-level equation.
    FnDecl,
    /// A `foreign` import or export declaration.
    ForeignDecl,
    /// Any other declaration.
    Decl,
    /// A data constructor declaration.
    Ctor,
    /// A record field (its declaration or its type).
    Field,
    /// A function clause.
    Clause,
    /// A `case` alternative.
    Alt,
    /// A guard, after `|`.
    Guard,
    /// The `if` condition position (the pair form of section 2.1).
    IfCond,
    /// An `if` branch position.
    IfBranch,
}

impl AttrSite {
    /// The named placement rejection of section 1.1.
    fn placement_error(self, family: &str) -> String {
        match family {
            "weight" | "likely" | "unlikely" => match self {
                AttrSite::FnDecl => format!(
                    "`#[{family}]` annotates one alternative of a decision point (attribute annotations specification sections 2 and 3); a function declaration carries `#[inline]`, `#[noinline]` and `#[cold]` (section 5), and a weight on a clause is written between the name and its patterns"
                ),
                _ => format!(
                    "`#[{family}]` annotates a `case` alternative, a function clause, a guard, or an `if` branch (attribute annotations specification sections 2 and 3)"
                ),
            },
            "cold" => "`#[cold]` is a weight wrapper on an alternative or a hint on a function declaration (attribute annotations specification sections 3 and 5); this position takes neither".to_string(),
            "inline" => {
                "`#[inline]` is a hint on a function declaration (attribute annotations specification section 5)".to_string()
            }
            "safety" => "`#[recursion]`, `#[stack]`, `#[no-alloc]`, `#[alloc]`, `#[max-complexity]`, `#[max-lines]`, `#[assert-min]`, `#[checked-params]`, `#[no-macros]`, `#[no-raw-ptr]` and `#[deny-warnings]` attach to a function definition (a signature or a top-level equation; attribute annotations specification section 10)".to_string(),
            "byval" | "sret" => "`#[byval]` and `#[sret]` attach to a `foreign` import or export declaration (the by-value aggregate ABI spelling of design/llvm-contract-attributes.md)".to_string(),
            "repr" => match self {
                AttrSite::FnDecl | AttrSite::Decl | AttrSite::Ctor => {
                    "`#[repr(...)]` attaches to a `data` or `record` declaration (attribute annotations specification section 4.1)".to_string()
                }
                _ => "`#[repr(...)]` attaches to a `data` or `record` declaration or to a field of a `repr(C)` record (attribute annotations specification section 4.1)".to_string(),
            },
            other => format!("`#[{other}]` is not admitted at this position (attribute annotations specification section 1.1)"),
        }
    }
}

/// The by-value aggregate ABI spelling one foreign declaration selects
/// (`#[byval]` and `#[sret]`: design/llvm-contract-attributes.md item 6,
/// the native spelling of the const-pointer-in/results-out aggregate
/// ABI).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ForeignAbi {
    /// `#[byval]`: the aggregate parameters cross by value.
    pub byval: bool,
    /// `#[sret]`: the aggregate result crosses by value.
    pub sret: bool,
}

/// What one attachment position's attribute list carries (attribute
/// annotations specification sections 2, 3, 4, 5 and 10).
#[derive(Clone, Debug, Default)]
pub(crate) struct AttrSet {
    /// The one weight-family value (the wrappers ARE weights).
    pub weight: Option<u32>,
    pub inline: Option<InlineHint>,
    /// `#[cold]` at a function declaration: the function-level hint of
    /// section 5.
    pub fn_cold: bool,
    pub repr: Option<ReprSpec>,
    /// The P20 safety assertions (attribute annotations specification
    /// section 10), verified at the definition.
    pub safety: SafetySet,
    /// The by-value aggregate ABI spelling at a foreign declaration.
    pub foreign_abi: ForeignAbi,
}

/// The raw argument text of one attribute: the parenthesized list, or
/// the `= literal` tail (attribute annotations specification section 1).
fn arg_text(args: &str) -> &str {
    let t = args.trim();
    t.strip_prefix('=').unwrap_or(t).trim()
}

/// The weight a weight-family attribute assigns (attribute annotations
/// specification sections 2 and 3: the wrappers are exactly the manual
/// weights 2000, 1 and 0).
fn weight_value(a: &crate::ast::Attribute) -> Result<u32> {
    let name = a.name();
    let raw = arg_text(a.args.as_deref().unwrap_or(""));
    let constant = match name.as_str() {
        "likely" => Some(2000),
        "unlikely" => Some(1),
        "cold" => Some(0),
        _ => None,
    };
    if let Some(w) = constant {
        if !raw.is_empty() {
            return Err(Error::at(
                a.span,
                format!(
                    "`#[{name}]` takes no arguments (attribute annotations specification section 3)"
                ),
            ));
        }
        return Ok(w);
    }
    // `#[weight(n)]`: one non-negative integer literal representable in
    // 32 bits (section 2).
    if raw.is_empty() || !raw.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::at(
            a.span,
            "`#[weight(n)]` takes one non-negative integer literal (attribute annotations specification section 2)",
        ));
    }
    raw.parse::<u32>().map_err(|_| {
        Error::at(
            a.span,
            "a weight is a non-negative integer literal representable in 32 bits (attribute annotations specification section 2)",
        )
    })
}

/// The parsed `#[repr(...)]` argument list (attribute annotations
/// specification section 4.1).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ReprSpec {
    /// `repr(C)`: the platform C layout algorithm over source-order
    /// fields.
    pub c_layout: bool,
    /// `repr(transparent)`: exactly the single field's representation.
    pub transparent: bool,
    /// `repr(align(N))`: the type's alignment floor (section 4.2: the
    /// attribute only raises).
    pub align: Option<u64>,
    /// `repr(packed)` or `repr(packed(N))`: the field alignment cap
    /// (1 for bare `packed`).
    pub packed: Option<u64>,
    /// `repr(u8|u16|u32|u64)`: the constructor discriminant's width in
    /// bytes.
    pub discriminant: Option<u64>,
    /// The representations as written, for the ABI table's `repr` fact.
    pub spelled: String,
}

impl ReprSpec {
    /// Does this representation lay out fields?
    fn has_fields(&self) -> bool {
        self.c_layout || self.transparent || self.align.is_some() || self.packed.is_some()
    }

    /// The field-level forms: `align(N)` and `packed(N)` (the section 9
    /// open decision admits them on fields of `repr(C)` records).
    fn field_only(&self) -> bool {
        !self.c_layout && !self.transparent && self.discriminant.is_none()
    }
}

/// Split a raw argument list at top-level commas (attribute annotations
/// specification section 1: `literal, { ",", literal }`).
fn split_top_commas(raw: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (i, c) in raw.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                out.push(&raw[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&raw[start..]);
    out
}

/// Parse one `#[repr(...)]` argument list (attribute annotations
/// specification sections 1 and 4.1).
fn parse_repr_args(a: &crate::ast::Attribute) -> Result<ReprSpec> {
    let raw = a.args.as_deref().unwrap_or("");
    if raw.trim_start().starts_with('=') {
        return Err(Error::at(
            a.span,
            "`#[repr(...)]` takes its representations in parentheses (attribute annotations specification section 1)",
        ));
    }
    if raw.trim().is_empty() {
        return Err(Error::at(
            a.span,
            "`#[repr(...)]` takes at least one representation (attribute annotations specification section 4.1)",
        ));
    }
    let mut spec = ReprSpec { spelled: raw.trim().to_string(), ..Default::default() };
    for item in split_top_commas(raw) {
        let t = item.trim();
        if t.is_empty() {
            return Err(Error::at(
                a.span,
                "an empty representation item (attribute annotations specification section 4.1)",
            ));
        }
        let mut ident_end = 0;
        for (i, c) in t.char_indices() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident_end = i + c.len_utf8();
            } else {
                break;
            }
        }
        let (ident, tail) = t.split_at(ident_end);
        let tail = tail.trim();
        let mut number = None;
        if !tail.is_empty() {
            let n = tail
                .strip_prefix('(')
                .and_then(|x| x.strip_suffix(')'))
                .map(str::trim)
                .filter(|x| !x.is_empty() && x.bytes().all(|b| b.is_ascii_digit()))
                .and_then(|x| x.parse::<u64>().ok());
            match n {
                Some(n) => number = Some(n),
                None => return Err(Error::at(a.span, repr_item_error(t))),
            }
        }
        let mut once = |set: bool| -> Result<()> {
            if set {
                Err(Error::at(
                    a.span,
                    format!(
                        "`{t}` is written twice in one `#[repr(...)]` (attribute annotations specification section 1.3)"
                    ),
                ))
            } else {
                Ok(())
            }
        };
        match (ident, number) {
            ("C", None) => {
                once(spec.c_layout)?;
                spec.c_layout = true;
            }
            ("transparent", None) => {
                once(spec.transparent)?;
                spec.transparent = true;
            }
            ("align", Some(n)) => {
                once(spec.align.is_some())?;
                if n == 0 || !n.is_power_of_two() {
                    return Err(Error::at(
                        a.span,
                        "`repr(align(N))` takes a non-zero power of two (attribute annotations specification section 4.1)",
                    ));
                }
                spec.align = Some(n);
            }
            ("packed", None) => {
                once(spec.packed.is_some())?;
                spec.packed = Some(1);
            }
            ("packed", Some(n)) => {
                once(spec.packed.is_some())?;
                if n == 0 || !n.is_power_of_two() {
                    return Err(Error::at(
                        a.span,
                        "`repr(packed(N))` takes a non-zero power of two (attribute annotations specification section 4.1)",
                    ));
                }
                spec.packed = Some(n);
            }
            ("u8", None) => {
                once(spec.discriminant.is_some())?;
                spec.discriminant = Some(1);
            }
            ("u16", None) => {
                once(spec.discriminant.is_some())?;
                spec.discriminant = Some(2);
            }
            ("u32", None) => {
                once(spec.discriminant.is_some())?;
                spec.discriminant = Some(4);
            }
            ("u64", None) => {
                once(spec.discriminant.is_some())?;
                spec.discriminant = Some(8);
            }
            _ => return Err(Error::at(a.span, repr_item_error(t))),
        }
    }
    // Combination rules (sections 4.1 and 4.2): `transparent` is exactly
    // one field's layout, `align` only raises, and a fixed discriminant
    // is its own representation.
    if spec.transparent
        && (spec.c_layout || spec.align.is_some() || spec.packed.is_some() || spec.discriminant.is_some())
    {
        return Err(Error::at(
            a.span,
            "`repr(transparent)` is exactly its field's representation and combines with no other representation (attribute annotations specification section 4.1)",
        ));
    }
    if spec.packed.is_some() && spec.align.is_some() {
        return Err(Error::at(
            a.span,
            "`repr(packed)` removes padding and `repr(align(N))` raises alignment; they conflict (attribute annotations specification section 4.1)",
        ));
    }
    if spec.discriminant.is_some()
        && (spec.c_layout || spec.align.is_some() || spec.packed.is_some())
    {
        return Err(Error::at(
            a.span,
            "a fixed discriminant representation combines with no other representation here (attribute annotations specification section 4.1)",
        ));
    }
    Ok(spec)
}

fn repr_item_error(t: &str) -> String {
    format!(
        "`{t}` is not a representation (attribute annotations specification section 4.1 expects `C`, `transparent`, `align(N)`, `packed`, `packed(N)`, or `u8`/`u16`/`u32`/`u64`)"
    )
}

// ==================== P20: the safety assertion vocabulary ====================
//
// Attribute annotations specification section 10;
// design/safety-assertion-attributes.md; decisions D111-D114. Every
// attribute is compiler-owned and verified at the definition: a
// violation is a compile error naming the attribute, the asserted rule,
// and the offending site. A satisfied assertion is a contract fact
// (recorded in `.cinchif`, hashed into `iface_hash` exactly like `repr`
// facts).

/// The `bounded(x)` argument: a natural number literal (the depth bound
/// as a number), or the name of the argument or size index the bound is
/// measured by (the `Nat`/`Fin` bounded-counter pattern of
/// design/safety-assertion-attributes.md rule 2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum BoundArg {
    Num(u64),
    Name(String),
}

impl std::fmt::Display for BoundArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundArg::Num(n) => write!(f, "{n}"),
            BoundArg::Name(n) => write!(f, "{n}"),
        }
    }
}

/// `#[recursion "none"|"tail"|"bounded(n)"]` (section 10).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RecursionSpec {
    /// No recursive calls (self or mutual).
    None,
    /// All recursive calls are tail calls (constant stack).
    Tail,
    /// Call depth statically at most the bound.
    Bounded(BoundArg),
}

/// `#[stack "constant"|"bounded(n)"]` (section 10).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StackSpec {
    /// Tail or non-recursive.
    Constant,
    /// Stack usage bounded by the bound.
    Bounded(BoundArg),
}

/// The safety assertions one attachment position carries (section 10),
/// each with the span naming the assertion in violations.
#[derive(Clone, Debug, Default)]
pub(crate) struct SafetySet {
    pub recursion: Option<(RecursionSpec, ast::Span)>,
    pub stack: Option<(StackSpec, ast::Span)>,
    pub no_alloc: Option<ast::Span>,
    pub alloc_once: Option<ast::Span>,
    pub max_complexity: Option<(u64, ast::Span)>,
    pub max_lines: Option<(u64, ast::Span)>,
    pub assert_min: Option<(u64, ast::Span)>,
    pub checked_params: Option<ast::Span>,
    pub no_macros: Option<ast::Span>,
    pub no_raw_ptr: Option<ast::Span>,
    pub deny_warnings: Option<ast::Span>,
}

impl SafetySet {
    /// Validate one safety assertion's arguments and record it (attribute
    /// annotations specification section 10).
    fn fill_from(&mut self, a: &crate::ast::Attribute) -> Result<()> {
        let span = a.span;
        match a.name().as_str() {
            "recursion" => {
                self.recursion = Some((parse_recursion_args(a)?, span));
            }
            "stack" => {
                self.stack = Some((parse_stack_args(a)?, span));
            }
            "no-alloc" => {
                no_arg(a)?;
                self.no_alloc = Some(span);
            }
            "alloc" => {
                if quoted_arg(a)? != "once" {
                    return Err(Error::at(
                        span,
                        "`#[alloc]` takes `\"once\"` (the initialization pattern of attribute annotations specification section 10)",
                    ));
                }
                self.alloc_once = Some(span);
            }
            "max-complexity" => {
                self.max_complexity = Some((uint_arg(a)?, span));
            }
            "max-lines" => {
                self.max_lines = Some((uint_arg(a)?, span));
            }
            "assert-min" => {
                self.assert_min = Some((uint_arg(a)?, span));
            }
            "checked-params" => {
                no_arg(a)?;
                self.checked_params = Some(span);
            }
            "no-macros" => {
                no_arg(a)?;
                self.no_macros = Some(span);
            }
            "no-raw-ptr" => {
                no_arg(a)?;
                self.no_raw_ptr = Some(span);
            }
            "deny-warnings" => {
                no_arg(a)?;
                self.deny_warnings = Some(span);
            }
            other => {
                return Err(Error::at(
                    span,
                    format!("internal error: `{other}` is not a safety assertion"),
                ))
            }
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.recursion.is_none()
            && self.stack.is_none()
            && self.no_alloc.is_none()
            && self.alloc_once.is_none()
            && self.max_complexity.is_none()
            && self.max_lines.is_none()
            && self.assert_min.is_none()
            && self.checked_params.is_none()
            && self.no_macros.is_none()
            && self.no_raw_ptr.is_none()
            && self.deny_warnings.is_none()
    }

    /// Merge a second attachment position's assertions onto the
    /// definition (a signature and its equations share the definition):
    /// one definition carries one of each assertion (section 1.3's
    /// duplicate rule).
    fn merge(&mut self, other: &SafetySet, name: &str) -> Result<()> {
        macro_rules! take1 {
            ($field:ident, $spelling:expr) => {
                if let Some(v) = other.$field.clone() {
                    if self.$field.is_some() {
                        return Err(Error::at(
                            v.1,
                            format!(
                                "{name} carries two `{}` assertions: one definition carries one (attribute annotations specification sections 1.3 and 10)",
                                $spelling
                            ),
                        ));
                    }
                    self.$field = Some(v);
                }
            };
        }
        macro_rules! take0 {
            ($field:ident, $spelling:expr) => {
                if let Some(s) = other.$field {
                    if self.$field.is_some() {
                        return Err(Error::at(
                            s,
                            format!(
                                "{name} carries two `{}` assertions: one definition carries one (attribute annotations specification sections 1.3 and 10)",
                                $spelling
                            ),
                        ));
                    }
                    self.$field = Some(s);
                }
            };
        }
        take1!(recursion, "#[recursion]");
        take1!(stack, "#[stack]");
        take0!(no_alloc, "#[no-alloc]");
        take0!(alloc_once, "#[alloc]");
        take1!(max_complexity, "#[max-complexity]");
        take1!(max_lines, "#[max-lines]");
        take1!(assert_min, "#[assert-min]");
        take0!(checked_params, "#[checked-params]");
        take0!(no_macros, "#[no-macros]");
        take0!(no_raw_ptr, "#[no-raw-ptr]");
        take0!(deny_warnings, "#[deny-warnings]");
        Ok(())
    }

    /// The contract facts the satisfied assertions record (decision
    /// D112), in a normalized spelling.
    fn fact_strings(&self) -> Vec<String> {
        let mut out = Vec::new();
        if let Some((r, _)) = &self.recursion {
            out.push(match r {
                RecursionSpec::None => "recursion=none".to_string(),
                RecursionSpec::Tail => "recursion=tail".to_string(),
                RecursionSpec::Bounded(b) => format!("recursion=bounded({b})"),
            });
        }
        if let Some((s, _)) = &self.stack {
            out.push(match s {
                StackSpec::Constant => "stack=constant".to_string(),
                StackSpec::Bounded(b) => format!("stack=bounded({b})"),
            });
        }
        if self.no_alloc.is_some() {
            out.push("no-alloc".to_string());
        }
        if self.alloc_once.is_some() {
            out.push("alloc=once".to_string());
        }
        if let Some((n, _)) = self.max_complexity {
            out.push(format!("complexity<={n}"));
        }
        if let Some((n, _)) = self.max_lines {
            out.push(format!("lines<={n}"));
        }
        if let Some((n, _)) = self.assert_min {
            out.push(format!("assert-min={n}"));
        }
        if self.checked_params.is_some() {
            out.push("checked-params".to_string());
        }
        if self.no_macros.is_some() {
            out.push("no-macros".to_string());
        }
        if self.no_raw_ptr.is_some() {
            out.push("no-raw-ptr".to_string());
        }
        if self.deny_warnings.is_some() {
            out.push("deny-warnings".to_string());
        }
        out
    }
}

/// Module-scope safety state (section 10): `#[profile "jpl"]` and
/// `#[deny-warnings]` on the module header.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ModuleSafety {
    pub profile_jpl: bool,
    pub deny_warnings: bool,
}

/// The unquoted argument of a string-argument attribute (`"tail"`), or
/// the raw `= literal` tail (attribute annotations specification
/// section 1).
fn quoted_arg(a: &crate::ast::Attribute) -> Result<String> {
    let raw = arg_text(a.args.as_deref().unwrap_or(""));
    let Some(inner) = raw.strip_prefix('"').and_then(|r| r.strip_suffix('"')) else {
        return Err(Error::at(
            a.span,
            format!(
                "`#[{}]` takes one string argument in quotes (attribute annotations specification sections 1 and 10)",
                a.name()
            ),
        ));
    };
    Ok(inner.to_string())
}

/// The `bounded(x)` argument: a natural number literal or a name.
fn bound_arg(raw: &str, span: ast::Span) -> Result<BoundArg> {
    let inner = raw
        .strip_prefix("bounded(")
        .and_then(|r| r.strip_suffix(')'))
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .ok_or_else(|| {
            Error::at(
                span,
                format!(
                    "`{raw}` is not a depth bound: `bounded(n)` takes a natural number or the name of the argument or size index the depth is bounded by (attribute annotations specification section 10)"
                ),
            )
        })?;
    if inner.bytes().all(|b| b.is_ascii_digit()) {
        return Ok(BoundArg::Num(inner.parse::<u64>().map_err(|_| {
            Error::at(span, "a depth bound is a natural number representable in 64 bits")
        })?));
    }
    if !inner.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
        || inner.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(true)
    {
        return Err(Error::at(
            span,
            format!(
                "`{inner}` is not a depth bound name (attribute annotations specification section 10: `bounded(n)` names an argument or a size index)"
            ),
        ));
    }
    Ok(BoundArg::Name(inner.to_string()))
}

/// Parse `#[recursion "none"|"tail"|"bounded(n)"]`.
fn parse_recursion_args(a: &crate::ast::Attribute) -> Result<RecursionSpec> {
    let s = quoted_arg(a)?;
    match s.as_str() {
        "none" => Ok(RecursionSpec::None),
        "tail" => Ok(RecursionSpec::Tail),
        _ if s.starts_with("bounded(") => Ok(RecursionSpec::Bounded(bound_arg(&s, a.span)?)),
        _ => Err(Error::at(
            a.span,
            format!(
                "`{s}` is not a recursion assertion: `#[recursion]` takes `\"none\"`, `\"tail\"` or `\"bounded(n)\"` (attribute annotations specification section 10)"
            ),
        )),
    }
}

/// Parse `#[stack "constant"|"bounded(n)"]`.
fn parse_stack_args(a: &crate::ast::Attribute) -> Result<StackSpec> {
    let s = quoted_arg(a)?;
    match s.as_str() {
        "constant" => Ok(StackSpec::Constant),
        _ if s.starts_with("bounded(") => Ok(StackSpec::Bounded(bound_arg(&s, a.span)?)),
        _ => Err(Error::at(
            a.span,
            format!(
                "`{s}` is not a stack assertion: `#[stack]` takes `\"constant\"` or `\"bounded(n)\"` (attribute annotations specification section 10)"
            ),
        )),
    }
}

/// Parse a bare natural number argument (`#[max-complexity 10]`).
fn uint_arg(a: &crate::ast::Attribute) -> Result<u64> {
    let raw = arg_text(a.args.as_deref().unwrap_or(""));
    if raw.is_empty() || !raw.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::at(
            a.span,
            format!(
                "`#[{}]` takes one natural number literal (attribute annotations specification section 10)",
                a.name()
            ),
        ));
    }
    raw.parse::<u64>().map_err(|_| {
        Error::at(a.span, "a bound is a natural number representable in 64 bits")
    })
}

/// A no-argument assertion: reject any argument.
fn no_arg(a: &crate::ast::Attribute) -> Result<()> {
    if !arg_text(a.args.as_deref().unwrap_or("")).is_empty() {
        return Err(Error::at(
            a.span,
            format!(
                "`#[{}]` takes no arguments (attribute annotations specification section 10)",
                a.name()
            ),
        ));
    }
    Ok(())
}

/// Validate a module header's safety attributes (attribute annotations
/// specification section 10): `#[profile "jpl"]` and `#[deny-warnings]`
/// (the `warn # error` pragma's module scope).
fn validate_module_attrs(attrs: &[crate::ast::Attribute]) -> Result<ModuleSafety> {
    let mut out = ModuleSafety::default();
    for a in attrs {
        match a.name().as_str() {
            "meta" => {}
            "profile" => {
                if quoted_arg(a)? != "jpl" {
                    return Err(Error::at(
                        a.span,
                        "`#[profile]` takes `\"jpl\"` (the JPL rule profile of attribute annotations specification section 10)",
                    ));
                }
                if out.profile_jpl {
                    return Err(Error::at(
                        a.span,
                        "one module carries one `#[profile \"jpl\"]` attribute (attribute annotations specification section 1.3)",
                    ));
                }
                out.profile_jpl = true;
            }
            "deny-warnings" => {
                no_arg(a)?;
                if out.deny_warnings {
                    return Err(Error::at(
                        a.span,
                        "one module carries one `#[deny-warnings]` attribute (attribute annotations specification section 1.3)",
                    ));
                }
                out.deny_warnings = true;
            }
            other => {
                return Err(Error::at(
                    a.span,
                    format!(
                        "unknown module attribute `{other}`: a module header carries `#[profile \"jpl\"]`, `#[deny-warnings]` and `#[meta(...)]` (attribute annotations specification section 10)"
                    ),
                ))
            }
        }
    }
    Ok(out)
}

// ------------------------------ the body facts walker ------------------------------

/// One named call site (for naming the offending call of a tail
/// violation).
#[derive(Clone, Debug)]
pub(crate) struct CallRec {
    pub callee: String,
    pub span: ast::Span,
    /// In tail position (the D104 walk).
    pub tail: bool,
    /// Under a hoisted closure (a lambda, a thunk, a `do` continuation):
    /// the call runs in the closure's own unit.
    pub closure: bool,
}

/// The shape of one argument at a self-recursive call site.
#[derive(Clone, Debug)]
pub(crate) enum ArgShape {
    /// A value of this parameter's subcomponent tree.
    Sub { strict: bool },
    /// `param - k`: a well-founded measure step of the parameter's
    /// value (core specification section 8).
    Measure,
    /// A numeric literal (the conservative call-graph ladder).
    Literal(i128),
    /// A dictionary or index placeholder.
    Hole,
    /// Anything else.
    Other,
}

/// One self-recursive call site with its argument shapes (the
/// `bounded(n)` structural proof reads these).
#[derive(Clone, Debug)]
pub(crate) struct SelfCallRec {
    pub span: ast::Span,
    pub tail: bool,
    pub closure: bool,
    pub args: Vec<ArgShape>,
    /// The equation the site sits in (the call-graph ladder walks the
    /// equations).
    pub eq: usize,
}

/// One allocation construction site (attribute annotations
/// specification section 10: cons, closures, thunks, `Mut`, strings,
/// `Bytes`, foreign aggregates). An allocation violation names it.
#[derive(Clone, Debug)]
pub(crate) struct AllocRec {
    pub what: String,
    pub span: ast::Span,
    /// Under a hoisted closure: the site runs whenever the closure runs,
    /// not at most once per enclosing call.
    pub closure: bool,
}

/// What one definition's body contains, collected from the surface
/// syntax with spans so violations name the offending site.
#[derive(Clone, Debug, Default)]
pub(crate) struct BodyFacts {
    /// The body's source line span (its syntax's first to last line).
    pub body_lines: u64,
    /// Decision branches over the body (cyclomatic complexity is
    /// `branches + 1`): one per `if`, one per `case` alternative beyond
    /// the first, one per guard.
    pub branches: u64,
    /// Runtime `decide` check sites, recorded while the body checks
    /// (attribute annotations specification section 10: proof
    /// obligations do not count).
    pub runtime_decides: Vec<ast::Span>,
    pub alloc_sites: Vec<AllocRec>,
    pub calls: Vec<CallRec>,
    pub self_calls: Vec<SelfCallRec>,
    /// Self references in value position (a recursive use that is not a
    /// static call).
    pub self_refs: Vec<ast::Span>,
    /// Structured-concurrency sites in the body (concurrency
    /// specification section 2): `withScope`, `spawn` and
    /// `joinAllOrCancel` call sites. The temporal liveness claims read
    /// them as their subject (core specification section 10).
    pub conc_sites: Vec<(String, ast::Span)>,
    /// Macro expansion provenance inside the definition (checked from
    /// P9's expansion provenance).
    pub macro_site: Option<ast::Span>,
    /// Foreign pointer operations in the body (the foreign name).
    pub ptr_ops: Vec<(String, ast::Span)>,
    /// Parameters used before any validating check (the
    /// `checked-params` rule): `(parameter, the offending use)`.
    pub unchecked_params: Vec<(String, ast::Span)>,
    /// The lowest and highest source line the body's syntax spans.
    pub min_line: u32,
    pub max_line: u32,
}

/// The foreign bindings a body's calls may name (attribute annotations
/// specification section 10's `#[no-raw-ptr]` and the foreign aggregate
/// allocation sites).
#[derive(Clone, Debug, Default)]
pub(crate) struct ForeignMarks {
    /// Foreign names whose binding contract carries a pointer.
    pub ptrs: HashSet<String>,
    /// Foreign names whose binding contract marshals an aggregate.
    pub aggregates: HashSet<String>,
}

/// Is this pattern refutable (a check that can fail)?
fn pat_refutable(p: &ast::Pat) -> bool {
    match p {
        ast::Pat::Var(..) | ast::Pat::Wild(_) | ast::Pat::Tuple(..) => false,
        ast::Pat::As(_, inner, _) => pat_refutable(inner),
        _ => true,
    }
}

/// The variable a pattern binds directly (its `x` in `x@p` or its bare
/// variable), if any.
fn pat_direct(p: &ast::Pat) -> Option<&ast::Name> {
    match p {
        ast::Pat::Var(n, _) => Some(n),
        ast::Pat::As(n, inner, _) => pat_direct(inner).or(Some(n)),
        _ => None,
    }
}

struct FactWalk<'a> {
    self_mod: &'a str,
    self_name: &'a str,
    /// The equation's argument positions: the name bound directly to
    /// each argument (empty when the head pattern destructures it).
    arg_names: Vec<Option<String>>,
    /// Per tracked position: `(validated, reported)`.
    param_state: Vec<(bool, bool)>,
    /// The equation's head-pattern binders: `(name, "#pos")`, the roots
    /// of the argument positions' subcomponent trees.
    head: Vec<(String, String)>,
    /// Nested binder frames, innermost last: `(name, parent)` pairs
    /// where a pattern binder's parent is its scrutinee's value root. A
    /// nested binder shadows a parameter or the definition's own name.
    frames: Vec<Vec<(String, Option<String>)>>,
    /// Under a hoisted closure.
    closure: bool,
    /// In tail position (the D104 walk).
    tail: bool,
    /// The current occurrence is a validating check (a `decide`
    /// operand).
    validating: bool,
    eq: usize,
    foreign: &'a ForeignMarks,
    facts: &'a mut BodyFacts,
}

impl<'a> FactWalk<'a> {
    fn in_scope(&self, name: &str) -> bool {
        self.frames.iter().any(|g| g.iter().any(|(b, _)| b == name))
    }

    /// The innermost binding of `name`'s subcomponent parent.
    fn parent_of(&self, name: &str) -> Option<Option<String>> {
        for g in self.frames.iter().rev() {
            if let Some((_, p)) = g.iter().rev().find(|(b, _)| b == name) {
                return Some(p.clone());
            }
        }
        if let Some((_, p)) = self.head.iter().rev().find(|(b, _)| b == name) {
            return Some(Some(p.clone()));
        }
        None
    }

    fn note_span(&mut self, span: ast::Span) {
        if self.facts.macro_site.is_none() && crate::lexer::is_generated(span) {
            self.facts.macro_site = Some(span);
        }
        if span.line > 0 {
            if self.facts.min_line == 0 || span.line < self.facts.min_line {
                self.facts.min_line = span.line;
            }
            if span.line > self.facts.max_line {
                self.facts.max_line = span.line;
            }
        }
    }

    fn bind(&mut self, name: &str) {
        self.frames.push(vec![(name.to_string(), None)]);
    }

    fn bind_pat(&mut self, p: &ast::Pat, parent: Option<&str>) {
        let mut names = Vec::new();
        collect_pat_names(p, &mut names);
        let frame: Vec<(String, Option<String>)> = names
            .into_iter()
            .map(|n| (n, parent.map(|p| p.to_string())))
            .collect();
        if !frame.is_empty() {
            self.frames.push(frame);
        }
    }

    /// The subcomponent chain of `name` back to the argument position
    /// `pos`'s value (`#pos`), if it belongs to that tree.
    fn chain_at(&self, name: &str, pos: usize) -> Option<Vec<String>> {
        let target = format!("#{pos}");
        let mut chain = vec![name.to_string()];
        let mut x = name.to_string();
        for _ in 0..self.frames.iter().map(|g| g.len()).sum::<usize>() + 1 {
            match self.parent_of(&x) {
                Some(Some(parent)) => {
                    chain.push(parent.clone());
                    if parent == target {
                        return Some(chain);
                    }
                    x = parent;
                }
                _ => return None,
            }
        }
        None
    }

    fn use_param(&mut self, name: &str, span: ast::Span) {
        for (i, an) in self.arg_names.iter().enumerate() {
            if an.as_deref() == Some(name) {
                if self.validating {
                    self.param_state[i].0 = true;
                } else if !self.param_state[i].0 && !self.param_state[i].1 {
                    self.param_state[i].1 = true;
                    self.facts.unchecked_params.push((name.to_string(), span));
                }
                return;
            }
        }
    }

    fn record_var(&mut self, p: &ast::VarPath) {
        let span = p.span();
        self.note_span(span);
        let name = p.name.text.as_str();
        if self.in_scope(name) {
            return;
        }
        if self.arg_names.iter().any(|a| a.as_deref() == Some(name)) {
            self.use_param(name, span);
            return;
        }
        let self_match = p.quals.is_empty()
            || (p.quals.len() == 1 && p.quals[0].text == self.self_mod);
        if self_match && name == self.self_name {
            // A bare self reference: a recursive use that is not a
            // static call (it cannot carry a tail claim).
            self.facts.self_refs.push(span);
        }
    }

    fn record_head(&mut self, p: &ast::VarPath, args: &[ast::Term]) {
        let span = p.span();
        self.note_span(span);
        let name = p.name.text.as_str();
        if self.in_scope(name) {
            // A local closure's call: no named unit's call site.
            return;
        }
        let is_param = self.arg_names.iter().any(|a| a.as_deref() == Some(name));
        if is_param {
            self.use_param(name, span);
        } else {
            let self_match = p.quals.is_empty()
                || (p.quals.len() == 1 && p.quals[0].text == self.self_mod);
            if self_match && name == self.self_name {
                let shapes = args
                    .iter()
                    .enumerate()
                    .map(|(i, a)| self.arg_shape(a, i))
                    .collect();
                self.facts.self_calls.push(SelfCallRec {
                    span,
                    tail: self.tail,
                    closure: self.closure,
                    args: shapes,
                    eq: self.eq,
                });
            } else {
                match name {
                    "withScope" => self.facts.conc_sites.push(("scope".to_string(), span)),
                    "spawn" => self.facts.conc_sites.push(("spawn".to_string(), span)),
                    "joinAllOrCancel" => self.facts.conc_sites.push(("join".to_string(), span)),
                    _ => {}
                }
                if name == "mutBorrow" {
                    // The `Mut` introduction is an allocation site
                    // (attribute annotations specification section 10:
                    // a `Mut` cell is fresh state).
                    self.facts.alloc_sites.push(AllocRec {
                        what: "a `Mut` cell (the `mutBorrow` introduction)".to_string(),
                        span,
                        closure: self.closure,
                    });
                }
                if self.foreign.ptrs.contains(name) {
                    self.facts.ptr_ops.push((name.to_string(), span));
                }
                if self.foreign.aggregates.contains(name) {
                    self.facts.alloc_sites.push(AllocRec {
                        what: format!("a foreign aggregate (the call to `{name}`)"),
                        span,
                        closure: self.closure,
                    });
                }
            }
        }
        self.facts.calls.push(CallRec {
            callee: name.to_string(),
            span,
            tail: self.tail,
            closure: self.closure,
        });
    }

    fn arg_shape(&self, a: &ast::Term, pos: usize) -> ArgShape {
        match a {
            ast::Term::Var(p) if p.quals.is_empty() => {
                match self.chain_at(&p.name.text, pos) {
                    Some(_) => ArgShape::Sub {
                        strict: self.arg_names[pos].as_deref() != Some(p.name.text.as_str()),
                    },
                    None => ArgShape::Other,
                }
            }
            ast::Term::Int(v, _) => v
                .parse::<i128>()
                .map(ArgShape::Literal)
                .unwrap_or(ArgShape::Other),
            ast::Term::Hole(_) => ArgShape::Hole,
            // `param - k` (or `nat-sub param k`): a measure step of the
            // parameter's value at this very position.
            ast::Term::App(f, args, _) if args.len() == 2 => {
                let head_name = match &**f {
                    ast::Term::Var(p) if p.quals.is_empty() && !self.in_scope(&p.name.text) => {
                        Some(p.name.text.as_str())
                    }
                    _ => None,
                };
                let step = matches!(head_name, Some("-") | Some("nat-sub"));
                let base = matches!(&args[0], ast::Term::Var(p)
                    if p.quals.is_empty()
                        && !self.in_scope(&p.name.text)
                        && self.arg_names[pos].as_deref() == Some(p.name.text.as_str()));
                let down = match &args[1] {
                    ast::Term::Int(v, _) => v.parse::<i128>().map(|k| k > 0).unwrap_or(false),
                    _ => false,
                };
                if step && base && down {
                    ArgShape::Measure
                } else {
                    ArgShape::Other
                }
            }
            _ => ArgShape::Other,
        }
    }

    fn walk_term(&mut self, t: &ast::Term) {
        self.note_span(t.span());
        match t {
            ast::Term::Var(p) => self.record_var(p),
            ast::Term::Con(p) => {
                self.note_span(p.span());
                self.facts.alloc_sites.push(AllocRec {
                    what: format!("the constructor `{}`", p.parts().last().unwrap_or(&"")),
                    span: p.span(),
                    closure: self.closure,
                });
            }
            ast::Term::Int(..) | ast::Term::Char(..) | ast::Term::Sym(..) | ast::Term::Unit(_)
            | ast::Term::Hole(_)
            | ast::Term::SpliceHole { .. } | ast::Term::SeqOp(..) => {}
            ast::Term::Str(_, s) => {
                // A string literal is a shared read-only constant (no
                // allocation), so it is not an allocation site.
                self.note_span(*s);
            }
            ast::Term::App(f, args, _) => {
                // The application head is the call; its arguments run
                // before it in non-tail position. A `decide` operand is
                // a validating check of the parameters it examines (the
                // `checked-params` rule's runtime `decide`).
                let deciding = matches!(&**f, ast::Term::Var(p)
                    if p.quals.is_empty()
                        && p.name.text == "decide"
                        && !self.in_scope(&p.name.text));
                for a in args {
                    let saved = self.validating;
                    if deciding {
                        self.validating = true;
                    }
                    self.walk_child(a);
                    self.validating = saved;
                }
                match &**f {
                    ast::Term::Var(p) => self.record_head(p, args),
                    ast::Term::Con(p) => {
                        self.note_span(p.span());
                        self.facts.alloc_sites.push(AllocRec {
                            what: format!("the constructor `{}`", p.parts().last().unwrap_or(&"")),
                            span: p.span(),
                            closure: self.closure,
                        });
                    }
                    other => self.walk_child(other),
                }
            }
            ast::Term::Lam(pats, body, s) => {
                self.note_span(*s);
                self.facts.alloc_sites.push(AllocRec {
                    what: "a closure (a lambda)".to_string(),
                    span: *s,
                    closure: self.closure,
                });
                let saved = self.closure;
                let depth = self.frames.len();
                self.closure = true;
                for p in pats {
                    self.bind_pat(p, None);
                }
                self.walk_term(body);
                self.closure = saved;
                self.frames.truncate(depth);
            }
            ast::Term::Let(decls, body, _) | ast::Term::Letrec(decls, body, _) => {
                let recursive = matches!(t, ast::Term::Letrec(..));
                if recursive {
                    self.facts.alloc_sites.push(AllocRec {
                        what: "a `letrec` fixpoint box".to_string(),
                        span: t.span(),
                        closure: self.closure,
                    });
                }
                let depth = self.frames.len();
                self.walk_decls(decls, recursive);
                let saved = self.tail;
                self.walk_term(body);
                self.tail = saved;
                self.frames.truncate(depth);
            }
            ast::Term::If { cond, then, els, .. } => {
                self.facts.branches += 1;
                self.walk_child(cond);
                let saved = self.tail;
                self.walk_term(then);
                self.walk_term(els);
                self.tail = saved;
            }
            ast::Term::Case(scrut, alts, _) => {
                if !alts.is_empty() {
                    self.facts.branches += (alts.len() - 1) as u64;
                }
                // A `case` whose alternatives can fail validates its
                // scrutinee (the `checked-params` rule).
                let validates = alts.iter().any(|a| pat_refutable(&a.pat));
                let scrut_param = match &**scrut {
                    ast::Term::Var(p) if p.quals.is_empty() && !self.in_scope(&p.name.text) => {
                        Some(p.name.text.clone())
                    }
                    _ => None,
                };
                if validates && scrut_param.is_some() {
                    self.validating = true;
                    self.walk_child(scrut);
                    self.validating = false;
                } else {
                    self.walk_child(scrut);
                }
                for a in alts {
                    if a.guard.is_some() {
                        self.facts.branches += 1;
                    }
                    let depth = self.frames.len();
                    self.bind_pat(&a.pat, scrut_param.as_deref());
                    if let Some(g) = &a.guard {
                        self.walk_child(g);
                    }
                    let saved = self.tail;
                    self.walk_term(&a.body);
                    self.tail = saved;
                    self.frames.truncate(depth);
                }
            }
            ast::Term::Ann(e, _, _) => {
                let saved = self.tail;
                self.walk_term(e);
                self.tail = saved;
            }
            ast::Term::Forget { value, .. } => {
                let saved = self.tail;
                self.walk_term(value);
                self.tail = saved;
            }
            ast::Term::Narrow { value, witness, .. } => {
                let saved = self.tail;
                self.walk_term(value);
                self.tail = saved;
                let saved = self.tail;
                self.walk_term(witness);
                self.tail = saved;
            }
            ast::Term::Infix(terms, ops, _) => {
                for t in terms {
                    self.walk_child(t);
                }
                for op in ops {
                    self.facts.calls.push(CallRec {
                        callee: op.text.clone(),
                        span: op.span,
                        tail: self.tail,
                        closure: self.closure,
                    });
                }
            }
            ast::Term::Neg(e, _) | ast::Term::Proj(e, _, _) => self.walk_child(e),
            ast::Term::Tuple(items, s) => {
                self.note_span(*s);
                self.facts.alloc_sites.push(AllocRec {
                    what: "a tuple construction".to_string(),
                    span: *s,
                    closure: self.closure,
                });
                for i in items {
                    self.walk_child(i);
                }
            }
            ast::Term::List(items, s) => {
                self.note_span(*s);
                self.facts.alloc_sites.push(AllocRec {
                    what: "a list construction (a `Bytes` buffer or cons cells)".to_string(),
                    span: *s,
                    closure: self.closure,
                });
                for i in items {
                    self.walk_child(i);
                }
            }
            ast::Term::Map(items, s) => {
                self.note_span(*s);
                self.facts.alloc_sites.push(AllocRec {
                    what: "a map construction".to_string(),
                    span: *s,
                    closure: self.closure,
                });
                for (k, v) in items {
                    self.walk_child(k);
                    self.walk_child(v);
                }
            }
            ast::Term::Range(start, step, end, s) => {
                self.note_span(*s);
                self.facts.alloc_sites.push(AllocRec {
                    what: "a range construction (a `Bytes` buffer or cons cells)".to_string(),
                    span: *s,
                    closure: self.closure,
                });
                self.walk_child(start);
                if let Some(x) = step {
                    self.walk_child(x);
                }
                self.walk_child(end);
            }
            ast::Term::Record(path, fields, s) => {
                self.note_span(*s);
                self.facts.alloc_sites.push(AllocRec {
                    what: format!(
                        "the record construction of `{}`",
                        path.parts().last().unwrap_or(&"")
                    ),
                    span: *s,
                    closure: self.closure,
                });
                for (_, v) in fields {
                    self.walk_child(v);
                }
            }
            ast::Term::Do(stmts, _) => {
                // One statement alone is the block's value in the
                // enclosing unit; the first statement of a longer block
                // is the bind's argument, and every later statement
                // lowers into the bind's continuation closure.
                let multi = stmts.len() > 1;
                let depth = self.frames.len();
                for (i, s) in stmts.iter().enumerate() {
                    let saved_closure = self.closure;
                    let saved_tail = self.tail;
                    if multi {
                        self.tail = false;
                        if i > 0 {
                            self.closure = true;
                        }
                    }
                    match s {
                        ast::DoStmt::Bind { pat, term, .. } => {
                            self.walk_term(term);
                            self.bind_pat(pat, None);
                        }
                        ast::DoStmt::Let { decls, .. } => self.walk_decls(decls, false),
                        ast::DoStmt::Expr { term, .. } => self.walk_term(term),
                    }
                    self.closure = saved_closure;
                    self.tail = saved_tail;
                }
                self.frames.truncate(depth);
            }
            ast::Term::Handle(body, handler, _) => {
                self.walk_child(body);
                self.walk_child(handler);
            }
            ast::Term::Handler(h) => {
                let saved = self.closure;
                let depth = self.frames.len();
                self.closure = true;
                self.bind_pat(&h.ret_pat, None);
                self.walk_term(&h.ret_body);
                for c in &h.clauses {
                    let clause_depth = self.frames.len();
                    for p in &c.pats {
                        self.bind_pat(p, None);
                    }
                    if let Some(r) = &c.resume {
                        self.bind(&r.text);
                    }
                    self.walk_term(&c.body);
                    self.frames.truncate(clause_depth);
                }
                self.closure = saved;
                self.frames.truncate(depth);
            }
            ast::Term::With { name, value, body, .. } => {
                self.walk_child(value);
                let depth = self.frames.len();
                self.bind(&name.text);
                let saved = self.tail;
                self.walk_term(body);
                self.tail = saved;
                self.frames.truncate(depth);
            }
            ast::Term::CapApply { head, supplies, .. } => {
                for (_, v) in supplies {
                    self.walk_child(v);
                }
                let saved = self.tail;
                self.walk_term(head);
                self.tail = saved;
            }
            ast::Term::Quote { body, .. } | ast::Term::Splice { body, .. } => {
                self.walk_child(body)
            }
            ast::Term::QuoteDecls { decls, .. } => {
                for d in decls {
                    self.walk_decl_tree(d);
                }
            }
            ast::Term::MacroCall { span, .. } => {
                self.note_span(*span);
                if self.facts.macro_site.is_none() {
                    self.facts.macro_site = Some(*span);
                }
            }
            ast::Term::CpsWrap(body, _) => {
                let saved = self.tail;
                self.walk_term(body);
                self.tail = saved;
            }
        }
    }

    /// A subterm in non-tail position.
    fn walk_child(&mut self, t: &ast::Term) {
        let saved = self.tail;
        self.tail = false;
        self.walk_term(t);
        self.tail = saved;
    }

    fn walk_decls(&mut self, decls: &[ast::LocalDecl], recursive: bool) {
        // A `let` group is sequential: later bindings see earlier ones.
        // A `letrec` group binds every name first, so a body's own name
        // is a local there, never the definition or a parameter.
        if recursive {
            let names: Vec<String> = decls
                .iter()
                .filter_map(|d| match d {
                    ast::LocalDecl::Eq { name, .. } => Some(name.text.clone()),
                    _ => None,
                })
                .collect();
            if !names.is_empty() {
                self.frames
                    .push(names.into_iter().map(|n| (n, None)).collect());
            }
        }
        for d in decls {
            match d {
                ast::LocalDecl::Sig { span, .. } => self.note_span(*span),
                ast::LocalDecl::Bind { pat, body, span } => {
                    self.note_span(*span);
                    self.walk_child(body);
                    self.bind_pat(pat, None);
                }
                ast::LocalDecl::Lazy { pat, body, span } => {
                    self.note_span(*span);
                    self.facts.alloc_sites.push(AllocRec {
                        what: "a `delay` thunk (a memo binding)".to_string(),
                        span: *span,
                        closure: self.closure,
                    });
                    let saved = self.closure;
                    self.closure = true;
                    self.walk_child(body);
                    self.closure = saved;
                    self.bind_pat(pat, None);
                }
                ast::LocalDecl::Eq { name, pats, body, span } => {
                    self.note_span(*span);
                    // A local function (its equation takes patterns) is a
                    // lambda and allocates a closure; a nullary binding is
                    // the strict `let` of the elaboration (check_terms),
                    // not a construction site.
                    if !pats.is_empty() {
                        self.facts.alloc_sites.push(AllocRec {
                            what: "a closure (a local function)".to_string(),
                            span: *span,
                            closure: self.closure,
                        });
                    }
                    let saved = self.closure;
                    if !pats.is_empty() {
                        self.closure = true;
                    }
                    for p in pats {
                        self.bind_pat(p, None);
                    }
                    self.walk_term(body);
                    self.closure = saved;
                    if !recursive {
                        self.bind(&name.text);
                    }
                }
            }
        }
    }

    fn walk_decl_tree(&mut self, d: &ast::Decl) {
        match d {
            ast::Decl::Attributed { inner, .. } => self.walk_decl_tree(inner),
            ast::Decl::Equation { eq, .. } => {
                let depth = self.frames.len();
                self.walk_decls(&eq.locals, true);
                self.walk_child(&eq.body);
                self.frames.truncate(depth);
            }
            _ => {}
        }
    }
}

fn collect_pat_names(p: &ast::Pat, out: &mut Vec<String>) {
    match p {
        ast::Pat::Var(n, _) => out.push(n.text.clone()),
        ast::Pat::As(n, inner, _) => {
            out.push(n.text.clone());
            collect_pat_names(inner, out);
        }
        ast::Pat::Con(_, args, _) | ast::Pat::Tuple(args, _) | ast::Pat::List(args, _) => {
            for a in args {
                collect_pat_names(a, out);
            }
        }
        ast::Pat::Record(_, fields, _) => {
            for (_, f) in fields {
                collect_pat_names(f, out);
            }
        }
        _ => {}
    }
}

/// Collect one definition's body facts from its equations (attribute
/// annotations specification section 10).
fn collect_body_facts(
    self_mod: &str,
    self_name: &str,
    eqs: &[ast::Equation],
    foreign: &ForeignMarks,
) -> BodyFacts {
    let mut facts = BodyFacts::default();
    for (i, e) in eqs.iter().enumerate() {
        let mut arg_names = Vec::new();
        for p in &e.pats {
            arg_names.push(pat_direct(p).map(|n| n.text.clone()));
        }
        let mut w = FactWalk {
            self_mod,
            self_name,
            param_state: vec![(false, false); e.pats.len()],
            arg_names,
            head: Vec::new(),
            frames: Vec::new(),
            closure: false,
            tail: true,
            validating: false,
            eq: i,
            foreign,
            facts: &mut facts,
        };
        for (pos, p) in e.pats.iter().enumerate() {
            // The argument at `pos` is the root of its subcomponent tree.
            let root = format!("#{pos}");
            let mut names = Vec::new();
            collect_pat_names(p, &mut names);
            for n in names {
                w.head.push((n, root.clone()));
            }
            // A head pattern that destructures the argument is the
            // boundary check; a bare variable defers validation to the
            // body (the `checked-params` rule).
            w.param_state[pos].0 = !matches!(p, ast::Pat::Var(..));
        }
        let depth = w.frames.len();
        w.walk_decls(&e.locals, true);
        w.walk_term(&e.body);
        w.frames.truncate(depth);
    }
    if facts.max_line >= facts.min_line && facts.min_line > 0 {
        facts.body_lines = (facts.max_line - facts.min_line + 1) as u64;
    }
    facts
}

// ------------------------------ the verification ------------------------------

/// The depth bound one proof method establishes (decision D113).
#[derive(Clone, Debug)]
pub(crate) enum Established {
    /// A number of nested recursive calls.
    Num(u64),
    /// The named argument or size index the depth is bounded by (the
    /// `Nat`/`Fin` bounded-counter pattern: "bounded by its arguments").
    Named(Vec<String>),
    /// Bounded by the size of a destructured argument the surface names
    /// no variable for: a bound no assertion can cite.
    Unnamed,
}

/// One definition's verified safety state (attribute annotations
/// specification section 10).
#[derive(Clone, Debug)]
pub(crate) struct SafetyRecord {
    /// The unit name (`Module.name`).
    pub key: String,
    pub display: String,
    pub module: String,
    pub attrs: SafetySet,
    pub facts: BodyFacts,
    pub def_span: ast::Span,
    pub exported: bool,
    /// A `spec`/`proof` definition: erased ghost code (the `jpl`
    /// profile's runtime rules do not reach it).
    pub logical: bool,
    /// Under the `jpl` profile (decision D114).
    pub jpl: bool,
    /// The structural depth proof (`bounded(n)`'s first method), with
    /// its report.
    pub structural: Option<(String, Established)>,
    /// The conservative call-graph depth proof (the fallback), with its
    /// report.
    pub call_graph: Option<(String, Established)>,
    /// The satisfied assertions as contract facts (decision D112).
    pub fact_strings: Vec<String>,
}

/// The `jpl` profile's numbers (decision D114): complexity 10, lines 60,
/// `assert-min 2` on exported functions.
pub(crate) const JPL_COMPLEXITY: u64 = 10;
pub(crate) const JPL_LINES: u64 = 60;
pub(crate) const JPL_ASSERT_MIN: u64 = 2;

/// One verified assertion's summary row (attribute annotations
/// specification section 10): per definition, the attribute, its
/// verification result, and for `bounded(x)` which proof method
/// established the bound (decision D113). The artifact manifest's
/// `safety` section records these; they name definitions, never paths.
#[derive(Clone, Debug)]
pub struct SafetyAssertion {
    /// The definition (`Module.name`).
    pub definition: String,
    /// The assertion as the contract facts spell it
    /// (`recursion=bounded(3)`, `no-alloc`, ...), or `jpl <rule>` for a
    /// rule the `jpl` profile mandated.
    pub attribute: String,
    /// The verification result (the proof method for `bounded(x)`).
    pub result: String,
}

/// The P20 safety assertion report (attribute annotations specification
/// section 10; decision D113: the report says which proof method was
/// used).
#[derive(Clone, Debug, Default)]
pub struct SafetyReport {
    /// One row per verified assertion (the definition's own attributes,
    /// and the `jpl` profile's mandated rules under the profile).
    pub assertions: Vec<SafetyAssertion>,
    /// The `jpl` profile's whole-program summary (decision D114);
    /// `None` without the profile.
    pub jpl: Option<JplSummary>,
}

/// The `jpl` profile's whole-program summary (decision D114).
#[derive(Clone, Debug, Default)]
pub struct JplSummary {
    /// The profiled user modules.
    pub modules: Vec<String>,
    /// Definitions checked under the profile.
    pub definitions: u64,
    /// Assertions verified (the profile's mandated set, plus each
    /// definition's own stricter attributes).
    pub assertions: u64,
}

/// Does the type carry a raw pointer or an opaque type (the
/// `#[no-raw-ptr]` signature rule)?
fn ty_has_raw_ptr(t: &Ty, opaque: &HashSet<String>, out: &mut Option<String>) -> bool {
    match t {
        Ty::Ptr { role, pointee } => {
            if matches!(role, PtrRole::Raw) {
                if out.is_none() {
                    *out = Some(format!("{role}"));
                }
                return true;
            }
            ty_has_raw_ptr(pointee, opaque, out)
        }
        Ty::Adt(name, args) => {
            if opaque.contains(name) {
                if out.is_none() {
                    *out = Some(format!("the opaque type `{name}`"));
                }
                return true;
            }
            args.iter().any(|a| ty_has_raw_ptr(a, opaque, out))
        }
        Ty::App(h, args) => {
            ty_has_raw_ptr(h, opaque, out) || args.iter().any(|a| ty_has_raw_ptr(a, opaque, out))
        }
        Ty::Tuple(items) => items.iter().any(|a| ty_has_raw_ptr(a, opaque, out)),
        Ty::Fun(a, _, _, b) => {
            ty_has_raw_ptr(a, opaque, out) || ty_has_raw_ptr(b, opaque, out)
        }
        Ty::Ref(_, a) | Ty::Mut(_, a) | Ty::Later(a) => ty_has_raw_ptr(a, opaque, out),
        Ty::Thunk { inner, .. } | Ty::Lazy { inner, .. } => ty_has_raw_ptr(inner, opaque, out),
        Ty::Pi { dom, cod, .. } => ty_has_raw_ptr(dom, opaque, out) || ty_has_raw_ptr(cod, opaque, out),
        Ty::Sigma { dom, cod, .. } => {
            ty_has_raw_ptr(dom, opaque, out) || ty_has_raw_ptr(cod, opaque, out)
        }
        _ => false,
    }
}

/// The depth bound of one strict decrease at a self-recursive call
/// (attribute annotations specification section 10: structural
/// recursion over `Vec`/`Nat` sizes proves it via the totality
/// machinery).
fn decrease_bound(doms: &[Ty], pos: usize, measure: bool, name: Option<&str>) -> Established {
    if measure {
        // A well-founded measure step of the parameter's value: the
        // depth is bounded by the entry value (the bounded-counter
        // pattern of design/safety-assertion-attributes.md rule 2).
        return match name {
            Some(n) => Established::Named(vec![n.to_string()]),
            None => Established::Unnamed,
        };
    }
    // A structural subcomponent descent: the depth is bounded by the
    // argument's size. A size index names that size exactly when it is
    // a variable; a LITERAL index is deliberately not claimed as a
    // numeric bound here, because a well-typed self-descent preserves
    // its index (the checker refuses a descent that would shrink one),
    // so a literal index never measures the descent's depth.
    let mut named = Vec::new();
    if let Some(n) = name {
        named.push(n.to_string());
    }
    if let Some(t) = doms.get(pos) {
        match t {
            Ty::Adt(_, args) => {
                for a in args {
                    if let Ty::Idx(ix) = a {
                        if let ITerm::Var(v) = &**ix {
                            named.push(v.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if named.is_empty() {
        Established::Unnamed
    } else {
        Established::Named(named)
    }
}

/// The conservative call-graph depth bound (decision D113's fallback):
/// the literal counter ladder, where every equation matches a literal
/// counter at position 0 and every recursive call steps the counter
/// down a literal that some equation matches. The bound is the deepest
/// chain of nested self-call frames from any entry counter. Returns
/// `None` when no conservative bound applies.
fn literal_ladder(eqs: &[ast::Equation], self_calls: &[SelfCallRec]) -> Option<u64> {
    if self_calls.is_empty() {
        return None;
    }
    if self_calls.iter().any(|c| c.closure || c.args.is_empty()) {
        return None;
    }
    // Every equation matches a literal counter at position 0.
    let mut lits: Vec<i128> = Vec::with_capacity(eqs.len());
    for e in eqs {
        let ast::Pat::Int(k, _) = &e.pats[0] else {
            return None;
        };
        lits.push(k.parse().ok()?);
    }
    // Every self-call steps the counter down a literal.
    let mut steps: Vec<Vec<i128>> = vec![Vec::new(); eqs.len()];
    for c in self_calls {
        let Some(ArgShape::Literal(m)) = c.args.first() else {
            return None;
        };
        steps[c.eq].push(*m);
    }
    // The frames one entry counter reaches: the equation matching the
    // counter runs, and its calls step strictly down to the equations
    // matching those literals.
    fn frames(k: i128, lits: &[i128], steps: &[Vec<i128>], seen: usize) -> Option<u64> {
        if seen > lits.len() {
            return None;
        }
        let eq = lits.iter().position(|l| *l == k)?;
        let mut deepest = 0u64;
        for &m in &steps[eq] {
            if m >= k {
                return None;
            }
            deepest = deepest.max(frames(m, lits, steps, seen + 1)?);
        }
        Some(1 + deepest)
    }
    let mut bound = 0u64;
    for &k in &lits {
        bound = bound.max(frames(k, &lits, &steps, 0)?);
    }
    Some(bound)
}

/// One definition's `bounded(n)` proof, structural first and the
/// conservative call-graph bound otherwise (decision D113).
fn bounded_proof(rec: &SafetyRecord) -> Result<(String, Established)> {
    if rec.facts.self_refs.is_empty() && rec.facts.self_calls.is_empty() {
        return Ok((
            "call-graph (no recursive calls; depth 0)".to_string(),
            Established::Num(0),
        ));
    }
    if let Some((report, est)) = &rec.structural {
        return Ok((report.clone(), est.clone()));
    }
    if let Some((report, est)) = &rec.call_graph {
        return Ok((report.clone(), est.clone()));
    }
    Err(Error::at(
        rec.def_span,
        format!(
            "no depth bound is established for {}: the structural proof does not apply and no conservative call-graph bound applies; a bound that cannot be established is refused, never trusted (attribute annotations specification section 10; decision D113)",
            rec.display
        ),
    ))
}

/// Establish the structural depth proof of one definition, if its
/// self-recursive calls all decrease (the totality machinery's
/// subcomponent chains, extended with the well-founded measure decrease
/// of core specification section 8).
fn structural_proof_for(
    eqs: &[ast::Equation],
    doms: &[Ty],
    facts: &BodyFacts,
    def_span: ast::Span,
) -> Option<(String, Established)> {
    let _ = def_span;
    if !facts.self_refs.is_empty() {
        return None;
    }
    if facts.self_calls.is_empty() {
        return None;
    }
    let mut bounds: Vec<Established> = Vec::new();
    for c in &facts.self_calls {
        if c.closure {
            // The call runs in a hoisted closure: no static descent
            // reaches it.
            return None;
        }
        let mut strict = None;
        for (i, shape) in c.args.iter().enumerate() {
            let (dec, measure) = match shape {
                ArgShape::Sub { strict } => (*strict, false),
                ArgShape::Measure => (true, true),
                _ => (false, false),
            };
            if dec {
                let name = eqs
                    .get(c.eq)
                    .and_then(|e| e.pats.get(i))
                    .and_then(pat_direct)
                    .map(|n| n.text.clone());
                strict = Some(decrease_bound(doms, i, measure, name.as_deref()));
                break;
            }
        }
        let Some(b) = strict else {
            // No argument decreases structurally: the totality
            // machinery refuses this recursion.
            return None;
        };
        bounds.push(b);
    }
    // The claim is established when some strict decrease names the
    // bound it gives.
    let established = bounds.into_iter().find(|b| !matches!(b, Established::Unnamed));
    let Some(Established::Named(ns)) = established else {
        // `decrease_bound` establishes named bounds only (the argument
        // or its size index); a bound no assertion can cite is not a
        // bound.
        return None;
    };
    let report = format!(
        "structural (depth <= {}: the size of the decreasing argument)",
        ns[0]
    );
    Some((report, Established::Named(ns)))
}

impl Checker {
    /// The foreign bindings of the collected modules (the `#[no-raw-ptr]`
    /// and foreign-aggregate marks the body walker reads).
    fn foreign_marks(&self) -> ForeignMarks {
        let mut marks = ForeignMarks::default();
        for (name, f) in &self.module_foreign {
            let _ = name;
            let mut ptr = false;
            let mut agg = false;
            for p in &f.params {
                ptr |= p.class == "ptr" || p.role == "raw" || p.role == "opaque-handle";
                agg |= p.class == "aggregate";
            }
            ptr |= f.result.class == "ptr"
                || f.result.role == "raw"
                || f.result.role == "opaque-handle";
            agg |= f.result.class == "aggregate";
            if ptr {
                marks.ptrs.insert(f.symbol.clone());
                marks.ptrs.insert(f.external.clone());
            }
            if agg {
                marks.aggregates.insert(f.symbol.clone());
                marks.aggregates.insert(f.external.clone());
            }
        }
        marks
    }

    /// Is `name` in this module's export surface (the `jpl` profile's
    /// `assert-min` rule counts exported functions)?
    fn surface_value(&self, module: &str, name: &str) -> bool {
        match self.module_surface.get(module) {
            Some(sm) => sm.iter().any(|(_, d)| d.name == name),
            None => self.declared_names(module).iter().any(|n| n == name),
        }
    }

    /// Record one definition's safety assertions (attribute annotations
    /// specification section 10): one definition carries one of each,
    /// merged across its signature and equations.
    fn record_fn_safety(
        &mut self,
        key: &str,
        set: &SafetySet,
        display: &str,
    ) -> Result<()> {
        if set.is_empty() {
            return Ok(());
        }
        let entry = self.safety_attrs.entry(key.to_string()).or_default();
        entry.merge(set, display)
    }

    /// Is the `jpl` profile in force for `module` (the `jpl` target
    /// profile covers the user's program; `#[profile "jpl"]` covers its
    /// module; decision D114)?
    fn jpl_for(&self, module: &str) -> bool {
        (self.jpl_target && self.jpl_user_modules.contains(module))
            || self
                .module_safety
                .get(module)
                .map(|m| m.profile_jpl)
                .unwrap_or(false)
    }

    /// Verify one definition's safety assertions at the definition
    /// (attribute annotations specification section 10; decision D112):
    /// a violation is a compile error naming the attribute, the asserted
    /// rule, and the offending site. The group claims (`#[recursion]`,
    /// `#[stack]`) verify against the checked program's D104 groups once
    /// every definition is elaborated.
    fn verify_definition_safety(
        &mut self,
        key: &str,
        module: &str,
        name: &str,
        eqs: &[ast::Equation],
        doms: &[Ty],
        logical: bool,
        def_span: ast::Span,
    ) -> Result<()> {
        let attrs = self.safety_attrs.get(key).cloned().unwrap_or_default();
        let jpl = !logical && self.jpl_for(module);
        if attrs.is_empty() && !jpl {
            return Ok(());
        }
        let display = format!("{module}.{name}");
        let exported = self.surface_value(module, name);
        let marks = self.foreign_marks();
        let mut facts = collect_body_facts(module, name, eqs, &marks);
        facts.runtime_decides = std::mem::take(&mut self.cur_runtime_decides);

        // Opaque types of the signature (the `#[no-raw-ptr]` rule).
        let mut opaque: HashSet<String> = HashSet::new();
        for (n, kinds) in &self.types {
            if kinds.iter().any(|k| k.opaque) {
                opaque.insert(n.clone());
            }
        }

        // -- `#[max-complexity n]` (JPL rule 4): branches + 1.
        let complexity_cap = match attrs.max_complexity {
            Some((n, span)) => {
                if jpl && n > JPL_COMPLEXITY {
                    return Err(Error::at(
                        span,
                        format!(
                            "`#[max-complexity {n}]` is looser than the `jpl` profile's rule (complexity {JPL_COMPLEXITY}): a definition's own attributes may be stricter than the profile, never looser (attribute annotations specification section 10; decision D114)"
                        ),
                    ));
                }
                n
            }
            None => {
                if jpl {
                    JPL_COMPLEXITY
                } else {
                    u64::MAX
                }
            }
        };
        if complexity_cap != u64::MAX {
            let c = facts.branches + 1;
            if c > complexity_cap {
                let who = if attrs.max_complexity.is_some() {
                    format!(
                        "`#[max-complexity {complexity_cap}]` on {display} asserts the bounded-complexity rule (JPL rule 4)"
                    )
                } else {
                    format!(
                        "the `jpl` profile mandates complexity {JPL_COMPLEXITY} for every definition (decision D114)"
                    )
                };
                return Err(Error::at(
                    attrs.max_complexity.map(|(_, s)| s).unwrap_or(def_span),
                    format!(
                        "{who}: the body's cyclomatic complexity (branches + 1) is {c}, above {complexity_cap}"
                    ),
                ));
            }
        }

        // -- `#[max-lines n]` (JPL rule 4): body source lines.
        let lines_cap = match attrs.max_lines {
            Some((n, span)) => {
                if jpl && n > JPL_LINES {
                    return Err(Error::at(
                        span,
                        format!(
                            "`#[max-lines {n}]` is looser than the `jpl` profile's rule (lines {JPL_LINES}): a definition's own attributes may be stricter than the profile, never looser (attribute annotations specification section 10; decision D114)"
                        ),
                    ));
                }
                n
            }
            None => {
                if jpl {
                    JPL_LINES
                } else {
                    u64::MAX
                }
            }
        };
        if lines_cap != u64::MAX && facts.body_lines > lines_cap {
            let who = if attrs.max_lines.is_some() {
                format!(
                    "`#[max-lines {lines_cap}]` on {display} asserts the small-functions rule (JPL rule 4)"
                )
            } else {
                format!(
                    "the `jpl` profile mandates at most {JPL_LINES} source lines (decision D114)"
                )
            };
            return Err(Error::at(
                attrs.max_lines.map(|(_, s)| s).unwrap_or(def_span),
                format!(
                    "{who}: the body spans {} source lines, above {lines_cap}",
                    facts.body_lines
                ),
            ));
        }

        // -- `#[assert-min n]` (JPL rule 5): runtime checks in the body;
        // proof obligations do not count.
        let assert_floor = match attrs.assert_min {
            Some((n, span)) => {
                if jpl && exported && n < JPL_ASSERT_MIN {
                    return Err(Error::at(
                        span,
                        format!(
                            "`#[assert-min {n}]` is looser than the `jpl` profile's rule ({JPL_ASSERT_MIN} runtime checks on exported functions): a definition's own attributes may be stricter than the profile, never looser (attribute annotations specification section 10; decision D114)"
                        ),
                    ));
                }
                n
            }
            None => {
                if jpl && exported {
                    JPL_ASSERT_MIN
                } else {
                    0
                }
            }
        };
        if (facts.runtime_decides.len() as u64) < assert_floor {
            let who = if attrs.assert_min.is_some() {
                format!(
                    "`#[assert-min {assert_floor}]` on {display} asserts the runtime-assertion rule (JPL rule 5)"
                )
            } else {
                format!(
                    "the `jpl` profile mandates {JPL_ASSERT_MIN} runtime checks on exported functions (decision D114)"
                )
            };
            return Err(Error::at(
                attrs.assert_min.map(|(_, s)| s).unwrap_or(def_span),
                format!(
                    "{who}: the body has {} runtime `decide` check(s) (proof obligations do not count), below {assert_floor}",
                    facts.runtime_decides.len()
                ),
            ));
        }

        // -- `#[checked-params]` (JPL rule 7): every parameter is
        // validated before first use.
        if attrs.checked_params.is_some() {
            for (p, span) in &facts.unchecked_params {
                return Err(Error::at(
                    *span,
                    format!(
                        "`#[checked-params]` on {display} asserts the boundary-validation rule (JPL rule 7): the parameter `{p}` is used here before any validating check (a refutable `case` match or a runtime `decide`)"
                    ),
                ));
            }
        }

        // -- `#[no-macros]` (JPL rule 8): no expansion inside.
        if attrs.no_macros.is_some() || jpl {
            if let Some(span) = facts.macro_site {
                let who = if attrs.no_macros.is_some() {
                    format!("`#[no-macros]` on {display} asserts the preprocessor rule (JPL rule 8)")
                } else {
                    format!(
                        "the `jpl` profile forbids macro expansion inside a definition (decision D114)"
                    )
                };
                return Err(Error::at(
                    attrs.no_macros.unwrap_or(span),
                    format!("{who}: a macro expansion occurred at this site"),
                ));
            }
        }

        // -- `#[no-raw-ptr]` (JPL rule 9): no raw pointer or opaque type
        // in the signature, no foreign pointer operation in the body.
        let sig_ptr = attrs.no_raw_ptr.is_some() || jpl;
        if sig_ptr {
            let mut where_ = None;
            for t in doms {
                if ty_has_raw_ptr(t, &opaque, &mut where_) {
                    let who = if attrs.no_raw_ptr.is_some() {
                        format!(
                            "`#[no-raw-ptr]` on {display} asserts the pointer rule (JPL rule 9)"
                        )
                    } else {
                        format!(
                            "the `jpl` profile forbids raw pointers and opaque types (decision D114)"
                        )
                    };
                    return Err(Error::at(
                        attrs.no_raw_ptr.unwrap_or(def_span),
                        format!(
                            "{who}: {} appears in the signature",
                            where_.unwrap_or_else(|| "a raw pointer".to_string())
                        ),
                    ));
                }
            }
            for (fname, span) in &facts.ptr_ops {
                let who = if attrs.no_raw_ptr.is_some() {
                    format!(
                        "`#[no-raw-ptr]` on {display} asserts the pointer rule (JPL rule 9)"
                    )
                } else {
                    format!(
                        "the `jpl` profile forbids foreign pointer operations (decision D114)"
                    )
                };
                return Err(Error::at(
                    attrs.no_raw_ptr.unwrap_or(*span),
                    format!("{who}: the foreign pointer operation `{fname}` runs here"),
                ));
            }
        }

        // -- `#[no-alloc]` (JPL rule 3): no allocation site in the body.
        if attrs.no_alloc.is_some() {
            for site in &facts.alloc_sites {
                return Err(Error::at(
                    site.span,
                    format!(
                        "`#[no-alloc]` on {display} asserts the no-allocation rule (JPL rule 3): {} is constructed here",
                        site.what
                    ),
                ));
            }
        }

        // -- `#[alloc "once"]` (JPL rule 3): the initialization pattern;
        // the `jpl` profile admits allocation only here.
        let alloc_cap = if attrs.alloc_once.is_some() {
            Some((1u64, format!(
                "`#[alloc \"once\"]` on {display} asserts the initialization pattern (JPL rule 3)"
            )))
        } else if jpl {
            Some((0u64, format!(
                "the `jpl` profile admits no allocation outside initialization (decision D114)"
            )))
        } else {
            None
        };
        if let Some((cap, who)) = alloc_cap {
            let cap_txt = if cap == 0 {
                "no allocation site may run per call".to_string()
            } else {
                format!("at most {cap} allocation site(s) run per call")
            };
            let mut seen = 0u64;
            for site in &facts.alloc_sites {
                if site.closure {
                    return Err(Error::at(
                        site.span,
                        format!(
                            "{who}: {} is constructed under a closure here: the site runs whenever the closure runs",
                            site.what
                        ),
                    ));
                }
                seen += 1;
                if seen > cap {
                    return Err(Error::at(
                        site.span,
                        format!(
                            "{who}: {} is constructed here{}: {cap_txt}",
                            site.what,
                            if seen > 1 { " as well" } else { "" }
                        ),
                    ));
                }
            }
        }

        // The `bounded(n)` depth proofs (decision D113): structural
        // first, the conservative call-graph bound otherwise. The group
        // claims check against the program's D104 groups later.
        let structural = structural_proof_for(eqs, doms, &facts, def_span);
        let call_graph = literal_ladder(eqs, &facts.self_calls).map(|b| {
            (
                format!("call-graph (the literal counter ladder: depth <= {b})"),
                Established::Num(b),
            )
        });

        let mut fact_strings = attrs.fact_strings();
        if jpl {
            fact_strings.push("profile=jpl".to_string());
        }
        fact_strings.sort();
        fact_strings.dedup();
        self.safety_records.push(SafetyRecord {
            key: key.to_string(),
            display,
            module: module.to_string(),
            attrs,
            facts,
            def_span,
            exported,
            logical,
            jpl,
            structural,
            call_graph,
            fact_strings: fact_strings.clone(),
        });
        if !fact_strings.is_empty() {
            self.safety_facts.insert(key.to_string(), fact_strings);
        }
        Ok(())
    }
}

// ============ temporal contracts (core specification section 10) ============

/// One verified temporal contract's manifest row (core specification
/// section 10): the definition and contract it attaches to, the kind and
/// formula, the verification result (which argument discharged it), and
/// the stated assumptions. Definition names, never paths.
#[derive(Clone, Debug)]
pub struct TemporalRow {
    /// The definition (`Module.name`).
    pub definition: String,
    /// The contract's name.
    pub contract: String,
    /// The temporal kind (`safety`, `liveness`, `recurrence`, `stability`).
    pub kind: String,
    /// The formula (`always (copyOut m < 5)`, `eventually (joined)`, ...).
    pub formula: String,
    /// The verification result (the proof method or discharge).
    pub result: String,
    /// The stated assumptions (the `assume` clauses).
    pub assumptions: Vec<String>,
}

/// The temporal contracts pending verification on the definition being
/// elaborated (core specification section 10): prepared before its body
/// checks (the parameter scope the formulas name) and verified after
/// constraint solving (the per-step write log's regions resolve there).
struct TemporalPending {
    module: String,
    name: String,
    /// The definition's global identity (the call discharge's self-check).
    id: GlobalId,
    contracts: Vec<ast::ContractDecl>,
    /// The definition's equations (the recurrence claim's cycle walk).
    eqs: Vec<ast::Equation>,
    /// The definition's parameter domains, in position order (the
    /// structural recursion proof reads them).
    doms: Vec<Ty>,
    /// The parameters the formulas may name: pattern name and domain
    /// type, in position order.
    params: Vec<(String, Ty)>,
    def_span: ast::Span,
}

/// A temporal formula's source rendering (the contract facts spell a
/// contract as written): comparisons and equality infix, arithmetic and
/// applications prefix, everything else structural.
fn render_formula_term(t: &ast::Term) -> String {
    fn arg(t: &ast::Term) -> String {
        match t {
            ast::Term::Var(_) | ast::Term::Con(_) | ast::Term::Int(..) => render_formula_term(t),
            // A prefix application binds tighter than the operators.
            ast::Term::App(f, _, _)
                if matches!(&**f, ast::Term::Var(p)
                    if p.quals.is_empty()
                        && !matches!(
                            p.name.text.as_str(),
                            "<" | "<=" | ">" | ">=" | "==" | "=" | "/=" | "!=" | "+" | "-" | "*"
                        )) =>
            {
                render_formula_term(t)
            }
            other => format!("({})", render_formula_term(other)),
        }
    }
    match t {
        ast::Term::Var(p) => {
            if p.quals.is_empty() {
                p.name.text.clone()
            } else {
                let mut s = p.quals.iter().map(|q| q.text.clone()).collect::<Vec<_>>().join(".");
                s.push('.');
                s.push_str(&p.name.text);
                s
            }
        }
        ast::Term::Con(p) => p.parts().join("."),
        ast::Term::Int(v, _) => v.clone(),
        ast::Term::Neg(e, _) => format!("-{}", arg(e)),
        ast::Term::Ann(e, _, _) => render_formula_term(e),
        ast::Term::App(f, args, _) => {
            if let ast::Term::Var(p) = &**f {
                if p.quals.is_empty()
                    && matches!(
                        p.name.text.as_str(),
                        "<" | "<=" | ">" | ">=" | "==" | "=" | "/=" | "!=" | "+" | "-" | "*"
                    )
                    && args.len() == 2
                {
                    return format!(
                        "{} {} {}",
                        arg(&args[0]),
                        p.name.text,
                        arg(&args[1])
                    );
                }
            }
            let mut s = render_formula_term(f);
            for a in args {
                s.push(' ');
                s.push_str(&arg(a));
            }
            s
        }
        other => format!("{other:?}"),
    }
}

impl Checker {
    /// Prepare the temporal contracts named on this definition (core
    /// specification section 10) before its body checks. The formulas'
    /// scope is the definition's parameters: the pattern variables of its
    /// equations, the same name at the same position in every equation a
    /// formula names (the dependent-parameter discipline).
    fn prepare_temporal(
        &mut self,
        module: &str,
        name: &str,
        eqs: &[ast::Equation],
        sig_tys: &[Ty],
    ) -> Result<()> {
        let key = (module.to_string(), name.to_string());
        let Some(contracts) = self.pending_contracts.remove(&key) else {
            self.cur_temporal = None;
            return Ok(());
        };
        let n_pats = eqs[0].pats.len();
        let mut params: Vec<(String, Ty)> = Vec::new();
        for (i, pat) in eqs[0].pats.iter().enumerate() {
            if let ast::Pat::Var(n0, _) = pat {
                params.push((n0.text.clone(), sig_tys[i].clone()));
            }
        }
        // Every name the contracts mention must be a parameter named the
        // same way in every equation (the values a step writes render
        // under the equation's own names).
        let mut mentioned: Vec<String> = Vec::new();
        for c in &contracts {
            for cl in &c.clauses {
                match cl {
                    ast::ContractClause::Formula { body, .. }
                    | ast::ContractClause::AssumeProp { body } => {
                        let mut refs = Vec::new();
                        ast::collect_term_refs(body, &mut refs);
                        for r in refs {
                            if r.quals.is_empty() && !mentioned.contains(&r.name) {
                                mentioned.push(r.name.clone());
                            }
                        }
                    }
                    ast::ContractClause::AssumeNamed { .. } => {}
                }
            }
        }
        for (i, pat) in eqs[0].pats.iter().enumerate() {
            let ast::Pat::Var(n0, _) = pat else {
                continue;
            };
            if !mentioned.contains(&n0.text) {
                continue;
            }
            for e in eqs {
                match e.pats.get(i) {
                    Some(ast::Pat::Var(n, _)) if n.text == n0.text => {}
                    _ => {
                        return Err(Error::at(
                            e.span,
                            format!(
                                "temporal contract: `{}` is named by a contract on `{name}`, so its position {i} must be a variable pattern with the same name in every equation (the equation at {} names it differently)",
                                n0.text, e.span
                            ),
                        ));
                    }
                }
            }
        }
        let mut doms: Vec<Ty> = sig_tys[..n_pats].to_vec();
        doms.iter_mut().for_each(|t| *t = self.uni.zonk(t));
        self.cur_temporal = Some(TemporalPending {
            module: module.to_string(),
            name: name.to_string(),
            id: fn_id(module, name),
            contracts,
            eqs: eqs.to_vec(),
            doms,
            params,
            def_span: eqs[0].span,
        });
        Ok(())
    }

    /// Verify the temporal contracts pending on the definition just
    /// elaborated (core specification section 10), now that the per-step
    /// write log's regions have resolved. A contract is verified where
    /// its claim is decidable and refused by name where it is not: never
    /// silently unchecked.
    fn check_temporal(&mut self, id: &GlobalId) -> Result<()> {
        let Some(pending) = self.cur_temporal.take() else {
            return Ok(());
        };
        debug_assert_eq!(&pending.id, id);
        for i in 0..pending.contracts.len() {
            self.check_one_temporal(&pending, i)?;
        }
        Ok(())
    }

    /// One contract's verification: the formula's index reading over the
    /// definition's parameters, then the kind's check.
    fn check_one_temporal(&mut self, pending: &TemporalPending, at: usize) -> Result<()> {
        let contract = &pending.contracts[at];
        // The parameters' scope for the formulas (the values' types come
        // from the signature).
        let mut env = Env::default();
        for (n, t) in &pending.params {
            let l = self.fresh_local();
            env.locals.push((n.clone(), l, Scheme::mono(t.clone())));
        }
        let mut formula: Option<&ast::Term> = None;
        let mut assumed: Vec<PTerm> = Vec::new();
        let mut assumption_rows: Vec<String> = Vec::new();
        for cl in &contract.clauses {
            match cl {
                ast::ContractClause::Formula { body, .. } => formula = Some(body),
                ast::ContractClause::AssumeProp { body } => {
                    let p = self.temporal_prop(&env, body, contract)?;
                    assumption_rows.push(format!("assume ({})", render_formula_term(body)));
                    assumed.push(p);
                }
                ast::ContractClause::AssumeNamed { text, .. } => {
                    assumption_rows.push(format!("assume \"{text}\""));
                }
            }
        }
        let fterm = formula.expect("the parser requires one formula per contract");
        let display = format!("{}.{}", pending.module, pending.name);
        if contract.kind == ast::ContractKind::Liveness {
            // A liveness claim names a finite obligation (`joined`,
            // `cleanup`) or is refused: it is not a state proposition.
            return self.check_temporal_liveness(
                pending,
                contract,
                fterm,
                &format!("{} ({})", contract.kind.operator(), render_formula_term(fterm)),
                &assumption_rows,
                &display,
            );
        }
        let prop = self.temporal_prop(&env, fterm, contract)?;
        let formula = format!("{} ({})", contract.kind.operator(), render_formula_term(fterm));
        match contract.kind {
            ast::ContractKind::Safety => self.check_temporal_safety(
                pending,
                contract,
                &env,
                &prop,
                &formula,
                &assumed,
                &assumption_rows,
                &display,
            ),
            ast::ContractKind::Stability => self.check_temporal_stability(
                pending,
                contract,
                &env,
                &prop,
                &formula,
                &assumption_rows,
                &display,
            ),
            ast::ContractKind::Recurrence => self.check_temporal_recurrence(
                pending,
                contract,
                &env,
                &prop,
                &formula,
                &assumption_rows,
                &display,
            ),
            ast::ContractKind::Liveness => unreachable!("handled above"),
        }
    }

    /// A temporal formula's index reading (core specification section
    /// 10): the proposition `P` of `always (P)` and friends, over the
    /// definition's parameters.
    fn temporal_prop(
        &self,
        env: &Env,
        term: &ast::Term,
        contract: &ast::ContractDecl,
    ) -> Result<PTerm> {
        let wrap = |e: Error| {
            Error::at(
                term.span(),
                format!(
                    "temporal contract `{}`: the formula states a comparison proposition (`a < b`, `Eq a b`, ...): {}",
                    contract.name.text, e.message
                ),
            )
        };
        let is_eq = matches!(term, ast::Term::App(f, _, _)
            if matches!(&**f, ast::Term::Con(p) if p.parts() == ["Eq"]));
        let (cmp, lhs, rhs) = self.decide_shape(term).map_err(wrap)?;
        let li = self.idx_of_term(&lhs, env).map_err(wrap)?;
        let ri = self.idx_of_term(&rhs, env).map_err(wrap)?;
        Ok(if is_eq {
            PTerm::Eq(Box::new(li), Box::new(ri))
        } else {
            PTerm::Cmp(cmp, Box::new(li), Box::new(ri))
        })
    }

    /// The state locations a temporal formula reads (the footprint of
    /// evidence invalidation over mutable state): `Mut`-sourced facts
    /// need the write discipline, `Ref`-sourced facts survive every
    /// transition by borrow exclusivity (core specification section 5).
    /// Regions resolve at check time.
    fn temporal_footprint(&self, prop: &PTerm, env: &Env) -> (Vec<String>, Vec<String>) {
        let (muts, refs) = self.ev_footprint(prop, env);
        let resolve = |rs: Vec<String>| {
            let mut out: Vec<String> = rs
                .iter()
                .map(|r| self.uni.resolve_region(r))
                .collect::<Vec<_>>();
            out.sort();
            out.dedup();
            out
        };
        (resolve(muts), resolve(refs))
    }

    /// The parameter name a written region is the borrow of (P5
    /// exclusivity gives at most one).
    fn temporal_subject(&self, pending: &TemporalPending, region: &str) -> Option<String> {
        let r = self.uni.resolve_region(region);
        for (n, t) in &pending.params {
            if let Ty::Mut(pr, _) = self.uni.zonk(t) {
                if self.uni.resolve_region(&pr) == r {
                    return Some(n.clone());
                }
            }
        }
        None
    }

    /// The inductive discharge of a call step (core specification section
    /// 10): a call to the definition itself that passes the invariant's
    /// variables through unchanged re-establishes exactly this invariant
    /// after it (the callee's contract is this contract).
    fn temporal_discharge(
        &self,
        pending: &TemporalPending,
        prop: &PTerm,
        callee: &GlobalId,
        args: &[Option<ITerm>],
    ) -> bool {
        if callee != &pending.id {
            return false;
        }
        let mut fv = Vec::new();
        prop.free_vars(&mut fv);
        for v in &fv {
            let Some(i) = pending.params.iter().position(|(n, _)| n == v) else {
                return false;
            };
            match args.get(i) {
                Some(Some(ITerm::Var(x))) if x == v => {}
                _ => return false,
            }
        }
        true
    }

    /// Record one verified contract as its interface fact and manifest
    /// row (the P20 contract-facts precedent).
    fn record_temporal_fact(
        &mut self,
        pending: &TemporalPending,
        contract: &ast::ContractDecl,
        formula: &str,
        result: String,
        assumptions: &[String],
    ) {
        let fact = crate::interface::ContractFact {
            name: pending.name.clone(),
            kind: contract.kind.keyword().to_string(),
            formula: formula.to_string(),
            assumptions: assumptions.to_vec(),
        };
        self.contract_facts
            .entry(pending.id.0.clone())
            .or_default()
            .push(fact);
        self.temporal_rows.push(TemporalRow {
            definition: format!("{}.{}", pending.module, pending.name),
            contract: contract.name.text.clone(),
            kind: contract.kind.keyword().to_string(),
            formula: formula.to_string(),
            result,
            assumptions: assumptions.to_vec(),
        });
    }

    /// `safety`: `always (P)` holds at every step of the definition's
    /// transitions (core specification section 10). The inductive
    /// invariant checks its initial state (the stated establishment) and
    /// every transition of the per-step write log: a step must leave P
    /// true (decided, self-preserving, or discharged by the definition's
    /// own recursion); a step that falsifies P is rejected naming the
    /// step; a step the checker cannot decide is refused, never trusted.
    #[allow(clippy::too_many_arguments)]
    fn check_temporal_safety(
        &mut self,
        pending: &TemporalPending,
        contract: &ast::ContractDecl,
        env: &Env,
        prop: &PTerm,
        formula: &str,
        assumed: &[PTerm],
        assumption_rows: &[String],
        display: &str,
    ) -> Result<()> {
        let contract_name = &contract.name.text;
        let (muts, refs) = self.temporal_footprint(prop, env);
        if muts.is_empty() && refs.is_empty() {
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal safety: the formula `{formula}` of the contract `{contract_name}` reads no borrowed state of `{display}`: an `always` claim is over the state transitions of core specification section 10 (name a `Mut` or `Ref` parameter)"
                ),
            ));
        }
        // The initial state (core specification section 10: an invariant
        // proof checks the initial state and each transition): the
        // contract states it as its establishment.
        if !assumed.iter().any(|a| PTerm::eq(a, prop)) {
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal safety: the contract `{contract_name}` on `{display}` has no establishment for `{formula}`: state the entry condition `assume = ({})` (core specification section 10: an inductive invariant checks its initial state)",
                    render_formula_term_of(formula)
                ),
            ));
        }
        if muts.is_empty() {
            // The invariant reads only read-only borrows: P5 exclusivity
            // lets no transition touch it.
            self.record_temporal_fact(
                pending,
                contract,
                formula,
                "verified (the invariant reads only read-only borrow state: no transition of the definition can falsify it)".to_string(),
                assumption_rows,
            );
            return Ok(());
        }
        let mut steps = 0u64;
        let mut discharged = 0u64;
        for w in &self.ev_writes {
            let hits = match &w.loc {
                None => true,
                Some(r) => {
                    let rr = self.uni.resolve_region(r);
                    muts.iter().any(|m| self.uni.resolve_region(m) == rr)
                }
            };
            if !hits {
                continue;
            }
            steps += 1;
            let refused = |q: String| {
                Error::at(
                    w.span,
                    format!(
                        "temporal safety: refused: the step at {span} {q}, and an obligation that cannot be established is refused, never trusted (core specification section 10)",
                        span = w.span
                    ),
                )
            };
            if w.loc.is_none() {
                return Err(refused(
                    "is an opaque write (an FFI / `RawPtr` write or `eval`) whose post-state is unknown".to_string(),
                ));
            }
            let subject = self
                .temporal_subject(pending, w.loc.as_ref().expect("checked above"));
            if let Some(v) = &w.value {
                let Some(subj) = subject else {
                    return Err(refused(
                        "writes state the invariant reads, but the written location names no parameter of the contract".to_string(),
                    ));
                };
                let q = prop.subst(&subj, v);
                match self.static_obligation(&q)? {
                    ObligStatus::Checked => continue,
                    ObligStatus::Disproved => {
                        return Err(Error::at(
                            w.span,
                            format!(
                                "temporal safety: the contract `{contract_name}` on `{display}` is falsified by the step at {}: the write leaves the state `{q}` false, so `{formula}` fails at this transition (core specification section 10)",
                                w.span
                            ),
                        ))
                    }
                    _ => {
                        if PTerm::eq(&q, prop) {
                            // The write restores the same state: the
                            // invariant is preserved.
                            continue;
                        }
                        return Err(refused(format!(
                            "leaves the state `{q}`, which the checker cannot decide"
                        )));
                    }
                }
            }
            if let Some((callee, args)) = &w.call {
                if self.temporal_discharge(pending, prop, callee, args) {
                    discharged += 1;
                    continue;
                }
                return Err(refused(
                    "is a call that may write the invariant's state and no contract discharges it (the checker discharges a step only from the definition's own recursion)".to_string(),
                ));
            }
            return Err(refused(
                "writes the invariant's state with no decidable post-state".to_string(),
            ));
        }
        self.record_temporal_fact(
            pending,
            contract,
            formula,
            format!(
                "verified (the invariant holds at every step: {steps} transition(s) of the state, {discharged} discharged by the inductive step)"
            ),
            assumption_rows,
        );
        Ok(())
    }

    /// `stability`: `stable (P)` survives the definition's transitions
    /// (core specification section 10: the dual of evidence invalidation).
    /// A fact survives exactly when no write touches its footprint (the
    /// evidence-invalidation rule of core specification section 5); a
    /// touching write kills it naming the step.
    fn check_temporal_stability(
        &mut self,
        pending: &TemporalPending,
        contract: &ast::ContractDecl,
        env: &Env,
        prop: &PTerm,
        formula: &str,
        assumption_rows: &[String],
        display: &str,
    ) -> Result<()> {
        let contract_name = &contract.name.text;
        let (muts, refs) = self.temporal_footprint(prop, env);
        if muts.is_empty() && refs.is_empty() {
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal stability: the formula `{formula}` of the contract `{contract_name}` reads no borrowed state of `{display}`: a stability claim is over a borrowed state's footprint (core specification section 10)"
                ),
            ));
        }
        if muts.is_empty() {
            // Immutable or read-only-borrow facts: nothing a transition
            // does can invalidate them (core specification section 5).
            self.record_temporal_fact(
                pending,
                contract,
                formula,
                if refs.is_empty() {
                    "verified (the fact reads no mutable state: no transition can invalidate it)".to_string()
                } else {
                    "verified (the fact reads only read-only borrow state: borrow exclusivity keeps it)".to_string()
                },
                assumption_rows,
            );
            return Ok(());
        }
        for w in &self.ev_writes {
            let hits = match &w.loc {
                None => true,
                Some(r) => {
                    let rr = self.uni.resolve_region(r);
                    muts.iter().any(|m| self.uni.resolve_region(m) == rr)
                }
            };
            if hits {
                return Err(Error::at(
                    w.span,
                    format!(
                        "temporal stability: the contract `{contract_name}` on `{display}` dies at the step at {}: {} touches the footprint of `{formula}`, so the fact would be invalidated there (evidence invalidation over mutable state; core specification sections 5 and 10)",
                        w.span,
                        if w.loc.is_none() { "an opaque write" } else { "the write" }
                    ),
                ));
            }
        }
        self.record_temporal_fact(
            pending,
            contract,
            formula,
            format!(
                "verified (no write touches the fact's footprint across the definition's {} transition(s))",
                self.ev_writes.len()
            ),
            assumption_rows,
        );
        Ok(())
    }

    /// `liveness`: `eventually (Q)` (core specification section 10). The
    /// honest scope is the finite structured-concurrency obligations: a
    /// join completes and a spawn's cleanup runs within the `withScope`
    /// bound (the P12 guarantees, concurrency specification section 2).
    /// An open-ended eventual claim is refused by name: it needs a
    /// well-founded progress argument and stated fairness assumptions.
    fn check_temporal_liveness(
        &mut self,
        pending: &TemporalPending,
        contract: &ast::ContractDecl,
        fterm: &ast::Term,
        formula: &str,
        assumption_rows: &[String],
        display: &str,
    ) -> Result<()> {
        let contract_name = &contract.name.text;
        let claim = match fterm {
            ast::Term::Var(p) if p.quals.is_empty() && p.name.text == "joined" => Some("joined"),
            ast::Term::Var(p) if p.quals.is_empty() && p.name.text == "cleanup" => Some("cleanup"),
            _ => None,
        };
        let Some(claim) = claim else {
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal liveness: refused: the contract `{contract_name}` on `{display}` claims `{formula}`, an unbounded eventual claim: it needs a well-founded progress argument and stated fairness assumptions, which this checker does not derive (core specification section 10): refused, never trusted. The finite structured-concurrency obligations `eventually = (joined)` and `eventually = (cleanup)` verify within their scope"
                ),
            ));
        };
        if assumption_rows.is_empty() {
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal liveness: the contract `{contract_name}` on `{display}` states no assumption: a liveness claim relies on the runtime's scheduling and cancellation policies and must state them (concurrency specification section 10; core specification section 10: temporal contracts state their assumptions when relied upon)"
                ),
            ));
        }
        let marks = self.foreign_marks();
        let facts = collect_body_facts(&pending.module, &pending.name, &pending.eqs, &marks);
        let has = |k: &str| facts.conc_sites.iter().any(|(k2, _)| k2 == k);
        if !has("scope") || !has("spawn") {
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal liveness: the contract `{contract_name}` on `{display}` has no subject: the bounded claim `{formula}` needs a structured `withScope` spawn/join in the definition (concurrency specification section 2)"
                ),
            ));
        }
        if claim == "joined" && !has("join") {
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal liveness: the contract `{contract_name}` on `{display}` finds no join: the claim `{formula}` needs a `joinAllOrCancel` in the definition's structured scope (concurrency specification section 2)"
                ),
            ));
        }
        let result = if claim == "joined" {
            "verified from the P12 structured-scope guarantee: every spawned handle is joined before the scope exits, so the join completes within the scope's bound (concurrency specification section 2)".to_string()
        } else {
            "verified from the P12 structured-scope guarantee: the scope's exit runs its spawned tasks' cleanup through the join/cancel discipline (concurrency specification section 2)".to_string()
        };
        self.record_temporal_fact(pending, contract, formula, result, assumption_rows);
        Ok(())
    }

    /// `recurrence`: `recur (P)` at every cycle of the definition's
    /// recursion (core specification section 10). The bound is the
    /// structural recursion (the D113 structural depth proof); the
    /// witness is a step re-establishing P on every cycle, found by a
    /// walk along evaluation order. A recursion with no structural bound
    /// is refused by name; an unwitnessed cycle is rejected naming the
    /// recursive call.
    fn check_temporal_recurrence(
        &mut self,
        pending: &TemporalPending,
        contract: &ast::ContractDecl,
        env: &Env,
        prop: &PTerm,
        formula: &str,
        assumption_rows: &[String],
        display: &str,
    ) -> Result<()> {
        let contract_name = &contract.name.text;
        let (muts, _refs) = self.temporal_footprint(prop, env);
        if muts.is_empty() {
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal recurrence: the formula `{formula}` of the contract `{contract_name}` reads no mutable state of `{display}`: a recurrence claim is over a `Mut` location its cycles re-establish (core specification section 10)"
                ),
            ));
        }
        let marks = self.foreign_marks();
        let facts = collect_body_facts(&pending.module, &pending.name, &pending.eqs, &marks);
        if facts.self_calls.is_empty() {
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal recurrence: the contract `{contract_name}` on `{display}` has no cycle: the definition does not recurse (core specification section 10: a recurrence claim is over the cycles of a recursion)"
                ),
            ));
        }
        let Some((report, _)) = structural_proof_for(
            &pending.eqs,
            &pending.doms,
            &facts,
            pending.def_span,
        ) else {
            let reason = if !facts.self_refs.is_empty() {
                "a bare recursive reference carries no descent"
            } else if facts.self_calls.iter().any(|c| c.closure) {
                "a recursive call in a hoisted closure has no static descent"
            } else {
                "no argument decreases structurally"
            };
            return Err(Error::at(
                contract.span,
                format!(
                    "temporal recurrence: refused: the recursion of `{display}` has no structural depth bound ({reason}), and a recurrence claim's bound comes from structural recursion (core specification sections 8 and 10): refused, never trusted"
                ),
            ));
        };
        // The steps that re-establish the claim's state: a write to the
        // footprint whose post-state is P (decided or restored).
        let mut establishes: Vec<ast::Span> = Vec::new();
        for w in &self.ev_writes {
            let Some(loc) = &w.loc else { continue };
            let rr = self.uni.resolve_region(loc);
            if !muts.iter().any(|m| self.uni.resolve_region(m) == rr) {
                continue;
            }
            let (Some(v), Some(subj)) = (&w.value, self.temporal_subject(pending, loc)) else {
                continue;
            };
            let q = prop.subst(&subj, v);
            if PTerm::eq(&q, prop) || self.static_obligation(&q)? == ObligStatus::Checked {
                establishes.push(w.span);
            }
        }
        let calls: Vec<ast::Span> = facts.self_calls.iter().map(|c| c.span).collect();
        for e in &pending.eqs {
            self.recur_walk(&e.body, false, &calls, &establishes, contract_name, formula, display)?;
        }
        self.record_temporal_fact(
            pending,
            contract,
            formula,
            format!(
                "verified (the state recurs at every cycle of the structural recursion: {report})"
            ),
            assumption_rows,
        );
        Ok(())
    }

    /// The recurrence claim's cycle walk (core specification section 10):
    /// every recursive call closes a cycle on which the claim's state must
    /// be re-established. The walk threads `established` in evaluation
    /// order (call by value); a branch joins its arms conservatively (a
    /// call after the branch needs the witness on every path), a lambda
    /// or local function body starts fresh (its calls run on their own
    /// cycles), and a recursive call resets the flag (each segment
    /// between calls re-establishes the state).
    fn recur_walk(
        &self,
        t: &ast::Term,
        established: bool,
        calls: &[ast::Span],
        establishes: &[ast::Span],
        contract_name: &str,
        formula: &str,
        display: &str,
    ) -> Result<bool> {
        let mut flag = established;
        match t {
            ast::Term::Int(..)
            | ast::Term::Str(..)
            | ast::Term::Char(..)
            | ast::Term::Sym(..)
            | ast::Term::Unit(_)
            | ast::Term::Hole(_)
            | ast::Term::SpliceHole { .. }
            | ast::Term::SeqOp(..) => {}
            ast::Term::Var(_) | ast::Term::Con(_) => {}
            ast::Term::Neg(e, _) | ast::Term::Ann(e, _, _) | ast::Term::Proj(e, _, _) => {
                flag = self.recur_walk(e, flag, calls, establishes, contract_name, formula, display)?;
            }
            ast::Term::Forget { value, .. } => {
                flag = self.recur_walk(value, flag, calls, establishes, contract_name, formula, display)?;
            }
            ast::Term::Narrow { value, witness, .. } => {
                flag = self.recur_walk(value, flag, calls, establishes, contract_name, formula, display)?;
                flag = self.recur_walk(witness, flag, calls, establishes, contract_name, formula, display)?;
            }
            ast::Term::App(f, args, _) => {
                flag = self.recur_walk(f, flag, calls, establishes, contract_name, formula, display)?;
                for a in args {
                    flag = self.recur_walk(a, flag, calls, establishes, contract_name, formula, display)?;
                }
                if let ast::Term::Var(p) = &**f {
                    if calls.contains(&p.span()) {
                        if !flag {
                            return Err(Error::at(
                                p.span(),
                                format!(
                                    "temporal recurrence: the contract `{contract_name}` on `{display}` is not witnessed at the recursive call at {}: no step of the cycle re-establishes `{formula}` (core specification section 10: a state recurs at every cycle)",
                                    p.span()
                                ),
                            ));
                        }
                        flag = false;
                    }
                }
            }
            ast::Term::Infix(ts, _, _) | ast::Term::Tuple(ts, _) | ast::Term::List(ts, _) => {
                for x in ts {
                    flag = self.recur_walk(x, flag, calls, establishes, contract_name, formula, display)?;
                }
            }
            ast::Term::Map(kvs, _) => {
                for (k, v) in kvs {
                    flag = self.recur_walk(k, flag, calls, establishes, contract_name, formula, display)?;
                    flag = self.recur_walk(v, flag, calls, establishes, contract_name, formula, display)?;
                }
            }
            ast::Term::Range(start, step, end, _) => {
                flag = self.recur_walk(start, flag, calls, establishes, contract_name, formula, display)?;
                if let Some(x) = step {
                    flag = self.recur_walk(x, flag, calls, establishes, contract_name, formula, display)?;
                }
                flag = self.recur_walk(end, flag, calls, establishes, contract_name, formula, display)?;
            }
            ast::Term::Record(_, fields, _) => {
                for (_, e) in fields {
                    flag = self.recur_walk(e, flag, calls, establishes, contract_name, formula, display)?;
                }
            }
            ast::Term::If { cond, then, els, .. } => {
                flag = self.recur_walk(cond, flag, calls, establishes, contract_name, formula, display)?;
                let a = self.recur_walk(then, flag, calls, establishes, contract_name, formula, display)?;
                let b = self.recur_walk(els, flag, calls, establishes, contract_name, formula, display)?;
                flag = a && b;
            }
            ast::Term::Case(s, alts, _) => {
                flag = self.recur_walk(s, flag, calls, establishes, contract_name, formula, display)?;
                let mut join = true;
                for alt in alts {
                    let mut arm = flag;
                    if let Some(g) = &alt.guard {
                        arm = self.recur_walk(g, arm, calls, establishes, contract_name, formula, display)?;
                    }
                    arm = self.recur_walk(&alt.body, arm, calls, establishes, contract_name, formula, display)?;
                    join = join && arm;
                }
                flag = join;
            }
            ast::Term::Lam(_, body, _) => {
                // The body runs on its own cycles: a fresh segment.
                let _ = self.recur_walk(body, false, calls, establishes, contract_name, formula, display)?;
            }
            ast::Term::Let(decls, body, _) => {
                for d in decls {
                    flag = self.recur_walk_local(d, flag, calls, establishes, contract_name, formula, display)?;
                }
                flag = self.recur_walk(body, flag, calls, establishes, contract_name, formula, display)?;
            }
            ast::Term::Letrec(decls, body, _) => {
                for d in decls {
                    // Fixpoint boxes evaluate when forced or called:
                    // their bodies run on their own cycles.
                    if let ast::LocalDecl::Eq { body, .. }
                    | ast::LocalDecl::Bind { body, .. }
                    | ast::LocalDecl::Lazy { body, .. } = d
                    {
                        let _ = self.recur_walk(body, false, calls, establishes, contract_name, formula, display)?;
                    }
                }
                flag = self.recur_walk(body, flag, calls, establishes, contract_name, formula, display)?;
            }
            ast::Term::Do(stmts, _) => {
                for s in stmts {
                    match s {
                        ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                            flag = self.recur_walk(term, flag, calls, establishes, contract_name, formula, display)?;
                        }
                        ast::DoStmt::Let { decls, .. } => {
                            for decl in decls {
                                flag = self.recur_walk_local(decl, flag, calls, establishes, contract_name, formula, display)?;
                            }
                        }
                    }
                }
            }
            ast::Term::Handle(e, h, _) => {
                flag = self.recur_walk(e, flag, calls, establishes, contract_name, formula, display)?;
                // The handler value is a construct, not this cycle's
                // control flow: its clause bodies run on their own.
                let _ = self.recur_walk(h, false, calls, establishes, contract_name, formula, display)?;
            }
            ast::Term::Handler(h) => {
                let _ = self.recur_walk(&h.ret_body, false, calls, establishes, contract_name, formula, display)?;
                for c in &h.clauses {
                    let _ = self.recur_walk(&c.body, false, calls, establishes, contract_name, formula, display)?;
                }
            }
            ast::Term::With { value, body, .. } => {
                flag = self.recur_walk(value, flag, calls, establishes, contract_name, formula, display)?;
                flag = self.recur_walk(body, flag, calls, establishes, contract_name, formula, display)?;
            }
            ast::Term::CapApply { head, supplies, .. } => {
                flag = self.recur_walk(head, flag, calls, establishes, contract_name, formula, display)?;
                for (_, e) in supplies {
                    flag = self.recur_walk(e, flag, calls, establishes, contract_name, formula, display)?;
                }
            }
            ast::Term::CpsWrap(e, _) => {
                flag = self.recur_walk(e, flag, calls, establishes, contract_name, formula, display)?;
            }
            // Staging forms are not ordinary control flow: a recursive
            // call under one is refused rather than trusted.
            ast::Term::Quote { span, .. }
            | ast::Term::QuoteDecls { span, .. }
            | ast::Term::Splice { span, .. }
            | ast::Term::MacroCall { span, .. } => {
                if self.term_has_span_in(t, calls) {
                    return Err(Error::at(
                        *span,
                        format!(
                            "temporal recurrence: refused: a recursive call of `{}` sits inside a staging construct, where the cycle walk does not reach: refused, never trusted (core specification section 10)",
                            contract_name
                        ),
                    ));
                }
            }
        }
        if establishes.contains(&t.span()) {
            flag = true;
        }
        Ok(flag)
    }

    /// The recurrence walk over one `let` binding: an eager binding runs
    /// in sequence, a lazy or function binding on its own cycles.
    fn recur_walk_local(
        &self,
        d: &ast::LocalDecl,
        established: bool,
        calls: &[ast::Span],
        establishes: &[ast::Span],
        contract_name: &str,
        formula: &str,
        display: &str,
    ) -> Result<bool> {
        match d {
            ast::LocalDecl::Bind { body, .. } => {
                self.recur_walk(body, established, calls, establishes, contract_name, formula, display)
            }
            ast::LocalDecl::Eq { pats, body, .. } if pats.is_empty() => {
                // A value binding: the body runs now.
                self.recur_walk(body, established, calls, establishes, contract_name, formula, display)
            }
            ast::LocalDecl::Eq { body, .. } | ast::LocalDecl::Lazy { body, .. } => {
                let _ = self.recur_walk(body, false, calls, establishes, contract_name, formula, display)?;
                Ok(established)
            }
            ast::LocalDecl::Sig { .. } => Ok(established),
        }
    }

    /// Does any subterm of `t` carry one of the given spans?
    fn term_has_span_in(&self, t: &ast::Term, spans: &[ast::Span]) -> bool {
        let mut found = false;
        let mut check = |s: &ast::Span| {
            if spans.contains(s) {
                found = true;
            }
        };
        self.for_each_span(t, &mut check);
        found
    }

    /// Visit every span of a term (the staging-construction guard).
    fn for_each_span(&self, t: &ast::Term, f: &mut dyn FnMut(&ast::Span)) {
        match t {
            ast::Term::Var(p) => f(&p.span()),
            ast::Term::Con(p) => f(&p.span()),
            ast::Term::Int(_, s)
            | ast::Term::Str(_, s)
            | ast::Term::Char(_, s) | ast::Term::Sym(_, s)
            | ast::Term::Unit(s)
            | ast::Term::Hole(s)
            | ast::Term::SeqOp(_, s)
            | ast::Term::SpliceHole { span: s, .. }
            | ast::Term::Neg(_, s)
            | ast::Term::CpsWrap(_, s) => f(s),
            ast::Term::App(g, args, s) => {
                f(s);
                self.for_each_span(g, f);
                for a in args {
                    self.for_each_span(a, f);
                }
            }
            ast::Term::Lam(_, b, s) | ast::Term::Quote { body: b, span: s, .. } => {
                f(s);
                self.for_each_span(b, f);
            }
            ast::Term::Let(ds, b, s) | ast::Term::Letrec(ds, b, s) => {
                f(s);
                for d in ds {
                    if let ast::LocalDecl::Eq { body, .. }
                    | ast::LocalDecl::Bind { body, .. }
                    | ast::LocalDecl::Lazy { body, .. } = d
                    {
                        self.for_each_span(body, f);
                    }
                }
                self.for_each_span(b, f);
            }
            ast::Term::If { cond, then, els, .. } => {
                self.for_each_span(cond, f);
                self.for_each_span(then, f);
                self.for_each_span(els, f);
            }
            ast::Term::Case(g, alts, s) => {
                f(s);
                self.for_each_span(g, f);
                for a in alts {
                    self.for_each_span(&a.body, f);
                }
            }
            ast::Term::Ann(e, _, _) | ast::Term::Proj(e, _, _) => self.for_each_span(e, f),
            ast::Term::Forget { value, .. } => self.for_each_span(value, f),
            ast::Term::Narrow { value, witness, .. } => {
                self.for_each_span(value, f);
                self.for_each_span(witness, f);
            }
            ast::Term::Infix(ts, _, s) | ast::Term::Tuple(ts, s) | ast::Term::List(ts, s) => {
                f(s);
                for x in ts {
                    self.for_each_span(x, f);
                }
            }
            ast::Term::Map(kvs, s) => {
                f(s);
                for (k, v) in kvs {
                    self.for_each_span(k, f);
                    self.for_each_span(v, f);
                }
            }
            ast::Term::Range(start, step, end, s) => {
                f(s);
                self.for_each_span(start, f);
                if let Some(x) = step {
                    self.for_each_span(x, f);
                }
                self.for_each_span(end, f);
            }
            ast::Term::Record(p, fields, s) => {
                f(s);
                f(&p.span());
                for (_, e) in fields {
                    self.for_each_span(e, f);
                }
            }
            ast::Term::Do(stmts, s) => {
                f(s);
                for st in stmts {
                    match st {
                        ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                            self.for_each_span(term, f)
                        }
                        ast::DoStmt::Let { decls, .. } => {
                            for decl in decls {
                                if let ast::LocalDecl::Eq { body, .. }
                                | ast::LocalDecl::Bind { body, .. }
                                | ast::LocalDecl::Lazy { body, .. } = decl
                                {
                                    self.for_each_span(body, f);
                                }
                            }
                        }
                    }
                }
            }
            ast::Term::Handle(e, h, s) => {
                f(s);
                self.for_each_span(e, f);
                self.for_each_span(h, f);
            }
            ast::Term::Handler(h) => {
                self.for_each_span(&h.ret_body, f);
                for c in &h.clauses {
                    self.for_each_span(&c.body, f);
                }
            }
            ast::Term::With { value, body, .. } => {
                self.for_each_span(value, f);
                self.for_each_span(body, f);
            }
            ast::Term::CapApply { head, supplies, .. } => {
                self.for_each_span(head, f);
                for (_, e) in supplies {
                    self.for_each_span(e, f);
                }
            }
            ast::Term::QuoteDecls { decls, span } => {
                f(span);
                for d in decls {
                    self.for_each_span_decl(d, f);
                }
            }
            ast::Term::Splice { body, span } => {
                f(span);
                self.for_each_span(body, f);
            }
            ast::Term::MacroCall { span, .. } => {
                f(span);
            }
        }
    }

    /// Visit every span of a staged declaration (the staging guard).
    fn for_each_span_decl(&self, d: &ast::Decl, f: &mut dyn FnMut(&ast::Span)) {
        match d {
            ast::Decl::Contract(c) => {
                f(&c.span);
                for cl in &c.clauses {
                    match cl {
                        ast::ContractClause::Formula { body, .. }
                        | ast::ContractClause::AssumeProp { body } => self.for_each_span(body, f),
                        ast::ContractClause::AssumeNamed { span, .. } => f(span),
                    }
                }
            }
            ast::Decl::Attributed { inner, .. } => self.for_each_span_decl(inner, f),
            _ => {}
        }
    }

    /// A temporal contract must name a definition of its own module
    /// (core specification section 10): the leftovers of the collection
    /// table name none the checker elaborates.
    fn check_unconsumed_contracts(&self) -> Result<()> {
        for ((module, target), contracts) in &self.pending_contracts {
            let c = &contracts[0];
            return Err(Error::at(
                c.span,
                format!(
                    "the temporal contract `{}` names no definition `{module}.{target}`: a contract attaches to a definition of its own module that the checker elaborates (core specification section 10)",
                    c.name.text
                ),
            ));
        }
        Ok(())
    }
}

/// The formula term of a rendered `always (...)` formula (the
/// establishment error spells the entry condition with it).
fn render_formula_term_of(formula: &str) -> &str {
    formula
        .split_once(" (")
        .and_then(|(_, rest)| rest.strip_suffix(')'))
        .unwrap_or(formula)
}

/// Re-state the verified P20 assertion rows as the LLVM contract
/// surface's facts on the definitions they belong to (the plan note
/// `design/llvm-contract-attributes.md`; decision D112's rows are the
/// input). The rows reach this point only after the group verification
/// accepted them, so a fact is recorded exactly when it is proved.
fn contract_facts(records: &[SafetyRecord], program: &mut Program) {
    for rec in records {
        let subs = format!("{}.", rec.key);
        for f in program.fns.iter_mut() {
            if f.id.0 != rec.key && !f.id.0.starts_with(&subs) {
                continue;
            }
            for fact in &rec.fact_strings {
                match fact.as_str() {
                    "no-alloc" => f.meta.contract.no_alloc = true,
                    "checked-params" if f.id.0 == rec.key => {
                        f.meta.contract.checked_params = true
                    }
                    other
                        if f.id.0 == rec.key
                            && (other.starts_with("recursion=bounded(")
                                || other == "recursion=none") =>
                    {
                        // The totality/`bounded` machinery proved the
                        // self-call chains are finite (`willreturn` is
                        // never guessed).
                        f.meta.contract.returns = true
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Verify the group-level safety claims against the checked program's
/// D104 tail-call groups (attribute annotations specification section
/// 10; decisions D112-D114): `#[recursion]` and `#[stack]` cover self
/// and mutual recursion within the definition's recursive group (the
/// `core::tail_report` marks are the source of truth), and the `jpl`
/// profile requires every recursion to establish a static bound.
pub(crate) fn verify_safety_groups(
    records: &[SafetyRecord],
    program: &Program,
) -> Result<SafetyReport> {
    let groups = crate::core::tail_report(program);
    let mut report = SafetyReport::default();
    let mut jpl_defs = 0u64;
    let mut jpl_asserts = 0u64;
    let mut jpl_modules: Vec<String> = Vec::new();
    for rec in records {
        if rec.jpl {
            jpl_defs += 1;
            if !jpl_modules.contains(&rec.module) {
                jpl_modules.push(rec.module.clone());
            }
        }
        let unit = rec.key.as_str();
        let group = groups.iter().find(|g| g.members.iter().any(|m| m == unit));
        let mut multi = false;
        let mut members: std::collections::BTreeSet<String> =
            std::collections::BTreeSet::new();
        members.insert(unit.to_string());
        if let Some(g) = group {
            for m in &g.members {
                members.insert(m.clone());
            }
            multi = g.members.len() > 1;
        }
        // The definition's units: its own and its hoisted closures.
        let prefix = format!("{unit}.");
        let mut subs: Vec<String> = Vec::new();
        for f in &program.fns {
            if f.id.0.starts_with(&prefix) {
                subs.push(f.id.0.clone());
            }
        }
        let hidden_call = rec.facts.self_calls.iter().find(|c| c.closure);
        let value_ref = rec.facts.self_refs.first();
        let recursive = multi
            || hidden_call.is_some()
            || value_ref.is_some()
            || !rec.facts.self_calls.is_empty()
            || group.is_some();

        // The span naming the offending call to a group member.
        let member_span = |want_tail: Option<bool>| -> Option<ast::Span> {
            for c in &rec.facts.calls {
                let hit = members
                    .iter()
                    .any(|m| m == &c.callee || m.ends_with(&format!(".{}", c.callee)));
                if !hit {
                    continue;
                }
                match want_tail {
                    Some(true) if !c.tail => continue,
                    Some(false) if c.tail => continue,
                    _ => {}
                }
                return Some(c.span);
            }
            None
        };

        // The tail claim: every recursive call is a jump or `musttail`
        // call (D104's sense).
        let tail_check = |aspan: ast::Span, who: &str| -> Result<()> {
            if let Some(c) = hidden_call {
                return Err(Error::at(
                    c.span,
                    format!(
                        "{who}: the recursive call at this site runs in a hoisted closure, so it is not a tail call of {display} (its frame separates the call from the definition's; decision D104)",
                        display = rec.display
                    ),
                ));
            }
            if let Some(s) = value_ref {
                return Err(Error::at(
                    *s,
                    format!(
                        "{who}: {display} refers to itself as a value at this site: a recursive use that is not a static call carries no tail claim",
                        display = rec.display
                    ),
                ));
            }
            if let Some(g) = group {
                if g.mark != "marked" {
                    return Err(Error::at(
                        aspan,
                        format!(
                            "{who}: the recursive group {:?} is not tail-marked ({}); its calls do not all lower to jumps or `musttail` calls",
                            g.members, g.mark
                        ),
                    ));
                }
            }
            let mut called: std::collections::BTreeSet<String> = members.clone();
            called.insert(unit.to_string());
            for site in crate::core::calls_to(program, unit, &called) {
                if !site.tail {
                    let span = member_span(Some(false)).unwrap_or(aspan);
                    return Err(Error::at(
                        span,
                        format!(
                            "{who}: the call to `{}` at this site is not in tail position",
                            site.callee
                        ),
                    ));
                }
            }
            for su in &subs {
                for site in crate::core::calls_to(program, su, &called) {
                    let span = member_span(Some(false)).or(member_span(None)).unwrap_or(aspan);
                    return Err(Error::at(
                        span,
                        format!(
                            "{who}: the call to `{}` at this site runs in the hoisted closure `{su}`, not in {display} itself",
                            site.callee,
                            display = rec.display
                        ),
                    ));
                }
            }
            Ok(())
        };

        // The `none` claim: no recursive calls at all.
        let none_check = |aspan: ast::Span| -> Result<()> {
            if let Some(c) = rec.facts.self_calls.first() {
                return Err(Error::at(
                    c.span,
                    format!(
                        "`#[recursion \"none\"]` on {} asserts the no-recursion rule (JPL rule 1): the recursive call at this site (self or mutual)",
                        rec.display
                    ),
                ));
            }
            if let Some(s) = value_ref {
                return Err(Error::at(
                    *s,
                    format!(
                        "`#[recursion \"none\"]` on {} asserts the no-recursion rule (JPL rule 1): the recursive use of the definition at this site",
                        rec.display
                    ),
                ));
            }
            if multi {
                let span = member_span(None).unwrap_or(aspan);
                return Err(Error::at(
                    span,
                    format!(
                        "`#[recursion \"none\"]` on {} asserts the no-recursion rule (JPL rule 1): the definition is in the recursive group {:?} (a mutual recursive call is at this site)",
                        rec.display,
                        group.map(|g| g.members.clone()).unwrap_or_default()
                    ),
                ));
            }
            Ok(())
        };

        let mut rows: Vec<(String, String)> = Vec::new();

        // -- `#[recursion ...]` (JPL rule 1).
        match &rec.attrs.recursion {
            Some((RecursionSpec::None, span)) => {
                none_check(*span)?;
                rows.push((
                    "recursion=none".to_string(),
                    "verified (no recursive calls)".to_string(),
                ));
            }
            Some((RecursionSpec::Tail, span)) => {
                let who = format!(
                    "`#[recursion \"tail\"]` on {} asserts the tail-recursion rule (JPL rule 1: constant stack)",
                    rec.display
                );
                tail_check(*span, &who)?;
                rows.push((
                    "recursion=tail".to_string(),
                    "verified (D104 tail marks)".to_string(),
                ));
            }
            Some((RecursionSpec::Bounded(arg), span)) => {
                if multi {
                    return Err(Error::at(
                        *span,
                        format!(
                            "`#[recursion \"bounded({arg})\"]` on {}: the recursive group {:?} is mutual: no depth bound is established for a mutual cycle (bounds that cannot be established are refused, never trusted; decision D113)",
                            rec.display,
                            group.map(|g| g.members.clone()).unwrap_or_default()
                        ),
                    ));
                }
                if let Some(c) = hidden_call {
                    return Err(Error::at(
                        c.span,
                        format!(
                            "`#[recursion \"bounded({arg})\"]` on {}: the recursion goes through a hoisted closure at this site: no depth bound is established (decision D113)",
                            rec.display
                        ),
                    ));
                }
                if let Some(s) = value_ref {
                    return Err(Error::at(
                        *s,
                        format!(
                            "`#[recursion \"bounded({arg})\"]` on {}: the recursion goes through a value reference at this site: no depth bound is established (decision D113)",
                            rec.display
                        ),
                    ));
                }
                let (proof, est) = bounded_proof(rec).map_err(|_| {
                    Error::at(
                        *span,
                        format!(
                            "`#[recursion \"bounded({arg})\"]` on {}: no depth bound is established for this recursion (the structural proof does not apply and no conservative call-graph bound applies); a bound that cannot be established is refused, never trusted (attribute annotations specification section 10; decision D113)",
                            rec.display
                        ),
                    )
                })?;
                match (arg, &est) {
                    (BoundArg::Num(m), Established::Num(b)) if *b <= *m => {}
                    (BoundArg::Name(n), Established::Named(ns)) if ns.contains(n) => {}
                    _ => {
                        return Err(Error::at(
                            *span,
                            format!(
                                "`#[recursion \"bounded({arg})\"]` on {}: the established bound is {proof}, which does not bound the depth by {arg} (bounds that cannot be established are refused, never trusted; decision D113)",
                                rec.display
                            ),
                        ));
                    }
                }
                rows.push((format!("recursion=bounded({arg})"), proof));
            }
            None => {}
        }

        // -- `#[stack ...]` (rule 1's stack companion).
        match &rec.attrs.stack {
            Some((StackSpec::Constant, span)) => {
                let who = format!(
                    "`#[stack \"constant\"]` on {} asserts constant stack (JPL rule 1: tail or non-recursive)",
                    rec.display
                );
                none_check(*span).or_else(|_| tail_check(*span, &who))?;
                rows.push(("stack=constant".to_string(), "verified".to_string()));
            }
            Some((StackSpec::Bounded(arg), span)) => {
                if multi || hidden_call.is_some() || value_ref.is_some() {
                    return Err(Error::at(
                        *span,
                        format!(
                            "`#[stack \"bounded({arg})\"]` on {}: no stack bound is established for this recursion (bounds that cannot be established are refused, never trusted; decision D113)",
                            rec.display
                        ),
                    ));
                }
                let (proof, est) = bounded_proof(rec).map_err(|_| {
                    Error::at(
                        *span,
                        format!(
                            "`#[stack \"bounded({arg})\"]` on {}: no stack bound is established for this recursion (the structural proof does not apply and no conservative call-graph bound applies); a bound that cannot be established is refused, never trusted (attribute annotations specification section 10; decision D113)",
                            rec.display
                        ),
                    )
                })?;
                match (arg, &est) {
                    (BoundArg::Num(m), Established::Num(b)) if *b <= *m => {}
                    (BoundArg::Name(n), Established::Named(ns)) if ns.contains(n) => {}
                    _ => {
                        return Err(Error::at(
                            *span,
                            format!(
                                "`#[stack \"bounded({arg})\"]` on {}: the established bound is {proof}, which does not bound the stack by {arg} (decision D113)",
                                rec.display
                            ),
                        ));
                    }
                }
                rows.push((format!("stack=bounded({arg})"), proof));
            }
            None => {}
        }

        // -- The `jpl` profile's recursion rule (decision D114): every
        // definition establishes a static bound (the mandated set at the
        // rule numbers; per-definition attributes may be stricter).
        if rec.jpl && rec.attrs.recursion.is_none() && rec.attrs.stack.is_none() {
            if recursive {
                let bound = if multi || hidden_call.is_some() || value_ref.is_some() {
                    None
                } else {
                    bounded_proof(rec).ok()
                };
                match bound {
                    Some((proof, _)) => rows.push(("jpl recursion".to_string(), proof)),
                    None => {
                        let tail_ok = tail_check(rec.def_span, "jpl").is_ok();
                        if !tail_ok {
                            return Err(Error::at(
                                rec.def_span,
                                format!(
                                    "the `jpl` profile requires a static recursion bound for every definition (decision D114): {} establishes none (its recursion is neither tail nor structurally or conservatively bounded)",
                                    rec.display
                                ),
                            ));
                        }
                        rows.push((
                            "jpl recursion".to_string(),
                            "tail (constant stack)".to_string(),
                        ));
                    }
                }
            } else {
                rows.push((
                    "jpl recursion".to_string(),
                    "verified (no recursive calls)".to_string(),
                ));
            }
        }

        // -- The remaining per-definition checks (verified at
        // `verify_definition_safety`): each satisfied attribute reports
        // its result. The group claims above already report theirs.
        for f in &rec.fact_strings {
            let result = if f == "no-alloc" {
                format!(
                    "verified ({} allocation site(s) in the body)",
                    rec.facts.alloc_sites.len()
                )
            } else if f == "alloc=once" {
                format!(
                    "verified ({} allocation site(s) run per call)",
                    rec.facts.alloc_sites.len()
                )
            } else if let Some(n) = f.strip_prefix("complexity<=") {
                format!(
                    "verified (cyclomatic complexity {} <= {n})",
                    rec.facts.branches + 1
                )
            } else if let Some(n) = f.strip_prefix("lines<=") {
                format!("verified ({} source line(s) <= {n})", rec.facts.body_lines)
            } else if let Some(n) = f.strip_prefix("assert-min=") {
                format!(
                    "verified ({} runtime `decide` check(s) >= {n})",
                    rec.facts.runtime_decides.len()
                )
            } else if f == "checked-params" {
                "verified (every parameter validated before first use)".to_string()
            } else if f == "no-macros" {
                "verified (no macro expansion inside the definition)".to_string()
            } else if f == "no-raw-ptr" {
                "verified (no raw pointer or opaque type in the signature, no foreign pointer operation in the body)".to_string()
            } else if f == "deny-warnings" {
                "verified (warnings in scope are compile errors)".to_string()
            } else {
                // The group claims and the profile rows report their own
                // results.
                continue;
            };
            rows.push((f.clone(), result));
        }

        // -- The `jpl` profile's mandated set at the rule numbers
        // (decision D114): rules a stricter per-definition attribute does
        // not already cover report their result under the profile.
        if rec.jpl {
            if rec.attrs.max_complexity.is_none() {
                rows.push((
                    format!("jpl complexity<={JPL_COMPLEXITY}"),
                    format!("verified (cyclomatic complexity {})", rec.facts.branches + 1),
                ));
            }
            if rec.attrs.max_lines.is_none() {
                rows.push((
                    format!("jpl lines<={JPL_LINES}"),
                    format!("verified ({} source lines)", rec.facts.body_lines),
                ));
            }
            if rec.exported && rec.attrs.assert_min.is_none() {
                rows.push((
                    format!("jpl assert-min={JPL_ASSERT_MIN}"),
                    format!(
                        "verified ({} runtime `decide` checks)",
                        rec.facts.runtime_decides.len()
                    ),
                ));
            }
            if rec.attrs.alloc_once.is_none() && rec.attrs.no_alloc.is_none() {
                rows.push((
                    "jpl alloc=none".to_string(),
                    format!(
                        "verified ({} allocation site(s))",
                        rec.facts.alloc_sites.len()
                    ),
                ));
            }
            if rec.attrs.no_macros.is_none() {
                rows.push((
                    "jpl no-macros".to_string(),
                    "verified (no macro expansion inside the definition)".to_string(),
                ));
            }
            if rec.attrs.no_raw_ptr.is_none() {
                rows.push((
                    "jpl no-raw-ptr".to_string(),
                    "verified (no raw pointer or opaque type in the signature, no foreign pointer operation in the body)".to_string(),
                ));
            }
            if rec.attrs.deny_warnings.is_none() {
                rows.push((
                    "jpl deny-warnings".to_string(),
                    "verified (warnings in scope are compile errors)".to_string(),
                ));
            }
            jpl_asserts += rows.len() as u64;
        }

        for (attribute, result) in rows {
            report.assertions.push(SafetyAssertion {
                definition: rec.display.clone(),
                attribute,
                result,
            });
        }
    }
    if !jpl_modules.is_empty() {
        jpl_modules.sort();
        report.jpl = Some(JplSummary {
            modules: jpl_modules,
            definitions: jpl_defs,
            assertions: jpl_asserts,
        });
    }
    Ok(report)
}

/// Does pattern `a` match everything pattern `b` matches (a sufficient
/// condition for a row to shadow a later one)?
fn pats_cover(a: &ast::Pat, b: &ast::Pat) -> bool {
    use ast::Pat;
    match (a, b) {
        (Pat::Var(..), _) | (Pat::Wild(_), _) => true,
        (Pat::As(_, inner, _), _) => pats_cover(inner, b),
        (_, Pat::As(_, inner, _)) => pats_cover(a, inner),
        (Pat::Unit(_), Pat::Unit(_)) => true,
        (Pat::Int(x, _), Pat::Int(y, _)) => x == y,
        (Pat::Con(pa, aa, _), Pat::Con(pb, ab, _)) => {
            pa.parts() == pb.parts()
                && aa.len() == ab.len()
                && aa.iter().zip(ab.iter()).all(|(x, y)| pats_cover(x, y))
        }
        (Pat::Record(pa, fa, _), Pat::Record(pb, fb, _)) => {
            pa.parts() == pb.parts()
                && fa.len() == fb.len()
                && fa
                    .iter()
                    .zip(fb.iter())
                    .all(|((na, x), (nb, y))| na.text == nb.text && pats_cover(x, y))
        }
        (Pat::Tuple(aa, _), Pat::Tuple(ab, _)) | (Pat::List(aa, _), Pat::List(ab, _)) => {
            aa.len() == ab.len() && aa.iter().zip(ab.iter()).all(|(x, y)| pats_cover(x, y))
        }
        _ => false,
    }
}

/// The uniform-word ABI of a boxed value (the compiler's calling
/// convention: one `i64` word, 8-byte aligned).
const WORD_SIZE: u64 = 8;
const WORD_ALIGN: u64 = 8;

impl Checker {
    /// Report a non-fatal diagnostic (attribute annotations
    /// specification section 2: the weight reports). Under
    /// `#[deny-warnings]` (a module, a definition, or the `jpl`
    /// profile; `warn # error` scoped) a warning-level report is a
    /// compile error (attribute annotations specification section 10).
    fn report(
        &mut self,
        code: &'static str,
        message: String,
        span: ast::Span,
        informational: bool,
    ) -> Result<()> {
        if !informational && self.deny_live() {
            return Err(Error::at(
                span,
                format!(
                    "{message} (denied by `#[deny-warnings]`/`warn # error`: a warning in scope is a compile error)"
                ),
            ));
        }
        self.diags.push(Diagnostic { code, message, span, informational });
        Ok(())
    }

    /// Is `warn # error` in force for the code being checked (attribute
    /// annotations specification section 10)?
    fn deny_live(&self) -> bool {
        self.cur_def_deny
            || self.jpl_for(&self.current_module)
            || self
                .module_safety
                .get(&self.current_module)
                .map(|m| m.deny_warnings)
                .unwrap_or(false)
    }

    /// An `if` decision point's branch weights from its three attribute
    /// positions (attribute annotations specification section 2.1): the
    /// condition-position attributes annotate the true branch and
    /// complete the pair with the reference base 1 for the false branch,
    /// and the branch-position attributes annotate their branch
    /// directly.
    fn if_weights(
        &mut self,
        cond_attrs: &[ast::Attribute],
        then_attrs: &[ast::Attribute],
        else_attrs: &[ast::Attribute],
        span: ast::Span,
    ) -> Result<(Option<u32>, Option<u32>)> {
        let c = self.validate_attr_site(cond_attrs, AttrSite::IfCond)?;
        let t = self.validate_attr_site(then_attrs, AttrSite::IfBranch)?;
        let e = self.validate_attr_site(else_attrs, AttrSite::IfBranch)?;
        if c.weight.is_some() && t.weight.is_some() {
            return Err(Error::at(
                span,
                "the true branch of this `if` is annotated twice (before the condition and before the `then` expression); annotate one (attribute annotations specification section 2.1)",
            ));
        }
        let true_w = t.weight.or(c.weight);
        let false_w = e.weight.or(if c.weight.is_some() { Some(1) } else { None });
        let mut ws = [true_w, false_w];
        self.complete_weights(&mut ws, "this `if`", span)?;
        Ok((ws[0], ws[1]))
    }

    /// The all-or-nothing rule (attribute annotations specification
    /// section 2.1): if any alternative of a decision point carries a
    /// weight, every alternative must carry one (directly or through a
    /// wrapper). Partial annotation is ambiguous under relativity, so it
    /// drops ALL the decision point's weights and reports
    /// `incomplete-branch-weights`.
    fn complete_weights(
        &mut self,
        ws: &mut [Option<u32>],
        what: &str,
        span: ast::Span,
    ) -> Result<()> {
        let any = ws.iter().any(|w| w.is_some());
        let all = ws.iter().all(|w| w.is_some());
        if any && !all {
            self.report(
                "incomplete-branch-weights",
                format!(
                    "{what} is only partly annotated with weights: all of its weights are dropped (attribute annotations specification section 2.1)"
                ),
                span,
                false,
            )?;
            for w in ws.iter_mut() {
                *w = None;
            }
        }
        Ok(())
    }

    /// The weights of one compiled decision point's alternatives
    /// (attribute annotations specification sections 2.1-2.3): an
    /// alternative's weight is the sum of the weights of the clauses
    /// whose right-hand side is reachable through it (the
    /// marginalization rule), a row counted at more than one path is the
    /// wildcard/or-pattern/guard-fallthrough inflation of section 2.3
    /// and is reported informationally, and explicit weights never mix
    /// with inference (a reachable row without a weight leaves the whole
    /// decision point unweighted, section 3.2).
    fn decision_weights(
        &mut self,
        branches: &[Vec<Row>],
        what: &str,
        span: ast::Span,
    ) -> Result<Vec<Option<u32>>> {
        let mut sums: Vec<u32> = Vec::new();
        let mut contrib: HashMap<usize, usize> = HashMap::new();
        let mut any = false;
        let mut mixed = false;
        for rows in branches {
            let mut sum: u32 = 0;
            for (j, r) in rows.iter().enumerate() {
                // An earlier unguarded row whose patterns cover this
                // one's shadows it: its right-hand side is unreachable
                // through this alternative (section 2.3).
                let reachable = rows[..j].iter().all(|earlier| {
                    !(earlier.guard.is_none()
                        && earlier.pats.len() == r.pats.len()
                        && earlier
                            .pats
                            .iter()
                            .zip(r.pats.iter())
                            .all(|(x, y)| pats_cover(x, y)))
                });
                if !reachable {
                    continue;
                }
                *contrib.entry(r.idx).or_insert(0) += 1;
                match r.weight {
                    Some(w) => {
                        any = true;
                        sum = sum.saturating_add(w);
                    }
                    None => mixed = true,
                }
            }
            sums.push(sum);
        }
        if !any || mixed {
            return Ok(sums.iter().map(|_| None).collect());
        }
        for rows in branches {
            for r in rows {
                if r.weight.is_some() && contrib.get(&r.idx).copied().unwrap_or(0) > 1 {
                    self.report(
                        "incoherent-weights",
                        format!(
                            "{what} counts a weighted clause at more than one path (wildcard, or-pattern, or guard fallthrough): the weights remain usable per decision point (attribute annotations specification section 2.3)"
                        ),
                        span,
                        true,
                    )?;
                    return Ok(sums.into_iter().map(Some).collect());
                }
            }
        }
        Ok(sums.into_iter().map(Some).collect())
    }

    /// Record one function declaration's hints (attribute annotations
    /// specification section 5); hints may ride the signature and the
    /// equations, but one function carries at most one inline hint and
    /// one `#[cold]`.
    fn record_fn_hints(
        &mut self,
        key: &str,
        inline: Option<InlineHint>,
        cold: bool,
        span: ast::Span,
    ) -> Result<()> {
        if inline.is_none() && !cold {
            return Ok(());
        }
        let entry = self.fn_hints.entry(key.to_string()).or_default();
        if let Some(i) = inline {
            if entry.inline.is_some() {
                return Err(Error::at(
                    span,
                    format!(
                        "{key} carries two inline hints: `#[inline]`, `#[inline(always)]`, `#[inline(never)]` and `#[noinline]` are one hint family (attribute annotations specification section 5)"
                    ),
                ));
            }
            entry.inline = Some(i);
        }
        if cold {
            if entry.cold {
                return Err(Error::at(
                    span,
                    format!(
                        "{key} carries two `#[cold]` hints (attribute annotations specification section 5)"
                    ),
                ));
            }
            entry.cold = true;
        }
        Ok(())
    }

    /// Compute every pending type's ABI layout facts (attribute
    /// annotations specification section 4.3); types without a
    /// representation attribute keep the uniform-word ABI and are still
    /// recorded.
    fn compute_layouts(&mut self) -> Result<()> {
        let keys: Vec<(String, String)> = self.layout_recipes.keys().cloned().collect();
        for (m, n) in keys {
            self.layout_of(&m, &n, &mut Vec::new())?;
        }
        Ok(())
    }

    /// The recorded ABI layout of a type, computed on demand: an
    /// explicitly laid out type resolves through its recipe (nested
    /// layouts recurse; a cycle through explicit layouts is an error),
    /// and everything else is the uniform word.
    fn layout_of(
        &mut self,
        module: &str,
        name: &str,
        stack: &mut Vec<(String, String)>,
    ) -> Result<crate::interface::AbiLayout> {
        let key = (module.to_string(), name.to_string());
        if let Some(l) = self.type_layouts.get(&key) {
            return Ok(l.clone());
        }
        if stack.contains(&key) {
            return Err(Error::plain(format!(
                "{module}.{name} has an infinite explicit layout: its fields' layouts depend on the type itself (attribute annotations specification section 4)"
            )));
        }
        let recipe = match self.layout_recipes.get(&key) {
            Some(r) => {
                let r = r.clone();
                Some(r)
            }
            None => None,
        };
        let recipe = match recipe {
            Some(r) => r,
            None => {
                let l = crate::interface::AbiLayout {
                    name: name.to_string(),
                    repr: "uniform-word".to_string(),
                    size: WORD_SIZE,
                    align: WORD_ALIGN,
                    fields: Vec::new(),
                    discriminant: None,
                };
                return Ok(l);
            }
        };
        stack.push(key.clone());
        let built = self.build_layout(&recipe, stack);
        stack.pop();
        let l = built?;
        self.type_layouts.insert(key, l.clone());
        Ok(l)
    }

    /// The C layout algorithm over source-order fields (attribute
    /// annotations specification section 4.1), with the representation's
    /// modifiers applied.
    fn build_layout(
        &mut self,
        recipe: &LayoutRecipe,
        stack: &mut Vec<(String, String)>,
    ) -> Result<crate::interface::AbiLayout> {
        use crate::interface::{AbiField, AbiLayout};
        let repr = &recipe.repr;
        let span = recipe.span;
        let err = |m: String| Error::at(span, m);
        if repr.discriminant.is_some() {
            if recipe.record {
                return Err(err(
                    "a `record` has no constructor discriminant (attribute annotations specification section 4.1)".to_string(),
                ));
            }
            // The discriminant is a recorded fact; the payload layout is
            // not fixed by this representation.
            return Ok(AbiLayout {
                name: recipe.name.clone(),
                repr: repr.spelled.clone(),
                size: WORD_SIZE,
                align: WORD_ALIGN,
                fields: Vec::new(),
                discriminant: repr.discriminant,
            });
        }
        let mut fields: Vec<(&String, &Ty, &ReprSpec)> = Vec::new();
        for (n, t, erased, fr) in &recipe.fields {
            if *erased {
                // Erased fields are evidence: no runtime slot (core
                // specification section 3).
                if fr.has_fields() {
                    return Err(err(format!(
                        "field {n} is erased and has no representation to lay out (attribute annotations specification section 4)"
                    )));
                }
                continue;
            }
            fields.push((n, t, fr));
        }
        if repr.transparent {
            if fields.len() != 1 {
                return Err(err(format!(
                    "`repr(transparent)` needs exactly one represented field, found {} (attribute annotations specification section 4.1)",
                    fields.len()
                )));
            }
            let (n, t, fr) = fields[0];
            if fr.has_fields() {
                return Err(err(format!(
                    "`repr(transparent)` is exactly field {n}'s representation: the field takes no layout modifier (attribute annotations specification section 4.1)"
                )));
            }
            let (size, align) = self.field_size_align(t, span, stack)?;
            return Ok(AbiLayout {
                name: recipe.name.clone(),
                repr: "transparent".to_string(),
                size,
                align,
                fields: vec![AbiField {
                    name: n.to_string(),
                    offset: 0,
                    size,
                    align,
                }],
                discriminant: None,
            });
        }
        if recipe.ctor_count > 1 && repr.has_fields() {
            return Err(err(
                "a multi-constructor `data` has no field layout to compute: `repr(C)`, `align` and `packed` apply to one constructor's source-order fields (attribute annotations specification section 4.1); use `#[repr(u8|u16|u32|u64)]` for a fixed discriminant".to_string(),
            ));
        }
        // The C algorithm: natural alignment, source order, tail padding
        // to the type's alignment; `packed` caps field alignment, a
        // field's `align` raises it.
        let mut offset: u64 = 0;
        let mut natural: u64 = 1;
        let mut out: Vec<AbiField> = Vec::new();
        for (n, t, fr) in &fields {
            let (size, nat) = self.field_size_align(t, span, stack)?;
            let mut align = nat;
            if let Some(raise) = fr.align {
                if raise < nat {
                    return Err(err(format!(
                        "`repr(align({raise}))` on field {n} only raises: the field's natural alignment is {nat} (attribute annotations specification section 4.2)"
                    )));
                }
                align = raise;
            }
            if let Some(cap) = fr.packed.or(repr.packed) {
                align = align.min(cap);
            }
            offset = offset.div_ceil(align) * align;
            out.push(AbiField {
                name: (*n).to_string(),
                offset,
                size,
                align: nat,
            });
            offset += size;
            natural = natural.max(align);
        }
        let mut type_align = natural;
        if let Some(raise) = repr.align {
            if raise < natural {
                return Err(err(format!(
                    "`repr(align({raise}))` only raises: {}'s natural alignment is {natural} (attribute annotations specification section 4.2)",
                    recipe.name
                )));
            }
            type_align = raise;
        }
        let size = offset.div_ceil(type_align) * type_align;
        Ok(AbiLayout {
            name: recipe.name.clone(),
            repr: repr.spelled.clone(),
            size,
            align: type_align,
            fields: out,
            discriminant: None,
        })
    }

    /// One field type's size and alignment: the primitive types
    /// specification section 2 table for unboxed primitives, the
    /// computed layout of an explicitly laid out type, and the uniform
    /// word for everything boxed. A type parameter has no fixed layout.
    fn field_size_align(
        &mut self,
        ty: &Ty,
        span: ast::Span,
        stack: &mut Vec<(String, String)>,
    ) -> Result<(u64, u64)> {
        match ty {
            Ty::Unit => Ok((0, 1)),
            Ty::Bool => Ok((1, 1)),
            Ty::Prim(s) => Ok((s.size(), s.align())),
            Ty::Adt(name, _) => {
                let kind = self.lookup_type(&[name.as_str()])?.clone();
                let key = (kind.module.clone(), kind.name.clone());
                if self.layout_recipes.contains_key(&key) {
                    let l = self.layout_of(&key.0, &key.1, stack)?;
                    Ok((l.size, l.align))
                } else {
                    Ok((WORD_SIZE, WORD_ALIGN))
                }
            }
            Ty::Param(_) | Ty::Meta(_) | Ty::Idx(_) | Ty::Prp(_) | Ty::Sort(_) => Err(Error::at(
                span,
                "a representation attribute fixes every field's layout, but this field's type is not fixed (a type parameter or an unresolved index); attribute annotations specification section 4.1".to_string(),
            )),
            _ => Ok((WORD_SIZE, WORD_ALIGN)),
        }
    }
}

/// Render a scheme for interface metadata: quantifiers, `using` context,
/// then the type with its effect rows.
fn render_scheme(s: &Scheme) -> String {
    let mut out = String::new();
    let all: Vec<String> = s
        .params
        .iter()
        .cloned()
        .chain(s.idx_params.iter().map(|p| p.name().to_string()))
        .chain(s.val_params.iter().map(|(n, _)| n.clone()))
        .chain(s.prop_params.iter().cloned())
        .chain(s.row_params.iter().cloned())
        .chain(s.region_params.iter().cloned())
        .collect();
    if !all.is_empty() {
        out.push_str("forall ");
        for p in all {
            out.push_str(&p);
            out.push(' ');
        }
        out.push_str(". ");
    }
    if !s.ctx.is_empty() {
        out.push_str("using (");
        for (i, (n, c)) in s.ctx.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(n);
            out.push_str(" :: ");
            out.push_str(&c.to_string());
        }
        out.push_str(") => ");
    }
    out.push_str(&s.ty.to_string());
    out
}

/// A module's declared import names, keyed by their full dotted names.
fn imports_of(m: &ast::Module) -> Vec<String> {
    m.decls
        .iter()
        .filter_map(|d| match d {
            ast::Decl::Import(i) => Some(i.path.parts().join(".")),
            _ => None,
        })
        .collect()
}

/// One JSON document's compile-time shape (surface grammar local
/// amendments section 11): the `json` converter's parsed form, before it
/// becomes constant code.
enum JsonVal {
    Null,
    Bool(bool),
    /// The raw digit run of a JSON number.
    Num(String),
    Str(String),
    Arr(Vec<JsonVal>),
    Obj(Vec<(String, JsonVal)>),
}

/// Parse a JSON document (RFC 8259 shapes) for the `json` converter.
fn parse_json(buf: &[u8]) -> Result<JsonVal, String> {
    let mut p = JsonParser { buf, i: 0 };
    p.ws();
    let v = p.value(0)?;
    p.ws();
    if p.i != buf.len() {
        return Err(format!("trailing content at byte {}", p.i));
    }
    Ok(v)
}

struct JsonParser<'a> {
    buf: &'a [u8],
    i: usize,
}

impl<'a> JsonParser<'a> {
    fn ws(&mut self) {
        while matches!(self.buf.get(self.i), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.i += 1;
        }
    }

    fn value(&mut self, depth: usize) -> Result<JsonVal, String> {
        // The converter budget (P9 discipline) bounds recursion too.
        if depth > 64 {
            return Err("the JSON document nests deeper than 64 levels".to_string());
        }
        match self.buf.get(self.i) {
            Some(b'n') => self.word("null").map(|_| JsonVal::Null),
            Some(b't') => self.word("true").map(|_| JsonVal::Bool(true)),
            Some(b'f') => self.word("false").map(|_| JsonVal::Bool(false)),
            Some(b'"') => Ok(JsonVal::Str(self.string()?)),
            Some(b'[') => {
                self.i += 1;
                let mut out = Vec::new();
                self.ws();
                if self.buf.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(JsonVal::Arr(out));
                }
                loop {
                    self.ws();
                    out.push(self.value(depth + 1)?);
                    self.ws();
                    match self.buf.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(JsonVal::Arr(out));
                        }
                        _ => return Err(format!("expected `,` or `]` at byte {}", self.i)),
                    }
                }
            }
            Some(b'{') => {
                self.i += 1;
                let mut out = Vec::new();
                self.ws();
                if self.buf.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(JsonVal::Obj(out));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.ws();
                    if self.buf.get(self.i) != Some(&b':') {
                        return Err(format!("expected `:` at byte {}", self.i));
                    }
                    self.i += 1;
                    self.ws();
                    let v = self.value(depth + 1)?;
                    out.push((k, v));
                    self.ws();
                    match self.buf.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(JsonVal::Obj(out));
                        }
                        _ => return Err(format!("expected `,` or `}}` at byte {}", self.i)),
                    }
                }
            }
            Some(b'-' | b'0'..=b'9') => Ok(JsonVal::Num(self.number()?)),
            Some(c) => Err(format!("unexpected byte {c:#04x} at {}", self.i)),
            None => Err("unexpected end of the JSON document".to_string()),
        }
    }

    fn word(&mut self, w: &str) -> Result<(), String> {
        if self.buf[self.i..].starts_with(w.as_bytes()) {
            self.i += w.len();
            Ok(())
        } else {
            Err(format!("expected `{w}` at byte {}", self.i))
        }
    }

    fn number(&mut self) -> Result<String, String> {
        let start = self.i;
        if self.buf.get(self.i) == Some(&b'-') {
            self.i += 1;
        }
        while matches!(self.buf.get(self.i), Some(b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-')) {
            self.i += 1;
        }
        let s = std::str::from_utf8(&self.buf[start..self.i])
            .map_err(|_| "a JSON number is not UTF-8".to_string())?
            .to_string();
        if s.is_empty() || s == "-" {
            return Err(format!("expected a number at byte {start}"));
        }
        Ok(s)
    }

    fn string(&mut self) -> Result<String, String> {
        if self.buf.get(self.i) != Some(&b'"') {
            return Err(format!("expected a string at byte {}", self.i));
        }
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        loop {
            match self.buf.get(self.i) {
                Some(b'"') => {
                    self.i += 1;
                    return String::from_utf8(out)
                        .map_err(|_| "a JSON string is not UTF-8".to_string());
                }
                Some(b'\\') => {
                    self.i += 1;
                    let esc = *self
                        .buf
                        .get(self.i)
                        .ok_or_else(|| "truncated escape".to_string())?;
                    self.i += 1;
                    match esc {
                        b'"' => out.push(b'"'),
                        b'\\' => out.push(b'\\'),
                        b'/' => out.push(b'/'),
                        b'b' => out.push(0x08),
                        b'f' => out.push(0x0c),
                        b'n' => out.push(b'\n'),
                        b'r' => out.push(b'\r'),
                        b't' => out.push(b'\t'),
                        b'u' => {
                            let cp = self.hex4()?;
                            let cp = if (0xD800..=0xDBFF).contains(&cp) {
                                if self.buf.get(self.i) == Some(&b'\\')
                                    && self.buf.get(self.i + 1) == Some(&b'u')
                                {
                                    self.i += 2;
                                    let lo = self.hex4()?;
                                    if !(0xDC00..=0xDFFF).contains(&lo) {
                                        return Err(format!(
                                            "invalid surrogate pair at byte {}",
                                            self.i
                                        ));
                                    }
                                    0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00)
                                } else {
                                    return Err(format!(
                                        "unpaired surrogate at byte {}",
                                        self.i
                                    ));
                                }
                            } else {
                                cp
                            };
                            out.extend(
                                char::from_u32(cp)
                                    .ok_or_else(|| {
                                        format!("invalid Unicode scalar value {cp:#x}")
                                    })?
                                    .to_string()
                                    .as_bytes(),
                            );
                        }
                        _ => return Err(format!("unknown escape at byte {}", self.i)),
                    }
                }
                Some(c) => {
                    out.push(*c);
                    self.i += 1;
                }
                None => return Err("unterminated JSON string".to_string()),
            }
        }
    }

    fn hex4(&mut self) -> Result<u32, String> {
        if self.i + 4 > self.buf.len() {
            return Err("truncated `\\u` escape".to_string());
        }
        let s = std::str::from_utf8(&self.buf[self.i..self.i + 4])
            .map_err(|_| "a `\\u` escape is not UTF-8".to_string())?;
        let v = u32::from_str_radix(s, 16)
            .map_err(|_| format!("invalid `\\u` escape `{s}`"))?;
        self.i += 4;
        Ok(v)
    }
}

impl Checker {
    /// Convert every pending instance head in declaration order (see
    /// `pending_instance_heads`): the class application and the context
    /// constraints resolve against the fully collected program, so a
    /// head may name any class or type of the program. The bodies check
    /// later, from `instance_defs_pending`.
    fn convert_instance_heads(&mut self) -> Result<()> {
        let pending = std::mem::take(&mut self.pending_instance_heads);
        for (module, inst) in pending {
            self.build_scope(&module, false);
            let params: Vec<String> = inst.params.iter().map(|p| p.text.clone()).collect();
            let mut ctx: Vec<InstCtx> = Vec::new();
            for c in &inst.ctx {
                match self.conv_oblig(c, &params) {
                    Ok(ConvOblig::Class(n, app)) => ctx.push(InstCtx::Dict(n, app)),
                    Ok(ConvOblig::Prop(o)) => ctx.push(InstCtx::Oblig(o)),
                    Err(e) => {
                        if self.drop_embedded_head(&module, &inst) {
                            continue;
                        }
                        return Err(e);
                    }
                }
            }
            let (class, args) = match self.class_app_of(&inst.target, &params) {
                Ok(x) => x,
                Err(e) => {
                    if self.drop_embedded_head(&module, &inst) {
                        continue;
                    }
                    return Err(e);
                }
            };
            if class == "Copy" {
                return Err(Error::at(
                    inst.span,
                    "Copy is a compiler-known class with structural instances (ownership and usage design, section 3); it has no user instances",
                ));
            }
            // A `Preserves` obligation names one of the class's methods
            // (the generic transfer's validity condition).
            for item in &ctx {
                if let InstCtx::Oblig(PropOblig::Preserves { method, .. }) = item {
                    let known = self
                        .sigs
                        .get(&class)
                        .map(|ss| {
                            ss.iter().any(|s| {
                                s.methods.iter().any(|(m, _)| m == method)
                            })
                        })
                        .unwrap_or(false);
                    if !known {
                        return Err(Error::at(
                            inst.span,
                            format!(
                                "`Preserves ({method})` names a method of {class}, but the class has no such method"
                            ),
                        ));
                    }
                }
            }
            let base_id = fn_id(&module, &inst.name.text);
            let idx = self.instance_list.len();
            self.instance_list.push(InstanceInfo {
                module: module.clone(),
                name: inst.name.text.clone(),
                params,
                ctx,
                target: ClassApp { class: class.clone(), args },
                base_id,
                implicit: inst.implicit,
                // A parameterized module's instance needs the module's
                // parameters; it resolves inside that module and is
                // published per instantiation.
                base: self.param_modules.contains(&module),
            });
            self.instances.entry(class).or_default().push(idx);
        }
        Ok(())
    }

    /// Drop an embedded family instance whose class home a shadowing
    /// source module replaced without declaring the class: such a
    /// program never had the instance (the source module is the
    /// program's version of that home), so its head and body both go.
    fn drop_embedded_head(&mut self, module: &str, inst: &ast::InstDecl) -> bool {
        if !self.embedded_modules.contains(module) {
            return false;
        }
        self.instance_defs_pending
            .retain(|(m, i)| !(m == module && i.name.text == inst.name.text));
        true
    }

    /// Instance coherence across the compiled set (design note, section 3,
    /// rule 4): two instances of one class with unifiable targets are
    /// ambiguous. The metadata-level check in `interface.rs` is the fast
    /// path; this is the authoritative check with real type machinery.
    fn check_instance_coherence(&mut self) -> Result<()> {
        let n = self.instance_list.len();
        for a in 0..n {
            for b in (a + 1)..n {
                // Default resolution stays coherent among the instances
                // it may select: the implicit ones. Explicit dictionaries
                // are selected by name (module arguments, `using` binders),
                // so several meaningful implementations of one class may
                // coexist and nothing silently merges them. A
                // parameterized module's declared instance resolves only
                // inside its module and is published per instantiation, so
                // it is compared through its published copies, not against
                // itself (module parameters as module arguments).
                if self.instance_list[a].base || self.instance_list[b].base {
                    continue;
                }
                if !self.instance_list[a].implicit || !self.instance_list[b].implicit {
                    continue;
                }
                let (class_a, class_b) =
                    (self.instance_list[a].target.class.clone(), self.instance_list[b].target.class.clone());
                if class_a != class_b {
                    continue;
                }
                let (pa, ta) = {
                    let i = &self.instance_list[a];
                    (i.params.clone(), i.target.clone())
                };
                let (pb, tb, name_a, name_b) = {
                    let i = &self.instance_list[b];
                    (i.params.clone(), i.target.clone(), self.instance_list[a].name.clone(), i.name.clone())
                };
                let mut map_a: HashMap<String, Ty> = HashMap::new();
                for p in &pa {
                    map_a.insert(p.clone(), self.uni.fresh());
                }
                let mut map_b: HashMap<String, Ty> = HashMap::new();
                for p in &pb {
                    map_b.insert(p.clone(), self.uni.fresh());
                }
                let rows = HashMap::new();
                let regs = HashMap::new();
                let ca = Checker::subst_full_class(&ta, &map_a, &rows, &regs);
                let cb = Checker::subst_full_class(&tb, &map_b, &rows, &regs);
                let conflict = self
                    .uni
                    .unify(
                        &Ty::Adt(ca.class.clone(), ca.args.clone()),
                        &Ty::Adt(cb.class.clone(), cb.args.clone()),
                    )
                    .is_ok();
                if conflict {
                    return Err(Error::plain(format!(
                        "ambiguous instance for {class_a}: {name_a} and {name_b} have unifiable targets"
                    )));
                }
            }
        }
        Ok(())
    }

    /// Build one module's `.cinchif` content (design note, section 2).
    fn module_iface(
        &self,
        module: &str,
        source_hash: &str,
        imports: &[(String, String)],
    ) -> crate::interface::Iface {
        use crate::interface::*;
        let mut iface = Iface::new(module, source_hash);
        for (m, h) in imports {
            iface.imports.push(IfaceRef {
                module: m.clone(),
                iface_hash: h.clone(),
            });
        }
        // The export surface (surface grammar local amendments section
        // 11): with `export` declarations exactly the names their items
        // resolve to, re-exports included; the declared names as before
        // without them. Instances, fixities and the foreign table stay
        // module-owned: they are coherence and contract facts, not
        // names.
        let mut surface: Vec<(String, SurfaceName)> = match self.module_surface.get(module) {
            Some(sm) => sm
                .iter()
                .map(|(l, d)| (l.clone(), d.clone()))
                .collect(),
            None => self
                .declared_names(module)
                .into_iter()
                .map(|n| {
                    (
                        n.clone(),
                        SurfaceName {
                            module: module.to_string(),
                            name: n,
                        },
                    )
                })
                .collect(),
        };
        surface.sort_by(|a, b| a.0.cmp(&b.0));
        for (local, d) in &surface {
            if let Some(v) = self.qualified.get(&(d.module.clone(), d.name.clone())) {
                match &v.kind {
                    ValueKind::TopFn | ValueKind::Prim(_) | ValueKind::PrimDecl { .. } => {
                        iface.exports.values.push(ValueExport {
                            name: local.clone(),
                            scheme: render_scheme(&v.scheme),
                            symbol: format!("{}.{}", d.module, d.name),
                            arity: (v.scheme.ty.arity()
                                + v.scheme.ctx.len()
                                + v.scheme.val_params.len()) as u64,
                        });
                        // The satisfied safety assertions are contract
                        // facts (attribute annotations specification
                        // section 10; decision D112): recorded exactly
                        // like `repr` facts, hashed into `iface_hash`.
                        if let Some(facts) =
                            self.safety_facts.get(&fn_id(&d.module, &d.name).0)
                        {
                            iface.assertions.push(crate::interface::AssertionFact {
                                name: local.clone(),
                                facts: facts.clone(),
                            });
                        }
                        // The temporal contracts verified on the
                        // definition (core specification section 10):
                        // contract facts exactly like the assertion rows
                        // above, hashed into `iface_hash`; their stated
                        // assumptions join the declared-assumptions table
                        // (core specification section 11: the acceptance
                        // manifest identifies what claimed guarantees
                        // rely on).
                        if let Some(facts) =
                            self.contract_facts.get(&fn_id(&d.module, &d.name).0)
                        {
                            for f in facts {
                                iface.contracts.push(crate::interface::ContractFact {
                                    name: local.clone(),
                                    kind: f.kind.clone(),
                                    formula: f.formula.clone(),
                                    assumptions: f.assumptions.clone(),
                                });
                                for a in &f.assumptions {
                                    iface.assumptions.declared.push(format!(
                                        "temporal {} on {}.{}: {a}",
                                        f.kind, d.module, d.name
                                    ));
                                }
                            }
                        }
                        // The `must-use` rows of the return-value
                        // discipline (decision D114's rule 7 default): a
                        // `Result`-returning value is discarded only
                        // through `ignore`.
                        let mut res: &Ty = &v.scheme.ty;
                        loop {
                            match res {
                                Ty::Fun(_, _, _, b) | Ty::Pi { cod: b, .. } => res = b,
                                _ => break,
                            }
                        }
                        if let Ty::Adt(n, _) = res {
                            if n == "Result" {
                                iface.must_use.push(local.clone());
                            }
                        }
                    }
                    _ => {}
                }
            }
            let t = self
                .types
                .get(&d.name)
                .and_then(|ts| ts.iter().find(|t| t.module == d.module))
                .cloned();
            if let Some(t) = t {
                let mut ctors = Vec::new();
                let mut params: Vec<String> = Vec::new();
                for (cname, tag) in &t.ctors {
                    if let Some(v) = self.qualified.get(&(d.module.clone(), cname.clone())) {
                        if let ValueKind::Ctor { field_tys, field_erased, ty_params, .. } = &v.kind {
                            if params.is_empty() {
                                params = ty_params.clone();
                            }
                            let mut map = HashMap::new();
                            for p in ty_params {
                                map.insert(p.clone(), Ty::Param(p.clone()));
                            }
                            let fields = field_tys
                                .iter()
                                .map(|f| Checker::subst_ty(f, &map).to_string())
                                .collect::<Vec<_>>();
                            // A constructor is surface exactly when it
                            // is a named entry of the export set
                            // (D143's rule), recorded under its
                            // exported spelling. Unnamed constructors
                            // (module-private like `StrBuf`, D298)
                            // must not leak through the export table
                            // to importing modules.
                            for (ctor_local, _) in surface.iter().filter(|(_, s)| {
                                s.module == d.module && s.name == *cname
                            }) {
                                ctors.push(CtorExport {
                                    name: ctor_local.clone(),
                                    tag: *tag as u64,
                                    fields: fields.clone(),
                                    erased: field_erased.clone(),
                                });
                            }
                        }
                    }
                }
                iface.exports.types.push(TypeExport {
                    name: local.clone(),
                    params,
                    kind: if t.fields.is_some() { "record".to_string() } else { "data".to_string() },
                    ctors,
                    fields: t.fields.clone().unwrap_or_default(),
                });
                // Computed layout facts (attribute annotations
                // specification section 4.3): part of the contract
                // projection, so a representation change changes the
                // interface identity.
                let mut layout = self
                    .type_layouts
                    .get(&(d.module.clone(), d.name.clone()))
                    .cloned()
                    .unwrap_or(crate::interface::AbiLayout {
                        name: local.clone(),
                        repr: "uniform-word".to_string(),
                        size: WORD_SIZE,
                        align: WORD_ALIGN,
                        fields: Vec::new(),
                        discriminant: None,
                    });
                layout.name = local.clone();
                iface.layouts.push(layout);
            }
            if let Some(s) = self
                .sigs
                .get(&d.name)
                .and_then(|ss| ss.iter().find(|s| s.module == d.module))
            {
                iface.exports.signatures.push(SigExport {
                    name: local.clone(),
                    params: s.params.clone(),
                    methods: s
                        .methods
                        .iter()
                        .map(|(n, sch)| MethodExport {
                            name: n.clone(),
                            scheme: render_scheme(sch),
                            default: self
                                .default_ids
                                .get(&(s.module.clone(), s.name.clone(), n.clone()))
                                .cloned(),
                        })
                        .collect(),
                });
            }
        }
        // `prim` declarations (primitive operations and inline target
        // IR specification section 5): a justified prim's contract is
        // its signature plus the law identity (bodies are
        // representation: replacing a body never stales dependents); an
        // assumed prim's body content is contract (editing it stales
        // dependents). Selection is representation and never enters the
        // contract projection (section 6).
        for p in &self.prims {
            if p.module != module {
                continue;
            }
            match p.tier {
                crate::core::PrimTier::Justified => {
                    for l in &p.laws {
                        iface.assumptions.declared.push(l.clone());
                    }
                }
                crate::core::PrimTier::Assumed => {
                    iface.assumptions.primitives.push(p.identity.clone());
                }
            }
            // The `unsafe` trust (section 12): a trusted body's declared
            // row is a recorded assumption with a per-body identity, so
            // body edits stale dependents through `iface_hash`.
            for b in &p.bodies {
                if b.trusted {
                    iface.assumptions.declared.push(b.trust_id.clone());
                }
            }
        }
        for i in &self.instance_list {
            if i.module != module {
                continue;
            }
            iface.exports.instances.push(InstanceExport {
                id: i.base_id.0.clone(),
                class: i.target.class.clone(),
                target: i.target.to_string(),
                ctx: i
                    .ctx
                    .iter()
                    .map(|item| match item {
                        InstCtx::Dict(n, c) => format!("{n} :: {c}"),
                        InstCtx::Oblig(PropOblig::Pred(p)) => p.to_string(),
                        InstCtx::Oblig(PropOblig::Preserves { method, pred }) => {
                            format!("Preserves ({method}) {pred}")
                        }
                    })
                    .collect(),
                methods: Vec::new(),
                implicit: i.implicit,
            });
        }
        for (m, f) in &self.module_fixities {
            if m == module {
                iface.exports.fixities.push(f.clone());
            }
        }
        let mut prims = Vec::new();
        let mut trusted = Vec::new();
        for (m, f) in &self.module_foreign {
            if m == module {
                iface.exports.foreign.push(f.clone());
                if f.kind == "intrinsic" {
                    prims.push(f.external.clone());
                } else if f.kind == "import" {
                    // The module trusts this external symbol: an
                    // unverified native library remains in the process's
                    // memory-safety trust boundary (core specification
                    // section 11's acceptance manifest; foreign function
                    // interface specification section 1).
                    trusted.push(f.external.clone());
                }
            }
        }
        // `eval` uses are build-visible assumptions in the same table as
        // the foreign ones (supercompilation and JIT specification
        // section 7).
        for (m, site) in &self.eval_sites {
            if m == module {
                trusted.push(format!("eval at {site}"));
            }
        }
        iface.assumptions.primitives = prims;
        iface.assumptions.foreign = trusted;
        for (m, e) in &self.upgrade_evidence {
            if m == module {
                iface.upgrades.push(e.clone());
            }
        }
        // Parameter identity is part of the contract projection (module
        // parameters as module arguments): a parameterized module records
        // its parameter signatures, and instantiation is representation,
        // recorded beside the layout table and excluded from the contract
        // hash (the target module's identity is unchanged by it).
        if let Some(ps) = self.module_params.get(module) {
            iface.params = ps.iter().map(|p| p.sig.clone()).collect();
        }
        for u in &self.units {
            if u.owner == module {
                iface.units.push(crate::interface::UnitExport {
                    name: u.name.clone(),
                    target: u.target.clone(),
                    args: u.arg_ids.clone(),
                    unit_id: u.unit_id.clone(),
                });
            }
        }
        // The law and preservation check rows of this module's
        // instances (refinement types design, "composability"): contract
        // facts hashed into `iface_hash`, exactly like the assertion
        // rows.
        let prefix = format!("{module}.");
        for l in &self.law_rows {
            if l.instance.starts_with(&prefix) {
                iface.laws.push(crate::interface::LawFact {
                    instance: l.instance.clone(),
                    class: l.class.clone(),
                    law: l.law.clone(),
                    status: l.status.clone(),
                });
            }
        }
        iface
    }
}

/// Check a program of one or more modules and elaborate it. `modules` is in
/// dependency order (imports first); the last module is the entry module and
/// must define `main :: Int`. Returns the program, one interface per module
/// (prelude first), and the staging report (captured inputs and generated
/// declarations) for the artifact manifest.
pub fn check_program_multi(
    prelude: &[ast::Module],
    modules: &[ast::Module],
    source_hashes: &HashMap<String, String>,
    scx: &StagingCx,
) -> Result<(Program, Vec<crate::interface::Iface>, StageReport)> {
    let (program, ifaces, report, _) =
        check_program_multi_diag(prelude, modules, source_hashes, scx)?;
    Ok((program, ifaces, report))
}

/// [`check_program_multi`], also returning the non-fatal diagnostics
/// (the branch weight reports of attribute annotations specification
/// section 2).
pub fn check_program_multi_diag(
    prelude: &[ast::Module],
    modules: &[ast::Module],
    source_hashes: &HashMap<String, String>,
    scx: &StagingCx,
) -> Result<(Program, Vec<crate::interface::Iface>, StageReport, Vec<Diagnostic>)> {
    check_program_multi_full(
        prelude,
        modules,
        source_hashes,
        scx,
        &[],
        &crate::core::TargetProfile::native(),
    )
}

/// [`check_program_multi_diag`] with the generated WIT adapters in scope
/// and the target profile's rules in force (WebAssembly and WASI
/// specification section 3; supercompilation and JIT specification
/// section 7: `allow_eval`; primitive types specification section 2: the
/// `Usize` width keys on the target).
///
/// `prelude` is the base-library family in collection order (the scheme
/// homes of the prelude split; the curated `Prelude` facade among them,
/// after its export targets): the facade is the implicit base layer
/// every other module sees. An explicit source module of a family name
/// shadows the embedded copy (the implicit base layer loses to explicit
/// source, decision D145's rule extended to module declarations), which
/// is what keeps the tests that assemble the shipped module sources
/// checking unchanged. The facade's own name is reserved.
pub fn check_program_multi_full(
    prelude: &[ast::Module],
    modules: &[ast::Module],
    source_hashes: &HashMap<String, String>,
    scx: &StagingCx,
    wit: &[crate::interface::WitAdapter],
    profile: &crate::core::TargetProfile,
) -> Result<(Program, Vec<crate::interface::Iface>, StageReport, Vec<Diagnostic>)> {
    let (program, ifaces, report, diags, _) =
        check_program_multi_full_probe(prelude, modules, source_hashes, scx, wit, profile, None)?;
    Ok((program, ifaces, report, diags))
}

/// A probe expression for [`check_program_multi_full_probe`]: the REPL's
/// `:type` and expression evaluation (P21), evaluated in `module`'s scope
/// against the checked program.
pub struct ProbeTerm<'a> {
    /// The module whose scope resolves the expression (the REPL session
    /// module).
    pub module: String,
    /// The expression to infer and evaluate.
    pub term: &'a ast::Term,
}

/// The probe's answer: the inferred scheme and, when the D92 evaluator
/// can run the elaborated code, its value. A probe failure never fails
/// the checked program; it is reported here.
pub struct ProbeResult {
    /// The inferred scheme and its rendering (`render_scheme`), when the
    /// expression elaborates. The scheme's `ctx` holds the constraints no
    /// instance discharges.
    pub scheme: Option<(crate::types::Scheme, String)>,
    /// The D92 evaluator's value, when the elaborated code runs.
    pub value: Option<crate::core::Value>,
    /// Why the value is absent (the evaluator cannot run this code, or
    /// its constraints are unresolved).
    pub eval_error: Option<String>,
    /// Why the scheme is absent: the expression does not elaborate.
    pub error: Option<crate::Error>,
}

/// [`check_program_multi_full`] with an optional probe (P21: the REPL is
/// backed by the same analysis the language server drives): after every
/// module's declarations are checked, the probe expression elaborates in
/// the named module's scope and evaluates through the D92 evaluator (the
/// staging interpreter), so the session's definitions are available to it.
pub fn check_program_multi_full_probe(
    prelude: &[ast::Module],
    modules: &[ast::Module],
    source_hashes: &HashMap<String, String>,
    scx: &StagingCx,
    wit: &[crate::interface::WitAdapter],
    profile: &crate::core::TargetProfile,
    probe: Option<&ProbeTerm<'_>>,
) -> Result<(
    Program,
    Vec<crate::interface::Iface>,
    StageReport,
    Vec<Diagnostic>,
    Option<ProbeResult>,
)> {
    check_program_multi_full_impl(prelude, modules, source_hashes, scx, wit, profile, probe)
}

/// The number of parameters a type's arrow spine declares: what an
/// equation at that type must bind under the arity rule.
fn arrow_count(ty: &Ty) -> usize {
    let mut n = 0;
    let mut rest = ty;
    loop {
        match rest {
            Ty::Fun(_, _, _, b) => rest = b,
            Ty::Pi { cod, .. } => rest = cod,
            _ => return n,
        }
        n += 1;
    }
}

/// Name the module a collect or check failure happened in.
///
/// A `Span` carries only a line and a column (`lexer::Span`), so a failure in
/// a module other than the entry reports a bare position with nothing saying
/// which of the program's modules raised it: the ladder's parser hit exactly
/// that, with six project modules in one compile and an error reading
/// `812:13: ...`. The entry's own parse errors already carry their file
/// (the parser formats it into the message), and this gives collect and check
/// failures the same courtesy. The file comes from the driver's module-to-path
/// map; an unknown module falls back to its own name, which is still better
/// than nothing. The suffix leaves the span intact, so the language server's
/// diagnostics keep their machine-readable position.
fn in_module(file: &str, e: Error) -> Error {
    if e.message.contains(" (in ") {
        return e;
    }
    Error { span: e.span, message: format!("{} (in {})", e.message, file) }
}

/// [`in_module`] resolved against the checker's module-to-file map.
fn in_named_module(
    files: &std::collections::HashMap<String, String>,
    module: &str,
    e: Error,
) -> Error {
    let where_ = match files.get(module) {
        Some(file) => file.clone(),
        None => module.to_string(),
    };
    in_module(&where_, e)
}

/// [`check_program_multi_full_probe`]'s body; the plain entry delegates
/// with no probe.
fn check_program_multi_full_impl(
    prelude: &[ast::Module],
    modules: &[ast::Module],
    source_hashes: &HashMap<String, String>,
    scx: &StagingCx,
    wit: &[crate::interface::WitAdapter],
    profile: &crate::core::TargetProfile,
    probe: Option<&ProbeTerm<'_>>,
) -> Result<(
    Program,
    Vec<crate::interface::Iface>,
    StageReport,
    Vec<Diagnostic>,
    Option<ProbeResult>,
)> {
    let Some(facade) = prelude.iter().find(|m| m.name.text == "Prelude") else {
        return Err(Error::plain("internal error: the prelude family has no facade"));
    };
    let prelude_name = facade.name.text.clone();
    let mut c = Checker::new();
    c.install_builtins();
    c.allow_eval = profile.allow_eval;
    c.usize_bits = profile.word_size;
    // Capability evidence of the target profile (primitive operations
    // and inline target IR specification section 6): a `prim` body's
    // `Available X` condition holds where `X` is listed.
    c.target_caps = {
        let mut caps = profile.required.clone();
        for f in &profile.features {
            if !caps.contains(f) {
                caps.push(f.clone());
            }
        }
        caps
    };
    c.target_arch = profile
        .triple
        .split('-')
        .next()
        .unwrap_or_default()
        .to_string();
    // P20: the `jpl` target profile makes the safety assertion set
    // mandatory for the user's program (decision D114); the prelude
    // family is the standard library, not flight code.
    c.jpl_target = profile.jpl;
    c.jpl_user_modules = modules.iter().map(|m| m.name.text.clone()).collect();
    c.install_wit_api(wit)?;
    c.stage_mode = scx.mode;
    c.module_files = scx.files.clone();
    c.fold_scx = Some(scx.clone());
    c.prelude_module = prelude_name.clone();
    let user_names: std::collections::HashSet<&str> =
        modules.iter().map(|m| m.name.text.as_str()).collect();
    for m in modules {
        if m.name.text == prelude_name {
            return Err(Error::at(
                m.name.span,
                format!(
                    "module {prelude_name} is the reserved prelude facade; a source module cannot take its name"
                ),
            ));
        }
    }
    // Collection order: the facade's export targets, then the facade
    // (the implicit base layer), then source modules that shadow family
    // homes (their programs' version of the home must exist before the
    // family members that name its classes), then the late family, then
    // the remaining source modules.
    let shadow_set: std::collections::HashSet<&str> = prelude
        .iter()
        .filter(|p| user_names.contains(p.name.text.as_str()))
        .map(|p| p.name.text.as_str())
        .collect();
    let facade_at = prelude.iter().position(|p| p.name.text == prelude_name);
    let mut family: Vec<&ast::Module> = Vec::new();
    for (i, p) in prelude.iter().enumerate() {
        if shadow_set.contains(p.name.text.as_str()) {
            // An explicit source module shadows the embedded copy.
            c.shadowed_modules.insert(p.name.text.clone());
            continue;
        }
        family.push(p);
        c.embedded_modules.insert(p.name.text.clone());
        let imports = imports_of(p);
        c.collect_module(p, &prelude_name, &imports)
            .map_err(|e| in_named_module(&c.module_files, &p.name.text, e))?;
        if Some(i) == facade_at {
            c.install_presentation_aliases();
            for m in modules {
                if shadow_set.contains(m.name.text.as_str()) {
                    let imports = imports_of(m);
                    c.collect_module(m, &prelude_name, &imports)
                        .map_err(|e| in_named_module(&c.module_files, &m.name.text, e))?;
                }
            }
        }
    }
    c.install_presentation_aliases();
    for m in modules {
        if shadow_set.contains(m.name.text.as_str()) {
            continue;
        }
        let imports = imports_of(m);
        c.collect_module(m, &prelude_name, &imports)
            .map_err(|e| in_named_module(&c.module_files, &m.name.text, e))?;
    }
    // Instance heads convert once every module is collected: a head
    // may name a class or type from a module collected later.
    c.convert_instance_heads()?;
    // Module parameter types convert once every name is collected (module
    // parameters as module arguments).
    c.finish_module_params()?;
    // ABI layout facts resolve once every type name is known (attribute
    // annotations specification section 4.3); the packed-borrow rule of
    // section 4.2 reads them during checking.
    c.compute_layouts()?;
    // Foreign binding contracts classify their parameters against the
    // recorded layouts (foreign function interface specification
    // section 2).
    c.build_foreign_contracts()?;
    // The prelude family compiles first: its functions and instances run
    // during compile-time evaluation.
    for p in &family {
        c.check_bodies(&p.name.text, &prelude_name)
            .map_err(|e| in_named_module(&c.module_files, &p.name.text, e))?;
    }
    // The staging phase (modules specification sections 7-10): compile
    // the compile-time fragment, schedule and run generators, expand macro
    // calls, and collect generated declarations under ordinary rules.
    let mut report = if c.has_staging_work() {
        c.run_staging(scx)?
    } else {
        StageReport {
            mode: scx.mode,
            inputs: Vec::new(),
            generated: Vec::new(),
            literal_folds: Vec::new(),
            data_imports: Vec::new(),
            safety: SafetyReport::default(),
            temporal: Vec::new(),
        }
    };
    // Generated declarations may declare instances: their heads convert
    // now that their modules are collected.
    c.convert_instance_heads()?;
    // Generated declarations may declare types with representations.
    c.compute_layouts()?;
    c.build_foreign_contracts()?;
    for m in modules {
        c.check_bodies(&m.name.text, &prelude_name)
            .map_err(|e| in_named_module(&c.module_files, &m.name.text, e))?;
    }
    // The guarded-letrec rule over the top-level value bindings (the
    // D302 compiler defect): an eager reference cycle among them would
    // re-enter its own thunk and crash the compiled binary at run time,
    // so it refuses here with the named diagnostic instead.
    c.check_top_value_cycles()?;
    c.check_instance_coherence()?;
    // `prim` bodies and class laws are checked after every module's
    // declarations (primitive operations and inline target IR
    // specification sections 5-6: justification, tiering and body
    // selection need the whole program's names and the target profile).
    c.check_prims()?;
    // The law tier evaluates the instances' real method code over
    // sample values whose shapes follow the predicate's bounds (a
    // preservation witness at `length xs <= 200` is a 200-element
    // list): the interpreter's per-call frames size the stack it runs
    // on, not the default one.
    let law = std::thread::scope(|s| {
        std::thread::Builder::new()
            .stack_size(512 * 1024 * 1024)
            .spawn_scoped(s, || c.check_laws())
            .expect("the law layer's thread")
            .join()
    });
    match law {
        Ok(r) => r?,
        Err(_) => {
            return Err(Error::plain(
                "internal error: the law layer's sample evaluation panicked",
            ))
        }
    }
    // Rewrite rules are checked after every module's declarations (their
    // `match`, `replace` and `justify` terms reference them).
    c.check_rewrites()?;
    // Upgrade declarations are checked after every module's records and
    // functions (runtime evolution specification section 3: migration
    // field checking over the two records' field tables).
    c.check_upgrades()?;
    // Temporal contracts (core specification section 10) verify with
    // their definitions; anything the collection table still holds names
    // no definition the checker elaborates.
    c.check_unconsumed_contracts()?;
    // Ownership discipline at the boundary: an owned foreign result is
    // released by its matching deallocator (foreign function interface
    // specification section 4).
    c.check_deallocators()?;
    // The collection literals' fold boundary (surface grammar local
    // amendments section 10) accumulates through every unit's check.
    report.literal_folds = std::mem::take(&mut c.literal_folds);
    report.data_imports = std::mem::take(&mut c.data_imports);
    report.temporal = std::mem::take(&mut c.temporal_rows);
    if modules.is_empty() {
        return Err(Error::plain("no modules to check"));
    }
    // P20: the group-level safety claims (`#[recursion]`, `#[stack]`)
    // verify against the checked program's D104 groups before any
    // interface is emitted (decisions D112-D114).
    let safety_records = std::mem::take(&mut c.safety_records);
    // Interfaces in dependency order: imports are known when built.
    let mut ifaces = Vec::new();
    let mut built: Vec<(String, String)> = Vec::new(); // (module, iface_hash)
    let mut emit = |c: &Checker, module: &str, imports: &[String]| {
        let mut refs: Vec<(String, String)> = Vec::new();
        for i in imports {
            if refs.iter().any(|(m, _)| m == i) {
                continue;
            }
            if let Some((_, h)) = built.iter().find(|(m, _)| m == i) {
                refs.push((i.clone(), h.clone()));
            }
        }
        let source_hash = source_hashes
            .get(module)
            .cloned()
            .unwrap_or_default();
        let iface = c.module_iface(module, &source_hash, &refs);
        let hash = iface.iface_hash();
        built.push((module.to_string(), hash));
        ifaces.push(iface);
    };
    // The prelude family emits in dependency order (each module's
    // imports are already built); user modules implicitly import the
    // facade.
    for p in &family {
        let imports = imports_of(p);
        emit(&c, &p.name.text, &imports);
    }
    for m in modules {
        let mut imports = imports_of(m);
        if !imports.contains(&prelude_name) {
            imports.push(prelude_name.clone());
        }
        emit(&c, &m.name.text, &imports);
    }
    let diags = std::mem::take(&mut c.diags);
    // P21's probe (the REPL is backed by the analysis layer): evaluate
    // the session's expression now that every module's code exists, just
    // before the checker folds into the finished program.
    let probe_out = match probe {
        Some(p) => Some(c.probe_term(scx, p)),
        None => None,
    };
    let mut program = c.finish(&scx.entry_modules, &prelude_name)?;
    // P20: the group-level safety claims verify over the finished
    // program's D104 groups; the report says which proof method
    // established each `bounded(n)` bound (decision D113).
    report.safety = verify_safety_groups(&safety_records, &program)?;
    // The verified rows re-state as the LLVM contract surface's facts
    // on their definitions (design/llvm-contract-attributes.md).
    contract_facts(&safety_records, &mut program);
    Ok((program, ifaces, report, diags, probe_out))
}

/// Check a single user module against the implicit prelude and elaborate it.
pub fn check_program(prelude: &[ast::Module], user: &ast::Module) -> Result<Program> {
    let mut hashes = HashMap::new();
    hashes.insert(user.name.text.clone(), String::new());
    let cx = StagingCx::default();
    Ok(check_program_multi(prelude, std::slice::from_ref(user), &hashes, &cx)?.0)
}

// ------------------------------------------------------------ staging (P9)

/// Inputs to the staging phase (modules specification sections 9 and 10).
#[derive(Clone)]
pub struct StagingCx {
    /// Per-module fixity tables, for parsing spliced `Syntax`.
    pub fixities: HashMap<String, crate::parser::FixityTable>,
    /// Module to source file, for expansion provenance.
    pub files: HashMap<String, String>,
    /// Captured inputs of a previous build (replay mode).
    pub prior_inputs: Vec<StageInput>,
    /// The build's compile-time I/O mode.
    pub mode: StageMode,
    /// The entry file's directory: file capabilities resolve against it.
    pub workdir: std::path::PathBuf,
    /// The entry file's modules: `main` may live in any of them.
    pub entry_modules: Vec<String>,
    /// The database schema introspection tool for `Stage.withDatabase`
    /// (modules specification section 11 delegates introspection to a
    /// tool run over the subprocess capability; decision D65).
    pub schema_tool: Option<String>,
}

impl Default for StagingCx {
    fn default() -> Self {
        StagingCx {
            fixities: HashMap::new(),
            files: HashMap::new(),
            prior_inputs: Vec::new(),
            mode: StageMode::Refresh,
            workdir: std::path::PathBuf::new(),
            entry_modules: Vec::new(),
            schema_tool: None,
        }
    }
}

impl Default for StageMode {
    fn default() -> Self {
        StageMode::Refresh
    }
}

/// What the staging phase produced: captured inputs and generated
/// declarations, recorded in the artifact manifest's build identity.
pub struct StageReport {
    pub mode: StageMode,
    pub inputs: Vec<StageInput>,
    /// Generated declarations per module, in emission order.
    pub generated: Vec<(String, Vec<String>)>,
    /// The collection literals' fold boundaries (surface grammar local
    /// amendments section 10): which constructions became constant
    /// values and which kept their runtime `from-list`/`from-map` calls.
    pub literal_folds: Vec<LitFold>,
    /// The `import data` bindings' provenance (surface grammar local
    /// amendments section 11): converter identity and file content hash.
    pub data_imports: Vec<DataImport>,
    /// The P20 safety assertion report (attribute annotations
    /// specification section 10): which proof method verified each
    /// `bounded(n)` claim (decision D113), and the `jpl` profile's
    /// whole-program summary (decision D114).
    pub safety: SafetyReport,
    /// The verified temporal contracts (core specification section 10):
    /// each contract's kind, formula, verification result and stated
    /// assumptions.
    pub temporal: Vec<TemporalRow>,
}

/// The compile-time evaluation context: everything the interpreter needs
/// beyond the checker itself.
struct StageCx {
    /// Function definitions available to interpretation (compiled units).
    fns: HashMap<String, crate::core::FnDef>,
    /// The module whose scope resolves quotation templates (definition
    /// site of the unit being interpreted).
    def_module: String,
    /// The module the interpreted code was written in (call site): spliced
    /// `Syntax` parses with its fixities.
    origin: String,
    /// The name of the unit being interpreted, for expansion provenance.
    unit_name: String,
    /// The invocation site of the current expansion, if any.
    call: Option<ast::Span>,
    /// The enclosing expansion, for provenance chains.
    parent: Option<ast::Span>,
    /// Fresh-name counter for generated binders (deterministic per build).
    fresh: usize,
    /// The generator's emitted declarations.
    emitted: Vec<crate::ast::Decl>,
    /// Named input snapshot scope (empty outside `Stage.snapshot`).
    snapshot: Vec<String>,
    /// Inputs captured so far.
    inputs: Vec<StageInput>,
    /// Captured inputs of the previous build (replay mode).
    prior: Vec<StageInput>,
    /// Macro expansion depth guard.
    depth: usize,
    /// Evaluation step budget (the staging termination policy).
    steps: u64,
    /// The entry directory: file capabilities resolve against it.
    workdir: std::path::PathBuf,
    /// Per-module fixity tables, for parsing spliced `Syntax`.
    fixities: HashMap<String, crate::parser::FixityTable>,
    /// Spawned subprocesses, killed if the stage fails (resource release).
    children: Vec<std::process::Child>,
    /// The database schema introspection tool (decision D65).
    schema_tool: Option<String>,
}

impl StageCx {
    fn new(scx: &StagingCx) -> StageCx {
        StageCx {
            fns: HashMap::new(),
            def_module: String::new(),
            origin: String::new(),
            unit_name: String::new(),
            call: None,
            parent: None,
            fresh: 0,
            emitted: Vec::new(),
            snapshot: Vec::new(),
            inputs: Vec::new(),
            prior: scx.prior_inputs.clone(),
            depth: 0,
            steps: 8_000_000,
            workdir: scx.workdir.clone(),
            fixities: scx.fixities.clone(),
            children: Vec::new(),
            schema_tool: scx.schema_tool.clone(),
        }
    }
}

/// Kill any subprocess still running when the stage ends (modules
/// specification section 10: cancellation and stage failure release
/// resources).
impl Drop for StageCx {
    fn drop(&mut self) {
        for c in &mut self.children {
            let _ = c.kill();
            let _ = c.wait();
        }
    }
}

/// A scheduled compile-time unit for the dependency graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum UnitKey {
    /// A `stage` or `macro` definition.
    Def(String, String),
    /// A `generate do` block by index.
    Gen(usize),
    /// An ordinary definition (compiled on demand).
    Ord(String, String),
}

impl UnitKey {
    fn desc(&self, checker: &Checker) -> String {
        match self {
            UnitKey::Def(m, n) => {
                if checker.macros.contains_key(&(m.clone(), n.clone())) {
                    format!("macro `{n}` in {m}")
                } else {
                    format!("stage definition `{n}` in {m}")
                }
            }
            UnitKey::Gen(i) => {
                let b = &checker.generate_blocks[*i];
                format!("generate block in {} (at {}:{})", b.module, b.span.line, b.span.col)
            }
            UnitKey::Ord(m, n) => format!("definition `{n}` in {m}"),
        }
    }
}

impl Checker {
    /// Does this compilation involve any compile-time work?
    fn has_staging_work(&self) -> bool {
        !self.staging.is_empty()
            || !self.generate_blocks.is_empty()
            || self.top_eqs.iter().any(|(_, _, e)| term_has_macro_call(&e.body))
    }

    /// The staging phase (modules specification sections 7-10): compile
    /// the compile-time fragment, schedule generators from the explicit
    /// dependency graph (a cycle is a diagnostic showing the cycle), run
    /// them, expand macro calls in ordinary code, and collect the
    /// generated declarations under ordinary rules.
    fn run_staging(&mut self, scx: &StagingCx) -> Result<StageReport> {
        let mut cx = StageCx::new(scx);
        // The explicit dependency graph over units (modules specification
        // section 9): a cycle is rejected before any execution, and units
        // compile on demand, in dependency order.
        let order = self.schedule_units()?;
        for key in &order {
            if let UnitKey::Gen(idx) = key {
                self.run_generator(&mut cx, *idx)?;
            }
        }
        // Macro calls in ordinary code expand before the ordinary check
        // (their expansions are checked under ordinary rules).
        self.expand_remaining_macros(&mut cx)?;
        let mut generated = Vec::new();
        let mut names: HashMap<String, Vec<String>> = HashMap::new();
        for (m, decls) in &self.stage_emitted {
            let ns: Vec<String> = decls.iter().filter_map(decl_name).map(|n| n.text.clone()).collect();
            names.insert(m.clone(), ns);
        }
        for (m, ns) in names {
            generated.push((m, ns));
        }
        generated.sort();
        Ok(StageReport {
            mode: scx.mode,
            inputs: std::mem::take(&mut cx.inputs),
            generated,
            literal_folds: Vec::new(),
            data_imports: Vec::new(),
            safety: SafetyReport::default(),
            temporal: Vec::new(),
        })
    }

    /// Build the staging dependency graph and topologically order it; a
    /// cyclic dependency is rejected with the cycle shown (modules
    /// specification section 9).
    fn schedule_units(&self) -> Result<Vec<UnitKey>> {
        let mut desc: HashMap<UnitKey, String> = HashMap::new();
        let mut edges: HashMap<UnitKey, Vec<UnitKey>> = HashMap::new();
        let mut add_unit = |key: UnitKey, desc: &mut HashMap<UnitKey, String>,
                            edges: &mut HashMap<UnitKey, Vec<UnitKey>>| {
            desc.entry(key.clone()).or_insert_with(|| key.desc(self));
            edges.entry(key.clone()).or_default();
        };
        for (module, name) in &self.staging {
            add_unit(UnitKey::Def(module.clone(), name.clone()), &mut desc, &mut edges);
        }
        for (module, name, _) in &self.top_eqs {
            add_unit(UnitKey::Ord(module.clone(), name.clone()), &mut desc, &mut edges);
        }
        for (i, b) in self.generate_blocks.iter().enumerate() {
            let _ = b;
            add_unit(UnitKey::Gen(i), &mut desc, &mut edges);
        }
        // Statically apparent generator outputs: `emit (quoteDecls where
        // ...)` names. A computed `emit` produces names only at run time;
        // those cannot satisfy a staging dependency (decision D47).
        let mut produced: HashMap<(String, String), UnitKey> = HashMap::new();
        for (i, b) in self.generate_blocks.iter().enumerate() {
            for stmt in &b.stmts {
                let term = match stmt {
                    ast::DoStmt::Expr { term, .. } => Some(term),
                    ast::DoStmt::Bind { term, .. } => Some(term),
                    ast::DoStmt::Let { .. } => None,
                };
                if let Some(ast::Term::App(f, args, _)) = term {
                    if let ast::Term::Var(p) = &**f {
                        if p.quals.is_empty() && p.name.text == "emit" && args.len() == 1 {
                            if let ast::Term::QuoteDecls { decls, .. } = &args[0] {
                                for d in decls {
                                    if let Some(n) = decl_name(d) {
                                        produced.insert((b.module.clone(), n.text.clone()), UnitKey::Gen(i));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Edges: each unit requires the units behind the names it
        // references.
        let unit_refs = |key: &UnitKey, checker: &Checker| -> Vec<ast::RefSite> {
            let mut out = Vec::new();
            match key {
                UnitKey::Def(m, n) => {
                    for (mod_, name, eq) in &checker.top_eqs {
                        if mod_ == m && name == n {
                            ast::collect_decl_refs(&ast::Decl::Equation {
                                modifier: None,
                                eq: eq.clone(),
                            }, &mut out);
                        }
                    }
                }
                UnitKey::Ord(m, n) => {
                    for (mod_, name, eq) in &checker.top_eqs {
                        if mod_ == m && name == n {
                            ast::collect_decl_refs(&ast::Decl::Equation {
                                modifier: None,
                                eq: eq.clone(),
                            }, &mut out);
                        }
                    }
                }
                UnitKey::Gen(i) => {
                    for stmt in &checker.generate_blocks[*i].stmts {
                        match stmt {
                            ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                                ast::collect_term_refs(term, &mut out)
                            }
                            ast::DoStmt::Let { decls, .. } => {
                                for decl in decls {
                                    match decl {
                                        ast::LocalDecl::Eq { body, .. }
                                        | ast::LocalDecl::Bind { body, .. }
                                        | ast::LocalDecl::Lazy { body, .. } => {
                                            ast::collect_term_refs(body, &mut out)
                                        }
                                        ast::LocalDecl::Sig { .. } => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            out
        };
        let unit_module = |key: &UnitKey, checker: &Checker| -> String {
            match key {
                UnitKey::Def(m, _) | UnitKey::Ord(m, _) => m.clone(),
                UnitKey::Gen(i) => checker.generate_blocks[*i].module.clone(),
            }
        };
        for key in desc.keys().cloned().collect::<Vec<_>>() {
            let module = unit_module(&key, self);
            let mut refs = unit_refs(&key, self);
            // A local binder may share a name with a top-level definition;
            // a reference that could be local adds no edge (missed edges
            // only weaken this check, never reject a valid program).
            let mut bound: Vec<String> = Vec::new();
            collect_unit_binders(&key, self, &mut bound);
            refs.retain(|r| !bound.contains(&r.name));
            let mut deps: Vec<UnitKey> = Vec::new();
            for r in &refs {
                let ref_key = if r.quals.is_empty() {
                    (module.clone(), r.name.clone())
                } else {
                    (r.quals[0].clone(), r.name.clone())
                };
                // A name a generator produces is a dependency on that
                // generator: compiling the reference needs its output.
                let target = self.resolve_top(&module, r).or_else(|| {
                    produced.get(&ref_key).map(|_| ref_key.clone())
                });
                let Some((tm, tn)) = target else { continue };
                if let Some(u) = self.unit_of(&tm, &tn, &produced) {
                    if u != key && !deps.contains(&u) {
                        deps.push(u);
                    }
                }
            }
            edges.insert(key, deps);
        }
        // Depth-first topological order with cycle reporting.
        let mut order: Vec<UnitKey> = Vec::new();
        let mut state: HashMap<UnitKey, u8> = HashMap::new(); // 1 visiting, 2 done
        let mut stack: Vec<UnitKey> = Vec::new();
        fn visit(
            key: &UnitKey,
            edges: &HashMap<UnitKey, Vec<UnitKey>>,
            desc: &HashMap<UnitKey, String>,
            state: &mut HashMap<UnitKey, u8>,
            order: &mut Vec<UnitKey>,
            stack: &mut Vec<UnitKey>,
        ) -> Result<()> {
            match state.get(key) {
                Some(2) => return Ok(()),
                Some(1) => {
                    let start = stack.iter().position(|k| k == key).unwrap_or(0);
                    let mut chain: Vec<String> =
                        stack[start..].iter().map(|k| desc[k].clone()).collect();
                    chain.push(desc[key].clone());
                    return Err(Error::plain(format!(
                        "stage dependency cycle: {}",
                        chain.join(" -> ")
                    )));
                }
                _ => {}
            }
            state.insert(key.clone(), 1);
            stack.push(key.clone());
            if let Some(deps) = edges.get(key) {
                for d in deps {
                    visit(d, edges, desc, state, order, stack)?;
                }
            }
            stack.pop();
            state.insert(key.clone(), 2);
            order.push(key.clone());
            Ok(())
        }
        for key in desc.keys() {
            visit(key, &edges, &desc, &mut state, &mut order, &mut stack)?;
        }
        Ok(order)
    }

    /// The unit behind a resolved name, if any.
    fn unit_of(
        &self,
        module: &str,
        name: &str,
        produced: &HashMap<(String, String), UnitKey>,
    ) -> Option<UnitKey> {
        let key = (module.to_string(), name.to_string());
        if self.staging.contains(&key) {
            return Some(UnitKey::Def(module.to_string(), name.to_string()));
        }
        if let Some(u) = produced.get(&key) {
            return Some(u.clone());
        }
        if self.top_schemes.contains_key(&key) {
            return Some(UnitKey::Ord(module.to_string(), name.to_string()));
        }
        None
    }

    /// Resolve a top-level name in `module`'s scope (own names, imports,
    /// then the prelude and the builtins), the definition-site rule of
    /// section 8.
    fn resolve_top(&self, module: &str, r: &ast::RefSite) -> Option<(String, String)> {
        if !r.quals.is_empty() {
            return Some((r.quals[0].clone(), r.name.clone()));
        }
        let mut visible = vec![module.to_string()];
        if let Some(imports) = self.module_imports.get(module) {
            for i in imports {
                if !visible.contains(i) {
                    visible.push(i.clone());
                }
            }
        }
        if let Some(imports) = self.module_stage_imports.get(module) {
            for i in imports {
                if !visible.contains(i) {
                    visible.push(i.clone());
                }
            }
        }
        if !visible.contains(&self.prelude_module) {
            visible.push(self.prelude_module.clone());
        }
        // Builtin values and types (`True`, `Nat`, the `Stage` API) are in
        // scope everywhere.
        visible.push("<builtin>".to_string());
        // A visible module resolves the name through its surface (the
        // prelude facade re-exports the split family's definitions):
        // the reported pair is the LOCAL spelling `<visible>.<name>`,
        // which resolves at every site that sees the same visibility
        // (the quote hygiene's qualified re-spelling relies on it).
        for m in &visible {
            let Some(d) = self.surface_def(m, &r.name) else {
                continue;
            };
            let hit = match r.kind {
                ast::RefKind::Value => self
                    .values
                    .get(&r.name)
                    .map(|vs| vs.iter().any(|v| v.module == d.module && v.name == d.name))
                    .unwrap_or(false),
                ast::RefKind::Type => self
                    .types
                    .get(&r.name)
                    .map(|ts| ts.iter().any(|t| t.module == d.module && t.name == d.name))
                    .unwrap_or(false),
            };
            if hit {
                return Some((m.clone(), r.name.clone()));
            }
        }
        None
    }

    /// Compile one `stage` definition or macro into its compile-time
    /// function unit.
    fn compile_unit(&mut self, cx: &mut StageCx, module: &str, name: &str) -> Result<()> {
        let id = fn_id(module, name);
        if cx.fns.contains_key(&id.0) {
            return Ok(());
        }
        let eqs: Vec<ast::Equation> = self
            .top_eqs
            .iter()
            .filter(|(m, n, _)| m == module && n == name)
            .map(|(_, _, e)| e.clone())
            .collect();
        let scheme = self
            .top_schemes
            .get(&(module.to_string(), name.to_string()))
            .cloned()
            .ok_or_else(|| Error::plain(format!("missing signature for {name}")))?;
        self.set_scope(module);
        let before = self.fns.len();
        self.compile_time = true;
        let r = self.check_equations(name, &id, &scheme, &eqs, Vec::new());
        self.compile_time = false;
        r?;
        for f in self.fns.drain(before..) {
            cx.fns.insert(f.id.0.clone(), f);
        }
        self.checked_early.insert((module.to_string(), name.to_string()));
        Ok(())
    }

    /// Build the checking module's scope (surface grammar local
    /// amendments section 11): its own names resolve first, then its
    /// imports in declaration order, then the prelude's. `include_stage`
    /// adds the `import stage` modules (the compile-time scope sees their
    /// generators; ordinary collection keeps them out).
    fn build_scope(&mut self, module: &str, include_stage: bool) {
        self.current_module = module.to_string();
        let mut views: Vec<ImportView> = Vec::new();
        if let Some(vs) = self.module_views.get(module).cloned() {
            for v in vs {
                if v.applied || (!include_stage && v.stage) {
                    continue;
                }
                views.push(v);
            }
        }
        if self.prelude_module != module {
            views.push(ImportView {
                target: self.prelude_module.clone(),
                alias: self.prelude_module.clone(),
                stage: false,
                applied: false,
                alias_only: false,
                members: None,
            });
        }
        let mut visible = vec![module.to_string()];
        for v in &views {
            if !visible.contains(&v.target) {
                visible.push(v.target.clone());
            }
        }
        self.scope_views = views;
        self.visible = visible;
    }

    /// One module's export surface and import hygiene (surface grammar
    /// local amendments section 11): with `export` declarations exactly
    /// the names their items resolve to, including re-exported imports;
    /// without them nothing is recorded and the module exposes its
    /// declared names as before. Duplicate unrenamed imported names are
    /// an error naming both sources (renaming is the fix).
    fn finish_module_surface(&mut self, module: &str) -> Result<()> {
        // Import hygiene: each import's bare contribution, by local name.
        let mut seen: Vec<(String, String)> = Vec::new();
        for v in self.module_views.get(module).cloned().unwrap_or_default() {
            if v.applied {
                continue;
            }
            if let Some(ms) = &v.members {
                // A selection names members of the source's surface
                // (surface grammar local amendments section 11).
                for (local, source) in ms {
                    if self.surface_def(&v.target, source).is_none() {
                        return Err(Error::plain(format!(
                            "`{}` has no member `{source}` to import (as `{local}`)",
                            v.target
                        )));
                    }
                }
            }
            if v.alias_only || v.stage {
                continue;
            }
            let names: Vec<String> = match &v.members {
                Some(ms) => ms.iter().map(|(l, _)| l.clone()).collect(),
                None => self
                    .module_surface
                    .get(&v.target)
                    .map(|s| s.keys().cloned().collect())
                    .unwrap_or_else(|| self.declared_names(&v.target)),
            };
            for n in names {
                if let Some((_, prev_target)) = seen.iter().find(|(l, _)| *l == n).cloned() {
                    return Err(Error::plain(format!(
                        "the name `{n}` is imported from both {prev_target} and {}: rename one (a selection like `import {} ({n} as ...)`)",
                        v.target, v.target
                    )));
                }
                seen.push((n, v.target.clone()));
            }
        }
        // The export surface (empty items: the declared names, as before).
        let items = match self.module_exports.get(module) {
            Some(items) if !items.is_empty() => items.clone(),
            _ => return Ok(()),
        };
        let mut surface: HashMap<String, SurfaceName> = HashMap::new();
        for item in &items {
            let entries: Vec<(String, SurfaceName)> = match item {
                ast::ExportItem::Name(n) => match self.export_target(module, None, &n.text) {
                    Some(d) => vec![(n.text.clone(), d)],
                    None => {
                        return Err(Error::at(
                            n.span,
                            format!("`{}` is not a name in scope to export", n.text),
                        ))
                    }
                },
                ast::ExportItem::Member(path) => {
                    let parts = path.parts();
                    let (qual, name) = (
                        parts[..parts.len() - 1].join("."),
                        parts[parts.len() - 1],
                    );
                    match self.export_target(module, Some(&qual), name) {
                        Some(d) => vec![(name.to_string(), d)],
                        None => {
                            return Err(Error::at(
                                path.span(),
                                format!("`{qual}.{name}` is not an imported member to export"),
                            ))
                        }
                    }
                }
                ast::ExportItem::Module(q) => {
                    // A whole import re-exports by the imported
                    // module's full dotted name (`export module
                    // Data.List`) or by its qualifier (`export module
                    // N` for `import M as N`).
                    let view = self
                        .module_views
                        .get(module)
                        .into_iter()
                        .flatten()
                        .find(|v| !v.applied && v.target == q.text)
                        .cloned()
                        .or_else(|| {
                            self.module_views
                                .get(module)
                                .into_iter()
                                .flatten()
                                .find(|v| !v.applied && v.alias == q.text)
                                .cloned()
                        });
                    match view {
                        Some(v) => {
                            let mut out = Vec::new();
                            match &v.members {
                                Some(ms) => {
                                    for (l, s) in ms {
                                        if let Some(d) = self.surface_def(&v.target, s) {
                                            out.push((l.clone(), d));
                                        }
                                    }
                                }
                                None => match self.module_surface.get(&v.target) {
                                    Some(sm) => {
                                        for (l, d) in sm {
                                            out.push((l.clone(), d.clone()));
                                        }
                                    }
                                    None => {
                                        for n in self.declared_names(&v.target) {
                                            if let Some(d) =
                                                self.surface_def(&v.target, &n)
                                            {
                                                out.push((n, d));
                                            }
                                        }
                                    }
                                },
                            }
                            out
                        }
                        None => {
                            return Err(Error::at(
                                q.span,
                                format!("`{}` is not an imported module to export", q.text),
                            ))
                        }
                    }
                }
            };
            for (local, d) in entries {
                if let Some(prev) = surface.get(&local) {
                    if prev.module != d.module || prev.name != d.name {
                        return Err(Error::plain(format!(
                            "duplicate export `{local}` (naming both {} and {})",
                            prev.module, d.module
                        )));
                    }
                    continue;
                }
                surface.insert(local, d);
            }
        }
        self.module_surface.insert(module.to_string(), surface);
        Ok(())
    }

    /// The names a module declares (its default export surface).
    fn declared_names(&self, module: &str) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for (n, vs) in &self.values {
            if vs.iter().any(|v| v.module == module) && !out.contains(n) {
                out.push(n.clone());
            }
        }
        for (n, ts) in &self.types {
            if ts.iter().any(|t| t.module == module) && !out.contains(n) {
                out.push(n.clone());
            }
        }
        for (n, ss) in &self.sigs {
            if ss.iter().any(|s| s.module == module) && !out.contains(n) {
                out.push(n.clone());
            }
        }
        out
    }

    /// The definition an `export` item names: a name in the exporting
    /// module's scope (its own, or bare through an import), optionally
    /// qualified by the import's qualifier (`export A.b`).
    fn export_target(
        &self,
        module: &str,
        qual: Option<&str>,
        name: &str,
    ) -> Option<SurfaceName> {
        let candidates = match qual {
            Some(q) => self.scope_qual_candidates(q, name),
            None => self.scope_candidates(name),
        };
        for (dm, dn) in candidates {
            if dm == "<builtin>" {
                continue;
            }
            if let Some(d) = self.surface_def(&dm, &dn) {
                return Some(d);
            }
        }
        None
    }

    /// One module's `import data` bindings (surface grammar local
    /// amendments section 11): the extension-keyed converter runs at
    /// compile time over the named file and binds its converted value
    /// under the import's name. The converter's identity and the file's
    /// content hash are provenance; a converter failure is a build error
    /// naming the file and the converter.
    fn collect_data_imports(&mut self, module: &str, m: &ast::Module) -> Result<()> {
        let imports = self
            .module_data_imports
            .get(module)
            .cloned()
            .unwrap_or_default();
        for d in imports {
            let bound = fn_id(module, &d.name.text);
            if self.fns.iter().any(|f| f.id == bound) {
                // Re-collection with generated declarations: the binding
                // converted already.
                continue;
            }
            let converter = match d.file.rsplit_once('.').map(|(_, e)| e) {
                Some("json") => "json",
                Some("txt") => "txt",
                Some("bytes") => "bytes",
                other => {
                    return Err(Error::at(
                        d.span,
                        format!(
                            "no converter for `{}`: the built-in converters are keyed by extension (`json`, `txt`, `bytes`)",
                            other.unwrap_or(&d.file)
                        ),
                    ))
                }
            };
            // The file resolves beside the importing module's source
            // (capability limits: the named file is the only input).
            let mut path = std::path::PathBuf::new();
            if let Some(src) = self.module_files.get(module) {
                if let Some(dir) = std::path::Path::new(src).parent() {
                    path = dir.to_path_buf();
                }
            }
            if path.as_os_str().is_empty() {
                path = self
                    .fold_scx
                    .as_ref()
                    .map(|s| s.workdir.clone())
                    .unwrap_or_default();
            }
            path = path.join(&d.file);
            let fail = |why: String| {
                Error::at(
                    d.span,
                    format!("the `{converter}` converter could not convert {}: {why}", d.file),
                )
            };
            let bytes = std::fs::read(&path).map_err(|e| fail(e.to_string()))?;
            // The converter budget (P9 discipline): a file beyond the cap
            // is a build error, not a silently truncated import.
            if bytes.len() > 4 * 1024 * 1024 {
                return Err(fail(format!(
                    "the file is {} bytes, over the 4 MiB converter budget",
                    bytes.len()
                )));
            }
            let hash = crate::interface::content_hash_bytes(&bytes);
            // The bound type: a declared signature is the decoded type,
            // otherwise the converter's default (`Json`, `String`,
            // `Bytes`).
            let declared = self
                .top_schemes
                .get(&(module.to_string(), d.name.text.clone()))
                .cloned();
            if let Some(s) = &declared {
                if !s.params.is_empty()
                    || !s.ctx.is_empty()
                    || !s.row_params.is_empty()
                    || !s.region_params.is_empty()
                {
                    return Err(fail(format!(
                        "the declared type of {} must be monomorphic",
                        d.name.text
                    )));
                }
            }
            let ty = match &declared {
                Some(s) => s.ty.clone(),
                None => match converter {
                    "json" => Ty::Adt("Json".into(), Vec::new()),
                    "txt" => Ty::Adt("String".into(), Vec::new()),
                    _ => Ty::Bytes,
                },
            };
            let body = match converter {
                "bytes" => {
                    if self.uni.zonk(&ty) != Ty::Bytes {
                        return Err(fail(format!(
                            "the `bytes` converter produces a Bytes value, not {ty}"
                        )));
                    }
                    ECore::ConstBytes(bytes.clone())
                }
                "txt" => {
                    if self.uni.zonk(&ty) != Ty::Adt("String".into(), Vec::new()) {
                        return Err(fail(format!(
                            "the `txt` converter produces a String value, not {ty}"
                        )));
                    }
                    ECore::ConstString {
                        ctor: self.string_ctor_id(),
                        bytes: bytes.clone(),
                    }
                }
                _ => {
                    let v = parse_json(&bytes).map_err(fail)?;
                    self.json_const(&v, &self.uni.zonk(&ty), d.span)
                        .map_err(fail)?
                }
            };
            let name = d.name.text.clone();
            if declared.is_none() {
                let scheme = Scheme::mono(ty.clone());
                self.add_value(
                    ValueInfo {
                        module: module.to_string(),
                        name: name.clone(),
                        scheme: scheme.clone(),
                        kind: ValueKind::TopFn,
                    },
                    d.span,
                )?;
                self.top_schemes
                    .insert((module.to_string(), name.clone()), scheme);
            }
            self.fns.push(finalize_fn(
                bound,
                Vec::new(),
                body,
                None,
                None,
                crate::core::FnHints::default(),
                crate::core::FnMeta::default(),
            ));
            self.data_imports.push(DataImport {
                module: module.to_string(),
                name,
                file: d.file.clone(),
                converter: converter.to_string(),
                hash,
            });
        }
        Ok(())
    }

    /// The constant value of one JSON document at `ty` (surface grammar
    /// local amendments section 11): the `Json`-shaped tree at `Json`, a
    /// decoded value at a declared type (scalars, `String`, `List`,
    /// `Option`, and records decoded field by name).
    fn json_const(&mut self, v: &JsonVal, ty: &Ty, span: ast::Span) -> Result<ECore, String> {
        // The constructors resolve through the prelude surface to their
        // Data.* homes (see `prelude_ctor_id`); the map is resolved
        // once so the recursive descent below borrows no checker.
        let mut ctors: HashMap<String, crate::core::GlobalId> = HashMap::new();
        for n in [
            "JNull", "JBool", "JNum", "JFloat", "JStr", "JArr", "JObj", "JArrEnd", "JObjEnd", "Nil",
            "Cons", "None", "Some",
        ] {
            ctors.insert(n.to_string(), self.prelude_ctor_id(n));
        }
        let pctor = |n: &str| ECore::Ctor(ctors[n].clone());
        let unit = |n: &str, arg: ECore| -> ECore { app_spine(pctor(n), vec![arg]) };
        if let Ty::Adt(name, args) = ty {
            match (name.as_str(), args.as_slice()) {
                ("Json", []) => {
                    return Ok(match v {
                        JsonVal::Null => pctor("JNull"),
                        JsonVal::Bool(b) => unit("JBool", ECore::Bool(*b)),
                        JsonVal::Num(d) => {
                            if d.contains('.') || d.contains('e') || d.contains('E') {
                                let f: f64 = d
                                    .parse()
                                    .map_err(|_| format!("{d} is not a JSON number"))?;
                                unit("JFloat", ECore::Int(f.to_bits() as i64))
                            } else {
                                unit(
                                    "JNum",
                                    ECore::ConstBig {
                                        nat: false,
                                        digits: d.clone(),
                                    },
                                )
                            }
                        }
                        JsonVal::Str(s) => unit("JStr", self.string_const(s)),
                        JsonVal::Arr(xs) => {
                            let mut spine = pctor("JArrEnd");
                            for x in xs.iter().rev() {
                                let ic = self.json_const(x, ty, span)?;
                                spine = app_spine(pctor("JArr"), vec![ic, spine]);
                            }
                            spine
                        }
                        JsonVal::Obj(kvs) => {
                            let mut spine = pctor("JObjEnd");
                            for (k, x) in kvs.iter().rev() {
                                let ic = self.json_const(x, ty, span)?;
                                spine = app_spine(
                                    pctor("JObj"),
                                    vec![self.string_const(k), ic, spine],
                                );
                            }
                            spine
                        }
                    });
                }
                ("String", []) => {
                    return match v {
                        JsonVal::Str(s) => Ok(self.string_const(s)),
                        _ => Err(format!("a JSON string is required for String")),
                    };
                }
                ("List", [t]) => {
                    let xs = match v {
                        JsonVal::Arr(xs) => xs,
                        _ => return Err(format!("a JSON array is required for List")),
                    };
                    let t = t.clone();
                    let mut spine = pctor("Nil");
                    for x in xs.iter().rev() {
                        let ic = self.json_const(x, &t, span)?;
                        spine = app_spine(pctor("Cons"), vec![ic, spine]);
                    }
                    return Ok(spine);
                }
                ("Option", [t]) => {
                    return match v {
                        JsonVal::Null => Ok(pctor("None")),
                        _ => {
                            let t = t.clone();
                            let ic = self.json_const(v, &t, span)?;
                            Ok(app_spine(pctor("Some"), vec![ic]))
                        }
                    };
                }
                _ => {}
            }
            // A record decoded field by field (the declared-type form).
            if args.is_empty() {
                if let Some(t) = self.lookup_type(&[name]).ok() {
                    if let Some(fields) = t.fields.clone() {
                        let kvs = match v {
                            JsonVal::Obj(kvs) => kvs,
                            _ => {
                                return Err(format!(
                                    "a JSON object is required for the record {name}"
                                ))
                            }
                        };
                        let (cname, _) = t
                            .ctors
                            .first()
                            .ok_or_else(|| format!("the record {name} has no constructor"))?;
                        let vinfo = self
                            .qualified
                            .get(&(t.module.clone(), cname.clone()))
                            .cloned()
                            .ok_or_else(|| {
                                format!("the record {name} has no construction plan")
                            })?;
                        let field_tys = match &vinfo.kind {
                            ValueKind::Ctor { field_tys, field_erased, .. } => {
                                if field_erased.iter().any(|e| *e) {
                                    return Err(format!(
                                        "the record {name} has erased fields the converter cannot fill"
                                    ));
                                }
                                field_tys.clone()
                            }
                            _ => return Err(format!("the record {name} has no construction plan")),
                        };
                        let mut args: Vec<ECore> = Vec::new();
                        for (i, f) in fields.iter().enumerate() {
                            let x = kvs.iter().find(|(k, _)| k == f).map(|(_, x)| x);
                            let x = x.ok_or_else(|| {
                                format!("the JSON object has no field `{f}` of {name}")
                            })?;
                            args.push(self.json_const(x, &field_tys[i], span)?);
                        }
                        return Ok(app_spine(
                            ECore::Ctor(fn_id(&t.module, cname)),
                            args,
                        ));
                    }
                }
            }
            return Err(format!("the `json` converter cannot produce the type {ty}"));
        }
        match (v, ty) {
            (JsonVal::Str(s), Ty::Bytes) => Ok(ECore::ConstBytes(s.as_bytes().to_vec())),
            (JsonVal::Bool(b), Ty::Bool) => Ok(ECore::Bool(*b)),
            (JsonVal::Num(d), Ty::Nat | Ty::Int) => {
                if d.contains('.') || d.contains('e') || d.contains('E') {
                    return Err(format!("{d} is not an integer"));
                }
                if *ty == Ty::Nat && d.starts_with('-') {
                    return Err(format!("the JSON number {d} is out of range for Nat"));
                }
                Ok(ECore::ConstBig {
                    nat: *ty == Ty::Nat,
                    digits: d.clone(),
                })
            }
            (JsonVal::Num(d), Ty::Prim(s @ (Scalar::F64 | Scalar::F32))) => {
                let f: f64 =
                    d.parse().map_err(|_| format!("{d} is not a JSON number"))?;
                let w = if *s == Scalar::F64 {
                    f.to_bits() as i64
                } else {
                    (f as f32).to_bits() as i64
                };
                Ok(ECore::Int(w))
            }
            (JsonVal::Num(d), t @ Ty::Prim(_)) => {
                let w = self.literal_word(t, d, span).map_err(|e| e.message)?;
                Ok(ECore::Int(w))
            }
            _ => Err(format!("the `json` converter cannot produce the type {ty}")),
        }
    }

    /// The core id behind a prelude surface name: the facade's `export`
    /// set names the home definition (the prelude split put every
    /// constructor in its Data.* home), so the data-import desugarings
    /// resolve `String`/`Nil`/`Some` through the surface instead of
    /// assuming a home module. The facade is fixed compiler input, so
    /// the names below resolve or the compilation failed earlier.
    fn prelude_ctor_id(&self, name: &str) -> crate::core::GlobalId {
        let d = self
            .surface_def(&self.prelude_module, name)
            .expect("the prelude surface names the desugaring constructors");
        let v = self
            .qualified
            .get(&(d.module.clone(), d.name.clone()))
            .expect("the prelude surface names the desugaring constructors");
        fn_id(&v.module, &v.name)
    }

    /// The runtime `String` representation constructor (`StrBuf`, one
    /// field over a `Bytes` buffer): a string constant's value form
    /// names it, so the evaluator and code generation build the exact
    /// block the `String` patterns match. The constructor is
    /// module-private (the string immutability contract: no API hands
    /// out a string's backing); its home is fixed compiler input (the
    /// prelude split put it in `Data.String`).
    fn string_ctor_id(&self) -> crate::core::GlobalId {
        let key = ("Data.String".to_string(), "StrBuf".to_string());
        assert!(
            self.qualified.contains_key(&key),
            "the prelude declares the string representation constructor"
        );
        fn_id(&key.0, &key.1)
    }

    /// The runtime `String` constant of a string's decoded bytes: a
    /// runtime literal's elaboration (`check_terms`) and a compile-time
    /// string's reified constant share this form (the shared read-only
    /// data-section object; `String` is immutable, so sharing is
    /// semantics-transparent).
    fn string_const(&mut self, s: &str) -> ECore {
        ECore::ConstString {
            ctor: self.string_ctor_id(),
            bytes: s.as_bytes().to_vec(),
        }
    }

    fn set_scope(&mut self, module: &str) {
        self.build_scope(module, true);
    }

    /// Run one `generate do` block: its body is a compile-time
    /// computation; `emit` appends declarations, which are validated,
    /// macro-expanded and collected under ordinary rules (modules
    /// specification section 8).
    fn run_generator(&mut self, cx: &mut StageCx, idx: usize) -> Result<()> {
        let (module, stmts, span) = {
            let b = &self.generate_blocks[idx];
            (b.module.clone(), b.stmts.clone(), b.span)
        };
        let name = format!("$gen{idx}");
        let eq = ast::Equation {
            attrs: Vec::new(),
            name: ast::Name::new(&name, span),
            pats: Vec::new(),
            body: ast::Term::Do(stmts, span),
            locals: Vec::new(),
            span,
        };
        let id = fn_id(&module, &name);
        let scheme = Scheme::mono(Ty::Unit);
        self.set_scope(&module);
        let before = self.fns.len();
        self.compile_time = true;
        let r = self.check_equations(&name, &id, &scheme, std::slice::from_ref(&eq), Vec::new());
        self.compile_time = false;
        r?;
        for f in self.fns.drain(before..) {
            cx.fns.insert(f.id.0.clone(), f);
        }
        // Interpretation: the generator's own scope.
        cx.def_module = module.clone();
        cx.origin = module.clone();
        cx.unit_name = format!("generate block in {module}");
        cx.call = Some(span);
        cx.emitted = Vec::new();
        cx.snapshot = Vec::new();
        let empty = Vec::new();
        let value = self.eval_fn(cx, &id, empty)?;
        if !matches!(value, crate::core::Value::Unit) {
            return Err(Error::at(
                span,
                "a `generate do` block must produce Unit (its declarations arrive through `emit`)",
            ));
        }
        let emitted = std::mem::take(&mut cx.emitted);
        self.collect_emitted(cx, &module, emitted, span)?;
        Ok(())
    }

    /// Validate, expand and collect one generator's output (the ordinary
    /// rules of section 8 apply to generated declarations).
    fn collect_emitted(
        &mut self,
        cx: &mut StageCx,
        module: &str,
        mut decls: Vec<ast::Decl>,
        span: ast::Span,
    ) -> Result<()> {
        let origin = self
            .module_files
            .get(module)
            .cloned()
            .unwrap_or_else(|| module.to_string());
        let site = crate::lexer::ExpSite {
            name: cx.unit_name.clone(),
            call: span,
            origin,
            template: span,
            parent: None,
        };
        let _ = &site;
        for d in &mut decls {
            if matches!(
                d,
                ast::Decl::Generate { .. }
                    | ast::Decl::ValueSig { modifier: Some(_), .. }
                    | ast::Decl::Equation { modifier: Some(_), .. }
            ) {
                return Err(Error::at(
                    span,
                    "generated declarations are checked under ordinary rules: a generator cannot emit `generate`, `stage` or `macro` declarations",
                ));
            }
            // Provenance is stamped at quote construction (the template's
            // nodes carry it; spliced caller syntax keeps its own spans).
        }
        // Attributes revalidate at their attachment position when the
        // generated forms are collected (attribute annotations
        // specification section 6).
        let mut flat: Vec<(Vec<ast::Attribute>, ast::Decl)> = Vec::new();
        for d in &decls {
            self.flatten_attributes(d, &mut flat)?;
        }
        // Macro calls in generated code expand at emission.
        for (_, d) in &mut flat {
            self.expand_macros_in_decl(cx, module, d)?;
        }
        let decls_out: Vec<ast::Decl> = flat
            .iter()
            .map(|(attrs, d)| {
                if attrs.is_empty() {
                    d.clone()
                } else {
                    ast::Decl::Attributed {
                        attrs: attrs.clone(),
                        inner: Box::new(d.clone()),
                    }
                }
            })
            .collect();
        self.stage_emitted
            .entry(module.to_string())
            .or_default()
            .extend(decls_out.iter().cloned());
        // Collect the generated declarations now: later generators and
        // on-demand compilation resolve their names.
        let mut m = ast::Module {
            attrs: Vec::new(),
            name: ast::Name::new(module, span),
            params: Vec::new(),
            imports: Vec::new(),
            decls: decls_out,
            span,
        };
        // `collect_module` resolves imports against its argument; the
        // generated declarations have none.
        let _ = &mut m;
        let prelude_name = self.prelude_module.clone();
        let save_current = self.current_module.clone();
        let save_visible = self.visible.clone();
        let r = self.collect_module(&m, &prelude_name, &[]);
        self.current_module = save_current;
        self.visible = save_visible;
        r
    }

    /// Expand every macro call of the not-yet-compiled ordinary code, so
    /// the ordinary check runs on expanded syntax (section 8: expansions
    /// must parse and elaborate under this same language).
    fn expand_remaining_macros(&mut self, cx: &mut StageCx) -> Result<()> {
        let pending: Vec<(String, String)> = self
            .top_eqs
            .iter()
            .filter(|(m, n, e)| {
                !self.checked_early.contains(&(m.clone(), n.clone()))
                    && !self.staging.contains(&(m.clone(), n.clone()))
                    && term_has_macro_call(&e.body)
            })
            .map(|(m, n, _)| (m.clone(), n.clone()))
            .collect();
        for (module, name) in pending {
            let mut eqs: Vec<ast::Equation> = self
                .top_eqs
                .iter()
                .filter(|(m, n, _)| m == &module && n == &name)
                .map(|(_, _, e)| e.clone())
                .collect();
            for eq in &mut eqs {
                self.expand_macros_in_term(cx, &module, &mut eq.body)?;
                for l in &mut eq.locals {
                    if let ast::LocalDecl::Eq { body, .. }
                    | ast::LocalDecl::Bind { body, .. }
                    | ast::LocalDecl::Lazy { body, .. } = l
                    {
                        self.expand_macros_in_term(cx, &module, body)?;
                    }
                }
            }
            let mut i = 0;
            for slot in self.top_eqs.iter_mut() {
                if slot.0 == module && slot.1 == name {
                    slot.2 = eqs[i].clone();
                    i += 1;
                }
            }
        }
        Ok(())
    }

    /// Expand the macro calls of one term in place.
    fn expand_macros_in_term(
        &mut self,
        cx: &mut StageCx,
        module: &str,
        t: &mut ast::Term,
    ) -> Result<()> {
        match t {
            ast::Term::MacroCall { path, args, span } => {
                let (value, _ty) = self.eval_macro_call(cx, module, path, args, *span)?;
                match value {
                    crate::core::Value::Code(ast) => *t = ast,
                    other => {
                        return Err(Error::at(
                            *span,
                            format!(
                                "a macro invocation in term position must expand to code (`Code t`), but this one produced {}",
                                value_kind_name(&other)
                            ),
                        ))
                    }
                }
                Ok(())
            }
            ast::Term::App(f, args, _) => {
                self.expand_macros_in_term(cx, module, f)?;
                for a in args {
                    self.expand_macros_in_term(cx, module, a)?;
                }
                Ok(())
            }
            ast::Term::Lam(_, body, _)
            | ast::Term::Neg(body, _)
            | ast::Term::Proj(body, _, _)
            | ast::Term::Ann(body, _, _)
            | ast::Term::CpsWrap(body, _)
            | ast::Term::Quote { body, .. }
            | ast::Term::Splice { body, .. } => self.expand_macros_in_term(cx, module, body),
            ast::Term::Forget { value, .. } => {
                self.expand_macros_in_term(cx, module, value)
            }
            ast::Term::Narrow { value, witness, .. } => {
                self.expand_macros_in_term(cx, module, value)?;
                self.expand_macros_in_term(cx, module, witness)
            }
            ast::Term::Let(decls, body, _) | ast::Term::Letrec(decls, body, _) => {
                for d in decls {
                    self.expand_macros_in_local(cx, module, d)?;
                }
                self.expand_macros_in_term(cx, module, body)
            }
            ast::Term::If { cond: a, then: b, els: c, .. } => {
                self.expand_macros_in_term(cx, module, a)?;
                self.expand_macros_in_term(cx, module, b)?;
                self.expand_macros_in_term(cx, module, c)
            }
            ast::Term::Case(scrut, alts, _) => {
                self.expand_macros_in_term(cx, module, scrut)?;
                for alt in alts {
                    if let Some(g) = &mut alt.guard {
                        self.expand_macros_in_term(cx, module, g)?;
                    }
                    self.expand_macros_in_term(cx, module, &mut alt.body)?;
                }
                Ok(())
            }
            ast::Term::Infix(ts, _, _) | ast::Term::Tuple(ts, _) | ast::Term::List(ts, _) => {
                for x in ts {
                    self.expand_macros_in_term(cx, module, x)?;
                }
                Ok(())
            }
            ast::Term::Map(kvs, _) => {
                for (k, v) in kvs {
                    self.expand_macros_in_term(cx, module, k)?;
                    self.expand_macros_in_term(cx, module, v)?;
                }
                Ok(())
            }
            ast::Term::Range(start, step, end, _) => {
                self.expand_macros_in_term(cx, module, start)?;
                if let Some(x) = step {
                    self.expand_macros_in_term(cx, module, x)?;
                }
                self.expand_macros_in_term(cx, module, end)?;
                Ok(())
            }
            ast::Term::Record(_, fields, _) => {
                for (_, e) in fields {
                    self.expand_macros_in_term(cx, module, e)?;
                }
                Ok(())
            }
            ast::Term::Do(stmts, _) => {
                for s in stmts {
                    match s {
                        ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                            self.expand_macros_in_term(cx, module, term)?;
                        }
                        ast::DoStmt::Let { decls, .. } => {
                            for decl in decls {
                                self.expand_macros_in_local(cx, module, decl)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            ast::Term::Handle(e, h, _) => {
                self.expand_macros_in_term(cx, module, e)?;
                self.expand_macros_in_term(cx, module, h)
            }
            ast::Term::Handler(h) => {
                self.expand_macros_in_term(cx, module, &mut h.ret_body)?;
                for c in &mut h.clauses {
                    self.expand_macros_in_term(cx, module, &mut c.body)?;
                }
                Ok(())
            }
            ast::Term::With { value, body, .. } => {
                self.expand_macros_in_term(cx, module, value)?;
                self.expand_macros_in_term(cx, module, body)
            }
            ast::Term::CapApply { head, supplies, .. } => {
                self.expand_macros_in_term(cx, module, head)?;
                for (_, e) in supplies {
                    self.expand_macros_in_term(cx, module, e)?;
                }
                Ok(())
            }
            ast::Term::QuoteDecls { decls, .. } => {
                for d in decls {
                    self.expand_macros_in_decl(cx, module, d)?;
                }
                Ok(())
            }
            ast::Term::Var(_)
            | ast::Term::Con(_)
            | ast::Term::Int(_, _)
            | ast::Term::Str(_, _)
            | ast::Term::Char(_, _) | ast::Term::Sym(_, _)
            | ast::Term::Unit(_)
            | ast::Term::Hole(_)
            | ast::Term::SpliceHole { .. }
            | ast::Term::SeqOp(_, _) => Ok(()),
        }
    }

    fn expand_macros_in_local(
        &mut self,
        cx: &mut StageCx,
        module: &str,
        d: &mut ast::LocalDecl,
    ) -> Result<()> {
        match d {
            ast::LocalDecl::Sig { .. } => Ok(()),
            ast::LocalDecl::Eq { body, .. }
            | ast::LocalDecl::Bind { body, .. }
            | ast::LocalDecl::Lazy { body, .. } => self.expand_macros_in_term(cx, module, body),
        }
    }

    fn expand_macros_in_decl(
        &mut self,
        cx: &mut StageCx,
        module: &str,
        d: &mut ast::Decl,
    ) -> Result<()> {
        match d {
            ast::Decl::Equation { eq, .. } => {
                self.expand_macros_in_term(cx, module, &mut eq.body)?;
                for l in &mut eq.locals {
                    self.expand_macros_in_local(cx, module, l)?;
                }
                Ok(())
            }
            ast::Decl::Signature(s) => {
                for eq in &mut s.defaults {
                    self.expand_macros_in_term(cx, module, &mut eq.body)?;
                    for l in &mut eq.locals {
                        self.expand_macros_in_local(cx, module, l)?;
                    }
                }
                Ok(())
            }
            ast::Decl::Instance(i) => {
                for m in &mut i.methods {
                    self.expand_macros_in_term(cx, module, &mut m.body)?;
                    for l in &mut m.locals {
                        self.expand_macros_in_local(cx, module, l)?;
                    }
                }
                Ok(())
            }
            ast::Decl::Attributed { inner, .. } => self.expand_macros_in_decl(cx, module, inner),
            ast::Decl::Generate { stmts, .. } => {
                for s in stmts {
                    match s {
                        ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                            self.expand_macros_in_term(cx, module, term)?;
                        }
                        ast::DoStmt::Let { decls, .. } => {
                            for decl in decls {
                                self.expand_macros_in_local(cx, module, decl)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Evaluate one macro invocation to its code or syntax value (the
    /// expansion is inserted wherever the value is used).
    fn eval_macro_call(
        &mut self,
        cx: &mut StageCx,
        module: &str,
        path: &ast::VarPath,
        args: &ast::TokenTree,
        span: ast::Span,
    ) -> Result<(crate::core::Value, Ty)> {
        self.set_scope(module);
        let v = self.resolve_var(path, span)?;
        if !matches!(v.kind, ValueKind::MacroFn) {
            return Err(Error::at(
                span,
                format!("`{}` is not a macro (only `macro` definitions take `!(...)` token-tree arguments)", v.name),
            ));
        }
        let groups = args.arg_groups();
        if let Some(info) = self.macros.get(&(v.module.clone(), v.name.clone())) {
            if info.arity != groups.len() && !(info.arity <= 1 && groups.len() == 1) {
                return Err(Error::at(
                    span,
                    format!(
                        "macro `{}` takes {} token-tree argument(s), but the invocation supplies {}",
                        v.name,
                        info.arity,
                        groups.len()
                    ),
                ));
            }
        }
        let id = fn_id(&v.module, &v.name);
        // Each parameter takes one token-tree group. A `Name` parameter
        // (D46) reifies its group's name lexeme to a `Name` value at
        // call time; every other parameter takes the group as `Syntax`.
        let mut doms: Vec<Ty> = Vec::new();
        let mut t = self.uni.zonk(&v.scheme.ty);
        for _ in 0..groups.len() {
            match t {
                Ty::Fun(dom, _, _, cod) => {
                    doms.push((*dom).clone());
                    t = *cod;
                }
                Ty::Pi { dom, cod, .. } => {
                    doms.push((*dom).clone());
                    t = *cod;
                }
                _ => break,
            }
        }
        let mut vals = Vec::new();
        for (i, g) in groups.iter().enumerate() {
            if let Some(Ty::Adt(n, args)) = doms.get(i) {
                if n == "Name" && args.is_empty() {
                    vals.push(crate::core::Value::Name(name_of_group(&g.tokens, span)?));
                    continue;
                }
            }
            vals.push(crate::core::Value::Syntax {
                tokens: g.tokens.clone(),
                origin: module.to_string(),
            });
        }
        let save_origin = cx.origin.clone();
        let save_def = cx.def_module.clone();
        let save_call = cx.call;
        let save_unit = cx.unit_name.clone();
        let save_parent = cx.parent;
        cx.origin = module.to_string();
        cx.call = Some(span);
        cx.unit_name = format!("macro `{}`", v.name);
        cx.parent = save_call;
        let r = self.eval_fn(cx, &id, vals);
        cx.origin = save_origin;
        cx.def_module = save_def;
        cx.call = save_call;
        cx.unit_name = save_unit;
        cx.parent = save_parent;
        let value = r?;
        let ty = match &value {
            crate::core::Value::Code(_) => Ty::Adt("Code".into(), vec![self.uni.fresh()]),
            crate::core::Value::Decls(_) => Ty::Adt("Declarations".into(), Vec::new()),
            crate::core::Value::Syntax { .. } => Ty::Adt("Syntax".into(), Vec::new()),
            crate::core::Value::Name(_) => Ty::Adt("Name".into(), Vec::new()),
            other => {
                return Err(Error::at(
                    span,
                    format!(
                        "a macro must produce `Code`, `Declarations`, `Syntax` or `Name`, but `{}` produced {}",
                        v.name,
                        value_kind_name(other)
                    ),
                ))
            }
        };
        Ok((value, ty))
    }
}

/// A `Name` argument's spelling: the group's single name lexeme, a tick
/// name quote (`'foo`/`''T`) or a bare name (the Template Haskell
/// name-quote correspondence, D123).
pub(crate) fn name_of_group(tokens: &[crate::lexer::Token], span: ast::Span) -> Result<String> {
    use crate::lexer::Tok;
    let names: Vec<&str> = tokens
        .iter()
        .filter(|t| !t.virtual_)
        .filter_map(|t| match &t.tok {
            Tok::SymLit(s) | Tok::TypeNameLit(s) | Tok::VarId(s) | Tok::ConId(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();
    let mut non_names = tokens
        .iter()
        .filter(|t| !t.virtual_)
        .filter(|t| !matches!(t.tok, Tok::SymLit(_) | Tok::TypeNameLit(_) | Tok::VarId(_) | Tok::ConId(_)));
    match (names.as_slice(), non_names.next()) {
        ([name], None) => Ok((*name).to_string()),
        _ => Err(Error::at(
            span,
            "a `Name` argument is a single name: a tick name quote (`'foo`) or a bare name",
        )),
    }
}

/// The spelling a reified name hole stands for (D46): the name term a
/// `Name`/`Sym` value or name quote reifies to.
fn name_of_term(t: &ast::Term) -> Result<String> {
    match t {
        ast::Term::Var(p) => Ok(p.name.text.clone()),
        ast::Term::Con(p) => Ok(p
            .segments
            .last()
            .map(|s| s.text.clone())
            .unwrap_or_default()),
        ast::Term::Sym(s, _) => Ok(s.clone()),
        other => Err(Error::at(
            other.span(),
            "a name splice must name one declaration: a `Name`, `Sym` or name quote",
        )),
    }
}

/// The spelling a name hole's value stands for (D46).
fn name_of_value(v: &crate::core::Value, span: ast::Span) -> Result<String> {
    match v {
        crate::core::Value::Name(s) | crate::core::Value::Sym(s) => Ok(s.clone()),
        crate::core::Value::Syntax { tokens, .. } => name_of_group(tokens, span),
        other => Err(Error::at(
            span,
            format!(
                "a name splice must name one declaration, found {}",
                value_kind_name(other)
            ),
        )),
    }
}

/// Fill a template declaration's name splices (D46): each `$( e )` name
/// slot takes the spelling its hole reified to. A post-substitution
/// nested quotation carries its reified name term directly.
fn splice_decl_names(d: &mut ast::Decl, terms: &[ast::Term]) -> Result<()> {
    match d {
        ast::Decl::Attributed { inner, .. } => splice_decl_names(inner, terms),
        ast::Decl::NameSpliced { name_tm, inner } => {
            let text = match &**name_tm {
                ast::Term::SpliceHole { idx, .. } => match terms.get(*idx) {
                    Some(t) => name_of_term(t)?,
                    None => return Err(Error::plain("a name splice has no value")),
                },
                other => name_of_term(other)?,
            };
            let span = name_tm.span();
            let mut decl = (**inner).clone();
            match &mut decl {
                ast::Decl::ValueSig { name, .. } => *name = ast::Name::new(text, span),
                ast::Decl::Equation { eq, .. } => eq.name = ast::Name::new(text, span),
                _ => {
                    return Err(Error::at(
                        span,
                        "a name splice names a value declaration (`name :: ty` or an equation)",
                    ))
                }
            }
            *d = decl;
            Ok(())
        }
        _ => Ok(()),
    }
}

/// The name of a value's kind, for diagnostics (secrets stay redacted).
fn value_kind_name(v: &crate::core::Value) -> &'static str {
    use crate::core::Value::*;
    match v {
        Word(_) => "a machine word",
        Big { nat: true, .. } => "a `Nat`",
        Big { nat: false, .. } => "an `Int`",
        Bytes(_) => "a `Bytes` buffer",
        Sym(_) => "an interned symbol",
        Bool(_) => "a `Bool`",
        Unit => "Unit",
        Adt { .. } => "a data value",
        Dict { .. } => "a dictionary",
        Clos { .. } => "a function",
        CtorFn { .. } => "a constructor",
        PrimFn { .. } => "a primitive",
        Code(_) => "code",
        Decls(_) => "declarations",
        Syntax { .. } => "syntax",
        Name(_) => "a name",
        Str(_) => "a `String`",
        Secret { .. } => "a secret",
        Db(_) => "a database introspection result",
        Schema(_) => "a schema snapshot",
        Cap(_) => "a capability",
        StageFn { .. } => "a compile-time operation",
    }
}

/// Does `t` contain a macro invocation (the staging-work test)?
fn term_has_macro_call(t: &ast::Term) -> bool {
    let mut found = false;
    fn walk(t: &ast::Term, found: &mut bool) {
        if *found {
            return;
        }
        match t {
            ast::Term::MacroCall { .. } => *found = true,
            ast::Term::App(f, args, _) => {
                walk(f, found);
                for a in args {
                    walk(a, found);
                }
            }
            ast::Term::Lam(_, b, _)
            | ast::Term::Neg(b, _)
            | ast::Term::Proj(b, _, _)
            | ast::Term::Ann(b, _, _)
            | ast::Term::CpsWrap(b, _)
            | ast::Term::Quote { body: b, .. }
            | ast::Term::Splice { body: b, .. } => walk(b, found),
            ast::Term::Forget { value, .. } => walk(value, found),
            ast::Term::Narrow { value, witness, .. } => {
                walk(value, found);
                walk(witness, found);
            }
            ast::Term::Let(decls, b, _) | ast::Term::Letrec(decls, b, _) => {
                for d in decls {
                    match d {
                        ast::LocalDecl::Eq { body, .. }
                        | ast::LocalDecl::Bind { body, .. }
                        | ast::LocalDecl::Lazy { body, .. } => walk(body, found),
                        ast::LocalDecl::Sig { .. } => {}
                    }
                }
                walk(b, found);
            }
            ast::Term::If { cond: a, then: b, els: c, .. } => {
                walk(a, found);
                walk(b, found);
                walk(c, found);
            }
            ast::Term::Case(s, alts, _) => {
                walk(s, found);
                for alt in alts {
                    walk(&alt.body, found);
                }
            }
            ast::Term::Infix(ts, _, _) | ast::Term::Tuple(ts, _) | ast::Term::List(ts, _) => {
                for x in ts {
                    walk(x, found);
                }
            }
            ast::Term::Map(kvs, _) => {
                for (k, v) in kvs {
                    walk(k, found);
                    walk(v, found);
                }
            }
            ast::Term::Range(start, step, end, _) => {
                walk(start, found);
                if let Some(x) = step {
                    walk(x, found);
                }
                walk(end, found);
            }
            ast::Term::Record(_, fields, _) => {
                for (_, e) in fields {
                    walk(e, found);
                }
            }
            ast::Term::Do(stmts, _) => {
                for s in stmts {
                    match s {
                        ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                            walk(term, found)
                        }
                        ast::DoStmt::Let { decls, .. } => {
                            for decl in decls {
                                if let ast::LocalDecl::Eq { body, .. }
                                | ast::LocalDecl::Bind { body, .. }
                                | ast::LocalDecl::Lazy { body, .. } = decl
                                {
                                    walk(body, found);
                                }
                            }
                        }
                    }
                }
            }
            ast::Term::Handle(a, b, _) => {
                walk(a, found);
                walk(b, found);
            }
            ast::Term::Handler(h) => {
                walk(&h.ret_body, found);
                for c in &h.clauses {
                    walk(&c.body, found);
                }
            }
            ast::Term::With { value, body, .. } => {
                walk(value, found);
                walk(body, found);
            }
            ast::Term::CapApply { head, supplies, .. } => {
                walk(head, found);
                for (_, e) in supplies {
                    walk(e, found);
                }
            }
            ast::Term::QuoteDecls { .. }
            | ast::Term::Var(_)
            | ast::Term::Con(_)
            | ast::Term::Int(_, _)
            | ast::Term::Str(_, _)
            | ast::Term::Char(_, _) | ast::Term::Sym(_, _)
            | ast::Term::Unit(_)
            | ast::Term::Hole(_)
            | ast::Term::SpliceHole { .. }
            | ast::Term::SeqOp(_, _) => {}
        }
    }
    walk(t, &mut found);
    found
}

/// Binder names of a unit's syntax, for the dependency graph's local-name
/// filter.
fn collect_unit_binders(key: &UnitKey, checker: &Checker, out: &mut Vec<String>) {
    let mut push_pat = |p: &ast::Pat, out: &mut Vec<String>| {
        let mut names = Vec::new();
        pat_names(p, &mut names);
        out.extend(names);
    };
    match key {
        UnitKey::Def(m, n) | UnitKey::Ord(m, n) => {
            for (mod_, name, eq) in &checker.top_eqs {
                if mod_ == m && name == n {
                    for p in &eq.pats {
                        push_pat(p, out);
                    }
                    for l in &eq.locals {
                        match l {
                            ast::LocalDecl::Sig { name, .. } => out.push(name.text.clone()),
                            ast::LocalDecl::Eq { name, pats, .. } => {
                                out.push(name.text.clone());
                                for p in pats {
                                    push_pat(p, out);
                                }
                            }
                            ast::LocalDecl::Bind { pat, .. } | ast::LocalDecl::Lazy { pat, .. } => {
                                push_pat(pat, out)
                            }
                        }
                    }
                }
            }
        }
        UnitKey::Gen(i) => {
            for stmt in &checker.generate_blocks[*i].stmts {
                match stmt {
                    ast::DoStmt::Bind { pat, .. } => push_pat(pat, out),
                    ast::DoStmt::Let { decls, .. } => {
                        for decl in decls {
                            match decl {
                                ast::LocalDecl::Sig { name, .. } => out.push(name.text.clone()),
                                ast::LocalDecl::Eq { name, .. } => out.push(name.text.clone()),
                                ast::LocalDecl::Bind { pat, .. }
                                | ast::LocalDecl::Lazy { pat, .. } => push_pat(pat, out),
                            }
                        }
                    }
                    ast::DoStmt::Expr { .. } => {}
                }
            }
        }
    }
}

/// The declared name of a declaration, when it has one.
fn decl_name(d: &ast::Decl) -> Option<&ast::Name> {
    match d {
        ast::Decl::ValueSig { name, .. } => Some(name),
        ast::Decl::Equation { eq, .. } => Some(&eq.name),
        ast::Decl::Attributed { inner, .. } => decl_name(inner),
        _ => None,
    }
}

/// Give every span of a generated term the expansion's provenance (modules
/// specification section 8: source maps retain expansion provenance), so
/// diagnostics in generated code trace back through the expansion.
fn stamp_term_spans(t: &mut ast::Term, site: &crate::lexer::ExpSite) {
    let stamp = |s: &mut ast::Span| {
        *s = crate::lexer::record_expansion(crate::lexer::ExpSite {
            name: site.name.clone(),
            call: site.call,
            origin: site.origin.clone(),
            template: *s,
            parent: site.parent,
        })
    };
    match t {
        ast::Term::Var(p) => {
            for q in &mut p.quals {
                stamp(&mut q.span);
            }
            stamp(&mut p.name.span);
        }
        ast::Term::Con(p) => {
            for s in &mut p.segments {
                stamp(&mut s.span);
            }
        }
        ast::Term::Int(_, s)
        | ast::Term::Str(_, s)
        | ast::Term::Char(_, s) | ast::Term::Sym(_, s)
        | ast::Term::Unit(s)
        | ast::Term::Hole(s)
        | ast::Term::SpliceHole { span: s, .. }
        | ast::Term::SeqOp(_, s) => stamp(s),
        ast::Term::App(f, args, s) => {
            stamp(s);
            stamp_term_spans(f, site);
            for a in args {
                stamp_term_spans(a, site);
            }
        }
        ast::Term::Lam(_, body, s) | ast::Term::Neg(body, s) | ast::Term::Proj(body, _, s) => {
            stamp(s);
            stamp_term_spans(body, site);
        }
        ast::Term::Let(_, body, s)
        | ast::Term::Letrec(_, body, s)
        | ast::Term::Ann(body, _, s)
        | ast::Term::CpsWrap(body, s) => {
            stamp(s);
            stamp_term_spans(body, site);
        }
        ast::Term::Forget { value, span: s, .. } => {
            stamp(s);
            stamp_term_spans(value, site);
        }
        ast::Term::Narrow { value, witness, span: s, .. } => {
            stamp(s);
            stamp_term_spans(value, site);
            stamp_term_spans(witness, site);
        }
        ast::Term::If { cond: a, then: b, els: c, span: s, .. } => {
            stamp(s);
            stamp_term_spans(a, site);
            stamp_term_spans(b, site);
            stamp_term_spans(c, site);
        }
        ast::Term::Case(scrut, alts, s) => {
            stamp(s);
            stamp_term_spans(scrut, site);
            for alt in alts {
                if let Some(g) = &mut alt.guard {
                    stamp_term_spans(g, site);
                }
                stamp_term_spans(&mut alt.body, site);
            }
        }
        ast::Term::Infix(ts, _, s) | ast::Term::Tuple(ts, s) | ast::Term::List(ts, s) => {
            stamp(s);
            for x in ts {
                stamp_term_spans(x, site);
            }
        }
        ast::Term::Map(kvs, s) => {
            stamp(s);
            for (k, v) in kvs {
                stamp_term_spans(k, site);
                stamp_term_spans(v, site);
            }
        }
        ast::Term::Range(start, step, end, s) => {
            stamp(s);
            stamp_term_spans(start, site);
            if let Some(x) = step {
                stamp_term_spans(x, site);
            }
            stamp_term_spans(end, site);
        }
        ast::Term::Record(_, fields, s) => {
            stamp(s);
            for (_, e) in fields {
                stamp_term_spans(e, site);
            }
        }
        ast::Term::Do(stmts, s) => {
            stamp(s);
            for st in stmts {
                match st {
                    ast::DoStmt::Bind { term, span, .. } | ast::DoStmt::Expr { term, span } => {
                        stamp(span);
                        stamp_term_spans(term, site);
                    }
                    ast::DoStmt::Let { span, .. } => stamp(span),
                }
            }
        }
        ast::Term::Handle(a, b, s) => {
            stamp(s);
            stamp_term_spans(a, site);
            stamp_term_spans(b, site);
        }
        ast::Term::Handler(h) => {
            stamp(&mut h.span);
            stamp_term_spans(&mut h.ret_body, site);
            for c in &mut h.clauses {
                stamp(&mut c.span);
                stamp_term_spans(&mut c.body, site);
            }
        }
        ast::Term::With { span, value, body, .. } => {
            stamp(span);
            stamp_term_spans(value, site);
            stamp_term_spans(body, site);
        }
        ast::Term::CapApply { span, head, supplies, .. } => {
            stamp(span);
            stamp_term_spans(head, site);
            for (_, e) in supplies {
                stamp_term_spans(e, site);
            }
        }
        ast::Term::Quote { body, span } | ast::Term::Splice { body, span } => {
            stamp(span);
            stamp_term_spans(body, site);
        }
        ast::Term::QuoteDecls { decls, span } => {
            stamp(span);
            for d in decls {
                stamp_decl_spans(d, site);
            }
        }
        ast::Term::MacroCall { span, .. } => stamp(span),
    }
}

/// Expansion provenance for a generated declaration's terms.
fn stamp_decl_spans(d: &mut ast::Decl, site: &crate::lexer::ExpSite) {
    match d {
        ast::Decl::Attributed { inner, .. } => stamp_decl_spans(inner, site),
        ast::Decl::Equation { eq, .. } => {
            stamp_term_spans(&mut eq.body, site);
            for l in &mut eq.locals {
                match l {
                    ast::LocalDecl::Eq { body, .. }
                    | ast::LocalDecl::Bind { body, .. }
                    | ast::LocalDecl::Lazy { body, .. } => stamp_term_spans(body, site),
                    ast::LocalDecl::Sig { .. } => {}
                }
            }
        }
        ast::Decl::Signature(s) => {
            for eq in &mut s.defaults {
                stamp_term_spans(&mut eq.body, site);
            }
        }
        ast::Decl::Instance(i) => {
            for m in &mut i.methods {
                stamp_term_spans(&mut m.body, site);
            }
        }
        ast::Decl::Generate { stmts, .. } => {
            for s in stmts {
                match s {
                    ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                        stamp_term_spans(term, site)
                    }
                    ast::DoStmt::Let { .. } => {}
                }
            }
        }
        _ => {}
    }
}

// ------------------------------------------------- staging: compile-time evaluation

use crate::core::big::Big;
use crate::core::{Cap, StageOp, Value};

/// The `Value` of a mathematical constant.
fn big_value(b: Big, nat: bool) -> Value {
    Value::Big { nat, digits: b.to_dec() }
}

/// The `Big` behind a mathematical value.
fn big_of(v: &Value, what: &str) -> Result<Big> {
    match v {
        Value::Big { digits, .. } => Big::from_dec(digits)
            .ok_or_else(|| Error::plain(format!("internal error: malformed {what} constant"))),
        other => Err(Error::plain(format!(
            "expected {what} at compile time, found {}",
            value_kind_name(other)
        ))),
    }
}

/// Width and signedness of a machine scalar (primitive types
/// specification section 2).
fn scalar_shape(name: &str) -> Option<(u32, bool)> {
    Some(match name {
        "I8" => (8, true),
        "U8" | "Char" => (8, false),
        "I16" => (16, true),
        "U16" => (16, false),
        "I32" => (32, true),
        "U32" => (32, false),
        "I64" | "Isize" => (64, true),
        "U64" | "Usize" => (64, false),
        _ => return None,
    })
}

/// Reduce a machine word to its type's representation.
fn mask_to(bits: u32, signed: bool, v: i64) -> i64 {
    if bits >= 64 {
        return v;
    }
    let m = (1i64 << bits) - 1;
    let r = v & m;
    if signed && r >> (bits - 1) & 1 == 1 {
        r | !m
    } else {
        r
    }
}

fn word_of(v: &Value, what: &str) -> Result<i64> {
    match v {
        Value::Word(w) => Ok(*w),
        Value::Bool(b) => Ok(*b as i64),
        Value::Unit => Ok(0),
        other => Err(Error::plain(format!(
            "expected {what} at compile time, found {}",
            value_kind_name(other)
        ))),
    }
}

fn str_of(v: &Value, what: &str) -> Result<String> {
    match v {
        Value::Str(s) => Ok(s.clone()),
        Value::Secret { .. } => Err(Error::plain(
            "a secret value must not be embedded in generated source, logs, or snapshots (modules specification section 10)",
        )),
        other => Err(Error::plain(format!(
            "expected {what} at compile time, found {}",
            value_kind_name(other)
        ))),
    }
}

impl Checker {
    /// Evaluate a compiled function at compile time, applied to `args`.
    fn eval_fn(&mut self, cx: &mut StageCx, id: &GlobalId, args: Vec<Value>) -> Result<Value> {
        let f = self.global_value(cx, &id.0)?;
        self.apply_value(cx, f, args)
    }

    /// The compile-time value of a global name.
    fn global_value(&mut self, cx: &mut StageCx, id: &str) -> Result<Value> {
        if let Some(op) = id.strip_prefix("<builtin>.Stage.") {
            let op = StageOp::from_name(op)
                .ok_or_else(|| Error::plain(format!("internal error: unknown stage operation {id}")))?;
            return Ok(Value::StageFn { op, args: Vec::new() });
        }
        if let Some(f) = cx.fns.get(id) {
            return Ok(self.closure_of(f));
        }
        // Constructors carry their tag in the value table.
        if let Some((m, n)) = id.rsplit_once('.') {
            if let Some(vs) = self.values.get(n) {
                for v in vs {
                    if v.module == m {
                        if let ValueKind::Ctor { tag, runtime_arity, .. } = &v.kind {
                            return Ok(Value::CtorFn {
                                unit: id.to_string(),
                                tag: *tag,
                                arity: *runtime_arity,
                                args: Vec::new(),
                            });
                        }
                    }
                }
            }
        }
        self.ensure_fn(cx, id)?;
        let f = cx
            .fns
            .get(id)
            .ok_or_else(|| Error::plain(format!("internal error: no code for {id}")))?;
        Ok(self.closure_of(f))
    }

    fn closure_of(&self, f: &crate::core::FnDef) -> Value {
        if let Some((tag, arity)) = f.ctor {
            return Value::CtorFn {
                unit: f.id.0.clone(),
                tag,
                arity,
                args: Vec::new(),
            };
        }
        if let Some(op) = f.prim {
            return Value::PrimFn { op, args: Vec::new() };
        }
        Value::Clos {
            fn_id: f.id.clone(),
            env: Vec::new(),
            args: Vec::new(),
            arity: f.params.len(),
        }
    }

    /// Make `id`'s code available to interpretation, compiling it on
    /// demand (modules specification section 9: a prerequisite is compiled
    /// before its use).
    fn ensure_fn(&mut self, cx: &mut StageCx, id: &str) -> Result<()> {
        if cx.fns.contains_key(id) {
            return Ok(());
        }
        if let Some(f) = self.fns.iter().find(|f| f.id.0 == id).cloned() {
            cx.fns.insert(id.to_string(), f);
            return Ok(());
        }
        let parts: Vec<&str> = id.split('.').collect();
        if parts.len() >= 3 && parts[parts.len() - 1].starts_with("lam") {
            let parent = format!("{}.{}", parts[0], parts[1]);
            self.ensure_fn(cx, &parent)?;
            if cx.fns.contains_key(id) {
                return Ok(());
            }
        }
        if parts.len() == 3 {
            // An instance method: compile the instance on demand.
            let (module, inst) = (parts[0].to_string(), parts[1].to_string());
            self.ensure_instance(cx, &module, &inst)?;
            if let Some(f) = self.fns.iter().find(|f| f.id.0 == id).cloned() {
                cx.fns.insert(id.to_string(), f);
                return Ok(());
            }
        }
        if parts.len() == 2 {
            let (module, name) = (parts[0].to_string(), parts[1].to_string());
            self.compile_ordinary_on_demand(cx, &module, &name)?;
        }
        if cx.fns.contains_key(id) {
            return Ok(());
        }
        if let Some(f) = self.fns.iter().find(|f| f.id.0 == id).cloned() {
            cx.fns.insert(id.to_string(), f);
            return Ok(());
        }
        Err(Error::plain(format!(
            "compile-time evaluation needs the definition of `{id}`, which is not available (a generator cannot use code that only exists after generation)"
        )))
    }

    /// Compile an ordinary definition for compile-time use: expand its
    /// macro calls, then check it under ordinary rules (modules
    /// specification section 8).
    fn compile_ordinary_on_demand(
        &mut self,
        cx: &mut StageCx,
        module: &str,
        name: &str,
    ) -> Result<()> {
        let key = (module.to_string(), name.to_string());
        if self.checked_early.contains(&key) {
            return Ok(());
        }
        if self.staging.contains(&key) {
            return self.compile_unit(cx, module, name);
        }
        let mut eqs: Vec<ast::Equation> = self
            .top_eqs
            .iter()
            .filter(|(m, n, _)| m == module && n == name)
            .map(|(_, _, e)| e.clone())
            .collect();
        if eqs.is_empty() {
            return Err(Error::plain(format!("no definition for {name}")));
        }
        for eq in &mut eqs {
            self.expand_macros_in_term(cx, module, &mut eq.body)?;
            for l in &mut eq.locals {
                self.expand_macros_in_local(cx, module, l)?;
            }
        }
        let scheme = self.top_schemes.get(&key).cloned().ok_or_else(|| {
            Error::plain(format!("missing signature for {name} (definitions state their types)"))
        })?;
        self.set_scope(module);
        let id = fn_id(module, name);
        let before = self.fns.len();
        self.check_equations(name, &id, &scheme, &eqs, Vec::new())?;
        for f in self.fns[before..].iter() {
            cx.fns.insert(f.id.0.clone(), f.clone());
        }
        self.checked_early.insert(key);
        Ok(())
    }

    /// Compile a class instance on demand (its dictionary methods run at
    /// compile time like any other code).
    fn ensure_instance(&mut self, cx: &mut StageCx, module: &str, name: &str) -> Result<()> {
        let base = fn_id(module, name);
        if self.instance_defs.iter().any(|i| i.id == base) {
            return Ok(());
        }
        let pending = self
            .instance_defs_pending
            .iter()
            .position(|(m, i)| m == module && i.name.text == name)
            .map(|i| self.instance_defs_pending.remove(i));
        let Some((m, mut inst)) = pending else {
            return Err(Error::plain(format!(
                "compile-time evaluation needs the instance {module}.{name}, which is not available"
            )));
        };
        // Instance methods may invoke macros: expand first.
        let mut as_decl = ast::Decl::Instance(inst.clone());
        self.expand_macros_in_decl(cx, module, &mut as_decl)?;
        if let ast::Decl::Instance(i) = as_decl {
            inst = i;
        }
        self.set_scope(module);
        let before = self.fns.len();
        self.check_instance(&m, &inst)?;
        for f in self.fns[before..].iter() {
            cx.fns.insert(f.id.0.clone(), f.clone());
        }
        Ok(())
    }

    /// Evaluate one core expression in `frame` (locals by id).
    fn eval_core(
        &mut self,
        cx: &mut StageCx,
        e: &crate::core::Core,
        frame: &mut Vec<Value>,
    ) -> Result<Value> {
        cx.steps = cx.steps.saturating_sub(1);
        if cx.steps == 0 {
            return Err(Error::plain(
                "compile-time evaluation exceeded its step budget (8000000 steps); a generator must terminate (the staging termination policy)",
            ));
        }
        use crate::core::Core;
        Ok(match e {
            Core::Int(i) => Value::Word(*i),
            Core::ConstBig { nat, digits } => Value::Big {
                nat: *nat,
                digits: digits.clone(),
            },
            Core::ConstBytes(b) => Value::Bytes(std::rc::Rc::new(std::cell::RefCell::new(
                b.clone(),
            ))),
            Core::ConstString { ctor, bytes } => {
                // The representation constructor applied to a buffer
                // word: the same `Value` shape a constructor call
                // produces, so the `String` patterns match it.
                let cv = self.global_value(cx, &ctor.0)?;
                let buf =
                    std::rc::Rc::new(std::cell::RefCell::new(bytes.clone()));
                self.apply_value(cx, cv, vec![Value::Bytes(buf)])?
            }
            Core::ConstSym(s) => Value::Sym(s.clone()),
            Core::Bool(b) => Value::Bool(*b),
            Core::Unit => Value::Unit,
            Core::Var(l) => frame.get(l.0).cloned().ok_or_else(|| {
                Error::plain(format!("internal error: unbound local {} at compile time", l.0))
            })?,
            Core::Global(g) => {
                let v = self.global_value(cx, &g.0)?;
                self.deref_value(cx, v)?
            }
            Core::Ctor(g) => {
                let v = self.global_value(cx, &g.0)?;
                self.deref_value(cx, v)?
            }
            Core::Foreign(_) => {
                return Err(Error::plain(
                    "a foreign import cannot run at compile time (it needs target code and a linked foreign library)",
                ))
            }
            Core::SpliceVal(_) => {
                return Err(Error::plain(
                    "internal error: a splice position reached the staging interpreter outside its quotation",
                ))
            }
            Core::Prim(op) => {
                let v = Value::PrimFn { op: *op, args: Vec::new() };
                self.deref_value(cx, v)?
            }
            Core::DictProj(d, i) => match self.eval_core(cx, d, frame)? {
                Value::Dict { methods } => methods.get(*i).cloned().ok_or_else(|| {
                    Error::plain("internal error: dictionary projection out of range")
                })?,
                other => {
                    return Err(Error::plain(format!(
                        "internal error: dictionary projection from {}",
                        value_kind_name(&other)
                    )))
                }
            },
            Core::Proj(d, i) => match self.eval_core(cx, d, frame)? {
                Value::Adt { fields, .. } => fields.get(i.slot).cloned().ok_or_else(|| {
                    Error::plain("internal error: record projection out of range")
                })?,
                other => {
                    return Err(Error::plain(format!(
                        "internal error: record projection from {}",
                        value_kind_name(&other)
                    )))
                }
            },
            Core::InstDict(g, args) => {
                let (module, name) = g.0.rsplit_once('.').unwrap_or(("", &g.0));
                self.ensure_instance(cx, module, name)?;
                let def = self
                    .instance_defs
                    .iter()
                    .find(|i| i.id == *g)
                    .cloned()
                    .ok_or_else(|| {
                        Error::plain(format!("internal error: no instance code for {}", g.0))
                    })?;
                let mut subs = Vec::new();
                for a in args {
                    subs.push(self.eval_core(cx, a, frame)?);
                }
                let mut methods = Vec::new();
                for m in &def.methods {
                    let f = self
                        .fns
                        .iter()
                        .find(|f| f.id == *m)
                        .cloned()
                        .ok_or_else(|| {
                            Error::plain(format!("internal error: no method code for {}", m.0))
                        })?;
                    let arity = f.params.len();
                    methods.push(Value::Clos {
                        fn_id: m.clone(),
                        env: Vec::new(),
                        args: subs.clone(),
                        arity,
                    });
                }
                Value::Dict { methods }
            }
            Core::App(f, args) => {
                let fv = self.eval_core(cx, f, frame)?;
                let mut vals = Vec::new();
                for a in args {
                    vals.push(self.eval_core(cx, a, frame)?);
                }
                self.apply_value(cx, fv, vals)?
            }
            Core::Lam { fn_id, env } => {
                let mut captured = Vec::new();
                for a in env {
                    captured.push(self.eval_core(cx, a, frame)?);
                }
                let arity = match cx.fns.get(&fn_id.0) {
                    Some(f) => f.params.len(),
                    None => usize::MAX,
                };
                Value::Clos {
                    fn_id: fn_id.clone(),
                    env: captured,
                    args: Vec::new(),
                    arity,
                }
            }
            Core::Let(l, v, body) => {
                let val = self.eval_core(cx, v, frame)?;
                let saved = set_local(frame, l.0, val);
                let r = self.eval_core(cx, body, frame);
                restore_local(frame, l.0, saved);
                r?
            }
            Core::RefSet(l, v) => {
                let val = self.eval_core(cx, v, frame)?;
                set_local(frame, l.0, val);
                Value::Unit
            }
            Core::If(c, t, f) => {
                if word_of(&self.eval_core(cx, c, frame)?, "a condition")? != 0 {
                    self.eval_core(cx, &t.body, frame)?
                } else {
                    self.eval_core(cx, &f.body, frame)?
                }
            }
            Core::Case { scrut, alts } => {
                let s = self.eval_core(cx, scrut, frame)?;
                let mut result = None;
                for alt in alts {
                    let mut saved: Vec<(usize, Option<Value>)> = Vec::new();
                    let matched = match &alt.pat {
                        crate::core::CPat::Wild => true,
                        crate::core::CPat::Var(l) => {
                            saved.push((l.0, set_local(frame, l.0, s.clone())));
                            true
                        }
                        crate::core::CPat::Con { tag, fields } => match &s {
                            Value::Adt { tag: t, fields: fs, .. } if t == tag => {
                                for (l, v) in fields.iter().zip(fs.iter()) {
                                    saved.push((l.0, set_local(frame, l.0, v.clone())));
                                }
                                true
                            }
                            _ => false,
                        },
                    };
                    if matched {
                        result = Some(self.eval_core(cx, &alt.body, frame)?);
                        for (l, old) in saved.into_iter().rev() {
                            restore_local(frame, l, old);
                        }
                        break;
                    }
                    for (l, old) in saved.into_iter().rev() {
                        restore_local(frame, l, old);
                    }
                }
                result.ok_or_else(|| {
                    Error::plain("the compile-time computation faulted (a match ran out)")
                })?
            }
            Core::Quote { template, holes, .. } => {
                self.construct_quote(cx, template, holes, frame)?
            }
            Core::DeclsQuote { template, holes, name_holes } => {
                self.construct_decls(cx, template, holes, name_holes, frame)?
            }
            Core::SyntaxLit(tokens) => Value::Syntax {
                tokens: tokens.clone(),
                origin: cx.origin.clone(),
            },
            Core::StrLit(s) => Value::Str(s.clone()),
            Core::BoxAlloc | Core::BoxInit(_, _) | Core::BoxGet(_) => {
                return Err(Error::plain(
                    "a recursive `letrec` value cannot be evaluated at compile time (the staging evaluator has no fixpoint boxes)",
                ))
            }
            Core::Demand(_) | Core::LazyAlloc | Core::LazyInit(_, _) => {
                return Err(Error::plain(
                    "lazy evaluation (`delay`, memo cells) is not available at compile time in this slice",
                ))
            }
            Core::Fault => {
                return Err(Error::plain(
                    "the compile-time computation faulted (the fault path ran)",
                ))
            }
        })
    }

    /// A nullary definition is a value: referencing a zero-arity function,
    /// constructor, or primitive runs it (top-level values are 0-ary
    /// thunks in the uniform word model).
    fn deref_value(&mut self, cx: &mut StageCx, v: Value) -> Result<Value> {
        let zero = match &v {
            Value::Clos { args, arity, .. } => *arity == 0 && args.is_empty(),
            Value::CtorFn { args, arity, .. } => *arity == 0 && args.is_empty(),
            Value::PrimFn { op, args } => op.arity() == 0 && args.is_empty(),
            Value::StageFn { op, args } => stage_op_arity(*op) == 0 && args.is_empty(),
            _ => false,
        };
        if zero {
            self.apply_value(cx, v, Vec::new())
        } else {
            Ok(v)
        }
    }

    /// Apply `f` to `args` at compile time, currying.
    fn apply_value(&mut self, cx: &mut StageCx, f: Value, mut args: Vec<Value>) -> Result<Value> {
        cx.steps = cx.steps.saturating_sub(1);
        if cx.steps == 0 {
            return Err(Error::plain(
                "compile-time evaluation exceeded its step budget (8000000 steps); a generator must terminate (the staging termination policy)",
            ));
        }
        // An empty application spine is the value itself: a nullary
        // constructor reaches application position as `App(Ctor, [])`
        // (the runtime passes it through identically).
        if args.is_empty() {
            match f {
                Value::Clos { .. } | Value::CtorFn { .. } | Value::PrimFn { .. }
                | Value::StageFn { .. } => {}
                other => return Ok(other),
            }
        }
        match f {
            Value::Clos { fn_id, mut env, args: mut acc, arity } => {
                acc.append(&mut args);
                let arity = if arity == usize::MAX {
                    self.ensure_fn(cx, &fn_id.0)?;
                    cx.fns.get(&fn_id.0).map(|f| f.params.len()).unwrap_or(0)
                } else {
                    arity
                };
                if acc.len() < arity {
                    return Ok(Value::Clos { fn_id, env, args: acc, arity });
                }
                let (now, rest) = acc.split_at(arity);
                env.extend(now.iter().cloned());
                self.ensure_fn(cx, &fn_id.0)?;
                let body = cx
                    .fns
                    .get(&fn_id.0)
                    .map(|f| f.body.clone())
                    .ok_or_else(|| Error::plain(format!("internal error: no code for {}", fn_id.0)))?;
                // A function's templates resolve at its definition site,
                // and its literal syntax was written in its module. The
                // unit name carries into expansion provenance.
                let (save_def, save_origin, save_unit) = (
                    cx.def_module.clone(),
                    cx.origin.clone(),
                    cx.unit_name.clone(),
                );
                {
                    // Strip a hoisted-lambda suffix, then the name.
                    let base = if fn_id
                        .0
                        .rsplit_once('.')
                        .map(|(_, last)| last.starts_with("lam"))
                        .unwrap_or(false)
                    {
                        fn_id.0.rsplit_once('.').map(|(p, _)| p.to_string()).unwrap_or_default()
                    } else {
                        fn_id.0.clone()
                    };
                    if let Some((m, n)) = base.rsplit_once('.') {
                        cx.def_module = m.to_string();
                        cx.origin = m.to_string();
                        cx.unit_name = if self.macros.contains_key(&(m.to_string(), n.to_string()))
                        {
                            format!("macro `{n}`")
                        } else if self.staging.contains(&(m.to_string(), n.to_string())) {
                            format!("stage definition `{n}`")
                        } else {
                            format!("definition `{n}`")
                        };
                    }
                }
                let mut frame = env;
                let r = self.eval_core(cx, &body, &mut frame);
                cx.def_module = save_def;
                cx.origin = save_origin;
                cx.unit_name = save_unit;
                let r = r?;
                if rest.is_empty() {
                    Ok(r)
                } else {
                    self.apply_value(cx, r, rest.to_vec())
                }
            }
            Value::PrimFn { op, args: mut acc } => {
                acc.append(&mut args);
                if acc.len() < op.arity() {
                    return Ok(Value::PrimFn { op, args: acc });
                }
                let (now, rest) = acc.split_at(op.arity());
                let r = self.eval_prim(cx, op, now)?;
                if rest.is_empty() {
                    Ok(r)
                } else {
                    self.apply_value(cx, r, rest.to_vec())
                }
            }
            Value::CtorFn { unit, tag, arity, args: mut acc } => {
                acc.append(&mut args);
                if acc.len() < arity {
                    return Ok(Value::CtorFn { unit, tag, arity, args: acc });
                }
                let (now, rest) = acc.split_at(arity);
                let r = Value::Adt { ctor: unit, tag, fields: now.to_vec() };
                if rest.is_empty() {
                    Ok(r)
                } else {
                    self.apply_value(cx, r, rest.to_vec())
                }
            }
            Value::StageFn { op, args: mut acc } => {
                acc.append(&mut args);
                let want = stage_op_arity(op);
                if acc.len() < want {
                    return Ok(Value::StageFn { op, args: acc });
                }
                let (now, rest) = acc.split_at(want);
                let r = self.eval_stage_op(cx, op, now)?;
                if rest.is_empty() {
                    Ok(r)
                } else {
                    self.apply_value(cx, r, rest.to_vec())
                }
            }
            other => Err(Error::plain(format!(
                "cannot apply {} at compile time",
                value_kind_name(&other)
            ))),
        }
    }

    /// Primitive operation semantics for compile-time evaluation.
    fn eval_prim(&mut self, cx: &mut StageCx, op: PrimOp, args: &[Value]) -> Result<Value> {
        use crate::core::{ArithMode, ArithOp, BitOp, CmpOp, IntOp, NatOp, PrimOp, ShiftOp};
        match op {
            PrimOp::Nat(nat) => {
                let a = big_of(&args[0], "`Nat`")?;
                let b = if args.len() > 1 { big_of(&args[1], "`Nat`")? } else { Big::zero() };
                Ok(match nat {
                    NatOp::Add => big_value(a.add(&b), true),
                    NatOp::Mul => big_value(a.mul(&b), true),
                    NatOp::Sub => {
                        let r = a.sub(&b);
                        if r.neg && !r.is_zero() {
                            return Err(Error::plain(
                                "`Nat` subtraction below zero faults at compile time (the runtime fault path is exit 71)",
                            ));
                        }
                        big_value(r, true)
                    }
                    NatOp::Div | NatOp::Rem => {
                        let (q, r) = a.divmod(&b).ok_or_else(|| {
                            Error::plain(
                                "division by zero faults at compile time (the runtime fault path is exit 72)",
                            )
                        })?;
                        big_value(if matches!(nat, NatOp::Div) { q } else { r }, true)
                    }
                    NatOp::Cmp => Value::Word(big_cmp_word(&a, &b)),
                    NatOp::FromI64 => {
                        let w = word_of(&args[0], "a machine integer")?;
                        if w < 0 {
                            return Err(Error::plain("`natFromI64` faults on a negative input"));
                        }
                        big_value(Big::from_i64(w), true)
                    }
                    NatOp::ToI64 => {
                        let w = a.to_i64().ok_or_else(|| {
                            Error::plain("`natToI64` faults when the value leaves `I64` range")
                        })?;
                        Value::Word(w)
                    }
                })
            }
            PrimOp::Int(int) => {
                let a = big_of(&args[0], "`Int`")?;
                let b = if args.len() > 1 { big_of(&args[1], "`Int`")? } else { Big::zero() };
                Ok(match int {
                    IntOp::Add => big_value(a.add(&b), false),
                    IntOp::Sub => big_value(a.sub(&b), false),
                    IntOp::Mul => big_value(a.mul(&b), false),
                    IntOp::Neg => big_value(a.negate(), false),
                    IntOp::Div | IntOp::Rem => {
                        let (q, r) = a.divmod(&b).ok_or_else(|| {
                            Error::plain(
                                "division by zero faults at compile time (the runtime fault path is exit 72)",
                            )
                        })?;
                        big_value(if matches!(int, IntOp::Div) { q } else { r }, false)
                    }
                    IntOp::Cmp => Value::Word(big_cmp_word(&a, &b)),
                    IntOp::FromI64 => big_value(Big::from_i64(word_of(&args[0], "a machine integer")?), false),
                    IntOp::ToI64 => Value::Word(a.to_i64().ok_or_else(|| {
                        Error::plain("`intToI64` faults when the value leaves `I64` range")
                    })?),
                })
            }
            PrimOp::EqBool => Ok(Value::Bool(
                (word_of(&args[0], "`Bool`")? != 0) == (word_of(&args[1], "`Bool`")? != 0),
            )),
            PrimOp::NeBool => Ok(Value::Bool(
                (word_of(&args[0], "`Bool`")? != 0) != (word_of(&args[1], "`Bool`")? != 0),
            )),
            PrimOp::NotBool => Ok(Value::Bool(word_of(&args[0], "`Bool`")? == 0)),
            PrimOp::AndBool => Ok(Value::Bool(
                word_of(&args[0], "`Bool`")? != 0 && word_of(&args[1], "`Bool`")? != 0,
            )),
            PrimOp::OrBool => Ok(Value::Bool(
                word_of(&args[0], "`Bool`")? != 0 || word_of(&args[1], "`Bool`")? != 0,
            )),
            PrimOp::CharToU32 => Ok(Value::Word(word_of(&args[0], "a `Char`")?)),
            PrimOp::U32ToChar => Ok(Value::Word(word_of(&args[0], "a `U32`")?)),
            PrimOp::Bound { ty, max } => {
                let (bits, signed) = scalar_shape(ty.name()).unwrap_or((64, true));
                let v = if max {
                    if signed {
                        (1i64 << (bits - 1)) - 1
                    } else {
                        mask_to(bits, false, -1)
                    }
                } else if signed {
                    -(1i64 << (bits - 1))
                } else {
                    0
                };
                Ok(Value::Word(v))
            }
            PrimOp::Cmp { ty, op: cmp } => {
                let (bits, signed) = scalar_shape(ty.name()).unwrap_or((64, true));
                let a = word_of(&args[0], "a machine word")?;
                let b = word_of(&args[1], "a machine word")?;
                let (a, b) = if signed {
                    (mask_to(bits, true, a), mask_to(bits, true, b))
                } else {
                    let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                    ((a as u64 & mask) as i64, (b as u64 & mask) as i64)
                };
                let ord = if signed {
                    a.cmp(&b)
                } else {
                    (a as u64).cmp(&(b as u64))
                };
                Ok(Value::Bool(match cmp {
                    CmpOp::Eq => ord.is_eq(),
                    CmpOp::Ne => ord.is_ne(),
                    CmpOp::Lt => ord.is_lt(),
                    CmpOp::Le => ord.is_le(),
                    CmpOp::Gt => ord.is_gt(),
                    CmpOp::Ge => ord.is_ge(),
                }))
            }
            PrimOp::Arith { ty, op: aop, mode } => {
                let (bits, signed) = scalar_shape(ty.name()).unwrap_or((64, true));
                let a = word_of(&args[0], "a machine word")?;
                let b = word_of(&args[1], "a machine word")?;
                let raw = match aop {
                    ArithOp::Add => a.wrapping_add(b),
                    ArithOp::Sub => a.wrapping_sub(b),
                    ArithOp::Mul => a.wrapping_mul(b),
                };
                Ok(Value::Word(match mode {
                    ArithMode::Wrapping => mask_to(bits, signed, raw),
                    ArithMode::Saturating => {
                        let w = mask_to(bits, signed, raw);
                        if signed {
                            let (lo, hi) = (-(1i64 << (bits - 1)), (1i64 << (bits - 1)) - 1);
                            // Re-do in wider space to detect overflow.
                            let wide = match aop {
                                ArithOp::Add => (a as i128) + (b as i128),
                                ArithOp::Sub => (a as i128) - (b as i128),
                                ArithOp::Mul => (a as i128) * (b as i128),
                            };
                            if wide > hi as i128 {
                                hi
                            } else if wide < lo as i128 {
                                lo
                            } else {
                                w
                            }
                        } else {
                            let max = if bits >= 64 { u64::MAX as i128 } else { (1i128 << bits) - 1 };
                            let wide = match aop {
                                ArithOp::Add => (a as u64 as i128) + (b as u64 as i128),
                                ArithOp::Sub => (a as u64 as i128) - (b as u64 as i128),
                                ArithOp::Mul => (a as u64 as i128) * (b as u64 as i128),
                            };
                            if wide > max {
                                max as i64
                            } else if wide < 0 {
                                0
                            } else {
                                w
                            }
                        }
                    }
                    ArithMode::Checked => {
                        return Err(Error::plain(
                            "checked machine arithmetic returns `Option` and is not available at compile time in this slice",
                        ))
                    }
                }))
            }
            PrimOp::Neg { ty, mode } => {
                let (bits, signed) = scalar_shape(ty.name()).unwrap_or((64, true));
                let a = word_of(&args[0], "a machine word")?;
                Ok(Value::Word(match mode {
                    ArithMode::Wrapping => mask_to(bits, signed, a.wrapping_neg()),
                    ArithMode::Saturating => {
                        let lo = -(1i64 << (bits - 1));
                        if signed && a == lo {
                            (1i64 << (bits - 1)) - 1
                        } else {
                            mask_to(bits, signed, a.wrapping_neg())
                        }
                    }
                    ArithMode::Checked => {
                        return Err(Error::plain(
                            "checked machine arithmetic returns `Option` and is not available at compile time in this slice",
                        ))
                    }
                }))
            }
            PrimOp::Shift { ty, op: sop, checked } => {
                let (bits, signed) = scalar_shape(ty.name()).unwrap_or((64, true));
                let a = word_of(&args[0], "a machine word")?;
                let n = word_of(&args[1], "a shift count")?;
                if checked && (n < 0 || n >= bits as i64) {
                    return Err(Error::plain(
                        "a checked shift out of its width returns `None` and is not available at compile time in this slice",
                    ));
                }
                let n = n.rem_euclid(bits as i64) as u32;
                let r = match sop {
                    ShiftOp::Shl => a.wrapping_shl(n),
                    ShiftOp::Shr => {
                        if signed {
                            a.wrapping_shr(n)
                        } else {
                            mask_to(bits, false, a).wrapping_shr(n)
                        }
                    }
                };
                Ok(Value::Word(mask_to(bits, signed, r)))
            }
            PrimOp::Rotate { ty, right } => {
                let (bits, signed) = scalar_shape(ty.name()).unwrap_or((64, true));
                let a = word_of(&args[0], "a machine word")? as u64;
                let n = word_of(&args[1], "a rotate count")?.rem_euclid(bits as i64) as u32;
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let a = a & mask;
                let r = if right {
                    ((a >> n) | (a << (bits - n))) & mask
                } else {
                    ((a << n) | (a >> (bits - n))) & mask
                };
                Ok(Value::Word(mask_to(bits, signed, r as i64)))
            }
            PrimOp::Bit { ty, op: bop } => {
                let (bits, signed) = scalar_shape(ty.name()).unwrap_or((64, true));
                let a = word_of(&args[0], "a machine word")?;
                let b = word_of(&args[1], "a machine word")?;
                let r = match bop {
                    BitOp::And => a & b,
                    BitOp::Or => a | b,
                    BitOp::Xor => a ^ b,
                };
                Ok(Value::Word(mask_to(bits, signed, r)))
            }
            PrimOp::BitNot { ty } => {
                let (bits, signed) = scalar_shape(ty.name()).unwrap_or((64, true));
                let a = word_of(&args[0], "a machine word")?;
                Ok(Value::Word(mask_to(bits, signed, !a)))
            }
            PrimOp::Bytes(bop) => {
                use crate::core::BytesOp;
                // The `Bytes` primitives speak `Nat` words (the intrinsic
                // units convert through `cinch_nat_to_i64` and back), so the
                // compile-time values are `Nat` words too.
                fn bytes_word(v: &Value, what: &str) -> Result<i64> {
                    let b = big_of(v, what)?;
                    b.to_i64().ok_or_else(|| {
                        Error::plain(format!("{what} leaves `i64` range at compile time"))
                    })
                }
                Ok(match bop {
                    BytesOp::New => {
                        let n = bytes_word(&args[0], "a `Bytes` length")?;
                        if n < 0 {
                            return Err(Error::plain(
                                "`bytesNew` faults on a negative length at compile time (the runtime fault path is exit 71)",
                            ));
                        }
                        Value::Bytes(std::rc::Rc::new(std::cell::RefCell::new(vec![
                            0u8;
                            n as usize
                        ])))
                    }
                    BytesOp::Len => match &args[0] {
                        Value::Bytes(b) => {
                            big_value(Big::from_i64(b.borrow().len() as i64), true)
                        }
                        other => {
                            return Err(Error::plain(format!(
                                "expected a `Bytes` buffer at compile time, found {}",
                                value_kind_name(other)
                            )))
                        }
                    },
                    BytesOp::Get => {
                        let i = bytes_word(&args[1], "a `Bytes` index")?;
                        match &args[0] {
                            Value::Bytes(b) => {
                                let buf = b.borrow();
                                let v = if i < 0 {
                                    None
                                } else {
                                    buf.get(i as usize).copied()
                                };
                                match v {
                                    Some(v) => big_value(Big::from_i64(v as i64), true),
                                    None => {
                                        return Err(Error::plain(
                                            "`bytesGet` out of bounds faults at compile time (the runtime fault path is exit 71)",
                                        ))
                                    }
                                }
                            }
                            other => {
                                return Err(Error::plain(format!(
                                    "expected a `Bytes` buffer at compile time, found {}",
                                    value_kind_name(other)
                                )))
                            }
                        }
                    }
                    BytesOp::Set => {
                        let i = bytes_word(&args[1], "a `Bytes` index")?;
                        let v = bytes_word(&args[2], "a `Bytes` value")?;
                        let Value::Bytes(b) = &args[0] else {
                            return Err(Error::plain(format!(
                                "expected a `Bytes` buffer at compile time, found {}",
                                value_kind_name(&args[0])
                            )));
                        };
                        let byte = u8::try_from(v).map_err(|_| {
                            Error::plain(
                                "`bytesSet` on a value outside 0..=255 faults at compile time (the runtime fault path is exit 71)",
                            )
                        })?;
                        let mut buf = b.borrow_mut();
                        let slot = if i < 0 {
                            None
                        } else {
                            buf.get_mut(i as usize)
                        };
                        let Some(slot) = slot else {
                            return Err(Error::plain(
                                "`bytesSet` out of bounds faults at compile time (the runtime fault path is exit 71)",
                            ));
                        };
                        *slot = byte;
                        Value::Unit
                    }
                    BytesOp::Copy => {
                        let di = bytes_word(&args[2], "a `Bytes` index")?;
                        let si = bytes_word(&args[3], "a `Bytes` index")?;
                        let n = bytes_word(&args[4], "a `Bytes` count")?;
                        let (Value::Bytes(dst), Value::Bytes(src)) = (&args[0], &args[1]) else {
                            return Err(Error::plain(
                                "expected two `Bytes` buffers at compile time for `bytesCopy`",
                            ));
                        };
                        let oob = || {
                            Error::plain(
                                "`bytesCopy` out of bounds faults at compile time (the runtime fault path is exit 71)",
                            )
                        };
                        if di < 0 || si < 0 || n < 0 {
                            return Err(oob());
                        }
                        let (di, si, n) = (di as usize, si as usize, n as usize);
                        // Read the source range out first: `dst` and `src`
                        // may be the same buffer (the runtime copy is a
                        // `memmove`).
                        let chunk: Vec<u8> = {
                            let s = src.borrow();
                            let end = si.checked_add(n).filter(|&e| e <= s.len()).ok_or_else(oob)?;
                            s[si..end].to_vec()
                        };
                        let mut d = dst.borrow_mut();
                        let end = di.checked_add(n).filter(|&e| e <= d.len()).ok_or_else(oob)?;
                        d[di..end].copy_from_slice(&chunk);
                        Value::Unit
                    }
                    BytesOp::Compare => {
                        let (Value::Bytes(a), Value::Bytes(b)) = (&args[0], &args[1]) else {
                            return Err(Error::plain(
                                "expected two `Bytes` buffers at compile time for `bytesCompare`",
                            ));
                        };
                        let ord = a.borrow().as_slice().cmp(b.borrow().as_slice());
                        Value::Word(ord as i64)
                    }
                })
            }
            PrimOp::Sym(sop) => {
                use crate::core::SymOp;
                fn sym_of(v: &Value) -> Result<&String> {
                    match v {
                        Value::Sym(s) => Ok(s),
                        other => Err(Error::plain(format!(
                            "expected an interned symbol at compile time, found {}",
                            value_kind_name(other)
                        ))),
                    }
                }
                Ok(match sop {
                    SymOp::Intern => {
                        let Value::Bytes(b) = &args[0] else {
                            return Err(Error::plain(format!(
                                "expected a `Bytes` name at compile time, found {}",
                                value_kind_name(&args[0])
                            )));
                        };
                        let s = std::str::from_utf8(&b.borrow())
                            .map_err(|_| {
                                Error::plain(
                                    "`symIntern` on a name that is not UTF-8 faults at compile time (the runtime fault path is exit 71)",
                                )
                            })?
                            .to_string();
                        // The table is keyed by name: the value stands
                        // for the one interned cell of the spelling.
                        Value::Sym(s)
                    }
                    SymOp::Name => {
                        let s = sym_of(&args[0])?;
                        Value::Bytes(std::rc::Rc::new(std::cell::RefCell::new(
                            s.as_bytes().to_vec(),
                        )))
                    }
                    SymOp::Eq => Value::Bool(sym_of(&args[0])? == sym_of(&args[1])?),
                    SymOp::Cmp => {
                        let o = sym_of(&args[0])?.as_bytes().cmp(sym_of(&args[1])?.as_bytes());
                        Value::Word(match o {
                            std::cmp::Ordering::Less => -1,
                            std::cmp::Ordering::Equal => 0,
                            std::cmp::Ordering::Greater => 1,
                        })
                    }
                })
            }
            PrimOp::BitScan { .. } => Err(Error::plain(
                "bit-scan primitives are not available at compile time in this slice",
            )),
            _ => Err(Error::plain(format!(
                "primitive `{}` is not available at compile time in this slice",
                op.symbol()
            ))),
        }
    }

    /// The compile-time capability API (modules specification section 10):
    /// capabilities determine permitted targets; effects record that I/O
    /// occurs; inputs are captured for the build identity.
    fn eval_stage_op(&mut self, cx: &mut StageCx, op: StageOp, args: &[Value]) -> Result<Value> {
        match op {
            StageOp::Snapshot => {
                let name = str_of(&args[0], "a snapshot name")?;
                let k = args[1].clone();
                cx.snapshot.push(name);
                let r = self.apply_value(cx, k, vec![Value::Cap(Cap::Root)]);
                cx.snapshot.pop();
                r
            }
            StageOp::WithReadFiles | StageOp::WithWriteFiles | StageOp::WithProcess => {
                require_cap(&args[0], Cap::Root, "the root capability")?;
                let cap = match op {
                    StageOp::WithReadFiles => Cap::File { write: false },
                    StageOp::WithWriteFiles => Cap::File { write: true },
                    _ => Cap::Proc,
                };
                self.apply_value(cx, args[1].clone(), vec![Value::Cap(cap)])
            }
            StageOp::ReadFile => {
                let cap = match &args[0] {
                    Value::Cap(c @ Cap::File { .. }) => *c,
                    other => {
                        return Err(Error::plain(format!(
                            "`readFile` needs a file capability, found {}",
                            value_kind_name(other)
                        )))
                    }
                };
                let _ = cap;
                let path = str_of(&args[1], "a file path")?;
                let contents = self.stage_read(cx, &path)?;
                Ok(Value::Str(contents))
            }
            StageOp::WriteFile => {
                require_file_write(&args[0])?;
                let path = str_of(&args[1], "a file path")?;
                let contents = str_of(&args[2], "file contents")?;
                let full = cx.workdir.join(&path);
                if let Some(parent) = full.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        Error::plain(format!("cannot create {}: {e}", parent.display()))
                    })?;
                }
                std::fs::write(&full, &contents)
                    .map_err(|e| Error::plain(format!("cannot write {}: {e}", full.display())))?;
                Ok(Value::Unit)
            }
            StageOp::Run => {
                require_cap(&args[0], Cap::Proc, "a subprocess capability")?;
                let tool = str_of(&args[1], "a tool name")?;
                let mut argv: Vec<String> = Vec::new();
                let mut cur = &args[2];
                loop {
                    match cur {
                        Value::Adt { tag: 0, .. } => break,
                        Value::Adt { tag: 1, fields, .. } if fields.len() == 2 => {
                            argv.push(str_of(&fields[0], "a tool argument")?);
                            cur = &fields[1];
                        }
                        other => {
                            return Err(Error::plain(format!(
                                "`run` takes a list of string arguments, found {}",
                                value_kind_name(other)
                            )))
                        }
                    }
                }
                self.stage_run(cx, &tool, &argv).map(Value::Str)
            }
            StageOp::Secret => {
                let name = str_of(&args[0], "a secret name")?;
                let value = std::env::var(&name).map_err(|_| {
                    Error::plain(format!(
                        "the secret `{name}` is not available in the build environment (it is referenced by name only; its value never enters generated source or snapshots)"
                    ))
                })?;
                self.record_input(
                    cx,
                    StageInput {
                        snapshot: cx.snapshot.last().cloned().unwrap_or_default(),
                        kind: "secret".to_string(),
                        name: name.clone(),
                        hash: String::new(),
                        captured: String::new(),
                        live: false,
                    },
                );
                Ok(Value::Secret { name, value })
            }
            StageOp::WithDatabase => {
                require_cap(&args[0], Cap::Root, "the root capability")?;
                let (dsn_name, dsn) = match &args[1] {
                    Value::Secret { name, value } => (name.clone(), value.clone()),
                    other => {
                        return Err(Error::plain(format!(
                            "`withDatabase` takes a `secret` connection string, found {}",
                            value_kind_name(other)
                        )))
                    }
                };
                let _op = &args[2];
                let k = args[3].clone();
                // Modules specification section 11 delegates database
                // introspection to a tool run over the subprocess
                // capability (decision D45): the snapshot comes from
                // running a tool, not from a new network stack (decision
                // D65). The credential travels in the tool's environment
                // and never enters the captured input.
                let tool = cx.schema_tool.clone().ok_or_else(|| {
                    Error::plain(
                        "no schema introspection tool is configured for `Stage.withDatabase` (modules specification section 11 delegates introspection to a tool run over the subprocess capability; decision D65)",
                    )
                })?;
                let raw = self.stage_run_env(cx, &tool, &[], &dsn_name, &dsn)?;
                self.apply_value(cx, k, vec![Value::Db(raw)])
            }
            // A schema snapshot's structural value: the stage API
            // synthesizes it, so it carries no constructor name (it is
            // never reifiable as a constant).
            StageOp::ReadSchema => {
                Ok(Value::Adt { ctor: String::new(), tag: 0, fields: Vec::new() })
            }
            StageOp::Introspect => {
                let raw = match &args[0] {
                    Value::Db(s) => s.clone(),
                    other => {
                        return Err(Error::plain(format!(
                            "`introspectConsistently` takes a database introspection result, found {}",
                            value_kind_name(other)
                        )))
                    }
                };
                let snap = self.interpret_schema(&raw)?;
                Ok(Value::Schema(snap))
            }
            StageOp::CodecDerive => {
                let snap = match &args[0] {
                    Value::Schema(s) => s.clone(),
                    other => {
                        return Err(Error::plain(format!(
                            "`CodecGenerator.derive` takes a schema snapshot, found {}",
                            value_kind_name(other)
                        )))
                    }
                };
                Ok(Value::Decls(self.derive_codecs(&snap)?))
            }
            StageOp::Emit => {
                let mut decls = match &args[0] {
                    Value::Decls(ds) => ds.clone(),
                    other => {
                        return Err(Error::plain(format!(
                            "`emit` takes declarations, found {}",
                            value_kind_name(other)
                        )))
                    }
                };
                cx.emitted.append(&mut decls);
                Ok(Value::Unit)
            }
            StageOp::QuoteNat | StageOp::QuoteInt | StageOp::QuoteBool => {
                let span = cx.call.unwrap_or(ast::Span { line: 0, col: 0 });
                match op {
                    StageOp::QuoteNat => {
                        let b = big_of(&args[0], "`Nat`")?;
                        Ok(Value::Code(ast::Term::Int(b.to_dec(), span)))
                    }
                    StageOp::QuoteInt => {
                        let b = big_of(&args[0], "`Int`")?;
                        Ok(Value::Code(ast::Term::Int(b.to_dec(), span)))
                    }
                    _ => {
                        let b = word_of(&args[0], "`Bool`")? != 0;
                        let name = if b { "True" } else { "False" };
                        Ok(Value::Code(ast::Term::Con(ast::ConPath {
                            segments: vec![ast::Name::new(name, span)],
                        })))
                    }
                }
            }
            StageOp::StrNat => {
                let s = str_of(&args[0], "a string")?;
                let b = Big::from_dec(s.trim()).ok_or_else(|| {
                    Error::plain(format!("`strNat` needs decimal digits, got `{s}`"))
                })?;
                if b.neg {
                    return Err(Error::plain("`strNat` rejects a negative `Nat`"));
                }
                Ok(big_value(b, true))
            }
            StageOp::NameToSym => {
                let name = match &args[0] {
                    Value::Name(n) => n.clone(),
                    other => {
                        return Err(Error::plain(format!(
                            "`name-to-sym` takes a `Name`, found {}",
                            value_kind_name(other)
                        )))
                    }
                };
                Ok(Value::Sym(name))
            }
            StageOp::SymToName => {
                let s = match &args[0] {
                    Value::Sym(n) => n.clone(),
                    other => {
                        return Err(Error::plain(format!(
                            "`sym-to-name` takes a `Sym`, found {}",
                            value_kind_name(other)
                        )))
                    }
                };
                Ok(Value::Name(s))
            }
            StageOp::NatStr => {
                let b = big_of(&args[0], "`Nat`")?;
                Ok(Value::Str(b.to_dec()))
            }
            StageOp::StrAppend => {
                let a = str_of(&args[0], "a string")?;
                let b = str_of(&args[1], "a string")?;
                Ok(Value::Str(format!("{a}{b}")))
            }
            StageOp::StrLen => {
                let s = str_of(&args[0], "a string")?;
                Ok(big_value(Big::from_i64(s.len() as i64), true))
            }
        }
    }

    /// A file read under the build mode (modules specification section 10).
    fn stage_read(&mut self, cx: &mut StageCx, path: &str) -> Result<String> {
        let snapshot = cx.snapshot.last().cloned().unwrap_or_default();
        let full = cx.workdir.join(path);
        let prior = cx
            .prior
            .iter()
            .find(|i| i.kind == "file" && i.name == path && i.snapshot == snapshot)
            .cloned();
        match self.stage_mode {
            StageMode::Replay => {
                let rec = prior.ok_or_else(|| {
                    Error::plain(format!(
                        "replay mode: no captured input for the file `{path}` in snapshot `{snapshot}` (run a refresh build to record it)"
                    ))
                })?;
                if let Ok(current) = std::fs::read(&full) {
                    let hash = crate::interface::content_hash(&String::from_utf8_lossy(&current));
                    if hash != rec.hash {
                        return Err(Error::plain(format!(
                            "replay mode: the captured input for `{path}` is incompatible with the file on disk (hashes differ); run a refresh build"
                        )));
                    }
                }
                self.record_input(cx, rec.clone());
                Ok(rec.captured)
            }
            mode => {
                let bytes = std::fs::read(&full).map_err(|e| {
                    Error::plain(format!("cannot read {}: {e}", full.display()))
                })?;
                let contents = String::from_utf8_lossy(&bytes).to_string();
                let hash = crate::interface::content_hash(&contents);
                self.record_input(
                    cx,
                    StageInput {
                        snapshot,
                        kind: "file".to_string(),
                        name: path.to_string(),
                        hash,
                        captured: contents.clone(),
                        live: mode == StageMode::Live,
                    },
                );
                Ok(contents)
            }
        }
    }

    /// A subprocess run under the build mode; the tool identity and output
    /// are inputs to the build identity.
    fn stage_run(&mut self, cx: &mut StageCx, tool: &str, argv: &[String]) -> Result<String> {
        self.stage_run_env(cx, tool, argv, "", "")
    }

    /// [`stage_run`] with one credential in the tool's environment
    /// (foreign function interface pairing aside, modules specification
    /// section 10: secrets authorize access but never enter snapshots;
    /// the captured input records the environment variable's name and the
    /// tool's output, never the credential).
    fn stage_run_env(
        &mut self,
        cx: &mut StageCx,
        tool: &str,
        argv: &[String],
        env_name: &str,
        env_value: &str,
    ) -> Result<String> {
        let snapshot = cx.snapshot.last().cloned().unwrap_or_default();
        let key = if env_name.is_empty() {
            format!("{tool} {}", argv.join(" "))
        } else {
            format!("{tool} {} env:{env_name}", argv.join(" "))
        };
        let prior = cx
            .prior
            .iter()
            .find(|i| i.kind == "subprocess" && i.name == key && i.snapshot == snapshot)
            .cloned();
        if self.stage_mode == StageMode::Replay {
            let rec = prior.ok_or_else(|| {
                Error::plain(format!(
                    "replay mode: no captured input for the subprocess `{key}` in snapshot `{snapshot}` (run a refresh build to record it)"
                ))
            })?;
            self.record_input(cx, rec.clone());
            return Ok(rec.captured);
        }
        let mut cmd = std::process::Command::new(tool);
        cmd.args(argv);
        if !cx.workdir.as_os_str().is_empty() {
            cmd.current_dir(&cx.workdir);
        }
        if !env_name.is_empty() {
            cmd.env(env_name, env_value);
        }
        let mut child = cmd
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| Error::plain(format!("cannot run `{tool}`: {e}")))?;
        let out = child.wait_with_output().map_err(|e| {
            Error::plain(format!("`{tool}` did not finish: {e}"))
        })?;
        if !out.status.success() {
            return Err(Error::plain(format!(
                "the subprocess `{key}` failed with status {} (a failed stage releases its resources; the failure is not retried)",
                out.status
            )));
        }
        let contents = String::from_utf8_lossy(&out.stdout).to_string();
        self.record_input(
            cx,
            StageInput {
                snapshot,
                kind: "subprocess".to_string(),
                name: key,
                hash: crate::interface::content_hash(&contents),
                captured: contents.clone(),
                live: self.stage_mode == StageMode::Live,
            },
        );
        Ok(contents)
    }

    /// `introspectConsistently` (modules specification section 11): the
    /// consistency check the sketch names. The tool's document is parsed
    /// and checked for internal consistency (unique table and column
    /// names, known column types); the interpreted schema records the
    /// dialect, collation, extension and nullability assumptions and the
    /// snapshot identity where the provider supports one. Generated
    /// binding proofs are relative to this capture.
    fn interpret_schema(&self, raw: &str) -> Result<crate::core::SchemaData> {
        use crate::core::{SchemaColumn, SchemaData, SchemaTable};
        let v = crate::interface::parse_json(raw)
            .map_err(|e| Error::plain(format!("the introspection output is not a schema document: {e}")))?;
        let obj = v
            .as_object()
            .map_err(|_| Error::plain("the introspection output is not a schema document (expected an object)"))?;
        let str_field = |k: &str| -> Result<String> {
            match obj.get(k) {
                Some(v) => v
                    .as_str()
                    .map(|s| s.to_string())
                    .map_err(|_| Error::plain(format!("schema field `{k}` is not a string"))),
                None => Ok(String::new()),
            }
        };
        let dialect = str_field("dialect")?;
        let collation = str_field("collation")?;
        let snapshot = str_field("snapshot")?;
        let mut extensions = Vec::new();
        if let Some(ext) = obj.get("extensions") {
            for e in ext
                .as_array()
                .map_err(|_| Error::plain("schema field `extensions` is not a list"))?
            {
                extensions.push(
                    e.as_str()
                        .map(|s| s.to_string())
                        .map_err(|_| Error::plain("a schema extension is not a string"))?,
                );
            }
        }
        let mut tables = Vec::new();
        let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let tabs = obj
            .get("tables")
            .ok_or_else(|| Error::plain("the schema document has no `tables` list"))?
            .as_array()
            .map_err(|_| Error::plain("schema field `tables` is not a list"))?;
        for t in tabs {
            let t = t
                .as_object()
                .map_err(|_| Error::plain("a schema table is not an object"))?;
            let name = t
                .get("name")
                .ok_or_else(|| Error::plain("a schema table has no `name`"))?
                .as_str()
                .map_err(|_| Error::plain("a schema table name is not a string"))?
                .to_string();
            if !seen.insert(name.clone()) {
                return Err(Error::plain(format!(
                    "inconsistent schema: table `{name}` appears twice (modules specification section 11: introspection must obtain a consistent snapshot)"
                )));
            }
            let mut columns = Vec::new();
            let mut cseen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            for c in t
                .get("columns")
                .ok_or_else(|| Error::plain(format!("schema table `{name}` has no `columns` list")))?
                .as_array()
                .map_err(|_| Error::plain(format!("schema table `{name}` columns are not a list")))?
            {
                let c = c
                    .as_object()
                    .map_err(|_| Error::plain("a schema column is not an object"))?;
                let cname = c
                    .get("name")
                    .ok_or_else(|| Error::plain("a schema column has no `name`"))?
                    .as_str()
                    .map_err(|_| Error::plain("a schema column name is not a string"))?
                    .to_string();
                if !cseen.insert(cname.clone()) {
                    return Err(Error::plain(format!(
                        "inconsistent schema: column `{cname}` appears twice in table `{name}` (modules specification section 11)"
                    )));
                }
                let ctype = c
                    .get("type")
                    .ok_or_else(|| Error::plain(format!("schema column `{name}.{cname}` has no `type`")))?
                    .as_str()
                    .map_err(|_| Error::plain("a schema column type is not a string"))?
                    .to_ascii_uppercase();
                match ctype.as_str() {
                    "INTEGER" | "REAL" | "BOOLEAN" | "TEXT" => {}
                    other => {
                        return Err(Error::plain(format!(
                            "unsupported column type `{other}` for {name}.{cname}: the interpreted schema vocabulary is INTEGER, REAL, BOOLEAN and TEXT (modules specification section 11)"
                        )))
                    }
                }
                let nullable = match c.get("nullable") {
                    Some(v) => v
                        .as_bool()
                        .map_err(|_| Error::plain(format!("schema column `{name}.{cname}` nullable is not a boolean")))?,
                    None => true,
                };
                columns.push(SchemaColumn { name: cname, ty: ctype, nullable });
            }
            tables.push(SchemaTable { name, columns });
        }
        Ok(SchemaData {
            dialect,
            collation,
            snapshot,
            extensions,
            tables,
            hash: crate::interface::content_hash(raw),
        })
    }

    /// `CodecGenerator.derive` (modules specification section 11): the
    /// codec generator, producing `quoteDecls`-shaped declarations from a
    /// captured schema: one row record and one `encode`/`decode` pair per
    /// table, in a fixed little-endian column format (INTEGER: 8 bytes;
    /// BOOLEAN: 1 byte). The generated declarations are ordinary checked
    /// declarations (P9's rules).
    fn derive_codecs(&self, snap: &crate::core::SchemaData) -> Result<Vec<ast::Decl>> {
        use crate::core::SchemaData;
        let _ = std::any::type_name::<SchemaData>();
        let sp = ast::Span { line: 0, col: 0 };
        let con = |n: &str| ast::Term::Con(ast::ConPath { segments: vec![ast::Name::new(n, sp)] });
        let var = |n: &str| {
            ast::Term::Var(ast::VarPath {
                quals: Vec::new(),
                name: ast::Name::new(n, sp),
            })
        };
        let app = |f: ast::Term, args: Vec<ast::Term>| ast::Term::App(Box::new(f), args, sp);
        let int = |n: u64| ast::Term::Int(n.to_string(), sp);
        let proj = |base: ast::Term, f: &str| {
            ast::Term::Proj(Box::new(base), ast::Name::new(f, sp), sp)
        };
        let con_ty = |n: &str| ast::Type::Con(ast::ConPath { segments: vec![ast::Name::new(n, sp)] }, sp);
        let app_ty = |f: ast::Type, a: ast::Type| ast::Type::App(Box::new(f), Box::new(a), sp);
        let fun_ty = |arg: ast::Type, res: ast::Type| ast::Type::Fun {
            arg: Box::new(arg),
            mult: None,
            eff: None,
            res: Box::new(res),
            span: sp,
        };
        let mut decls = Vec::new();
        let mut used: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for t in &snap.tables {
            let tname = codec_type_name(&t.name)?;
            if !used.insert(tname.clone()) {
                return Err(Error::plain(format!(
                    "the codec generator cannot derive two types named {tname} (table {})",
                    t.name
                )));
            }
            let mut fields: Vec<ast::FieldDecl> = Vec::new();
            let mut cols: Vec<(String, u64, u64, bool)> = Vec::new();
            let mut offset = 0u64;
            for c in &t.columns {
                if !codec_field_name(&c.name) {
                    return Err(Error::plain(format!(
                        "column {}.{}` is not a field name (the codec generator derives one record field per column)",
                        t.name, c.name
                    )));
                }
                let (fty, size, is_bool) = match c.ty.as_str() {
                    "INTEGER" => (con_ty("I64"), 8u64, false),
                    "BOOLEAN" => (con_ty("Bool"), 1u64, true),
                    other => {
                        return Err(Error::plain(format!(
                            "the codec generator's column encoders cover INTEGER and BOOLEAN in this slice; {}.{} is {other} (REAL needs float bit access and TEXT a length-prefixed copy; decision D65)",
                            t.name, c.name
                        )))
                    }
                };
                fields.push(ast::FieldDecl {
                    attrs: Vec::new(),
                    name: ast::Name::new(&c.name, sp),
                    ty: fty,
                    erased: false,
                    span: sp,
                });
                cols.push((c.name.clone(), offset, size, is_bool));
                offset += size;
            }
            decls.push(ast::Decl::Record(ast::RecordDecl {
                name: ast::Name::new(&tname, sp),
                params: Vec::new(),
                fields,
                span: sp,
            }));
            // `encodeT row`: a fresh buffer of the fixed row width, each
            // column written little-endian at its offset.
            let row = ast::Name::new("row", sp);
            let buf = ast::Name::new("buf", sp);
            let mut binds = vec![ast::LocalDecl::Bind {
                pat: ast::Pat::Var(buf.clone(), sp),
                body: app(var("bytes-new"), vec![int(offset)]),
                span: sp,
            }];
            for (fname, off, size, is_bool) in &cols {
                for k in 0..*size {
                    let byte_val = if *is_bool {
                        app(var("nat-from-i64"), vec![ast::Term::If {
                            cond: Box::new(proj(var("row"), fname)),
                            then: Box::new(int(1)),
                            els: Box::new(int(0)),
                            cond_attrs: Vec::new(),
                            then_attrs: Vec::new(),
                            else_attrs: Vec::new(),
                            span: sp,
                        }])
                    } else {
                        app(var("nat-from-i64"), vec![app(var("bit-and-i64"), vec![
                            app(var("wrapping-shr-i64"), vec![proj(var("row"), fname), int(k * 8)]),
                            int(255),
                        ])])
                    };
                    binds.push(ast::LocalDecl::Bind {
                        pat: ast::Pat::Wild(sp),
                        body: app(var("bytes-set"), vec![var("buf"), int(off + k), byte_val]),
                        span: sp,
                    });
                }
            }
            let enc = format!("encode{tname}");
            decls.push(ast::Decl::ValueSig {
                name: ast::Name::new(&enc, sp),
                modifier: None,
                owned_attrs: Vec::new(),
                ty: fun_ty(con_ty(&tname), con_ty("Bytes")),
                span: sp,
            });
            decls.push(ast::Decl::Equation {
                modifier: None,
                eq: ast::Equation {
                    attrs: Vec::new(),
                    name: ast::Name::new(&enc, sp),
                    pats: vec![ast::Pat::Var(row, sp)],
                    body: ast::Term::Let(binds, Box::new(var("buf")), sp),
                    locals: Vec::new(),
                    span: sp,
                },
            });
            // `decodeT b`: the fixed row width checked up front; each
            // column reassembled little-endian at its offset.
            let b = ast::Name::new("b", sp);
            let mut fields_out: Vec<(ast::Name, ast::Term)> = Vec::new();
            for (fname, off, size, is_bool) in &cols {
                let v = if *is_bool {
                    ast::Term::If {
                        cond: Box::new(app(var("=="), vec![
                            app(var("nat-to-i64"), vec![app(var("bytes-get"), vec![var("b"), int(*off)])]),
                            int(1),
                        ])),
                        then: Box::new(con("True")),
                        els: Box::new(con("False")),
                        cond_attrs: Vec::new(),
                        then_attrs: Vec::new(),
                        else_attrs: Vec::new(),
                        span: sp,
                    }
                } else {
                    let mut acc = int(0);
                    for k in 0..*size {
                        let byte = app(var("nat-to-i64"), vec![
                            app(var("bytes-get"), vec![var("b"), int(off + k)]),
                        ]);
                        let shifted = app(var("wrapping-shl-i64"), vec![byte, int(k * 8)]);
                        acc = app(var("wrapping-add-i64"), vec![acc, shifted]);
                    }
                    acc
                };
                fields_out.push((ast::Name::new(fname, sp), v));
            }
            let record = ast::Term::Record(
                ast::ConPath { segments: vec![ast::Name::new(&tname, sp)] },
                fields_out,
                sp,
            );
            let some = app(con("Some"), vec![record]);
            let body = ast::Term::If {
                cond: Box::new(app(var("=="), vec![
                    app(var("bytes-len"), vec![var("b")]),
                    int(offset),
                ])),
                then: Box::new(some),
                els: Box::new(con("None")),
                cond_attrs: Vec::new(),
                then_attrs: Vec::new(),
                else_attrs: Vec::new(),
                span: sp,
            };
            let dec = format!("decode{tname}");
            decls.push(ast::Decl::ValueSig {
                name: ast::Name::new(&dec, sp),
                modifier: None,
                owned_attrs: Vec::new(),
                ty: fun_ty(con_ty("Bytes"), app_ty(con_ty("Option"), con_ty(&tname))),
                span: sp,
            });
            decls.push(ast::Decl::Equation {
                modifier: None,
                eq: ast::Equation {
                    attrs: Vec::new(),
                    name: ast::Name::new(&dec, sp),
                    pats: vec![ast::Pat::Var(b, sp)],
                    body,
                    locals: Vec::new(),
                    span: sp,
                },
            });
        }
        Ok(decls)
    }

    fn record_input(&mut self, cx: &mut StageCx, input: StageInput) {
        if input.kind != "secret" {
            // Secrets record their name only (modules specification
            // section 10).
            cx.inputs.retain(|i| {
                !(i.kind == input.kind && i.name == input.name && i.snapshot == input.snapshot)
            });
        }
        cx.inputs.push(input);
    }

    /// Build one `Code` value: hygiene first (fresh binders and
    /// definition-site names), then hole substitution (caller syntax keeps
    /// its binding relationships), then macro expansion of spliced calls
    /// (modules specification section 8).
    fn construct_quote(
        &mut self,
        cx: &mut StageCx,
        template: &ast::Term,
        holes: &[crate::core::Core],
        frame: &mut Vec<Value>,
    ) -> Result<Value> {
        let vals = self.eval_holes(cx, holes, frame)?;
        let renamed = self.hygienic_term(cx, template, &[])?;
        let mut out = renamed;
        self.stamp_generated(cx, &mut out);
        let terms = self.hole_terms(cx, &vals, &[])?;
        let mut out = ast::subst_holes(&out, &terms);
        let module = cx.origin.clone();
        self.expand_macros_in_term(cx, &module, &mut out)?;
        Ok(Value::Code(out))
    }

    /// Build one `Declarations` value (the same discipline over a
    /// declaration template).
    fn construct_decls(
        &mut self,
        cx: &mut StageCx,
        template: &[ast::Decl],
        holes: &[crate::core::Core],
        name_holes: &[usize],
        frame: &mut Vec<Value>,
    ) -> Result<Value> {
        let vals = self.eval_holes(cx, holes, frame)?;
        let terms = self.hole_terms(cx, &vals, name_holes)?;
        // Name splices resolve first (D46): a declaration's computed
        // name lands before the template's own names are collected, so
        // sibling references and hygiene see it like a written name.
        let mut template: Vec<ast::Decl> = template.to_vec();
        for d in &mut template {
            splice_decl_names(d, &terms)?;
        }
        let mut own_values = Vec::new();
        let mut own_types = Vec::new();
        for d in &template {
            match d {
                ast::Decl::ValueSig { name, .. } | ast::Decl::Equation { eq: ast::Equation { name, .. }, .. } => {
                    own_values.push(name.text.clone())
                }
                ast::Decl::Data(data) => own_types.push(data.name.text.clone()),
                ast::Decl::Record(r) | ast::Decl::Codata(r) => own_types.push(r.name.text.clone()),
                ast::Decl::Signature(s) => own_types.push(s.name.text.clone()),
                ast::Decl::Attributed { inner, .. } => {
                    if let Some(n) = decl_name(inner) {
                        own_values.push(n.text.clone());
                    }
                }
                _ => {}
            }
        }
        let mut decls = Vec::new();
        for d in &template {
            let mut d = d.clone();
            self.hygienic_decl(cx, &mut d, &own_values, &own_types)?;
            decls.push(d);
        }
        let site = self.expansion_site(cx, ast::Span { line: 0, col: 0 });
        for d in &mut decls {
            stamp_decl_spans(d, &site);
        }
        let mut decls: Vec<ast::Decl> =
            decls.iter().map(|d| ast::subst_holes_decl(d, &terms)).collect();
        let module = cx.origin.clone();
        for d in &mut decls {
            self.expand_macros_in_decl(cx, &module, d)?;
        }
        Ok(Value::Decls(decls))
    }

    fn eval_holes(
        &mut self,
        cx: &mut StageCx,
        holes: &[crate::core::Core],
        frame: &mut Vec<Value>,
    ) -> Result<Vec<Value>> {
        let mut vals = Vec::new();
        for h in holes {
            vals.push(self.eval_core(cx, h, frame)?);
        }
        Ok(vals)
    }

    /// The spliced syntax of each hole, parsed where it belongs. A name
    /// hole (D46) reifies to the name term its value stands for.
    fn hole_terms(
        &mut self,
        cx: &mut StageCx,
        vals: &[Value],
        name_holes: &[usize],
    ) -> Result<Vec<ast::Term>> {
        let mut terms = Vec::new();
        for (i, v) in vals.iter().enumerate() {
            let span = cx.call.unwrap_or(ast::Span { line: 0, col: 0 });
            if name_holes.contains(&i) {
                let text = name_of_value(v, span)?;
                terms.push(ast::Term::Var(ast::VarPath {
                    quals: Vec::new(),
                    name: ast::Name::new(text, span),
                }));
                continue;
            }
            match v {
                Value::Code(t) => terms.push(t.clone()),
                Value::Syntax { tokens, origin } => {
                    let fix = cx
                        .fixities
                        .get(origin)
                        .cloned()
                        .unwrap_or_default();
                    terms.push(crate::parser::parse_term_tokens(tokens, &fix)?);
                }
                // Splice-time reification (D233, extended by D46): a
                // `Sym` value becomes a symbol constant, a `Name` a
                // reference to the named binding.
                Value::Sym(s) => terms.push(ast::Term::Sym(s.clone(), span)),
                Value::Name(n) => terms.push(ast::Term::Var(ast::VarPath {
                    quals: Vec::new(),
                    name: ast::Name::new(n.clone(), span),
                })),
                other => {
                    return Err(Error::plain(format!(
                        "a template hole takes code, syntax, a name or a symbol, found {}",
                        value_kind_name(other)
                    )))
                }
            }
        }
        Ok(terms)
    }

    /// The hygienic rename of one template: generated binders are fresh,
    /// free top-level names resolve at the definition site (section 8).
    fn hygienic_term(
        &mut self,
        cx: &mut StageCx,
        t: &ast::Term,
        own: &[String],
    ) -> Result<ast::Term> {
        let def_module = cx.def_module.clone();
        let resolve = |kind: ast::RefKind, name: &str| -> Option<(String, String)> {
            let site = ast::RefSite {
                kind,
                quals: Vec::new(),
                name: name.to_string(),
                span: ast::Span { line: 0, col: 0 },
            };
            self.resolve_top(&def_module, &site)
        };
        let mut fresh = |base: &str| -> String {
            cx.fresh += 1;
            format!("{base}${}", cx.fresh)
        };
        let mut hcx = ast::HygCx {
            resolve: &resolve,
            fresh: &mut fresh,
            own_values: own.to_vec(),
            own_types: Vec::new(),
        };
        let mut scope = ast::HygScope::new();
        ast::hyg_term(t, &mut hcx, &mut scope)
    }

    fn hygienic_decl(
        &mut self,
        cx: &mut StageCx,
        d: &mut ast::Decl,
        own_values: &[String],
        own_types: &[String],
    ) -> Result<()> {
        let def_module = cx.def_module.clone();
        let resolve = |kind: ast::RefKind, name: &str| -> Option<(String, String)> {
            let site = ast::RefSite {
                kind,
                quals: Vec::new(),
                name: name.to_string(),
                span: ast::Span { line: 0, col: 0 },
            };
            self.resolve_top(&def_module, &site)
        };
        let mut fresh = |base: &str| -> String {
            cx.fresh += 1;
            format!("{base}${}", cx.fresh)
        };
        let mut hcx = ast::HygCx {
            resolve: &resolve,
            fresh: &mut fresh,
            own_values: own_values.to_vec(),
            own_types: own_types.to_vec(),
        };
        let mut scope = ast::HygScope::new();
        let replaced = ast::hyg_decl(d, &mut hcx, &mut scope)?;
        *d = replaced;
        Ok(())
    }

    /// The provenance of generated syntax (section 8: source maps retain
    /// expansion provenance).
    fn expansion_site(&self, cx: &StageCx, template: ast::Span) -> crate::lexer::ExpSite {
        crate::lexer::ExpSite {
            name: cx.unit_name.clone(),
            call: cx.call.unwrap_or(template),
            origin: self
                .module_files
                .get(&cx.def_module)
                .cloned()
                .unwrap_or_else(|| cx.def_module.clone()),
            template,
            parent: cx.parent,
        }
    }

    fn stamp_generated(&self, cx: &StageCx, t: &mut ast::Term) {
        let site = self.expansion_site(cx, t.span());
        stamp_term_spans(t, &site);
    }
}

fn set_local(frame: &mut Vec<Value>, idx: usize, v: Value) -> Option<Value> {
    if frame.len() <= idx {
        frame.resize_with(idx + 1, || Value::Unit);
    }
    Some(std::mem::replace(&mut frame[idx], v))
}

fn restore_local(frame: &mut Vec<Value>, idx: usize, old: Option<Value>) {
    if let Some(old) = old {
        if idx < frame.len() {
            frame[idx] = old;
        }
    }
}

fn big_cmp_word(a: &Big, b: &Big) -> i64 {
    match a.cmp(b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn stage_op_arity(op: StageOp) -> usize {
    use StageOp::*;
    match op {
        Snapshot | WithReadFiles | WithWriteFiles | WithProcess | ReadFile | StrAppend => 2,
        WriteFile | Run => 3,
        WithDatabase => 4,
        Secret | Emit | StrNat | NatStr | StrLen | QuoteNat | QuoteInt | QuoteBool | Introspect
        | CodecDerive | NameToSym | SymToName => 1,
        ReadSchema => 0,
    }
}

fn require_cap(v: &Value, want: Cap, what: &str) -> Result<()> {
    match v {
        Value::Cap(c) if *c == want => Ok(()),
        other => Err(Error::plain(format!(
            "expected {what}, found {}",
            value_kind_name(other)
        ))),
    }
}

fn require_file_write(v: &Value) -> Result<()> {
    match v {
        Value::Cap(Cap::File { write: true }) => Ok(()),
        Value::Cap(Cap::File { .. }) => Err(Error::plain(
            "`writeFile` needs the write refinement (`Stage.withWriteFiles`); a file reader has no write authority by default (modules specification section 10)",
        )),
        other => Err(Error::plain(format!(
            "expected a file-write capability, found {}",
            value_kind_name(other)
        ))),
    }
}

/// A parameter is an honest pure callback when its type is a function
/// whose effect rows are all empty (effect honesty makes the declared row
/// an upper bound on everything the callback performs).
fn pure_arrow_ty(t: &Ty) -> bool {
    fn go(t: &Ty, saw: &mut bool) -> bool {
        match t {
            Ty::Fun(a, _, eff, b) => {
                *saw = true;
                eff.is_pure() && go(a, saw) && go(b, saw)
            }
            Ty::Pi { dom, eff, cod, .. } => {
                *saw = true;
                eff.is_pure() && go(dom, saw) && go(cod, saw)
            }
            _ => true,
        }
    }
    let mut saw = false;
    let ok = go(t, &mut saw);
    saw && ok
}

// ==================== `prim` inline-body containment (P18) ====================

/// The LLVM instruction vocabulary an `llvm` fragment may use
/// (primitive operations and inline target IR specification section 4's
/// family grammar: one straight-line function body).
const LLVM_OPCODES: &[&str] = &[
    "add", "sub", "mul", "udiv", "sdiv", "urem", "srem", "shl", "lshr", "ashr", "and", "or", "xor",
    "fadd", "fsub", "fmul", "fdiv", "frem", "fneg", "icmp", "fcmp", "select", "freeze", "trunc",
    "zext", "sext", "fptrunc", "fpext", "fptoui", "fptosi", "uitofp", "sitofp", "ptrtoint",
    "inttoptr", "bitcast", "addrspacecast", "extractvalue", "insertvalue", "extractelement",
    "insertelement", "shufflevector", "alloca", "load", "store", "fence", "cmpxchg", "atomicrmw",
    "getelementptr", "ret", "br", "switch", "indirectbr", "invoke", "call", "callbr", "resume",
    "unreachable", "va_arg", "landingpad", "catchpad", "cleanuppad", "catchswitch", "catchret",
    "cleanupret", "phi",
];

/// Allocation is forbidden in a `prim` body outright (section 4: no
/// allocation), so even a `uses` contract may not authorize these.
const ALLOCATION_SYMS: &[&str] = &[
    "malloc",
    "calloc",
    "realloc",
    "free",
    "aligned_alloc",
    "posix_memalign",
    "strdup",
    "strndup",
];

/// I/O is forbidden in a `prim` body outright (section 4: no I/O).
const IO_SYMS: &[&str] = &[
    "printf", "fprintf", "sprintf", "snprintf", "puts", "putchar", "fwrite", "fread", "write",
    "read", "open", "close", "exit", "abort", "_exit", "perror", "scanf", "fscanf", "sscanf",
];

/// A `prim` body's `when` condition (primitive operations and inline
/// target IR specification sections 2 and 13): the platform chapter's
/// capability evidence (`Available X`), or a platform predicate keyed on
/// the target profile's triple (`arch <name>`).
enum WhenCond {
    Capability(String),
    Arch(String),
}

impl WhenCond {
    fn render(&self) -> String {
        match self {
            WhenCond::Capability(c) => format!("Available {c}"),
            WhenCond::Arch(a) => format!("arch {a}"),
        }
    }

    /// Does the condition hold in the target profile (its capability
    /// evidence and its arch)?
    fn holds(&self, caps: &[String], arch: &str) -> bool {
        match self {
            WhenCond::Capability(c) => caps.iter().any(|x| x == c),
            WhenCond::Arch(a) => a == arch,
        }
    }
}

/// Is this type a prim ABI scalar (the declared ABI's widths)?
fn is_prim_abi_ty(ty: &Ty) -> bool {
    matches!(ty, Ty::Prim(_) | Ty::Bool)
}

/// The declared effect row of a `prim` type is the body's entire
/// permitted effect (section 4's determinism of effects): anything the
/// fragment performs beyond pure computation must be visible in it.
fn prim_effectful(ty: &Ty) -> bool {
    let mut cur = ty;
    loop {
        match cur {
            Ty::Fun(_, _, e, b) | Ty::Pi { eff: e, cod: b, .. } => {
                if !e.is_pure() {
                    return true;
                }
                cur = b;
            }
            _ => return false,
        }
    }
}

/// One LLVM text line without its `;` comment (the fragment is kept
/// verbatim; comments hide nothing from containment).
fn strip_llvm_comment(line: &str) -> String {
    let mut out = String::new();
    let mut in_str = false;
    let mut cs = line.chars();
    while let Some(c) = cs.next() {
        match c {
            '"' => {
                in_str = !in_str;
                out.push(c);
            }
            '\\' if in_str => {
                out.push(c);
                if let Some(n) = cs.next() {
                    out.push(n);
                }
            }
            ';' if !in_str => break,
            _ => out.push(c),
        }
    }
    out
}

/// Every `@symbol` reference in one LLVM text line (`@"..."` quoted or
/// bare), left to right.
fn llvm_symbols(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let cs: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == '@' {
            i += 1;
            if cs.get(i) == Some(&'"') {
                i += 1;
                let mut s = String::new();
                while i < cs.len() && cs[i] != '"' {
                    if cs[i] == '\\' && i + 1 < cs.len() {
                        i += 1;
                    }
                    s.push(cs[i]);
                    i += 1;
                }
                i += 1;
                out.push(s);
            } else {
                let mut s = String::new();
                while i < cs.len()
                    && (cs[i].is_ascii_alphanumeric() || matches!(cs[i], '.' | '_' | '$' | '-'))
                {
                    s.push(cs[i]);
                    i += 1;
                }
                if !s.is_empty() {
                    out.push(s);
                }
            }
        } else {
            i += 1;
        }
    }
    out
}

/// Structural instance matching for law checks: a type parameter
/// matches any argument (the coherence rule guarantees at most one
/// instance matches).
fn ty_match(pattern: &Ty, concrete: &Ty) -> bool {
    match (pattern, concrete) {
        (Ty::Param(_), _) | (_, Ty::Param(_)) => true,
        (Ty::Prim(a), Ty::Prim(b)) => a == b,
        (Ty::Bool, Ty::Bool) | (Ty::Nat, Ty::Nat) | (Ty::Int, Ty::Int) | (Ty::Bytes, Ty::Bytes)
        | (Ty::Sym, Ty::Sym) => true,
        (Ty::Adt(a, x), Ty::Adt(b, y)) => {
            a == b
                && x.len() == y.len()
                && x.iter().zip(y).all(|(p, c)| ty_match(p, c))
        }
        (Ty::Tuple(x), Ty::Tuple(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, c)| ty_match(p, c))
        }
        (Ty::Fun(a1, _, _, b1), Ty::Fun(a2, _, _, b2))
        | (Ty::Pi { dom: a1, cod: b1, .. }, Ty::Pi { dom: a2, cod: b2, .. }) => {
            ty_match(a1, a2) && ty_match(b1, b2)
        }
        (Ty::App(h1, x), Ty::App(h2, y)) => {
            ty_match(h1, h2) && x.len() == y.len() && x.iter().zip(y).all(|(p, c)| ty_match(p, c))
        }
        _ => false,
    }
}

/// The numeric value of a sample (machine word or bignum): the
/// preservation predicates' element comparison over samples.
fn value_num(v: &Value) -> Option<i128> {
    match v {
        Value::Word(w) => Some(*w as i128),
        Value::Big { digits, .. } => digits.parse().ok(),
        _ => None,
    }
}

/// The baked `length` measure at a sample: a list spine's element
/// count (an `Adt` spine of a two-field `Cons` and a zero-field `Nil`)
/// or a byte buffer's size.
fn value_len(v: &Value) -> Option<i128> {
    match v {
        Value::Bytes(b) => Some(b.borrow().len() as i128),
        Value::Adt { .. } => {
            let mut n: i128 = 0;
            let mut cur = v;
            loop {
                match cur {
                    Value::Adt { fields, .. } if fields.is_empty() => return Some(n),
                    Value::Adt { fields, .. } if fields.len() >= 2 => {
                        n = n.checked_add(1)?;
                        cur = &fields[1];
                    }
                    _ => return None,
                }
            }
        }
        _ => None,
    }
}

/// The sample-wise law comparison: values compare structurally, and a
/// value the machinery cannot compare (a closure, a dictionary, a
/// secret) is honestly opaque, never a law failure.
enum LawCmp {
    Equal,
    NotEqual,
    Opaque,
}

fn law_value_eq(a: &Value, b: &Value) -> LawCmp {
    use crate::core::Value::*;
    match (a, b) {
        (Word(x), Word(y)) => {
            if x == y {
                LawCmp::Equal
            } else {
                LawCmp::NotEqual
            }
        }
        (Big { nat: n1, digits: d1 }, Big { nat: n2, digits: d2 }) => {
            if n1 == n2 && d1 == d2 {
                LawCmp::Equal
            } else {
                LawCmp::NotEqual
            }
        }
        (Word(x), Big { nat, digits }) | (Big { nat, digits }, Word(x)) => {
            let xn = *x >= 0;
            let xd = x.abs().to_string();
            if *nat == xn && digits.trim_start_matches('0') == xd.trim_start_matches('0') {
                LawCmp::Equal
            } else {
                LawCmp::NotEqual
            }
        }
        (Bool(x), Bool(y)) => {
            if x == y {
                LawCmp::Equal
            } else {
                LawCmp::NotEqual
            }
        }
        (Unit, Unit) => LawCmp::Equal,
        (Str(x), Str(y)) => {
            if x == y {
                LawCmp::Equal
            } else {
                LawCmp::NotEqual
            }
        }
        (Bytes(x), Bytes(y)) => {
            if *x.borrow() == *y.borrow() {
                LawCmp::Equal
            } else {
                LawCmp::NotEqual
            }
        }
        (Adt { tag: t1, fields: f1, .. }, Adt { tag: t2, fields: f2, .. }) => {
            if t1 != t2 || f1.len() != f2.len() {
                return LawCmp::NotEqual;
            }
            let mut op = LawCmp::Equal;
            for (x, y) in f1.iter().zip(f2.iter()) {
                match law_value_eq(x, y) {
                    LawCmp::NotEqual => return LawCmp::NotEqual,
                    LawCmp::Opaque => op = LawCmp::Opaque,
                    LawCmp::Equal => {}
                }
            }
            op
        }
        (CtorFn { tag: t1, args: a1, .. }, CtorFn { tag: t2, args: a2, .. })
            if a1.is_empty() && a2.is_empty() =>
        {
            if t1 == t2 {
                LawCmp::Equal
            } else {
                LawCmp::NotEqual
            }
        }
        _ => LawCmp::Opaque,
    }
}

/// The rendering of one sampled value in a law report.
fn law_value_render(v: &Value) -> String {
    match v {
        Value::Word(n) => n.to_string(),
        Value::Big { digits, .. } => digits.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Unit => "()".to_string(),
        Value::Str(s) => format!("{s:?}"),
        Value::Bytes(b) => format!("{:?}", b.borrow()),
        Value::Adt { tag, fields, .. } => {
            let fs: Vec<String> = fields.iter().map(law_value_render).collect();
            format!("<adt {tag} {}>", fs.join(", "))
        }
        other => format!("{other:?}"),
    }
}

/// Sample values of one machine scalar at its boundaries (the law
/// machinery's sample sets; floats are not sampled: NaN compares equal
/// to nothing).
fn scalar_samples(s: &crate::types::Scalar) -> Vec<Value> {
    use crate::types::Scalar::*;
    let w = |n: i64| Value::Word(n);
    match s {
        U8 => vec![w(0), w(1), w(255)],
        I8 => vec![w(-128), w(-1), w(0), w(127)],
        U16 => vec![w(0), w(1), w(65535)],
        I16 => vec![w(-32768), w(0), w(32767)],
        U32 => vec![w(0), w(1), w(4294967295)],
        I32 => vec![w(-2147483648), w(0), w(2147483647)],
        U64 => vec![w(0), w(1), w(-1)],
        I64 => vec![w(-1), w(0), w(1), w(i64::MIN)],
        Usize | Isize => vec![w(0), w(1), w(-1)],
        Char => vec![w(0), w(65), w(0x10FFFF)],
        F32 | F64 => Vec::new(),
    }
}

/// The declared ABI shape of one prim scalar (the inline fragment's
/// widths; primitive operations and inline target IR specification
/// section 4).
fn prim_abi_shape(ty: &Ty, usize_bits: u32) -> crate::core::PrimAbiTy {
    use crate::core::PrimAbiTy;
    use crate::types::Scalar;
    match ty {
        Ty::Prim(Scalar::F32) => PrimAbiTy::F32,
        Ty::Prim(Scalar::F64) => PrimAbiTy::F64,
        Ty::Prim(Scalar::Usize) => PrimAbiTy::Int { bits: usize_bits, signed: false },
        Ty::Prim(Scalar::Isize) => PrimAbiTy::Int { bits: usize_bits, signed: true },
        Ty::Prim(s) => PrimAbiTy::Int { bits: s.width(), signed: s.is_signed() },
        // `Bool` crosses as its `i8` value bits.
        _ => PrimAbiTy::Int { bits: 8, signed: false },
    }
}

// ------------------------------------------------------------ P21: the REPL probe

impl Checker {
    /// [`solve_wants`] for the probe: the wants no instance discharges
    /// return residual (the probe's scheme states them as its context)
    /// instead of failing the build.
    fn solve_wants_residual(
        &mut self,
        wants: Vec<Want>,
    ) -> Result<(HashMap<usize, DictExpr>, Vec<Want>)> {
        let mut solutions = HashMap::new();
        let mut pending: Vec<usize> = (0..wants.len()).collect();
        loop {
            let mut progress = false;
            let mut next = Vec::new();
            for &i in &pending {
                match self.solve(&wants[i].class, &wants[i].scope, 0, wants[i].span)? {
                    Solved(d) => {
                        solutions.insert(wants[i].id, d);
                        progress = true;
                    }
                    Ambiguous => next.push(i),
                }
            }
            pending = next;
            if pending.is_empty() {
                return Ok((solutions, Vec::new()));
            }
            if !progress {
                // The same literal defaulting as [`solve_wants`]
                // (decision D7), then whatever remains is residual.
                let lit_metas = self.literal_metas();
                let mut defaulted = false;
                for &i in &pending {
                    let wanted_z = self.uni.zonk_class(&wants[i].class);
                    for a in &wanted_z.args {
                        for m in self.uni.free_metas(a) {
                            if lit_metas.contains(&m) {
                                let ty = match m {
                                    EitherMeta::Type(id) => Ty::Meta(id),
                                    EitherMeta::Row(_) => continue,
                                };
                                self.uni.unify(&ty, &Ty::Int).unwrap_or_else(|_| {
                                    panic!("literal defaulting to Int must succeed")
                                });
                                defaulted = true;
                            }
                        }
                    }
                    if defaulted {
                        break;
                    }
                }
                if defaulted {
                    continue;
                }
                let mut residual: Vec<Want> = Vec::new();
                for (i, w) in wants.into_iter().enumerate() {
                    if pending.contains(&i) {
                        residual.push(w);
                    }
                }
                return Ok((solutions, residual));
            }
        }
    }

    /// P21: infer, elaborate and evaluate one closed expression in
    /// `p.module`'s scope through the D92 evaluator (the staging
    /// interpreter the literal fold runs), against the checked program's
    /// code. Runs after every module's declarations are checked, so all
    /// of the session's definitions are available to interpretation.
    /// Probe failures are reported in the result, never as check errors.
    fn probe_term(&mut self, scx: &StagingCx, p: &ProbeTerm<'_>) -> ProbeResult {
        match self.probe_infer(scx, p) {
            Ok(r) => r,
            Err(e) => ProbeResult {
                scheme: None,
                value: None,
                eval_error: None,
                error: Some(e),
            },
        }
    }

    fn probe_infer(&mut self, scx: &StagingCx, p: &ProbeTerm<'_>) -> Result<ProbeResult> {
        // The REPL is a compile-time context: its expressions elaborate
        // where the D92 evaluator runs them.
        self.set_scope(&p.module);
        self.compile_time = true;
        self.pending_refs.clear();
        self.reset_usage();
        self.smaller.clear();
        let want_base = self.want_counter;
        let mut env = Env::default();
        let (ecore, ty) = self.infer(&mut env, p.term)?;
        // The probe's constraint wants, isolated from anything earlier
        // phases left pending.
        let (probe_wants, rest): (Vec<Want>, Vec<Want>) = std::mem::take(&mut self.wants)
            .into_iter()
            .partition(|w| w.id >= want_base);
        self.wants = rest;
        let (sol, residual) = self.solve_wants_residual(probe_wants)?;
        // The scheme: generalized like a let binding (the value
        // restriction), with the residual constraints as its context,
        // rendered against the same quantified names.
        let zonked = self.uni.zonk(&ty);
        let scheme = if is_syntactic_value(p.term) {
            let env_metas = env_free_metas(&self.uni, &env);
            let lit_metas = self.literal_metas();
            let mine = self.uni.free_metas(&zonked);
            let quantify: Vec<EitherMeta> = mine
                .into_iter()
                .filter(|m| !env_metas.contains(m) && !lit_metas.contains(m))
                .collect();
            let (ty2, tps, rps) = self.uni.generalize(&zonked, &quantify);
            let mut ctx = Vec::new();
            for (i, w) in residual.iter().enumerate() {
                let class = self.uni.zonk_class(&w.class);
                let args = class
                    .args
                    .iter()
                    .map(|a| self.uni.generalize(a, &quantify).0)
                    .collect();
                ctx.push((format!("c{i}"), ClassApp { class: class.class, args }));
            }
            crate::types::Scheme {
            obligs: Vec::new(),
                params: tps,
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: rps,
                region_params: Vec::new(),
                ctx,
                ty: ty2,
            }
        } else {
            crate::types::Scheme::mono(zonked)
        };
        let scheme_text = render_scheme(&scheme);
        let mut out = ProbeResult {
            scheme: Some((scheme, scheme_text)),
            value: None,
            eval_error: None,
            error: None,
        };
        if !residual.is_empty() {
            out.eval_error = Some(format!(
                "the expression's constraints are unresolved in this scope ({})",
                residual
                    .iter()
                    .map(|w| self.uni.zonk_class(&w.class).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            return Ok(out);
        }
        // Elaborate to code exactly like a definition body (dictionary
        // solving, literal resolution, the collection fold, lambda
        // hoisting) and run it through the evaluator.
        let id = GlobalId(format!("{}.probe", p.module));
        let mut body = self.fill_holes(&ecore, &sol)?;
        let lits = self.resolved_literals()?;
        let vals = self.resolved_idx_args()?;
        resolve_lit_words(&mut body, &lits);
        resolve_idx_words(&mut body, &vals);
        self.fold_pass(&mut body, &id.0.clone(), true);
        let mut hoisted = Vec::new();
        hoist_lambdas(&mut body, &id.0, &mut self.lam_counter, &mut hoisted);
        let before = self.fns.len();
        self.fns.push(finalize_fn(
            id.clone(),
            Vec::new(),
            body,
            None,
            None,
            crate::core::FnHints::default(),
            crate::core::FnMeta::default(),
        ));
        for h in hoisted {
            self.fns.push(finalize_fn(
                h.id,
                h.params,
                h.body,
                None,
                None,
                crate::core::FnHints::default(),
                crate::core::FnMeta::default(),
            ));
        }
        let mut cx = StageCx::new(scx);
        let mut frame: Vec<Value> = Vec::new();
        let r = self.eval_core(&mut cx, &crate::core::Core::Global(id), &mut frame);
        // The probe's units are not part of the checked program.
        self.fns.truncate(before);
        match r {
            Ok(v) => out.value = Some(v),
            Err(e) => out.eval_error = Some(e.to_string()),
        }
        Ok(out)
    }
}


// ------------------------------------------------------------ collection and programs

/// Expected type of a compiler primitive, used to validate `foreign import`
/// declarations of `safelang.prim.*` and `safelang.runtime.*` intrinsics.
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
            Ty::Thunk { inner, .. } | Ty::Lazy { inner, .. } => {
                // A thunk's body runs at force time: the same positive
                // reading as a product component.
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
        Ty::Thunk { inner, .. } | Ty::Lazy { inner, .. } => ty_mentions(inner, name),
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

impl Checker {
    fn install_builtins(&mut self) {
        for n in ["Nat", "Int", "Bool", "Unit", "Bytes"] {
            self.types.entry(n.to_string()).or_default().push(TypeKind {
                module: "<builtin>".into(),
                name: n.into(),
                arity: 0,
                param_kinds: Vec::new(),
                ctors: Vec::new(),
                fields: None,
                field_erased: Vec::new(),
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
            field_erased: Vec::new(),
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
                field_erased: Vec::new(),
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
        });
        self.install_index_families();
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
                    ParamKind::Type => type_params.push(p.clone()),
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
            field_erased: Vec::new(),
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
                (CPat::Con { tag: 0, fields: vec![] }, crate::core::Core::Fault),
                (
                    CPat::Con { tag: 1, fields: vec![LocalId(2), LocalId(3)] },
                    crate::core::Core::If(
                        Box::new(i_is_zero),
                        Box::new(crate::core::Core::Var(LocalId(2))),
                        Box::new(crate::core::Core::App(
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
        });
        self.fns.push(FnDef {
            id: GlobalId("<builtin>.getCopy".to_string()),
            params: vec!["_d".to_string(), "v".to_string(), "fin".to_string()],
            body: crate::core::Core::Let(
                LocalId(3),
                Box::new(crate::core::Core::Proj(
                    Box::new(crate::core::Core::Var(LocalId(2))),
                    0,
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
        });
        // Evidence constructors (core specification section 3): `refl`
        // proves `Eq x x`, `congr` lifts equality across a named total
        // function. Both are erased: they check as proof terms and never
        // become runtime code.
        let refl = ValueInfo {
            module: "<builtin>".to_string(),
            name: "refl".to_string(),
            scheme: Scheme {
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

    /// Decompose a class application like `Add (List a)`.
    fn class_app_of(&self, ty: &ast::Type, vars: &[String]) -> Result<(String, Vec<Ty>)> {
        let mut args = Vec::new();
        let mut cur = ty;
        loop {
            match cur {
                ast::Type::App(f, a, _) => {
                    args.push(self.conv_type(a, vars)?);
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
                    return Ok((sig.name, args));
                }
                other => {
                    return Err(Error::at(
                        other.span(),
                        "expected a signature application like `Add a`",
                    ));
                }
            }
        }
    }

    fn collect_module(
        &mut self,
        m: &ast::Module,
        prelude_name: &str,
        imports: &[String],
    ) -> Result<()> {
        let module = m.name.text.clone();
        self.current_module = module.clone();
        // Visibility: the module's own names, then its imports, then the
        // prelude (design note: compiled interface metadata, section 3).
        self.visible = vec![module.clone()];
        for i in imports {
            if !self.visible.contains(i) {
                self.visible.push(i.clone());
            }
        }
        if !self.visible.contains(&prelude_name.to_string()) {
            self.visible.push(prelude_name.to_string());
        }
        self.module_imports.insert(module.clone(), imports.to_vec());
        let mut schemes: HashMap<String, Scheme> = HashMap::new();
        let mut eqs: Vec<(String, ast::Equation)> = Vec::new();

        for d in &m.decls {
            match d {
                ast::Decl::Import(i) => {
                    let parts = i.path.parts();
                    let name = parts.last().copied().unwrap_or("");
                    if name != prelude_name && !imports.iter().any(|x| x == name) {
                        return Err(Error::at(
                            i.span,
                            format!(
                                "module {module} imports {name}, which the driver did not resolve"
                            ),
                        ));
                    }
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
                ast::Decl::ValueSig { name, modifier, ty, span } => {
                    let logical = match modifier.as_ref().map(|m| m.text.as_str()) {
                        None => None,
                        Some("spec") => Some("spec"),
                        Some("proof") => Some("proof"),
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
                    schemes.insert(name.text.clone(), s);
                }
                ast::Decl::Equation { modifier, eq } => {
                    match modifier.as_ref().map(|m| m.text.as_str()) {
                        None | Some("spec") | Some("proof") => {}
                        Some(m) => {
                            return Err(Error::at(
                                modifier.as_ref().unwrap().span,
                                format!("modifier {m} is not supported in the bootstrap slice"),
                            ))
                        }
                    }
                    if let Some(mod_) = modifier {
                        if !self.logical.contains(&(module.clone(), eq.name.text.clone()))
                            && schemes.get(&eq.name.text).is_some()
                        {
                            return Err(Error::at(
                                mod_.span,
                                format!(
                                    "the signature of {} has no `{}` modifier",
                                    eq.name.text, mod_.text
                                ),
                            ));
                        }
                    }
                    eqs.push((eq.name.text.clone(), eq.clone()));
                }
                ast::Decl::Foreign(f) => {
                    if !f.is_import {
                        return Err(Error::at(f.span, "foreign export is not supported in the bootstrap slice"));
                    }
                    if f.abi != "ccall" {
                        return Err(Error::at(
                            f.span,
                            "only the ccall ABI is supported in the bootstrap slice",
                        ));
                    }
                    let Some(op) = PrimOp::from_symbol(&f.symbol) else {
                        return Err(Error::at(
                            f.span,
                            format!(
                                "foreign symbol {} is not a safelang intrinsic; only safelang.prim.* and safelang.runtime.* intrinsics are supported in the bootstrap slice",
                                f.symbol
                            ),
                        ));
                    };
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
                            notes: "safelang intrinsic".to_string(),
                        },
                    ));
                }
                ast::Decl::Data(data) => {
                    if Scalar::parse(&data.name.text).is_some()
                        || matches!(data.name.text.as_str(), "Nat" | "Int" | "Bool" | "Unit" | "Bytes")
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
                        field_erased: Vec::new(),
                    });
                    for (tag, c) in data.ctors.iter().enumerate() {
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
                                    ParamKind::Type => {
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
                                        ParamKind::Type => Ty::Param(params[i].clone()),
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
                                ParamKind::Type => type_params.push(p.clone()),
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
                                },
                            },
                            c.name.span,
                        )?;
                        ctors.push((c.name.text.clone(), tag));
                    }
                    let entry = self
                        .types
                        .get_mut(&data.name.text)
                        .and_then(|ts| ts.iter_mut().find(|t| t.module == module))
                        .expect("type registered above");
                    entry.ctors = ctors;
                }
                ast::Decl::Record(rec) => {
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
                    for f in &rec.fields {
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
                        field_erased: field_erased.clone(),
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
                    let res_args: Vec<Ty> = params
                        .iter()
                        .zip(&param_kinds)
                        .map(|(p, k)| match k {
                            ParamKind::Type => Ty::Param(p.clone()),
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
                            ParamKind::Type => type_params.push(p.clone()),
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
                    let params: Vec<String> = sig.params.iter().map(|p| p.text.clone()).collect();
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
                                    for c in cs {
                                        let (n, app) = self.conv_class(c, &params)?;
                                        own_ctx.push((n, app));
                                    }
                                    rest = body;
                                }
                                _ => break,
                            }
                        }
                        let mut vars = params.clone();
                        vars.extend(extra.iter().cloned());
                        let ty = self.conv_type(rest, &vars)?;
                        if ty.arity() == 0 {
                            return Err(Error::at(
                                mname.span,
                                format!("class method {} must have a function type in the bootstrap slice", mname.text),
                            ));
                        }
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
                            self.add_value(info, mname.span)?;
                        }
                    }
                    let info = SigInfo {
                        module: module.clone(),
                        name: sig.name.text.clone(),
                        params,
                        methods,
                    };
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
                    let params: Vec<String> = inst.params.iter().map(|p| p.text.clone()).collect();
                    let mut ctx = Vec::new();
                    for c in &inst.ctx {
                        ctx.push(self.conv_class(c, &params)?);
                    }
                    let (class, args) = self.class_app_of(&inst.target, &params)?;
                    if class == "Copy" {
                        return Err(Error::at(
                            inst.span,
                            "Copy is a compiler-known class with structural instances (ownership and usage design, section 3); it has no user instances",
                        ));
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
                    });
                    self.instances.entry(class).or_default().push(idx);
                    self.instance_defs_pending.push((module.clone(), inst.clone()));
                }
            }
        }

        // Top-level values resolve by name from other bodies, including
        // their own (recursion).
        for (name, scheme) in &schemes {
            self.add_value(
                ValueInfo {
                    module: module.clone(),
                    name: name.clone(),
                    scheme: scheme.clone(),
                    kind: ValueKind::TopFn,
                },
                m.name.span,
            )?;
        }

        // Every equation needs a signature; every signature needs an equation.
        for (name, eq) in &eqs {
            if !schemes.contains_key(name) {
                return Err(Error::at(
                    eq.span,
                    format!("missing signature for {name} (this draft does not silently infer top-level types)"),
                ));
            }
        }
        for (name, _) in &schemes {
            if name != "main" && !eqs.iter().any(|(n, _)| n == name) {
                return Err(Error::plain(format!("missing defining equation for {name}")));
            }
        }
        for (name, scheme) in schemes {
            self.top_schemes.insert((module.clone(), name), scheme);
        }
        for (name, eq) in eqs {
            self.top_eqs.push((module.clone(), name, eq));
        }
        Ok(())
    }

    fn check_bodies(&mut self, module: &str, prelude_name: &str) -> Result<()> {
        self.current_module = module.to_string();
        self.visible = vec![module.to_string()];
        if let Some(imports) = self.module_imports.get(module).cloned() {
            for i in imports {
                if !self.visible.contains(&i) {
                    self.visible.push(i);
                }
            }
        }
        if !self.visible.contains(&prelude_name.to_string()) {
            self.visible.push(prelude_name.to_string());
        }

        // Group this module's equations by name, preserving order.
        let mut names: Vec<String> = Vec::new();
        for (m, name, _) in &self.top_eqs {
            if m == module && !names.contains(name) {
                names.push(name.clone());
            }
        }
        for name in names {
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
            self.check_equations(&name, &id, &scheme, &eqs, Vec::new())?;
        }

        // Mutually recursive logical definitions would let a looping
        // function inhabit a `Prop`: the per-definition structural check
        // covers self-recursion, and this covers reference cycles among
        // `spec`/`proof` definitions (core specification sections 3
        // and 8).
        self.check_logical_cycles()?;

        // Constructors as code generation units: the block stores the
        // non-erased fields only (core specification section 3).
        for t in self.types.values().flatten().filter(|t| t.module == module).cloned().collect::<Vec<_>>() {
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
                });
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
            self.check_instance(&m, &inst)?;
        }
        Ok(())
    }

    fn check_instance(&mut self, module: &str, inst: &ast::InstDecl) -> Result<()> {
        let params: Vec<String> = inst.params.iter().map(|p| p.text.clone()).collect();
        let (class, args) = self.class_app_of(&inst.target, &params)?;
        let sig = self
            .sigs
            .get(&class)
            .and_then(|ss| ss.iter().find(|s| self.visible.contains(&s.module)))
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

        let subdict_count = inst.ctx.len();
        let mut method_fns = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for (mname, mscheme) in sig.methods.clone() {
            let eqs: Vec<ast::Equation> = inst
                .methods
                .iter()
                .filter(|e| e.name.text == mname)
                .cloned()
                .collect();
            if eqs.is_empty() {
                return Err(Error::at(
                    inst.span,
                    format!("instance {} does not define method {mname}", inst.name.text),
                ));
            }
            // Substitute the class parameters with the instance's type
            // arguments; the method's own quantifiers become fresh rigid
            // parameters.
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
            // Method function parameters are the instance's subdicts followed
            // by any extra constraints of the method itself; the class
            // constraint is satisfied by this instance and takes no parameter.
            let inst_ctx = self.instance_list[self_idx].ctx.clone();
            let mut ctx: Vec<(String, ClassApp)> = inst_ctx
                .iter()
                .map(|(n, app)| (n.clone(), Self::subst_class(app, &map)))
                .collect();
            for (n, c) in mscheme.ctx.iter().skip(1) {
                ctx.push((n.clone(), Self::subst_class(c, &map)));
            }
            let gid = GlobalId(format!("{module}.{}.{}", inst.name.text, mname));
            let scheme = Scheme {
                params: extra,
                idx_params: Vec::new(),
                val_params: Vec::new(),
                prop_params: Vec::new(),
                row_params: mscheme.row_params.clone(),
                region_params: mscheme.region_params.clone(),
                ctx,
                ty,
            };
            self.check_equations(&mname, &gid, &scheme, &eqs, Vec::new())?;
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

        let base_id = fn_id(module, &inst.name.text);
        self.instance_defs.push(InstanceDef {
            id: base_id,
            method_count: sig.methods.len(),
            subdict_count,
            methods: method_fns,
        });
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

    fn check_equations(
        &mut self,
        name: &str,
        id: &GlobalId,
        scheme: &Scheme,
        eqs: &[ast::Equation],
        extra_dicts: Vec<(LocalId, ClassApp)>,
    ) -> Result<()> {
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
        let mut env = Env::default();
        let mut param_ids = Vec::new();
        for (l, c) in extra_dicts {
            env.dicts.push((l, c));
            param_ids.push(l);
        }
        for (n, c) in &ctx {
            let l = self.fresh_local();
            env.dicts.push((l, c.clone()));
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
                        format!("{name} has fewer parameters than its signature ({other})"),
                    ));
                }
            }
        }
        if n_pats > 0
            && matches!(self.uni.zonk(&rest), Ty::Fun(_, _, _, _) | Ty::Pi { .. })
        {
            return Err(Error::at(
                eqs[0].span,
                format!("{name} has more parameters than its signature"),
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
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let covered = self.matrix_covered(&rows, &doms)?;
        // The body runs at the last application, so its effects belong to
        // the last arrow; intermediate arrows are pure closure construction,
        // and a nullary definition is pure.
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
        self.check_usage()?;
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
            return self.finish_fn(id.clone(), param_ids, body, None, None, true);
        }
        self.pending_refs.clear();
        self.current_logical = None;
        self.finish_fn(id.clone(), param_ids, body, None, None, false)
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
                                    ECore::Hole(_) | ECore::IdxPending(_) | ECore::Unit => {}
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
                    walk(t, id, smaller, extra, params, bad);
                    walk(f, id, smaller, extra, params, bad);
                }
                ECore::LazyInit(_, t) => walk(t, id, smaller, extra, params, bad),
                ECore::Case { scrut, alts } => {
                    walk(scrut, id, smaller, extra, params, bad);
                    for (_, b) in alts {
                        walk(b, id, smaller, extra, params, bad);
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
    ) -> Result<()> {
        let sol = self.solve_wants()?;
        body = self.fill_holes(&body, &sol)?;
        self.resolve_literals(&mut body)?;
        if !erased {
            self.resolve_idx_pends(&mut body)?;
        } else {
            self.idx_pends.clear();
        }
        let mut hoisted = Vec::new();
        hoist_lambdas(&mut body, &id.0, &mut self.lam_counter, &mut hoisted);
        if !erased {
            self.fns.push(finalize_fn(id, param_ids, body, prim, ctor));
            for h in hoisted {
                self.fns.push(finalize_fn(h.id, h.params, h.body, None, None));
            }
        }
        Ok(())
    }

    fn finish(mut self, user: &ast::Module, prelude_name: &str) -> Result<Program> {
        let user_module = user.name.text.clone();
        self.current_module = user_module.clone();
        self.visible = vec![user_module.clone()];
        if let Some(imports) = self.module_imports.get(&user_module).cloned() {
            for i in imports {
                if !self.visible.contains(&i) {
                    self.visible.push(i);
                }
            }
        }
        if !self.visible.contains(&prelude_name.to_string()) {
            self.visible.push(prelude_name.to_string());
        }
        let main_info = match self.lookup_value(None, "main") {
            Ok(info) if info.module == user_module => Some(info),
            // Library modules check without an entry point; `emit-ir`,
            // `build`, and `run` name the requirement at codegen time.
            _ => None,
        };
        let mut main_exit = None;
        if let Some(main_info) = main_info {
            if !main_info.scheme.params.is_empty() || !main_info.scheme.ctx.is_empty() {
                return Err(Error::plain("main must have type Int in the bootstrap slice"));
            }
            if self.uni.zonk(&main_info.scheme.ty) != Ty::Int {
                return Err(Error::plain(format!(
                    "main must have type Int in the bootstrap slice, got {}",
                    self.uni.zonk(&main_info.scheme.ty)
                )));
            }
            // The exit adapter (mathematical `Int` slice): `main :: Int`
            // returns a boxed bignum while the C `main` truncates its unit's
            // word to `i32`, so the checker synthesizes `"<module>.main.exit"`,
            // whose body converts the `main` thunk's result to a machine `i64`
            // through `sl_int_to_i64`, and `Program.main` points at it.
            let main_id = fn_id(&user_module, "main");
            let exit_id = GlobalId(format!("{user_module}.main.exit"));
            self.fns.push(FnDef {
                id: exit_id.clone(),
                params: Vec::new(),
                body: crate::core::Core::App(
                    Box::new(crate::core::Core::Prim(PrimOp::Int(IntOp::ToI64))),
                    vec![crate::core::Core::Global(main_id)],
                ),
                prim: None,
                ctor: None,
            });
            main_exit = Some(exit_id);
        }
        Ok(Program {
            fns: self.fns,
            instances: self.instance_defs,
            main: main_exit,
        })
    }
}

fn wrap_eq_body(e: &ast::Equation) -> ast::Term {
    if e.locals.is_empty() {
        e.body.clone()
    } else {
        ast::Term::Let(e.locals.clone(), Box::new(e.body.clone()), e.span)
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

/// A module's declared import names.
fn imports_of(m: &ast::Module) -> Vec<String> {
    m.decls
        .iter()
        .filter_map(|d| match d {
            ast::Decl::Import(i) => i.path.parts().last().map(|s| s.to_string()),
            _ => None,
        })
        .collect()
}

impl Checker {
    /// Instance coherence across the compiled set (design note, section 3,
    /// rule 4): two instances of one class with unifiable targets are
    /// ambiguous. The metadata-level check in `interface.rs` is the fast
    /// path; this is the authoritative check with real type machinery.
    fn check_instance_coherence(&mut self) -> Result<()> {
        let n = self.instance_list.len();
        for a in 0..n {
            for b in (a + 1)..n {
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

    /// Build one module's `.sliface` content (design note, section 2).
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
        for v in self.qualified.values() {
            if v.module != module {
                continue;
            }
            match &v.kind {
                ValueKind::TopFn | ValueKind::Prim(_) => {
                    iface.exports.values.push(ValueExport {
                        name: v.name.clone(),
                        scheme: render_scheme(&v.scheme),
                        symbol: format!("{module}.{}", v.name),
                        arity: (v.scheme.ty.arity()
                            + v.scheme.ctx.len()
                            + v.scheme.val_params.len()) as u64,
                    });
                }
                _ => {}
            }
        }
        for ts in self.types.values() {
            for t in ts {
                if t.module != module {
                    continue;
                }
                let mut ctors = Vec::new();
                let mut params: Vec<String> = Vec::new();
                for (cname, tag) in &t.ctors {
                    if let Some(v) = self.qualified.get(&(module.to_string(), cname.clone())) {
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
                                .collect();
                            ctors.push(CtorExport {
                                name: cname.clone(),
                                tag: *tag as u64,
                                fields,
                                erased: field_erased.clone(),
                            });
                        }
                    }
                }
                iface.exports.types.push(TypeExport {
                    name: t.name.clone(),
                    params,
                    kind: if t.fields.is_some() { "record".to_string() } else { "data".to_string() },
                    ctors,
                    fields: t.fields.clone().unwrap_or_default(),
                });
            }
        }
        for s in self.sigs.values().flatten() {
            if s.module != module {
                continue;
            }
            iface.exports.signatures.push(SigExport {
                name: s.name.clone(),
                params: s.params.clone(),
                methods: s
                    .methods
                    .iter()
                    .map(|(n, sch)| MethodExport {
                        name: n.clone(),
                        scheme: render_scheme(sch),
                    })
                    .collect(),
            });
        }
        for i in &self.instance_list {
            if i.module != module {
                continue;
            }
            iface.exports.instances.push(InstanceExport {
                id: i.base_id.0.clone(),
                class: i.target.class.clone(),
                target: i.target.to_string(),
                ctx: i.ctx.iter().map(|(n, c)| format!("{n} :: {c}")).collect(),
                methods: Vec::new(),
            });
        }
        for (m, f) in &self.module_fixities {
            if m == module {
                iface.exports.fixities.push(f.clone());
            }
        }
        let mut prims = Vec::new();
        for (m, f) in &self.module_foreign {
            if m == module {
                iface.exports.foreign.push(f.clone());
                prims.push(f.external.clone());
            }
        }
        iface.assumptions.primitives = prims;
        iface
    }
}

/// Check a program of one or more modules and elaborate it. `modules` is in
/// dependency order (imports first); the last module is the entry module and
/// must define `main :: Int`. Returns the program and one interface per
/// module (prelude first), each carrying its import identities.
pub fn check_program_multi(
    prelude: &ast::Module,
    modules: &[ast::Module],
    source_hashes: &HashMap<String, String>,
) -> Result<(Program, Vec<crate::interface::Iface>)> {
    let mut c = Checker::new();
    c.install_builtins();
    let prelude_name = prelude.name.text.clone();
    c.prelude_module = prelude_name.clone();
    c.collect_module(prelude, &prelude_name, &[])?;
    for m in modules {
        let imports = imports_of(m);
        c.collect_module(m, &prelude_name, &imports)?;
    }
    c.check_bodies(&prelude_name, &prelude_name)?;
    for m in modules {
        c.check_bodies(&m.name.text, &prelude_name)?;
    }
    c.check_instance_coherence()?;
    let entry = modules
        .last()
        .ok_or_else(|| Error::plain("no modules to check"))?;
    // Interfaces in dependency order: imports are known when built.
    let mut ifaces = Vec::new();
    let mut built: Vec<(String, String)> = Vec::new(); // (module, iface_hash)
    let mut emit = |c: &Checker, module: &str, imports: &[String]| {
        let mut refs: Vec<(String, String)> = Vec::new();
        for i in imports {
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
    // The prelude has no imports; other modules implicitly import it.
    emit(&c, &prelude_name, &[]);
    for m in modules {
        let mut imports = imports_of(m);
        if m.name.text != prelude_name && !imports.contains(&prelude_name) {
            imports.push(prelude_name.clone());
        }
        emit(&c, &m.name.text, &imports);
    }
    let program = c.finish(entry, &prelude_name)?;
    Ok((program, ifaces))
}

/// Check a single user module against the implicit prelude and elaborate it.
pub fn check_program(prelude: &ast::Module, user: &ast::Module) -> Result<Program> {
    let mut hashes = HashMap::new();
    hashes.insert(user.name.text.clone(), String::new());
    Ok(check_program_multi(prelude, std::slice::from_ref(user), &hashes)?.0)
}

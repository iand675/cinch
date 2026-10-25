
// ------------------------------------------------------------ collection and programs

/// Expected type of a compiler primitive, used to validate `foreign import`
/// declarations of `safelang.prim.*` and `safelang.runtime.*` intrinsics.
fn prim_scheme(op: PrimOp) -> Scheme {
    let (args, res) = op.sig();
    Scheme::mono(Ty::funs(&args, res))
}

impl Checker {
    fn install_builtins(&mut self) {
        for n in ["Nat", "Int", "Bool", "Unit", "Bytes"] {
            self.types.entry(n.to_string()).or_default().push(TypeKind {
                module: "<builtin>".into(),
                name: n.into(),
                arity: 0,
                ctors: Vec::new(),
                fields: None,
            });
        }
        // `Io r`: runtime capability evidence for identity `r` (design
        // section 4, decision D21). The value is a uniform word; the
        // identity lives in the type.
        self.types.entry("Io".to_string()).or_default().push(TypeKind {
            module: "<builtin>".into(),
            name: "Io".into(),
            arity: 1,
            ctors: Vec::new(),
            fields: None,
        });
        // Machine scalars are primitives of the language, not user types
        // (primitive types specification, section 2).
        for s in Scalar::ALL {
            self.types.entry(s.name().to_string()).or_default().push(TypeKind {
                module: "<builtin>".into(),
                name: s.name().into(),
                arity: 0,
                ctors: Vec::new(),
                fields: None,
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
                    if let Some(mod_) = modifier {
                        return Err(Error::at(
                            mod_.span,
                            format!("modifier {} is not supported in the bootstrap slice", mod_.text),
                        ));
                    }
                    if schemes.contains_key(&name.text) {
                        return Err(Error::at(*span, format!("duplicate signature for {}", name.text)));
                    }
                    schemes.insert(name.text.clone(), self.conv_scheme(ty)?);
                }
                ast::Decl::Equation { modifier, eq } => {
                    if let Some(mod_) = modifier {
                        return Err(Error::at(
                            mod_.span,
                            format!("modifier {} is not supported in the bootstrap slice", mod_.text),
                        ));
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
                    let params: Vec<String> = data.params.iter().map(|p| p.text.clone()).collect();
                    let mut ctors: Vec<(String, usize)> = Vec::new();
                    // Register the type before converting constructor types,
                    // which may mention the type being declared.
                    self.types.entry(data.name.text.clone()).or_default().push(TypeKind {
                        module: module.clone(),
                        name: data.name.text.clone(),
                        arity: params.len(),
                        ctors: Vec::new(),
                        fields: None,
                    });
                    for (tag, c) in data.ctors.iter().enumerate() {
                        // Constructor type: optional extra foralls, then
                        // arrows into the declared type applied to exactly
                        // its parameters.
                        let mut extra: Vec<String> = Vec::new();
                        let mut rest = &c.ty;
                        while let ast::Type::Forall(binders, body, _) = rest {
                            for b in binders {
                                extra.push(b.name.text.clone());
                            }
                            rest = body;
                        }
                        let mut vars = params.clone();
                        vars.extend(extra.iter().cloned());
                        let mut field_tys: Vec<(Ty, Mult)> = Vec::new();
                        let mut cur = rest;
                        let result = loop {
                            match cur {
                                ast::Type::Fun { arg, res, mult, eff, span } => {
                                    if let Some(e) = eff {
                                        return Err(Error::at(
                                            e.span,
                                            "effect rows are not supported in the bootstrap slice",
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
                                    field_tys.push((t, m));
                                    cur = res;
                                }
                                other => break other,
                            }
                        };
                        let res_ty = self.conv_type(result, &vars)?;
                        let expected = Ty::Adt(
                            data.name.text.clone(),
                            params.iter().map(|p| Ty::Param(p.clone())).collect(),
                        );
                        if res_ty != expected {
                            return Err(Error::at(
                                c.name.span,
                                format!(
                                    "constructor {} must return {}; computed indices are not supported in the bootstrap slice",
                                    c.name.text, expected
                                ),
                            ));
                        }
                        let mut scheme_params = params.clone();
                        scheme_params.extend(extra);
                        let field_plain: Vec<Ty> =
                            field_tys.iter().map(|(t, _)| t.clone()).collect();
                        let scheme = Scheme {
                            params: scheme_params.clone(),
                            row_params: Vec::new(),
                            region_params: Vec::new(),
                            ctx: Vec::new(),
                            ty: field_tys.iter().rev().fold(res_ty, |acc, (t, m)| {
                                Ty::fun_mult(t.clone(), *m, EffRow::pure(), acc)
                            }),
                        };
                        self.add_value(
                            ValueInfo {
                                module: module.clone(),
                                name: c.name.text.clone(),
                                scheme,
                                kind: ValueKind::Ctor {
                                    tag,
                                    arity: field_plain.len(),
                                    field_tys: field_plain,
                                    ty_params: params.clone(),
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
                    let params: Vec<String> = rec.params.iter().map(|p| p.text.clone()).collect();
                    let mut field_names: Vec<String> = Vec::new();
                    for f in &rec.fields {
                        if field_names.iter().any(|n| n == &f.name.text) {
                            return Err(Error::at(
                                f.name.span,
                                format!("duplicate record field {}", f.name.text),
                            ));
                        }
                        field_names.push(f.name.text.clone());
                    }
                    // Register the type before converting field types, which
                    // may mention the record being declared.
                    self.types.entry(rec.name.text.clone()).or_default().push(TypeKind {
                        module: module.clone(),
                        name: rec.name.text.clone(),
                        arity: params.len(),
                        ctors: Vec::new(),
                        fields: Some(field_names.clone()),
                    });
                    let vars = params.clone();
                    let mut field_tys = Vec::new();
                    for f in &rec.fields {
                        field_tys.push(self.conv_type(&f.ty, &vars)?);
                    }
                    let res_ty = Ty::Adt(
                        rec.name.text.clone(),
                        params.iter().map(|p| Ty::Param(p.clone())).collect(),
                    );
                    let scheme = Scheme {
                        params: params.clone(),
                        row_params: Vec::new(),
                        region_params: Vec::new(),
                        ctx: Vec::new(),
                        ty: field_tys.iter().rev().fold(res_ty, |acc, t| {
                            Ty::fun_mult(t.clone(), default_mult(t), EffRow::pure(), acc)
                        }),
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
                                field_tys,
                                ty_params: params.clone(),
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

        // Constructors as code generation units.
        for t in self.types.values().flatten().filter(|t| t.module == module).cloned().collect::<Vec<_>>() {
            for (cname, tag) in &t.ctors {
                let v = self
                    .qualified
                    .get(&(module.to_string(), cname.clone()))
                    .cloned()
                    .unwrap();
                let (arity, _) = match &v.kind {
                    ValueKind::Ctor { arity, field_tys, .. } => (*arity, field_tys.len()),
                    _ => unreachable!(),
                };
                self.fns.push(FnDef {
                    id: fn_id(module, cname),
                    params: (0..arity).map(|i| format!("f{i}")).collect(),
                    body: crate::core::Core::Unit,
                    prim: None,
                    ctor: Some((*tag, arity)),
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

    fn check_equations(
        &mut self,
        name: &str,
        id: &GlobalId,
        scheme: &Scheme,
        eqs: &[ast::Equation],
        extra_dicts: Vec<(LocalId, ClassApp)>,
    ) -> Result<()> {
        let (ty, ctx, _map) = self.rigidify(scheme);
        self.reset_usage();
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
        for _ in 0..n_pats {
            rest = self.uni.zonk(&rest);
            match rest {
                Ty::Fun(a, m, eff, b) => {
                    doms.push(*a);
                    mults.push(m);
                    effs.push(eff);
                    rest = *b;
                }
                other => {
                    return Err(Error::at(
                        eqs[0].span,
                        format!("{name} has fewer parameters than its signature ({other})"),
                    ));
                }
            }
        }
        if n_pats > 0 && matches!(self.uni.zonk(&rest), Ty::Fun(_, _, _, _)) {
            return Err(Error::at(
                eqs[0].span,
                format!("{name} has more parameters than its signature"),
            ));
        }
        let mut value_params = Vec::new();
        for _ in 0..n_pats {
            value_params.push(self.fresh_local());
        }
        param_ids.extend(value_params.iter().copied());
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
        self.finish_fn(id.clone(), param_ids, body, None, None)
    }

    /// Solve the equation's dictionary wants, hoist lambdas and emit the
    /// code generation unit plus its hoisted lambdas.
    fn finish_fn(
        &mut self,
        id: GlobalId,
        param_ids: Vec<LocalId>,
        mut body: ECore,
        prim: Option<PrimOp>,
        ctor: Option<(usize, usize)>,
    ) -> Result<()> {
        let sol = self.solve_wants()?;
        body = self.fill_holes(&body, &sol)?;
        self.resolve_literals(&mut body)?;
        let mut hoisted = Vec::new();
        hoist_lambdas(&mut body, &id.0, &mut self.lam_counter, &mut hoisted);
        self.fns.push(finalize_fn(id, param_ids, body, prim, ctor));
        for h in hoisted {
            self.fns.push(finalize_fn(h.id, h.params, h.body, None, None));
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
    let all: Vec<&String> = s
        .params
        .iter()
        .chain(s.row_params.iter())
        .chain(s.region_params.iter())
        .collect();
    if !all.is_empty() {
        out.push_str("forall ");
        for p in all {
            out.push_str(p);
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
                        arity: (v.scheme.ty.arity() + v.scheme.ctx.len()) as u64,
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
                        if let ValueKind::Ctor { arity, field_tys, ty_params, .. } = &v.kind {
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
                                erased: vec![false; *arity],
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

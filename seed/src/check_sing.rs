// Zero-cost runtime singletons (`Sing`), `from-sing`, `with-sing`, `sing`,
// `sing-eq`, direct `Sing` pattern matching with branch refinement, and direct
// singleton-lifted calls to promoted functions and constructors.
//
// Runtime representation invariant: `Sing T x` has the EXACT same runtime word
// and reference-counting `Class` as `T` (`repr.rs`). Therefore:
// - `from-sing s` compiles to `s` (zero instructions, zero allocation).
// - `with-sing v (\s -> body)` compiles to `let s = v in body` (zero copy, zero closure).
// - `f sa sb` for a promoted function `f` calls the compiled `f` directly on `sa, sb`.
// - `case s of C a b -> ...` compiles to a direct `Case` on `T` (zero wrapper unboxing).

impl Checker {
    fn next_sing_skolem(&mut self, prefix: &str) -> String {
        let id = self.gadt_skolem_counter;
        self.gadt_skolem_counter += 1;
        format!("${prefix}_{id}")
    }

    /// Replace exact occurrences of a rigid singleton target (`Ty::Param`,
    /// `Ty::Idx(ITerm::Var)`, or `Ty::Idx(ITerm::LocalValue)`) inside `ty`.
    fn subst_sing_target(ty: &Ty, target: &Ty, replacement: &Ty) -> Ty {
        if ty == target {
            return replacement.clone();
        }
        match ty {
            Ty::Adt(n, args) => Ty::Adt(
                n.clone(),
                args.iter()
                    .map(|a| Self::subst_sing_target(a, target, replacement))
                    .collect(),
            ),
            Ty::Tuple(args) => Ty::Tuple(
                args.iter()
                    .map(|a| Self::subst_sing_target(a, target, replacement))
                    .collect(),
            ),
            Ty::App(h, args) => Ty::App(
                Box::new(Self::subst_sing_target(h, target, replacement)),
                args.iter()
                    .map(|a| Self::subst_sing_target(a, target, replacement))
                    .collect(),
            ),
            Ty::Fun(a, m, e, b) => Ty::fun_mult(
                Self::subst_sing_target(a, target, replacement),
                *m,
                e.clone(),
                Self::subst_sing_target(b, target, replacement),
            ),
            Ty::Pi { var, dom, mult, eff, cod } => Ty::Pi {
                var: var.clone(),
                dom: Box::new(Self::subst_sing_target(dom, target, replacement)),
                mult: *mult,
                eff: eff.clone(),
                cod: Box::new(Self::subst_sing_target(cod, target, replacement)),
            },
            Ty::Sigma { var, dom, cod } => Ty::Sigma {
                var: var.clone(),
                dom: Box::new(Self::subst_sing_target(dom, target, replacement)),
                cod: Box::new(Self::subst_sing_target(cod, target, replacement)),
            },
            Ty::Idx(it) => {
                if let (Ty::Idx(t_it), Ty::Idx(r_it)) = (target, replacement) {
                    if it == t_it {
                        return replacement.clone();
                    }
                    return Ty::Idx(Box::new(Self::subst_iterm_exact(it, t_it, r_it)));
                }
                if let ITerm::TyFamily(name, args) = it.as_ref() {
                    return Ty::Idx(Box::new(ITerm::TyFamily(
                        name.clone(),
                        args.iter()
                            .map(|a| Self::subst_sing_target(a, target, replacement))
                            .collect(),
                    )));
                }
                ty.clone()
            }
            _ => ty.clone(),
        }
    }

    fn subst_iterm_exact(it: &ITerm, target: &ITerm, replacement: &ITerm) -> ITerm {
        if it == target {
            return replacement.clone();
        }
        match it {
            ITerm::Bin(op, a, b) => ITerm::Bin(
                *op,
                Box::new(Self::subst_iterm_exact(a, target, replacement)),
                Box::new(Self::subst_iterm_exact(b, target, replacement)),
            )
            .normalize(),
            ITerm::App(f, args) => ITerm::App(
                Box::new(Self::subst_iterm_exact(f, target, replacement)),
                args.iter()
                    .map(|a| Self::subst_iterm_exact(a, target, replacement))
                    .collect(),
            ),
            ITerm::TyFamily(name, args) => ITerm::TyFamily(
                name.clone(),
                args.iter()
                    .map(|a| {
                        Self::subst_sing_target(
                            a,
                            &Ty::Idx(Box::new(target.clone())),
                            &Ty::Idx(Box::new(replacement.clone())),
                        )
                    })
                    .collect(),
            ),
            _ => it.clone(),
        }
    }

    /// Apply a branch refinement (`target -> replacement`) to `expected`,
    /// `branch_tys`, and all local variable schemes in `branch_rows`, working
    /// uniformly for `Ty::Param`, `ITerm::Var`, AND `ITerm::LocalValue`.
    fn apply_sing_branch_refinement(
        &self,
        target: &Ty,
        replacement: &Ty,
        branch_expected: &mut Ty,
        branch_tys: &mut [Ty],
        branch_rows: &mut [Row],
    ) {
        *branch_expected = self.uni.zonk(&Self::subst_sing_target(branch_expected, target, replacement));
        for bt in branch_tys.iter_mut() {
            *bt = self.uni.zonk(&Self::subst_sing_target(bt, target, replacement));
        }
        for row in branch_rows.iter_mut() {
            for (_, _, scheme) in &mut row.env.locals {
                scheme.ty = self.uni.zonk(&Self::subst_sing_target(&scheme.ty, target, replacement));
            }
        }
    }

    /// Synthesize the runtime `ECore` value for `sing :: Sing carrier val`.
    /// Uses constants/constructors directly when `val` is concrete, or reuses
    /// in-scope `Sing` witnesses from `env.locals` when `val` contains rigid
    /// promoted variables.
    fn synthesize_sing(
        &mut self,
        env: &mut Env,
        carrier: &Ty,
        val: &Ty,
        span: ast::Span,
    ) -> Result<ECore> {
        let zcarrier = self.uni.zonk(carrier);
        let zval = self.uni.zonk(val);

        // 1. Reuse an in-scope `Sing` witness if one already holds `zval`.
        for (_, local, scheme) in env.locals.iter().rev() {
            if scheme.params.is_empty() && scheme.idx_params.is_empty() {
                if let Ty::Adt(name, args) = self.uni.zonk(&scheme.ty) {
                    if name == "Sing" && args.len() == 2 && self.uni.zonk(&args[1]) == zval {
                        return Ok(ECore::Var(*local));
                    }
                }
            }
        }

        // 2. Concrete promoted values and structured combinations.
        match &zval {
            Ty::Unit => Ok(ECore::Unit),
            Ty::Idx(it) => self.synthesize_sing_idx(env, &zcarrier, it, span),
            Ty::Tuple(items) => {
                let item_carriers = match &zcarrier {
                    Ty::Tuple(cs) if cs.len() == items.len() => cs.clone(),
                    _ => items
                        .iter()
                        .map(|v| self.infer_sing_carrier(v, span))
                        .collect::<Result<Vec<_>>>()?,
                };
                let gid = self.tuple_ctor(items.len(), span)?;
                let mut cores = Vec::with_capacity(items.len());
                for (c, v) in item_carriers.iter().zip(items.iter()) {
                    cores.push(self.synthesize_sing(env, c, v, span)?);
                }
                Ok(app_spine(ECore::Ctor(gid, item_carriers), cores))
            }
            Ty::Adt(id, args) => {
                let s = id.as_str();
                if s == crate::promote::PROMOTED_TRUE && args.is_empty() {
                    return Ok(ECore::Bool(true));
                }
                if s == crate::promote::PROMOTED_FALSE && args.is_empty() {
                    return Ok(ECore::Bool(false));
                }
                if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 && args.is_empty() {
                    return Ok(self.string_const(&s[1..s.len() - 1]));
                }
                if let Some(ctor_id) = s.strip_prefix("ctor:") {
                    if let Some((mod_part, ctor_name)) = ctor_id.rsplit_once('.') {
                        let val_info = self
                            .qualified
                            .get(&(mod_part.to_string(), ctor_name.to_string()))
                            .cloned()
                            .or_else(|| self.lookup_value(None, ctor_name).ok())
                            .ok_or_else(|| {
                                Error::at(
                                    span,
                                    format!("cannot synthesize runtime constructor `{ctor_name}` for `sing`"),
                                )
                            })?;
                        let ty_args = match &zcarrier {
                            Ty::Adt(_, targs) => targs.clone(),
                            _ => Vec::new(),
                        };
                        let parts = self.ctor_parts(&val_info, &ty_args)?;
                        let runtime_fields: Vec<Ty> = parts
                            .fields
                            .iter()
                            .filter(|(_, _, erased)| !*erased)
                            .map(|(_, t, _)| t.clone())
                            .collect();
                        if runtime_fields.len() != args.len() {
                            return Err(Error::at(
                                span,
                                format!(
                                    "cannot synthesize `sing` for constructor `{ctor_name}`: expected {} fields, got {}",
                                    runtime_fields.len(),
                                    args.len()
                                ),
                            ));
                        }
                        let mut cores = Vec::with_capacity(args.len());
                        for (ft, av) in runtime_fields.iter().zip(args.iter()) {
                            cores.push(self.synthesize_sing(env, ft, av, span)?);
                        }
                        let gid = GlobalId(format!("{}.{}", val_info.module, val_info.name));
                        let inst: Vec<Ty> = if let ValueKind::Ctor { ty_params, .. } = &val_info.kind {
                            ty_args.into_iter().take(ty_params.len()).collect()
                        } else {
                            Vec::new()
                        };
                        return Ok(app_spine(ECore::Ctor(gid, inst), cores));
                    }
                }
                if let Some(fn_id) = s.strip_prefix(crate::promote::PROMOTED_FN_PREFIX) {
                    if let Some(pfn) = self.uni.promoted_fn(fn_id).cloned() {
                        let mut arg_cores = Vec::with_capacity(args.len());
                        let mut arg_carriers = Vec::with_capacity(args.len());
                        for (i, av) in args.iter().enumerate() {
                            let ac = self
                                .infer_sing_carrier(av, span)
                                .or_else(|_| {
                                    pfn.param_tys.get(i).cloned().ok_or_else(|| {
                                        Error::at(span, "cannot infer argument carrier for `sing`")
                                    })
                                })?;
                            arg_cores.push(self.synthesize_sing(env, &ac, av, span)?);
                            arg_carriers.push(ac);
                        }
                        let (call_core, _) =
                            self.emit_promoted_runtime_call(&pfn, &arg_carriers, arg_cores, env, span)?;
                        return Ok(call_core);
                    }
                }
                Err(Error::at(
                    span,
                    crate::diagnostic!(
                        "cannot synthesize runtime singleton for `Sing ({})`: no `Sing` witness in scope",
                        zval
                    ),
                ))
            }
            Ty::Param(p) => Err(Error::at(
                span,
                format!(
                    "cannot synthesize runtime singleton for `Sing {p}`: no `Sing {p}` witness is in scope"
                ),
            )),
            _ => Err(Error::at(
                span,
                crate::diagnostic!(
                    "cannot synthesize runtime singleton for `Sing ({})`",
                    zval
                ),
            )),
        }
    }

    fn synthesize_sing_idx(
        &mut self,
        env: &mut Env,
        carrier: &Ty,
        it: &ITerm,
        span: ast::Span,
    ) -> Result<ECore> {
        let resolved = self.uni.resolve_idx(it).normalize();
        // Check in-scope `Sing` witness matching this exact index term.
        let as_ty = Ty::Idx(Box::new(resolved.clone()));
        for (_, local, scheme) in env.locals.iter().rev() {
            if scheme.params.is_empty() && scheme.idx_params.is_empty() {
                if let Ty::Adt(name, args) = self.uni.zonk(&scheme.ty) {
                    if name == "Sing" && args.len() == 2 && self.uni.zonk(&args[1]) == as_ty {
                        return Ok(ECore::Var(*local));
                    }
                }
            }
        }
        match &resolved {
            ITerm::Num(digits) => self.literal_const(carrier, digits, span),
            ITerm::Bin(op, a, b) => {
                let ca = self.synthesize_sing_idx(env, carrier, a, span)?;
                let cb = self.synthesize_sing_idx(env, carrier, b, span)?;
                let op_name = match op {
                    IdxOp::Add => "+",
                    IdxOp::Sub => "-",
                    IdxOp::Mul => "*",
                };
                self.emit_sing_binop_runtime(op_name, ca, carrier, cb, carrier, env, span)
                    .map(|(c, _)| c)
            }
            ITerm::TyFamily(head, args) => {
                if let Some(fn_id) = head.as_str().strip_prefix(crate::promote::PROMOTED_FN_PREFIX) {
                    if let Some(pfn) = self.uni.promoted_fn(fn_id).cloned() {
                        let mut arg_cores = Vec::with_capacity(args.len());
                        let mut arg_carriers = Vec::with_capacity(args.len());
                        for (i, av) in args.iter().enumerate() {
                            let ac = self
                                .infer_sing_carrier(av, span)
                                .or_else(|_| {
                                    pfn.param_tys.get(i).cloned().ok_or_else(|| {
                                        Error::at(span, "cannot infer argument carrier for `sing`")
                                    })
                                })?;
                            arg_cores.push(self.synthesize_sing(env, &ac, av, span)?);
                            arg_carriers.push(ac);
                        }
                        let (call_core, _) =
                            self.emit_promoted_runtime_call(&pfn, &arg_carriers, arg_cores, env, span)?;
                        return Ok(call_core);
                    }
                }
                Err(Error::at(
                    span,
                    crate::diagnostic!(
                        "cannot synthesize runtime singleton for `Sing ({})`: no `Sing` witness in scope",
                        as_ty
                    ),
                ))
            }
            ITerm::Var(v) => Err(Error::at(
                span,
                format!(
                    "cannot synthesize runtime singleton for `Sing {v}`: no `Sing {v}` witness is in scope"
                ),
            )),
            _ => Err(Error::at(
                span,
                crate::diagnostic!(
                    "cannot synthesize runtime singleton for `Sing ({})`",
                    as_ty
                ),
            )),
        }
    }

    /// Emit a runtime call to an already-compiled promoted function `pfn` on
    /// `arg_cores` (whose runtime types are `arg_carriers`), resolving any
    /// polymorphic type arguments through `self.infer_variable`.
    fn emit_promoted_runtime_call(
        &mut self,
        pfn: &crate::promote::PromotedFn,
        arg_carriers: &[Ty],
        arg_cores: Vec<ECore>,
        env: &mut Env,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let Some((mod_part, fn_name)) = pfn.identity.rsplit_once('.') else {
            return Err(Error::at(
                span,
                format!("internal error: invalid promoted function id `{}`", pfn.identity),
            ));
        };
        let v = self
            .qualified
            .get(&(mod_part.to_string(), fn_name.to_string()))
            .cloned()
            .or_else(|| self.lookup_value(None, fn_name).ok())
            .ok_or_else(|| Error::at(span, format!("unknown promoted function `{}`", pfn.identity)))?;
        let (mut fn_core, mut fn_ty) = self.value_use(&v, span, env)?;
        let mut applied_args = Vec::with_capacity(arg_cores.len());
        for (ac, a_core) in arg_carriers.iter().zip(arg_cores.into_iter()) {
            fn_ty = self.uni.zonk_arrow_head(fn_ty);
            if matches!(fn_ty, Ty::Forall(_)) {
                let (c, t) = self.poly_elim_all(fn_core, fn_ty);
                fn_core = c;
                fn_ty = t;
            }
            match fn_ty {
                Ty::Fun(dom, _, _, cod) | Ty::Pi { dom, cod, .. } => {
                    self.uni
                        .unify(ac, &dom)
                        .map_err(|m| Error::at(span, m))?;
                    applied_args.push(a_core);
                    fn_ty = *cod;
                }
                other => {
                    return Err(Error::at(
                        span,
                        crate::diagnostic!("cannot apply promoted function of type {}", other),
                    ));
                }
            }
        }
        let call_core = app_spine(fn_core, applied_args);
        Ok((call_core, self.uni.zonk(&fn_ty)))
    }

    /// Emit a runtime binary operator (`+`, `-`, `*`, `==`, `<`, `&&`, etc.)
    /// on two already-elaborated runtime values by checking the synthetic call
    /// through Cinch's own operator/class resolution.
    fn emit_sing_binop_runtime(
        &mut self,
        op_name: &str,
        lhs_core: ECore,
        lhs_ty: &Ty,
        rhs_core: ECore,
        rhs_ty: &Ty,
        env: &Env,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let l1 = self.fresh_local();
        let l2 = self.fresh_local();
        self.note_local_ty(l1, lhs_ty);
        self.note_local_ty(l2, rhs_ty);
        let mut sub_env = env.clone();
        sub_env.locals.push((
            "$sing_lhs".into(),
            l1,
            Scheme::mono(lhs_ty.clone()),
        ));
        sub_env.locals.push((
            "$sing_rhs".into(),
            l2,
            Scheme::mono(rhs_ty.clone()),
        ));
        let syn = ast::Term::App(
            Box::new(ast::Term::Var(ast::VarPath {
                quals: Vec::new(),
                name: ast::Name::new(op_name.to_string(), span),
            })),
            vec![
                ast::Term::Var(ast::VarPath {
                    quals: Vec::new(),
                    name: ast::Name::new("$sing_lhs".to_string(), span),
                }),
                ast::Term::Var(ast::VarPath {
                    quals: Vec::new(),
                    name: ast::Name::new("$sing_rhs".to_string(), span),
                }),
            ],
            span,
        );
        let (op_core, res_ty) = self.infer(&mut sub_env, &syn)?;
        let wrapped = ECore::Let(
            l1,
            Box::new(lhs_core),
            Box::new(ECore::Let(l2, Box::new(rhs_core), Box::new(op_core))),
        );
        Ok((wrapped, self.uni.zonk(&res_ty)))
    }

    /// Emit a runtime boolean equality test `l1 == l2` for ANY carrier type
    /// (works for scalars, `String`, tuples, `List`, `Option`, and user `data`
    /// types even without an explicit `Eq` instance!).
    fn emit_sing_eq_bool(
        &mut self,
        l1: LocalId,
        l2: LocalId,
        carrier: &Ty,
        env: &Env,
        span: ast::Span,
    ) -> Result<ECore> {
        let zc = self.uni.zonk(carrier);
        match &zc {
            Ty::Unit => Ok(ECore::Bool(true)),
            Ty::Bool => {
                // `if l1 then l2 else not l2`
                let not_l2 = ECore::If(
                    Box::new(ECore::Var(l2)),
                    CBranch::new(ECore::Bool(false)),
                    CBranch::new(ECore::Bool(true)),
                );
                Ok(ECore::If(
                    Box::new(ECore::Var(l1)),
                    CBranch::new(ECore::Var(l2)),
                    CBranch::new(not_l2),
                ))
            }
            Ty::Nat | Ty::Int | Ty::Prim(_) => {
                Ok(literal_eq_cond(&zc, ECore::Var(l1), ECore::Var(l2)))
            }
            Ty::Tuple(items) => {
                let n = items.len();
                let f1: Vec<LocalId> = (0..n).map(|_| self.fresh_local()).collect();
                let f2: Vec<LocalId> = (0..n).map(|_| self.fresh_local()).collect();
                for ((a, b), ty) in f1.iter().zip(f2.iter()).zip(items.iter()) {
                    self.note_local_ty(*a, ty);
                    self.note_local_ty(*b, ty);
                }
                let mut acc = ECore::Bool(true);
                for i in (0..n).rev() {
                    let eq_i = self.emit_sing_eq_bool(f1[i], f2[i], &items[i], env, span)?;
                    acc = if matches!(acc, ECore::Bool(true)) {
                        eq_i
                    } else {
                        ECore::If(
                            Box::new(eq_i),
                            CBranch::new(acc),
                            CBranch::new(ECore::Bool(false)),
                        )
                    };
                }
                let inner = ECore::Case {
                    scrut: Box::new(ECore::Var(l2)),
                    alts: vec![CAlt::new(CPat::con(0, f2), acc)],
                };
                Ok(ECore::Case {
                    scrut: Box::new(ECore::Var(l1)),
                    alts: vec![CAlt::new(CPat::con(0, f1), inner)],
                })
            }
            Ty::Adt(name, args) => {
                // First try Cinch's own `Eq` instance if available (handles `String`,
                // `Option`, and user types with custom `Eq` cleanly!).
                if let Ok((eq_core, res_ty)) = self.try_emit_eq_class(l1, l2, &zc, env, span) {
                    if self.uni.zonk(&res_ty) == Ty::Bool {
                        return Ok(eq_core);
                    }
                }
                // Built-in recursive structural equality for `List a` (works for ANY
                // element type `a`, even without `Eq (List a)` or `Eq a` in scope!).
                if name.as_str() == crate::known::LIST && args.len() == 1 {
                    let elem_ty = self.uni.zonk(&args[0]);
                    let ea = self.fresh_local();
                    let eb = self.fresh_local();
                    self.note_local_ty(ea, &elem_ty);
                    self.note_local_ty(eb, &elem_ty);
                    let elem_eq_body = self.emit_sing_eq_bool(ea, eb, &elem_ty, env, span)?;
                    let elem_eq_lam = ECore::Lam {
                        params: vec![ea, eb],
                        body: Box::new(elem_eq_body),
                    };
                    let elem_eq_local = self.fresh_local();
                    let elem_eq_ty = Ty::fun(elem_ty.clone(), Ty::fun(elem_ty.clone(), Ty::Bool));
                    self.note_local_ty(elem_eq_local, &elem_eq_ty);

                    let mut sub_env = env.clone();
                    sub_env.locals.push(("$sing_elem_eq".into(), elem_eq_local, Scheme::mono(elem_eq_ty)));
                    sub_env.locals.push(("$sing_l1".into(), l1, Scheme::mono(zc.clone())));
                    sub_env.locals.push(("$sing_l2".into(), l2, Scheme::mono(zc.clone())));

                    let nm = |s: &str| ast::Name::new(s.to_string(), span);
                    let var = |s: &str| ast::Term::Var(ast::VarPath { quals: Vec::new(), name: nm(s) });
                    let con_pat = |c: &str, ps: Vec<ast::Pat>| {
                        ast::Pat::Con(ast::ConPath { segments: vec![nm(c)] }, ps, span)
                    };
                    let alt = |pat: ast::Pat, body: ast::Term| ast::Alt {
                        attrs: Vec::new(),
                        pat,
                        guard: None,
                        guard_attrs: Vec::new(),
                        body,
                        span,
                    };
                    let bool_con = |b: &str| ast::Term::Con(ast::ConPath { segments: vec![nm(b)] });
                    let nil_branch = ast::Term::Case(
                        Box::new(var("$ys")),
                        vec![
                            alt(con_pat("Nil", vec![]), bool_con("True")),
                            alt(con_pat("Cons", vec![ast::Pat::Wild(span), ast::Pat::Wild(span)]), bool_con("False")),
                        ],
                        span,
                    );
                    let cons_branch = ast::Term::Case(
                        Box::new(var("$ys")),
                        vec![
                            alt(con_pat("Nil", vec![]), bool_con("False")),
                            alt(
                                con_pat("Cons", vec![ast::Pat::Var(nm("$y"), span), ast::Pat::Var(nm("$yr"), span)]),
                                ast::Term::If {
                                    cond: Box::new(ast::Term::App(
                                        Box::new(var("$sing_elem_eq")),
                                        vec![var("$x"), var("$y")],
                                        span,
                                    )),
                                    then: Box::new(ast::Term::App(
                                        Box::new(var("$sing_leq")),
                                        vec![var("$xr"), var("$yr")],
                                        span,
                                    )),
                                    els: Box::new(bool_con("False")),
                                    cond_attrs: Vec::new(),
                                    then_attrs: Vec::new(),
                                    else_attrs: Vec::new(),
                                    span,
                                },
                            ),
                        ],
                        span,
                    );
                    let leq_body = ast::Term::Case(
                        Box::new(var("$xs")),
                        vec![
                            alt(con_pat("Nil", vec![]), nil_branch),
                            alt(
                                con_pat("Cons", vec![ast::Pat::Var(nm("$x"), span), ast::Pat::Var(nm("$xr"), span)]),
                                cons_branch,
                            ),
                        ],
                        span,
                    );
                    let letrec_term = ast::Term::Letrec(
                        vec![ast::LocalDecl::Eq {
                            name: nm("$sing_leq"),
                            pats: vec![ast::Pat::Var(nm("$xs"), span), ast::Pat::Var(nm("$ys"), span)],
                            body: leq_body,
                            span,
                        }],
                        Box::new(ast::Term::App(
                            Box::new(var("$sing_leq")),
                            vec![var("$sing_l1"), var("$sing_l2")],
                            span,
                        )),
                        span,
                    );
                    let checked = self.check(&mut sub_env, &letrec_term, &Ty::Bool)?;
                    return Ok(ECore::Let(elem_eq_local, Box::new(elem_eq_lam), Box::new(checked)));
                }
                // Structural fallback for non-recursive user `data` types (like
                // `data Layout where RowMajor; ColMajor` or `Axis`) without an `Eq` instance!
                let info = self.lookup_known_type(name)?.clone();
                if info.ctors.is_empty() {
                    return Err(Error::at(
                        span,
                        crate::diagnostic!(
                            "cannot compare singletons of type {}: no `Eq` instance and no data constructors",
                            zc
                        ),
                    ));
                }
                let mut outer_alts = Vec::with_capacity(info.ctors.len());
                for (cname, tag) in &info.ctors {
                    let v = self.ctor_of_type(&info, cname)?;
                    let parts = self.ctor_parts(&v, args)?;
                    let rfields: Vec<Ty> = parts
                        .fields
                        .iter()
                        .filter(|(_, _, erased)| !*erased)
                        .map(|(_, t, _)| t.clone())
                        .collect();
                    let f1: Vec<LocalId> = (0..rfields.len()).map(|_| self.fresh_local()).collect();
                    let f2: Vec<LocalId> = (0..rfields.len()).map(|_| self.fresh_local()).collect();
                    for ((a, b), ty) in f1.iter().zip(f2.iter()).zip(rfields.iter()) {
                        self.note_local_ty(*a, ty);
                        self.note_local_ty(*b, ty);
                    }
                    let mut eq_fields = ECore::Bool(true);
                    for i in (0..rfields.len()).rev() {
                        if rfields[i] == zc {
                            return Err(Error::at(
                                span,
                                crate::diagnostic!(
                                    "cannot compare recursive singletons of type {} without an `Eq` instance in scope",
                                    zc
                                ),
                            ));
                        }
                        let eq_i = self.emit_sing_eq_bool(f1[i], f2[i], &rfields[i], env, span)?;
                        eq_fields = if matches!(eq_fields, ECore::Bool(true)) {
                            eq_i
                        } else {
                            ECore::If(
                                Box::new(eq_i),
                                CBranch::new(eq_fields),
                                CBranch::new(ECore::Bool(false)),
                            )
                        };
                    }
                    let mut inner_alts = Vec::with_capacity(info.ctors.len());
                    for (cname2, tag2) in &info.ctors {
                        if tag2 == tag {
                            inner_alts.push(CAlt::new(CPat::con(*tag2, f2.clone()), eq_fields.clone()));
                        } else {
                            let v2 = self.ctor_of_type(&info, cname2)?;
                            let parts2 = self.ctor_parts(&v2, args)?;
                            let ar2 = parts2.fields.iter().filter(|(_, _, e)| !*e).count();
                            let w2: Vec<LocalId> = (0..ar2).map(|_| self.fresh_local()).collect();
                            for (w, (_, t, _)) in w2.iter().zip(parts2.fields.iter().filter(|(_, _, e)| !*e)) {
                                self.note_local_ty(*w, t);
                            }
                            inner_alts.push(CAlt::new(CPat::con(*tag2, w2), ECore::Bool(false)));
                        }
                    }
                    let inner_case = ECore::Case {
                        scrut: Box::new(ECore::Var(l2)),
                        alts: inner_alts,
                    };
                    outer_alts.push(CAlt::new(CPat::con(*tag, f1), inner_case));
                }
                Ok(ECore::Case {
                    scrut: Box::new(ECore::Var(l1)),
                    alts: outer_alts,
                })
            }
            _ => Err(Error::at(
                span,
                crate::diagnostic!("cannot compare singletons of type {}", zc),
            )),
        }
    }

    fn try_emit_eq_class(
        &mut self,
        l1: LocalId,
        l2: LocalId,
        carrier: &Ty,
        env: &Env,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let trial = self.uni.snapshot();
        let wants_len = self.wants.len();
        let lits_len = self.literal_pends.len();
        let mut sub_env = env.clone();
        sub_env.locals.push(("$seq_a".into(), l1, Scheme::mono(carrier.clone())));
        sub_env.locals.push(("$seq_b".into(), l2, Scheme::mono(carrier.clone())));
        let syn = ast::Term::App(
            Box::new(ast::Term::Var(ast::VarPath {
                quals: Vec::new(),
                name: ast::Name::new("==".to_string(), span),
            })),
            vec![
                ast::Term::Var(ast::VarPath {
                    quals: Vec::new(),
                    name: ast::Name::new("$seq_a".to_string(), span),
                }),
                ast::Term::Var(ast::VarPath {
                    quals: Vec::new(),
                    name: ast::Name::new("$seq_b".to_string(), span),
                }),
            ],
            span,
        );
        match self.infer(&mut sub_env, &syn) {
            Ok(res) => {
                // Verify any newly emitted `Eq` wants can actually resolve right now
                // so a missing `Eq` instance cleanly triggers our structural fallback!
                let mut ok = true;
                for w in &self.wants[wants_len..] {
                    let wanted = self.uni.zonk_class(&w.class);
                    let mut resolver = DictionaryResolver {
                        checker: self,
                        uni: self.uni.clone(),
                        transfers: Vec::new(),
                    };
                    if resolver.solve(&wanted, &w.scope, 0, w.span).is_err() {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    self.uni.commit(trial);
                    Ok(res)
                } else {
                    self.uni.rollback_to(trial);
                    self.wants.truncate(wants_len);
                    self.literal_pends.truncate(lits_len);
                    Err(Error::at(span, "no Eq instance"))
                }
            }
            Err(e) => {
                self.uni.rollback_to(trial);
                self.wants.truncate(wants_len);
                self.literal_pends.truncate(lits_len);
                Err(e)
            }
        }
    }

    fn is_sing_builtin(&self, env: &Env, name: &ast::Name) -> bool {
        if name.local_binding().is_some()
            || name.resolved_binding().is_some()
            || name.declaration_binding().is_some()
        {
            return false;
        }
        env.lookup_name(name).is_none() && self.lookup_value(None, &name.text).is_err()
    }

    /// Check a term directly against an expected `Sing carrier val` type.
    /// Returns `Some(Ok(core))` or `Some(Err(e))` if handled, or `None` to fall
    /// back to normal bidirectional checking.
    fn try_check_against_sing(
        &mut self,
        env: &mut Env,
        term: &ast::Term,
        carrier: &Ty,
        val: &Ty,
        span: ast::Span,
    ) -> Option<Result<ECore>> {
        let zcarrier = self.uni.zonk(carrier);
        let zval = self.uni.zonk(val);
        match term {
            // `sing`: compile-time reflection into a runtime `Sing` value.
            ast::Term::Var(p)
                if p.quals.is_empty()
                    && p.name.text == "sing"
                    && self.is_sing_builtin(env, &p.name) =>
            {
                Some(self.synthesize_sing(env, &zcarrier, &zval, span))
            }
            // Literal numerals against `Sing Nat val` / `Sing Int val`.
            ast::Term::Int(d, lspan) => {
                let lit_val = Ty::Idx(Box::new(ITerm::Num(d.clone().into())));
                if let Err(m) = self.uni.unify(&lit_val, &zval) {
                    return Some(Err(Error::at(*lspan, m)));
                }
                Some(self.check_scoped_numeric_literal(env, term, &zcarrier, d, *lspan))
            }
            // String literal against `Sing String val`.
            ast::Term::Str(s, lspan) => {
                let lit_val = Ty::Adt(format!("\"{s}\"").into(), Vec::new());
                if let Err(m) = self.uni.unify(&lit_val, &zval) {
                    return Some(Err(Error::at(*lspan, m)));
                }
                Some(Ok(self.string_const(s)))
            }
            // Unit literal against `Sing Unit val`.
            ast::Term::Unit(lspan) => {
                if let Err(m) = self.uni.unify(&Ty::Unit, &zval) {
                    return Some(Err(Error::at(*lspan, m)));
                }
                Some(Ok(ECore::Unit))
            }
            // Nullary constructor / `True` / `False` against `Sing carrier val`.
            ast::Term::Con(cp) => {
                let parts = cp.parts();
                let promoted_val = if parts.as_slice() == ["True"] {
                    Ty::Adt(crate::promote::PROMOTED_TRUE.into(), Vec::new())
                } else if parts.as_slice() == ["False"] {
                    Ty::Adt(crate::promote::PROMOTED_FALSE.into(), Vec::new())
                } else if let Some(ci) = self.lookup_promoted_ctor_path(cp) {
                    if !ci.field_tys.is_empty() {
                        return None;
                    }
                    Ty::Adt(ci.identity.into(), Vec::new())
                } else {
                    return None;
                };
                if let Err(m) = self.uni.unify(&promoted_val, &zval) {
                    return Some(Err(Error::at(span, m)));
                }
                Some(self.check(env, term, &zcarrier))
            }
            // List literal `[a, b, ...]` against `Sing (List elem) val`.
            ast::Term::List(items, lspan) => {
                let elem_carrier = match &zcarrier {
                    Ty::Adt(id, args) if id.as_str() == crate::known::LIST && args.len() == 1 => {
                        args[0].clone()
                    }
                    _ => return None,
                };
                let mut item_cores = Vec::with_capacity(items.len());
                let mut item_vals = Vec::with_capacity(items.len());
                for item in items {
                    let iv = self.uni.fresh();
                    let elem_sing = Ty::Adt("Sing".into(), vec![elem_carrier.clone(), iv.clone()]);
                    let ic = match self.check(env, item, &elem_sing) {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    item_cores.push(ic);
                    item_vals.push(self.uni.zonk(&iv));
                }
                let mut list_val = Ty::Adt(crate::types::PROMOTED_NIL.into(), Vec::new());
                for iv in item_vals.into_iter().rev() {
                    list_val = Ty::Adt(crate::types::PROMOTED_CONS.into(), vec![iv, list_val]);
                }
                if let Err(m) = self.uni.unify(&list_val, &zval) {
                    return Some(Err(Error::at(*lspan, m)));
                }
                let nil_gid = GlobalId(format!("{}.Nil", "Data.Core"));
                let cons_gid = GlobalId(format!("{}.Cons", "Data.Core"));
                let mut acc = ECore::Ctor(nil_gid, vec![elem_carrier.clone()]);
                for ic in item_cores.into_iter().rev() {
                    acc = app_spine(
                        ECore::Ctor(cons_gid.clone(), vec![elem_carrier.clone()]),
                        vec![ic, acc],
                    );
                }
                Some(Ok(acc))
            }
            // Tuple literal `(a, b)` against `Sing (A, B) val`.
            ast::Term::Tuple(items, tspan) => {
                let elem_carriers = match &zcarrier {
                    Ty::Tuple(cs) if cs.len() == items.len() => cs.clone(),
                    _ => return None,
                };
                let mut item_cores = Vec::with_capacity(items.len());
                let mut item_vals = Vec::with_capacity(items.len());
                for (item, ec) in items.iter().zip(elem_carriers.iter()) {
                    let iv = self.uni.fresh();
                    let elem_sing = Ty::Adt("Sing".into(), vec![ec.clone(), iv.clone()]);
                    let ic = match self.check(env, item, &elem_sing) {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    item_cores.push(ic);
                    item_vals.push(self.uni.zonk(&iv));
                }
                let tup_val = Ty::Tuple(item_vals);
                if let Err(m) = self.uni.unify(&tup_val, &zval) {
                    return Some(Err(Error::at(*tspan, m)));
                }
                let gid = match self.tuple_ctor(items.len(), *tspan) {
                    Ok(g) => g,
                    Err(e) => return Some(Err(e)),
                };
                Some(Ok(app_spine(ECore::Ctor(gid, elem_carriers), item_cores)))
            }
            // Constructor application `C a b` against `Sing carrier val`.
            ast::Term::App(head, args, aspan) if matches!(head.as_ref(), ast::Term::Con(_)) => {
                let ast::Term::Con(cp) = head.as_ref() else { unreachable!() };
                let ci = self.lookup_promoted_ctor_path(cp)?;
                if args.len() != ci.field_tys.len() {
                    return None;
                }
                Some((|| {
                    let val_info = self
                        .qualified
                        .get(&(ci.module.clone(), ci.name.clone()))
                        .cloned()
                        .or_else(|| self.lookup_value(None, &ci.name).ok())
                        .ok_or_else(|| Error::at(*aspan, format!("unknown constructor `{}`", ci.name)))?;
                    let ty_args = match &zcarrier {
                        Ty::Adt(_, targs) => targs.clone(),
                        _ => Vec::new(),
                    };
                    let parts = self.ctor_parts(&val_info, &ty_args)?;
                    let runtime_fields: Vec<Ty> = parts
                        .fields
                        .iter()
                        .filter(|(_, _, erased)| !*erased)
                        .map(|(_, t, _)| t.clone())
                        .collect();
                    if runtime_fields.len() != args.len() {
                        return Err(Error::at(*aspan, "constructor arity mismatch"));
                    }
                    let mut arg_cores = Vec::with_capacity(args.len());
                    let mut arg_vals = Vec::with_capacity(args.len());
                    for (arg, ft) in args.iter().zip(runtime_fields.iter()) {
                        let iv = self.uni.fresh();
                        let field_sing = Ty::Adt("Sing".into(), vec![ft.clone(), iv.clone()]);
                        let ac = self.check(env, arg, &field_sing)?;
                        arg_cores.push(ac);
                        arg_vals.push(self.uni.zonk(&iv));
                    }
                    let ctor_val = Ty::Adt(ci.identity.clone().into(), arg_vals);
                    self.uni
                        .unify(&ctor_val, &zval)
                        .map_err(|m| Error::at(*aspan, m))?;
                    let gid = GlobalId(format!("{}.{}", val_info.module, val_info.name));
                    let inst: Vec<Ty> = if let ValueKind::Ctor { ty_params, .. } = &val_info.kind {
                        ty_args.into_iter().take(ty_params.len()).collect()
                    } else {
                        Vec::new()
                    };
                    Ok(app_spine(ECore::Ctor(gid, inst), arg_cores))
                })())
            }
            _ => None,
        }
    }

    /// Recursive GADT equality refinement for `Refl :: Same a a` matched
    /// against `Same lhs rhs` (works for types, promoted data, and indices).
    fn match_refl_eq(
        &mut self,
        lhs: &Ty,
        rhs: &Ty,
        ty_refinements: &mut HashMap<String, Ty>,
        idx_refinements: &mut HashMap<String, ITerm>,
    ) -> std::result::Result<(), CtorMatch> {
        let a = self.uni.zonk(lhs);
        let b = self.uni.zonk(rhs);
        if a == b {
            return Ok(());
        }
        match (&a, &b) {
            (Ty::Meta(_), _) | (_, Ty::Meta(_)) => {
                self.uni.unify(&a, &b).map_err(|_| CtorMatch::Refuted)
            }
            (Ty::Param(p), _) if !ty_mentions(&b, p.as_str()) => {
                ty_refinements.insert(p.to_string(), b);
                Ok(())
            }
            (_, Ty::Param(p)) if !ty_mentions(&a, p.as_str()) => {
                ty_refinements.insert(p.to_string(), a);
                Ok(())
            }
            (Ty::Idx(ia), Ty::Idx(ib)) => {
                let na = self.uni.resolve_idx(ia).normalize();
                let nb = self.uni.resolve_idx(ib).normalize();
                if na == nb {
                    return Ok(());
                }
                match (&na, &nb) {
                    (ITerm::Var(v), _) if !nb.mentions(v.as_str()) => {
                        idx_refinements.insert(v.to_string(), nb);
                        Ok(())
                    }
                    (_, ITerm::Var(v)) if !na.mentions(v.as_str()) => {
                        idx_refinements.insert(v.to_string(), na);
                        Ok(())
                    }
                    (ITerm::Num(x), ITerm::Num(y)) if x != y => Err(CtorMatch::Refuted),
                    _ => self.uni.unify(&a, &b).map_err(|_| CtorMatch::Refuted),
                }
            }
            (Ty::Idx(ia), other) => {
                let na = self.uni.resolve_idx(ia).normalize();
                if let ITerm::Var(v) = na {
                    if !ty_mentions(other, v.as_str()) {
                        ty_refinements.insert(v.to_string(), other.clone());
                        return Ok(());
                    }
                }
                Err(CtorMatch::Refuted)
            }
            (other, Ty::Idx(ib)) => {
                let nb = self.uni.resolve_idx(ib).normalize();
                if let ITerm::Var(v) = nb {
                    if !ty_mentions(other, v.as_str()) {
                        ty_refinements.insert(v.to_string(), other.clone());
                        return Ok(());
                    }
                }
                Err(CtorMatch::Refuted)
            }
            (Ty::Adt(n1, a1), Ty::Adt(n2, a2)) if n1 == n2 && a1.len() == a2.len() => {
                for (x, y) in a1.iter().zip(a2.iter()) {
                    self.match_refl_eq(x, y, ty_refinements, idx_refinements)?;
                }
                Ok(())
            }
            (Ty::Tuple(a1), Ty::Tuple(a2)) if a1.len() == a2.len() => {
                for (x, y) in a1.iter().zip(a2.iter()) {
                    self.match_refl_eq(x, y, ty_refinements, idx_refinements)?;
                }
                Ok(())
            }
            _ => {
                if ty_heads_conflict(&a, &b, &self.uni) {
                    Err(CtorMatch::Refuted)
                } else {
                    Err(CtorMatch::Unknown(crate::diagnostic!("{} vs {}", a, b)))
                }
            }
        }
    }

    /// Check/infer built-in zero-cost singleton operations:
    /// - `from-sing s` -> `(s_core, carrier)` (zero instructions!)
    /// - `with-sing v (\s -> body)` -> `let s = v in body` (zero copy, zero closure!)
    /// - `sing-eq s1 s2` -> `Option (Same v1 v2)`
    fn try_sing_builtin_app(
        &mut self,
        env: &mut Env,
        f: &ast::Term,
        args: &[ast::Term],
        expected: Option<&Ty>,
        span: ast::Span,
    ) -> Option<Result<(ECore, Ty)>> {
        let ast::Term::Var(p) = f else { return None };
        if !p.quals.is_empty() || !self.is_sing_builtin(env, &p.name) {
            return None;
        }
        match p.name.text.as_str() {
            "from-sing" => {
                if args.len() != 1 {
                    return Some(Err(Error::at(
                        span,
                        format!("`from-sing` expects 1 argument, got {}", args.len()),
                    )));
                }
                Some((|| {
                    let (s_core, s_ty) = self.infer(env, &args[0])?;
                    let zs = self.uni.zonk(&s_ty);
                    match zs {
                        Ty::Adt(name, sargs) if name == "Sing" && sargs.len() == 2 => {
                            Ok((s_core, sargs[0].clone()))
                        }
                        other => Err(Error::at(
                            args[0].span(),
                            crate::diagnostic!(
                                "`from-sing` expects a `Sing` value, got {}",
                                other
                            ),
                        )),
                    }
                })())
            }
            "with-sing" => {
                if args.len() != 2 {
                    return Some(Err(Error::at(
                        span,
                        format!("`with-sing` expects 2 arguments (`with-sing val (\\s -> body)`), got {}", args.len()),
                    )));
                }
                Some((|| {
                    let (val_core, val_ty) = self.infer(env, &args[0])?;
                    let zval_ty = self.uni.zonk(&val_ty);
                    if Self::ty_contains_erased_universe(&zval_ty) {
                        return Err(Error::at(
                            args[0].span(),
                            crate::diagnostic!(
                                "cannot reify erased type {} with `with-sing`",
                                zval_ty
                            ),
                        ));
                    }
                    let sk = self.next_sing_skolem("sing");
                    self.sing_skolem_carriers.insert(sk.clone(), zval_ty.clone());
                    let sk_val = if Self::is_scalar_idx_ty(&zval_ty) && zval_ty != Ty::Bool {
                        Ty::Idx(Box::new(ITerm::Var(sk.clone().into())))
                    } else {
                        Ty::Param(sk.clone().into())
                    };
                    let sing_ty = Ty::Adt("Sing".into(), vec![zval_ty.clone(), sk_val]);

                    // Zero-closure fast path: `with-sing v (\s -> body)` compiles directly to `let s = v in body`!
                    if let ast::Term::Lam(pats, body, _) = &args[1] {
                        if pats.len() == 1 {
                            match &pats[0] {
                                ast::Pat::Var(_, _) | ast::Pat::Wild(_) => {
                                    let s_local = self.fresh_local();
                                    self.note_local_ty(s_local, &sing_ty);
                                    let mut body_env = env.clone();
                                    if let ast::Pat::Var(n, _) = &pats[0] {
                                        self.bind_syntax_local(
                                            &mut body_env,
                                            n,
                                            s_local,
                                            Scheme::mono(sing_ty.clone()),
                                            None,
                                            n.span,
                                        )?;
                                    }
                                    let (body_core, body_ty) = if let Some(exp) = expected {
                                        let c = self.check(&mut body_env, body, exp)?;
                                        (c, self.uni.zonk(exp))
                                    } else {
                                        let (c, t) = self.infer(&mut body_env, body)?;
                                        (c, self.uni.zonk(&t))
                                    };
                                    if ty_mentions(&body_ty, &sk) {
                                        return Err(Error::at(
                                            span,
                                            crate::diagnostic!(
                                                "existential singleton variable from `with-sing` escapes its scope in return type {}",
                                                body_ty
                                            ),
                                        ));
                                    }
                                    return Ok((
                                        ECore::Let(s_local, Box::new(val_core), Box::new(body_core)),
                                        body_ty,
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }

                    // General path: continuation expression `k :: Sing sk -> r`.
                    let ret_ty = expected.cloned().unwrap_or_else(|| self.uni.fresh());
                    let k_ty = Ty::fun(sing_ty, ret_ty.clone());
                    let k_core = self.check(env, &args[1], &k_ty)?;
                    let zret = self.uni.zonk(&ret_ty);
                    if ty_mentions(&zret, &sk) {
                        return Err(Error::at(
                            span,
                            crate::diagnostic!(
                                "existential singleton variable from `with-sing` escapes its scope in return type {}",
                                zret
                            ),
                        ));
                    }
                    Ok((app_spine(k_core, vec![val_core]), zret))
                })())
            }
            "sing-eq" => {
                if args.len() != 2 {
                    return Some(Err(Error::at(
                        span,
                        format!("`sing-eq` expects 2 arguments, got {}", args.len()),
                    )));
                }
                Some((|| {
                    let (c1, t1) = self.infer(env, &args[0])?;
                    let (c2, t2) = self.infer(env, &args[1])?;
                    let (carrier1, v1) = match self.uni.zonk(&t1) {
                        Ty::Adt(n, a) if n == "Sing" && a.len() == 2 => (a[0].clone(), a[1].clone()),
                        other => {
                            return Err(Error::at(
                                args[0].span(),
                                crate::diagnostic!("`sing-eq` expects a `Sing` value, got {}", other),
                            ));
                        }
                    };
                    let (carrier2, v2) = match self.uni.zonk(&t2) {
                        Ty::Adt(n, a) if n == "Sing" && a.len() == 2 => (a[0].clone(), a[1].clone()),
                        other => {
                            return Err(Error::at(
                                args[1].span(),
                                crate::diagnostic!("`sing-eq` expects a `Sing` value, got {}", other),
                            ));
                        }
                    };
                    self.uni
                        .unify(&carrier1, &carrier2)
                        .map_err(|m| Error::at(span, m))?;
                    let carrier = self.uni.zonk(&carrier1);
                    let l1 = self.fresh_local();
                    let l2 = self.fresh_local();
                    self.note_local_ty(l1, &carrier);
                    self.note_local_ty(l2, &carrier);
                    let eq_core = self.emit_sing_eq_bool(l1, l2, &carrier, env, span)?;

                    let same_ty = Ty::Adt("Same".into(), vec![self.uni.zonk(&v1), self.uni.zonk(&v2)]);
                    let opt_ty = Ty::Adt(crate::known::OPTION.into(), vec![same_ty.clone()]);
                    let refl_core = ECore::Ctor(GlobalId("<builtin>.Refl".into()), vec![self.uni.zonk(&v1)]);
                    let some_core = app_spine(
                        ECore::Ctor(GlobalId(format!("{}.Some", "Data.Core")), vec![same_ty.clone()]),
                        vec![refl_core],
                    );
                    let none_core = ECore::Ctor(
                        GlobalId(format!("{}.None", "Data.Core")),
                        vec![same_ty],
                    );
                    let branch = ECore::If(
                        Box::new(eq_core),
                        CBranch::new(some_core),
                        CBranch::new(none_core),
                    );
                    let core = ECore::Let(
                        l1,
                        Box::new(c1),
                        Box::new(ECore::Let(l2, Box::new(c2), Box::new(branch))),
                    );
                    Ok((core, opt_ty))
                })())
            }
            _ => None,
        }
    }

    /// Cheap syntactic peek to decide if any call argument is clearly a `Sing`
    /// value, so normal non-singleton calls pay zero overhead.
    fn term_looks_like_sing(&self, env: &Env, term: &ast::Term) -> bool {
        match term {
            ast::Term::Var(vp) => {
                if vp.quals.is_empty() {
                    if let Some((_, scheme)) = env.lookup(&vp.name.text) {
                        return matches!(self.uni.zonk(&scheme.ty), Ty::Adt(n, a) if n == "Sing" && a.len() == 2);
                    }
                    if vp.name.text == "sing" {
                        return true;
                    }
                }
                false
            }
            ast::Term::Ann(_, ast::Type::App(h, _, _), _) => {
                matches!(h.as_ref(), ast::Type::Con(cp, _) if cp.parts().as_slice() == ["Sing"])
            }
            ast::Term::App(head, args, _) => {
                if let ast::Term::Var(vp) = head.as_ref() {
                    if vp.quals.is_empty()
                        && matches!(vp.name.text.as_str(), "from-sing" | "with-sing" | "sing-eq")
                    {
                        return false;
                    }
                    // If the callee is a known function whose return type is explicitly
                    // `Sing`, the call produces a `Sing`. If its return type is NOT `Sing`
                    // and it takes `Sing` parameters (e.g. `sing-len :: Sing xs -> Nat`),
                    // it does NOT produce a `Sing`!
                    let qual_str = if vp.quals.is_empty() {
                        None
                    } else {
                        Some(
                            vp.quals
                                .iter()
                                .map(|q| q.text.as_str())
                                .collect::<Vec<_>>()
                                .join("."),
                        )
                    };
                    if let Ok(v) = self.lookup_value(qual_str.as_deref(), &vp.name.text) {
                        let mut cur = &v.scheme.ty;
                        let mut takes_sing = false;
                        while let Ty::Fun(dom, _, _, cod) | Ty::Pi { dom, cod, .. } = cur {
                            if matches!(dom.as_ref(), Ty::Adt(n, _) if n == "Sing") {
                                takes_sing = true;
                            }
                            cur = cod;
                        }
                        if matches!(cur, Ty::Adt(n, _) if n == "Sing") {
                            return true;
                        }
                        if takes_sing {
                            return false;
                        }
                        let id = format!("{}.{}", v.module, v.name);
                        let is_promoted_or_op = self.uni.promoted_fn(&id).is_some()
                            || (vp.quals.is_empty()
                                && (vp.name.text == "not"
                                    || Self::promoted_binop_of(&vp.name.text).is_some()));
                        if !is_promoted_or_op {
                            return false;
                        }
                    } else if vp.quals.is_empty()
                        && vp.name.text != "not"
                        && Self::promoted_binop_of(&vp.name.text).is_none()
                    {
                        return false;
                    }
                }
                args.iter().any(|a| self.term_looks_like_sing(env, a))
            }
            _ => false,
        }
    }

    /// Infer or check one argument of a singleton-lifted call as `Sing carrier val`.
    fn check_sing_lifted_arg(
        &mut self,
        env: &mut Env,
        arg: &ast::Term,
        carrier_hint: Option<&Ty>,
    ) -> Result<(ECore, Ty, Ty)> {
        if self.term_looks_like_sing(env, arg) {
            let (c, t) = self.infer(env, arg)?;
            if let Ty::Adt(n, a) = self.uni.zonk(&t) {
                if n == "Sing" && a.len() == 2 {
                    if let Some(hint) = carrier_hint {
                        self.uni.unify(&a[0], hint).map_err(|m| Error::at(arg.span(), m))?;
                    }
                    return Ok((c, self.uni.zonk(&a[0]), self.uni.zonk(&a[1])));
                }
            }
        }
        // Allow literals (`0`, `True`, `"hi"`, `[1, 2]`, `RowMajor`, `sing`) directly
        // as arguments to singleton-lifted calls (e.g. `s + 1`, `append-shape s [4]`)!
        let carrier = carrier_hint.cloned().unwrap_or_else(|| self.uni.fresh());
        let val_meta = self.uni.fresh();
        let sing_exp = Ty::Adt("Sing".into(), vec![carrier.clone(), val_meta.clone()]);
        if let Some(res) = self.try_check_against_sing(env, arg, &carrier, &val_meta, arg.span()) {
            let c = res?;
            return Ok((c, self.uni.zonk(&carrier), self.uni.zonk(&val_meta)));
        }
        let (c, t) = self.infer(env, arg)?;
        self.uni.unify(&t, &sing_exp).map_err(|m| Error::at(arg.span(), m))?;
        Ok((c, self.uni.zonk(&carrier), self.uni.zonk(&val_meta)))
    }

    /// Direct zero-cost singleton lifting for:
    /// 1. Any promoted pure function `f s1 s2 ...` -> calls compiled `f` directly,
    ///    returning `Sing R (f v1 v2 ...)`.
    /// 2. Any promoted binary/unary operator (`+`, `-`, `*`, `==`, `<`, `&&`, `not`)
    ///    on `Sing` arguments.
    /// 3. Any data constructor `C s1 s2 ...` applied to `Sing` arguments.
    fn try_infer_sing_lifted_app(
        &mut self,
        env: &mut Env,
        f: &ast::Term,
        args: &[ast::Term],
        span: ast::Span,
    ) -> Option<Result<(ECore, Ty)>> {
        if args.is_empty() || !args.iter().any(|a| self.term_looks_like_sing(env, a)) {
            return None;
        }
        match f {
            // Data constructor applied to `Sing` arguments: `Cons sh st :: Sing (Cons h t)`.
            ast::Term::Con(cp) => {
                let ci = self.lookup_promoted_ctor_path(cp)?;
                if args.len() != ci.field_tys.len() {
                    return None;
                }
                Some((|| {
                    let val_info = self
                        .qualified
                        .get(&(ci.module.clone(), ci.name.clone()))
                        .cloned()
                        .or_else(|| self.lookup_value(None, &ci.name).ok())
                        .ok_or_else(|| Error::at(span, format!("unknown constructor `{}`", ci.name)))?;
                    let mut param_map = HashMap::new();
                    let mut fresh_params = Vec::new();
                    if let ValueKind::Ctor { ty_params, .. } = &val_info.kind {
                        for tp in ty_params {
                            let m = self.uni.fresh();
                            param_map.insert(tp.clone(), m.clone());
                            fresh_params.push(m);
                        }
                    }
                    let mut arg_cores = Vec::with_capacity(args.len());
                    let mut arg_vals = Vec::with_capacity(args.len());
                    for (i, arg) in args.iter().enumerate() {
                        let fty = Self::subst_ty(&ci.field_tys[i], &param_map);
                        let (ac, _, av) = self.check_sing_lifted_arg(env, arg, Some(&fty))?;
                        arg_cores.push(ac);
                        arg_vals.push(av);
                    }
                    let mut parent_ty = &val_info.scheme.ty;
                    while let Ty::Fun(_, _, _, cod) | Ty::Pi { cod, .. } = parent_ty {
                        parent_ty = cod;
                    }
                    let res_carrier = self.uni.zonk(&Self::subst_ty(parent_ty, &param_map));
                    let inst: Vec<Ty> = fresh_params.into_iter().map(|t| self.uni.zonk(&t)).collect();
                    let gid = GlobalId(format!("{}.{}", val_info.module, val_info.name));
                    let core = app_spine(ECore::Ctor(gid, inst), arg_cores);
                    let res_val = Ty::Adt(ci.identity.clone().into(), arg_vals);
                    Ok((core, Ty::Adt("Sing".into(), vec![res_carrier, res_val])))
                })())
            }
            ast::Term::Var(vp) => {
                if vp.quals.is_empty() && env.lookup(&vp.name.text).is_some() {
                    return None;
                }
                // Built-in operators (`+`, `-`, `*`, `==`, `/=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `not`)
                if vp.quals.is_empty() {
                    if vp.name.text == "not" && args.len() == 1 {
                        return Some((|| {
                            let (ac, _, av) = self.check_sing_lifted_arg(env, &args[0], Some(&Ty::Bool))?;
                            let core = ECore::If(
                                Box::new(ac),
                                CBranch::new(ECore::Bool(false)),
                                CBranch::new(ECore::Bool(true)),
                            );
                            let res_val = self.uni.zonk(&Ty::Adt(
                                format!("{}Data.Bool.not", crate::promote::PROMOTED_FN_PREFIX).into(),
                                vec![av],
                            ));
                            Ok((core, Ty::Adt("Sing".into(), vec![Ty::Bool, res_val])))
                        })());
                    }
                    if let Some(op) = Self::promoted_binop_of(&vp.name.text) {
                        if args.len() == 2 {
                            return Some((|| {
                                use crate::promote::PBinOp;
                                let (hint_l, hint_r) = match op {
                                    PBinOp::And | PBinOp::Or => (Some(Ty::Bool), Some(Ty::Bool)),
                                    PBinOp::StrConcat => {
                                        let s = Ty::Adt(crate::known::STRING.into(), Vec::new());
                                        (Some(s.clone()), Some(s))
                                    }
                                    _ => (None, None),
                                };
                                let (lc, lt, lv) = self.check_sing_lifted_arg(env, &args[0], hint_l.as_ref())?;
                                let (rc, rt, rv) = self.check_sing_lifted_arg(env, &args[1], hint_r.as_ref().or(Some(&lt)))?;
                                let (op_core, res_carrier) =
                                    self.emit_sing_binop_runtime(&vp.name.text, lc, &lt, rc, &rt, env, span)?;
                                let res_val = match op {
                                    PBinOp::Add | PBinOp::Sub | PBinOp::Mul => {
                                        let iop = match op {
                                            PBinOp::Add => IdxOp::Add,
                                            PBinOp::Sub => IdxOp::Sub,
                                            PBinOp::Mul => IdxOp::Mul,
                                            _ => unreachable!(),
                                        };
                                        let la = crate::promote::PVal::from_ty(&lv).to_iterm().ok_or_else(|| {
                                            Error::at(span, "left operand of lifted arithmetic is not an index value")
                                        })?;
                                        let ra = crate::promote::PVal::from_ty(&rv).to_iterm().ok_or_else(|| {
                                            Error::at(span, "right operand of lifted arithmetic is not an index value")
                                        })?;
                                        Ty::Idx(Box::new(ITerm::Bin(iop, Box::new(la), Box::new(ra)).normalize()))
                                    }
                                    _ => {
                                        // Evaluate concrete comparisons/booleans via native evaluator!
                                        let lp = crate::promote::PVal::from_ty(&lv);
                                        let rp = crate::promote::PVal::from_ty(&rv);
                                        let mut dummy = crate::promote::PromotedEnv::with_builtins();
                                        let _ = &mut dummy;
                                        let evaled = self.uni.promoted_env().eval_pvals_to_ty(
                                            &format!("__binop_{:?}", op),
                                            vec![lp.clone(), rp.clone()],
                                        );
                                        if let Some(ev) = evaled {
                                            ev
                                        } else {
                                            // Direct constant fold on concrete values via PExpr.
                                            let tmp_id = format!("__sing_op_{:?}", op);
                                            let mut env_tmp = self.uni.promoted_env().clone();
                                            env_tmp.register(crate::promote::PromotedFn {
                                                identity: tmp_id.clone(),
                                                type_params: Vec::new(),
                                                param_tys: vec![lt.clone(), rt.clone()],
                                                ret_ty: res_carrier.clone(),
                                                returns_idx: Self::is_scalar_idx_ty(&res_carrier) && res_carrier != Ty::Bool,
                                                body: crate::promote::PromotedBody::Native(
                                                    crate::promote::NativeBuiltin::BinOp(op),
                                                ),
                                            });
                                            env_tmp.eval_to_ty(&tmp_id, &[lv.clone(), rv.clone()]).unwrap_or_else(|| {
                                                Ty::Adt(
                                                    format!("{}{}", crate::promote::PROMOTED_FN_PREFIX, vp.name.text).into(),
                                                    vec![lv, rv],
                                                )
                                            })
                                        }
                                    }
                                };
                                Ok((op_core, Ty::Adt("Sing".into(), vec![res_carrier, self.uni.zonk(&res_val)])))
                            })());
                        }
                    }
                }
                // Promoted pure function applied to `Sing` arguments!
                let qual_str = if vp.quals.is_empty() {
                    None
                } else {
                    Some(
                        vp.quals
                            .iter()
                            .map(|q| q.text.as_str())
                            .collect::<Vec<_>>()
                            .join("."),
                    )
                };
                let pfn = self.lookup_promoted_fn(qual_str.as_deref(), &vp.name.text, span).ok().flatten()?;
                if args.len() != pfn.arity() {
                    return None;
                }
                // Do not hijack a function that itself takes `Sing` parameters!
                if pfn.param_tys.iter().any(|pt| matches!(pt, Ty::Adt(n, _) if n == "Sing")) {
                    return None;
                }
                Some((|| {
                    let mut tp_env = HashMap::new();
                    for tp in &pfn.type_params {
                        tp_env.insert(tp.clone(), self.uni.fresh());
                    }
                    let mut arg_cores = Vec::with_capacity(args.len());
                    let mut arg_carriers = Vec::with_capacity(args.len());
                    let mut arg_vals = Vec::with_capacity(args.len());
                    for (i, arg) in args.iter().enumerate() {
                        let expected_c = Self::subst_ty(&pfn.param_tys[i], &tp_env);
                        let (ac, actual_c, av) = self.check_sing_lifted_arg(env, arg, Some(&expected_c))?;
                        arg_cores.push(ac);
                        arg_carriers.push(actual_c);
                        arg_vals.push(av);
                    }
                    let (call_core, res_carrier) =
                        self.emit_promoted_runtime_call(&pfn, &arg_carriers, arg_cores, env, span)?;
                    let fn_head = format!("{}{}", crate::promote::PROMOTED_FN_PREFIX, pfn.identity);
                    let raw_val = if pfn.returns_idx {
                        Ty::Idx(Box::new(ITerm::TyFamily(fn_head.into(), arg_vals)))
                    } else {
                        Ty::Adt(fn_head.into(), arg_vals)
                    };
                    let res_val = self.uni.zonk(&raw_val);
                    Ok((call_core, Ty::Adt("Sing".into(), vec![res_carrier, res_val])))
                })())
            }
            _ => None,
        }
    }

    /// Direct zero-cost pattern matching on `s :: Sing carrier val`.
    /// Uses ordinary constructors, booleans, tuples, and literals; refines
    /// `val` in every branch; statically refutes impossible arms; and emits
    /// the exact same runtime `Case`/`If` on `carrier` with zero wrapper unboxing!
    fn compile_sing_rows(
        &mut self,
        rows: Vec<Row>,
        cols: Vec<ECore>,
        types: Vec<Ty>,
        expected: &Ty,
        ctx_span: ast::Span,
        uncovered: &str,
        carrier: Ty,
        val: Ty,
    ) -> Result<ECore> {
        let col0 = cols[0].clone();
        let ty0 = types[0].clone();
        let zcarrier = self.uni.zonk(&carrier);
        let zval = self.uni.zonk(&val);
        let observed = self.inline_cols.get(&match &col0 {
            ECore::Var(l) => *l,
            _ => LocalId(usize::MAX),
        }).cloned().unwrap_or_else(|| col0.clone());

        // Identify whether `zval` is a rigid variable/local value that branch matching refines.
        let is_refinable_val = matches!(
            &zval,
            Ty::Param(_) | Ty::Idx(_)
        ) && !matches!(&zval, Ty::Idx(it) if matches!(it.as_ref(), ITerm::Num(_)));

        match zcarrier {
            // 1. Direct pattern matching on `Sing Bool b` (`True` / `False`).
            Ty::Bool => {
                let known_bool = match &zval {
                    Ty::Adt(id, a) if id.as_str() == crate::promote::PROMOTED_TRUE && a.is_empty() => Some(true),
                    Ty::Adt(id, a) if id.as_str() == crate::promote::PROMOTED_FALSE && a.is_empty() => Some(false),
                    _ => None,
                };
                let mut true_rows = Vec::new();
                let mut false_rows = Vec::new();
                for r in rows {
                    let p0 = r.pats[0].clone();
                    let tag = match &p0 {
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => None,
                        ast::Pat::Con(p, args, span) => {
                            let val_b = match p.parts().as_slice() {
                                ["True"] => true,
                                ["False"] => false,
                                _ => return Err(Error::at(*span, "pattern does not match type `Sing Bool`")),
                            };
                            if !args.is_empty() {
                                return Err(Error::at(*span, "True and False take no arguments"));
                            }
                            if let Some(kb) = known_bool {
                                if kb != val_b {
                                    return Err(Error::at(
                                        *span,
                                        crate::diagnostic!(
                                            "impossible arm: `{}` cannot match `Sing {}`",
                                            if val_b { "True" } else { "False" },
                                            zval
                                        ),
                                    ));
                                }
                            }
                            Some(val_b)
                        }
                        other => return Err(Error::at(other.span(), "pattern does not match type `Sing Bool`")),
                    };
                    let mut a = r;
                    a.pats.remove(0);
                    match tag {
                        Some(true) => {
                            self.mark_consumed(&mut a, &col0);
                            true_rows.push(a);
                        }
                        Some(false) => {
                            self.mark_consumed(&mut a, &col0);
                            false_rows.push(a);
                        }
                        None => {
                            if let ast::Pat::Var(n, _) = &p0 {
                                if let ECore::Var(l) = &col0 {
                                    self.bind_syntax_local(&mut a.env, n, *l, Scheme::mono(ty0.clone()), None, n.span)?;
                                }
                            }
                            if known_bool != Some(false) {
                                true_rows.push(a.clone());
                            }
                            if known_bool != Some(true) {
                                false_rows.push(a);
                            }
                        }
                    }
                }
                let rest_cols = cols[1..].to_vec();
                let rest_tys = types[1..].to_vec();
                if known_bool == Some(true) {
                    if true_rows.is_empty() {
                        return Err(Error::at(ctx_span, "non-exhaustive patterns: `True` is not covered"));
                    }
                    return self.compile_rows(true_rows, rest_cols, rest_tys, expected, ctx_span, "True");
                }
                if known_bool == Some(false) {
                    if false_rows.is_empty() {
                        return Err(Error::at(ctx_span, "non-exhaustive patterns: `False` is not covered"));
                    }
                    return self.compile_rows(false_rows, rest_cols, rest_tys, expected, ctx_span, "False");
                }
                if true_rows.is_empty() || false_rows.is_empty() {
                    return Err(Error::at(ctx_span, "non-exhaustive patterns: True and False must both be covered"));
                }
                let mut t_exp = expected.clone();
                let mut t_tys = rest_tys.clone();
                if is_refinable_val {
                    self.apply_sing_branch_refinement(
                        &zval,
                        &Ty::Adt(crate::promote::PROMOTED_TRUE.into(), Vec::new()),
                        &mut t_exp,
                        &mut t_tys,
                        &mut true_rows,
                    );
                }
                let (t, tm) = self.isolate(|s| {
                    s.compile_rows(true_rows, rest_cols.clone(), t_tys, &t_exp, ctx_span, "True")
                })?;
                let mut f_exp = expected.clone();
                let mut f_tys = rest_tys;
                if is_refinable_val {
                    self.apply_sing_branch_refinement(
                        &zval,
                        &Ty::Adt(crate::promote::PROMOTED_FALSE.into(), Vec::new()),
                        &mut f_exp,
                        &mut f_tys,
                        &mut false_rows,
                    );
                }
                let (f, fm) = self.isolate(|s| {
                    s.compile_rows(false_rows, rest_cols, f_tys, &f_exp, ctx_span, "False")
                })?;
                self.merge_branches(vec![tm, fm]);
                Ok(ECore::If(Box::new(observed), CBranch::new(t), CBranch::new(f)))
            }

            // 2. Direct pattern matching on `Sing (A, B, ...) tup`.
            Ty::Tuple(items) => {
                let n = items.len();
                let mut sub_rows = Vec::new();
                let mut var_rows = Vec::new();
                for mut r in rows {
                    let p0 = r.pats[0].clone();
                    match &p0 {
                        ast::Pat::Var(name, _) => {
                            if let ECore::Var(l) = &col0 {
                                self.bind_syntax_local(&mut r.env, name, *l, Scheme::mono(ty0.clone()), None, name.span)?;
                            }
                            let mut a = r;
                            a.pats.remove(0);
                            var_rows.push(a);
                        }
                        ast::Pat::Wild(_) => {
                            let mut a = r;
                            a.pats.remove(0);
                            var_rows.push(a);
                        }
                        ast::Pat::Tuple(pats, span) => {
                            if pats.len() != n {
                                return Err(Error::at(
                                    *span,
                                    format!("tuple pattern has {} components, expected {n}", pats.len()),
                                ));
                            }
                            self.mark_consumed(&mut r, &col0);
                            let mut a = r.clone();
                            let mut ps = pats.clone();
                            ps.extend(r.pats[1..].iter().cloned());
                            a.pats = ps;
                            sub_rows.push(a);
                        }
                        other => return Err(Error::at(other.span(), crate::diagnostic!("pattern does not match {}", ty0))),
                    }
                }
                let fields: Vec<LocalId> = (0..n).map(|_| self.fresh_local()).collect();
                let mut branch = sub_rows;
                for r in &var_rows {
                    let mut a = r.clone();
                    let mut pats = vec![ast::Pat::Wild(ctx_span); n];
                    pats.extend(a.pats.iter().cloned());
                    a.pats = pats;
                    branch.push(a);
                }
                branch.sort_by_key(|r| r.idx);
                let mut field_vals = Vec::with_capacity(n);
                let mut branch_tys = Vec::with_capacity(n + types.len() - 1);
                let known_items = match &zval {
                    Ty::Tuple(vs) if vs.len() == n => Some(vs.clone()),
                    _ => None,
                };
                for (i, item_carrier) in items.iter().enumerate() {
                    let fv = if let Some(ref ks) = known_items {
                        ks[i].clone()
                    } else {
                        let sk = self.next_sing_skolem("sing_f");
                        self.sing_skolem_carriers.insert(sk.clone(), item_carrier.clone());
                        if Self::is_scalar_idx_ty(item_carrier) && *item_carrier != Ty::Bool {
                            Ty::Idx(Box::new(ITerm::Var(sk.into())))
                        } else {
                            Ty::Param(sk.into())
                        }
                    };
                    let sing_ft = Ty::Adt("Sing".into(), vec![item_carrier.clone(), fv.clone()]);
                    self.note_local_ty(fields[i], &sing_ft);
                    field_vals.push(fv);
                    branch_tys.push(sing_ft);
                }
                branch_tys.extend(types[1..].iter().cloned());
                let mut branch_cols: Vec<ECore> = fields.iter().map(|l| ECore::Var(*l)).collect();
                branch_cols.extend(cols[1..].iter().cloned());
                let mut branch_exp = expected.clone();
                if known_items.is_none() && is_refinable_val {
                    self.apply_sing_branch_refinement(
                        &zval,
                        &Ty::Tuple(field_vals),
                        &mut branch_exp,
                        &mut branch_tys,
                        &mut branch,
                    );
                }
                let core = self.compile_rows(branch, branch_cols, branch_tys, &branch_exp, ctx_span, uncovered)?;
                Ok(ECore::Case {
                    scrut: Box::new(observed),
                    alts: vec![CAlt::new(CPat::con(0, fields), core)],
                })
            }

            // 3. Direct pattern matching on `Sing (ADT args) val` (e.g. `Sing [10, 20]`,
            //    `Sing xs` where `xs :: List Nat`, `Sing l` where `l :: Layout`, etc.).
            Ty::Adt(adt_name, adt_args) => {
                let info = self.lookup_known_type(adt_name.as_str())?.clone();
                let mut ctor_rows: Vec<Vec<Row>> = (0..info.ctors.len()).map(|_| Vec::new()).collect();
                let mut var_rows: Vec<Row> = Vec::new();
                // Inspect concrete head if `zval` is already a promoted constructor!
                let known_ctor: Option<(String, Vec<Ty>)> = match &zval {
                    Ty::Adt(id, a) if id.starts_with("ctor:") => Some((id.to_string(), a.clone())),
                    _ => None,
                };
                for mut r in rows {
                    let p0 = r.pats[0].clone();
                    match &p0 {
                        ast::Pat::Var(n, _) => {
                            if let ECore::Var(l) = &col0 {
                                self.bind_syntax_local(&mut r.env, n, *l, Scheme::mono(ty0.clone()), None, n.span)?;
                            }
                            let mut a = r;
                            a.pats.remove(0);
                            var_rows.push(a);
                        }
                        ast::Pat::Wild(_) => {
                            let mut a = r;
                            a.pats.remove(0);
                            var_rows.push(a);
                        }
                        ast::Pat::Cons(head, tail, span) if adt_name.as_str() == crate::known::LIST => {
                            let cp = ast::ConPath { segments: vec![ast::Name::new("Cons".to_string(), *span)] };
                            let v = self.ctor_for_type(&info, &cp, *span)?;
                            let (tag, arity) = match &v.kind {
                                ValueKind::Ctor { tag, arity, .. } => (*tag, *arity),
                                _ => return Err(Error::at(*span, format!("{} is not a constructor", v.name))),
                            };
                            let ctor_id = format!("ctor:{}.{}", v.module, v.name);
                            if let Some((ref kid, _)) = known_ctor {
                                if kid != &ctor_id {
                                    return Err(Error::at(
                                        *span,
                                        crate::diagnostic!(
                                            "impossible arm: constructor {} cannot match {}",
                                            v.name,
                                            ty0
                                        ),
                                    ));
                                }
                            }
                            let cargs = vec![(**head).clone(), (**tail).clone()];
                            if cargs.len() != arity {
                                return Err(Error::at(*span, "constructor arity mismatch"));
                            }
                            let mut a = r.clone();
                            self.mark_consumed(&mut a, &col0);
                            let mut pats = cargs;
                            pats.extend(r.pats[1..].iter().cloned());
                            a.pats = pats;
                            ctor_rows[tag].push(a);
                        }
                        ast::Pat::List(lpats, span) if adt_name.as_str() == crate::known::LIST => {
                            // Desugar `[]` / `[p1, ...]` into `Nil` / `Cons` patterns!
                            let mut desugared = ast::Pat::Con(
                                ast::ConPath { segments: vec![ast::Name::new("Nil".to_string(), *span)] },
                                Vec::new(),
                                *span,
                            );
                            for lp in lpats.iter().rev() {
                                desugared = ast::Pat::Con(
                                    ast::ConPath { segments: vec![ast::Name::new("Cons".to_string(), *span)] },
                                    vec![lp.clone(), desugared],
                                    *span,
                                );
                            }
                            if let ast::Pat::Con(cp, cargs, cspan) = desugared {
                                let v = self.ctor_for_type(&info, &cp, cspan)?;
                                let (tag, arity) = match &v.kind {
                                    ValueKind::Ctor { tag, arity, .. } => (*tag, *arity),
                                    _ => return Err(Error::at(cspan, format!("{} is not a constructor", v.name))),
                                };
                                let ctor_id = format!("ctor:{}.{}", v.module, v.name);
                                if let Some((ref kid, _)) = known_ctor {
                                    if kid != &ctor_id {
                                        return Err(Error::at(
                                            cspan,
                                            crate::diagnostic!(
                                                "impossible arm: constructor {} cannot match {}",
                                                v.name,
                                                ty0
                                            ),
                                        ));
                                    }
                                }
                                if cargs.len() != arity {
                                    return Err(Error::at(cspan, "constructor arity mismatch"));
                                }
                                let mut a = r.clone();
                                self.mark_consumed(&mut a, &col0);
                                let mut pats = cargs;
                                pats.extend(r.pats[1..].iter().cloned());
                                a.pats = pats;
                                ctor_rows[tag].push(a);
                            }
                        }
                        ast::Pat::Con(p, args, span) => {
                            let v = self.ctor_for_type(&info, p, *span)?;
                            let (tag, arity) = match &v.kind {
                                ValueKind::Ctor { tag, arity, .. } => (*tag, *arity),
                                _ => return Err(Error::at(*span, format!("{} is not a constructor", v.name))),
                            };
                            if args.len() != arity {
                                return Err(Error::at(
                                    *span,
                                    format!(
                                        "constructor {} expects {} field patterns, got {}",
                                        p.parts().join("."),
                                        arity,
                                        args.len()
                                    ),
                                ));
                            }
                            let ctor_id = format!("ctor:{}.{}", v.module, v.name);
                            if let Some((ref kid, _)) = known_ctor {
                                if kid != &ctor_id {
                                    return Err(Error::at(
                                        *span,
                                        crate::diagnostic!(
                                            "impossible arm: constructor {} cannot match {}",
                                            v.name,
                                            ty0
                                        ),
                                    ));
                                }
                            }
                            let mut a = r.clone();
                            self.mark_consumed(&mut a, &col0);
                            let mut pats = args.clone();
                            pats.extend(r.pats[1..].iter().cloned());
                            a.pats = pats;
                            ctor_rows[tag].push(a);
                        }
                        other => return Err(Error::at(other.span(), crate::diagnostic!("pattern does not match {}", ty0))),
                    }
                }

                let mut alts = Vec::new();
                let mut alt_maps = Vec::new();
                for (cname, tag) in info.ctors.clone() {
                    let v = self.ctor_of_type(&info, &cname)?;
                    let ctor_id = format!("ctor:{}.{}", v.module, v.name);
                    if let Some((ref kid, _)) = known_ctor {
                        if kid != &ctor_id {
                            // Statically refuted arm — dead code, skip without exhaustiveness error!
                            continue;
                        }
                    }
                    let parts = self.ctor_parts(&v, &adt_args)?;
                    let runtime_fields: Vec<Ty> = parts
                        .fields
                        .iter()
                        .filter(|(_, _, erased)| !*erased)
                        .map(|(_, t, _)| t.clone())
                        .collect();
                    let arity = runtime_fields.len();
                    let fields: Vec<LocalId> = (0..arity).map(|_| self.fresh_local()).collect();
                    if let ECore::Var(l) = &col0 {
                        for f in &fields {
                            self.smaller.insert(*f, *l);
                        }
                    }
                    let mut branch = std::mem::take(&mut ctor_rows[tag]);
                    for r in &var_rows {
                        let mut a = r.clone();
                        let mut pats = vec![ast::Pat::Wild(ctx_span); arity];
                        pats.extend(a.pats.iter().cloned());
                        a.pats = pats;
                        branch.push(a);
                    }
                    branch.sort_by_key(|r| r.idx);
                    if branch.is_empty() {
                        return Err(Error::at(
                            ctx_span,
                            format!("non-exhaustive patterns: constructor {cname} is not covered"),
                        ));
                    }

                    let mut field_vals = Vec::with_capacity(arity);
                    let mut branch_tys = Vec::with_capacity(arity + types.len() - 1);
                    for (i, ft) in runtime_fields.iter().enumerate() {
                        let fv = if let Some((_, ref kargs)) = known_ctor {
                            kargs.get(i).cloned().unwrap_or_else(|| Ty::Param(self.next_sing_skolem("sing_f").into()))
                        } else {
                            let sk = self.next_sing_skolem("sing_f");
                            self.sing_skolem_carriers.insert(sk.clone(), ft.clone());
                            if Self::is_scalar_idx_ty(ft) && *ft != Ty::Bool {
                                Ty::Idx(Box::new(ITerm::Var(sk.into())))
                            } else {
                                Ty::Param(sk.into())
                            }
                        };
                        let sing_ft = Ty::Adt("Sing".into(), vec![ft.clone(), fv.clone()]);
                        self.note_local_ty(fields[i], &sing_ft);
                        field_vals.push(fv);
                        branch_tys.push(sing_ft);
                    }
                    branch_tys.extend(types[1..].iter().cloned());
                    let mut branch_cols: Vec<ECore> = fields.iter().map(|l| ECore::Var(*l)).collect();
                    branch_cols.extend(cols[1..].iter().cloned());

                    let mut branch_exp = expected.clone();
                    if known_ctor.is_none() && is_refinable_val {
                        self.apply_sing_branch_refinement(
                            &zval,
                            &Ty::Adt(ctor_id.into(), field_vals),
                            &mut branch_exp,
                            &mut branch_tys,
                            &mut branch,
                        );
                    }

                    let (core, m) = self.isolate(|s| {
                        s.compile_rows(
                            branch,
                            branch_cols,
                            branch_tys,
                            &branch_exp,
                            ctx_span,
                            &format!("constructor {cname}"),
                        )
                    })?;
                    alt_maps.push(m);
                    alts.push(CAlt::new(CPat::con(tag, fields), core));
                }
                self.merge_branches(alt_maps);
                Ok(ECore::Case {
                    scrut: Box::new(observed),
                    alts,
                })
            }

            // 4. Direct literal pattern matching on `Sing Nat n` / `Sing Int n`.
            Ty::Nat | Ty::Int => {
                let mut lit_values: Vec<String> = Vec::new();
                let mut lit_rows: Vec<Vec<Row>> = Vec::new();
                let mut default_rows: Vec<Row> = Vec::new();
                let known_lit: Option<String> = match &zval {
                    Ty::Idx(it) => match it.as_ref() {
                        ITerm::Num(d) => Some(d.to_string()),
                        _ => None,
                    },
                    _ => None,
                };
                for r in rows {
                    let p0 = r.pats[0].clone();
                    match &p0 {
                        ast::Pat::Int(digits, span) => {
                            if let Some(ref kl) = known_lit {
                                if kl != digits {
                                    return Err(Error::at(
                                        *span,
                                        format!("impossible arm: literal `{digits}` cannot match `Sing {kl}`"),
                                    ));
                                }
                            }
                            let mut a = r;
                            a.pats.remove(0);
                            if let Some(pos) = lit_values.iter().position(|v| v == digits) {
                                lit_rows[pos].push(a);
                            } else {
                                lit_values.push(digits.clone());
                                lit_rows.push(vec![a]);
                            }
                        }
                        ast::Pat::Var(n, _) => {
                            let mut a = r;
                            if let ECore::Var(l) = &col0 {
                                self.bind_syntax_local(&mut a.env, n, *l, Scheme::mono(ty0.clone()), None, n.span)?;
                            }
                            a.pats.remove(0);
                            default_rows.push(a);
                        }
                        ast::Pat::Wild(_) => {
                            let mut a = r;
                            a.pats.remove(0);
                            default_rows.push(a);
                        }
                        other => return Err(Error::at(other.span(), crate::diagnostic!("pattern does not match {}", ty0))),
                    }
                }
                let rest_cols = cols[1..].to_vec();
                let rest_tys = types[1..].to_vec();
                if let Some(ref kl) = known_lit {
                    if let Some(pos) = lit_values.iter().position(|v| v == kl) {
                        let mut branch = std::mem::take(&mut lit_rows[pos]);
                        branch.extend(default_rows);
                        branch.sort_by_key(|r| r.idx);
                        return self.compile_rows(branch, rest_cols, rest_tys, expected, ctx_span, kl);
                    }
                }
                if default_rows.is_empty() {
                    return Err(Error::at(ctx_span, "non-exhaustive numeric patterns: add a default `_` arm"));
                }
                let mut branch_maps = Vec::new();
                let (mut chain, dm) = self.isolate(|s| {
                    s.compile_rows(default_rows.clone(), rest_cols.clone(), rest_tys.clone(), expected, ctx_span, "default")
                })?;
                branch_maps.push(dm);
                for (digits, mut lrows) in lit_values.into_iter().zip(lit_rows.into_iter()).rev() {
                    for dr in &default_rows {
                        lrows.push(dr.clone());
                    }
                    lrows.sort_by_key(|r| r.idx);
                    let mut l_exp = expected.clone();
                    let mut l_tys = rest_tys.clone();
                    if is_refinable_val {
                        self.apply_sing_branch_refinement(
                            &zval,
                            &Ty::Idx(Box::new(ITerm::Num(digits.clone().into()))),
                            &mut l_exp,
                            &mut l_tys,
                            &mut lrows,
                        );
                    }
                    let (body, bm) = self.isolate(|s| {
                        s.compile_rows(lrows, rest_cols.clone(), l_tys, &l_exp, ctx_span, &digits)
                    })?;
                    branch_maps.push(bm);
                    let lit_core = self.literal_const(&zcarrier, &digits, ctx_span)?;
                    let cmp = literal_eq_cond(&zcarrier, observed.clone(), lit_core);
                    chain = ECore::If(Box::new(cmp), CBranch::new(body), CBranch::new(chain));
                }
                self.merge_branches(branch_maps);
                Ok(chain)
            }

            other => Err(Error::at(
                ctx_span,
                crate::diagnostic!("pattern matching on `Sing` is not supported for carrier {}", other),
            )),
        }
    }
}

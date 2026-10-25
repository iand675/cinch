// Included into `check.rs` at module scope: term elaboration, the flat
// pattern-match compiler, and local declarations. Kept separate for
// readability only.

impl Checker {
    // ------------------------------------------------------------ terms

    /// Is this term a record-projection place: a variable or a projection
    /// chain rooted at a variable (`x.f.g`)?
    fn is_place(t: &ast::Term) -> bool {
        match t {
            ast::Term::Var(_) => true,
            ast::Term::Proj(e, _, _) => Self::is_place(e),
            _ => false,
        }
    }

    /// The local a projection place is rooted at, if any.
    fn place_local(&self, env: &Env, t: &ast::Term) -> Option<LocalId> {
        match t {
            ast::Term::Var(p) if p.quals.is_empty() => env.lookup(&p.name.text).map(|(l, _)| l),
            ast::Term::Proj(e, _, _) => self.place_local(env, e),
            _ => None,
        }
    }

    /// Is `name` an unshadowed builtin form (`delay`, `force`, `demand`,
    /// `copyOut`, `borrow`, `mutBorrow`)? A user definition of the same
    /// name shadows the builtin (decision D2).
    fn is_builtin_form(&self, env: &Env, name: &str) -> bool {
        matches!(name, "delay" | "force" | "demand" | "copyOut" | "borrow" | "mutBorrow")
            && env.lookup(name).is_none()
            && self.lookup_value(None, name).is_err()
    }

    fn infer(&mut self, env: &mut Env, term: &ast::Term) -> Result<(ECore, Ty)> {
        match term {
            // In inference position a literal keeps the default `Int` type
            // (the mathematical `Int`, decision D7); against a known
            // expected type it elaborates to that type with range checking
            // (primitive types specification, section 7).
            ast::Term::Int(v, _) => Ok((
                ECore::ConstBig { nat: false, digits: v.clone() },
                Ty::Int,
            )),
            ast::Term::Unit(_) => Ok((ECore::Unit, Ty::Unit)),
            ast::Term::Str(_, span) => Err(Error::at(
                *span,
                "string values are not supported in the bootstrap slice",
            )),
            ast::Term::Char(s, span) => {
                Ok((ECore::Int(char_literal_word(s, *span)?), Ty::Prim(Scalar::Char)))
            }
            ast::Term::Hole(span) => Err(Error::at(
                *span,
                "typed holes are not supported in the bootstrap slice",
            )),
            ast::Term::Var(p) => {
                if p.quals.is_empty()
                    && self.is_builtin_form(env, &p.name.text)
                {
                    return Err(Error::at(
                        p.span(),
                        format!("{} must be applied in the bootstrap slice", p.name.text),
                    ));
                }
                if p.quals.is_empty() {
                    if let Some((l, s)) = env.lookup(&p.name.text) {
                        let (t, _) = self.instantiate(&s);
                        let t = self.uni.zonk(&t);
                        // A bare use of a memo cell demands it and copies
                        // the cached value out; the binding name elaborates
                        // to `demand` followed by `copyOut` (laziness and
                        // reclamation specification, section 5). Memo
                        // cells cache `Share` values, so the copy always
                        // has structural evidence.
                        if let Ty::Lazy { eff, region, inner } = t {
                            self.note_use(l, p.span(), &[])?;
                            self.new_want(
                                ClassApp { class: "Copy".into(), args: vec![(*inner).clone()] },
                                p.span(),
                                env,
                            );
                            self.perform(&EffRow {
                                labels: vec![EffLabel::with_arg("Eval", region)],
                                tail: RowTail::Closed,
                            })?;
                            self.perform(&eff)?;
                            return Ok((ECore::Demand(Box::new(ECore::Var(l))), *inner));
                        }
                        let mut path = std::mem::take(&mut self.place_path);
                        path.reverse();
                        self.note_use(l, p.span(), &path)?;
                        return Ok((ECore::Var(l), t));
                    }
                }
                self.place_path.clear();
                let v = self.resolve_var(p, p.span())?;
                self.value_use(&v, p.span(), env)
            }
            ast::Term::Con(p) => {
                let v = self.resolve_con(p, p.span())?;
                self.value_use(&v, p.span(), env)
            }
            ast::Term::App(f, args, span) => {
                // Builtin deferral and borrow forms: `delay`, `force`,
                // `demand`, `copyOut`, `borrow`, `mutBorrow`. They are
                // special forms; a user definition of the same name shadows
                // them.
                if let ast::Term::Var(p) = &**f {
                    if p.quals.is_empty() && self.is_builtin_form(env, &p.name.text) {
                        return self.infer_builtin_app(env, &p.name.text, args, *span);
                    }
                }
                let (mut core, mut ty) = self.infer(env, f)?;
                for a in args {
                    ty = self.uni.zonk(&ty);
                    let (arg_core, cod) = match ty {
                        Ty::Fun(dom, _, eff, cod) => {
                            self.perform(&eff)?;
                            (self.check(env, a, &dom)?, *cod)
                        }
                        Ty::Meta(_) => {
                            let dom = self.uni.fresh();
                            let cod = self.uni.fresh();
                            let eff = self.uni.fresh_row();
                            self.uni
                                .unify(&ty, &Ty::fun_mult(dom.clone(), Mult::Flex, eff.clone(), cod.clone()))
                                .map_err(|m| Error::at(f.span(), m))?;
                            self.perform(&eff)?;
                            (self.check(env, a, &dom)?, cod)
                        }
                        other => {
                            return Err(Error::at(
                                a.span(),
                                format!("cannot apply a value of type {other}"),
                            ));
                        }
                    };
                    core = app_spine(core, vec![arg_core]);
                    ty = cod;
                }
                Ok((core, ty))
            }
            ast::Term::Lam(_, _, _) => {
                let t = self.uni.fresh();
                let core = self.check_lam(env, term, &t)?;
                Ok((core, t))
            }
            ast::Term::Let(decls, body, span) => {
                let mut env2 = env.clone();
                let (binds, group) = self.elab_locals(&mut env2, decls)?;
                let (bcore, bty) = self.infer(&mut env2, body)?;
                if let Some(g) = &group {
                    self.discharge_group(g, &bty, *span)?;
                }
                Ok((wrap_lets(binds, bcore), bty))
            }
            ast::Term::If(c, t, e, _) => {
                let ccore = self.check(env, c, &Ty::Bool)?;
                let ((tcore, tty), tm) = self.isolate(|s| s.infer(env, t))?;
                let (ecore, em) = self.isolate(|s| s.check(env, e, &tty))?;
                self.merge_branches(vec![tm, em]);
                Ok((ECore::If(Box::new(ccore), Box::new(tcore), Box::new(ecore)), tty))
            }
            ast::Term::Case(scrut, alts, span) => {
                let (scrut_core, scrut_ty) = self.infer(env, scrut)?;
                let scrut_ty = self.uni.zonk(&scrut_ty);
                let res = self.uni.fresh();
                let core = self.compile_alts(env, alts, scrut_core, scrut_ty, &res, *span)?;
                Ok((core, res))
            }
            ast::Term::Ann(e, ty, _) => {
                let t = self.conv_type(ty, &[])?;
                let core = self.check(env, e, &t)?;
                Ok((core, t))
            }
            ast::Term::Infix(_, _, span) => Err(Error::at(
                *span,
                "internal error: unresolved infix chain reached the checker",
            )),
            ast::Term::Neg(e, span) => {
                // Prefix negation is the class method `negate`, never a builtin.
                let negate = ast::Term::Var(ast::VarPath {
                    quals: Vec::new(),
                    name: ast::Name::new("negate", *span),
                });
                let app = ast::Term::App(Box::new(negate), vec![(**e).clone()], *span);
                self.infer(env, &app)
            }
            ast::Term::Tuple(items, span) => {
                let gid = self.tuple_ctor(items.len(), *span)?;
                let mut cores = Vec::new();
                let mut tys = Vec::new();
                for it in items {
                    // Check each component against a fresh variable, so a
                    // literal component picks up its type from the tuple's
                    // expected type (primitive types specification,
                    // section 7).
                    let t = self.uni.fresh();
                    let c = self.check(env, it, &t)?;
                    cores.push(c);
                    tys.push(t);
                }
                Ok((app_spine(ECore::Ctor(gid), cores), Ty::Tuple(tys)))
            }
            ast::Term::List(items, span) => {
                // `[e1, ..., en]` desugars to `Cons e1 (... (Cons en Nil))`
                // against the prelude `List` type.
                let prelude = self.prelude_module.clone();
                let nil = self
                    .lookup_value(Some(&prelude), "Nil")
                    .map_err(|e| Error::at(*span, e.message))?;
                let cons = self
                    .lookup_value(Some(&prelude), "Cons")
                    .map_err(|e| Error::at(*span, e.message))?;
                let elem_ty = self.uni.fresh();
                let list_ty = Ty::Adt("List".into(), vec![elem_ty.clone()]);
                let (mut core, nil_ty) = self.value_use(&nil, *span, env)?;
                self.uni
                    .unify(&nil_ty, &list_ty)
                    .map_err(|m| Error::at(*span, m))?;
                for item in items.iter().rev() {
                    let ic = self.check(env, item, &elem_ty)?;
                    let (cons_core, cons_ty) = self.value_use(&cons, *span, env)?;
                    let want =
                        Ty::funs(&[elem_ty.clone(), list_ty.clone()], list_ty.clone());
                    self.uni
                        .unify(&cons_ty, &want)
                        .map_err(|m| Error::at(*span, m))?;
                    core = app_spine(cons_core, vec![ic, core]);
                }
                Ok((core, list_ty))
            }
            ast::Term::Proj(e, field, span) => {
                // `x.f.g` is a record-projection place rooted at a
                // variable; usage counting tracks these places per binder
                // (ownership and usage design, section 2).
                let (core, ty) = if Self::is_place(e) {
                    self.place_path.push(field.text.clone());
                    let r = self.infer(env, e);
                    self.place_path.pop();
                    r?
                } else {
                    let saved = std::mem::take(&mut self.place_path);
                    let r = self.infer(env, e);
                    self.place_path = saved;
                    r?
                };
                let ty = self.uni.zonk(&ty);
                // The operand's type decides the record when it is known;
                // otherwise the field name must resolve to one record and
                // the operand is checked against it.
                let (rec_name, args) = match &ty {
                    Ty::Adt(n, args) => (n.clone(), args.clone()),
                    Ty::Meta(_) => {
                        let (info, _) = self.find_record_field(&field.text, *span)?;
                        let fresh: Vec<Ty> =
                            (0..info.arity).map(|_| self.uni.fresh()).collect();
                        let want = Ty::Adt(info.name.clone(), fresh.clone());
                        self.uni
                            .unify(&ty, &want)
                            .map_err(|m| Error::at(*span, m))?;
                        (info.name, fresh)
                    }
                    other => {
                        return Err(Error::at(
                            *span,
                            format!(
                                "cannot project field {} from a value of type {other}",
                                field.text
                            ),
                        ));
                    }
                };
                let info = self.lookup_type(&[&rec_name]).map_err(|e| {
                    Error::at(*span, e.message)
                })?;
                let fields = info.fields.ok_or_else(|| {
                    Error::at(
                        *span,
                        format!(
                            "{rec_name} is not a record; field projection needs a record type"
                        ),
                    )
                })?;
                let idx = fields.iter().position(|f| f == &field.text).ok_or_else(|| {
                    Error::at(field.span, format!("record {rec_name} has no field {}", field.text))
                })?;
                let v = self.lookup_value(None, &rec_name)?;
                let field_ty = match &v.kind {
                    ValueKind::Ctor { field_tys, ty_params, .. } => {
                        let mut map = HashMap::new();
                        for (p, a) in ty_params.iter().zip(&args) {
                            map.insert(p.clone(), a.clone());
                        }
                        Self::subst_ty(&field_tys[idx], &map)
                    }
                    _ => {
                        return Err(Error::at(*span, format!("{rec_name} has no constructor")));
                    }
                };
                Ok((ECore::Proj(Box::new(core), idx), field_ty))
            }
            ast::Term::Record(path, fields, span) => {
                let v = self.resolve_con(path, *span)?;
                let parts = path.parts();
                let info = self
                    .lookup_type(&parts)
                    .map_err(|_| Error::at(*span, format!("unknown type {}", parts.join("."))))?;
                let names = info.fields.ok_or_else(|| {
                    Error::at(
                        *span,
                        format!(
                            "{} is not a record; `{} {{ ... }}` construction needs a record type",
                            parts.join("."),
                            parts.join(".")
                        ),
                    )
                })?;
                let mut assigned: HashMap<&str, &ast::Term> = HashMap::new();
                for (n, t) in fields {
                    if assigned.insert(n.text.as_str(), t).is_some() {
                        return Err(Error::at(
                            n.span,
                            format!("duplicate record field {}", n.text),
                        ));
                    }
                }
                for (n, _) in fields {
                    if !names.iter().any(|f| f == &n.text) {
                        return Err(Error::at(
                            n.span,
                            format!("unknown record field {} on {}", n.text, names.join(", ")),
                        ));
                    }
                }
                let mut missing = names
                    .iter()
                    .filter(|f| !assigned.contains_key(f.as_str()));
                if let Some(f) = missing.next() {
                    return Err(Error::at(
                        *span,
                        format!("record construction is missing field {f}"),
                    ));
                }
                let (ctor_core, mut rest) = self.value_use(&v, *span, env)?;
                // The constructor's arrows are in declaration order; field
                // expressions evaluate in source order (core specification:
                // record fields are left-to-right).
                let mut doms: Vec<(&str, Ty)> = Vec::new();
                for f in names.iter() {
                    rest = self.uni.zonk(&rest);
                    match rest {
                        Ty::Fun(a, _, _, b) => {
                            doms.push((f.as_str(), *a));
                            rest = *b;
                        }
                        other => {
                            return Err(Error::at(
                                *span,
                                format!(
                                    "record constructor {} has type {}, which takes no field {f}",
                                    v.name, other
                                ),
                            ));
                        }
                    }
                }
                let mut evaluated: HashMap<&str, ECore> = HashMap::new();
                for (n, t) in fields {
                    let dom = doms
                        .iter()
                        .find(|(f, _)| *f == n.text.as_str())
                        .map(|(_, d)| d.clone())
                        .expect("field membership checked above");
                    evaluated.insert(n.text.as_str(), self.check(env, t, &dom)?);
                }
                let arg_cores: Vec<ECore> = doms
                    .iter()
                    .map(|(f, _)| {
                        evaluated
                            .remove(*f)
                            .expect("every declared field was assigned")
                    })
                    .collect();
                Ok((app_spine(ctor_core, arg_cores), rest))
            }
        }
    }

    fn check(&mut self, env: &mut Env, term: &ast::Term, expected: &Ty) -> Result<ECore> {
        match term {
            ast::Term::Lam(_, _, _) => self.check_lam(env, term, expected),
            ast::Term::If(c, t, e, _) => {
                let ccore = self.check(env, c, &Ty::Bool)?;
                let (tcore, tm) = self.isolate(|s| s.check(env, t, expected))?;
                let (ecore, em) = self.isolate(|s| s.check(env, e, expected))?;
                self.merge_branches(vec![tm, em]);
                Ok(ECore::If(Box::new(ccore), Box::new(tcore), Box::new(ecore)))
            }
            ast::Term::Let(decls, body, span) => {
                let mut env2 = env.clone();
                let (binds, group) = self.elab_locals(&mut env2, decls)?;
                let bcore = self.check(&mut env2, body, expected)?;
                if let Some(g) = &group {
                    self.discharge_group(g, expected, *span)?;
                }
                Ok(wrap_lets(binds, bcore))
            }
            ast::Term::Case(scrut, alts, span) => {
                let (scrut_core, scrut_ty) = self.infer(env, scrut)?;
                let scrut_ty = self.uni.zonk(&scrut_ty);
                self.compile_alts(env, alts, scrut_core, scrut_ty, expected, *span)
            }
            ast::Term::Int(v, span) => self.lit_check(expected, v, *span),
            ast::Term::Neg(inner, span) => {
                // Prefix `-` folds into a literal and is range-checked with
                // it when the expected type is known (primitive types
                // specification, section 7); otherwise it elaborates to the
                // class method `negate`, so `-x` on a machine integer is
                // rejected for its missing `Neg` instance. A folded `-` at
                // `Nat` is rejected as an out-of-range literal.
                let t = self.uni.zonk(expected);
                if matches!(t, Ty::Int | Ty::Nat | Ty::Prim(_)) {
                    if let Some(value) = neg_fold(inner) {
                        return self.lit_check(&t, &value, *span);
                    }
                }
                let (core, ty) = self.infer(env, term)?;
                self.uni
                    .unify(&ty, expected)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(core)
            }
            ast::Term::App(f, args, span) => {
                // `borrow e` / `mutBorrow e` are bidirectional: the
                // borrow's region comes from the expected type (ownership
                // and usage design, section 4).
                if let ast::Term::Var(p) = &**f {
                    if p.quals.is_empty()
                        && matches!(p.name.text.as_str(), "borrow" | "mutBorrow")
                        && self.is_builtin_form(env, &p.name.text)
                    {
                        let is_mut = p.name.text == "mutBorrow";
                        return self.check_borrow_form(env, is_mut, args, *span, expected);
                    }
                }
                let (core, ty) = self.infer(env, term)?;
                self.uni
                    .unify(&ty, expected)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(core)
            }
            _ => {
                let (core, ty) = self.infer(env, term)?;
                self.uni
                    .unify(&ty, expected)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(core)
            }
        }
    }

    /// `delay e`, `force t`, `demand d`: builtin deferral forms from the
    /// laziness and reclamation specification. They are special forms so
    /// their effect rows are synthesized exactly.
    fn infer_builtin_app(
        &mut self,
        env: &mut Env,
        name: &str,
        args: &[ast::Term],
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        if args.len() != 1 {
            return Err(Error::at(
                span,
                format!("{name} takes exactly one argument in the bootstrap slice"),
            ));
        }
        match name {
            "delay" => {
                // arg : Unit ->{e} a; result Thunk {e} a; creating the thunk
                // performs nothing.
                let eff = self.uni.fresh_row();
                let inner = self.uni.fresh();
                let fty = Ty::fun_eff(Ty::Unit, eff.clone(), inner.clone());
                let arg = self.check(env, &args[0], &fty)?;
                Ok((
                    app_spine(ECore::Prim(PrimOp::Delay), vec![arg]),
                    Ty::Thunk { eff, inner: Box::new(inner) },
                ))
            }
            "force" => {
                let (arg, ty) = self.infer(env, &args[0])?;
                let eff = self.uni.fresh_row();
                let inner = self.uni.fresh();
                self.uni
                    .unify(&ty, &Ty::Thunk { eff: eff.clone(), inner: Box::new(inner.clone()) })
                    .map_err(|m| {
                        Error::at(args[0].span(), format!("{m} (force expects a Thunk)"))
                    })?;
                self.perform(&eff)?;
                Ok((app_spine(ECore::Prim(PrimOp::Force), vec![arg]), inner))
            }
            "demand" => {
                // `demand` yields a borrow of the cached value, so the
                // cache is never silently duplicated (laziness and
                // reclamation specification, section 5); `copyOut`
                // produces the owned value. A named cell resolves through
                // its `Lazy` type here, without the bare-use demand.
                let (arg, ty) = match &args[0] {
                    ast::Term::Var(p) if p.quals.is_empty() => {
                        match env.lookup(&p.name.text) {
                            Some((l, s)) => {
                                let (t, _) = self.instantiate(&s);
                                let t = self.uni.zonk(&t);
                                if matches!(t, Ty::Lazy { .. }) {
                                    self.note_use(l, p.span(), &[])?;
                                    (ECore::Var(l), t)
                                } else {
                                    self.infer(env, &args[0])?
                                }
                            }
                            None => self.infer(env, &args[0])?,
                        }
                    }
                    _ => self.infer(env, &args[0])?,
                };
                match self.uni.zonk(&ty) {
                    Ty::Lazy { eff, region, inner } => {
                        self.perform(&EffRow {
                            labels: vec![EffLabel::with_arg("Eval", region.clone())],
                            tail: RowTail::Closed,
                        })?;
                        self.perform(&eff)?;
                        Ok((
                            ECore::Demand(Box::new(arg)),
                            Ty::Ref(region, inner),
                        ))
                    }
                    Ty::Meta(_) => Err(Error::at(
                        args[0].span(),
                        "cannot determine the memo type of demand's argument; add an annotation",
                    )),
                    other => Err(Error::at(
                        args[0].span(),
                        format!("demand expects a Lazy cell, got {other}"),
                    )),
                }
            }
            "copyOut" => {
                // Copy a value out of a borrow (or duplicate a `Copy`
                // value): explicit copying requires `Copy` evidence and
                // coherence with the source's multiplicity (ownership and
                // usage design, sections 2 and 3).
                let (arg_core, arg_ty) = self.infer(env, &args[0])?;
                let arg_ty = self.uni.zonk(&arg_ty);
                let (copied, from_borrow) = match &arg_ty {
                    Ty::Ref(_, inner) | Ty::Mut(_, inner) => ((**inner).clone(), true),
                    t => (t.clone(), false),
                };
                if !from_borrow && Self::is_place(&args[0]) {
                    if let Some(l) = self.place_local(env, &args[0]) {
                        if let Some(m) = self.effective_mult(l) {
                            if m != Mult::Many {
                                let name = self
                                    .obligations
                                    .get(&l)
                                    .map(|o| o.name.clone())
                                    .unwrap_or_else(|| "_".to_string());
                                return Err(Error::at(
                                    args[0].span(),
                                    format!(
                                        "multiplicity {m}: copyOut would copy {name}, but a value bound {m} cannot be duplicated (copying needs a %many binder or a borrow)"
                                    ),
                                ));
                            }
                        }
                    }
                }
                self.new_want(
                    ClassApp { class: "Copy".into(), args: vec![copied.clone()] },
                    span,
                    env,
                );
                Ok((arg_core, copied))
            }
            "borrow" | "mutBorrow" => Err(Error::at(
                span,
                format!(
                    "cannot infer the region of {name}; add a type annotation (`{name} e` against `Ref r a` or `Mut r a`)"
                ),
            )),
            _ => unreachable!(),
        }
    }

    /// `borrow e` / `mutBorrow e` against an expected `Ref r a` / `Mut r a`
    /// (ownership and usage design, section 4). A borrow's region comes
    /// from its expected type; the elaboration is the operand's uniform
    /// word (a borrow is a pointer word at runtime).
    fn check_borrow_form(
        &mut self,
        env: &mut Env,
        is_mut: bool,
        args: &[ast::Term],
        span: ast::Span,
        expected: &Ty,
    ) -> Result<ECore> {
        if args.len() != 1 {
            return Err(Error::at(
                span,
                "borrow and mutBorrow take exactly one argument in the bootstrap slice",
            ));
        }
        let want = self.uni.zonk(expected);
        let (region, inner) = match (&want, is_mut) {
            (Ty::Ref(r, inner), false) => (r.clone(), (**inner).clone()),
            (Ty::Mut(r, inner), true) => (r.clone(), (**inner).clone()),
            (Ty::Meta(_), _) => {
                return Err(Error::at(
                    span,
                    "cannot infer the region of this borrow; add a type annotation (`Ref r a` or `Mut r a`)",
                ));
            }
            (other, _) => {
                let form = if is_mut { "Mut r a" } else { "Ref r a" };
                return Err(Error::at(
                    span,
                    format!("borrow against {other}: expected a {form}"),
                ));
            }
        };
        let _ = region;
        self.check(env, &args[0], &inner)
    }

    fn check_lam(&mut self, env: &mut Env, term: &ast::Term, expected: &Ty) -> Result<ECore> {
        let (pats, body, span) = match term {
            ast::Term::Lam(pats, body, span) => (pats, body, *span),
            _ => unreachable!(),
        };
        let mut env2 = env.clone();
        let mut params = Vec::new();
        let mut rest = expected.clone();
        let mut effs = Vec::new();
        for p in pats {
            rest = self.uni.zonk(&rest);
            let (dom, mult, eff) = match rest {
                Ty::Fun(a, m, eff, b) => {
                    let d = *a;
                    rest = *b;
                    (d, m, eff)
                }
                Ty::Meta(_) => {
                    let a = self.uni.fresh();
                    let b = self.uni.fresh();
                    let eff = self.uni.fresh_row();
                    self.uni
                        .unify(&rest, &Ty::fun_mult(a.clone(), Mult::Flex, eff.clone(), b.clone()))
                        .map_err(|m| Error::at(p.span(), m))?;
                    rest = b;
                    (a, Mult::Flex, eff)
                }
                other => {
                    return Err(Error::at(
                        p.span(),
                        format!("lambda parameter against non-function type {other}"),
                    ));
                }
            };
            let l = self.fresh_local();
            match p {
                ast::Pat::Var(n, _) => {
                    self.bind_local(
                        &mut env2,
                        &n.text,
                        l,
                        Scheme::mono(dom),
                        Some(mult),
                        n.span,
                    )?;
                }
                ast::Pat::Wild(_) => {
                    self.register_obligation(l, "_", Some(mult), dom, p.span());
                }
                other => {
                    return Err(Error::at(
                        other.span(),
                        "only variable and wildcard patterns are supported in lambdas in the bootstrap slice",
                    ));
                }
            }
            params.push(l);
            effs.push(eff);
        }
        // Curried semantics: the body runs at the last application, so the
        // body's effects belong to the last arrow and intermediate arrows
        // are pure (closure construction only). The body is a lazy
        // position for the enclosing binders (ownership and usage design,
        // section 2).
        let saved = self.enter_perf();
        self.lam_depth += 1;
        let bcore = self.check(&mut env2, body, &rest);
        self.lam_depth -= 1;
        let bcore = bcore?;
        let performed = self.exit_perf(saved);
        match effs.split_last() {
            Some((last, earlier)) => {
                for e in earlier {
                    self.uni
                        .unify_rows(e, &EffRow::pure())
                        .map_err(|m| Error::at(span, m))?;
                }
                self.uni
                    .unify_rows(last, &performed)
                    .map_err(|m| Error::at(span, m))?;
            }
            None => {}
        }
        Ok(ECore::Lam { params, body: Box::new(bcore) })
    }

    /// Instantiate a resolved value: dictionary constraints become wants and
    /// methods elaborate to dictionary projection.
    fn value_use(&mut self, v: &ValueInfo, span: ast::Span, env: &mut Env) -> Result<(ECore, Ty)> {
        match &v.kind {
            ValueKind::BoolLit(b) => return Ok((ECore::Bool(*b), Ty::Bool)),
            _ => {}
        }
        let (ty, ctx) = self.instantiate(&v.scheme);
        let mut holes = Vec::new();
        for (_, c) in &ctx {
            let id = self.new_want(c.clone(), span, env);
            holes.push(ECore::Hole(id));
        }
        let core = match &v.kind {
            ValueKind::TopFn => ECore::Global(fn_id(&v.module, &v.name)),
            ValueKind::Prim(op) => ECore::Prim(*op),
            ValueKind::Ctor { .. } => ECore::Ctor(fn_id(&v.module, &v.name)),
            ValueKind::Method { idx, .. } => {
                // The class constraint is `ctx[0]`: it becomes the dictionary
                // the method is projected from.
                let dict = holes.remove(0);
                ECore::DictProj(Box::new(dict), *idx)
            }
            ValueKind::BoolLit(_) => unreachable!(),
        };
        let core = if holes.is_empty() { core } else { app_spine(core, holes) };
        Ok((core, ty))
    }

    // ------------------------------------------------------------ local declarations

    fn elab_locals(
        &mut self,
        env: &mut Env,
        decls: &[ast::LocalDecl],
    ) -> Result<(Vec<(LocalId, ECore)>, Option<MemoGroup>)> {
        // Signatures first.
        let mut sigs: HashMap<String, Ty> = HashMap::new();
        for d in decls {
            if let ast::LocalDecl::Sig { name, ty, span } = d {
                if sigs.contains_key(&name.text) {
                    return Err(Error::at(*span, "duplicate local signature"));
                }
                let s = self.conv_scheme(ty)?;
                if !s.params.is_empty()
                    || !s.ctx.is_empty()
                    || !s.row_params.is_empty()
                    || !s.region_params.is_empty()
                {
                    return Err(Error::at(
                        *span,
                        "local signatures with forall or using are not supported in the bootstrap slice",
                    ));
                }
                sigs.insert(name.text.clone(), s.ty);
            }
        }

        // Memo group: all `~` bindings of one block share one region and one
        // `Eval` label (laziness and reclamation specification, section 5).
        let lazy_decls: Vec<&ast::LocalDecl> = decls
            .iter()
            .filter(|d| matches!(d, ast::LocalDecl::Lazy { .. }))
            .collect();
        let group = if lazy_decls.is_empty() {
            None
        } else {
            let region = self.fresh_region();
            let label = EffLabel::with_arg("Eval", region.clone());
            self.active_regions.push(region.clone());
            Some(MemoGroup { region, label })
        };
        // Pre-bind the cell names so siblings can reference each other and
        // strict bindings can reference them too.
        let mut cell_info: Vec<(LocalId, EffRow, Ty)> = Vec::new();
        if let Some(g) = &group {
            for d in &lazy_decls {
                let pat = match d {
                    ast::LocalDecl::Lazy { pat, .. } => pat,
                    _ => unreachable!(),
                };
                let name = match pat {
                    ast::Pat::Var(n, _) => n.text.clone(),
                    ast::Pat::Wild(_) => {
                        return Err(Error::at(
                            pat.span(),
                            "a memo binding must have a name",
                        ));
                    }
                    other => {
                        return Err(Error::at(
                            other.span(),
                            "memo bindings support only variable patterns in the bootstrap slice",
                        ));
                    }
                };
                let l = self.fresh_local();
                let eff = self.uni.fresh_row();
                let inner = self.uni.fresh();
                let cell_ty = Ty::Lazy {
                    eff: eff.clone(),
                    region: g.region.clone(),
                    inner: Box::new(inner.clone()),
                };
                self.bind_local(env, &name, l, Scheme::mono(cell_ty), None, pat.span())?;
                cell_info.push((l, eff, inner));
            }
        }

        // Strict bindings in source order first, so `~` bodies may see all
        // of them (cell evaluation is deferred past the whole block).
        let mut out = Vec::new();
        for d in decls {
            if matches!(d, ast::LocalDecl::Lazy { .. }) {
                continue;
            }
            match d {
                ast::LocalDecl::Sig { .. } => {}
                ast::LocalDecl::Eq { name, pats, body, span } => {
                    for p in pats {
                        if !matches!(p, ast::Pat::Var(..) | ast::Pat::Wild(..)) {
                            return Err(Error::at(
                                *span,
                                "local function equations support only variable and wildcard patterns in the bootstrap slice",
                            ));
                        }
                    }
                    let l = self.fresh_local();
                    let (core, ty, is_value) = if pats.is_empty() {
                        let (core, ty) = match sigs.get(&name.text).cloned() {
                            Some(t) => (self.check(env, body, &t)?, t),
                            None => self.infer(env, body)?,
                        };
                        (core, ty, is_syntactic_value(body))
                    } else {
                        // A local function is a lambda; `check_lam` enforces
                        // the per-arrow effect discipline.
                        let lam = ast::Term::Lam(pats.clone(), Box::new(body.clone()), *span);
                        match sigs.get(&name.text).cloned() {
                            Some(t) => (self.check_lam(env, &lam, &t)?, t, true),
                            None => {
                                let t = self.uni.fresh();
                                let c = self.check_lam(env, &lam, &t)?;
                                (c, self.uni.zonk(&t), true)
                            }
                        }
                    };
                    let scheme = self.generalize_local(env, &ty, is_value);
                    self.bind_local(env, &name.text, l, scheme, None, name.span)?;
                    out.push((l, core));
                }
                ast::LocalDecl::Bind { pat, body, span } => {
                    let (bc, bt) = self.infer(env, body)?;
                    let l = self.fresh_local();
                    match pat {
                        ast::Pat::Var(n, _) => {
                            let scheme =
                                self.generalize_local(env, &bt, is_syntactic_value(body));
                            self.bind_local(env, &n.text, l, scheme, None, n.span)?;
                        }
                        ast::Pat::Wild(_) => {}
                        ast::Pat::Unit(_)
                        | ast::Pat::Tuple(_, _)
                        | ast::Pat::As(_, _, _) => {
                            // Irrefutable structural patterns destructured
                            // through field projections.
                            out.push((l, bc));
                            self.destructure_bind(env, pat, l, &bt, &mut out)?;
                            continue;
                        }
                        _ => {
                            return Err(Error::at(
                                *span,
                                "refutable patterns in let bindings are not supported in the bootstrap slice; use case",
                            ));
                        }
                    }
                    out.push((l, bc));
                }
                ast::LocalDecl::Lazy { .. } => unreachable!(),
            }
        }

        // Memo cell bodies: each becomes a thunk closure stored into its
        // cell. The black-hole rule applies to every body.
        if let Some(g) = &group {
            let mut inits: Vec<(LocalId, ECore)> = Vec::new();
            for (d, (l, eff, inner)) in lazy_decls.iter().zip(cell_info.iter()) {
                let body = match d {
                    ast::LocalDecl::Lazy { body, .. } => body,
                    _ => unreachable!(),
                };
                let saved = self.enter_perf();
                self.memo_depth += 1;
                let checked = self.infer(env, body);
                self.memo_depth -= 1;
                let (bc, bt) = checked?;
                let performed = self.exit_perf(saved);
                self.uni
                    .require_lacks(&performed, g.label.clone())
                    .map_err(|m| {
                        Error::at(
                            body.span(),
                            format!("{m}: this memo binding demands its own memo group"),
                        )
                    })?;
                self.uni
                    .unify_rows(eff, &performed)
                    .map_err(|m| Error::at(body.span(), m))?;
                self.uni
                    .unify(inner, &bt)
                    .map_err(|m| Error::at(body.span(), m))?;
                let unit_param = self.fresh_local();
                let thunk = ECore::Lam {
                    params: vec![unit_param],
                    body: Box::new(bc),
                };
                inits.push((*l, thunk));
            }
            // Allocate every cell first (siblings capture each other), then
            // store the thunks.
            for (l, _) in &inits {
                out.push((*l, ECore::LazyAlloc));
            }
            for (l, thunk) in inits {
                let tmp = self.fresh_local();
                out.push((tmp, ECore::LazyInit(l, Box::new(thunk))));
            }
        }
        Ok((out, group))
    }

    /// Bind the components of an irrefutable structural pattern to fresh
    /// locals by field projection, extending `out` with one let per binder.
    fn destructure_bind(
        &mut self,
        env: &mut Env,
        pat: &ast::Pat,
        l: LocalId,
        ty: &Ty,
        out: &mut Vec<(LocalId, ECore)>,
    ) -> Result<()> {
        match pat {
            ast::Pat::Var(n, _) => {
                let scheme = self.generalize_local(env, ty, true);
                self.bind_local(env, &n.text, l, scheme, None, n.span)?;
            }
            ast::Pat::Wild(_) | ast::Pat::Unit(_) => {}
            ast::Pat::As(n, inner, _) => {
                let scheme = self.generalize_local(env, ty, true);
                self.bind_local(env, &n.text, l, scheme, None, n.span)?;
                self.destructure_bind(env, inner, l, ty, out)?;
            }
            ast::Pat::Tuple(items, span) => {
                let n = items.len();
                let zonked = self.uni.zonk(ty);
                let comps = match &zonked {
                    Ty::Tuple(c) if c.len() == n => c.clone(),
                    Ty::Meta(_) => {
                        let comps: Vec<Ty> = (0..n).map(|_| self.uni.fresh()).collect();
                        self.uni
                            .unify(&zonked, &Ty::Tuple(comps.clone()))
                            .map_err(|m| Error::at(*span, m))?;
                        comps
                    }
                    other => {
                        return Err(Error::at(
                            *span,
                            format!("pattern does not match type {other}"),
                        ));
                    }
                };
                for (i, item) in items.iter().enumerate() {
                    let fl = self.fresh_local();
                    out.push((fl, ECore::Proj(Box::new(ECore::Var(l)), i)));
                    self.destructure_bind(env, item, fl, &comps[i], out)?;
                }
            }
            other => {
                return Err(Error::at(
                    other.span(),
                    "refutable patterns in let bindings are not supported in the bootstrap slice; use case",
                ));
            }
        }
        Ok(())
    }

    /// Generalize a local binding (let-polymorphism) under the value
    /// restriction: only syntactic values generalize, so a memo cell or an
    /// effectful computation is never shared polymorphically.
    fn generalize_local(&mut self, env: &Env, ty: &Ty, is_value: bool) -> Scheme {
        if !is_value {
            return Scheme::mono(ty.clone());
        }
        let env_metas = env_free_metas(&self.uni, env);
        // A deferred literal's type stays monomorphic: one literal has one
        // type (primitive types specification, section 7).
        let lit_metas = self.literal_metas();
        let mine = self.uni.free_metas(ty);
        let quantify: Vec<EitherMeta> = mine
            .into_iter()
            .filter(|m| !env_metas.contains(m) && !lit_metas.contains(m))
            .collect();
        if quantify.is_empty() {
            return Scheme::mono(ty.clone());
        }
        let (ty2, tps, rps) = self.uni.generalize(ty, &quantify);
        Scheme {
            params: tps,
            row_params: rps,
            region_params: Vec::new(),
            ctx: Vec::new(),
            ty: ty2,
        }
    }

    // ------------------------------------------------------------ pattern matching

    fn compile_alts(
        &mut self,
        env: &mut Env,
        alts: &[ast::Alt],
        scrut_core: ECore,
        scrut_ty: Ty,
        expected: &Ty,
        span: ast::Span,
    ) -> Result<ECore> {
        let sloc = self.fresh_local();
        let mut rows: Vec<Row> = Vec::new();
        for (i, a) in alts.iter().enumerate() {
            rows.push(Row {
                pats: vec![self.expand_pat(&a.pat)?],
                guard: a.guard.clone(),
                body: a.body.clone(),
                env: env.clone(),
                idx: i,
                consumed: Vec::new(),
            });
        }
        let covered = self.matrix_covered(&rows, &[scrut_ty.clone()])?;
        let core = self.compile_rows(
            rows,
            vec![ECore::Var(sloc)],
            vec![scrut_ty],
            expected,
            span,
            covered,
        )?;
        Ok(ECore::Let(sloc, Box::new(scrut_core), Box::new(core)))
    }

    /// Pattern-matrix compilation with source-order, fall-through
    /// semantics: the first row that matches wins, and a guarded row whose
    /// guard fails falls through to the later rows. `covered` says whether
    /// the pattern part of the whole match covers its scrutinee space; it
    /// chooses the fallback a fall-through reaches once every applicable
    /// row has failed its guard.
    fn compile_rows(
        &mut self,
        rows: Vec<Row>,
        cols: Vec<ECore>,
        types: Vec<Ty>,
        expected: &Ty,
        ctx_span: ast::Span,
        covered: bool,
    ) -> Result<ECore> {
        if rows.is_empty() {
            return self.run_out(covered);
        }
        for r in &rows {
            check_row_binders(&r.pats)?;
        }
        if types.is_empty() {
            let first = &rows[0];
            if let Some(g) = &first.guard {
                // The row's destructure runs on every path that reaches
                // the row, before its guard.
                for l in &first.consumed {
                    self.note_use(*l, ctx_span, &[])?;
                }
                // The guard is a Bool expression in the row's scope (all
                // pattern variables bound); its effects join the enclosing
                // performed scope. A failing guard falls through.
                let mut genv = first.env.clone();
                let gcore = self.check(&mut genv, g, &Ty::Bool)?;
                let (body_core, bm) = self.isolate(|s| {
                    let mut env = first.env.clone();
                    s.check(&mut env, &first.body, expected)
                })?;
                let (rest_core, rm) = self.isolate(|s| {
                    s.compile_rows(
                        rows[1..].to_vec(),
                        cols.clone(),
                        types.clone(),
                        expected,
                        ctx_span,
                        covered,
                    )
                })?;
                self.merge_branches(vec![bm, rm]);
                return Ok(ECore::If(
                    Box::new(gcore),
                    Box::new(body_core),
                    Box::new(rest_core),
                ));
            }
            for l in &first.consumed {
                self.note_use(*l, ctx_span, &[])?;
            }
            let mut env = first.env.clone();
            return self.check(&mut env, &first.body, expected);
        }
        let col0 = cols[0].clone();
        let ty0 = self.uni.zonk(&types[0]);
        let mut rows = rows;

        // Expand as-patterns in column 0 by binding the alias.
        for r in rows.iter_mut() {
            let mut pat = r.pats[0].clone();
            loop {
                match pat {
                    ast::Pat::As(n, inner, _) => {
                        if let ECore::Var(l) = &col0 {
                            self.bind_local(
                                &mut r.env,
                                &n.text,
                                *l,
                                Scheme::mono(ty0.clone()),
                                None,
                                n.span,
                            )?;
                        }
                        pat = (*inner).clone();
                    }
                    other => {
                        r.pats[0] = other;
                        break;
                    }
                }
            }
        }

        // Variable and wildcard patterns match a column of any type.
        if rows
            .iter()
            .all(|r| matches!(r.pats[0], ast::Pat::Var(..) | ast::Pat::Wild(_)))
        {
            return self.consume_column(rows, cols, types, expected, ctx_span, covered);
        }

        // Borrow matching (ownership and usage design, section 4): a
        // borrowed column matches its referent's patterns and binds
        // references to the fields; the elaboration is the referent's
        // uniform word.
        let (borrow, inner_ty) = match &ty0 {
            Ty::Ref(r, inner) => (Some((r.clone(), false)), (**inner).clone()),
            Ty::Mut(r, inner) => (Some((r.clone(), true)), (**inner).clone()),
            other => (None, other.clone()),
        };
        let wrap = |t: &Ty| -> Ty {
            match &borrow {
                Some((r, true)) => Ty::Mut(r.clone(), Box::new(t.clone())),
                Some((r, false)) => Ty::Ref(r.clone(), Box::new(t.clone())),
                None => t.clone(),
            }
        };
        match &inner_ty {
            Ty::Unit => {
                for r in rows.iter_mut() {
                    let p0 = r.pats[0].clone();
                    match &p0 {
                        ast::Pat::Unit(_) => self.mark_consumed(r, &col0),
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => {}
                        _ => {
                            return Err(Error::at(
                                p0.span(),
                                "pattern does not match type Unit",
                            ));
                        }
                    }
                }
                self.consume_column(rows, cols, types, expected, ctx_span, covered)
            }
            Ty::Prim(s @ (Scalar::F32 | Scalar::F64)) => {
                // Floats have no `Eq` instance (primitive types
                // specification, section 5), so only variables and
                // wildcards may match them.
                for r in &rows {
                    if !matches!(r.pats[0], ast::Pat::Var(..) | ast::Pat::Wild(_)) {
                        return Err(Error::at(
                            r.pats[0].span(),
                            format!(
                                "pattern does not match type {s}: floats have no Eq instance; compare with feq{s} in a guard"
                            ),
                        ));
                    }
                }
                self.consume_column(rows, cols, types, expected, ctx_span, covered)
            }
            Ty::Bytes => {
                // `Bytes` values have no literals or constructors to match;
                // only variables and wildcards (caught above).
                for r in &rows {
                    if !matches!(r.pats[0], ast::Pat::Var(..) | ast::Pat::Wild(_)) {
                        return Err(Error::at(
                            r.pats[0].span(),
                            "pattern does not match type Bytes",
                        ));
                    }
                }
                self.consume_column(rows, cols, types, expected, ctx_span, covered)
            }
            Ty::Int | Ty::Nat | Ty::Prim(_) => {
                // Integer literal patterns: a literal matches when the
                // column equals it; rows fall through in source order.
                // Machine scalars compare canonical words with `eqI64`
                // (every value has a canonical `i64` word, primitive types
                // specification, section 2, so word equality is value
                // equality); mathematical `Nat`/`Int` compare through the
                // runtime's `natCmp`/`intCmp` immediate. The comparison is
                // built here in the match compiler, so it can move to the
                // primitive table later.
                let mut literals: Vec<(String, ECore)> = Vec::new();
                let mut defaults: Vec<Row> = Vec::new();
                let mut lit_rows: Vec<(String, Row)> = Vec::new();
                for mut r in rows.into_iter() {
                    let p0 = r.pats[0].clone();
                    match &p0 {
                        ast::Pat::Var(n, _) => {
                            if let ECore::Var(l) = &col0 {
                                self.bind_local(
                                    &mut r.env,
                                    &n.text,
                                    *l,
                                    Scheme::mono(ty0.clone()),
                                    None,
                                    n.span,
                                )?;
                            }
                            let mut a = r.clone();
                            a.pats = r.pats[1..].to_vec();
                            defaults.push(a);
                        }
                        ast::Pat::Wild(_) => {
                            let mut a = r.clone();
                            a.pats = r.pats[1..].to_vec();
                            defaults.push(a);
                        }
                        ast::Pat::Int(v, _) => {
                            let node = self.literal_const(&inner_ty, v, p0.span())?;
                            if !literals.iter().any(|(x, _)| x == v) {
                                literals.push((v.clone(), node));
                            }
                            self.mark_consumed(&mut r, &col0);
                            let mut a = r.clone();
                            a.pats = r.pats[1..].to_vec();
                            lit_rows.push((v.clone(), a));
                        }
                        other => {
                            return Err(Error::at(
                                other.span(),
                                format!("pattern does not match type {inner_ty}"),
                            ));
                        }
                    }
                }
                let rest_cols = cols[1..].to_vec();
                let rest_tys = types[1..].to_vec();
                // Values no literal matched take the var and wildcard
                // rows. Every literal arm is its own evaluation path.
                let mut branch_maps = Vec::new();
                let (mut chain, m) = self.isolate(|s| {
                    s.compile_rows(
                        defaults.clone(),
                        rest_cols.clone(),
                        rest_tys.clone(),
                        expected,
                        ctx_span,
                        covered,
                    )
                })?;
                branch_maps.push(m);
                for (v, node) in literals.into_iter().rev() {
                    let mut branch = defaults.clone();
                    branch.extend(
                        lit_rows.iter().filter(|(x, _)| *x == v).map(|(_, r)| r.clone()),
                    );
                    branch.sort_by_key(|r| r.idx);
                    let (body, m) = self.isolate(|s| {
                        s.compile_rows(
                            branch,
                            rest_cols.clone(),
                            rest_tys.clone(),
                            expected,
                            ctx_span,
                            covered,
                        )
                    })?;
                    branch_maps.push(m);
                    let cmp = literal_eq_cond(&inner_ty, col0.clone(), node);
                    chain = ECore::If(Box::new(cmp), Box::new(body), Box::new(chain));
                }
                self.merge_branches(branch_maps);
                Ok(chain)
            }
            Ty::Bool => {
                let mut true_rows = Vec::new();
                let mut false_rows = Vec::new();
                for r in rows {
                    let p0 = r.pats[0].clone();
                    let tag = match &p0 {
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => None,
                        ast::Pat::Con(p, args, span) => {
                            let val = match p.parts().as_slice() {
                                ["True"] => true,
                                ["False"] => false,
                                _ => {
                                    return Err(Error::at(
                                        *span,
                                        format!("unknown constructor {}", p.parts().join(".")),
                                    ));
                                }
                            };
                            if !args.is_empty() {
                                return Err(Error::at(*span, "True and False take no arguments"));
                            }
                            Some(val)
                        }
                        other => {
                            return Err(Error::at(other.span(), "pattern does not match type Bool"));
                        }
                    };
                    let mut a = r.clone();
                    a.pats = r.pats[1..].to_vec();
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
                                    self.bind_local(
                                        &mut a.env,
                                        &n.text,
                                        *l,
                                        Scheme::mono(ty0.clone()),
                                        None,
                                        n.span,
                                    )?;
                                }
                            }
                            true_rows.push(a.clone());
                            false_rows.push(a);
                        }
                    }
                }
                if true_rows.is_empty() || false_rows.is_empty() {
                    return Err(Error::at(
                        ctx_span,
                        "non-exhaustive patterns: True and False must both be covered",
                    ));
                }
                let rest_cols = cols[1..].to_vec();
                let rest_tys = types[1..].to_vec();
                let (t, tm) = self.isolate(|s| {
                    s.compile_rows(
                        true_rows,
                        rest_cols.clone(),
                        rest_tys.clone(),
                        expected,
                        ctx_span,
                        covered,
                    )
                })?;
                let (f, fm) = self.isolate(|s| {
                    s.compile_rows(false_rows, rest_cols, rest_tys, expected, ctx_span, covered)
                })?;
                self.merge_branches(vec![tm, fm]);
                Ok(ECore::If(Box::new(col0), Box::new(t), Box::new(f)))
            }
            Ty::Adt(_, ty_args) => {
                let info = self.lookup_type(&[type_name_str(&inner_ty)])?.clone();
                let ty_args = ty_args.clone();
                // Classify rows by the column-0 constructor; each branch
                // specializes its rows by prepending the field patterns,
                // so nesting compiles recursively in source order.
                let mut ctor_rows: Vec<Vec<Row>> =
                    (0..info.ctors.len()).map(|_| Vec::new()).collect();
                let mut var_rows: Vec<Row> = Vec::new();
                for mut r in rows.into_iter() {
                    let p0 = r.pats[0].clone();
                    match &p0 {
                        ast::Pat::Var(n, _) => {
                            if let ECore::Var(l) = &col0 {
                                self.bind_local(
                                    &mut r.env,
                                    &n.text,
                                    *l,
                                    Scheme::mono(ty0.clone()),
                                    None,
                                    n.span,
                                )?;
                            }
                            let mut a = r.clone();
                            a.pats = r.pats[1..].to_vec();
                            var_rows.push(a);
                        }
                        ast::Pat::Wild(_) => {
                            let mut a = r.clone();
                            a.pats = r.pats[1..].to_vec();
                            var_rows.push(a);
                        }
                        ast::Pat::Con(p, args, span) => {
                            let (tag, arity) = self.ctor_for_type(&info, p, *span)?;
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
                            let mut a = r.clone();
                            self.mark_consumed(&mut a, &col0);
                            let mut pats = args.clone();
                            pats.extend(r.pats[1..].iter().cloned());
                            a.pats = pats;
                            ctor_rows[tag].push(a);
                        }
                        other => {
                            return Err(Error::at(
                                other.span(),
                                format!("pattern does not match type {ty0}"),
                            ));
                        }
                    }
                }
                let mut alts = Vec::new();
                let mut alt_maps = Vec::new();
                for (name, tag) in info.ctors.clone() {
                    let (arity, field_tys) =
                        self.ctor_field_tys(&self.ctor_of_type(&info, &name)?, &ty_args)?;
                    let fields: Vec<LocalId> = (0..arity).map(|_| self.fresh_local()).collect();
                    let mut branch = std::mem::take(&mut ctor_rows[tag]);
                    for r in &var_rows {
                        // A var or wildcard row matches this constructor
                        // too; its fields are unconstrained.
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
                            format!("non-exhaustive patterns: constructor {name} is not covered"),
                        ));
                    }
                    let mut branch_cols: Vec<ECore> =
                        fields.iter().map(|l| ECore::Var(*l)).collect();
                    branch_cols.extend(cols[1..].iter().cloned());
                    let mut branch_tys: Vec<Ty> = field_tys.iter().map(|t| wrap(t)).collect();
                    branch_tys.extend(types[1..].iter().cloned());
                    let (core, m) = self.isolate(|s| {
                        s.compile_rows(
                            branch,
                            branch_cols,
                            branch_tys,
                            expected,
                            ctx_span,
                            covered,
                        )
                    })?;
                    alt_maps.push(m);
                    alts.push((CPat::Con { tag, fields }, core));
                }
                self.merge_branches(alt_maps);
                Ok(ECore::Case { scrut: Box::new(col0), alts })
            }
            Ty::Tuple(items) => {
                // A tuple is a boxed product: one block, tag 0, elements
                // as words in order. Destructure through the same
                // constructor alternative machinery.
                let n = items.len();
                let mut sub_rows: Vec<Row> = Vec::new();
                let mut var_rows: Vec<Row> = Vec::new();
                for mut r in rows.into_iter() {
                    let p0 = r.pats[0].clone();
                    match &p0 {
                        ast::Pat::Var(name, _) => {
                            if let ECore::Var(l) = &col0 {
                                self.bind_local(
                                    &mut r.env,
                                    &name.text,
                                    *l,
                                    Scheme::mono(ty0.clone()),
                                    None,
                                    name.span,
                                )?;
                            }
                            let mut a = r.clone();
                            a.pats = r.pats[1..].to_vec();
                            var_rows.push(a);
                        }
                        ast::Pat::Wild(_) => {
                            let mut a = r.clone();
                            a.pats = r.pats[1..].to_vec();
                            var_rows.push(a);
                        }
                        ast::Pat::Tuple(pats, span) => {
                            if pats.len() != n {
                                return Err(Error::at(
                                    *span,
                                    format!(
                                        "tuple pattern has {} components but the tuple type has {n}",
                                        pats.len()
                                    ),
                                ));
                            }
                            self.mark_consumed(&mut r, &col0);
                            let mut a = r.clone();
                            let mut ps = pats.clone();
                            ps.extend(r.pats[1..].iter().cloned());
                            a.pats = ps;
                            sub_rows.push(a);
                        }
                        other => {
                            return Err(Error::at(
                                other.span(),
                                format!("pattern does not match type {ty0}"),
                            ));
                        }
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
                let mut branch_cols: Vec<ECore> = fields.iter().map(|l| ECore::Var(*l)).collect();
                branch_cols.extend(cols[1..].iter().cloned());
                let mut branch_tys: Vec<Ty> = items.iter().map(|t| wrap(t)).collect();
                branch_tys.extend(types[1..].iter().cloned());
                let core = self.compile_rows(
                    branch,
                    branch_cols,
                    branch_tys,
                    expected,
                    ctx_span,
                    covered,
                )?;
                Ok(ECore::Case {
                    scrut: Box::new(col0),
                    alts: vec![(CPat::Con { tag: 0, fields }, core)],
                })
            }
            Ty::Param(p) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a value of abstract type {p}"),
            )),
            Ty::Fun(a, _, _, b) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a function value ({a} -> {b})"),
            )),
            Ty::Thunk { inner, .. } => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a thunk (Thunk _ {inner}); force it first"),
            )),
            Ty::Lazy { inner, .. } => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a memo cell (Lazy _ _ {inner}); demand it first"),
            )),
            Ty::Ref(_, _) | Ty::Mut(_, _) => Err(Error::at(
                ctx_span,
                "cannot pattern match on a borrow of a borrow in the bootstrap slice; copyOut it first",
            )),
            Ty::Meta(_) => Err(Error::at(
                ctx_span,
                "cannot determine the type of the scrutinee; add a type annotation",
            )),
        }
    }

    /// A non-variable pattern in a column consumes the column's value on
    /// its row's path (usage counting; ownership and usage design,
    /// section 2).
    fn mark_consumed(&self, r: &mut Row, col: &ECore) {
        if let ECore::Var(l) = col {
            if self.obligations.contains_key(l) {
                r.consumed.push(*l);
            }
        }
    }

    fn consume_column(
        &mut self,
        mut rows: Vec<Row>,
        cols: Vec<ECore>,
        types: Vec<Ty>,
        expected: &Ty,
        ctx_span: ast::Span,
        covered: bool,
    ) -> Result<ECore> {
        for r in rows.iter_mut() {
            let p0 = r.pats[0].clone();
            if let ast::Pat::Var(n, _) = &p0 {
                if let ECore::Var(l) = &cols[0] {
                    let t = self.uni.zonk(&types[0]);
                    self.bind_local(&mut r.env, &n.text, *l, Scheme::mono(t), None, n.span)?;
                }
            }
            r.pats = r.pats[1..].to_vec();
        }
        self.compile_rows(
            rows,
            cols[1..].to_vec(),
            types[1..].to_vec(),
            expected,
            ctx_span,
            covered,
        )
    }

    /// The fallback when a match runs out at runtime. This is never a
    /// value: the pattern part is normally covered (the checker rejects
    /// non-exhaustive matrices), and a failed guard on every applicable row
    /// means the coverage proof was incomplete. Returning a word here would
    /// crash on boxed result types (review P0), so it is a defined runtime
    /// fault in every case.
    fn run_out(&self, _covered: bool) -> Result<ECore> {
        Ok(ECore::Fault)
    }

    /// Desugar the surface pattern sugar: list patterns become nested
    /// prelude `Cons`/`Nil` constructor patterns, and record patterns
    /// become constructor patterns with the fields reordered into
    /// declaration order.
    fn expand_pat(&self, p: &ast::Pat) -> Result<ast::Pat> {
        Ok(match p {
            ast::Pat::List(items, span) => {
                let mut out = ast::Pat::Con(
                    ast::ConPath {
                        segments: vec![
                            ast::Name::new(self.prelude_module.clone(), *span),
                            ast::Name::new("Nil", *span),
                        ],
                    },
                    Vec::new(),
                    *span,
                );
                for item in items.iter().rev() {
                    out = ast::Pat::Con(
                        ast::ConPath {
                            segments: vec![
                                ast::Name::new(self.prelude_module.clone(), *span),
                                ast::Name::new("Cons", *span),
                            ],
                        },
                        vec![self.expand_pat(item)?, out],
                        *span,
                    );
                }
                out
            }
            ast::Pat::Record(path, fields, span) => {
                let parts = path.parts();
                let info = self.lookup_type(&parts).map_err(|_| {
                    Error::at(*span, format!("unknown type {}", parts.join(".")))
                })?;
                let names = info.fields.clone().ok_or_else(|| {
                    Error::at(
                        *span,
                        format!(
                            "{} is not a record; `{} {{ ... }}` patterns need a record type",
                            parts.join("."),
                            parts.join(".")
                        ),
                    )
                })?;
                let mut assigned: HashMap<&str, &ast::Pat> = HashMap::new();
                for (n, pat) in fields {
                    if assigned.insert(n.text.as_str(), pat).is_some() {
                        return Err(Error::at(
                            n.span,
                            format!("duplicate record field {}", n.text),
                        ));
                    }
                }
                for (n, _) in fields {
                    if !names.contains(&n.text) {
                        return Err(Error::at(
                            n.span,
                            format!("unknown record field {}", n.text),
                        ));
                    }
                }
                if let Some(f) = names.iter().find(|f| !assigned.contains_key(f.as_str())) {
                    return Err(Error::at(*span, format!("record pattern is missing field {f}")));
                }
                let mut args = Vec::new();
                for f in &names {
                    args.push(self.expand_pat(assigned[f.as_str()])?);
                }
                ast::Pat::Con(path.clone(), args, *span)
            }
            ast::Pat::As(n, inner, span) => {
                ast::Pat::As(n.clone(), Box::new(self.expand_pat(inner)?), *span)
            }
            ast::Pat::Tuple(items, span) => ast::Pat::Tuple(
                items
                    .iter()
                    .map(|i| self.expand_pat(i))
                    .collect::<Result<Vec<_>>>()?,
                *span,
            ),
            ast::Pat::Con(path, args, span) => ast::Pat::Con(
                path.clone(),
                args.iter()
                    .map(|i| self.expand_pat(i))
                    .collect::<Result<Vec<_>>>()?,
                *span,
            ),
            other => other.clone(),
        })
    }

    /// The constructor of `info`'s own type, resolved in the type's module
    /// so a user constructor of the same name cannot shadow it.
    fn ctor_of_type(&self, info: &TypeKind, name: &str) -> Result<ValueInfo> {
        self.qualified
            .get(&(info.module.clone(), name.to_string()))
            .cloned()
            .ok_or_else(|| Error::plain(format!("unknown constructor {name}")))
    }

    /// Resolve a constructor pattern against the type being matched. The
    /// constructor must belong to that type; before this check, a foreign
    /// constructor was silently matched against a same-position tag of the
    /// scrutinee's type (review P0).
    fn ctor_for_type(
        &self,
        info: &TypeKind,
        p: &ast::ConPath,
        span: ast::Span,
    ) -> Result<(usize, usize)> {
        let v = self.resolve_con(p, span)?;
        match &v.kind {
            ValueKind::Ctor { tag, arity, .. } => {
                if v.module != info.module
                    || !info.ctors.iter().any(|(n, t)| n == &v.name && *t == *tag)
                {
                    return Err(Error::at(
                        span,
                        format!(
                            "constructor {} does not belong to type {}",
                            v.name, info.name
                        ),
                    ));
                }
                Ok((*tag, *arity))
            }
            _ => Err(Error::at(span, format!("{} is not a constructor", v.name))),
        }
    }

    /// A constructor's arity and field types at the type arguments
    /// `ty_args` of the type being matched.
    fn ctor_field_tys(&self, v: &ValueInfo, ty_args: &[Ty]) -> Result<(usize, Vec<Ty>)> {
        match &v.kind {
            ValueKind::Ctor { arity, field_tys, ty_params, .. } => {
                let mut map = HashMap::new();
                for (p, a) in ty_params.iter().zip(ty_args) {
                    map.insert(p.clone(), a.clone());
                }
                Ok((
                    *arity,
                    field_tys.iter().map(|t| Self::subst_ty(t, &map)).collect(),
                ))
            }
            _ => Err(Error::plain(format!("{} is not a constructor", v.name))),
        }
    }

    /// Drop column 0 of one row.
    fn drop_row_col(&self, r: &Row) -> Row {
        let mut a = r.clone();
        a.pats = r.pats[1..].to_vec();
        a
    }

    /// Does the pattern part of this row matrix cover its scrutinee space,
    /// ignoring guards? Integer literals cover an unbounded domain, so an
    /// integer column is covered only with a catch-all row.
    fn matrix_covered(&self, rows: &[Row], types: &[Ty]) -> Result<bool> {
        if rows.is_empty() {
            return Ok(false);
        }
        if types.is_empty() {
            return Ok(true);
        }
        let mut rows = rows.to_vec();
        // As-patterns hide nothing about coverage.
        for r in rows.iter_mut() {
            while let ast::Pat::As(_, inner, _) = &r.pats[0] {
                r.pats[0] = (**inner).clone();
            }
        }
        if rows
            .iter()
            .all(|r| matches!(r.pats[0], ast::Pat::Var(..) | ast::Pat::Wild(_)))
        {
            let rest: Vec<Row> = rows.iter().map(|r| self.drop_row_col(r)).collect();
            return self.matrix_covered(&rest, &types[1..]);
        }
        let ty0 = self.uni.zonk(&types[0]);
        // A borrowed column covers its referent's pattern space; nested
        // field columns borrow too (ownership and usage design, section 4).
        let (borrow, inner_ty) = match &ty0 {
            Ty::Ref(r, inner) => (Some((r.clone(), false)), (**inner).clone()),
            Ty::Mut(r, inner) => (Some((r.clone(), true)), (**inner).clone()),
            other => (None, other.clone()),
        };
        let wrap = |t: &Ty| -> Ty {
            match &borrow {
                Some((r, true)) => Ty::Mut(r.clone(), Box::new(t.clone())),
                Some((r, false)) => Ty::Ref(r.clone(), Box::new(t.clone())),
                None => t.clone(),
            }
        };
        match &inner_ty {
            Ty::Unit => {
                if rows.iter().any(|r| {
                    !matches!(r.pats[0], ast::Pat::Unit(_) | ast::Pat::Var(..) | ast::Pat::Wild(_))
                }) {
                    return Ok(false);
                }
                let rest: Vec<Row> = rows.iter().map(|r| self.drop_row_col(r)).collect();
                self.matrix_covered(&rest, &types[1..])
            }
            Ty::Int | Ty::Nat | Ty::Prim(_) => {
                // Integer columns cover an unbounded domain (machine
                // scalars included), so a catch-all row is required.
                let mut literals: Vec<String> = Vec::new();
                let mut defaults: Vec<Row> = Vec::new();
                let mut lit_rows: Vec<(String, Row)> = Vec::new();
                for r in &rows {
                    match &r.pats[0] {
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => {
                            defaults.push(self.drop_row_col(r));
                        }
                        ast::Pat::Int(v, _) => {
                            if !literals.contains(v) {
                                literals.push(v.clone());
                            }
                            lit_rows.push((v.clone(), self.drop_row_col(r)));
                        }
                        _ => return Ok(false),
                    }
                }
                if defaults.is_empty() {
                    return Ok(false);
                }
                if !self.matrix_covered(&defaults, &types[1..])? {
                    return Ok(false);
                }
                for v in literals {
                    let mut branch = defaults.clone();
                    branch.extend(
                        lit_rows.iter().filter(|(x, _)| *x == v).map(|(_, r)| r.clone()),
                    );
                    if !self.matrix_covered(&branch, &types[1..])? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Ty::Bool => {
                let mut branches = (Vec::new(), Vec::new());
                for r in &rows {
                    let tag = match &r.pats[0] {
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => None,
                        ast::Pat::Con(p, args, _) if args.is_empty() => {
                            match p.parts().as_slice() {
                                ["True"] => Some(true),
                                ["False"] => Some(false),
                                _ => return Ok(false),
                            }
                        }
                        _ => return Ok(false),
                    };
                    match tag {
                        Some(true) => branches.0.push(self.drop_row_col(r)),
                        Some(false) => branches.1.push(self.drop_row_col(r)),
                        None => {
                            branches.0.push(self.drop_row_col(r));
                            branches.1.push(self.drop_row_col(r));
                        }
                    }
                }
                Ok(self.matrix_covered(&branches.0, &types[1..])?
                    && self.matrix_covered(&branches.1, &types[1..])?)
            }
            Ty::Adt(_, ty_args) => {
                let info = match self.lookup_type(&[type_name_str(&inner_ty)]) {
                    Ok(i) => i,
                    Err(_) => return Ok(false),
                };
                let mut ctor_rows: Vec<Vec<Row>> =
                    (0..info.ctors.len()).map(|_| Vec::new()).collect();
                let mut var_rows: Vec<(ast::Span, Row)> = Vec::new();
                for r in &rows {
                    match &r.pats[0] {
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => {
                            var_rows.push((r.pats[0].span(), self.drop_row_col(r)));
                        }
                        ast::Pat::Con(p, args, _) => {
                            // A constructor from another type means this
                            // prover cannot claim coverage.
                            let (tag, arity) =
                                match self.ctor_for_type(&info, p, r.pats[0].span()) {
                                    Ok(t) => t,
                                    Err(_) => return Ok(false),
                                };
                            if args.len() != arity || tag >= ctor_rows.len() {
                                return Ok(false);
                            }
                            let mut a = self.drop_row_col(r);
                            let mut pats = args.clone();
                            pats.append(&mut a.pats);
                            a.pats = pats;
                            ctor_rows[tag].push(a);
                        }
                        _ => return Ok(false),
                    }
                }
                for (name, tag) in &info.ctors {
                    let (arity, field_tys) = match self
                        .ctor_of_type(&info, name)
                        .and_then(|v| self.ctor_field_tys(&v, ty_args))
                    {
                        Ok(x) => x,
                        Err(_) => return Ok(false),
                    };
                    let mut branch = std::mem::take(&mut ctor_rows[*tag]);
                    for (span, r) in &var_rows {
                        let mut a = r.clone();
                        let mut pats = vec![ast::Pat::Wild(*span); arity];
                        pats.append(&mut a.pats);
                        a.pats = pats;
                        branch.push(a);
                    }
                    let mut tys: Vec<Ty> = field_tys.iter().map(|t| wrap(t)).collect();
                    tys.extend(types[1..].iter().cloned());
                    if !self.matrix_covered(&branch, &tys)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Ty::Tuple(items) => {
                let n = items.len();
                let mut sub_rows: Vec<Row> = Vec::new();
                for r in &rows {
                    match &r.pats[0] {
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => {
                            let span = r.pats[0].span();
                            let mut a = self.drop_row_col(r);
                            let mut pats = vec![ast::Pat::Wild(span); n];
                            pats.append(&mut a.pats);
                            a.pats = pats;
                            sub_rows.push(a);
                        }
                        ast::Pat::Tuple(pats, _) if pats.len() == n => {
                            let mut a = self.drop_row_col(r);
                            let mut ps = pats.clone();
                            ps.append(&mut a.pats);
                            a.pats = ps;
                            sub_rows.push(a);
                        }
                        _ => return Ok(false),
                    }
                }
                let mut tys: Vec<Ty> = items.iter().map(|t| wrap(t)).collect();
                tys.extend(types[1..].iter().cloned());
                self.matrix_covered(&sub_rows, &tys)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Clone)]
struct Row {
    pats: Vec<ast::Pat>,
    guard: Option<ast::Term>,
    body: ast::Term,
    env: Env,
    /// Source position among the match's rows, for fall-through order.
    idx: usize,
    /// Column locals this row's non-variable patterns consume (usage
    /// counting notes one use per consumed column on the row's path,
    /// ownership and usage design, section 2).
    consumed: Vec<LocalId>,
}

/// A row's patterns must bind every variable at most once; duplicate
/// binders previously shadowed silently (review P2).
fn check_row_binders(pats: &[ast::Pat]) -> Result<()> {
    fn walk(p: &ast::Pat, seen: &mut Vec<String>) -> Result<()> {
        match p {
            ast::Pat::Var(n, span) | ast::Pat::As(n, _, span) => {
                if seen.contains(&n.text) {
                    return Err(Error::at(
                        *span,
                        format!("variable {} is bound twice in the same pattern", n.text),
                    ));
                }
                seen.push(n.text.clone());
                if let ast::Pat::As(_, inner, _) = p {
                    walk(inner, seen)?;
                }
            }
            ast::Pat::Con(_, args, _) | ast::Pat::Tuple(args, _) | ast::Pat::List(args, _) => {
                for a in args {
                    walk(a, seen)?;
                }
            }
            ast::Pat::Record(_, fields, _) => {
                for (_, a) in fields {
                    walk(a, seen)?;
                }
            }
            ast::Pat::Wild(_) | ast::Pat::Unit(_) | ast::Pat::Int(_, _) => {}
        }
        Ok(())
    }
    let mut seen = Vec::new();
    for p in pats {
        walk(p, &mut seen)?;
    }
    Ok(())
}

fn type_name_str(t: &Ty) -> &str {
    match t {
        Ty::Adt(n, _) => n,
        _ => "",
    }
}

/// A chain of prefix `-` over an integer literal folds into one literal
/// (primitive types specification, section 7: the prefix folds and is
/// range-checked with the literal).
fn neg_fold(t: &ast::Term) -> Option<String> {
    match t {
        ast::Term::Int(v, _) => Some(ast::negate_literal(v)),
        ast::Term::Neg(inner, _) => neg_fold(inner).map(|v| ast::negate_literal(&v)),
        _ => None,
    }
}

/// Parse a decimal digit string (optional leading `-`) into an `i128`;
/// `None` when it does not fit (then no machine scalar can hold it).
fn literal_i128(digits: &str) -> Option<i128> {
    digits.parse::<i128>().ok()
}

/// `dec = dec / d` on a canonical decimal numeral, exactly (the caller
/// guarantees divisibility).
fn dec_div_small(dec: &mut String, d: u32) {
    let mut carry = 0u32;
    let mut out = String::with_capacity(dec.len());
    for c in dec.chars() {
        let x = carry * 10 + c.to_digit(10).unwrap();
        out.push(char::from_digit(x / d, 10).unwrap());
        carry = x % d;
    }
    *dec = out.trim_start_matches('0').to_string();
    if dec.is_empty() {
        dec.push('0');
    }
}

/// Is the non-negative decimal numeral `mag` exactly representable in an
/// IEEE binary float with significand precision `p` and maximum exponent
/// `emax`? An integer `odd * 2^a` (odd part `odd`) is exactly
/// representable iff `odd` fits `p` bits and its binary exponent is at
/// most `emax`.
fn float_exact_digits(mag: &str, p: u32, emax: i32) -> bool {
    let mut n = mag.trim_start_matches('0').to_string();
    if n.is_empty() {
        return true; // zero
    }
    let mut a: i32 = 0;
    while n.ends_with('0') || n.ends_with('2') || n.ends_with('4') || n.ends_with('6')
        || n.ends_with('8')
    {
        dec_div_small(&mut n, 2);
        a += 1;
    }
    let mut b = 0;
    while n.ends_with('0') || n.ends_with('5') {
        dec_div_small(&mut n, 5);
        b += 1;
    }
    // The odd part is `5^b` times what remains.
    for _ in 0..b {
        let mut carry = 0u32;
        let mut out = String::with_capacity(n.len() + 1);
        for c in n.chars() {
            let x = c.to_digit(10).unwrap() * 5 + carry;
            out.push(char::from_digit(x % 10, 10).unwrap());
            carry = x / 10;
        }
        while carry > 0 {
            out.push(char::from_digit(carry % 10, 10).unwrap());
            carry /= 10;
        }
        n = out.chars().rev().collect();
    }
    // `n` is the odd part: it must fit the significand, and the binary
    // exponent `floor(log2 n) + a` must fit.
    let limit: u128 = (1u128 << p) - 1;
    let Ok(odd) = n.parse::<u128>() else {
        return false;
    };
    if odd > limit {
        return false;
    }
    let bits = (128 - odd.leading_zeros()) as i32;
    bits - 1 + a <= emax
}

/// The condition `col == lit` for one integer literal-pattern branch.
/// Machine scalars compare canonical words with `eqI64` (word equality is
/// value equality); mathematical `Nat`/`Int` compare through the runtime's
/// `natCmp`/`intCmp` immediate (primitive types specification, section 7).
fn literal_eq_cond(ty: &Ty, col: ECore, lit: ECore) -> ECore {
    let i64eq = |a: ECore, b: ECore| {
        app_spine(
            ECore::Prim(PrimOp::Cmp { ty: Scalar::I64, op: CmpOp::Eq }),
            vec![a, b],
        )
    };
    match ty {
        Ty::Int => i64eq(
            app_spine(ECore::Prim(PrimOp::Int(IntOp::Cmp)), vec![col, lit]),
            ECore::Int(0),
        ),
        Ty::Nat => i64eq(
            app_spine(ECore::Prim(PrimOp::Nat(NatOp::Cmp)), vec![col, lit]),
            ECore::Int(0),
        ),
        _ => i64eq(col, lit),
    }
}

/// The code point of a character literal (section 6: one Unicode scalar
/// value; the lexer applies the escape rules and rejects non-scalar
/// escapes).
fn char_literal_word(s: &str, span: ast::Span) -> Result<i64> {
    match s.chars().collect::<Vec<_>>().as_slice() {
        [c] => Ok(*c as u32 as i64),
        _ => Err(Error::at(
            span,
            "a character literal must be a single Unicode scalar value",
        )),
    }
}

fn wrap_lets(binds: Vec<(LocalId, ECore)>, body: ECore) -> ECore {
    let mut out = body;
    for (l, v) in binds.into_iter().rev() {
        out = ECore::Let(l, Box::new(v), Box::new(out));
    }
    out
}

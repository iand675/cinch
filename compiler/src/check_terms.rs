// Included into `check.rs` at module scope: term elaboration, the flat
// pattern-match compiler, and local declarations. Kept separate for
// readability only.

/// The status of a proof obligation (core specification section 4):
/// checked static evidence may be used wherever its assumptions hold; a
/// disproved obligation, unsupported reasoning, and a proof-search timeout
/// are never proofs; an unresolved obligation waits for an explicit
/// runtime decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ObligStatus {
    Checked,
    Disproved,
    Unsupported,
    Timeout,
    Unresolved,
}

/// A partially evaluated index expression: numerals reduce, everything
/// else is stuck (or the evaluator ran out of fuel).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stuck {
    Val(i128),
    Stuck,
    Timeout,
}

/// Which construct's local block [`Checker::elab_locals`] is elaborating:
/// the `local_block` grammar is shared by `let`/`where` and by the `do`
/// statement's `let` group, with different disciplines.
#[derive(Clone, Copy, PartialEq, Eq)]
enum LocalForm {
    /// `let b1; ...; bn in e` and `where` blocks (surface grammar local
    /// amendments section 9): the group replaces nested single `let`s.
    Plain,
    /// A `let` statement in `do` (the surface EBNF `do_statement =
    /// "let", local_block`): every strict binding is pure and strict in
    /// either interpretation (its interpretation does not sequence
    /// effects), one group never binds a name twice, and under a handled
    /// computation only simple value bindings sequence (decision D24's
    /// rule for deferred bodies).
    DoStmt {
        /// Whether the block sits under a handled computation (the CPS
        /// elaboration is active).
        handled: bool,
    },
}

/// A constructor's fields and result with the matched type's arguments
/// substituted in.
struct CtorParts {
    /// Field binder name, substituted type, erasure flag.
    fields: Vec<(String, Ty, bool)>,
    /// Constructor-local index binders still to solve.
    local_idx: Vec<IxParam>,
    /// The result's arguments.
    res_args: Vec<Ty>,
}

/// Whether a constructor can build the type being matched.
enum CtorMatch {
    Possible(CtorParts),
    /// The indices refute it: this constructor is impossible here.
    Refuted,
    /// The index equations are stuck on rigid indices.
    Unknown(String),
}

/// Is this type argument fully ground: a numeral after normalization?
fn idx_ground(t: &Ty) -> bool {
    match t {
        Ty::Idx(it) => matches!(it.normalize(), ITerm::Num(_)),
        Ty::Prp(p) => match p.normalize() {
            PTerm::Var(_) | PTerm::MVar(_) => false,
            PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
                matches!(a.normalize(), ITerm::Num(_)) && matches!(b.normalize(), ITerm::Num(_))
            }
            PTerm::Pred(_, a) => matches!(a.normalize(), ITerm::Num(_)),
            PTerm::Not(q) => idx_ground(&Ty::Prp(q)),
        },
        _ => true,
    }
}

/// Rename the index-level variables of a type (a constructor's field
/// binders become the pattern's names).
fn subst_idx_names(t: &Ty, map: &HashMap<String, String>) -> Ty {
    let mut out = t.clone();
    for (from, to) in map {
        out = out.subst_idx(from, &ITerm::Var(to.clone()));
    }
    out
}

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
    /// `copyOut`, `borrow`, `mutBorrow`, `refSet`, `ignore`)? A user
    /// definition of the same name shadows the builtin (decision D2).
    fn is_builtin_form(&self, env: &Env, name: &str) -> bool {
        matches!(name, "delay" | "force" | "demand" | "copyOut" | "borrow" | "mutBorrow" | "refSet" | "decide" | "ignore" | "unrefine")
            && env.lookup(name).is_none()
            && self.lookup_value(None, name).is_err()
    }

    /// The structured-concurrency builtin behind an application head
    /// (`withScope`, `spawn`, `joinAllOrCancel`), when it is unshadowed:
    /// their application elaboration is [`Checker::infer_conc_app`]. The
    /// `Io r` operations are ordinary builtin values.
    fn conc_builtin(&self, env: &Env, f: &ast::Term) -> Option<ValueInfo> {
        let ast::Term::Var(p) = f else { return None };
        if !p.quals.is_empty() || env.lookup(&p.name.text).is_some() {
            return None;
        }
        let v = self.resolve_var(p, p.span()).ok()?;
        match v.kind {
            ValueKind::Prim(PrimOp::Conc(
                ConcOp::WithScope | ConcOp::Spawn | ConcOp::JoinN(_),
            )) => Some(v),
            _ => None,
        }
    }

    /// `withScope body`, `spawn scope body`, `joinAllOrCancel scope h1 ..
    /// hn` (concurrency specification, section 2): the scope body and task
    /// bodies run exactly once, so their captures are one-shot transfers;
    /// a scope's task handles and scope values stay inside the scope; and
    /// a spawn's body effects flow into the spawning scope's row.
    fn infer_conc_app(
        &mut self,
        env: &mut Env,
        v: &ValueInfo,
        args: &[ast::Term],
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let op = match v.kind {
            ValueKind::Prim(PrimOp::Conc(op)) => op,
            _ => unreachable!("conc_builtin only yields concurrency builtins"),
        };
        match op {
            ConcOp::WithScope => {
                if args.len() != 1 {
                    return Err(Error::at(
                        span,
                        "withScope takes exactly one argument: the scope body",
                    ));
                }
                let r = self.fresh_region();
                self.uni.note_flex_region(&r);
                // The scope region is rigid per call: handles of distinct
                // scopes never unify (section 2).
                let s = self.fresh_region();
                let e = self.uni.fresh_row();
                let a = self.uni.fresh();
                let body_ty = Ty::fun_mult(
                    conc_scope_ty(&r, &s),
                    Mult::Many,
                    e.clone(),
                    a.clone(),
                );
                let want = ClassApp {
                    class: "Io".to_string(),
                    args: vec![Ty::Region(r.clone())],
                };
                let hole = self.new_want(want, span, env);
                let outer = self.active_regions.clone();
                self.scope_frames.push(ScopeFrame {
                    region: s.clone(),
                    outer: outer.clone(),
                });
                let lam = matches!(&args[0], ast::Term::Lam(_, _, _));
                if lam {
                    self.oneshot_frames.push(OneshotFrame {
                        lam: self.lam_depth + 1,
                        task: false,
                        scope: s.clone(),
                        outer,
                    });
                }
                let kcore = self.check(env, &args[0], &body_ty);
                if lam {
                    self.oneshot_frames.pop();
                }
                self.scope_frames.pop();
                let kcore = kcore?;
                // No task handle or scope value leaves its scope
                // (concurrency specification, section 2).
                let res = self.uni.zonk(&a);
                let mut names = Vec::new();
                all_regions(&res, &mut names);
                if names.iter().any(|n| {
                    self.uni.resolve_region(n) == self.uni.resolve_region(&s)
                }) {
                    return Err(Error::at(
                        args[0].span(),
                        format!(
                            "region escape: a task handle or scope of region {s} leaves its `withScope` in {res} (concurrency specification, section 2)"
                        ),
                    ));
                }
                // Scope exit joins or cancels the children before the
                // result returns and may not complete: `{Diverge | e}`,
                // and the body's effects are the caller's (section 2).
                self.perform(&EffRow {
                    labels: vec![EffLabel::plain("Diverge")],
                    tail: RowTail::Closed,
                })?;
                self.perform(&e)?;
                Ok((
                    app_spine(
                        ECore::Prim(PrimOp::Conc(ConcOp::WithScope)),
                        vec![ECore::Hole(hole), kcore],
                    ),
                    a,
                ))
            }
            ConcOp::Spawn => {
                if args.len() != 2 {
                    return Err(Error::at(
                        span,
                        "spawn takes a scope and one task body",
                    ));
                }
                let r = self.fresh_region();
                self.uni.note_flex_region(&r);
                let s = self.fresh_region();
                self.uni.note_flex_region(&s);
                let sarg = self.check(env, &args[0], &conc_scope_ty(&r, &s))?;
                let s = self.uni.resolve_region(&s);
                let x = self.uni.fresh();
                let a = self.uni.fresh();
                let eb = self.uni.fresh_row();
                let body_ty = Ty::fun_mult(
                    Ty::Unit,
                    Mult::Many,
                    eb.clone(),
                    conc_result_ty(&x, &a),
                );
                // A literal task body runs exactly once on its thread; an
                // opaque closure keeps the discipline it was defined with.
                let lam = matches!(&args[1], ast::Term::Lam(_, _, _));
                let outer = self
                    .scope_frames
                    .iter()
                    .rev()
                    .find(|f| f.region == s)
                    .map(|f| f.outer.clone())
                    .unwrap_or_default();
                if lam {
                    self.oneshot_frames.push(OneshotFrame {
                        lam: self.lam_depth + 1,
                        task: true,
                        scope: s.clone(),
                        outer,
                    });
                }
                let bcore = self.check(env, &args[1], &body_ty);
                if lam {
                    self.oneshot_frames.pop();
                }
                let bcore = bcore?;
                // Spawning makes the body's effects the spawning scope's
                // responsibility (section 3).
                self.perform(&EffRow {
                    labels: vec![EffLabel::with_arg("Async", r.clone())],
                    tail: RowTail::Closed,
                })?;
                self.perform(&eb)?;
                Ok((
                    app_spine(
                        ECore::Prim(PrimOp::Conc(ConcOp::Spawn)),
                        vec![sarg, bcore],
                    ),
                    conc_task_ty(&s, &x, &a),
                ))
            }
            ConcOp::JoinN(_) => {
                if args.len() < 2 {
                    return Err(Error::at(
                        span,
                        "joinAllOrCancel takes a scope and at least one task handle",
                    ));
                }
                let r = self.fresh_region();
                self.uni.note_flex_region(&r);
                let s = self.fresh_region();
                self.uni.note_flex_region(&s);
                let mut cores = vec![self.check(env, &args[0], &conc_scope_ty(&r, &s))?];
                let s = self.uni.resolve_region(&s);
                let x = self.uni.fresh();
                let mut parts: Vec<Ty> = Vec::new();
                for h in &args[1..] {
                    let t = self.uni.fresh();
                    cores.push(self.check(env, h, &conc_task_ty(&s, &x, &t))?);
                    parts.push(t);
                }
                let payload = if parts.len() == 1 {
                    parts.into_iter().next().expect("one handle")
                } else {
                    Ty::Tuple(parts)
                };
                self.perform(&EffRow {
                    labels: vec![
                        EffLabel::with_arg("Async", r.clone()),
                        EffLabel::with_arg("Cancel", r),
                        EffLabel::plain("Diverge"),
                    ],
                    tail: RowTail::Closed,
                })?;
                Ok((
                    ECore::App(
                        Box::new(ECore::Prim(PrimOp::Conc(ConcOp::JoinN(cores.len() - 1)))),
                        cores,
                    ),
                    conc_result_ty(&x, &payload),
                ))
            }
            ConcOp::IoWrite | ConcOp::IoRead | ConcOp::IoSleep | ConcOp::PollCancel => {
                unreachable!("the Io r operations take the ordinary application path")
            }
        }
    }

    /// A collection literal (surface grammar local amendments section 10):
    /// `[e1, ..., en]` desugars to `from-list` at the expected type's
    /// `FromList c e` instance over the `Cons` spine of its elements, and
    /// `{k => v}` to `from-map` over a `List (k, v)` of pairs at the
    /// `FromMap m k v` instance (`items` carries the flattened key/value
    /// terms for a map). `expected` is the check position's container
    /// type; in inference position the container is a fresh type the
    /// instance solve defaults to `List`/`Map` (decision D142). The call
    /// is a `LitFold` boundary: the fold pass reifies stage-known
    /// elements to a constant value, and the marker lowers to the runtime
    /// class call otherwise.
    fn elab_collection(
        &mut self,
        env: &mut Env,
        items: &[ast::Term],
        map: bool,
        expected: Option<&Ty>,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let prelude = self.prelude_module.clone();
        let method = if map { "from-map" } else { "from-list" };
        let minfo = self
            .lookup_value(Some(&prelude), method)
            .map_err(|e| Error::at(span, e.message))?;
        let (mcore, mty) = self.value_use(&minfo, span, env)?;
        let mut cores: Vec<ECore> = Vec::new();
        let elem = if map {
            if items.len() % 2 != 0 {
                return Err(Error::plain(
                    "internal error: a map literal without key/value pairs",
                ));
            }
            let k = self.uni.fresh();
            let v = self.uni.fresh();
            let gid = self.tuple_ctor(2, span)?;
            for kv in items.chunks(2) {
                let kc = self.check(env, &kv[0], &k)?;
                let vc = self.check(env, &kv[1], &v)?;
                cores.push(app_spine(ECore::Ctor(gid.clone()), vec![kc, vc]));
            }
            Ty::Tuple(vec![k, v])
        } else {
            let e = self.uni.fresh();
            for it in items {
                cores.push(self.check(env, it, &e)?);
            }
            e
        };
        let list_ty = Ty::Adt("List".into(), vec![elem.clone()]);
        let nil = self
            .lookup_value(Some(&prelude), "Nil")
            .map_err(|e| Error::at(span, e.message))?;
        let cons = self
            .lookup_value(Some(&prelude), "Cons")
            .map_err(|e| Error::at(span, e.message))?;
        let (mut core, nil_ty) = self.value_use(&nil, span, env)?;
        self.uni
            .unify(&nil_ty, &list_ty)
            .map_err(|m| Error::at(span, m))?;
        for ic in cores.into_iter().rev() {
            let (cons_core, cons_ty) = self.value_use(&cons, span, env)?;
            let want = Ty::funs(&[elem.clone(), list_ty.clone()], list_ty.clone());
            self.uni
                .unify(&cons_ty, &want)
                .map_err(|m| Error::at(span, m))?;
            core = app_spine(cons_core, vec![ic, core]);
        }
        let result = match expected {
            Some(t) => t.clone(),
            None => self.uni.fresh(),
        };
        let want = Ty::funs(&[list_ty], result.clone());
        self.uni
            .unify(&mty, &want)
            .map_err(|m| Error::at(span, m))?;
        let idx = self.literal_folds.len();
        self.literal_folds.push(LitFold {
            unit: String::new(),
            site: format!("{}:{}", span.line, span.col),
            kind: if map { "map" } else { "list" }.to_string(),
            folded: false,
        });
        Ok((
            ECore::LitFold {
                idx,
                body: Box::new(app_spine(mcore, vec![core])),
            },
            result,
        ))
    }

    /// A range literal (the surface range form `[e1..e2]` / `[e1, e2
    /// .. e3]`): `from-list (enum-from-to e1 e2)` or `from-list
    /// (enum-from-then-to e1 e2 e3)` at the expected type's `FromList c
    /// e` instance, the enumeration stepping through the `Enum` class
    /// (Data.Enum). `expected` is the check position's container type;
    /// in inference position the container is a fresh type the instance
    /// solve defaults to `List` (decision D142). Like every collection
    /// construction the call is a `LitFold` boundary: the fold pass
    /// reifies a stage-known construction to a constant value, and the
    /// marker lowers to the runtime class calls otherwise.
    fn elab_range(
        &mut self,
        env: &mut Env,
        start: &ast::Term,
        step: Option<&ast::Term>,
        end: &ast::Term,
        expected: Option<&Ty>,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let prelude = self.prelude_module.clone();
        let method = if step.is_some() {
            "enum-from-then-to"
        } else {
            "enum-from-to"
        };
        let einfo = self
            .lookup_value(Some(&prelude), method)
            .map_err(|e| Error::at(span, e.message))?;
        let (ecore, ety) = self.value_use(&einfo, span, env)?;
        let a = self.uni.fresh();
        let sc = self.check(env, start, &a)?;
        let stc = match step {
            Some(t) => Some(self.check(env, t, &a)?),
            None => None,
        };
        let ec = self.check(env, end, &a)?;
        let list_ty = Ty::Adt("List".into(), vec![a.clone()]);
        let doms: Vec<Ty> = if step.is_some() {
            vec![a.clone(), a.clone(), a.clone()]
        } else {
            vec![a.clone(), a.clone()]
        };
        self.uni
            .unify(&ety, &Ty::funs(&doms, list_ty.clone()))
            .map_err(|m| Error::at(span, m))?;
        let mut args = vec![sc];
        args.extend(stc);
        args.push(ec);
        // The P19 expected-type boundary: `from-list` at the expected
        // type's `FromList c e` instance.
        let minfo = self
            .lookup_value(Some(&prelude), "from-list")
            .map_err(|e| Error::at(span, e.message))?;
        let (mcore, mty) = self.value_use(&minfo, span, env)?;
        let result = match expected {
            Some(t) => t.clone(),
            None => self.uni.fresh(),
        };
        let want = Ty::funs(&[list_ty], result.clone());
        self.uni
            .unify(&mty, &want)
            .map_err(|m| Error::at(span, m))?;
        let idx = self.literal_folds.len();
        self.literal_folds.push(LitFold {
            unit: String::new(),
            site: format!("{}:{}", span.line, span.col),
            kind: "range".to_string(),
            folded: false,
        });
        Ok((
            ECore::LitFold {
                idx,
                body: Box::new(app_spine(mcore, vec![app_spine(ecore, args)])),
            },
            result,
        ))
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
            ast::Term::Str(s, span) => {
                // `String` is a compile-time type in this slice (decision
                // D43): literals elaborate where compile-time code runs.
                // The runtime `String` type has shipped (built from
                // `Bytes` through `string-from-bytes`, Prelude.lang);
                // literal syntax for it is a surface future item.
                if !self.compile_time {
                    return Err(Error::at(
                        *span,
                        "string literals are compile-time only (build runtime strings from `Bytes` with `string-from-bytes`; literal syntax for the runtime `String` is a surface future item)",
                    ));
                }
                Ok((ECore::StrLit(s.clone()), Ty::Adt("String".into(), Vec::new())))
            }
            ast::Term::Char(s, span) => {
                Ok((ECore::Int(char_literal_word(s, *span)?), Ty::Prim(Scalar::Char)))
            }
            ast::Term::Sym(v, _) => {
                // `'name` interns at compile time and elaborates to the
                // constant of the same cell every equal spelling takes
                // (primitive types specification section 9).
                Ok((ECore::ConstSym(v.clone()), Ty::Sym))
            }
            ast::Term::Quote { body, span } => self.infer_quote(env, body, *span),
            ast::Term::QuoteDecls { decls, span } => self.infer_quote_decls(env, decls, *span),
            ast::Term::Splice { span, .. } => Err(Error::at(
                *span,
                "a splice `$( ... )` is only valid inside a quotation",
            )),
            ast::Term::SpliceHole { idx, span } => match self.hole_tys.get(*idx) {
                Some(t) => Ok((ECore::QuoteHole(*idx), t.clone())),
                None => Err(Error::at(
                    *span,
                    "internal error: a template hole outside template elaboration",
                )),
            },
            ast::Term::MacroCall { path, args, span } => {
                self.macro_call_value(env, path, args, *span)
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
                        let (t, _, _) = self.instantiate(&s);
                        let t = self.uni.zonk(&t);
                        // A bare use of a memo cell demands it and copies
                        // the cached value out; the binding name elaborates
                        // to `demand` followed by `copyOut` (laziness and
                        // reclamation specification, section 5). Memo
                        // cells cache `Share` values, so the copy always
                        // has structural evidence.
                        if let Ty::Lazy { eff, region, inner } = t {
                            self.note_use(l, p.span(), &[], &env.dicts)?;
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
                        self.note_use(l, p.span(), &path, &env.dicts)?;
                        return Ok((ECore::Var(l), t));
                    }
                }
                self.place_path.clear();
                // The bare-`pure` rule: a bare `pure` reaches the class
                // method through its constraint when one carries it.
                if p.quals.is_empty()
                    && p.name.text == "pure"
                    && env.lookup(&p.name.text).is_none()
                {
                    if let Some(v) = self.constraint_pure(env) {
                        return self.value_use(&v, p.span(), env);
                    }
                }
                let v = self.resolve_var(p, p.span())?;
                self.value_use(&v, p.span(), env)
            }
            ast::Term::Con(p) => {
                // A capitalized module parameter (`Seq :: ResultSequencing
                // ConfigError`) is an ordinary binding of the module scope:
                // it resolves to its binder before any constructor (module
                // parameters as module arguments).
                if let Some((l, ty)) = self.module_param_use(p) {
                    return Ok((ECore::Var(l), ty));
                }
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
                    if p.quals.is_empty() && self.is_provider_pure(env, &p.name.text) {
                        // `pure e` is the selected provider's return.
                        let seq = ast::Term::App(
                            Box::new(ast::Term::SeqOp(ast::SeqKind::Pure, *span)),
                            args.clone(),
                            *span,
                        );
                        return self.infer(env, &seq);
                    }
                    if env.lookup(&p.name.text).is_none() {
                        if let Ok(v) = self.resolve_var(p, p.span()) {
                            if matches!(v.kind, ValueKind::EffectOp { .. }) {
                                // An operation call outside a handler's
                                // dispatch reach: its label surfaces and
                                // the call is the unhandled fault path
                                // (decision D23).
                                return self.infer_unhandled_op(env, &v, args, *span);
                            }
                        }
                    }
                }
                // Structured concurrency (concurrency specification,
                // section 2): the one-shot body and scope confinement of
                // `withScope`, `spawn` and `joinAllOrCancel`.
                if let Some(v) = self.conc_builtin(env, f) {
                    return self.infer_conc_app(env, &v, args, *span);
                }
                // A constructor application is type-directed even in
                // inference position (erased evidence arguments never
                // reach runtime).
                if let ast::Term::Con(p) = &**f {
                    let fresh = self.uni.fresh();
                    return self.ctor_app(env, p, args, &fresh, *span);
                }
                let (mut core, mut ty) = self.infer(env, f)?;
                // A callee that writes memory at an unknown target (a
                // foreign call, `eval`, a mutating `Bytes` primitive) is an
                // opaque write that invalidates all mutable-location
                // evidence (evidence invalidation over mutable state).
                self.note_opaque_write_from(&core, *span);
                for a in args {
                    ty = self.uni.zonk(&ty);
                    let (arg_core, cod) = match ty {
                        Ty::Fun(dom, m, eff, cod) => {
                            self.perform(&eff)?;
                            let erased = matches!(self.uni.zonk(&dom), Ty::Prp(_) | Ty::Sort(_));
                            let c = self.check(env, a, &dom)?;
                            // Record writes after the argument, so the
                            // domain's region has unified with the passed
                            // borrow's region.
                            self.note_call_arg_writes(env, args, &dom, &eff, *span, &core);
                            // A task handle is consumed exactly once by
                            // its function: a sink that could drop it is
                            // refused (concurrency specification,
                            // section 2). Checked after the argument so
                            // an inferred domain is resolved.
                            self.check_handle_param(&dom, m, a.span())?;
                            (if erased { None } else { Some(c) }, *cod)
                        }
                        Ty::Pi { var, dom, eff, cod, .. } => {
                            self.perform(&eff)?;
                            let erased = matches!(self.uni.zonk(&dom), Ty::Prp(_) | Ty::Sort(_));
                            let c = self.check(env, a, &dom)?;
                            self.note_call_arg_writes(env, args, &dom, &eff, *span, &core);
                            let subst = self.idx_of_term(a, env).map_err(|e| {
                                Error::at(
                                    a.span(),
                                    format!(
                                        "{} (this function's result type mentions its argument)",
                                        e.message
                                    ),
                                )
                            })?;
                            (if erased { None } else { Some(c) }, cod.subst_idx(&var, &subst))
                        }
                        Ty::Meta(_) => {
                            let dom = self.uni.fresh();
                            let cod = self.uni.fresh();
                            let eff = self.uni.fresh_row();
                            self.uni
                                .unify(&ty, &Ty::fun_mult(dom.clone(), Mult::Flex, eff.clone(), cod.clone()))
                                .map_err(|m| Error::at(f.span(), m))?;
                            self.perform(&eff)?;
                            (Some(self.check(env, a, &dom)?), cod)
                        }
                        other => {
                            return Err(Error::at(
                                a.span(),
                                format!("cannot apply a value of type {other}"),
                            ));
                        }
                    };
                    if let Some(c) = arg_core {
                        core = app_spine(core, vec![c]);
                    }
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
                let (binds, group) = self.elab_locals(&mut env2, decls, LocalForm::Plain)?;
                let (bcore, bty) = self.infer(&mut env2, body)?;
                if let Some(g) = &group {
                    self.discharge_group(g, &bty, *span)?;
                }
                Ok((wrap_lets(binds, bcore), bty))
            }
            ast::Term::Letrec(decls, body, span) => {
                let mut env2 = env.clone();
                let (binds, group) = self.elab_letrec(&mut env2, decls)?;
                let (bcore, bty) = self.infer(&mut env2, body)?;
                if let Some(g) = &group {
                    self.discharge_group(g, &bty, *span)?;
                }
                Ok((wrap_lets(binds, bcore), bty))
            }
            ast::Term::If { cond, then, els, cond_attrs, then_attrs, else_attrs, span } => {
                let (tw, fw) =
                    self.if_weights(cond_attrs, then_attrs, else_attrs, *span)?;
                let ccore = self.check(env, cond, &Ty::Bool)?;
                let ((tcore, tty), tm) = self.isolate(|s| s.infer(env, then))?;
                let (ecore, em) = self.isolate(|s| s.check(env, els, &tty))?;
                self.merge_branches(vec![tm, em]);
                Ok((
                    ECore::If(
                        Box::new(ccore),
                        CBranch::weighted(tw, tcore),
                        CBranch::weighted(fw, ecore),
                    ),
                    tty,
                ))
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
            ast::Term::List(items, span) => self.elab_collection(env, items, false, None, *span),
            ast::Term::Map(kvs, span) => {
                let flat: Vec<ast::Term> = kvs
                    .iter()
                    .flat_map(|(k, v)| [k.clone(), v.clone()])
                    .collect();
                self.elab_collection(env, &flat, true, None, *span)
            }
            ast::Term::Range(start, step, end, span) => {
                self.elab_range(env, start, step.as_deref(), end, None, *span)
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
                // Erased fields occupy no runtime slot: the block stores
                // the others in declaration order (core specification
                // section 3).
                let erased = info.field_erased.get(idx).copied().unwrap_or(false);
                let runtime_idx =
                    info.field_erased[..idx].iter().filter(|e| !**e).count();
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
                if erased {
                    return Ok((ECore::Unit, field_ty));
                }
                if self
                    .codata_types
                    .contains(&(info.module.clone(), info.name.clone()))
                {
                    // A codata observation runs the field's closure (codata
                    // and guardedness design, section 2).
                    return Ok((
                        app_spine(
                            ECore::Proj(
                                Box::new(core),
                                ProjField {
                                    slot: runtime_idx,
                                    access: self.field_access_plan(
                                        &info.module,
                                        &info.name,
                                        runtime_idx,
                                        &field_ty,
                                    ),
                                },
                            ),
                            vec![ECore::Unit],
                        ),
                        field_ty,
                    ));
                }
                Ok((
                    ECore::Proj(
                        Box::new(core),
                        ProjField {
                            slot: runtime_idx,
                            access: self.field_access_plan(
                                &info.module,
                                &info.name,
                                runtime_idx,
                                &field_ty,
                            ),
                        },
                    ),
                    field_ty,
                ))
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
                let codata = self
                    .codata_types
                    .contains(&(info.module.clone(), info.name.clone()));
                // The constructor's arrows are in declaration order; field
                // expressions evaluate in source order (core specification:
                // record fields are left-to-right). A dependent field type
                // mentions earlier fields by name, and erased evidence
                // fields are checked but never stored (core specification
                // section 3).
                let (mut rest, _ctx, _val) = self.instantiate(&v.scheme);
                let ctor_core = ECore::Ctor(fn_id(&v.module, &v.name));
                let mut doms: Vec<(String, Ty, bool)> = Vec::new();
                for f in names.iter() {
                    rest = self.uni.zonk(&rest);
                    match rest {
                        Ty::Fun(a, _, _, b) => {
                            let erased = matches!(self.uni.zonk(&a), Ty::Prp(_) | Ty::Sort(_));
                            doms.push((f.clone(), *a, erased));
                            rest = *b;
                        }
                        Ty::Pi { var, dom, cod, .. } => {
                            let subst = match assigned.get(var.as_str()) {
                                Some(t) => self.idx_of_term(t, env)?,
                                None => ITerm::Var(var.clone()),
                            };
                            let erased = matches!(self.uni.zonk(&dom), Ty::Prp(_) | Ty::Sort(_));
                            doms.push((f.clone(), *dom, erased));
                            rest = cod.subst_idx(&var, &subst);
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
                        .find(|(f, _, _)| *f == n.text)
                        .map(|(_, d, _)| d.clone())
                        .expect("field membership checked above");
                    let c = if codata {
                        // A codata value is a record of delayed
                        // observations (design section 2): each value
                        // becomes a closure run at observation time, so it
                        // is a lazy position for the enclosing binders
                        // (ownership and usage design, section 2) and its
                        // performed effects surface at observation, where
                        // the declared type carries no effect row.
                        let saved = self.enter_perf();
                        self.lam_depth += 1;
                        let c = self.check(env, t, &dom);
                        self.lam_depth -= 1;
                        let c = c?;
                        let performed = self.exit_perf(saved);
                        self.uni
                            .unify_rows(&performed, &EffRow::pure())
                            .map_err(|_| {
                                Error::at(
                                    t.span(),
                                    "an observation value must be effect-free: it runs at observation time and its declared type carries no effect row; defer effectful or diverging computation in a `delay` (a `Thunk {e}` or `Later` observation)",
                                )
                            })?;
                        let unit = self.fresh_local();
                        ECore::Lam { params: vec![unit], body: Box::new(c) }
                    } else {
                        self.check(env, t, &dom)?
                    };
                    evaluated.insert(n.text.as_str(), c);
                }
                let mut arg_cores: Vec<ECore> = Vec::new();
                for (f, _, erased) in &doms {
                    let c = evaluated
                        .remove(f.as_str())
                        .expect("every declared field was assigned");
                    if !*erased {
                        arg_cores.push(c);
                    }
                }
                let core = if arg_cores.is_empty() {
                    ctor_core
                } else {
                    app_spine(ctor_core, arg_cores)
                };
                Ok((core, rest))
            }
            ast::Term::Do(stmts, span) => {
                let shape = self.result_shape(env, *span)?;
                let core = self.elab_do(env, stmts, &shape, None, *span)?;
                Ok((core, shape))
            }
            ast::Term::Handle(e, h, _) => self.handle_expr(env, e, h, None),
            ast::Term::Handler(h) => self.handler_value(env, h),
            ast::Term::With { name, value, body, span } => {
                let (vc, vty) = self.infer(env, value)?;
                let mut env2 = env.clone();
                let (l, bind, region) =
                    self.bind_capability(&mut env2, name, vc, vty, *span)?;
                let (bcore, bty) = self.infer(&mut env2, body)?;
                self.check_cap_escape(&region, &bty, *span)?;
                Ok((wrap_lets(vec![(l, bind)], bcore), bty))
            }
            ast::Term::CapApply { head, supplies, span } => {
                self.cap_apply(env, head, supplies, *span)
            }
            ast::Term::SeqOp(kind, span) => self.seq_method(env, *kind, *span),
            ast::Term::CpsWrap(_, span) => Err(Error::at(
                *span,
                "internal error: a continuation wrapper escaped handler elaboration",
            )),
        }
    }

    fn check(&mut self, env: &mut Env, term: &ast::Term, expected: &Ty) -> Result<ECore> {
        // A proposition is checked as an erased proof term (core
        // specification section 3).
        if let Ty::Prp(p) = self.uni.zonk(expected) {
            return self.check_proof(env, term, &p);
        }
        // Constructor applications are type-directed: the expected type
        // solves the constructor's index parameters before its arguments
        // are checked, and erased evidence arguments never reach runtime.
        match term {
            ast::Term::Con(p) => {
                let want = self.uni.zonk(expected);
                let (core, ty) = self.ctor_app(env, p, &[], &want, p.span())?;
                self.uni
                    .unify(&ty, &want)
                    .map_err(|m| Error::at(term.span(), m))?;
                return Ok(core);
            }
            ast::Term::App(f, args, span) => {
                if let ast::Term::Con(p) = &**f {
                    let want = self.uni.zonk(expected);
                    let (core, ty) = self.ctor_app(env, p, args, &want, *span)?;
                    self.uni
                        .unify(&ty, &want)
                        .map_err(|m| Error::at(term.span(), m))?;
                    return Ok(core);
                }
            }
            _ => {}
        }
        match term {
            ast::Term::CpsWrap(body, span) => {
                // A handler's alternative body: elaborate in the enclosing
                // continuation-passing context; its code produces the
                // computation's answer, not a value of `expected`.
                let cont = self.current_cont.last().cloned().ok_or_else(|| {
                    Error::at(*span, "internal error: continuation wrapper outside a continuation")
                })?;
                self.cps(env, body, &cont)
            }
            ast::Term::Lam(_, _, _) => self.check_lam(env, term, expected),
            ast::Term::If { cond, then, els, cond_attrs, then_attrs, else_attrs, span } => {
                let (tw, fw) =
                    self.if_weights(cond_attrs, then_attrs, else_attrs, *span)?;
                let ccore = self.check(env, cond, &Ty::Bool)?;
                let (tcore, tm) = self.isolate(|s| s.check(env, then, expected))?;
                let (ecore, em) = self.isolate(|s| s.check(env, els, expected))?;
                self.merge_branches(vec![tm, em]);
                Ok(ECore::If(
                    Box::new(ccore),
                    CBranch::weighted(tw, tcore),
                    CBranch::weighted(fw, ecore),
                ))
            }
            ast::Term::Let(decls, body, span) => {
                let mut env2 = env.clone();
                let (binds, group) = self.elab_locals(&mut env2, decls, LocalForm::Plain)?;
                let bcore = self.check(&mut env2, body, expected)?;
                if let Some(g) = &group {
                    self.discharge_group(g, expected, *span)?;
                }
                Ok(wrap_lets(binds, bcore))
            }
            ast::Term::Letrec(decls, body, span) => {
                let mut env2 = env.clone();
                let (binds, group) = self.elab_letrec(&mut env2, decls)?;
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
            ast::Term::List(items, span) => {
                let (core, _) = self.elab_collection(env, items, false, Some(expected), *span)?;
                Ok(core)
            }
            ast::Term::Map(kvs, span) => {
                let flat: Vec<ast::Term> = kvs
                    .iter()
                    .flat_map(|(k, v)| [k.clone(), v.clone()])
                    .collect();
                let (core, _) = self.elab_collection(env, &flat, true, Some(expected), *span)?;
                Ok(core)
            }
            ast::Term::Range(start, step, end, span) => {
                let (core, _) =
                    self.elab_range(env, start, step.as_deref(), end, Some(expected), *span)?;
                Ok(core)
            }
            ast::Term::Int(v, span) => self.lit_check(expected, v, *span),
            ast::Term::Quote { body, span } => {
                // Against a known `Code t`, check the template at `t`
                // directly (better errors than infer-and-unify).
                if let Ty::Adt(name, args) = self.uni.zonk(expected) {
                    if name == "Code" && args.len() == 1 {
                        return self.check_quote_template(env, body, &args[0], *span);
                    }
                }
                let (core, ty) = self.infer_quote(env, body, *span)?;
                self.uni
                    .unify(&ty, expected)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(core)
            }
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
                self.unify_arg(&ty, expected)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(core)
            }
            ast::Term::App(f, args, span) => {
                // `delay (\() -> e)` against `Later a` builds the guarded
                // observation modality (core language specification
                // section 9; codata and guardedness design, section 2).
                if let ast::Term::Var(p) = &**f {
                    if p.quals.is_empty()
                        && p.name.text == "delay"
                        && args.len() == 1
                        && self.is_builtin_form(env, "delay")
                    {
                        if let Ty::Later(inner) = self.uni.zonk(expected) {
                            return self.check_later_delay(env, args, *span, *inner);
                        }
                    }
                }
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
                // Structured concurrency (concurrency specification,
                // section 2).
                if let Some(v) = self.conc_builtin(env, f) {
                    let (core, ty) = self.infer_conc_app(env, &v, args, *span)?;
                    self.unify_arg(&ty, expected)
                        .map_err(|m| Error::at(term.span(), m))?;
                    return Ok(core);
                }
                let (core, ty) = self.infer(env, term)?;
                self.unify_arg(&ty, expected)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(core)
            }
            ast::Term::Tuple(items, span) => {
                // A dependent pair is a two-word product whose second
                // component's type mentions the first (core specification
                // section 1: `Sigma (n :: Nat). Vec n a` retains `n`).
                if let Ty::Sigma { var, dom, cod } = self.uni.zonk(expected) {
                    if items.len() != 2 {
                        return Err(Error::at(
                            *span,
                            "a dependent pair has exactly two components",
                        ));
                    }
                    let c0 = self.check(env, &items[0], &dom)?;
                    let idx = self.idx_of_term(&items[0], env).map_err(|e| {
                        Error::at(
                            items[0].span(),
                            format!(
                                "{} (the first component of a dependent pair must be expressible as an index value)",
                                e.message
                            ),
                        )
                    })?;
                    let c1 = self.check(env, &items[1], &cod.subst_idx(&var, &idx))?;
                    let gid = self.tuple_ctor(2, *span)?;
                    return Ok(app_spine(ECore::Ctor(gid), vec![c0, c1]));
                }
                let (core, ty) = self.infer(env, term)?;
                self.unify_arg(&ty, expected)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(core)
            }
            _ => {
                let (core, ty) = self.infer(env, term)?;
                self.unify_arg(&ty, expected)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(core)
            }
        }
    }

    /// `delay (\() -> e)` against `Later a`: the guarded observation of
    /// the codata discipline (core language specification section 9).
    /// `Later` is a logical modality with no effect row, so its body must
    /// be pure: effectful or diverging computation belongs in a
    /// `Thunk {e}` value, whose row records it.
    fn check_later_delay(
        &mut self,
        env: &mut Env,
        args: &[ast::Term],
        span: ast::Span,
        inner: Ty,
    ) -> Result<ECore> {
        if args.len() != 1 {
            return Err(Error::at(
                span,
                "delay takes exactly one argument in the bootstrap slice",
            ));
        }
        let eff = self.uni.fresh_row();
        let fty = Ty::fun_eff(Ty::Unit, eff.clone(), inner);
        let arg = self.check(env, &args[0], &fty)?;
        self.uni.unify_rows(&eff, &EffRow::pure()).map_err(|_| {
            Error::at(
                span,
                "a `delay` building a `Later` must be pure: `Later` is a logical modality with no effect row (core language specification section 9); defer effectful or diverging computation in a `Thunk {e}` value",
            )
        })?;
        // Creating the guard performs nothing; at runtime the Later keeps
        // the thunk representation (design section 6).
        Ok(app_spine(ECore::Prim(PrimOp::Delay), vec![arg]))
    }

    /// `delay e`, `force t`, `demand d`: builtin deferral forms from the
    /// laziness and reclamation specification. They are special forms so
    /// their effect rows are synthesized exactly.
    // ------------------------------------------------------------ staging (P9)

    /// `[| e |]`: a typed quotation, a `Code t` value built at compile
    /// time from template `e` (modules specification section 7).
    fn infer_quote(
        &mut self,
        env: &mut Env,
        body: &ast::Term,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let t = self.uni.fresh();
        let core = self.check_quote_template(env, body, &t, span)?;
        Ok((core, Ty::Adt("Code".into(), vec![t])))
    }

    /// Check one quotation template at `expected`, elaborating its holes
    /// in compile-time mode and its body in closed runtime-side scope: a
    /// quotation cannot capture the enclosing locals (decision D43), and
    /// its body's effects are latent in the code (decision D44), checked
    /// under ordinary rules when the code is spliced into declarations.
    fn check_quote_template(
        &mut self,
        env: &mut Env,
        body: &ast::Term,
        expected: &Ty,
        span: ast::Span,
    ) -> Result<ECore> {
        let _ = span;
        let mut template = body.clone();
        let mut holes: Vec<ast::HoleExpr> = Vec::new();
        ast::collect_template_holes(&mut template, &mut holes);
        let (hole_cores, hole_tys) = self.elaborate_holes(env, &holes)?;
        let mut tenv = Env::default();
        let save_perf = std::mem::take(&mut self.perf);
        let save_ct = self.compile_time;
        let save_holes = std::mem::take(&mut self.hole_tys);
        self.compile_time = false;
        self.hole_tys = hole_tys;
        let r = self.check(&mut tenv, &template, expected);
        self.perf = save_perf;
        self.compile_time = save_ct;
        self.hole_tys = save_holes;
        // The template stays AST and is checked again under ordinary
        // rules wherever the code lands; its elaboration is retained as
        // the quote's reified body, the program a runtime `eval` runs
        // (supercompilation and JIT specification section 7).
        let body = r?;
        Ok(ECore::Quote {
            template: Box::new(template),
            holes: hole_cores,
            body: Box::new(body),
        })
    }

    /// `quoteDecls where { ... }`: a declaration-group value (modules
    /// specification section 8: the generated declarations are checked
    /// under ordinary rules at emission, so the template is only parsed
    /// here and its holes elaborated).
    fn infer_quote_decls(
        &mut self,
        env: &mut Env,
        decls: &[ast::Decl],
        _span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let mut template = decls.to_vec();
        let mut holes: Vec<ast::HoleExpr> = Vec::new();
        ast::collect_decl_template_holes(&mut template, &mut holes);
        let (hole_cores, _) = self.elaborate_holes(env, &holes)?;
        Ok((
            ECore::DeclsQuote { template, holes: hole_cores },
            Ty::Adt("Declarations".into(), Vec::new()),
        ))
    }

    /// Elaborate a template's holes in compile-time mode; each hole's
    /// position type is the code it splices (`Code t` fills a `t` hole) or
    /// a fresh variable for raw `Syntax`.
    fn elaborate_holes(
        &mut self,
        env: &mut Env,
        holes: &[ast::HoleExpr],
    ) -> Result<(Vec<ECore>, Vec<Ty>)> {
        let mut cores = Vec::new();
        let mut tys = Vec::new();
        for h in holes {
            let (core, ty) = self.elaborate_hole(env, h)?;
            cores.push(core);
            tys.push(ty);
        }
        Ok((cores, tys))
    }

    fn elaborate_hole(
        &mut self,
        env: &mut Env,
        h: &ast::HoleExpr,
    ) -> Result<(ECore, Ty)> {
        let (core, ty, span) = match h {
            ast::HoleExpr::Term(t) => {
                let save = self.compile_time;
                self.compile_time = true;
                let r = self.infer(env, t);
                self.compile_time = save;
                let (core, ty) = r?;
                (core, ty, t.span())
            }
            ast::HoleExpr::Macro { path, args, span } => {
                let (core, ty) = self.macro_call_value(env, path, args, *span)?;
                (core, ty, *span)
            }
        };
        let ty = self.uni.zonk(&ty);
        match ty {
            Ty::Adt(name, args) if name == "Code" && args.len() == 1 => Ok((core, args[0].clone())),
            Ty::Adt(name, args) if name == "Syntax" && args.is_empty() => {
                let fresh = self.uni.fresh();
                Ok((core, fresh))
            }
            other => Err(Error::at(
                span,
                format!(
                    "a splice or macro invocation in a template must produce `Code t` or `Syntax`, but this one produces {other}"
                ),
            )),
        }
    }

    /// `m!( tokens )`: apply the macro to its `Syntax` arguments at
    /// compile time (the expansion is inserted where the value is used;
    /// in runtime code the staging pass expands it before checking).
    fn macro_call_value(
        &mut self,
        env: &mut Env,
        path: &ast::VarPath,
        args: &ast::TokenTree,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        if !self.compile_time {
            return Err(Error::at(
                span,
                "a macro invocation must be expanded before checking; it is only callable from compile-time code or a template",
            ));
        }
        let v = self.resolve_var(path, span)?;
        if !matches!(v.kind, ValueKind::MacroFn) {
            return Err(Error::at(
                span,
                format!("`{}` is not a macro (only `macro` definitions take `!(...)` token-tree arguments)", v.name),
            ));
        }
        let groups = args.arg_groups();
        let key = (v.module.clone(), v.name.clone());
        if let Some(info) = self.macros.get(&key) {
            let want = if info.arity == 1 { 1 } else { info.arity };
            if info.arity != groups.len() && !(info.arity == 1 && groups.len() == 1) {
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
            let _ = want;
        }
        let (mut ty, ctx, val_args) = self.instantiate(&v.scheme);
        if !ctx.is_empty() || !val_args.is_empty() {
            return Err(Error::at(
                span,
                "a macro cannot be polymorphic or constrained in this slice",
            ));
        }
        let mut arg_cores = Vec::new();
        for g in &groups {
            arg_cores.push(ECore::SyntaxLit(g.tokens.clone()));
        }
        // The invocation applies one argument per token-tree group: the
        // result type is what remains after that many arrows.
        ty = self.uni.zonk(&ty);
        for _ in &groups {
            ty = self.uni.zonk(&ty);
            ty = match ty {
                Ty::Fun(_, _, _, b) => *b,
                Ty::Pi { cod, .. } => *cod,
                other => other,
            };
        }
        let id = GlobalId(format!("{}.{}", v.module, v.name));
        Ok((app_spine(ECore::Global(id), arg_cores), ty))
    }

    /// The `Result` must-use default (attribute annotations
    /// specification section 10; decision D114's rule 7 default):
    /// discarding a `Result`-typed value without `ignore` is a check
    /// error (the return-value discipline of JPL rule 7).
    fn check_must_use_discard(&mut self, discarded: &Ty, span: ast::Span) -> Result<()> {
        let t = self.uni.zonk(discarded);
        if let Ty::Adt(n, _) = &t {
            if n == "Result" {
                return Err(Error::at(
                    span,
                    "discarding a `Result`-typed expression without `ignore`: check the outcome or discard it on purpose (`ignore e`; the return-value discipline of JPL rule 7, decision D114)",
                ));
            }
        }
        Ok(())
    }

    fn infer_builtin_app(
        &mut self,
        env: &mut Env,
        name: &str,
        args: &[ast::Term],
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        // `refSet m v` is the one builtin with two arguments (a write
        // through an exclusive borrow), so it dispatches before the
        // single-argument spine below.
        if name == "refSet" {
            return self.infer_refset_app(env, args, span);
        }
        if args.len() != 1 {
            return Err(Error::at(
                span,
                format!("{name} takes exactly one argument in the bootstrap slice"),
            ));
        }
        match name {
            "ignore" => {
                // `ignore e`: the explicit discard of the return-value
                // discipline (attribute annotations specification
                // section 10; decision D114's rule 7 default). The value
                // runs and is dropped on purpose; a `Result` discarded
                // any other way is a check error.
                let (core, ty) = self.infer(env, &args[0])?;
                let l = self.fresh_local();
                self.register_obligation(l, "_", None, ty, span);
                Ok((
                    ECore::Let(l, Box::new(core), Box::new(ECore::Unit)),
                    Ty::Unit,
                ))
            }
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
                if let Ty::Later(inner) = self.uni.zonk(&ty) {
                    // Forcing a `Later` is the next tick of the guarded
                    // observation: it runs the guard body and performs no
                    // effects beyond what the body does (core language
                    // specification section 9).
                    return Ok((app_spine(ECore::Prim(PrimOp::Force), vec![arg]), *inner));
                }
                let eff = self.uni.fresh_row();
                let inner = self.uni.fresh();
                self.uni
                    .unify(&ty, &Ty::Thunk { eff: eff.clone(), inner: Box::new(inner.clone()) })
                    .map_err(|m| {
                        Error::at(args[0].span(), format!("{m} (force expects a Thunk or a Later)"))
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
                                let (t, _, _) = self.instantiate(&s);
                                let t = self.uni.zonk(&t);
                                if matches!(t, Ty::Lazy { .. }) {
                                    self.note_use(l, p.span(), &[], &env.dicts)?;
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
            "unrefine" => {
                // `unrefine e`: a refinement's free projection (refinement
                // types design: the container IS the value, so the
                // projection is the identity function at runtime).
                let (core, ty) = self.infer(env, &args[0])?;
                let ty = self.uni.zonk(&ty);
                let base = self.refine_base(&ty).ok_or_else(|| {
                    Error::at(
                        args[0].span(),
                        format!(
                            "`unrefine` projects a refinement to its container, but this is {ty} (a refinement value IS its container value)"
                        ),
                    )
                })?;
                Ok((core, base))
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
            "decide" => {
                // `decide P` with a supplied decision procedure (core
                // specification section 4): the result is `Yes evidence`
                // or `No refutation`, scoped to the examined values. The
                // compiler distinguishes static decisions, disproved
                // obligations, unsupported reasoning and timeouts; none of
                // the failing outcomes is a proof.
                if args.len() != 1 {
                    return Err(Error::at(span, "`decide` takes exactly one proposition"));
                }
                // The structural decidable vocabulary (`NotNull e`,
                // `NonZero e`, `Ascii e`, `Ordered e`): a named predicate
                // over one value (refinement types design: refinements'
                // discharge lives in this `Dec`/`decide` world).
                let pred_shape = match &args[0] {
                    ast::Term::App(f, pargs, _) if pargs.len() == 1 => {
                        let head = match &**f {
                            ast::Term::Con(p) => Some(p.parts().join(".")),
                            ast::Term::Var(p) if p.quals.is_empty() => {
                                Some(p.name.text.clone())
                            }
                            _ => None,
                        };
                        head.and_then(|h| PredFn::lookup(&h))
                            .map(|pf| (pf, &pargs[0] as &ast::Term))
                    }
                    _ => None,
                };
                if let Some((pf, operand)) = pred_shape {
                    let (oc, oty) = self.infer(env, operand)?;
                    let oty = self.uni.zonk(&oty);
                    self.check_pred_operand(pf, &oty, operand.span())?;
                    let oi = self.idx_of_term(operand, env)?;
                    let prop = PTerm::Pred(pf, Box::new(oi));
                    let dec_ty =
                        Ty::Adt("Dec".into(), vec![Ty::Prp(Box::new(prop.clone()))]);
                    let yes = ECore::Ctor(fn_id("<builtin>", "Yes"));
                    let no = ECore::Ctor(fn_id("<builtin>", "No"));
                    match self.static_obligation(&prop)? {
                        ObligStatus::Checked => return Ok((yes, dec_ty)),
                        ObligStatus::Disproved => return Ok((no, dec_ty)),
                        ObligStatus::Timeout => {
                            return Err(Error::at(
                                span,
                                format!(
                                    "obligation status: timeout: deciding `{prop}` ran out of reduction steps"
                                ),
                            ))
                        }
                        ObligStatus::Unsupported => {
                            return Err(Error::at(
                                span,
                                format!(
                                    "obligation status: unsupported: there is no decision procedure for `{prop}`"
                                ),
                            ))
                        }
                        ObligStatus::Unresolved => {}
                    }
                    // The runtime test of the predicate's decision
                    // procedure: compiled from the compiler's own
                    // vocabulary, never from source a shadowing module
                    // could lie with.
                    self.cur_runtime_decides.push(span);
                    let test = self.pred_runtime_test(pf, &oty, oc, operand.span())?;
                    return Ok((
                        ECore::If(Box::new(test), CBranch::new(yes), CBranch::new(no)),
                        dec_ty,
                    ));
                }
                let is_eq = matches!(&args[0], ast::Term::App(f, _, _) if matches!(&**f, ast::Term::Con(p) if p.parts() == ["Eq"]));
                let (cmp, lhs, rhs) = self.decide_shape(&args[0])?;
                // Both operands check against one type, so literals
                // adapt to the other side (primitive types specification,
                // section 7).
                let cmp_ty = self.uni.fresh();
                let lc = self.decide_operand(env, &lhs, &cmp_ty)?;
                let rc = self.decide_operand(env, &rhs, &cmp_ty)?;
                // Two bare numerals compare at `Nat` (decision D7's
                // literal defaulting, applied to `decide`).
                if matches!(self.uni.zonk(&cmp_ty), Ty::Meta(_)) {
                    self.uni
                        .unify(&cmp_ty, &Ty::Nat)
                        .map_err(|m| Error::at(span, m))?;
                }
                let lt = self.uni.zonk(&cmp_ty);
                let li = self.idx_of_term(&lhs, env)?;
                let ri = self.idx_of_term(&rhs, env)?;
                let prop = if is_eq {
                    PTerm::Eq(Box::new(li), Box::new(ri))
                } else {
                    PTerm::Cmp(cmp, Box::new(li), Box::new(ri))
                };
                let dec_ty = Ty::Adt("Dec".into(), vec![Ty::Prp(Box::new(prop.clone()))]);
                let yes = ECore::Ctor(fn_id("<builtin>", "Yes"));
                let no = ECore::Ctor(fn_id("<builtin>", "No"));
                match self.static_obligation(&prop)? {
                    ObligStatus::Checked => return Ok((yes, dec_ty)),
                    ObligStatus::Disproved => return Ok((no, dec_ty)),
                    ObligStatus::Timeout => {
                        return Err(Error::at(
                            span,
                            format!(
                                "obligation status: timeout: deciding `{prop}` ran out of reduction steps"
                            ),
                        ))
                    }
                    ObligStatus::Unsupported => {
                        return Err(Error::at(
                            span,
                            format!(
                                "obligation status: unsupported: there is no decision procedure for `{prop}`"
                            ),
                        ))
                    }
                    ObligStatus::Unresolved => {}
                }
                // An executable comparison over the examined values: a
                // runtime check of the `#[assert-min]` count (attribute
                // annotations specification section 10; proof
                // obligations do not count).
                self.cur_runtime_decides.push(span);
                // An executable comparison over the examined values; the
                // evidence exists only on the successful branch.
                let cmp_op = match cmp {
                    IdxCmp::Eq => CmpOp::Eq,
                    IdxCmp::Ne => CmpOp::Ne,
                    IdxCmp::Lt => CmpOp::Lt,
                    IdxCmp::Le => CmpOp::Le,
                    IdxCmp::Gt => CmpOp::Gt,
                    IdxCmp::Ge => CmpOp::Ge,
                };
                let test = match &lt {
                    Ty::Nat => {
                        let imm = ECore::App(
                            Box::new(ECore::Prim(PrimOp::Nat(NatOp::Cmp))),
                            vec![lc, rc],
                        );
                        ECore::App(
                            Box::new(ECore::Prim(PrimOp::Cmp { ty: Scalar::I64, op: cmp_op })),
                            vec![imm, ECore::Int(0)],
                        )
                    }
                    Ty::Int => {
                        let imm = ECore::App(
                            Box::new(ECore::Prim(PrimOp::Int(IntOp::Cmp))),
                            vec![lc, rc],
                        );
                        ECore::App(
                            Box::new(ECore::Prim(PrimOp::Cmp { ty: Scalar::I64, op: cmp_op })),
                            vec![imm, ECore::Int(0)],
                        )
                    }
                    Ty::Prim(s) if s.is_int_like() || *s == Scalar::Char => ECore::App(
                        Box::new(ECore::Prim(PrimOp::Cmp { ty: *s, op: cmp_op })),
                        vec![lc, rc],
                    ),
                    other => {
                        return Err(Error::at(
                            span,
                            format!(
                                "obligation status: unsupported: `decide` compares `Nat`, `Int`, and machine integers, not {other}"
                            ),
                        ))
                    }
                };
                Ok((
                    ECore::If(
                        Box::new(test),
                        CBranch::new(yes),
                        CBranch::new(no),
                    ),
                    dec_ty,
                ))
            }
            _ => unreachable!(),
        }
    }

    /// `refSet m v`: write `v` through the exclusive `Mut` borrow `m`,
    /// mutating the borrowed location (evidence invalidation over mutable
    /// state). The write invalidates every evidence item about that
    /// location derived before it. It lowers to a store of `v` into the
    /// borrow's slot, so later reads of `m` observe the new value.
    fn infer_refset_app(
        &mut self,
        env: &mut Env,
        args: &[ast::Term],
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        if args.len() != 2 {
            return Err(Error::at(
                span,
                "`refSet` takes a `Mut` borrow and a value: `refSet m v` writes the borrowed location",
            ));
        }
        let (m_core, m_ty) = self.infer(env, &args[0])?;
        let m_ty = self.uni.zonk(&m_ty);
        let (region, inner) = match &m_ty {
            Ty::Mut(r, inner) => (r.clone(), (**inner).clone()),
            other => {
                return Err(Error::at(
                    args[0].span(),
                    format!(
                        "`refSet` writes through an exclusive `Mut` borrow, but this is {other} (a `Ref` borrow is read-only; use `mutBorrow` for a writable location)"
                    ),
                ))
            }
        };
        let v_core = self.check(env, &args[1], &inner)?;
        // The write targets the borrow's held cell, so it must be a bound
        // place (a variable) whose slot the store updates.
        let m_local = match m_core {
            ECore::Var(l) => l,
            _ => {
                return Err(Error::at(
                    args[0].span(),
                    "`refSet` writes through a bound `Mut` borrow (a variable naming the location)",
                ))
            }
        };
        // Record the mutation so evidence about `region` derived earlier
        // goes stale at this point. The written value's index shape is
        // the temporal contracts' post-step state (core specification
        // section 10).
        let v_idx = self.idx_of_term(&args[1], env).ok();
        self.note_ev_write_loc(region, span, v_idx);
        // Lower to a write through the borrow's cell that yields `Unit`.
        Ok((ECore::RefSet(m_local, Box::new(v_core)), Ty::Unit))
    }

    /// The shape `decide` accepts: a comparison (`i < n`) or `Eq x y`.
    fn decide_shape(&self, t: &ast::Term) -> Result<(IdxCmp, ast::Term, ast::Term)> {
        if let ast::Term::App(f, args, span) = t {
            if args.len() == 2 {
                match &**f {
                    ast::Term::Var(p) => {
                        let cmp = match p.name.text.as_str() {
                            "<" => Some(IdxCmp::Lt),
                            "<=" => Some(IdxCmp::Le),
                            ">" => Some(IdxCmp::Gt),
                            ">=" => Some(IdxCmp::Ge),
                            "==" | "=" => Some(IdxCmp::Eq),
                            "/=" | "!=" => Some(IdxCmp::Ne),
                            _ => None,
                        };
                        if let Some(cmp) = cmp {
                            return Ok((cmp, args[0].clone(), args[1].clone()));
                        }
                    }
                    ast::Term::Con(p) if p.parts() == ["Eq"] => {
                        return Ok((IdxCmp::Eq, args[0].clone(), args[1].clone()));
                    }
                    ast::Term::Con(p) if p.parts() == ["Not"] => {
                        return Err(Error::at(
                            *span,
                            "obligation status: unsupported: `decide` has no decision procedure for a negated proposition",
                        ));
                    }
                    _ => {}
                }
            }
        }
        if let ast::Term::Var(p) = t {
            return Err(Error::at(
                p.span(),
                format!(
                    "obligation status: unsupported: there is no decision procedure for the proposition variable `{}`",
                    p.name.text
                ),
            ));
        }
        Err(Error::at(
            t.span(),
            "obligation status: unsupported: `decide` takes a comparison (`i < n`) or `Eq x y` over values it can compare",
        ))
    }

    // ------------------------------------------------- dependent core (P7)

    /// Resolve a constructor name against the expected type first (type
    /// directed), so `Nil` and `Cons` patterns and constructions pick the
    /// `Vec` constructors even though the prelude's `List` owns those
    /// names, then fall back to ordinary resolution.
    fn resolve_ctor_for(
        &self,
        path: &ast::ConPath,
        expected: Option<&Ty>,
        span: ast::Span,
    ) -> Result<ValueInfo> {
        if let Some(Ty::Adt(tname, _)) = expected {
            if let Ok(info) = self.lookup_type(&[tname]) {
                let parts = path.parts();
                let last = parts[parts.len() - 1];
                let qualified_ok = parts.len() == 1 || parts[0] == info.module;
                if qualified_ok && info.ctors.iter().any(|(n, _)| n == last) {
                    if let Some(v) = self.qualified.get(&(info.module.clone(), last.to_string())) {
                        if matches!(v.kind, ValueKind::Ctor { .. }) {
                            let mut v = v.clone();
                            v.name = last.to_string();
                            return Ok(v);
                        }
                    }
                }
            }
        }
        self.resolve_con(path, span)
    }

    /// Constructor application, type-directed: the expected type solves
    /// the constructor's index parameters before its arguments are
    /// checked, and erased evidence arguments are checked but never passed
    /// at runtime (core specification section 3).
    fn ctor_app(
        &mut self,
        env: &mut Env,
        path: &ast::ConPath,
        args: &[ast::Term],
        expected: &Ty,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        // A capitalized module parameter used bare (`Seq`) is the module
        // scope's binding, not a constructor (module parameters as module
        // arguments).
        if args.is_empty() {
            if let Some((l, ty)) = self.module_param_use(path) {
                self.uni
                    .unify(&ty, expected)
                    .map_err(|m| Error::at(span, m))?;
                return Ok((ECore::Var(l), ty));
            }
        }
        let v = self.resolve_ctor_for(path, Some(expected), span)?;
        if matches!(&v.kind, ValueKind::Ctor { .. })
            && self
                .codata_types
                .contains(&(v.module.clone(), v.name.clone()))
        {
            return Err(Error::at(
                span,
                format!(
                    "a codata value is built only by its observed construction form `{} {{ ... }}`: its observations are closures run at observation time",
                    v.name
                ),
            ));
        }
        let (tag, arity, field_erased, runtime_arity, ctor_refine) = match &v.kind {
            ValueKind::Prim(PrimOp::Eval) if args.len() == 1 => {
                return self.infer_eval_app(env, args, span);
            }
            ValueKind::Ctor { tag, arity, field_erased, runtime_arity, refine, .. } => {
                (*tag, *arity, field_erased.clone(), *runtime_arity, refine.clone())
            }
            _ => {
                // Not a constructor: ordinary application.
                let (mut core, mut ty) = self.value_use(&v, span, env)?;
                for a in args {
                    ty = self.uni.zonk(&ty);
                    let (dom, cod, eff) = match ty {
                        Ty::Fun(d, _, e, c) => (d, c, e),
                        Ty::Pi { dom: d, eff: e, cod: c, var, .. } => {
                            let subst = self.idx_of_term(a, env)?;
                            let c = c.subst_idx(&var, &subst);
                            (d, Box::new(c), e)
                        }
                        other => {
                            return Err(Error::at(
                                a.span(),
                                format!("cannot apply a value of type {other}"),
                            ))
                        }
                    };
                    self.perform(&eff)?;
                    let ac = self.check(env, a, &dom)?;
                    core = app_spine(core, vec![ac]);
                    ty = *cod;
                }
                return Ok((core, ty));
            }
        };
        if args.len() != arity {
            if args.len() < arity && !field_erased.iter().any(|e| *e) {
                // A constructor without erased fields works under
                // application through the apply stub.
                let (mut core, mut ty) = self.value_use(&v, span, env)?;
                for a in args {
                    ty = self.uni.zonk(&ty);
                    let (dom, cod, eff) = match ty {
                        Ty::Fun(d, _, e, c) => (d, c, e),
                        Ty::Pi { dom: d, eff: e, cod: c, var, .. } => {
                            let subst = self.idx_of_term(a, env)?;
                            let c = c.subst_idx(&var, &subst);
                            (d, Box::new(c), e)
                        }
                        other => {
                            return Err(Error::at(
                                a.span(),
                                format!("cannot apply a value of type {other}"),
                            ))
                        }
                    };
                    self.perform(&eff)?;
                    let ac = self.check(env, a, &dom)?;
                    core = app_spine(core, vec![ac]);
                    ty = *cod;
                }
                return Ok((core, ty));
            }
            return Err(Error::at(
                span,
                format!(
                    "constructor {} takes {} arguments{}, got {}",
                    v.name,
                    arity,
                    if field_erased.iter().any(|e| *e) {
                        " (its erased evidence arguments keep it from partial application)"
                    } else {
                        ""
                    },
                    args.len()
                ),
            ));
        }
        let (ty, _ctx, _) = self.instantiate(&v.scheme);
        // Peel the argument chain, standing each dependent binder for an
        // index metavariable so the result type can be solved first.
        let mut fields: Vec<(Option<usize>, Ty, Mult, EffRow)> = Vec::new();
        let mut cur = self.uni.zonk(&ty);
        let result = loop {
            match cur {
                Ty::Fun(d, m, e, b) => {
                    fields.push((None, *d, m, e));
                    cur = self.uni.zonk(&b);
                }
                Ty::Pi { var, dom, mult, eff, cod } => {
                    let mty = self.uni.fresh_idx();
                    let mid = match &mty {
                        Ty::Idx(t) => match &**t {
                            ITerm::MVar(m) => Some(*m),
                            _ => None,
                        },
                        _ => None,
                    };
                    let subst = match &mty {
                        Ty::Idx(t) => (**t).clone(),
                        _ => unreachable!(),
                    };
                    fields.push((mid, dom.subst_idx(&var, &subst), mult, eff));
                    cur = self.uni.zonk(&cod.subst_idx(&var, &subst));
                }
                other => break other,
            }
        };
        // The result's indices come from the expected type: `Fin 7 refl`
        // against `Fin 3` knows `7 < 3` before it looks at the evidence.
        self.uni.unify(&result, expected).map_err(|m| {
            Error::at(
                span,
                format!(
                    "constructor {} cannot build this value: {m}",
                    v.name
                ),
            )
        })?;
        let mut cores: Vec<ECore> = Vec::new();
        let mut kept: Vec<ECore> = Vec::new();
        for (i, a) in args.iter().enumerate() {
            let (mid, dom, mult, eff) = &fields[i];
            self.perform(eff)?;
            let decide_shaped = matches!(a, ast::Term::App(f, dargs, _)
                if dargs.len() == 1
                    && matches!(&**f, ast::Term::Var(p)
                        if p.quals.is_empty() && p.name.text == "decide"));
            let ac = if ctor_refine.is_some() && i + 1 == arity && decide_shaped {
                // The refinement constructor's `Dec` discharge (refinement
                // types design: direct construction takes a `Dec`
                // argument that erases). The discharge must decide
                // `Checked` at this construction: a forged or undecidable
                // discharge is refused naming it.
                let dom = self.uni.zonk(dom);
                let dec_ty = match &dom {
                    Ty::Prp(p) => Ty::Adt("Dec".into(), vec![Ty::Prp(p.clone())]),
                    other => {
                        return Err(Error::at(
                            a.span(),
                            format!(
                                "the refinement's witness is a proposition, not {other}"
                            ),
                        ))
                    }
                };
                let c = self.check(env, a, &dec_ty)?;
                if let Ty::Prp(p) = &dom {
                    match self.static_obligation(p)? {
                        ObligStatus::Checked => {}
                        ObligStatus::Disproved => {
                            return Err(Error::at(
                                a.span(),
                                format!(
                                    "the discharge `{p}` is disproved: no refined value can be constructed here (a failed check must not produce evidence)"
                                ),
                            ))
                        }
                        ObligStatus::Timeout => {
                            return Err(Error::at(
                                a.span(),
                                format!(
                                    "the discharge `{p}` ran out of reduction steps at this construction (none of these outcomes is a proof)"
                                ),
                            ))
                        }
                        ObligStatus::Unsupported | ObligStatus::Unresolved => {
                            return Err(Error::at(
                                a.span(),
                                format!(
                                    "the discharge `{p}` is not decided at this construction: discharge it in the `Dec`/`decide` world (`case decide ({p}) of Yes w -> ...`) and pass the witness, or use the total constructor"
                                ),
                            ))
                        }
                    }
                }
                c
            } else {
                self.check(env, a, dom)?
            };
            if let Some(m) = mid {
                let it = self.idx_of_term(a, env).map_err(|e| {
                    Error::at(
                        a.span(),
                        format!(
                            "{} (this constructor's result type mentions its argument, so the argument must be expressible as an index value)",
                            e.message
                        ),
                    )
                })?;
                self.uni.bind_idx(*m, it).map_err(|m| {
                    Error::at(a.span(), format!("constructor {}: {m}", v.name))
                })?;
            }
            let _ = mult;
            cores.push(ac);
        }
        for (i, c) in cores.into_iter().enumerate() {
            if !field_erased[i] {
                kept.push(c);
            }
        }
        debug_assert_eq!(kept.len(), runtime_arity);
        if ctor_refine.is_some() {
            // The refinement's transparent constructor (refinement types
            // design: the runtime representation IS the container): the
            // value is its container value, with no wrapper allocation.
            return Ok((kept.into_iter().next().expect("the container value"), result));
        }
        Ok((
            app_spine(ECore::Ctor(fn_id(&v.module, &v.name)), kept),
            result,
        ))
    }

    /// A named definition referenced from index position: a total
    /// function's code generation unit. A parameterized module's units
    /// need their module parameters, which are not index-level arguments,
    /// so dependent types cannot mention them (module parameters as
    /// module arguments).
    fn idx_ref(&self, v: &ValueInfo, span: ast::Span) -> Result<ITerm> {
        if self.param_modules.contains(&v.module) {
            return Err(Error::at(
                span,
                format!(
                    "{} belongs to the parameterized module {}; a dependent type cannot mention it (module parameters are not index-level arguments)",
                    v.name, v.module
                ),
            ));
        }
        Ok(ITerm::Fn(fn_id(&v.module, &v.name).0))
    }

    /// A term in index position (a value a dependent type mentions):
    /// variables, numerals, mathematical arithmetic, and applications of
    /// named total definitions (the surface shares term syntax between
    /// kinds, types and values).
    fn idx_of_term(&self, t: &ast::Term, env: &Env) -> Result<ITerm> {
        Ok(match t {
            ast::Term::Int(d, _) => ITerm::Num(d.clone()),
            ast::Term::Ann(e, _, _) => return self.idx_of_term(e, env),
            ast::Term::Neg(inner, span) => match neg_fold(inner) {
                Some(v) => ITerm::Num(v),
                None => {
                    return Err(Error::at(
                        *span,
                        "this expression cannot appear in a dependent type",
                    ))
                }
            },
            ast::Term::Var(p) => {
                if p.quals.is_empty() {
                    if env.lookup(&p.name.text).is_some() {
                        ITerm::Var(p.name.text.clone())
                    } else if matches!(p.name.text.as_str(), "toNat" | "length") {
                        ITerm::Fn(format!("<builtin>.{}", p.name.text))
                    } else {
                        let v = self.resolve_var(p, p.span())?;
                        self.idx_ref(&v, p.span())?
                    }
                } else {
                    let v = self.resolve_var(p, p.span())?;
                    self.idx_ref(&v, p.span())?
                }
            }
            ast::Term::Con(p) => {
                let v = self.resolve_con(p, p.span())?;
                self.idx_ref(&v, p.span())?
            }
            ast::Term::App(f, args, span) => {
                // A borrow read (`copyOut m`) in index position is a live
                // reading of the borrowed location: a proposition over it
                // is a refinement guarantee about that location, valid
                // until the location is written (evidence invalidation
                // over mutable state).
                if let ast::Term::Var(p) = &**f {
                    if p.quals.is_empty()
                        && matches!(p.name.text.as_str(), "copyOut" | "borrow" | "mutBorrow")
                        && args.len() == 1
                    {
                        return self.idx_of_term(&args[0], env);
                    }
                }
                // In index position `+`, `-` and `*` are mathematical
                // arithmetic, never class methods (decision D29).
                if let ast::Term::Var(p) = &**f {
                    if p.quals.is_empty() {
                        let opc = match p.name.text.as_str() {
                            "+" => Some(IdxOp::Add),
                            "-" => Some(IdxOp::Sub),
                            "*" => Some(IdxOp::Mul),
                            _ => None,
                        };
                        if let Some(opc) = opc {
                            if args.len() == 2 {
                                return Ok(ITerm::Bin(
                                    opc,
                                    Box::new(self.idx_of_term(&args[0], env)?),
                                    Box::new(self.idx_of_term(&args[1], env)?),
                                ));
                            }
                        }
                    }
                }
                let mut out = Vec::new();
                for a in args {
                    out.push(self.idx_of_term(a, env)?);
                }
                ITerm::App(Box::new(self.idx_of_term(f, env)?), out)
            }
            other => {
                return Err(Error::at(
                    other.span(),
                    "this expression cannot appear in a dependent type (only variables, numerals, arithmetic, and total function applications can)",
                ))
            }
        })
    }

    /// Check an erased proof term against the proposition it must prove.
    /// Proof terms check and vanish: they never become runtime code (core
    /// specification section 3), and a failed check never produces
    /// evidence.
    fn check_proof(&mut self, env: &mut Env, term: &ast::Term, prop: &PTerm) -> Result<ECore> {
        match self.static_obligation(prop)? {
            ObligStatus::Disproved => {
                return Err(Error::at(
                    term.span(),
                    format!(
                        "obligation disproved: `{prop}` is false, so no evidence for it can exist here (a failed check must not produce evidence)"
                    ),
                ))
            }
            ObligStatus::Timeout => {
                return Err(Error::at(
                    term.span(),
                    format!(
                        "obligation status: timeout: deciding `{prop}` ran out of reduction steps (none of these outcomes is a proof)"
                    ),
                ))
            }
            _ => {}
        }
        match term {
            ast::Term::Hole(span) => Err(Error::at(
                *span,
                "an unknown proof hole is not evidence; a deployment profile rejects unresolved obligations",
            )),
            ast::Term::Var(p) => {
                if p.quals.is_empty() {
                    if let Some((l, s)) = env.lookup(&p.name.text) {
                        let (t, _, _) = self.instantiate(&s);
                        let t = self.uni.zonk(&t);
                        return match t {
                            Ty::Prp(q) => {
                                self.uni.unify_prop(&q, prop).map_err(|m| {
                                    Error::at(
                                        term.span(),
                                        format!(
                                            "{m}: `{}` is evidence of `{q}`, not of `{prop}`",
                                            p.name.text
                                        ),
                                    )
                                })?;
                                // Evidence invalidation over mutable
                                // state: record the use; staleness is
                                // checked after regions unify.
                                self.record_ev_use(l, &q, env, term.span());
                                Ok(ECore::Unit)
                            }
                            other => Err(Error::at(
                                term.span(),
                                format!(
                                    "`{}` has type {other}, which is not a proposition",
                                    p.name.text
                                ),
                            )),
                        };
                    }
                }
                let v = self.resolve_var(p, p.span())?;
                let (t, _, _) = self.instantiate(&v.scheme);
                let t = self.uni.zonk(&t);
                match t {
                    Ty::Prp(q) => {
                        self.uni.unify_prop(&q, prop).map_err(|m| {
                            Error::at(
                                term.span(),
                                format!("{m}: `{}` is evidence of `{q}`, not of `{prop}`", v.name),
                            )
                        })?;
                        // A reference to a logical definition stays visible
                        // to the termination and cycle checks; the body is
                        // erased before code generation.
                        let target = (v.module.clone(), v.name.clone());
                        let core = match v.kind {
                            ValueKind::TopFn => {
                                if self.logical.contains(&target) {
                                    self.pending_refs.push(target);
                                }
                                self.fn_ref(&v, p.span())?
                            }
                            _ => ECore::Unit,
                        };
                        Ok(core)
                    }
                    Ty::Fun(..) | Ty::Pi { .. } => Err(Error::at(
                        term.span(),
                        format!("evidence former `{}` must be applied to its evidence", v.name),
                    )),
                    other => Err(Error::at(
                        term.span(),
                        format!("`{}` has type {other}, which is not a proposition", v.name),
                    )),
                }
            }
            ast::Term::App(f, args, span) => {
                let p = match &**f {
                    ast::Term::Var(p) => p,
                    other => {
                        return Err(Error::at(
                            other.span(),
                            "a proof term is `refl`, `congr e`, or the name of evidence",
                        ))
                    }
                };
                let v = if p.quals.is_empty() && env.lookup(&p.name.text).is_none() {
                    self.resolve_var(p, p.span())?
                } else if p.quals.is_empty() {
                    return Err(Error::at(
                        *span,
                        "a proof term is `refl`, `congr e`, or the name of evidence",
                    ));
                } else {
                    self.resolve_var(p, p.span())?
                };
                let (mut ty, _, _) = self.instantiate(&v.scheme);
                let mut arg_cores: Vec<ECore> = Vec::new();
                for a in args {
                    let z = self.uni.zonk(&ty);
                    let (dom, cod) = match z {
                        Ty::Fun(d, _, _, c) => (d, c),
                        Ty::Pi { var, dom: d, cod: c, .. } => {
                            let subst = self.idx_of_term(a, env)?;
                            (d, Box::new(c.subst_idx(&var, &subst)))
                        }
                        other => {
                            return Err(Error::at(
                                a.span(),
                                format!("too many evidence arguments ({other} is not a function)"),
                            ))
                        }
                    };
                    let d = self.uni.zonk(&dom);
                    match d {
                        Ty::Prp(q) => {
                            arg_cores.push(self.check_proof(env, a, &q)?);
                        }
                        other => {
                            return Err(Error::at(
                                a.span(),
                                format!(
                                    "an evidence argument states a proposition, but this one has type {other}"
                                ),
                            ))
                        }
                    }
                    ty = *cod;
                }
                match self.uni.zonk(&ty) {
                    Ty::Prp(q) => {
                        self.uni.unify_prop(&q, prop).map_err(|m| {
                            Error::at(*span, format!("{m}: this term proves `{q}`, not `{prop}`"))
                        })?;
                        // The reference stays visible to the termination
                        // and cycle checks; proof bodies are erased before
                        // code generation.
                        let target = (v.module.clone(), v.name.clone());
                        let head = match v.kind {
                            ValueKind::TopFn => {
                                if self.logical.contains(&target) {
                                    self.pending_refs.push(target);
                                }
                                self.fn_ref(&v, *span)?
                            }
                            _ => ECore::Unit,
                        };
                        Ok(if arg_cores.is_empty() {
                            head
                        } else {
                            app_spine(head, arg_cores)
                        })
                    }
                    other => Err(Error::at(
                        *span,
                        format!("this proof former does not prove a proposition ({other})"),
                    )),
                }
            }
            other => Err(Error::at(
                other.span(),
                "a proof term is `refl`, `congr e`, or the name of evidence",
            )),
        }
    }

    /// The status of a proof obligation (core specification section 4):
    /// checked static evidence, a disproved obligation, unsupported
    /// reasoning, a search timeout, or an unresolved obligation that only
    /// a runtime `decide` can settle. None of the failing outcomes is a
    /// proof.
    fn static_obligation(&self, p: &PTerm) -> Result<ObligStatus> {
        Ok(match self.uni.resolve_prop(p).normalize() {
            PTerm::Cmp(op, a, b) => {
                let a = self.uni.resolve_idx(&a).normalize();
                let b = self.uni.resolve_idx(&b).normalize();
                match (self.eval_idx(&a, 32), self.eval_idx(&b, 32)) {
                    (Stuck::Val(x), Stuck::Val(y)) => {
                        let holds = match op {
                            IdxCmp::Eq => x == y,
                            IdxCmp::Ne => x != y,
                            IdxCmp::Lt => x < y,
                            IdxCmp::Le => x <= y,
                            IdxCmp::Gt => x > y,
                            IdxCmp::Ge => x >= y,
                        };
                        if holds {
                            ObligStatus::Checked
                        } else {
                            ObligStatus::Disproved
                        }
                    }
                    (Stuck::Timeout, _) | (_, Stuck::Timeout) => ObligStatus::Timeout,
                    _ => ObligStatus::Unresolved,
                }
            }
            PTerm::Eq(a, b) => {
                if a.eq(&b) {
                    ObligStatus::Checked
                } else {
                    match (self.eval_idx(&a, 32), self.eval_idx(&b, 32)) {
                        (Stuck::Val(x), Stuck::Val(y)) if x != y => ObligStatus::Disproved,
                        (Stuck::Timeout, _) | (_, Stuck::Timeout) => ObligStatus::Timeout,
                        _ => ObligStatus::Unresolved,
                    }
                }
            }
            PTerm::Not(q) => match self.static_obligation(&q)? {
                ObligStatus::Checked => ObligStatus::Disproved,
                ObligStatus::Disproved => ObligStatus::Checked,
                other => other,
            },
            // The structural decidable vocabulary: decided by structural
            // evaluation over constructor forms and numerals (refinement
            // types design: predicates inhabit the decidable logic).
            PTerm::Pred(pf, a) => self.pred_obligation(pf, &a)?,
            // A proposition variable has no decision procedure.
            PTerm::Var(_) | PTerm::MVar(_) => ObligStatus::Unsupported,
        })
    }

    /// The constructor form of an index value: the resolved constructor's
    /// runtime arity and its arguments (`Cons h t`'s shape), or `None`
    /// for values that are not constructor applications.
    fn ctor_form(&self, t: &ITerm) -> Option<(usize, Vec<ITerm>)> {
        let (head, args): (&str, Vec<ITerm>) = match t {
            ITerm::Fn(k) => (k, Vec::new()),
            ITerm::App(f, args) => match &**f {
                ITerm::Fn(k) => (k, args.clone()),
                _ => return None,
            },
            _ => return None,
        };
        let (m, n) = head.rsplit_once('.')?;
        for v in self.values.get(n)? {
            if v.module == m {
                if let ValueKind::Ctor { runtime_arity, .. } = &v.kind {
                    return Some((*runtime_arity, args));
                }
            }
        }
        None
    }

    /// The static decision of one structural-vocabulary predicate
    /// (refinement types design: `NotNull`/`NonZero`/`Ascii`/`Ordered`
    /// decide over constructor forms and numerals at check time).
    fn pred_obligation(&self, pf: PredFn, it: &ITerm) -> Result<ObligStatus> {
        let it = self.uni.resolve_idx(it).normalize();
        Ok(match pf {
            PredFn::NonZero => match self.eval_idx(&it, 32) {
                Stuck::Val(v) => {
                    if v != 0 {
                        ObligStatus::Checked
                    } else {
                        ObligStatus::Disproved
                    }
                }
                Stuck::Timeout => ObligStatus::Timeout,
                Stuck::Stuck => ObligStatus::Unresolved,
            },
            PredFn::NotNull => match self.ctor_form(&it) {
                Some((runtime_arity, _)) => {
                    if runtime_arity == 0 {
                        ObligStatus::Disproved
                    } else {
                        ObligStatus::Checked
                    }
                }
                None => ObligStatus::Unresolved,
            },
            PredFn::Ordered => {
                // Walk the list's constructor spine, collecting the
                // numeric elements; the predicate holds exactly when
                // every adjacent pair is non-decreasing.
                let mut cur = it;
                let mut elems: Vec<i128> = Vec::new();
                let mut status = ObligStatus::Checked;
                loop {
                    match self.ctor_form(&cur) {
                        Some((runtime_arity, _)) if runtime_arity == 0 => break,
                        Some((runtime_arity, args)) if runtime_arity >= 2 && args.len() >= 2 => {
                            match self.eval_idx(&args[0], 32) {
                                Stuck::Val(v) => elems.push(v),
                                Stuck::Timeout => {
                                    status = ObligStatus::Timeout;
                                    break;
                                }
                                Stuck::Stuck => {
                                    status = ObligStatus::Unresolved;
                                    break;
                                }
                            }
                            cur = self.uni.resolve_idx(&args[1]).normalize();
                        }
                        _ => {
                            status = ObligStatus::Unresolved;
                            break;
                        }
                    }
                }
                if matches!(status, ObligStatus::Checked)
                    && elems.windows(2).any(|w| w[0] > w[1])
                {
                    status = ObligStatus::Disproved;
                }
                status
            }
            // No text literal exists in index position: `Ascii` decides
            // at runtime (`decide`) and over law samples only.
            PredFn::Ascii => ObligStatus::Unresolved,
        })
    }

    /// One type's constructors as `(tag, runtime arity)` pairs (the
    /// first in-scope registration of the name, the law sampler's rule).
    fn ctor_shapes(&self, name: &str) -> Option<Vec<(usize, usize)>> {
        let tk = self.types.get(name)?.first()?;
        let mut out = Vec::new();
        for (cname, tag) in &tk.ctors {
            let ra = self
                .qualified
                .get(&(tk.module.clone(), cname.clone()))
                .and_then(|v| match &v.kind {
                    ValueKind::Ctor { runtime_arity, .. } => Some(*runtime_arity),
                    _ => None,
                })
                .unwrap_or(1);
            out.push((*tag, ra));
        }
        Some(out)
    }

    /// The runtime decision test of one structural-vocabulary predicate
    /// (refinement types design). The tests are compiled from the
    /// compiler's own vocabulary: their semantics are baked, never read
    /// from source a shadowing module could lie with.
    fn pred_runtime_test(
        &mut self,
        pf: PredFn,
        oty: &Ty,
        operand: ECore,
        span: ast::Span,
    ) -> Result<ECore> {
        Ok(match pf {
            PredFn::NonZero => {
                // `x /= 0`: the numeric comparison test, exactly the
                // `decide (x /= 0)` lowering (the zero is a boxed word
                // wherever the operand is).
                let imm = match oty {
                    Ty::Nat => ECore::App(
                        Box::new(ECore::Prim(PrimOp::Nat(NatOp::Cmp))),
                        vec![
                            operand,
                            ECore::ConstBig { nat: true, digits: "0".to_string() },
                        ],
                    ),
                    Ty::Int => ECore::App(
                        Box::new(ECore::Prim(PrimOp::Int(IntOp::Cmp))),
                        vec![
                            operand,
                            ECore::ConstBig { nat: false, digits: "0".to_string() },
                        ],
                    ),
                    Ty::Prim(s) if s.is_int_like() => {
                        return Ok(ECore::App(
                            Box::new(ECore::Prim(PrimOp::Cmp {
                                ty: *s,
                                op: CmpOp::Ne,
                            })),
                            vec![operand, ECore::Int(0)],
                        ))
                    }
                    other => {
                        return Err(Error::at(
                            span,
                            format!(
                                "`NonZero` decides numeric values, not `{other}` (a wrong predicate for this value is refused naming the predicate)"
                            ),
                        ))
                    }
                };
                ECore::App(
                    Box::new(ECore::Prim(PrimOp::Cmp {
                        ty: Scalar::I64,
                        op: CmpOp::Ne,
                    })),
                    vec![imm, ECore::Int(0)],
                )
            }
            PredFn::NotNull => {
                // The constructor test: the empty (nullary) constructor
                // refutes the predicate, every other form proves it.
                let Ty::Adt(name, _) = oty else {
                    return Err(Error::at(
                        span,
                        format!(
                            "`NotNull` decides list-shaped values, not `{oty}` (a wrong predicate for this value is refused naming the predicate)"
                        ),
                    ));
                };
                let shapes = self.ctor_shapes(name).unwrap_or_default();
                let alts = shapes
                    .into_iter()
                    .map(|(tag, ra)| {
                        CAlt::new(
                            CPat::Con { tag, fields: Vec::new() },
                            ECore::Bool(ra != 0),
                        )
                    })
                    .collect();
                ECore::Case {
                    scrut: Box::new(operand),
                    alts,
                }
            }
            PredFn::Ascii => {
                // The byte scan over the text's bytes (`String` projects
                // its `Bytes` backing field).
                let bytes = match oty {
                    Ty::Bytes => operand,
                    Ty::Adt(name, _) if name == "String" => ECore::Proj(
                        Box::new(operand),
                        ProjField { slot: 0, access: None },
                    ),
                    other => {
                        return Err(Error::at(
                            span,
                            format!(
                                "`Ascii` decides text values (`String`), not `{other}` (a wrong predicate for this value is refused naming the predicate)"
                            ),
                        ))
                    }
                };
                let gid = self.install_ascii_test();
                ECore::App(
                    Box::new(ECore::Global(gid)),
                    vec![
                        bytes,
                        ECore::ConstBig { nat: true, digits: "0".to_string() },
                    ],
                )
            }
            PredFn::Ordered => {
                // The adjacent-pair scan over the list's constructor
                // spine (elements compare as `Nat` words).
                let Ty::Adt(name, _) = oty else {
                    return Err(Error::at(
                        span,
                        format!(
                            "`Ordered` decides lists of numerics, not `{oty}` (a wrong predicate for this value is refused naming the predicate)"
                        ),
                    ));
                };
                let gid = self.install_ordered_test(name, span)?;
                ECore::App(Box::new(ECore::Global(gid)), vec![operand])
            }
        })
    }

    /// The scan behind `decide (Ascii s)`: `all (< 128) (bytes of s)`,
    /// as a recursive unit over the byte words (the `Bytes` prim ABI
    /// speaks boxed `Nat` indices and byte values).
    fn install_ascii_test(&mut self) -> GlobalId {
        let gid = GlobalId(format!("<builtin>.refine-ascii-{}", self.fns.len()));
        // Params: the bytes word, the boxed `Nat` index.
        let b = Core::Var(LocalId(0));
        let i = Core::Var(LocalId(1));
        let len = Core::App(
            Box::new(Core::Prim(PrimOp::Bytes(BytesOp::Len))),
            vec![b.clone()],
        );
        let end_cmp = Core::App(
            Box::new(Core::Prim(PrimOp::Nat(NatOp::Cmp))),
            vec![i.clone(), len],
        );
        let at_end = Core::App(
            Box::new(Core::Prim(PrimOp::Cmp { ty: Scalar::I64, op: CmpOp::Ge })),
            vec![end_cmp, Core::Int(0)],
        );
        let byte = Core::App(
            Box::new(Core::Prim(PrimOp::Bytes(BytesOp::Get))),
            vec![b.clone(), i.clone()],
        );
        let ascii_cmp = Core::App(
            Box::new(Core::Prim(PrimOp::Nat(NatOp::Cmp))),
            vec![byte, Core::ConstBig { nat: true, digits: "128".to_string() }],
        );
        let ascii = Core::App(
            Box::new(Core::Prim(PrimOp::Cmp { ty: Scalar::I64, op: CmpOp::Lt })),
            vec![ascii_cmp, Core::Int(0)],
        );
        let next = Core::App(
            Box::new(Core::Prim(PrimOp::Nat(NatOp::Add))),
            vec![i, Core::ConstBig { nat: true, digits: "1".to_string() }],
        );
        let recur = Core::App(
            Box::new(Core::Global(gid.clone())),
            vec![b, next],
        );
        let body = Core::If(
            Box::new(at_end),
            CBranch::new(Core::Bool(true)),
            CBranch::new(Core::If(
                Box::new(ascii),
                CBranch::new(recur),
                CBranch::new(Core::Bool(false)),
            )),
        );
        self.fns.push(FnDef {
            id: gid.clone(),
            params: vec!["b".to_string(), "i".to_string()],
            body,
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
            meta: crate::core::FnMeta::default(),
        });
        gid
    }

    /// The scan behind `decide (Ordered xs)`: every adjacent pair of
    /// `Nat` elements is non-decreasing, as a recursive unit over the
    /// list's constructor spine.
    fn install_ordered_test(
        &mut self,
        type_name: &str,
        span: ast::Span,
    ) -> Result<GlobalId> {
        let shapes = self.ctor_shapes(type_name).unwrap_or_default();
        let mut empty: Option<usize> = None;
        let mut cons: Option<usize> = None;
        for (tag, ra) in shapes {
            if ra == 0 && empty.is_none() {
                empty = Some(tag);
            } else if ra == 2 && cons.is_none() {
                cons = Some(tag);
            }
        }
        let (Some(nil_tag), Some(cons_tag)) = (empty, cons) else {
            return Err(Error::at(
                span,
                format!(
                    "`Ordered` decides lists of numerics (an empty constructor and an element-tail constructor), and {type_name} is not that shape (a wrong predicate for this value is refused naming the predicate)"
                ),
            ));
        };
        let gid = GlobalId(format!("<builtin>.refine-ordered-{}", self.fns.len()));
        // Params: the list word (LocalId 0). Inner binders: head and
        // tail of the outer cons (1, 2), head and tail of the inner
        // cons (3, 4).
        let elem_cmp = Core::App(
            Box::new(Core::Prim(PrimOp::Cmp { ty: Scalar::I64, op: CmpOp::Le })),
            vec![
                Core::App(
                    Box::new(Core::Prim(PrimOp::Nat(NatOp::Cmp))),
                    vec![Core::Var(LocalId(1)), Core::Var(LocalId(3))],
                ),
                Core::Int(0),
            ],
        );
        let recur = Core::App(
            Box::new(Core::Global(gid.clone())),
            vec![Core::Var(LocalId(2))],
        );
        let inner = Core::Case {
            scrut: Box::new(Core::Var(LocalId(2))),
            alts: vec![
                CAlt::new(
                    CPat::Con { tag: nil_tag, fields: Vec::new() },
                    Core::Bool(true),
                ),
                CAlt::new(
                    CPat::Con {
                        tag: cons_tag,
                        fields: vec![LocalId(3), LocalId(4)],
                    },
                    Core::If(
                        Box::new(elem_cmp),
                        CBranch::new(recur),
                        CBranch::new(Core::Bool(false)),
                    ),
                ),
            ],
        };
        let body = Core::Case {
            scrut: Box::new(Core::Var(LocalId(0))),
            alts: vec![
                CAlt::new(
                    CPat::Con { tag: nil_tag, fields: Vec::new() },
                    Core::Bool(true),
                ),
                CAlt::new(
                    CPat::Con {
                        tag: cons_tag,
                        fields: vec![LocalId(1), LocalId(2)],
                    },
                    inner,
                ),
            ],
        };
        self.fns.push(FnDef {
            id: gid.clone(),
            params: vec!["xs".to_string()],
            body,
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
            meta: crate::core::FnMeta::default(),
        });
        Ok(gid)
    }

    /// Evaluate an index expression on numerals (dependent core kernel
    /// design, section 4: mathematical operations reduce on numerals
    /// only), with a fuel bound so the evaluator always terminates.
    fn eval_idx(&self, t: &ITerm, fuel: u32) -> Stuck {
        if fuel == 0 {
            return Stuck::Timeout;
        }
        match t {
            ITerm::Num(d) => match d.parse::<i128>() {
                Ok(v) => Stuck::Val(v),
                Err(_) => Stuck::Stuck,
            },
            ITerm::Bin(op, a, b) => {
                let a = self.eval_idx(&self.uni.resolve_idx(a), fuel - 1);
                let b = self.eval_idx(&self.uni.resolve_idx(b), fuel - 1);
                match (a, b) {
                    (Stuck::Val(x), Stuck::Val(y)) => {
                        let v = match op {
                            IdxOp::Add => x.checked_add(y),
                            IdxOp::Sub => x.checked_sub(y),
                            IdxOp::Mul => x.checked_mul(y),
                        };
                        match v {
                            Some(v) => Stuck::Val(v),
                            None => Stuck::Stuck,
                        }
                    }
                    (Stuck::Timeout, _) | (_, Stuck::Timeout) => Stuck::Timeout,
                    _ => Stuck::Stuck,
                }
            }
            _ => Stuck::Stuck,
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
        self.check_misaligned_place(env, &args[0], is_mut)?;
        self.check(env, &args[0], &inner)
    }

    /// The packed-borrow safety rule (attribute annotations
    /// specification section 4.2): a field whose offset is not a multiple
    /// of its natural alignment cannot be borrowed, since a reference
    /// asserts alignment. A borrowable place is a variable with a chain
    /// of field projections.
    fn check_misaligned_place(
        &self,
        env: &Env,
        t: &ast::Term,
        is_mut: bool,
    ) -> Result<()> {
        let mut proj: Vec<&ast::Name> = Vec::new();
        let mut cur = t;
        while let ast::Term::Proj(base, field, _) = cur {
            proj.push(field);
            cur = base;
        }
        let mut ty = match cur {
            ast::Term::Var(p) if p.quals.is_empty() => match env.lookup(&p.name.text) {
                Some((_, scheme)) => scheme.ty.clone(),
                None => return Ok(()),
            },
            _ => return Ok(()),
        };
        for f in proj.into_iter().rev() {
            let ty2 = self.uni.zonk(&ty);
            // A borrowed place projects through the borrow.
            let ty2 = match ty2 {
                Ty::Ref(_, inner) | Ty::Mut(_, inner) => *inner,
                other => other,
            };
            let ty_name = match &ty2 {
                Ty::Adt(name, _) => name.clone(),
                _ => return Ok(()),
            };
            let kind = match self.lookup_type(&[ty_name.as_str()]) {
                Ok(k) => k.clone(),
                Err(_) => return Ok(()),
            };
            let names = match &kind.fields {
                Some(names) => names,
                None => return Ok(()),
            };
            let idx = match names.iter().position(|n| n == &f.text) {
                Some(i) => i,
                None => return Ok(()),
            };
            if let Some(l) =
                self.type_layouts.get(&(kind.module.clone(), kind.name.clone()))
            {
                if let Some(ff) = l.fields.iter().find(|ff| ff.name == f.text) {
                    if ff.align != 0 && ff.offset % ff.align != 0 {
                        let form = if is_mut { "mutBorrow" } else { "borrow" };
                        return Err(Error::at(
                            f.span,
                            format!(
                                "field {} of {} sits at offset {} but its alignment is {}: a field whose offset is not a multiple of its natural alignment cannot be borrowed, since `{}` asserts alignment; copy the field instead (attribute annotations specification section 4.2)",
                                f.text, kind.name, ff.offset, ff.align, form
                            ),
                        ));
                    }
                }
            }
            // The next projection steps through this field's type.
            let fty = match self
                .qualified
                .get(&(kind.module.clone(), kind.name.clone()))
                .map(|v| &v.kind)
            {
                Some(ValueKind::Ctor { field_tys, ty_params, .. }) => {
                    let mut map = HashMap::new();
                    if let Ty::Adt(_, args) = &ty2 {
                        for (p, a) in ty_params.iter().zip(args.iter()) {
                            map.insert(p.clone(), a.clone());
                        }
                    }
                    Checker::subst_ty(&field_tys[idx], &map)
                }
                _ => return Ok(()),
            };
            ty = fty;
        }
        Ok(())
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
            let erased_dom = matches!(self.uni.zonk(&dom), Ty::Prp(_) | Ty::Sort(_));
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
                // `\() -> e`: the unit pattern of a guard thunk body
                // (`delay (\() -> ...)`, the codata spelling).
                ast::Pat::Unit(pat_span) => {
                    match self.uni.zonk(&dom) {
                        Ty::Unit | Ty::Meta(_) => {}
                        other => {
                            return Err(Error::at(
                                *pat_span,
                                format!("the unit pattern `()` cannot match {other}"),
                            ))
                        }
                    }
                    self.register_obligation(l, "_", Some(mult), dom, p.span());
                }
                other => {
                    return Err(Error::at(
                        other.span(),
                        "only variable and wildcard patterns are supported in lambdas in the bootstrap slice",
                    ));
                }
            }
            // An erased parameter takes no runtime slot (core
            // specification section 3).
            if !erased_dom {
                params.push(l);
            }
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

    /// One `decide` operand (core specification section 4): the bare
    /// index builtins `length` and `toNat` compute their runtime value
    /// through the matching primitive (`bytesLen` over `Bytes`, the
    /// bignum conversion from a machine word), so an obligation over
    /// them gets the same executable comparison as its index reading;
    /// every other operand checks ordinarily (literal defaulting and all).
    fn decide_operand(
        &mut self,
        env: &mut Env,
        t: &ast::Term,
        expected: &Ty,
    ) -> Result<ECore> {
        let (bname, barg): (Option<String>, Option<&ast::Term>) = match t {
            ast::Term::App(f, args, _) if args.len() == 1 => match &**f {
                ast::Term::Var(p)
                    if p.quals.is_empty()
                        && matches!(p.name.text.as_str(), "length" | "toNat") =>
                {
                    (Some(p.name.text.clone()), Some(&args[0]))
                }
                _ => (None, None),
            },
            _ => (None, None),
        };
        if let (Some(name), Some(arg)) = (bname, barg) {
            if name == "length" {
                let bytes_ty = Ty::Bytes;
                if let Ok(ac) = self.check(env, arg, &bytes_ty) {
                    self.uni.unify(expected, &Ty::Nat).map_err(|m| {
                        Error::at(t.span(), m)
                    })?;
                    return Ok(app_spine(
                        ECore::Prim(PrimOp::Bytes(BytesOp::Len)),
                        vec![ac],
                    ));
                }
            }
            if name == "toNat" {
                let word = self.uni.fresh();
                let ac = self.check(env, arg, &word)?;
                let zw = self.uni.zonk(&word);
                if matches!(zw, Ty::Prim(s) if s.is_int_like()) {
                    self.uni.unify(expected, &Ty::Nat).map_err(|m| {
                        Error::at(t.span(), m)
                    })?;
                    return Ok(app_spine(
                        ECore::Prim(PrimOp::Nat(NatOp::FromI64)),
                        vec![ac],
                    ));
                }
            }
        }
        self.check(env, t, expected)
    }

    /// One `eval` use (supercompilation and JIT specification section
    /// 7): a target profile may forbid runtime code generation entirely,
    /// every use is a build-visible assumption recorded beside the
    /// foreign ones, and a `spec`/`proof` definition (erased evidence)
    /// may never contain one.
    fn eval_use(&mut self, span: ast::Span) -> Result<()> {
        if !self.allow_eval {
            return Err(Error::at(
                span,
                "this target profile forbids runtime code generation (`eval` of a typed quote; supercompilation and JIT specification section 7)",
            ));
        }
        if self.current_logical.is_some() {
            return Err(Error::at(
                span,
                "eval is never evidence: a `spec`/`proof` definition is erased and must not use runtime code generation (supercompilation and JIT specification section 7)",
            ));
        }
        self.eval_sites.push((self.current_module.clone(), format!("{span}")));
        Ok(())
    }

    /// `eval q` (supercompilation and JIT specification section 7): the
    /// quote is typed by its `Code a` type when constructed, so no
    /// capability or effect belongs in `eval`'s signature. The result is
    /// executable code's value only: a proposition or sort result is
    /// evidence and is refused.
    fn infer_eval_app(
        &mut self,
        env: &mut Env,
        args: &[ast::Term],
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        self.eval_use(span)?;
        let t = self.uni.fresh();
        let q = self.check(env, &args[0], &Ty::Adt("Code".into(), vec![t.clone()]))?;
        let zt = self.uni.zonk(&t);
        if matches!(zt, Ty::Prp(_) | Ty::Sort(_)) {
            return Err(Error::at(
                span,
                "eval is never evidence: a runtime-generated value must not inhabit a proposition or erased position (supercompilation and JIT specification section 7)",
            ));
        }
        Ok((app_spine(ECore::Prim(PrimOp::Eval), vec![q]), zt))
    }

    /// Instantiate a resolved value: dictionary constraints become wants and
    /// methods elaborate to dictionary projection.
    fn value_use(&mut self, v: &ValueInfo, span: ast::Span, env: &mut Env) -> Result<(ECore, Ty)> {
        // `eval` uses are profile-gated and recorded wherever they
        // appear (supercompilation and JIT specification section 7).
        if matches!(v.kind, ValueKind::Prim(PrimOp::Eval)) {
            self.eval_use(span)?;
        }
        match &v.kind {
            ValueKind::StageBuiltin(op) => {
                // The compile-time capability API (`Stage.*`): its values
                // exist only during compilation (modules specification
                // section 10).
                if !self.compile_time {
                    return Err(Error::at(
                        span,
                        format!(
                            "{} is compile-time only: it cannot appear in runtime code (only in `stage`/`macro` definitions, `generate do` blocks, and quotation splices)",
                            v.name
                        ),
                    ));
                }
                let (ty, _ctx, _val_args) = self.instantiate(&v.scheme);
                return Ok((ECore::Global(op.global_id()), ty));
            }
            ValueKind::StageFn | ValueKind::MacroFn => {
                if !self.compile_time {
                    return Err(Error::at(
                        span,
                        format!(
                            "{} is a compile-time definition (`stage`/`macro`): it cannot appear in runtime code",
                            v.name
                        ),
                    ));
                }
            }
            ValueKind::BoolLit(b) => return Ok((ECore::Bool(*b), Ty::Bool)),
            ValueKind::EffectOp { head, .. } => {
                return Err(Error::at(
                    span,
                    format!(
                        "effect operation {} must be applied (it dispatches to the enclosing `handle` of {head})",
                        v.name
                    ),
                ));
            }
            ValueKind::ProofCtor => {
                // Evidence constructors are erased: they are checked as
                // proof terms and vanish at runtime (core specification
                // section 3).
                let (ty, _, _) = self.instantiate(&v.scheme);
                return Ok((ECore::Unit, ty));
            }
            ValueKind::Ctor { field_erased, .. } => {
                if self
                    .codata_types
                    .contains(&(v.module.clone(), v.name.clone()))
                {
                    return Err(Error::at(
                        span,
                        format!(
                            "a codata value is built only by its observed construction form `{} {{ ... }}`: its observations are closures run at observation time",
                            v.name
                        ),
                    ));
                }
                if field_erased.iter().any(|e| *e) {
                    return Err(Error::at(
                        span,
                        format!(
                            "constructor {} has erased evidence arguments and must be fully applied",
                            v.name
                        ),
                    ));
                }
            }
            _ => {}
        }
        let (ty, ctx, val_args) = self.instantiate(&v.scheme);
        // A `spec`/`proof` definition is erased ghost code: another
        // logical definition may reference it (the termination and cycle
        // checks see those references), but computational code may not
        // (core specification section 3).
        let target = (v.module.clone(), v.name.clone());
        if self.logical.contains(&target) {
            if self.current_logical.is_none() {
                return Err(Error::at(
                    span,
                    format!(
                        "{} is a `spec`/`proof` definition: it is erased ghost code and cannot be used computationally",
                        v.name
                    ),
                ));
            }
            self.pending_refs.push(target);
        }
        let mut holes = Vec::new();
        for (_, c) in &ctx {
            let id = self.new_want(c.clone(), span, env);
            holes.push(ECore::Hole(id));
        }
        // Implicit value arguments are runtime arguments (decision D29):
        // each becomes a pending, reified to runtime code once unification
        // has fixed its index.
        let mut names: HashMap<String, LocalId> = HashMap::new();
        for (n, l, _) in &env.locals {
            names.entry(n.clone()).or_insert(*l);
        }
        let mut idx_holes = Vec::new();
        for (i, m) in val_args.iter().enumerate() {
            let iterm = match m {
                Ty::Idx(it) => (**it).clone(),
                other => {
                    return Err(Error::at(
                        span,
                        format!("internal error: implicit argument of type {other} is not an index value"),
                    ))
                }
            };
            let nat = !matches!(v.scheme.val_params.get(i).map(|(_, t)| t), Some(Ty::Int));
            let id = self.idx_pends.len();
            self.idx_pends.push(IdxPend {
                iterm,
                names: names.clone(),
                nat,
                span,
            });
            idx_holes.push(ECore::IdxPending(id));
        }
        let core = match &v.kind {
            ValueKind::TopFn | ValueKind::PrimDecl { .. } => self.fn_ref(v, span)?,
            ValueKind::StageFn | ValueKind::MacroFn => {
                ECore::Global(fn_id(&v.module, &v.name))
            }
            ValueKind::StageBuiltin(op) => ECore::Global(op.global_id()),
            ValueKind::Prim(op) => ECore::Prim(*op),
            ValueKind::Foreign { unit, .. } => ECore::Foreign(unit.clone()),
            ValueKind::Ctor { .. } => ECore::Ctor(fn_id(&v.module, &v.name)),
            ValueKind::Method { idx, .. } => {
                // The class constraint is `ctx[0]`: it becomes the dictionary
                // the method is projected from.
                let dict = holes.remove(0);
                ECore::DictProj(Box::new(dict), *idx)
            }
            ValueKind::Law => {
                return Err(Error::at(
                    span,
                    format!(
                        "a class law is erased evidence (primitive operations and inline target IR specification section 5); `{}` never becomes runtime code",
                        v.name
                    ),
                ))
            }
            ValueKind::BoolLit(_) | ValueKind::EffectOp { .. } | ValueKind::ProofCtor => {
                unreachable!()
            }
        };
        holes.extend(idx_holes);
        let core = if holes.is_empty() { core } else { app_spine(core, holes) };
        Ok((core, ty))
    }

    // ------------------------------------------------------------ local declarations

    /// Elaborate one strict local binding body. In a `do` statement's
    /// `let` group the body must be pure: the statement's interpretation
    /// does not sequence effects (the sequencing spec's `do` rules), so
    /// its value is computed strictly and out of band.
    fn elab_local_strict(
        &mut self,
        env: &mut Env,
        body: &ast::Term,
        dspan: ast::Span,
        sig: Option<Ty>,
        form: LocalForm,
    ) -> Result<(ECore, Ty)> {
        match form {
            LocalForm::Plain => match sig {
                Some(t) => Ok((self.check(env, body, &t)?, t)),
                None => self.infer(env, body),
            },
            LocalForm::DoStmt { .. } => {
                let saved = self.enter_perf();
                let checked = match sig {
                    Some(t) => self.check(env, body, &t).map(|c| (c, t)),
                    None => self.infer(env, body),
                };
                let performed = self.exit_perf(saved);
                let (core, ty) = checked?;
                self.uni
                    .unify_rows(&performed, &EffRow::pure())
                    .map_err(|_| {
                        Error::at(
                            dspan,
                            "a `let` statement in `do` must be pure (its interpretation does not sequence effects)",
                        )
                    })?;
                Ok((core, ty))
            }
        }
    }

    fn elab_locals(
        &mut self,
        env: &mut Env,
        decls: &[ast::LocalDecl],
        form: LocalForm,
    ) -> Result<(Vec<(LocalId, ECore)>, Option<MemoGroup>)> {
        // A `do` statement's `let` group: one group never binds the same
        // name twice (it replaces nested single `let`s; a second binding
        // of one name in the group is a mistake, not a shadow), and
        // under a handled computation only simple value bindings
        // sequence (decision D24).
        if let LocalForm::DoStmt { handled } = form {
            let mut seen: HashSet<String> = HashSet::new();
            for d in decls {
                let mut names: Vec<(String, ast::Span)> = Vec::new();
                match d {
                    ast::LocalDecl::Sig { .. } => {}
                    ast::LocalDecl::Eq { name, pats, span, .. } => {
                        if handled && !pats.is_empty() {
                            return Err(Error::at(
                                *span,
                                "local functions and memo bindings under a handled computation are not supported in the bootstrap slice",
                            ));
                        }
                        names.push((name.text.clone(), name.span));
                    }
                    ast::LocalDecl::Bind { pat, span, .. } => {
                        if handled
                            && !matches!(pat, ast::Pat::Var(..) | ast::Pat::Wild(..))
                        {
                            return Err(Error::at(
                                *span,
                                "destructuring `let` bindings under a handled computation are not supported in the bootstrap slice",
                            ));
                        }
                        let mut ns = Vec::new();
                        collect_pat_names(pat, &mut ns);
                        names.extend(ns.into_iter().map(|n| (n, pat.span())));
                    }
                    ast::LocalDecl::Lazy { pat, span, .. } => {
                        if handled {
                            return Err(Error::at(
                                *span,
                                "local functions and memo bindings under a handled computation are not supported in the bootstrap slice",
                            ));
                        }
                        let mut ns = Vec::new();
                        collect_pat_names(pat, &mut ns);
                        names.extend(ns.into_iter().map(|n| (n, pat.span())));
                    }
                }
                for (n, s) in names {
                    if n != "_" && !seen.insert(n.clone()) {
                        return Err(Error::at(
                            s,
                            format!("duplicate binding `{n}` in the `let` group"),
                        ));
                    }
                }
            }
        }
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
                        let (core, ty) = self.elab_local_strict(
                            env,
                            body,
                            *span,
                            sigs.get(&name.text).cloned(),
                            form,
                        )?;
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
                    let (bc, bt) = self.elab_local_strict(env, body, *span, None, form)?;
                    let l = self.fresh_local();
                    match pat {
                        ast::Pat::Var(n, _) => {
                            let scheme =
                                self.generalize_local(env, &bt, is_syntactic_value(body));
                            self.bind_local(env, &n.text, l, scheme, None, n.span)?;
                        }
                        ast::Pat::Wild(_) => {
                            // The `Result` must-use default (decision
                            // D114's rule 7): `let _ = e` discards the
                            // value.
                            if !is_ignore_discard(body) {
                                self.check_must_use_discard(&bt, *span)?;
                            }
                            if matches!(form, LocalForm::DoStmt { .. }) {
                                // The discarded value keeps its
                                // multiplicity obligation (a linear value
                                // is never dropped silently).
                                self.register_obligation(l, "_", None, bt, *span);
                            }
                        }
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

    /// `letrec b1; ...; bn in e`: one recursive binding group (surface
    /// grammar local amendments section 9). Every use of a group name
    /// that closes a cycle sits under a lambda, a `~` memo body, or a
    /// `delay`; an eager cycle is a compile error. Recursive function
    /// uses become direct calls into the member's own code generation
    /// unit (so self tail calls stay jumps and mutual tail calls stay
    /// tail-marked, decision D104); recursive value uses read a fixpoint
    /// box filled at binding time.
    fn elab_letrec<'a>(
        &mut self,
        env: &mut Env,
        decls: &'a [ast::LocalDecl],
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

        // The members: name equations (`x = e`, `f pats = body`) and `~`
        // memo bindings. A value whose body is a lambda is the function
        // equation written with `=`; it compiles as one.
        struct RecMember<'a> {
            name: String,
            span: ast::Span,
            /// The function's parameters; empty for a value.
            pats: Vec<ast::Pat>,
            is_fun: bool,
            is_cell: bool,
            body: &'a ast::Term,
            l: LocalId,
            ty: Ty,
            boxed: Option<LocalId>,
            cell: Option<(EffRow, Ty)>,
        }
        let mut mems: Vec<RecMember> = Vec::new();
        let mut by_name: HashSet<String> = HashSet::new();
        fn rec_push<'a>(
            mems: &mut Vec<RecMember<'a>>,
            by_name: &mut HashSet<String>,
            name: String,
            span: ast::Span,
            pats: Vec<ast::Pat>,
            is_fun: bool,
            is_cell: bool,
            body: &'a ast::Term,
        ) -> Result<()> {
            if !name.is_empty() && !by_name.insert(name.clone()) {
                return Err(Error::at(
                    span,
                    format!("duplicate binding `{name}` in the `letrec` group"),
                ));
            }
            mems.push(RecMember {
                name,
                span,
                pats,
                is_fun,
                is_cell,
                body,
                l: LocalId(0),
                ty: Ty::Unit,
                boxed: None,
                cell: None,
            });
            Ok(())
        }
        for d in decls {
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
                    let (pats, body, is_fun) = match (pats.is_empty(), body) {
                        (true, ast::Term::Lam(ps, b, _)) => {
                            (ps.clone(), &**b as &ast::Term, true)
                        }
                        (false, _) => (pats.clone(), body, true),
                        (true, _) => (Vec::new(), body, false),
                    };
                    rec_push(
                        &mut mems,
                        &mut by_name,
                        name.text.clone(),
                        name.span,
                        pats,
                        is_fun,
                        false,
                        body,
                    )?;
                }
                ast::LocalDecl::Bind { pat, body, .. } => match pat {
                    ast::Pat::Var(n, _) => {
                        let (pats, body, is_fun) = match body {
                            ast::Term::Lam(ps, b, _) => (ps.clone(), &**b as &ast::Term, true),
                            _ => (Vec::new(), body, false),
                        };
                        rec_push(
                            &mut mems,
                            &mut by_name,
                            n.text.clone(),
                            n.span,
                            pats,
                            is_fun,
                            false,
                            body,
                        )?;
                    }
                    ast::Pat::Wild(_) => rec_push(
                        &mut mems,
                        &mut by_name,
                        String::new(),
                        pat.span(),
                        Vec::new(),
                        false,
                        false,
                        body,
                    )?,
                    other => {
                        return Err(Error::at(
                            other.span(),
                            "a `letrec` binding needs a name equation (`x = e`, `f pats = body`) or a `~` memo binding; destructuring binds are outside the recursive group",
                        ))
                    }
                },
                ast::LocalDecl::Lazy { pat, body, .. } => {
                    let n = match pat {
                        ast::Pat::Var(n, _) => n.clone(),
                        ast::Pat::Wild(_) => {
                            return Err(Error::at(pat.span(), "a memo binding must have a name"))
                        }
                        other => {
                            return Err(Error::at(
                                other.span(),
                                "memo bindings support only variable patterns in the bootstrap slice",
                            ))
                        }
                    };
                    rec_push(
                        &mut mems,
                        &mut by_name,
                        n.text.clone(),
                        n.span,
                        Vec::new(),
                        false,
                        true,
                        body,
                    )?;
                }
            }
        }

        // The memo group: all `~` bindings share one region and `Eval`
        // label (laziness and reclamation specification, section 5).
        let group = if mems.iter().any(|m| m.is_cell) {
            let region = self.fresh_region();
            let label = EffLabel::with_arg("Eval", region.clone());
            self.active_regions.push(region.clone());
            Some(MemoGroup { region, label })
        } else {
            None
        };

        // Pre-bind every member, so bodies see the whole group. The
        // group is monomorphic within itself.
        let env0 = env.clone();
        for m in &mut mems {
            m.l = self.fresh_local();
            if m.is_cell {
                let g = group.as_ref().unwrap();
                let eff = self.uni.fresh_row();
                let inner = self.uni.fresh();
                let cell_ty = Ty::Lazy {
                    eff: eff.clone(),
                    region: g.region.clone(),
                    inner: Box::new(inner.clone()),
                };
                m.cell = Some((eff, inner));
                if !m.name.is_empty() {
                    self.bind_local(env, &m.name, m.l, Scheme::mono(cell_ty), None, m.span)?;
                }
            } else {
                let ty = sigs.get(&m.name).cloned().unwrap_or_else(|| self.uni.fresh());
                m.ty = ty.clone();
                if !m.name.is_empty() {
                    self.bind_local(env, &m.name, m.l, Scheme::mono(ty), None, m.span)?;
                }
            }
        }

        // Elaborate the bodies. A value member of a cyclic group reads
        // through a fixpoint box, so every value member gets one.
        for m in &mut mems {
            if !m.is_cell && !m.is_fun && !m.name.is_empty() {
                m.boxed = Some(self.fresh_local());
            }
        }
        let mut cores: Vec<ECore> = Vec::new();
        for m in &mems {
            if m.is_cell {
                let (eff, inner) = m.cell.clone().unwrap();
                let g = group.as_ref().unwrap();
                let saved = self.enter_perf();
                self.memo_depth += 1;
                let checked = self.infer(env, m.body);
                self.memo_depth -= 1;
                let (bc, bt) = checked?;
                let performed = self.exit_perf(saved);
                self.uni
                    .require_lacks(&performed, g.label.clone())
                    .map_err(|e| {
                        Error::at(
                            m.span,
                            format!("{e}: this memo binding demands its own memo group"),
                        )
                    })?;
                self.uni
                    .unify_rows(&eff, &performed)
                    .map_err(|e| Error::at(m.span, e))?;
                self.uni
                    .unify(&inner, &bt)
                    .map_err(|e| Error::at(m.span, e))?;
                let unit_param = self.fresh_local();
                cores.push(ECore::Lam {
                    params: vec![unit_param],
                    body: Box::new(bc),
                });
            } else if m.is_fun {
                let lam =
                    ast::Term::Lam(m.pats.clone(), Box::new(m.body.clone()), m.span);
                cores.push(self.check_lam(env, &lam, &m.ty)?);
            } else {
                match sigs.get(&m.name) {
                    Some(t) => cores.push(self.check(env, m.body, t)?),
                    None => {
                        let (c, t) = self.infer(env, m.body)?;
                        self.uni
                            .unify(&t, &m.ty)
                            .map_err(|e| Error::at(m.span, e))?;
                        cores.push(c);
                    }
                }
            }
        }

        // The guarded-letrec rule (section 9): a use that closes a cycle
        // sits under a lambda, a `~` memo body, or a `delay`.
        let members: Vec<(LocalId, ast::Span, String)> =
            mems.iter().map(|m| (m.l, m.span, m.name.clone())).collect();
        check_rec_guards(&members, &cores)?;

        // Captured words for each function member's unit: free variables
        // from outside the group, siblings' boxes and cells, and
        // (transitively) siblings' captured words.
        let member_locals: HashSet<LocalId> = mems.iter().map(|m| m.l).collect();
        let mut by_local: HashMap<LocalId, usize> = HashMap::new();
        for (i, m) in mems.iter().enumerate() {
            by_local.insert(m.l, i);
        }
        let mut base: Vec<Vec<LocalId>> = Vec::new();
        let mut refs: Vec<Vec<usize>> = Vec::new();
        for core in cores.iter() {
            let (params, body): (&[LocalId], &ECore) = match core {
                ECore::Lam { params, body } => (params, body),
                other => (&[], other),
            };
            let mut bound: Vec<LocalId> = params.to_vec();
            bound.extend(member_locals.iter().copied());
            let mut free = Vec::new();
            free_locals(body, &mut bound, &mut free);
            base.push(free);
            let mut rs = Vec::new();
            rec_collect_refs(core, &by_local, &mut rs);
            refs.push(rs);
        }
        let mut captured: Vec<Vec<LocalId>> = vec![Vec::new(); mems.len()];
        loop {
            let mut changed = false;
            for (i, m) in mems.iter().enumerate() {
                if !m.is_fun {
                    continue;
                }
                let mut want = base[i].clone();
                for &r in &refs[i] {
                    let rm = &mems[r];
                    if rm.is_fun {
                        want.extend(captured[r].iter().copied());
                    } else if rm.is_cell {
                        want.push(rm.l);
                    } else if let Some(b) = rm.boxed {
                        want.push(b);
                    }
                }
                want.sort_by_key(|l| l.0);
                want.dedup();
                if want != captured[i] {
                    captured[i] = want;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        // Give each function member its code generation unit, then
        // rewrite the group's uses: direct calls and closures into the
        // member's unit, fixpoint box reads for recursive values.
        let mut subs: HashMap<LocalId, RecSub> = HashMap::new();
        let mut unit_ids: Vec<Option<GlobalId>> = Vec::new();
        for (i, m) in mems.iter().enumerate() {
            if m.is_fun {
                let id = GlobalId(format!(
                    "{}.letrec#{}",
                    self.current_module, self.lam_counter
                ));
                self.lam_counter += 1;
                subs.insert(m.l, RecSub::Fun(id.clone(), captured[i].clone()));
                unit_ids.push(Some(id));
            } else {
                if let Some(b) = m.boxed {
                    subs.insert(m.l, RecSub::Val(b));
                }
                unit_ids.push(None);
            }
        }
        for core in cores.iter_mut() {
            subst_rec_refs(core, &subs);
        }

        // The bindings: memo cells first, then the fixpoint boxes, then
        // the members (every reference is order-free: values go through
        // boxes and functions through their units).
        let mut out: Vec<(LocalId, ECore)> = Vec::new();
        for m in &mems {
            if m.is_cell {
                out.push((m.l, ECore::LazyAlloc));
            }
        }
        for m in &mems {
            if let Some(b) = m.boxed {
                out.push((b, ECore::BoxAlloc));
            }
        }
        for (i, m) in mems.iter().enumerate() {
            let core = cores[i].clone();
            if m.is_cell {
                let tmp = self.fresh_local();
                out.push((tmp, ECore::LazyInit(m.l, Box::new(core))));
            } else if m.is_fun {
                let id = unit_ids[i].clone().unwrap();
                out.push((
                    m.l,
                    ECore::MkClos {
                        fn_id: id.clone(),
                        env: captured[i].iter().map(|c| ECore::Var(*c)).collect(),
                    },
                ));
                self.pending_rec.push((id, core));
            } else if let Some(b) = m.boxed {
                out.push((m.l, ECore::BoxInit(b, Box::new(core))));
            } else {
                out.push((m.l, core));
            }
        }

        // The group is monomorphic within itself; its named members
        // generalize for the body like any other local binding.
        for m in &mems {
            if m.name.is_empty() || m.is_cell {
                continue;
            }
            let is_value = if m.is_fun {
                true
            } else if m.is_cell {
                false
            } else {
                is_syntactic_value(m.body)
            };
            let ty = self.uni.zonk(&m.ty);
            let scheme = self.generalize_local(&env0, &ty, is_value);
            self.bind_local(env, &m.name, m.l, scheme, None, m.span)?;
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
                    out.push((fl, ECore::Proj(Box::new(ECore::Var(l)), ProjField::slot(i))));
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
            idx_params: Vec::new(),
            val_params: Vec::new(),
            prop_params: Vec::new(),
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
        // Every alternative carries its weight beside it, and a guarded
        // one its guard's weight (attribute annotations specification
        // sections 2 and 3).
        let mut weights: Vec<Option<u32>> = Vec::new();
        let mut guard_weights: Vec<Option<u32>> = Vec::new();
        for a in alts {
            let set = self.validate_attr_site(&a.attrs, AttrSite::Alt)?;
            let gset = self.validate_attr_site(&a.guard_attrs, AttrSite::Guard)?;
            if gset.weight.is_some() && a.guard.is_none() {
                return Err(Error::at(
                    a.span,
                    "a guard's weight annotates a guard: write `pat | #[weight(n)] guard -> ...` (attribute annotations specification section 2)",
                ));
            }
            weights.push(set.weight);
            guard_weights.push(gset.weight);
        }
        self.complete_weights(&mut weights, "this `case`", span)?;
        let mut rows: Vec<Row> = Vec::new();
        for (i, a) in alts.iter().enumerate() {
            rows.push(Row {
                pats: vec![self.expand_pat(&a.pat)?],
                guard: a.guard.clone(),
                body: a.body.clone(),
                env: env.clone(),
                idx: i,
                consumed: Vec::new(),
                weight: weights[i],
                guard_weight: guard_weights[i],
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
                    self.note_use(*l, ctx_span, &[], &first.env.dicts)?;
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
                // The guard's own decision point (attribute annotations
                // specification sections 2.1 and 3): the guard's weight
                // annotates its pass outcome, and the reference base 1
                // completes the pair for the fallthrough.
                let gw = first.guard_weight;
                return Ok(ECore::If(
                    Box::new(gcore),
                    CBranch::weighted(gw, body_core),
                    CBranch::weighted(gw.map(|_| 1), rest_core),
                ));
            }
            for l in &first.consumed {
                self.note_use(*l, ctx_span, &[], &first.env.dicts)?;
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
            Ty::Ptr { role, .. } => {
                // Boundary pointers carry one role and have no
                // constructors to match; only variables and wildcards
                // (caught above).
                for r in &rows {
                    if !matches!(r.pats[0], ast::Pat::Var(..) | ast::Pat::Wild(_)) {
                        return Err(Error::at(
                            r.pats[0].span(),
                            format!("pattern does not match type {role} (a boundary pointer is matched by a variable or wildcard only)"),
                        ));
                    }
                }
                self.consume_column(rows, cols, types, expected, ctx_span, covered)
            }
            Ty::Int | Ty::Nat | Ty::Prim(_) | Ty::Sym => {
                // Integer literal patterns: a literal matches when the
                // column equals it; rows fall through in source order.
                // Machine scalars compare canonical words with `eqI64`
                // (every value has a canonical `i64` word, primitive types
                // specification, section 2, so word equality is value
                // equality); mathematical `Nat`/`Int` compare through the
                // runtime's `natCmp`/`intCmp` immediate. `Sym` literal
                // patterns test identity (the interned cell; primitive
                // types specification section 9). The comparison is
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
                        ast::Pat::Int(v, _) | ast::Pat::Sym(v, _) => {
                            // A `Sym` column takes `Sym` literals and
                            // every other column integer literals; the
                            // wrong shape is a type error, not a test.
                            if matches!(p0, ast::Pat::Sym(..)) != matches!(inner_ty, Ty::Sym) {
                                return Err(Error::at(
                                    p0.span(),
                                    format!("pattern does not match type {inner_ty}"),
                                ));
                            }
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
                // Branch weights (attribute annotations specification
                // section 2.2) marginalize over the literal alternatives
                // and the default; the nested comparisons see the rest
                // as their other alternative.
                let mut branch_lists: Vec<Vec<Row>> = Vec::new();
                for (v, _) in &literals {
                    let mut branch = defaults.clone();
                    branch.extend(
                        lit_rows.iter().filter(|(x, _)| x == v).map(|(_, r)| r.clone()),
                    );
                    branch.sort_by_key(|r| r.idx);
                    branch_lists.push(branch);
                }
                branch_lists.push(defaults.clone());
                let sums = self.decision_weights(&branch_lists, "this literal match", ctx_span)?;
                let (lit_sums, default_sum) = sums.split_at(literals.len());
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
                let mut rest_w = default_sum.first().copied().unwrap_or(None);
                for (i, (v, node)) in literals.into_iter().enumerate().rev() {
                    let branch = branch_lists[i].clone();
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
                    // One binary decision point per comparison: the
                    // literal's alternative against everything left.
                    let (tw, fw) = pair_weights(lit_sums[i], rest_w);
                    chain = ECore::If(
                        Box::new(cmp),
                        CBranch::weighted(tw, body),
                        CBranch::weighted(fw, chain),
                    );
                    rest_w = match (lit_sums[i], rest_w) {
                        (Some(a), Some(b)) => Some(a.saturating_add(b)),
                        _ => None,
                    };
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
                // Branch weights (attribute annotations specification
                // section 2.2): each side's weight is the sum of the
                // weights of the rows reachable through it.
                let sums = self.decision_weights(
                    &[true_rows.clone(), false_rows.clone()],
                    "this `Bool` match",
                    ctx_span,
                )?;
                let (tw, fw) = pair_weights(sums[0], sums[1]);
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
                Ok(ECore::If(
                    Box::new(col0),
                    CBranch::weighted(tw, t),
                    CBranch::weighted(fw, f),
                ))
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
                let mut parts_by_tag: Vec<Option<CtorParts>> =
                    (0..info.ctors.len()).map(|_| None).collect();
                let mut binder_maps: HashMap<usize, HashMap<String, String>> = HashMap::new();
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
                            let v = self.ctor_for_type(&info, p, *span)?;
                            let (tag, arity) = match &v.kind {
                                ValueKind::Ctor { tag, arity, .. } => (*tag, *arity),
                                _ => {
                                    return Err(Error::at(*span, format!("{} is not a constructor", v.name)))
                                }
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
                            // Index equations decide whether this
                            // constructor can build the type at all
                            // (dependent core kernel design, section 5).
                            if parts_by_tag[tag].is_none() {
                                match self.match_ctor(&v, &ty_args)? {
                                    CtorMatch::Possible(parts) => {
                                        parts_by_tag[tag] = Some(parts);
                                    }
                                    CtorMatch::Refuted => {}
                                    CtorMatch::Unknown(msg) => {
                                        return Err(Error::at(
                                            *span,
                                            format!(
                                                "cannot match constructor {} against {}: the index equation {msg} has no solution here (this slice does not refine rigid indices; reshape the type or dispatch through `decide`)",
                                                v.name, ty0
                                            ),
                                        ));
                                    }
                                }
                            }
                            if parts_by_tag[tag].is_none() {
                                return Err(Error::at(
                                    *span,
                                    format!(
                                        "impossible arm: constructor {} cannot build {} (the indices refute it), so this arm can never match",
                                        v.name, ty0
                                    ),
                                ));
                            }
                            let parts = parts_by_tag[tag].as_ref().unwrap();
                            // Dependent field types mention earlier field
                            // binders by name (Fin's `bound :: i < n`);
                            // every row must agree on those names.
                            let map = binder_maps.entry(tag).or_default();
                            for (i, sp) in args.iter().enumerate() {
                                let b = &parts.fields[i].0;
                                if let ast::Pat::Var(n, _) = sp {
                                    match map.get(b) {
                                        Some(prev) if *prev != n.text => {
                                            return Err(Error::at(
                                                n.span,
                                                format!(
                                                    "constructor {} has dependent field types: every row must bind the same names (field `{}` binds `{}` here but `{}` in another row)",
                                                    v.name, b, n.text, prev
                                                ),
                                            ))
                                        }
                                        _ => {
                                            map.insert(b.clone(), n.text.clone());
                                        }
                                    }
                                } else if parts
                                    .fields
                                    .iter()
                                    .enumerate()
                                    .any(|(j, (_, t, _))| j > i && ty_mentions(t, b))
                                {
                                    return Err(Error::at(
                                        sp.span(),
                                        format!(
                                            "a field whose name later fields mention must be a variable pattern (the type of a later field of {} mentions `{}`)",
                                            v.name, b
                                        ),
                                    ));
                                }
                            }
                            // Erased fields bind evidence names; their
                            // patterns never become runtime columns (core
                            // specification section 3).
                            let mut kept: Vec<ast::Pat> = Vec::new();
                            for (i, sp) in args.iter().enumerate() {
                                if parts.fields[i].2 {
                                    match sp {
                                        ast::Pat::Wild(_) => {}
                                        ast::Pat::Var(n, _) => {
                                            let l = self.fresh_local();
                                            self.bind_local(
                                                &mut r.env,
                                                &n.text,
                                                l,
                                                Scheme::mono(parts.fields[i].1.clone()),
                                                Some(Mult::Zero),
                                                n.span,
                                            )?;
                                        }
                                        other => {
                                            return Err(Error::at(
                                                other.span(),
                                                "an erased field binds evidence: only variable and wildcard patterns are supported",
                                            ))
                                        }
                                    }
                                } else {
                                    kept.push(sp.clone());
                                }
                            }
                            let mut a = r.clone();
                            self.mark_consumed(&mut a, &col0);
                            let mut pats = kept;
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
                let mut alts: Vec<ECore> = Vec::new();
                let mut alt_maps = Vec::new();
                let mut alt_pats: Vec<CPat> = Vec::new();
                let mut branch_lists: Vec<Vec<Row>> = Vec::new();
                for (name, tag) in info.ctors.clone() {
                    // A constructor with no explicit arm may still run
                    // (wildcard rows reach it), so it gets a branch unless
                    // the indices refute it.
                    let parts = match parts_by_tag[tag].take() {
                        Some(p) => p,
                        None => {
                            let v = self.ctor_of_type(&info, &name)?;
                            match self.match_ctor(&v, &ty_args)? {
                                CtorMatch::Possible(p) => p,
                                CtorMatch::Refuted => continue,
                                CtorMatch::Unknown(_) => self.ctor_parts(&v, &ty_args)?,
                            }
                        }
                    };
                    let runtime_fields: Vec<(String, Ty)> = parts
                        .fields
                        .iter()
                        .filter(|(_, _, e)| !*e)
                        .map(|(b, t, _)| (b.clone(), t.clone()))
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
                    // Dependent field types mention earlier field binders
                    // by name; the rows' pattern names substituted in.
                    let bmap = binder_maps.get(&tag).cloned().unwrap_or_default();
                    let field_tys: Vec<Ty> = runtime_fields
                        .iter()
                        .map(|(_, t)| wrap(&subst_idx_names(t, &bmap)))
                        .collect();
                    let mut branch_cols: Vec<ECore> =
                        fields.iter().map(|l| ECore::Var(*l)).collect();
                    branch_cols.extend(cols[1..].iter().cloned());
                    let mut branch_tys: Vec<Ty> = field_tys;
                    branch_tys.extend(types[1..].iter().cloned());
                    let (core, m) = self.isolate(|s| {
                        s.compile_rows(
                            branch.clone(),
                            branch_cols,
                            branch_tys,
                            expected,
                            ctx_span,
                            covered,
                        )
                    })?;
                    branch_lists.push(branch);
                    alt_maps.push(m);
                    alt_pats.push(CPat::Con { tag, fields });
                    alts.push(core);
                }
                self.merge_branches(alt_maps);
                if alts.is_empty() {
                    // Every constructor is refuted: the match runs on no
                    // value (an impossible scrutinee type).
                    return Err(Error::at(
                        ctx_span,
                        format!("no constructor of {} can build this value (the indices refute them all)", ty0),
                    ));
                }
                // Branch weights (attribute annotations specification
                // section 2.2): each constructor's alternative sums the
                // weights of the rows reachable through it.
                let sums = self.decision_weights(&branch_lists, "this `case`", ctx_span)?;
                let alts = alt_pats
                    .into_iter()
                    .zip(alts)
                    .zip(sums)
                    .map(|((pat, body), w)| CAlt::weighted(pat, w, body))
                    .collect();
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
                    alts: vec![CAlt::new(CPat::Con { tag: 0, fields }, core)],
                })
            }
            Ty::Param(p) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a value of abstract type {p}"),
            )),
            Ty::App(h, _) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a value of applied abstract type {h}"),
            )),
            Ty::Former(f) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on the type former {:?}", f),
            )),
            Ty::Fun(a, _, _, b) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a function value ({a} -> {b})"),
            )),
            Ty::Thunk { inner, .. } => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a thunk (Thunk _ {inner}); force it first"),
            )),
            Ty::Later(inner) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a Later ({inner}); force it first"),
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
            Ty::Region(r) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a runtime identity ({r})"),
            )),
            Ty::Handler { .. } => Err(Error::at(
                ctx_span,
                "cannot pattern match on a handler value in the bootstrap slice",
            )),
            Ty::Sigma { var, dom, cod } => {
                // A dependent pair destructures like a tuple; the first
                // component's pattern name ties the columns together (the
                // second component's type mentions it).
                let mut sub_rows: Vec<Row> = Vec::new();
                let mut var_rows: Vec<Row> = Vec::new();
                let mut binder: Option<String> = None;
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
                            if pats.len() != 2 {
                                return Err(Error::at(
                                    *span,
                                    format!(
                                        "a dependent pair pattern has two components, got {}",
                                        pats.len()
                                    ),
                                ));
                            }
                            let nm = match &pats[0] {
                                ast::Pat::Var(n, _) => n.text.clone(),
                                ast::Pat::Wild(_) => var.clone(),
                                other => {
                                    return Err(Error::at(
                                        other.span(),
                                        "the first component of a dependent pair pattern must be a variable or wildcard (its name ties the pair together)",
                                    ))
                                }
                            };
                            match &binder {
                                None => binder = Some(nm),
                                Some(b) if *b == nm => {}
                                Some(b) => {
                                    return Err(Error::at(
                                        *span,
                                        format!(
                                            "every row must bind the same name for a dependent pair's first component (`{nm}` here, `{b}` in another row)"
                                        ),
                                    ))
                                }
                            }
                            self.mark_consumed(&mut r, &col0);
                            let mut ps = pats.clone();
                            ps.extend(r.pats[1..].iter().cloned());
                            r.pats = ps;
                            sub_rows.push(r);
                        }
                        other => {
                            return Err(Error::at(
                                other.span(),
                                format!("pattern does not match type {ty0}"),
                            ));
                        }
                    }
                }
                let name = binder.unwrap_or_else(|| var.clone());
                let fields: Vec<LocalId> = (0..2).map(|_| self.fresh_local()).collect();
                if let ECore::Var(l) = &col0 {
                    for f in &fields {
                        self.smaller.insert(*f, *l);
                    }
                }
                let mut branch = sub_rows;
                for r in &var_rows {
                    let mut a = r.clone();
                    let mut pats = vec![ast::Pat::Wild(ctx_span); 2];
                    pats.extend(a.pats.iter().cloned());
                    a.pats = pats;
                    branch.push(a);
                }
                branch.sort_by_key(|r| r.idx);
                let mut branch_cols: Vec<ECore> = fields.iter().map(|l| ECore::Var(*l)).collect();
                branch_cols.extend(cols[1..].iter().cloned());
                let mut branch_tys: Vec<Ty> = vec![
                    wrap(&dom),
                    wrap(&cod.subst_idx(&var, &ITerm::Var(name))),
                ];
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
                    alts: vec![CAlt::new(CPat::Con { tag: 0, fields }, core)],
                })
            }
            Ty::Pi { .. } => Err(Error::at(
                ctx_span,
                "cannot pattern match on a dependent function value",
            )),
            Ty::Sort(s) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on the universe {s}"),
            )),
            Ty::Idx(t) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on the index value {t}"),
            )),
            Ty::Prp(p) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on the proposition {p} (evidence is erased)"),
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
                if self
                    .codata_types
                    .contains(&(info.module.clone(), info.name.clone()))
                {
                    return Err(Error::at(
                        *span,
                        "a codata value is observed, not destructured: use `.field` projection (codata has no constructor patterns)",
                    ));
                }
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

    /// Resolve a constructor pattern against the type being matched,
    /// preferring the type's own constructors so `Nil` and `Cons` match
    /// `Vec n a` even though the prelude's `List` owns those names. The
    /// constructor must belong to that type; before this check, a foreign
    /// constructor was silently matched against a same-position tag of the
    /// scrutinee's type (review P0).
    fn ctor_for_type(&self, info: &TypeKind, p: &ast::ConPath, span: ast::Span) -> Result<ValueInfo> {
        if info.refine.is_some() {
            return Err(Error::at(
                span,
                format!(
                    "{} is a refinement's transparent constructor: it has no constructor patterns (project with `unrefine` and match the container's constructors)",
                    p.parts().join(".")
                ),
            ));
        }
        if self
            .codata_types
            .contains(&(info.module.clone(), info.name.clone()))
        {
            return Err(Error::at(
                span,
                "a codata value is observed, not destructured: use `.field` projection (codata has no constructor patterns)",
            ));
        }
        let parts = p.parts();
        let last = parts[parts.len() - 1];
        if parts.len() == 1 || parts[0] == info.module {
            if info.ctors.iter().any(|(n, _)| n == last) {
                if let Ok(v) = self.ctor_of_type(info, last) {
                    if matches!(v.kind, ValueKind::Ctor { .. }) {
                        let mut v = v;
                        v.name = last.to_string();
                        return Ok(v);
                    }
                }
            }
        }
        let v = self.resolve_con(p, span)?;
        match &v.kind {
            ValueKind::Ctor { tag, .. } => {
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
                Ok(v)
            }
            _ => Err(Error::at(span, format!("{} is not a constructor", v.name))),
        }
    }

    /// A constructor's fields and result at the type arguments of the
    /// matched type: family parameters substitute positionally, and the
    /// constructor-local index binders (`Cons`'s `k`) remain for the
    /// caller to solve.
    fn ctor_parts(&self, v: &ValueInfo, ty_args: &[Ty]) -> Result<CtorParts> {
        if self
            .codata_types
            .contains(&(v.module.clone(), v.name.clone()))
        {
            return Err(Error::plain(
                "a codata value is observed, not destructured: use `.field` projection (codata has no constructor patterns)",
            ));
        }
        match &v.kind {
            ValueKind::Ctor {
                field_tys,
                field_erased,
                field_binders,
                ty_params,
                local_idx,
                res_args,
                ..
            } => {
                let mut map = HashMap::new();
                for (p, a) in ty_params.iter().zip(ty_args) {
                    map.insert(p.clone(), a.clone());
                }
                let fields = field_tys
                    .iter()
                    .enumerate()
                    .map(|(i, t)| {
                        (field_binders[i].clone(), Self::subst_ty(t, &map), field_erased[i])
                    })
                    .collect();
                Ok(CtorParts {
                    fields,
                    local_idx: local_idx.clone(),
                    res_args: res_args.iter().map(|t| Self::subst_ty(t, &map)).collect(),
                })
            }
            _ => Err(Error::plain(format!("{} is not a constructor", v.name))),
        }
    }

    /// Can this constructor build the matched type? Its result indices
    /// must unify with the type's (dependent core kernel design, section
    /// 5): solvable equations make it possible, a ground contradiction or
    /// an out-of-range solution refutes it, and an equation stuck on rigid
    /// indices is unknown (this slice rejects those matches rather than
    /// guessing; it does not yet refine rigid indices in a branch).
    fn match_ctor(&mut self, v: &ValueInfo, ty_args: &[Ty]) -> Result<CtorMatch> {
        let parts = self.ctor_parts(v, ty_args)?;
        let mut meta_of: HashMap<String, Ty> = HashMap::new();
        for p in &parts.local_idx {
            let m = self.uni.fresh_idx();
            meta_of.insert(p.name().to_string(), m);
        }
        // Solve the index equations on a trial substitution and commit it
        // on success: the metavariables must belong to the unifier the
        // branch elaboration continues with.
        let mut trial = self.uni.clone();
        let res: Vec<Ty> = parts
            .res_args
            .iter()
            .map(|t| Self::subst_ty(t, &meta_of))
            .collect();
        for (a, b) in res.iter().zip(ty_args.iter()) {
            if trial.unify(a, b).is_ok() {
                continue;
            }
            let za = trial.zonk(a);
            let zb = trial.zonk(b);
            if idx_ground(&za) && idx_ground(&zb) {
                return Ok(CtorMatch::Refuted);
            }
            return Ok(CtorMatch::Unknown(format!("{za} vs {zb}")));
        }
        // An inversion guess like `?k + 1 = 0` solves with `?k := -1`,
        // which is not a natural number: the constructor is impossible.
        for p in &parts.local_idx {
            if let IxParam::Val(n, t) = p {
                if *t == Ty::Nat {
                    if let Some(Ty::Idx(it)) = meta_of.get(n) {
                        let solved = trial.resolve_idx(it).normalize();
                        if let ITerm::Num(d) = &solved {
                            if d.parse::<i128>().map(|v| v < 0).unwrap_or(false) {
                                return Ok(CtorMatch::Refuted);
                            }
                        }
                    }
                }
            }
        }
        let fields = parts
            .fields
            .into_iter()
            .map(|(b, t, e)| (b, Self::subst_ty(&t, &meta_of), e))
            .collect();
        let res_args = res.clone();
        self.uni = trial;
        Ok(CtorMatch::Possible(CtorParts {
            fields,
            local_idx: parts.local_idx,
            res_args,
        }))
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
    fn matrix_covered(&mut self, rows: &[Row], types: &[Ty]) -> Result<bool> {
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
                let mut parts_by_tag: Vec<Option<CtorParts>> =
                    (0..info.ctors.len()).map(|_| None).collect();
                for r in &rows {
                    match &r.pats[0] {
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => {
                            var_rows.push((r.pats[0].span(), self.drop_row_col(r)));
                        }
                        ast::Pat::Con(p, args, _) => {
                            // A constructor from another type means this
                            // prover cannot claim coverage.
                            let v = match self.ctor_for_type(&info, p, r.pats[0].span()) {
                                Ok(v) => v,
                                Err(_) => return Ok(false),
                            };
                            let (tag, arity) = match &v.kind {
                                ValueKind::Ctor { tag, arity, .. } => (*tag, *arity),
                                _ => return Ok(false),
                            };
                            if args.len() != arity || tag >= ctor_rows.len() {
                                return Ok(false);
                            }
                            // The indices decide whether the constructor
                            // can build the type at all: an arm for a
                            // refuted constructor is impossible, and a
                            // match stuck on rigid indices is not
                            // provable here.
                            if parts_by_tag[tag].is_none() {
                                match self.match_ctor(&v, ty_args) {
                                    Ok(CtorMatch::Possible(parts)) => {
                                        parts_by_tag[tag] = Some(parts);
                                    }
                                    Ok(CtorMatch::Refuted) => {
                                        return Err(Error::at(
                                            r.pats[0].span(),
                                            format!(
                                                "impossible arm: constructor {} cannot build {} (the indices refute it), so this arm can never match",
                                                v.name, inner_ty
                                            ),
                                        ))
                                    }
                                    Ok(CtorMatch::Unknown(msg)) => {
                                        return Err(Error::at(
                                            r.pats[0].span(),
                                            format!(
                                                "cannot match constructor {} against {}: the index equation {msg} has no solution here (this slice does not refine rigid indices)",
                                                v.name, inner_ty
                                            ),
                                        ))
                                    }
                                    Err(_) => return Ok(false),
                                }
                            }
                            let parts = parts_by_tag[tag].as_ref().unwrap();
                            // Erased fields bind evidence by name; they
                            // cover themselves and take no column.
                            let mut kept: Vec<ast::Pat> = Vec::new();
                            for (i, sp) in args.iter().enumerate() {
                                if !parts.fields[i].2 {
                                    kept.push(sp.clone());
                                }
                            }
                            let mut a = self.drop_row_col(r);
                            let mut pats = kept;
                            pats.append(&mut a.pats);
                            a.pats = pats;
                            ctor_rows[tag].push(a);
                        }
                        _ => return Ok(false),
                    }
                }
                for (name, tag) in &info.ctors {
                    // A constructor with no explicit arm may still run
                    // (wildcard rows reach it), so it must be covered
                    // unless the indices refute it.
                    let parts = match parts_by_tag[*tag].take() {
                        Some(p) => p,
                        None => {
                            let v = match self.ctor_of_type(&info, name) {
                                Ok(v) => v,
                                Err(_) => return Ok(false),
                            };
                            match self.match_ctor(&v, ty_args) {
                                Ok(CtorMatch::Possible(p)) => p,
                                Ok(CtorMatch::Refuted) => continue,
                                Ok(CtorMatch::Unknown(_)) => {
                                    match self.ctor_parts(&v, ty_args) {
                                        Ok(p) => p,
                                        Err(_) => return Ok(false),
                                    }
                                }
                                Err(_) => return Ok(false),
                            }
                        }
                    };
                    let arity = parts.fields.iter().filter(|(_, _, e)| !*e).count();
                    let mut branch = std::mem::take(&mut ctor_rows[*tag]);
                    for (span, r) in &var_rows {
                        let mut a = r.clone();
                        let mut pats = vec![ast::Pat::Wild(*span); arity];
                        pats.append(&mut a.pats);
                        a.pats = pats;
                        branch.push(a);
                    }
                    let field_tys: Vec<Ty> = parts
                        .fields
                        .iter()
                        .filter(|(_, _, e)| !*e)
                        .map(|(_, t, _)| t.clone())
                        .collect();
                    let mut tys: Vec<Ty> = field_tys.iter().map(|t| wrap(t)).collect();
                    tys.extend(types[1..].iter().cloned());
                    if !self.matrix_covered(&branch, &tys)? {
                        return Ok(false);
                    }
                    let _ = name;
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
                            pats.extend(a.pats.iter().cloned());
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
            Ty::Sigma { var, dom, cod } => {
                let mut sub_rows: Vec<Row> = Vec::new();
                for r in &rows {
                    match &r.pats[0] {
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => {
                            let span = r.pats[0].span();
                            let mut a = self.drop_row_col(r);
                            let mut pats = vec![ast::Pat::Wild(span); 2];
                            pats.extend(a.pats.iter().cloned());
                            a.pats = pats;
                            sub_rows.push(a);
                        }
                        ast::Pat::Tuple(pats, _) if pats.len() == 2 => {
                            let mut a = self.drop_row_col(r);
                            let mut ps = pats.clone();
                            ps.append(&mut a.pats);
                            a.pats = ps;
                            sub_rows.push(a);
                        }
                        _ => return Ok(false),
                    }
                }
                let mut tys: Vec<Ty> = vec![
                    wrap(&dom),
                    wrap(&cod.subst_idx(&var, &ITerm::Var(var.clone()))),
                ];
                tys.extend(types[1..].iter().cloned());
                self.matrix_covered(&sub_rows, &tys)
            }
            _ => Ok(false),
        }
    }
}

// ------------------------------------------------------------ effects, handlers, do (P6)

impl Checker {
    /// Bind a continuation closure `\v -> body` to a fresh local and run
    /// `then` with it. The body runs at most once per execution of its
    /// build site, so it is elaborated at a continuation position (usage
    /// counting; ownership and usage design section 2).
    fn bind_cont(
        &mut self,
        env: &mut Env,
        kind: ContKind,
        risky: bool,
        make_body: impl FnOnce(&mut Self, &mut Env, LocalId, Ty) -> Result<ECore>,
        make_then: impl FnOnce(&mut Self, &mut Env, &Cont) -> Result<ECore>,
    ) -> Result<ECore> {
        let kv = self.fresh_local();
        let v = self.fresh_local();
        let ty = self.uni.fresh();
        let frame = self.conts.len();
        self.conts.push(ContFrame { kind, risky, consumes_linear: false });
        self.cont_depth += 1;
        let body = make_body(self, env, v, ty.clone());
        self.cont_depth -= 1;
        let body = body?;
        let then = make_then(self, env, &Cont { kv, ty, frame })?;
        Ok(ECore::Let(
            kv,
            Box::new(ECore::Lam { params: vec![v], body: Box::new(body) }),
            Box::new(then),
        ))
    }

    /// Does this subterm contain a call to an effect operation? Operation
    /// calls must be elaborated in continuation-passing style to capture
    /// their resumption; this syntactic check is the conservative gate.
    fn term_may_dispatch(&self, t: &ast::Term) -> bool {
        fn walk(chk: &Checker, t: &ast::Term, found: &mut bool) {
            if *found {
                return;
            }
            match t {
                ast::Term::Var(p) => {
                    if chk.effect_ops.keys().any(|(m, n)| {
                        n == &p.name.text
                            && (p.quals.is_empty() || p.quals[0].text == *m)
                    }) {
                        *found = true;
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
                | ast::Term::SeqOp(_, _) => {}
                ast::Term::Quote { body, .. } | ast::Term::Splice { body, .. } => {
                    walk(chk, body, found)
                }
                ast::Term::QuoteDecls { .. } => {}
                ast::Term::App(f, args, _) => {
                    walk(chk, f, found);
                    for a in args {
                        walk(chk, a, found);
                    }
                }
                ast::Term::Neg(e, _) => walk(chk, e, found),
                ast::Term::Lam(_, b, _) => walk(chk, b, found),
                ast::Term::Let(decls, b, _) | ast::Term::Letrec(decls, b, _) => {
                    for d in decls {
                        match d {
                            ast::LocalDecl::Sig { .. } => {}
                            ast::LocalDecl::Eq { body, .. }
                            | ast::LocalDecl::Bind { body, .. }
                            | ast::LocalDecl::Lazy { body, .. } => walk(chk, body, found),
                        }
                    }
                    walk(chk, b, found);
                }
                ast::Term::If { cond: c, then: x, els: y, .. } => {
                    walk(chk, c, found);
                    walk(chk, x, found);
                    walk(chk, y, found);
                }
                ast::Term::Case(s, alts, _) => {
                    walk(chk, s, found);
                    for a in alts {
                        walk(chk, &a.body, found);
                    }
                }
                ast::Term::Ann(e, _, _) | ast::Term::Proj(e, _, _) => walk(chk, e, found),
                ast::Term::Infix(ts, _, _) => {
                    for x in ts {
                        walk(chk, x, found);
                    }
                }
                ast::Term::Tuple(items, _) | ast::Term::List(items, _) => {
                    for x in items {
                        walk(chk, x, found);
                    }
                }
                ast::Term::Map(kvs, _) => {
                    for (k, v) in kvs {
                        walk(chk, k, found);
                        walk(chk, v, found);
                    }
                }
                ast::Term::Range(start, step, end, _) => {
                    walk(chk, start, found);
                    if let Some(x) = step {
                        walk(chk, x, found);
                    }
                    walk(chk, end, found);
                }
                ast::Term::Record(_, fields, _) => {
                    for (_, x) in fields {
                        walk(chk, x, found);
                    }
                }
                ast::Term::Do(stmts, _) => {
                    for s in stmts {
                        match s {
                            ast::DoStmt::Bind { term, .. } | ast::DoStmt::Expr { term, .. } => {
                                walk(chk, term, found)
                            }
                            ast::DoStmt::Let { decls, .. } => {
                                for decl in decls {
                                    match decl {
                                        ast::LocalDecl::Sig { .. } => {}
                                        ast::LocalDecl::Eq { body, .. }
                                        | ast::LocalDecl::Bind { body, .. }
                                        | ast::LocalDecl::Lazy { body, .. } => {
                                            walk(chk, body, found)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ast::Term::Handle(e, h, _) => {
                    walk(chk, e, found);
                    walk(chk, h, found);
                }
                ast::Term::Handler(h) => {
                    walk(chk, &h.ret_body, found);
                    for c in &h.clauses {
                        walk(chk, &c.body, found);
                    }
                }
                ast::Term::With { value, body, .. } => {
                    walk(chk, value, found);
                    walk(chk, body, found);
                }
                ast::Term::CapApply { head, supplies, .. } => {
                    walk(chk, head, found);
                    for (_, x) in supplies {
                        walk(chk, x, found);
                    }
                }
                ast::Term::CpsWrap(b, _) => walk(chk, b, found),
            }
        }
        let mut found = false;
        walk(self, t, &mut found);
        found
    }

    /// Elaborate `term` in continuation-passing style: every value it
    /// computes flows into the continuation `kv`, and operation calls
    /// capture their continuation as the one-shot resumption given to the
    /// handler clause (design section 3).
    fn cps(&mut self, env: &mut Env, term: &ast::Term, kv: &Cont) -> Result<ECore> {
        if !self.term_may_dispatch(term) {
            // No operation can fire here: ordinary evaluation, then flow.
            let (core, ty) = self.infer(env, term)?;
            self.uni
                .unify(&ty, &kv.ty)
                .map_err(|m| Error::at(term.span(), m))?;
            return Ok(app_spine(ECore::Var(kv.kv), vec![core]));
        }
        match term {
            ast::Term::App(f, args, span) => self.cps_app(env, f, args, kv, *span),
            ast::Term::Neg(e, span) => {
                let negate = ast::Term::Var(ast::VarPath {
                    quals: Vec::new(),
                    name: ast::Name::new("negate", *span),
                });
                let app = ast::Term::App(Box::new(negate), vec![(**e).clone()], *span);
                self.cps(env, &app, kv)
            }
            ast::Term::If { cond: c, then: t, els: e, cond_attrs, then_attrs, else_attrs, span } => {
                let (tw, fw) = self.if_weights(cond_attrs, then_attrs, else_attrs, *span)?;
                let tc = (**t).clone();
                let ec = (**e).clone();
                let kv2 = kv.clone();
                let cond = (**c).clone();
                let cond2 = (**c).clone();
                self.bind_cont(
                    env,
                    ContKind::Once,
                    self.term_may_dispatch(c),
                    move |this, env, v, vty| {
                        this.uni
                            .unify(&vty, &Ty::Bool)
                            .map_err(|m| Error::at(cond2.span(), m))?;
                        let (tcore, tm) = this.isolate(|s| s.cps(env, &tc, &kv2))?;
                        let (ecore, em) = this.isolate(|s| s.cps(env, &ec, &kv2))?;
                        this.merge_branches(vec![tm, em]);
                        Ok(ECore::If(
                            Box::new(ECore::Var(v)),
                            CBranch::weighted(tw, tcore),
                            CBranch::weighted(fw, ecore),
                        ))
                    },
                    move |this, env, k| this.cps(env, &cond, k),
                )
            }
            ast::Term::Case(scrut, alts, span) => self.cps_case(env, scrut, alts, kv, *span),
            ast::Term::Let(decls, body, span) => self.cps_let(env, decls, body, kv, *span),
            ast::Term::Letrec(decls, body, span) => {
                self.cps_letrec(env, decls, body, kv, *span)
            }
            ast::Term::Do(stmts, span) => {
                let shape = self.result_shape(env, *span)?;
                self.elab_do(env, stmts, &shape, Some(kv), *span)
            }
            ast::Term::Handle(e, h, _) => {
                let (core, _) = self.handle_expr(env, e, h, Some(kv))?;
                Ok(core)
            }
            ast::Term::With { name, value, body, span } => {
                let name2 = name.clone();
                let body2 = (**body).clone();
                let kv2 = kv.clone();
                let sp = *span;
                let val = (**value).clone();
                self.bind_cont(
                    env,
                    ContKind::Once,
                    self.term_may_dispatch(value),
                    move |this, env, v, vty| {
                        let (l, bind, region) =
                            this.bind_capability(env, &name2, ECore::Var(v), vty, sp)?;
                        let code = this.cps(env, &body2, &kv2)?;
                        this.check_cap_escape(&region, &kv2.ty, sp)?;
                        Ok(wrap_lets(vec![(l, bind)], code))
                    },
                    move |this, env, k| this.cps(env, &val, k),
                )
            }
            ast::Term::Handler(h) => {
                // Handler construction evaluates no computation (the
                // clauses are closures), so it is a plain value here.
                let (core, ty) = self.handler_value(env, h)?;
                self.uni
                    .unify(&ty, &kv.ty)
                    .map_err(|m| Error::at(h.span, m))?;
                Ok(app_spine(ECore::Var(kv.kv), vec![core]))
            }
            ast::Term::CpsWrap(b, _) => self.cps(env, b, kv),
            // Container forms with operations inside: bind each subterm to
            // a fresh local in evaluation order, then rebuild the term over
            // the locals and run its ordinary elaboration.
            ast::Term::Ann(_, _, _)
            | ast::Term::Tuple(_, _)
            | ast::Term::List(_, _)
            | ast::Term::Map(_, _)
            | ast::Term::Range(_, _, _, _)
            | ast::Term::Record(_, _, _)
            | ast::Term::Proj(_, _, _)
            | ast::Term::CapApply { .. }
            | ast::Term::Infix(_, _, _) => self.cps_rebuild(env, term, kv),
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
            | ast::Term::Quote { .. }
            | ast::Term::Splice { .. }
            | ast::Term::Lam(_, _, _)
            | ast::Term::SeqOp(_, _) => {
                // Leaves and lambdas: an operation inside a lambda body
                // does not dispatch (the closure may run anywhere), so the
                // ordinary elaboration is exact here.
                let (core, ty) = self.infer(env, term)?;
                self.uni
                    .unify(&ty, &kv.ty)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(app_spine(ECore::Var(kv.kv), vec![core]))
            }
        }
    }
}

impl Checker {
    /// Application in continuation-passing style: the head, then the
    /// arguments in evaluation order, each into a continuation; the final
    /// body performs the application and flows to `kv`. An operation call
    /// dispatches to the enclosing handler instead.
    fn cps_app(
        &mut self,
        env: &mut Env,
        f: &ast::Term,
        args: &[ast::Term],
        kv: &Cont,
        span: ast::Span,
    ) -> Result<ECore> {
        if let ast::Term::Var(p) = f {
            if p.quals.is_empty() && env.lookup(&p.name.text).is_none() {
                if self.is_provider_pure(env, &p.name.text) {
                    let seq = ast::Term::App(
                        Box::new(ast::Term::SeqOp(ast::SeqKind::Pure, span)),
                        args.to_vec(),
                        span,
                    );
                    return self.cps(env, &seq, kv);
                }
                let op = self.resolve_var(p, p.span()).ok().filter(|v| {
                    matches!(v.kind, ValueKind::EffectOp { .. })
                });
                if let Some(v) = op {
                    return self.cps_op(env, &v, args, kv, span);
                }
            }
        }
        let fc = f.clone();
        self.bind_cont(
            env,
            ContKind::Once,
            self.term_may_dispatch(f),
            move |this, env, v_f, ty_f| {
                this.cps_apply_args(env, v_f, ty_f, args, kv, span, Vec::new())
            },
            move |this, env, k| this.cps(env, &fc, k),
        )
    }

    /// Argument coercion at application and check sites (foreign
    /// function interface specification section 3): an owned boundary
    /// pointer may flow into a borrow, retained or raw position, because
    /// ownership subsumes a borrow of the same pointer. Every other role
    /// pair unifies exactly, so borrowed data is never claimed as owned
    /// and allocator identities never convert.
    fn unify_arg(&mut self, inferred: &Ty, expected: &Ty) -> std::result::Result<(), String> {
        if let (
            Ty::Ptr { role: ar, pointee: ap },
            Ty::Ptr { role: pr, pointee: pp },
        ) = (inferred, expected)
        {
            let weakens = matches!(ar, crate::types::PtrRole::Owned(_))
                && matches!(
                    pr,
                    crate::types::PtrRole::Borrowed
                        | crate::types::PtrRole::Retained
                        | crate::types::PtrRole::Raw
                );
            if weakens {
                return self.uni.unify(ap, pp);
            }
        }
        self.uni.unify(inferred, expected)
    }

    fn cps_apply_args(
        &mut self,
        env: &mut Env,
        v_f: LocalId,
        ty_f: Ty,
        args: &[ast::Term],
        kv: &Cont,
        span: ast::Span,
        mut done: Vec<(LocalId, Ty)>,
    ) -> Result<ECore> {
        match args.split_first() {
            None => {
                // Perform the application and flow the result to the
                // continuation.
                let mut ty = ty_f;
                for (_, aty) in &done {
                    ty = self.uni.zonk(&ty);
                    let (dom, eff, cod) = match ty {
                        Ty::Fun(dom, _, eff, cod) => (*dom, eff, *cod),
                        Ty::Meta(_) => {
                            // The head's type is not yet determined (a
                            // continuation body elaborates before its
                            // subterm): open a fresh arrow.
                            let dom = self.uni.fresh();
                            let eff = self.uni.fresh_row();
                            let cod = self.uni.fresh();
                            self.uni
                                .unify(
                                    &ty,
                                    &Ty::fun_mult(
                                        dom.clone(),
                                        Mult::Flex,
                                        eff.clone(),
                                        cod.clone(),
                                    ),
                                )
                                .map_err(|m| Error::at(span, m))?;
                            (dom, eff, cod)
                        }
                        other => {
                            return Err(Error::at(
                                span,
                                format!("cannot apply a value of type {other}"),
                            ))
                        }
                    };
                    self.perform(&eff)?;
                    self.unify_arg(aty, &dom)
                        .map_err(|m| Error::at(span, m))?;
                    ty = cod;
                }
                self.uni
                    .unify(&ty, &kv.ty)
                    .map_err(|m| Error::at(span, m))?;
                let mut core = ECore::Var(v_f);
                for (l, _) in &done {
                    core = app_spine(core, vec![ECore::Var(*l)]);
                }
                Ok(app_spine(ECore::Var(kv.kv), vec![core]))
            }
            Some((arg, rest)) => {
                let ac = arg.clone();
                let restv = rest.to_vec();
                let kv2 = kv.clone();
                let ty2 = ty_f.clone();
                self.bind_cont(
                    env,
                    ContKind::Once,
                    self.term_may_dispatch(arg),
                    move |this, env, v_a, ty_a| {
                        done.push((v_a, ty_a));
                        this.cps_apply_args(env, v_f, ty2, &restv, &kv2, span, done)
                    },
                    move |this, env, k| this.cps(env, &ac, k),
                )
            }
        }
    }

    /// An effect operation call: evaluate the arguments in order, then
    /// dispatch through the enclosing handler's vtable, handing the clause
    /// the one-shot resumption (the continuation of this call). Without an
    /// enclosing handler for the label the call is the defined unhandled
    /// fault path (decision D23): its label still surfaces in the
    /// performed row, so effect honesty rejects unhandled operations at
    /// check time before any program can run one.
    fn cps_op(
        &mut self,
        env: &mut Env,
        v: &ValueInfo,
        args: &[ast::Term],
        kv: &Cont,
        span: ast::Span,
    ) -> Result<ECore> {
        let (op_ty, ctx, _) = self.instantiate(&v.scheme);
        if !ctx.is_empty() {
            return Err(Error::at(
                span,
                format!("effect operation {} takes no class constraints in the bootstrap slice", v.name),
            ));
        }
        self.cps_op_args(env, v, op_ty, args, kv, span, Vec::new())
    }

    fn cps_op_args(
        &mut self,
        env: &mut Env,
        v: &ValueInfo,
        op_ty: Ty,
        args: &[ast::Term],
        kv: &Cont,
        span: ast::Span,
        mut done: Vec<(LocalId, Ty)>,
    ) -> Result<ECore> {
        match args.split_first() {
            None => self.cps_op_dispatch(env, v, op_ty, done, kv, span),
            Some((arg, rest)) => {
                let ac = arg.clone();
                let restv = rest.to_vec();
                let kv2 = kv.clone();
                let ty2 = op_ty.clone();
                self.bind_cont(
                    env,
                    ContKind::Once,
                    self.term_may_dispatch(arg),
                    move |this, env, v_a, ty_a| {
                        done.push((v_a, ty_a));
                        this.cps_op_args(env, v, ty2, &restv, &kv2, span, done)
                    },
                    move |this, env, k| this.cps(env, &ac, k),
                )
            }
        }
    }

    fn cps_op_dispatch(
        &mut self,
        env: &mut Env,
        v: &ValueInfo,
        op_ty: Ty,
        done: Vec<(LocalId, Ty)>,
        kv: &Cont,
        span: ast::Span,
    ) -> Result<ECore> {
        // Walk the operation's arrows: the arguments' domains and the
        // result type, and the performed label on the label arrow.
        let mut ty = op_ty;
        let n = done.len();
        for (i, (_, aty)) in done.iter().enumerate() {
            ty = self.uni.zonk(&ty);
            let (dom, eff, cod) = match ty {
                Ty::Fun(dom, _, eff, cod) => (*dom, eff, *cod),
                Ty::Meta(_) => {
                    let dom = self.uni.fresh();
                    let eff = self.uni.fresh_row();
                    let cod = self.uni.fresh();
                    self.uni
                        .unify(
                            &ty,
                            &Ty::fun_mult(dom.clone(), Mult::Flex, eff.clone(), cod.clone()),
                        )
                        .map_err(|m| Error::at(span, m))?;
                    (dom, eff, cod)
                }
                other => {
                    return Err(Error::at(
                        span,
                        format!("effect operation {} is applied to too many arguments ({other})", v.name),
                    ))
                }
            };
            self.uni
                .unify(&dom, aty)
                .map_err(|m| Error::at(span, m))?;
            ty = cod;
            if !eff.is_pure() {
                // The label arrow: the operation is fully applied here.
                if i + 1 != n {
                    return Err(Error::at(
                        span,
                        format!("effect operation {} is applied to too many arguments", v.name),
                    ));
                }
                let resolved = self.uni.resolve_row(&eff);
                let label = match resolved.labels.as_slice() {
                    [label] if resolved.tail == RowTail::Closed => label.clone(),
                    _ => {
                        return Err(Error::at(
                            span,
                            format!("effect operation {} must perform exactly its effect's label", v.name),
                        ))
                    }
                };
                return self.dispatch_op(env, v, label, &done, ty, kv, span);
            }
        }
        Err(Error::at(
            span,
            format!("effect operation {} is not applied to enough arguments", v.name),
        ))
    }

    /// An effect operation call outside any handler's dispatch reach: the
    /// performed label surfaces honestly and the call is the defined
    /// unhandled fault path (exit 70) as defense in depth (decision D23).
    fn infer_unhandled_op(
        &mut self,
        env: &mut Env,
        v: &ValueInfo,
        args: &[ast::Term],
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let (op_ty, ctx, _) = self.instantiate(&v.scheme);
        if !ctx.is_empty() {
            return Err(Error::at(
                span,
                format!(
                    "effect operation {} takes no class constraints in the bootstrap slice",
                    v.name
                ),
            ));
        }
        let mut ty = op_ty;
        let mut binds: Vec<(LocalId, ECore)> = Vec::new();
        for (i, a) in args.iter().enumerate() {
            ty = self.uni.zonk(&ty);
            let (dom, eff, cod) = match ty {
                Ty::Fun(dom, _, eff, cod) => (*dom, eff, *cod),
                Ty::Meta(_) => {
                    let dom = self.uni.fresh();
                    let eff = self.uni.fresh_row();
                    let cod = self.uni.fresh();
                    self.uni
                        .unify(
                            &ty,
                            &Ty::fun_mult(dom.clone(), Mult::Flex, eff.clone(), cod.clone()),
                        )
                        .map_err(|m| Error::at(span, m))?;
                    (dom, eff, cod)
                }
                other => {
                    return Err(Error::at(
                        span,
                        format!(
                            "effect operation {} is applied to too many arguments ({other})",
                            v.name
                        ),
                    ))
                }
            };
            let ac = self.check(env, a, &dom)?;
            let l = self.fresh_local();
            binds.push((l, ac));
            ty = cod;
            if !eff.is_pure() {
                if i + 1 != args.len() {
                    return Err(Error::at(
                        span,
                        format!("effect operation {} is applied to too many arguments", v.name),
                    ));
                }
                let resolved = self.uni.resolve_row(&eff);
                match resolved.labels.as_slice() {
                    [label] if resolved.tail == RowTail::Closed => {
                        self.perform(&EffRow {
                            labels: vec![label.clone()],
                            tail: RowTail::Closed,
                        })?;
                        return Ok((wrap_lets(binds, ECore::Fault), ty));
                    }
                    _ => {
                        return Err(Error::at(
                            span,
                            format!(
                                "effect operation {} must perform exactly its effect's label",
                                v.name
                            ),
                        ))
                    }
                }
            }
        }
        Err(Error::at(
            span,
            format!("effect operation {} is not applied to enough arguments", v.name),
        ))
    }

    /// Build the dispatch (or unhandled fault) code for one operation
    /// call, with every argument already evaluated.
    fn dispatch_op(
        &mut self,
        env: &mut Env,
        v: &ValueInfo,
        label: EffLabel,
        done: &[(LocalId, Ty)],
        result: Ty,
        kv: &Cont,
        span: ast::Span,
    ) -> Result<ECore> {
        let _ = env;
        let slot = match &v.kind {
            ValueKind::EffectOp { slot, .. } => *slot,
            _ => unreachable!("dispatch_op is called for effect operations"),
        };
        self.uni
            .unify(&result, &kv.ty)
            .map_err(|m| Error::at(span, m))?;
        // The innermost handler extent for this label, at this lexical
        // level (lambda and memo bodies do not dispatch).
        let candidates: Vec<(usize, Option<String>)> = self
            .extents
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                e.lam == self.lam_depth
                    && e.memo == self.memo_depth
                    && e.label.head == label.head
            })
            .map(|(i, e)| (i, e.label.arg.clone()))
            .collect();
        let mut found = None;
        for (i, arg) in candidates.into_iter().rev() {
            match (&arg, &label.arg) {
                (Some(x), Some(y)) => {
                    let mut trial = self.uni.clone();
                    if trial.unify_regions(x, y).is_ok() {
                        self.uni = trial;
                        found = Some(i);
                        break;
                    }
                }
                (None, None) => {
                    found = Some(i);
                    break;
                }
                _ => {}
            }
        }
        let Some(idx) = found else {
            // Unhandled operation: the label surfaces honestly and the
            // call takes the defined fault path (exit 70) as defense in
            // depth (decision D23).
            self.perform(&EffRow { labels: vec![label], tail: RowTail::Closed })?;
            return Ok(ECore::Fault);
        };
        // The resumption: the continuation of this call, one-shot.
        if self.conts[kv.frame].consumes_linear {
            self.promote_resumptions(span)?;
        }
        let rv = self.fresh_local();
        let resumption = ECore::Lam {
            params: vec![rv],
            body: Box::new(app_spine(ECore::Var(kv.kv), vec![ECore::Var(rv)])),
        };
        let vtable = self.extents[idx].vtable;
        let mut core = ECore::Proj(Box::new(ECore::Var(vtable)), ProjField::slot(slot));
        for (l, _) in done {
            core = app_spine(core, vec![ECore::Var(*l)]);
        }
        Ok(app_spine(core, vec![resumption]))
    }
}

impl Checker {
    /// `case` in continuation-passing style: the scrutinee flows into a
    /// continuation whose body runs the ordinary pattern compiler over
    /// `CpsWrap`ped alternative bodies (each flows to `kv`).
    fn cps_case(
        &mut self,
        env: &mut Env,
        scrut: &ast::Term,
        alts: &[ast::Alt],
        kv: &Cont,
        span: ast::Span,
    ) -> Result<ECore> {
        let kv2 = kv.clone();
        let alts2: Vec<ast::Alt> = alts
            .iter()
            .map(|a| {
                let mut a = a.clone();
                a.body = ast::Term::CpsWrap(Box::new(a.body.clone()), a.span);
                a
            })
            .collect();
        let scrut2 = scrut.clone();
        self.bind_cont(
            env,
            ContKind::Once,
            self.term_may_dispatch(scrut),
            move |this, env, v, vty| {
                let ans = this.uni.fresh();
                this.current_cont.push(kv2.clone());
                let code = this.compile_alts(env, &alts2, ECore::Var(v), vty, &ans, span);
                this.current_cont.pop();
                code
            },
            move |this, env, k| this.cps(env, &scrut2, k),
        )
    }

    /// `letrec` in continuation-passing position: the group elaborates
    /// recursively in ordinary style, then the body continues (a group
    /// body that itself dispatches an operation elaborates without its
    /// resumption, as a local lambda body does).
    fn cps_letrec(
        &mut self,
        env: &mut Env,
        decls: &[ast::LocalDecl],
        body: &ast::Term,
        kv: &Cont,
        span: ast::Span,
    ) -> Result<ECore> {
        let mut env2 = env.clone();
        let (binds, group) = self.elab_letrec(&mut env2, decls)?;
        let bcore = self.cps(&mut env2, body, kv)?;
        if let Some(g) = &group {
            self.discharge_group(g, &kv.ty, span)?;
        }
        Ok(wrap_lets(binds, bcore))
    }

    /// `let` in continuation-passing style. Only simple value bindings
    /// sequence here (decision D24): a local function or memo binding under
    /// a handled computation would need its operations dispatched from a
    /// deferred body, which the bootstrap rejects.
    fn cps_let(
        &mut self,
        env: &mut Env,
        decls: &[ast::LocalDecl],
        body: &ast::Term,
        kv: &Cont,
        span: ast::Span,
    ) -> Result<ECore> {
        match decls.split_first() {
            None => self.cps(env, body, kv),
            Some((d, rest)) => match d {
                ast::LocalDecl::Sig { .. } => self.cps_let(env, rest, body, kv, span),
                ast::LocalDecl::Bind { pat, body: dbody, span: dspan } => {
                    let (name, _) = match pat {
                        ast::Pat::Var(n, _) => (Some(n.text.clone()), ()),
                        ast::Pat::Wild(_) => (None, ()),
                        other => {
                            return Err(Error::at(
                                other.span(),
                                "destructuring `let` bindings under a handled computation are not supported in the bootstrap slice",
                            ))
                        }
                    };
                    let dspan = *dspan;
                    let risky = self.term_may_dispatch(dbody);
                    // The `Result` must-use default (decision D114's
                    // rule 7): `let _ = e` discards the value.
                    let dbody_ignored = is_ignore_discard(dbody);
                    let restv = rest.to_vec();
                    let body2 = body.clone();
                    let kv2 = kv.clone();
                    let db = dbody.clone();
                    self.bind_cont(
                        env,
                        ContKind::Once,
                        risky,
                        move |this, env, v, vty| {
                            let mut env2 = env.clone();
                            let l = match &name {
                                Some(n) => {
                                    this.bind_local(
                                        &mut env2,
                                        n,
                                        v,
                                        Scheme::mono(vty),
                                        None,
                                        dspan,
                                    )?;
                                    v
                                }
                                None => {
                                    if !dbody_ignored {
                                        this.check_must_use_discard(&vty, dspan)?;
                                    }
                                    this.register_obligation(v, "_", None, vty, dspan);
                                    v
                                }
                            };
                            let code = this.cps_let(&mut env2, &restv, &body2, &kv2, span)?;
                            Ok(wrap_lets(vec![(l, ECore::Var(v))], code))
                        },
                        move |this, env, k| this.cps(env, &db, k),
                    )
                }
                ast::LocalDecl::Eq { .. } | ast::LocalDecl::Lazy { .. } => Err(Error::at(
                    span,
                    "local functions and memo bindings under a handled computation are not supported in the bootstrap slice",
                )),
            },
        }
    }

    /// A container form with operations inside: bind each immediate
    /// subterm to a fresh local in evaluation order, then rebuild the form
    /// over the locals and elaborate it ordinarily.
    fn cps_rebuild(&mut self, env: &mut Env, term: &ast::Term, kv: &Cont) -> Result<ECore> {
        let subs = immediate_subterms(term);
        self.cps_rebuild_go(env, term, subs, 0, kv, Vec::new())
    }

    fn cps_rebuild_go(
        &mut self,
        env: &mut Env,
        template: &ast::Term,
        subs: Vec<ast::Term>,
        idx: usize,
        kv: &Cont,
        mut bound: Vec<String>,
    ) -> Result<ECore> {
        if idx >= subs.len() {
            let rebuilt = replace_subterms(template, &bound);
            let (core, ty) = self.infer(env, &rebuilt)?;
            self.uni
                .unify(&ty, &kv.ty)
                .map_err(|m| Error::at(template.span(), m))?;
            return Ok(app_spine(ECore::Var(kv.kv), vec![core]));
        }
        let sub = subs[idx].clone();
        let name = format!("s#{}", self.param_counter);
        self.param_counter += 1;
        let name2 = name.clone();
        let kv2 = kv.clone();
        let template_span = template.span();
        self.bind_cont(
            env,
            ContKind::Once,
            self.term_may_dispatch(&sub),
            move |this, env, v, vty| {
                this.bind_local(env, &name2, v, Scheme::mono(vty), None, template_span)?;
                bound.push(name2);
                this.cps_rebuild_go(env, template, subs, idx + 1, &kv2, bound)
            },
            move |this, env, k| this.cps(env, &sub, k),
        )
    }

    /// The result shape `M b` of a `do` block, from the provider's
    /// `pure` (`pure :: a -> M a`).
    fn result_shape(&mut self, env: &mut Env, span: ast::Span) -> Result<Ty> {
        let (_, ty) = self.seq_method(env, ast::SeqKind::Pure, span)?;
        match self.uni.zonk(&ty) {
            Ty::Fun(_, _, _, res) => Ok(*res),
            other => Err(Error::at(
                span,
                format!("internal error: the provider's pure has type {other}"),
            )),
        }
    }

    /// `do { stmts }` through the selected provider's `bind`/`pure`
    /// methods (design section 5). With `kv` the block's result flows into
    /// the continuation (continuation-passing mode); without it the block
    /// is a value.
    fn elab_do(
        &mut self,
        env: &mut Env,
        stmts: &[ast::DoStmt],
        shape: &Ty,
        kv: Option<&Cont>,
        span: ast::Span,
    ) -> Result<ECore> {
        let (last, init) = stmts.split_last().ok_or_else(|| {
            Error::at(span, "internal error: empty do block")
        })?;
        let last_term = match last {
            ast::DoStmt::Expr { term, .. } => term,
            ast::DoStmt::Bind { span: s, .. } | ast::DoStmt::Let { span: s, .. } => {
                return Err(Error::at(
                    *s,
                    "the last statement of a do block must be an expression",
                ))
            }
        };
        if init.is_empty() {
            return match kv {
                Some(kv) => self.cps(env, last_term, kv),
                None => self.check(env, last_term, shape),
            };
        }
        let rest = &stmts[1..];
        let (pat, stmt, sspan) = match &init[0] {
            ast::DoStmt::Bind { pat, term, span } => (Some(pat), term, *span),
            ast::DoStmt::Expr { term, span } => (None, term, *span),
            ast::DoStmt::Let { decls, span: lspan } => {
                // `let` plus its local block binds one pure group
                // strictly, in either interpretation (the sequencing
                // spec's `do` rules; the surface EBNF `do_statement =
                // "let", local_block`).
                let mut env2 = env.clone();
                let (binds, group) = self.elab_locals(
                    &mut env2,
                    decls,
                    LocalForm::DoStmt {
                        handled: kv.is_some(),
                    },
                )?;
                let rest_code = self.elab_do(&mut env2, rest, shape, kv, span)?;
                if let Some(g) = &group {
                    self.discharge_group(g, shape, *lspan)?;
                }
                return Ok(wrap_lets(binds, rest_code));
            }
        };
        // Refutable patterns need a typed failure operation; the direct,
        // Result and Option bootstrap providers do not supply one
        // (decision D25).
        let bind_name = match pat {
            None | Some(ast::Pat::Wild(_)) => None,
            Some(ast::Pat::Var(n, _)) => Some(n.text.clone()),
            Some(other) => {
                return Err(Error::at(
                    other.span(),
                    "refutable patterns in `do` bindings need a provider failure operation; the direct, Result and Option bootstrap providers do not supply one",
                ))
            }
        };
        let (bind_core, bind_ty) = self.seq_method(env, ast::SeqKind::Bind, sspan)?;
        let bind_ty = self.uni.zonk(&bind_ty);
        // `bind :: M a -> (a ->{ef} M b) ->{ef} M b`: the statement's type,
        // the continuation, and the block's result shape.
        let (stmt_ty, cont_ty) = match &bind_ty {
            Ty::Fun(a, _, _, rest) => ((**a).clone(), (**rest).clone()),
            other => {
                return Err(Error::at(
                    sspan,
                    format!("internal error: the provider's bind has type {other}"),
                ))
            }
        };
        // The payload `a` is the continuation's parameter.
        let payload = match &cont_ty {
            Ty::Fun(k, _, _, _) => match &**k {
                Ty::Fun(p, _, _, _) => (**p).clone(),
                other => {
                    return Err(Error::at(
                        sspan,
                        format!("internal error: the provider's bind continues with {other}"),
                    ))
                }
            },
            other => {
                return Err(Error::at(
                    sspan,
                    format!("internal error: the provider's bind takes {other}"),
                ))
            }
        };
        let kind = if select_provider(&self.seq_binders, sspan)?.class == "DirectSequencing" {
            ContKind::Once
        } else {
            // A short-circuiting provider may skip the continuation.
            ContKind::AtMost
        };
        let risky = self.term_may_dispatch(stmt);
        let sp = span;
        // The `Result` must-use default (decision D114's rule 7): a
        // wildcard statement discards the payload, and a `Result` may be
        // discarded only through `ignore`.
        let stmt_ignored = is_ignore_discard(stmt);
        match kv {
            None => {
                let stmt_core = self.check(env, stmt, &stmt_ty)?;
                let l = self.fresh_local();
                let mut env2 = env.clone();
                match &bind_name {
                    Some(n) => {
                        self.bind_local(&mut env2, n, l, Scheme::mono(payload), None, sspan)?
                    }
                    None => {
                        if !stmt_ignored {
                            self.check_must_use_discard(&payload, sspan)?;
                        }
                        self.register_obligation(l, "_", None, payload, sspan)
                    }
                }
                self.conts.push(ContFrame { kind, risky, consumes_linear: false });
                self.cont_depth += 1;
                let rest_code = self.elab_do(&mut env2, rest, shape, None, sp);
                self.cont_depth -= 1;
                let rest_code = rest_code?;
                let lam = ECore::Lam { params: vec![l], body: Box::new(rest_code) };
                Ok(app_spine(bind_core, vec![stmt_core, lam]))
            }
            Some(kv) => {
                let rest2 = rest.to_vec();
                let shape2 = shape.clone();
                let kv2 = kv.clone();
                let stmt2 = stmt.clone();
                let bname = bind_name.clone();
                self.bind_cont(
                    env,
                    kind,
                    risky,
                    move |this, env, v, vty| {
                        this.uni
                            .unify(&vty, &stmt_ty)
                            .map_err(|m| Error::at(sspan, m))?;
                        let l = this.fresh_local();
                        let mut env2 = env.clone();
                        match &bname {
                            Some(n) => {
                                this.bind_local(
                                    &mut env2,
                                    n,
                                    l,
                                    Scheme::mono(payload),
                                    None,
                                    sspan,
                                )?
                            }
                            None => {
                                if !stmt_ignored {
                                    this.check_must_use_discard(&payload, sspan)?;
                                }
                                this.register_obligation(l, "_", None, payload, sspan)
                            }
                        }
                        let rest_code =
                            this.elab_do(&mut env2, &rest2, &shape2, Some(&kv2), sp)?;
                        let lam = ECore::Lam { params: vec![l], body: Box::new(rest_code) };
                        Ok(app_spine(bind_core, vec![ECore::Var(v), lam]))
                    },
                    move |this, env, k| this.cps(env, &stmt2, k),
                )
            }
        }
    }
}

impl Checker {
    /// `handler L [r] where { return p -> e; op pats [resume k] -> e }`:
    /// a handler record of clause closures (design section 3). Returns the
    /// vtable, its type, the `resume k` binders per operation slot, and
    /// whether those binders may still be promoted to `%1`.
    fn handler_value_full(
        &mut self,
        env: &mut Env,
        h: &ast::HandlerExpr,
        out_cont: Option<&Cont>,
    ) -> Result<(ECore, Ty, Vec<Option<LocalId>>, bool)> {
        let span = h.span;
        if h.effect.segments.len() > 1 {
            return Err(Error::at(
                h.effect.span(),
                "effect label paths deeper than one segment are not supported",
            ));
        }
        let head = h.effect.segments[0].text.clone();
        let (region_param, ops) = {
            let e = self.effects.get(&head).ok_or_else(|| {
                Error::at(h.effect.span(), format!("unknown effect {head}"))
            })?;
            (e.region_param.clone(), e.ops.clone())
        };
        match (&region_param, &h.region) {
            (Some(_), None) => {
                return Err(Error::at(
                    span,
                    format!("effect {head} has a region parameter: write `handler {head} r where`"),
                ))
            }
            (None, Some(n)) => {
                return Err(Error::at(
                    n.span,
                    format!("effect {head} has no region parameter"),
                ))
            }
            _ => {}
        }
        let label = EffLabel {
            head: head.clone(),
            arg: h.region.as_ref().map(|n| n.text.clone()),
        };
        let input = self.uni.fresh();
        let output = self.uni.fresh();
        if let Some(kv) = out_cont {
            self.uni
                .unify(&output, &kv.ty)
                .map_err(|m| Error::at(span, m))?;
        }
        let mut clause_eff = EffRow::pure();
        let mut closures: Vec<ECore> = Vec::new();
        // The return clause: `return p -> e`.
        let x = self.fresh_local();
        let mut env2 = env.clone();
        self.lam_depth += 1;
        match &h.ret_pat {
            ast::Pat::Var(n, _) => self.bind_local(
                &mut env2,
                &n.text,
                x,
                Scheme::mono(input.clone()),
                None,
                n.span,
            )?,
            ast::Pat::Wild(_) => {
                self.register_obligation(x, "_", None, input.clone(), span)
            }
            other => {
                return Err(Error::at(
                    other.span(),
                    "handler clauses take variable and wildcard patterns in the bootstrap slice",
                ))
            }
        }
        let saved = self.enter_perf();
        let body = self.check(&mut env2, &h.ret_body, &output);
        self.lam_depth -= 1;
        let body = body?;
        let performed = self.exit_perf(saved);
        clause_eff = self
                .uni
                .union_rows(&clause_eff, &performed)
                .map_err(|m| Error::plain(m))?;
        closures.push(ECore::Lam { params: vec![x], body: Box::new(body) });
        // One clause per operation, in declaration order (slot order).
        let mut k_binders: Vec<Option<LocalId>> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for (op_name, op_scheme) in &ops {
            let slot_clause = h
                .clauses
                .iter()
                .find(|c| &c.op.text == op_name)
                .ok_or_else(|| {
                    Error::at(
                        span,
                        format!("handler for {head} is missing a clause for operation {op_name}"),
                    )
                })?;
            seen.push(op_name.clone());
            let (op_ty, op_ctx, _) = self.instantiate(op_scheme);
            if !op_ctx.is_empty() {
                return Err(Error::at(
                    slot_clause.span,
                    format!("effect operation {op_name} takes no class constraints"),
                ));
            }
            let mut ty = op_ty;
            let mut params: Vec<LocalId> = Vec::new();
            let mut env3 = env.clone();
            let mut saw_label = false;
            self.lam_depth += 1;
            for p in slot_clause.pats.iter() {
                let (dom, cod) = match self.uni.zonk(&ty) {
                    Ty::Fun(a, _, e, b) => {
                        if !e.is_pure() {
                            saw_label = true;
                        }
                        (*a, *b)
                    }
                    other => {
                        return Err(Error::at(
                            p.span(),
                            format!("clause for {op_name} has too many patterns ({other})"),
                        ))
                    }
                };
                let l = self.fresh_local();
                match p {
                    ast::Pat::Var(pn, _) => {
                        self.bind_local(&mut env3, &pn.text, l, Scheme::mono(dom), None, pn.span)?
                    }
                    ast::Pat::Wild(_) => {
                        self.register_obligation(l, "_", None, dom, p.span())
                    }
                    other => {
                        return Err(Error::at(
                            other.span(),
                            "handler clauses take variable and wildcard patterns in the bootstrap slice",
                        ))
                    }
                }
                params.push(l);
                ty = cod;
            }
            if !saw_label {
                return Err(Error::at(
                    slot_clause.span,
                    format!("clause for {op_name} has too few patterns"),
                ));
            }
            // Everything past the label arrow is the operation's result.
            let result = ty;
            let k = match &slot_clause.resume {
                Some(kn) => {
                    let kl = self.fresh_local();
                    self.bind_local(
                        &mut env3,
                        &kn.text,
                        kl,
                        Scheme::mono(Ty::fun(result, output.clone())),
                        Some(Mult::Aff),
                        kn.span,
                    )?;
                    params.push(kl);
                    Some(kl)
                }
                None => {
                    // The resumption slot exists even when the clause
                    // cannot continue the computation.
                    let kl = self.fresh_local();
                    self.register_obligation(
                        kl,
                        "_",
                        Some(Mult::Aff),
                        Ty::fun(result, output.clone()),
                        slot_clause.span,
                    );
                    params.push(kl);
                    None
                }
            };
            k_binders.push(k);
            let saved = self.enter_perf();
            let body = self.check(&mut env3, &slot_clause.body, &output);
            self.lam_depth -= 1;
            let body = body?;
            let performed = self.exit_perf(saved);
            clause_eff = self
                .uni
                .union_rows(&clause_eff, &performed)
                .map_err(|m| Error::plain(m))?;
            closures.push(ECore::Lam { params, body: Box::new(body) });
        }
        for c in &h.clauses {
            if !seen.contains(&c.op.text) {
                return Err(Error::at(
                    c.op.span,
                    format!("{c_op} is not an operation of {head}", c_op = c.op.text),
                ));
            }
        }
        let n = closures.len();
        let gid = self.tuple_ctor(n, span)?;
        let core = app_spine(ECore::Ctor(gid), closures);
        let ty = Ty::Handler {
            label,
            input: Box::new(input),
            output: Box::new(output),
            eff: clause_eff,
            resumption: Mult::Flex,
        };
        Ok((core, ty, k_binders, true))
    }

    fn handler_value(
        &mut self,
        env: &mut Env,
        h: &ast::HandlerExpr,
    ) -> Result<(ECore, Ty)> {
        let (core, ty, _, _) = self.handler_value_full(env, h, None)?;
        Ok((core, ty))
    }

    /// `handle e with h`: the handled computation is elaborated in
    /// continuation-passing style inside a handler extent; operations of
    /// the handled label performed in its own expression dispatch to `h`,
    /// and the handled label must not arrive through a call (decision
    /// D23).
    fn handle_expr(
        &mut self,
        env: &mut Env,
        handled: &ast::Term,
        h_term: &ast::Term,
        kv: Option<&Cont>,
    ) -> Result<(ECore, Ty)> {
        let span = h_term.span();
        let (hcore, hty, k_binders, flexible) = if let ast::Term::Handler(h) = h_term {
            self.handler_value_full(env, h, kv)?
        } else {
            let (core, ty) = self.infer(env, h_term)?;
            (core, ty, Vec::new(), false)
        };
        let (label, input, output, clause_eff) = match self.uni.zonk(&hty) {
            Ty::Handler { label, input, output, eff, .. } => {
                (label, *input, *output, eff)
            }
            other => {
                return Err(Error::at(
                    span,
                    format!("handle expects a handler value, got {other}"),
                ))
            }
        };
        if let Some(kv) = kv {
            self.uni
                .unify(&output, &kv.ty)
                .map_err(|m| Error::at(span, m))?;
        }
        // The prompt: the return clause, then the surrounding
        // continuation.
        let hv = self.fresh_local();
        let pv = self.fresh_local();
        let ret_call = app_spine(
            ECore::Proj(Box::new(ECore::Var(hv)), ProjField::slot(0)),
            vec![ECore::Var(pv)],
        );
        let prompt_body = match kv {
            Some(kv) => app_spine(ECore::Var(kv.kv), vec![ret_call]),
            None => ret_call,
        };
        let prompt_frame = self.conts.len();
        self.conts.push(ContFrame {
            kind: ContKind::Once,
            risky: false,
            consumes_linear: false,
        });
        self.cont_depth += 1;
        let kv0_local = self.fresh_local();
        let kv0 = Cont { kv: kv0_local, ty: input.clone(), frame: prompt_frame };
        self.extents.push(HandlerExtent {
            label: label.clone(),
            vtable: hv,
            k_binders,
            flexible,
            lam: self.lam_depth,
            memo: self.memo_depth,
        });
        let saved = self.enter_perf();
        let ecode = self.cps(env, handled, &kv0);
        self.extents.pop();
        self.cont_depth -= 1;
        let ecode = ecode?;
        let pe = self.exit_perf(saved);
        // The handled label must not arrive through a call: dispatch is
        // lexical in the bootstrap (decision D23).
        let resolved = self.uni.resolve_row(&pe);
        for l in &resolved.labels {
            if l.head != label.head {
                continue;
            }
            let same = match (&l.arg, &label.arg) {
                (Some(x), Some(y)) => {
                    let mut trial = self.uni.clone();
                    let ok = trial.unify_regions(x, y).is_ok();
                    if ok {
                        self.uni = trial;
                    }
                    ok
                }
                (None, None) => true,
                _ => false,
            };
            if same {
                return Err(Error::at(
                    span,
                    format!(
                        "the handled effect {{{label}}} is performed through a called computation; the bootstrap handler calculus dispatches operations only from the handled computation's own expression"
                    ),
                ));
            }
        }
        self.perform(&pe)?;
        self.perform(&clause_eff)?;
        let prompt = ECore::Lam { params: vec![pv], body: Box::new(prompt_body) };
        Ok((wrap_lets(vec![(hv, hcore), (kv0_local, prompt)], ecode), output))
    }

    /// The selected provider's `pure`/`bind` method value and type (the
    /// provider dictionary resolves from the selection's class).
    fn seq_method(
        &mut self,
        env: &mut Env,
        kind: ast::SeqKind,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let class = select_provider(&self.seq_binders, span)?;
        let path = ast::ConPath {
            segments: vec![ast::Name::new(class.class.clone(), span)],
        };
        let sig = self
            .lookup_sig(&path)
            .map_err(|e| Error::at(span, e.message))?;
        let mname = match kind {
            ast::SeqKind::Pure => "pure",
            ast::SeqKind::Bind => "bind",
        };
        let (idx, mscheme) = sig
            .methods
            .iter()
            .enumerate()
            .find(|(_, (n, _))| n == mname)
            .map(|(i, (_, s))| (i, s.clone()))
            .ok_or_else(|| {
                Error::at(
                    span,
                    format!("sequencing provider {} has no {mname} method", class.class),
                )
            })?;
        let (ty, ctx, _) = self.instantiate(&mscheme);
        let mut holes = Vec::new();
        for (_, c) in &ctx {
            let id = self.new_want(c.clone(), span, env);
            holes.push(ECore::Hole(id));
        }
        let dict = holes.remove(0);
        let core = if holes.is_empty() {
            ECore::DictProj(Box::new(dict), idx)
        } else {
            app_spine(ECore::DictProj(Box::new(dict), idx), holes)
        };
        Ok((core, ty))
    }

    /// Is `name` the selected provider's `pure` form (decision D22)? A
    /// local or user definition of the same name shadows it, and a class
    /// constraint carrying `pure` answers first (the bare-`pure` rule:
    /// bare uses keep the D22 provider meaning only when no constrained
    /// method claims the name).
    fn is_provider_pure(&self, env: &Env, name: &str) -> bool {
        if name != "pure" || env.lookup(name).is_some() {
            return false;
        }
        if self.constraint_pure(env).is_some() {
            return false;
        }
        match self.lookup_value(None, name) {
            Ok(v) => match &v.kind {
                ValueKind::Method { .. } => v
                    .scheme
                    .ctx
                    .first()
                    .map(|(_, c)| SEQ_CLASSES.contains(&c.class.as_str()))
                    .unwrap_or(false),
                _ => false,
            },
            Err(_) => true,
        }
    }

    /// The class method a bare `pure` reaches through its constraint (the
    /// bare-`pure` rule): the nearest in-scope `using` binder whose class
    /// is not a sequencing provider and whose method set carries `pure`.
    /// The provider classes are excluded so `do` keeps its lexically
    /// selected provider (decision D22).
    fn constraint_pure(&self, env: &Env) -> Option<ValueInfo> {
        for (_l, c) in env.dicts.iter().rev() {
            if SEQ_CLASSES.contains(&c.class.as_str()) {
                continue;
            }
            let Some(sig) = self
                .sigs
                .get(&c.class)
                .and_then(|ss| ss.iter().find(|s| self.def_in_scope(&s.module, &s.name)))
            else {
                continue;
            };
            if sig.methods.iter().any(|(n, _)| n == "pure") {
                return self
                    .qualified
                    .get(&(sig.module.clone(), "pure".to_string()))
                    .cloned();
            }
        }
        None
    }

    /// Bind a capability name to a value of type `Io r` (decision D21):
    /// the binder is both a term-level value and a solver-visible provider.
    fn bind_capability(
        &mut self,
        env: &mut Env,
        name: &ast::Name,
        core: ECore,
        ty: Ty,
        span: ast::Span,
    ) -> Result<(LocalId, ECore, String)> {
        let z = self.uni.zonk(&ty);
        let region = match &z {
            Ty::Adt(n, args) if n == "Io" && args.len() == 1 => match &args[0] {
                Ty::Region(r) => r.clone(),
                other => {
                    return Err(Error::at(
                        span,
                        format!(
                            "a capability's argument is a runtime identity, got {other}"
                        ),
                    ))
                }
            },
            Ty::Meta(_) => {
                let r = self.fresh_region();
                self.uni.note_flex_region(&r);
                self.uni
                    .unify(&ty, &Ty::Adt("Io".into(), vec![Ty::Region(r.clone())]))
                    .map_err(|m| Error::at(span, m))?;
                r
            }
            other => {
                return Err(Error::at(
                    span,
                    format!(
                        "capability supply {} must have type `Io r`, got {other}",
                        name.text
                    ),
                ))
            }
        };
        let l = self.fresh_local();
        self.bind_local(
            env,
            &name.text,
            l,
            Scheme::mono(Ty::Adt("Io".into(), vec![Ty::Region(region.clone())])),
            Some(Mult::Many),
            span,
        )?;
        env.dicts
            .push((l, ClassApp { class: "Io".into(), args: vec![Ty::Region(region.clone())] }));
        Ok((l, core, region))
    }

    /// A capability for a still-flex runtime identity must not leak that
    /// identity through the supply's scope (design section 4).
    fn check_cap_escape(&mut self, region: &str, ty: &Ty, span: ast::Span) -> Result<()> {
        let r = self.uni.resolve_region(region);
        if !self.uni.is_flex_region(&r) {
            return Ok(());
        }
        let t = self.uni.zonk(ty);
        let mut regions = Vec::new();
        escape_regions(&t, &mut regions);
        if regions.iter().any(|x| self.uni.resolve_region(x) == r) {
            return Err(Error::at(
                span,
                format!(
                    "runtime identity {r} escapes its capability scope; package the runtime to let it out"
                ),
            ));
        }
        Ok(())
    }

    /// `f @{io = e}`: explicit capability supplies at a call (decision
    /// D21). Named capability parameters take the supplies; the rest are
    /// solved as usual.
    fn cap_apply(
        &mut self,
        env: &mut Env,
        head: &ast::Term,
        supplies: &[(ast::Name, ast::Term)],
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let v = match head {
            ast::Term::Var(p) => self.resolve_var(p, p.span())?,
            other => {
                return Err(Error::at(
                    other.span(),
                    "explicit capability supplies apply to a named function",
                ))
            }
        };
        if matches!(v.kind, ValueKind::EffectOp { .. }) {
            return Err(Error::at(
                span,
                format!("effect operation {} must be applied", v.name),
            ));
        }
        if matches!(v.kind, ValueKind::BoolLit(_)) {
            return Err(Error::at(span, "a boolean literal takes no capability supplies"));
        }
        let (ty, ctx, _) = self.instantiate(&v.scheme);
        let mut holes = Vec::new();
        for (n, c) in &ctx {
            if is_capability_class(&c.class) {
                if let Some((_, term)) = supplies.iter().find(|(sn, _)| &sn.text == n) {
                    let want = Ty::Adt(c.class.clone(), c.args.clone());
                    holes.push(self.check(env, term, &want)?);
                    continue;
                }
            }
            let id = self.new_want(c.clone(), span, env);
            holes.push(ECore::Hole(id));
        }
        for (sn, _) in supplies {
            if !ctx
                .iter()
                .any(|(n, c)| &sn.text == n && is_capability_class(&c.class))
            {
                return Err(Error::at(
                    sn.span,
                    format!("{} has no capability parameter {}", v.name, sn.text),
                ));
            }
        }
        let core = match &v.kind {
            ValueKind::TopFn | ValueKind::PrimDecl { .. } => self.fn_ref(&v, span)?,
            ValueKind::StageFn | ValueKind::MacroFn => {
                ECore::Global(fn_id(&v.module, &v.name))
            }
            ValueKind::Prim(op) => ECore::Prim(*op),
            ValueKind::Foreign { unit, .. } => ECore::Foreign(unit.clone()),
            ValueKind::Ctor { .. } => ECore::Ctor(fn_id(&v.module, &v.name)),
            ValueKind::Method { idx, .. } => {
                let dict = holes.remove(0);
                ECore::DictProj(Box::new(dict), *idx)
            }
            ValueKind::StageBuiltin(_)
            | ValueKind::BoolLit(_)
            | ValueKind::EffectOp { .. }
            | ValueKind::Law
            | ValueKind::ProofCtor => {
                return Err(Error::at(span, "this value takes no capability supplies"))
            }
        };
        let core = if holes.is_empty() { core } else { app_spine(core, holes) };
        Ok((core, ty))
    }
}

/// The immediate subterms of a container form, in evaluation order.
fn immediate_subterms(t: &ast::Term) -> Vec<ast::Term> {
    match t {
        ast::Term::Ann(e, _, _) | ast::Term::Proj(e, _, _) => vec![(**e).clone()],
        ast::Term::Tuple(items, _) | ast::Term::List(items, _) => items.clone(),
        ast::Term::Map(kvs, _) => {
            kvs.iter().flat_map(|(k, v)| [k.clone(), v.clone()]).collect()
        }
        ast::Term::Range(start, step, end, _) => {
            let mut v = vec![(**start).clone()];
            v.extend(step.iter().map(|x| (**x).clone()));
            v.push((**end).clone());
            v
        }
        ast::Term::Record(_, fields, _) => {
            fields.iter().map(|(_, x)| x.clone()).collect()
        }
        ast::Term::CapApply { head, supplies, .. } => {
            let mut v = vec![(**head).clone()];
            v.extend(supplies.iter().map(|(_, x)| x.clone()));
            v
        }
        _ => Vec::new(),
    }
}

/// Rebuild a container form with its immediate subterms replaced by
/// variable references to already-evaluated locals.
fn replace_subterms(t: &ast::Term, names: &[String]) -> ast::Term {
    let span = t.span();
    let mk = |n: &String| {
        ast::Term::Var(ast::VarPath {
            quals: Vec::new(),
            name: ast::Name::new(n.clone(), span),
        })
    };
    match t {
        ast::Term::Ann(_, ty, s) => {
            ast::Term::Ann(Box::new(mk(&names[0])), ty.clone(), *s)
        }
        ast::Term::Proj(_, f, s) => {
            ast::Term::Proj(Box::new(mk(&names[0])), f.clone(), *s)
        }
        ast::Term::Tuple(_, s) => ast::Term::Tuple(names.iter().map(mk).collect(), *s),
        ast::Term::List(_, s) => ast::Term::List(names.iter().map(mk).collect(), *s),
        ast::Term::Map(_, s) => ast::Term::Map(
            names.chunks(2).map(|c| (mk(&c[0]), mk(&c[1]))).collect(),
            *s,
        ),
        ast::Term::Range(_, step, _, s) => {
            let start = Box::new(mk(&names[0]));
            let (step, end) = if step.is_some() {
                (Some(Box::new(mk(&names[1]))), mk(&names[2]))
            } else {
                (None, mk(&names[1]))
            };
            ast::Term::Range(start, step, Box::new(end), *s)
        }
        ast::Term::Record(p, fields, s) => ast::Term::Record(
            p.clone(),
            fields
                .iter()
                .zip(names.iter())
                .map(|((n, _), nn)| (n.clone(), mk(nn)))
                .collect(),
            *s,
        ),
        ast::Term::CapApply { supplies, .. } => ast::Term::CapApply {
            head: Box::new(mk(&names[0])),
            supplies: supplies
                .iter()
                .zip(names.iter().skip(1))
                .map(|((n, _), nn)| (n.clone(), mk(nn)))
                .collect(),
            span,
        },
        _ => t.clone(),
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
    /// The clause's or alternative's weight, beside the alternative it
    /// annotates (attribute annotations specification section 2.1).
    weight: Option<u32>,
    /// The guard's weight: its pass outcome, with the reference base 1
    /// completing the pair for the fallthrough (sections 2.1 and 3).
    guard_weight: Option<u32>,
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
            ast::Pat::Wild(_) | ast::Pat::Unit(_) | ast::Pat::Int(_, _) | ast::Pat::Sym(_, _) => {}
        }
        Ok(())
    }
    let mut seen = Vec::new();
    for p in pats {
        walk(p, &mut seen)?;
    }
    Ok(())
}

/// Is this term an explicit `ignore e` (the purposeful discard of the
/// `Result` must-use default; attribute annotations specification
/// section 10)?
fn is_ignore_discard(t: &ast::Term) -> bool {
    match t {
        ast::Term::Ann(e, _, _) => is_ignore_discard(e),
        ast::Term::App(f, args, _) => {
            matches!(&**f, ast::Term::Var(p)
                if p.quals.is_empty() && p.name.text == "ignore")
                && args.len() == 1
        }
        _ => false,
    }
}

/// A binary decision point's weights: both alternatives carry one or
/// neither does (attribute annotations specification section 2.1).
fn pair_weights(a: Option<u32>, b: Option<u32>) -> (Option<u32>, Option<u32>) {
    if a.is_some() && b.is_some() {
        (a, b)
    } else {
        (None, None)
    }
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
/// `Sym` literals test identity through `symEq` (the interned cell;
/// section 9).
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
        Ty::Sym => app_spine(ECore::Prim(PrimOp::Sym(SymOp::Eq)), vec![col, lit]),
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

/// How a `letrec` group member is reached from its siblings' bodies
/// (surface grammar local amendments section 9).
enum RecSub {
    /// A function member: its code generation unit and captured words.
    Fun(GlobalId, Vec<LocalId>),
    /// A value member: its fixpoint box.
    Val(LocalId),
}

/// The guarded-letrec rule (surface grammar local amendments section 9):
/// a use of a group name that closes a dependency cycle sits under a
/// lambda, a `~` memo body, or a `delay`. An eager cycle is a compile
/// error naming the variable and the missing guard.
fn check_rec_guards(members: &[(LocalId, ast::Span, String)], cores: &[ECore]) -> Result<()> {
    let mut by_local: HashMap<LocalId, usize> = HashMap::new();
    for (i, (l, _, _)) in members.iter().enumerate() {
        by_local.insert(*l, i);
    }
    // (owner, referenced, guarded)
    let mut edges: Vec<(usize, usize, bool)> = Vec::new();
    for (owner, core) in cores.iter().enumerate() {
        rec_walk(core, 0, owner, &by_local, &mut edges);
    }
    let n = members.len();
    let mut reach = vec![vec![false; n]; n];
    for &(o, r, _) in &edges {
        if o < n && r < n {
            reach[o][r] = true;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if reach[i][k] && reach[k][j] {
                    reach[i][j] = true;
                }
            }
        }
    }
    for &(o, r, guarded) in &edges {
        if !guarded && (o == r || reach[r][o]) {
            let name = &members[r].2;
            return Err(Error::at(
                members[o].1,
                format!(
                    "unguarded recursion; wrap the use in `delay` or a lambda (the use of `{name}` closes an eager cycle in the binding group)"
                ),
            ));
        }
    }
    Ok(())
}

/// Collect the group uses in one member's core as `(owner, referenced,
/// guarded)` edges; `guard` counts the suspension positions entered.
fn rec_walk(
    e: &ECore,
    guard: i32,
    owner: usize,
    by_local: &HashMap<LocalId, usize>,
    edges: &mut Vec<(usize, usize, bool)>,
) {
    match e {
        ECore::LitFold { body, .. } => rec_walk(body, guard, owner, by_local, edges),
        ECore::Var(l) => {
            if let Some(&r) = by_local.get(l) {
                edges.push((owner, r, guard >= 1));
            }
        }
        ECore::Int(_)
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Global(_)
        | ECore::Ctor(_)
        | ECore::Prim(_)
        | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::LazyAlloc
        | ECore::BoxAlloc
        | ECore::SyntaxLit(_)
        | ECore::StrLit(_) => {}
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                rec_walk(h, guard, owner, by_local, edges);
            }
            rec_walk(body, guard, owner, by_local, edges);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                rec_walk(h, guard, owner, by_local, edges);
            }
        }
        ECore::DictProj(d, _) | ECore::Proj(d, _) | ECore::Demand(d) | ECore::BoxGet(d) => {
            rec_walk(d, guard, owner, by_local, edges);
        }
        ECore::App(f, args) => {
            rec_walk(f, guard, owner, by_local, edges);
            // A `delay`'s argument is one of the suspension positions.
            let bump = if matches!(**f, ECore::Prim(PrimOp::Delay)) { 1 } else { 0 };
            for a in args {
                rec_walk(a, guard + bump, owner, by_local, edges);
            }
        }
        ECore::InstDict(_, args) | ECore::MkClos { env: args, .. } => {
            for a in args {
                rec_walk(a, guard, owner, by_local, edges);
            }
        }
        // A lambda body is a suspension position; a `~` memo body is one
        // because its thunk is a lambda.
        ECore::Lam { body, .. } => rec_walk(body, guard + 1, owner, by_local, edges),
        ECore::Let(_, v, body) => {
            rec_walk(v, guard, owner, by_local, edges);
            rec_walk(body, guard, owner, by_local, edges);
        }
        ECore::BoxInit(_, v) | ECore::LazyInit(_, v) | ECore::RefSet(_, v) => {
            rec_walk(v, guard, owner, by_local, edges)
        }
        ECore::If(c, t, f) => {
            rec_walk(c, guard, owner, by_local, edges);
            rec_walk(&t.body, guard, owner, by_local, edges);
            rec_walk(&f.body, guard, owner, by_local, edges);
        }
        ECore::Case { scrut, alts } => {
            rec_walk(scrut, guard, owner, by_local, edges);
            for alt in alts {
                rec_walk(&alt.body, guard, owner, by_local, edges);
            }
        }

    }
}

/// The group members referenced from one member's core, by index.
fn rec_collect_refs(e: &ECore, by_local: &HashMap<LocalId, usize>, out: &mut Vec<usize>) {
    match e {
        ECore::LitFold { body, .. } => rec_collect_refs(body, by_local, out),
        ECore::Var(l) => {
            if let Some(&r) = by_local.get(l) {
                out.push(r);
            }
        }
        ECore::Int(_)
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Global(_)
        | ECore::Ctor(_)
        | ECore::Prim(_)
        | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::LazyAlloc
        | ECore::BoxAlloc
        | ECore::SyntaxLit(_)
        | ECore::StrLit(_) => {}
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                rec_collect_refs(h, by_local, out);
            }
            rec_collect_refs(body, by_local, out);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                rec_collect_refs(h, by_local, out);
            }
        }
        ECore::DictProj(d, _) | ECore::Proj(d, _) | ECore::Demand(d) | ECore::BoxGet(d) => {
            rec_collect_refs(d, by_local, out)
        }
        ECore::App(f, args) => {
            rec_collect_refs(f, by_local, out);
            for a in args {
                rec_collect_refs(a, by_local, out);
            }
        }
        ECore::InstDict(_, args) | ECore::MkClos { env: args, .. } => {
            for a in args {
                rec_collect_refs(a, by_local, out);
            }
        }
        ECore::Lam { body, .. } => rec_collect_refs(body, by_local, out),
        ECore::Let(_, v, body) => {
            rec_collect_refs(v, by_local, out);
            rec_collect_refs(body, by_local, out);
        }
        ECore::BoxInit(_, v) | ECore::LazyInit(_, v) | ECore::RefSet(_, v) => {
            rec_collect_refs(v, by_local, out)
        }
        ECore::If(c, t, f) => {
            rec_collect_refs(c, by_local, out);
            rec_collect_refs(&t.body, by_local, out);
            rec_collect_refs(&f.body, by_local, out);
        }
        ECore::Case { scrut, alts } => {
            rec_collect_refs(scrut, by_local, out);
            for alt in alts {
                rec_collect_refs(&alt.body, by_local, out);
            }
        }

    }
}

/// Rewrite one member's core: function uses become direct calls into the
/// member's code generation unit (or closures over it), recursive value
/// uses read the value's fixpoint box at use time.
fn subst_rec_refs(e: &mut ECore, subs: &HashMap<LocalId, RecSub>) {
    if let ECore::App(f, args) = e {
        if let ECore::Var(l) = &**f {
            if let Some(RecSub::Fun(id, cap)) = subs.get(l) {
                let id = id.clone();
                let cap = cap.clone();
                for a in args.iter_mut() {
                    subst_rec_refs(a, subs);
                }
                let mut new_args: Vec<ECore> = cap.iter().map(|c| ECore::Var(*c)).collect();
                new_args.append(args);
                *e = ECore::App(Box::new(ECore::Global(id)), new_args);
                return;
            }
        }
    }
    match e {
        ECore::LitFold { body, .. } => subst_rec_refs(body, subs),
        ECore::Var(l) => match subs.get(l) {
            Some(RecSub::Fun(id, cap)) => {
                let (id, cap) = (id.clone(), cap.clone());
                *e = ECore::MkClos {
                    fn_id: id,
                    env: cap.iter().map(|c| ECore::Var(*c)).collect(),
                };
            }
            Some(RecSub::Val(b)) => {
                let b = *b;
                *e = ECore::BoxGet(Box::new(ECore::Var(b)));
            }
            None => {}
        },
        ECore::Int(_)
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Global(_)
        | ECore::Ctor(_)
        | ECore::Prim(_)
        | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::LazyAlloc
        | ECore::BoxAlloc
        | ECore::SyntaxLit(_)
        | ECore::StrLit(_) => {}
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                subst_rec_refs(h, subs);
            }
            subst_rec_refs(body, subs);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                subst_rec_refs(h, subs);
            }
        }
        ECore::DictProj(d, _) | ECore::Proj(d, _) | ECore::Demand(d) | ECore::BoxGet(d) => {
            subst_rec_refs(d, subs)
        }
        ECore::App(f, args) => {
            subst_rec_refs(f, subs);
            for a in args {
                subst_rec_refs(a, subs);
            }
        }
        ECore::InstDict(_, args) | ECore::MkClos { env: args, .. } => {
            for a in args {
                subst_rec_refs(a, subs);
            }
        }
        ECore::Lam { body, .. } => subst_rec_refs(body, subs),
        ECore::Let(_, v, body) => {
            subst_rec_refs(v, subs);
            subst_rec_refs(body, subs);
        }
        ECore::BoxInit(_, v) | ECore::LazyInit(_, v) | ECore::RefSet(_, v) => {
            subst_rec_refs(v, subs)
        }
        ECore::If(c, t, f) => {
            subst_rec_refs(c, subs);
            subst_rec_refs(&mut t.body, subs);
            subst_rec_refs(&mut f.body, subs);
        }
        ECore::Case { scrut, alts } => {
            subst_rec_refs(scrut, subs);
            for alt in alts {
                subst_rec_refs(&mut alt.body, subs);
            }
        }

    }
}

fn wrap_lets(binds: Vec<(LocalId, ECore)>, body: ECore) -> ECore {
    let mut out = body;
    for (l, v) in binds.into_iter().rev() {
        out = ECore::Let(l, Box::new(v), Box::new(out));
    }
    out
}

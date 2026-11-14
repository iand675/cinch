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

/// The baked measure shape a `decide` operand computes through the
/// compiler's own vocabulary (never source a shadowing module could lie
/// with): a byte buffer's size or a list spine's element count.
enum MeasureShape {
    Bytes,
    List(String),
}

/// Which construct's local block [`Checker::elab_locals`] is elaborating:
/// the `local_block` grammar is shared by `let`/`where` and by the `do`
/// statement's `let` group, with different disciplines.
#[derive(Clone, Copy, PartialEq, Eq)]
enum LocalForm {
    /// `let b1; ...; bn in e` and `where` blocks (surface grammar local
    /// amendments section 9): the group replaces nested single `let`s.
    Plain,
    /// An erased evidence expression. Inferred proposition bindings must
    /// themselves be checked evidence before their names enter the scope.
    Proof,
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
            PTerm::Lit(_) => true,
            PTerm::Var(_) | PTerm::MVar(_) => false,
            PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => {
                matches!(a.normalize(), ITerm::Num(_)) && matches!(b.normalize(), ITerm::Num(_))
            }
            PTerm::Pred(_, a) => matches!(a.normalize(), ITerm::Num(_)),
            PTerm::Not(q) => idx_ground(&Ty::Prp(q)),
            PTerm::And(a, b) | PTerm::Or(a, b) => {
                idx_ground(&Ty::Prp(a)) && idx_ground(&Ty::Prp(b))
            }
        },
        _ => true,
    }
}

/// Rename the index-level variables of a type (a constructor's field
/// binders become the pattern's names).
fn subst_idx_names(t: &Ty, map: &HashMap<String, String>) -> Ty {
    let mut out = t.clone();
    for (from, to) in map {
        out = out.subst_idx(from, &ITerm::Var(to.clone().into()));
    }
    out
}

impl Checker {
    // ------------------------------------------------------------ terms

    fn syntax_index_value(&self, env: &Env, name: &ast::Name) -> Result<ITerm> {
        let (local, _) = env.lookup_name(name).ok_or_else(|| Error::at(name.span,
            format!("captured local `{}` is not in scope", name.text)))?;
        let original = self.value_aliases.get(&local).copied().unwrap_or(local);
        if self.quantified_value_locals.contains(&original) {
            return Ok(ITerm::LocalValue(original));
        }
        for (spelling, id, _) in env.locals.iter().rev() {
            if env.lookup(spelling).is_some_and(|(active, _)| active == *id)
                && self.value_aliases.get(id).copied().unwrap_or(*id) == original
            { return Ok(ITerm::Var(spelling.clone().into())); }
        }
        Ok(ITerm::LocalValue(original))
    }

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
            ast::Term::Var(p) if p.quals.is_empty() => env.lookup_name(&p.name).map(|(l, _)| l),
            ast::Term::Proj(e, _, _) => self.place_local(env, e),
            _ => None,
        }
    }

    /// Is `name` an unshadowed builtin form (`delay`, `force`, `demand`,
    /// `copyOut`, `borrow`, `mutBorrow`, `refSet`, `ignore`)? A user
    /// definition of the same name shadows the builtin (decision D2).
    fn is_builtin_form(&self, env: &Env, name: &ast::Name) -> bool {
        if name.local_binding().is_some() || name.resolved_binding().is_some() || name.declaration_binding().is_some() { return false; }
        if let Some(scope) = name.scoped_reference() {
            return scope.is_special_form() && scope.spelling() != "pure";
        }
        let spelling = name.text.as_str();
        ast::builtin_syntax_form(spelling)
            && env.lookup_name(name).is_none()
            && self.lookup_value(None, spelling).is_err()
    }

    /// The structured-concurrency builtin behind an application head
    /// (`withScope`, `spawn`, `joinAllOrCancel`), when it is unshadowed:
    /// their application elaboration is [`Checker::infer_conc_app`]. The
    /// `Io r` operations are ordinary builtin values.
    fn conc_builtin(&self, env: &Env, f: &ast::Term) -> Option<ValueInfo> {
        let ast::Term::Var(p) = f else { return None };
        if !p.quals.is_empty() || env.lookup_name(&p.name).is_some() {
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
                    args: vec![Ty::Region(r.clone().into())],
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
                        parallel: false,
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
                        crate::diagnostic!(
                            "region escape: a task handle or scope of region {s} leaves its `withScope` in {} (concurrency specification, section 2)",
                            res
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
                // A literal task body runs exactly once on its thread.
                // A previously built closure retains its original captures.
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
                        parallel: false,
                        scope: s.clone().to_string(),
                        outer: outer.clone(),
                    });
                }
                let bcore = self.check(env, &args[1], &body_ty);
                if lam {
                    self.oneshot_frames.pop();
                }
                let bcore = bcore?;
                if !lam {
                    self.note_spawned_closure(&bcore, &s, &outer, args[1].span(), &env.dicts,
                                              self.checking_task_forwarder)?;
                }
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

    /// The deterministic pure-parallelism builtin behind an application
    /// head (`parBoth`, `parTabulate`, `parMap`), when it is unshadowed:
    /// their application elaboration is [`Checker::infer_par_app`]. A
    /// definition of the same name shadows the builtin (decision D2).
    fn par_builtin(&self, env: &Env, f: &ast::Term) -> Option<ValueInfo> {
        let ast::Term::Var(p) = f else { return None };
        if !p.quals.is_empty() || env.lookup_name(&p.name).is_some() {
            return None;
        }
        let v = self.resolve_var(p, p.span()).ok()?;
        match v.kind {
            ValueKind::Prim(PrimOp::Par { .. }) => Some(v),
            _ => None,
        }
    }

    /// `parBoth f g`, `parTabulate n f`, `parMap f xs`
    /// (`design/parallel-evaluation.md` sections 2.1 to 2.3). The meaning of
    /// each is its sequential defining equation, so parallelism is only
    /// permission: no result depends on the participant count or the
    /// schedule. Four obligations are checked here, because none can travel
    /// through an ordinary function type:
    ///
    /// - the callback is written literally at the call site, so its captures
    ///   are visible (a function-typed parameter's are not);
    /// - its effect row is parallel-safe (the closed empty row or
    ///   `{Diverge}`), because every other label would make the schedule
    ///   observable or let a branch hold a single-threaded resource;
    /// - each capture is `Share`, or (in `parBoth` only) moves into the one
    ///   branch that consumes it;
    /// - `parMap` has array storage, so its element type must be `Share`.
    ///
    /// The capture classification is deferred to the definition's end, when
    /// unification has resolved the binder types (`check_parallel_captures`).
    fn infer_par_app(
        &mut self,
        env: &mut Env,
        v: &ValueInfo,
        args: &[ast::Term],
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let op = match v.kind {
            ValueKind::Prim(PrimOp::Par { op, .. }) => op,
            _ => unreachable!("par_builtin only yields the parallel builtins"),
        };
        use crate::core::ParOp;
        match op {
            ParOp::Both => {
                if args.len() != 2 {
                    return Err(Error::at(span, "parBoth takes exactly two callbacks"));
                }
                let a = self.uni.fresh();
                let b = self.uni.fresh();
                let f = self.parallel_callback(env, &args[0], &Ty::Unit, &a, "parBoth")?;
                let g = self.parallel_callback(env, &args[1], &Ty::Unit, &b, "parBoth")?;
                let classes = [self.class_of_ty(&a), self.class_of_ty(&b)];
                Ok((
                    app_spine(
                        ECore::Prim(PrimOp::Par { op: ParOp::Both, classes }),
                        vec![f, g],
                    ),
                    Ty::Tuple(vec![a, b]),
                ))
            }
            ParOp::Tabulate => {
                if args.len() != 2 {
                    return Err(Error::at(
                        span,
                        "parTabulate takes a count and one callback",
                    ));
                }
                let n = self.check(env, &args[0], &Ty::Nat)?;
                let a = self.uni.fresh();
                let f = self.parallel_callback(env, &args[1], &Ty::Nat, &a, "parTabulate")?;
                let classes = [self.class_of_ty(&a), crate::repr::Class::Raw];
                Ok((
                    app_spine(
                        ECore::Prim(PrimOp::Par { op: ParOp::Tabulate, classes }),
                        vec![n, f],
                    ),
                    Ty::Adt("Array".to_string().into(), vec![a]),
                ))
            }
            ParOp::MapArray => {
                if args.len() != 2 {
                    return Err(Error::at(span, "parMap takes a callback and one list"));
                }
                let elem = self.uni.fresh();
                let out = self.uni.fresh();
                let f = self.parallel_callback(env, &args[0], &elem, &out, "parMap")?;
                let xs_ty = Ty::Adt(crate::known::LIST.into(), vec![elem.clone()]);
                let xs = self.check(env, &args[1], &xs_ty)?;
                // The input array is one block every participant reads, so
                // its element type must be shareable (section 2.3). The
                // result type has no constraint: each result is built by one
                // participant and moves to the caller at the join.
                let actual = self.uni.zonk(&elem);
                if !self.share_safe(&actual, &env.dicts) {
                    return Err(Error::at(
                        args[1].span(),
                        crate::diagnostic!(
                            "parMap's element type must be Share: {} is stored in the shared input array, which every participant reads (design/parallel-evaluation.md section 2.3)",
                            actual
                        ),
                    ));
                }
                // `map f xs` = `array-to-list (parMapArray f (array-from-list xs))`,
                // so the runtime never builds list cells (section 4.2).
                let array_in = Ty::Adt("Array".to_string().into(), vec![elem.clone()]);
                let from_value = self.prelude_value("Data.Array", "array-from-list")?;
                let from = self.value_use(&from_value, span, env)?;
                let (arr, _) = self.apply_known(from, xs, &xs_ty, &array_in, args[1].span())?;
                let classes = [self.class_of_ty(&out), crate::repr::Class::Raw];
                let mapped = app_spine(
                    ECore::Prim(PrimOp::Par { op: ParOp::MapArray, classes }),
                    vec![f, arr],
                );
                let array_out = Ty::Adt("Array".to_string().into(), vec![out.clone()]);
                let to_value = self.prelude_value("Data.Array", "array-to-list")?;
                let to = self.value_use(&to_value, span, env)?;
                let list_out = Ty::Adt(crate::known::LIST.into(), vec![out.clone()]);
                self.apply_known(to, mapped, &array_out, &list_out, span)
            }
        }
    }

    /// Check one parallel callback against `domain ->{e} result`, restrict
    /// it to the parallel-safe row, and record its captures for the
    /// definition's end. The callback's performed row joins the call's,
    /// exactly as evaluating the defining equation would.
    fn parallel_callback(
        &mut self,
        env: &mut Env,
        arg: &ast::Term,
        domain: &Ty,
        result: &Ty,
        op: &'static str,
    ) -> Result<ECore> {
        let row = self.uni.fresh_row();
        let callback_ty = Ty::fun_mult(domain.clone(), Mult::Many, row.clone(), result.clone());
        // A literal callback body runs inside the call: its uses are
        // one-shot for `parBoth`'s branches (each runs exactly once and
        // may consume a `%1` capture) and many-shot for the other two (the
        // capture rules refuse a move unconditionally). Either way the
        // closure is dropped before the call returns, so it cannot outlive
        // a borrowed region.
        let lam = matches!(arg, ast::Term::Lam(_, _, _));
        if lam {
            self.oneshot_frames.push(OneshotFrame {
                lam: self.lam_depth + 1,
                task: false,
                parallel: true,
                scope: String::new(),
                outer: Vec::new(),
            });
        }
        let core = self.check(env, arg, &callback_ty);
        if lam {
            self.oneshot_frames.pop();
        }
        let core = core?;
        self.parallel_safe_row(&row, arg.span(), op)?;
        self.note_parallel_closure(&core, arg.span(), &env.dicts, op)?;
        let resolved = self.uni.resolve_row(&row);
        self.perform(&resolved)?;
        Ok(core)
    }

    /// A parallel callback may perform only the parallel-safe row: the
    /// closed empty row or `{Diverge}` (section 2.2). `Diverge` is admitted
    /// because the sequential reading already permits it and the caller's
    /// row carries the label; every other label would make the schedule
    /// observable, pin a participant, or let a branch hold a resource whose
    /// operations are single-threaded. An effect handled inside the branch
    /// never appears in its row, so a branch may install its own handler.
    fn parallel_safe_row(&mut self, row: &EffRow, span: ast::Span, op: &'static str) -> Result<()> {
        let resolved = self.uni.resolve_row(row);
        if let Some(label) = resolved
            .labels
            .iter()
            .find(|l| l.head.as_str() != "Diverge")
        {
            return Err(Error::at(
                span,
                format!(
                    "a {op} callback may perform only the parallel-safe row ({{}} or {{Diverge}}), but it performs {label}: order, foreign state and thread affinity are not promised across a parallel call (design/parallel-evaluation.md section 2.2)"
                ),
            ));
        }
        match &resolved.tail {
            RowTail::Closed => Ok(()),
            RowTail::Rigid(_) => Err(Error::at(
                span,
                format!(
                    "a {op} callback's effects must be the empty row or {{Diverge}}, but its row is {resolved}, whose tail is a rigid row variable: a row-polymorphic caller cannot supply a parallel callback in this slice (design/parallel-evaluation.md section 2.2)"
                ),
            )),
            // An unsolved tail carries no further label the body observed,
            // so it closes here, exactly as an unsolved row defaults at the
            // end of a definition. Closing the tail rather than comparing
            // the whole row with the empty one keeps the labels the branch
            // performed, which the call carries to its caller.
            RowTail::Meta(_) => {
                let closed = EffRow {
                    labels: resolved.labels.clone(),
                    tail: RowTail::Closed,
                };
                self.uni
                    .unify_rows(&resolved, &closed)
                    .map_err(|m| Error::at(span, m))
            }
        }
    }

    /// Record the captures of a parallel callback (section 2.3). A callback
    /// is read literally at the call site: a lambda literal, a global
    /// function name, a partial application of a global, or a local alias of
    /// any of these. A function-typed parameter is refused, because its
    /// captures are unknown.
    fn note_parallel_closure(
        &mut self,
        core: &ECore,
        span: ast::Span,
        dicts: &[(LocalId, ClassApp)],
        op: &'static str,
    ) -> Result<()> {
        let many_shot = op != "parBoth";
        let mut captures = self.callback_captures(core, op, span)?;
        let mut seen = HashSet::new();
        while let Some(local) = captures.pop() {
            let origin = self.value_aliases.get(&local).copied().unwrap_or(local);
            if !seen.insert(origin) {
                continue;
            }
            if let Some(nested) = self.local_closure_captures.get(&origin) {
                let nested = nested.clone();
                captures.extend(nested);
                continue;
            }
            let Some(binding) = self.obligations.get(&origin) else { continue };
            self.par_captures.push(ParCapture {
                binder: origin,
                name: binding.name.clone(),
                ty: binding.ty.clone(),
                span,
                dicts: dicts.to_vec(),
                many_shot,
                op,
            });
        }
        Ok(())
    }

    /// The locals a literal callback reads from the caller's frame, or the
    /// refusal for a callback whose captures are not visible here.
    fn callback_captures(
        &self,
        core: &ECore,
        op: &'static str,
        span: ast::Span,
    ) -> Result<Vec<LocalId>> {
        let mut captures = Vec::new();
        match core {
            ECore::Lam { .. } => {
                free_locals(core, &mut Vec::new(), &mut captures);
                Ok(captures)
            }
            // A global function name has no captures.
            ECore::Global(..) => Ok(captures),
            // A partial application of a global to arguments captures
            // whatever those arguments read.
            ECore::App(head, args) => {
                let mut head = &**head;
                while let ECore::App(f, _) = head {
                    head = &**f;
                }
                if !matches!(head, ECore::Global(..)) {
                    return Err(Error::at(span, self.callback_refusal(op, None)));
                }
                for a in args {
                    free_locals(a, &mut Vec::new(), &mut captures);
                }
                Ok(captures)
            }
            ECore::Var(local) => {
                let origin = self.value_aliases.get(local).copied().unwrap_or(*local);
                match self.local_closure_captures.get(&origin) {
                    Some(known) => Ok(known.clone()),
                    None => Err(Error::at(span, self.callback_refusal(op, Some(origin)))),
                }
            }
            _ => Err(Error::at(span, self.callback_refusal(op, None))),
        }
    }

    /// The refusal for a callback whose captures cannot be read, naming the
    /// binding when the callback is a function-typed one.
    fn callback_refusal(&self, op: &'static str, local: Option<LocalId>) -> String {
        let name = local
            .and_then(|l| self.obligations.get(&l))
            .map(|b| {
                format!(
                    ": {} is a function-typed binding whose captures are unknown",
                    b.name
                )
            })
            .unwrap_or_default();
        format!(
            "{} must be applied directly to a callback written at the call site (a lambda literal, a global function, or a partial application of one){name} (design/parallel-evaluation.md section 2.3)",
            op
        )
    }

    /// The representation class of a type as the enclosing definition's
    /// class vector sees it: a type parameter takes its index, a resolved
    /// type its own class, and an undetermined metavariable `Raw` (no word
    /// of such a type is ever produced at run time).
    fn class_of_ty(&self, ty: &Ty) -> crate::repr::Class {
        let z = self.uni.zonk(ty);
        crate::repr::repr_of(&z, &self.body_tparams, self, &mut |_| {})
    }

    /// A prelude value by its defining module, whether or not the current
    /// module imports that module: the parallel lowering needs the array
    /// conversions, which are the prelude's business, not the caller's.
    fn prelude_value(&self, module: &str, name: &str) -> Result<ValueInfo> {
        if let Ok(v) = self.lookup_value(Some(module), name) {
            return Ok(v);
        }
        self.values
            .get(name)
            .and_then(|vs| vs.iter().find(|v| v.module == module))
            .cloned()
            .ok_or_else(|| {
                Error::plain(format!(
                    "the prelude's {module}.{name} is unavailable (the parallel builtins lower through it)"
                ))
            })
    }

    /// Apply an elaborated argument to an elaborated function, unifying the
    /// arrow's domain with the argument's type and its codomain with
    /// `result` (the caller states both ends, and the middle pins the
    /// instantiation's classes).
    fn apply_known(
        &mut self,
        (core, ty): (ECore, Ty),
        arg: ECore,
        arg_ty: &Ty,
        result: &Ty,
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let ty = self.uni.zonk(&ty);
        let Ty::Fun(dom, _, eff, cod) = ty else {
            return Err(Error::plain(
                "internal error: a value used as a conversion is not a function",
            ));
        };
        self.uni
            .unify(&dom, arg_ty)
            .map_err(|m| Error::at(span, m))?;
        self.uni
            .unify(&cod, result)
            .map_err(|m| Error::at(span, m))?;
        self.perform(&eff)?;
        Ok((app_spine(core, vec![arg]), (*cod).clone()))
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
                cores.push(app_spine(ECore::Ctor(gid.clone(), vec![k.clone(), v.clone()]), vec![kc, vc]));
            }
            Ty::Tuple(vec![k, v])
        } else {
            let e = self.uni.fresh();
            for it in items {
                cores.push(self.check(env, it, &e)?);
            }
            e
        };
        let list_ty = Ty::Adt(crate::known::LIST.into(), vec![elem.clone()]);
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
            site: format!("{}:{}", span.line(), span.col()),
            kind: if map { "map" } else { "list" }.to_string(),
            folded: false,
            result_ty: result.clone(),
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
        let list_ty = Ty::Adt(crate::known::LIST.into(), vec![a.clone()]);
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
            site: format!("{}:{}", span.line(), span.col()),
            kind: "range".to_string(),
            folded: false,
            result_ty: result.clone(),
        });
        Ok((
            ECore::LitFold {
                idx,
                body: Box::new(app_spine(mcore, vec![app_spine(ecore, args)])),
            },
            result,
        ))
    }

    /// Direct calls receive preconditions from the exact instantiation that
    /// produced their callee, including type, index, row and evidence bindings.
    fn infer_variable(&mut self, env: &mut Env, p: &ast::VarPath, call_arity: usize) -> Result<(ECore, Ty, Vec<PropOblig>)> {
        if p.quals.is_empty()
            && self.is_builtin_form(env, &p.name)
        {
            return Err(Error::at(
                p.span(),
                format!("{} must be applied in the bootstrap slice", p.name.text),
            ));
        }
        if p.quals.is_empty() {
            if let Some((l, s)) = env.lookup_name(&p.name) {
                let (t, copy, obligations) = self.instantiate_local(l, &s);
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
                    return Ok((ECore::Demand(Box::new(ECore::Var(l)), (*inner).clone()), *inner, obligations));
                }
                let mut path = std::mem::take(&mut self.place_path);
                path.reverse();
                self.note_use(l, p.span(), &path, &env.dicts)?;
                return Ok((copy.unwrap_or(ECore::Var(l)), t, obligations));
            }
        }
        self.place_path.clear();
        // The bare-`pure` rule: a bare `pure` reaches the class
        // method through its constraint when one carries it.
        if p.quals.is_empty()
            && p.name.text == "pure"
            && p.name.local_binding().is_none()
            && p.name.resolved_binding().is_none()
            && p.name.scoped_reference().is_none()
            && env.lookup_name(&p.name).is_none()
        {
            if let Some(v) = self.constraint_pure(env) {
                return self.value_use_with_obligations(&v, p.span(), env);
            }
        }
        let v = self.resolve_var(p, p.span())?;
        if matches!(v.kind, ValueKind::Prim(PrimOp::Conc(ConcOp::Spawn))) {
            return Err(Error::at(p.span(), "spawn must be applied directly to check the task callback"));
        }
        if let ValueKind::Prim(PrimOp::Par { op, .. }) = v.kind {
            return Err(Error::at(
                p.span(),
                format!(
                    "{} must be applied directly to check its parallel callback (design/parallel-evaluation.md section 4.1)",
                    match op {
                        crate::core::ParOp::Both => "parBoth",
                        crate::core::ParOp::Tabulate => "parTabulate",
                        crate::core::ParOp::MapArray => "parMap",
                    }
                ),
            ));
        }
        if call_arity != 2 && !self.task_forwarders.is_empty()
            && self.task_forwarders.contains(&fn_id(&v.module, &v.name))
        {
            return Err(Error::at(p.span(), "a task forwarding helper must be fully applied in its source compilation"));
        }
        if matches!(v.kind, ValueKind::EffectOp { .. }) {
            return self.operation_function_value(env, p, &v, p.span()).map(|(core, ty)| (core, ty, Vec::new()));
        }
        self.value_use_with_obligations(&v, p.span(), env)
    }

    fn infer(&mut self, env: &mut Env, term: &ast::Term) -> Result<(ECore, Ty)> {
        let scope = self.compile_time.then(|| self.code_quantifiers.clone());
        let (core, ty) = self.infer_inner(env, term)?;
        // A synthesized polytype is used at a type: instantiate it
        // (design/rank-n-polymorphism.md section 3). No metavariable is
        // ever bound to a polytype, so the unzonked type answers exactly
        // and this hot path pays no traversal.
        let (core, ty) = if matches!(ty, Ty::Forall(_)) { self.poly_elim_all(core, ty) } else { (core, ty) };
        self.remember_runtime_index_reading(term, env, &core, &ty);
        Ok((self.code_expectation(core, &ty, term.span(), scope), ty))
    }

    fn remember_runtime_index_reading(&mut self, term: &ast::Term, env: &Env, core: &ECore, carrier: &Ty) {
        let ast::Term::App(head, args, _) = term else { return };
        let ast::Term::Var(path) = &**head else { return };
        if args.len() != 2 || !matches!(path.name.source_spelling(), "+" | "-" | "*") { return; }
        // Retain only the selected callable, not a second copy of a large
        // argument tree at every node of nested arithmetic.
        let mut selected = core; let mut arity = 0;
        while let ECore::App(function, arguments) = selected { arity += arguments.len(); selected = function; }
        if arity != 2 { return; }
        self.runtime_index_readings.push(RuntimeIndexReading {
            name: path.name.clone(), locals: env.locals.iter().map(|(_, local, _)| *local).collect(),
            dictionaries: env.dicts.iter().map(|(local, _)| *local).collect(), core: selected.clone(), carrier: carrier.clone(),
        });
    }

    fn infer_inner(&mut self, env: &mut Env, term: &ast::Term) -> Result<(ECore, Ty)> {
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
            ast::Term::Str(s, _span) => {
                // A runtime `String` literal (D43's deferral closed):
                // a shared read-only data-section constant over the D89
                // representation, the same constant form the `import
                // data` converters bind. `String` is immutable, so the
                // shared object is semantics-transparent: every
                // evaluation references the one constant (no
                // construction, no allocation), and the `#[no-alloc]`/
                // `jpl` alloc-site fact stays silent for literals. In
                // compile-time position the staging interpreter keeps
                // its own string values (decision D88), whose escapes
                // the lexer decoded the same way.
                if self.compile_time {
                    return Ok((
                        ECore::StrLit(s.clone()),
                        Ty::Adt(crate::known::STRING.into(), Vec::new()),
                    ));
                }
                Ok((self.string_const(s), Ty::Adt(crate::known::STRING.into(), Vec::new())))
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
            ast::Term::Var(p) => self.infer_variable(env, p, 0).map(|(core, ty, _)| (core, ty)),
            ast::Term::Con(p) => {
                // A capitalized module parameter (`Seq :: ResultSequencing
                // ConfigError`) is an ordinary binding of the module scope:
                // it resolves to its binder before any constructor (module
                // parameters as module arguments).
                if let Some((l, ty)) = self.module_param_use(p) {
                    return Ok((ECore::Var(l), ty));
                }
                if let Some(value) = self.named_instance_value(env, p)? { return Ok(value); }
                let v = self.resolve_con(p, p.span())?;
                self.record_quote_constructor(p, &v);
                self.value_use(&v, p.span(), env)
            }
            ast::Term::App(f, args, span) => {
                if let Some((operation, ty)) = self.annotated_operation(env, f)? {
                    return self.infer_operation_args(env, &operation, ty, args, *span);
                }
                // Builtin deferral and borrow forms: `delay`, `force`,
                // `demand`, `copyOut`, `borrow`, `mutBorrow`. They are
                // special forms; a user definition of the same name shadows
                // them.
                if let ast::Term::Var(p) = &**f {
                    if p.quals.is_empty() && self.is_builtin_form(env, &p.name) {
                        return self.infer_builtin_app(env, p.name.source_spelling(), args, *span);
                    }
                    if p.quals.is_empty() && self.is_provider_pure(env, &p.name) {
                        // `pure e` is the selected provider's return.
                        let seq = ast::Term::App(
                            Box::new(ast::Term::SeqOp(ast::SeqKind::Pure, *span)),
                            args.clone(),
                            *span,
                        );
                        return self.infer(env, &seq);
                    }
                    if !p.quals.is_empty() || env.lookup_name(&p.name).is_none() {
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
                // Deterministic pure parallelism (design/parallel-evaluation.md
                // section 2): the callbacks are read literally at the call.
                if let Some(v) = self.par_builtin(env, f) {
                    return self.infer_par_app(env, &v, args, *span);
                }
                // A constructor application is type-directed even in
                // inference position (erased evidence arguments never
                // reach runtime).
                if let ast::Term::Con(p) = &**f {
                    let fresh = self.uni.fresh();
                    return self.ctor_app(env, p, args, &fresh, *span);
                }
                // Private argument witnesses belong only to this call.
                let mut call_env = env.clone();
                let env = &mut call_env;
                let mut call_binds = Vec::new();
                let mut retained_method = None;
                // A `using` context's refinement obligations discharge
                // at the call's arguments (refinement types design,
                // "instance heads at refinements"): a signature's own
                // obligations now, an instance's after resolution.
                // A qualified callee (`Data.Nat.nat-sub a b`) carries its
                // obligations exactly like a bare one.
                // Only a variable callee is "applied directly": its
                // obligations discharge below. Any other callee form (a
                // conditional, a parenthesized application) elaborates as
                // an ordinary value, so an obligation-bearing function
                // inside it is refused as a value; and the arguments are
                // never in callee position.
                let saved_depth = std::mem::replace(
                    &mut self.app_callee_depth,
                    usize::from(matches!(&**f, ast::Term::Var(_))),
                );
                let inferred = match &**f {
                    ast::Term::Var(path) => {
                        let scope = self.compile_time.then(|| self.code_quantifiers.clone());
                        self.infer_variable(env, path, args.len()).map(|(core, ty, obligations)| {
                            let forwarder = matches!(&core, ECore::Global(id, _) if self.task_forwarders.contains(id));
                            (self.code_expectation(core, &ty, path.span(), scope), ty, obligations, forwarder)
                        })
                    }
                    _ => self.infer(env, f).map(|(core, ty)| (core, ty, Vec::new(), false)),
                };
                self.app_callee_depth = saved_depth;
                let (mut core, mut ty, callee_obligs, forwarder) = inferred?;
                // A callee that writes memory at an unknown target (a
                // foreign call, `eval`, a mutating `Bytes` primitive) is an
                // opaque write that invalidates all mutable-location
                // evidence (evidence invalidation over mutable state).
                self.note_opaque_write_from(&core, *span);
                let mut call_args: Vec<(Option<ITerm>, Ty, ast::Span)> = Vec::new();
                for (position, a) in args.iter().enumerate() {
                    ty = self.uni.zonk(&ty);
                    if matches!(ty, Ty::Forall(_)) {
                        let (c, t) = self.poly_elim_all(core, ty);
                        core = c;
                        ty = t;
                    }
                    let prefix_ty = ty.clone();
                    // Only a direct global carries a temporal call-step identity.
                    // Do not copy an arbitrarily large partial application.
                    let write_core = matches!(&core, ECore::Global(..)).then(|| core.clone());
                    let mut apply_erased = false;
                    let (arg_core, cod) = match ty {
                        Ty::Fun(dom, m, eff, cod) => {
                            self.perform(&eff)?;
                            let logical = matches!(self.uni.zonk(&dom), Ty::Prp(_) | Ty::Sort(_));
                            apply_erased = m == Mult::Zero && !logical;
                            let (c, reading) = if apply_erased {
                                (None, self.check_erased_value_argument(env, a, &dom, false)?)
                            } else {
                                let (c, reading) = self.check_call_argument(
                                    env, a, &dom, &prefix_ty, &mut core,
                                    &mut call_binds, &mut retained_method, !callee_obligs.is_empty(),
                                )?;
                                (if logical { None } else { Some(c) }, reading)
                            };
                            call_args.push((reading, self.uni.zonk(&dom), a.span()));
                            // An erased parameter does not give the callee a
                            // runtime borrow. The callee's row still applies.
                            let write_domain = if apply_erased { &Ty::Unit } else { &dom };
                            self.note_call_arg_writes(env, args, write_domain, &eff, *span, write_core.as_ref().unwrap_or(&core));
                            if !apply_erased {
                                self.check_handle_param(&dom, m, a.span())?;
                            }
                            (c, *cod)
                        }
                        Ty::Pi { var, dom, mult, eff, cod } => {
                            self.perform(&eff)?;
                            let logical = matches!(self.uni.zonk(&dom), Ty::Prp(_) | Ty::Sort(_));
                            apply_erased = mult == Mult::Zero && !logical;
                            let (c, reading) = if apply_erased {
                                (None, self.check_erased_value_argument(env, a, &dom, true)?)
                            } else {
                                let (c, reading) = self.check_call_argument(
                                    env, a, &dom, &prefix_ty, &mut core,
                                    &mut call_binds, &mut retained_method, true,
                                )?;
                                (if logical { None } else { Some(c) }, reading)
                            };
                            let write_domain = if apply_erased { &Ty::Unit } else { &dom };
                            self.note_call_arg_writes(env, args, write_domain, &eff, *span, write_core.as_ref().unwrap_or(&core));
                            let subst = reading.ok_or_else(|| Error::at(
                                a.span(), "this argument has no index reading (this function's result type mentions its argument)",
                            ))?;
                            call_args.push((Some(subst.clone()), self.uni.zonk(&dom), a.span()));
                            (c, cod.subst_idx(&var, &subst))
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
                                crate::diagnostic!("cannot apply a value of type {}", other),
                            ));
                        }
                    };
                    if forwarder && position == 1 {
                        let Some((_, scope_ty, _)) = call_args.first() else {
                            return Err(Error::at(*span, "task forwarding requires a checked scope argument"));
                        };
                        let scope_ty = self.uni.zonk(scope_ty);
                        let Ty::Adt(owner, regions) = &scope_ty else {
                            return Err(Error::at(*span, "task forwarding requires a checked scope argument"));
                        };
                        let [Ty::Region(_), Ty::Region(scope)] = regions.as_slice() else {
                            return Err(Error::at(*span, "task forwarding requires a checked scope argument"));
                        };
                        let Some(callback) = &arg_core else {
                            return Err(Error::at(a.span(), "task forwarding requires an executable callback"));
                        };
                        if owner != "Scope" {
                            return Err(Error::at(*span, "task forwarding requires a checked scope argument"));
                        }
                        let scope = self.uni.resolve_region(scope);
                        let outer = self.scope_frames.iter().rev()
                            .find(|frame| frame.region == scope)
                            .map(|frame| frame.outer.clone()).unwrap_or_default();
                        self.note_spawned_closure(callback, &scope, &outer, a.span(), &env.dicts, false)?;
                    }
                    if apply_erased {
                        core = app_spine(ECore::Prim(PrimOp::ApplyErased), vec![core]);
                    } else if let Some(c) = arg_core {
                        core = app_spine(core, vec![c]);
                    }
                    ty = cod;
                }
                // A result refinement naming an argument states its fact
                // over the argument's reading here, when that reading is
                // an admissible fact term (a byte read or an effectful
                // call could change under the fact): otherwise, and for a
                // partial application, it drops to the container.
                {
                    let zty = self.uni.zonk(&ty);
                    if Checker::has_positional_result(&zty) {
                        let readings: Vec<Option<ITerm>> = args.iter().enumerate()
                            .map(|(i, a)| call_args.get(i).and_then(|(reading, _, _)| reading.clone())
                                .filter(|it| matches!(it, ITerm::Var(name) if name.starts_with("$call-argument:")))
                                .or_else(|| if self.fact_term_ok(env, a) { self.idx_of_term(a, env).ok() } else { None }))
                            .collect();
                        let resolved = Checker::resolve_positional_result(&zty, &readings);
                        if resolved != zty {
                            ty = resolved;
                        }
                    }
                }
                // A signature's refinement obligations discharge at the
                // call (the `Dec` evidence path); a method call's
                // instance obligations record for the discharge after
                // resolution.
                if !callee_obligs.is_empty() {
                    for o in &callee_obligs {
                        let crate::types::PropOblig::Pred(p) = o else {
                            continue;
                        };
                        // The positional binder `$a{i}` names the i-th
                        // argument of the call.
                        let mut fv = Vec::new();
                        p.free_vars(&mut fv);
                        let mut inst = p.clone();
                        let mut bound = true;
                        let mut idxs: Vec<usize> = Vec::new();
                        for v in &fv {
                            let idx = v
                                .strip_prefix("$a")
                                .and_then(|i| i.parse::<usize>().ok())
                                .ok_or_else(|| {
                                    Error::at(
                                        *span,
                                        format!(
                                            "the refinement obligation `{p}` names no argument of this function"
                                        ),
                                    )
                                })?;
                            idxs.push(idx);
                            match call_args.get(idx).and_then(|(it, _, _)| it.clone()) {
                                Some(it) => inst = inst.subst(v, &it),
                                None => bound = false,
                            }
                        }
                        if !bound {
                            if idxs.iter().all(|index| *index < args.len()) {
                                let mut displayed = p.clone();
                                for (index, argument) in args.iter().enumerate() {
                                    if let Some(reading) = self.diagnostic_term_index(argument, env) {
                                        displayed = displayed.subst(&format!("$a{index}"), &reading);
                                    }
                                }
                                return Err(Error::at(*span, format!("the refinement obligation `{displayed}` needs `Dec` evidence or a branch fact at this use: an actual runtime argument has no checked mathematical index reading")));
                            }
                            return Err(Error::at(
                                *span,
                                format!(
                                    "the refinement obligation `{p}` must be discharged at this call: apply this function to all of its arguments here (an obligation-bearing function must be fully applied)"
                                ),
                            ));
                        }
                        if self.decide_closed_oblig(&inst, *span)? {
                            continue;
                        }
                        let static_ok = idxs.iter().any(|idx| {
                            call_args
                                .get(*idx)
                                .map(|(_, ty, _)| self.static_discharge(ty, &inst))
                                .unwrap_or(false)
                        });
                        if static_ok {
                            continue;
                        }
                        let ev = self.evidence_in(env);
                        if let Err(mut error) = self.discharge_oblig(&ev, &inst, *span) {
                            // Private witnesses identify one evaluation for
                            // proof checking. Diagnostics still name the
                            // user's original expression. Reify only after
                            // failure, without changing the successful path.
                            let mut displayed = inst.clone();
                            for index in &idxs {
                                if let Some((Some(ITerm::Var(name)), _, _)) = call_args.get(*index) {
                                    if name.starts_with("$call-argument:") {
                                        if let Some(reading) = args.get(*index)
                                            .and_then(|argument| self.idx_of_term(argument, env).ok())
                                        {
                                            displayed = displayed.subst(name, &reading);
                                        }
                                    }
                                }
                            }
                            error.message = error.message.replace(
                                &format!("`{inst}`"), &format!("`{displayed}`"),
                            );
                            return Err(error);
                        }
                    }
                }
                // A returned function or data wrapper can retain arguments
                // in a closure even when its visible type omits them. Apply
                // the same borrowed-environment rule as an explicit lambda:
                // an active structured-scope region must remain visible in
                // the returned value type. Known values whose checked fields
                // cannot retain closures keep their ordinary path.
                let result_ty = self.uni.zonk(&ty);
                if !self.scope_frames.is_empty() && self.value_may_hide_scope(&result_ty, &mut Vec::new(), 64) {
                    let mut visible = Vec::new();
                    all_regions(&result_ty, &mut visible);
                    for (_, argument_ty, argument_span) in &call_args {
                        let mut captured = Vec::new();
                        all_regions(&self.uni.zonk(argument_ty), &mut captured);
                        for region in captured {
                            let region = self.uni.resolve_region(&region);
                            let active = self.scope_frames.iter().any(|frame|
                                self.uni.resolve_region(&frame.region) == region);
                            if active && !visible.iter().any(|name| self.uni.resolve_region(name) == region) {
                                return Err(Error::at(*argument_span, format!(
                                    "region escape: a returned value captures structured scope region {region} without retaining its lifetime in its type (concurrency specification, section 2)"
                                )));
                            }
                        }
                    }
                }
                if let Some((want, method)) = retained_method.or_else(|| callee_want(&core).map(|want|
                    (want, head_method(&core).filter(|(w, _)| *w == want).map(|(_, i)| i)))) {
                    self.prop_calls.push(PropCall {
                        want,
                        method,
                        args: call_args,
                        evidence: self.evidence_in(env),
                        residual_fun: matches!(
                            self.uni.zonk(&ty),
                            Ty::Fun(..) | Ty::Pi { .. } | Ty::Meta(_)
                        ),
                        span: *span,
                    });
                }
                Ok((wrap_lets(call_binds, core), ty))
            }
            ast::Term::Lam(_, _, _) => {
                let t = self.uni.fresh();
                let core = self.check_lam(env, term, &t)?;
                Ok((core, t))
            }
            ast::Term::Let(decls, body, span) => self.in_let(*span, |s| {
                let mut env2 = env.clone();
                let (binds, group) = s.elab_locals(&mut env2, decls, LocalForm::Plain)?;
                let (bcore, bty) = s.infer(&mut env2, body)?;
                if let Some(g) = &group {
                    s.discharge_group(g, &bty, *span)?;
                }
                Ok((wrap_lets(binds, bcore), bty))
            }),
            ast::Term::Letrec(decls, body, span) => self.in_let(*span, |s| {
                let mut env2 = env.clone();
                let recursive = s.elab_letrec(&mut env2, decls)?;
                let (bcore, bty) = s.infer(&mut env2, body)?;
                let scoped = s.finish_recursive_bindings(recursive, bcore, &bty, *span)?;
                Ok((scoped, bty))
            }),
            ast::Term::If { cond, then, els, cond_attrs, then_attrs, else_attrs, span } => {
                let (tw, fw) =
                    self.if_weights(cond_attrs, then_attrs, else_attrs, *span)?;
                // `if c then a else b` is `case c of True -> a; False ->
                // b`: the condition reads as a `Bool` column and each
                // branch binds that column's facts by the match rule
                // (dependent core kernel design section 8).
                let (ccore, reading) = self.cond_reading(env, cond)?;
                let cspan = cond.span();
                let ((tcore, tty), tm) = self.isolate(|s| {
                    s.in_branch(env, &reading, &BranchKey::Bool(true), cspan, |s, env| s.infer(env, then))
                })?;
                let (ecore, em) = self.isolate(|s| {
                    s.in_branch(env, &reading, &BranchKey::Bool(false), cspan, |s, env| {
                        s.check(env, els, &tty)
                    })
                })?;
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
                let (scrut_core, scrut_ty, reading) = self.infer_scrutinee(env, scrut)?;
                let scrut_ty = self.uni.zonk(&scrut_ty);
                let res = self.uni.fresh();
                let core =
                    self.compile_alts(env, alts, scrut_core, scrut_ty, reading, &res, *span)?;
                Ok((core, res))
            }
            ast::Term::Ann(e, ty, _) => {
                if matches!(ty, ast::Type::Forall(..)) {
                    return self.infer_forall_annotation(env, e, ty);
                }
                let t = self.annotation_type(env, ty)?;
                if matches!(self.uni.zonk(&t), Ty::Prp(_)) {
                    // A Code contract for Prop must enter the evidence
                    // checker, rather than admitting a computational value
                    // merely because its type can unify with a proposition.
                    return Ok((self.check(env, e, &t)?, t));
                }
                if matches!(ty, ast::Type::Elaborated(..))
                    && !matches!(self.uni.zonk(&t), Ty::Fun(..) | Ty::Pi { .. })
                {
                    // A native Code expectation validates the carrier without
                    // forgetting stronger facts inferred from the actual tree.
                    // Function expectations still guide lambda usage checking.
                    let (core, actual) = self.infer(env, e)?;
                    if matches!(self.uni.zonk(&t), Ty::Meta(_)) {
                        self.uni.unify(&t, &actual).map_err(|message| Error::at(e.span(), message))?;
                    } else {
                        self.check_subsume(env, e, &actual, &t)?;
                    }
                    return Ok((core, actual));
                }
                let core = self.check(env, e, &t)?;
                Ok((core, t))
            }
            ast::Term::Forget { atoms, value, span } => {
                // Partial weakening (refinement types design): the
                // value's refinement predicate minus the named
                // top-level conjuncts, at zero runtime cost (the same
                // word; the dropped evidence is simply no longer
                // required).
                let (core, vty) = self.infer(env, value)?;
                let vty = self.uni.zonk(&vty);
                let view = self.refine_view(&vty).ok_or_else(|| {
                    Error::at(
                        *span,
                        crate::diagnostic!(
                            "`forget` gives up conjuncts of a refinement's predicate, but this is {} (a plain container carries no predicate; `unrefine` projects a refined value to its container)",
                            vty
                        ),
                    )
                })?;
                let conj = pred_conjuncts(&view.pred);
                let mut keep: Vec<bool> = vec![true; conj.len()];
                for a in atoms {
                    let named = self.conv_pred_former(a, &[])?;
                    for nc in pred_conjuncts(&named) {
                        let mut found = false;
                        for (i, c) in conj.iter().enumerate() {
                            if keep[i] && pred_alpha_eq(c, &nc) {
                                keep[i] = false;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            return Err(Error::at(
                                *span,
                                format!(
                                    "`forget` gives up `{nc}`, which is not a top-level conjunct of `{}` (it operates on top-level conjuncts only; a structural drop inside a disjunction or a comparison goes through an ascription)",
                                    view.pred
                                ),
                            ));
                        }
                    }
                }
                let rest: Vec<PTerm> = conj
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| keep[*i])
                    .map(|(_, c)| c.clone())
                    .collect();
                if rest.is_empty() {
                    return Err(Error::at(
                        *span,
                        "`forget` keeps at least one conjunct (forgetting every conjunct reaches the container: `unrefine` is the explicit forgetful arrow, a different direction never conflated with weakening)",
                    ));
                }
                let mut it = rest.into_iter();
                let mut pred = it.next().expect("nonempty");
                for c in it {
                    pred = PTerm::And(Box::new(pred), Box::new(c));
                }
                let mut args = vec![Ty::Prp(Box::new(pred))];
                if let Some(b) = view.base {
                    args.push(b);
                }
                Ok((core, Ty::Adt("refined".to_string().into(), args)))
            }
            ast::Term::Narrow { value, target, witness, span } => {
                // The checked strengthening cast (refinement types
                // design, "narrowings"): always checked, carrying a
                // `Dec` of the stronger predicate through the same
                // evidence path as the transparent constructor (a
                // forged or stale witness is refused exactly so).
                let (core, vty) = self.infer(env, value)?;
                let vty = self.uni.zonk(&vty);
                let t = self.conv_type_in_env(env, target, &[])?;
                let tv = self.refine_view(&t).ok_or_else(|| {
                    Error::at(
                        target.span(),
                        crate::diagnostic!(
                            "`narrow` strengthens to a refinement (`refined P T` or a named refinement), but this is {} (the transparent constructor builds a refined value from its container)",
                            t
                        ),
                    )
                })?;
                let sv = self.refine_view(&vty).ok_or_else(|| {
                    Error::at(
                        value.span(),
                        crate::diagnostic!(
                            "`narrow` moves along the refinement lattice, but this value is at the plain container {} (the transparent constructor builds a refined value from its container; weakening along the lattice is implicit)",
                            vty
                        ),
                    )
                })?;
                // The target keeps every fact the value carries:
                // `narrow` never forgets (dropping goes through
                // `forget` or an ascription).
                if !pred_implies(&tv.pred, &sv.pred) {
                    return Err(Error::at(
                        *span,
                        format!(
                            "`narrow` strengthens along the lattice: the target's `{}` must imply the value's `{}` (the stronger point keeps every fact the value carries); drop conjuncts with `forget` or an ascription",
                            tv.pred, sv.pred
                        ),
                    ));
                }
                if let (Some(a), Some(b)) = (&sv.base, &tv.base) {
                    self.uni
                        .unify(a, b)
                        .map_err(|m| Error::at(*span, format!("`narrow`: {m}")))?;
                }
                // The stronger predicate speaks about the value (the
                // transparent constructor's rule): the witness is
                // evidence about the value's own index.
                let idx = self.idx_of_term(value, env).map_err(|e| {
                    Error::at(
                        value.span(),
                        format!(
                            "`narrow`'s value must be expressible as an index value (the stronger predicate speaks about it): {}",
                            e.message
                        ),
                    )
                })?;
                let mut target_pred = tv.pred.clone();
                let mut frees = Vec::new();
                target_pred.free_vars(&mut frees);
                for v in frees {
                    target_pred = target_pred.subst(&v, &idx);
                }
                if matches!(&**witness, ast::Term::Hole(_)) {
                    return Err(Error::at(
                        *span,
                        format!(
                            "`narrow` carries a `Dec` of the stronger predicate `{}` (pass it with `with w`: discharged statically where the implication table decides, or through the `Dec`/`decide` evidence path); weakening to a weaker point is implicit and needs no evidence",
                            target_pred
                        ),
                    ));
                }
                let prop = Ty::Prp(Box::new(target_pred.clone()));
                let decide_shaped = matches!(&**witness, ast::Term::App(f, dargs, _)
                    if dargs.len() == 1
                        && matches!(&**f, ast::Term::Var(p)
                            if p.quals.is_empty() && p.name.text == "decide"));
                if decide_shaped {
                    let dec_ty = Ty::Adt("Dec".into(), vec![prop]);
                    self.check(env, witness, &dec_ty)?;
                    match self.static_obligation(&target_pred)? {
                        ObligStatus::Checked => {}
                        ObligStatus::Disproved => {
                            return Err(Error::at(
                                witness.span(),
                                format!(
                                    "the discharge `{}` is disproved: no refined value can be constructed here (a failed check must not produce evidence)",
                                    target_pred
                                ),
                            ))
                        }
                        ObligStatus::Timeout => {
                            return Err(Error::at(
                                witness.span(),
                                format!(
                                    "the discharge `{}` ran out of reduction steps at this construction (none of these outcomes is a proof)",
                                    target_pred
                                ),
                            ))
                        }
                        ObligStatus::Unsupported | ObligStatus::Unresolved => {
                            return Err(Error::at(
                                witness.span(),
                                format!(
                                    "the discharge `{}` is not decided at this construction: discharge it in the `Dec`/`decide` world (`case decide ({}) of Yes w -> ...`) and pass the witness",
                                    target_pred, target_pred
                                ),
                            ))
                        }
                    }
                } else {
                    self.check(env, witness, &prop)?;
                }
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
                Ok((app_spine(ECore::Ctor(gid, tys.clone()), cores), Ty::Tuple(tys)))
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
                        let want = Ty::Adt(info.identity.clone().into(), fresh.clone());
                        self.uni
                            .unify(&ty, &want)
                            .map_err(|m| Error::at(*span, m))?;
                        (info.identity.as_str().into(), fresh)
                    }
                    other => {
                        return Err(Error::at(
                            *span,
                            crate::diagnostic!(
                                "cannot project field {} from a value of type {}",
                                field.text, other
                            ),
                        ));
                    }
                };
                let info = self.lookup_known_type(&rec_name).map_err(|e| {
                    Error::at(*span, e.message)
                })?;
                let fields = info.fields.ok_or_else(|| {
                    Error::at(
                        *span,
                        crate::diagnostic!(
                            "{} is not a record; field projection needs a record type",
                            crate::spelling::Nominal(&rec_name)
                        ),
                    )
                })?;
                let idx = fields.iter().position(|f| f == &field.text).ok_or_else(|| {
                    Error::at(field.span, crate::diagnostic!("unknown record field {} on {}", field.text, crate::spelling::Nominal(&rec_name)))
                })?;
                // Erased fields occupy no runtime slot: the block stores
                // the others in declaration order (core specification
                // section 3).
                let erased = info.field_erased.get(idx).copied().unwrap_or(false);
                let runtime_idx =
                    info.field_erased[..idx].iter().filter(|e| !**e).count();
                let v = self.qualified.get(&(info.module.clone(), info.name.clone())).cloned()
                    .ok_or_else(|| Error::at(*span, crate::diagnostic!("unknown constructor for {}", crate::spelling::Nominal(&rec_name))))?;
                let field_ty = match &v.kind {
                    ValueKind::Ctor { field_tys, ty_params, .. } => {
                        let mut map = HashMap::new();
                        for (p, a) in ty_params.iter().zip(&args) {
                            map.insert(p.clone(), a.clone());
                        }
                        Self::subst_ty(&field_tys[idx], &map)
                    }
                    _ => {
                        return Err(Error::at(*span, crate::diagnostic!("{} has no constructor", crate::spelling::Nominal(&rec_name))));
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
                                    class: Class::Raw,
                                    mutable: false,
                                },
                                // The stored observation closure.
                                Ty::fun(Ty::Unit, field_ty.clone()),
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
                            class: Class::Raw,
                            mutable: false,
                        },
                        field_ty.clone(),
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
                let (mut rest, _ctx, _val, ptys) = self.instantiate_at(&v.scheme);
                let ctor_core = ECore::Ctor(fn_id(&v.module, &v.name), ptys);
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
                                crate::diagnostic!(
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
                        self.note_local_ty(unit, &Ty::Unit);
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

    /// Instantiate a polytype-typed value at fresh metavariables: a
    /// projection of its family's one method at the use's own class vector
    /// (design/rank-n-polymorphism.md sections 3 and 4).
    fn poly_elim(&mut self, value: ECore, scheme: &Scheme) -> (ECore, Ty) {
        let ((ty, _, _, _), bindings) = self.instantiate_scoped(scheme);
        let own: Vec<Ty> = scheme.params.iter()
            .map(|p| bindings.types.get(p).cloned().unwrap_or_else(|| self.uni.fresh()))
            .collect();
        let family = poly_family(scheme);
        (ECore::DictProj(Box::new(value), 0, Some((family, own)), ty.clone()), ty)
    }

    /// Whether `term` is a variable or a call whose declared type yields a
    /// polytype once its arguments are applied. It reads declared types
    /// only, so it has no effect on inference or usage.
    fn synthesizes_polytype(&self, env: &Env, term: &ast::Term) -> bool {
        let (head, arguments) = match term {
            ast::Term::Var(p) => (p, 0),
            ast::Term::App(f, args, _) => match &**f {
                ast::Term::Var(p) => (p, args.len()),
                _ => return false,
            },
            _ => return false,
        };
        let declared = if head.quals.is_empty() {
            match env.lookup_name(&head.name) {
                Some((_, local)) => local.ty,
                None => match self.resolve_var(head, head.span()) { Ok(v) => v.scheme.ty, Err(_) => return false },
            }
        } else {
            match self.resolve_var(head, head.span()) { Ok(v) => v.scheme.ty, Err(_) => return false }
        };
        let mut ty = self.uni.zonk(&declared);
        for _ in 0..arguments {
            ty = match ty {
                Ty::Fun(_, _, _, cod) | Ty::Pi { cod, .. } => *cod,
                _ => return false,
            };
        }
        matches!(ty, Ty::Forall(_))
    }

    /// Instantiate every polytype at the top of an inferred type.
    fn poly_elim_all(&mut self, mut core: ECore, ty: Ty) -> (ECore, Ty) {
        let mut ty = self.uni.zonk(&ty);
        while let Ty::Forall(scheme) = ty {
            let (c, t) = self.poly_elim(core, &scheme);
            core = c;
            ty = self.uni.zonk(&t);
        }
        (core, ty)
    }

    /// Check a term against a polytype: skolemise its binders, check the
    /// body under a skolem scope and a fresh performed-effect frame, and
    /// require the body to be pure (design/rank-n-polymorphism.md
    /// section 3).
    fn check_poly_intro(&mut self, env: &mut Env, term: &ast::Term, scheme: &Scheme) -> Result<ECore> {
        let expected = Ty::Forall(Box::new(scheme.clone()));
        // A term that already synthesizes a polytype (a local of one, or a
        // call whose result is one) passes through unchanged: rebuilding
        // the dictionary would allocate and re-evaluate it per slot.
        if self.synthesizes_polytype(env, term) {
            let (core, actual) = self.infer_inner(env, term)?;
            let actual = self.uni.zonk(&actual);
            if matches!(actual, Ty::Forall(_)) {
                self.uni.unify(&actual, &expected).map_err(|m| Error::at(term.span(), m))?;
                return Ok(core);
            }
            return Err(Error::at(term.span(), crate::diagnostic!(
                "expected the polymorphic type {}, but this has type {}", expected, actual)));
        }
        let (rigid, _, bindings) = self.rigidify(scheme);
        let skolems: Vec<String> = scheme.params.iter()
            .filter_map(|p| match bindings.types.get(p) { Some(Ty::Param(n)) => Some(n.to_string()), _ => None })
            .collect();
        let mark = self.uni.enter_skolems(skolems.clone());
        let tparams_before = self.body_tparams.len();
        self.body_tparams.extend(skolems.iter().cloned());
        let saved = self.enter_perf();
        // The body is evaluated once per reached class vector, so its
        // captures count like a lambda body's: unboundedly.
        self.lam_depth += 1;
        let checked = self.check(env, term, &rigid);
        self.lam_depth -= 1;
        let performed = self.exit_perf(saved);
        self.body_tparams.truncate(tparams_before);
        self.uni.exit_skolems(mark);
        let body = checked?;
        if self.uni.unify_rows(&performed, &EffRow::pure()).is_err() {
            return Err(Error::at(term.span(), crate::diagnostic!(
                "a polymorphic value must be pure to build; this body performs {}",
                self.uni.resolve_row(&performed)
            )));
        }
        Ok(ECore::PolyIntro { skolems, body: Box::new(body), ty: rigid })
    }

    fn check(&mut self, env: &mut Env, term: &ast::Term, expected: &Ty) -> Result<ECore> {
        let scope = self.compile_time.then(|| self.code_quantifiers.clone());
        let core = self.check_inner(env, term, expected)?;
        self.remember_runtime_index_reading(term, env, &core, expected);
        Ok(self.code_expectation(core, expected, term.span(), scope))
    }

    fn check_inner(&mut self, env: &mut Env, term: &ast::Term, expected: &Ty) -> Result<ECore> {
        if let ast::Term::Ann(body, annotation @ ast::Type::Elaborated(..), _) = term {
            let actual = self.annotation_type(env, annotation)?;
            // An inference-created arrow is not permission to discard an
            // outer declared usage. Unify before checking when possible;
            // ordinary subsumption still validates refinements afterwards.
            let trial = self.uni.snapshot();
            if self.uni.unify(&actual, expected).is_ok() {
                self.uni.commit(trial);
            } else {
                self.uni.rollback_to(trial);
            }
            let actual = Self::code_usage_hint(&self.uni.zonk(&actual), &self.uni.zonk(expected));
            let previous_goal_site = self.proof_goal_site;
            if previous_goal_site.is_none() && matches!(self.uni.zonk(expected), Ty::Prp(_)) {
                self.proof_goal_site = Some(term.span());
            }
            let checked = self.check(env, body, &actual);
            self.proof_goal_site = previous_goal_site;
            let core = checked?;
            self.check_subsume(env, term, &actual, expected)?;
            return Ok(core);
        }
        // A proposition is checked as an erased proof term (core
        // specification section 3).
        if let Ty::Prp(p) = self.uni.zonk(expected) {
            return self.check_proof(env, term, &p);
        }
        // A polytype is checked by generalisation
        // (design/rank-n-polymorphism.md section 3). Predicativity keeps
        // polytypes out of metavariable bindings, so no zonk is needed.
        if let Ty::Forall(scheme) = expected {
            return self.check_poly_intro(env, term, scheme);
        }
        // Constructor applications are type-directed: the expected type
        // solves the constructor's index parameters before its arguments
        // are checked, and erased evidence arguments never reach runtime.
        match term {
            ast::Term::Con(p) => {
                if let Some((core, ty)) = self.named_instance_value(env, p)? {
                    self.uni.unify(&ty, expected).map_err(|m| Error::at(p.span(), m))?;
                    return Ok(core);
                }
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
                let (ccore, reading) = self.cond_reading(env, cond)?;
                let cspan = cond.span();
                let (tcore, tm) = self.isolate(|s| {
                    s.in_branch(env, &reading, &BranchKey::Bool(true), cspan, |s, env| {
                        s.check(env, then, expected)
                    })
                })?;
                let (ecore, em) = self.isolate(|s| {
                    s.in_branch(env, &reading, &BranchKey::Bool(false), cspan, |s, env| {
                        s.check(env, els, expected)
                    })
                })?;
                self.merge_branches(vec![tm, em]);
                Ok(ECore::If(
                    Box::new(ccore),
                    CBranch::weighted(tw, tcore),
                    CBranch::weighted(fw, ecore),
                ))
            }
            ast::Term::Let(decls, body, span) => self.in_let(*span, |s| {
                let mut env2 = env.clone();
                let (binds, group) = s.elab_locals(&mut env2, decls, LocalForm::Plain)?;
                let scoped = s.numeric_contract_in_value_scope(expected, env, &env2);
                let bcore = s.check(&mut env2, body, &scoped)?;
                if let Some(g) = &group {
                    s.discharge_group(g, expected, *span)?;
                }
                Ok(wrap_lets(binds, bcore))
            }),
            ast::Term::Letrec(decls, body, span) => self.in_let(*span, |s| {
                let mut env2 = env.clone();
                let recursive = s.elab_letrec(&mut env2, decls)?;
                let scoped = s.numeric_contract_in_value_scope(expected, env, &env2);
                let bcore = s.check(&mut env2, body, &scoped)?;
                s.finish_recursive_bindings(recursive, bcore, expected, *span)
            }),
            ast::Term::Case(scrut, alts, span) => {
                let (scrut_core, scrut_ty, reading) = self.infer_scrutinee(env, scrut)?;
                let scrut_ty = self.uni.zonk(&scrut_ty);
                self.compile_alts(env, alts, scrut_core, scrut_ty, reading, expected, *span)
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
            ast::Term::Int(v, span) => self.check_scoped_numeric_literal(env, term, expected, v, *span),
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
                let numeric_refinement = self.refine_view(&t).and_then(|view| view.base)
                    .is_some_and(|base| matches!(base, Ty::Int | Ty::Nat | Ty::Prim(_)));
                if matches!(t, Ty::Int | Ty::Nat | Ty::Prim(_)) || numeric_refinement {
                    if let Some(value) = neg_fold(inner) {
                        return self.check_scoped_numeric_literal(env, term, &t, &value, *span);
                    }
                }
                let (core, ty) = self.infer(env, term)?;
                self.check_subsume(env, term, &ty, expected)?;
                Ok(core)
            }
            ast::Term::App(f, args, span) => {
                // `delay (\() -> e)` against `Later a` builds the guarded
                // observation modality (core language specification
                // section 9; codata and guardedness design, section 2).
                if let ast::Term::Var(p) = &**f {
                    if p.quals.is_empty()
                        && p.name.source_spelling() == "delay"
                        && args.len() == 1
                        && self.is_builtin_form(env, &p.name)
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
                        && matches!(p.name.source_spelling(), "borrow" | "mutBorrow")
                        && self.is_builtin_form(env, &p.name)
                    {
                        let is_mut = p.name.source_spelling() == "mutBorrow";
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
                if let Some(v) = self.par_builtin(env, f) {
                    let (core, ty) = self.infer_par_app(env, &v, args, *span)?;
                    self.unify_arg(&ty, expected)
                        .map_err(|m| Error::at(term.span(), m))?;
                    return Ok(core);
                }
                let (core, ty) = self.infer(env, term)?;
                self.check_subsume(env, term, &ty, expected)?;
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
                    let cod = cod.subst_idx(&var, &idx);
                    let c1 = self.check(env, &items[1], &cod)?;
                    let gid = self.tuple_ctor(2, *span)?;
                    return Ok(app_spine(ECore::Ctor(gid, vec![(*dom).clone(), cod]), vec![c0, c1]));
                }
                let (core, ty) = self.infer(env, term)?;
                self.unify_arg(&ty, expected)
                    .map_err(|m| Error::at(term.span(), m))?;
                Ok(core)
            }
            _ => {
                let (core, ty) = self.infer(env, term)?;
                self.check_subsume(env, term, &ty, expected)?;
                Ok(core)
            }
        }
    }

    /// Check the discarded operand in its own usage and effect scope. It
    /// supplies an index when needed, but never commits a runtime use or a
    /// strict argument binding to the surrounding call.
    fn check_erased_value_argument(
        &mut self,
        env: &Env,
        argument: &ast::Term,
        expected: &Ty,
        dependent: bool,
    ) -> Result<Option<ITerm>> {
        let outer: HashSet<_> = self.obligations.keys().copied().collect();
        let effects = self.enter_perf();
        let writes = self.ev_writes.len();
        let captures = self.task_captures.len();
        let checked = self.isolate(|checker| {
            checker.check(&mut env.clone(), argument, expected)?;
            if dependent {
                let carrier = checker.uni.zonk(expected);
                checker.checked_term_index(argument, env, Some(&carrier)).map(Some)
            } else {
                Ok(checker.idx_of_term(argument, env).ok())
            }
        });
        let performed = self.exit_perf(effects);
        let wrote = self.ev_writes.len() != writes;
        self.ev_writes.truncate(writes);
        self.task_captures.truncate(captures);
        let (reading, uses) = checked?;
        if wrote || self.uni.unify_rows(&performed, &EffRow::pure()).is_err() {
            return Err(Error::at(
                argument.span(), "multiplicity %0: an erased argument cannot perform effects or writes",
            ));
        }
        for (local, usage) in uses {
            if !outer.contains(&local) {
                self.uses.entry(local).or_default().seq_add(&usage);
            }
        }
        Ok(reading)
    }

    /// Keep an opaque refined argument's actual value, rather than naming
    /// another evaluation of its source expression in a proof. The prefix
    /// is bound first so curried calls still evaluate left to right.
    fn check_call_argument(
        &mut self,
        env: &mut Env,
        argument: &ast::Term,
        expected: &Ty,
        prefix_ty: &Ty,
        prefix: &mut ECore,
        binds: &mut Vec<(LocalId, ECore)>,
        retained_method: &mut Option<(usize, Option<usize>)>,
        index_required: bool,
    ) -> Result<(ECore, Option<ITerm>)> {
        if !index_required { return self.check_call_argument_inner(env, argument, expected, prefix_ty, prefix, binds, retained_method); }
        self.pending_runtime_arithmetic.borrow_mut().push(Vec::new());
        let result = self.check_call_argument_inner(env, argument, expected, prefix_ty, prefix, binds, retained_method);
        let certificates = self.pending_runtime_arithmetic.borrow_mut().pop().expect("call index scope");
        if result.as_ref().is_ok_and(|(_, index)| index.is_some()) { self.use_runtime_arithmetic(certificates, argument.span()); }
        result
    }

    fn check_call_argument_inner(
        &mut self,
        env: &mut Env,
        argument: &ast::Term,
        expected: &Ty,
        prefix_ty: &Ty,
        prefix: &mut ECore,
        binds: &mut Vec<(LocalId, ECore)>,
        retained_method: &mut Option<(usize, Option<usize>)>,
    ) -> Result<(ECore, Option<ITerm>)> {
        let ordinary_app = match argument {
            ast::Term::App(f, _, _) => match &**f {
                ast::Term::Con(_) => false,
                ast::Term::Var(p) if p.quals.is_empty() && self.is_builtin_form(env, &p.name) => false,
                _ => true,
            },
            _ => false,
        };
        // A polytype domain is checked by generalisation, never by
        // subsumption of an inferred (and so instantiated) argument.
        if !ordinary_app || matches!(self.uni.zonk(expected), Ty::Prp(_) | Ty::Sort(_) | Ty::Forall(_)) {
            return Ok((self.check(env, argument, expected)?, self.idx_of_term(argument, env).ok()));
        }
        // Ordinary applications use inference followed by subsumption in
        // `check` too. Constructors, lambdas and proof terms retain their
        // original bidirectional checking path above.
        let (checked, actual) = self.infer(env, argument)?;
        self.check_subsume(env, argument, &actual, expected)?;
        let actual = self.uni.zonk(&actual);
        let refinement = self.refine_view(&actual)
            .filter(|r| r.base.is_some() && !self.fact_term_ok(env, argument));
        let Some(refinement) = refinement else {
            return Ok((checked, self.idx_of_term(argument, env).ok()));
        };
        if retained_method.is_none() {
            *retained_method = callee_want(prefix).map(|want|
                (want, head_method(prefix).filter(|(w, _)| *w == want).map(|(_, i)| i)));
        }
        let function = self.fresh_local();
        self.note_local_ty(function, prefix_ty);
        binds.push((function, std::mem::replace(prefix, ECore::Var(function))));
        let value = self.fresh_local();
        let name = format!("$call-argument:{}", value.0);
        let carrier = refinement.base.unwrap();
        self.note_local_ty(value, &carrier);
        env.locals.push((name.clone(), value, Scheme::mono(carrier)));
        // A separate proof binder retains write-log provenance. Storing
        // the refinement on the temporary itself would bypass staleness
        // checks when a later argument writes a location in its predicate.
        let predicate = refinement.pred.subst(&refinement.binder, &ITerm::Var(name.clone().into()));
        if let Some(fact) = self.mk_fact(env, predicate, self.ev_writes.len()) {
            self.bind_facts(env, &[fact], self.ev_writes.len(), argument.span())?;
        }
        binds.push((value, checked));
        Ok((ECore::Var(value), Some(ITerm::Var(name.into()))))
    }

    /// The inferred type of a checked term meets its expected type. A
    /// plain value at a refinement whose container it is enters the
    /// refinement when the predicate holds of it here (refinement types
    /// design: the refined value IS its container, so the coercion is the
    /// same word): decided statically over its index reading, or
    /// entailed by evidence in scope (a `Dec` binder or a branch
    /// condition's fact, D192 staleness included). Every other pair goes
    /// through the argument coercion.
    // A dependent refinement can require the actual lexical facts (for
    // example 0 <= limit for a Nat argument). Keep static literal/range
    // checks first, then use the same checked subsumption as an ascription.
    // Closed numeric carriers avoid trial inference or pending literal metas;
    // a failed scoped check retains the original literal diagnostic.
    fn check_scoped_numeric_literal(
        &mut self, env: &mut Env, term: &ast::Term, expected: &Ty,
        digits: &str, span: ast::Span,
    ) -> Result<ECore> {
        let original = self.lit_check(expected, digits, span);
        if original.is_ok() { return original }
        let Some(RefineView { base: Some(base), .. }) = self.refine_view(&self.uni.zonk(expected)) else {
            return original;
        };
        let base = self.uni.zonk(&base);
        if !matches!(base, Ty::Nat | Ty::Int | Ty::Prim(_)) { return original }
        let Ok(core) = self.lit_check(&base, digits, span) else { return original };
        match self.check_subsume(env, term, &base, expected) {
            Ok(()) => Ok(core),
            Err(_) => original,
        }
    }

    fn check_subsume(
        &mut self,
        env: &mut Env,
        term: &ast::Term,
        inferred: &Ty,
        expected: &Ty,
    ) -> Result<()> {
        let mut inf = self.uni.zonk(inferred);
        let exp = self.uni.zonk(expected);
        // A comparison-refined value (a range of a number, or a relation
        // like `bytes-len out == n`) is its container with a fact (see
        // `unify_arg`): where the lattice table cannot weaken it to the
        // expected refinement, it enters that refinement as its container
        // does, by evidence, and its own predicate is one of the facts (a
        // local's refined predicate, or an application's result
        // refinement read by the discharge). `let m = mask-i64 x 255 in
        // nat-from-i64 m` is an `InRange 0 255 Nat` this way.
        if let (Some(iv), Some(ev)) = (self.refine_view(&inf), self.refine_view(&exp)) {
            if Checker::is_comparison_refinement(&iv) && ev.base.is_some() && !pred_implies(&iv.pred, &ev.pred) {
                if let Some(b) = iv.base {
                    inf = b;
                }
            }
        }
        if !matches!(inf, Ty::Meta(_)) && self.refine_view(&inf).is_none() {
            if let Some(RefineView { base: Some(base), pred, binder }) = self.refine_view(&exp) {
                if self.uni.unify(&inf, &base).is_ok() {
                    let it = self.required_term_index(term, env).map_err(|e| {
                        Error::at(
                            term.span(),
                            format!(
                                "a plain value enters the refinement `{pred}` only when the predicate is proved of it, and this expression has no index reading to state it over ({})",
                                e.message
                            ),
                        )
                    })?;
                    let inst = pred.subst(&binder, &it);
                    return match self.static_obligation(&inst)? {
                        ObligStatus::Checked => Ok(()),
                        ObligStatus::Disproved => Err(Error::at(
                            term.span(),
                            format!(
                                "obligation disproved: `{inst}` is false, so this value cannot enter the refinement (a failed check must not produce evidence)"
                            ),
                        )),
                        ObligStatus::Timeout => Err(Error::at(
                            term.span(),
                            format!(
                                "obligation status: timeout: deciding `{inst}` ran out of reduction steps (none of these outcomes is a proof)"
                            ),
                        )),
                        ObligStatus::Unsupported | ObligStatus::Unresolved => {
                            let ev = self.evidence_in(env);
                            self.discharge_oblig(&ev, &inst, term.span())
                        }
                    };
                }
            }
        }
        self.unify_arg(&inf, &exp).map_err(|m| Error::at(term.span(), m))
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
        let save_record = self.record_quote_bindings;
        let save_holes = std::mem::take(&mut self.hole_tys);
        self.compile_time = false;
        self.record_quote_bindings = true;
        self.hole_tys = hole_tys;
        let r = self.check(&mut tenv, &template, expected);
        self.perf = save_perf;
        self.compile_time = save_ct;
        self.record_quote_bindings = save_record;
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
        let name_holes = holes
            .iter()
            .enumerate()
            .filter(|(_, h)| matches!(h, ast::HoleExpr::Name(_)))
            .map(|(i, _)| i)
            .collect();
        let (hole_cores, _) = self.elaborate_holes(env, &holes)?;
        Ok((
            ECore::DeclsQuote { template, holes: hole_cores, name_holes },
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
            ast::HoleExpr::Name(t) => {
                // A name-position splice (D46): its value names the
                // generated declaration, so a `Name`, `Sym` or name
                // quote reifies to a name at splice time.
                let save = self.compile_time;
                self.compile_time = true;
                let r = self.infer(env, t);
                self.compile_time = save;
                let (core, ty) = r?;
                let ty = self.uni.zonk(&ty);
                match &ty {
                    Ty::Adt(name, args) if (name == "Name" && args.is_empty()) => {}
                    Ty::Sym => {}
                    Ty::Adt(name, args) if name == "Syntax" && args.is_empty() => {}
                    other => {
                        return Err(Error::at(
                            t.span(),
                            crate::diagnostic!(
                                "a name splice must produce a `Name`, `Sym` or `Syntax`, but this one produces {}",
                                other
                            ),
                        ))
                    }
                }
                return Ok((core, ty));
            }
            ast::HoleExpr::Macro { path, args, span } => {
                let (core, ty) = self.macro_call_value(env, path, args, *span)?;
                (core, ty, *span)
            }
        };
        let ty = self.uni.zonk(&ty);
        match &ty {
            Ty::Adt(name, args) if name == "Code" && args.len() == 1 => Ok((core, args[0].clone())),
            Ty::Adt(name, args) if name == "Syntax" && args.is_empty() => {
                let fresh = self.uni.fresh();
                Ok((core, fresh))
            }
            // Splice-time reification (D233, extended by D46): a `Sym`
            // reifies to a symbol constant and a `Name` to a reference
            // to the named binding, both checked where the code lands.
            Ty::Sym => Ok((core, ty)),
            Ty::Adt(name, args) if name == "Name" && args.is_empty() => Ok((core, ty)),
            other => Err(Error::at(
                span,
                crate::diagnostic!(
                    "a splice or macro invocation in a template must produce `Code t`, `Syntax`, `Name` or `Sym`, but this one produces {}",
                    other
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
        let key = (v.module.clone(), v.name.clone());
        let groups = match self.macros.get(&key) {
            Some(info) => args.macro_arg_groups(info.arity),
            None => args.arg_groups(),
        };
        if let Some(info) = self.macros.get(&key) {
            if info.arity != groups.len() {
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
        let (ty, ctx, val_args) = self.instantiate(&v.scheme);
        if !ctx.is_empty() || !val_args.is_empty() {
            return Err(Error::at(
                span,
                "a macro cannot be polymorphic or constrained in this slice",
            ));
        }
        // The invocation applies one argument per token-tree group: the
        // result type is what remains after that many arrows. A `Name`
        // parameter reifies its group's name lexeme at call time (the
        // `sym-to-name` reification, D46).
        let mut arg_cores = Vec::new();
        let mut rest = self.uni.zonk(&ty);
        for g in &groups {
            let dom = match &rest {
                Ty::Fun(d, _, _, b) => {
                    let d = (**d).clone();
                    rest = self.uni.zonk(b);
                    d
                }
                Ty::Pi { dom, cod, .. } => {
                    let d = (**dom).clone();
                    rest = self.uni.zonk(cod);
                    d
                }
                other => {
                    rest = other.clone();
                    Ty::Unit
                }
            };
            if let Ty::Adt(n, args) = &dom {
                if n == "Name" && args.is_empty() {
                    let nm = name_of_group(&g.tokens, span)?;
                    arg_cores.push(app_spine(
                        ECore::Global(crate::core::StageOp::SymToName.global_id(), Vec::new()),
                        vec![ECore::ConstSym(nm)],
                    ));
                    continue;
                }
            }
            let scope = env.syntax_scope(&self.current_module, span);
            let scope = match &g.lexical {
                Some(names) => scope.with_lexical_names(names.clone()),
                None => scope,
            };
            let globals = g.globals.clone().unwrap_or_else(|| self.syntax_global_scope(&self.current_module));
            arg_cores.push(ECore::SyntaxLit(g.tokens.clone(), scope.with_globals(globals)
                .with_declarations(g.declarations.clone().unwrap_or_default())));
        }
        let ty = rest;
        let id = GlobalId(format!("{}.{}", v.module, v.name));
        // A macro is monomorphic (rejected above otherwise).
        Ok((app_spine(ECore::Global(id, Vec::new()), arg_cores), ty))
    }

    /// The `Result` must-use default (attribute annotations
    /// specification section 10; decision D114's rule 7 default):
    /// discarding a `Result`-typed value without `ignore` is a check
    /// error (the return-value discipline of JPL rule 7).
    fn check_must_use_discard(&mut self, discarded: &Ty, span: ast::Span) -> Result<()> {
        let t = self.uni.zonk(discarded);
        if let Ty::Adt(n, _) = &t {
            if n == crate::known::RESULT {
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
                        match env.lookup_name(&p.name) {
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
                            ECore::Demand(Box::new(arg), (*inner).clone()),
                            Ty::Ref(region, inner),
                        ))
                    }
                    Ty::Meta(_) => Err(Error::at(
                        args[0].span(),
                        "cannot determine the memo type of demand's argument; add an annotation",
                    )),
                    other => Err(Error::at(
                        args[0].span(),
                        crate::diagnostic!("demand expects a Lazy cell, got {}", other),
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
                        crate::diagnostic!(
                            "`unrefine` projects a refinement to its container, but this is {} (a refinement value IS its container value)",
                            ty
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
                let (arg_core, copied, from_borrow) = match arg_ty {
                    Ty::Mut(_, inner) => (Self::read_mutable_cell(arg_core, &inner), *inner, true),
                    Ty::Ref(_, inner) => (arg_core, *inner, true),
                    other => (arg_core, other, false),
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
                // Compound propositions (`P && Q`, `P || Q`, `Not P`):
                // the components decide through the same machinery and
                // the tests combine (compound predicates: conjunction
                // and disjunction of decidable propositions are
                // decidable).
                if let Some(core) = self.decide_compound(env, &args[0], span)? {
                    return Ok(core);
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
                    let yes = ECore::Ctor(fn_id("<builtin>", "Yes"), Vec::new());
                    let no = ECore::Ctor(fn_id("<builtin>", "No"), Vec::new());
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
                let yes = ECore::Ctor(fn_id("<builtin>", "Yes"), Vec::new());
                let no = ECore::Ctor(fn_id("<builtin>", "No"), Vec::new());
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
                let test = self.cmp_runtime_test(cmp, &lt, lc, rc, span)?;
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
                    crate::diagnostic!(
                        "`refSet` writes through an exclusive `Mut` borrow, but this is {} (a `Ref` borrow is read-only; use `mutBorrow` for a writable location)",
                        other
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
        self.note_ev_write_loc(region.to_string(), span, v_idx);
        // Lower to a write through the borrow's cell that yields `Unit`.
        Ok((ECore::RefSet(m_local, Box::new(v_core), inner), Ty::Unit))
    }

    /// A compound proposition under `decide` (`P && Q`, `P || Q`,
    /// `Not P`; compound predicates: conjunction and disjunction of
    /// decidable propositions are decidable). Each component decides
    /// through the same `decide` machinery (a synthesized sub-decision
    /// per side) and the components' decisions combine; the elaboration
    /// keeps `decide`'s convention: a static `Yes`/`No` constructor or
    /// `if test then Yes else No`. `None` when the term is not a
    /// compound. A negated proposition keeps the runtime refusal (its
    /// static decision still decides it).
    fn decide_compound(
        &mut self,
        env: &mut Env,
        t: &ast::Term,
        span: ast::Span,
    ) -> Result<Option<(ECore, Ty)>> {
        enum Comp {
            Yes,
            No,
            Test(Box<ECore>),
        }
        fn comp_of(c: ECore) -> Comp {
            match c {
                ECore::Ctor(g, _) if g.0 == "<builtin>.Yes" => Comp::Yes,
                ECore::Ctor(g, _) if g.0 == "<builtin>.No" => Comp::No,
                ECore::If(test, y, n)
                    if matches!(&*y.body, ECore::Ctor(g, _) if g.0 == "<builtin>.Yes")
                        && matches!(&*n.body, ECore::Ctor(g, _) if g.0 == "<builtin>.No") =>
                {
                    Comp::Test(test)
                }
                other => Comp::Test(Box::new(other)),
            }
        }
        fn dec_prop(ty: &Ty) -> Result<PTerm> {
            match ty {
                Ty::Adt(n, args) if n == "Dec" && args.len() == 1 => match &args[0] {
                    Ty::Prp(p) => Ok((**p).clone()),
                    other => Err(Error::plain(crate::diagnostic!(
                        "`decide` produces `Dec p`, not `Dec {}`",
                        other
                    ))),
                },
                other => Err(Error::plain(crate::diagnostic!(
                    "`decide` produces `Dec p`, not `{}`",
                    other
                ))),
            }
        }
        // One component's decision through the same `decide` machinery.
        let sub = |a: &ast::Term| {
            ast::Term::App(
                Box::new(ast::Term::Var(ast::VarPath {
                    quals: Vec::new(),
                    name: ast::Name::new("decide", a.span()),
                })),
                vec![a.clone()],
                a.span(),
            )
        };
        let yes = ECore::Ctor(fn_id("<builtin>", "Yes"), Vec::new());
        let no = ECore::Ctor(fn_id("<builtin>", "No"), Vec::new());
        let dec_ty = |p: PTerm| Ty::Adt("Dec".into(), vec![Ty::Prp(Box::new(p))]);
        match t {
            ast::Term::Con(path) if path.parts() == ["True"] || path.parts() == ["False"] => {
                let value = path.parts() == ["True"];
                Ok(Some((if value { yes } else { no }, dec_ty(PTerm::Lit(value)))))
            }
            // `P && Q` and `P || Q` are the short-circuiting `if`
            // forms the surface reduces them to (primitive types
            // specification section 10): the components decide
            // through the same machinery.
            ast::Term::If { cond, then, els, .. } => {
                let (is_and, la, ra) = match (&**els, &**then) {
                    (ast::Term::Con(p), q) if p.parts() == ["False"] => (true, &**cond, q),
                    (q, ast::Term::Con(p)) if p.parts() == ["True"] => (false, &**cond, q),
                    _ => return Ok(None),
                };
                let (lc, lty) = self.infer(env, &sub(la))?;
                let (rc, rty) = self.infer(env, &sub(ra))?;
                let lp = dec_prop(&self.uni.zonk(&lty))?;
                let rp = dec_prop(&self.uni.zonk(&rty))?;
                let prop = if is_and {
                    PTerm::And(Box::new(lp), Box::new(rp))
                } else {
                    PTerm::Or(Box::new(lp), Box::new(rp))
                };
                let dty = dec_ty(prop);
                let l = comp_of(lc);
                let r = comp_of(rc);
                let mk = |test: ECore| {
                    ECore::If(
                        Box::new(test),
                        CBranch::new(yes.clone()),
                        CBranch::new(no.clone()),
                    )
                };
                let combined = match (&l, &r, is_and) {
                    (Comp::Yes, Comp::Yes, _) => yes,
                    (Comp::No, _, true) | (_, Comp::No, true) => no,
                    (Comp::Yes, _, false) | (_, Comp::Yes, false) => yes,
                    (Comp::No, Comp::No, _) => no,
                    (Comp::Test(a), Comp::Test(b), _) => {
                        let (a, b) = ((**a).clone(), (**b).clone());
                        if is_and {
                            mk(ECore::If(
                                Box::new(a),
                                CBranch::new(b),
                                CBranch::new(ECore::Bool(false)),
                            ))
                        } else {
                            mk(ECore::If(
                                Box::new(a),
                                CBranch::new(ECore::Bool(true)),
                                CBranch::new(b),
                            ))
                        }
                    }
                    (Comp::Test(a), _, _) => mk((**a).clone()),
                    (_, Comp::Test(b), _) => mk((**b).clone()),
                };
                Ok(Some((combined, dty)))
            }
            ast::Term::App(f, args, _) if args.len() == 1 => {
                match &**f {
                    ast::Term::Con(p) if p.parts() == ["Not"] => {}
                    _ => return Ok(None),
                }
                let (cc, cty) = self.infer(env, &sub(&args[0]))?;
                let cp = dec_prop(&self.uni.zonk(&cty))?;
                let dty = dec_ty(PTerm::Not(Box::new(cp)));
                Ok(Some(match comp_of(cc) {
                    Comp::Yes => (no, dty),
                    Comp::No => (yes, dty),
                    Comp::Test(_) => {
                        return Err(Error::at(
                            span,
                            "obligation status: unsupported: `decide` has no decision procedure for a negated proposition",
                        ))
                    }
                }))
            }
            _ => Ok(None),
        }
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

    /// The executable comparison `decide` and a fact-bearing branch
    /// condition share: the operands' own primitive comparison at their
    /// numeric type (the bignum `Nat`/`Int` three-way compare against
    /// zero, or the machine scalar's compare). It is compiled from the
    /// compiler's vocabulary, never from a class method a shadowing
    /// module could lie with, so the fact a branch records is exactly
    /// what ran.
    fn cmp_runtime_test(
        &self,
        cmp: IdxCmp,
        lt: &Ty,
        lc: ECore,
        rc: ECore,
        span: ast::Span,
    ) -> Result<ECore> {
        let cmp_op = match cmp {
            IdxCmp::Eq => CmpOp::Eq,
            IdxCmp::Ne => CmpOp::Ne,
            IdxCmp::Lt => CmpOp::Lt,
            IdxCmp::Le => CmpOp::Le,
            IdxCmp::Gt => CmpOp::Gt,
            IdxCmp::Ge => CmpOp::Ge,
        };
        Ok(match lt {
            Ty::Nat => {
                let imm = ECore::App(Box::new(ECore::Prim(PrimOp::Nat(NatOp::Cmp))), vec![lc, rc]);
                ECore::App(
                    Box::new(ECore::Prim(PrimOp::Cmp { ty: Scalar::I64, op: cmp_op })),
                    vec![imm, ECore::Int(0)],
                )
            }
            Ty::Int => {
                let imm = ECore::App(Box::new(ECore::Prim(PrimOp::Int(IntOp::Cmp))), vec![lc, rc]);
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
                    crate::diagnostic!(
                        "obligation status: unsupported: `decide` compares `Nat`, `Int`, and machine integers, not {}",
                        other
                    ),
                ))
            }
        })
    }

    /// A condition elaborated for its facts (dependent core kernel
    /// design section 8: `if`/`case` introduce facts into their
    /// branches). Returns the `Bool` test and the facts that hold when it
    /// is true and when it is false, each stamped with the write-log
    /// position of its own test. A condition the checker cannot read as a
    /// predicate checks as an ordinary `Bool` and carries no facts.
    fn elab_cond(
        &mut self,
        env: &mut Env,
        cond: &ast::Term,
    ) -> Result<(ECore, Vec<Fact>, Vec<Fact>)> {
        if let ast::Term::If { cond: a, then: t, els: e, cond_attrs, then_attrs, else_attrs, span } =
            cond
        {
            // `p && q` and `p || q` are the short-circuit forms of
            // `case p of True -> q; False -> False` (and the dual): the
            // right operand is the `True` (`False`) row, so it is checked
            // with that row's facts, by the same branch rule as any match.
            let is_and = matches!(&**e, ast::Term::Con(p) if p.parts() == ["False"]);
            let is_or = !is_and && matches!(&**t, ast::Term::Con(p) if p.parts() == ["True"]);
            if is_and || is_or {
                let (tw, fw) = self.if_weights(cond_attrs, then_attrs, else_attrs, *span)?;
                let (ac, ap, an) = self.elab_cond(env, a)?;
                let reading = Reading {
                    kind: ReadingKind::Bool { pos: ap.clone(), neg: an.clone() },
                    end: self.ev_writes.len(),
                };
                let (rhs, konst) = if is_and { (&**t, &**e) } else { (&**e, &**t) };
                let fact_span = a.span();
                let ((rc, rp, rn), rm) = self.isolate(|s| {
                    s.in_branch(env, &reading, &BranchKey::Bool(is_and), fact_span, |s, env| {
                        s.elab_cond(env, rhs)
                    })
                })?;
                let (kc, km) = self.isolate(|s| s.check(env, konst, &Ty::Bool))?;
                let core = if is_and {
                    self.merge_branches(vec![rm, km]);
                    ECore::If(Box::new(ac), CBranch::weighted(tw, rc), CBranch::weighted(fw, kc))
                } else {
                    self.merge_branches(vec![km, rm]);
                    ECore::If(Box::new(ac), CBranch::weighted(tw, kc), CBranch::weighted(fw, rc))
                };
                // `p && q` holds: both hold (a side without a reading
                // still leaves the other's facts). It fails: one of them
                // fails, which needs both readings. `||` is the dual.
                let (pos, neg) = if is_and {
                    ([ap, rp].concat(), fact_or(an, rn))
                } else {
                    (fact_or(ap, rp), [an, rn].concat())
                };
                return Ok((core, pos, neg));
            }
        }
        // `not c`, when `not` is the prelude's Boolean negation: the
        // facts swap. The negation lowers to the primitive itself (the
        // prelude's `not` is exactly `prim-not-bool`).
        if self.is_prelude_not(env, cond) {
            let ast::Term::App(_, args, _) = cond else { unreachable!() };
            let (c, pos, neg) = self.elab_cond(env, &args[0])?;
            return Ok((app_spine(ECore::Prim(PrimOp::NotBool), vec![c]), neg, pos));
        }
        // A comparison through the prelude's `Ord`/`Eq` method checks
        // exactly as the plain condition does; its fact, when it has one,
        // is read off that elaboration.
        let read = match self.cond_comparison(env, cond)? {
            Some((test, None)) => return Ok((test, Vec::new(), Vec::new())),
            Some((test, Some((prop, certificate)))) => Some((test, negate_fact(&prop), prop, vec![certificate])),
            None => self
                .cond_predicate(env, cond)?
                .map(|(test, prop)| (test, PTerm::Not(Box::new(prop.clone())), prop, Vec::new())),
        };
        if let Some((test, neg, pos, certificates)) = read {
            let at = self.ev_writes.len();
            let pos = self.mk_fact(env, pos, at).map(|mut f| { f.comparisons = certificates.clone(); f }).into_iter().collect();
            let neg = self.mk_fact(env, neg, at).map(|mut f| { f.comparisons = certificates; f }).into_iter().collect();
            return Ok((test, pos, neg));
        }
        Ok((self.check(env, cond, &Ty::Bool)?, Vec::new(), Vec::new()))
    }

    /// A condition as a `Bool` match column: its test and the reading
    /// its `True`/`False` branches bind.
    fn cond_reading(&mut self, env: &mut Env, cond: &ast::Term) -> Result<(ECore, Reading)> {
        let (c, pos, neg) = self.elab_cond(env, cond)?;
        Ok((c, Reading { kind: ReadingKind::Bool { pos, neg }, end: self.ev_writes.len() }))
    }

    /// Whether a term is `not c` with `not` the prelude's negation.
    fn is_prelude_not(&self, env: &Env, t: &ast::Term) -> bool {
        let ast::Term::App(f, args, _) = t else { return false };
        let ast::Term::Var(p) = &**f else { return false };
        args.len() == 1
            && (!p.quals.is_empty() || env.lookup_name(&p.name).is_none())
            && self
                .resolve_var(p, p.span())
                .is_ok_and(|v| v.name == "not" && v.module == "Data.Bool" && matches!(v.kind, ValueKind::TopFn))
    }

    /// Whether a `case` scrutinee reads as a predicate (so it elaborates
    /// through `elab_cond`): the short-circuit forms, the prelude's `not`,
    /// a structural predicate, or a comparison through the prelude's
    /// `Ord`/`Eq` method. Anything else infers as usual.
    fn cond_readable(&self, env: &Env, t: &ast::Term) -> bool {
        match t {
            ast::Term::If { then, els, .. } => {
                matches!(&**els, ast::Term::Con(p) if p.parts() == ["False"])
                    || matches!(&**then, ast::Term::Con(p) if p.parts() == ["True"])
            }
            ast::Term::App(f, args, _) => match &**f {
                ast::Term::Con(p) => args.len() == 1 && PredFn::lookup(&p.parts().join(".")).is_some(),
                ast::Term::Var(_) if self.is_prelude_not(env, t) => true,
                ast::Term::Var(op) => args.len() == 2 && self.comparison_op(env, op).is_some(),
                _ => false,
            },
            _ => false,
        }
    }

    /// The comparison a binary operator denotes when it is the prelude's
    /// `Ord`/`Eq` method (a module's own `<` is an arbitrary function
    /// whose truth states nothing).
    fn comparison_op(&self, env: &Env, op: &ast::VarPath) -> Option<IdxCmp> {
        if op.quals.is_empty() && env.lookup_name(&op.name).is_some() {
            return None;
        }
        let value = self.resolve_var(op, op.span()).ok()?;
        // Imports may rename an operator. Facts follow the resolved method,
        // never the spelling the caller chose for it.
        let (cmp, class) = match value.name.as_str() {
            "<" => (IdxCmp::Lt, crate::known::ORD),
            "<=" => (IdxCmp::Le, crate::known::ORD),
            ">" => (IdxCmp::Gt, crate::known::ORD),
            ">=" => (IdxCmp::Ge, crate::known::ORD),
            "==" => (IdxCmp::Eq, crate::known::EQ),
            "/=" => (IdxCmp::Ne, crate::known::EQ),
            _ => return None,
        };
        (value.module == crate::known::split(class).0 && matches!(value.kind, ValueKind::Method { .. })).then_some(cmp)
    }

    /// A comparison condition (`i < n`, `d == 0`, ...) whose operator is
    /// the prelude's `Ord`/`Eq` method. Reading a condition as a predicate
    /// never changes how it typechecks: it elaborates as the ordinary
    /// `Bool` application (the class method at its instance, the same
    /// test and cost as a plain `if`), and the fact is extracted only
    /// when that elaboration settled the compared type at a total order
    /// (`Nat`, `Int`, machine integers, `Char`; never floats, whose
    /// comparisons are not trichotomous under NaN) and both operands are
    /// terms a fact may name. `None` when the term is not such a
    /// comparison; `Some((test, None))` when it is one without a fact.
    fn cond_comparison(
        &mut self,
        env: &mut Env,
        cond: &ast::Term,
    ) -> Result<Option<(ECore, Option<(PTerm, ComparisonCertificate)>)>> {
        let ast::Term::App(f, args, _) = cond else {
            return Ok(None);
        };
        let ast::Term::Var(op) = &**f else {
            return Ok(None);
        };
        if args.len() != 2 {
            return Ok(None);
        }
        let Some(cmp) = self.comparison_op(env, op) else {
            return Ok(None);
        };
        let test = self.check(env, cond, &Ty::Bool)?;
        Ok(Some((test.clone(), self.comparison_fact(env, &test, cmp, &args[0], &args[1]))))
    }

    /// The fact an elaborated comparison states, when it states one: the
    /// method's class argument (the compared type, solved by the ordinary
    /// elaboration) is a total order, and both operands are terms a fact
    /// may name.
    fn comparison_fact(
        &self,
        env: &Env,
        test: &ECore,
        declared_cmp: IdxCmp,
        lhs: &ast::Term,
        rhs: &ast::Term,
    ) -> Option<(PTerm, ComparisonCertificate)> {
        let mut head = test;
        while let ECore::App(f, _) = head {
            head = f;
        }
        let ECore::DictProj(dict, slot, _, _) = head else { return None };
        let ECore::Hole(want) = &**dict else { return None };
        let w = self.wants.iter().find(|w| w.id == *want)?;
        let compared = self.uni.zonk(w.class.args.first()?);
        if !fact_numeric(&compared) {
            return None;
        }
        // An Ord/Eq declaration describes a callable, not its equation.
        // Resolve this exact want (including lexical dictionaries), then
        // read only the selected implementation's checked computation.
        let mut certificate = self.checked_comparison_method(w, *slot, &compared, declared_cmp)?;
        let cmp = certificate.comparison;
        if !self.fact_term_ok(env, lhs) || !self.fact_term_ok(env, rhs) {
            return None;
        }
        // Index arithmetic is mathematical; a machine integer's wraps.
        if matches!(compared, Ty::Prim(_)) && (has_arith(lhs) || has_arith(rhs)) {
            return None;
        }
        let ((li, ri), arithmetic) = self.with_runtime_arithmetic(|| Ok((self.idx_of_term(lhs, env)?, self.idx_of_term(rhs, env)?))).ok()?;
        certificate.arithmetic = arithmetic;
        let prop = PTerm::Cmp(cmp, Box::new(li), Box::new(ri));
        match self.static_obligation(&prop) {
            Err(_) | Ok(ObligStatus::Timeout | ObligStatus::Unsupported) => None,
            Ok(_) => Some((prop, certificate)),
        }
    }

    /// A structural predicate written as a condition (`if NonZero d
    /// then ...`): it decides through `decide`'s own machinery and the
    /// branch keeps only the test. Its static decision folds the test to
    /// a constant exactly as `decide` folds it.
    fn cond_predicate(
        &mut self,
        env: &mut Env,
        cond: &ast::Term,
    ) -> Result<Option<(ECore, PTerm)>> {
        let ast::Term::App(f, args, span) = cond else {
            return Ok(None);
        };
        let head = match &**f {
            ast::Term::Con(p) => p.parts().join("."),
            _ => return Ok(None),
        };
        if args.len() != 1 || PredFn::lookup(&head).is_none() || !self.fact_term_ok(env, &args[0]) {
            return Ok(None);
        }
        // The condition is a test, not a counted `decide` (the
        // `#[assert-min]` runtime-check count reads `decide` sites).
        let counted = self.cur_runtime_decides.len();
        let (core, dty) = self.infer_builtin_app(env, "decide", std::slice::from_ref(cond), *span)?;
        self.cur_runtime_decides.truncate(counted);
        let prop = match self.uni.zonk(&dty) {
            Ty::Adt(n, a) if n == "Dec" && a.len() == 1 => match &a[0] {
                Ty::Prp(p) => (**p).clone(),
                _ => return Ok(None),
            },
            _ => return Ok(None),
        };
        let test = match core {
            ECore::Ctor(g, _) if g.0 == "<builtin>.Yes" => ECore::Bool(true),
            ECore::Ctor(g, _) if g.0 == "<builtin>.No" => ECore::Bool(false),
            ECore::If(test, _, _) => *test,
            _ => return Ok(None),
        };
        Ok(Some((test, prop)))
    }

    /// Whether a term may be the subject of a fact: its value is fixed
    /// for as long as the names it reads are, so two occurrences of it
    /// denote the same value. Numerals, locals, borrow reads (their
    /// writes are tracked), arithmetic, `bytes-len` (a buffer's size
    /// never changes), and a
    /// global function applied to all of its arguments whose arrows
    /// perform nothing and take no borrow, pointer or byte buffer (a
    /// buffer's contents change under `bytes-set` with no tracked
    /// footprint, so a fact about them could go stale unseen).
    fn fact_term_ok(&self, env: &Env, t: &ast::Term) -> bool {
        match t {
            ast::Term::Int(..) | ast::Term::Char(..) => true,
            ast::Term::Neg(inner, _) => matches!(&**inner, ast::Term::Int(..)),
            ast::Term::Ann(e, _, _) => self.fact_term_ok(env, e),
            ast::Term::Var(p) if p.quals.is_empty() && env.lookup_name(&p.name).is_some() => true,
            ast::Term::Var(p) => self.resolve_var(p, p.span()).is_ok_and(|v| {
                matches!(v.kind, ValueKind::TopFn)
                    && !matches!(v.scheme.ty, Ty::Fun(..) | Ty::Pi { .. })
            }),
            ast::Term::App(f, args, _) => {
                let ast::Term::Var(p) = &**f else {
                    return false;
                };
                if p.quals.is_empty() && env.lookup_name(&p.name).is_some() {
                    return false;
                }
                if p.quals.is_empty() {
                    match (p.name.text.as_str(), args.len()) {
                        ("copyOut" | "borrow" | "unrefine", 1) => {
                            return matches!(&args[0], ast::Term::Var(v)
                                if v.quals.is_empty() && env.lookup_name(&v.name).is_some());
                        }
                        ("+" | "-" | "*", 2) => {
                            return self.fact_term_ok(env, &args[0]) && self.fact_term_ok(env, &args[1]);
                        }
                        // The index reading takes these names as the
                        // baked measures whatever they resolve to here, so
                        // a fact over them could state something other
                        // than what ran.
                        ("length" | "toNat", _) => return false,
                        _ => {}
                    }
                }
                let Ok(v) = self.resolve_var(p, p.span()) else {
                    return false;
                };
                if !args.iter().all(|a| self.fact_term_ok(env, a)) {
                    return false;
                }
                if matches!(v.kind, ValueKind::Prim(PrimOp::Bytes(BytesOp::Len))) {
                    return args.len() == 1;
                }
                // A pure arithmetic intrinsic applied to all of its
                // arguments (`u32-to-i64 c`, `bit-and-i64 x 255`,
                // `nat-rem n 10`) is a value of its arguments alone.
                if let ValueKind::Prim(op) = v.kind {
                    return op.is_pure_arith() && args.len() == op.arity();
                }
                if !matches!(v.kind, ValueKind::TopFn) {
                    return false;
                }
                let mut ty = v.scheme.ty.clone();
                for _ in args {
                    let (dom, eff, cod) = match ty {
                        Ty::Fun(dom, _, eff, cod) => (*dom, eff, *cod),
                        Ty::Pi { dom, eff, cod, .. } => (*dom, eff, *cod),
                        _ => return false,
                    };
                    let pure = eff.labels.is_empty() && matches!(eff.tail, RowTail::Closed);
                    let mut regions = Vec::new();
                    borrow_regions(&dom, &mut regions);
                    if !pure || !regions.is_empty() || matches!(dom, Ty::Bytes | Ty::Ptr { .. }) {
                        return false;
                    }
                    ty = cod;
                }
                true
            }
            _ => false,
        }
    }

    /// Whether a comparison operand is known, before elaboration, to be a
    /// value of a numeric type `decide` compares (`Nat`, `Int`, a machine
    /// integer, `Char`; never a float). A conservative look at the
    /// operand's shape that performs no elaboration: a local's recorded
    /// type, a borrow read, arithmetic over such operands, or a global
    /// function applied to all of its arguments whose result type is a
    /// numeric type outright (`list-length :: List a -> Nat` qualifies:
    /// its type parameter never reaches the result).
    fn cond_operand_numeric(&self, env: &Env, t: &ast::Term) -> bool {
        match t {
            ast::Term::Char(..) => true,
            ast::Term::Ann(e, _, _) => self.cond_operand_numeric(env, e),
            ast::Term::Var(p) if p.quals.is_empty() => match env.lookup_name(&p.name) {
                Some((_, s)) => s.params.is_empty() && fact_numeric(&self.uni.zonk(&s.ty)),
                None => false,
            },
            ast::Term::App(f, args, _) => {
                let ast::Term::Var(p) = &**f else {
                    return false;
                };
                if !p.quals.is_empty() {
                    return false;
                }
                match (p.name.text.as_str(), args.len()) {
                    ("+" | "-" | "*", 2) if env.lookup_name(&p.name).is_none() => {
                        self.cond_operand_numeric(env, &args[0])
                            || self.cond_operand_numeric(env, &args[1])
                    }
                    ("copyOut", 1) if env.lookup("copyOut").is_none() => match &args[0] {
                        ast::Term::Var(m) if m.quals.is_empty() => {
                            match env.lookup_name(&m.name).map(|(_, s)| self.uni.zonk(&s.ty)) {
                                Some(Ty::Mut(_, inner) | Ty::Ref(_, inner)) => fact_numeric(&inner),
                                _ => false,
                            }
                        }
                        _ => false,
                    },
                    _ if env.lookup_name(&p.name).is_none() => {
                        let Ok(v) = self.resolve_var(p, p.span()) else {
                            return false;
                        };
                        if !matches!(v.kind, ValueKind::TopFn) {
                            return false;
                        }
                        let mut ty = v.scheme.ty.clone();
                        for _ in args {
                            ty = match ty {
                                Ty::Fun(_, _, _, cod) => *cod,
                                Ty::Pi { cod, .. } => *cod,
                                _ => return false,
                            };
                        }
                        fact_numeric(&ty)
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    /// The locals an index term's names denote here, or `None` when a
    /// name is not a local (a fact over it could not be checked against
    /// a later scope).
    fn term_names(&self, env: &Env, it: &ITerm) -> Option<Vec<(String, LocalId)>> {
        let mut values = Vec::new(); it.local_values(&mut values);
        if !self.values_in_scope(env, &values) { return None; }
        let mut fv = Vec::new();
        it.free_vars(&mut fv);
        fv.into_iter().map(|v| env.lookup(&v).map(|(l, _)| (v, l))).collect()
    }

    /// A fact over `prop`, tested at write-log position `at`, with the
    /// locals its names denote here.
    fn mk_fact(&self, env: &Env, prop: PTerm, at: usize) -> Option<Fact> {
        let mut values = Vec::new(); prop.local_values(&mut values);
        if !self.values_in_scope(env, &values) { return None; }
        let mut fv = Vec::new();
        prop.free_vars(&mut fv);
        let names = fv
            .into_iter()
            .map(|v| env.lookup(&v).map(|(l, _)| (v, l)))
            .collect::<Option<Vec<_>>>()?;
        Some(Fact { prop, at, names, comparisons: Vec::new() })
    }

    /// A `case` scrutinee elaborated for what its value states (the
    /// facts its match's branches bind): a predicate (`case d == 0 of`),
    /// a numeric or list-shaped value a fact may name (`case d of 0 ->
    /// ...`), or the three-way `comparing k x y` at a numeric key. A
    /// scrutinee with no reading infers as usual.
    fn infer_scrutinee(
        &mut self,
        env: &mut Env,
        scrut: &ast::Term,
    ) -> Result<(ECore, Ty, Option<Reading>)> {
        if self.cond_readable(env, scrut) {
            let (c, pos, neg) = self.elab_cond(env, scrut)?;
            let reading = (!pos.is_empty() || !neg.is_empty()).then(|| Reading {
                kind: ReadingKind::Bool { pos, neg },
                end: self.ev_writes.len(),
            });
            return Ok((c, Ty::Bool, reading));
        }
        let (c, ty) = self.infer(env, scrut)?;
        let ty = self.uni.zonk(&ty);
        let end = self.ev_writes.len();
        let valued = fact_numeric(&ty) || matches!(&ty, Ty::Adt(n, _) if self.list_shaped(n));
        let wraps = matches!(ty, Ty::Prim(_)) && has_arith(scrut);
        let reading = if valued && !wraps && self.fact_term_ok(env, scrut) {
            self.idx_of_term(scrut, env).ok().and_then(|it| {
                let names = self.term_names(env, &it)?;
                Some(Reading { kind: ReadingKind::Value(it, names), end })
            })
        } else if matches!(&ty, Ty::Adt(n, _) if n == crate::known::ORDERING) {
            self.ordering_reading(env, scrut, &c, end)
        } else {
            None
        };
        Ok((c, ty, reading))
    }

    /// `comparing k x y` (the prelude's three-way comparison; `Ord`
    /// itself has only the four boolean methods) at a numeric key type:
    /// its `LT`/`EQ`/`GT` rows state `k x < k y`, `k x == k y`, `k x >
    /// k y`. The key is the identity lambda or a function a fact may
    /// name (a local, or a pure global) with a numeric result.
    fn ordering_reading(&self, env: &Env, scrut: &ast::Term, core: &ECore, end: usize) -> Option<Reading> {
        let ast::Term::App(_, args, span) = scrut else { return None };
        let [k, x, y] = args.as_slice() else { return None };
        let ECore::App(helper, arguments) = core else { return None };
        let ECore::Global(id, _) = &**helper else { return None };
        if id.0 != crate::known::COMPARING { return None; }
        let [ECore::Hole(want), key, _, _] = arguments.as_slice() else { return None };
        let want = self.wants.iter().find(|entry| entry.id == *want)?;
        if want.class.class != crate::known::ORD { return None; }
        let [carrier] = want.class.args.as_slice() else { return None };
        let carrier = self.uni.zonk(carrier);
        if !fact_numeric(&carrier) { return None; }
        let (less_slot, greater_slot) = self.checked_ordering_helper(id)?;
        let mut less = self.checked_comparison_method(want, less_slot, &carrier, IdxCmp::Lt)?;
        let mut greater = self.checked_comparison_method(want, greater_slot, &carrier, IdxCmp::Gt)?;
        if less.comparison != IdxCmp::Lt || greater.comparison != IdxCmp::Gt { return None; }
        if !self.fact_term_ok(env, x) || !self.fact_term_ok(env, y) {
            return None;
        }
        let keyed = |a: &ast::Term| -> Option<(ast::Term, bool)> {
            match k {
                ast::Term::Lam(..) => {
                    let ECore::Lam { params, body } = key else { return None };
                    let [parameter] = params.as_slice() else { return None };
                    if !matches!(&**body, ECore::Var(id) if id == parameter) { return None; }
                    Some((a.clone(), self.cond_operand_numeric(env, a)))
                }
                ast::Term::Var(kp) if kp.quals.is_empty() && env.lookup_name(&kp.name).is_some() => {
                    let app = ast::Term::App(Box::new(k.clone()), vec![a.clone()], *span);
                    let (_, s) = env.lookup_name(&kp.name)?;
                    let numeric = s.params.is_empty()
                        && matches!(self.uni.zonk(&s.ty), Ty::Fun(_, _, eff, cod)
                            if eff.labels.is_empty()
                                && matches!(eff.tail, RowTail::Closed)
                                && fact_numeric(&cod));
                    Some((app, numeric))
                }
                ast::Term::Var(_) => {
                    let app = ast::Term::App(Box::new(k.clone()), vec![a.clone()], *span);
                    let ok = self.fact_term_ok(env, &app);
                    Some((app.clone(), ok && self.cond_operand_numeric(env, &app)))
                }
                _ => None,
            }
        };
        let (kx, nx) = keyed(x)?;
        let (ky, ny) = keyed(y)?;
        // One side suffices: both keys have the key's one result type
        // (a literal operand adapts to the other side).
        if !(nx || ny) {
            return None;
        }
        if matches!(carrier, Ty::Prim(_)) && (has_arith(&kx) || has_arith(&ky)) { return None; }
        let ((a, b), arithmetic) = self.with_runtime_arithmetic(|| {
            Ok((self.idx_of_term(&kx, env)?.normalize(), self.idx_of_term(&ky, env)?.normalize()))
        }).ok()?;
        less.arithmetic = arithmetic.clone();
        greater.arithmetic = arithmetic;
        let mut names = self.term_names(env, &a)?;
        names.extend(self.term_names(env, &b)?);
        Some(Reading { kind: ReadingKind::Order(a, b, names, vec![less, greater]), end })
    }

    /// The facts one branch of a match establishes about its column:
    /// the one rule behind `if`, `case`, guards and `&&`/`||`. A branch's
    /// fact is what its test proves about the column's value on every
    /// path into it. The decision tree's branches are exact, so a default
    /// branch knows the value is none of the literals, and an earlier
    /// guarded row never removes values from a later branch (its failing
    /// guard falls through inside its own branch).
    fn branch_facts(reading: &Reading, key: &BranchKey) -> Vec<Fact> {
        let at = reading.end;
        let mk = |prop: PTerm, names: &[(String, LocalId)]| Fact { prop, at, names: names.to_vec(), comparisons: Vec::new() };
        match (&reading.kind, key) {
            (ReadingKind::Bool { pos, .. }, BranchKey::Bool(true)) => pos.clone(),
            (ReadingKind::Bool { neg, .. }, BranchKey::Bool(false)) => neg.clone(),
            (ReadingKind::Value(it, names), key) => {
                value_facts(it, key).into_iter().map(|p| mk(p, names)).collect()
            }
            (ReadingKind::Order(a, b, names, comparisons), BranchKey::Ctor { name, .. }) => {
                let op = match name.as_str() {
                    "LT" => IdxCmp::Lt,
                    "EQ" => IdxCmp::Eq,
                    "GT" => IdxCmp::Gt,
                    _ => return Vec::new(),
                };
                let mut fact = mk(PTerm::Cmp(op, Box::new(a.clone()), Box::new(b.clone())), names);
                fact.comparisons.clone_from(comparisons);
                vec![fact]
            }
            _ => Vec::new(),
        }
    }

    /// Bind one branch's facts into each of its rows: the column's
    /// reading (the scrutinee's own statement), and the same statement
    /// about every name the row binds to the column (`x -> nat-div n x`
    /// after a `0` row knows `x /= 0`, as does a function equation's
    /// parameter after a literal equation).
    fn bind_column_facts(
        &mut self,
        rows: &mut [Row],
        col0: &ECore,
        key: &BranchKey,
        span: ast::Span,
    ) -> Result<()> {
        let ECore::Var(l) = col0 else { return Ok(()) };
        let reading = self.col_readings.get(l).cloned();
        let now = self.ev_writes.len();
        for r in rows.iter_mut() {
            if let Some(rd) = &reading {
                let facts = Self::branch_facts(rd, key);
                self.bind_facts(&mut r.env, &facts, rd.end, span)?;
            }
            let bound: Vec<String> = r
                .env
                .locals
                .iter()
                .filter(|(n, x, _)| x == l && r.env.lookup(n).is_some_and(|(y, _)| y == *l))
                .map(|(n, _, _)| n.clone())
                .collect();
            for n in bound {
                let facts: Vec<Fact> = value_facts(&ITerm::Var(n.clone().into()), key)
                    .into_iter()
                    .map(|prop| Fact { prop, at: now, names: vec![(n.clone(), *l)], comparisons: Vec::new() })
                    .collect();
                self.bind_facts(&mut r.env, &facts, now, span)?;
            }
        }
        Ok(())
    }

    /// Run `f` in one branch of `reading`'s match, with the branch's
    /// facts in scope. Without facts the scope is the caller's own.
    fn in_branch<T>(
        &mut self,
        env: &mut Env,
        reading: &Reading,
        key: &BranchKey,
        span: ast::Span,
        f: impl FnOnce(&mut Self, &mut Env) -> Result<T>,
    ) -> Result<T> {
        let facts = Self::branch_facts(reading, key);
        if facts.is_empty() {
            return f(self, env);
        }
        let mut env2 = env.clone();
        self.bind_facts(&mut env2, &facts, reading.end, span)?;
        f(self, &mut env2)
    }

    /// Bind facts as hidden evidence binders, one per conjunct, so each
    /// atom is judged over its own names (an obligation about `d` is
    /// discharged by `d /= 0` even when the condition also spoke about
    /// `x`). A fact whose names no longer denote the values they did at
    /// its test (a row rebinding the name) is not bound. The binder is
    /// derived here (writes in sibling branches elaborated earlier are
    /// not on its path); writes between its test and the end of its
    /// condition (`end`) still count against it.
    fn bind_facts(&mut self, env: &mut Env, facts: &[Fact], end: usize, span: ast::Span) -> Result<()> {
        for f in facts {
            let mut values = Vec::new(); f.prop.local_values(&mut values);
            if !self.values_in_scope(env, &values) { continue; }
            let same = f.names.iter().all(|(n, l)| env.lookup(n).is_some_and(|(x, _)| x == *l));
            if !same {
                continue;
            }
            for atom in pred_conjuncts(&f.prop) {
                let l = self.fresh_local();
                if !f.comparisons.is_empty() { self.comparison_facts.insert(l, f.comparisons.clone()); }
                self.bind_local(
                    env,
                    COND_FACT,
                    l,
                    Scheme::mono(Ty::Prp(Box::new(atom))),
                    Some(Mult::Zero),
                    span,
                )?;
                if f.at < end {
                    self.ev_pre.insert(l, (f.at, end));
                }
            }
        }
        Ok(())
    }

    /// Bind a definition's own obligations (`param_facts`) into one
    /// row's scope: each positional binder `$a{i}` becomes the name the
    /// row binds to the i-th parameter's local. An obligation over a
    /// parameter the row's pattern does not name (a constructor
    /// pattern, a wildcard) is not bound in that row.
    fn bind_param_facts(
        &mut self,
        env: &mut Env,
        facts: &[(PTerm, Vec<LocalId>)],
        span: ast::Span,
    ) -> Result<()> {
        let now = self.ev_writes.len();
        let mut out: Vec<Fact> = Vec::new();
        'facts: for (p, locals) in facts {
            let mut fv = Vec::new();
            p.free_vars(&mut fv);
            let mut prop = p.clone();
            let mut names: Vec<(String, LocalId)> = Vec::new();
            for v in &fv {
                let Some(l) = v
                    .strip_prefix("$a")
                    .and_then(|i| i.parse::<usize>().ok())
                    .and_then(|i| locals.get(i))
                else {
                    continue 'facts;
                };
                let Some(n) = env
                    .locals
                    .iter()
                    .rev()
                    .map(|(n, _, _)| n)
                    .find(|n| env.lookup(n).is_some_and(|(x, _)| x == *l))
                    .cloned()
                else {
                    continue 'facts;
                };
                prop = prop.subst(v, &ITerm::Var(n.clone().into()));
                names.push((n, *l));
            }
            out.push(Fact { prop, at: now, names, comparisons: Vec::new() });
        }
        self.bind_facts(env, &out, now, span)
    }

    /// A definition's declared result as one row's body checks against
    /// it: a result refinement naming parameter `i` (`refined (i <=
    /// bytes-len out) Bytes`, stored with the positional `$a{i}`) states
    /// its predicate over the name the row's pattern gives that
    /// parameter. A parameter the row leaves unnamed (a constructor
    /// pattern, a wildcard) stays positional, which no fact mentions, so
    /// the promise is refused there rather than weakened.
    fn row_result(&self, env: &Env, expected: &Ty) -> Ty {
        let exp = self.uni.zonk(expected);
        if self.result_params.is_empty() || !Checker::has_positional_result(&exp) {
            return exp;
        }
        let Ty::Adt(n, targs) = &exp else { return exp };
        let [Ty::Prp(p), base] = targs.as_slice() else { return exp };
        let mut pred = p.subst(&crate::types::pred_binder(p), &ITerm::Var("$it".to_string().into()));
        for (i, l) in self.result_params.iter().enumerate() {
            let name = env
                .locals
                .iter()
                .rev()
                .map(|(n, _, _)| n)
                .find(|n| env.lookup(n).is_some_and(|(x, _)| x == *l));
            if let Some(name) = name {
                pred = pred.subst(&format!("$a{i}"), &ITerm::Var(name.clone().into()));
            }
        }
        Ty::Adt(n.clone(), vec![Ty::Prp(Box::new(pred)), base.clone()])
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
        if path.segments.last().is_some_and(|name| name.resolved_binding().is_some()) {
            return self.resolve_con(path, span);
        }
        if let Some(scope) = path.segments.last().and_then(|name| name.scoped_reference()) {
            let candidates = self.scoped_constructors(scope);
            if let Some(Ty::Adt(identity, _)) = expected {
                if let Ok(info) = self.lookup_known_type(identity) {
                    if let Some(value) = candidates.iter().find(|value| value.module == info.module
                        && info.ctors.iter().any(|(name, _)| *name == value.name))
                    {
                        self.record_quote_constructor(path, value);
                        return Ok(value.clone());
                    }
                }
            }
            let value = self.resolve_con(path, span)?;
            self.record_quote_constructor(path, &value);
            return Ok(value);
        }
        if let Some(Ty::Adt(tname, _)) = expected {
            if let Ok(info) = self.lookup_known_type(tname) {
                let parts = path.parts();
                let last = parts[parts.len() - 1];
                let qualified_ok = parts.len() == 1 || parts[0] == info.module;
                if qualified_ok && info.ctors.iter().any(|(n, _)| n == last) {
                    if let Some(v) = self.qualified.get(&(info.module.clone(), last.to_string())) {
                        if matches!(v.kind, ValueKind::Ctor { .. }) {
                            // Surface gate (D143/D298/D308): a
                            // module-private constructor never crosses
                            // an import, in term or pattern position.
                            if v.module != self.current_module
                                && self.surface_def(&v.module, last).is_none()
                            {
                                return Err(Error::at(
                                    span,
                                    format!(
                                        "constructor {} is not exported by {}",
                                        last, v.module
                                    ),
                                ));
                            }
                            let mut v = v.clone();
                            v.name = last.to_string();
                            if self.record_quote_bindings {
                                path.segments.last().unwrap().record_binding(ast::RefKind::Value, &v.module, &v.name);
                            }
                            return Ok(v);
                        }
                    }
                }
            }
        }
        let value = self.resolve_con(path, span)?;
        self.record_quote_constructor(path, &value);
        Ok(value)
    }

    fn record_quote_constructor(&self, path: &ast::ConPath, value: &ValueInfo) {
        self.record_editor_reference(path.span(), ast::RefKind::Value, &value.module, &value.name);
        if self.record_quote_bindings {
            if let Some(name) = path.segments.last() {
                name.record_binding(ast::RefKind::Value, &value.module, &value.name);
            }
        }
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
                                crate::diagnostic!("cannot apply a value of type {}", other),
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
                                crate::diagnostic!("cannot apply a value of type {}", other),
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
        let (ty, _ctx, _, ptys) = self.instantiate_at(&v.scheme);
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
                            crate::diagnostic!(
                                "the refinement's witness is a proposition, not {}",
                                other
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
            } else if let Ty::Prp(prop) = self.uni.zonk(dom) {
                self.check_constructor_proof(env, a, &prop)?
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
            app_spine(ECore::Ctor(fn_id(&v.module, &v.name), ptys), kept),
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
        Ok(ITerm::Fn(value_index_id(v).into()))
    }

    /// Bridge an already checked source term into the shared type-position
    /// index converter. Names and annotations remain structural; there is no
    /// printed-source round trip or second spelling-based resolution scope.
    fn checked_term_index(
        &self,
        term: &ast::Term,
        env: &Env,
        expected: Option<&Ty>,
    ) -> Result<ITerm> {
        let input = self.term_index_input(term, env)?;
        self.with_type_values(env, |checker| checker.conv_idx_expected(&input, &[], expected))
    }

    fn term_index_input(&self, term: &ast::Term, env: &Env) -> Result<ast::Type> {
        let span = term.span();
        Ok(match term {
            ast::Term::Var(path) if path.quals.is_empty() => ast::Type::Var(path.name.clone(), span),
            ast::Term::Var(path) => {
                let mut segments = path.quals.clone();
                segments.push(path.name.clone());
                ast::Type::Con(ast::ConPath { segments }, span)
            }
            ast::Term::Con(path) => ast::Type::Con(path.clone(), span),
            ast::Term::Int(digits, _) => ast::Type::Num(digits.clone(), span),
            ast::Term::Char(character, _) => ast::Type::IndexOperand(
                Box::new(ITerm::Num(char_literal_word(character, span)?.to_string().into())),
                Box::new(Ty::Prim(Scalar::Char)), span,
            ),
            ast::Term::Neg(inner, _) => match neg_fold(inner) {
                Some(digits) => ast::Type::Num(digits, span),
                None => return Err(Error::at(span, "this expression cannot appear in a dependent type")),
            },
            ast::Term::Ann(body, annotation, _) => {
                ast::Type::IndexAnnotated(Box::new(self.term_index_input(body, env)?), Box::new(annotation.clone()), span)
            }
            ast::Term::App(head, args, _) => {
                if let ast::Term::Var(path) = &**head {
                    if path.quals.is_empty() && args.len() == 2 {
                        let operation = match path.name.source_spelling() {
                            "+" => Some(IdxOp::Add), "-" => Some(IdxOp::Sub), "*" => Some(IdxOp::Mul), _ => None,
                        };
                        if let Some(operation) = operation {
                            let (index, carrier) = self.runtime_arithmetic_index(term, args, env, operation, span)?;
                            return Ok(ast::Type::IndexOperand(Box::new(index), Box::new(carrier), span));
                        }
                    }
                }
                // Borrow observations and refinement erasure retain the
                // existing mathematical index meaning of the observed value.
                if matches!(&**head, ast::Term::Var(path) if path.quals.is_empty()
                    && matches!(path.name.text.as_str(), "copyOut" | "borrow" | "mutBorrow" | "unrefine"))
                    && args.len() == 1 {
                    return self.term_index_input(&args[0], env);
                }
                let mut input = self.term_index_input(head, env)?;
                for arg in args {
                    input = ast::Type::App(Box::new(input), Box::new(self.term_index_input(arg, env)?), span);
                }
                input
            }
            ast::Term::Lam(patterns, body, _) => {
                let mut input = self.term_index_input(body, env)?;
                for pattern in patterns.iter().rev() {
                    let ast::Pat::Var(name, _) = pattern else {
                        return Err(Error::at(pattern.span(), "an index-level lambda binds plain names"));
                    };
                    input = ast::Type::Lam(name.clone(), Box::new(input), span);
                }
                input
            }
            other => return Err(Error::at(other.span(), "this expression cannot appear in a dependent type")),
        })
    }

    /// A term in index position (a value a dependent type mentions):
    /// variables, numerals, mathematical arithmetic, and applications of
    /// named total definitions (the surface shares term syntax between
    /// kinds, types and values).
    /// Source-shaped diagnostic only. This reading is never used as a type,
    /// proposition, proof or constraint; runtime arithmetic may not establish
    /// the mathematical equation displayed in a failed obligation.
    fn diagnostic_term_index(&self, term: &ast::Term, env: &Env) -> Option<ITerm> {
        match term {
            ast::Term::Ann(value, _, _) => self.diagnostic_term_index(value, env),
            ast::Term::App(head, args, _) if args.len() == 2 => {
                if let ast::Term::Var(path) = &**head {
                    let operation = match path.name.source_spelling() { "+" => Some(IdxOp::Add), "-" => Some(IdxOp::Sub), "*" => Some(IdxOp::Mul), _ => None };
                    if let Some(operation) = operation {
                        return Some(ITerm::Bin(operation, Box::new(self.diagnostic_term_index(&args[0], env)?), Box::new(self.diagnostic_term_index(&args[1], env)?)));
                    }
                }
                self.idx_of_term(term, env).ok()
            }
            _ => self.idx_of_term(term, env).ok(),
        }
    }

    fn idx_of_term(&self, t: &ast::Term, env: &Env) -> Result<ITerm> {
        Ok(match t {
            ast::Term::Int(d, _) => ITerm::Num(d.clone().into()),
            // A char literal in index position is its code point (the
            // `Char` scalar's word).
            ast::Term::Char(s, span) => {
                ITerm::Num(char_literal_word(s, *span)?.to_string().into())
            }
            ast::Term::Ann(..) => return self.checked_term_index(t, env, None),
            ast::Term::Neg(inner, span) => match neg_fold(inner) {
                Some(v) => ITerm::Num(v.into()),
                None => {
                    return Err(Error::at(
                        *span,
                        "this expression cannot appear in a dependent type",
                    ))
                }
            },
            ast::Term::Var(p) => {
                if p.quals.is_empty() {
                    if let Some((local, _)) = env.lookup_name(&p.name) {
                        let original = self.value_aliases.get(&local).copied().unwrap_or(local);
                        if p.name.local_binding().is_some() || self.quantified_value_locals.contains(&original) {
                            self.syntax_index_value(env, &p.name)?
                        } else {
                            ITerm::Var(p.name.text.clone().into())
                        }
                    } else if p.name.local_binding().is_none() && matches!(p.name.text.as_str(), "toNat" | "length") {
                        ITerm::Fn(format!("<builtin>.{}", p.name.text).into())
                    } else {
                        let v = self.resolve_var(p, p.span())?;
                        self.idx_ref(&v, p.span())?
                    }
                } else {
                    let v = self.resolve_var(p, p.span())?;
                    self.idx_ref(&v, p.span())?
                }
            }
            ast::Term::Lam(..) => return self.checked_term_index(t, env, None),
            ast::Term::Con(p) => {
                let v = self.resolve_con(p, p.span())?;
                self.idx_ref(&v, p.span())?
            }
            ast::Term::App(f, args, span) => {
                // Runtime constructor applications carry their actual
                // checked type instantiation, just as type-level index
                // applications do. A bare constructor identity would
                // invent a different subject from Cons @(Char) here.
                if matches!(&**f, ast::Term::Con(_)) {
                    return self.checked_term_index(t, env, None);
                }
                // A borrow read (`copyOut m`) in index position is a live
                // reading of the borrowed location: a proposition over it
                // is a refinement guarantee about that location, valid
                // until the location is written (evidence invalidation
                // over mutable state). `unrefine e` in index position is
                // the value itself (the refinement's runtime
                // representation IS its container).
                if let ast::Term::Var(p) = &**f {
                    if p.quals.is_empty()
                        && matches!(p.name.text.as_str(), "copyOut" | "borrow" | "mutBorrow" | "unrefine")
                        && args.len() == 1
                    {
                        return self.idx_of_term(&args[0], env);
                    }
                }
                // This is a runtime expression crossing into index position.
                // Its selected implementation must establish the equation;
                // D29 applies directly only to expressions already in types.
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
                                return self.runtime_arithmetic_index(t, args, env, opc, *span).map(|(index, _)| index);
                            }
                        }
                    }
                }
                // Facts and signature obligations must retain the same
                // checked instantiation of a generic measure. Dropping its
                // type/dictionary arguments here invents a different opaque
                // value from the one the dependent signature names.
                if let ast::Term::Var(path) = &**f {
                    if (!path.quals.is_empty() || env.lookup_name(&path.name).is_none())
                        && self.resolve_var(path, *span).is_ok_and(|value|
                            matches!(value.kind, ValueKind::TopFn)
                                && Self::closed_index_signature(&value.scheme).is_none()) {
                        return self.checked_term_index(t, env, None);
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
    fn note_value_alias(&mut self, local: LocalId, core: &ECore) {
        if let ECore::Var(original) = core {
            let original = self.value_aliases.get(original).copied().unwrap_or(*original);
            self.value_aliases.insert(local, original);
        }
        if matches!(core, ECore::Lam { .. }) {
            let mut captures = Vec::new();
            free_locals(core, &mut Vec::new(), &mut captures);
            self.local_closure_captures.insert(local, captures);
        }
    }

    fn note_spawned_closure(
        &mut self, core: &ECore, scope: &str, outer: &[String], span: ast::Span,
        dicts: &[(LocalId, ClassApp)], forwarded_parameter: bool,
    ) -> Result<()> {
        let unknown = || Error::at(span, "a task callback crosses the task boundary without shareable evidence");
        let mut captures = Vec::new();
        match core {
            ECore::Lam { .. } => free_locals(core, &mut Vec::new(), &mut captures),
            ECore::Var(local) => {
                let origin = self.value_aliases.get(local).unwrap_or(local);
                let Some(known) = self.local_closure_captures.get(origin) else {
                    return if forwarded_parameter { Ok(()) } else { Err(unknown()) };
                };
                captures.extend(known);
            }
            _ => return Err(unknown()),
        }
        let mut seen = HashSet::new();
        while let Some(local) = captures.pop() {
            let origin = self.value_aliases.get(&local).copied().unwrap_or(local);
            if !seen.insert(origin) { continue }
            if let Some(nested) = self.local_closure_captures.get(&origin) {
                captures.extend(nested);
                continue;
            }
            let Some(binding) = self.obligations.get(&origin) else { continue };
            self.task_captures.push(TaskCapture {
                binder: origin,
                name: binding.name.clone(),
                ty: binding.ty.clone(),
                span,
                dicts: dicts.to_vec(),
                scope: scope.to_string(),
                scopes: self.scope_frames.iter().map(|frame| frame.region.clone()).collect(),
                active: self.active_regions.clone(),
                outer: outer.to_vec(),
                transfer_allowed: false,
            });
        }
        Ok(())
    }

    /// Give equal runtime values the same current lexical spelling while
    /// retaining a proof's derivation scope. Never reinterpret its old names
    /// in the caller's newer scope. Internally named hidden locals remain
    /// resolvable by identity, not by a guessed source spelling.
    fn proof_in_value_scope(
        &self, prop: &PTerm, source: &HashMap<String, LocalId>, current: &Env,
    ) -> PTerm {
        self.proof_values_in_scope(&self.reduce_prop(prop), source, current)
    }

    fn proof_values_in_scope(
        &self, prop: &PTerm, source: &HashMap<String, LocalId>, current: &Env,
    ) -> PTerm {
        let mut visible = HashMap::new();
        for (name, local, _) in &current.locals {
            if current.lookup(name).is_some_and(|(active, _)| active == *local) {
                let original = self.value_aliases.get(local).copied().unwrap_or(*local);
                visible.insert(original, name.clone());
            }
        }
        let mut out = prop.clone();
        let mut names = Vec::new(); out.free_vars(&mut names);
        // Simultaneous substitution avoids an alias's spelling being
        // mistaken for another source name which occurs in the predicate.
        let replacements: Vec<_> = names.iter().filter_map(|name| {
            let local = *source.get(name)?;
            let original = self.value_aliases.get(&local).copied().unwrap_or(local);
            Some((name.clone(), original))
        }).collect();
        for (name, original) in &replacements { out = out.subst(name, &ITerm::LocalValue(*original)); }
        out.subst_local_values(&self.proof_pattern_equalities)
            .subst_local_values(&visible.into_iter().map(|(local, name)| (local, ITerm::Var(name.into()))).collect())
    }

    fn check_constructor_proof(&mut self, env: &mut Env, term: &ast::Term, prop: &PTerm) -> Result<ECore> {
        if let ast::Term::Var(path) = term {
            if path.quals.is_empty() {
                if let Some((local, mut scheme)) = env.lookup_name(&path.name) {
                    if let Ty::Prp(statement) = self.uni.zonk(&scheme.ty) {
                        if let Some(origin) = self.proof_value_scopes.get(&local) {
                            let caller: HashMap<_, _> = env.locals.iter().map(|(name, id, _)| (name.clone(), *id)).collect();
                            let statement = self.proof_in_value_scope(&statement, origin, env);
                            let goal = self.proof_in_value_scope(prop, &caller, env);
                            scheme.ty = Ty::Prp(Box::new(statement));
                            let mut scoped = env.clone();
                            let slot = scoped.name_slot(&path.name).expect("looked-up proof local");
                            scoped.locals[slot].2 = scheme;
                            return self.check_proof(&mut scoped, term, &goal);
                        }
                    }
                }
            }
        }
        self.check_proof(env, term, prop)
    }

    fn check_proof_former(&self, value: &ValueInfo, span: ast::Span) -> Result<()> {
        let target = (value.module.clone(), value.name.clone());
        if self.staging.contains(&target) {
            return Err(Error::at(span,
                "a compile-time definition is not runtime evidence; expand checked Code instead"));
        }
        if matches!(value.kind, ValueKind::TopFn) && !self.logical.contains(&target) {
            return Err(Error::at(span,
                crate::diagnostic!("an evidence former must belong to the checked, total logical fragment: {}.{} has type {} and is computational", value.module, value.name, value.scheme.ty)));
        }
        Ok(())
    }

    fn check_proof_locals(
        &mut self, env: &Env, decls: &[ast::LocalDecl], body: &ast::Term,
        prop: &PTerm, span: ast::Span, recursive: bool,
    ) -> Result<ECore> {
        self.in_let(span, |checker| {
            let outer: HashSet<_> = checker.obligations.keys().copied().collect();
            let source: HashMap<_, _> = env.locals.iter()
                .map(|(name, local, _)| (name.clone(), *local)).collect();
            let saved_effects = checker.enter_perf();
            let saved_logical = checker.current_logical.clone();
            if checker.current_logical.is_none() {
                checker.current_logical = Some((checker.current_module.clone(), "<erased proof let>".into()));
            }
            let checked = checker.isolate(|checker| {
                let mut scope = env.clone();
                let bindings = if recursive {
                    checker.elab_letrec_form(&mut scope, decls, LocalForm::Proof)?
                } else {
                    let (bindings, memo) = checker.elab_locals(&mut scope, decls, LocalForm::Proof)?;
                    RecursiveBindings { bindings, memo, knots: Vec::new(), functions: HashMap::new() }
                };
                let goal = checker.proof_values_in_scope(prop, &source, &scope);
                let body = checker.check_proof(&mut scope, body, &goal)?;
                checker.pending_erasure_checks.push((checker.current_module.clone(), span,
                    wrap_lets(bindings.bindings.clone(), ECore::Unit)));
                checker.finish_recursive_bindings(bindings, body, &Ty::Prp(Box::new(goal)), span)
            });
            let performed = checker.exit_perf(saved_effects);
            checker.current_logical = saved_logical;
            let (core, uses) = checked?;
            checker.uni.unify_rows(&performed, &EffRow::pure()).map_err(|_| Error::at(
                span, "an erased proof let must be pure: its bindings cannot perform effects or diverge",
            ))?;
            for (local, usage) in uses {
                if outer.contains(&local) {
                    if usage.consumptions().max > 0
                        && matches!(checker.effective_mult(local), Some(Mult::One | Mult::Aff))
                    {
                        return Err(Error::at(span,
                            "an erased proof let cannot consume a linear or affine runtime value"));
                    }
                    // Proof computations do not count as runtime uses of
                    // their captures, including erased logical parameters.
                } else if checker.effective_mult(local) != Some(Mult::Zero) {
                    // Its own binders still obey ordinary usage rules.
                    checker.uses.entry(local).or_default().seq_add(&usage);
                }
            }
            Ok(core)
        })
    }

    fn check_proof(&mut self, env: &mut Env, term: &ast::Term, prop: &PTerm) -> Result<ECore> {
        let source = env.locals.iter().map(|(name, local, _)| (name.clone(), *local)).collect();
        let frozen_prop = self.proof_values_in_scope(prop, &source, &Env::default());
        if matches!(term, ast::Term::Ann(_, ast::Type::Elaborated(..), _)) {
            // Constructor evidence and congruence arguments enter here
            // directly. Native macro Code contracts must still be checked,
            // with the actual proposition guiding their nested proof trees.
            // A Code metavariable may be solved outside a later generated
            // binder. Store identities in its contract, never current aliases.
            return self.check(env, term, &Ty::Prp(Box::new(frozen_prop)));
        }
        let scoped_prop = self.proof_values_in_scope(&frozen_prop, &HashMap::new(), env);
        let prop = &scoped_prop;
        let status = self.static_obligation(prop)?;
        // A hypothetical proof parameter remains usable even when its
        // proposition is closed and false (the domain is uninhabited).
        // Static decision failure cannot erase an already checked lexical
        // hypothesis. The actual proof term still checks below, including
        // exact proposition identity and evidence-use invalidation.
        let supplied = matches!(status, ObligStatus::Disproved | ObligStatus::Timeout)
            && self.evidence_in(env).items.iter().any(|(_, premise, _, _)| {
                self.reduce_prop(premise).eq(&self.reduce_prop(prop))
            });
        match status {
            ObligStatus::Disproved if !supplied => {
                return Err(Error::at(
                    term.span(),
                    format!(
                        "obligation disproved: `{prop}` is false, so no evidence for it can exist here (a failed check must not produce evidence)"
                    ),
                ))
            }
            ObligStatus::Timeout if !supplied => {
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
            ast::Term::Let(decls, body, span) => self.check_proof_locals(env, decls, body, prop, *span, false),
            ast::Term::Letrec(decls, body, span) => self.check_proof_locals(env, decls, body, prop, *span, true),
            ast::Term::Hole(span) => Err(Error::at(
                *span,
                "an unknown proof hole is not evidence; a deployment profile rejects unresolved obligations",
            )),
            ast::Term::Var(p) => {
                if p.quals.is_empty() {
                    if let Some((l, s)) = env.lookup_name(&p.name) {
                        let (t, _, _) = self.instantiate(&s);
                        let t = self.uni.zonk(&t);
                        return match t {
                            Ty::Prp(q) => {
                                let q = self.proof_value_scopes.get(&l)
                                    .map(|origin| self.proof_in_value_scope(&q, origin, env)).unwrap_or(*q);
                                self.unify_prop_reduced(&q, prop).map_err(|m| {
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
                                crate::diagnostic!(
                                    "`{}` has type {}, which is not a proposition",
                                    p.name.text, other
                                ),
                            )),
                        };
                    }
                }
                let v = self.resolve_var(p, p.span())?;
                self.check_proof_former(&v, p.span())?;
                let (t, context, _, ptys) = self.instantiate_at(&v.scheme);
                for constraint in &context { self.new_context_want(constraint, p.span(), env); }
                let t = self.uni.zonk(&t);
                match t {
                    Ty::Prp(q) => {
                        self.unify_prop_reduced(&q, prop).map_err(|m| {
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
                                self.fn_ref(&v, ptys, p.span())?
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
                        crate::diagnostic!("`{}` has type {}, which is not a proposition", v.name, other),
                    )),
                }
            }
            ast::Term::App(f, args, span) => {
                let p = match &**f {
                    ast::Term::Var(p) => p,
                    other => {
                        return Err(Error::at(
                            other.span(),
                            "a proof term is `refl`, `congr [f] e`, a `case` on the induction variable, or the name of evidence",
                        ))
                    }
                };
                let v = if p.quals.is_empty() && env.lookup_name(&p.name).is_none() {
                    self.resolve_var(p, p.span())?
                } else if p.quals.is_empty() {
                    return Err(Error::at(
                        *span,
                        "a proof term is `refl`, `congr [f] e`, a `case` on the induction variable, or the name of evidence",
                    ));
                } else {
                    self.resolve_var(p, p.span())?
                };
                if matches!(v.kind, ValueKind::ProofSearch) {
                    return self.search_proof(env, args, prop, *span);
                }
                self.check_proof_former(&v, *span)?;
                // `congr F e`: the lifting function is an explicit index
                // term (e.g. `\ys -> Cons (f (g x)) ys`) and the evidence
                // argument follows it. The one-argument form `congr e`
                // keeps the lifting function implicit.
                if matches!(v.kind, ValueKind::ProofCtor)
                    && v.name == "congr"
                    && args.len() == 2
                {
                    let f_it = self.idx_of_term(&args[0], env)?;
                    let a = self.uni.fresh_idx_term();
                    let b = self.uni.fresh_idx_term();
                    let evidence_core = self.check_proof(
                        env,
                        &args[1],
                        &PTerm::Eq(Box::new(a.clone()), Box::new(b.clone())),
                    )?;
                    let q = PTerm::Eq(
                        Box::new(self.reduce_idx(&ITerm::App(
                            Box::new(f_it.clone()),
                            vec![a],
                        ))),
                        Box::new(self.reduce_idx(&ITerm::App(Box::new(f_it), vec![b]))),
                    );
                    let source = env.locals.iter().map(|(name, local, _)| (name.clone(), *local)).collect();
                    let q = self.proof_values_in_scope(&q, &source, env);
                    return match self.unify_prop_reduced(&q, prop) {
                        Ok(()) => Ok(evidence_core),
                        Err(m) => Err(Error::at(
                            *span,
                            format!(
                                "{m}: `congr` with this lifting function proves `{}`, not `{prop}`",
                                self.reduce_prop(&q)
                            ),
                        )),
                    };
                }
                let (mut ty, context, _, ptys) = self.instantiate_at(&v.scheme);
                for constraint in &context { self.new_context_want(constraint, *span, env); }
                let mut arg_cores: Vec<ECore> = Vec::new();
                for a in args {
                    let z = self.uni.zonk(&ty);
                    match z {
                        Ty::Fun(d, _, _, c) => {
                            let d = self.uni.zonk(&d);
                            match d {
                                Ty::Prp(q) => {
                                    arg_cores.push(self.check_proof(env, a, &q)?);
                                }
                                other => {
                                    return Err(Error::at(
                                        a.span(),
                                        crate::diagnostic!(
                                            "an evidence argument states a proposition, but this one has type {}",
                                            other
                                        ),
                                    ))
                                }
                            }
                            ty = *c;
                        }
                        Ty::Pi { var, dom: d, cod: c, .. } => {
                            // A dependent binder's argument is the index
                            // value it stands for (a proof former's value
                            // parameter); it states evidence only when
                            // the binder itself does.
                            let d = self.uni.zonk(&d);
                            if let Ty::Prp(q) = &d {
                                arg_cores.push(self.check_proof(env, a, q)?);
                            } else {
                                let core = self.in_erased_proof(a.span(), |checker| checker.check(env, a, &d))?;
                                self.pending_erasure_checks.push((self.current_module.clone(), a.span(), core.clone()));
                                arg_cores.push(core);
                            }
                            // Checking the carrier first solves implicit type
                            // arguments before the structural index is formed.
                            let carrier = self.uni.zonk(&d);
                            let subst = self.checked_term_index(a, env, Some(&carrier))?;
                            ty = c.subst_idx(&var, &subst);
                        }
                        other => {
                            return Err(Error::at(
                                a.span(),
                                crate::diagnostic!("too many evidence arguments ({} is not a function)", other),
                            ))
                        }
                    }
                }
                match self.uni.zonk(&ty) {
                    Ty::Prp(q) => {
                        self.unify_prop_reduced(&q, prop).map_err(|m| {
                            Error::at(*span, crate::diagnostic!("{}: this term proves `{}`, not `{}`", m, q, prop))
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
                                self.fn_ref(&v, ptys, *span)?
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
                        crate::diagnostic!("this proof former does not prove a proposition ({})", other),
                    )),
                }
            }
            ast::Term::Case(scrut, alts, span) => {
                for alt in alts {
                    if let Some(guard) = &alt.guard {
                        return Err(Error::at(guard.span(), "a proof's `case` alternative takes no guard"));
                    }
                }
                self.in_erased_proof(*span, |checker| {
                    let (scrut_core, scrut_ty, reading) = checker.infer_scrutinee(env, scrut)?;
                    checker.pending_erasure_checks.push((checker.current_module.clone(), scrut.span(), scrut_core.clone()));
                    let scrut_ty = checker.uni.zonk(&scrut_ty);
                    checker.compile_alts(env, alts, scrut_core, scrut_ty, reading,
                        &Ty::Prp(Box::new(prop.clone())), *span)
                })
            }
            other => Err(Error::at(
                other.span(),
                "a proof term is `refl`, `congr [f] e`, a `case` on the induction variable, or the name of evidence",
            )),
        }
    }

    /// Erased checking may inspect unrestricted runtime values, but it
    /// cannot consume a linear capture or introduce effects into a proof.
    fn in_erased_proof<T>(&mut self, span: ast::Span,
        check: impl FnOnce(&mut Self) -> Result<T>) -> Result<T> {
        let outer: HashSet<_> = self.obligations.keys().copied().collect();
        let effects = self.enter_perf();
        let checked = self.isolate(check);
        let performed = self.exit_perf(effects);
        let (result, uses) = checked?;
        self.uni.unify_rows(&performed, &EffRow::pure()).map_err(|_| Error::at(
            span, "an erased proof must be pure: its values cannot perform effects or diverge"))?;
        for (local, usage) in uses {
            if outer.contains(&local) {
                if usage.consumptions().max > 0
                    && matches!(self.effective_mult(local), Some(Mult::One | Mult::Aff)) {
                    return Err(Error::at(span, "an erased proof cannot consume a linear or affine runtime value"));
                }
            } else if self.effective_mult(local) != Some(Mult::Zero) {
                self.uses.entry(local).or_default().seq_add(&usage);
            }
        }
        Ok(result)
    }

    fn refine_proof_rows(&self, rows: &mut [Row], column: &ECore,
        form: ITerm, fields: &[LocalId]) {
        let ECore::Var(local) = column else { return };
        let subject = self.value_aliases.get(local).copied().unwrap_or(*local);
        let replacement = HashMap::from([(subject, form.clone())]);
        for row in rows {
            if let Some(scope) = &mut row.proof {
                for value in scope.equalities.values_mut() {
                    *value = value.subst_local_values(&replacement);
                }
                scope.equalities.insert(subject, form.clone());
                scope.values.extend(fields.iter().copied());
            }
        }
    }

    fn refine_proof_bool_rows(&self, rows: &mut [Row], column: &ECore,
        value: bool, span: ast::Span) -> Result<()> {
        if rows.iter().any(|row| row.proof.is_some()) {
            let constructor = self.qualified.values().find(|v|
                matches!(v.kind, ValueKind::BoolLit(found) if found == value))
                .ok_or_else(|| Error::at(span, "missing checked Bool constructor"))?;
            self.refine_proof_rows(rows, column,
                ITerm::Fn(fn_id(&constructor.module, &constructor.name).0.into()), &[]);
        }
        Ok(())
    }

    fn check_pattern_row(&mut self, row: &Row, env: &mut Env, expected: &Ty) -> Result<ECore> {
        let Some(scope) = &row.proof else {
            let expected = self.row_result(env, expected);
            return self.check(env, &row.body, &expected);
        };
        let equalities = std::mem::replace(&mut self.proof_pattern_equalities, scope.equalities.clone());
        let values = std::mem::replace(&mut self.proof_pattern_values, scope.values.clone());
        // Inference/native Code annotations keep this expectation beyond
        // the current lexical scope; do not thaw its subjects to spellings.
        let goal = scope.goal.subst_local_values(&scope.equalities);
        let result = self.check(env, &row.body, &Ty::Prp(Box::new(goal)));
        self.proof_pattern_equalities = equalities;
        self.proof_pattern_values = values;
        result
    }

    /// Experimental plan executor. It sees the checker's actual goal and
    /// shadow-safe evidence snapshot, never text guessed by a syntax macro.
    /// Failed alternatives do not unify metas or record evidence uses.
    fn search_proof(
        &mut self,
        env: &Env,
        args: &[ast::Term],
        prop: &PTerm,
        span: ast::Span,
    ) -> Result<ECore> {
        let span = self.proof_goal_site.unwrap_or(span);
        let [ast::Term::Str(plan, _)] = args else {
            return Err(Error::at(span, "Proof.search expects one literal tactic plan"));
        };
        if plan.len() > 128 {
            return Err(Error::at(span, "tactic plan exceeds 128 bytes"));
        }
        let steps: Vec<_> = plan.split_whitespace().collect();
        if steps.is_empty() || steps.len() > 8
            || steps.iter().any(|s| !matches!(*s, "assumption" | "linear"))
        {
            return Err(Error::at(span, "tactic plan needs 1–8 steps: assumption or linear"));
        }
        // The bootstrap branch tracks proof values across strict aliases
        // and shadowing. Put the goal in the same current value scope as
        // `evidence_in`, rather than comparing old source spellings.
        let caller = env.locals.iter().map(|(name, id, _)| (name.clone(), *id)).collect();
        // Keep opaque measures intact for arithmetic search, just as
        // `evidence_in` does. Reduction belongs to exact proof matching.
        let goal = self.proof_values_in_scope(prop, &caller, env);
        let reduced_goal = self.reduce_prop(&goal);
        let evidence = self.evidence_in(env);
        for step in steps {
            let used = match step {
                "assumption" => evidence.items.iter().rposition(|(_, q, _, _)| {
                    // These are free local indices, not refinement
                    // binders: never match up to renaming (i < n is not
                    // evidence for n < i, or for j < n).
                    self.reduce_prop(q).eq(&reduced_goal)
                }).map(|i| vec![i]),
                // `linear_discharge` searches for certificates; the trusted
                // `check_linear_proof` rechecks them before it returns. Only
                // facts actually cited by those certificates count as uses.
                "linear" => self.linear_discharge(&evidence, &goal),
                _ => unreachable!("plan validated above"),
            };
            if let Some(used) = used {
                let items: Vec<_> = used.iter().map(|i| &evidence.items[*i]).collect();
                self.note_ev_used(&items, span);
                return Ok(ECore::Unit);
            }
        }
        // The scoped facts are a variable-length list, so they are one
        // `Spell` argument rather than a string built ahead of the
        // message: a name ambiguous across a fact and the goal is
        // written in full, exactly as in the rest of the message.
        Err(Error::at(span, crate::diagnostic!(
            "tactic plan `{plan}` could not prove `{}`; scoped facts: [{}]",
            prop,
            crate::spelling::Facts {
                items: &evidence.items,
                refined: &evidence.refined,
                separator: "; ",
            }
        )))
    }

    /// Proof-goal matching with named-helper reduction: the proof layer
    /// may unfold named total definitions (delta) and reduce `case` on
    /// constructor forms (iota) to bridge `Eq` goals, beyond the D28
    /// conversion ordinary types use (which never unfolds user
    /// definitions).
    fn unify_prop_reduced(&mut self, q: &PTerm, prop: &PTerm) -> std::result::Result<(), String> {
        let q = self.reduce_prop(q);
        let p = self.reduce_prop(prop);
        self.uni.unify_prop(&q, &p)
    }

    fn reduce_prop(&self, p: &PTerm) -> PTerm {
        match self.uni.resolve_prop(p) {
            PTerm::Cmp(op, a, b) => PTerm::Cmp(
                op,
                Box::new(self.reduce_idx(&a)),
                Box::new(self.reduce_idx(&b)),
            ),
            PTerm::Eq(a, b) => {
                PTerm::Eq(Box::new(self.reduce_idx(&a)), Box::new(self.reduce_idx(&b)))
            }
            PTerm::Not(q) => PTerm::Not(Box::new(self.reduce_prop(&q))),
            PTerm::Pred(pf, a) => PTerm::Pred(pf, Box::new(self.reduce_idx(&a))),
            other => other,
        }
    }

    /// The named-helper reduction of one index term, bounded by fuel so
    /// reduction terminates by construction.
    fn reduce_idx(&self, t: &ITerm) -> ITerm {
        self.reduce_idx_fuel(t, 64)
    }

    fn reduce_idx_fuel(&self, t: &ITerm, fuel: u32) -> ITerm {
        let t = self.uni.resolve_idx(t).normalize();
        if fuel == 0 {
            return t;
        }
        match t {
            ITerm::App(f, args) => {
                let args: Vec<ITerm> =
                    args.iter().map(|a| self.reduce_idx_fuel(a, fuel - 1)).collect();
                if let ITerm::Fn(k) = &*f {
                    if let Some(r) = self.unfold_helper(k, &args, fuel - 1) {
                        return r;
                    }
                } else if let ITerm::Inst(instantiation) = &*f {
                    if let IndexHead::Function(id) = &instantiation.head {
                        // Projection returns an already checked argument or
                        // field unchanged. It never fabricates an uninstantiated
                        // constructor/function from the helper's source text.
                        let mut applied = instantiation.dictionaries.clone();
                        for argument in &instantiation.arguments {
                            if let IndexArgument::Value { value, runtime: true, .. } = argument {
                                applied.push(value.clone());
                            }
                        }
                        applied.extend(args.iter().cloned());
                        if let Some(projected) = self.reduce_checked_index_projection(&GlobalId(id.clone()), &applied, fuel - 1) {
                            return projected;
                        }
                    }
                }
                let f = self.reduce_idx_fuel(&f, fuel - 1);
                ITerm::App(Box::new(f), args)
            }
            ITerm::Lam(x, body) => {
                ITerm::Lam(x, Box::new(self.reduce_idx_fuel(&body, fuel - 1)))
            }
            ITerm::Bin(op, a, b) => ITerm::Bin(
                op,
                Box::new(self.reduce_idx_fuel(&a, fuel - 1)),
                Box::new(self.reduce_idx_fuel(&b, fuel - 1)),
            ),
            other => other,
        }
    }

    /// One delta step: the named helper's equation clauses over the
    /// reduced arguments (first-order match), the matched body
    /// interpreted over index terms.
    fn unfold_helper(&self, gid: &str, args: &[ITerm], fuel: u32) -> Option<ITerm> {
        if fuel == 0 {
            return None;
        }
        if let Some(index) = self.reduce_checked_numeric_helper(gid, args, fuel) { return Some(index); }
        for (m, n, eq) in &self.top_eqs {
            if fn_id(m, n).0 != gid {
                continue;
            }
            let mut bind = HashMap::new();
            if !self.match_idx_pats(&eq.pats, args, &mut bind) {
                continue;
            }
            if eq.guard.as_ref().is_some_and(|guard| !guard_always_true(guard)) {
                return None;
            }
            if let Some(r) = self.reduce_eq_body(&eq.body, &bind, fuel) {
                return Some(r);
            }
        }
        None
    }

    /// First-order match of an equation's patterns against index-term
    /// arguments.
    fn match_idx_pats(
        &self,
        pats: &[ast::Pat],
        args: &[ITerm],
        bind: &mut HashMap<String, ITerm>,
    ) -> bool {
        if pats.len() != args.len() {
            return false;
        }
        for (p, a) in pats.iter().zip(args.iter()) {
            match p {
                ast::Pat::Var(n, _) => {
                    bind.insert(n.text.clone(), a.clone());
                }
                ast::Pat::Wild(_) => {}
                ast::Pat::Int(d, _) => {
                    if !matches!(a, ITerm::Num(x) if x == d) {
                        return false;
                    }
                }
                ast::Pat::Con(path, subpats, _) => {
                    let v = match self.resolve_con(path, path.span()) {
                        Ok(v) => v,
                        Err(_) => return false,
                    };
                    let gid = fn_id(&v.module, &v.name).0;
                    let (head, cargs) = match self.idx_ctor(a) {
                        Some(x) => x,
                        None => return false,
                    };
                    if head != gid || cargs.len() != subpats.len() {
                        return false;
                    }
                    if !self.match_idx_pats(subpats, &cargs, bind) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }

    /// The constructor form of an index value: its code generation unit
    /// and arguments (`Cons h t`'s shape), or `None` for values that are
    /// not constructor applications.
    fn idx_ctor(&self, term: &ITerm) -> Option<(String, Vec<ITerm>)> {
        let (head, arguments) = match term {
            ITerm::App(head, arguments) => (&**head, arguments.clone()),
            head => (head, Vec::new()),
        };
        let id = match head {
            ITerm::Fn(id) => id.as_str(),
            ITerm::Inst(instantiation) => match &instantiation.head {
                IndexHead::Function(id) => id.as_str(),
                _ => return None,
            },
            _ => return None,
        };
        let (module, name) = id.rsplit_once('.')?;
        let value = self.qualified.get(&(module.to_string(), name.to_string()))?;
        matches!(value.kind, ValueKind::Ctor { .. }).then(|| (id.to_string(), arguments))
    }

    /// Reduce selections over checked local identities. Returning a subterm
    /// preserves its full quantified types/evidence; unsupported computations
    /// stay opaque instead of losing that metadata during source reduction.
    fn reduce_checked_index_projection(&self, id: &GlobalId, arguments: &[ITerm], fuel: u32) -> Option<ITerm> {
        let (parameters, body) = self.erasure_bodies.get(id)?;
        if parameters.len() != arguments.len() { return None; }
        let locals = parameters.iter().copied().zip(arguments.iter().cloned()).collect();
        self.project_index_core(body, &locals, fuel)
    }

    fn project_index_core(&self, body: &ECore, locals: &HashMap<LocalId, ITerm>, fuel: u32) -> Option<ITerm> {
        if fuel == 0 { return None; }
        match body {
            ECore::Var(local) => locals.get(local).cloned(),
            ECore::Let(local, value, body) => {
                let value = self.project_index_core(value, locals, fuel - 1)?;
                let mut nested = locals.clone(); nested.insert(*local, value);
                self.project_index_core(body, &nested, fuel - 1)
            }
            ECore::Case { scrut, alts } => {
                let value = self.project_index_core(scrut, locals, fuel - 1)?;
                let (id, arguments) = self.idx_ctor(&value)?;
                let (module, name) = id.rsplit_once('.')?;
                let constructor = self.qualified.get(&(module.to_string(), name.to_string()))?;
                let ValueKind::Ctor { tag, field_erased, .. } = &constructor.kind else { return None; };
                if arguments.len() != field_erased.len() { return None; }
                let fields: Vec<_> = arguments.into_iter().zip(field_erased).filter_map(|(value, erased)| (!erased).then_some(value)).collect();
                for alt in alts {
                    let mut nested = locals.clone();
                    match &alt.pat {
                        CPat::Con { tag: candidate, fields: binders, .. } if candidate == tag => {
                            if fields.len() != binders.len() { return None; }
                            nested.extend(binders.iter().copied().zip(fields.iter().cloned()));
                        }
                        CPat::Var(local, _) => { nested.insert(*local, value.clone()); }
                        CPat::Wild => {},
                        _ => continue,
                    }
                    return self.project_index_core(&alt.body, &nested, fuel - 1);
                }
                None
            }
            _ => None,
        }
    }

    /// Interpret a named helper's equation body over index terms
    /// (constructor-form matching, case iota, and beta); `None` is an
    /// unreducible position and the application stays stuck.
    fn reduce_eq_body(
        &self,
        t: &ast::Term,
        bind: &HashMap<String, ITerm>,
        fuel: u32,
    ) -> Option<ITerm> {
        if fuel == 0 {
            return None;
        }
        Some(match t {
            ast::Term::Int(d, _) => ITerm::Num(d.clone().into()),
            ast::Term::Ann(e, _, _) => return self.reduce_eq_body(e, bind, fuel - 1),
            ast::Term::Var(p) => {
                match (p.quals.is_empty(), bind.get(&p.name.text)) {
                    (true, Some(it)) => it.clone(),
                    _ => {
                        if let Ok(v) = self.resolve_var(p, p.span()) {
                            ITerm::Fn(fn_id(&v.module, &v.name).0.into())
                        } else {
                            ITerm::Var(p.name.text.clone().into())
                        }
                    }
                }
            }
            ast::Term::Con(p) => {
                let v = self.resolve_con(p, p.span()).ok()?;
                ITerm::Fn(fn_id(&v.module, &v.name).0.into())
            }
            ast::Term::Lam(pats, body, _) => {
                let mut bind2 = bind.clone();
                let mut names = Vec::new();
                for p in pats {
                    match p {
                        ast::Pat::Var(n, _) => {
                            names.push(n.text.clone());
                            bind2.insert(n.text.clone(), ITerm::Var(n.text.clone().into()));
                        }
                        _ => return None,
                    }
                }
                let mut it = self.reduce_eq_body(body, &bind2, fuel - 1)?;
                for n in names.into_iter().rev() {
                    it = ITerm::Lam(n.into(), Box::new(it));
                }
                it
            }
            ast::Term::App(f, args, _) => {
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
                        if let (Some(opc), 2) = (opc, args.len()) {
                            // The checked Core reader above supplies arithmetic
                            // equations. Raw overloaded source is never authority.
                            let _ = opc;
                            return None;
                        }
                    }
                }
                let head = self.reduce_eq_body(f, bind, fuel - 1)?;
                let mut out = Vec::new();
                for a in args {
                    out.push(self.reduce_eq_body(a, bind, fuel - 1)?);
                }
                if let ITerm::Fn(k) = &head {
                    if let Some(r) = self.unfold_helper(k, &out, fuel - 1) {
                        return Some(r);
                    }
                }
                // Beta and iota fold into the reduct (a bound function
                // position applies to the interpreted arguments).
                ITerm::App(Box::new(head), out).normalize()
            }
            ast::Term::Case(scrut, alts, _) => {
                let s = self.reduce_eq_body(scrut, bind, fuel - 1)?;
                for alt in alts {
                    if alt.guard.is_some() {
                        return None;
                    }
                    let mut bind2 = bind.clone();
                    if self.match_idx_pats(std::slice::from_ref(&alt.pat), &[s.clone()], &mut bind2)
                    {
                        return self.reduce_eq_body(&alt.body, &bind2, fuel - 1);
                    }
                }
                return None;
            }
            _ => return None,
        })
    }

    /// A call's obligation over arguments the checker already knows
    /// (numerals, arithmetic over them): decided now. `Ok(true)` when it
    /// holds, a refusal naming it when it is false (`3 - 5` at `Nat`
    /// states `5 <= 3`), and `Ok(false)` when it needs evidence.
    pub(crate) fn decide_closed_oblig(&self, obl: &PTerm, span: ast::Span) -> Result<bool> {
        match self.static_obligation(obl)? {
            ObligStatus::Checked => Ok(true),
            ObligStatus::Disproved => Err(Error::at(
                span,
                format!(
                    "the obligation `{obl}` of this call is false: its arguments are known here and do not satisfy it"
                ),
            )),
            ObligStatus::Unsupported | ObligStatus::Timeout | ObligStatus::Unresolved => Ok(false),
        }
    }

    /// The status of a proof obligation (core specification section 4):
    /// checked static evidence, a disproved obligation, unsupported
    /// reasoning, a search timeout, or an unresolved obligation that only
    /// a runtime `decide` can settle. None of the failing outcomes is a
    /// proof.
    fn static_obligation(&self, p: &PTerm) -> Result<ObligStatus> {
        Ok(match self.uni.resolve_prop(p).normalize() {
            PTerm::Lit(true) => ObligStatus::Checked,
            PTerm::Lit(false) => ObligStatus::Disproved,
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
            // Conjunction and disjunction of decidable propositions
            // are decidable (`Dec A` and `Dec B` decide both): both
            // conjuncts check for `&&`, either disjunct for `||`, and
            // a refuted side refutes the conjunction (or the whole
            // disjunction when both sides refute).
            PTerm::And(a, b) => {
                match (self.static_obligation(&a)?, self.static_obligation(&b)?) {
                    (ObligStatus::Disproved, _) | (_, ObligStatus::Disproved) => {
                        ObligStatus::Disproved
                    }
                    (ObligStatus::Checked, ObligStatus::Checked) => ObligStatus::Checked,
                    (ObligStatus::Timeout, _) | (_, ObligStatus::Timeout) => {
                        ObligStatus::Timeout
                    }
                    _ => ObligStatus::Unresolved,
                }
            }
            PTerm::Or(a, b) => match (self.static_obligation(&a)?, self.static_obligation(&b)?) {
                (ObligStatus::Checked, _) | (_, ObligStatus::Checked) => ObligStatus::Checked,
                (ObligStatus::Disproved, ObligStatus::Disproved) => ObligStatus::Disproved,
                (ObligStatus::Timeout, _) | (_, ObligStatus::Timeout) => ObligStatus::Timeout,
                _ => ObligStatus::Unresolved,
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
        let (function, args) = match t {
            ITerm::App(function, args) => (&**function, args.clone()),
            other => (other, Vec::new()),
        };
        let head = match function {
            ITerm::Fn(identity) => identity.as_str(),
            // Quantified element types do not add runtime fields. Retain
            // their identity in the index; only inspect the checked
            // constructor's runtime shape here. Hidden dictionary/index
            // arguments need explicit layout reconstruction, so stay opaque.
            ITerm::Inst(call) if call.dictionaries.is_empty() && call.arguments.is_empty() => match &call.head {
                IndexHead::Function(identity) => identity.as_str(),
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
            // A list of chars decides structurally over its constructor
            // spine (every element below 128); no text literal exists in
            // index position, so a non-spine operand decides at runtime
            // (`decide`) and over law samples only.
            PredFn::Ascii => {
                if self.ctor_form(&it).is_none() {
                    ObligStatus::Unresolved
                } else {
                    let mut cur = it.clone();
                    let mut status = ObligStatus::Checked;
                    loop {
                        match self.ctor_form(&cur) {
                            Some((ra, _)) if ra == 0 => break,
                            Some((ra, args)) if ra >= 2 && args.len() >= 2 => {
                                match self.eval_idx(&args[0], 32) {
                                    Stuck::Val(v) => {
                                        if !(0..128).contains(&v) {
                                            status = ObligStatus::Disproved;
                                            break;
                                        }
                                    }
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
                    status
                }
            }
        })
    }

    /// One type's constructors as `(tag, runtime arity)` pairs (the
    /// first in-scope registration of the name, the law sampler's rule).
    fn ctor_shapes(&self, name: &str) -> Option<Vec<(usize, usize)>> {
        let tk = self.known_type(name)?;
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
        // A refined value reads as its container (the runtime
        // representation IS the container).
        let oty_owned = self
            .refine_view(oty)
            .and_then(|v| v.base)
            .unwrap_or_else(|| oty.clone());
        let oty = &oty_owned;
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
                            crate::diagnostic!(
                                "`NonZero` decides numeric values, not `{}` (a wrong predicate for this value is refused naming the predicate)",
                                other
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
                        crate::diagnostic!(
                            "`NotNull` decides list-shaped values, not `{}` (a wrong predicate for this value is refused naming the predicate)",
                            oty
                        ),
                    ));
                };
                let shapes = self.ctor_shapes(name).unwrap_or_default();
                let alts = shapes
                    .into_iter()
                    .map(|(tag, ra)| {
                        CAlt::new(
                            CPat::con(tag, Vec::new()),
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
                // The char scan over a list spine, or the byte scan
                // over the text's bytes (`String` projects its `Bytes`
                // backing field).
                let list = match oty {
                    Ty::Adt(name, args)
                        if name != crate::known::STRING
                            && self.list_shaped(name) =>
                    {
                        let _ = args;
                        Some(name.clone())
                    }
                    _ => None,
                };
                if let Some(name) = list {
                    let gid = self.install_ascii_char_test(&name, span)?;
                    return Ok(ECore::App(Box::new(ECore::Global(gid, Vec::new())), vec![operand]));
                }
                let bytes = match oty {
                    Ty::Bytes => operand,
                    Ty::Adt(name, _) if name == crate::known::STRING => ECore::Proj(
                        Box::new(operand),
                        ProjField::slot(0, Class::Raw),
                        Ty::Bytes,
                    ),
                    other => {
                        return Err(Error::at(
                            span,
                            crate::diagnostic!(
                                "`Ascii` decides text values (`String`) and lists of chars, not `{}` (a wrong predicate for this value is refused naming the predicate)",
                                other
                            ),
                        ))
                    }
                };
                let gid = self.install_ascii_test();
                ECore::App(
                    Box::new(ECore::Global(gid, Vec::new())),
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
                        crate::diagnostic!(
                            "`Ordered` decides lists of numerics, not `{}` (a wrong predicate for this value is refused naming the predicate)",
                            oty
                        ),
                    ));
                };
                let gid = self.install_ordered_test(name, span)?;
                ECore::App(Box::new(ECore::Global(gid, Vec::new())), vec![operand])
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
            Box::new(Core::Global(gid.clone(), Vec::new())),
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
            repr: crate::core::UnitRepr {
                tparams: 0,
                params: vec![Class::Obj, Class::TObj],
                fields: Vec::new(),
                ..Default::default()
            },
        });
        gid
    }

    /// The scan behind `decide (Ascii xs)` over a list of chars: every
    /// element's code point is below 128, a recursive unit over the
    /// list's constructor spine (a baked test).
    fn install_ascii_char_test(
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
                crate::diagnostic!(
                    "`Ascii` decides a list of chars (an empty constructor and an element-tail constructor), and {} is not that shape (a wrong predicate for this value is refused naming the predicate)",
                    crate::spelling::Nominal(type_name)
                ),
            ));
        };
        let gid = GlobalId(format!("<builtin>.refine-ascii-chars-{}", self.fns.len()));
        // Params: the list word (LocalId 0). Inner binders: head and
        // tail of the cons (1, 2).
        let elem_ok = Core::App(
            Box::new(Core::Prim(PrimOp::Cmp {
                ty: Scalar::Char,
                op: CmpOp::Lt,
            })),
            vec![Core::Var(LocalId(1)), Core::Int(128)],
        );
        let recur = Core::App(
            Box::new(Core::Global(gid.clone(), Vec::new())),
            vec![Core::Var(LocalId(2))],
        );
        let body = Core::Case {
            scrut: Box::new(Core::Var(LocalId(0))),
            alts: vec![
                CAlt::new(CPat::con(nil_tag, Vec::new()), Core::Bool(true)),
                CAlt::new(
                    CPat::Con {
                        tag: cons_tag,
                        fields: vec![LocalId(1), LocalId(2)],
                        classes: vec![Class::Raw, Class::Obj],
                    },
                    Core::If(
                        Box::new(elem_ok),
                        CBranch::new(recur),
                        CBranch::new(Core::Bool(false)),
                    ),
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
            repr: crate::core::UnitRepr { tparams: 0, params: vec![Class::Obj], fields: Vec::new(), ..Default::default() },
        });
        Ok(gid)
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
                crate::diagnostic!(
                    "`Ordered` decides lists of numerics (an empty constructor and an element-tail constructor), and {} is not that shape (a wrong predicate for this value is refused naming the predicate)",
                    crate::spelling::Nominal(type_name)
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
            Box::new(Core::Global(gid.clone(), Vec::new())),
            vec![Core::Var(LocalId(2))],
        );
        let inner = Core::Case {
            scrut: Box::new(Core::Var(LocalId(2))),
            alts: vec![
                CAlt::new(CPat::con(nil_tag, Vec::new()), Core::Bool(true)),
                CAlt::new(
                    CPat::Con {
                        tag: cons_tag,
                        fields: vec![LocalId(3), LocalId(4)],
                        classes: vec![Class::TObj, Class::Obj],
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
                CAlt::new(CPat::con(nil_tag, Vec::new()), Core::Bool(true)),
                CAlt::new(
                    CPat::Con {
                        tag: cons_tag,
                        fields: vec![LocalId(1), LocalId(2)],
                        classes: vec![Class::TObj, Class::Obj],
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
            repr: crate::core::UnitRepr { tparams: 0, params: vec![Class::Obj], fields: Vec::new(), ..Default::default() },
        });
        Ok(gid)
    }

    /// The scan behind the baked `length` measure: the element count of
    /// a list spine, a recursive unit over the container's constructor
    /// forms (`Nil` counts 0 and `Cons` adds one).
    fn install_length_test(
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
                crate::diagnostic!(
                    "`length` measures a list spine (an empty constructor and an element-tail constructor) or a byte buffer, and {} is not that shape (a wrong measure for this value is refused naming the measure)",
                    crate::spelling::Nominal(type_name)
                ),
            ));
        };
        let gid = GlobalId(format!("<builtin>.refine-length-{}", self.fns.len()));
        // Params: the list word (LocalId 0); the cons binders (1, 2).
        let recur = Core::App(
            Box::new(Core::Global(gid.clone(), Vec::new())),
            vec![Core::Var(LocalId(2))],
        );
        let succ = Core::App(
            Box::new(Core::Prim(PrimOp::Nat(NatOp::Add))),
            vec![
                Core::ConstBig { nat: true, digits: "1".to_string() },
                recur,
            ],
        );
        // The element binder is never read: whatever its class, the
        // unit neither counts nor keeps it, so it is `Raw` here and the
        // cell's own layout governs its field when the cell is freed.
        let body = Core::Case {
            scrut: Box::new(Core::Var(LocalId(0))),
            alts: vec![
                CAlt::new(
                    CPat::con(nil_tag, Vec::new()),
                    Core::ConstBig { nat: true, digits: "0".to_string() },
                ),
                CAlt::new(
                    CPat::Con {
                        tag: cons_tag,
                        fields: vec![LocalId(1), LocalId(2)],
                        classes: vec![Class::Raw, Class::Obj],
                    },
                    succ,
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
            repr: crate::core::UnitRepr { tparams: 0, params: vec![Class::Obj], fields: Vec::new(), ..Default::default() },
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
            // The baked measures decide at check time where their
            // operand is a constructor spine or a numeral (the D92
            // evaluator's spine walk): `length` counts the elements,
            // `toNat` is the numeric value itself.
            ITerm::App(f, args) if args.len() == 1 => match &**f {
                ITerm::Fn(k) if k == "<builtin>.toNat" => {
                    self.eval_idx(&self.uni.resolve_idx(&args[0]), fuel - 1)
                }
                ITerm::Fn(k) if k == "<builtin>.length" => {
                    self.eval_idx_len(&self.uni.resolve_idx(&args[0]), fuel)
                }
                _ => Stuck::Stuck,
            },
            _ => Stuck::Stuck,
        }
    }

    /// The baked `length` measure over a constructor spine at check
    /// time: an empty constructor counts 0 and a two-field
    /// element-tail constructor adds one; anything else is stuck.
    fn eval_idx_len(&self, t: &ITerm, fuel: u32) -> Stuck {
        let mut n: i128 = 0;
        let mut cur = t.normalize();
        let mut steps = fuel;
        loop {
            if steps == 0 {
                return Stuck::Timeout;
            }
            steps -= 1;
            match self.ctor_form(&cur) {
                Some((ra, _)) if ra == 0 => return Stuck::Val(n),
                Some((ra, args)) if ra >= 2 && args.len() >= 2 => {
                    match n.checked_add(1) {
                        Some(v) => n = v,
                        None => return Stuck::Stuck,
                    }
                    cur = self.uni.resolve_idx(&args[1]).normalize();
                }
                _ => return Stuck::Stuck,
            }
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
                    crate::diagnostic!("borrow against {}: expected a {form}", other),
                ));
            }
        };
        // A borrow of a place the body builds is a borrow of that place's
        // own scope, so it can never have a region the signature
        // quantifies: the caller names that region and supplies the place
        // it lives in, and the caller's place outlives this body while
        // this body's temporary does not. `Ref r a` in a declared type is
        // therefore inhabited only by a value the caller passed in, never
        // by one this body lends itself (ownership and usage design,
        // section 4).
        let resolved = self.uni.resolve_region(region.as_str());
        if let Some(caller) = self
            .caller_regions
            .iter()
            .find(|r| self.uni.resolve_region(r.as_str()) == resolved)
        {
            let form = if is_mut { "mutBorrow" } else { "borrow" };
            let shown = self.source_region_name(caller).unwrap_or_else(|| caller.clone());
            return Err(Error::at(
                span,
                format!(
                    "region escape: `{form}` lends a place this body constructs, but the region `{shown}` comes from the signature: the caller names it and supplies the place in it, so a borrow at that region must outlive the call and a place built here does not (ownership and usage design, section 4); a declared borrow is inhabited by a place the caller passes in"
                ),
            ));
        }
        self.check_misaligned_place(env, &args[0], is_mut)?;
        let value = self.check(env, &args[0], &inner)?;
        if is_mut { self.make_mutable_cell(value, inner, span) } else { Ok(value) }
    }

    fn make_mutable_cell(&mut self, value: ECore, payload: Ty, span: ast::Span) -> Result<ECore> {
        let id = self.word_block_ctor(GlobalId(crate::core::MUTABLE_CELL.into()), 1, span)?;
        Ok(app_spine(ECore::Ctor(id, vec![payload]), vec![value]))
    }

    fn read_mutable_cell(cell: ECore, payload: &Ty) -> ECore {
        ECore::Proj(Box::new(cell), ProjField::mutable_cell(), payload.clone())
    }

    // Structural matches already give each mutable field its own local
    // value. Keep that copy model, but bind each field's own cell so the
    // location survives calls from the branch.
    fn mutable_pattern_fields(
        &mut self,
        fields: Vec<LocalId>,
        types: &[Ty],
        span: ast::Span,
        body: ECore,
    ) -> Result<(Vec<LocalId>, ECore)> {
        let mut slots = Vec::with_capacity(fields.len());
        let mut cells = Vec::new();
        for (field, ty) in fields.into_iter().zip(types) {
            if let Ty::Mut(_, payload) = ty {
                let slot = self.fresh_local();
                self.note_local_ty(slot, payload);
                let cell = self.make_mutable_cell(ECore::Var(slot), (**payload).clone(), span)?;
                cells.push((field, cell));
                slots.push(slot);
            } else {
                slots.push(field);
            }
        }
        Ok((slots, wrap_lets(cells, body)))
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
            ast::Term::Var(p) if p.quals.is_empty() => match env.lookup_name(&p.name) {
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
            let kind = match self.lookup_known_type(ty_name.as_str()) {
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
        check_row_binders(pats)?;
        let mut env2 = env.clone();
        let mut match_pats = Vec::new();
        let mut match_cols = Vec::new();
        let mut match_types = Vec::new();
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
                        crate::diagnostic!("lambda parameter against non-function type {}", other),
                    ));
                }
            };
            let l = self.fresh_local();
            let logical = matches!(self.uni.zonk(&dom), Ty::Prp(_) | Ty::Sort(_));
            let computational_erased = !logical && mult == Mult::Zero;
            match p {
                ast::Pat::Var(n, _) => {
                    self.bind_syntax_local(
                        &mut env2,
                        &n,
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
                                crate::diagnostic!("the unit pattern `()` cannot match {}", other),
                            ))
                        }
                    }
                    self.register_obligation(l, "_", Some(mult), dom, p.span());
                }
                other => {
                    if logical || computational_erased {
                        return Err(Error::at(other.span(), "an erased lambda parameter needs a variable or wildcard pattern"));
                    }
                    self.register_obligation(l, "a lambda pattern", Some(mult), dom.clone(), p.span());
                    match_pats.push(self.expand_pat(other)?);
                    match_cols.push(ECore::Var(l));
                    match_types.push(dom);
                }
            }
            // This parameter belongs to the lambda body, not its enclosing
            // scope. A further lambda still sees it as an outer capture.
            self.obligations.get_mut(&l).expect("registered lambda parameter").lam = self.lam_depth + 1;
            // An erased parameter takes no runtime slot (core
            // specification section 3).
            if !logical {
                params.push(if computational_erased { None } else { Some(l) });
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
        let bcore = if match_pats.is_empty() {
            self.check(&mut env2, body, &rest)
        } else {
            self.compile_rows(
                vec![Row { pats: match_pats, guard: None, guard_locals: Vec::new(), body: body.as_ref().clone(), env: env2,
                    idx: 0, consumed: Vec::new(), weight: None, guard_weight: None, proof: None }],
                match_cols, match_types, &rest, span, "a lambda argument combination",
            )
        };
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
        if params.is_empty() {
            Ok(ECore::Lam { params: Vec::new(), body: Box::new(bcore) })
        } else {
            Ok(erased_parameter_stages(&params, bcore))
        }
    }

    /// The baked measure shape of a `decide` operand read from the
    /// operand's own type without consuming it (a plain place: the
    /// baked tests apply to values whose shape the type states). `None`
    /// hands the operand to the ordinary source path below.
    fn measure_shape(&mut self, env: &Env, arg: &ast::Term) -> Option<MeasureShape> {
        // `unrefine e` and borrow reads are the value itself at
        // runtime (the refinement's runtime representation IS its
        // container), so the measure reads their operand's shape.
        let mut arg = arg;
        while let ast::Term::App(f, args, _) = arg {
            if args.len() == 1
                && matches!(&**f, ast::Term::Var(p)
                    if p.quals.is_empty()
                        && matches!(p.name.text.as_str(), "copyOut" | "borrow" | "mutBorrow" | "unrefine"))
            {
                arg = &args[0];
            } else {
                break;
            }
        }
        let ast::Term::Var(p) = arg else { return None };
        if !p.quals.is_empty() {
            return None;
        }
        let (_, s) = env.lookup_name(&p.name)?;
        let (t, _, _) = self.instantiate(&s);
        let t = self.uni.zonk(&t);
        // A refined value measures as its container (the runtime
        // representation IS the container).
        let t = self.refine_view(&t).and_then(|v| v.base).unwrap_or(t);
        match t {
            Ty::Bytes => Some(MeasureShape::Bytes),
            Ty::Adt(name, _) => {
                // A list spine: an empty constructor and a two-field
                // element-tail constructor.
                let shapes = self.ctor_shapes(&name)?;
                let nil = shapes.iter().any(|(_, ra)| *ra == 0);
                let cons = shapes.iter().any(|(_, ra)| *ra == 2);
                if nil && cons {
                    Some(MeasureShape::List(name.to_string()))
                } else {
                    None
                }
            }
            _ => None,
        }
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
                // The baked measures (refinement types design: the
                // runtime `decide` lowering computes the measure from
                // the compiler's own vocabulary, never from source a
                // shadowing module could lie with): a byte buffer's
                // size, a list spine's element count, and for any other
                // container the ordinary source `length` below.
                // Infer once, even for global constants and computed
                // operands. A source length function must never replace
                // the baked index measure merely because the operand is
                // not a local variable.
                let (ac, aty) = self.infer(env, arg)?;
                let actual = self.uni.zonk(&aty);
                let carrier = self.refine_base(&actual).unwrap_or(actual);
                self.uni.unify(expected, &Ty::Nat).map_err(|m| Error::at(t.span(), m))?;
                let supported = self.length_measure_carrier(&carrier);
                return match carrier {
                    Ty::Bytes => Ok(app_spine(ECore::Prim(PrimOp::Bytes(BytesOp::Len)), vec![ac])),
                    Ty::Adt(name, _) if supported => {
                        let gid = self.install_length_test(&name, t.span())?;
                        Ok(app_spine(ECore::Global(gid, Vec::new()), vec![ac]))
                    }
                    other => Err(Error::at(arg.span(), crate::diagnostic!("length index operand needs Bytes or a list-shaped carrier, got {}", other))),
                };

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
    // An operation value forwards its arguments through the same checked
    // application path as an explicit lambda. Keep the caller's resolved
    // spelling, including facade aliases, and use unspellable fresh locals.
    fn operation_function_value(
        &mut self, env: &mut Env, path: &ast::VarPath, value: &ValueInfo, span: ast::Span,
    ) -> Result<(ECore, Ty)> {
        let (ty, context, _, _) = self.instantiate_at(&value.scheme);
        if !context.is_empty() {
            return Err(Error::at(span, "effect operation values cannot carry class constraints"));
        }
        let mut result = &ty;
        let mut patterns = Vec::new();
        let mut arguments = Vec::new();
        loop {
            result = match result {
                Ty::Fun(_, _, _, output) => output,
                Ty::Pi { cod, .. } => cod,
                _ => break,
            };
            let name = ast::Name::new(self.fresh_param("$operation-argument"), span);
            patterns.push(ast::Pat::Var(name.clone(), span));
            arguments.push(ast::Term::Var(ast::VarPath { quals: Vec::new(), name }));
        }
        if patterns.is_empty() {
            return Err(Error::at(span, "effect operations must have an argument"));
        }
        let body = ast::Term::App(Box::new(ast::Term::Var(path.clone())), arguments, span);
        let lambda = ast::Term::Lam(patterns, Box::new(body), span);
        let core = self.check(env, &lambda, &ty)?;
        Ok((core, ty))
    }

    fn value_use(&mut self, v: &ValueInfo, span: ast::Span, env: &mut Env) -> Result<(ECore, Ty)> {
        self.value_use_with_obligations(v, span, env).map(|(core, ty, _)| (core, ty))
    }

    fn value_use_with_obligations(&mut self, v: &ValueInfo, span: ast::Span, env: &mut Env) -> Result<(ECore, Ty, Vec<PropOblig>)> {
        self.record_editor_reference(span, ast::RefKind::Value, &v.module, &v.name);
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
                let (ty, _ctx, _val_args, ptys) = self.instantiate_at(&v.scheme);
                return Ok((ECore::Global(op.global_id(), ptys), ty, Vec::new()));
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
            ValueKind::BoolLit(b) => return Ok((ECore::Bool(*b), Ty::Bool, Vec::new())),
            ValueKind::EffectOp { head, .. } => {
                return Err(Error::at(
                    span,
                    format!(
                        "effect operation {} must be applied (it dispatches to the enclosing `handle` of {head})",
                        v.name
                    ),
                ));
            }
            ValueKind::ProofSearch => {
                return Err(Error::at(span, "Proof.search needs an expected proposition; it cannot be inferred or used at runtime"));
            }
            ValueKind::ProofCtor => {
                // Evidence constructors are erased: they are checked as
                // proof terms and vanish at runtime (core specification
                // section 3).
                let (ty, _, _) = self.instantiate(&v.scheme);
                return Ok((ECore::Unit, ty, Vec::new()));
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
        let ((ty, ctx, val_args, ptys), stage_types) = self.instantiate_scoped(&v.scheme);
        let obligations = Self::instantiated_obligations(&v.scheme, &stage_types);
        // Used as a value, a function keeps its result refinements,
        // including those naming its own arguments by position (they
        // name the same function's arguments wherever the value goes, and
        // a later application resolves them). A function flows into a
        // function expectation covariantly in its result (`unify_arg`),
        // so `map char-code cs` is a `List Nat`, `f :: Char -> InRange 0
        // 1114111 Nat = char-code` keeps the range, and a result naming
        // an argument weakens to its base wherever it lands on a type
        // variable.
        // A function whose `using` context states refinement
        // obligations constrains its callers: it must be applied
        // directly (the obligations discharge at its call's arguments).
        // A nullary value has no call arguments: discharge its instantiated
        // predicates at the reference instead. Scoped evidence is checked
        // first, including an actual hypothesis of an uninhabited domain;
        // otherwise only a checked static proposition suffices. Unknown
        // instantiations are never solved by assuming the obligation.
        if !v.scheme.obligs.is_empty() && self.app_callee_depth == 0 {
            if v.scheme.ty.arity() != 0 || obligations.iter().any(|o| !matches!(o, PropOblig::Pred(_))) {
                return Err(Error::at(
                    span,
                    format!(
                        "{} must be applied directly: its `using` context states refinement obligations, which discharge at its call's arguments",
                        v.name
                    ),
                ));
            }
            let evidence = self.evidence_in(env);
            for obligation in &obligations {
                let PropOblig::Pred(predicate) = obligation else { unreachable!() };
                let scoped = self.discharge_oblig(&evidence, predicate, span);
                if scoped.is_ok() || self.decide_closed_oblig(predicate, span)? {
                    continue;
                }
                scoped?;
            }
        }
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
        for constraint in &ctx {
            let id = self.new_context_want(constraint, span, env);
            holes.push(ECore::Hole(id));
        }
        // Implicit value arguments are runtime arguments (decision D29):
        // each becomes a pending, reified to runtime code once unification
        // has fixed its index.
        let mut names: HashMap<String, LocalId> = HashMap::new();
        for (n, l, _) in &env.locals {
            names.insert(n.clone(), *l);
        }
        let scope = IndexRuntimeScope { values: names, locals: env.locals.iter().map(|(_, local, _)| *local).collect(), dictionaries: env.dicts.iter().map(|(local, _)| *local).collect() };
        let mut idx_holes = Vec::new();
        for (i, m) in val_args.iter().enumerate() {
            let iterm = match m {
                Ty::Idx(it) => (**it).clone(),
                other => {
                    return Err(Error::at(
                        span,
                        crate::diagnostic!("internal error: implicit argument of type {} is not an index value", other),
                    ))
                }
            };
            let carrier = v.scheme.val_params.get(i).map(|(_, carrier)| Self::subst_namespaces(
                carrier, &stage_types.types, &stage_types.rows, &stage_types.regions, &stage_types.evidence,
            )).ok_or_else(|| Error::at(span, "implicit index argument has no checked carrier"))?;
            let id = self.idx_pends.len();
            self.idx_pends.push(IdxPend {
                iterm,
                scope: scope.clone(),
                carrier,
                span,
            });
            idx_holes.push(ECore::IdxPending(id));
        }
        let core = match &v.kind {
            ValueKind::TopFn if self.compile_time => self.stage_reference(v, ptys, stage_types, span)?,
            ValueKind::TopFn | ValueKind::PrimDecl { .. } => self.fn_ref(v, ptys, span)?,
            ValueKind::StageFn | ValueKind::MacroFn => {
                self.stage_reference(v, ptys, stage_types, span)?
            }
            ValueKind::StageBuiltin(op) => ECore::Global(op.global_id(), ptys),
            ValueKind::Prim(op) => ECore::Prim(*op),
            ValueKind::Foreign { unit, .. } => ECore::Foreign(unit.clone()),
            ValueKind::Ctor { .. } => ECore::Ctor(fn_id(&v.module, &v.name), ptys),
            ValueKind::Method { idx, .. } => {
                // The class constraint is `ctx[0]`: it becomes the dictionary
                // the method is projected from. It is a method call's
                // want: its instance's refinement obligations discharge
                // at the call's arguments.
                let dict = holes.remove(0);
                if let ECore::Hole(want) = &dict {
                    if let Some(w) = self.wants.iter_mut().find(|w| w.id == *want) {
                        w.method = true;
                    }
                }
                // The slot holds the method value itself, or a closure
                // awaiting the method's own dictionaries and indices.
                let slot_ty = if holes.is_empty() && idx_holes.is_empty() {
                    ty.clone()
                } else {
                    Ty::fun(Ty::Unit, ty.clone())
                };
                ECore::DictProj(Box::new(dict), *idx, self.runtime_method_site(&v.module, &v.scheme, &ptys)?, slot_ty)
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
            ValueKind::BoolLit(_) | ValueKind::EffectOp { .. } | ValueKind::ProofCtor | ValueKind::ProofSearch => {
                unreachable!()
            }
        };
        holes.extend(idx_holes);
        let core = if holes.is_empty() { core } else { app_spine(core, holes) };
        Ok((core, ty, obligations))
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
        observe_effects: bool,
    ) -> Result<(ECore, Ty, Option<EffRow>)> {
        match form {
            LocalForm::Plain | LocalForm::Proof => {
                let saved = observe_effects.then(|| self.enter_perf());
                let checked: Result<(ECore, Ty)> = (|| match sig {
                    Some(t) => Ok((self.check(env, body, &t)?, t)),
                    None => {
                        let (core, ty) = self.infer(env, body)?;
                        // Inference alone is not evidence admission.
                        if let Ty::Prp(prop) = self.uni.zonk(&ty) {
                            Ok((self.check_proof(env, body, &prop)?, ty))
                        } else {
                            Ok((core, ty))
                        }
                    }
                })();
                let performed = saved.map(|saved| self.exit_perf(saved));
                let (core, ty) = checked?;
                if let Some(row) = &performed {
                    self.perform(row)?;
                }
                Ok((core, ty, performed))
            }
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
                Ok((core, ty, None))
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
                let s = self.conv_scheme_in_env(env, ty)?;
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
                    ast::Pat::Var(n, _) => n.clone(),
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
                let inner = sigs.get(&name.text).cloned().unwrap_or_else(|| self.uni.fresh());
                let cell_ty = Ty::Lazy {
                    eff: eff.clone(),
                    region: g.region.clone().into(),
                    inner: Box::new(inner.clone()),
                };
                self.bind_syntax_local(env, &name, l, Scheme::mono(cell_ty), None, pat.span())?;
                cell_info.push((l, eff, inner));
            }
        }

        // Check strict bindings in source order. Their effects are recorded
        // separately so we can validate the eventual memo setup order.
        let mut out = Vec::new();
        let mut strict_effects = Vec::new();
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
                        let (core, ty, effects) = self.elab_local_strict(
                            env,
                            body,
                            *span,
                            sigs.get(&name.text).cloned(),
                            form,
                            group.is_some(),
                        )?;
                        if let Some(row) = effects {
                            strict_effects.push((out.len(), *span, row));
                        }
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
                    let scheme = self.generalize_local(env, l, &ty, is_value.then_some(&core));
                    let equation = self.checked_nat_let_fact(env, &name.text, l, &core, &ty);
                    self.note_value_alias(l, &core);
                    // A strict alias transfers the source binder's usage
                    // discipline. Otherwise `let y = x` launders a %1/%aff
                    // parameter into an unrestricted local (also observable
                    // through syntax templates that introduce temporaries).
                    let mult = match &core {
                        ECore::Var(source) => self.effective_mult(*source),
                        _ => None,
                    };
                    self.bind_syntax_local(env, &name, l, scheme, mult, name.span)?;
                    if let Some(fact) = equation { self.bind_facts(env, &[fact], self.ev_writes.len(), *span)?; }
                    // A let-generalized local's uses are copies of its
                    // definition, so the binding itself is never read.
                    if !self.gen_defs.contains_key(&l) {
                        out.push((l, core));
                    }
                }
                ast::LocalDecl::Bind { pat, body, span } => {
                    let (bc, bt, effects) =
                        self.elab_local_strict(env, body, *span, None, form, group.is_some())?;
                    if let Some(row) = effects {
                        strict_effects.push((out.len(), *span, row));
                    }
                    let l = self.fresh_local();
                    self.note_local_ty(l, &bt);
                    match pat {
                        ast::Pat::Var(n, _) => {
                            let def = is_syntactic_value(body).then_some(&bc);
                            let scheme = self.generalize_local(env, l, &bt, def);
                            let equation = self.checked_nat_let_fact(env, &n.text, l, &bc, &bt);
                            self.note_value_alias(l, &bc);
                            let mult = match &bc {
                                ECore::Var(source) => self.effective_mult(*source),
                                _ => None,
                            };
                            self.bind_syntax_local(env, &n, l, scheme, mult, n.span)?;
                            if let Some(fact) = equation { self.bind_facts(env, &[fact], self.ev_writes.len(), *span)?; }
                            if self.gen_defs.contains_key(&l) {
                                continue;
                            }
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
                            // through field projections. The components
                            // generalize under the value restriction, as a
                            // variable binding does.
                            let def = is_syntactic_value(body).then(|| bc.clone());
                            out.push((l, bc));
                            self.destructure_bind(env, pat, l, &bt, def, &mut out)?;
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
                let checked = self.check(env, body, inner);
                self.memo_depth -= 1;
                let performed = self.exit_perf(saved);
                let bc = checked?;
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
                self.new_want(
                    ClassApp { class: "Copy".into(), args: vec![inner.clone()] },
                    body.span(),
                    env,
                );
                let unit_param = self.fresh_local();
                self.note_local_ty(unit_param, &Ty::Unit);
                let thunk = ECore::Lam {
                    params: vec![unit_param],
                    body: Box::new(bc),
                };
                inits.push((*l, thunk));
            }
            out = self.schedule_local_memos(g, out, inits, strict_effects)?;
        }
        Ok((out, group))
    }

    fn schedule_local_memos(
        &mut self,
        group: &MemoGroup,
        strict: Vec<(LocalId, ECore)>,
        inits: Vec<(LocalId, ECore)>,
        strict_effects: Vec<(usize, ast::Span, EffRow)>,
    ) -> Result<Vec<(LocalId, ECore)>> {
        let strict_ids: HashSet<_> = strict.iter().map(|(id, _)| *id).collect();
        let mut out = Vec::new();
        let mut waiting = Vec::new();
        let mut reachable = HashMap::<LocalId, HashSet<LocalId>>::new();
        for (cell, thunk) in inits {
            out.push((cell, ECore::LazyAlloc));
            reachable.insert(cell, HashSet::from([cell]));
            let mut captures = Vec::new();
            free_locals(&thunk, &mut Vec::new(), &mut captures);
            captures.retain(|id| strict_ids.contains(id));
            waiting.push((cell, thunk, captures));
        }
        let demands: HashMap<_, _> = strict_effects
            .into_iter()
            .filter(|(_, _, row)| self.uni.resolve_row(row).labels.contains(&group.label))
            .map(|(index, span, _)| (index, span))
            .collect();
        let mut bound = HashSet::new();
        self.install_ready_memos(&mut out, &mut waiting, &bound);
        for (index, (id, value)) in strict.into_iter().enumerate() {
            let mut free = Vec::new();
            free_locals(&value, &mut Vec::new(), &mut free);
            let cells: HashSet<_> = free
                .iter()
                .filter_map(|capture| reachable.get(capture))
                .flatten()
                .copied()
                .collect();
            if let Some(&site) = demands.get(&index) {
                if waiting.iter().any(|(cell, _, _)| cells.is_empty() || cells.contains(cell)) {
                    return Err(Error::at(
                        site,
                        "a strict binding demands a memo before its captured locals are initialized",
                    ));
                }
            }
            reachable.insert(id, cells);
            out.push((id, value));
            bound.insert(id);
            self.install_ready_memos(&mut out, &mut waiting, &bound);
        }
        debug_assert!(waiting.is_empty());
        Ok(out)
    }

    fn install_ready_memos(
        &mut self,
        out: &mut Vec<(LocalId, ECore)>,
        waiting: &mut Vec<(LocalId, ECore, Vec<LocalId>)>,
        bound: &HashSet<LocalId>,
    ) {
        for (cell, thunk, captures) in std::mem::take(waiting) {
            if captures.iter().all(|capture| bound.contains(capture)) {
                let id = self.fresh_local();
                self.note_local_ty(id, &Ty::Unit);
                out.push((id, ECore::LazyInit(cell, Box::new(thunk))));
            } else {
                waiting.push((cell, thunk, captures));
            }
        }
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
    ) -> Result<RecursiveBindings> {
        self.elab_letrec_form(env, decls, LocalForm::Plain)
    }

    fn elab_letrec_form<'a>(
        &mut self,
        env: &mut Env,
        decls: &'a [ast::LocalDecl],
        form: LocalForm,
    ) -> Result<RecursiveBindings> {
        // Signatures first.
        let mut sigs: HashMap<String, Ty> = HashMap::new();
        for d in decls {
            if let ast::LocalDecl::Sig { name, ty, span } = d {
                if sigs.contains_key(&name.text) {
                    return Err(Error::at(*span, "duplicate local signature"));
                }
                let s = self.conv_scheme_in_env(env, ty)?;
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
            syntax_name: ast::Name,
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
            syntax_name: ast::Name,
            span: ast::Span,
            pats: Vec<ast::Pat>,
            is_fun: bool,
            is_cell: bool,
            body: &'a ast::Term,
        ) -> Result<()> {
            let name = syntax_name.text.clone();
            if !name.is_empty() && !by_name.insert(name.clone()) {
                return Err(Error::at(
                    span,
                    format!("duplicate binding `{name}` in the `letrec` group"),
                ));
            }
            mems.push(RecMember {
                name,
                syntax_name,
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
                        name.clone(),
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
                            n.clone(),
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
                        ast::Name::new("", pat.span()),
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
                        n.clone(),
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
                    region: g.region.clone().into(),
                    inner: Box::new(inner.clone()),
                };
                m.cell = Some((eff, inner));
                if !m.name.is_empty() {
                    self.bind_syntax_local(env, &m.syntax_name, m.l, Scheme::mono(cell_ty), None, m.span)?;
                }
            } else {
                let ty = sigs.get(&m.name).cloned().unwrap_or_else(|| self.uni.fresh());
                m.ty = ty.clone();
                if !m.name.is_empty() {
                    self.bind_syntax_local(env, &m.syntax_name, m.l, Scheme::mono(ty), None, m.span)?;
                }
            }
        }

        // Elaborate the bodies before deciding which members need boxes.
        let mut cores: Vec<ECore> = Vec::new();
        let mut initializer_effects = Vec::with_capacity(mems.len());
        for m in &mems {
            let mut effects = EffRow::pure();
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
                self.note_local_ty(unit_param, &Ty::Unit);
                cores.push(ECore::Lam {
                    params: vec![unit_param],
                    body: Box::new(bc),
                });
            } else if m.is_fun {
                let lam =
                    ast::Term::Lam(m.pats.clone(), Box::new(m.body.clone()), m.span);
                cores.push(self.check_lam(env, &lam, &m.ty)?);
            } else {
                let saved = std::mem::replace(&mut self.perf, EffRow::pure());
                let checked = (|| match sigs.get(&m.name).filter(|_| form != LocalForm::Proof) {
                    Some(t) => self.check(env, m.body, t),
                    None => {
                        let (c, t) = self.infer(env, m.body)?;
                        self.uni
                            .unify(&t, &m.ty)
                            .map_err(|e| Error::at(m.span, e))?;
                        Ok(c)
                    }
                })();
                effects = self.exit_perf(saved);
                cores.push(checked?);
                self.perform(&effects)?;
            }
            initializer_effects.push(effects);
        }

        let members: Vec<(LocalId, ast::Span, String)> =
            mems.iter().map(|m| (m.l, m.span, m.name.clone())).collect();
        let effectful: Vec<_> = initializer_effects
            .iter()
            .map(|row| !self.uni.resolve_row(row).is_pure())
            .collect();
        let functions: Vec<_> = mems.iter().map(|m| m.is_fun).collect();
        let (initializer_order, forward_captures) =
            order_rec_initializers(&members, &cores, &effectful, &functions)?;
        let mut knots = Vec::new();
        for (m, captures_before_init) in mems.iter_mut().zip(forward_captures) {
            if captures_before_init && !m.is_cell && !m.is_fun {
                let slot = self.fresh_local();
                self.note_local_class(slot, Class::Obj);
                m.boxed = Some(slot);
                knots.push(KnotSlot { local: slot, name: m.name.clone(), ty: m.ty.clone() });
            }
        }

        // Do this after the eager-cycle check: erased proof cores contain
        // Unit and would otherwise hide `letrec p = p` from that check.
        for (index, member) in mems.iter().enumerate() {
            if !member.is_cell && !member.is_fun {
                if let Ty::Prp(prop) = self.uni.zonk(&member.ty) {
                    cores[index] = self.check_proof(env, member.body, &prop)?;
                }
            }
        }

        // Captured words for each function member's unit: free variables
        // from outside the group, sibling values or their slots, and
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
                    } else {
                        want.push(rm.boxed.unwrap_or(rm.l));
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
                    subs.insert(m.l, RecSub::Val(b, m.ty.clone()));
                }
                unit_ids.push(None);
            }
        }
        for core in cores.iter_mut() {
            subst_rec_refs(core, &subs);
        }

        // Allocate cells and boxes before initializing members in dependency
        // order. Suspended references can capture the uninitialized slots.
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
        for i in initializer_order {
            let m = &mems[i];
            let core = cores[i].clone();
            if m.is_cell {
                let tmp = self.fresh_local();
                self.note_local_ty(tmp, &Ty::Unit);
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
                out.push((m.l, ECore::BoxInit(b, Box::new(core), m.ty.clone())));
            } else {
                out.push((m.l, core));
            }
        }

        // The group is monomorphic within itself; its named members
        // generalize for the body like any other local binding. A member's
        // definition is the whole group's bindings: a copy at one
        // instantiation copies every member, and the group's units with
        // them (`Checker::gen_copy`).
        for m in &mems {
            if m.name.is_empty() || m.is_cell {
                continue;
            }
            // A copy of a scoped group would own a different knot. Keep
            // its bindings at this scope's single evaluation.
            let generalizable = knots.is_empty() && (m.is_fun || is_syntactic_value(m.body));
            let def = generalizable.then(|| wrap_lets(out.clone(), ECore::Var(m.l)));
            let ty = self.uni.zonk(&m.ty);
            let scheme = self.generalize_local(&env0, m.l, &ty, def.as_ref());
            self.bind_syntax_local(env, &m.syntax_name, m.l, scheme, None, m.span)?;
        }
        let functions = mems.iter().zip(unit_ids).zip(captured).filter_map(|((member, unit), captures)| {
            unit.map(|id| (id, (member.ty.clone(), captures.len())))
        }).collect();
        Ok(RecursiveBindings { bindings: out, memo: group, knots, functions })
    }

    fn finish_recursive_bindings(
        &mut self,
        recursive: RecursiveBindings,
        body: ECore,
        result_ty: &Ty,
        span: ast::Span,
    ) -> Result<ECore> {
        let RecursiveBindings { mut bindings, memo, knots, functions } = recursive;
        if let Some(group) = &memo {
            self.discharge_group(group, result_ty, span)?;
        }
        if knots.is_empty() {
            return Ok(wrap_lets(bindings, body));
        }

        let mut flow = KnotFlow {
            checker: self,
            roots: &knots,
            functions: &functions,
            locals: knots.iter().enumerate().map(|(index, slot)| (slot.local, index)).collect(),
            visiting: HashSet::new(),
            initialized: HashSet::new(),
            suspended: false,
            span,
        };
        for (local, initializer) in &bindings {
            let captured = flow.value(initializer)?;
            flow.bind(*local, captured);
        }
        let captured = flow.value(&body)?;
        flow.reject(flow.in_type(captured, result_ty))?;

        let result = self.fresh_local();
        self.note_local_ty(result, result_ty);
        bindings.push((result, body));
        for slot in knots.into_iter().rev() {
            let unit = self.fresh_local();
            self.note_local_ty(unit, &Ty::Unit);
            bindings.push((unit, ECore::BoxClear(Box::new(ECore::Var(slot.local)), slot.ty)));
        }
        Ok(wrap_lets(bindings, ECore::Var(result)))
    }

    /// Bind the components of an irrefutable structural pattern to fresh
    /// locals by field projection, extending `out` with one let per binder.
    /// `def` is the definition of `l` when the bound value is a syntactic
    /// value; a component's definition is it with the projections down to
    /// the component.
    fn destructure_bind(
        &mut self,
        env: &mut Env,
        pat: &ast::Pat,
        l: LocalId,
        ty: &Ty,
        def: Option<ECore>,
        out: &mut Vec<(LocalId, ECore)>,
    ) -> Result<()> {
        match pat {
            ast::Pat::Var(n, _) => {
                let scheme = self.generalize_local(env, l, ty, def.as_ref());
                self.bind_syntax_local(env, &n, l, scheme, None, n.span)?;
            }
            ast::Pat::Wild(_) | ast::Pat::Unit(_) => {}
            ast::Pat::As(n, inner, _) => {
                let scheme = self.generalize_local(env, l, ty, def.as_ref());
                self.bind_syntax_local(env, &n, l, scheme, None, n.span)?;
                self.destructure_bind(env, inner, l, ty, def, out)?;
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
                            crate::diagnostic!("pattern does not match type {}", other),
                        ));
                    }
                };
                for (i, item) in items.iter().enumerate() {
                    let fl = self.fresh_local();
                    self.note_local_ty(fl, &comps[i]);
                    let field =
                        ECore::Proj(Box::new(ECore::Var(l)), ProjField::slot(i, Class::Raw), comps[i].clone());
                    let fdef = def.as_ref().map(|d| ECore::Let(l, Box::new(d.clone()), Box::new(field.clone())));
                    out.push((fl, field));
                    self.destructure_bind(env, item, fl, &comps[i], fdef, out)?;
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
    /// effectful computation is never shared polymorphically. `def` is the
    /// binding's definition when it is a syntactic value (`None`: it stays
    /// monomorphic). The quantified type metavariables of `l` and its
    /// definition are recorded: each use elaborates to its own copy of the
    /// definition ([`Checker::gen_copy`]), and the caller drops the
    /// binding itself where nothing else reads it.
    fn generalize_local(&mut self, env: &Env, l: LocalId, ty: &Ty, def: Option<&ECore>) -> Scheme {
        let Some(def) = def else {
            return Scheme::mono(ty.clone());
        };
        let env_metas = env_free_metas(&self.uni, env);
        // A deferred literal's type stays monomorphic: one literal has one
        // type (primitive types specification, section 7).
        let lit_metas = self.literal_metas();
        let mut code_metas = Vec::new();
        stored_code_metas(&self.uni.zonk(ty), &mut code_metas);
        let mut mine = self.uni.free_metas(ty);
        // A value's deferred dictionaries are copied with its definition.
        // Quantify their private type variables too: an empty collection's
        // element is not yet visible in its unconstrained container type.
        // Captured/literal/code variables retain the ordinary restrictions.
        let mut parts = CopyParts::default();
        def_parts(def, &mut parts);
        for want in self.wants.iter().filter(|want| parts.wants.contains(&want.id)) {
            for argument in &want.class.args {
                stored_code_metas(&self.uni.zonk(argument), &mut code_metas);
                for meta in self.uni.free_metas(argument) {
                    if !mine.contains(&meta) { mine.push(meta); }
                }
            }
        }
        let quantify: Vec<EitherMeta> = mine
            .into_iter()
            .filter(|m| !env_metas.contains(m) && !lit_metas.contains(m) && !code_metas.contains(m))
            .collect();
        if quantify.is_empty() {
            return Scheme::mono(ty.clone());
        }
        let (ty2, tps, rps) = self.uni.generalize(ty, &quantify);
        let metas: Vec<crate::types::MetaId> = quantify
            .iter()
            .filter_map(|m| match m {
                EitherMeta::Type(t) => Some(*t),
                EitherMeta::Row(_) => None,
            })
            .collect();
        if !metas.is_empty() {
            self.gen_defs.insert(l, def.clone());
        }
        self.gen_locals.insert(l, metas);
        Scheme {
            evidence_params: Vec::new(),
            con_params: Vec::new(),
            obligs: Vec::new(),
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
        reading: Option<Reading>,
        expected: &Ty,
        span: ast::Span,
    ) -> Result<ECore> {
        let sloc = self.fresh_local();
        self.note_local_ty(sloc, &scrut_ty);
        self.note_value_alias(sloc, &scrut_core);
        // The scrutinee column's reading: each branch of the decision
        // tree over it binds the fact its test establishes.
        if let Some(r) = reading {
            self.col_readings.insert(sloc, r);
        }
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
                guard_locals: Vec::new(),
                body: a.body.clone(),
                env: env.clone(),
                idx: i,
                consumed: Vec::new(),
                weight: weights[i],
                guard_weight: guard_weights[i],
                proof: if let Ty::Prp(goal) = self.uni.zonk(expected) {
                    let source = env.locals.iter().map(|(name, local, _)| (name.clone(), *local)).collect();
                    Some(ProofPatternScope {
                        goal: self.proof_values_in_scope(&goal, &source, &Env::default()),
                        equalities: self.proof_pattern_equalities.clone(),
                        values: self.proof_pattern_values.clone(),
                    })
                } else { None },
            });
        }
        // A `Bool` scrutinee no pattern names is used once, as the
        // branch test: the column branches on it directly, so the match
        // costs what `if` costs (no slot for the test's value).
        let scrut_core = if matches!(self.uni.zonk(&scrut_ty), Ty::Bool)
            && rows.iter().all(|r| matches!(r.pats[0], ast::Pat::Con(..) | ast::Pat::Wild(_)))
        {
            self.inline_cols.insert(sloc, scrut_core);
            None
        } else {
            Some(scrut_core)
        };
        let core = self.compile_rows(
            rows,
            vec![ECore::Var(sloc)],
            vec![scrut_ty],
            expected,
            span,
            "a scrutinee value",
        );
        self.col_readings.remove(&sloc);
        let held = self.inline_cols.remove(&sloc);
        let core = core?;
        match scrut_core.or(held) {
            Some(sc) => Ok(ECore::Let(sloc, Box::new(sc), Box::new(core))),
            None => Ok(core),
        }
    }

    /// Empty rows cover a data column only when its declaration has no
    /// constructors or every declared constructor is actually refuted.
    /// Unknown equations remain possible. Trials must not constrain later
    /// columns, even when an earlier constructor was merely possible.
    fn empty_data_column(&mut self, ty: &Ty) -> bool {
        let Ty::Adt(name, args) = self.uni.zonk(ty) else { return false };
        let Ok(info) = self.lookup_known_type(&name) else { return false };
        if !info.data_family || info.opaque || info.refine.is_some() {
            return false;
        }
        for (name, _) in &info.ctors {
            let Ok(constructor) = self.ctor_of_type(&info, name) else { return false };
            let trial = self.uni.snapshot();
            let matched = self.match_ctor(&constructor, &args);
            self.uni.rollback_to(trial);
            if !matches!(matched, Ok(CtorMatch::Refuted)) {
                return false;
            }
        }
        true
    }

    fn compile_guarded_row(
        &mut self,
        row: &Row,
        guard: &ast::Term,
        later: &[Row],
        expected: &Ty,
        span: ast::Span,
        uncovered: &str,
    ) -> Result<ECore> {
        // The pattern and its where bindings run before the test. Their
        // bindings are visible in the test and the body, never later rows.
        for local in &row.consumed {
            self.note_use(*local, span, &[], &row.env.dicts)?;
        }
        let mut scope = row.env.clone();
        let (binds, group) = if row.guard_locals.is_empty() {
            (Vec::new(), None)
        } else {
            self.elab_locals(&mut scope, &row.guard_locals, LocalForm::Plain)?
        };
        let (test, reading) = self.cond_reading(&mut scope.clone(), guard)?;
        let guard_span = guard.span();
        let body_type = if row.guard_locals.is_empty() { expected.clone() } else {
            let expected = self.row_result(&row.env, expected);
            self.numeric_contract_in_value_scope(&expected, &row.env, &scope)
        };
        let (body, passed) = self.isolate(|checker| {
            let facts = Self::branch_facts(&reading, &BranchKey::Bool(true));
            checker.bind_facts(&mut scope, &facts, reading.end, guard_span)?;
            checker.check_pattern_row(row, &mut scope, &body_type)
        })?;
        let mut later = later.to_vec();
        let facts = Self::branch_facts(&reading, &BranchKey::Bool(false));
        for row in &mut later {
            self.bind_facts(&mut row.env, &facts, reading.end, guard_span)?;
        }
        let (fallback, failed) = self.isolate(|checker| {
            checker.compile_rows(later, Vec::new(), Vec::new(), expected, span, uncovered)
        })?;
        self.merge_branches(vec![passed, failed]);
        if let Some(group) = &group {
            self.discharge_group(group, expected, span)?;
        }
        Ok(wrap_lets(binds, ECore::If(
            Box::new(test),
            CBranch::weighted(row.guard_weight, body),
            CBranch::weighted(row.guard_weight.map(|_| 1), fallback),
        )))
    }

    /// Pattern-matrix compilation with source-order, fall-through
    /// semantics: the first row that matches wins, and a guarded row whose
    /// guard fails falls through to the later rows. Compilation is the
    /// coverage check: a path that runs out of rows is a value no
    /// unguarded row covers, so a guarded row covers nothing (only the
    /// literal guard `True` counts as unguarded) and the match is rejected
    /// rather than faulting at runtime. `uncovered` names the value space
    /// the current path stands for, for that diagnostic.
    fn compile_rows(
        &mut self,
        rows: Vec<Row>,
        cols: Vec<ECore>,
        types: Vec<Ty>,
        expected: &Ty,
        ctx_span: ast::Span,
        uncovered: &str,
    ) -> Result<ECore> {
        if rows.is_empty() {
            for (column, ty) in cols.iter().zip(&types) {
                if self.empty_data_column(ty) {
                    return Ok(ECore::Case { scrut: Box::new(column.clone()), alts: Vec::new() });
                }
            }
            return Err(Error::at(
                ctx_span,
                format!(
                    "non-exhaustive patterns: {uncovered} is not covered by an unguarded row (a guarded row covers nothing, since its guard can fail); add an unguarded final row such as `_ -> ...`"
                ),
            ));
        }
        for r in &rows {
            check_row_binders(&r.pats)?;
        }
        if types.is_empty() && !self.param_facts.is_empty() {
            // Every column is consumed, so each row's patterns have named
            // the parameters they bind: the definition's own obligations
            // become evidence over those names. Taken while the rows
            // check, so a match inside a body does not bind them again.
            let pf = std::mem::take(&mut self.param_facts);
            let mut rows = rows;
            let mut out = Ok(());
            for r in rows.iter_mut() {
                out = self.bind_param_facts(&mut r.env, &pf, ctx_span);
                if out.is_err() {
                    break;
                }
            }
            let out = out.and_then(|()| self.compile_rows(rows, cols, types, expected, ctx_span, uncovered));
            self.param_facts = pf;
            return out;
        }
        if types.is_empty() {
            let first = &rows[0];
            if let Some(guard) = first.guard.as_ref().filter(|guard| !guard_always_true(guard)) {
                let compile = |checker: &mut Self| {
                    checker.compile_guarded_row(first, guard, &rows[1..], expected, ctx_span, uncovered)
                };
                return if first.guard_locals.is_empty() { compile(self) } else { self.in_let(ctx_span, compile) };
            }
            for l in &first.consumed {
                self.note_use(*l, ctx_span, &[], &first.env.dicts)?;
            }
            let mut env = first.env.clone();
            return self.check_pattern_row(first, &mut env, expected);
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
                            self.bind_syntax_local(
                                &mut r.env,
                                &n,
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
            return self.consume_column(rows, cols, types, expected, ctx_span, uncovered);
        }

        // Borrow matching (ownership and usage design, section 4): a
        // borrowed column matches its referent's patterns and binds
        // references to the fields; the elaboration is the referent's
        // uniform word.
        let (borrow, inner_ty) = match &ty0 {
            Ty::Ref(r, inner) => (Some((r.clone(), false)), (**inner).clone()),
            Ty::Mut(r, inner) => (Some((r.clone(), true)), (**inner).clone()),
            // A range-refined number matches as its number (literal
            // rows over `bytes-len b`); its range stays a fact of the
            // scrutinee.
            other => match self.refine_view(other) {
                Some(rv) if self.is_range_refinement(&rv) => (None, rv.base.clone().unwrap_or_else(|| other.clone())),
                _ => (None, other.clone()),
            },
        };
        let observed = if matches!(ty0, Ty::Mut(..)) {
            Self::read_mutable_cell(col0.clone(), &inner_ty)
        } else {
            col0.clone()
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
                self.consume_column(rows, cols, types, expected, ctx_span, uncovered)
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
                self.consume_column(rows, cols, types, expected, ctx_span, uncovered)
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
                self.consume_column(rows, cols, types, expected, ctx_span, uncovered)
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
                self.consume_column(rows, cols, types, expected, ctx_span, uncovered)
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
                                self.bind_syntax_local(
                                    &mut r.env,
                                    &n,
                                    *l,
                                    Scheme::mono(ty0.clone()),
                                    None,
                                    n.span,
                                )?;
                            }
                            // Consuming column 0 moves the row instead of
                            // copying it: the remaining patterns shift down
                            // and the environment is not cloned.
                            let mut a = r;
                            a.pats.remove(0);
                            defaults.push(a);
                        }
                        ast::Pat::Wild(_) => {
                            let mut a = r;
                            a.pats.remove(0);
                            defaults.push(a);
                        }
                        ast::Pat::Int(v, _) | ast::Pat::Sym(v, _) => {
                            // A `Sym` column takes `Sym` literals and
                            // every other column integer literals; the
                            // wrong shape is a type error, not a test.
                            if matches!(p0, ast::Pat::Sym(..)) != matches!(inner_ty, Ty::Sym) {
                                return Err(Error::at(
                                    p0.span(),
                                    crate::diagnostic!("pattern does not match type {}", inner_ty),
                                ));
                            }
                            let node = self.literal_const(&inner_ty, v, p0.span())?;
                            if !literals.iter().any(|(x, _)| x == v) {
                                literals.push((v.clone(), node));
                            }
                            self.mark_consumed(&mut r, &col0);
                            let mut a = r;
                            a.pats.remove(0);
                            lit_rows.push((v.clone(), a));
                        }
                        other => {
                            return Err(Error::at(
                                other.span(),
                                crate::diagnostic!("pattern does not match type {}", inner_ty),
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
                let default_space = if literals.is_empty() {
                    uncovered.to_string()
                } else {
                    let lits: Vec<&str> = literals.iter().map(|(v, _)| v.as_str()).collect();
                    format!("a value other than {}", lits.join(", "))
                };
                // A numeric column's branches state its value: the
                // default branch is none of the literals, a literal's
                // branch is that literal.
                let numeric = fact_numeric(&ty0);
                let lit_names: Vec<String> = literals.iter().map(|(v, _)| v.clone()).collect();
                let mut default_rows = defaults.clone();
                if numeric {
                    self.bind_column_facts(&mut default_rows, &col0, &BranchKey::Default(lit_names), ctx_span)?;
                }
                let (mut chain, m) = self.isolate(|s| {
                    s.compile_rows(
                        default_rows,
                        rest_cols.clone(),
                        rest_tys.clone(),
                        expected,
                        ctx_span,
                        &default_space,
                    )
                })?;
                branch_maps.push(m);
                let mut rest_w = default_sum.first().copied().unwrap_or(None);
                for (i, (v, node)) in literals.into_iter().enumerate().rev() {
                    let mut branch = branch_lists[i].clone();
                    if numeric {
                        self.refine_proof_rows(&mut branch, &col0, ITerm::Num(v.clone().into()), &[]);
                    }
                    if numeric {
                        self.bind_column_facts(&mut branch, &col0, &BranchKey::Lit(v.clone()), ctx_span)?;
                    }
                    let (body, m) = self.isolate(|s| {
                        s.compile_rows(
                            branch,
                            rest_cols.clone(),
                            rest_tys.clone(),
                            expected,
                            ctx_span,
                            &format!("the value {v}"),
                        )
                    })?;
                    branch_maps.push(m);
                    let cmp = literal_eq_cond(&inner_ty, observed.clone(), node);
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
                            let constructor = self.resolve_ctor_for(p, Some(&Ty::Bool), *span)?;
                            let val = match constructor.kind {
                                ValueKind::BoolLit(value) => value,
                                _ => return Err(Error::at(*span, "pattern does not match type Bool")),
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
                    // Consuming column 0 moves the row instead of copying it:
                    // the remaining patterns shift down and the environment
                    // is not cloned.
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
                                    self.bind_syntax_local(
                                        &mut a.env,
                                        &n,
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
                let mut true_rows = true_rows;
                self.refine_proof_bool_rows(&mut true_rows, &col0, true, ctx_span)?;
                self.bind_column_facts(&mut true_rows, &col0, &BranchKey::Bool(true), ctx_span)?;
                let (t, tm) = self.isolate(|s| {
                    s.compile_rows(
                        true_rows,
                        rest_cols.clone(),
                        rest_tys.clone(),
                        expected,
                        ctx_span,
                        "True",
                    )
                })?;
                // Bound after the `True` branch is elaborated: its writes
                // are not on the `False` path.
                let mut false_rows = false_rows;
                self.refine_proof_bool_rows(&mut false_rows, &col0, false, ctx_span)?;
                self.bind_column_facts(&mut false_rows, &col0, &BranchKey::Bool(false), ctx_span)?;
                let (f, fm) = self.isolate(|s| {
                    s.compile_rows(false_rows, rest_cols, rest_tys, expected, ctx_span, "False")
                })?;
                self.merge_branches(vec![tm, fm]);
                // A held-back scrutinee test branches here directly.
                let held = match &col0 {
                    ECore::Var(l) => self.inline_cols.remove(l),
                    _ => None,
                };
                let test = held.unwrap_or(observed);
                Ok(ECore::If(
                    Box::new(test),
                    CBranch::weighted(tw, t),
                    CBranch::weighted(fw, f),
                ))
            }
            Ty::Adt(_, ty_args) => {
                let info = self.lookup_known_type(&type_name_str(&inner_ty))?.clone();
                let ty_args = ty_args.clone();
                // A list-shaped column's constructor branches state its
                // (non-)emptiness (the `NotNull` vocabulary predicate);
                // a borrowed column states nothing about its referent.
                let list_shaped = self.list_shaped(&info.identity)
                    && !matches!(ty0, Ty::Ref(..) | Ty::Mut(..));
                // Classify rows by the column-0 constructor; each branch
                // specializes its rows by prepending the field patterns,
                // so nesting compiles recursively in source order.
                let mut ctor_rows: Vec<Vec<Row>> =
                    (0..info.ctors.len()).map(|_| Vec::new()).collect();
                let mut var_rows: Vec<Row> = Vec::new();
                let mut parts_by_tag: Vec<Option<CtorParts>> =
                    (0..info.ctors.len()).map(|_| None).collect();
                let mut binder_maps: HashMap<usize, HashMap<String, String>> = HashMap::new();
                let mut erased_bindings: HashMap<LocalId, usize> = HashMap::new();
                for mut r in rows.into_iter() {
                    let p0 = r.pats[0].clone();
                    match &p0 {
                        ast::Pat::Var(n, _) => {
                            if let ECore::Var(l) = &col0 {
                                self.bind_syntax_local(
                                    &mut r.env,
                                    &n,
                                    *l,
                                    Scheme::mono(ty0.clone()),
                                    None,
                                    n.span,
                                )?;
                            }
                            // Consuming column 0 moves the row instead of
                            // copying it: the remaining patterns shift down
                            // and the environment is not cloned.
                            let mut a = r;
                            a.pats.remove(0);
                            var_rows.push(a);
                        }
                        ast::Pat::Wild(_) => {
                            let mut a = r;
                            a.pats.remove(0);
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
                                            crate::diagnostic!(
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
                                    crate::diagnostic!(
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
                                if !parts.fields.iter().skip(i + 1).any(|(_, t, _)| ty_mentions(t, b)) {
                                    // Ordinary fields have no name to substitute
                                    // in later field types. Their row-local
                                    // pattern binders may differ freely.
                                    continue;
                                }
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
                                } else {
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
                                            erased_bindings.insert(l, i);
                                            self.bind_syntax_local(
                                                &mut r.env,
                                                &n,
                                                l,
                                                Scheme::mono(subst_idx_names(&parts.fields[i].1, map)),
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
                                crate::diagnostic!("pattern does not match type {}", ty0),
                            ));
                        }
                    }
                }
                let mut alts: Vec<ECore> = Vec::new();
                let mut alt_maps = Vec::new();
                let mut alt_pats: Vec<CPat> = Vec::new();
                let mut branch_lists: Vec<Vec<Row>> = Vec::new();
                // Which `branch_lists` entry each alternative's weight sums.
                let mut alt_branch: Vec<usize> = Vec::new();
                // Constructors no row names are reached by the variable and
                // wildcard rows alone, and those rows see the same matrix
                // under every such constructor. When two or more share it,
                // and its branch would bind no facts and no field, it is
                // compiled once as one wildcard alternative instead of once
                // per constructor (which multiplies through nested columns).
                let column_reading = match &col0 {
                    ECore::Var(l) => self.col_readings.get(l).cloned(),
                    _ => None,
                };
                let share_default = !var_rows.is_empty()
                    && !matches!(ty0, Ty::Mut(..))
                    && !list_shaped
                    && !matches!(column_reading.as_ref().map(|r| &r.kind), Some(ReadingKind::Order(..)))
                    && var_rows.iter().all(|r| r.proof.is_none())
                    && info.ctors.iter().filter(|(_, tag)| ctor_rows[*tag].is_empty()).count() >= 2;
                let mut default_alt: Option<(ECore, HashMap<LocalId, UseNode>, usize)> = None;
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
                    if share_default && ctor_rows[tag].is_empty() {
                        // Weights still count the shared rows once per
                        // constructor, exactly as separate branches did.
                        let mut shared = var_rows.clone();
                        shared.sort_by_key(|r| r.idx);
                        let branch_index = branch_lists.len();
                        if default_alt.is_none() {
                            let key = BranchKey::Ctor { name: name.clone(), arity, list: list_shaped };
                            let mut branch_rows = shared.clone();
                            self.bind_column_facts(&mut branch_rows, &col0, &key, ctx_span)?;
                            let (core, m) = self.isolate(|s| {
                                s.compile_rows(
                                    branch_rows,
                                    cols[1..].to_vec(),
                                    types[1..].to_vec(),
                                    expected,
                                    ctx_span,
                                    &format!("constructor {name}"),
                                )
                            })?;
                            default_alt = Some((core, m, branch_index));
                        }
                        branch_lists.push(shared);
                        continue;
                    }
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
                    for (f, t) in fields.iter().zip(&branch_tys) {
                        self.note_local_ty(*f, t);
                    }
                    branch_tys.extend(types[1..].iter().cloned());
                    let mut branch_rows = branch.clone();
                    let constructor = self.ctor_of_type(&info, &name)?;
                    let mut field_values = Vec::new();
                    let mut operands = Vec::new();
                    let mut substitutions: Vec<(String, ITerm)> = Vec::new();
                    let mut runtime = fields.iter();
                    for (binder, carrier, erased) in &parts.fields {
                        let mut carrier = carrier.clone();
                        for (name, value) in &substitutions {
                            carrier = carrier.subst_idx(name, value);
                        }
                        // An erased field is still a possessed logical
                        // component of this typed constructor branch.
                        let local = if *erased { self.fresh_local() }
                            else { *runtime.next().expect("runtime constructor field") };
                        if *erased { self.note_local_ty(local, &carrier); }
                        let value = ITerm::LocalValue(local);
                        operands.push(ast::Type::IndexOperand(Box::new(value.clone()), Box::new(carrier), ctx_span));
                        substitutions.push((binder.clone(), value));
                        field_values.push(local);
                    }
                    // Erased field binders were introduced before the
                    // runtime columns. Complete only their own derivation
                    // scopes now that those columns have actual identities.
                    for row in &branch_rows {
                        for (_, local, _) in &row.env.locals {
                            if let Some(position) = erased_bindings.get(local) {
                                let mut origin = self.proof_value_scopes.get(local).cloned().unwrap_or_default();
                                for (index, (binder, _, _)) in parts.fields.iter().enumerate().take(*position) {
                                    let name = bmap.get(binder).unwrap_or(binder);
                                    origin.insert(name.clone(), field_values[index]);
                                }
                                self.proof_value_scopes.insert(*local, origin);
                                self.note_value_alias(*local, &ECore::Var(field_values[*position]));
                            }
                        }
                    }
                    if branch_rows.iter().any(|row| row.proof.is_some()) {
                        let args: Vec<_> = operands.iter().collect();
                        let signature = self.with_type_values(&branch_rows[0].env, |checker|
                            checker.index_signature(&constructor.scheme, &args, &[], Some(&inner_ty), ctx_span))?
                            .ok_or_else(|| Error::at(ctx_span, "constructor branch has no checked index signature"))?;
                        let head = if signature.types.is_empty() && signature.dictionaries.is_empty() && signature.arguments.is_empty() {
                            ITerm::Fn(value_index_id(&constructor).into())
                        } else {
                            ITerm::Inst(Box::new(IndexInstantiation {
                                head: IndexHead::Function(value_index_id(&constructor)), types: signature.types,
                                dictionaries: signature.dictionaries, arguments: signature.arguments,
                            }))
                        };
                        let form = if field_values.is_empty() { head } else {
                            ITerm::App(Box::new(head), field_values.iter().map(|local| ITerm::LocalValue(*local)).collect())
                        };
                        self.refine_proof_rows(&mut branch_rows, &col0, form, &field_values);
                    }
                    let key = BranchKey::Ctor { name: name.clone(), arity, list: list_shaped };
                    self.bind_column_facts(&mut branch_rows, &col0, &key, ctx_span)?;
                    let (core, m) = self.isolate(|s| {
                        s.compile_rows(
                            branch_rows,
                            branch_cols,
                            branch_tys,
                            expected,
                            ctx_span,
                            &format!("constructor {name}"),
                        )
                    })?;
                    let (slots, core) = if matches!(ty0, Ty::Mut(..)) {
                        let carriers = runtime_fields.iter().map(|(_, ty)| wrap(&subst_idx_names(ty, &bmap))).collect::<Vec<_>>();
                        self.mutable_pattern_fields(fields, &carriers, ctx_span, core)?
                    } else {
                        (fields, core)
                    };
                    alt_branch.push(branch_lists.len());
                    branch_lists.push(branch);
                    alt_maps.push(m);
                    alt_pats.push(CPat::con(tag, slots));
                    alts.push(core);
                }
                if let Some((core, m, branch_index)) = default_alt {
                    alt_branch.push(branch_index);
                    alt_maps.push(m);
                    alt_pats.push(CPat::Wild);
                    alts.push(core);
                }
                self.merge_branches(alt_maps);
                if alts.is_empty() {
                    // Every constructor is refuted: the match runs on no
                    // value (an impossible scrutinee type).
                    return Err(Error::at(
                        ctx_span,
                        crate::diagnostic!("no constructor of {} can build this value (the indices refute them all)", ty0),
                    ));
                }
                // Branch weights (attribute annotations specification
                // section 2.2): each constructor's alternative sums the
                // weights of the rows reachable through it.
                let sums = self.decision_weights(&branch_lists, "this `case`", ctx_span)?;
                let alts = alt_pats
                    .into_iter()
                    .zip(alts)
                    .zip(alt_branch)
                    .map(|((pat, body), branch)| CAlt::weighted(pat, sums[branch], body))
                    .collect();
                Ok(ECore::Case { scrut: Box::new(observed), alts })
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
                                self.bind_syntax_local(
                                    &mut r.env,
                                    &name,
                                    *l,
                                    Scheme::mono(ty0.clone()),
                                    None,
                                    name.span,
                                )?;
                            }
                            // Consuming column 0 moves the row instead of
                            // copying it: the remaining patterns shift down
                            // and the environment is not cloned.
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
                                crate::diagnostic!("pattern does not match type {}", ty0),
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
                for (f, t) in fields.iter().zip(&branch_tys) {
                    self.note_local_ty(*f, t);
                }
                let field_types = branch_tys.clone();
                branch_tys.extend(types[1..].iter().cloned());
                let core = self.compile_rows(
                    branch,
                    branch_cols,
                    branch_tys,
                    expected,
                    ctx_span,
                    uncovered,
                )?;
                let (slots, core) = if matches!(ty0, Ty::Mut(..)) {
                    self.mutable_pattern_fields(fields, &field_types, ctx_span, core)?
                } else {
                    (fields, core)
                };
                Ok(ECore::Case {
                    scrut: Box::new(observed),
                    alts: vec![CAlt::new(CPat::con(0, slots), core)],
                })
            }
            Ty::Param(p) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on a value of abstract type {p}"),
            )),
            Ty::App(h, _) => Err(Error::at(
                ctx_span,
                crate::diagnostic!("cannot pattern match on a value of applied abstract type {}", h),
            )),
            Ty::Former(f) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on the type former {:?}", f),
            )),
            Ty::Fun(a, _, _, b) => Err(Error::at(
                ctx_span,
                crate::diagnostic!("cannot pattern match on a function value ({} -> {})", a, b),
            )),
            Ty::Thunk { inner, .. } => Err(Error::at(
                ctx_span,
                crate::diagnostic!("cannot pattern match on a thunk (Thunk _ {}); force it first", inner),
            )),
            Ty::Later(inner) => Err(Error::at(
                ctx_span,
                crate::diagnostic!("cannot pattern match on a Later ({}); force it first", inner),
            )),
            Ty::Lazy { inner, .. } => Err(Error::at(
                ctx_span,
                crate::diagnostic!("cannot pattern match on a memo cell (Lazy _ _ {}); demand it first", inner),
            )),
            Ty::Ref(_, _) | Ty::Mut(_, _) => Err(Error::at(
                ctx_span,
                "cannot pattern match on a borrow of a borrow in the bootstrap slice; copyOut it first",
            )),
            Ty::Meta(_) => {
                // A concrete pattern supplies its own input shape. A
                // constructor's owner comes from normal lexical lookup,
                // never from a later application argument or a global
                // spelling search. The ordinary matrix compiler still
                // checks every row, field, family and uncovered case.
                let mut inferred = None;
                for row in &rows {
                    let pattern = self.expand_pat(&row.pats[0])?;
                    let shape = match pattern {
                        ast::Pat::Unit(_) => Ty::Unit,
                        ast::Pat::Int(..) => Ty::Int,
                        ast::Pat::Sym(..) => Ty::Sym,
                        ast::Pat::Tuple(items, _) => {
                            Ty::Tuple(items.iter().map(|_| self.uni.fresh()).collect())
                        }
                        ast::Pat::Con(path, _, span) => {
                            let constructor = self.resolve_con(&path, span)?;
                            match &constructor.kind {
                                ValueKind::BoolLit(_) => Ty::Bool,
                                ValueKind::Ctor { .. } => {
                                    let (mut result, _, _, _) = self.instantiate_at(&constructor.scheme);
                                    loop {
                                        result = match result {
                                            Ty::Fun(_, _, _, cod) | Ty::Pi { cod, .. } => *cod,
                                            other => break other,
                                        };
                                    }
                                }
                                _ => return Err(Error::at(span, "a constructor pattern requires a constructor")),
                            }
                        }
                        ast::Pat::Var(..) | ast::Pat::Wild(_) => continue,
                        other => return Err(Error::at(other.span(), "cannot infer this pattern's input shape")),
                    };
                    inferred = Some(shape);
                    break;
                }
                let shape = inferred.ok_or_else(|| Error::at(
                    ctx_span, "cannot determine the type of the scrutinee; add a type annotation"))?;
                self.uni.unify(&inner_ty, &shape).map_err(|message| Error::at(ctx_span, message))?;
                self.compile_rows(rows, cols, types, expected, ctx_span, uncovered)
            }
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
                                self.bind_syntax_local(
                                    &mut r.env,
                                    &name,
                                    *l,
                                    Scheme::mono(ty0.clone()),
                                    None,
                                    name.span,
                                )?;
                            }
                            // Consuming column 0 moves the row instead of
                            // copying it: the remaining patterns shift down
                            // and the environment is not cloned.
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
                                ast::Pat::Wild(_) => var.clone().to_string(),
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
                                crate::diagnostic!("pattern does not match type {}", ty0),
                            ));
                        }
                    }
                }
                let name = binder.unwrap_or_else(|| var.clone().to_string());
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
                    wrap(&cod.subst_idx(&var, &ITerm::Var(name.into()))),
                ];
                for (f, t) in fields.iter().zip(&branch_tys) {
                    self.note_local_ty(*f, t);
                }
                let field_types = branch_tys.clone();
                branch_tys.extend(types[1..].iter().cloned());
                let core = self.compile_rows(
                    branch,
                    branch_cols,
                    branch_tys,
                    expected,
                    ctx_span,
                    uncovered,
                )?;
                let (slots, core) = if matches!(ty0, Ty::Mut(..)) {
                    self.mutable_pattern_fields(fields, &field_types, ctx_span, core)?
                } else {
                    (fields, core)
                };
                Ok(ECore::Case {
                    scrut: Box::new(observed),
                    alts: vec![CAlt::new(CPat::con(0, slots), core)],
                })
            }
            Ty::Pi { .. } => Err(Error::at(
                ctx_span,
                "cannot pattern match on a dependent function value",
            )),
            Ty::Forall(_) => Err(Error::at(
                ctx_span,
                "cannot pattern match on a polymorphic value; bind it with a variable and use it at a type",
            )),
            Ty::Sort(s) => Err(Error::at(
                ctx_span,
                format!("cannot pattern match on the universe {s}"),
            )),
            Ty::Idx(t) => Err(Error::at(
                ctx_span,
                crate::diagnostic!("cannot pattern match on the index value {}", t),
            )),
            Ty::Prp(p) => Err(Error::at(
                ctx_span,
                crate::diagnostic!("cannot pattern match on the proposition {} (evidence is erased)", p),
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
        uncovered: &str,
    ) -> Result<ECore> {
        for r in rows.iter_mut() {
            let p0 = r.pats[0].clone();
            if let ast::Pat::Var(n, _) = &p0 {
                if let ECore::Var(l) = &cols[0] {
                    let t = self.uni.zonk(&types[0]);
                    self.bind_syntax_local(&mut r.env, &n, *l, Scheme::mono(t), None, n.span)?;
                }
            }
            // Shift the consumed pattern off in place: the remaining ones
            // move, they are not copied.
            r.pats.remove(0);
        }
        self.compile_rows(
            rows,
            cols[1..].to_vec(),
            types[1..].to_vec(),
            expected,
            ctx_span,
            uncovered,
        )
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
        if p.segments.last().is_some_and(|name| name.resolved_binding().is_none()) {
            if let Some(scope) = p.segments.last().and_then(|name| name.scoped_reference()) {
                if let Some(value) = self.scoped_constructors(scope).into_iter().find(|value| value.module == info.module
                    && info.ctors.iter().any(|(name, _)| *name == value.name))
                {
                    self.record_quote_constructor(p, &value);
                    return Ok(value);
                }
                return Err(Error::at(span, crate::diagnostic!("captured constructor `{}` does not belong to {}", scope.spelling(), crate::spelling::Nominal(&info.identity))));
            }
        }
        if p.segments.last().is_some_and(|name| name.resolved_binding().is_none())
            && (parts.len() == 1 || parts[0] == info.module) {
            if info.ctors.iter().any(|(n, _)| n == last) {
                if let Ok(v) = self.ctor_of_type(info, last) {
                    if matches!(v.kind, ValueKind::Ctor { .. }) {
                        // The constructor must also be surface from
                        // here (D143's named-set rule, D298): a
                        // module-private constructor like `StrBuf` is
                        // visible to its own module's patterns but
                        // never through an import.
                        if v.module != self.current_module
                            && self.surface_def(&v.module, last).is_none()
                        {
                            return Err(Error::at(
                                span,
                                format!(
                                    "constructor {} is not exported by {}",
                                    last, v.module
                                ),
                            ));
                        }
                        let mut v = v;
                        v.name = last.to_string();
                        if self.record_quote_bindings {
                            p.segments.last().unwrap().record_binding(ast::RefKind::Value, &v.module, &v.name);
                        }
                        return Ok(v);
                    }
                }
            }
        }
        let v = self.resolve_con(p, span)?;
        self.record_quote_constructor(p, &v);
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
        let trial = self.uni.snapshot();
        let res: Vec<Ty> = parts
            .res_args
            .iter()
            .map(|t| Self::subst_ty(t, &meta_of))
            .collect();
        for (a, b) in res.iter().zip(ty_args.iter()) {
            if self.uni.unify(a, b).is_ok() {
                continue;
            }
            let za = self.uni.zonk(a);
            let zb = self.uni.zonk(b);
            self.uni.rollback_to(trial);
            if idx_ground(&za) && idx_ground(&zb) {
                return Ok(CtorMatch::Refuted);
            }
            return Ok(CtorMatch::Unknown(crate::diagnostic!("{} vs {}", za, zb)));
        }
        // An inversion guess like `?k + 1 = 0` solves with `?k := -1`,
        // which is not a natural number: the constructor is impossible.
        for p in &parts.local_idx {
            if let IxParam::Val(n, t) = p {
                if *t == Ty::Nat {
                    if let Some(Ty::Idx(it)) = meta_of.get(n) {
                        let solved = self.uni.resolve_idx(it).normalize();
                        if let ITerm::Num(d) = &solved {
                            if d.parse::<i128>().map(|v| v < 0).unwrap_or(false) {
                                self.uni.rollback_to(trial);
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
        self.uni.commit(trial);
        Ok(CtorMatch::Possible(CtorParts {
            fields,
            local_idx: parts.local_idx,
            res_args,
        }))
    }
}

/// The literal guard `True` is the always-true guard spelling (Cinch has
/// no `otherwise`): its row covers what an unguarded row covers.
fn guard_always_true(g: &ast::Term) -> bool {
    matches!(g, ast::Term::Con(p) if p.parts().as_slice() == ["True"])
}

/// The name of a branch condition's hidden evidence binder. It is not
/// an identifier, so source can never name, shadow or forge it; only the
/// discharge path reads it.
const COND_FACT: &str = "%cond";

/// One fact a branch establishes: the proposition, the write-log
/// position of the test that established it (writes after it on the
/// path invalidate it, D192), and the locals its names denoted there (a
/// scope where a name denotes another value does not receive it).
#[derive(Clone, Debug)]
pub(crate) struct Fact {
    comparisons: Vec<ComparisonCertificate>,
    prop: PTerm,
    at: usize,
    names: Vec<(String, LocalId)>,
}

/// What a match column's value is known to be about, so each branch of
/// its decision tree binds the fact its test establishes. `end` is the
/// write-log position once the scrutinee has been evaluated.
#[derive(Clone, Debug)]
pub(crate) struct Reading {
    kind: ReadingKind,
    end: usize,
}

#[derive(Clone, Debug)]
enum ReadingKind {
    /// A `Bool` read as a predicate: the facts of its two outcomes.
    Bool { pos: Vec<Fact>, neg: Vec<Fact> },
    /// A numeric or list-shaped value with an index reading: a literal
    /// branch states equality, the default branch disequality with every
    /// literal, a constructor branch emptiness or non-emptiness.
    Value(ITerm, Vec<(String, LocalId)>),
    /// `comparing k x y`: `LT`/`EQ`/`GT` state the comparison of the two
    /// keys.
    Order(ITerm, ITerm, Vec<(String, LocalId)>, Vec<ComparisonCertificate>),
}

/// Which branch of a column's test a row set sits in.
pub(crate) enum BranchKey {
    Bool(bool),
    /// The literal's branch (numeric columns only).
    Lit(String),
    /// The branch no literal took (numeric columns only).
    Default(Vec<String>),
    /// A constructor's branch; `list` when the type is list-shaped, so
    /// a nullary constructor refutes `NotNull` and any other proves it.
    Ctor { name: String, arity: usize, list: bool },
}

/// The statements a value branch makes about the column's value `it`.
fn value_facts(it: &ITerm, key: &BranchKey) -> Vec<PTerm> {
    let num = |k: &str| Box::new(ITerm::Num(k.to_string().into()));
    match key {
        BranchKey::Lit(k) => vec![PTerm::Cmp(IdxCmp::Eq, Box::new(it.clone()), num(k))],
        BranchKey::Default(lits) => lits
            .iter()
            .map(|k| PTerm::Cmp(IdxCmp::Ne, Box::new(it.clone()), num(k)))
            .collect(),
        BranchKey::Ctor { arity, list: true, .. } => {
            let p = PTerm::Pred(PredFn::NotNull, Box::new(it.clone()));
            vec![if *arity == 0 { PTerm::Not(Box::new(p)) } else { p }]
        }
        _ => Vec::new(),
    }
}

/// Whether a term uses `+`, `-` or `*` anywhere: index arithmetic is
/// mathematical, so a fact over a machine integer's wrapping arithmetic
/// would state something other than what ran.
fn has_arith(t: &ast::Term) -> bool {
    match t {
        ast::Term::App(f, args, _) => {
            matches!(&**f, ast::Term::Var(p) if p.quals.is_empty() && matches!(p.name.text.as_str(), "+" | "-" | "*"))
                || has_arith(f)
                || args.iter().any(has_arith)
        }
        ast::Term::Ann(e, _, _) => has_arith(e),
        _ => false,
    }
}

/// The types whose comparisons are facts: total orders (`Nat`, `Int`,
/// machine integers, `Char`). Floats are excluded: under IEEE a NaN
/// operand makes `not (x < y)` differ from `x >= y`.
fn fact_numeric(t: &Ty) -> bool {
    match t {
        Ty::Nat | Ty::Int => true,
        Ty::Prim(s) => s.is_int_like() || *s == Scalar::Char,
        _ => false,
    }
}

/// The fact that one side holds: it needs both readings; it is
/// established at the earlier of the two tests.
fn fact_or(a: Vec<Fact>, b: Vec<Fact>) -> Vec<Fact> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let conj = |v: &[Fact]| {
        let mut it = v.iter().map(|f| f.prop.clone());
        let first = it.next().expect("nonempty");
        it.fold(first, |acc, p| PTerm::And(Box::new(acc), Box::new(p)))
    };
    let at = a.iter().chain(&b).map(|f| f.at).min().expect("nonempty");
    let mut names: Vec<(String, LocalId)> = Vec::new();
    for (n, l) in a.iter().chain(&b).flat_map(|f| f.names.iter()) {
        if !names.iter().any(|(m, _)| m == n) {
            names.push((n.clone(), *l));
        }
    }
    let comparisons = a.iter().chain(&b).flat_map(|f| f.comparisons.iter().cloned()).collect();
    vec![Fact { prop: PTerm::Or(Box::new(conj(&a)), Box::new(conj(&b))), at, names, comparisons }]
}

/// A comparison's refutation as the complementary comparison (`not (x <
/// 5)` is `x >= 5`), the form the implication table's interval rows read.
fn negate_fact(p: &PTerm) -> PTerm {
    match p {
        PTerm::Cmp(op, a, b) => {
            let op = match op {
                IdxCmp::Lt => IdxCmp::Ge,
                IdxCmp::Le => IdxCmp::Gt,
                IdxCmp::Gt => IdxCmp::Le,
                IdxCmp::Ge => IdxCmp::Lt,
                IdxCmp::Eq => IdxCmp::Ne,
                IdxCmp::Ne => IdxCmp::Eq,
            };
            PTerm::Cmp(op, a.clone(), b.clone())
        }
        other => PTerm::Not(Box::new(other.clone())),
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
        // The continuation is a closure over its value.
        self.note_local_class(kv, Class::Obj);
        self.note_local_ty(v, &ty);
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
                ast::Term::Forget { value, .. } => walk(chk, value, found),
                ast::Term::Narrow { value, witness, .. } => {
                    walk(chk, value, found);
                    walk(chk, witness, found);
                }
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
            // The value weakens into the continuation's type as any
            // argument does (a refined result, a function's result).
            let (core, ty) = self.infer(env, term)?;
            self.unify_arg(&ty, &kv.ty)
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
                let dispatches = self.term_may_dispatch(c);
                // A condition no operation can interrupt elaborates in
                // direct style first (it runs first), so its facts are
                // known to the branches; the continuation then receives
                // the test exactly as the CPS'd condition would pass it.
                let (pre, reading) = if dispatches {
                    (None, Reading { kind: ReadingKind::Bool { pos: Vec::new(), neg: Vec::new() }, end: 0 })
                } else {
                    let (cc, reading) = self.cond_reading(env, c)?;
                    (Some(cc), reading)
                };
                let cspan = c.span();
                self.bind_cont(
                    env,
                    ContKind::Once,
                    dispatches,
                    move |this, env, v, vty| {
                        this.uni
                            .unify(&vty, &Ty::Bool)
                            .map_err(|m| Error::at(cond2.span(), m))?;
                        let (tcore, tm) = this.isolate(|s| {
                            s.in_branch(env, &reading, &BranchKey::Bool(true), cspan, |s, env| {
                                s.cps(env, &tc, &kv2)
                            })
                        })?;
                        let (ecore, em) = this.isolate(|s| {
                            s.in_branch(env, &reading, &BranchKey::Bool(false), cspan, |s, env| {
                                s.cps(env, &ec, &kv2)
                            })
                        })?;
                        this.merge_branches(vec![tm, em]);
                        Ok(ECore::If(
                            Box::new(ECore::Var(v)),
                            CBranch::weighted(tw, tcore),
                            CBranch::weighted(fw, ecore),
                        ))
                    },
                    move |this, env, k| match pre {
                        Some(cc) => {
                            this.uni
                                .unify(&Ty::Bool, &k.ty)
                                .map_err(|m| Error::at(cond.span(), m))?;
                            Ok(app_spine(ECore::Var(k.kv), vec![cc]))
                        }
                        None => this.cps(env, &cond, k),
                    },
                )
            }
            ast::Term::Case(scrut, alts, span) => self.cps_case(env, scrut, alts, kv, *span),
            ast::Term::Let(decls, body, span) => {
                self.in_let(*span, |s| s.cps_let(env, decls, body, kv, *span))
            }
            ast::Term::Letrec(decls, body, span) => {
                self.in_let(*span, |s| s.cps_letrec(env, decls, body, kv, *span))
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
            | ast::Term::Forget { .. }
            | ast::Term::Narrow { .. }
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
                // A function leaf weakens into its continuation's arrow
                // covariantly in its result (`unify_arg`): the arrow was
                // opened with the call site's result before this head
                // elaborated.
                let (core, ty) = self.infer(env, term)?;
                self.unify_arg(&ty, &kv.ty)
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
        if let Some((operation, ty)) = self.annotated_operation(env, f)? {
            return self.cps_op_args(env, &operation, ty, args, kv, span, Vec::new());
        }
        if let ast::Term::Var(p) = f {
            if !p.quals.is_empty() || env.lookup_name(&p.name).is_none() {
                if p.quals.is_empty() && self.is_provider_pure(env, &p.name) {
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
        // A function flows into a function expectation covariantly in its
        // result, so a refined result weakens exactly as a refined value
        // does (`map char-code cs`, `f :: Char -> Nat = char-code`). The
        // parameter and the effects unify exactly.
        if let Ty::Fun(d, m, e, c) = self.uni.zonk(inferred) {
            if Self::final_result_refined(self, &c) && matches!(self.uni.zonk(expected), Ty::Fun(..) | Ty::Meta(_)) {
                let res = self.uni.fresh();
                self.uni.unify(&Ty::Fun(d, m, e, Box::new(res.clone())), expected)?;
                return self.unify_arg(&c, &res);
            }
        }
        // Weakening along the refinement lattice (refinement types
        // design, "narrowings and the implication theory"): a `refined
        // p` value IS a `refined q` value whenever the table derives `p
        // implies q`, at zero cost (the same word). The reverse
        // direction is never implicit: strengthening goes through
        // `narrow` with a `Dec`.
        if let (Some(iv), Some(ev)) = (self.refine_view(inferred), self.refine_view(expected)) {
            let base_ok = match (&iv.base, &ev.base) {
                (Some(a), Some(b)) => self.uni.conv_eq(a, b),
                _ => true,
            };
            if base_ok {
                if pred_implies(&iv.pred, &ev.pred) {
                    return Ok(());
                }
                return Err(crate::diagnostic!(
                    "a `{}` value does not flow to `{}`: the implication table cannot derive `{}` implies `{}` (weakening is implicit and free where implication is decidable; strengthen with `narrow` and a `Dec`, or weaken by ascription to a point the table derives)",
                    inferred, expected, iv.pred, ev.pred
                ));
            }
        }
        // Weakening to the top of the lattice: a comparison refinement (a
        // range of a number, `bytes-len out == n` of a buffer) is a fact
        // about its container, never a carrier with instances of its own,
        // so where no refinement is demanded it flows as the container at
        // zero cost, into a plain expectation or an open one (a class
        // method's carrier, `c - 48`, a tuple or list element):
        // `nat-from-i64 (u32-to-i64 x)`, `nat-add (bytes-get b i) 1`, and
        // `c + 1` is the `Nat` sum, not a value claimed in range. Other
        // refinements (`NonEmpty`, `Sorted`) are no silent subtype of
        // their container: they project through `unrefine`, and keep
        // their point under an open expectation (`Some nz` stays an
        // `Option (NonZero Nat)`). The fact a let-bound result carries
        // stays in the local's type, where the evidence reads it.
        if let Some(iv) = self.refine_view(inferred) {
            let exp = self.uni.zonk(expected);
            if self.refine_view(&exp).is_none() && Checker::is_comparison_refinement(&iv) {
                if let Some(base) = &iv.base {
                    return self.unify_arg(base, &exp);
                }
            }
        }
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
                                crate::diagnostic!("cannot apply a value of type {}", other),
                            ))
                        }
                    };
                    self.perform(&eff)?;
                    self.unify_arg(aty, &dom)
                        .map_err(|m| Error::at(span, m))?;
                    ty = cod;
                }
                // A continuation-passing application has no argument
                // readings, so a result conjunct naming an argument drops,
                // and the result weakens into the continuation's type as
                // any value does (`nat-to-int (char-code (poke ()))`).
                let ty = Checker::strip_positional_results(&self.uni.zonk(&ty));
                self.unify_arg(&ty, &kv.ty)
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
                        crate::diagnostic!("effect operation {} is applied to too many arguments ({})", v.name, other),
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
        self.infer_operation_args(env, v, op_ty, args, span)
    }

    fn infer_operation_args(
        &mut self,
        env: &mut Env,
        v: &ValueInfo,
        op_ty: Ty,
        args: &[ast::Term],
        span: ast::Span,
    ) -> Result<(ECore, Ty)> {
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
                        crate::diagnostic!(
                            "effect operation {} is applied to too many arguments ({})",
                            v.name, other
                        ),
                    ))
                }
            };
            let ac = self.check(env, a, &dom)?;
            let l = self.fresh_local();
            self.note_local_ty(l, &dom);
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
            .map(|(i, e)| (i, e.label.arg.map(|a| a.to_string())))
            .collect();
        let mut found = None;
        for (i, arg) in candidates.into_iter().rev() {
            match (&arg, &label.arg) {
                (Some(x), Some(y)) => {
                    // A failed region unification changes nothing, so no
                    // trial scope is needed.
                    if self.uni.unify_regions(x, y).is_ok() {
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
        self.note_local_ty(rv, &result);
        let resumption = ECore::Lam {
            params: vec![rv],
            body: Box::new(app_spine(ECore::Var(kv.kv), vec![ECore::Var(rv)])),
        };
        let vtable = self.extents[idx].vtable;
        // A vtable slot holds the handler clause's closure.
        let mut core = ECore::Proj(
            Box::new(ECore::Var(vtable)),
            ProjField::slot(slot, Class::Raw),
            Ty::fun(Ty::Unit, Ty::Unit),
        );
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
        let dispatches = self.term_may_dispatch(scrut);
        // A scrutinee no operation can interrupt elaborates in direct
        // style first (it runs first), so its reading reaches the rows;
        // the continuation receives its value as the CPS'd scrutinee
        // would pass it.
        let pre = if dispatches { None } else { Some(self.infer_scrutinee(env, scrut)?) };
        let (pre_core, reading) = match pre {
            Some((c, t, r)) => (Some((c, t)), r),
            None => (None, None),
        };
        let pre_ty = pre_core.as_ref().map(|(_, t)| t.clone());
        self.bind_cont(
            env,
            ContKind::Once,
            dispatches,
            move |this, env, v, vty| {
                if let Some(t) = &pre_ty {
                    this.uni.unify(&vty, t).map_err(|m| Error::at(span, m))?;
                }
                let vty = this.uni.zonk(&vty);
                let ans = this.uni.fresh();
                this.current_cont.push(kv2.clone());
                let code = this.compile_alts(env, &alts2, ECore::Var(v), vty, reading, &ans, span);
                this.current_cont.pop();
                code
            },
            move |this, env, k| match pre_core {
                Some((c, _)) => Ok(app_spine(ECore::Var(k.kv), vec![c])),
                None => this.cps(env, &scrut2, k),
            },
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
        let recursive = self.elab_letrec(&mut env2, decls)?;
        let bcore = self.cps(&mut env2, body, kv)?;
        self.finish_recursive_bindings(recursive, bcore, &kv.ty, span)
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
                        ast::Pat::Var(n, _) => (Some(n.clone()), ()),
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
                                    this.bind_syntax_local(
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
                crate::diagnostic!("internal error: the provider's pure has type {}", other),
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
            Some(ast::Pat::Var(n, _)) => Some(n.clone()),
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
                    crate::diagnostic!("internal error: the provider's bind has type {}", other),
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
                        crate::diagnostic!("internal error: the provider's bind continues with {}", other),
                    ))
                }
            },
            other => {
                return Err(Error::at(
                    sspan,
                    crate::diagnostic!("internal error: the provider's bind takes {}", other),
                ))
            }
        };
        let kind = match select_provider(&self.seq_binders, sspan)?.class.as_str() {
            crate::known::DIRECT_SEQUENCING => ContKind::Once,
            // A transaction runs again whenever an attempt is abandoned,
            // and its continuations with it.
            STM_SEQUENCING => ContKind::Many,
            // A short-circuiting provider may skip the continuation.
            _ => ContKind::AtMost,
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
                        self.bind_syntax_local(&mut env2, n, l, Scheme::mono(payload), None, sspan)?
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
                                this.bind_syntax_local(
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
        let head = self.effect_head(&h.effect, true)?;
        let mut owned = h.clone();
        for clause in &mut owned.clauses {
            if clause.op.declaration_binding().is_some() || clause.op.resolved_binding().is_some()
                || clause.op.scoped_reference().is_some() || clause.op.local_binding().is_some() {
                let path = ast::VarPath { quals: Vec::new(), name: clause.op.clone() };
                let operation = self.resolve_var(&path, clause.op.span)?;
                if !matches!(&operation.kind, ValueKind::EffectOp { head: owner, .. } if *owner == head) {
                    return Err(Error::at(clause.op.span, "handler operation belongs to a different effect"));
                }
                clause.op.text = operation.name;
            }
        }
        let h = &owned;
        let mut clause_names = Vec::new();
        for clause in &h.clauses {
            if clause_names.contains(&clause.op.text) {
                return Err(Error::at(
                    clause.op.span,
                    format!("duplicate handler clause for operation {}", clause.op.text),
                ));
            }
            clause_names.push(clause.op.text.clone());
        }
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
            head: head.clone().into(),
            arg: h.region.as_ref().map(|n| n.text.as_str().into()),
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
        // Each clause closure's type, the clause tuple's instantiation: its
        // class vector makes the tuple's fields counted references, so
        // dropping the handler releases the clause closures.
        let mut clause_tys: Vec<Ty> = Vec::new();
        // The return clause: `return p -> e`.
        let x = self.fresh_local();
        let mut env2 = env.clone();
        self.lam_depth += 1;
        match &h.ret_pat {
            ast::Pat::Var(n, _) => self.bind_syntax_local(
                &mut env2,
                &n,
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
        clause_tys.push(Ty::fun(input.clone(), output.clone()));
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
            let (op_ty, op_ctx, _, op_params) = self.instantiate_at(op_scheme);
            // A clause implements every instantiation of the operation;
            // checking it at fresh flexible types would let `forall a`
            // specialize to Int even when this invocation returns Bool.
            let mut clause_rigids = HashMap::new();
            for (name, actual) in op_scheme.params.iter().zip(op_params) {
                let rigid = self.fresh_param(name);
                self.uni
                    .unify(&actual, &Ty::Param(rigid.clone().into()))
                    .map_err(|message| Error::at(slot_clause.span, message))?;
                clause_rigids.insert(rigid, Ty::Unit);
            }
            if !op_ctx.is_empty() {
                return Err(Error::at(
                    slot_clause.span,
                    format!("effect operation {op_name} takes no class constraints"),
                ));
            }
            let mut ty = op_ty;
            let mut params: Vec<LocalId> = Vec::new();
            let mut param_tys: Vec<Ty> = Vec::new();
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
                            crate::diagnostic!("clause for {op_name} has too many patterns ({})", other),
                        ))
                    }
                };
                let l = self.fresh_local();
                param_tys.push(dom.clone());
                match p {
                    ast::Pat::Var(pn, _) => {
                        self.bind_syntax_local(&mut env3, &pn, l, Scheme::mono(dom), None, pn.span)?
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
            let k_ty = Ty::fun(result, output.clone());
            param_tys.push(k_ty.clone());
            let k = match &slot_clause.resume {
                Some(kn) => {
                    let kl = self.fresh_local();
                    self.bind_syntax_local(
                        &mut env3,
                        &kn,
                        kl,
                        Scheme::mono(k_ty),
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
                        k_ty,
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
            // Operation-local skolems cannot become the enclosing answer
            // type, handler input, or a captured local/dictionary type.
            let mut outer_types = vec![self.uni.zonk(&input), self.uni.zonk(&output)];
            outer_types.extend(env.locals.iter().map(|(_, _, scheme)| self.uni.zonk(&scheme.ty)));
            outer_types.extend(env.dicts.iter().flat_map(|(_, class)| class.args.iter().map(|ty| self.uni.zonk(ty))));
            if outer_types.iter().any(|ty| Self::subst_full(ty, &clause_rigids, &HashMap::new(), &HashMap::new()) != *ty) {
                return Err(Error::at(slot_clause.span, "operation-local type parameter escapes its handler clause"));
            }
            clause_eff = self
                .uni
                .union_rows(&clause_eff, &performed)
                .map_err(|m| Error::plain(m))?;
            closures.push(ECore::Lam { params, body: Box::new(body) });
            clause_tys.push(param_tys.into_iter().rev().fold(output.clone(), |acc, p| Ty::fun(p, acc)));
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
        let core = app_spine(ECore::Ctor(gid, clause_tys), closures);
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
                    crate::diagnostic!("handle expects a handler value, got {}", other),
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
        // The handler value is its clause tuple; the prompt takes the
        // handled computation's result.
        self.note_local_class(hv, Class::Obj);
        self.note_local_ty(pv, &input);
        let ret_call = app_spine(
            // Slot 0 holds the return clause's closure.
            ECore::Proj(
                Box::new(ECore::Var(hv)),
                ProjField::slot(0, Class::Raw),
                Ty::fun(Ty::Unit, Ty::Unit),
            ),
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
        self.note_local_class(kv0_local, Class::Obj);
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
                // A failed region unification changes nothing.
                (Some(x), Some(y)) => self.uni.unify_regions(x, y).is_ok(),
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
        // The selection is a class key, the declaring identity: it is
        // looked up as one, never re-resolved as a spelling in this scope.
        let sig = self
            .class_sigs
            .get(&class.class)
            .and_then(|sigs| sigs.first())
            .cloned()
            .ok_or_else(|| Error::at(span, crate::diagnostic!("unknown signature {}", crate::spelling::Nominal(&class.class))))?;
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
                    crate::diagnostic!("sequencing provider {} has no {mname} method", crate::spelling::Nominal(&class.class)),
                )
            })?;
        let (ty, ctx, _, ptys) = self.instantiate_at(&mscheme);
        let mut holes = Vec::new();
        for constraint in &ctx {
            let id = self.new_context_want(constraint, span, env);
            holes.push(ECore::Hole(id));
        }
        let dict = holes.remove(0);
        let core = if holes.is_empty() {
            ECore::DictProj(Box::new(dict), idx, self.runtime_method_site(&sig.module, &mscheme, &ptys)?, ty.clone())
        } else {
            let slot = ECore::DictProj(
                Box::new(dict),
                idx,
                self.runtime_method_site(&sig.module, &mscheme, &ptys)?,
                Ty::fun(Ty::Unit, ty.clone()),
            );
            app_spine(slot, holes)
        };
        Ok((core, ty))
    }

    /// Is `name` the selected provider's `pure` form (decision D22)? A
    /// local or user definition of the same name shadows it, and a class
    /// constraint carrying `pure` answers first (the bare-`pure` rule:
    /// bare uses keep the D22 provider meaning only when no constrained
    /// method claims the name).
    fn is_provider_pure(&self, env: &Env, name: &ast::Name) -> bool {
        if name.source_spelling() != "pure" || name.local_binding().is_some() || name.resolved_binding().is_some() {
            return false;
        }
        let value = if let Some(scope) = name.scoped_reference() {
            self.scoped_values(scope).into_iter().next()
        } else {
            if env.lookup(&name.text).is_some() || self.constraint_pure(env).is_some() { return false; }
            self.lookup_value(None, &name.text).ok()
        };
        match value {
            Some(v) => match &v.kind {
                ValueKind::Method { .. } => v
                    .scheme
                    .ctx
                    .first()
                    .map(|(_, c)| SEQ_CLASSES.contains(&c.class.as_str()))
                    .unwrap_or(false),
                _ => false,
            },
            None => name.scoped_reference().map(|scope| scope.is_special_form()).unwrap_or(true),
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
                .class_sigs
                .get(&c.class)
                .and_then(|ss| ss.first())
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

    /// Named instance values use the actual factory and ordinary scoped
    /// dictionary obligations, independently of implicit instance search.
    fn named_instance_value(&mut self, env: &mut Env, path: &ast::ConPath) -> Result<Option<(ECore, Ty)>> {
        if self.resolve_con(path, path.span()).is_ok() { return Ok(None); }
        let parts = path.parts();
        let reference = path.segments.last().expect("nonempty constructor path");
        let candidates = if let Some(binding) = reference.resolved_binding() {
            if binding.kind() != ast::RefKind::Value { return Ok(None); }
            vec![(binding.module().to_owned(), binding.name().to_owned())]
        } else if let Some(scope) = reference.scoped_reference() {
            scope.candidates(ast::RefKind::Value).iter()
                .map(|binding| (binding.module().to_owned(), binding.name().to_owned())).collect()
        } else {
            match parts.as_slice() {
                [name] => self.scope_candidates(name),
                [qual, name] => self.scope_qual_candidates(qual, name),
                _ => return Ok(None),
            }
        };
        let Some(inst) = candidates.iter().find_map(|(module, name)| {
            self.instance_list.iter().find(|i| i.module == *module && i.name == *name).cloned()
        }) else { return Ok(None); };
        if !inst.method_obligs.is_empty() {
            return Err(Error::at(path.span(), Self::method_oblig_refusal(&inst, &inst.target)));
        }
        let actuals: Vec<Ty> = inst.params.iter().map(|_| self.uni.fresh()).collect();
        let substitution: HashMap<String, Ty> = inst.params.iter().cloned().zip(actuals.iter().cloned()).collect();
        let target = Self::subst_class(&inst.target, &substitution);
        let mut args = Vec::new();
        if inst.base {
            if inst.module != self.current_module {
                return Err(Error::at(path.span(), "a parameterized module's dictionary needs its instantiated module"));
            }
            args.extend(self.cur_mod_params.iter().map(|(_, local, _)| ECore::Var(*local)));
        }
        for item in &inst.ctx {
            match item {
                InstCtx::Dict(_, constraint) => {
                    let want = self.new_want(Self::subst_class(constraint, &substitution), path.span(), env);
                    args.push(ECore::Hole(want));
                }
                InstCtx::Oblig(_) => return Err(Error::at(path.span(), "an explicit constrained factory needs checked refinement evidence")),
            }
        }
        let ty = Ty::Adt(target.class.into(), target.args);
        Ok(Some((ECore::InstDict(inst.base_id, args, actuals, true), ty)))
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
    ) -> Result<(LocalId, ECore, Option<String>)> {
        let z = self.uni.zonk(&ty);
        if let Ty::Adt(class, args) = &z {
            if class != "Io" && self.class_sigs.get(class.as_str()).is_some_and(|sigs| sigs.iter().any(|sig| sig.params.len() == args.len())) {
                let local = self.fresh_local();
                self.bind_syntax_local(env, &name, local, Scheme::mono(z.clone()), Some(Mult::Many), span)?;
                env.dicts.push((local, ClassApp { class: class.clone().to_string(), args: args.clone() }));
                if let Some(binding) = name.local_binding() {
                    env.syntax_dictionaries.insert(binding.clone(), local);
                }
                return Ok((local, core, None));
            }
        }
        let region = match &z {
            Ty::Adt(n, args) if n == "Io" && args.len() == 1 => match &args[0] {
                Ty::Region(r) => r.clone(),
                other => {
                    return Err(Error::at(
                        span,
                        crate::diagnostic!(
                            "a capability's argument is a runtime identity, got {}",
                            other
                        ),
                    ))
                }
            },
            Ty::Meta(_) => {
                let r = self.fresh_region();
                self.uni.note_flex_region(&r);
                self.uni
                    .unify(&ty, &Ty::Adt("Io".into(), vec![Ty::Region(r.clone().into())]))
                    .map_err(|m| Error::at(span, m))?;
                r
.into()            }
            other => {
                return Err(Error::at(
                    span,
                    crate::diagnostic!(
                        "supply {} must have type `Io r` or another checked signature, got {}",
                        name.text, other
                    ),
                ))
            }
        };
        let l = self.fresh_local();
        self.bind_syntax_local(
            env,
            &name,
            l,
            Scheme::mono(Ty::Adt("Io".into(), vec![Ty::Region(region.clone())])),
            Some(Mult::Many),
            span,
        )?;
        env.dicts
            .push((l, ClassApp { class: "Io".into(), args: vec![Ty::Region(region.clone())] }));
        if let Some(binding) = name.local_binding() {
            env.syntax_dictionaries.insert(binding.clone(), l);
        }
        Ok((l, core, Some(region.to_string())))
    }

    /// A capability for a still-flex runtime identity must not leak that
    /// identity through the supply's scope (design section 4).
    fn check_cap_escape(&mut self, region: &Option<String>, ty: &Ty, span: ast::Span) -> Result<()> {
        let Some(region) = region else { return Ok(()); };
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
        let ((ty, ctx, _, ptys), stage_types) = self.instantiate_scoped(&v.scheme);
        let mut holes = Vec::new();
        let mut supplied_bindings = Vec::new();
        for constraint in &ctx {
            let (n, c) = (&constraint.name, &constraint.class);
            if is_capability_class(&c.class) {
                if let Some((_, term)) = supplies.iter().find(|(sn, _)| &sn.text == n) {
                    let want = Ty::Adt(c.class.clone().into(), c.args.clone());
                    let checked = self.check(env, term, &want)?;
                    let local = match checked {
                        ECore::Var(local) => local,
                        expression => {
                            let local = self.fresh_local(); self.note_local_ty(local, &want);
                            supplied_bindings.push((local, expression)); local
                        }
                    };
                    self.uni.unify_idx(&constraint.witness, &IndexDictionaryBinding::from(local).0)
                        .map_err(|error| Error::at(span, error))?;
                    holes.push(ECore::Var(local));
                    continue;
                }
            }
            let id = self.new_context_want(constraint, span, env);
            holes.push(ECore::Hole(id));
        }
        for (sn, _) in supplies {
            if !ctx
                .iter()
                .any(|constraint| sn.text == constraint.name && is_capability_class(&constraint.class.class))
            {
                return Err(Error::at(
                    sn.span,
                    format!("{} has no capability parameter {}", v.name, sn.text),
                ));
            }
        }
        let core = match &v.kind {
            ValueKind::TopFn if self.compile_time => self.stage_reference(&v, ptys, stage_types, span)?,
            ValueKind::TopFn | ValueKind::PrimDecl { .. } => self.fn_ref(&v, ptys, span)?,
            ValueKind::StageFn | ValueKind::MacroFn => {
                self.stage_reference(&v, ptys, stage_types, span)?
            }
            ValueKind::Prim(op) => ECore::Prim(*op),
            ValueKind::Foreign { unit, .. } => ECore::Foreign(unit.clone()),
            ValueKind::Ctor { .. } => ECore::Ctor(fn_id(&v.module, &v.name), ptys),
            ValueKind::Method { idx, .. } => {
                let dict = holes.remove(0);
                let slot_ty = if holes.is_empty() { ty.clone() } else { Ty::fun(Ty::Unit, ty.clone()) };
                ECore::DictProj(Box::new(dict), *idx, self.runtime_method_site(&v.module, &v.scheme, &ptys)?, slot_ty)
            }
            ValueKind::StageBuiltin(_)
            | ValueKind::BoolLit(_)
            | ValueKind::EffectOp { .. }
            | ValueKind::Law
            | ValueKind::ProofCtor
            | ValueKind::ProofSearch => {
                return Err(Error::at(span, "this value takes no capability supplies"))
            }
        };
        let core = if holes.is_empty() { core } else { app_spine(core, holes) };
        Ok((wrap_lets(supplied_bindings, core), ty))
    }
}

/// The immediate subterms of a container form, in evaluation order.
fn immediate_subterms(t: &ast::Term) -> Vec<ast::Term> {
    match t {
        ast::Term::Ann(e, _, _) | ast::Term::Proj(e, _, _) => vec![(**e).clone()],
        ast::Term::Forget { value, .. } => vec![(**value).clone()],
        ast::Term::Narrow { value, witness, .. } => {
            vec![(**value).clone(), (**witness).clone()]
        }
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
        ast::Term::Forget { atoms, span: s, .. } => ast::Term::Forget {
            atoms: atoms.clone(),
            value: Box::new(mk(&names[0])),
            span: *s,
        },
        ast::Term::Narrow { target, span: s, .. } => ast::Term::Narrow {
            value: Box::new(mk(&names[0])),
            target: target.clone(),
            witness: Box::new(mk(&names[1])),
            span: *s,
        },
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
struct ProofPatternScope {
    /// Frozen before introducing any row-local pattern binders.
    goal: PTerm,
    equalities: HashMap<LocalId, ITerm>,
    values: HashSet<LocalId>,
}

#[derive(Clone)]
struct Row {
    proof: Option<ProofPatternScope>,
    pats: Vec<ast::Pat>,
    guard: Option<ast::Term>,
    guard_locals: Vec<ast::LocalDecl>,
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

/// The method call's dictionary want behind an elaborated callee: the
/// want's hole under the method projection (possibly applied to the
/// instance's implicit arguments).
fn callee_want(core: &ECore) -> Option<usize> {
    match core {
        ECore::Hole(id) => Some(*id),
        ECore::DictProj(h, ..) => callee_want(h),
        ECore::App(f, args) => {
            callee_want(f).or_else(|| args.iter().find_map(callee_want))
        }
        _ => None,
    }
}

/// The want and method index when an elaborated call's head is a
/// method projected straight from its dictionary hole (`x - y`,
/// possibly with the method's own implicit arguments applied): the one
/// call form whose arguments are the method's own arguments.
fn head_method(core: &ECore) -> Option<(usize, usize)> {
    match core {
        ECore::App(f, _) => head_method(f),
        ECore::DictProj(h, idx, ..) => match &**h {
            ECore::Hole(w) => Some((*w, *idx)),
            _ => None,
        },
        _ => None,
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
        for c in n.chars().rev() {
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
    /// A value member: its fixpoint box and the value's type.
    Val(LocalId, Ty),
}

/// Initializers with observable evaluation stay in source order. A closed
/// effect row alone is insufficient: mutable projections, reference writes,
/// and opaque calls can observe state independently of row effects.
fn rec_initializer_stable(core: &ECore, partial: &impl Fn(LocalId, usize) -> bool) -> bool {
    let stable = |node: &ECore| rec_initializer_stable(node, partial);
    match core {
        ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_)
        | ECore::ConstString { .. } | ECore::ConstSym(_) | ECore::Lit(_)
        | ECore::Bool(_) | ECore::Unit | ECore::Var(_)
        | ECore::Ctor(..) | ECore::Prim(_) | ECore::Lam { .. }
        | ECore::LazyAlloc | ECore::BoxAlloc => true,
        ECore::LitFold { body, .. } => stable(body),
        ECore::Proj(value, field, _) => !field.mutable && stable(value),
        ECore::App(head, args) => {
            let known_stable = match &**head {
                ECore::Ctor(..) => true,
                ECore::Prim(op) => op.is_pure_arith(),
                ECore::Var(id) => partial(*id, args.len()),
                _ => false,
            };
            known_stable && args.iter().all(stable)
        }
        ECore::Let(_, value, body) => stable(value) && stable(body),
        ECore::If(condition, yes, no) => {
            stable(condition) && stable(&yes.body) && stable(&no.body)
        }
        ECore::Case { scrut, alts } => {
            stable(scrut) && alts.iter().all(|alt| stable(&alt.body))
        }
        ECore::MkClos { env, .. } => env.iter().all(stable),
        _ => false,
    }
}

struct RecursiveBindings {
    bindings: Vec<(LocalId, ECore)>,
    memo: Option<MemoGroup>,
    knots: Vec<KnotSlot>,
    functions: HashMap<GlobalId, (Ty, usize)>,
}

struct KnotSlot {
    local: LocalId,
    name: String,
    ty: Ty,
}

/// Track the scoped slot carried by a value, including hidden closure
/// captures. One witness suffices: every tracked slot ends at this scope.
struct KnotFlow<'a> {
    checker: &'a Checker,
    roots: &'a [KnotSlot],
    functions: &'a HashMap<GlobalId, (Ty, usize)>,
    locals: HashMap<LocalId, usize>,
    visiting: HashSet<GlobalId>,
    initialized: HashSet<LocalId>,
    suspended: bool,
    span: ast::Span,
}

impl KnotFlow<'_> {
    fn reject(&self, captured: Option<usize>) -> Result<()> {
        if let Some(index) = captured {
            return Err(Error::at(self.span, format!(
                "region escape: a value retains recursive binding `{}`",
                self.roots[index].name
            )));
        }
        Ok(())
    }

    fn in_type(&self, captured: Option<usize>, ty: &Ty) -> Option<usize> {
        captured.filter(|_| self.checker.value_may_hide_scope(ty, &mut Vec::new(), 32))
    }

    fn bind(&mut self, local: LocalId, captured: Option<usize>) {
        let captured = match self.checker.local_tys.get(&local) {
            Some(ty) => self.in_type(captured, ty),
            None => captured,
        };
        if let Some(slot) = captured {
            self.locals.insert(local, slot);
        }
    }

    fn values(&mut self, nodes: &[ECore]) -> Result<Option<usize>> {
        let mut captured = None;
        for node in nodes {
            captured = captured.or(self.value(node)?);
        }
        Ok(captured)
    }

    fn term_type(&self, node: &ECore) -> Option<Ty> {
        let ty = match node {
            ECore::Var(local) => self.checker.local_tys.get(local)?,
            ECore::MkClos { fn_id, .. } | ECore::Global(fn_id, _) => &self.functions.get(fn_id)?.0,
            ECore::Proj(_, _, ty) | ECore::DictProj(.., ty) | ECore::Demand(_, ty)
            | ECore::BoxGet(_, ty) | ECore::BoxInit(_, _, ty) => ty,
            _ => return None,
        };
        Some(self.checker.uni.zonk(ty))
    }

    // A closure retains its captures, but the result of calling it only
    // retains them if its checked return type can carry such a value.
    fn call_result(
        &mut self,
        captured: Option<usize>,
        callee: &ECore,
        arguments: usize,
    ) -> Result<Option<usize>> {
        if captured.is_none() {
            return Ok(None);
        }
        if let ECore::Lam { params, body } = callee {
            if params.len() == arguments {
                return self.value(body);
            }
        }
        let Some(mut ty) = self.term_type(callee) else { return Ok(captured) };
        for _ in 0..arguments {
            ty = match ty {
                Ty::Fun(_, _, _, cod) | Ty::Pi { cod, .. } => *cod,
                _ => return Ok(captured),
            };
        }
        Ok(self.in_type(captured, &ty))
    }

    fn force_result(&mut self, captured: Option<usize>, args: &[ECore]) -> Result<Option<usize>> {
        let [thunk] = args else { return Ok(captured) };
        match self.term_type(thunk) {
            Some(Ty::Thunk { inner, .. } | Ty::Later(inner)) => return Ok(self.in_type(captured, &inner)),
            _ => {}
        }
        if let ECore::App(head, args) = thunk {
            if let (ECore::Prim(PrimOp::Delay), [callback]) = (&**head, args.as_slice()) {
                return self.call_result(captured, callback, 1);
            }
        }
        Ok(captured)
    }

    fn check_suspended(&mut self, body: &ECore) -> Result<()> {
        let previous = std::mem::replace(&mut self.suspended, true);
        let result = self.value(body);
        self.suspended = previous;
        result.map(|_| ())
    }

    fn check_recursive_body(&mut self, id: &GlobalId, captured: Option<usize>) -> Result<()> {
        if captured.is_some() && self.visiting.insert(id.clone()) {
            if let Some((_, body)) = self.checker.pending_rec.iter().find(|(candidate, _)| candidate == id) {
                self.check_suspended(body)?;
            }
            self.visiting.remove(id);
        }
        Ok(())
    }

    fn value(&mut self, node: &ECore) -> Result<Option<usize>> {
        Ok(match node {
            ECore::Var(local) => self.locals.get(local).copied(),
            ECore::LitFold { body, .. } => self.value(body)?,
            ECore::Let(local, value, body) => {
                let captured = self.value(value)?;
                self.bind(*local, captured);
                let result = self.value(body)?;
                self.locals.remove(local);
                result
            }
            ECore::BoxGet(slot, _) if !self.suspended && matches!(&**slot, ECore::Var(local)
                if self.roots.iter().any(|root| root.local == *local) && !self.initialized.contains(local)) => {
                // The runtime faults before an uninitialized slot yields a value.
                None
            }
            ECore::Proj(value, _, ty) | ECore::DictProj(value, .., ty)
            | ECore::Demand(value, ty) | ECore::BoxGet(value, ty) => {
                let captured = self.value(value)?;
                self.in_type(captured, ty)
            }
            ECore::If(condition, yes, no) => {
                self.value(condition)?;
                self.value(&yes.body)?.or(self.value(&no.body)?)
            }
            ECore::Case { scrut, alts } => {
                let scrut = self.value(scrut)?;
                let mut captured = None;
                for alt in alts {
                    let fields = match &alt.pat {
                        CPat::Con { fields, .. } => fields.as_slice(),
                        CPat::Var(local, _) => std::slice::from_ref(local),
                        CPat::Wild => &[],
                    };
                    for &field in fields {
                        self.bind(field, scrut);
                    }
                    captured = captured.or(self.value(&alt.body)?);
                    for field in fields {
                        self.locals.remove(field);
                    }
                }
                captured
            }
            ECore::Lam { params, body } => {
                let mut bound = params.clone();
                let mut free = Vec::new();
                free_locals(body, &mut bound, &mut free);
                self.check_suspended(body)?;
                free.iter().find_map(|local| self.locals.get(local).copied())
            }
            // A generalisation body is built at dictionary construction:
            // it is suspended exactly as a lambda body is.
            ECore::PolyIntro { body, .. } => {
                let mut free = Vec::new();
                free_locals(body, &mut Vec::new(), &mut free);
                self.check_suspended(body)?;
                free.iter().find_map(|local| self.locals.get(local).copied())
            }
            ECore::MkClos { fn_id, env } => {
                let captured = self.values(env)?;
                self.check_recursive_body(fn_id, captured)?;
                captured
            }
            ECore::App(head, args) => {
                if let ECore::Global(id, _) = &**head {
                    if let Some((_, count)) = self.functions.get(id) {
                        // Direct local calls receive the lifted environment
                        // ahead of their source arguments.
                        let (captures, arguments) = args.split_at(*count);
                        let retained = self.values(captures)?;
                        let supplied = self.values(arguments)?;
                        self.reject(supplied)?;
                        self.check_recursive_body(id, retained)?;
                        return self.call_result(retained, head, arguments.len());
                    }
                }
                let callee = self.value(head)?;
                let arguments = self.values(args)?;
                match &**head {
                    ECore::Ctor(id, ..) if id.0 != crate::core::MUTABLE_CELL => arguments,
                    ECore::Prim(PrimOp::Delay) => arguments,
                    ECore::Prim(PrimOp::Force) => self.force_result(arguments, args)?,
                    ECore::Prim(op) if op.is_pure_arith() => None,
                    _ => {
                        // The callee may retain an argument, and a partial
                        // application retains its callee.
                        self.reject(arguments)?;
                        self.call_result(callee, head, args.len())?
                    }
                }
            }
            ECore::BoxInit(slot, payload, _) => {
                let captured = self.value(payload)?;
                if self.roots.iter().any(|root| root.local == *slot) {
                    self.initialized.insert(*slot);
                } else {
                    self.reject(captured)?;
                }
                captured
            }
            ECore::BoxClear(slot, _) => {
                self.value(slot)?;
                None
            }
            ECore::LazyInit(_, value) | ECore::RefSet(_, value, _) => {
                let captured = self.value(value)?;
                self.reject(captured)?;
                None
            }
            ECore::InstDict(_, values, _, _) | ECore::DeclsQuote { holes: values, .. } => {
                let captured = self.values(values)?;
                self.reject(captured)?;
                None
            }
            ECore::Quote { holes, body, .. } => {
                let captured = self.values(holes)?.or(self.value(body)?);
                self.reject(captured)?;
                None
            }
            ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_)
            | ECore::ConstString { .. } | ECore::ConstSym(_) | ECore::Lit(_)
            | ECore::Bool(_) | ECore::Unit | ECore::Global(..) | ECore::Ctor(..)
            | ECore::Prim(_) | ECore::Foreign(_) | ECore::LazyAlloc | ECore::BoxAlloc
            | ECore::Fault | ECore::Hole(_) | ECore::QuoteHole(_) | ECore::IdxPending(_)
            | ECore::SyntaxLit(..) | ECore::StrLit(_) => None,
        })
    }
}

#[derive(Clone, Copy)]
struct RecUse {
    owner: usize,
    target: usize,
    guard_depth: usize,
    applied_arguments: Option<usize>,
}

/// Follow a group alias when an initializer applies a local function.
fn rec_callable_member(
    mut member: usize,
    cores: &[ECore],
    members: &HashMap<LocalId, usize>,
) -> Option<(usize, usize)> {
    let mut arguments = 0usize;
    for _ in 0..cores.len() {
        match &cores[member] {
            ECore::Var(alias) => member = *members.get(alias)?,
            ECore::App(head, bound) => {
                let ECore::Var(alias) = &**head else { return None };
                member = *members.get(alias)?;
                arguments = arguments.saturating_add(bound.len());
            }
            ECore::Lam { params, .. } if arguments < params.len() => {
                return Some((member, params.len() - arguments));
            }
            _ => return None,
        }
    }
    None
}

/// The guarded-letrec rule (surface grammar local amendments section 9):
/// a use of a group name that closes a dependency cycle sits under a
/// lambda, a `~` memo body, or a `delay`. An eager cycle is a compile
/// error naming the variable and the missing guard.
fn order_rec_initializers(
    members: &[(LocalId, ast::Span, String)],
    cores: &[ECore],
    effectful: &[bool],
    functions: &[bool],
) -> Result<(Vec<usize>, Vec<bool>)> {
    let mut by_local: HashMap<LocalId, usize> = HashMap::new();
    for (i, (l, _, _)) in members.iter().enumerate() {
        by_local.insert(*l, i);
    }
    let partial = |id, arguments| {
        by_local.get(&id)
            .and_then(|&member| rec_callable_member(member, cores, &by_local))
            .is_some_and(|(_, remaining)| arguments < remaining)
    };
    let ordered: Vec<_> = cores.iter().zip(effectful)
        .map(|(core, has_effects)| *has_effects || !rec_initializer_stable(core, &partial))
        .collect();
    let mut edges = Vec::new();
    for (owner, core) in cores.iter().enumerate() {
        rec_walk(core, 0, owner, &by_local, &mut edges);
    }
    let n = members.len();
    let mut reach = vec![vec![false; n]; n];
    for edge in &edges {
        reach[edge.owner][edge.target] = true;
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
    for edge in &edges {
        if edge.guard_depth == 0 && (edge.owner == edge.target || reach[edge.target][edge.owner]) {
            let name = &members[edge.target].2;
            return Err(Error::at(
                members[edge.owner].1,
                format!(
                    "unguarded recursion; wrap the use in `delay` or a lambda (the use of `{name}` closes an eager cycle in the binding group)"
                ),
            ));
        }
    }

    // A saturated call or memo demand enters its body during initialization.
    // Merely storing a closure may capture a box whose value comes later.
    let mut dependencies = vec![vec![false; n]; n];
    for (owner, required) in dependencies.iter_mut().enumerate() {
        let mut pending: Vec<_> = edges.iter().copied()
            .filter(|edge| edge.owner == owner && edge.guard_depth == 0)
            .collect();
        let mut entered = HashSet::new();
        while let Some(edge) = pending.pop() {
            required[edge.target] = true;
            if let Some(arguments) = edge.applied_arguments {
                if let Some((callee, remaining)) = rec_callable_member(edge.target, cores, &by_local) {
                    if arguments >= remaining && entered.insert(callee) {
                        pending.extend(edges.iter().copied()
                            .filter(|edge| edge.owner == callee && edge.guard_depth == 1));
                    }
                }
            }
        }
    }
    let mut initialized = vec![false; n];
    let mut order = Vec::with_capacity(n);
    while let Some(first) = initialized.iter().position(|done| !done) {
        let next_ordered = (first..n).find(|&i| !initialized[i] && ordered[i]);
        let ready = (first..n).find(|&i| {
            !initialized[i]
                && (!ordered[i] || next_ordered == Some(i))
                && dependencies[i]
                    .iter()
                    .zip(&initialized)
                    .all(|(needed, done)| !*needed || *done)
        });
        let Some(next) = ready else {
            return Err(Error::at(
                members[first].1,
                "recursive initializers cannot satisfy their dependencies in source evaluation order",
            ));
        };
        initialized[next] = true;
        order.push(next);
    }
    // Lifted functions can be materialized by any initializer that
    // references them. Propagate the earliest such site through their
    // capture chains, including mutually recursive functions.
    let mut position = vec![0; n];
    for (at, &member) in order.iter().enumerate() {
        position[member] = at;
    }
    let mut earliest = position.clone();
    loop {
        let mut changed = false;
        for edge in &edges {
            if functions[edge.target] && earliest[edge.owner] < earliest[edge.target] {
                earliest[edge.target] = earliest[edge.owner];
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let mut forward_captures = vec![false; n];
    for edge in &edges {
        if edge.guard_depth > 0 && earliest[edge.owner] <= position[edge.target] {
            forward_captures[edge.target] = true;
        }
    }
    Ok((order, forward_captures))
}

/// Collect reads and applications of group members. Each lambda or delay
/// records another suspension around the use.
fn rec_walk(
    e: &ECore,
    guard: usize,
    owner: usize,
    by_local: &HashMap<LocalId, usize>,
    edges: &mut Vec<RecUse>,
) {
    let mut record = |id, applied_arguments| {
        if let Some(&target) = by_local.get(id) {
            edges.push(RecUse { owner, target, guard_depth: guard, applied_arguments });
        }
    };
    match e {
        ECore::LitFold { body, .. } => rec_walk(body, guard, owner, by_local, edges),
        ECore::Var(l) => record(l, None),
        ECore::Int(_)
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Global(..)
        | ECore::Ctor(..)
        | ECore::Prim(_)
        | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::LazyAlloc
        | ECore::BoxAlloc
        | ECore::SyntaxLit(..)
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
        ECore::Demand(cell, _) => match &**cell {
            ECore::Var(id) => record(id, Some(1)),
            other => rec_walk(other, guard, owner, by_local, edges),
        },
        ECore::DictProj(d, ..) | ECore::Proj(d, ..) | ECore::BoxGet(d, _) | ECore::BoxClear(d, _) => {
            rec_walk(d, guard, owner, by_local, edges);
        }
        ECore::App(f, args) => {
            match &**f {
                ECore::Var(id) => record(id, Some(args.len())),
                other => rec_walk(other, guard, owner, by_local, edges),
            }
            // A `delay`'s argument is one of the suspension positions.
            let bump = if matches!(**f, ECore::Prim(PrimOp::Delay)) { 1 } else { 0 };
            for a in args {
                rec_walk(a, guard + bump, owner, by_local, edges);
            }
        }
        ECore::InstDict(_, args, _, _) | ECore::MkClos { env: args, .. } => {
            for a in args {
                rec_walk(a, guard, owner, by_local, edges);
            }
        }
        // A lambda body is a suspension position; a `~` memo body is one
        // because its thunk is a lambda.
        ECore::Lam { body, .. } | ECore::PolyIntro { body, .. } => rec_walk(body, guard + 1, owner, by_local, edges),
        ECore::Let(_, v, body) => {
            rec_walk(v, guard, owner, by_local, edges);
            rec_walk(body, guard, owner, by_local, edges);
        }
        ECore::BoxInit(_, v, _) | ECore::LazyInit(_, v) | ECore::RefSet(_, v, _) => {
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
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Global(..)
        | ECore::Ctor(..)
        | ECore::Prim(_)
        | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::LazyAlloc
        | ECore::BoxAlloc
        | ECore::SyntaxLit(..)
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
        ECore::DictProj(d, ..) | ECore::Proj(d, ..) | ECore::Demand(d, _) | ECore::BoxGet(d, _) | ECore::BoxClear(d, _) => {
            rec_collect_refs(d, by_local, out)
        }
        ECore::App(f, args) => {
            rec_collect_refs(f, by_local, out);
            for a in args {
                rec_collect_refs(a, by_local, out);
            }
        }
        ECore::InstDict(_, args, _, _) | ECore::MkClos { env: args, .. } => {
            for a in args {
                rec_collect_refs(a, by_local, out);
            }
        }
        ECore::Lam { body, .. } | ECore::PolyIntro { body, .. } => rec_collect_refs(body, by_local, out),
        ECore::Let(_, v, body) => {
            rec_collect_refs(v, by_local, out);
            rec_collect_refs(body, by_local, out);
        }
        ECore::BoxInit(_, v, _) | ECore::LazyInit(_, v) | ECore::RefSet(_, v, _) => {
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

/// The top-level counterpart of [`rec_walk`] (the D302 check): every
/// use of a top-level name in one recorded binding's body is an
/// `(owner index, referenced, guarded)` edge, where `guard` counts the
/// suspension positions entered exactly as the guarded-letrec rule
/// counts them (lambda bodies and `delay` arguments). The owner is the
/// recorded binding's index into the checker's `top_binds`.
fn top_rec_walk(e: &ECore, guard: i32, owner: usize, edges: &mut Vec<(usize, GlobalId, bool)>) {
    match e {
        ECore::LitFold { body, .. } => top_rec_walk(body, guard, owner, edges),
        ECore::Global(g, _) => edges.push((owner, g.clone(), guard >= 1)),
        ECore::Var(_)
        | ECore::Int(_)
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Ctor(..)
        | ECore::Prim(_)
        | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::LazyAlloc
        | ECore::BoxAlloc
        | ECore::SyntaxLit(..)
        | ECore::StrLit(_) => {}
        ECore::Quote { holes, body, .. } => {
            for h in holes {
                top_rec_walk(h, guard, owner, edges);
            }
            top_rec_walk(body, guard, owner, edges);
        }
        ECore::DeclsQuote { holes, .. } => {
            for h in holes {
                top_rec_walk(h, guard, owner, edges);
            }
        }
        ECore::DictProj(d, ..) | ECore::Proj(d, ..) | ECore::Demand(d, _) | ECore::BoxGet(d, _) | ECore::BoxClear(d, _) => {
            top_rec_walk(d, guard, owner, edges);
        }
        ECore::App(f, args) => {
            top_rec_walk(f, guard, owner, edges);
            // A `delay`'s argument is one of the suspension positions.
            let bump = if matches!(**f, ECore::Prim(PrimOp::Delay)) { 1 } else { 0 };
            for a in args {
                top_rec_walk(a, guard + bump, owner, edges);
            }
        }
        ECore::InstDict(_, args, _, _) | ECore::MkClos { env: args, .. } => {
            for a in args {
                top_rec_walk(a, guard, owner, edges);
            }
        }
        // A lambda body is a suspension position; a `~` memo body is one
        // because its thunk is a lambda.
        ECore::Lam { body, .. } | ECore::PolyIntro { body, .. } => top_rec_walk(body, guard + 1, owner, edges),
        ECore::Let(_, v, body) => {
            top_rec_walk(v, guard, owner, edges);
            top_rec_walk(body, guard, owner, edges);
        }
        ECore::BoxInit(_, v, _) | ECore::LazyInit(_, v) | ECore::RefSet(_, v, _) => {
            top_rec_walk(v, guard, owner, edges)
        }
        ECore::If(c, t, f) => {
            top_rec_walk(c, guard, owner, edges);
            top_rec_walk(&t.body, guard, owner, edges);
            top_rec_walk(&f.body, guard, owner, edges);
        }
        ECore::Case { scrut, alts } => {
            top_rec_walk(scrut, guard, owner, edges);
            for alt in alts {
                top_rec_walk(&alt.body, guard, owner, edges);
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
                *e = ECore::App(Box::new(ECore::Global(id, Vec::new())), new_args);
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
            Some(RecSub::Val(b, ty)) => {
                let (b, ty) = (*b, ty.clone());
                *e = ECore::BoxGet(Box::new(ECore::Var(b)), ty);
            }
            None => {}
        },
        ECore::Int(_)
        | ECore::ConstBig { .. } | ECore::ConstBytes(_) | ECore::ConstString { .. } | ECore::ConstSym(_)
        | ECore::Lit(_)
        | ECore::Bool(_)
        | ECore::Unit
        | ECore::Global(..)
        | ECore::Ctor(..)
        | ECore::Prim(_)
        | ECore::Foreign(_)
        | ECore::Hole(_)
        | ECore::QuoteHole(_)
        | ECore::IdxPending(_)
        | ECore::Fault
        | ECore::LazyAlloc
        | ECore::BoxAlloc
        | ECore::SyntaxLit(..)
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
        ECore::DictProj(d, ..) | ECore::Proj(d, ..) | ECore::Demand(d, _) | ECore::BoxGet(d, _) | ECore::BoxClear(d, _) => {
            subst_rec_refs(d, subs)
        }
        ECore::App(f, args) => {
            subst_rec_refs(f, subs);
            for a in args {
                subst_rec_refs(a, subs);
            }
        }
        ECore::InstDict(_, args, _, _) | ECore::MkClos { env: args, .. } => {
            for a in args {
                subst_rec_refs(a, subs);
            }
        }
        ECore::Lam { body, .. } | ECore::PolyIntro { body, .. } => subst_rec_refs(body, subs),
        ECore::Let(_, v, body) => {
            subst_rec_refs(v, subs);
            subst_rec_refs(body, subs);
        }
        ECore::BoxInit(_, v, _) | ECore::LazyInit(_, v) | ECore::RefSet(_, v, _) => {
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

impl Checker {
    // Numeric result contracts are scoped before a let introduces names.
    // Rebase their free value names to surviving aliases or internal local
    // identities; a replacement source spelling is never the old value.
    fn numeric_contract_in_value_scope(&self, ty: &Ty, source: &Env, current: &Env) -> Ty {
        if !self.int_view(&self.uni.zonk(ty)).is_some_and(|(_, refinement)| refinement.is_some()) { return ty.clone() }
        let mut out = ty.clone(); let mut names = std::collections::BTreeSet::new(); let mut replacements = HashMap::new();
        for (name, id, _) in source.locals.iter().rev() {
            if !names.insert(name.clone()) || current.lookup(name).map(|(active, _)| active) == Some(*id) { continue }
            let original = self.value_aliases.get(id).copied().unwrap_or(*id);
            let visible = current.locals.iter().rev().find(|(name, local, _)| current.lookup(name).is_some_and(|(active, _)| active == *local) && self.value_aliases.get(local).copied().unwrap_or(*local) == original).map(|(name, _, _)| name.clone());
            out = out.subst_idx(name, &ITerm::LocalValue(original));
            if let Some(name) = visible { replacements.insert(original, ITerm::Var(name.into())); }
        }
        out.subst_local_values(&replacements)
    }
}

#[cfg(test)]
mod float_integer_exactness_tests {
    use super::float_exact_digits;

    #[test]
    fn decimal_exactness_matches_independent_binary_significands() {
        let mut coefficients: Vec<u128> = (0_u128..256).map(|n| 5 * (2 * n + 1)).collect();
        for precision in [24_u32, 53] {
            for delta in -70_i128..=70 {
                coefficients.push(((1_i128 << precision) + delta) as u128);
            }
        }
        for coefficient in coefficients {
            for shift in [0_u32, 8, 31, 63, 100, 121] {
                let Some(value) = coefficient.checked_mul(1_u128 << shift) else { continue };
                let odd = value >> value.trailing_zeros();
                let significant_bits = 128 - odd.leading_zeros();
                let exponent = (127 - value.leading_zeros()) as i32;
                for (precision, maximum_exponent) in [(24, 127), (53, 1023)] {
                    let expected = significant_bits <= precision && exponent <= maximum_exponent;
                    assert_eq!(float_exact_digits(&value.to_string(), precision, maximum_exponent), expected,
                        "decimal {value}, precision {precision}, exponent limit {maximum_exponent}");
                }
            }
        }
        assert!(float_exact_digits("0", 24, 127));
        assert!(float_exact_digits("340282346638528859811704183484516925440", 24, 127));
        assert!(!float_exact_digits("340282366920938463463374607431768211456", 24, 127));
        assert!(float_exact_digits("179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368", 53, 1023));
        assert!(!float_exact_digits("179769313486231590772930519078902473361797697894230657273430081157732675805500963132708477322407536021120113879871393357658789768814416622492847430639474124377767893424865485276302219601246094119453082952085005768838150682342462881473913110540827237163350510684586298239947245938479716304835356329624224137216", 53, 1023));
    }
}

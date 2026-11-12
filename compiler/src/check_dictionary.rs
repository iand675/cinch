// Dictionary selection with an explicit inference store, reusable by index checks.
// Each resolver owns its inference store; term elaboration commits it explicitly.
struct DictionaryTransfer {
    instance: usize,
    obligs: Vec<PropOblig>,
    container: Option<GlobalId>,
    wanted: ClassApp,
    subdicts: Vec<(String, Vec<Ty>)>,
    span: ast::Span,
}

struct DictionaryResolver<'a> {
    checker: &'a Checker,
    uni: Unifier,
    transfers: Vec<DictionaryTransfer>,
}

impl DictionaryResolver<'_> {
    fn literal_metas(&self) -> Vec<EitherMeta> {
        let mut result = Vec::new();
        for pending in &self.checker.literal_pends {
            for meta in self.uni.free_metas(&pending.ty) {
                if !result.contains(&meta) {
                    result.push(meta);
                }
            }
        }
        result
    }

    fn lexical<B: Clone>(&mut self, wanted: &ClassApp, scope: &[(B, ClassApp)]) -> Option<DictExpr<B>> {
        let wanted_z = self.uni.zonk_class(wanted);
        for (local, binder) in scope.iter().rev() {
            let binder_z = self.uni.zonk_class(binder);
            if binder_z.class == wanted_z.class {
                let trial = self.uni.snapshot();
                let ok = self.uni.unify(
                    &Ty::Adt(binder_z.class.clone().into(), binder_z.args.clone()),
                    &Ty::Adt(wanted_z.class.clone().into(), wanted_z.args.clone()),
                );
                if ok.is_ok() {
                    self.uni.commit(trial);
                    return Some(DictExpr::Binder(local.clone()));
                }
                self.uni.rollback_to(trial);
            }
        }
        None
    }

    fn solve<B: Clone + From<LocalId>>(
        &mut self,
        wanted: &ClassApp,
        scope: &[(B, ClassApp)],
        depth: usize,
        span: ast::Span,
    ) -> Result<SolveOutcome<B>> {
        if depth > 16 {
            return Err(Error::at(
                span,
                "dictionary resolution too deep (recursive instance?)",
            ));
        }
        let wanted_z = self.uni.zonk_class(wanted);
        // The same lexical matcher serves term calls and law telescopes.
        if let Some(dictionary) = self.lexical(&wanted_z, scope) { return Ok(Solved(dictionary)); }
        // Runtime capabilities are lexical evidence only: there are no
        // `Io` instances, and nothing global is searched (decision D21).
        if is_capability_class(&wanted_z.class) {
            return Err(Error::at(
                span,
                crate::diagnostic!(
                    "no {} capability in scope; supply one lexically with `with (io = ...) in` or `f @{{io = ...}}`, or take it in a `using` clause",
                    wanted_z
                ),
            ));
        }
        let class = wanted_z.class.clone();
        // The `Copy` class is compiler-known with structural instances
        // (ownership and usage design, section 3); there are no user
        // instances and its dictionary is a marker.
        if class == "Copy" {
            if let [t] = wanted_z.args.as_slice() {
                if !self.uni.free_metas(t).is_empty() {
                    // An undetermined literal may still fix the type.
                    return Ok(Ambiguous);
                }
                return if self.checker.copy_structural(t, scope, &mut Vec::new()) {
                    Ok(Solved(DictExpr::Marker))
                } else {
                    Err(Error::at(
                        span,
                        crate::diagnostic!(
                            "no Copy evidence for {} (copying needs a Copy type or a `using (copy :: Copy a)` constraint)",
                            t
                        ),
                    ))
                };
            }
            return Err(Error::at(
                span,
                crate::diagnostic!("Copy expects exactly one type argument, got {}", wanted_z),
            ));
        }
        // Collection literals default their container when nothing else
        // determines it (surface grammar local amendments section 10):
        // `[..]` unconstrained is a `List`, `{..}` a `Map`.
        if class == crate::known::FROM_LIST && wanted_z.args.len() == 2 {
            if matches!(self.uni.zonk(&wanted_z.args[0]), Ty::Meta(_)) {
                let list = Ty::Adt(crate::known::LIST.into(), vec![wanted_z.args[1].clone()]);
                self.uni
                    .unify(&wanted_z.args[0], &list)
                    .map_err(|m| Error::at(span, m))?;
            }
        } else if class == crate::known::FROM_MAP && wanted_z.args.len() == 3 {
            if matches!(self.uni.zonk(&wanted_z.args[0]), Ty::Meta(_)) {
                let map = Ty::Adt(
                    crate::known::MAP.into(),
                    vec![wanted_z.args[1].clone(), wanted_z.args[2].clone()],
                );
                self.uni
                    .unify(&wanted_z.args[0], &map)
                    .map_err(|m| Error::at(span, m))?;
            }
        }
        let wanted_z = self.uni.zonk_class(wanted);
        let wanted_app = Ty::Adt(class.clone().into(), wanted_z.args.clone());
        let mut matches: Vec<MatchCand> = Vec::new();
        // Candidate trials run inside `outer`. Every trial is rolled back
        // after its candidate is measured, so between trials the store only
        // gains the candidates' fresh instance parameters. The counters
        // let the winner be replayed on exactly the store its trial saw.
        let outer = self.uni.snapshot();
        let (mut fresh_types, mut fresh_props) = (0usize, 0usize);
        // The searched bucket is a dependency (design/compilation-speed.md
        // section 3, "negative lookups are dependencies"): record it before
        // the trials so a search that finds nothing is recorded exactly
        // like one that finds a candidate. The head comes from the wanted
        // class application; an undetermined head records `*`, which makes
        // the dependency coarser rather than absent.
        let bucket_head = wanted_z
            .args
            .first()
            .map(|t| self.uni.zonk(t).to_string())
            .and_then(|t| crate::driver::query::head_symbol(&t))
            .unwrap_or_else(|| "*".to_string());
        self.checker.record_instance_search(&class, &bucket_head);
        if let Some(idxs) = self.checker.resolution_class_indices(&class) {
            for i in idxs {
                if !self.checker.resolution_instances()[i].implicit && Some(i) != self.checker.self_instance
                {
                    continue;
                }
                // A parameterized module's declared instance needs its
                // module's parameters: it resolves only inside that module,
                // where the parameters are the definition's leading
                // dictionary binders (module parameters as module
                // arguments). Each instantiation publishes its own copy.
                if self.checker.resolution_instances()[i].base
                    && self.checker.resolution_instances()[i].module != self.checker.current_module
                    && Some(i) != self.checker.self_instance
                {
                    continue;
                }
                let inst = &self.checker.resolution_instances()[i];
                // One-way match: the instance's parameters become fresh
                // metavariables and the target is unified with the wanted
                // class application in a trial scope, so a candidate that
                // fails leaves no bindings. A wanted metavariable is
                // determined by the unique matching instance, which is what
                // fixes literal types (primitive types specification,
                // section 7). Predicate parameters (`refined p T`'s `p`)
                // become proposition metavariables and bind by proposition
                // unification in the lattice match.
                let prop_params = Checker::prop_param_names(&inst.params, &inst.target);
                let mut fresh: HashMap<String, Ty> = HashMap::new();
                let fresh_before = (fresh_types, fresh_props);
                for p in &inst.params {
                    fresh.insert(
                        p.clone(),
                        if prop_params.contains(p) {
                            fresh_props += 1;
                            self.uni.fresh_prop()
                        } else {
                            fresh_types += 1;
                            self.uni.fresh()
                        },
                    );
                }
                let target = Checker::subst_class(&inst.target, &fresh);
                let trial = self.uni.snapshot();
                let target_app = Ty::Adt(class.clone().into(), target.args.clone());
                // Structural matching first (the ordinary case), then the
                // refinement lattice: a head at `refined h` serves every
                // want whose predicate implies `h` (the value weakens to
                // the head), and a plain head serves refined wants (the
                // container's methods at the point).
                let matched = self.uni.unify(&target_app, &wanted_app).is_ok()
                    || self
                        .checker
                        .lattice_match(&target.args, &wanted_z.args, &mut self.uni);
                if !matched {
                    self.uni.rollback_to(trial);
                    continue;
                }
                // A candidate whose target leaves an instance parameter
                // bound only to an UNDETERMINED metavariable is not
                // applicable to THIS want; it is not a malformed
                // instance. `Eq (Down a)` structurally matches a want
                // `Eq ?x` (from an unannotated literal, say) by binding
                // `?x := Down ?a`, which determines nothing, and the
                // wanted type is meant to come from the context (literal
                // defaulting, D7) rather than from an instance chosen
                // here. Skipping the candidate lets a later candidate or
                // the deferral below handle the want; erroring here
                // aborted the whole obligation as soon as any instance
                // with a parameter sat earlier in the family than the
                // nullary instance that used to answer it.
                let mut sub = HashMap::new();
                let mut determined = true;
                for p in &inst.params {
                    let t = self.uni.zonk(&fresh[p]);
                    if t == fresh[p] {
                        determined = false;
                        break;
                    }
                    sub.insert(p.clone(), t);
                }
                if !determined {
                    self.uni.rollback_to(trial);
                    continue;
                }
                let mut cond = self.checker.head_cond(&target.args, &self.uni);
                for item in &inst.ctx {
                    if let InstCtx::Oblig(PropOblig::Pred(p)) = item {
                        // A constraint-form instance's applicability
                        // condition is its obligation set (ordered by the
                        // same implication lattice).
                        let p = Checker::subst_pt(p, &sub);
                        let p = self.uni.resolve_prop(&p);
                        cond = Some(match cond {
                            None => p,
                            Some(q) => PTerm::And(Box::new(q), Box::new(p)),
                        });
                    }
                }
                self.uni.rollback_to(trial);
                matches.push(MatchCand {
                    idx: i,
                    sub,
                    fresh,
                    fresh_before,
                    cond,
                    generic_head: !prop_params.is_empty(),
                });
            }
        }
        if matches.is_empty() {
            self.uni.commit(outer);
            return Err(Error::at(
                span,
                crate::diagnostic!(
                    "no instance for {}{}",
                    wanted_z,
                    undetermined_hint(&self.uni, &wanted_z)
                ),
            ));
        }
        // Most-specific lattice resolution (refinement types design,
        // "instance heads at refinements"): among the candidates whose
        // heads the want matches, the strongest head condition wins (the
        // implication lattice); a predicate-parameterized head (the
        // generic transfer) loses a tie to a stated head. Incomparable
        // strongest candidates have no winner: the coherence error.
        let stronger = |a: &Option<PTerm>, b: &Option<PTerm>| {
            Checker::cond_implies(a, b) && !Checker::cond_implies(b, a)
        };
        let maximal: Vec<usize> = (0..matches.len())
            .filter(|&c| {
                !(0..matches.len()).any(|o| o != c && stronger(&matches[o].cond, &matches[c].cond))
            })
            .collect();
        // The maximal candidates group into strength-equivalence classes
        // (mutually implying head conditions); classes are pairwise
        // incomparable.
        let mut classes: Vec<Vec<usize>> = Vec::new();
        for &c in &maximal {
            match classes
                .iter_mut()
                .find(|cl| Checker::cond_implies(&matches[c].cond, &matches[cl[0]].cond))
            {
                Some(cl) => cl.push(c),
                None => classes.push(vec![c]),
            }
        }
        let ground = self.uni.free_metas(&wanted_app).is_empty();
        let ambiguous = if classes.len() > 1 {
            if !ground {
                self.uni.commit(outer);
                return Ok(Ambiguous);
            }
            // Incomparable strongest heads at a common refinement: no
            // winner (the coherence rule, D119 extended).
            let a = &self.checker.resolution_instances()[matches[classes[0][0]].idx];
            let b = &self.checker.resolution_instances()[matches[classes[1][0]].idx];
            let refinement = wanted_z
                .args
                .iter()
                .find_map(|t| self.checker.refine_view(t))
                .map(|v| v.pred);
            let point: &dyn crate::spelling::Spell = match &refinement {
                Some(pred) => pred,
                None => &wanted_z,
            };
            self.uni.commit(outer);
            return Err(Error::at(
                span,
                crate::diagnostic!(
                    "coherence: instances `{}` (head {}) and `{}` (head {}) are incomparable constrained instances, and both match this refinement ({}) (the coherence rule: a use at a common refinement needs one most-specific instance; state one head with a predicate implying the other)",
                    a.name, a.target, b.name, b.target, point
                ),
            ));
        } else {
            // One strongest class: a stated head beats the generic
            // transfer (most-specific, like typeclass instance selection);
            // unresolved ties defer to the fixpoint like any
            // underdetermined want.
            let class = &classes[0];
            let stated: Vec<usize> = class
                .iter()
                .copied()
                .filter(|&c| !matches[c].generic_head)
                .collect();
            match stated.as_slice() {
                [only] => Some(*only),
                [] if class.len() == 1 => Some(class[0]),
                _ => None,
            }
        };
        let pick = match ambiguous {
            Some(p) => p,
            None => {
                self.uni.commit(outer);
                return Ok(Ambiguous);
            }
        };
        let cand = &matches[pick];
        let (idx, sub) = (cand.idx, cand.sub.clone());
        // A unique match that leaves an instance parameter bound
        // only to an undetermined literal defers to literal
        // defaulting (decision D7): `[1]` with no other constraint
        // is a `List Int`, not an unsolved literal type.
        let lits = self.literal_metas();
        let undetermined = sub
            .values()
            .any(|t| matches!(t, Ty::Meta(m) if lits.contains(&EitherMeta::Type(*m))));
        if undetermined {
            self.uni.commit(outer);
            return Ok(Ambiguous);
        }
        let (ctx, base_id, base) = {
            let inst = &self.checker.resolution_instances()[idx];
            (inst.ctx.clone(), inst.base_id.clone(), inst.base)
        };
        // Replay the winner: drop every allocation and binding since
        // `outer`, allocate as many instance parameters as existed when
        // the winner's trial ran (its own last, in the same order, so they
        // get the same ids), and redo its deterministic match.
        self.uni.rollback_to(outer);
        let (types_before, props_before) = cand.fresh_before;
        for _ in 0..types_before {
            self.uni.fresh();
        }
        for _ in 0..props_before {
            self.uni.fresh_prop();
        }
        {
            let inst = &self.checker.resolution_instances()[idx];
            let prop_params = Checker::prop_param_names(&inst.params, &inst.target);
            for p in &inst.params {
                let meta = if prop_params.contains(p) { self.uni.fresh_prop() } else { self.uni.fresh() };
                debug_assert!(meta == cand.fresh[p], "replayed instance parameter ids match the trial's");
            }
            let target = Checker::subst_class(&inst.target, &cand.fresh);
            let target_app = Ty::Adt(class.clone().into(), target.args.clone());
            let matched = self.uni.unify(&target_app, &wanted_app).is_ok()
                || self.checker.lattice_match(&target.args, &wanted_z.args, &mut self.uni);
            debug_assert!(matched, "replaying the winning candidate's match succeeds");
        }
        let mut subs = Vec::new();
        // The module's own instance dictionary closes over the
        // module's parameters (the definition's leading binders).
        if base {
            for (_, l, _) in &self.checker.cur_mod_params {
                subs.push(DictExpr::Binder(B::from(*l)));
            }
        }
        let mut obligs: Vec<PropOblig> = Vec::new();
        let mut container: Option<GlobalId> = None;
        let mut subdicts: Vec<(String, Vec<Ty>)> = Vec::new();
        for item in &ctx {
            let c = match item {
                InstCtx::Dict(_, c) => Checker::subst_class(c, &sub),
                InstCtx::Oblig(o) => {
                    obligs.push(Checker::subst_oblig(o, &sub));
                    continue;
                }
            };
            match self.solve(&c, scope, depth + 1, span)? {
                Solved(d) => {
                    // A sub-dictionary is applied inside the
                    // outer instance's methods, never at a call
                    // the caller sees: an instance whose methods
                    // carry obligations over their own arguments
                    // cannot serve one.
                    if let DictExpr::Instance(id, _, _) = &d {
                        if let Some(si) = self
                            .checker
                            .resolution_instances()
                            .iter()
                            .find(|i| i.base_id == *id && !i.method_obligs.is_empty())
                        {
                            let cz = self.uni.zonk_class(&c);
                            return Err(Error::at(
                                span,
                                Checker::method_oblig_refusal(si, &cz),
                            ));
                        }
                    }
                    // The generic transfer's rep-identity
                    // dictionary is the container's: the same-class
                    // sub-dictionary names its owner for the law
                    // rows.
                    if container.is_none() && c.class == class {
                        if let DictExpr::Instance(id, _, _) = &d {
                            container = Some(id.clone());
                        }
                    }
                    subdicts.push((c.class.clone(), c.args.clone()));
                    subs.push(d);
                }
                Ambiguous => {
                    let cz = self.uni.zonk_class(&c);
                    return Err(Error::at(
                        span,
                        crate::diagnostic!(
                            "ambiguous instances for {}{}",
                            cz,
                            undetermined_hint(&self.uni, &cz)
                        ),
                    ));
                }
            }
        }
        let inst_info = self.checker.resolution_instances()[idx].clone();
        self.transfers.push(DictionaryTransfer {
            instance: idx,
            obligs,
            container,
            wanted: wanted.clone(),
            subdicts,
            span,
        });
        // The dictionary's class vector ranges over the instance's
        // own parameters, as the match bound them.
        let tys: Vec<Ty> = inst_info
            .params
            .iter()
            .map(|p| sub.get(p).cloned().unwrap_or(Ty::Unit))
            .collect();
        Ok(Solved(DictExpr::Instance(base_id, subs, tys)))
    }
}

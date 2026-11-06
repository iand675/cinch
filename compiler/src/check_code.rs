// Native expectations for staged Code. A wrapper is an ordinary quotation
// over one code hole; it never rewrites the captured body's names or grants
// authority to a spelling, and adds no field to the runtime Code ABI.
#[derive(Clone, Default)]
struct StageTypeBindings {
    types: HashMap<String, Ty>,
    rows: HashMap<String, RowTail>,
    regions: HashMap<String, String>,
    evidence: HashMap<IndexEvidence, ITerm>,
}

#[derive(Clone)]
struct StageCall {
    target: GlobalId,
    arguments: StageTypeBindings,
}

impl Checker {
    /// Internal Code annotations do not turn an operation head into a
    /// first-class value. Validate their full carrier once, then dispatch
    /// with that same instantiation and the ordinary handler machinery.
    fn annotated_operation(
        &mut self,
        env: &mut Env,
        head: &ast::Term,
    ) -> Result<Option<(ValueInfo, Ty)>> {
        let mut term = head;
        let mut annotations = Vec::new();
        while let ast::Term::Ann(body, annotation @ ast::Type::Elaborated(..), _) = term {
            annotations.push(annotation);
            term = body;
        }
        if annotations.is_empty() {
            return Ok(None);
        }
        let ast::Term::Var(path) = term else {
            return Ok(None);
        };
        if path.quals.is_empty() && env.lookup_name(&path.name).is_some() {
            return Ok(None);
        }
        let Ok(operation) = self.resolve_var(path, path.span()) else {
            return Ok(None);
        };
        if !matches!(operation.kind, ValueKind::EffectOp { .. }) {
            return Ok(None);
        }
        let (ty, context, _) = self.instantiate(&operation.scheme);
        if !context.is_empty() {
            return Err(Error::at(
                head.span(),
                format!(
                    "effect operation {} takes no class constraints in the bootstrap slice",
                    operation.name
                ),
            ));
        }
        for annotation in annotations {
            let expected = self.annotation_type(env, annotation)?;
            self.uni
                .unify(&ty, &expected)
                .map_err(|message| Error::at(head.span(), message))?;
        }
        self.record_editor_reference(
            path.span(),
            ast::RefKind::Value,
            &operation.module,
            &operation.name,
        );
        Ok(Some((operation, self.uni.zonk(&ty))))
    }

    /// A private interpreter reference, allocated only after ordinary resolution
    /// and instantiation. Its spelling cannot resolve a source name or acquire
    /// authority. Runtime references retain their original code generation unit.
    fn stage_reference(
        &self,
        value: &ValueInfo,
        types: Vec<Ty>,
        arguments: StageTypeBindings,
        span: ast::Span,
    ) -> Result<ECore> {
        let core = self.fn_ref(value, types, span)?;
        let mut calls = self.stage_calls.borrow_mut();
        let id = GlobalId(format!("<checked-stage-call:{}>", calls.len()));
        calls.insert(
            id.0.clone(),
            StageCall {
                target: fn_id(&value.module, &value.name),
                arguments,
            },
        );
        fn reference(core: ECore, id: &GlobalId) -> ECore {
            match core {
                ECore::Global(_, types) => ECore::Global(id.clone(), types),
                ECore::App(head, args) => ECore::App(Box::new(reference(*head, id)), args),
                _ => unreachable!(
                    "fn_ref constructs a global reference with optional module arguments"
                ),
            }
        }
        Ok(reference(core, &id))
    }

    fn stage_invocation(
        &self,
        cx: &mut StageCx,
        target: GlobalId,
        types: StageTypeBindings,
    ) -> GlobalId {
        cx.fresh += 1;
        let id = GlobalId(format!("<stage-invocation:{}>", cx.fresh));
        cx.invocations.insert(id.0.clone(), (target, types));
        id
    }

    /// Fresh local inference variables belong to one interpreter invocation,
    /// not to one annotation and not to the compiled function shared by calls.
    fn stage_resolve_type(&mut self, cx: &mut StageCx, ty: &Ty) -> Ty {
        let (ty, parameters, rows) = self.uni.generalize_for_code(ty);
        for name in parameters {
            cx.code_types
                .types
                .entry(name)
                .or_insert_with(|| self.uni.fresh());
        }
        for name in rows {
            cx.code_types
                .rows
                .entry(name)
                .or_insert_with(|| self.uni.fresh_row().tail);
        }
        self.uni.zonk(&Self::subst_namespaces(
            &ty,
            &cx.code_types.types,
            &cx.code_types.rows,
            &cx.code_types.regions,
            &cx.code_types.evidence,
        ))
    }

    fn stage_call_bindings(
        &mut self,
        cx: &mut StageCx,
        call: &StageCall,
    ) -> Result<StageTypeBindings> {
        let Some((source, native)) = self.stage_definition_scopes.get(&call.target.0).cloned()
        else {
            return Ok(StageTypeBindings::default());
        };
        let mut result = StageTypeBindings::default();
        for (formal, rigid) in source.params.iter().zip(&native.params) {
            if let Some(actual) = call.arguments.types.get(formal) {
                result
                    .types
                    .insert(rigid.clone(), self.stage_resolve_type(cx, actual));
            }
        }
        for formal in source
            .idx_params
            .iter()
            .map(IxParam::name)
            .chain(source.val_params.iter().map(|(name, _)| name.as_str()))
            .chain(source.prop_params.iter().map(String::as_str))
        {
            if let Some(actual) = call.arguments.types.get(formal) {
                result
                    .types
                    .insert(formal.to_string(), self.stage_resolve_type(cx, actual));
            }
        }
        for ((formal, _), rigid) in source.ctx.iter().zip(&native.evidence_params) {
            if let Some(actual) = call.arguments.evidence.get(&dictionary_parameter(formal)) {
                let Ty::Idx(actual) = self.stage_resolve_type(cx, &Ty::Idx(Box::new(actual.clone()))) else { unreachable!() };
                result.evidence.insert(rigid.clone(), *actual);
            }
        }
        for formal in &source.evidence_params {
            if let Some(actual) = call.arguments.evidence.get(formal) {
                let Ty::Idx(actual) = self.stage_resolve_type(cx, &Ty::Idx(Box::new(actual.clone()))) else { unreachable!() };
                result.evidence.insert(formal.clone(), *actual);
            }
        }
        for (formal, rigid) in source.row_params.iter().zip(&native.row_params) {
            if let Some(tail) = call.arguments.rows.get(formal) {
                let row = EffRow {
                    labels: Vec::new(),
                    tail: tail.clone(),
                };
                let carrier = Ty::Fun(Box::new(Ty::Unit), Mult::Flex, row, Box::new(Ty::Unit));
                let Ty::Fun(_, _, actual, _) = self.stage_resolve_type(cx, &carrier) else {
                    unreachable!()
                };
                let fresh = self.uni.fresh_row();
                self.uni.unify_rows(&fresh, &actual).map_err(Error::plain)?;
                result.rows.insert(rigid.clone(), fresh.tail);
            }
        }
        for (formal, rigid) in source.region_params.iter().zip(&native.region_params) {
            if let Some(actual) = call.arguments.regions.get(formal) {
                let actual = self.uni.resolve_region(actual);
                result.regions.insert(
                    rigid.clone(),
                    cx.code_types
                        .regions
                        .get(&actual)
                        .cloned()
                        .unwrap_or(actual),
                );
            }
        }
        Ok(result)
    }

    fn fill_stage_scope(&mut self, types: &mut StageTypeBindings, scope: &Scheme) {
        for parameter in &scope.evidence_params {
            types.evidence.entry(parameter.clone()).or_insert_with(|| self.uni.fresh_idx_term());
        }
        for name in &scope.params {
            types
                .types
                .entry(name.clone())
                .or_insert_with(|| self.uni.fresh());
        }
        for parameter in &scope.idx_params {
            types
                .types
                .entry(parameter.name().to_string())
                .or_insert_with(|| match parameter {
                    IxParam::Val(_, Ty::Sort(Sort::Type(_))) => self.uni.fresh(),
                    IxParam::Val(_, Ty::Sort(Sort::Prop)) | IxParam::Prop(..) => {
                        self.uni.fresh_prop()
                    }
                    IxParam::Val(..) => self.uni.fresh_idx(),
                });
        }
        for name in &scope.row_params {
            types
                .rows
                .entry(name.clone())
                .or_insert_with(|| self.uni.fresh_row().tail);
        }
        for name in &scope.region_params {
            if !types.regions.contains_key(name) {
                let fresh = self.fresh_param(name);
                self.uni.note_flex_region(&fresh);
                types.regions.insert(name.clone(), fresh);
            }
        }
    }

    fn instantiate_code_template(&mut self, cx: &mut StageCx, template: &ast::Term) -> ast::Term {
        let mut template = template.clone();
        if let ast::Term::Ann(_, ast::Type::Elaborated(scope, _), _) = &mut template {
            self.fill_stage_scope(&mut cx.code_types, scope);
            scope.ty = self.stage_resolve_type(cx, &scope.ty);
            // The instantiated native type retains shared inference identities.
            // Re-quantifying here would let every insertion choose another type.
            scope.params.clear();
            scope.evidence_params.clear();
            scope.con_params.clear();
            scope.idx_params.clear();
            scope.row_params.clear();
            scope.region_params.clear();
        }
        template
    }

    fn code_scope(source: &Scheme) -> Scheme {
        let mut scope = source.clone();
        if source.evidence_params.is_empty() {
            scope.evidence_params.extend(source.ctx.iter().map(|(name, _)| dictionary_parameter(name)));
        }
        scope.idx_params.extend(
            source
                .val_params
                .iter()
                .map(|(name, ty)| IxParam::Val(name.clone(), ty.clone())),
        );
        scope
            .idx_params
            .extend(source.prop_params.iter().cloned().map(IxParam::Prop));
        scope.val_params.clear();
        scope.prop_params.clear();
        // This schema describes the payload's quantified type, not the
        // producer's function context. The original function scheme and call
        // elaboration retain that context. Payload predicates stay in ty.
        scope.ctx.clear();
        scope.obligs.clear();
        scope
    }

    fn code_expectation(
        &self,
        core: ECore,
        ty: &Ty,
        span: ast::Span,
        scope: Option<Scheme>,
    ) -> ECore {
        let Some(scope) = scope else { return core };
        let mut scope = Self::code_scope(&scope);
        let Ty::Adt(name, arguments) = self.uni.zonk(ty) else {
            return core;
        };
        if name != "Code" || arguments.len() != 1 {
            return core;
        }
        scope.ty = arguments[0].clone();
        ECore::Quote {
            template: Box::new(ast::Term::Ann(
                Box::new(ast::Term::SpliceHole { idx: 0, span }),
                ast::Type::Elaborated(Box::new(scope), span),
                span,
            )),
            holes: vec![core],
            body: Box::new(ECore::QuoteHole(0)),
        }
    }

    fn lower_code_expectation(&self, template: &ast::Term) -> Box<ast::Term> {
        let mut template = template.clone();
        if let ast::Term::Ann(_, ast::Type::Elaborated(scope, _), _) = &mut template {
            let (ty, parameters, rows) = self.uni.generalize_for_code(&scope.ty);
            scope.ty = ty;
            scope.params.extend(parameters);
            scope.row_params.extend(rows);
        }
        Box::new(template)
    }

    /// Validate the source's universal contract before instantiating the
    /// checked body. Instantiating the signature first would accept a Bool
    /// implementation of `forall a. a -> a` at a Bool-only use site.
    fn infer_forall_annotation(
        &mut self, env: &mut Env, body: &ast::Term, annotation: &ast::Type,
    ) -> Result<(ECore, Ty)> {
        let scheme = self.conv_scheme_in_env(env, annotation)?;
        if !scheme.ctx.is_empty() || !scheme.obligs.is_empty() || !scheme.evidence_params.is_empty()
            || !scheme.idx_params.is_empty() || !scheme.val_params.is_empty() || !scheme.prop_params.is_empty()
        {
            return Err(Error::at(annotation.span(),
                "quantified term annotations with evidence or runtime index parameters are not yet supported"));
        }
        let previous = self.code_quantifiers.clone();
        let (rigid, _, bindings) = self.rigidify(&scheme);
        let previous_annotations = self.body_annotation_scope.clone();
        for name in bindings.types.keys().chain(bindings.rows.keys()).chain(bindings.regions.keys()) {
            self.body_annotation_scope.types.remove(name);
            self.body_annotation_scope.rows.remove(name);
            self.body_annotation_scope.regions.remove(name);
        }
        self.body_annotation_scope.types.extend(bindings.types.clone());
        self.body_annotation_scope.rows.extend(bindings.rows.clone());
        self.body_annotation_scope.regions.extend(bindings.regions.clone());
        let recursion_start = self.pending_rec.len();
        let checked = self.check(env, body, &rigid);
        self.code_quantifiers = previous;
        self.body_annotation_scope = previous_annotations;
        let checked = checked?;
        let recursion_end = self.pending_rec.len();
        let ((_, _, _, _), instantiated) = self.instantiate_scoped(&scheme);
        let mut replacement = StageTypeBindings::default();
        for (name, original) in &bindings.types {
            if let Ty::Param(rigid) = original {
                replacement.types.insert(rigid.clone(), instantiated.types[name].clone());
            }
        }
        for (name, original) in &bindings.rows {
            if let RowTail::Rigid(rigid) = original {
                replacement.rows.insert(rigid.clone(), instantiated.rows[name].clone());
            }
        }
        for (name, rigid) in &bindings.regions {
            replacement.regions.insert(rigid.clone(), instantiated.regions[name].clone());
        }
        // A signature's skolems belong to its body, never to a captured
        // outer value or dictionary. Reject the usual escape through an
        // outer inference variable before returning an instantiated copy.
        let escapes = |ty: &Ty| {
            let ty = self.uni.zonk(ty);
            Self::subst_namespaces(&ty, &replacement.types, &replacement.rows,
                &replacement.regions, &replacement.evidence) != ty
        };
        if env.locals.iter().any(|(_, _, scope)| escapes(&scope.ty))
            || env.dicts.iter().any(|(_, class)| class.args.iter().any(&escapes))
        {
            return Err(Error::at(annotation.span(), "quantified annotation parameter escapes into a captured binding"));
        }
        let (ty, core) = self.copy_checked_definition(&checked, &HashMap::new(), &replacement, &rigid);
        // The rigid validation body is a template, not an additional
        // runtime unit. Only its instantiated copies can be lowered with
        // concrete representation classes. Pre-existing captured units stay.
        self.pending_rec.drain(recursion_start..recursion_end);
        Ok((core, ty))
    }

    fn annotation_type(&mut self, env: &Env, annotation: &ast::Type) -> Result<Ty> {
        if let ast::Type::Elaborated(scope, span) = annotation {
            if !scope.ctx.is_empty() || !scope.obligs.is_empty() || !scope.val_params.is_empty() {
                return Err(Error::at(
                    *span,
                    "Code expectation has unchecked evidence or runtime arguments",
                ));
            }
            let (ty, _, _, _) = self.instantiate_at(scope);
            let ty = match ty {
                Ty::Prp(prop) => {
                    let source = env.locals.iter().map(|(name, local, _)| (name.clone(), *local)).collect();
                    Ty::Prp(Box::new(self.proof_values_in_scope(&prop, &source, env)))
                }
                other => other,
            };
            return Ok(ty);
        }
        let scope = &self.body_annotation_scope;
        let mut parameters: Vec<String> = scope.types.keys().chain(scope.rows.keys())
            .chain(scope.regions.keys()).cloned().collect();
        parameters.sort();
        parameters.dedup();
        let actual = self.conv_type_in_env(env, annotation, &parameters)?;
        Ok(Self::subst_namespaces(&actual, &scope.types, &scope.rows, &scope.regions, &scope.evidence))
    }

    /// Carry concrete usage constraints through internal inference arrows.
    /// Other type information stays the annotation's; ordinary subsumption
    /// subsequently checks it against the caller's full expected type.
    fn code_usage_hint(actual: &Ty, expected: &Ty) -> Ty {
        let mult = |a, e| if a == Mult::Flex { e } else { a };
        let pair = |a: &Ty, e: &Ty| Box::new(Self::code_usage_hint(a, e));
        let items = |a: &[Ty], e: &[Ty]| {
            a.iter()
                .zip(e)
                .map(|(a, e)| Self::code_usage_hint(a, e))
                .collect()
        };
        match (actual, expected) {
            (Ty::Fun(a, m, r, b), Ty::Fun(x, n, _, y)) => {
                Ty::Fun(pair(a, x), mult(*m, *n), r.clone(), pair(b, y))
            }
            (
                Ty::Pi {
                    var,
                    dom,
                    mult: m,
                    eff,
                    cod,
                },
                Ty::Pi {
                    dom: x,
                    mult: n,
                    cod: y,
                    ..
                },
            ) => Ty::Pi {
                var: var.clone(),
                dom: pair(dom, x),
                mult: mult(*m, *n),
                eff: eff.clone(),
                cod: pair(cod, y),
            },
            (Ty::Sigma { var, dom, cod }, Ty::Sigma { dom: x, cod: y, .. }) => Ty::Sigma {
                var: var.clone(),
                dom: pair(dom, x),
                cod: pair(cod, y),
            },
            (Ty::Adt(n, a), Ty::Adt(m, b)) if n == m && a.len() == b.len() => {
                Ty::Adt(n.clone(), items(a, b))
            }
            (Ty::App(h, a), Ty::App(g, b)) if a.len() == b.len() => {
                Ty::App(pair(h, g), items(a, b))
            }
            (Ty::Tuple(a), Ty::Tuple(b)) if a.len() == b.len() => Ty::Tuple(items(a, b)),
            (Ty::Thunk { eff, inner }, Ty::Thunk { inner: b, .. }) => Ty::Thunk {
                eff: eff.clone(),
                inner: pair(inner, b),
            },
            (Ty::Later(a), Ty::Later(b)) => Ty::Later(pair(a, b)),
            (Ty::Lazy { eff, region, inner }, Ty::Lazy { inner: b, .. }) => Ty::Lazy {
                eff: eff.clone(),
                region: region.clone(),
                inner: pair(inner, b),
            },
            (Ty::Ref(r, a), Ty::Ref(_, b)) => Ty::Ref(r.clone(), pair(a, b)),
            (Ty::Mut(r, a), Ty::Mut(_, b)) => Ty::Mut(r.clone(), pair(a, b)),
            (Ty::Ptr { role, pointee }, Ty::Ptr { pointee: b, .. }) => Ty::Ptr {
                role: role.clone(),
                pointee: pair(pointee, b),
            },
            (
                Ty::Handler {
                    label,
                    input,
                    output,
                    eff,
                    resumption,
                },
                Ty::Handler {
                    input: a,
                    output: b,
                    resumption: m,
                    ..
                },
            ) => Ty::Handler {
                label: label.clone(),
                input: pair(input, a),
                output: pair(output, b),
                eff: eff.clone(),
                resumption: mult(*resumption, *m),
            },
            _ => actual.clone(),
        }
    }
}

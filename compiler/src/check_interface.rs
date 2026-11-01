// Definition-site contracts for rank-one values and class dictionaries.
// Kernel regions and closed builtin effects retain their checked syntax.
// Unsupported dependent and open-row schemes keep `resolved: null`.
// Display spellings must never be used to guess a nominal's definition.

#[derive(Clone)]
struct DefinitionScheme {
    text: String,
    references: std::collections::BTreeSet<(String, String)>,
    con_params: Vec<(String, usize)>,
}

impl Checker {
    // Inspect supported kind syntax without calling conversion again:
    // metadata capture must not add new native inference constraints.
    fn source_contract_kind(&self, kind: &ast::Type) -> Option<ParamKind> {
        match kind {
            ast::Type::Con(path, _) if path.parts().as_slice() == ["Type"] => Some(ParamKind::Type),
            ast::Type::Fun { arg, res, mult: None, eff: None, .. }
                if matches!(self.source_contract_kind(arg), Some(ParamKind::Type)) => {
                    match self.source_contract_kind(res)? {
                        ParamKind::Type => Some(ParamKind::Con(1)),
                        ParamKind::Con(n) => Some(ParamKind::Con(n + 1)),
                        _ => None,
                    }
                }
            _ => None,
        }
    }

    fn remember_interface_scheme(
        &mut self, module: &str, name: &str, source: &ast::Type,
        scheme: &Scheme, inherited: &[ast::TypeBinder],
    ) {
        self.remember_source_scheme(module, name, source, scheme, inherited, None, false);
    }

    fn remember_interface_constructor(
        &mut self, module: &str, name: &str, source: &ast::Type,
        scheme: &Scheme, inherited: &[ast::TypeBinder],
    ) {
        // A constructor-local binder may be existential. Publishing it as
        // an ordinary universal would let a pattern's witness escape.
        if matches!(source, ast::Type::Forall(..)) { return }
        self.remember_source_scheme(module, name, source, scheme, inherited, None, false);
    }

    fn remember_interface_instance(&mut self, source: &ast::InstDecl, info: &InstanceInfo, notes: &[(String, ParamKind)]) {
        // Conditional method preconditions and module arguments must not
        // disappear when an instance is imported as a dictionary factory.
        if info.base || !info.method_obligs.is_empty()
            || info.ctx.iter().any(|c| matches!(c, InstCtx::Oblig(_))) { return }
        let mut scheme = Scheme::mono(Ty::Adt(info.target.class.clone(), info.target.args.clone()));
        scheme.params = info.params.clone();
        scheme.ctx = InstCtx::dicts(&info.ctx);
        for parameter in &source.params {
            match self.resolve_binder_kind(&parameter.text, None, notes, parameter.span) {
                Ok(ParamKind::Type) => {},
                Ok(ParamKind::Con(arity)) => scheme.con_params.push((parameter.text.clone(), arity)),
                _ => return,
            }
        }
        let binders = source.params.iter().map(|name| ast::TypeBinder {
            name: name.clone(), ann: None, implicit: false,
        }).collect::<Vec<_>>();
        let ty = ast::Type::Using(source.ctx.clone(), Box::new(source.target.clone()), source.span);
        self.remember_source_scheme(&info.module, &format!("@instance:{}", info.name),
            &ty, &scheme, &binders, None, true);
    }

    fn remember_source_scheme(
        &mut self, module: &str, name: &str, source: &ast::Type,
        scheme: &Scheme, inherited: &[ast::TypeBinder], class: Option<&SigInfo>, class_target: bool,
    ) {
        if !scheme.idx_params.is_empty() || !scheme.val_params.is_empty()
            || !scheme.prop_params.is_empty() || !scheme.row_params.is_empty() { return }
        let mut references = std::collections::BTreeSet::new();
        let mut vars = Vec::new();
        let Some(mut prefix) = self.resolved_binders(inherited, scheme, &mut vars) else { return };
        // Regions inferred from a latent row need binders too, even when
        // the surface signature omitted an explicit forall (notably main).
        let mut explicit: Vec<String> = inherited.iter().map(|b| b.name.text.clone()).collect();
        let mut outer = source;
        loop {
            match outer {
                ast::Type::Forall(binders, body, _) => {
                    explicit.extend(binders.iter().map(|b| b.name.text.clone()));
                    outer = body;
                }
                ast::Type::Using(_, body, _) => outer = body,
                _ => break,
            }
        }
        for region in &scheme.region_params {
            if !explicit.contains(region) {
                prefix.push(format!("{{{region} :: Region}}"));
                vars.push(region.clone());
            }
        }
        let mut context = Vec::new();
        let mut predicate_count = 0;

        if let Some(class) = class {
            references.insert((class.module.clone(), class.name.clone()));
            let args = class.params.iter().map(|p| format!(" {p}")).collect::<String>();
            let mut binder = "dict".to_string();
            while scheme.ctx.iter().skip(1).any(|(name, _)| name == &binder) { binder.push('_'); }
            context.push(format!("{binder} :: {}.{}{args}", class.module, class.name));
        }
        // Hoist the scheme's outer binders and contexts into one forall.
        // A method's inherited parameters must not create a nested forall.
        let mut body = source;
        loop {
            match body {
                ast::Type::Forall(binders, next, _) => {
                    let Some(extra) = self.resolved_binders(binders, scheme, &mut vars) else { return };
                    prefix.extend(extra);
                    body = next;
                }
                ast::Type::Using(bounds, next, _) => {
                    for bound in bounds {
                        match bound {
                            ast::Oblig::Class(bound) => {
                                let Some(class) = self.resolved_source_constraint(bound, scheme, &vars, &mut references) else { return };
                                context.push(class);
                            }
                            ast::Oblig::Pred(_) => predicate_count += 1,
                            ast::Oblig::Preserves { .. } => return,
                        }
                    }
                    body = next;
                }
                _ => break,
            }
        }
        if context.len() != scheme.ctx.len() || predicate_count != scheme.obligs.len() { return }
        let Some(predicates) = self.resolved_numeric_preconditions(body, scheme) else { return };
        context.extend(predicates);
        let text = if class_target { self.resolved_source_class_type(body, scheme, &vars, &mut references) }
            else { self.resolved_source_type(body, scheme, &vars, &mut references) };
        let Some(mut text) = text else { return };
        if !context.is_empty() { text = format!("using ({}) => {text}", context.join(", ")); }
        if !prefix.is_empty() { text = format!("forall {}. {text}", prefix.join(" ")); }
        self.interface_source_schemes.insert((module.to_string(), name.to_string()), DefinitionScheme {
            text, references, con_params: scheme.con_params.clone(),
        });
    }

    fn resolved_source_constraint(
        &self, bound: &ast::Constraint, scheme: &Scheme, vars: &[String],
        references: &mut std::collections::BTreeSet<(String, String)>,
    ) -> Option<String> {
        let app = self.resolved_source_class_application(&bound.class, &bound.args, scheme, vars, references)?;
        Some(format!("{} :: {app}", bound.name.text))
    }

    fn resolved_source_class_type(
        &self, source: &ast::Type, scheme: &Scheme, vars: &[String],
        references: &mut std::collections::BTreeSet<(String, String)>,
    ) -> Option<String> {
        let mut head = source;
        let mut args = Vec::new();
        while let ast::Type::App(f, arg, _) = head { args.push((**arg).clone()); head = f; }
        args.reverse();
        let ast::Type::Con(class, _) = head else { return None };
        self.resolved_source_class_application(class, &args, scheme, vars, references)
    }

    fn resolved_source_class_application(
        &self, path: &ast::ConPath, arguments: &[ast::Type], scheme: &Scheme, vars: &[String],
        references: &mut std::collections::BTreeSet<(String, String)>,
    ) -> Option<String> {
        let class = self.lookup_sig(path).ok()?;
        if class.module == "<builtin>" {
            // Io is a kernel capability, not a nominal declaration in an
            // imaginary module. Region arguments must retain their kind.
            if class.name != "Io" || arguments.len() != 1 { return None }
            let ast::Type::Var(region, _) = &arguments[0] else { return None };
            return (vars.contains(&region.text) && scheme.region_params.contains(&region.text))
                .then(|| format!("Io {}", region.text));
        }
        if class.param_kinds.iter()
            .any(|k| !matches!(k, ParamKind::Type | ParamKind::Con(_))) { return None }
        references.insert((class.module.clone(), class.name.clone()));
        let args = arguments.iter().map(|ty| self.resolved_source_type(ty, scheme, vars, references))
            .collect::<Option<Vec<_>>>()?;
        let args = args.iter().map(|t| format!(" {t}")).collect::<String>();
        Some(format!("{}.{}{args}", class.module, class.name))
    }

    fn resolved_binders(&self, binders: &[ast::TypeBinder], scheme: &Scheme, vars: &mut Vec<String>) -> Option<Vec<String>> {
        let mut out = Vec::new();
        for binder in binders {
            let name = &binder.name.text;
            if vars.contains(name) { return None }
            // Explicit annotations were checked by the source checker;
            // inferred kinds come from its resulting scheme.
            let kind = if scheme.region_params.contains(name) { "Region".to_string() }
                else if scheme.params.contains(name) {
                    let arity = scheme.con_params.iter().find(|(n, _)| n == name).map(|(_, n)| *n).unwrap_or(0);
                    vec!["Type"; arity + 1].join(" -> ")
                } else { return None };
            out.push(if binder.implicit { format!("{{{name} :: {kind}}}") } else { format!("({name} :: {kind})") });
            vars.push(name.clone());
        }
        Some(out)
    }

    fn resolved_source_type(
        &self, source: &ast::Type, scheme: &Scheme, vars: &[String],
        references: &mut std::collections::BTreeSet<(String, String)>,
    ) -> Option<String> {
        Some(match source {
            ast::Type::Con(path, _) => {
                let parts = path.parts();
                let name = parts.join(".");
                if parts.len() == 1 && vars.contains(&name) { name }
                else if parts.as_slice() == ["Io"] { "Io".to_string() }
                else if parts.len() == 1 && (Scalar::parse(&name).is_some()
                    || matches!(name.as_str(), "Int" | "Nat" | "Bool" | "Unit" | "Bytes" | "Sym")) { name }
                else {
                    let info = self.lookup_type(&parts).ok()?;
                    // Kernel modalities and application units need richer
                    // identities than a source data/opaque declaration.
                    if info.module == "<builtin>" { return None }
                    references.insert((info.module.clone(), info.name.clone()));
                    format!("{}.{}", info.module, info.name)
                }
            }
            ast::Type::Var(name, _) if vars.contains(&name.text) => name.text.clone(),
            // This source signature was already checked by Native.
            // Literal index syntax is retained; the consumer checks its
            // actual argument kind and numerical carrier.
            ast::Type::Num(number, _) => number.clone(),
            ast::Type::App(f, x, _) => format!("({} {})",
                self.resolved_source_type(f, scheme, vars, references)?,
                self.resolved_source_type(x, scheme, vars, references)?),
            ast::Type::Fun { arg, res, mult: None, eff, .. } => {
                let effects = match eff {
                    None => String::new(),
                    Some(row) => self.resolved_kernel_effect_row(row, scheme, vars)?,
                };
                format!("({} ->{} {})", self.resolved_source_type(arg, scheme, vars, references)?,
                    effects, self.resolved_source_type(res, scheme, vars, references)?)
            },
            ast::Type::DepFun(name, domain, None, row, result, _) => {
                if vars.contains(&name.text) { return None }
                let domain = self.resolved_source_type(domain, scheme, vars, references)?;
                let mut bound = vars.to_vec();
                bound.push(name.text.clone());
                let effects = match row {
                    None => String::new(),
                    Some(row) => self.resolved_kernel_effect_row(row, scheme, &bound)?,
                };
                format!("(({} :: {}) ->{} {})", name.text, domain, effects,
                    self.resolved_source_type(result, scheme, &bound, references)?)
            }
            ast::Type::Unit(_) => "()".to_string(),
            ast::Type::Tuple(items, _) => format!("({})", items.iter()
                .map(|t| self.resolved_source_type(t, scheme, vars, references))
                .collect::<Option<Vec<_>>>()?.join(", ")),
            // Preserve the display contract, with an explicit refusal in
            // the consumer, rather than erasing an unsupported obligation.
            _ => return None,
        })
    }

    // Use the checked predicates, whose argument positions are normalized
    // to $aN by conv_scheme. Rebind only actual computational arguments.
    // Exporting a conditional signature must not guess from display text.
    fn resolved_numeric_preconditions(&self, mut source: &ast::Type, scheme: &Scheme) -> Option<Vec<String>> {
        let mut checked = &scheme.ty;
        let mut names: Vec<Option<String>> = Vec::new();
        let mut numeric: Vec<String> = Vec::new();
        loop {
            let (name, result) = match source {
                ast::Type::DepFun(name, _, None, _, result, _) => (Some(name.text.clone()), &**result),
                ast::Type::Fun { res, mult: None, .. } => (None, &**res),
                _ => break,
            };
            let (domain, codomain) = match checked {
                Ty::Pi { dom, cod, .. } => (&**dom, &**cod),
                Ty::Fun(dom, _, _, cod) => (&**dom, &**cod),
                _ => return None,
            };
            if let Some(name) = &name {
                if names.iter().any(|prior| prior.as_ref() == Some(name)) { return None }
                if matches!(domain, Ty::Nat | Ty::Int) { numeric.push(name.clone()); }
            }
            names.push(name);
            source = result;
            checked = codomain;
        }
        fn index(term: &ITerm, numeric: &[String]) -> bool {
            match term {
                ITerm::Var(name) => numeric.contains(name),
                ITerm::Num(_) => true,
                ITerm::Bin(_, a, b) => index(a, numeric) && index(b, numeric),
                _ => false,
            }
        }
        fn predicate(term: &PTerm, numeric: &[String]) -> bool {
            match term {
                PTerm::Cmp(_, a, b) => index(a, numeric) && index(b, numeric),
                // Preserve the checked structural proposition itself.
                // Calls and other structural vocabularies still need
                // their own operand contracts; a spelling is not proof.
                PTerm::Pred(PredFn::NonZero, value) => index(value, numeric),
                PTerm::And(a, b) | PTerm::Or(a, b) => predicate(a, numeric) && predicate(b, numeric),
                _ => false,
            }
        }
        let mut out = Vec::new();
        for obligation in &scheme.obligs {
            let PropOblig::Pred(original) = obligation else { return None };
            let mut rebound = original.clone();
            for (position, name) in names.iter().enumerate() {
                if let Some(name) = name {
                    rebound = rebound.subst(&format!("$a{position}"), &ITerm::Var(name.clone()));
                }
            }
            if !predicate(&rebound, &numeric) { return None }
            out.push(rebound.to_string());
        }
        Some(out)
    }

    fn resolved_kernel_effect_row(&self, row: &ast::EffectRow, scheme: &Scheme, vars: &[String]) -> Option<String> {
        if row.tail.is_some() { return None }
        let mut labels = Vec::new();
        for (path, arguments) in &row.labels {
            let parts = path.parts();
            let [name] = parts.as_slice() else { return None };
            match (*name, arguments.as_slice()) {
                ("IO" | "Async" | "Cancel" | "Eval", [region])
                    if vars.contains(&region.text) && scheme.region_params.contains(&region.text) => {
                    labels.push(format!("{name} {}", region.text));
                }
                ("Diverge" | "Blocking", []) => labels.push((*name).to_string()),
                _ => return None,
            }
        }
        labels.sort();
        labels.dedup();
        Some(format!("{{{}}}", labels.join(", ")))
    }

    // A closed numeric template has only its actual declared parameters
    // and binder. Calls require a separate defining measure contract.
    fn resolved_refinement_template(&self, info: &TypeKind) -> Option<crate::interface::RefinementExport> {
        let r = info.refine.as_ref()?;
        let carrier = match &r.base {
            Ty::Nat | Ty::Int | Ty::Bool | Ty::Unit | Ty::Bytes | Ty::Sym | Ty::Prim(_) => r.base.to_string(),
            Ty::Param(name) if r.params.contains(name) => name.clone(),
            _ => return None,
        };
        let mut names = r.params.clone();
        names.push(r.binder.clone());
        fn index(value: &ITerm, names: &[String]) -> bool {
            match value {
                ITerm::Var(name) => names.contains(name),
                ITerm::Num(_) => true,
                ITerm::Bin(_, a, b) => index(a, names) && index(b, names),
                _ => false,
            }
        }
        fn predicate(value: &PTerm, names: &[String]) -> bool {
            match value {
                PTerm::Cmp(_, a, b) | PTerm::Eq(a, b) => index(a, names) && index(b, names),
                PTerm::And(a, b) | PTerm::Or(a, b) => predicate(a, names) && predicate(b, names),
                PTerm::Not(body) => predicate(body, names),
                PTerm::Pred(PredFn::NonZero, value) => index(value, names),
                _ => false,
            }
        }
        predicate(&r.pred, &names).then(|| crate::interface::RefinementExport {
            binder: r.binder.clone(), carrier, predicate: r.pred.to_string(),
        })
    }

    fn resolved_interface_scheme(&self, module: &str, name: &str) -> Option<crate::interface::ResolvedScheme> {
        let source = self.interface_source_schemes.get(&(module.to_string(), name.to_string()))?;
        let mut nominals = Vec::new();
        for (module, name) in &source.references {
            let info = self.types.get(name)?.iter().find(|t| &t.module == module)?;
            if info.fields.is_some() { return None }
            let refined = info.refine.is_some();
            if info.param_kinds.iter().any(|k| match k {
                ParamKind::Type | ParamKind::Con(_) => false,
                ParamKind::Val(ty) if refined => !matches!(ty, Ty::Nat | Ty::Int),
                _ => true,
            }) { return None }
            let refinement = if refined { Some(self.resolved_refinement_template(info)?) } else { None };
            let signature = self.qualified_sigs.contains_key(&(module.clone(), name.clone()));
            let params = info.refine.as_ref().map(|r| r.params.as_slice()).unwrap_or(&[]);
            let parameters = self.interface_parameters(module, name, params, &info.param_kinds);
            nominals.push(crate::interface::NominalContract {
                refinement,
                identity: format!("{module}.{name}"),
                kind: if refined { "refinement" } else if signature { "signature" } else if info.opaque { "opaque" } else { "data" }.to_string(),
                parameters,
                constructors: if refined || signature || info.opaque { None } else { Some(info.ctors.iter().map(|(c, _)| format!("{module}.{c}")).collect()) },
            });
        }
        Some(crate::interface::ResolvedScheme { scheme: source.text.clone(), nominals })
    }
}

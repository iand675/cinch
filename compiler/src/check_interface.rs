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
    // Portable observation terms use a closed kernel wire operation. It
    // is emitted only for the checked primitive identity, never a function
    // spelling, and only over actual buffer-typed binders in this scope.
    fn interface_carrier_is(&self, ty: &Ty, bytes: bool, seen: &mut Vec<Ty>) -> bool {
        let direct = if bytes { matches!(ty, Ty::Bytes) } else { matches!(ty, Ty::Nat | Ty::Int) };
        if direct { return true }
        if !matches!(ty, Ty::Adt(..)) || seen.contains(ty) { return false }
        seen.push(ty.clone());
        self.refine_view(ty).and_then(|v| v.base)
            .is_some_and(|base| self.interface_carrier_is(&base, bytes, seen))
    }

    fn portable_index(term: &ITerm, numeric: &[String], buffers: &[String]) -> Option<ITerm> {
        Some(match term {
            ITerm::Var(n) if numeric.contains(n) => term.clone(),
            ITerm::Num(_) => term.clone(),
            ITerm::Bin(op, a, b) => ITerm::Bin(*op,
                Box::new(Self::portable_index(a, numeric, buffers)?),
                Box::new(Self::portable_index(b, numeric, buffers)?)),
            ITerm::App(f, args) if matches!(&**f, ITerm::Fn(n) if n == BYTE_LENGTH_INDEX) => {
                let [ITerm::Var(n)] = args.as_slice() else { return None };
                if !buffers.contains(n) { return None }
                ITerm::app_fn("b.bytesLen", vec![ITerm::Var(n.clone())])
            }
            _ => return None,
        })
    }

    fn portable_predicate(term: &PTerm, numeric: &[String], buffers: &[String], negation: bool) -> Option<PTerm> {
        Some(match term {
            PTerm::Cmp(op, a, b) => PTerm::Cmp(*op,
                Box::new(Self::portable_index(a, numeric, buffers)?),
                Box::new(Self::portable_index(b, numeric, buffers)?)),
            PTerm::Eq(a, b) if negation => PTerm::Eq(
                Box::new(Self::portable_index(a, numeric, buffers)?),
                Box::new(Self::portable_index(b, numeric, buffers)?)),
            PTerm::And(a, b) => PTerm::And(
                Box::new(Self::portable_predicate(a, numeric, buffers, negation)?),
                Box::new(Self::portable_predicate(b, numeric, buffers, negation)?)),
            PTerm::Or(a, b) => PTerm::Or(
                Box::new(Self::portable_predicate(a, numeric, buffers, negation)?),
                Box::new(Self::portable_predicate(b, numeric, buffers, negation)?)),
            PTerm::Not(p) if negation => PTerm::Not(Box::new(Self::portable_predicate(p, numeric, buffers, true)?)),
            PTerm::Pred(PredFn::NonZero, value) => PTerm::Pred(PredFn::NonZero,
                Box::new(Self::portable_index(value, numeric, buffers)?)),
            _ => return None,
        })
    }

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
            else { self.resolved_checked_source_type(body, &scheme.ty, scheme, &vars, &mut references, &[]) };
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


    // Keep the original syntax's defining identities, but read anonymous
    // predicates from the checked type. Conversion must never run again
    // during interface emission: it can mutate inference and kind state.
    fn resolved_checked_source_type(
        &self, source: &ast::Type, checked: &Ty, scheme: &Scheme, vars: &[String],
        references: &mut std::collections::BTreeSet<(String, String)>,
        arguments: &[(Option<String>, Ty)],
    ) -> Option<String> {
        fn contains_refined(source: &ast::Type) -> bool {
            match source {
                ast::Type::Con(path, _) => path.parts().as_slice() == ["refined"],
                ast::Type::Var(name, _) => name.text == "refined",
                ast::Type::App(f, x, _) => contains_refined(f) || contains_refined(x),
                ast::Type::Fun { arg, res, .. } => contains_refined(arg) || contains_refined(res),
                ast::Type::DepFun(_, a, _, _, b, _) => contains_refined(a) || contains_refined(b),
                ast::Type::Tuple(items, _) => items.iter().any(contains_refined),
                _ => false,
            }
        }
        if !contains_refined(source) {
            return self.resolved_source_type(source, scheme, vars, references);
        }
        let mut head = source;
        let mut source_args = Vec::new();
        while let ast::Type::App(f, x, _) = head { source_args.push(&**x); head = f; }
        source_args.reverse();
        if matches!(head, ast::Type::Con(path, _) if path.parts().as_slice() == ["refined"])
            || matches!(head, ast::Type::Var(name, _) if name.text == "refined") {
            let Ty::Adt(name, args) = checked else { return None };
            let [Ty::Prp(original), carrier] = args.as_slice() else { return None };
            if name != "refined" || source_args.len() != 2 { return None }
            let carrier_text = self.resolved_checked_source_type(source_args[1], carrier, scheme, vars, references, arguments)?;
            let binder = crate::types::pred_binder(original);
            let mut free = Vec::new();
            original.free_vars(&mut free);
            let mut result = "cinch-result".to_string();
            while vars.contains(&result) || free.contains(&result)
                || arguments.iter().any(|(name, _)| name.as_ref() == Some(&result)) {
                result.push_str("-r");
            }
            let mut predicate = original.subst(&binder, &ITerm::Var(result.clone()));
            // A refined argument retains its contract at the boundary,
            // while its checked projection may still be mathematical.
            // Follow actual definition-site views, never a printed name,
            // and do not read through borrows or mutable byte contents.
            fn mathematical(checker: &Checker, ty: &Ty, seen: &mut Vec<Ty>) -> bool {
                if matches!(ty, Ty::Nat | Ty::Int) { return true }
                if !matches!(ty, Ty::Adt(..)) || seen.contains(ty) { return false }
                seen.push(ty.clone());
                checker.refine_view(ty).and_then(|view| view.base)
                    .is_some_and(|base| mathematical(checker, &base, seen))
            }
            let mut numeric = Vec::new();
            let mut buffers = Vec::new();
            if self.interface_carrier_is(carrier, true, &mut Vec::new()) { buffers.push(result.clone()); }
            if mathematical(self, carrier, &mut Vec::new()) { numeric.push(result); }
            for (position, (name, ty)) in arguments.iter().enumerate() {
                if let Some(name) = name {
                    predicate = predicate.subst(&format!("$a{position}"), &ITerm::Var(name.clone()));
                    if mathematical(self, ty, &mut Vec::new()) { numeric.push(name.clone()); }
                    if self.interface_carrier_is(ty, true, &mut Vec::new()) { buffers.push(name.clone()); }
                }
            }
            let predicate = Self::portable_predicate(&predicate, &numeric, &buffers, false)?;
            return Some(format!("(refined ({predicate}) {carrier_text})"));
        }
        match (source, checked) {
            (ast::Type::Fun { arg, res, mult: None, eff, .. }, Ty::Fun(domain, _, _, codomain)) => {
                let domain_text = self.resolved_checked_source_type(arg, domain, scheme, vars, references, arguments)?;
                let effects = match eff { None => String::new(), Some(row) => self.resolved_kernel_effect_row(row, scheme, vars)? };
                let mut next = arguments.to_vec();
                next.push((None, (**domain).clone()));
                let result = self.resolved_checked_source_type(res, codomain, scheme, vars, references, &next)?;
                Some(format!("({domain_text} ->{effects} {result})"))
            }
            (ast::Type::DepFun(name, domain, None, row, result, _), Ty::Fun(dom, _, _, cod) | Ty::Pi { dom, cod, .. }) => {
                if vars.contains(&name.text) { return None }
                let domain_text = self.resolved_checked_source_type(domain, dom, scheme, vars, references, arguments)?;
                let mut bound = vars.to_vec();
                bound.push(name.text.clone());
                let mut next = arguments.to_vec();
                next.push((Some(name.text.clone()), (**dom).clone()));
                let effects = match row { None => String::new(), Some(row) => self.resolved_kernel_effect_row(row, scheme, &bound)? };
                let result = self.resolved_checked_source_type(result, cod, scheme, &bound, references, &next)?;
                Some(format!("(({} :: {domain_text}) ->{effects} {result})", name.text))
            }
            (ast::Type::Tuple(items, _), Ty::Tuple(types)) if items.len() == types.len() => {
                let items = items.iter().zip(types).map(|(s, t)| self.resolved_checked_source_type(s, t, scheme, vars, references, arguments)).collect::<Option<Vec<_>>>()?;
                Some(format!("({})", items.join(", ")))
            }
            (ast::Type::App(..), Ty::Adt(_, types) | Ty::App(_, types)) if source_args.len() == types.len() => {
                let head = self.resolved_source_type(head, scheme, vars, references)?;
                let args = source_args.iter().zip(types).map(|(s, t)| self.resolved_checked_source_type(s, t, scheme, vars, references, arguments)).collect::<Option<Vec<_>>>()?;
                Some(format!("({head} {})", args.join(" ")))
            }
            _ => None,
        }
    }

    // Use the checked predicates, whose argument positions are normalized
    // to $aN by conv_scheme. Rebind only actual computational arguments.
    // Exporting a conditional signature must not guess from display text.
    fn resolved_numeric_preconditions(&self, mut source: &ast::Type, scheme: &Scheme) -> Option<Vec<String>> {
        let mut checked = &scheme.ty;
        let mut names: Vec<Option<String>> = Vec::new();
        let mut numeric: Vec<String> = Vec::new();
        let mut buffers: Vec<String> = Vec::new();
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
                if self.interface_carrier_is(domain, false, &mut Vec::new()) { numeric.push(name.clone()); }
                if self.interface_carrier_is(domain, true, &mut Vec::new()) { buffers.push(name.clone()); }
            }
            names.push(name);
            source = result;
            checked = codomain;
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
            out.push(Self::portable_predicate(&rebound, &numeric, &buffers, false)?.to_string());
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
        let mut numeric = Vec::new();
        let mut buffers = Vec::new();
        for (name, kind) in r.params.iter().zip(&info.param_kinds) {
            if let ParamKind::Val(ty) = kind {
                if self.interface_carrier_is(ty, false, &mut Vec::new()) { numeric.push(name.clone()); }
                if self.interface_carrier_is(ty, true, &mut Vec::new()) { buffers.push(name.clone()); }
            }
        }
        // A generic numeric template retains its historical parameter
        // carrier. It may not use that abstract carrier as a byte buffer.
        if matches!(&r.base, Ty::Param(n) if r.params.contains(n)) || self.interface_carrier_is(&r.base, false, &mut Vec::new()) {
            numeric.push(r.binder.clone());
        }
        if self.interface_carrier_is(&r.base, true, &mut Vec::new()) { buffers.push(r.binder.clone()); }
        Some(crate::interface::RefinementExport {
            binder: r.binder.clone(), carrier,
            predicate: Self::portable_predicate(&r.pred, &numeric, &buffers, true)?.to_string(),
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
                ParamKind::Val(ty) if refined => !matches!(ty, Ty::Nat | Ty::Int | Ty::Bytes),
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

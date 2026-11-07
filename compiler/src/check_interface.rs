// Definition-site contracts for rank-one values and class dictionaries.
// Kernel regions and closed builtin effects retain their checked syntax.
// Unsupported dependent and open-row schemes keep `resolved: null`.
// Display spellings must never be used to guess a nominal's definition.

#[derive(Clone)]
struct DefinitionScheme {
    text: String,
    references: std::collections::BTreeSet<(String, String)>,
    con_params: Vec<(String, usize)>,
    measure_calls: std::collections::BTreeSet<String>,
}

// Every portable predicate binder retains its actual checked carrier sort.
// Machine values may be compared as mathematical readings, but their
// arithmetic is an opaque computation and cannot be serialized as exact +/−/*.
#[derive(Clone, Copy, PartialEq, Eq)]
enum InterfaceIndexSort { Mathematical, Machine, Bytes }
#[derive(Default)]
struct InterfaceIndexScope {
    locals: std::collections::BTreeMap<String, InterfaceIndexSort>,
    propositions: std::collections::BTreeSet<String>,
    allow_measures: bool,
    measure_calls: std::cell::RefCell<std::collections::BTreeSet<String>>,
}

impl Checker {
    fn interface_index_sort(&self, ty: &Ty, seen: &mut Vec<Ty>) -> Option<InterfaceIndexSort> {
        use InterfaceIndexSort::*;
        let direct = match ty {
            Ty::Nat | Ty::Int => Some(Mathematical),
            Ty::Prim(Scalar::I8 | Scalar::I16 | Scalar::I32 | Scalar::I64
                | Scalar::U8 | Scalar::U16 | Scalar::U32 | Scalar::U64) => Some(Machine),
            Ty::Bytes => Some(Bytes),
            _ => None,
        };
        if direct.is_some() { return direct }
        if !matches!(ty, Ty::Adt(..)) || seen.contains(ty) { return None }
        seen.push(ty.clone());
        self.refine_view(ty).and_then(|v| v.base)
            .and_then(|base| self.interface_index_sort(&base, seen))
    }

    fn bind_interface_index(&self, scope: &mut InterfaceIndexScope, name: &str, ty: &Ty) {
        if let Some(sort) = self.interface_index_sort(ty, &mut Vec::new()) {
            scope.locals.insert(name.to_string(), sort);
        }
    }

    fn interface_measure_signature(&self, identity: &str) -> Option<(Vec<InterfaceIndexSort>, InterfaceIndexSort)> {
        let value = self.qualified.values().find(|value| fn_id(&value.module, &value.name).0 == identity)?;
        let sc = &value.scheme;
        if !sc.params.is_empty() || !sc.evidence_params.is_empty() || !sc.idx_params.is_empty() || !sc.val_params.is_empty()
            || !sc.prop_params.is_empty() || !sc.region_params.is_empty() || !sc.row_params.is_empty()
            || !sc.ctx.is_empty() || !sc.obligs.is_empty() { return None }
        let convert = |ty: &Ty| match ty {
            Ty::Nat | Ty::Int => Some(InterfaceIndexSort::Mathematical),
            Ty::Prim(Scalar::I64) => Some(InterfaceIndexSort::Machine),
            _ => None,
        };
        let mut ty = &sc.ty;
        let mut parameters = Vec::new();
        while let Ty::Fun(domain, _, row, result) = ty {
            if !row.is_pure() { return None }
            parameters.push(convert(domain)?);
            ty = result;
        }
        let result = self.interface_index_sort(ty, &mut Vec::new())?;
        if result == InterfaceIndexSort::Bytes { return None }
        Some((parameters, result))
    }

    fn portable_index(&self, term: &ITerm, scope: &InterfaceIndexScope) -> Option<(ITerm, InterfaceIndexSort)> {
        use InterfaceIndexSort::*;
        Some(match term {
            ITerm::Var(n) => {
                let sort = *scope.locals.get(n)?;
                if sort == Bytes { return None }
                (term.clone(), sort)
            }
            ITerm::Num(_) => (term.clone(), Mathematical),
            ITerm::Bin(op, a, b) => {
                let (left, ls) = self.portable_index(a, scope)?;
                let (right, rs) = self.portable_index(b, scope)?;
                if ls != Mathematical || rs != Mathematical { return None }
                (ITerm::Bin(*op, Box::new(left), Box::new(right)), Mathematical)
            }
            ITerm::App(f, args) if matches!(&**f, ITerm::Fn(n) if n == BYTE_LENGTH_INDEX) => {
                let [ITerm::Var(n)] = args.as_slice() else { return None };
                if scope.locals.get(n) != Some(&Bytes) { return None }
                (ITerm::app_fn("b.bytesLen", vec![ITerm::Var(n.clone())]), Mathematical)
            }
            ITerm::App(f, args) if scope.allow_measures => {
                let ITerm::Fn(identity) = &**f else { return None };
                let (parameters, result) = self.interface_measure_signature(identity)?;
                if parameters.len() != args.len() { return None }
                let mut arguments = Vec::new();
                for (argument, expected) in args.iter().zip(parameters) {
                    let (value, actual) = self.portable_index(argument, scope)?;
                    if actual != expected { return None }
                    arguments.push(value);
                }
                scope.measure_calls.borrow_mut().insert(identity.clone());
                (ITerm::app_fn(identity.clone(), arguments), result)
            }
            _ => return None,
        })
    }

    fn portable_predicate(&self, term: &PTerm, scope: &InterfaceIndexScope, negation: bool) -> Option<PTerm> {
        Some(match term {
            PTerm::Lit(_) => term.clone(),
            PTerm::Var(name) if scope.propositions.contains(name) => term.clone(),
            PTerm::Cmp(op, a, b) => PTerm::Cmp(*op,
                Box::new(self.portable_index(a, scope)?.0),
                Box::new(self.portable_index(b, scope)?.0)),
            PTerm::Eq(a, b) if negation => PTerm::Eq(
                Box::new(self.portable_index(a, scope)?.0),
                Box::new(self.portable_index(b, scope)?.0)),
            PTerm::And(a, b) => PTerm::And(
                Box::new(self.portable_predicate(a, scope, negation)?),
                Box::new(self.portable_predicate(b, scope, negation)?)),
            PTerm::Or(a, b) => PTerm::Or(
                Box::new(self.portable_predicate(a, scope, negation)?),
                Box::new(self.portable_predicate(b, scope, negation)?)),
            PTerm::Not(p) if negation => PTerm::Not(Box::new(self.portable_predicate(p, scope, true)?)),
            PTerm::Pred(PredFn::NonZero, value) => PTerm::Pred(PredFn::NonZero,
                Box::new(self.portable_index(value, scope)?.0)),
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
        if !info.base && info.params.is_empty() && info.ctx.is_empty() && !info.method_contracts.is_empty() {
            let head = Scheme::mono(Ty::Adt(info.target.class.clone(), info.target.args.clone()));
            self.remember_source_scheme(&info.module, &format!("@conditional-head:{}", info.name),
                &source.target, &head, &[], None, true);
        }
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

    fn checked_conditional_method(&self, info: &InstanceInfo, slot: usize) -> Option<crate::interface::CheckedInstanceMethod> {
        if info.base || !info.params.is_empty() || !info.ctx.is_empty() { return None }
        let definition = self.instance_defs.iter().find(|definition| definition.id == info.base_id)?;
        if definition.tparams != 0 || definition.subdict_count != 0 { return None }
        let implementation = definition.methods.get(slot)?;
        if !self.fns.iter().any(|function| function.id == *implementation) { return None }
        Some(crate::interface::CheckedInstanceMethod {
            head: self.resolved_interface_scheme(&info.module, &format!("@conditional-head:{}", info.name))?,
            implementation: implementation.0.clone(),
            resolved: self.resolved_interface_scheme(&info.module, &format!("@conditional-method:{}:{slot}", info.name))?,
            body: info.method_contracts.iter().find(|(index, _, _)| *index == slot).and_then(|(_, _, scheme)| self.guarded_nat_body(implementation, scheme)),
        })
    }

    fn remember_source_scheme(
        &mut self, module: &str, name: &str, source: &ast::Type,
        scheme: &Scheme, inherited: &[ast::TypeBinder], class: Option<&SigInfo>, class_target: bool,
    ) {
        if !scheme.evidence_params.is_empty() || !scheme.idx_params.is_empty()
            || scheme.val_params.iter().any(|(_, ty)| !matches!(ty, Ty::Nat | Ty::Int | Ty::Bytes))
            || !scheme.row_params.is_empty() { return }
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
        let Some((predicates, measure_calls)) = self.resolved_numeric_preconditions(body, scheme) else { return };
        context.extend(predicates);
        let text = if class_target { self.resolved_source_class_type(body, scheme, &vars, &mut references) }
            else { self.resolved_checked_source_type(body, &scheme.ty, scheme, &vars, &mut references, &[]) };
        let Some(mut text) = text else { return };
        if !context.is_empty() { text = format!("using ({}) => {text}", context.join(", ")); }
        if !prefix.is_empty() { text = format!("forall {}. {text}", prefix.join(" ")); }
        self.interface_source_schemes.insert((module.to_string(), name.to_string()), DefinitionScheme {
            text, references, con_params: scheme.con_params.clone(), measure_calls,
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
                else if scheme.prop_params.contains(name) { "Prop".to_string() }
                else if let Some((_, carrier)) = scheme.val_params.iter().find(|(n, _)| n == name) {
                    match carrier { Ty::Nat | Ty::Int | Ty::Bytes => carrier.to_string(), _ => return None }
                } else if scheme.params.contains(name) {
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
                else if parts.as_slice() == ["->"] { "(->)".to_string() }
                else if parts.as_slice() == ["Io"] { "Io".to_string() }
                else if parts.len() == 1 && (Scalar::parse(&name).is_some()
                    || matches!(name.as_str(), "Int" | "Nat" | "Bool" | "Unit" | "Bytes" | "Sym" | "Array" | "Tensor" | "Columnar")) { name }
                else {
                    let info = self.lookup_type(&parts).ok()?;
                    // Scope and Task are owned kernel type constructors,
                    // not nominal declarations in a fabricated module. Resolve
                    // the actual declaration before recognizing them, so a
                    // source type with the same spelling retains its owner.
                    if info.module == "<builtin>" {
                        if matches!(info.name.as_str(), "Scope" | "Task" | "Dec") { info.name.clone() }
                        else { return None }
                    } else {
                        references.insert((info.module.clone(), info.name.clone()));
                        format!("{}.{}", info.module, info.name)
                    }
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
        fn carries_predicates(ty: &Ty) -> bool {
            match ty {
                Ty::Prp(_) => true,
                Ty::Fun(domain, _, _, result) | Ty::Pi { dom: domain, cod: result, .. } =>
                    carries_predicates(domain) || carries_predicates(result),
                Ty::Adt(_, arguments) | Ty::App(_, arguments) | Ty::Tuple(arguments) =>
                    arguments.iter().any(carries_predicates),
                _ => false,
            }
        }
        // Keep existing canonical application parentheses for contracts
        // without predicates. Only logical structure needs checked printing.
        if !carries_predicates(checked) {
            return self.resolved_source_type(source, scheme, vars, references);
        }
        // Print propositions from their checked structure. Operator source
        // syntax has already been resolved into application spines, whose
        // generic prefix display is not portable proposition syntax.
        if let Ty::Prp(original) = checked {
            let mut scope = InterfaceIndexScope::default();
            for name in &scheme.prop_params {
                if vars.contains(name) { scope.propositions.insert(name.clone()); }
            }
            for (name, carrier) in &scheme.val_params {
                if vars.contains(name) { self.bind_interface_index(&mut scope, name, carrier); }
            }
            let mut predicate = (**original).clone();
            for (position, (name, carrier)) in arguments.iter().enumerate() {
                if let Some(name) = name {
                    predicate = predicate.subst(&format!("$a{position}"), &ITerm::Var(name.clone()));
                    self.bind_interface_index(&mut scope, name, carrier);
                }
            }
            return Some(self.portable_predicate(&predicate, &scope, true)?.to_string());
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
            let mut scope = InterfaceIndexScope::default();
            self.bind_interface_index(&mut scope, &result, carrier);
            for (position, (name, ty)) in arguments.iter().enumerate() {
                if let Some(name) = name {
                    predicate = predicate.subst(&format!("$a{position}"), &ITerm::Var(name.clone()));
                    self.bind_interface_index(&mut scope, name, ty);
                }
            }
            let predicate = self.portable_predicate(&predicate, &scope, false)?;
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
            _ => self.resolved_source_type(source, scheme, vars, references),
        }
    }

    // Use the checked predicates, whose argument positions are normalized
    // to $aN by conv_scheme. Rebind only actual computational arguments.
    // Exporting a conditional signature must not guess from display text.
    fn resolved_numeric_preconditions(&self, mut source: &ast::Type, scheme: &Scheme) -> Option<(Vec<String>, std::collections::BTreeSet<String>)> {
        let mut checked = &scheme.ty;
        let mut names: Vec<Option<String>> = Vec::new();
        let mut scope = InterfaceIndexScope::default();
        scope.allow_measures = true;
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
                self.bind_interface_index(&mut scope, name, domain);
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
            out.push(self.portable_predicate(&rebound, &scope, false)?.to_string());
        }
        Some((out, scope.measure_calls.into_inner()))
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
        let mut scope = InterfaceIndexScope::default();
        for (name, kind) in r.params.iter().zip(&info.param_kinds) {
            if let ParamKind::Val(ty) = kind {
                self.bind_interface_index(&mut scope, name, ty);
            }
        }
        // A generic numeric template retains its historical parameter
        // carrier. It may not use that abstract carrier as a byte buffer.
        self.bind_interface_index(&mut scope, &r.binder, &r.base);
        if matches!(&r.base, Ty::Param(n) if r.params.contains(n)) {
            scope.locals.insert(r.binder.clone(), InterfaceIndexSort::Mathematical);
        }
        Some(crate::interface::RefinementExport {
            binder: r.binder.clone(), carrier,
            predicate: self.portable_predicate(&r.pred, &scope, true)?.to_string(),
        })
    }

    fn resolved_interface_scheme(&self, module: &str, name: &str) -> Option<crate::interface::ResolvedScheme> {
        let source = self.interface_source_schemes.get(&(module.to_string(), name.to_string()))?;
        // Identities were captured by checked definition-site conversion.
        // Finalization reads those actual values, never re-resolves a source
        // spelling in the emitter's current module. No scheme recursion is
        // licensed by this first closed measure tier.
        let mut measures = Vec::new();
        for identity in &source.measure_calls {
            let value = self.qualified.values().find(|value| fn_id(&value.module, &value.name).0 == *identity)?;
            let defining = self.interface_source_schemes.get(&(value.module.clone(), value.name.clone()))?;
            if !defining.measure_calls.is_empty() { return None }
            let total_body = self.interface_value_body(value)?;
            let resolved = self.resolved_interface_scheme(&value.module, &value.name)?;
            measures.push(crate::interface::MeasureContract { identity: identity.clone(), resolved: Box::new(resolved), total_body });
        }

        let mut nominals = Vec::new();
        for (module, name) in &source.references {
            let info = self.types.get(name)?.iter().find(|t| &t.module == module)?;
            if info.fields.is_some() { return None }
            let refined = info.refine.is_some();
            if info.param_kinds.iter().any(|k| match k {
                ParamKind::Type | ParamKind::Con(_) => false,
                ParamKind::Val(ty) if refined || info.opaque => !matches!(ty, Ty::Nat | Ty::Int | Ty::Bytes),
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
        Some(crate::interface::ResolvedScheme { scheme: source.text.clone(), nominals, measures })
    }
}

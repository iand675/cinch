// Computations discarded with a proof must terminate as well as being pure.
// Inspect checked bodies, retaining the ordinary structural recursion check;
// an arrow's effect row is not a certificate that an arbitrary body terminates.
impl Checker {
    fn check_erased_computations(&self, module: &str) -> Result<()> {
        for (owner, span, expression) in &self.erasure_checks {
            if owner == module {
                self.erased_computation(expression, &HashMap::new(), &mut Vec::new(), *span)?;
            }
        }
        Ok(())
    }

    fn erased_computation(
        &self, expression: &ECore, locals: &HashMap<LocalId, ECore>,
        visiting: &mut Vec<GlobalId>, span: ast::Span,
    ) -> Result<()> {
        let unsupported = || Error::at(span,
            "an erased proof let must terminate: this computation has no checked termination argument");
        match expression {
            ECore::Int(_) | ECore::ConstBig { .. } | ECore::ConstBytes(_)
            | ECore::ConstString { .. } | ECore::ConstSym(_) | ECore::StrLit(_)
            | ECore::Lit(_) | ECore::IdxPending(_) | ECore::Bool(_)
            | ECore::Unit | ECore::Var(_) | ECore::Ctor(..) | ECore::BoxAlloc => Ok(()),
            ECore::Global(id, _) => self.erased_call(id, &[], locals, visiting, span),
            ECore::Prim(operation) if total_primitive(*operation).is_some() => Ok(()),
            ECore::Let(local, value, body) => {
                self.erased_computation(value, locals, visiting, span)?;
                let mut nested = locals.clone();
                nested.insert(*local, *value.clone());
                self.erased_computation(body, &nested, visiting, span)
            }
            ECore::If(condition, yes, no) => {
                self.erased_computation(condition, locals, visiting, span)?;
                self.erased_computation(&yes.body, locals, visiting, span)?;
                self.erased_computation(&no.body, locals, visiting, span)
            }
            ECore::Case { scrut, alts } => {
                self.erased_computation(scrut, locals, visiting, span)?;
                for alt in alts { self.erased_computation(&alt.body, locals, visiting, span)?; }
                Ok(())
            }
            ECore::Lam { .. } => Ok(()),
            ECore::MkClos { env, .. } => {
                for value in env { self.erased_computation(value, locals, visiting, span)?; }
                Ok(())
            }
            ECore::Proj(value, ..) | ECore::BoxInit(_, value, _)
            | ECore::LitFold { body: value, .. } => self.erased_computation(value, locals, visiting, span),
            ECore::InstDict(_, contexts, _, _) => {
                for value in contexts { self.erased_computation(value, locals, visiting, span)?; }
                Ok(())
            }
            ECore::App(function, arguments) => {
                let mut groups = vec![arguments.as_slice()];
                let mut function = &**function;
                while let ECore::App(head, arguments) = function {
                    groups.push(arguments);
                    function = head;
                }
                let arguments: Vec<_> = groups.into_iter().rev().flatten().collect();
                for argument in &arguments { self.erased_computation(argument, locals, visiting, span)?; }
                let mut aliases = HashSet::new();
                while let ECore::Var(local) = function {
                    if !aliases.insert(*local) { return Err(unsupported()); }
                    function = locals.get(local).ok_or_else(unsupported)?;
                }
                match function {
                    ECore::Global(id, _) => self.erased_call(id, &arguments, locals, visiting, span),
                    ECore::MkClos { fn_id, env } => {
                        let mut applied: Vec<_> = env.iter().collect();
                        applied.extend(arguments);
                        self.erased_call(fn_id, &applied, locals, visiting, span)
                    }
                    ECore::Ctor(..) => Ok(()),
                    ECore::Prim(operation) if total_primitive(*operation).is_some() => Ok(()),
                    ECore::Lam { params, body } if arguments.len() == params.len() => {
                        let mut nested = locals.clone();
                        for (parameter, argument) in params.iter().zip(arguments) {
                            nested.insert(*parameter, argument.clone());
                        }
                        self.erased_computation(body, &nested, visiting, span)
                    }
                    // Higher-order values without a checked body must not
                    // become a new termination axiom through erasure.
                    _ => Err(unsupported()),
                }
            }
            _ => Err(unsupported()),
        }
    }

    fn erased_call(
        &self, id: &GlobalId, arguments: &[&ECore], locals: &HashMap<LocalId, ECore>,
        visiting: &mut Vec<GlobalId>, span: ast::Span,
    ) -> Result<()> {
        let unsupported = || Error::at(span, format!(
            "an erased proof let must terminate: {} has no checked termination argument", id.0));
        if let Some(unit) = self.fns.iter().find(|unit| unit.id == *id) {
            if unit.ctor.is_some() || unit.prim.is_some_and(|op| total_primitive(op).is_some()) {
                return Ok(());
            }
        }
        let (parameters, body) = self.erasure_bodies.get(id).ok_or_else(unsupported)?;
        // A result function needs its own body/argument certificate. Do not
        // mistake a verified first-order call for arbitrary overapplication.
        if arguments.len() < parameters.len() { return Ok(()) }
        if arguments.len() > parameters.len() { return Err(unsupported()); }
        self.check_termination(id, parameters, body, span, &id.0)?;
        if visiting.last() == Some(id) { return Ok(()) }
        if visiting.contains(id) { return Err(unsupported()); }
        visiting.push(id.clone());
        let mut nested = locals.clone();
        for (parameter, argument) in parameters.iter().zip(arguments) {
            nested.insert(*parameter, (*argument).clone());
        }
        let result = self.erased_computation(body, &nested, visiting, span);
        visiting.pop();
        result
    }
}

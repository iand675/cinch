// Native producer for portable total bodies. Ordinary callable schemes and
// class/slot spellings grant no semantics; resolve the actual closed method
// field, then inspect its checked body and fixed primitive operations.

/// A bounded symbolic execution of an actual checked comparison. Parameters
/// carry their exact scalar type; the only observations admitted below are
/// fixed comparison primitives. A Bool constant is never a numeric equation.
#[derive(Clone, Debug, PartialEq)]
enum ComparisonReading {
    Argument(usize, Ty),
    Arithmetic(Ty, ITerm),
    Ordering(Ty, usize, usize),
    Predicate(IdxCmp, Ty, usize, usize),
    Word(i64),
    Bool(bool),
    Function(GlobalId, Vec<ComparisonReading>),
    Primitive(PrimOp),
    Dictionary(GlobalId),
}

fn reverse_comparison(cmp: IdxCmp) -> IdxCmp {
    match cmp {
        IdxCmp::Lt => IdxCmp::Gt, IdxCmp::Le => IdxCmp::Ge,
        IdxCmp::Gt => IdxCmp::Lt, IdxCmp::Ge => IdxCmp::Le,
        other => other,
    }
}
fn negate_comparison(cmp: IdxCmp) -> IdxCmp {
    match cmp {
        IdxCmp::Lt => IdxCmp::Ge, IdxCmp::Le => IdxCmp::Gt,
        IdxCmp::Gt => IdxCmp::Le, IdxCmp::Ge => IdxCmp::Lt,
        IdxCmp::Eq => IdxCmp::Ne, IdxCmp::Ne => IdxCmp::Eq,
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ArithmeticCertificate {
    dictionary: GlobalId,
    slot: usize,
    carrier: Ty,
    operation: IdxOp,
}

#[derive(Clone, Debug, PartialEq)]
struct ComparisonCertificate {
    dictionary: GlobalId,
    slot: usize,
    carrier: Ty,
    comparison: IdxCmp,
    arithmetic: Vec<ArithmeticCertificate>,
}

impl Checker {
    fn checked_comparison_method(&self, want: &Want, slot: usize, ty: &Ty, anticipated: IdxCmp) -> Option<ComparisonCertificate> {
        let wanted = self.uni.zonk_class(&want.class);
        if wanted.args.iter().any(|ty| !self.uni.free_metas(ty).is_empty()) { return None }
        // Use the ordinary resolver with a private inference store. This
        // preserves visibility, lexical priority and instance specificity;
        // speculative inspection must not commit constraints or transfers.
        let mut resolver = DictionaryResolver { checker: self, uni: self.uni.clone(), transfers: Vec::new() };
        let Solved(DictExpr::Instance(id, dictionaries, types)) = resolver.solve(&wanted, &want.scope, 0, want.span).ok()? else { return None };
        if !dictionaries.is_empty() || !types.is_empty()
            || resolver.transfers.iter().any(|transfer| !transfer.obligs.is_empty() || transfer.container.is_some() || !transfer.subdicts.is_empty()) { return None }
        // Forward declarations may be selected before their bodies. Their
        // anticipated reading is a proof obligation, never a trusted axiom:
        // only cited facts are checked against actual bodies before finish.
        let comparison = if self.instance_defs.iter().any(|d| d.id == id) {
            self.read_selected_comparison(&id, slot, ty)?
        } else { anticipated };
        Some(ComparisonCertificate { dictionary: id, slot, carrier: ty.clone(), comparison, arithmetic: Vec::new() })
    }

    /// Runtime operands may denote mathematical index arithmetic only when
    /// the actual callable selected in this lexical scope computes it.
    /// Type-position operators keep their independent mathematical meaning.
    fn with_runtime_arithmetic<T>(&self, apply: impl FnOnce() -> Result<T>) -> Result<(T, Vec<ArithmeticCertificate>)> {
        self.pending_runtime_arithmetic.borrow_mut().push(Vec::new());
        let result = apply();
        let certificates = self.pending_runtime_arithmetic.borrow_mut().pop().expect("arithmetic reading scope");
        result.map(|value| (value, certificates))
    }

    fn use_runtime_arithmetic(&self, certificates: Vec<ArithmeticCertificate>, span: ast::Span) {
        let mut used = self.used_runtime_arithmetic.borrow_mut();
        for certificate in certificates {
            if !used.iter().any(|(old, _, _)| old == &certificate) {
                used.push((certificate, span, self.current_module.clone()));
            }
        }
    }

    fn required_term_index(&self, term: &ast::Term, env: &Env) -> Result<ITerm> {
        let (index, certificates) = self.with_runtime_arithmetic(|| self.idx_of_term(term, env))?;
        self.use_runtime_arithmetic(certificates, term.span());
        Ok(index)
    }

    fn read_selected_arithmetic(&self, id: &GlobalId, slot: usize, carrier: &Ty, operation: IdxOp) -> bool {
        let Some(implementation) = self.closed_comparison_method(id, slot) else { return false };
        let arguments = vec![ComparisonReading::Argument(0, carrier.clone()), ComparisonReading::Argument(1, carrier.clone())];
        let actual = self.read_comparison_call(implementation, arguments, &mut 4096, &mut Vec::new());
        let expected = ITerm::Bin(operation, Box::new(ITerm::Var("$arithmetic:0".into())), Box::new(ITerm::Var("$arithmetic:1".into()))).normalize();
        matches!(actual, Some(ComparisonReading::Arithmetic(ref ty, ref expression)) if ty == carrier && expression.normalize() == expected)
    }

    fn runtime_arithmetic_index(&self, term: &ast::Term, arguments: &[ast::Term], env: &Env, operation: IdxOp, span: ast::Span) -> Result<(ITerm, Ty)> {
        let refusal = || Error::at(span, "runtime arithmetic needs an actual checked mathematical implementation before it can appear in a dependent type or refinement fact");
        let ast::Term::App(head, _, _) = term else { return Err(refusal()) };
        let ast::Term::Var(path) = &**head else { return Err(refusal()) };
        let scope = env.locals.iter().map(|(_, local, _)| *local).collect::<Vec<_>>();
        let dictionaries = env.dicts.iter().map(|(local, _)| *local).collect::<Vec<_>>();
        let reading = self.runtime_index_readings.iter().rev().find(|reading|
            reading.name.same_syntax_node(&path.name) && reading.locals == scope && reading.dictionaries == dictionaries).ok_or_else(refusal)?;
        let carrier = self.checked_runtime_arithmetic(reading, operation).ok_or_else(refusal)?;
        Ok((ITerm::Bin(operation, Box::new(self.idx_of_term(&arguments[0], env)?), Box::new(self.idx_of_term(&arguments[1], env)?)), carrier))
    }

    fn checked_runtime_arithmetic(&self, reading: &RuntimeIndexReading, operation: IdxOp) -> Option<Ty> {
        let result = self.uni.zonk(&reading.carrier);
        let result = self.refine_base(&result).unwrap_or(result);
        if !matches!(result, Ty::Nat | Ty::Int) { return None; }
        let head = &reading.core;
        let symbolic = vec![ComparisonReading::Argument(0, result.clone()), ComparisonReading::Argument(1, result.clone())];
        let actual = match head {
            ECore::DictProj(dictionary, slot, _, _) => {
                let implementation = match &**dictionary {
                    ECore::Hole(id) => {
                        let want = self.wants.iter().find(|want| want.id == *id)?;
                        let wanted = self.uni.zonk_class(&want.class);
                        if wanted.args.iter().any(|ty| !self.uni.free_metas(ty).is_empty()) { return None; }
                        let mut resolver = DictionaryResolver { checker: self, uni: self.uni.clone(), transfers: Vec::new() };
                        let Solved(DictExpr::Instance(id, contexts, types)) = resolver.solve(&wanted, &want.scope, 0, want.span).ok()? else { return None };
                        if !contexts.is_empty() || !types.is_empty() { return None; }
                        if !self.instance_defs.iter().any(|instance| instance.id == id) {
                            let mut pending = self.pending_runtime_arithmetic.borrow_mut();
                            let certificates = pending.last_mut()?;
                            let certificate = ArithmeticCertificate { dictionary: id, slot: *slot, carrier: result.clone(), operation };
                            if !certificates.contains(&certificate) { certificates.push(certificate); }
                            return Some(result);
                        }
                        self.closed_comparison_method(&id, *slot)?.clone()
                    }
                    ECore::InstDict(id, contexts, types, _) if contexts.is_empty() && types.is_empty() => self.closed_comparison_method(id, *slot)?.clone(),
                    _ => return None,
                };
                self.read_comparison_call(&implementation, symbolic, &mut 4096, &mut Vec::new())
            }
            ECore::Prim(primitive) => comparison_primitive(*primitive, &symbolic),
            ECore::Global(id, types) if types.is_empty() => self.read_comparison_call(id, symbolic, &mut 4096, &mut Vec::new()),
            _ => None,
        };
        let expected = ITerm::Bin(operation, Box::new(ITerm::Var("$arithmetic:0".into())), Box::new(ITerm::Var("$arithmetic:1".into()))).normalize();
        if !matches!(actual, Some(ComparisonReading::Arithmetic(ref carrier, ref expression)) if *carrier == result && expression.normalize() == expected) { return None; }
        Some(result)
    }

    /// Named runtime helpers reduce through their actual checked Core,
    /// whose dictionary holes are already filled. Source syntax alone
    /// cannot certify an overloaded operation after its want scope ends.
    fn reduce_checked_numeric_helper(&self, id: &str, arguments: &[ITerm], fuel: u32) -> Option<ITerm> {
        let id = GlobalId(id.into());
        let (parameters, body) = self.erasure_bodies.get(&id)?;
        if parameters.len() != arguments.len() { return None; }
        let locals = parameters.iter().zip(arguments).map(|(parameter, argument)| {
            let carrier = self.uni.zonk(self.local_tys.get(parameter)?);
            let carrier = self.refine_base(&carrier).unwrap_or(carrier);
            matches!(carrier, Ty::Nat | Ty::Int).then(|| (*parameter, ComparisonReading::Arithmetic(carrier, argument.clone())))
        }).collect::<Option<HashMap<_, _>>>()?;
        let result = self.read_comparison_core(body, &locals, &mut (fuel as usize * 64), &mut vec![id])?;
        match result { ComparisonReading::Arithmetic(_, expression) => Some(expression.normalize()), _ => None }
    }

    fn read_selected_comparison(&self, id: &GlobalId, slot: usize, ty: &Ty) -> Option<IdxCmp> {
        let implementation = self.closed_comparison_method(id, slot)?;
        let args = vec![ComparisonReading::Argument(0, ty.clone()), ComparisonReading::Argument(1, ty.clone())];
        let result = self.read_comparison_call(implementation, args, &mut 4096, &mut Vec::new())?;
        match result {
            ComparisonReading::Predicate(cmp, carrier, 0, 1) if carrier == *ty => Some(cmp),
            ComparisonReading::Predicate(cmp, carrier, 1, 0) if carrier == *ty => Some(reverse_comparison(cmp)),
            _ => None,
        }
    }

    fn note_comparison_use(&self, local: LocalId, span: ast::Span) {
        if let Some(certificates) = self.comparison_facts.get(&local) {
            let mut used = self.used_comparisons.borrow_mut();
            for certificate in certificates {
                if !used.iter().any(|(old, _, _)| old == certificate) {
                    used.push((certificate.clone(), span, self.current_module.clone()));
                }
            }
        }
    }

    fn check_used_comparisons(&self) -> Result<()> {
        for (certificate, span, module) in self.used_comparisons.borrow().iter() {
            for arithmetic in &certificate.arithmetic { self.check_arithmetic_certificate(arithmetic, *span, module)?; }
            if self.read_selected_comparison(&certificate.dictionary, certificate.slot, &certificate.carrier) != Some(certificate.comparison) {
                return Err(Error::at(*span, format!(
                    "a refinement obligation cites a comparison fact whose actual selected implementation {} at slot {} does not establish {:?} over {} (in {module})",
                    certificate.dictionary.0, certificate.slot, certificate.comparison, certificate.carrier)));
            }
        }
        for (certificate, span, module) in self.used_runtime_arithmetic.borrow().iter() {
            self.check_arithmetic_certificate(certificate, *span, module)?;
        }
        Ok(())
    }

    fn check_arithmetic_certificate(&self, certificate: &ArithmeticCertificate, span: ast::Span, module: &str) -> Result<()> {
        if !self.read_selected_arithmetic(&certificate.dictionary, certificate.slot, &certificate.carrier, certificate.operation) {
            return Err(Error::at(span, format!("a dependent index or refinement obligation cites runtime arithmetic whose actual selected implementation {} at slot {} does not establish {:?} over {} (in {module})", certificate.dictionary.0, certificate.slot, certificate.operation, certificate.carrier)));
        }
        Ok(())
    }

    fn closed_comparison_method(&self, id: &GlobalId, slot: usize) -> Option<&GlobalId> {
        let instance = self.instance_defs.iter().find(|d| d.id == *id)?;
        if instance.tparams != 0 || instance.subdict_count != 0 || *instance.own.get(slot)? != 0 { return None }
        instance.methods.get(slot)
    }

    fn read_comparison_call(&self, id: &GlobalId, args: Vec<ComparisonReading>, fuel: &mut usize, visiting: &mut Vec<GlobalId>) -> Option<ComparisonReading> {
        *fuel = fuel.checked_sub(1)?;
        if visiting.len() >= 64 || visiting.contains(id) { return None }
        if let Some(function) = self.fns.iter().find(|function| function.id == *id) {
            if let Some(op) = function.prim {
                if function.params.len() != args.len() { return None }
                return comparison_primitive(op, &args);
            }
        }
        // These are checked pure bodies retained before runtime lowering,
        // with their real parameter identities. No names or class slots
        // supply an equation, and foreign/effectful/assumed prims are absent.
        let (parameters, body) = self.erasure_bodies.get(id)?;
        if args.len() < parameters.len() { return Some(ComparisonReading::Function(id.clone(), args)) }
        if args.len() != parameters.len() { return None }
        let locals = parameters.iter().copied().zip(args).collect();
        visiting.push(id.clone());
        let result = self.read_comparison_core(body, &locals, fuel, visiting);
        visiting.pop();
        result
    }

    fn read_comparison_core(&self, body: &ECore, locals: &HashMap<LocalId, ComparisonReading>, fuel: &mut usize, visiting: &mut Vec<GlobalId>) -> Option<ComparisonReading> {
        use ComparisonReading as R;
        *fuel = fuel.checked_sub(1)?;
        Some(match body {
            ECore::Var(id) => locals.get(id)?.clone(),
            ECore::Int(value) => R::Word(*value),
            ECore::ConstBig { nat, digits } => R::Arithmetic(if *nat { Ty::Nat } else { Ty::Int }, ITerm::Num(digits.clone())),
            ECore::Bool(value) => R::Bool(*value),
            ECore::Prim(op) => R::Primitive(*op),
            ECore::Global(id, actuals) if actuals.is_empty() => {
                if self.erasure_bodies.get(id).is_some_and(|(params, _)| params.is_empty()) {
                    return self.read_comparison_call(id, Vec::new(), fuel, visiting);
                }
                R::Function(id.clone(), Vec::new())
            }
            ECore::MkClos { fn_id, env } => R::Function(fn_id.clone(), env.iter().map(|arg| self.read_comparison_core(arg, locals, fuel, visiting)).collect::<Option<_>>()?),
            ECore::InstDict(id, dictionaries, types, _) if dictionaries.is_empty() && types.is_empty() => R::Dictionary(id.clone()),
            ECore::DictProj(dictionary, slot, _, _) => {
                let R::Dictionary(id) = self.read_comparison_core(dictionary, locals, fuel, visiting)? else { return None };
                R::Function(self.closed_comparison_method(&id, *slot)?.clone(), Vec::new())
            }
            ECore::Let(id, value, body) => {
                let value = self.read_comparison_core(value, locals, fuel, visiting)?;
                let mut nested = locals.clone(); nested.insert(*id, value);
                return self.read_comparison_core(body, &nested, fuel, visiting);
            }
            ECore::App(function, arguments) => {
                let mut groups = vec![arguments.as_slice()]; let mut head = &**function;
                while let ECore::App(function, arguments) = head { groups.push(arguments.as_slice()); head = function; }
                let function = self.read_comparison_core(head, locals, fuel, visiting)?;
                let mut args = Vec::new();
                for group in groups.into_iter().rev() {
                    for arg in group { args.push(self.read_comparison_core(arg, locals, fuel, visiting)?); }
                }
                return match function {
                    R::Primitive(op) => comparison_primitive(op, &args),
                    R::Function(id, mut captures) => { captures.extend(args); self.read_comparison_call(&id, captures, fuel, visiting) }
                    _ => None,
                };
            }
            ECore::If(condition, yes, no) => {
                match self.read_comparison_core(condition, locals, fuel, visiting)? {
                    R::Bool(true) => return self.read_comparison_core(&yes.body, locals, fuel, visiting),
                    R::Bool(false) => return self.read_comparison_core(&no.body, locals, fuel, visiting),
                    R::Predicate(..) => {
                        let yes = self.read_comparison_core(&yes.body, locals, fuel, visiting)?;
                        let no = self.read_comparison_core(&no.body, locals, fuel, visiting)?;
                        if yes != no { return None }
                        yes
                    }
                    _ => return None,
                }
            }
            _ => return None,
        })
    }
}

fn comparison_primitive(op: PrimOp, arguments: &[ComparisonReading]) -> Option<ComparisonReading> {
    use ComparisonReading as R;
    let cmp = |op| match op {
        CmpOp::Eq => IdxCmp::Eq, CmpOp::Ne => IdxCmp::Ne, CmpOp::Lt => IdxCmp::Lt,
        CmpOp::Le => IdxCmp::Le, CmpOp::Gt => IdxCmp::Gt, CmpOp::Ge => IdxCmp::Ge,
    };
    let numeric = |value: &R, carrier: &Ty| match value {
        R::Argument(slot, ty) if ty == carrier => Some(ITerm::Var(format!("$arithmetic:{slot}"))),
        R::Arithmetic(ty, expression) if ty == carrier => Some(expression.clone()),
        _ => None,
    };
    let arithmetic = match op {
        PrimOp::Nat(NatOp::Add) => Some((Ty::Nat, IdxOp::Add)),
        PrimOp::Nat(NatOp::Sub) => Some((Ty::Nat, IdxOp::Sub)),
        PrimOp::Nat(NatOp::Mul) => Some((Ty::Nat, IdxOp::Mul)),
        PrimOp::Int(IntOp::Add) => Some((Ty::Int, IdxOp::Add)),
        PrimOp::Int(IntOp::Sub) => Some((Ty::Int, IdxOp::Sub)),
        PrimOp::Int(IntOp::Mul) => Some((Ty::Int, IdxOp::Mul)),
        _ => None,
    };
    if let (Some((carrier, operation)), [left, right]) = (arithmetic, arguments) {
        let expression = ITerm::Bin(operation, Box::new(numeric(left, &carrier)?), Box::new(numeric(right, &carrier)?)).normalize();
        return Some(R::Arithmetic(carrier, expression));
    }
    match (op, arguments) {
        (PrimOp::Nat(NatOp::Cmp), [R::Argument(a, Ty::Nat), R::Argument(b, Ty::Nat)]) => Some(R::Ordering(Ty::Nat, *a, *b)),
        (PrimOp::Int(IntOp::Cmp), [R::Argument(a, Ty::Int), R::Argument(b, Ty::Int)]) => Some(R::Ordering(Ty::Int, *a, *b)),
        (PrimOp::Cmp { ty: Scalar::I64, op }, [R::Ordering(ty, a, b), R::Word(0)]) => Some(R::Predicate(cmp(op), ty.clone(), *a, *b)),
        (PrimOp::Cmp { ty: Scalar::I64, op }, [R::Word(0), R::Ordering(ty, a, b)]) => Some(R::Predicate(reverse_comparison(cmp(op)), ty.clone(), *a, *b)),
        (PrimOp::Cmp { ty, op }, [R::Argument(a, Ty::Prim(left)), R::Argument(b, Ty::Prim(right))])
            if (ty.is_int_like() || ty == Scalar::Char) && ty == *left && ty == *right => Some(R::Predicate(cmp(op), Ty::Prim(ty), *a, *b)),
        (PrimOp::NotBool, [R::Predicate(cmp, ty, a, b)]) => Some(R::Predicate(negate_comparison(*cmp), ty.clone(), *a, *b)),
        (PrimOp::NotBool, [R::Bool(value)]) => Some(R::Bool(!value)),
        _ => None,
    }
}

impl Checker {
    fn total_sort(&self, ty: &Ty, seen: &mut Vec<Ty>) -> Option<crate::total::Sort> {
        use crate::total::Sort;
        Some(match ty {
            Ty::Nat => Sort::Nat,
            Ty::Int => Sort::Int,
            Ty::Bool => Sort::Bool,
            Ty::Bytes => Sort::Bytes,
            Ty::Prim(Scalar::I64) => Sort::I64,
            Ty::Adt(..) if !seen.contains(ty) => {
                seen.push(ty.clone());
                let base = self.refine_view(ty)?.base?;
                return self.total_sort(&base, seen);
            }
            _ => return None,
        })
    }
    fn remember_total_body(&mut self, id: &GlobalId, parameters: &[LocalId], result: crate::total::Sort, body: &ECore) {
        let parameters_sort = parameters.iter().map(|id| {
            let ty = self.uni.zonk(self.local_tys.get(id)?);
            self.total_sort(&ty, &mut Vec::new())
        }).collect::<Option<Vec<_>>>();
        let Some(parameters_sort) = parameters_sort else { self.interface_guarded_bodies.remove(&id.0); return };
        // Rechecking an unsupported replacement must remove old authority.
        self.interface_guarded_bodies.remove(&id.0);
        if result == crate::total::Sort::Nat && parameters.iter().all(|id| self.local_tys.get(id).is_some_and(|ty| self.uni.zonk(ty) == Ty::Nat)) {
            let scope = parameters.iter().enumerate().map(|(slot, id)| (*id, slot)).collect::<Vec<_>>();
            if let Some(expression) = pending_guarded_nat(body, &scope) {
                self.interface_guarded_bodies.insert(id.0.clone(), (parameters.len(), expression));
            }
        }
        let Some(body) = pending_total_expr(body, parameters) else { return };
        self.interface_total_bodies.insert(id.0.clone(), crate::total::PendingDefinition {
            identity: id.0.clone(), parameters: parameters_sort, result, body,
        });
    }
    fn checked_total_bodies(&self) -> crate::total::CheckedGraph {
        let primitives = self.fns.iter().filter_map(|function| {
            let operation = total_primitive(function.prim?)?;
            if function.params.len() != operation.signature().0.len() { return None }
            Some((function.id.0.clone(), operation))
        }).collect();
        let mut producer = TotalProducer {
            pending: &self.interface_total_bodies, instances: &self.instance_defs,
            primitives, graph: crate::total::CheckedGraph::empty(),
            visiting: std::collections::BTreeSet::new(), refused: std::collections::BTreeSet::new(),
        };
        for identity in self.interface_total_bodies.keys() { producer.definition(identity); }
        producer.graph
    }
}

fn total_primitive(primitive: PrimOp) -> Option<crate::total::Op> {
    use crate::total::Op;
    Some(match primitive {
        PrimOp::Nat(NatOp::Add) => Op::NatAdd,
        PrimOp::Nat(NatOp::Mul) => Op::NatMul,
        PrimOp::Nat(NatOp::Cmp) => Op::NatCmp,
        PrimOp::Int(IntOp::Add) => Op::IntAdd,
        PrimOp::Int(IntOp::Sub) => Op::IntSub,
        PrimOp::Int(IntOp::Mul) => Op::IntMul,
        PrimOp::Int(IntOp::Neg) => Op::IntNeg,
        PrimOp::Int(IntOp::Cmp) => Op::IntCmp,
        PrimOp::Int(IntOp::FromNat) => Op::IntFromNat,
        PrimOp::Int(IntOp::FromI64) => Op::IntFromI64,
        PrimOp::Bytes(BytesOp::Len) => Op::BytesLen,
        PrimOp::Cmp { ty: Scalar::I64, op } => match op {
            CmpOp::Eq => Op::I64Eq, CmpOp::Ne => Op::I64Ne,
            CmpOp::Lt => Op::I64Lt, CmpOp::Le => Op::I64Le,
            CmpOp::Gt => Op::I64Gt, CmpOp::Ge => Op::I64Ge,
        },
        PrimOp::EqBool => Op::BoolEq, PrimOp::NeBool => Op::BoolNe,
        PrimOp::NotBool => Op::BoolNot, PrimOp::AndBool => Op::BoolAnd, PrimOp::OrBool => Op::BoolOr,
        _ => return None,
    })
}

fn pending_total_expr(body: &ECore, scope: &[LocalId]) -> Option<crate::total::PendingExpr> {
    pending_total_expr_inner(body, scope, &[])
}
fn total_dictionary(expression: &ECore, dictionaries: &[(LocalId, String)]) -> Option<String> {
    match expression {
        ECore::InstDict(id, contexts, actuals, _) if contexts.is_empty() && actuals.is_empty() => Some(id.0.clone()),
        ECore::Var(id) => dictionaries.iter().rev().find(|(local, _)| local == id).map(|(_, name)| name.clone()),
        _ => None,
    }
}
fn pending_total_expr_inner(body: &ECore, scope: &[LocalId], dictionaries: &[(LocalId, String)]) -> Option<crate::total::PendingExpr> {
    use crate::total::{PendingExpr as P, Sort};
    let arguments = |args: &[ECore]| args.iter().map(|arg| pending_total_expr_inner(arg, scope, dictionaries)).collect::<Option<Vec<_>>>();
    Some(match body {
        ECore::Var(id) => P::Local(scope.iter().rposition(|bound| bound == id)?),
        ECore::ConstBig { nat, digits } => P::Number(if *nat { Sort::Nat } else { Sort::Int }, digits.clone()),
        ECore::Int(value) => P::Number(Sort::I64, value.to_string()),
        ECore::Bool(value) => P::Bool(*value),
        ECore::Global(id, actuals) if actuals.is_empty() => P::Call(id.0.clone(), Vec::new()),
        ECore::If(condition, yes, no) => P::If(Box::new(pending_total_expr_inner(condition, scope, dictionaries)?), Box::new(pending_total_expr_inner(&yes.body, scope, dictionaries)?), Box::new(pending_total_expr_inner(&no.body, scope, dictionaries)?)),
        ECore::Let(id, value, body) => {
            if let Some(dictionary) = total_dictionary(value, dictionaries) {
                let mut nested = dictionaries.to_vec(); nested.push((*id, dictionary));
                return pending_total_expr_inner(body, scope, &nested);
            }
            let value = pending_total_expr_inner(value, scope, dictionaries)?;
            let mut inner = scope.to_vec(); inner.push(*id);
            P::Let(Box::new(value), Box::new(pending_total_expr_inner(body, &inner, dictionaries)?))
        }
        ECore::App(function, args) => {
            // Recover the actual curried spine, preserving argument order.
            let mut groups = vec![args.as_slice()]; let mut head = &**function;
            while let ECore::App(function, args) = head { groups.push(args.as_slice()); head = function; }
            let mut args = Vec::new();
            for group in groups.into_iter().rev() { args.extend(arguments(group)?); }
            match head {
                ECore::Prim(operation) => P::Primitive(total_primitive(*operation)?, args),
                ECore::Global(id, actuals) if actuals.is_empty() => P::Call(id.0.clone(), args),
                ECore::DictProj(dictionary, slot, _, _) => P::Selected { dictionary: total_dictionary(dictionary, dictionaries)?, slot: *slot, arguments: args },
                _ => return None,
            }
        }
        // No holes, unknown dictionary providers, effects, content reads,
        // closure environments, foreign calls or unchecked partial operations.
        _ => return None,
    })
}

struct TotalProducer<'a> {
    pending: &'a std::collections::BTreeMap<String, crate::total::PendingDefinition>,
    instances: &'a [crate::core::InstanceDef],
    primitives: std::collections::BTreeMap<String, crate::total::Op>,
    graph: crate::total::CheckedGraph,
    visiting: std::collections::BTreeSet<String>,
    refused: std::collections::BTreeSet<String>,
}
impl TotalProducer<'_> {
    fn definition(&mut self, identity: &str) -> Option<()> {
        use crate::total::Expr;
        if self.graph.get(identity).is_some() { return Some(()) }
        if self.refused.contains(identity) || !self.visiting.insert(identity.to_string()) { return None }
        let candidate = if let Some(pending) = self.pending.get(identity).cloned() {
            self.expression(&pending.body).map(|body| (pending.identity, pending.parameters, pending.result, body))
        } else if let Some(operation) = self.primitives.get(identity) {
            let parameters = operation.signature().0.to_vec();
            let arguments = (0..parameters.len()).map(Expr::Local).collect();
            Some((identity.to_string(), parameters, operation.signature().1, Expr::Primitive(*operation, arguments)))
        } else { None };
        self.visiting.remove(identity);
        if let Some((identity, parameters, result, body)) = candidate {
            if self.graph.infer(&body, &parameters).ok() == Some(result) && self.graph.define(identity.clone(), parameters, body).is_ok() { return Some(()) }
        }
        self.refused.insert(identity.to_string()); None
    }
    fn expression(&mut self, expression: &crate::total::PendingExpr) -> Option<crate::total::Expr> {
        use crate::total::{PendingExpr as P, Expr as E};
        Some(match expression {
            P::Local(slot) => E::Local(*slot),
            P::Number(sort, decimal) => E::Number { sort: *sort, decimal: decimal.clone() },
            P::Bool(value) => E::Bool(*value),
            P::Primitive(op, arguments) => E::Primitive(*op, self.arguments(arguments)?),
            P::Call(target, arguments) => {
                self.definition(target)?;
                E::Call { target: target.clone(), arguments: self.arguments(arguments)? }
            }
            P::Selected { dictionary, slot, arguments } => {
                let instance = self.instances.iter().find(|instance| instance.id.0 == *dictionary)?;
                if instance.tparams != 0 || instance.subdict_count != 0 { return None }
                let target = instance.methods.get(*slot)?.0.clone();
                self.definition(&target)?;
                E::Call { target, arguments: self.arguments(arguments)? }
            }
            P::Let(value, body) => E::Let { value: Box::new(self.expression(value)?), body: Box::new(self.expression(body)?) },
            P::If(condition, yes, no) => E::If { condition: Box::new(self.expression(condition)?), yes: Box::new(self.expression(yes)?), no: Box::new(self.expression(no)?) },
        })
    }
    fn arguments(&mut self, arguments: &[crate::total::PendingExpr]) -> Option<Vec<crate::total::Expr>> {
        arguments.iter().map(|argument| self.expression(argument)).collect()
    }
}

impl Checker {
    fn interface_method_bodies(&self, instance: &InstanceInfo) -> Vec<crate::interface::InstanceMethodBody> {
        if instance.base || !instance.params.is_empty() || !instance.ctx.is_empty() || !instance.method_contracts.is_empty() { return Vec::new(); }
        let Some(definition) = self.instance_defs.iter().find(|definition| definition.id == instance.base_id) else { return Vec::new() };
        if definition.tparams != 0 || definition.subdict_count != 0 { return Vec::new(); }
        definition.methods.iter().enumerate().filter_map(|(slot, implementation)| {
            if *definition.own.get(slot)? != 0 { return None; }
            let scheme = self.interface_method_schemes.get(implementation)?;
            if !scheme.params.is_empty() || !scheme.idx_params.is_empty() || !scheme.val_params.is_empty()
                || !scheme.prop_params.is_empty() || !scheme.evidence_params.is_empty() || !scheme.region_params.is_empty()
                || !scheme.row_params.is_empty() || !scheme.ctx.is_empty() || !scheme.obligs.is_empty() { return None; }
            let graph = self.interface_checked_bodies.as_ref()?;
            let actual = graph.get(&implementation.0)?;
            let mut ty = &scheme.ty; let mut parameters = Vec::new();
            while let Ty::Fun(domain, _, row, result) = ty {
                if !row.is_pure() { return None; }
                parameters.push(plain_total_sort(domain)?); ty = result;
            }
            if parameters != actual.parameters || plain_total_sort(ty)? != actual.result { return None; }
            let declaration = render_scheme(scheme);
            if declaration != crate::interface::total_declaration(actual) { return None; }
            Some(crate::interface::InstanceMethodBody {
                slot: slot as u64, implementation: implementation.0.clone(),
                resolved: crate::interface::ResolvedScheme { scheme: declaration, nominals: Vec::new(), measures: Vec::new() },
                total_body: graph.closure([implementation.0.as_str()]).ok()?,
            })
        }).collect()
    }

    fn interface_value_body(&self, value: &ValueInfo) -> Option<crate::total::CheckedGraph> {
        let scheme = &value.scheme;
        if !scheme.params.is_empty() || !scheme.idx_params.is_empty() || !scheme.val_params.is_empty()
            || !scheme.prop_params.is_empty() || !scheme.region_params.is_empty() || !scheme.row_params.is_empty()
            || !scheme.ctx.is_empty() || !scheme.obligs.is_empty() { return None }
        self.resolved_interface_scheme(&value.module, &value.name)?;
        let identity = fn_id(&value.module, &value.name).0;
        let graph = self.interface_checked_bodies.as_ref()?;
        let definition = graph.get(&identity)?;
        let mut ty = &scheme.ty;
        let mut parameters = Vec::new();
        while let Ty::Fun(domain, _, row, result) = ty {
            if !row.is_pure() { return None }
            // Refinement domains, Pi binders and hidden proof/dictionary
            // contexts must never disappear through a base-sort projection.
            parameters.push(plain_total_sort(domain)?);
            ty = result;
        }
        if parameters != definition.parameters { return None }
        if let Some(sort) = plain_total_sort(ty) {
            if sort != definition.result { return None }
        } else {
            let view = self.refine_view(ty)?;
            let base = view.base?;
            let sort = plain_total_sort(&base)?;
            if !matches!(sort, crate::total::Sort::Nat | crate::total::Sort::Int | crate::total::Sort::I64)
                || sort != definition.result || !literal_result_predicate(&view.pred, &view.binder) { return None }
            // This mirrors the independent consumer's supported finite
            // envelope tier. More general refinements retain their scheme
            // without advertising executable index authority.
            let mut fuel = 4096;
            if !finite_total_result(graph, &definition.body, &vec![false; parameters.len()], &mut fuel) { return None }
        }
        graph.closure([identity.as_str()]).ok()
    }
}
fn plain_total_sort(ty: &Ty) -> Option<crate::total::Sort> {
    use crate::total::Sort;
    Some(match ty { Ty::Nat => Sort::Nat, Ty::Int => Sort::Int, Ty::Bool => Sort::Bool,
        Ty::Bytes => Sort::Bytes, Ty::Prim(Scalar::I64) => Sort::I64, _ => return None })
}
fn literal_result_predicate(predicate: &PTerm, binder: &str) -> bool {
    fn atom(term: &ITerm, binder: &str) -> bool { matches!(term, ITerm::Num(_)) || matches!(term, ITerm::Var(name) if name == binder) }
    match predicate {
        PTerm::Cmp(op, a, b) => *op != IdxCmp::Ne && atom(a, binder) && atom(b, binder),
        PTerm::Eq(a, b) => atom(a, binder) && atom(b, binder),
        PTerm::And(a, b) => literal_result_predicate(a, binder) && literal_result_predicate(b, binder),
        _ => false,
    }
}
fn finite_total_result(graph: &crate::total::CheckedGraph, expression: &crate::total::Expr, scope: &[bool], fuel: &mut usize) -> bool {
    use crate::total::Expr;
    if *fuel == 0 { return false }
    *fuel -= 1;
    match expression {
        Expr::Number { .. } => true,
        Expr::Local(slot) => scope.get(*slot).copied().unwrap_or(false),
        Expr::If { yes, no, .. } => finite_total_result(graph, yes, scope, fuel) && finite_total_result(graph, no, scope, fuel),
        Expr::Let { value, body } => {
            let value = finite_total_result(graph, value, scope, fuel);
            let mut nested = scope.to_vec(); nested.push(value);
            finite_total_result(graph, body, &nested, fuel)
        }
        Expr::Call { target, arguments } => {
            let actual = arguments.iter().map(|argument| finite_total_result(graph, argument, scope, fuel)).collect::<Vec<_>>();
            graph.get(target).is_some_and(|callee| finite_total_result(graph, &callee.body, &actual, fuel))
        }
        _ => false,
    }
}


#[derive(Clone)]
enum PendingGuardedNat {
    Argument(usize),
    Subtract(Box<PendingGuardedNat>, Box<PendingGuardedNat>),
    Call(String, Vec<PendingGuardedNat>),
}
fn pending_guarded_nat(body: &ECore, scope: &[(LocalId, usize)]) -> Option<PendingGuardedNat> {
    use PendingGuardedNat as P;
    Some(match body {
        ECore::Var(id) => P::Argument(scope.iter().rev().find(|(bound, _)| bound == id)?.1),
        ECore::Let(id, value, body) => {
            // Only a pure parameter alias can disappear from a strict let;
            // a discarded partial computation must never be erased here.
            let ECore::Var(original) = &**value else { return None };
            let slot = scope.iter().rev().find(|(bound, _)| bound == original)?.1;
            let mut inner = scope.to_vec(); inner.push((*id, slot));
            return pending_guarded_nat(body, &inner);
        }
        ECore::App(function, arguments) => {
            let mut groups = vec![arguments.as_slice()]; let mut head = &**function;
            while let ECore::App(function, arguments) = head { groups.push(arguments.as_slice()); head = function; }
            let mut values = Vec::new();
            for group in groups.into_iter().rev() {
                for value in group { values.push(pending_guarded_nat(value, scope)?); }
            }
            match head {
                ECore::Prim(PrimOp::Nat(NatOp::Sub)) if values.len() == 2 => {
                    let mut values = values.into_iter(); P::Subtract(Box::new(values.next()?), Box::new(values.next()?))
                }
                ECore::Global(identity, actuals) if actuals.is_empty() => P::Call(identity.0.clone(), values),
                _ => return None,
            }
        }
        _ => return None,
    })
}
impl Checker {
    fn guarded_nat_body(&self, identity: &GlobalId, scheme: &Scheme) -> Option<crate::interface::GuardedNatBody> {
        let mut ty = &scheme.ty; let mut arity = 0usize;
        loop {
            match ty {
                Ty::Fun(domain, _, row, result) | Ty::Pi { dom: domain, eff: row, cod: result, .. } => {
                    if **domain != Ty::Nat || !row.is_pure() { return None }
                    arity += 1; ty = result;
                }
                Ty::Nat => break,
                _ => return None,
            }
        }
        let (count, pending) = self.interface_guarded_bodies.get(&identity.0)?;
        if *count != arity { return None }
        fn resolve(checker: &Checker, term: &PendingGuardedNat, depth: usize) -> Option<crate::interface::GuardedNatExpr> {
            if depth > 64 { return None }
            use crate::interface::GuardedNatExpr as E;
            Some(match term {
                PendingGuardedNat::Argument(slot) => E::Argument(*slot as u64),
                PendingGuardedNat::Subtract(a, b) => E::Subtract(Box::new(resolve(checker, a, depth + 1)?), Box::new(resolve(checker, b, depth + 1)?)),
                PendingGuardedNat::Call(identity, args) => {
                    let function = checker.fns.iter().find(|f| f.id.0 == *identity)?;
                    if function.prim != Some(PrimOp::Nat(NatOp::Sub)) || function.params.len() != 2 || args.len() != 2 { return None }
                    E::Subtract(Box::new(resolve(checker, &args[0], depth + 1)?), Box::new(resolve(checker, &args[1], depth + 1)?))
                }
            })
        }
        Some(crate::interface::GuardedNatBody { parameters: arity as u64, root: resolve(self, pending, 0)? })
    }
}

impl Checker {
    // A strict Nat result is evaluated once. Its equation comes from the
    // checked primitive/body selected by its actual dictionary, never from
    // the source operator spelling. The ordinary deferred call discharge
    // still checks the call's original, pre-binding evidence snapshot.
    fn checked_nat_let_fact(&self, env: &Env, name: &str, local: LocalId, core: &ECore, ty: &Ty) -> Option<Fact> {
        let ty = self.uni.zonk(ty);
        let carrier = self.refine_view(&ty).and_then(|r| r.base).unwrap_or(ty);
        if carrier != Ty::Nat || !matches!(core, ECore::App(..) | ECore::Var(_)) { return None }
        let value = self.checked_nat_reading(env, core, 0)?;
        let mut names = self.term_names(env, &value)?;
        names.push((name.to_owned(), local));
        Some(Fact { prop: PTerm::Cmp(IdxCmp::Eq, Box::new(ITerm::Var(name.to_owned())), Box::new(value)), at: self.ev_writes.len(), names, comparisons: Vec::new() })
    }
    fn checked_nat_reading(&self, env: &Env, core: &ECore, depth: usize) -> Option<ITerm> {
        if depth > 64 { return None }
        match core {
            ECore::Var(local) => {
                let (name, _, scheme) = env.locals.iter().rev().find(|(_, id, _)| id == local)?;
                let ty = self.uni.zonk(&scheme.ty);
                let carrier = self.refine_view(&ty).and_then(|r| r.base).unwrap_or(ty);
                if carrier != Ty::Nat || !scheme.params.is_empty() || env.lookup(name)?.0 != *local { return None }
                Some(ITerm::Var(name.clone()))
            }
            ECore::ConstBig { nat: true, digits } => Some(ITerm::Num(digits.clone())),
            ECore::Lit(id) => {
                let literal = self.literal_pends.get(*id)?;
                if self.uni.zonk(&literal.ty) != Ty::Nat { return None }
                Some(ITerm::Num(literal.value.clone()))
            }
            ECore::App(function, arguments) => {
                let mut groups = vec![arguments.as_slice()]; let mut head = &**function;
                while let ECore::App(function, arguments) = head { groups.push(arguments.as_slice()); head = function; }
                let arguments = groups.into_iter().rev().flatten().collect::<Vec<_>>();
                let primitive = match head {
                    ECore::Prim(op) => Some(*op),
                    ECore::Global(id, actuals) if actuals.is_empty() => self.fns.iter().find(|f| f.id == *id).and_then(|f| f.prim),
                    _ => None,
                };
                if primitive == Some(PrimOp::Bytes(BytesOp::Len)) && arguments.len() == 1 {
                    let ECore::Var(local) = arguments[0] else { return None };
                    let (name, _, scheme) = env.locals.iter().rev().find(|(_, id, _)| id == local)?;
                    let ty = self.uni.zonk(&scheme.ty);
                    let carrier = self.refine_view(&ty).and_then(|r| r.base).unwrap_or(ty);
                    if carrier != Ty::Bytes || !scheme.params.is_empty() || env.lookup(name)?.0 != *local { return None }
                    return Some(ITerm::app_fn(BYTE_LENGTH_INDEX, vec![ITerm::Var(name.clone())]))
                }
                let values = arguments.into_iter().map(|argument| self.checked_nat_reading(env, argument, depth + 1)).collect::<Option<Vec<_>>>()?;
                if let Some(operation) = primitive { return Self::checked_nat_primitive_reading(operation, &values) }
                if let ECore::Global(identity, actuals) = head {
                    return if actuals.is_empty() { self.checked_nat_function_reading(identity, &values, depth + 1) } else { None }
                }
                let ECore::DictProj(dictionary, slot, _, _) = head else { return None };
                let ECore::Hole(want) = &**dictionary else { return None };
                let want = self.wants.iter().find(|w| w.id == *want)?;
                let wanted = self.uni.zonk_class(&want.class);
                if wanted.args.iter().any(|ty| !self.uni.free_metas(ty).is_empty()) || want.scope.iter().any(|(_, given)| given.class == wanted.class) { return None }
                let mut selected = None;
                for instance in self.instance_list.iter().filter(|i| i.implicit && !i.base && i.target.class == wanted.class) {
                    // General instance factories need explicit captured-body
                    // reconstruction. Be conservative until it is available.
                    if !instance.params.is_empty() { return None }
                    let mut trial = self.uni.clone();
                    if !self.lattice_match(&instance.target.args, &wanted.args, &mut trial) { continue }
                    if !instance.ctx.is_empty() || selected.is_some() { return None }
                    selected = Some(instance);
                }
                let instance = selected?;
                let definition = self.instance_defs.iter().find(|d| d.id == instance.base_id && d.tparams == 0 && d.subdict_count == 0)?;
                let implementation = definition.methods.get(*slot)?;
                let Some((_, _, scheme)) = instance.method_contracts.iter().find(|(index, _, _)| index == slot) else {
                    return self.checked_nat_function_reading(implementation, &values, depth + 1)
                };
                let body = self.guarded_nat_body(implementation, scheme)?;
                if body.parameters as usize != values.len() { return None }
                fn instantiate(expression: &crate::interface::GuardedNatExpr, values: &[ITerm]) -> Option<ITerm> {
                    use crate::interface::GuardedNatExpr as E;
                    Some(match expression {
                        E::Argument(slot) => values.get(*slot as usize)?.clone(),
                        E::Subtract(a, b) => ITerm::Bin(IdxOp::Sub, Box::new(instantiate(a, values)?), Box::new(instantiate(b, values)?)),
                    })
                }
                instantiate(&body.root, &values)
            }
            _ => None,
        }
    }
    fn checked_nat_primitive_reading(operation: PrimOp, values: &[ITerm]) -> Option<ITerm> {
        let [a, b] = values else { return None };
        let operation = match operation { PrimOp::Nat(NatOp::Add) => IdxOp::Add, PrimOp::Nat(NatOp::Sub) => IdxOp::Sub, PrimOp::Nat(NatOp::Mul) => IdxOp::Mul, _ => return None };
        Some(ITerm::Bin(operation, Box::new(a.clone()), Box::new(b.clone())))
    }
    fn checked_nat_function_reading(&self, identity: &GlobalId, values: &[ITerm], depth: usize) -> Option<ITerm> {
        if depth > 64 { return None }
        if let Some(function) = self.fns.iter().find(|f| f.id == *identity) {
            if let Some(operation) = function.prim {
                if function.params.len() != values.len() { return None }
                return Self::checked_nat_primitive_reading(operation, values)
            }
        }
        let definition = self.interface_total_bodies.get(&identity.0)?;
        if definition.result != crate::total::Sort::Nat || definition.parameters.len() != values.len() || definition.parameters.iter().any(|s| *s != crate::total::Sort::Nat) { return None }
        self.checked_nat_pending_reading(&definition.body, values, depth + 1)
    }
    fn checked_nat_pending_reading(&self, body: &crate::total::PendingExpr, values: &[ITerm], depth: usize) -> Option<ITerm> {
        use crate::total::{PendingExpr as P, Op, Sort};
        if depth > 64 { return None }
        Some(match body {
            P::Local(slot) => values.get(*slot)?.clone(),
            P::Number(Sort::Nat, number) => ITerm::Num(number.clone()),
            P::Primitive(operation, arguments) => {
                let operation = match operation { Op::NatAdd => PrimOp::Nat(NatOp::Add), Op::NatMul => PrimOp::Nat(NatOp::Mul), _ => return None };
                let arguments = arguments.iter().map(|argument| self.checked_nat_pending_reading(argument, values, depth + 1)).collect::<Option<Vec<_>>>()?;
                Self::checked_nat_primitive_reading(operation, &arguments)?
            }
            P::Call(identity, arguments) => {
                let arguments = arguments.iter().map(|argument| self.checked_nat_pending_reading(argument, values, depth + 1)).collect::<Option<Vec<_>>>()?;
                self.checked_nat_function_reading(&GlobalId(identity.clone()), &arguments, depth + 1)?
            }
            P::Selected { dictionary, slot, arguments } => {
                let definition = self.instance_defs.iter().find(|d| d.id.0 == *dictionary && d.tparams == 0 && d.subdict_count == 0)?;
                let implementation = definition.methods.get(*slot)?;
                let arguments = arguments.iter().map(|argument| self.checked_nat_pending_reading(argument, values, depth + 1)).collect::<Option<Vec<_>>>()?;
                self.checked_nat_function_reading(implementation, &arguments, depth + 1)?
            }
            P::Let(value, body) => {
                let value = self.checked_nat_pending_reading(value, values, depth + 1)?;
                let mut nested = values.to_vec(); nested.push(value);
                self.checked_nat_pending_reading(body, &nested, depth + 1)?
            }
            _ => return None,
        })
    }

}

// Native producer for portable total bodies. Ordinary callable schemes and
// class/slot spellings grant no semantics; resolve the actual closed method
// field, then inspect its checked body and fixed primitive operations.
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
        ECore::InstDict(id, contexts, actuals) if contexts.is_empty() && actuals.is_empty() => Some(id.0.clone()),
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

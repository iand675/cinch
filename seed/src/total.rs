//! Checked, closed first-order bodies for stable numeric observations.
//!
//! This representation carries executable semantics, not a purity hint or a
//! class-name convention. It is deliberately independent of source spelling.
//! Consumers must validate the whole dependency graph before using a call as
//! an index. No schema-7 interface admits these bodies yet.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sort { Nat, Int, Bool, Bytes, I64 }

/// Every admitted operation has a fixed total runtime-ABI meaning. In
/// particular there is no wrapping arithmetic, partial Nat subtraction,
/// division, buffer-content read, allocation, foreign call or arbitrary prim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    NatAdd, NatMul, NatCmp,
    IntAdd, IntSub, IntMul, IntNeg, IntCmp, IntFromNat, IntFromI64,
    I64Eq, I64Ne, I64Lt, I64Le, I64Gt, I64Ge,
    BoolEq, BoolNe, BoolNot, BoolAnd, BoolOr,
    BytesLen,
}
impl Op {
    pub fn signature(self) -> (&'static [Sort], Sort) {
        use Sort::*;
        match self {
            Self::NatAdd | Self::NatMul => (&[Nat, Nat], Nat),
            Self::NatCmp => (&[Nat, Nat], I64),
            Self::IntAdd | Self::IntSub | Self::IntMul => (&[Int, Int], Int),
            Self::IntNeg => (&[Int], Int),
            Self::IntCmp => (&[Int, Int], I64),
            Self::IntFromNat => (&[Nat], Int),
            Self::IntFromI64 => (&[I64], Int),
            Self::I64Eq | Self::I64Ne | Self::I64Lt | Self::I64Le | Self::I64Gt | Self::I64Ge => (&[I64, I64], Bool),
            Self::BoolEq | Self::BoolNe | Self::BoolAnd | Self::BoolOr => (&[Bool, Bool], Bool),
            Self::BoolNot => (&[Bool], Bool),
            Self::BytesLen => (&[Bytes], Nat),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    /// A lexical slot: parameters first, then strict-let binders in scope.
    Local(usize),
    Number { sort: Sort, decimal: String },
    Bool(bool),
    Primitive(Op, Vec<Expr>),
    Call { target: String, arguments: Vec<Expr> },
    Let { value: Box<Expr>, body: Box<Expr> },
    If { condition: Box<Expr>, yes: Box<Expr>, no: Box<Expr> },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Definition {
    pub identity: String,
    pub parameters: Vec<Sort>,
    pub result: Sort,
    pub body: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invalid(pub String);

/// The constructor is private. A valid graph is closed, well typed and
/// acyclic; recursively calling a pure-typed function cannot manufacture it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CheckedGraph { definitions: BTreeMap<String, Definition> }

fn number(sort: Sort, decimal: &str) -> bool {
    let digits = decimal.strip_prefix('-').unwrap_or(decimal);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) { return false }
    match sort {
        Sort::Nat => !decimal.starts_with('-'),
        Sort::Int => true,
        Sort::I64 => decimal.parse::<i64>().is_ok(),
        _ => false,
    }
}

impl CheckedGraph {
    pub fn check(definitions: Vec<Definition>) -> Result<Self, Invalid> {
        let mut by_name = BTreeMap::new();
        for definition in definitions {
            if definition.identity.is_empty() || by_name.insert(definition.identity.clone(), definition).is_some() {
                return Err(Invalid("empty or duplicate total-body identity".into()));
            }
        }
        let graph = Self { definitions: by_name };
        for definition in graph.definitions.values() {
            let inferred = graph.infer(&definition.body, &definition.parameters)?;
            if inferred != definition.result { return Err(Invalid(format!("{}: body result sort disagrees with its declaration", definition.identity))) }
        }
        let mut done = BTreeSet::new();
        for identity in graph.definitions.keys() { graph.acyclic(identity, &mut BTreeSet::new(), &mut done)?; }
        Ok(graph)
    }
    pub fn empty() -> Self { Self { definitions: BTreeMap::new() } }
    /// Incremental sessions can retain ordinary callers when a callee body
    /// changes. Rebuild their semantic closure from the current definitions;
    /// missing/invalid/cyclic bodies lose authority instead of keeping stale
    /// transitive contracts. Conflicting duplicate identities are discarded.
    pub fn closed_subset(definitions: Vec<Definition>) -> Self {
        use std::collections::btree_map::Entry;

        let mut pending = BTreeMap::new();
        let mut conflicts = BTreeSet::new();
        for definition in definitions {
            match pending.entry(definition.identity.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert((definition, 0));
                }
                Entry::Occupied(entry) if entry.get().0 != definition => {
                    conflicts.insert(definition.identity);
                }
                Entry::Occupied(_) => {}
            }
        }
        for name in conflicts {
            pending.remove(&name);
        }

        // Admit a definition only after every dependency was admitted.
        // Missing or invalid dependencies and cycles never reach the queue.
        let mut users = BTreeMap::<_, Vec<String>>::new();
        let mut ready = Vec::new();
        for (name, (definition, waiting)) in &mut pending {
            let mut calls = BTreeSet::new();
            dependencies(&definition.body, &mut calls);
            *waiting = calls.len();
            if calls.is_empty() {
                ready.push(name.clone());
            }
            for callee in calls {
                users.entry(callee.to_owned()).or_default().push(name.clone());
            }
        }
        let mut graph = Self::empty();
        while let Some(name) = ready.pop() {
            let (definition, _) = pending.remove(&name).expect("ready definition is pending");
            if graph.infer(&definition.body, &definition.parameters).ok() != Some(definition.result) {
                continue;
            }
            let dependents = users.remove(&name).unwrap_or_default();
            graph.definitions.insert(name, definition);
            for dependent in dependents {
                let (_, waiting) = pending.get_mut(&dependent).expect("dependent is pending");
                *waiting -= 1;
                if *waiting == 0 {
                    ready.push(dependent);
                }
            }
        }
        graph
    }
    /// Add a body only after all its callees. Inference uses checked existing
    /// definitions; a self/forward reference has no authority in this graph.
    pub fn define(&mut self, identity: String, parameters: Vec<Sort>, body: Expr) -> Result<(), Invalid> {
        if identity.is_empty() || self.definitions.contains_key(&identity) { return Err(Invalid("empty or duplicate total-body identity".into())) }
        let result = self.infer(&body, &parameters)?;
        self.definitions.insert(identity.clone(), Definition { identity, parameters, result, body });
        Ok(())
    }
    /// Project only the requested definitions and their complete dependencies.
    /// The caller must choose roots from the owning package's checked exports;
    /// this operation proves closure, not ownership or a declared refinement.
    pub fn closure<'a>(&self, roots: impl IntoIterator<Item=&'a str>) -> Result<Self, Invalid> {
        let mut pending = roots.into_iter().map(str::to_string).collect::<Vec<_>>();
        let mut selected = BTreeMap::new();
        while let Some(identity) = pending.pop() {
            if selected.contains_key(&identity) { continue }
            let definition = self.get(&identity)
                .ok_or_else(|| Invalid(format!("missing total-body root: {identity}")))?;
            let mut calls = BTreeSet::new();
            dependencies(&definition.body, &mut calls);
            pending.extend(calls.into_iter().map(str::to_owned));
            selected.insert(identity, definition.clone());
        }
        // A subgraph of an already checked DAG remains checked when every
        // referenced callee is retained. It cannot introduce new semantics.
        Ok(Self { definitions: selected })
    }
    /// Check whether every definition belongs to this root's closure.
    /// This walks an already checked graph without copying any of its bodies.
    pub(crate) fn closure_is_whole(&self, identity: &str) -> Result<bool, Invalid> {
        let mut pending = vec![identity];
        let mut seen = BTreeSet::new();
        while let Some(name) = pending.pop() {
            if !seen.insert(name) {
                continue;
            }
            let definition = self.get(name)
                .ok_or_else(|| Invalid(format!("missing total-body root: {name}")))?;
            let mut calls = BTreeSet::new();
            dependencies(&definition.body, &mut calls);
            pending.extend(calls);
        }
        Ok(seen.len() == self.definitions.len())
    }
    pub fn definitions(&self) -> impl Iterator<Item=&Definition> { self.definitions.values() }
    pub fn get(&self, identity: &str) -> Option<&Definition> { self.definitions.get(identity) }
    pub fn infer(&self, expression: &Expr, scope: &[Sort]) -> Result<Sort, Invalid> {
        let fail = || Invalid("ill-scoped or ill-sorted total body".into());
        match expression {
            Expr::Local(slot) => scope.get(*slot).copied().ok_or_else(fail),
            Expr::Number { sort, decimal } => if number(*sort, decimal) { Ok(*sort) } else { Err(fail()) },
            Expr::Bool(_) => Ok(Sort::Bool),
            Expr::Primitive(operation, arguments) => {
                let (parameters, result) = operation.signature();
                self.arguments(arguments, parameters, scope)?; Ok(result)
            }
            Expr::Call { target, arguments } => {
                let callee = self.get(target).ok_or_else(|| Invalid(format!("missing total body: {target}")))?;
                self.arguments(arguments, &callee.parameters, scope)?; Ok(callee.result)
            }
            Expr::Let { value, body } => {
                let sort = self.infer(value, scope)?;
                let mut nested = scope.to_vec(); nested.push(sort);
                self.infer(body, &nested)
            }
            Expr::If { condition, yes, no } => {
                if self.infer(condition, scope)? != Sort::Bool { return Err(fail()) }
                let yes = self.infer(yes, scope)?;
                if yes != self.infer(no, scope)? { return Err(fail()) }
                Ok(yes)
            }
        }
    }
    fn arguments(&self, arguments: &[Expr], parameters: &[Sort], scope: &[Sort]) -> Result<(), Invalid> {
        if arguments.len() != parameters.len() { return Err(Invalid("total call/primitive arity mismatch".into())) }
        for (argument, expected) in arguments.iter().zip(parameters) {
            if self.infer(argument, scope)? != *expected { return Err(Invalid("total call/primitive argument sort mismatch".into())) }
        }
        Ok(())
    }
    fn acyclic<'a>(&'a self, identity: &'a str, visiting: &mut BTreeSet<&'a str>, done: &mut BTreeSet<&'a str>) -> Result<(), Invalid> {
        if done.contains(identity) { return Ok(()) }
        if !visiting.insert(identity) { return Err(Invalid(format!("cyclic total-body dependency: {identity}"))) }
        let mut calls = BTreeSet::new(); dependencies(&self.definitions[identity].body, &mut calls);
        for target in calls { self.acyclic(target, visiting, done)?; }
        visiting.remove(identity); done.insert(identity); Ok(())
    }
    /// Stable structural content, including the exact transitive bodies.
    /// This is the preimage for the interface's eventual versioned digest,
    /// not a cryptographic or semantic-equivalence claim.
    pub fn content(&self, identity: &str) -> Option<String> {
        fn visit<'a>(graph: &'a CheckedGraph, identity: &'a str, seen: &mut BTreeSet<&'a str>, out: &mut String) {
            if !seen.insert(identity) { return }
            let definition = &graph.definitions[identity];
            let mut calls = BTreeSet::new(); dependencies(&definition.body, &mut calls);
            for target in calls { visit(graph, target, seen, out); }
            out.push_str(&format!("total-body-v1:{definition:?}\n"));
        }
        self.get(identity)?;
        let mut out = String::new(); visit(self, identity, &mut BTreeSet::new(), &mut out); Some(out)
    }
}

fn dependencies<'a>(expression: &'a Expr, out: &mut BTreeSet<&'a str>) {
    match expression {
        Expr::Call { target, arguments } => { out.insert(target); for arg in arguments { dependencies(arg, out) } }
        Expr::Primitive(_, arguments) => for arg in arguments { dependencies(arg, out) },
        Expr::Let { value, body } => { dependencies(value, out); dependencies(body, out); }
        Expr::If { condition, yes, no } => { dependencies(condition, out); dependencies(yes, out); dependencies(no, out); }
        _ => {}
    }
}

/// Extraction inputs are retained after Native has checked types and filled
/// dictionary/literal holes, before source scope and local types disappear.
#[derive(Clone, Debug)]
pub(crate) struct PendingDefinition {
    pub identity: String,
    pub parameters: Vec<Sort>,
    pub result: Sort,
    pub body: PendingExpr,
}
#[derive(Clone, Debug)]
pub(crate) enum PendingExpr {
    Local(usize), Number(Sort, String), Bool(bool),
    Primitive(Op, Vec<PendingExpr>),
    Call(String, Vec<PendingExpr>),
    Selected { dictionary: String, slot: usize, arguments: Vec<PendingExpr> },
    Let(Box<PendingExpr>, Box<PendingExpr>),
    If(Box<PendingExpr>, Box<PendingExpr>, Box<PendingExpr>),
}

include!("total_codec.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_subset_retains_only_acyclic_well_sorted_dependency_closures() {
        let define = |identity: &str, body| Definition {
            identity: identity.into(),
            parameters: Vec::new(),
            result: Sort::Nat,
            body,
        };
        let call = |target: &str| Expr::Call {
            target: target.into(),
            arguments: Vec::new(),
        };
        let zero = || Expr::Number { sort: Sort::Nat, decimal: "0".into() };

        let graph = CheckedGraph::closed_subset(vec![
            define("end", Expr::Primitive(Op::NatAdd, vec![call("middle"), call("middle")])),
            define("missing-user", call("missing")),
            define("ill-sorted-user", call("ill-sorted")),
            define("middle", call("start")),
            define("start", zero()),
            define("start", zero()),
            define("cycle-a", call("cycle-b")),
            define("cycle-b", call("cycle-a")),
            define("cycle-user", call("cycle-a")),
            define("recursive", call("recursive")),
            define("ill-sorted", Expr::Bool(true)),
            define("conflict-user", call("conflict")),
            define("conflict", zero()),
            define("conflict", Expr::Bool(false)),
        ]);
        let names: Vec<_> = graph.definitions().map(|definition| definition.identity.as_str()).collect();
        assert_eq!(names, ["end", "middle", "start"]);
        assert!(graph.closure_is_whole("end").unwrap());
        assert!(!graph.closure_is_whole("middle").unwrap());
        assert_eq!(graph.closure_is_whole("missing").unwrap_err().0, "missing total-body root: missing");
    }
}

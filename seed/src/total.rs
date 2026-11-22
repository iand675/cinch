//! Checked, closed first-order bodies for stable numeric observations.
//!
//! This representation carries executable semantics, not a purity hint or a
//! class-name convention. It is deliberately independent of source spelling.
//! Consumers must validate the whole dependency graph before using a call as
//! an index. No schema-7 interface admits these bodies yet.
use crate::package_interfaces::PackedClosure;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

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
#[derive(Clone)]
pub struct CheckedGraph {
    storage: BodyStorage,
}

type Definitions = Arc<BTreeMap<Arc<str>, Arc<CheckedDefinition>>>;

#[derive(Clone)]
enum BodyStorage {
    // A clone shares its membership index. Distinct closures also share
    // checked definitions and their direct edges.
    Owned(Definitions),
    Packed(PackedClosure),
}

impl Default for CheckedGraph {
    fn default() -> Self {
        Self { storage: BodyStorage::Owned(Arc::default()) }
    }
}

impl fmt::Debug for CheckedGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CheckedGraph").field("definitions", self.owned()).finish()
    }
}

impl PartialEq for CheckedGraph {
    fn eq(&self, other: &Self) -> bool {
        self.owned() == other.owned()
    }
}
impl Eq for CheckedGraph {}
impl PartialOrd for CheckedGraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CheckedGraph {
    fn cmp(&self, other: &Self) -> Ordering {
        self.owned().cmp(other.owned())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CheckedDefinition {
    definition: Definition,
    calls: Vec<Box<str>>,
}

impl CheckedDefinition {
    fn new(definition: Definition) -> Self {
        let mut calls = BTreeSet::new();
        dependencies(&definition.body, &mut calls);
        let calls = calls.into_iter().map(Box::from).collect();
        Self { definition, calls }
    }
}

struct BodyScope<'a> {
    parameters: &'a [Sort],
    locals: Vec<Sort>,
}

impl BodyScope<'_> {
    fn get(&self, slot: usize) -> Option<Sort> {
        match slot.checked_sub(self.parameters.len()) {
            Some(local) => self.locals.get(local).copied(),
            None => self.parameters.get(slot).copied(),
        }
    }
}

pub(crate) fn valid_number(sort: Sort, decimal: &str) -> bool {
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
    pub(crate) fn from_packed(closure: PackedClosure) -> Self {
        Self { storage: BodyStorage::Packed(closure) }
    }

    pub(crate) fn write_packed(&self, out: &mut impl fmt::Write) -> fmt::Result {
        match &self.storage {
            BodyStorage::Packed(closure) => closure.write_canonical(out),
            BodyStorage::Owned(_) => Err(fmt::Error),
        }
    }

    fn from_owned(definitions: BTreeMap<Arc<str>, Arc<CheckedDefinition>>) -> Self {
        Self { storage: BodyStorage::Owned(Arc::new(definitions)) }
    }

    fn owned(&self) -> &Definitions {
        match &self.storage {
            BodyStorage::Owned(definitions) => definitions,
            BodyStorage::Packed(closure) => closure.materialized().owned(),
        }
    }

    fn owned_mut(&mut self) -> &mut BTreeMap<Arc<str>, Arc<CheckedDefinition>> {
        if let BodyStorage::Packed(closure) = &self.storage {
            self.storage = BodyStorage::Owned(Arc::clone(closure.materialized().owned()));
        }
        let BodyStorage::Owned(definitions) = &mut self.storage else { unreachable!() };
        Arc::make_mut(definitions)
    }

    pub(crate) fn signature(&self, name: &str) -> Option<(&[Sort], Sort)> {
        match &self.storage {
            BodyStorage::Packed(closure) => closure.signature(name),
            BodyStorage::Owned(_) => self.get(name).map(|entry| (entry.parameters.as_slice(), entry.result)),
        }
    }

    pub fn check(definitions: Vec<Definition>) -> Result<Self, Invalid> {
        let mut by_name = BTreeMap::new();
        for definition in definitions {
            if definition.identity.is_empty() {
                return Err(Invalid("empty or duplicate total-body identity".into()));
            }
            let name = Arc::from(definition.identity.as_str());
            if by_name.insert(name, Arc::new(CheckedDefinition::new(definition))).is_some() {
                return Err(Invalid("empty or duplicate total-body identity".into()));
            }
        }
        let graph = Self::from_owned(by_name);
        for definition in graph.definitions() {
            let inferred = graph.infer(&definition.body, &definition.parameters)?;
            if inferred != definition.result { return Err(Invalid(format!("{}: body result sort disagrees with its declaration", definition.identity))) }
        }
        graph.check_acyclic()?;
        Ok(graph)
    }
    pub fn empty() -> Self { Self::default() }
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
                    entry.insert((CheckedDefinition::new(definition), 0));
                }
                Entry::Occupied(entry) if entry.get().0.definition != definition => {
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
        for (name, (entry, waiting)) in &mut pending {
            *waiting = entry.calls.len();
            if entry.calls.is_empty() {
                ready.push(name.clone());
            }
            for callee in &entry.calls {
                users.entry(callee.as_ref().to_owned()).or_default().push(name.clone());
            }
        }
        let mut graph = Self::empty();
        while let Some(name) = ready.pop() {
            let (entry, _) = pending.remove(&name).expect("ready definition is pending");
            let definition = &entry.definition;
            if graph.infer(&definition.body, &definition.parameters).ok() != Some(definition.result) {
                continue;
            }
            let dependents = users.remove(&name).unwrap_or_default();
            graph.owned_mut().insert(Arc::from(name), Arc::new(entry));
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
        if identity.is_empty() || self.signature(&identity).is_some() { return Err(Invalid("empty or duplicate total-body identity".into())) }
        let result = self.infer(&body, &parameters)?;
        self.owned_mut().insert(
            Arc::from(identity.as_str()),
            Arc::new(CheckedDefinition::new(Definition { identity, parameters, result, body })),
        );
        Ok(())
    }
    /// Project only the requested definitions and their complete dependencies.
    /// The caller must choose roots from the owning package's checked exports;
    /// this operation proves closure, not ownership or a declared refinement.
    pub fn closure<'a>(&'a self, roots: impl IntoIterator<Item = &'a str>) -> Result<Self, Invalid> {
        if let BodyStorage::Packed(closure) = &self.storage {
            return closure.closure(roots).map(Self::from_packed);
        }
        let definitions = self.owned();
        let mut pending = roots.into_iter().collect::<Vec<_>>();
        let mut selected = BTreeMap::new();
        while let Some(identity) = pending.pop() {
            if selected.contains_key(identity) {
                continue;
            }
            let (key, entry) = definitions.get_key_value(identity)
                .ok_or_else(|| Invalid(format!("missing total-body root: {identity}")))?;
            pending.extend(entry.calls.iter().map(Box::as_ref));
            selected.insert(Arc::clone(key), Arc::clone(entry));
        }
        // A subgraph of an already checked DAG remains checked when every
        // referenced callee is retained. It cannot introduce new semantics.
        Ok(Self::from_owned(selected))
    }
    /// Check whether every definition belongs to this root's closure.
    /// This walks an already checked graph without copying any of its bodies.
    pub(crate) fn closure_is_whole(&self, identity: &str) -> Result<bool, Invalid> {
        if let BodyStorage::Packed(closure) = &self.storage {
            return closure.closure_is_whole(identity);
        }
        let definitions = self.owned();
        let mut pending = vec![identity];
        let mut seen = BTreeSet::new();
        while let Some(name) = pending.pop() {
            if !seen.insert(name) {
                continue;
            }
            let entry = definitions.get(name)
                .ok_or_else(|| Invalid(format!("missing total-body root: {name}")))?;
            pending.extend(entry.calls.iter().map(Box::as_ref));
        }
        Ok(seen.len() == definitions.len())
    }
    pub fn definitions(&self) -> impl Iterator<Item = &Definition> {
        self.owned().values().map(|entry| &entry.definition)
    }
    pub fn get(&self, identity: &str) -> Option<&Definition> {
        self.owned().get(identity).map(|entry| &entry.definition)
    }
    pub fn infer(&self, expression: &Expr, parameters: &[Sort]) -> Result<Sort, Invalid> {
        self.infer_in(expression, &mut BodyScope { parameters, locals: Vec::new() })
    }

    fn infer_in(&self, expression: &Expr, scope: &mut BodyScope<'_>) -> Result<Sort, Invalid> {
        let fail = || Invalid("ill-scoped or ill-sorted total body".into());
        match expression {
            Expr::Local(slot) => scope.get(*slot).ok_or_else(fail),
            Expr::Number { sort, decimal } => if valid_number(*sort, decimal) { Ok(*sort) } else { Err(fail()) },
            Expr::Bool(_) => Ok(Sort::Bool),
            Expr::Primitive(operation, arguments) => {
                let (parameters, result) = operation.signature();
                self.arguments(arguments, parameters, scope)?; Ok(result)
            }
            Expr::Call { target, arguments } => {
                let (parameters, result) = self.signature(target).ok_or_else(|| Invalid(format!("missing total body: {target}")))?;
                self.arguments(arguments, parameters, scope)?; Ok(result)
            }
            Expr::Let { value, body } => {
                let sort = self.infer_in(value, scope)?;
                scope.locals.push(sort);
                let result = self.infer_in(body, scope);
                scope.locals.pop();
                result
            }
            Expr::If { condition, yes, no } => {
                if self.infer_in(condition, scope)? != Sort::Bool { return Err(fail()) }
                let yes = self.infer_in(yes, scope)?;
                if yes != self.infer_in(no, scope)? { return Err(fail()) }
                Ok(yes)
            }
        }
    }
    fn arguments(&self, arguments: &[Expr], parameters: &[Sort], scope: &mut BodyScope<'_>) -> Result<(), Invalid> {
        if arguments.len() != parameters.len() { return Err(Invalid("total call/primitive arity mismatch".into())) }
        for (argument, expected) in arguments.iter().zip(parameters) {
            if self.infer_in(argument, scope)? != *expected { return Err(Invalid("total call/primitive argument sort mismatch".into())) }
        }
        Ok(())
    }
    fn check_acyclic(&self) -> Result<(), Invalid> {
        enum Visit<'a> { Enter(&'a str), Leave(&'a str) }

        let mut visiting = BTreeSet::new();
        let mut done = BTreeSet::new();
        let mut pending = Vec::new();
        let definitions = self.owned();
        for root in definitions.keys() {
            pending.push(Visit::Enter(root.as_ref()));
            while let Some(visit) = pending.pop() {
                match visit {
                    Visit::Enter(identity) => {
                        if done.contains(identity) {
                            continue;
                        }
                        if !visiting.insert(identity) {
                            return Err(Invalid(format!("cyclic total-body dependency: {identity}")));
                        }
                        pending.push(Visit::Leave(identity));
                        pending.extend(
                            definitions[identity].calls.iter().rev()
                                .map(|callee| Visit::Enter(callee)),
                        );
                    }
                    Visit::Leave(identity) => {
                        visiting.remove(identity);
                        done.insert(identity);
                    }
                }
            }
        }
        Ok(())
    }
    /// Stable structural content, including the exact transitive bodies.
    /// This is the preimage for the interface's eventual versioned digest,
    /// not a cryptographic or semantic-equivalence claim.
    pub fn content(&self, identity: &str) -> Option<String> {
        use std::fmt::Write;

        enum Visit<'a> {
            Enter(&'a str),
            Leave(&'a Definition),
        }

        self.get(identity)?;
        let mut pending = vec![Visit::Enter(identity)];
        let mut seen = BTreeSet::new();
        let mut output = String::new();
        while let Some(visit) = pending.pop() {
            match visit {
                Visit::Enter(name) if seen.insert(name) => {
                    let entry = &self.owned()[name];
                    pending.push(Visit::Leave(&entry.definition));
                    pending.extend(entry.calls.iter().rev().map(|callee| Visit::Enter(callee)));
                }
                Visit::Enter(_) => (),
                Visit::Leave(definition) => {
                    writeln!(output, "total-body-v1:{definition:?}")
                        .expect("writing to a String cannot fail");
                }
            }
        }
        Some(output)
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
    fn checked_closures_share_bodies_and_let_scopes_restore_their_slots() {
        let mut graph = CheckedGraph::empty();
        graph.define("leaf".into(), vec![Sort::Bool], Expr::If {
            condition: Box::new(Expr::Local(0)),
            yes: Box::new(Expr::Let {
                value: Box::new(Expr::Bool(true)),
                body: Box::new(Expr::Local(1)),
            }),
            no: Box::new(Expr::Local(0)),
        }).unwrap();
        assert!(graph.infer(&Expr::If {
            condition: Box::new(Expr::Bool(false)),
            yes: Box::new(Expr::Let {
                value: Box::new(Expr::Bool(true)),
                body: Box::new(Expr::Local(0)),
            }),
            no: Box::new(Expr::Local(0)),
        }, &[]).is_err());

        let closure = graph.closure(["leaf"]).unwrap();
        assert!(std::ptr::eq(graph.get("leaf").unwrap(), closure.get("leaf").unwrap()));
        assert_eq!(closure.to_json(), graph.to_json());
        let previous = graph.clone();
        graph.define("new".into(), vec![], Expr::Bool(false)).unwrap();
        assert!(previous.get("new").is_none());
        assert_eq!(previous.to_json(), closure.to_json());
    }

    #[test]
    fn package_length_call_chains_are_checked_without_recursing_on_the_stack() {
        let count = 25_000;
        let name = |index| format!("body.{index:06}");
        let definitions = (0..count)
            .map(|index| Definition {
                identity: name(index),
                parameters: Vec::new(),
                result: Sort::Bool,
                body: if index + 1 == count {
                    Expr::Bool(true)
                } else {
                    Expr::Call { target: name(index + 1), arguments: Vec::new() }
                },
            })
            .collect();
        let graph = CheckedGraph::check(definitions).unwrap();
        assert_eq!(graph.closure([name(0).as_str()]).unwrap().definitions().count(), count);
        let content = graph.content(&name(0)).unwrap();
        let mut rows = content.lines();
        assert!(rows.next().unwrap().contains(&name(count - 1)));
        assert!(rows.last().unwrap().contains(&name(0)));
    }

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

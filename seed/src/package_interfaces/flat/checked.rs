//! Verified flat bodies retain byte ranges in the package backing.
use super::{indices, row_offset, word_range, FlatBodies, Node, BODY_ROW, NODE_ROW};
use crate::interface::write_canonical_str;
use crate::package_interfaces::{invalid, words, Decode, PackageBytes, Strings, MAX_DEPTH};
use crate::total::{valid_number, CheckedGraph, Definition, Invalid, Sort};
use std::collections::HashSet;
use std::fmt;
use std::ops::Range;
use std::sync::{Arc, OnceLock};

struct FlatLayout {
    strings: Range<usize>,
    string_rows: Range<usize>,
    bodies: Range<usize>,
    edges: Range<usize>,
    nodes: Range<usize>,
    operands: Range<usize>,
}

impl FlatLayout {
    fn new(table: &FlatBodies<'_>, bytes: &[u8]) -> Self {
        // Every section is sliced from the same file in Package::open.
        let range = |section: &[u8]| {
            let start = section.as_ptr() as usize - bytes.as_ptr() as usize;
            start..start + section.len()
        };
        Self {
            strings: range(table.strings.data),
            string_rows: range(table.strings.spans),
            bodies: range(table.bodies),
            edges: range(table.edges),
            nodes: range(table.nodes),
            operands: range(table.operands),
        }
    }

    fn view<'a>(&self, bytes: &'a [u8]) -> FlatBodies<'a> {
        FlatBodies {
            strings: Strings {
                data: &bytes[self.strings.clone()],
                spans: &bytes[self.string_rows.clone()],
            },
            bodies: &bytes[self.bodies.clone()],
            edges: &bytes[self.edges.clone()],
            nodes: &bytes[self.nodes.clone()],
            operands: &bytes[self.operands.clone()],
        }
    }
}

struct BodyRow {
    parameters: Box<[Sort]>,
    result: Sort,
    root: u32,
    first_edge: u32,
    edge_count: u32,
}

pub(in crate::package_interfaces) struct CheckedTable {
    bytes: Arc<PackageBytes>,
    layout: FlatLayout,
    rows: Vec<BodyRow>,
}

#[derive(Clone)]
pub(crate) struct PackedClosure {
    table: Arc<CheckedTable>,
    projection: Arc<Projection>,
}

struct Projection {
    definitions: Vec<u32>,
    owned: OnceLock<CheckedGraph>,
}

impl FlatBodies<'_> {
    pub(in crate::package_interfaces) fn check(
        &self,
        bytes: Arc<PackageBytes>,
    ) -> Decode<Arc<CheckedTable>> {
        let mut rows = Vec::with_capacity(self.bodies.len() / BODY_ROW);
        let mut previous = None;
        for row in self.bodies.chunks_exact(BODY_ROW) {
            let [name, result, first, count, root, first_edge, edge_count, reserved] =
                words(row, 0)?;
            let identity = self.strings.get(name)?;
            if identity.is_empty()
                || previous.is_some_and(|earlier| earlier >= identity)
                || reserved != 0
            {
                return Err(invalid("invalid packed body definition row"));
            }
            previous = Some(identity);
            let parameters = indices(word_range(self.operands, first, count)?)
                .map(|sort| self.sort(sort))
                .collect::<Decode<_>>()?;
            rows.push(BodyRow {
                parameters,
                result: self.sort(result)?,
                root,
                first_edge,
                edge_count,
            });
        }

        let mut check = BodyCheck {
            table: self,
            rows: &rows,
            parameters: &[],
            locals: Vec::new(),
            targets: Vec::new(),
            next: 0,
        };
        for (id, row) in rows.iter().enumerate() {
            check.parameters = &row.parameters;
            let result = check.expression(row.root, u32::MAX, MAX_DEPTH)?;
            if result != row.result {
                return Err(invalid(&format!(
                    "{}: body result sort disagrees with its declaration",
                    self.definition_name(id as u32)?
                )));
            }
            check.targets.sort_unstable();
            check.targets.dedup();
            if !indices(word_range(self.edges, row.first_edge, row.edge_count)?)
                .eq(check.targets.drain(..))
            {
                return Err(invalid(
                    "packed body dependencies disagree with its expression",
                ));
            }
        }
        if check.next as usize != self.nodes.len() / NODE_ROW {
            return Err(invalid("unreferenced packed body node"));
        }
        verify_acyclic(self, &rows)?;
        let layout = FlatLayout::new(self, bytes.bytes());
        Ok(Arc::new(CheckedTable {
            bytes,
            layout,
            rows,
        }))
    }
}

struct BodyCheck<'a, 'b> {
    table: &'b FlatBodies<'a>,
    rows: &'b [BodyRow],
    parameters: &'b [Sort],
    locals: Vec<Sort>,
    targets: Vec<u32>,
    next: u32,
}

impl BodyCheck<'_, '_> {
    fn expression(&mut self, id: u32, parent: u32, depth: usize) -> Decode<Sort> {
        if depth == 0 {
            return Err(invalid("over-nested packed body"));
        }
        if id as usize >= self.table.nodes.len() / NODE_ROW {
            return Err(invalid("packed body node outside its section"));
        }
        if id < self.next || id >= parent {
            return Err(invalid("packed body child is not in postorder"));
        }
        let depth = depth - 1;
        let mismatch = || invalid("ill-scoped or ill-sorted total body");
        let sort = match self.table.node(id)? {
            Node::Local(slot) => {
                let slot = slot as usize;
                let found = match slot.checked_sub(self.parameters.len()) {
                    Some(local) => self.locals.get(local),
                    None => self.parameters.get(slot),
                };
                found.copied().ok_or_else(mismatch)?
            }
            Node::Number(sort, decimal) => {
                if !valid_number(sort, decimal) {
                    return Err(mismatch());
                }
                sort
            }
            Node::Bool(_) => Sort::Bool,
            Node::Primitive(op, args) => {
                let (parameters, result) = op.signature();
                self.arguments(id, depth, args, parameters)?;
                result
            }
            Node::Call(target, args) => {
                let rows = self.rows;
                let callee = rows
                    .get(target as usize)
                    .ok_or_else(|| invalid("packed body definition outside its section"))?;
                self.arguments(id, depth, args, &callee.parameters)?;
                self.targets.push(target);
                callee.result
            }
            Node::Let(value, body) => {
                let sort = self.expression(value, id, depth)?;
                self.locals.push(sort);
                let body_sort = self.expression(body, id, depth)?;
                self.locals.pop();
                body_sort
            }
            Node::If(condition, yes, no) => {
                if self.expression(condition, id, depth)? != Sort::Bool {
                    return Err(mismatch());
                }
                let yes = self.expression(yes, id, depth)?;
                if self.expression(no, id, depth)? != yes {
                    return Err(mismatch());
                }
                yes
            }
        };
        if self.next != id {
            return Err(invalid("unreferenced packed body node"));
        }
        self.next += 1;
        Ok(sort)
    }

    fn arguments(&mut self, id: u32, depth: usize, args: &[u8], parameters: &[Sort]) -> Decode<()> {
        if args.len() / 4 != parameters.len() {
            return Err(invalid("total call/primitive arity mismatch"));
        }
        for (node, expected) in indices(args).zip(parameters) {
            if self.expression(node, id, depth)? != *expected {
                return Err(invalid("total call/primitive argument sort mismatch"));
            }
        }
        Ok(())
    }
}

fn verify_acyclic(table: &FlatBodies<'_>, rows: &[BodyRow]) -> Decode<()> {
    enum State {
        Unvisited,
        Visiting,
        Done,
    }
    let mut state: Vec<_> = (0..rows.len()).map(|_| State::Unvisited).collect();
    let mut pending = Vec::new();
    for id in 0..rows.len() as u32 {
        pending.push((id, false));
        while let Some((id, leaving)) = pending.pop() {
            let state = &mut state[id as usize];
            if leaving {
                *state = State::Done;
                continue;
            }
            match state {
                State::Done => continue,
                State::Visiting => {
                    return Err(invalid(&format!(
                        "cyclic total-body dependency: {}",
                        table.definition_name(id)?
                    )));
                }
                State::Unvisited => *state = State::Visiting,
            }
            let row = &rows[id as usize];
            pending.push((id, true));
            pending.extend(
                indices(word_range(table.edges, row.first_edge, row.edge_count)?)
                    .rev()
                    .map(|target| (target, false)),
            );
        }
    }
    Ok(())
}

impl CheckedTable {
    fn view(&self) -> FlatBodies<'_> {
        self.layout.view(self.bytes.bytes())
    }

    pub(in crate::package_interfaces) fn definition_name(&self, id: u32) -> Decode<&str> {
        self.view().definition_name(id)
    }

    fn reachable(&self, mut pending: Vec<u32>) -> Vec<u32> {
        let view = self.view();
        let mut visited = HashSet::new();
        while let Some(id) = pending.pop() {
            if visited.insert(id) {
                let row = &self.rows[id as usize];
                let edges = word_range(view.edges, row.first_edge, row.edge_count)
                    .expect("checked dependency span");
                pending.extend(indices(edges));
            }
        }
        let mut definitions: Vec<_> = visited.into_iter().collect();
        definitions.sort_unstable();
        definitions
    }

    pub(in crate::package_interfaces) fn closure_by_id(
        self: &Arc<Self>,
        id: u32,
    ) -> Decode<PackedClosure> {
        let row = self
            .rows
            .get(id as usize)
            .ok_or_else(|| invalid("packed body definition outside its section"))?;
        let definitions = if row.edge_count == 0 {
            vec![id]
        } else {
            self.reachable(vec![id])
        };
        Ok(PackedClosure::new(self, definitions))
    }
}

impl PackedClosure {
    fn new(table: &Arc<CheckedTable>, definitions: Vec<u32>) -> Self {
        Self {
            table: Arc::clone(table),
            projection: Arc::new(Projection {
                definitions,
                owned: OnceLock::new(),
            }),
        }
    }

    fn definition_id(&self, name: &str) -> Option<u32> {
        let view = self.table.view();
        let ids = &self.projection.definitions;
        let at = ids
            .binary_search_by(|id| {
                view.definition_name(*id)
                    .expect("checked identity")
                    .cmp(name)
            })
            .ok()?;
        Some(ids[at])
    }

    pub(crate) fn signature(&self, name: &str) -> Option<(&[Sort], Sort)> {
        let row = &self.table.rows[self.definition_id(name)? as usize];
        Some((&row.parameters, row.result))
    }

    pub(crate) fn closure<'a>(
        &self,
        roots: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, Invalid> {
        let mut ids = Vec::new();
        for name in roots {
            ids.push(
                self.definition_id(name)
                    .ok_or_else(|| Invalid(format!("missing total-body root: {name}")))?,
            );
        }
        Ok(Self::new(&self.table, self.table.reachable(ids)))
    }

    pub(crate) fn closure_is_whole(&self, name: &str) -> Result<bool, Invalid> {
        let id = self
            .definition_id(name)
            .ok_or_else(|| Invalid(format!("missing total-body root: {name}")))?;
        Ok(self.table.reachable(vec![id]).len() == self.projection.definitions.len())
    }

    pub(crate) fn materialized(&self) -> &CheckedGraph {
        self.projection.owned.get_or_init(|| {
            let view = self.table.view();
            let definitions = self
                .projection
                .definitions
                .iter()
                .map(|&id| {
                    let row = &self.table.rows[id as usize];
                    Definition {
                        identity: view.definition_name(id).expect("checked identity").into(),
                        parameters: row.parameters.to_vec(),
                        result: row.result,
                        body: view.materialize(row.root),
                    }
                })
                .collect();
            CheckedGraph::check(definitions).expect("checked closure")
        })
    }

    pub(crate) fn write_canonical(&self, out: &mut impl fmt::Write) -> fmt::Result {
        let view = self.table.view();
        out.write_str("{\"definitions\":[")?;
        for (index, id) in self.projection.definitions.iter().copied().enumerate() {
            if index != 0 {
                out.write_char(',')?;
            }
            let row = &self.table.rows[id as usize];
            out.write_str("{\"body\":")?;
            view.write_expression(out, row.root)?;
            out.write_str(",\"identity\":")?;
            write_canonical_str(out, view.definition_name(id).map_err(|_| fmt::Error)?)?;
            out.write_str(",\"parameters\":[")?;
            for (i, parameter) in row.parameters.iter().enumerate() {
                if i != 0 {
                    out.write_char(',')?;
                }
                write_canonical_str(out, parameter.name())?;
            }
            write!(out, "],\"result\":\"{}\"}}", row.result.name())?;
        }
        out.write_str("],\"version\":1}")
    }
}

impl FlatBodies<'_> {
    fn write_expression(&self, out: &mut impl fmt::Write, id: u32) -> fmt::Result {
        match self.node(id).map_err(|_| fmt::Error)? {
            Node::Local(slot) => write!(out, "{{\"node\":\"local\",\"slot\":{slot}}}"),
            Node::Number(sort, decimal) => {
                out.write_str("{\"decimal\":")?;
                write_canonical_str(out, decimal)?;
                write!(out, ",\"node\":\"number\",\"sort\":\"{}\"}}", sort.name())
            }
            Node::Bool(flag) => write!(out, "{{\"node\":\"bool\",\"value\":{flag}}}"),
            Node::Primitive(op, args) => {
                self.write_arguments(out, args)?;
                write!(out, ",\"node\":\"primitive\",\"operation\":\"{op:?}\"}}")
            }
            Node::Call(target, args) => {
                self.write_arguments(out, args)?;
                out.write_str(",\"node\":\"call\",\"target\":")?;
                write_canonical_str(out, self.definition_name(target).map_err(|_| fmt::Error)?)?;
                out.write_char('}')
            }
            Node::Let(value, body) => {
                out.write_str("{\"body\":")?;
                self.write_expression(out, body)?;
                out.write_str(",\"node\":\"let\",\"value\":")?;
                self.write_expression(out, value)?;
                out.write_char('}')
            }
            Node::If(condition, yes, no) => {
                out.write_str("{\"condition\":")?;
                self.write_expression(out, condition)?;
                out.write_str(",\"no\":")?;
                self.write_expression(out, no)?;
                out.write_str(",\"node\":\"if\",\"yes\":")?;
                self.write_expression(out, yes)?;
                out.write_char('}')
            }
        }
    }

    fn write_arguments(&self, out: &mut impl fmt::Write, args: &[u8]) -> fmt::Result {
        out.write_str("{\"arguments\":[")?;
        for (i, id) in indices(args).enumerate() {
            if i != 0 {
                out.write_char(',')?;
            }
            self.write_expression(out, id)?;
        }
        out.write_char(']')
    }
}

#[cfg(test)]
mod tests;

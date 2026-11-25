//! Publish a CINCHPK4 interface set from checked compiler artifacts.
use super::{invalid, Decode, HEADER, MAX_DEPTH};
use crate::interface::{Iface, JsonValue as Value};
use crate::total::{CheckedGraph, Definition, Expr};
use std::borrow::Cow;
use std::collections::{hash_map::Entry, BTreeSet, HashMap};
use std::io::Write;
use std::path::Path;

fn word(value: usize) -> Decode<u32> {
    u32::try_from(value).map_err(|_| invalid("interface package exceeds the 32-bit format"))
}

fn put(out: &mut Vec<u8>, words: impl IntoIterator<Item = u32>) {
    for word in words {
        out.extend_from_slice(&word.to_le_bytes());
    }
}

#[derive(Default)]
struct Strings<'a> {
    ids: HashMap<Cow<'a, str>, u32>,
    spans: Vec<[u32; 2]>,
    bytes: Vec<u8>,
}

impl<'a> Strings<'a> {
    fn id(&mut self, text: &'a str) -> Decode<u32> {
        self.intern(text, Cow::Borrowed)
    }

    fn copied_id(&mut self, text: &str) -> Decode<u32> {
        self.intern(text, |text| Cow::Owned(text.to_owned()))
    }

    fn intern<'text>(
        &mut self,
        text: &'text str,
        retain: impl FnOnce(&'text str) -> Cow<'a, str>,
    ) -> Decode<u32> {
        if let Some(&id) = self.ids.get(text) {
            return Ok(id);
        }
        let id = word(self.spans.len())?;
        self.spans
            .push([word(self.bytes.len())?, word(text.len())?]);
        self.bytes.extend_from_slice(text.as_bytes());
        self.ids.insert(retain(text), id);
        Ok(id)
    }

    fn span(&mut self, text: &'a str) -> Decode<[u32; 2]> {
        let id = self.id(text)?;
        Ok(self.spans[id as usize])
    }

    fn section(self) -> Decode<Vec<u8>> {
        let mut data = Vec::new();
        put(&mut data, [word(self.spans.len())?]);
        put(&mut data, self.spans.into_iter().flatten());
        data.extend(self.bytes);
        Ok(data)
    }
}

fn collect_definitions<'a>(
    value: &'a Value,
    definitions: &mut HashMap<&'a str, &'a Definition>,
) -> Decode<()> {
    match value {
        Value::PackedGraph(graph) => {
            for definition in graph.definitions() {
                match definitions.entry(&definition.identity) {
                    Entry::Vacant(entry) => {
                        entry.insert(definition);
                    }
                    Entry::Occupied(entry) => {
                        let prior = *entry.get();
                        if !std::ptr::eq(prior, definition) && prior != definition {
                            return Err(invalid("conflicting checked body definitions"));
                        }
                    }
                }
            }
        }
        Value::Arr(items) => {
            for item in items {
                collect_definitions(item, definitions)?;
            }
        }
        Value::Obj(fields) => {
            for child in fields.values() {
                collect_definitions(child, definitions)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[derive(Default)]
struct Writer<'a> {
    strings: Strings<'a>,
    body_ids: HashMap<&'a str, u32>,
    nodes: Vec<u8>,
    operands: Vec<u8>,
    argument_ids: Vec<u32>,
    edges: Vec<u8>,
}

impl<'a> Writer<'a> {
    fn body_id(&self, name: &str) -> Decode<u32> {
        self.body_ids
            .get(name)
            .copied()
            .ok_or_else(|| invalid("checked body references a missing definition"))
    }

    fn expr(&mut self, expression: &'a Expr, edges: &mut BTreeSet<u32>, depth: usize) -> Decode<u32> {
        if depth >= MAX_DEPTH {
            return Err(invalid("over-nested packed body"));
        }
        let next = depth + 1;
        let node = match expression {
            Expr::Local(slot) => [0, word(*slot)?, 0, 0],
            Expr::Number { sort, decimal } => [
                1,
                self.strings.id(sort.name())?,
                self.strings.id(decimal)?,
                0,
            ],
            Expr::Bool(value) => [2, u32::from(*value), 0, 0],
            Expr::Primitive(op, arguments) => {
                let (at, len) = self.arguments(arguments, edges, next)?;
                [3, self.strings.id(op.name())?, at, len]
            }
            Expr::Call { target, arguments } => {
                let callee = self.body_id(target)?;
                edges.insert(callee);
                let (at, len) = self.arguments(arguments, edges, next)?;
                [4, callee, at, len]
            }
            Expr::Let { value, body } => [
                5,
                self.expr(value, edges, next)?,
                self.expr(body, edges, next)?,
                0,
            ],
            Expr::If { condition, yes, no } => [
                6,
                self.expr(condition, edges, next)?,
                self.expr(yes, edges, next)?,
                self.expr(no, edges, next)?,
            ],
        };
        let id = word(self.nodes.len() / 16)?;
        put(&mut self.nodes, node);
        Ok(id)
    }

    fn arguments(
        &mut self,
        arguments: &'a [Expr],
        edges: &mut BTreeSet<u32>,
        depth: usize,
    ) -> Decode<(u32, u32)> {
        // A nested expression drains only its own suffix. The earlier
        // sibling roots stay in place until this argument list is complete.
        let start = self.argument_ids.len();
        for argument in arguments {
            let id = self.expr(argument, edges, depth)?;
            self.argument_ids.push(id);
        }
        let at = word(self.operands.len() / 4)?;
        let len = word(self.argument_ids.len() - start)?;
        put(&mut self.operands, self.argument_ids.drain(start..));
        Ok((at, len))
    }

    fn definition(&mut self, definition: &'a Definition, rows: &mut Vec<u8>) -> Decode<()> {
        let params = word(self.operands.len() / 4)?;
        let arity = word(definition.parameters.len())?;
        for sort in &definition.parameters {
            put(&mut self.operands, [self.strings.id(sort.name())?]);
        }
        let mut edges = BTreeSet::new();
        let root = self.expr(&definition.body, &mut edges, 0)?;
        put(
            rows,
            [
                self.strings.id(&definition.identity)?,
                self.strings.id(definition.result.name())?,
                params,
                arity,
                root,
                word(self.edges.len() / 4)?,
                word(edges.len())?,
                0,
            ],
        );
        put(&mut self.edges, edges);
        Ok(())
    }

    fn body_reference(&self, name: &str, graph: &CheckedGraph) -> Decode<u32> {
        if !graph
            .closure_is_whole(name)
            .map_err(|err| invalid(&err.0))?
        {
            return Err(invalid(
                "checked body document contains unrelated definitions",
            ));
        }
        self.body_id(name)
    }

    fn value<'text>(
        &mut self,
        value: &'text Value,
        out: &mut Vec<u8>,
        use_body_ids: bool,
        depth: usize,
        intern: impl Fn(&mut Strings<'a>, &'text str) -> Decode<u32> + Copy,
    ) -> Decode<()> {
        if depth > MAX_DEPTH {
            return Err(invalid("over-nested interface document"));
        }
        let next = depth + 1;
        match value {
            Value::Null => put(out, [0]),
            Value::Bool(value) => put(out, [1 + u32::from(*value)]),
            Value::Num(value) => put(out, [3, *value as u32, (*value >> 32) as u32]),
            Value::Str(text) => put(out, [4, intern(&mut self.strings, text)?]),
            Value::Arr(items) => {
                put(out, [5, word(items.len())?]);
                for item in items {
                    self.value(item, out, use_body_ids, next, intern)?;
                }
            }
            Value::Obj(fields) => {
                put(out, [6, word(fields.len())?]);
                for (name, field) in fields {
                    put(out, [intern(&mut self.strings, name)?]);
                    if use_body_ids && name == "total_body" {
                        if let Value::PackedGraph(graph) = field {
                            let root = fields
                                .get("symbol")
                                .or_else(|| fields.get("implementation"))
                                .or_else(|| fields.get("identity"))
                                .ok_or_else(|| invalid("checked body document lacks a root name"))?
                                .as_str()?;
                            put(out, [9, self.body_reference(root, graph)?]);
                            continue;
                        }
                    }
                    self.value(field, out, use_body_ids, next, intern)?;
                }
            }
            Value::PackedGraph(graph) if !use_body_ids => {
                // Temporary ABI graph documents cannot borrow their strings.
                self.value(&graph.materialized_json_value(), out, false, depth, Strings::copied_id)?;
            }
            _ => return Err(invalid("unsupported value in packed interface document")),
        }
        Ok(())
    }
}

struct EncodedPackage {
    header: Vec<u8>,
    sections: [Vec<u8>; 9],
    length: usize,
}

impl EncodedPackage {
    fn write_to(&self, mut output: impl Write) -> std::io::Result<()> {
        output.write_all(&self.header)?;
        for section in &self.sections {
            output.write_all(section)?;
        }
        Ok(())
    }
}

pub(crate) fn encode_interfaces(rows: &[(&Iface, &str)]) -> crate::Result<Vec<u8>> {
    let encoded = encode(rows, None).map_err(|err| crate::Error::plain(err.to_string()))?;
    let mut bytes = Vec::with_capacity(encoded.length);
    encoded.write_to(&mut bytes).expect("writing a Vec cannot fail");
    Ok(bytes)
}

fn encode(rows: &[(&Iface, &str)], abi: Option<&Value>) -> Decode<EncodedPackage> {
    let mut ordered: Vec<_> = rows.iter().enumerate().collect();
    ordered.sort_unstable_by_key(|(_, (iface, _))| &iface.module);
    if ordered
        .windows(2)
        .any(|pair| pair[0].1 .0.module == pair[1].1 .0.module)
    {
        return Err(invalid("duplicate package module"));
    }
    let mut input_to_sorted = vec![0; rows.len()];
    let mut documents = Vec::with_capacity(rows.len());
    for (pos, (input, (iface, _))) in ordered.iter().enumerate() {
        input_to_sorted[*input] = word(pos)?;
        documents.push(iface.packed_document());
    }
    let mut definitions = HashMap::new();
    for document in &documents {
        collect_definitions(document, &mut definitions)?;
    }
    let mut definitions: Vec<_> = definitions.into_values().collect();
    definitions.sort_unstable_by_key(|definition| definition.identity.as_str());
    let mut writer = Writer::default();
    for (pos, definition) in definitions.iter().enumerate() {
        writer.body_ids.insert(&definition.identity, word(pos)?);
    }
    let mut bodies = Vec::new();
    for definition in definitions {
        writer.definition(definition, &mut bodies)?;
    }
    let mut modules = Vec::new();
    let mut exports = Vec::new();
    let mut contracts = Vec::new();
    for (pos, ((_, (iface, digest)), document)) in ordered.iter().zip(&documents).enumerate() {
        let start = word(contracts.len())?;
        writer.value(document, &mut contracts, true, 0, Strings::id)?;
        let mut values: Vec<_> = iface.exports.values.iter().collect();
        values.sort_unstable_by_key(|value| &value.name);
        if values.windows(2).any(|pair| pair[0].name == pair[1].name) {
            return Err(invalid("duplicate package value export"));
        }
        let [name_at, name_len] = writer.strings.span(&iface.module)?;
        let [hash_at, hash_len] = writer.strings.span(digest)?;
        put(
            &mut modules,
            [
                name_at,
                name_len,
                hash_at,
                hash_len,
                word(exports.len() / 32)?,
                word(values.len())?,
                start,
                word(contracts.len())? - start,
                input_to_sorted[pos],
            ],
        );
        for value in values {
            let [name_at, name_len] = writer.strings.span(&value.name)?;
            let [symbol_at, symbol_len] = writer.strings.span(&value.symbol)?;
            let [scheme_at, scheme_len] = writer.strings.span(&value.scheme)?;
            let body = if value.total_body.is_some() {
                writer.body_id(&value.symbol)?
            } else {
                u32::MAX
            };
            let arity =
                u32::try_from(value.arity).map_err(|_| invalid("packed value arity overflow"))?;
            put(
                &mut exports,
                [
                    name_at, name_len, symbol_at, symbol_len, scheme_at, scheme_len, arity, body,
                ],
            );
        }
    }
    let mut packed_abi = Vec::new();
    if let Some(abi) = abi {
        put(&mut packed_abi, [word(writer.strings.spans.len())?]);
        writer.value(abi, &mut packed_abi, false, 0, Strings::id)?;
    }
    let sections = [
        writer.strings.section()?,
        modules,
        exports,
        bodies,
        writer.edges,
        writer.nodes,
        writer.operands,
        contracts,
        packed_abi,
    ];
    let total = HEADER + sections.iter().map(Vec::len).sum::<usize>();
    let length = word(total)?;
    let mut header = Vec::with_capacity(HEADER);
    header.extend_from_slice(b"CINCHPK4");
    put(&mut header, [length, 4]);
    let mut offset = HEADER;
    for section in &sections {
        put(&mut header, [word(offset)?, word(section.len())?]);
        offset += section.len();
    }
    Ok(EncodedPackage { header, sections, length: total })
}

pub(crate) fn publish(path: &Path, rows: &[(&Iface, &str)], abi: &Value) -> crate::Result<()> {
    publish_package(path, rows, Some(abi))
}

pub(crate) fn publish_interfaces(path: &Path, rows: &[(&Iface, &str)]) -> crate::Result<()> {
    publish_package(path, rows, None)
}

fn publish_package(path: &Path, rows: &[(&Iface, &str)], abi: Option<&Value>) -> crate::Result<()> {
    let error =
        |err: &dyn std::fmt::Display| crate::Error::plain(format!("{}: {err}", path.display()));
    let encoded = encode(rows, abi).map_err(|err| error(&err))?;
    let directory = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut file = tempfile::NamedTempFile::new_in(directory).map_err(|err| error(&err))?;
    encoded.write_to(&mut file).map_err(|err| error(&err))?;
    // Existing readers may have the old file mapped. Publish a fresh inode.
    file.persist(path).map_err(|err| error(&err))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::{ResolvedScheme, ValueExport};
    use crate::total::{Op, Sort};

    #[test]
    fn emitted_package_preserves_checked_graph_and_interface_digest() {
        let helper = Definition {
            identity: "Zulu.helper".into(),
            parameters: vec![Sort::Nat],
            result: Sort::Nat,
            body: Expr::Primitive(
                Op::NatAdd,
                vec![
                    Expr::Local(0),
                    Expr::Primitive(
                        Op::NatMul,
                        vec![
                            Expr::Local(0),
                            Expr::Number {
                                sort: Sort::Nat,
                                decimal: "2".into(),
                            },
                        ],
                    ),
                ],
            ),
        };
        let root = Definition {
            identity: "Zulu.root".into(),
            parameters: vec![Sort::Nat],
            result: Sort::Nat,
            body: Expr::Let {
                value: Box::new(Expr::Call {
                    target: helper.identity.clone(),
                    arguments: vec![Expr::Local(0)],
                }),
                body: Box::new(Expr::If {
                    condition: Box::new(Expr::Bool(true)),
                    yes: Box::new(Expr::Local(1)),
                    no: Box::new(Expr::Local(0)),
                }),
            },
        };
        let mut first = Iface::new("Zulu", "source");
        first.exports.values.push(ValueExport {
            name: "root".into(),
            symbol: root.identity.clone(),
            total_body: Some(CheckedGraph::check(vec![root, helper]).unwrap()),
            keyed_ordering: None,
            kernel: None,
            scheme: "Nat -> Nat".into(),
            resolved: Some(ResolvedScheme {
                scheme: "Nat -> Nat".into(),
                measures: vec![],
                nominals: vec![],
            }),
            arity: 1,
        });
        let second = Iface::new("Alpha", "source");
        let digests = [first.iface_hash(), second.iface_hash()];
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("library.cpk");
        publish(
            &output,
            &[(&first, &digests[0]), (&second, &digests[1])],
            &Value::obj([("schema", Value::Num(1))]),
        )
        .unwrap();
        let actual = crate::package_interfaces::read_verified_identities(&output).unwrap();
        assert_eq!(
            actual[&first.module],
            (first.runtime_abi, digests[0].clone())
        );
        assert_eq!(
            actual[&second.module],
            (second.runtime_abi, digests[1].clone())
        );
    }
}

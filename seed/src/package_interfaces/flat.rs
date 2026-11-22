//! Indexed checked-body records. Node and operand views borrow the mapping.
use super::{invalid, words, Decode, Strings, BODY_ROW};
use crate::total::{Expr, Op, Sort};

mod checked;
pub(super) use checked::CheckedTable;
pub(crate) use checked::PackedClosure;

const NODE_ROW: usize = 16;

pub(super) struct FlatBodies<'a> {
    strings: Strings<'a>,
    bodies: &'a [u8],
    edges: &'a [u8],
    nodes: &'a [u8],
    operands: &'a [u8],
}

enum Node<'a> {
    Local(u32),
    Number(Sort, &'a str),
    Bool(bool),
    Primitive(Op, &'a [u8]),
    Call(u32, &'a [u8]),
    Let(u32, u32),
    If(u32, u32, u32),
}

fn row_offset(id: u32, width: usize) -> Decode<usize> {
    (id as usize)
        .checked_mul(width)
        .ok_or_else(|| invalid("packed body index overflow"))
}

fn word_range(data: &[u8], start: u32, count: u32) -> Decode<&[u8]> {
    let start = row_offset(start, 4)?;
    let end = row_offset(count, 4)?
        .checked_add(start)
        .ok_or_else(|| invalid("packed body span overflow"))?;
    data.get(start..end)
        .ok_or_else(|| invalid("packed body span outside its section"))
}

fn indices(data: &[u8]) -> impl DoubleEndedIterator<Item = u32> + '_ {
    data.chunks_exact(4)
        .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
}

impl<'a> FlatBodies<'a> {
    pub(super) fn open(
        strings: Strings<'a>,
        bodies: &'a [u8],
        edges: &'a [u8],
        nodes: &'a [u8],
        operands: &'a [u8],
    ) -> Decode<Self> {
        if nodes.len() % NODE_ROW != 0 || operands.len() % 4 != 0 {
            return Err(invalid("mis-sized packed body node or operand section"));
        }
        Ok(Self {
            strings,
            bodies,
            edges,
            nodes,
            operands,
        })
    }

    fn sort(&self, id: u32) -> Decode<Sort> {
        Sort::parse(self.strings.get(id)?).ok_or_else(|| invalid("unknown packed body sort"))
    }

    pub(super) fn definition_name(&self, id: u32) -> Decode<&'a str> {
        let [name] = words(self.bodies, row_offset(id, BODY_ROW)?)
            .map_err(|_| invalid("packed body definition outside its section"))?;
        self.strings.get(name)
    }

    fn node(&self, id: u32) -> Decode<Node<'a>> {
        let [tag, a, b, c] = words(self.nodes, row_offset(id, NODE_ROW)?)?;
        Ok(match (tag, a, b, c) {
            (0, slot, 0, 0) => Node::Local(slot),
            (1, sort, literal, 0) => Node::Number(self.sort(sort)?, self.strings.get(literal)?),
            (2, flag @ 0..=1, 0, 0) => Node::Bool(flag == 1),
            (3, op, first, count) => Node::Primitive(
                Op::parse(self.strings.get(op)?)
                    .ok_or_else(|| invalid("unknown packed body primitive"))?,
                word_range(self.operands, first, count)?,
            ),
            (4, target, first, count) => {
                Node::Call(target, word_range(self.operands, first, count)?)
            }
            (5, value, body, 0) => Node::Let(value, body),
            (6, condition, yes, no) => Node::If(condition, yes, no),
            _ => return Err(invalid("invalid packed body node")),
        })
    }

    // Called only on tables that have passed the shape and depth checks.
    fn materialize(&self, id: u32) -> Expr {
        let child = |node| self.materialize(node);
        match self.node(id).expect("checked body node") {
            Node::Local(slot) => Expr::Local(slot as usize),
            Node::Number(sort, decimal) => Expr::Number {
                sort,
                decimal: decimal.into(),
            },
            Node::Bool(value) => Expr::Bool(value),
            Node::Primitive(op, args) => Expr::Primitive(op, indices(args).map(child).collect()),
            Node::Call(target, args) => Expr::Call {
                target: self.definition_name(target).expect("checked call target").into(),
                arguments: indices(args).map(child).collect(),
            },
            Node::Let(value, body) => Expr::Let {
                value: Box::new(child(value)),
                body: Box::new(child(body)),
            },
            Node::If(condition, yes, no) => Expr::If {
                condition: Box::new(child(condition)),
                yes: Box::new(child(yes)),
                no: Box::new(child(no)),
            },
        }
    }
}

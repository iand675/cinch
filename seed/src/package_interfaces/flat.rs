//! Indexed checked-body records. Node and operand views borrow the mapping.
use super::{invalid, words, Decode, Strings, BODY_ROW, MAX_DEPTH};
use crate::total::{Definition, Expr, Op, Sort};

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

fn indices(data: &[u8]) -> impl Iterator<Item = u32> + '_ {
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

    pub(super) fn definitions(&self) -> Decode<Vec<Definition>> {
        let mut definitions = Vec::with_capacity(self.bodies.len() / BODY_ROW);
        let mut seen = vec![false; self.nodes.len() / NODE_ROW];
        let mut previous = None;
        for row in self.bodies.chunks_exact(BODY_ROW) {
            let [name, result, first, count, root, edge, edges, reserved] = words(row, 0)?;
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
            let mut targets = Vec::new();
            let body = self.materialize(root, MAX_DEPTH, &mut seen, &mut targets)?;
            targets.sort_unstable();
            targets.dedup();
            if !indices(word_range(self.edges, edge, edges)?).eq(targets) {
                return Err(invalid(
                    "packed body dependencies disagree with its expression",
                ));
            }
            definitions.push(Definition {
                identity: identity.into(),
                parameters,
                result: self.sort(result)?,
                body,
            });
        }
        if seen.contains(&false) {
            return Err(invalid("unreferenced packed body node"));
        }
        Ok(definitions)
    }

    fn materialize(
        &self,
        id: u32,
        depth: usize,
        seen: &mut [bool],
        targets: &mut Vec<u32>,
    ) -> Decode<Expr> {
        if depth == 0 {
            return Err(invalid("over-nested packed body"));
        }
        let Some(slot) = seen.get_mut(id as usize) else {
            return Err(invalid("packed body node outside its section"));
        };
        // The producer writes each expression once in postorder. Checking
        // ownership here bounds allocation by the node table, even for a
        // crafted table whose children repeatedly reference the same row.
        if std::mem::replace(slot, true) {
            return Err(invalid("reused packed body node"));
        }
        let mut child = |node| {
            if node >= id {
                return Err(invalid("packed body child is not in postorder"));
            }
            self.materialize(node, depth - 1, seen, targets)
        };
        Ok(match self.node(id)? {
            Node::Local(slot) => Expr::Local(slot as usize),
            Node::Number(sort, decimal) => Expr::Number {
                sort,
                decimal: decimal.into(),
            },
            Node::Bool(value) => Expr::Bool(value),
            Node::Primitive(op, args) => {
                Expr::Primitive(op, indices(args).map(&mut child).collect::<Decode<_>>()?)
            }
            Node::Call(target, args) => {
                let arguments = indices(args).map(&mut child).collect::<Decode<_>>()?;
                targets.push(target);
                Expr::Call {
                    target: self.definition_name(target)?.into(),
                    arguments,
                }
            }
            Node::Let(value, body) => Expr::Let {
                value: Box::new(child(value)?),
                body: Box::new(child(body)?),
            },
            Node::If(condition, yes, no) => Expr::If {
                condition: Box::new(child(condition)?),
                yes: Box::new(child(yes)?),
                no: Box::new(child(no)?),
            },
        })
    }
}

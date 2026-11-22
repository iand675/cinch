use super::*;
use crate::total::{Expr, Op};

const ADD: &str = "Example.add\"\\\nπ";
const WRAP: &str = "Example.wrap";

struct Example {
    nodes: [[u32; 4]; 10],
    edge: u32,
}

impl Example {
    fn new() -> Self {
        Self {
            nodes: [
                [0, 0, 0, 0], // add: local argument
                [1, 0, 1, 0], // 7
                [3, 2, 2, 2], // NatAdd with two operands
                [0, 0, 0, 0], // wrap: value of let
                [2, 1, 0, 0], // if condition
                [0, 1, 0, 0], // bound local
                [4, 0, 4, 1], // call add, passing the bound local
                [1, 0, 1, 0], // false branch
                [6, 4, 6, 7], // if
                [5, 3, 8, 0], // let
            ],
            edge: 0,
        }
    }

    fn check(self) -> Decode<Arc<CheckedTable>> {
        let mut words = Vec::new();
        let mut strings = Vec::new();
        for text in ["Nat", "7", "NatAdd", ADD, WRAP] {
            words.extend([strings.len() as u32, text.len() as u32]);
            strings.extend_from_slice(text.as_bytes());
        }
        let bodies = words.len() * 4;
        words.extend([3, 0, 0, 1, 2, 0, 0, 0]);
        words.extend([4, 0, 1, 1, 9, 0, 1, 0]);
        let edges = words.len() * 4;
        words.push(self.edge);
        let nodes = words.len() * 4;
        words.extend(self.nodes.into_iter().flatten());
        let operands = words.len() * 4;
        words.extend([0, 0, 0, 1, 5]);
        let text = words.len() * 4;
        let mut bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
        bytes.extend(strings);
        let bytes = mapped(bytes);
        let data = bytes.bytes();
        let table = FlatBodies::open(
            Strings {
                data: &data[text..],
                spans: &data[..bodies],
            },
            &data[bodies..edges],
            &data[edges..nodes],
            &data[nodes..operands],
            &data[operands..text],
        )?;
        table.check(Arc::clone(&bytes))
    }
}

fn mapped(bytes: Vec<u8>) -> Arc<PackageBytes> {
    #[cfg(unix)]
    {
        use std::io::Write;
        let mut file = tempfile::tempfile().unwrap();
        file.write_all(&bytes).unwrap();
        // No writer keeps access to this temporary file.
        Arc::new(unsafe { PackageBytes::open(&file) }.unwrap())
    }
    #[cfg(not(unix))]
    Arc::new(PackageBytes(bytes))
}

#[test]
fn writes_checked_closure_without_materializing_any_nodes() {
    let projection = Example::new().check().unwrap().closure_by_id(1).unwrap();
    let graph = CheckedGraph::from_packed(projection.clone());
    let number = || Expr::Number {
        sort: Sort::Nat,
        decimal: "7".into(),
    };
    let mut reference = CheckedGraph::empty();
    reference
        .define(
            ADD.into(),
            vec![Sort::Nat],
            Expr::Primitive(Op::NatAdd, vec![Expr::Local(0), number()]),
        )
        .unwrap();
    reference
        .define(
            WRAP.into(),
            vec![Sort::Nat],
            Expr::Let {
                value: Box::new(Expr::Local(0)),
                body: Box::new(Expr::If {
                    condition: Box::new(Expr::Bool(true)),
                    yes: Box::new(Expr::Call {
                        target: ADD.into(),
                        arguments: vec![Expr::Local(1)],
                    }),
                    no: Box::new(number()),
                }),
            },
        )
        .unwrap();

    assert_eq!(graph.signature(WRAP), Some((&[Sort::Nat][..], Sort::Nat)));
    assert!(graph.closure_is_whole(WRAP).unwrap());
    assert!(!graph.closure_is_whole(ADD).unwrap());
    assert_eq!(graph.to_json(), reference.to_json());
    assert_eq!(
        graph.closure([ADD]).unwrap().to_json(),
        reference.closure([ADD]).unwrap().to_json()
    );
    assert!(projection.projection.owned.get().is_none());
    assert_eq!(graph.get(WRAP), reference.get(WRAP));
    assert!(projection.projection.owned.get().is_some());
}

#[test]
fn rejects_bad_scopes_sorts_nodes_edges_and_cycles() {
    let rejects = |example: Example, text: &str| {
        let Err(error) = example.check() else {
            panic!("malformed body table was accepted")
        };
        assert!(error.to_string().contains(text), "{error}");
    };

    let mut bad = Example::new();
    bad.nodes[5][1] = 2;
    rejects(bad, "ill-scoped or ill-sorted");

    let mut bad = Example::new();
    bad.nodes[4] = [1, 0, 1, 0];
    rejects(bad, "ill-scoped or ill-sorted");

    let mut bad = Example::new();
    bad.nodes[8][3] = 6;
    rejects(bad, "child is not in postorder");

    let mut bad = Example::new();
    bad.nodes[6][3] = 0;
    rejects(bad, "arity mismatch");

    let mut bad = Example::new();
    bad.edge = 1;
    rejects(bad, "dependencies disagree");

    let mut bad = Example::new();
    bad.edge = 1;
    bad.nodes[6][1] = 1;
    rejects(bad, "cyclic total-body dependency");
}

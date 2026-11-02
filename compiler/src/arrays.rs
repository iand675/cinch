//! Checked array intrinsics. Element representation is carried through the
//! existing class-vector specialization, never recovered from a type spelling.
use crate::core::Class;
use crate::types::{Scalar, Ty};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrayOp {
    FromList,
    Replicate,
    Len,
    Get,
    Set,
    Slice,
    Append,
    Reshape,
    Shape,
    Data,
    TensorGet,
    TensorSet,
    Column,
    Zip,
    Rows,
    Columns,
    Row,
    Left,
    Right,
    Leaf,
}

impl ArrayOp {
    pub const ALL: [Self; 20] = [
        Self::FromList,
        Self::Replicate,
        Self::Len,
        Self::Get,
        Self::Set,
        Self::Slice,
        Self::Append,
        Self::Reshape,
        Self::Shape,
        Self::Data,
        Self::TensorGet,
        Self::TensorSet,
        Self::Column,
        Self::Zip,
        Self::Rows,
        Self::Columns,
        Self::Row,
        Self::Left,
        Self::Right,
        Self::Leaf,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::FromList => "arrayFromList",
            Self::Replicate => "arrayReplicate",
            Self::Len => "arrayLen",
            Self::Get => "arrayGet",
            Self::Set => "arraySet",
            Self::Slice => "arraySlice",
            Self::Append => "arrayAppend",
            Self::Reshape => "tensorReshape",
            Self::Shape => "tensorShape",
            Self::Data => "tensorData",
            Self::TensorGet => "tensorGet",
            Self::TensorSet => "tensorSet",
            Self::Column => "columnarColumn",
            Self::Zip => "columnarZip",
            Self::Rows => "columnarRows",
            Self::Columns => "columnarColumns",
            Self::Row => "columnarGet",
            Self::Left => "columnarLeft",
            Self::Right => "columnarRight",
            Self::Leaf => "columnarArray",
        }
    }

    pub fn params(self, packed: Option<Scalar>) -> Vec<String> {
        if packed.is_some() {
            vec![]
        } else if matches!(self, Self::Zip | Self::Left | Self::Right) {
            vec!["a".into(), "b".into()]
        } else {
            vec!["a".into()]
        }
    }

    pub fn sig(self, packed: Option<Scalar>) -> (Vec<Ty>, Ty) {
        let a = packed
            .map(Ty::Prim)
            .unwrap_or_else(|| Ty::Param("a".into()));
        let b = Ty::Param("b".into());
        let app = |name: &str, t| Ty::Adt(name.into(), vec![t]);
        let array = |t| app("Array", t);
        let tensor = |t| app("Tensor", t);
        let col = |t| app("Columnar", t);
        let opt = |t| app("Option", t);
        match self {
            Self::FromList => (vec![app("List", a.clone())], array(a)),
            Self::Replicate => (vec![Ty::Nat, a.clone()], array(a)),
            Self::Len => (vec![array(a)], Ty::Nat),
            Self::Get => (vec![array(a.clone()), Ty::Nat], opt(a)),
            Self::Set => (vec![array(a.clone()), Ty::Nat, a.clone()], opt(array(a))),
            Self::Slice => (vec![array(a.clone()), Ty::Nat, Ty::Nat], opt(array(a))),
            Self::Append => (vec![array(a.clone()), array(a.clone())], array(a)),
            Self::Reshape => (vec![array(Ty::Nat), array(a.clone())], opt(tensor(a))),
            Self::Shape => (vec![tensor(a)], array(Ty::Nat)),
            Self::Data => (vec![tensor(a.clone())], array(a)),
            Self::TensorGet => (vec![tensor(a.clone()), array(Ty::Nat)], opt(a)),
            Self::TensorSet => (vec![tensor(a.clone()), array(Ty::Nat), a.clone()], opt(tensor(a))),
            Self::Column => (vec![array(a.clone())], col(a)),
            Self::Zip => (
                vec![col(a.clone()), col(b.clone())],
                opt(col(Ty::Tuple(vec![a, b]))),
            ),
            Self::Rows | Self::Columns => (vec![col(a)], Ty::Nat),
            Self::Row => (vec![col(a.clone()), Ty::Nat], opt(a)),
            Self::Left => (vec![col(Ty::Tuple(vec![a.clone(), b]))], opt(col(a))),
            Self::Right => (vec![col(Ty::Tuple(vec![a, b.clone()]))], opt(col(b))),
            Self::Leaf => (vec![col(a.clone())], opt(array(a))),
        }
    }

    pub fn returns_option(self) -> bool {
        matches!(
            self,
            Self::Get
                | Self::Set
                | Self::Slice
                | Self::Reshape
                | Self::TensorGet
                | Self::TensorSet
                | Self::Zip
                | Self::Row
                | Self::Left
                | Self::Right
                | Self::Leaf
        )
    }

    pub fn result_class(self, classes: [Class; 2]) -> Class {
        if matches!(self, Self::Get | Self::TensorGet | Self::Row) {
            classes[0]
        } else {
            Class::Obj
        }
    }
}

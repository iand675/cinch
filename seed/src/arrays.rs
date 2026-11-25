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
        let opt = |t| app(crate::known::OPTION, t);
        match self {
            Self::FromList => (vec![app(crate::known::LIST, a.clone())], array(a)),
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

/// Data-parallel packed array & device compute operations preserved through
/// specialization and Typed Array IR optimization before LLVM / Metal lowering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrayComputeOp {
    // Direct packed tabulation / map / zip-with / ordered fold
    TabulateU8,
    TabulateI32,
    TabulateU32,
    TabulateF32,
    TabulateF64,
    MapU8,
    MapI32,
    MapU32,
    MapF32,
    MapF64,
    ZipWithI32,
    ZipWithF32,
    FoldlI32,
    FoldlF32,
    // Fast scalar-kernel array ops (fusion producers & consumers)
    ZipBinopF32,
    ZipBinopI32,
    MapUnaryF32,
    ReduceI32,
    ReduceU32,
    ReduceI64,
    ReduceF32,
    ReduceF64,
    // Fused kernels (target of Typed Array IR producer-consumer fusion)
    DotF32,
    DotF64,
    DotI32,
    StridedDotF32,
    FusedMapReduceF32,
    // Nested parallelism & data-parallel primitives
    RowReduceF32,
    RowReduceI32,
    ScanI32,
    ScanU32,
    FilterI32Gt,
    FilterPackedI32,
    GatherF32,
    GatherI32,
    ScatterUniqueU32,
    ScatterCombineI32,
    HistogramU8,
    HistogramU32,
    HistogramBatchU8,
    Stencil2DU8,
    Stencil2DF32,
    SegmentedReduceF32,
    SpmvCsrF32,
    PackStrided1D,
    PackStrided2D,
    SetExecMode,
    LastReportBytes,
    // Host/Device boundary (Metal GPU / CPU resident buffers)
    DeviceOpen,
    DeviceClose,
    DeviceResetTelemetry,
    DeviceUploadArray,
    DeviceDownloadU8,
    DeviceDownloadU32,
    DeviceDownloadI32,
    DeviceDownloadF32,
    DeviceBufferFree,
    DeviceBufferBytes,
    DeviceDotF32,
    DeviceScanI32,
    DeviceFilterI32Gt,
    DeviceRowReduceF32,
    DeviceStencilHistogramU8,
    DeviceStencil2DF32,
    DeviceSpmvCsrF32,
    DeviceReportBytes,
}

impl ArrayComputeOp {
    pub const ALL: [Self; 66] = [
        Self::TabulateU8,
        Self::TabulateI32,
        Self::TabulateU32,
        Self::TabulateF32,
        Self::TabulateF64,
        Self::MapU8,
        Self::MapI32,
        Self::MapU32,
        Self::MapF32,
        Self::MapF64,
        Self::ZipWithI32,
        Self::ZipWithF32,
        Self::FoldlI32,
        Self::FoldlF32,
        Self::ZipBinopF32,
        Self::ZipBinopI32,
        Self::MapUnaryF32,
        Self::ReduceI32,
        Self::ReduceU32,
        Self::ReduceI64,
        Self::ReduceF32,
        Self::ReduceF64,
        Self::DotF32,
        Self::DotF64,
        Self::DotI32,
        Self::StridedDotF32,
        Self::FusedMapReduceF32,
        Self::RowReduceF32,
        Self::RowReduceI32,
        Self::ScanI32,
        Self::ScanU32,
        Self::FilterI32Gt,
        Self::FilterPackedI32,
        Self::GatherF32,
        Self::GatherI32,
        Self::ScatterUniqueU32,
        Self::ScatterCombineI32,
        Self::HistogramU8,
        Self::HistogramU32,
        Self::HistogramBatchU8,
        Self::Stencil2DU8,
        Self::Stencil2DF32,
        Self::SegmentedReduceF32,
        Self::SpmvCsrF32,
        Self::PackStrided1D,
        Self::PackStrided2D,
        Self::SetExecMode,
        Self::LastReportBytes,
        Self::DeviceOpen,
        Self::DeviceClose,
        Self::DeviceResetTelemetry,
        Self::DeviceUploadArray,
        Self::DeviceDownloadU8,
        Self::DeviceDownloadU32,
        Self::DeviceDownloadI32,
        Self::DeviceDownloadF32,
        Self::DeviceBufferFree,
        Self::DeviceBufferBytes,
        Self::DeviceDotF32,
        Self::DeviceScanI32,
        Self::DeviceFilterI32Gt,
        Self::DeviceRowReduceF32,
        Self::DeviceStencilHistogramU8,
        Self::DeviceStencil2DF32,
        Self::DeviceSpmvCsrF32,
        Self::DeviceReportBytes,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::TabulateU8 => "arrayTabulateU8",
            Self::TabulateI32 => "arrayTabulateI32",
            Self::TabulateU32 => "arrayTabulateU32",
            Self::TabulateF32 => "arrayTabulateF32",
            Self::TabulateF64 => "arrayTabulateF64",
            Self::MapU8 => "arrayMapU8",
            Self::MapI32 => "arrayMapI32",
            Self::MapU32 => "arrayMapU32",
            Self::MapF32 => "arrayMapF32",
            Self::MapF64 => "arrayMapF64",
            Self::ZipWithI32 => "arrayZipWithI32",
            Self::ZipWithF32 => "arrayZipWithF32",
            Self::FoldlI32 => "arrayFoldlI32",
            Self::FoldlF32 => "arrayFoldlF32",
            Self::ZipBinopF32 => "arrayZipBinopF32",
            Self::ZipBinopI32 => "arrayZipBinopI32",
            Self::MapUnaryF32 => "arrayMapUnaryF32",
            Self::ReduceI32 => "arrayReduceI32",
            Self::ReduceU32 => "arrayReduceU32",
            Self::ReduceI64 => "arrayReduceI64",
            Self::ReduceF32 => "arrayReduceF32",
            Self::ReduceF64 => "arrayReduceF64",
            Self::DotF32 => "arrayDotF32",
            Self::DotF64 => "arrayDotF64",
            Self::DotI32 => "arrayDotI32",
            Self::StridedDotF32 => "arrayStridedDotF32",
            Self::FusedMapReduceF32 => "arrayFusedMapReduceF32",
            Self::RowReduceF32 => "arrayRowReduceF32",
            Self::RowReduceI32 => "arrayRowReduceI32",
            Self::ScanI32 => "arrayScanI32",
            Self::ScanU32 => "arrayScanU32",
            Self::FilterI32Gt => "arrayFilterI32Gt",
            Self::FilterPackedI32 => "arrayFilterI32",
            Self::GatherF32 => "arrayGatherF32",
            Self::GatherI32 => "arrayGatherI32",
            Self::ScatterUniqueU32 => "arrayScatterUniqueU32",
            Self::ScatterCombineI32 => "arrayScatterCombineI32",
            Self::HistogramU8 => "arrayHistogramU8",
            Self::HistogramU32 => "arrayHistogramU32",
            Self::HistogramBatchU8 => "arrayHistogramBatchU8",
            Self::Stencil2DU8 => "arrayStencil2DU8",
            Self::Stencil2DF32 => "arrayStencil2DF32",
            Self::SegmentedReduceF32 => "arraySegmentedReduceF32",
            Self::SpmvCsrF32 => "arraySpmvCsrF32",
            Self::PackStrided1D => "arrayPackStrided1D",
            Self::PackStrided2D => "arrayPackStrided2D",
            Self::SetExecMode => "arraySetExecMode",
            Self::LastReportBytes => "arrayLastReportBytes",
            Self::DeviceOpen => "deviceOpen",
            Self::DeviceClose => "deviceClose",
            Self::DeviceResetTelemetry => "deviceResetTelemetry",
            Self::DeviceUploadArray => "deviceUploadArray",
            Self::DeviceDownloadU8 => "deviceDownloadU8",
            Self::DeviceDownloadU32 => "deviceDownloadU32",
            Self::DeviceDownloadI32 => "deviceDownloadI32",
            Self::DeviceDownloadF32 => "deviceDownloadF32",
            Self::DeviceBufferFree => "deviceBufferFree",
            Self::DeviceBufferBytes => "deviceBufferBytes",
            Self::DeviceDotF32 => "deviceDotF32",
            Self::DeviceScanI32 => "deviceScanI32",
            Self::DeviceFilterI32Gt => "deviceFilterI32Gt",
            Self::DeviceRowReduceF32 => "deviceRowReduceF32",
            Self::DeviceStencilHistogramU8 => "deviceStencilHistogramU8",
            Self::DeviceStencil2DF32 => "deviceStencil2DF32",
            Self::DeviceSpmvCsrF32 => "deviceSpmvCsrF32",
            Self::DeviceReportBytes => "deviceReportBytes",
        }
    }

    pub fn params(self) -> Vec<String> {
        match self {
            Self::PackStrided1D | Self::PackStrided2D | Self::DeviceUploadArray => vec!["a".into()],
            _ => vec![],
        }
    }

    pub fn sig(self) -> (Vec<Ty>, Ty) {
        let app = |name: &str, t| Ty::Adt(name.into(), vec![t]);
        let arr = |t| app("Array", t);
        let opt = |t| app(crate::known::OPTION, t);
        let a = Ty::Param("a".into());
        let u8t = Ty::Prim(Scalar::U8);
        let i32t = Ty::Prim(Scalar::I32);
        let u32t = Ty::Prim(Scalar::U32);
        let i64t = Ty::Prim(Scalar::I64);
        let f32t = Ty::Prim(Scalar::F32);
        let f64t = Ty::Prim(Scalar::F64);
        let bytes = Ty::Bytes;
        let fun = |x, y| Ty::fun(x, y);
        match self {
            Self::TabulateU8 => (vec![Ty::Nat, fun(Ty::Nat, u8t.clone())], arr(u8t)),
            Self::TabulateI32 => (vec![Ty::Nat, fun(Ty::Nat, i32t.clone())], arr(i32t)),
            Self::TabulateU32 => (vec![Ty::Nat, fun(Ty::Nat, u32t.clone())], arr(u32t)),
            Self::TabulateF32 => (vec![Ty::Nat, fun(Ty::Nat, f32t.clone())], arr(f32t)),
            Self::TabulateF64 => (vec![Ty::Nat, fun(Ty::Nat, f64t.clone())], arr(f64t)),
            Self::MapU8 => (vec![fun(u8t.clone(), u8t.clone()), arr(u8t.clone())], arr(u8t)),
            Self::MapI32 => (vec![fun(i32t.clone(), i32t.clone()), arr(i32t.clone())], arr(i32t)),
            Self::MapU32 => (vec![fun(u32t.clone(), u32t.clone()), arr(u32t.clone())], arr(u32t)),
            Self::MapF32 => (vec![fun(f32t.clone(), f32t.clone()), arr(f32t.clone())], arr(f32t)),
            Self::MapF64 => (vec![fun(f64t.clone(), f64t.clone()), arr(f64t.clone())], arr(f64t)),
            Self::ZipWithI32 => (
                vec![fun(i32t.clone(), fun(i32t.clone(), i32t.clone())), arr(i32t.clone()), arr(i32t.clone())],
                opt(arr(i32t)),
            ),
            Self::ZipWithF32 => (
                vec![fun(f32t.clone(), fun(f32t.clone(), f32t.clone())), arr(f32t.clone()), arr(f32t.clone())],
                opt(arr(f32t)),
            ),
            Self::FoldlI32 => (
                vec![fun(i32t.clone(), fun(i32t.clone(), i32t.clone())), i32t.clone(), arr(i32t.clone())],
                i32t,
            ),
            Self::FoldlF32 => (
                vec![fun(f32t.clone(), fun(f32t.clone(), f32t.clone())), f32t.clone(), arr(f32t.clone())],
                f32t,
            ),
            Self::ZipBinopF32 => (vec![arr(f32t.clone()), arr(f32t.clone()), i64t], opt(arr(f32t))),
            Self::ZipBinopI32 => (vec![arr(i32t.clone()), arr(i32t.clone()), i64t], opt(arr(i32t))),
            Self::MapUnaryF32 => (vec![arr(f32t.clone()), i64t, f32t.clone()], arr(f32t)),
            Self::ReduceI32 => (vec![arr(i32t.clone()), i64t], i32t),
            Self::ReduceU32 => (vec![arr(u32t.clone()), i64t], u32t),
            Self::ReduceI64 => (vec![arr(i64t.clone()), i64t.clone()], i64t),
            Self::ReduceF32 => (vec![arr(f32t.clone()), i64t], f32t),
            Self::ReduceF64 => (vec![arr(f64t.clone()), i64t], f64t),
            Self::DotF32 => (vec![arr(f32t.clone()), arr(f32t.clone()), i64t], f32t),
            Self::DotF64 => (vec![arr(f64t.clone()), arr(f64t.clone()), i64t], f64t),
            Self::DotI32 => (vec![arr(i32t.clone()), arr(i32t.clone())], i32t),
            Self::StridedDotF32 => (
                vec![arr(f32t.clone()), Ty::Nat, Ty::Nat, arr(f32t.clone()), Ty::Nat, Ty::Nat, Ty::Nat, i64t],
                f32t,
            ),
            Self::FusedMapReduceF32 => (vec![arr(f32t.clone()), i64t.clone(), f32t.clone(), i64t], f32t),
            Self::RowReduceF32 => (vec![arr(f32t.clone()), Ty::Nat, Ty::Nat, i64t], opt(arr(f32t))),
            Self::RowReduceI32 => (vec![arr(i32t.clone()), Ty::Nat, Ty::Nat, i64t], opt(arr(i32t))),
            Self::ScanI32 => (vec![arr(i32t.clone()), i64t, Ty::Bool], arr(i32t)),
            Self::ScanU32 => (vec![arr(u32t.clone()), i64t, Ty::Bool], arr(u32t)),
            Self::FilterI32Gt => (vec![arr(i32t.clone()), i32t.clone()], arr(i32t)),
            Self::FilterPackedI32 => (vec![fun(i32t.clone(), Ty::Bool), arr(i32t.clone())], arr(i32t)),
            Self::GatherF32 => (vec![arr(f32t.clone()), arr(i32t), i64t], arr(f32t)),
            Self::GatherI32 => (vec![arr(i32t.clone()), arr(i32t.clone()), i64t], arr(i32t)),
            Self::ScatterUniqueU32 => (vec![arr(u32t.clone()), arr(u32t.clone()), arr(u32t.clone())], arr(u32t)),
            Self::ScatterCombineI32 => (vec![arr(i32t.clone()), arr(i32t.clone()), arr(i32t.clone()), i64t], arr(i32t)),
            Self::HistogramU8 => (vec![arr(u8t), Ty::Nat], arr(u32t)),
            Self::HistogramU32 => (vec![arr(u32t.clone()), Ty::Nat], arr(u32t)),
            Self::HistogramBatchU8 => (vec![arr(u8t), Ty::Nat, Ty::Nat, Ty::Nat, Ty::Nat], opt(arr(u32t))),
            Self::Stencil2DU8 => (
                vec![arr(u8t.clone()), Ty::Nat, Ty::Nat, Ty::Nat, arr(i32t.clone()), i32t, i64t, Ty::Nat],
                opt(arr(u8t)),
            ),
            Self::Stencil2DF32 => (
                vec![arr(f32t.clone()), Ty::Nat, Ty::Nat, Ty::Nat, arr(f32t.clone()), i64t, Ty::Nat],
                opt(arr(f32t)),
            ),
            Self::SegmentedReduceF32 => (vec![arr(f32t.clone()), arr(u32t), i64t], arr(f32t)),
            Self::SpmvCsrF32 => (vec![arr(f32t.clone()), arr(u32t.clone()), arr(u32t), arr(f32t.clone()), i64t], arr(f32t)),
            Self::PackStrided1D => (vec![arr(a.clone()), Ty::Nat, Ty::Nat, Ty::Nat], opt(arr(a))),
            Self::PackStrided2D => (vec![arr(a.clone()), Ty::Nat, Ty::Nat, Ty::Nat, Ty::Nat, Ty::Nat], opt(arr(a))),
            Self::SetExecMode => (vec![i64t], Ty::Unit),
            Self::LastReportBytes => (vec![Ty::Unit], bytes),
            Self::DeviceOpen => (vec![i64t.clone()], i64t),
            Self::DeviceClose => (vec![i64t.clone()], i64t),
            Self::DeviceResetTelemetry => (vec![i64t.clone()], i64t),
            Self::DeviceUploadArray => (vec![i64t.clone(), arr(a)], i64t),
            Self::DeviceDownloadU8 => (vec![i64t.clone(), i64t], opt(arr(u8t))),
            Self::DeviceDownloadU32 => (vec![i64t.clone(), i64t], opt(arr(u32t))),
            Self::DeviceDownloadI32 => (vec![i64t.clone(), i64t], opt(arr(i32t))),
            Self::DeviceDownloadF32 => (vec![i64t.clone(), i64t], opt(arr(f32t))),
            Self::DeviceBufferFree => (vec![i64t.clone()], i64t),
            Self::DeviceBufferBytes => (vec![i64t.clone()], i64t),
            Self::DeviceDotF32 => (vec![i64t.clone(), i64t.clone(), i64t.clone(), Ty::Nat, i64t], opt(f32t)),
            Self::DeviceScanI32 => (vec![i64t.clone(), i64t.clone(), Ty::Nat, Ty::Bool], i64t),
            Self::DeviceFilterI32Gt => (vec![i64t.clone(), i64t.clone(), Ty::Nat, i32t], i64t),
            Self::DeviceRowReduceF32 => (vec![i64t.clone(), i64t.clone(), Ty::Nat, Ty::Nat], i64t),
            Self::DeviceStencilHistogramU8 => (
                vec![i64t.clone(), i64t.clone(), Ty::Nat, Ty::Nat, Ty::Nat, arr(i32t.clone()), i32t, i64t.clone(), Ty::Nat, Ty::Nat],
                i64t,
            ),
            Self::DeviceStencil2DF32 => (
                vec![i64t.clone(), i64t.clone(), Ty::Nat, Ty::Nat, Ty::Nat, arr(f32t), i64t.clone(), Ty::Nat],
                i64t,
            ),
            Self::DeviceSpmvCsrF32 => (
                vec![i64t.clone(), i64t.clone(), i64t.clone(), i64t.clone(), i64t.clone(), Ty::Nat, Ty::Nat, i64t.clone()],
                i64t,
            ),
            Self::DeviceReportBytes => (vec![i64t], bytes),
        }
    }

    pub fn returns_option(self) -> bool {
        matches!(
            self,
            Self::ZipWithI32
                | Self::ZipWithF32
                | Self::ZipBinopF32
                | Self::ZipBinopI32
                | Self::RowReduceF32
                | Self::RowReduceI32
                | Self::HistogramBatchU8
                | Self::Stencil2DU8
                | Self::Stencil2DF32
                | Self::PackStrided1D
                | Self::PackStrided2D
                | Self::DeviceDownloadU8
                | Self::DeviceDownloadU32
                | Self::DeviceDownloadI32
                | Self::DeviceDownloadF32
                | Self::DeviceDotF32
        )
    }

    pub fn option_payload_class(self) -> Class {
        if self == Self::DeviceDotF32 {
            Class::Raw
        } else {
            Class::Obj
        }
    }
}

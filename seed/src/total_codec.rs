// Versioned body data is a separate artifact until interface schema migration
// and independent Cinch admission are connected. Unknown/duplicate fields are
// errors; a caller cannot add an ignored semantic qualifier.
use crate::interface::{parse_json, JsonValue as J};
use std::collections::HashMap;

impl Sort {
    pub fn name(self) -> &'static str { match self { Self::Nat => "Nat", Self::Int => "Int", Self::Bool => "Bool", Self::Bytes => "Bytes", Self::I64 => "I64" } }
    pub(crate) fn parse(name: &str) -> Option<Self> { Some(match name { "Nat" => Self::Nat, "Int" => Self::Int, "Bool" => Self::Bool, "Bytes" => Self::Bytes, "I64" => Self::I64, _ => return None }) }
}
impl Op {
    pub(crate) fn parse(name: &str) -> Option<Self> {
        Some(match name {
            "NatAdd" => Self::NatAdd, "NatMul" => Self::NatMul, "NatCmp" => Self::NatCmp,
            "IntAdd" => Self::IntAdd, "IntSub" => Self::IntSub, "IntMul" => Self::IntMul,
            "IntNeg" => Self::IntNeg, "IntCmp" => Self::IntCmp, "IntFromNat" => Self::IntFromNat, "IntFromI64" => Self::IntFromI64,
            "I64Eq" => Self::I64Eq, "I64Ne" => Self::I64Ne, "I64Lt" => Self::I64Lt,
            "I64Le" => Self::I64Le, "I64Gt" => Self::I64Gt, "I64Ge" => Self::I64Ge,
            "BoolEq" => Self::BoolEq, "BoolNe" => Self::BoolNe, "BoolNot" => Self::BoolNot,
            "BoolAnd" => Self::BoolAnd, "BoolOr" => Self::BoolOr, "BytesLen" => Self::BytesLen,
            _ => return None,
        })
    }
}
fn expr_json(expression: &Expr) -> J {
    match expression {
        Expr::Local(slot) => J::obj([("node", "local".into()), ("slot", J::Num(*slot as u64))]),
        Expr::Number { sort, decimal } => J::obj([("node", "number".into()), ("sort", sort.name().into()), ("decimal", decimal.into())]),
        Expr::Bool(value) => J::obj([("node", "bool".into()), ("value", (*value).into())]),
        Expr::Primitive(operation, arguments) => J::obj([("node", "primitive".into()), ("operation", format!("{operation:?}").into()), ("arguments", J::arr(arguments, expr_json))]),
        Expr::Call { target, arguments } => J::obj([("node", "call".into()), ("target", target.into()), ("arguments", J::arr(arguments, expr_json))]),
        Expr::Let { value, body } => J::obj([("node", "let".into()), ("value", expr_json(value)), ("body", expr_json(body))]),
        Expr::If { condition, yes, no } => J::obj([("node", "if".into()), ("condition", expr_json(condition)), ("yes", expr_json(yes)), ("no", expr_json(no))]),
    }
}
fn definition_json(definition: &Definition) -> J {
    J::obj([
        ("identity", definition.identity.as_str().into()),
        ("parameters", J::arr(&definition.parameters, |sort| sort.name().into())),
        ("result", definition.result.name().into()),
        ("body", expr_json(&definition.body)),
    ])
}
fn object<'a>(value: &'a J, fields: &[&str]) -> Result<&'a BTreeMap<String, J>, Invalid> {
    let o = value.as_object().map_err(|e| Invalid(e.to_string()))?;
    if o.len() != fields.len() || !fields.iter().all(|key| o.contains_key(*key)) { return Err(Invalid("unexpected total-body fields".into())) }
    Ok(o)
}
fn string(value: &J) -> Result<&str, Invalid> { value.as_str().map_err(|e| Invalid(e.to_string())) }
fn array(value: &J) -> Result<&Vec<J>, Invalid> { value.as_array().map_err(|e| Invalid(e.to_string())) }
fn decode_expr(value: &J) -> Result<Expr, Invalid> {
    let initial = value.as_object().map_err(|e| Invalid(e.to_string()))?;
    let node = initial.get("node").ok_or_else(|| Invalid("missing total-body node".into()))?;
    Ok(match string(node)? {
        "local" => { let o=object(value, &["node","slot"])?; Expr::Local(usize::try_from(o["slot"].as_num().map_err(|e| Invalid(e.to_string()))?).map_err(|_| Invalid("total slot overflow".into()))?) }
        "number" => { let o=object(value,&["node","sort","decimal"])?; Expr::Number { sort: Sort::parse(string(&o["sort"])?).ok_or_else(|| Invalid("unknown total sort".into()))?, decimal:string(&o["decimal"])?.into() } }
        "bool" => { let o=object(value,&["node","value"])?; Expr::Bool(o["value"].as_bool().map_err(|e| Invalid(e.to_string()))?) }
        "primitive" => { let o=object(value,&["node","operation","arguments"])?; Expr::Primitive(Op::parse(string(&o["operation"])?).ok_or_else(|| Invalid("unknown total primitive".into()))?, array(&o["arguments"])?.iter().map(decode_expr).collect::<Result<_,_>>()?) }
        "call" => { let o=object(value,&["node","target","arguments"])?; Expr::Call { target:string(&o["target"])?.into(), arguments:array(&o["arguments"])?.iter().map(decode_expr).collect::<Result<_,_>>()? } }
        "let" => { let o=object(value,&["node","value","body"])?; Expr::Let { value:Box::new(decode_expr(&o["value"])?), body:Box::new(decode_expr(&o["body"])?) } }
        "if" => { let o=object(value,&["node","condition","yes","no"])?; Expr::If { condition:Box::new(decode_expr(&o["condition"])?), yes:Box::new(decode_expr(&o["yes"])?), no:Box::new(decode_expr(&o["no"])?) } }
        _ => return Err(Invalid("unknown total-body node".into())),
    })
}
impl CheckedGraph {
    pub fn to_json(&self) -> String {
        self.to_json_value().to_canonical_string()
    }

    pub(crate) fn to_json_value(&self) -> J {
        J::obj([("version", J::Num(1)), ("definitions", J::arr(self.definitions(), definition_json))])
    }

    /// Several exports can reference the same checked definition. A document's
    /// cache uses allocation identity so distinct definitions with the same
    /// symbol still serialize independently for the coherence checker.
    pub(crate) fn to_shared_json_value(
        &self,
        rows: &mut HashMap<*const Definition, Arc<str>>,
    ) -> J {
        let definitions = J::arr(self.definitions(), |definition| {
            let canonical = rows
                .entry(std::ptr::from_ref(definition))
                .or_insert_with(|| Arc::from(definition_json(definition).to_canonical_string()));
            J::CanonicalBodyRow(Arc::clone(canonical))
        });
        J::obj([("version", J::Num(1)), ("definitions", definitions)])
    }
    pub fn from_json(text: &str) -> Result<Self, Invalid> {
        let data = parse_json(text).map_err(|e| Invalid(e.to_string()))?;
        Self::from_json_value(&data)
    }

    pub(crate) fn from_json_value(data: &J) -> Result<Self, Invalid> {
        if let J::PackedBody(graph) = data {
            return Ok(graph.clone());
        }
        let root=object(data,&["version","definitions"])?;
        if root["version"].as_num().map_err(|e| Invalid(e.to_string()))? != 1 { return Err(Invalid("unknown total-body version".into())) }
        let rows = array(&root["definitions"])?;
        let mut definitions = Vec::with_capacity(rows.len());
        for row in rows {
            let o=object(row,&["identity","parameters","result","body"])?;
            definitions.push(Definition {
                identity:string(&o["identity"])?.into(),
                parameters:array(&o["parameters"])?.iter().map(|p| Sort::parse(string(p)?).ok_or_else(|| Invalid("unknown total parameter sort".into()))).collect::<Result<_,_>>()?,
                result:Sort::parse(string(&o["result"])?).ok_or_else(|| Invalid("unknown total result sort".into()))?,
                body:decode_expr(&o["body"])?,
            });
        }
        Self::check(definitions)
    }
}

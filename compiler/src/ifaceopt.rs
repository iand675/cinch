//! Interface optimisation artifacts: the small library units' Core bodies,
//! written beside a module's `.cinchif` and read back by the inliner.
//!
//! A `.cinchif` is the *contract* a dependent is invalidated by
//! (`design/compiled-interface-metadata.md`), so a body may not go in it: a
//! body-only edit would stale every dependent and change the interface
//! identity. This module defines the second artifact those bodies need
//! instead (`design/inline-unfolding-policy.md` section 12, mechanism A):
//!
//! ```text
//! build/<Module>.cinchif   the contract (schema 10)
//! build/<Module>.cinchopt  the unfoldings for its small units (schema 1)
//! ```
//!
//! The optimisation artifact is keyed by the interface identity it was
//! produced against (`iface_hash`), so a consumer refuses an artifact whose
//! contract has moved rather than inlining against a stale body. Its own
//! bytes have a content hash, which the package execution interface records,
//! so a dependency's unfoldings are as pinned as its object.
//!
//! Only the *unfoldable* class is carried: the bodies the policy copies at a
//! call site without a size test (a primop wrapper, a field projection, a
//! literal, a shallow decision, a delegation to another such unit). A body
//! outside the class would either be refused by the consumer's own admission
//! rule or, worse, grow the program, so the artifact does not carry it. The
//! encoder is deliberately narrower than [`crate::core::Core`]: a node it
//! cannot spell (a closure, a join, a lazy cell, a boundary pointer) makes
//! the writer skip that body, and the reader refuses an unknown tag. The
//! consumer therefore sees exactly what the producer could spell and never
//! guesses at a body.
//!
//! The artifact is not part of any interface identity. A library whose
//! bodies changed but whose contracts did not keeps the same `.cinchif`
//! bytes and the same `iface_hash`; only the `.cinchopt` content hash moves.

use crate::core::{
    CAlt, CBranch, CPat, Core, FieldAccess, FieldShape, FnDef, GlobalId, LocalId, PrimOp,
    ProjField,
};
use crate::interface::{content_hash, parse_json, Iface, JsonValue, ParseError};
use crate::repr::{letters, Class};
use std::collections::{BTreeMap, BTreeSet};

/// The optimisation artifact's own schema. Independent of the interface
/// schema on purpose: a body encoding change must not force every
/// `.cinchif` in the world to be rebuilt.
pub const SCHEMA: u32 = 1;

/// One body in the artifact: a unit of the module, its parameter
/// representation classes and its Core body. `params` is `UnitRepr::params`
/// (a caller needs it to give a bound argument the class the count pass
/// expects); `arity` is `params.len()`, carried so a reader can reject a
/// truncated row without decoding the body.
#[derive(Clone, Debug)]
pub struct BodyExport {
    pub unit: String,
    pub arity: usize,
    pub params: Vec<Class>,
    pub body: Core,
}

/// The artifact for one module.
#[derive(Clone, Debug)]
pub struct Optimisation {
    pub schema: u32,
    pub language: String,
    pub runtime_abi: u32,
    pub compiler: String,
    pub module: String,
    /// The interface identity these bodies were produced against.
    pub iface_hash: String,
    pub bodies: Vec<BodyExport>,
}

impl Optimisation {
    pub fn new(module: impl Into<String>, iface_hash: impl Into<String>) -> Optimisation {
        Optimisation {
            schema: SCHEMA,
            language: crate::interface::LANGUAGE_VERSION.to_string(),
            runtime_abi: crate::interface::RUNTIME_ABI_VERSION,
            compiler: env!("CARGO_PKG_VERSION").to_string(),
            module: module.into(),
            iface_hash: iface_hash.into(),
            bodies: Vec::new(),
        }
    }

    /// Canonical bytes: bodies sorted by unit, every object in key order,
    /// no insignificant whitespace. Two runs over the same program write
    /// the same bytes, which the package manifest's content hash relies on.
    pub fn to_json(&self) -> String {
        let mut bodies = self.bodies.clone();
        bodies.sort_by(|a, b| a.unit.cmp(&b.unit));
        let rows = bodies
            .iter()
            .filter_map(|body| {
                let encoded = encode_core(&body.body)?;
                Some(JsonValue::obj([
                    ("unit", body.unit.as_str().into()),
                    ("arity", (body.arity as u64).into()),
                    ("params", letters(&body.params).into()),
                    ("body", encoded),
                ]))
            })
            .collect::<Vec<_>>();
        let doc = JsonValue::obj([
            ("schema", self.schema.into()),
            ("language", self.language.as_str().into()),
            ("runtime_abi", self.runtime_abi.into()),
            ("compiler", self.compiler.as_str().into()),
            ("module", self.module.as_str().into()),
            ("iface_hash", self.iface_hash.as_str().into()),
            ("bodies", JsonValue::Arr(rows)),
        ]);
        doc.to_canonical_string()
    }

    /// The artifact's own identity: the content hash of its canonical bytes.
    /// The package execution interface records it beside the interface hash.
    pub fn content_hash(&self) -> String {
        content_hash(&self.to_json())
    }

    pub fn from_json(text: &str) -> Result<Optimisation, ParseError> {
        let doc = parse_json(text)?;
        let o = doc.as_object()?;
        let schema = field_num(o, "schema")?;
        if schema != SCHEMA as u64 {
            return Err(ParseError::from(format!(
                "optimisation schema {schema} is not supported (expected {SCHEMA})"
            )));
        }
        let runtime_abi = field_num(o, "runtime_abi")?;
        if runtime_abi != crate::interface::RUNTIME_ABI_VERSION as u64 {
            return Err(ParseError::from(format!(
                "optimisation artifact runtime ABI {runtime_abi} does not match this compiler's {}",
                crate::interface::RUNTIME_ABI_VERSION
            )));
        }
        let rows = field(o, "bodies")?.as_array()?;
        let mut bodies = Vec::with_capacity(rows.len());
        let mut seen = BTreeSet::new();
        for row in rows {
            let r = row.as_object()?;
            let unit = field_str(r, "unit")?;
            if !seen.insert(unit.clone()) {
                return Err(ParseError::from(format!("duplicate optimisation body for {unit}")));
            }
            let arity = field_num(r, "arity")? as usize;
            let params = decode_classes(&field_str(r, "params")?)?;
            if params.len() != arity {
                return Err(ParseError::from(format!(
                    "optimisation body {unit} has {arity} parameter(s) but {} class(es)",
                    params.len()
                )));
            }
            let body = decode_core(field(r, "body")?)?;
            if free_locals(&body).iter().any(|l| l.0 >= arity) {
                return Err(ParseError::from(format!(
                    "optimisation body {unit} reads a local outside its parameters"
                )));
            }
            bodies.push(BodyExport { unit, arity, params, body });
        }
        Ok(Optimisation {
            schema: SCHEMA,
            language: field_str(o, "language")?,
            runtime_abi: crate::interface::RUNTIME_ABI_VERSION,
            compiler: field_str(o, "compiler")?,
            module: field_str(o, "module")?,
            iface_hash: field_str(o, "iface_hash")?,
            bodies,
        })
    }
}

/// The bodies of one module a `.cinchopt` carries.
///
/// A unit is exported when its body is in the writer's encodable subset, its
/// parameter classes are concrete (an unresolved `Param` would make the
/// consumer's copy unverifiable), and every unit it names in a value
/// position is either defined in this program or a primitive. The `prim` and
/// constructor units are skipped: neither has a Core body a consumer could
/// copy.
pub fn for_module(iface: &Iface, program: &crate::core::Program) -> Optimisation {
    for_module_in(iface, program, &crate::simplify::unfoldable_names(program))
}

/// [`for_module`] with the class computed once by the caller: the artifact
/// writer classifies a whole program for every module it writes, and the
/// class is a property of the program, not of the module.
pub fn for_module_in(
    iface: &Iface,
    program: &crate::core::Program,
    class: &BTreeSet<String>,
) -> Optimisation {
    let mut out = Optimisation::new(iface.module.clone(), iface.iface_hash());
    let prefix = format!("{}.", iface.module);
    let defined: BTreeSet<String> = program.fns.iter().map(|f| f.id.0.clone()).collect();
    for f in &program.fns {
        if !f.id.0.starts_with(&prefix) || !class.contains(&f.id.0) {
            continue;
        }
        if let Some(body) = exportable(f, &defined) {
            out.bodies.push(body);
        }
    }
    out
}

fn exportable(f: &FnDef, defined: &BTreeSet<String>) -> Option<BodyExport> {
    // A `prim` unit's body is inline target IR, not Core, and a constructor
    // unit's body is generated as a block allocation; neither is a Core body
    // a consumer can copy.
    if f.prim.is_some() || f.ctor.is_some() || f.meta.prim_body {
        return None;
    }
    if f.repr.params.iter().any(|c| c.is_param()) {
        return None;
    }
    if free_locals(&f.body).iter().any(|l| l.0 >= f.params.len()) {
        return None;
    }
    // Every unit named in a value position must be resolvable by a
    // consumer: defined in this program or a primitive.
    let mut names: Vec<&str> = Vec::new();
    global_refs(&f.body, &mut names);
    for name in names {
        if defined.contains(name)
            || name.starts_with("cinch.prim.")
            || name.starts_with("cinch.runtime.")
            || name.starts_with("<builtin>.")
        {
            continue;
        }
        return None;
    }
    encode_core(&f.body)?;
    Some(BodyExport {
        unit: f.id.0.clone(),
        arity: f.params.len(),
        params: f.repr.params.clone(),
        body: f.body.clone(),
    })
}

fn field<'a>(o: &'a BTreeMap<String, JsonValue>, k: &str) -> Result<&'a JsonValue, ParseError> {
    o.get(k).ok_or_else(|| ParseError::from(format!("missing {k}")))
}

fn field_num(o: &BTreeMap<String, JsonValue>, k: &str) -> Result<u64, ParseError> {
    field(o, k)?.as_num()
}

fn field_str(o: &BTreeMap<String, JsonValue>, k: &str) -> Result<String, ParseError> {
    Ok(field(o, k)?.as_str()?.to_string())
}

fn num(value: &JsonValue) -> Result<usize, ParseError> {
    Ok(value.as_num()? as usize)
}

fn signed(value: &JsonValue) -> Result<i64, ParseError> {
    let text = value.as_str()?;
    text.parse::<i64>().map_err(|_| ParseError::from(format!("invalid integer {text}")))
}

/// The local ids a body reads without binding them itself, in first-use
/// order. The writer uses it to insist a body is closed over its parameters
/// and the reader to reject one that is not.
pub fn free_locals(e: &Core) -> Vec<LocalId> {
    let mut free = Vec::new();
    let mut seen = BTreeSet::new();
    free_walk(e, &BTreeSet::new(), &mut |local| {
        if seen.insert(local.0) {
            free.push(local);
        }
    });
    free
}

fn free_walk(e: &Core, bound: &BTreeSet<usize>, visit: &mut impl FnMut(LocalId)) {
    match e {
        Core::Var(l) => {
            if !bound.contains(&l.0) {
                visit(*l);
            }
        }
        Core::Let(l, _, value, body) => {
            free_walk(value, bound, visit);
            let mut inner = bound.clone();
            inner.insert(l.0);
            free_walk(body, &inner, visit);
        }
        Core::Join { params, body, rest, .. } => {
            free_walk(rest, bound, visit);
            let mut inner = bound.clone();
            for (l, _) in params {
                inner.insert(l.0);
            }
            free_walk(body, &inner, visit);
        }
        Core::Case { scrut, alts } => {
            free_walk(scrut, bound, visit);
            for alt in alts {
                let mut inner = bound.clone();
                match &alt.pat {
                    CPat::Var(l, _) => {
                        inner.insert(l.0);
                    }
                    CPat::Con { fields, .. } => {
                        for l in fields {
                            inner.insert(l.0);
                        }
                    }
                    CPat::Wild => {}
                }
                free_walk(&alt.body, &inner, visit);
            }
        }
        _ => {
            for child in crate::core::child_slots_imm(e) {
                free_walk(child, bound, visit);
            }
        }
    }
}

fn global_refs<'a>(e: &'a Core, out: &mut Vec<&'a str>) {
    match e {
        Core::Global(g, _) | Core::Ctor(g, _) => out.push(&g.0),
        _ => {}
    }
    for child in crate::core::child_slots_imm(e) {
        global_refs(child, out);
    }
}

// ---------------------------------------------------------------- classes

fn decode_classes(text: &str) -> Result<Vec<Class>, ParseError> {
    let mut out = Vec::with_capacity(text.len());
    for c in text.chars() {
        out.push(match c {
            'R' => Class::Raw,
            'O' => Class::Obj,
            'T' => Class::TObj,
            // A parameter class has no round-trippable spelling (the letter
            // drops its index), and no exported body may carry one.
            other => return Err(ParseError::from(format!("invalid representation class {other}"))),
        });
    }
    Ok(out)
}

fn class_str(class: Class) -> String {
    letters(&[class])
}

// -------------------------------------------------------------------- Core
//
// One canonical array per node, the tag first. The subset is exactly the
// class the inliner copies without a size test; a decoder that meets an
// unknown tag fails rather than inventing a node.

pub fn encode_core(e: &Core) -> Option<JsonValue> {
    Some(match e {
        Core::Int(n) => JsonValue::Arr(vec!["int".into(), n.to_string().into()]),
        Core::Bool(b) => JsonValue::Arr(vec!["bool".into(), (*b).into()]),
        Core::Unit => JsonValue::Arr(vec!["unit".into()]),
        Core::ConstBig { nat, digits } => {
            JsonValue::Arr(vec!["big".into(), (*nat).into(), digits.as_str().into()])
        }
        Core::ConstBytes(bytes) => {
            JsonValue::Arr(vec!["bytes".into(), JsonValue::Arr(byte_rows(bytes))])
        }
        Core::ConstString { ctor, bytes } => JsonValue::Arr(vec![
            "string".into(),
            ctor.0.as_str().into(),
            JsonValue::Arr(byte_rows(bytes)),
        ]),
        Core::ConstSym(name) => JsonValue::Arr(vec!["sym".into(), name.as_str().into()]),
        Core::StrLit(text) => JsonValue::Arr(vec!["lit".into(), text.as_str().into()]),
        Core::Var(l) => JsonValue::Arr(vec!["var".into(), (l.0 as u64).into()]),
        Core::Global(g, classes) => JsonValue::Arr(vec![
            "global".into(),
            g.0.as_str().into(),
            letters(classes).into(),
        ]),
        Core::Ctor(g, classes) => JsonValue::Arr(vec![
            "ctor".into(),
            g.0.as_str().into(),
            letters(classes).into(),
        ]),
        Core::Prim(op) => JsonValue::Arr(vec!["prim".into(), op.symbol().into()]),
        Core::Foreign(sym) => JsonValue::Arr(vec!["foreign".into(), sym.as_str().into()]),
        Core::Proj(inner, field) => JsonValue::Arr(vec![
            "proj".into(),
            encode_core(inner)?,
            (field.slot as u64).into(),
            class_str(field.class).into(),
            encode_access(field.access.as_ref()),
            field.mutable.into(),
        ]),
        Core::DictProj(inner, index, site, class) => {
            if site.is_some() {
                return None;
            }
            JsonValue::Arr(vec![
                "dictproj".into(),
                encode_core(inner)?,
                (*index as u64).into(),
                class_str(*class).into(),
            ])
        }
        Core::App(head, args) => JsonValue::Arr(vec![
            "app".into(),
            encode_core(head)?,
            encode_all(args, encode_core)?,
        ]),
        Core::Let(l, class, value, body) => JsonValue::Arr(vec![
            "let".into(),
            (l.0 as u64).into(),
            class_str(*class).into(),
            encode_core(value)?,
            encode_core(body)?,
        ]),
        Core::Case { scrut, alts } => JsonValue::Arr(vec![
            "case".into(),
            encode_core(scrut)?,
            encode_all(alts, |alt| {
                Some(JsonValue::Arr(vec![
                    encode_pat(&alt.pat),
                    alt.weight.map_or(JsonValue::Null, |w| (w as u64).into()),
                    encode_core(&alt.body)?,
                ]))
            })?,
        ]),
        Core::If(condition, then, otherwise) => JsonValue::Arr(vec![
            "if".into(),
            encode_core(condition)?,
            encode_branch(then)?,
            encode_branch(otherwise)?,
        ]),
        Core::Dup(l, class, body) => JsonValue::Arr(vec![
            "dup".into(),
            (l.0 as u64).into(),
            class_str(*class).into(),
            encode_core(body)?,
        ]),
        Core::Drop(l, class, body) => JsonValue::Arr(vec![
            "drop".into(),
            (l.0 as u64).into(),
            class_str(*class).into(),
            encode_core(body)?,
        ]),
        _ => return None,
    })
}

fn encode_all<T>(items: &[T], f: impl Fn(&T) -> Option<JsonValue>) -> Option<JsonValue> {
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        out.push(f(item)?);
    }
    Some(JsonValue::Arr(out))
}

fn byte_rows(bytes: &[u8]) -> Vec<JsonValue> {
    bytes.iter().map(|b| (*b as u64).into()).collect()
}

fn encode_branch(branch: &CBranch<Core>) -> Option<JsonValue> {
    Some(JsonValue::Arr(vec![
        branch.weight.map_or(JsonValue::Null, |w| (w as u64).into()),
        encode_core(&branch.body)?,
    ]))
}

fn encode_pat(pat: &CPat) -> JsonValue {
    match pat {
        CPat::Var(l, class) => JsonValue::Arr(vec![
            "var".into(),
            (l.0 as u64).into(),
            class_str(*class).into(),
        ]),
        CPat::Wild => JsonValue::Arr(vec!["wild".into()]),
        CPat::Con { tag, fields, classes } => JsonValue::Arr(vec![
            "con".into(),
            (*tag as u64).into(),
            JsonValue::arr(fields, |l| (l.0 as u64).into()),
            letters(classes).into(),
        ]),
    }
}

fn encode_access(access: Option<&FieldAccess>) -> JsonValue {
    match access {
        None => JsonValue::Null,
        Some(a) => JsonValue::Arr(vec![
            a.offset.into(),
            a.size.into(),
            encode_shape(&a.shape),
            a.range.map_or(JsonValue::Null, |(lo, hi)| {
                JsonValue::Arr(vec![lo.to_string().into(), hi.to_string().into()])
            }),
            a.range.is_some().into(),
        ]),
    }
}

fn encode_shape(shape: &FieldShape) -> JsonValue {
    match shape {
        FieldShape::Scalar { size, signed } => {
            JsonValue::Arr(vec!["scalar".into(), (*size).into(), (*signed).into()])
        }
        FieldShape::Word { align } => JsonValue::Arr(vec!["word".into(), (*align).into()]),
        FieldShape::Inline { size, tag, fields } => JsonValue::Arr(vec![
            "inline".into(),
            (*size).into(),
            (*tag as u64).into(),
            JsonValue::arr(fields.iter(), |f| {
                JsonValue::Arr(vec![
                    f.offset.into(),
                    f.size.into(),
                    encode_shape(&f.shape),
                    f.range.map_or(JsonValue::Null, |(lo, hi)| {
                        JsonValue::Arr(vec![lo.to_string().into(), hi.to_string().into()])
                    }),
                    f.range.is_some().into(),
                ])
            }),
        ]),
    }
}

pub fn decode_core(value: &JsonValue) -> Result<Core, ParseError> {
    let parts = value.as_array()?;
    let tag = parts
        .first()
        .ok_or_else(|| ParseError::from("empty optimisation node".to_string()))?
        .as_str()?;
    let arg = |i: usize| -> Result<&JsonValue, ParseError> {
        parts
            .get(i)
            .ok_or_else(|| ParseError::from(format!("optimisation node {tag} is missing field {i}")))
    };
    let one_class = |i: usize| -> Result<Class, ParseError> {
        let mut classes = decode_classes(arg(i)?.as_str()?)?;
        if classes.len() != 1 {
            return Err(ParseError::from(format!("optimisation node {tag} needs one class")));
        }
        Ok(classes.remove(0))
    };
    let local = |i: usize| -> Result<LocalId, ParseError> { Ok(LocalId(num(arg(i)?)?)) };
    Ok(match tag {
        "int" => Core::Int(signed(arg(1)?)?),
        "bool" => Core::Bool(arg(1)?.as_bool()?),
        "unit" => Core::Unit,
        "big" => Core::ConstBig { nat: arg(1)?.as_bool()?, digits: arg(2)?.as_str()?.to_string() },
        "bytes" => Core::ConstBytes(bytes(arg(1)?)?),
        "string" => Core::ConstString {
            ctor: GlobalId(arg(1)?.as_str()?.to_string()),
            bytes: bytes(arg(2)?)?,
        },
        "sym" => Core::ConstSym(arg(1)?.as_str()?.to_string()),
        "lit" => Core::StrLit(arg(1)?.as_str()?.to_string()),
        "var" => Core::Var(local(1)?),
        "global" => Core::Global(
            GlobalId(arg(1)?.as_str()?.to_string()),
            decode_classes(arg(2)?.as_str()?)?,
        ),
        "ctor" => Core::Ctor(
            GlobalId(arg(1)?.as_str()?.to_string()),
            decode_classes(arg(2)?.as_str()?)?,
        ),
        "prim" => {
            let symbol = arg(1)?.as_str()?;
            Core::Prim(
                PrimOp::from_symbol(symbol)
                    .ok_or_else(|| ParseError::from(format!("unknown primitive {symbol}")))?,
            )
        }
        "foreign" => Core::Foreign(arg(1)?.as_str()?.to_string()),
        "proj" => Core::Proj(
            Box::new(decode_core(arg(1)?)?),
            ProjField {
                slot: num(arg(2)?)?,
                access: decode_access(arg(3)?)?,
                class: one_class(4)?,
                mutable: arg(5)?.as_bool()?,
            },
        ),
        "dictproj" => {
            Core::DictProj(Box::new(decode_core(arg(1)?)?), num(arg(2)?)?, None, one_class(3)?)
        }
        "app" => {
            let head = Box::new(decode_core(arg(1)?)?);
            let mut args = Vec::new();
            for a in arg(2)?.as_array()? {
                args.push(decode_core(a)?);
            }
            Core::App(head, args)
        }
        "let" => Core::Let(
            local(1)?,
            one_class(2)?,
            Box::new(decode_core(arg(3)?)?),
            Box::new(decode_core(arg(4)?)?),
        ),
        "case" => {
            let scrut = Box::new(decode_core(arg(1)?)?);
            let mut alts = Vec::new();
            for a in arg(2)?.as_array()? {
                let row = a.as_array()?;
                if row.len() != 3 {
                    return Err(ParseError::from(
                        "optimisation case alternative needs three fields".to_string(),
                    ));
                }
                alts.push(CAlt {
                    pat: decode_pat(&row[0])?,
                    weight: weight(&row[1])?,
                    body: Box::new(decode_core(&row[2])?),
                });
            }
            Core::Case { scrut, alts }
        }
        "if" => {
            let condition = Box::new(decode_core(arg(1)?)?);
            let then = decode_branch(arg(2)?)?;
            let otherwise = decode_branch(arg(3)?)?;
            Core::If(condition, then, otherwise)
        }
        "dup" => Core::Dup(local(1)?, one_class(2)?, Box::new(decode_core(arg(3)?)?)),
        "drop" => Core::Drop(local(1)?, one_class(2)?, Box::new(decode_core(arg(3)?)?)),
        other => return Err(ParseError::from(format!("unknown optimisation node {other}"))),
    })
}

fn decode_branch(value: &JsonValue) -> Result<CBranch<Core>, ParseError> {
    let parts = value.as_array()?;
    if parts.len() != 2 {
        return Err(ParseError::from("optimisation branch needs two fields".to_string()));
    }
    Ok(CBranch { weight: weight(&parts[0])?, body: Box::new(decode_core(&parts[1])?) })
}

fn weight(value: &JsonValue) -> Result<Option<u32>, ParseError> {
    match value {
        JsonValue::Null => Ok(None),
        other => Ok(Some(other.as_num()? as u32)),
    }
}

fn decode_pat(value: &JsonValue) -> Result<CPat, ParseError> {
    let parts = value.as_array()?;
    let tag = parts
        .first()
        .ok_or_else(|| ParseError::from("empty optimisation pattern".to_string()))?
        .as_str()?;
    Ok(match tag {
        "var" => CPat::Var(
            LocalId(num(parts.get(1).ok_or_else(|| ParseError::from("pattern needs an id".to_string()))?)?),
            decode_classes(
                parts.get(2).ok_or_else(|| ParseError::from("pattern needs a class".to_string()))?.as_str()?,
            )?
            .into_iter()
            .next()
            .ok_or_else(|| ParseError::from("optimisation variable pattern needs one class".to_string()))?,
        ),
        "wild" => CPat::Wild,
        "con" => {
            let mut fields = Vec::new();
            for f in parts
                .get(2)
                .ok_or_else(|| ParseError::from("constructor pattern needs fields".to_string()))?
                .as_array()?
            {
                fields.push(LocalId(num(f)?));
            }
            let classes = decode_classes(
                parts
                    .get(3)
                    .ok_or_else(|| ParseError::from("constructor pattern needs classes".to_string()))?
                    .as_str()?,
            )?;
            if classes.len() != fields.len() {
                return Err(ParseError::from(
                    "optimisation constructor pattern class count disagrees with its fields"
                        .to_string(),
                ));
            }
            CPat::Con { tag: num(parts.get(1).ok_or_else(|| ParseError::from("pattern needs a tag".to_string()))?)?, fields, classes }
        }
        other => return Err(ParseError::from(format!("unknown optimisation pattern {other}"))),
    })
}

fn bytes(value: &JsonValue) -> Result<Vec<u8>, ParseError> {
    let mut out = Vec::new();
    for b in value.as_array()? {
        let n = b.as_num()?;
        if n > 0xff {
            return Err(ParseError::from("optimisation byte constant out of range".to_string()));
        }
        out.push(n as u8);
    }
    Ok(out)
}

fn decode_access(value: &JsonValue) -> Result<Option<FieldAccess>, ParseError> {
    match value {
        JsonValue::Null => Ok(None),
        other => {
            let parts = other.as_array()?;
            let range = if parts.get(4).is_some_and(|v| matches!(v, JsonValue::Bool(true))) {
                let r = parts
                    .get(3)
                    .ok_or_else(|| ParseError::from("boundary access range is missing".to_string()))?
                    .as_array()?;
                if r.len() != 2 {
                    return Err(ParseError::from("boundary access range needs two bounds".to_string()));
                }
                Some((signed(&r[0])?, signed(&r[1])?))
            } else {
                None
            };
            Ok(Some(FieldAccess {
                offset: parts
                    .first()
                    .ok_or_else(|| ParseError::from("boundary access is missing its offset".to_string()))?
                    .as_num()?,
                size: parts
                    .get(1)
                    .ok_or_else(|| ParseError::from("boundary access is missing its size".to_string()))?
                    .as_num()?,
                shape: decode_shape(
                    parts
                        .get(2)
                        .ok_or_else(|| ParseError::from("boundary access is missing its shape".to_string()))?,
                )?,
                range,
            }))
        }
    }
}

fn decode_shape(value: &JsonValue) -> Result<FieldShape, ParseError> {
    let parts = value.as_array()?;
    let tag = parts
        .first()
        .ok_or_else(|| ParseError::from("empty boundary field shape".to_string()))?
        .as_str()?;
    Ok(match tag {
        "scalar" => FieldShape::Scalar {
            size: parts
                .get(1)
                .ok_or_else(|| ParseError::from("scalar shape needs a size".to_string()))?
                .as_num()?,
            signed: parts
                .get(2)
                .ok_or_else(|| ParseError::from("scalar shape needs signedness".to_string()))?
                .as_bool()?,
        },
        "word" => FieldShape::Word {
            align: parts
                .get(1)
                .ok_or_else(|| ParseError::from("word shape needs an alignment".to_string()))?
                .as_num()?,
        },
        "inline" => {
            let mut fields = Vec::new();
            for f in parts
                .get(3)
                .ok_or_else(|| ParseError::from("inline shape needs fields".to_string()))?
                .as_array()?
            {
                fields.push(
                    decode_access(f)?.ok_or_else(|| ParseError::from("inline field is null".to_string()))?,
                );
            }
            FieldShape::Inline {
                size: parts
                    .get(1)
                    .ok_or_else(|| ParseError::from("inline shape needs a size".to_string()))?
                    .as_num()?,
                tag: num(parts.get(2).ok_or_else(|| ParseError::from("inline shape needs a tag".to_string()))?)?,
                fields,
            }
        }
        other => return Err(ParseError::from(format!("unknown boundary field shape {other}"))),
    })
}

// --------------------------------------------------------------- consuming

/// The foreign body table a consumer's inliner gets, plus the modules those
/// bodies came from. A unit the table defines is not emitted by this
/// compilation: its definition stays in the object the artifact describes,
/// so the caller links that object.
pub struct ForeignTable {
    pub bodies: Vec<FnDef>,
    pub modules: BTreeSet<String>,
}

/// Read the optimisation artifacts named by an interface manifest.
///
/// The manifest is the same one a consumer already passes for its imported
/// contracts (`{"schema":1,"interfaces":[[module, iface_hash, path], ...]}`).
/// Each row's artifact sits beside its `.cinchif`; a missing artifact is
/// simply an interface with no unfoldings, which is what every producer
/// that predates this format writes. A present artifact whose module or
/// interface identity disagrees with the row is refused: inlining against a
/// stale body would be a wrong-code bug, not a missed optimization, so the
/// mismatch is reported instead of ignored.
pub fn load_manifest(path: &std::path::Path) -> crate::Result<ForeignTable> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| crate::Error::plain(format!("{}: {e}", path.display())))?;
    let doc =
        parse_json(&text).map_err(|e| crate::Error::plain(format!("{}: {e}", path.display())))?;
    let root = doc
        .as_object()
        .map_err(|e| crate::Error::plain(format!("{}: {e}", path.display())))?;
    let schema = root
        .get("schema")
        .and_then(|v| v.as_num().ok())
        .ok_or_else(|| {
            crate::Error::plain(format!("{}: missing interface manifest schema", path.display()))
        })?;
    if schema != 1 {
        return Err(crate::Error::plain(format!(
            "{}: unsupported interface manifest schema {schema}",
            path.display()
        )));
    }
    let rows = root.get("interfaces").and_then(|v| v.as_array().ok()).ok_or_else(|| {
        crate::Error::plain(format!("{}: missing interface manifest rows", path.display()))
    })?;
    let mut table = ForeignTable { bodies: Vec::new(), modules: BTreeSet::new() };
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for row in rows {
        let parts = row
            .as_array()
            .map_err(|e| crate::Error::plain(format!("{}: {e}", path.display())))?;
        if parts.len() != 3 {
            return Err(crate::Error::plain(format!(
                "{}: an interface manifest row needs three fields",
                path.display()
            )));
        }
        let module = parts[0]
            .as_str()
            .map_err(|e| crate::Error::plain(format!("{}: {e}", path.display())))?;
        let iface_hash = parts[1]
            .as_str()
            .map_err(|e| crate::Error::plain(format!("{}: {e}", path.display())))?;
        let iface_path = std::path::PathBuf::from(
            parts[2]
                .as_str()
                .map_err(|e| crate::Error::plain(format!("{}: {e}", path.display())))?,
        );
        let opt_path = iface_path.with_extension("cinchopt");
        if !opt_path.is_file() {
            continue;
        }
        let opt_text = std::fs::read_to_string(&opt_path)
            .map_err(|e| crate::Error::plain(format!("{}: {e}", opt_path.display())))?;
        let artifact = Optimisation::from_json(&opt_text)
            .map_err(|e| crate::Error::plain(format!("{}: {e}", opt_path.display())))?;
        if artifact.module != module {
            return Err(crate::Error::plain(format!(
                "{}: optimisation artifact names module {}, expected {module}",
                opt_path.display(),
                artifact.module
            )));
        }
        if artifact.iface_hash != iface_hash {
            return Err(crate::Error::plain(format!(
                "{}: optimisation artifact was produced against interface {} but the manifest pins {}; rebuild the producer",
                opt_path.display(),
                artifact.iface_hash,
                iface_hash
            )));
        }
        // The row's own `.cinchif` must still be the contract the artifact
        // was made against. The producer writes the hash; a consumer that
        // recomputes it is not trusting the manifest's copy.
        let iface_text = std::fs::read_to_string(&iface_path)
            .map_err(|e| crate::Error::plain(format!("{}: {e}", iface_path.display())))?;
        let iface = Iface::from_json(&iface_text)
            .map_err(|e| crate::Error::plain(format!("{}: {e}", iface_path.display())))?;
        if iface.module != module || iface.iface_hash() != iface_hash {
            return Err(crate::Error::plain(format!(
                "{}: interface identity disagrees with the manifest row",
                iface_path.display()
            )));
        }
        if !seen.insert(module.to_string()) {
            continue;
        }
        table.modules.insert(module.to_string());
        table.bodies.extend(foreign_defs(&artifact));
    }
    Ok(table)
}

/// The [`FnDef`]s a consumer's inliner consults for one artifact's bodies.
pub fn foreign_defs(artifact: &Optimisation) -> Vec<FnDef> {
    artifact.bodies.iter().cloned().map(body_def).collect()
}

/// One artifact body as an [`FnDef`] the inliner can consult. The fields a
/// caller of the unit would read are the ones an artifact carries; a body
/// has no scheme, no hints and no optimizer facts, and none of them is an
/// input to the admission rule or to a copy.
fn body_def(body: BodyExport) -> FnDef {
    let params = (0..body.arity).map(|i| format!("a{i}")).collect();
    FnDef {
        id: GlobalId(body.unit),
        params,
        body: body.body,
        prim: None,
        ctor: None,
        hints: crate::core::FnHints::default(),
        meta: crate::core::FnMeta::default(),
        repr: crate::core::UnitRepr {
            tparams: 0,
            result: None,
            dictionary_values: Vec::new(),
            params: body.params,
            fields: Vec::new(),
            data: None,
        },
    }
}

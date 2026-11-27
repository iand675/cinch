//! Interface optimisation artifacts: the small library units' Core bodies,
//! written beside the checked interface package and read back by the inliner.
//!
//! A `.cinchif` is the *contract* a dependent is invalidated by
//! (`design/compiled-interface-metadata.md`), so a body may not go in it: a
//! body-only edit would stale every dependent and change the interface
//! identity. This module defines the second artifact those bodies need
//! instead (`design/inline-unfolding-policy.md` section 12, mechanism A):
//!
//! ```text
//! build/<entry>.cpk        checked contracts and optional unfoldings
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
//! body changes that preserve the checked contract leave its `iface_hash` stable.

use crate::core::{
    CAlt, CBranch, CPat, Core, FieldAccess, FieldShape, FnDef, GlobalId, LocalId, PrimOp,
    ProjField,
};
use crate::interface::{content_hash, parse_json, Iface, JsonValue, ParseError};
use crate::repr::{letters, Class};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// The optimisation artifact's own schema. Independent of the interface
/// schema on purpose: a body encoding change must not force every
/// checked contract to be rebuilt.
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

/// One polymorphic template in the artifact: a generic function (`tparams > 0`)
/// or parameter-dependent constructor (`ctor.is_some()`, `fields` containing
/// `Param(i)`). Consumers linking against a prebuilt library object instantiate
/// these templates on demand at consumer-local `Class::Val { size, align }`
/// layouts without rebuilding the dependency.
#[derive(Clone, Debug)]
pub struct TemplateExport {
    pub unit: String,
    pub tparams: usize,
    pub arity: usize,
    pub params: Vec<Class>,
    pub result: Option<Class>,
    pub ctor: Option<(usize, usize)>,
    pub fields: Vec<Class>,
    pub data: Option<String>,
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
    /// Polymorphic Core templates for cross-module `Class::Val` specialization.
    pub templates: Vec<TemplateExport>,
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
            templates: Vec::new(),
        }
    }

    /// Canonical bytes: bodies sorted by unit, every object in key order,
    /// no insignificant whitespace. Two runs over the same program write
    /// the same bytes, which the package manifest's content hash relies on.
    pub fn to_json(&self) -> String {
        self.document().to_canonical_string()
    }

    pub(crate) fn document(&self) -> JsonValue {
        let mut bodies: Vec<_> = self.bodies.iter().collect();
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
        let mut tpls: Vec<_> = self.templates.iter().collect();
        tpls.sort_by(|a, b| a.unit.cmp(&b.unit));
        let template_rows = tpls
            .iter()
            .filter_map(|tpl| {
                let encoded = if tpl.ctor.is_some() {
                    JsonValue::Null
                } else {
                    encode_core_with(&tpl.body, true)?
                };
                let mut fields: BTreeMap<String, JsonValue> = BTreeMap::new();
                fields.insert("unit".into(), tpl.unit.as_str().into());
                fields.insert("tparams".into(), (tpl.tparams as u64).into());
                fields.insert("arity".into(), (tpl.arity as u64).into());
                fields.insert("params".into(), letters(&tpl.params).into());
                fields.insert("body".into(), encoded);
                if let Some(res) = tpl.result {
                    fields.insert("result".into(), class_str(res).into());
                }
                if let Some((tag, arity)) = tpl.ctor {
                    fields.insert("ctor".into(), JsonValue::Arr(vec![(tag as u64).into(), (arity as u64).into()]));
                    fields.insert("fields".into(), letters(&tpl.fields).into());
                }
                if let Some(data) = &tpl.data {
                    fields.insert("data".into(), data.as_str().into());
                }
                Some(JsonValue::Obj(fields))
            })
            .collect::<Vec<_>>();
        let mut top: BTreeMap<String, JsonValue> = BTreeMap::new();
        top.insert("schema".into(), self.schema.into());
        top.insert("language".into(), self.language.as_str().into());
        top.insert("runtime_abi".into(), self.runtime_abi.into());
        top.insert("compiler".into(), self.compiler.as_str().into());
        top.insert("module".into(), self.module.as_str().into());
        top.insert("iface_hash".into(), self.iface_hash.as_str().into());
        top.insert("bodies".into(), JsonValue::Arr(rows));
        if !template_rows.is_empty() {
            top.insert("templates".into(), JsonValue::Arr(template_rows));
        }
        JsonValue::Obj(top)
    }

    /// The artifact's own identity: the content hash of its canonical bytes.
    /// The package execution interface records it beside the interface hash.
    pub fn content_hash(&self) -> String {
        content_hash(&self.to_json())
    }

    pub fn from_json(text: &str) -> Result<Optimisation, ParseError> {
        Self::parse(text, false)
    }

    /// [`from_json`] for a consumer: a body whose encoding this reader does
    /// not understand is skipped, not fatal. The artifact is an optional
    /// optimisation input, so a producer that knows more node kinds than
    /// this reader must not make the module unusable; the bodies this
    /// reader can spell are still inlined. Structural faults (schema,
    /// runtime ABI, a malformed row) are still refused.
    pub fn from_json_lenient(text: &str) -> Result<Optimisation, ParseError> {
        Self::parse(text, true)
    }

    fn parse(text: &str, skip_undecodable: bool) -> Result<Optimisation, ParseError> {
        Self::from_document(&parse_json(text)?, skip_undecodable)
    }

    fn from_document(doc: &JsonValue, skip_undecodable: bool) -> Result<Optimisation, ParseError> {
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
            let body = match decode_core(field(r, "body")?) {
                Ok(body) => body,
                Err(_) if skip_undecodable => continue,
                Err(error) => return Err(error),
            };
            if free_locals(&body).iter().any(|l| l.0 >= arity) {
                if skip_undecodable {
                    continue;
                }
                return Err(ParseError::from(format!(
                    "optimisation body {unit} reads a local outside its parameters"
                )));
            }
            bodies.push(BodyExport { unit, arity, params, body });
        }
        let mut templates = Vec::new();
        if let Some(tpl_val) = o.get("templates") {
            let tpl_rows = tpl_val.as_array()?;
            let mut seen_tpl = BTreeSet::new();
            for row in tpl_rows {
                let r = row.as_object()?;
                let unit = field_str(r, "unit")?;
                if !seen_tpl.insert(unit.clone()) {
                    return Err(ParseError::from(format!("duplicate optimisation template for {unit}")));
                }
                let tparams = field_num(r, "tparams")? as usize;
                let arity = field_num(r, "arity")? as usize;
                let params = decode_classes_with(&field_str(r, "params")?, true)?;
                let result = match r.get("result") {
                    Some(v) if !matches!(v, JsonValue::Null) => {
                        let mut cs = decode_classes_with(v.as_str()?, true)?;
                        if cs.len() != 1 {
                            return Err(ParseError::from(format!("optimisation template {unit} needs one result class")));
                        }
                        Some(cs.remove(0))
                    }
                    _ => None,
                };
                let ctor = match r.get("ctor") {
                    Some(v) if !matches!(v, JsonValue::Null) => {
                        let arr = v.as_array()?;
                        if arr.len() != 2 {
                            return Err(ParseError::from(format!("optimisation template {unit} ctor needs [tag, arity]")));
                        }
                        Some((num(&arr[0])?, num(&arr[1])?))
                    }
                    _ => None,
                };
                let fields = match r.get("fields") {
                    Some(v) if !matches!(v, JsonValue::Null) => decode_classes_with(v.as_str()?, true)?,
                    _ => Vec::new(),
                };
                let data = match r.get("data") {
                    Some(v) if !matches!(v, JsonValue::Null) => Some(v.as_str()?.to_string()),
                    _ => None,
                };
                let body = if ctor.is_some() {
                    Core::Unit
                } else {
                    match decode_core_with(field(r, "body")?, true) {
                        Ok(body) => body,
                        Err(_) if skip_undecodable => continue,
                        Err(error) => return Err(error),
                    }
                };
                if ctor.is_none() && free_locals(&body).iter().any(|l| l.0 >= arity) {
                    if skip_undecodable {
                        continue;
                    }
                    return Err(ParseError::from(format!(
                        "optimisation template {unit} reads a local outside its parameters"
                    )));
                }
                templates.push(TemplateExport {
                    unit,
                    tparams,
                    arity,
                    params,
                    result,
                    ctor,
                    fields,
                    data,
                    body,
                });
            }
        }
        Ok(Optimisation {
            schema: SCHEMA,
            language: field_str(o, "language")?,
            runtime_abi: crate::interface::RUNTIME_ABI_VERSION,
            compiler: field_str(o, "compiler")?,
            module: field_str(o, "module")?,
            iface_hash: field_str(o, "iface_hash")?,
            bodies,
            templates,
        })
    }
}

/// The optional unfoldings for one module.
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
    let defined: HashSet<&str> = program.fns.iter().map(|f| f.id.0.as_str()).collect();
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

/// Shared lookups for writing a program's per-module optimisation artifacts.
pub struct ModuleBodyIndex<'a> {
    defined: HashSet<&'a str>,
    candidates: HashMap<&'a str, Vec<&'a FnDef>>,
    templates: HashMap<&'a str, Vec<&'a FnDef>>,
    template_defined: HashSet<&'a str>,
}

impl<'a> ModuleBodyIndex<'a> {
    pub fn new(program: &'a crate::core::Program, class: &BTreeSet<String>) -> Self {
        Self::new_with_templates(None, program, class)
    }

    pub fn new_with_templates(
        checked: Option<&'a crate::core::Program>,
        program: &'a crate::core::Program,
        class: &BTreeSet<String>,
    ) -> Self {
        let mut defined = HashSet::with_capacity(program.fns.len());
        let mut candidates = HashMap::<&str, Vec<&FnDef>>::new();
        for function in &program.fns {
            let name = function.id.0.as_str();
            defined.insert(name);
            if class.contains(name) {
                // A parent module's prefix also matches its nested modules.
                // Preserve program order within every matching artifact.
                for (dot, _) in name.match_indices('.') {
                    candidates.entry(&name[..dot]).or_default().push(function);
                }
            }
        }
        let mut template_defined = HashSet::new();
        let mut templates = HashMap::<&str, Vec<&FnDef>>::new();
        if let Some(checked) = checked {
            for function in &checked.fns {
                let name = function.id.0.as_str();
                template_defined.insert(name);
                let is_poly_fn = function.repr.tparams > 0
                    && function.prim.is_none()
                    && function.ctor.is_none()
                    && !function.meta.prim_body;
                let is_poly_ctor = function.ctor.is_some()
                    && function.repr.tparams > 0
                    && function.repr.fields.iter().any(|c| c.is_param());
                if is_poly_fn || is_poly_ctor {
                    for (dot, _) in name.match_indices('.') {
                        templates.entry(&name[..dot]).or_default().push(function);
                    }
                }
            }
        }
        Self { defined, candidates, templates, template_defined }
    }

    pub fn for_module(&self, iface: &Iface) -> Optimisation {
        self.for_contract(&iface.module, iface.iface_hash())
    }

    pub(crate) fn for_contract(&self, module: &str, contract_hash: String) -> Optimisation {
        let mut artifact = Optimisation::new(module.to_owned(), contract_hash);
        if let Some(functions) = self.candidates.get(module) {
            artifact.bodies.extend(
                functions.iter().filter_map(|function| exportable(function, &self.defined)),
            );
        }
        if let Some(tpls) = self.templates.get(module) {
            artifact.templates.extend(
                tpls.iter().filter_map(|function| exportable_template(function, &self.template_defined)),
            );
        }
        artifact
    }
}

fn exportable_template(f: &FnDef, defined: &HashSet<&str>) -> Option<TemplateExport> {
    if f.repr.tparams == 0 || f.prim.is_some() || f.meta.prim_body {
        return None;
    }
    if let Some((tag, arity)) = f.ctor {
        if !f.repr.fields.iter().any(|c| c.is_param()) {
            return None;
        }
        return Some(TemplateExport {
            unit: f.id.0.clone(),
            tparams: f.repr.tparams,
            arity: f.params.len(),
            params: f.repr.params.clone(),
            result: f.repr.result,
            ctor: Some((tag, arity)),
            fields: f.repr.fields.clone(),
            data: f.repr.data.clone(),
            body: Core::Unit,
        });
    }
    if free_locals(&f.body).iter().any(|l| l.0 >= f.params.len()) {
        return None;
    }
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
    encode_core_with(&f.body, true)?;
    Some(TemplateExport {
        unit: f.id.0.clone(),
        tparams: f.repr.tparams,
        arity: f.params.len(),
        params: f.repr.params.clone(),
        result: f.repr.result,
        ctor: None,
        fields: Vec::new(),
        data: None,
        body: f.body.clone(),
    })
}

fn exportable(f: &FnDef, defined: &HashSet<&str>) -> Option<BodyExport> {
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
    decode_classes_with(text, false)
}

fn decode_classes_with(text: &str, allow_param: bool) -> Result<Vec<Class>, ParseError> {
    crate::repr::parse_classes(text, allow_param)
        .ok_or_else(|| ParseError::from(format!("invalid representation classes {text}")))
}

fn class_str(class: Class) -> String {
    letters(&[class])
}

fn class_str_all(classes: &[Class]) -> String {
    letters(classes)
}

// -------------------------------------------------------------------- Core
//
// One canonical array per node, the tag first. The subset is exactly the
// class the inliner copies without a size test; a decoder that meets an
// unknown tag fails rather than inventing a node.

pub fn encode_core(e: &Core) -> Option<JsonValue> {
    encode_core_with(e, false)
}

pub fn encode_core_with(e: &Core, allow_template_nodes: bool) -> Option<JsonValue> {
    let rec = |sub: &Core| encode_core_with(sub, allow_template_nodes);
    Some(match e {
        Core::Int(n) => JsonValue::Arr(vec!["int".into(), n.to_string().into()]),
        Core::Wide(bits) => JsonValue::Arr(vec!["wide".into(), bits.to_string().into()]),
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
            rec(inner)?,
            (field.slot as u64).into(),
            class_str(field.class).into(),
            encode_access(field.access.as_ref()),
            field.mutable.into(),
            class_str_all(&field.before).into(),
        ]),
        Core::DictProj(inner, index, site, class) => {
            if site.is_some() {
                return None;
            }
            JsonValue::Arr(vec![
                "dictproj".into(),
                rec(inner)?,
                (*index as u64).into(),
                class_str(*class).into(),
            ])
        }
        Core::App(head, args) => JsonValue::Arr(vec![
            "app".into(),
            rec(head)?,
            encode_all(args, rec)?,
        ]),
        Core::Let(l, class, value, body) => JsonValue::Arr(vec![
            "let".into(),
            (l.0 as u64).into(),
            class_str(*class).into(),
            rec(value)?,
            rec(body)?,
        ]),
        Core::Case { scrut, alts } => JsonValue::Arr(vec![
            "case".into(),
            rec(scrut)?,
            encode_all(alts, |alt| {
                Some(JsonValue::Arr(vec![
                    encode_pat(&alt.pat),
                    alt.weight.map_or(JsonValue::Null, |w| (w as u64).into()),
                    rec(&alt.body)?,
                ]))
            })?,
        ]),
        Core::If(condition, then, otherwise) => JsonValue::Arr(vec![
            "if".into(),
            rec(condition)?,
            encode_branch_with(then, allow_template_nodes)?,
            encode_branch_with(otherwise, allow_template_nodes)?,
        ]),
        Core::Dup(l, class, body) => JsonValue::Arr(vec![
            "dup".into(),
            (l.0 as u64).into(),
            class_str(*class).into(),
            rec(body)?,
        ]),
        Core::Drop(l, class, body) => JsonValue::Arr(vec![
            "drop".into(),
            (l.0 as u64).into(),
            class_str(*class).into(),
            rec(body)?,
        ]),
        Core::Lam { fn_id, env, classes } if allow_template_nodes => JsonValue::Arr(vec![
            "lam".into(),
            fn_id.0.as_str().into(),
            encode_all(env, rec)?,
            letters(classes).into(),
        ]),
        Core::InstDict(id, subs, classes) if allow_template_nodes => JsonValue::Arr(vec![
            "instdict".into(),
            id.0.as_str().into(),
            encode_all(subs, rec)?,
            letters(classes).into(),
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

fn encode_branch_with(branch: &CBranch<Core>, allow_template_nodes: bool) -> Option<JsonValue> {
    Some(JsonValue::Arr(vec![
        branch.weight.map_or(JsonValue::Null, |w| (w as u64).into()),
        encode_core_with(&branch.body, allow_template_nodes)?,
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
    decode_core_with(value, false)
}

pub fn decode_core_with(value: &JsonValue, allow_param: bool) -> Result<Core, ParseError> {
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
        let mut classes = decode_classes_with(arg(i)?.as_str()?, allow_param)?;
        if classes.len() != 1 {
            return Err(ParseError::from(format!("optimisation node {tag} needs one class")));
        }
        Ok(classes.remove(0))
    };
    let local = |i: usize| -> Result<LocalId, ParseError> { Ok(LocalId(num(arg(i)?)?)) };
    Ok(match tag {
        "int" => Core::Int(signed(arg(1)?)?),
        "wide" => {
            let text = arg(1)?.as_str()?;
            Core::Wide(text.parse::<u128>().map_err(|_| ParseError::from(format!("invalid wide constant {text}")))?)
        }
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
            decode_classes_with(arg(2)?.as_str()?, allow_param)?,
        ),
        "ctor" => Core::Ctor(
            GlobalId(arg(1)?.as_str()?.to_string()),
            decode_classes_with(arg(2)?.as_str()?, allow_param)?,
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
            Box::new(decode_core_with(arg(1)?, allow_param)?),
            // The encoder's order: slot, class, access, mutable, and the
            // earlier fields' classes (absent in an interface written
            // before wide fields, where every earlier field is a word).
            ProjField {
                slot: num(arg(2)?)?,
                class: one_class(3)?,
                access: decode_access(arg(4)?)?,
                mutable: arg(5)?.as_bool()?,
                before: match parts.get(6) {
                    Some(v) => decode_classes_with(v.as_str()?, allow_param)?,
                    None => Vec::new(),
                },
            },
        ),
        "dictproj" => {
            Core::DictProj(Box::new(decode_core_with(arg(1)?, allow_param)?), num(arg(2)?)?, None, one_class(3)?)
        }
        "app" => {
            let head = Box::new(decode_core_with(arg(1)?, allow_param)?);
            let mut args = Vec::new();
            for a in arg(2)?.as_array()? {
                args.push(decode_core_with(a, allow_param)?);
            }
            Core::App(head, args)
        }
        "let" => Core::Let(
            local(1)?,
            one_class(2)?,
            Box::new(decode_core_with(arg(3)?, allow_param)?),
            Box::new(decode_core_with(arg(4)?, allow_param)?),
        ),
        "case" => {
            let scrut = Box::new(decode_core_with(arg(1)?, allow_param)?);
            let mut alts = Vec::new();
            for a in arg(2)?.as_array()? {
                let row = a.as_array()?;
                if row.len() != 3 {
                    return Err(ParseError::from(
                        "optimisation case alternative needs three fields".to_string(),
                    ));
                }
                alts.push(CAlt {
                    pat: decode_pat_with(&row[0], allow_param)?,
                    weight: weight(&row[1])?,
                    body: Box::new(decode_core_with(&row[2], allow_param)?),
                });
            }
            Core::Case { scrut, alts }
        }
        "if" => {
            let condition = Box::new(decode_core_with(arg(1)?, allow_param)?);
            let then = decode_branch_with(arg(2)?, allow_param)?;
            let otherwise = decode_branch_with(arg(3)?, allow_param)?;
            Core::If(condition, then, otherwise)
        }
        "dup" => Core::Dup(local(1)?, one_class(2)?, Box::new(decode_core_with(arg(3)?, allow_param)?)),
        "drop" => Core::Drop(local(1)?, one_class(2)?, Box::new(decode_core_with(arg(3)?, allow_param)?)),
        "lam" if allow_param => {
            let fn_id = GlobalId(arg(1)?.as_str()?.to_string());
            let mut env = Vec::new();
            for e in arg(2)?.as_array()? {
                env.push(decode_core_with(e, allow_param)?);
            }
            let classes = decode_classes_with(arg(3)?.as_str()?, allow_param)?;
            Core::Lam { fn_id, env, classes }
        }
        "instdict" if allow_param => {
            let id = GlobalId(arg(1)?.as_str()?.to_string());
            let mut subs = Vec::new();
            for s in arg(2)?.as_array()? {
                subs.push(decode_core_with(s, allow_param)?);
            }
            let classes = decode_classes_with(arg(3)?.as_str()?, allow_param)?;
            Core::InstDict(id, subs, classes)
        }
        other => return Err(ParseError::from(format!("unknown optimisation node {other}"))),
    })
}

fn decode_branch_with(value: &JsonValue, allow_param: bool) -> Result<CBranch<Core>, ParseError> {
    let parts = value.as_array()?;
    if parts.len() != 2 {
        return Err(ParseError::from("optimisation branch needs two fields".to_string()));
    }
    Ok(CBranch { weight: weight(&parts[0])?, body: Box::new(decode_core_with(&parts[1], allow_param)?) })
}

fn weight(value: &JsonValue) -> Result<Option<u32>, ParseError> {
    match value {
        JsonValue::Null => Ok(None),
        other => Ok(Some(other.as_num()? as u32)),
    }
}

fn decode_pat_with(value: &JsonValue, allow_param: bool) -> Result<CPat, ParseError> {
    let parts = value.as_array()?;
    let tag = parts
        .first()
        .ok_or_else(|| ParseError::from("empty optimisation pattern".to_string()))?
        .as_str()?;
    Ok(match tag {
        "var" => CPat::Var(
            LocalId(num(parts.get(1).ok_or_else(|| ParseError::from("pattern needs an id".to_string()))?)?),
            decode_classes_with(
                parts.get(2).ok_or_else(|| ParseError::from("pattern needs a class".to_string()))?.as_str()?,
                allow_param,
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
            let classes = decode_classes_with(
                parts
                    .get(3)
                    .ok_or_else(|| ParseError::from("constructor pattern needs classes".to_string()))?
                    .as_str()?,
                allow_param,
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
    pub templates: Vec<FnDef>,
    pub modules: BTreeSet<String>,
}

/// Authenticate the package's optional unfoldings against its checked
/// contracts before exposing bodies to the inliner.
pub fn load_package(path: &std::path::Path) -> crate::Result<ForeignTable> {
    let (interfaces, metadata) = crate::package_interfaces::read_interfaces_with_abi(path)?;
    let mut table = ForeignTable { bodies: Vec::new(), templates: Vec::new(), modules: BTreeSet::new() };
    let Some(JsonValue::Obj(metadata)) = metadata else {
        return Ok(table);
    };
    let Some(optimisations) = metadata.get("optimisations") else {
        return Ok(table);
    };
    let error = |message: String| crate::Error::plain(format!("{}: {message}", path.display()));
    let mut contracts: BTreeMap<_, _> = interfaces.into_iter().map(|(iface, hash)| (iface.module, hash)).collect();
    // A private compiler-owned module can supply implementation evidence
    // to its object consumer without entering the public module directory.
    if let Some(JsonValue::Arr(rows)) = metadata.get("private_interfaces") {
        for row in rows {
            let fields = row.as_object().map_err(|e| error(e.to_string()))?;
            let name = field_str(fields, "module").map_err(|e| error(e.to_string()))?;
            let declared = field_str(fields, "iface_hash").map_err(|e| error(e.to_string()))?;
            let document = field(fields, "interface").map_err(|e| error(e.to_string()))?;
            let iface = Iface::from_document(document).map_err(|e| error(e.to_string()))?;
            if iface.module != name || iface.iface_hash() != declared
                || contracts.insert(name.clone(), declared).is_some()
            {
                return Err(error(format!("invalid private interface for {name}")));
            }
        }
    }
    for (module, document) in optimisations.as_object().map_err(|e| error(e.to_string()))? {
        let artifact = Optimisation::from_document(document, true).map_err(|e| error(e.to_string()))?;
        let iface_hash = contracts.get(module)
            .ok_or_else(|| error(format!("optimisation artifact has no checked interface for {module}")))?;
        if &artifact.module != module || &artifact.iface_hash != iface_hash {
            return Err(error(format!(
                "optimisation artifact for {module} was produced against interface {} but the package pins {iface_hash}",
                artifact.iface_hash,
            )));
        }
        table.modules.insert(module.clone());
        table.bodies.extend(foreign_defs(&artifact));
        table.templates.extend(template_defs(&artifact));
    }
    Ok(table)
}

/// The [`FnDef`]s a consumer's inliner consults for one artifact's bodies.
pub fn foreign_defs(artifact: &Optimisation) -> Vec<FnDef> {
    artifact.bodies.iter().cloned().map(body_def).collect()
}

/// The polymorphic template [`FnDef`]s a consumer's specializer consults to
/// instantiate external polymorphic functions/constructors at local `Val` layouts.
pub fn template_defs(artifact: &Optimisation) -> Vec<FnDef> {
    artifact.templates.iter().cloned().map(template_def).collect()
}

fn template_def(tpl: TemplateExport) -> FnDef {
    let params = (0..tpl.arity).map(|i| format!("a{i}")).collect();
    FnDef {
        id: GlobalId(tpl.unit),
        params,
        body: tpl.body,
        prim: None,
        ctor: tpl.ctor,
        hints: crate::core::FnHints::default(),
        meta: crate::core::FnMeta::default(),
        repr: crate::core::UnitRepr {
            tparams: tpl.tparams,
            result: tpl.result,
            dictionary_values: Vec::new(),
            params: tpl.params,
            fields: tpl.fields,
            data: tpl.data,
            ..Default::default()
        },
    }
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
            ..Default::default()
        },
    }
}

//! C-callable exports and C++ classes whose implementations live in Cinch.
use super::{cpp_name, direct_declaration, fail, identifier, scalar, Bindings, DirectImport};
use crate::Result;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Function {
    pub name: String,
    #[serde(default)]
    pub symbol: Option<String>,
    pub function: String,
    #[serde(default)]
    pub params: Vec<String>,
    pub result: String,
    #[serde(default)]
    pub effects: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Method {
    pub name: String,
    pub function: String,
    #[serde(default)]
    pub params: Vec<String>,
    pub result: String,
    #[serde(default)]
    pub effects: Vec<String>,
    #[serde(default, rename = "override")]
    pub override_method: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default)]
    pub initial: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hook {
    pub function: String,
    #[serde(default)]
    pub effects: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Class {
    pub name: String,
    pub cpp: String,
    #[serde(default)]
    pub base: Option<String>,
    #[serde(default)]
    pub tool: bool,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub signals: Vec<Signal>,
    #[serde(default)]
    pub fields: Vec<Field>,
    #[serde(default)]
    pub methods: Vec<Method>,
    #[serde(default)]
    pub native_methods: Vec<super::Function>,
    #[serde(default)]
    pub constructor: Option<Hook>,
    #[serde(default)]
    pub destructor: Option<Hook>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Signal {
    pub name: String,
    #[serde(default)]
    pub params: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Godot {
    #[serde(default = "entry_default")]
    pub entry_symbol: String,
    /// An existing godot-cpp checkout; absent means fetch the pinned SDK.
    #[serde(default)]
    pub godot_cpp: Option<String>,
}
fn entry_default() -> String {
    "cinch_godot_init".into()
}

#[derive(Default)]
pub struct Generated {
    pub names: Vec<(String, bool)>,
    pub api: String,
    pub cpp: String,
    pub header: String,
    pub c_header: String,
    pub exports: String,
}

pub(super) fn c_type(ty: &str) -> Result<String> {
    if let Some(t) = super::godot::ty(ty) {
        return Ok(t.cpp.into());
    }
    if ty == "Bool" {
        return Ok("bool".into());
    }
    if ty == "Object" {
        return Ok("godot::Object*".into());
    }
    if [
        "RawPtr ",
        "Borrowed ",
        "Retained ",
        "Owned ",
        "Out ",
        "MaybeNull ",
    ]
    .iter()
    .any(|p| ty.starts_with(p))
    {
        Ok("void*".into())
    } else {
        Ok(scalar(ty)?.into())
    }
}
fn cap(name: &str) -> String {
    let mut chars = name.chars();
    format!(
        "{}{}",
        chars.next().unwrap().to_ascii_uppercase(),
        chars.as_str()
    )
}
fn qualified_function(name: &str) -> Result<String> {
    let Some((module, function)) = name.rsplit_once('.') else {
        return Err(fail("an exported implementation must be Module.function"));
    };
    let word = |s: &str, upper: bool| {
        crate::lexer::lex(s)
            .ok()
            .is_some_and(|tokens| match tokens.first().map(|t| &t.tok) {
                Some(crate::lexer::Tok::ConId(text)) if upper => text == s,
                Some(crate::lexer::Tok::VarId(text)) if !upper => text == s,
                _ => false,
            })
    };
    if !module.split('.').all(|n| word(n, true)) || !word(function, false) {
        return Err(fail(format!("invalid Cinch implementation `{name}`")));
    }
    Ok(module.into())
}
fn signature(params: &[String], result: &str, effects: &[String]) -> String {
    let params = if params.is_empty() {
        "Unit".into()
    } else {
        params.join(" -> ")
    };
    if effects.is_empty() {
        format!("{params} -> {result}")
    } else {
        format!("{params} ->{{{}}} {result}", effects.join(", "))
    }
}

struct Context {
    imports: BTreeMap<String, String>,
    proxies: String,
    prototypes: String,
    next: usize,
    symbols: BTreeSet<String>,
    godot: bool,
}
impl Context {
    fn reserve(&mut self, symbol: &str) -> Result<()> {
        if !self.symbols.insert(symbol.into()) {
            return Err(fail(format!(
                "duplicate generated export symbol `{symbol}`"
            )));
        }
        Ok(())
    }
    fn callback(
        &mut self,
        symbol: &str,
        target: &str,
        params: &[String],
        result: &str,
        effects: &[String],
    ) -> Result<()> {
        self.reserve(symbol)?;
        let module = qualified_function(target)?;
        let next_alias = format!("ExportModule{}", self.imports.len());
        let alias = self.imports.entry(module).or_insert(next_alias);
        let target = format!("{alias}.{}", target.rsplit('.').next().unwrap());
        let mapped = self.godot
            && (params.iter().any(|t| super::godot::rich(t)) || super::godot::rich(result));
        let native_params = params
            .iter()
            .map(|t| {
                if self.godot {
                    super::godot::low(t, false)
                } else {
                    t.clone()
                }
            })
            .collect::<Vec<_>>();
        let native_result = if self.godot {
            super::godot::low(result, true)
        } else {
            result.into()
        };
        let mut native_effects = effects.to_vec();
        if mapped {
            for effect in ["Foreign Cxx", "IO", "Diverge"] {
                if !native_effects.iter().any(|e| e == effect) {
                    native_effects.push(effect.into());
                }
            }
        }
        let ty = signature(&native_params, &native_result, &native_effects);
        direct_declaration(&DirectImport {
            name: "probe".into(),
            symbol: symbol.into(),
            signature: ty.clone(),
        })?;
        let proxy = format!("export{}", self.next);
        self.next += 1;
        let args = (0..params.len().max(1))
            .map(|i| format!("a{i}"))
            .collect::<Vec<_>>();
        let mut body = String::new();
        let mut pass = Vec::new();
        for (i, t) in params.iter().enumerate() {
            if self.godot && (super::godot::needs_copy(t) || t == "Bool") {
                body.push_str(&format!(
                    "let v{i} = {} in ",
                    super::godot::read(t, &args[i])
                ));
                pass.push(format!("v{i}"));
            } else {
                pass.push(args[i].clone());
            }
        }
        if params.is_empty() {
            pass.push("a0".into());
        }
        body.push_str(&format!("let result = {target} {} in ", pass.join(" ")));
        body.push_str(&if self.godot {
            super::godot::write(result, "result")
        } else {
            "result".into()
        });
        self.proxies.push_str(&format!("  {proxy} :: {ty}\n  {proxy} {} = {body}\n  foreign export ccall \"{symbol}\" {proxy} :: {ty}\n",args.join(" ")));
        let params = native_params
            .iter()
            .filter(|t| t.as_str() != "Unit")
            .enumerate()
            .map(|(i, t)| Ok(format!("{} a{i}", c_type(t)?)))
            .collect::<Result<Vec<_>>>()?;
        self.prototypes.push_str(&format!(
            "extern \"C\" {} {symbol}({}) noexcept;\n",
            c_type(&native_result)?,
            params.join(", ")
        ));
        Ok(())
    }
}

pub fn generate(b: &Bindings) -> Result<Generated> {
    let mut g = Generated::default();
    if b.exports.is_empty() && b.export_classes.is_empty() {
        if b.godot.is_some() {
            return Err(fail("Godot output requires at least one exported class"));
        }
        return Ok(g);
    }
    if b.godot.is_none() {
        let rich_free = b.exports.iter().any(|f| {
            f.params
                .iter()
                .chain(std::iter::once(&f.result))
                .any(|t| super::godot::rich(t))
        });
        let rich_classes = b.export_classes.iter().any(|c| {
            c.script.is_some()
                || !c.signals.is_empty()
                || c.fields.iter().any(|f| super::godot::rich(&f.ty))
                || c.methods.iter().any(|m| {
                    m.params
                        .iter()
                        .chain(std::iter::once(&m.result))
                        .any(|t| super::godot::rich(t))
                })
                || c.native_methods.iter().any(|m| {
                    m.params
                        .iter()
                        .chain(std::iter::once(&m.result))
                        .any(|t| super::godot::rich(t))
                })
        });
        if rich_free || rich_classes {
            return Err(fail(
                "Godot value types, scripts, and signals require godot output",
            ));
        }
    }
    let prefix = format!("cinch_export_{}", b.module.replace('.', "_"));
    let mut cx = Context {
        imports: BTreeMap::new(),
        proxies: String::new(),
        prototypes: String::new(),
        next: 0,
        symbols: BTreeSet::new(),
        godot: b.godot.is_some(),
    };
    let bridge = super::godot::bridge(b)?;
    g.names.extend(bridge.names);
    g.api.push_str(&bridge.api);
    let mut bodies = String::new();
    let mut free_names = BTreeSet::new();
    let mut cpp_classes = BTreeSet::new();
    for f in &b.exports {
        if !identifier(&f.name, false) || !free_names.insert(f.name.clone()) {
            return Err(fail("invalid or duplicate exported function name"));
        }
        let symbol = f
            .symbol
            .clone()
            .unwrap_or_else(|| format!("{prefix}_{}", f.name));
        if !cpp_name(&symbol) || symbol.contains("::") {
            return Err(fail(
                "an export symbol in a generated header must be a C identifier",
            ));
        }
        cx.callback(&symbol, &f.function, &f.params, &f.result, &f.effects)?;
        let (args, values) = cpp_arguments(&f.params)?;
        if f.symbol.is_none() {
            let body = cpp_callback(&symbol, &[], &f.params, &f.result, b.godot.is_some());
            bodies.push_str(&format!(
                "inline {} {}({}) noexcept {{ {body} }}\n",
                c_type(&f.result)?,
                f.name,
                args.join(", ")
            ));
        }
    }
    let mut api_bodies = String::new();
    let mut godot_bases = BTreeSet::new();
    for c in &b.export_classes {
        super::validate_opaque_name(&c.name)?;
        if !identifier(&c.name, true) || !cpp_name(&c.cpp) || !cpp_classes.insert(c.cpp.clone()) {
            return Err(fail("invalid or duplicate exported C++ class"));
        }
        let parts = c.cpp.split("::").collect::<Vec<_>>();
        let short = parts.last().unwrap();
        if b.godot.is_some() && c.script.is_none() && *short != c.name {
            return Err(fail("a Godot class's C++ and Cinch names must match"));
        }
        if free_names.contains(*short) {
            return Err(fail(
                "an exported function and class cannot share a C++ name",
            ));
        }
        let base = c.base.as_deref();
        if let Some(base) = base {
            if !cpp_name(base) {
                return Err(fail("invalid C++ base class"));
            }
        }
        if b.godot.is_some() {
            let base = base.ok_or_else(|| fail("a Godot class needs a godot:: base"))?;
            let header = godot_base_header(base)?;
            godot_bases.insert(header);
        }
        g.names.push((c.name.clone(), true));
        g.api
            .push_str(&format!("  opaque type {} :: Type\n", c.name));
        let borrowed = format!("Borrowed {}", c.name);
        if b.godot.is_none() {
            let owner = format!(
                "Owned {}{}Heap {}",
                b.module.replace('.', ""),
                c.name,
                c.name
            );
            for delete in [false, true] {
                let verb = if delete { "delete" } else { "new" };
                let name = format!("{verb}{}", c.name);
                let symbol = format!("{prefix}_{}_{verb}", c.name);
                cx.reserve(&symbol)?;
                g.names.push((name.clone(), false));
                let params = if delete {
                    vec![owner.clone()]
                } else {
                    Vec::new()
                };
                let result = if delete { "Unit" } else { &owner };
                g.api.push_str(&direct_declaration(&DirectImport {
                    name,
                    symbol: symbol.clone(),
                    signature: signature(
                        &params,
                        result,
                        &["Foreign Cxx".into(), "IO".into(), "Diverge".into()],
                    ),
                })?);
                if delete {
                    cx.prototypes.push_str(&format!(
                        "extern \"C\" void {symbol}(void* self) noexcept;\n"
                    ));
                    api_bodies.push_str(&format!("extern \"C\" void {symbol}(void* self) noexcept {{ delete static_cast<{}*>(self); }}\n", c.cpp));
                } else {
                    cx.prototypes
                        .push_str(&format!("extern \"C\" void* {symbol}() noexcept;\n"));
                    api_bodies.push_str(&format!("extern \"C\" void* {symbol}() noexcept {{ try {{ return new {}; }} catch (...) {{ std::terminate(); }} }}\n", c.cpp));
                }
            }
        }
        let mut members = BTreeSet::new();
        let mut fields = String::new();
        let mut methods = String::new();
        let mut binds = String::new();
        for f in &c.fields {
            if !identifier(&f.name, false) || !members.insert(f.name.clone()) || f.ty == "Unit" {
                return Err(fail("invalid or duplicate exported field"));
            }
            let ty = c_type(&f.ty)?;
            let initial = if b.godot.is_some() && super::godot::rich(&f.ty) {
                super::godot::initial(&f.ty, &f.initial)?
            } else {
                field_initial(f)?
            };
            fields.push_str(&format!(
                "  {ty} field_{} = static_cast<{ty}>({initial});\n",
                f.name
            ));
            for verb in ["get", "set"] {
                if !members.insert(format!("{verb}_{}", f.name)) {
                    return Err(fail("field accessor collides with a member"));
                }
            }
            methods.push_str(&format!("  {ty} get_{}() const noexcept {{ return field_{}; }}\n  void set_{}({ty} value) noexcept {{ field_{} = value; }}\n", f.name, f.name, f.name, f.name));
            if b.godot.is_some() && c.script.is_none() {
                let kind = super::godot::variant_kind(&f.ty)?;
                let usage = if f.ty == "Variant" {
                    ", godot::PROPERTY_HINT_NONE, \"\", godot::PROPERTY_USAGE_DEFAULT | godot::PROPERTY_USAGE_NIL_IS_VARIANT"
                } else {
                    ""
                };
                binds.push_str(&format!("    godot::ClassDB::bind_method(godot::D_METHOD(\"get_{}\"), &{}::get_{});\n    godot::ClassDB::bind_method(godot::D_METHOD(\"set_{}\", \"value\"), &{}::set_{});\n    ADD_PROPERTY(godot::PropertyInfo(godot::Variant::{kind}, \"{}\"{usage}), \"set_{}\", \"get_{}\");\n", f.name, short, f.name, f.name, short, f.name, f.name, f.name, f.name));
            }
            for set in [false, true] {
                let verb = if set { "set" } else { "get" };
                let name = format!("{verb}{}{}", c.name, cap(&f.name));
                let symbol = format!("{prefix}_{}_{}_{}", c.name, verb, f.name);
                cx.reserve(&symbol)?;
                let mut params = vec![borrowed.clone()];
                if set {
                    params.push(f.ty.clone());
                }
                let result = if set { "Unit" } else { &f.ty };
                api_import(&mut g, &name, &symbol, &params, result, b.godot.is_some())?;
                let cparams = ffi_arguments(&params, b.godot.is_some())?;
                let input = cpp_arg(&f.ty, "a1", b.godot.is_some());
                let action = if set {
                    format!("self->field_{} = {input};", f.name)
                } else {
                    cpp_return(&f.ty, &format!("self->field_{}", f.name), b.godot.is_some())
                };
                let ret = if b.godot.is_some() {
                    super::godot::low(result, true)
                } else {
                    result.into()
                };
                api_bodies.push_str(&format!("extern \"C\" {} {symbol}({}) noexcept {{ auto* self = static_cast<{}*>(a0); {action} }}\n",c_type(&ret)?,cparams.join(", "),c.cpp));
            }
        }
        for m in &c.methods {
            if !cpp_name(&m.name) || m.name.contains("::") || !members.insert(m.name.clone()) {
                return Err(fail("invalid or duplicate exported method"));
            }
            if m.override_method && base.is_none() {
                return Err(fail("an override requires a base class"));
            }
            let symbol = format!("{prefix}_{}_{}", c.name, m.name);
            let mut params = vec![borrowed.clone()];
            params.extend(m.params.clone());
            cx.callback(&symbol, &m.function, &params, &m.result, &m.effects)?;
            let (args, values) = cpp_arguments(&m.params)?;
            let body = cpp_callback(&symbol, &["this"], &m.params, &m.result, b.godot.is_some());
            methods.push_str(&format!(
                "  {} {}({}) noexcept{} {{ {body} }}\n",
                c_type(&m.result)?,
                m.name,
                args.join(", "),
                if m.override_method && c.script.is_none() {
                    " override"
                } else {
                    ""
                }
            ));
            if b.godot.is_some() && c.script.is_none() && !m.override_method {
                let args = (0..m.params.len())
                    .map(|i| format!(", \"arg{i}\""))
                    .collect::<String>();
                binds.push_str(&format!(
                    "    godot::ClassDB::bind_method(godot::D_METHOD(\"{}\"{args}), &{}::{});\n",
                    m.name, short, m.name
                ));
            }
        }
        for m in &c.native_methods {
            if !identifier(&m.name, false) || !cpp_name(&m.cpp) || m.cpp.contains("::") {
                return Err(fail("invalid native receiver method"));
            }
            let name = format!("{}{}", m.name, c.name);
            let symbol = format!("{prefix}_{}_native_{}", c.name, m.name);
            cx.reserve(&symbol)?;
            let mut params = vec![borrowed.clone()];
            params.extend(m.params.clone());
            api_import(
                &mut g,
                &name,
                &symbol,
                &params,
                &m.result,
                b.godot.is_some(),
            )?;
            let args = ffi_arguments(&params, b.godot.is_some())?;
            let values = m
                .params
                .iter()
                .enumerate()
                .map(|(i, t)| cpp_arg(t, &format!("a{}", i + 1), b.godot.is_some()))
                .collect::<Vec<_>>();
            let receiver = if c.script.is_some() {
                format!(
                    "godot::Object::cast_to<{}>(static_cast<{}*>(a0)->_cinch_owner)",
                    base.unwrap(),
                    c.cpp
                )
            } else {
                format!("static_cast<{}*>(a0)", c.cpp)
            };
            let action = cpp_return(
                &m.result,
                &format!("receiver->{}({})", m.cpp, values.join(", ")),
                b.godot.is_some(),
            );
            let result = if b.godot.is_some() {
                super::godot::low(&m.result, true)
            } else {
                m.result.clone()
            };
            api_bodies.push_str(&format!("extern \"C\" {} {symbol}({}) noexcept {{ auto* receiver={receiver}; if(!receiver)std::terminate(); {action} }}\n",c_type(&result)?,args.join(", ")));
        }
        if b.godot.is_some() {
            let owner_name = format!("get{}Object", c.name);
            let identity_name = format!("get{}ObjectId", c.name);
            let owner_symbol = format!("{prefix}_{}_object", c.name);
            cx.reserve(&owner_symbol)?;
            api_import(
                &mut g,
                &identity_name,
                &owner_symbol,
                &[borrowed.clone()],
                "U64",
                true,
            )?;
            g.names.push((owner_name.clone(), false));
            // A Borrowed receiver promises not to capture its address, including
            // in the return value. Resolve its engine identity in a separate call
            // rather than returning a base pointer that aliases a native receiver.
            g.api.push_str(&format!("  {owner_name} :: {borrowed} ->{{Foreign Cxx, IO}} MaybeNull (RawPtr GDObject)\n  {owner_name} self = gdObjectFromId ({identity_name} self)\n"));
            let owner = if c.script.is_some() {
                format!("static_cast<{}*>(a0)->_cinch_owner", c.cpp)
            } else {
                format!("static_cast<{}*>(a0)", c.cpp)
            };
            api_bodies.push_str(&format!("extern \"C\" uint64_t {owner_symbol}(void* a0) noexcept {{ auto* owner=static_cast<godot::Object*>({owner}); return owner ? owner->get_instance_id() : 0; }}\n"));
            let mut signal_names = BTreeSet::new();
            for signal in &c.signals {
                if !cpp_name(&signal.name)
                    || signal.name.contains("::")
                    || members.contains(&signal.name)
                    || signal.params.iter().any(|p| p == "Unit")
                    || !signal_names.insert(signal.name.clone())
                {
                    return Err(fail("invalid or duplicate Godot signal"));
                }
                let name = format!("emit{}{}", c.name, cap(&signal.name));
                let symbol = format!("{prefix}_{}_signal_{}", c.name, signal.name);
                cx.reserve(&symbol)?;
                let mut params = vec![borrowed.clone()];
                params.extend(signal.params.clone());
                api_import(&mut g, &name, &symbol, &params, "Unit", true)?;
                let args = ffi_arguments(&params, true)?;
                let mut values = vec![format!("\"{}\"", signal.name)];
                values.extend(
                    signal
                        .params
                        .iter()
                        .enumerate()
                        .map(|(i, t)| cpp_arg(t, &format!("a{}", i + 1), true)),
                );
                api_bodies.push_str(&format!("extern \"C\" void {symbol}({}) noexcept {{ auto* owner=static_cast<godot::Object*>({owner}); if(!owner)cinch_gd_bridge::fault(); auto status=owner->emit_signal({}); if(status!=godot::OK && status!=godot::ERR_UNAVAILABLE)cinch_gd_bridge::fault(); }}\n",args.join(", "),values.join(", ")));
                if c.script.is_none() {
                    let mut info = vec![format!("\"{}\"", signal.name)];
                    for (i, t) in signal.params.iter().enumerate() {
                        let usage = if t == "Variant" {
                            ", godot::PROPERTY_HINT_NONE, \"\", godot::PROPERTY_USAGE_DEFAULT | godot::PROPERTY_USAGE_NIL_IS_VARIANT"
                        } else {
                            ""
                        };
                        info.push(format!(
                            "godot::PropertyInfo(godot::Variant::{}, \"arg{i}\"{usage})",
                            super::godot::variant_kind(t)?
                        ));
                    }
                    binds.push_str(&format!(
                        "    ADD_SIGNAL(godot::MethodInfo({}));\n",
                        info.join(", ")
                    ));
                }
            }
        }
        let mut create = String::new();
        let mut destroy = String::new();
        for (hook, verb, body) in [
            (&c.constructor, "create", &mut create),
            (&c.destructor, "destroy", &mut destroy),
        ] {
            if let Some(hook) = hook {
                let symbol = format!("{prefix}_{}_{verb}", c.name);
                cx.callback(
                    &symbol,
                    &hook.function,
                    &[borrowed.clone()],
                    "Unit",
                    &hook.effects,
                )?;
                body.push_str(&format!("{symbol}(this);"));
            }
        }
        for namespace in &parts[..parts.len() - 1] {
            bodies.push_str(&format!("namespace {namespace} {{\n"));
        }
        bodies.push_str(&format!(
            "class {short}{} {{\n",
            if c.script.is_some() {
                String::new()
            } else {
                base.map(|v| format!(" : public {v}")).unwrap_or_default()
            }
        ));
        if b.godot.is_some() && c.script.is_none() {
            bodies.push_str(&format!("  GDCLASS({short}, {});\n", base.unwrap()));
        }
        // State is public for the C ABI field accessors; copy/move are deleted
        // so this identity cannot silently change under a live Cinch borrow.
        let assignments = if b.godot.is_some() && c.script.is_none() {
            // GDCLASS already declares its own private copy assignment.
            String::new()
        } else {
            format!("  {short}& operator=(const {short}&) = delete;\n  {short}& operator=({short}&&) = delete;\n")
        };
        let constructor = if c.script.is_some() {
            format!("{short}(godot::Object* owner) noexcept : _cinch_owner(owner) {{ {create} }}")
        } else {
            format!("{short}() noexcept {{ {create} }}")
        };
        let owner_field = if c.script.is_some() {
            "  godot::Object* _cinch_owner;\n"
        } else {
            ""
        };
        bodies.push_str(&format!("public:\n{owner_field}{fields}  {constructor}\n  ~{short}() noexcept {{ {destroy} }}\n  {short}(const {short}&) = delete;\n  {short}({short}&&) = delete;\n{assignments}{methods}"));
        if b.godot.is_some() && c.script.is_none() {
            bodies.push_str(&format!(
                "protected:\n  static void _bind_methods() {{\n{binds}  }}\n"
            ));
        }
        bodies.push_str("};\n");
        for _ in &parts[..parts.len() - 1] {
            bodies.push_str("}\n");
        }
    }
    let stem = b.module.replace('.', "_");
    let mut includes = String::from("#pragma once\n#include <cstdint>\n");
    for header in &b.export_headers {
        if header.is_empty()
            || header
                .chars()
                .any(|c| c.is_control() || c == '"' || c == '\\')
        {
            return Err(fail("invalid export header path"));
        }
        includes.push_str(&format!("#include \"{header}\"\n"));
    }
    if b.godot.is_some() {
        includes
            .push_str("#include <godot_cpp/core/class_db.hpp>\n#include <godot_cpp/godot.hpp>\n");
        includes.push_str("#include <memory>\n#include <godot_cpp/variant/variant.hpp>\n#include <godot_cpp/variant/string.hpp>\n#include <godot_cpp/variant/string_name.hpp>\n#include <godot_cpp/variant/array.hpp>\n#include <godot_cpp/variant/dictionary.hpp>\n#include <godot_cpp/variant/vector2.hpp>\n#include <godot_cpp/variant/vector3.hpp>\n#include <godot_cpp/variant/vector4.hpp>\n#include <godot_cpp/variant/vector2i.hpp>\n#include <godot_cpp/variant/vector3i.hpp>\n#include <godot_cpp/variant/vector4i.hpp>\n#include <godot_cpp/variant/color.hpp>\n");
        for base in godot_bases {
            includes.push_str(&format!("#include <godot_cpp/classes/{base}.hpp>\n"));
        }
    }
    let prototypes = cx
        .prototypes
        .replace("extern \"C\" ", "")
        .replace("std::", "")
        .replace("()", "(void)")
        .lines()
        .map(|line| {
            format!(
                "CINCH_FFI_EXPORT {}",
                line.replace(" noexcept;", " CINCH_FFI_NOEXCEPT;")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    g.c_header = format!("#pragma once\n#include <stdint.h>\n#if defined(__GNUC__)\n#define CINCH_FFI_EXPORT __attribute__((visibility(\"default\")))\n#else\n#define CINCH_FFI_EXPORT\n#endif\n#ifdef __cplusplus\nextern \"C\" {{\n#define CINCH_FFI_NOEXCEPT noexcept\n#else\n#define CINCH_FFI_NOEXCEPT\n#endif\n{prototypes}\n#ifdef __cplusplus\n}}\n#endif\n#undef CINCH_FFI_NOEXCEPT\n#undef CINCH_FFI_EXPORT\n");
    g.header = format!("{includes}\n#include \"{stem}.h\"\n{bodies}");
    g.cpp = format!(
        "{}\n#include \"{stem}.hpp\"\n#include <exception>\n{api_bodies}",
        bridge.cpp
    );
    if let Some(godot) = &b.godot {
        cx.reserve(&godot.entry_symbol)?;
        if !cpp_name(&godot.entry_symbol)
            || godot.entry_symbol.contains("::")
            || b.export_classes.is_empty()
        {
            return Err(fail("invalid Godot entry or missing class"));
        }
        // Native-class implementation modules are editor resources too.
        g.cpp.push_str(&super::script::generate(b)?);
        g.cpp.push_str(include_str!("script_highlighter.hpp"));
        g.cpp.push_str("\nstatic void cinch_godot_initialize(godot::ModuleInitializationLevel level) {\n  if (level == godot::MODULE_INITIALIZATION_LEVEL_EDITOR) {\n");
        g.cpp.push_str("cinch_scripts::initialize_editor();\n");
        g.cpp.push_str(
            "return; }\n  if (level != godot::MODULE_INITIALIZATION_LEVEL_SCENE) return;\n",
        );
        for c in b.export_classes.iter().filter(|c| c.script.is_none()) {
            let register = if c.tool {
                "register_class"
            } else {
                "register_runtime_class"
            };
            g.cpp
                .push_str(&format!("  godot::ClassDB::{register}<{}>();\n", c.cpp));
        }
        g.cpp.push_str("cinch_scripts::initialize();\n");
        g.cpp.push_str(
            "}\nstatic void cinch_godot_terminate(godot::ModuleInitializationLevel level) {\n",
        );
        g.cpp.push_str("if(level==godot::MODULE_INITIALIZATION_LEVEL_EDITOR)cinch_scripts::terminate_editor();\nif(level==godot::MODULE_INITIALIZATION_LEVEL_SCENE)cinch_scripts::terminate();\n");
        g.cpp.push_str(&format!("}}\nextern \"C\" GDExtensionBool GDE_EXPORT {}(GDExtensionInterfaceGetProcAddress get_proc, GDExtensionClassLibraryPtr library, GDExtensionInitialization* initialization) {{\n  godot::GDExtensionBinding::InitObject init(get_proc, library, initialization);\n  init.register_initializer(cinch_godot_initialize);\n  init.register_terminator(cinch_godot_terminate);\n  init.set_minimum_library_initialization_level(godot::MODULE_INITIALIZATION_LEVEL_SCENE);\n  return init.init();\n}}\n", godot.entry_symbol));
    }
    g.exports = format!(
        "-- Generated native export entries.\nmodule {}Exports where\n  import {}\n",
        b.module, b.module
    );
    for (import, alias) in cx.imports {
        g.exports
            .push_str(&format!("  import {import} as {alias}\n"));
    }
    g.exports.push_str(&cx.proxies);
    Ok(g)
}

pub(super) fn field_initial(field: &Field) -> Result<String> {
    let value = if field.initial.is_null() {
        serde_json::json!(0)
    } else {
        field.initial.clone()
    };
    let invalid = || {
        fail(format!(
            "initializer for {} must fit {}",
            field.name, field.ty
        ))
    };
    if field.ty.starts_with('F') {
        let n = value.as_f64().ok_or_else(invalid)?;
        if !n.is_finite() || (field.ty == "F32" && !(n as f32).is_finite()) {
            return Err(invalid());
        }
        return Ok(format!("{n:e}"));
    }
    let bits = field.ty[1..].parse::<u32>().map_err(|_| invalid())?;
    if field.ty.starts_with('I') {
        let n = value.as_i64().ok_or_else(invalid)?;
        if bits < 64
            && (i128::from(n) < -(1i128 << (bits - 1)) || i128::from(n) >= (1i128 << (bits - 1)))
        {
            return Err(invalid());
        }
        if n == i64::MIN {
            Ok("(-9223372036854775807LL - 1)".into())
        } else {
            Ok(format!("{n}LL"))
        }
    } else {
        let n = value.as_u64().ok_or_else(invalid)?;
        if bits < 64 && u128::from(n) >= (1u128 << bits) {
            return Err(invalid());
        }
        Ok(format!("{n}ULL"))
    }
}

fn cpp_arguments(params: &[String]) -> Result<(Vec<String>, Vec<String>)> {
    let mut args = Vec::new();
    let mut values = Vec::new();
    for (i, ty) in params.iter().enumerate() {
        if ty == "Unit" {
            return Err(fail("Unit parameters must be elided"));
        }
        args.push(if super::godot::ty(ty).is_some() {
            format!("const {}& a{i}", c_type(ty)?)
        } else {
            format!("{} a{i}", c_type(ty)?)
        });
        values.push(format!("a{i}"));
    }
    Ok((args, values))
}

pub fn godot_base_header(base: &str) -> Result<String> {
    Ok(match base {
        "godot::Node" => "node", "godot::Node2D" => "node2d", "godot::Node3D" => "node3d",
        "godot::RefCounted" => "ref_counted", "godot::Resource" => "resource", "godot::Control" => "control",
        _ => return Err(fail("supported Godot bases: Node, Node2D, Node3D, RefCounted, Resource, Control (qualified godot::)")),
    }.into())
}

pub fn extra_files(b: &Bindings) -> Result<BTreeMap<String, String>> {
    let g = generate(b)?;
    let mut files = BTreeMap::new();
    if !g.header.is_empty() {
        files.insert(format!("{}.h", b.module.replace('.', "_")), g.c_header);
        files.insert(format!("{}.hpp", b.module.replace('.', "_")), g.header);
        files.insert(
            format!("{}Exports.cinch", b.module.replace('.', "/")),
            g.exports,
        );
    }
    Ok(files)
}

fn api_import(
    g: &mut Generated,
    name: &str,
    symbol: &str,
    params: &[String],
    result: &str,
    godot: bool,
) -> Result<()> {
    if !godot || (!params.iter().any(|p| super::godot::rich(p)) && !super::godot::rich(result)) {
        g.names.push((name.into(), false));
        g.api.push_str(&direct_declaration(&DirectImport {
            name: name.into(),
            symbol: symbol.into(),
            signature: signature(params, result, &["Foreign Cxx".into(), "IO".into()]),
        })?);
        return Ok(());
    }
    let raw = format!("raw{}", cap(name));
    let native = params
        .iter()
        .map(|p| super::godot::low(p, false))
        .collect::<Vec<_>>();
    let nr = super::godot::low(result, true);
    g.names.extend([(name.into(), false), (raw.clone(), false)]);
    g.api.push_str(&direct_declaration(&DirectImport {
        name: raw.clone(),
        symbol: symbol.into(),
        signature: signature(&native, &nr, &["Foreign Cxx".into(), "IO".into()]),
    })?);
    let logical = params
        .iter()
        .map(|p| super::godot::logical(p, false))
        .collect::<Vec<_>>();
    let lr = super::godot::logical(result, true);
    let args = (0..params.len().max(1))
        .map(|i| format!("a{i}"))
        .collect::<Vec<_>>();
    let mut pass = Vec::new();
    let mut body = String::new();
    let mut deletes = Vec::new();
    for (i, p) in params.iter().enumerate() {
        if super::godot::needs_copy(p) {
            body.push_str(&format!(
                "let native{i} = {} in ",
                super::godot::write(p, &args[i])
            ));
            pass.push(format!("native{i}"));
            deletes.push(format!(
                "let drop{i} = gd{}Delete native{i} in ",
                super::godot::ty(p).unwrap().name
            ));
        } else if p == "Bool" {
            pass.push(format!("(gdBoolWord {})", args[i]));
        } else {
            pass.push(args[i].clone());
        }
    }
    if params.is_empty() {
        pass.push("()".into());
    }
    body.push_str(&format!("let nativeResult = {raw} {} in ", pass.join(" ")));
    if super::godot::needs_copy(result) {
        body.push_str(&format!(
            "let result = {} in let dropResult = gd{}Delete nativeResult in ",
            super::godot::read(result, "nativeResult"),
            super::godot::ty(result).unwrap().name
        ));
    } else if result == "Bool" {
        body.push_str("let result = gdWordBool nativeResult in ");
    } else {
        body.push_str("let result = nativeResult in ");
    }
    for delete in deletes {
        body.push_str(&delete);
    }
    body.push_str("result");
    g.api.push_str(&format!(
        "  {name} :: {}\n  {name} {} = {body}\n",
        signature(
            &logical,
            &lr,
            &["Foreign Cxx".into(), "IO".into(), "Diverge".into()]
        ),
        args.join(" ")
    ));
    Ok(())
}
fn ffi_arguments(params: &[String], godot: bool) -> Result<Vec<String>> {
    params
        .iter()
        .enumerate()
        .map(|(i, t)| {
            Ok(format!(
                "{} a{i}",
                c_type(&if godot {
                    super::godot::low(t, false)
                } else {
                    t.clone()
                })?
            ))
        })
        .collect()
}
fn cpp_arg(ty: &str, arg: &str, godot: bool) -> String {
    if godot {
        if let Some(t) = super::godot::ty(ty) {
            return format!("(*static_cast<{}*>({arg}))", t.cpp);
        }
        if ty == "Bool" {
            return format!("({arg}!=0)");
        }
        if ty == "Object" {
            return format!("static_cast<godot::Object*>({arg})");
        }
    }
    arg.into()
}
fn cpp_return(ty: &str, expr: &str, godot: bool) -> String {
    if godot {
        if let Some(t) = super::godot::ty(ty) {
            return format!("return new {}({expr});", t.cpp);
        }
        if ty == "Bool" {
            return format!("return ({expr})?1:0;");
        }
    }
    if ty == "Unit" {
        format!("{expr}; return;")
    } else {
        format!("return {expr};")
    }
}
fn cpp_callback(
    symbol: &str,
    initial: &[&str],
    params: &[String],
    result: &str,
    godot: bool,
) -> String {
    let mut body = String::new();
    let mut pass = initial.iter().map(|p| p.to_string()).collect::<Vec<_>>();
    for (i, p) in params.iter().enumerate() {
        if godot {
            if let Some(t) = super::godot::ty(p) {
                body.push_str(&format!("{} native{i}=a{i}; ", t.cpp));
                pass.push(format!("&native{i}"));
                continue;
            }
            if p == "Bool" {
                pass.push(format!("(a{i}?1:0)"));
                continue;
            }
        }
        pass.push(format!("a{i}"));
    }
    let call = format!("{symbol}({})", pass.join(", "));
    if godot {
        if let Some(t) = super::godot::ty(result) {
            body.push_str(&format!(
                "std::unique_ptr<{}> result(static_cast<{}*>({call})); return *result;",
                t.cpp, t.cpp
            ));
            return body;
        }
        if result == "Bool" {
            body.push_str(&format!("return {call}!=0;"));
            return body;
        }
        if result == "Object" {
            body.push_str(&format!("return static_cast<godot::Object*>({call});"));
            return body;
        }
    }
    body.push_str(&if result == "Unit" {
        format!("{call}; return;")
    } else {
        format!("return {call};")
    });
    body
}

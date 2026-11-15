//! AOT Godot script descriptors and checked Variant dispatch.
use super::{exports, fail, godot, Bindings};
use crate::Result;
use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

pub fn load_sources(b: &mut Bindings, root: &Path) -> Result<()> {
    let root = if root.as_os_str().is_empty() {
        Path::new(".")
    } else {
        root
    };
    let root = root.canonicalize().map_err(|e| fail(e.to_string()))?;
    let mut paths = BTreeSet::new();
    let mut files = BTreeSet::new();
    for c in &b.export_classes {
        if let Some(path) = &c.script {
            if b.godot.is_none() {
                return Err(fail("script requires Godot output"));
            }
            if path.contains('\\')
                || path.contains(':')
                || path.chars().any(char::is_control)
                || path
                    .split('/')
                    .any(|p| p.is_empty() || p == "." || p == "..")
                || Path::new(path).extension().and_then(|x| x.to_str()) != Some("cinch")
                || !Path::new(path)
                    .components()
                    .all(|p| matches!(p, Component::Normal(_)))
                || !paths.insert(path.clone())
            {
                return Err(fail(
                    "each script must have a unique project-relative .cinch path without ..",
                ));
            }
            let source = root
                .join(path)
                .canonicalize()
                .map_err(|e| fail(format!("{path}: {e}")))?;
            if !files.insert(source.clone()) {
                return Err(fail("each script class needs a distinct source file"));
            }
            if !source.starts_with(&root) {
                return Err(fail("script must be inside the Godot project"));
            }
            b.script_sources.insert(
                path.clone(),
                std::fs::read_to_string(source).map_err(|e| fail(e.to_string()))?,
            );
            if !b.sources.contains(path) {
                b.sources.push(path.clone());
            }
        }
    }
    if b.godot.is_some() {
        for path in &b.sources {
            if !b.script_sources.contains_key(path) {
                b.script_sources.insert(
                    path.clone(),
                    std::fs::read_to_string(root.join(path))
                        .map_err(|e| fail(format!("{path}: {e}")))?,
                );
            }
        }
    }
    Ok(())
}
fn string(s: &str) -> String {
    format!("String::utf8({}, {})", godot::literal(s), s.len())
}
fn kind(s: &str) -> Result<String> {
    Ok(format!("Variant::{}", godot::variant_kind(s)?))
}
fn convert(s: &str, expr: &str) -> Result<String> {
    Ok(if s == "Variant" {
        expr.into()
    } else if s == "Object" {
        format!("static_cast<Object*>({expr})")
    } else if s.starts_with('I') || s.starts_with('U') {
        format!(
            "static_cast<{}>(static_cast<int64_t>({expr}))",
            exports::c_type(s)?
        )
    } else if s.starts_with('F') {
        format!(
            "static_cast<{}>(static_cast<double>({expr}))",
            exports::c_type(s)?
        )
    } else {
        format!("static_cast<{}>({expr})", exports::c_type(s)?)
    })
}
fn check(s: &str, expr: &str) -> Result<String> {
    let mut cond = format!("compatible({expr},{})", kind(s)?);
    if s.starts_with('I') || s.starts_with('U') {
        let ct = exports::c_type(s)?;
        cond = format!("({expr}).get_type()==Variant::INT");
        // Godot integers are signed 64 bit, including numeric properties in the inspector.
        if s.starts_with('U') {
            cond.push_str(&format!(" && static_cast<int64_t>({expr})>=0"));
        }
        if s != "I64" && s != "U64" {
            cond.push_str(&format!(" && static_cast<int64_t>({expr})>=std::numeric_limits<{ct}>::min() && static_cast<int64_t>({expr})<=std::numeric_limits<{ct}>::max()"));
        }
    }
    if s == "F32" {
        cond.push_str(&format!(" && (!std::isfinite(static_cast<double>({expr})) || std::abs(static_cast<double>({expr}))<=std::numeric_limits<float>::max())"));
    }
    Ok(cond)
}
pub fn generate(b: &Bindings) -> Result<String> {
    let mut tables = String::from("static void make_templates(){\n");
    for (path, source) in &b.script_sources {
        // External implementation modules cannot be opened as project resources.
        if Path::new(path)
            .components()
            .all(|p| matches!(p, Component::Normal(_) | Component::CurDir))
        {
            let normalized = Path::new(path)
                .components()
                .filter_map(|p| {
                    if let Component::Normal(p) = p {
                        p.to_str()
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("/");
            tables.push_str(&format!(
                "compiled_sources.emplace_back({}, {});\n",
                string(&format!("res://{normalized}")),
                string(source)
            ));
        }
    }
    let mut dispatch = String::new();
    for (i, c) in b
        .export_classes
        .iter()
        .filter(|c| c.script.is_some())
        .enumerate()
    {
        let path = c.script.as_ref().unwrap();
        let source = b.script_sources.get(path).ok_or_else(|| {
            fail("script source snapshots must be loaded before generating Godot scripts")
        })?;
        dispatch.push_str(&format!("static bool get_{i}(void* p,const StringName& n,Variant& out){{auto* s=static_cast<{}*>(p);\n",c.cpp));
        for f in &c.fields {
            dispatch.push_str(&format!(
                "if(n=={}){{out=s->get_{}();return true;}}\n",
                string(&f.name),
                f.name
            ));
        }
        dispatch.push_str("return false;}\n");
        dispatch.push_str(&format!("static bool set_{i}(void* p,const StringName& n,const Variant& v){{auto* s=static_cast<{}*>(p);\n",c.cpp));
        for f in &c.fields {
            dispatch.push_str(&format!(
                "if(n=={}){{if(!({}))return false;s->set_{}({});return true;}}\n",
                string(&f.name),
                check(&f.ty, "v")?,
                f.name,
                convert(&f.ty, "v")?
            ));
        }
        dispatch.push_str("return false;}\n");
        dispatch.push_str(&format!("static Variant call_{i}(void* p,const StringName& n,const Variant** args,int64_t count,GDExtensionCallError& e){{auto* s=static_cast<{}*>(p);\n",c.cpp));
        for m in &c.methods {
            dispatch.push_str(&format!("if(n=={}){{if(count!={}){{error(e,count<{}?GDEXTENSION_CALL_ERROR_TOO_FEW_ARGUMENTS:GDEXTENSION_CALL_ERROR_TOO_MANY_ARGUMENTS,0,{});return Variant();}}\n",string(&m.name),m.params.len(),m.params.len(),m.params.len()));
            for (j, p) in m.params.iter().enumerate() {
                dispatch.push_str(&format!("if(!({})){{error(e,GDEXTENSION_CALL_ERROR_INVALID_ARGUMENT,{j},{});return Variant();}}\n",check(p,&format!("*args[{j}]"))?,kind(p)?));
            }
            let args = m
                .params
                .iter()
                .enumerate()
                .map(|(j, p)| convert(p, &format!("*args[{j}]")))
                .collect::<Result<Vec<_>>>()?
                .join(", ");
            let call = format!("s->{}({args})", m.name);
            if m.result == "Unit" {
                dispatch.push_str(&format!("{call};return Variant();}}\n"));
            } else {
                dispatch.push_str(&format!("return Variant({call});}}\n"));
            }
        }
        dispatch.push_str("error(e,GDEXTENSION_CALL_ERROR_INVALID_METHOD);return Variant();}\n");
        tables.push_str(&format!("{{auto t=std::make_shared<Template>();t->path={};t->name={};t->base={};t->source={};t->tool={};\n",string(&format!("res://{path}")),string(&c.name),string(c.base.as_ref().unwrap().rsplit("::").next().unwrap()),string(source),c.tool));
        for f in &c.fields {
            let initial = if godot::rich(&f.ty) {
                godot::initial(&f.ty, &f.initial)?
            } else {
                exports::field_initial(f)?
            };
            tables.push_str(&format!(
                "t->fields.push_back(Field{{{}, {}, Variant(static_cast<{}>({initial})), {}}});\n",
                string(&f.name),
                kind(&f.ty)?,
                exports::c_type(&f.ty)?,
                f.ty == "Variant"
            ));
        }
        for m in &c.methods {
            let name = m.function.rsplit('.').next().unwrap();
            let line = source
                .lines()
                .position(|l| l.trim_start().starts_with(&format!("{name} ::")))
                .map(|x| x + 1)
                .unwrap_or(1);
            tables.push_str(&format!(
                "t->methods.push_back(Method{{{},{{{}}},{},{line},{}}});\n",
                string(&m.name),
                m.params
                    .iter()
                    .map(|p| kind(p))
                    .collect::<Result<Vec<_>>>()?
                    .join(","),
                kind(&m.result)?,
                m.result == "Variant"
            ));
        }
        for s in &c.signals {
            tables.push_str(&format!(
                "t->signals.push_back(Signal{{{},{{{}}}}});\n",
                string(&s.name),
                s.params
                    .iter()
                    .map(|p| kind(p))
                    .collect::<Result<Vec<_>>>()?
                    .join(",")
            ));
        }
        tables.push_str(&format!("t->create=[](Object* o)->void*{{return new {}(o);}};t->destroy=[](void* p){{delete static_cast<{}*>(p);}};t->get=get_{i};t->set=set_{i};t->call=call_{i};templates.push_back(t);}}\n",c.cpp,c.cpp));
    }
    tables.push_str("}\n");
    Ok(include_str!("script_runtime.hpp").replace("// CINCH_SCRIPT_TABLES", &(dispatch + &tables)))
}

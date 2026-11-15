//! Explicit C++ binding generation. C++ owns layout, overload resolution and
//! destruction; Cinch sees only fixed-width scalars and linear opaque pointers.
use crate::{Error, Result};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bindings {
    pub module: String,
    pub header: String,
    #[serde(default)]
    pub functions: Vec<Function>,
    #[serde(default)]
    pub classes: Vec<Class>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Function {
    pub name: String,
    pub cpp: String,
    #[serde(default)]
    pub params: Vec<String>,
    pub result: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Class {
    pub name: String,
    pub cpp: String,
    pub constructor: String,
    pub destructor: String,
    #[serde(default)]
    pub params: Vec<String>,
    #[serde(default)]
    pub methods: Vec<Function>,
}

fn fail(message: impl Into<String>) -> Error {
    Error::plain(format!("C++ bindings: {}", message.into()))
}

fn identifier(s: &str, upper: bool) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| {
        if upper {
            c.is_ascii_uppercase()
        } else {
            c.is_ascii_lowercase()
        }
    }) && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        && crate::lexer::lex(s).ok().is_some_and(|tokens| {
            matches!(
                tokens.first().map(|t| &t.tok),
                Some(crate::lexer::Tok::VarId(_) | crate::lexer::Tok::ConId(_))
            )
        })
}

// No arbitrary C++ expressions: templates can be given a qualified alias in
// the included header. This also keeps generated includes and symbols literal.
fn cpp_name(s: &str) -> bool {
    !s.is_empty()
        && s.split("::").all(|p| {
            let mut cs = p.chars();
            cs.next()
                .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                && cs.all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
}

fn scalar(s: &str) -> Result<&'static str> {
    Ok(match s {
        "Unit" => "void",
        "I8" => "std::int8_t",
        "U8" => "std::uint8_t",
        "I16" => "std::int16_t",
        "U16" => "std::uint16_t",
        "I32" => "std::int32_t",
        "U32" => "std::uint32_t",
        "I64" => "std::int64_t",
        "U64" => "std::uint64_t",
        "F32" => "float",
        "F64" => "double",
        _ => {
            return Err(fail(format!(
                "unsupported boundary type `{s}`; use a fixed-width scalar"
            )))
        }
    })
}

fn arguments(params: &[String]) -> Result<(Vec<String>, Vec<String>)> {
    let mut declarations = Vec::new();
    let mut names = Vec::new();
    for (i, ty) in params.iter().enumerate() {
        if ty == "Unit" {
            return Err(fail(
                "Unit parameters must be elided (use an empty params array)",
            ));
        }
        declarations.push(format!("{} a{i}", scalar(ty)?));
        names.push(format!("a{i}"));
    }
    Ok((declarations, names))
}

/// Generate a checked-surface module and a C++17 translation unit. Every
/// wrapper catches all C++ exceptions and terminates: no unwinder can cross
/// the C ABI. This is a trusted native boundary, not a verification claim.
pub fn generate(bindings: &Bindings) -> Result<(String, String)> {
    if !bindings.module.split('.').all(|s| identifier(s, true)) {
        return Err(fail("module must be a qualified Cinch module name"));
    }
    if bindings.header.is_empty()
        || bindings
            .header
            .chars()
            .any(|c| c.is_control() || c == '"' || c == '\\')
    {
        return Err(fail("invalid header path"));
    }
    if bindings.functions.is_empty() && bindings.classes.is_empty() {
        return Err(fail("at least one function or class is required"));
    }
    let prefix = format!("cinch_cxx_{}", bindings.module.replace('.', "_"));
    let mut names = BTreeSet::new();
    let mut register = |name: &str, upper: bool| -> Result<()> {
        if !identifier(name, upper) || !names.insert(name.to_string()) {
            return Err(fail(format!("invalid or duplicate Cinch name `{name}`")));
        }
        Ok(())
    };
    for f in &bindings.functions {
        register(&f.name, false)?;
    }
    for c in &bindings.classes {
        if scalar(&c.name).is_ok()
            || [
                "Owned",
                "Borrowed",
                "RawPtr",
                "Retained",
                "Out",
                "MaybeNull",
                "Int",
                "Nat",
                "Bool",
                "Type",
            ]
            .contains(&c.name.as_str())
        {
            return Err(fail(format!(
                "class name `{}` shadows a boundary type",
                c.name
            )));
        }
        register(&c.name, true)?;
        register(&c.constructor, false)?;
        register(&c.destructor, false)?;
        for m in &c.methods {
            register(&m.name, false)?;
        }
    }
    let mut cinch = format!("-- Generated C++ bindings; trusted native calls, exceptions terminate.\nmodule {} where\n  export {}\n", bindings.module, names.into_iter().collect::<Vec<_>>().join(", "));
    let mut cpp = format!("// Generated by cinch cxx. Do not let exceptions cross the C ABI.\n#include <cstdint>\n#include <exception>\n#include \"{}\"\n", bindings.header);
    let declare = |out: &mut String, name: &str, params: &[String], result: &str| {
        let args = if params.is_empty() {
            "Unit".to_string()
        } else {
            params.join(" -> ")
        };
        out.push_str(&format!("  foreign import ccall \"{prefix}_{name}\" {name} :: {args} ->{{Foreign Cxx, IO, Diverge}} {result}\n"));
    };
    let wrapper = |out: &mut String, name: &str, result: &str, params: &[String], expr: &str| {
        out.push_str(&format!("extern \"C\" {result} {prefix}_{name}({}) noexcept {{\n  try {{ {expr}; }} catch (...) {{ std::terminate(); }}\n}}\n", params.join(", ")));
    };
    for f in &bindings.functions {
        if !cpp_name(&f.cpp) {
            return Err(fail("invalid qualified C++ function name"));
        }
        let (args, values) = arguments(&f.params)?;
        wrapper(
            &mut cpp,
            &f.name,
            scalar(&f.result)?,
            &args,
            &format!("return {}({})", f.cpp, values.join(", ")),
        );
        declare(&mut cinch, &f.name, &f.params, &f.result);
    }
    for c in &bindings.classes {
        if !cpp_name(&c.cpp) {
            return Err(fail("invalid qualified C++ class name"));
        }
        cinch.push_str(&format!("  opaque type {} :: Type\n", c.name));
        let owned = format!(
            "Owned {}{}Heap {}",
            bindings.module.replace('.', ""),
            c.name,
            c.name
        );
        let (args, values) = arguments(&c.params)?;
        wrapper(
            &mut cpp,
            &c.constructor,
            "void*",
            &args,
            &format!("return new {}({})", c.cpp, values.join(", ")),
        );
        declare(&mut cinch, &c.constructor, &c.params, &owned);
        wrapper(
            &mut cpp,
            &c.destructor,
            "void",
            &["void* self".into()],
            &format!("delete static_cast<{}*>(self)", c.cpp),
        );
        declare(&mut cinch, &c.destructor, &[owned], "Unit");
        for m in &c.methods {
            // A method is a single member, never an expression or namespace.
            if !cpp_name(&m.cpp) || m.cpp.contains("::") {
                return Err(fail("invalid C++ member name"));
            }
            let (mut args, values) = arguments(&m.params)?;
            args.insert(0, "void* self".into());
            wrapper(
                &mut cpp,
                &m.name,
                scalar(&m.result)?,
                &args,
                &format!(
                    "return static_cast<{}*>(self)->{}({})",
                    c.cpp,
                    m.cpp,
                    values.join(", ")
                ),
            );
            let mut params = m.params.clone();
            params.insert(0, format!("Borrowed {}", c.name));
            declare(&mut cinch, &m.name, &params, &m.result);
        }
    }
    Ok((cinch, cpp))
}

pub fn cli(args: &[String]) -> i32 {
    let run = || -> Result<()> {
        if args.len() != 3 || args[1] != "--out-dir" {
            return Err(fail("usage: cinch cxx bindings.json --out-dir DIR"));
        }
        let input = std::fs::read_to_string(&args[0]).map_err(|e| fail(e.to_string()))?;
        let spec: Bindings = serde_json::from_str(&input).map_err(|e| fail(e.to_string()))?;
        let (cinch, cpp) = generate(&spec)?;
        let dir = Path::new(&args[2]);
        // Qualified module names map to the same module tree as authored Cinch.
        let module_path = dir
            .join(spec.module.replace('.', "/"))
            .with_extension("cinch");
        std::fs::create_dir_all(module_path.parent().unwrap()).map_err(|e| fail(e.to_string()))?;
        std::fs::write(module_path, cinch).map_err(|e| fail(e.to_string()))?;
        std::fs::write(
            dir.join(format!("{}.cpp", spec.module.replace('.', "_"))),
            cpp,
        )
        .map_err(|e| fail(e.to_string()))?;
        Ok(())
    };
    match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            2
        }
    }
}

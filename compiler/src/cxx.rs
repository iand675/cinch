//! Native binding generation: exact linker symbols use the existing FFI.
//! Optional C++ shims handle scalar APIs, methods and class allocation.
use crate::{Error, Result};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bindings {
    pub module: String,
    #[serde(default)]
    pub header: String,
    #[serde(default)]
    pub imports: Vec<DirectImport>,
    #[serde(default)]
    pub opaque_types: Vec<String>,
    #[serde(default)]
    pub functions: Vec<Function>,
    #[serde(default)]
    pub classes: Vec<Class>,
}

/// An exact linker symbol and the full signature supported by native FFI.
/// No C++ code is generated for these entries.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DirectImport {
    pub name: String,
    pub symbol: String,
    pub signature: String,
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

fn validate_opaque_name(name: &str) -> Result<()> {
    if scalar(name).is_ok()
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
        .contains(&name)
    {
        return Err(fail(format!(
            "opaque name `{name}` shadows a boundary type"
        )));
    }
    Ok(())
}

fn direct_declaration(import: &DirectImport) -> Result<String> {
    if import.symbol.is_empty() || import.symbol.chars().any(char::is_control) {
        return Err(fail(
            "a direct import needs a nonempty linker symbol without control characters",
        ));
    }
    // The lexer uses the same quoted-string escapes as JSON. Keep symbol
    // identity literal, including decorated C++ names, quotes and backslashes.
    let symbol = serde_json::to_string(&import.symbol).map_err(|e| fail(e.to_string()))?;
    let decl = format!(
        "  foreign import ccall {symbol} {} :: {}\n",
        import.name, import.signature
    );
    let snippet = format!("module Direct where\n{decl}");
    let tokens = crate::layout::layout(crate::lexer::lex(&snippet)?)?;
    let parsed = crate::parser::parse_module(&tokens, &crate::parser::FixityTable::default())?;
    // Parse the full native signature, but never allow it to introduce more
    // source declarations. ABI admissibility stays with the ordinary checker.
    match parsed.module.decls.as_slice() {
        [crate::ast::Decl::Foreign(f)]
            if f.name.text == import.name && f.symbol == import.symbol =>
        {
            Ok(decl)
        }
        _ => Err(fail(
            "a direct import signature must contain exactly one foreign declaration",
        )),
    }
}

/// Generate a native module and, when requested, a C++17 shim. Direct imports
/// preserve exact linker identities and native signatures. Only shim calls
/// catch exceptions; direct imports require a no-unwind contract.
pub fn generate(bindings: &Bindings) -> Result<(String, String)> {
    if !bindings.module.split('.').all(|s| identifier(s, true)) {
        return Err(fail("module must be a qualified Cinch module name"));
    }
    let needs_cpp = !bindings.functions.is_empty() || !bindings.classes.is_empty();
    if (needs_cpp && bindings.header.is_empty())
        || bindings
            .header
            .chars()
            .any(|c| c.is_control() || c == '"' || c == '\\')
    {
        return Err(fail("invalid header path"));
    }
    if !needs_cpp && bindings.imports.is_empty() {
        return Err(fail("at least one import, function or class is required"));
    }
    let prefix = format!("cinch_cxx_{}", bindings.module.replace('.', "_"));
    let mut names = BTreeSet::new();
    let mut register = |name: &str, upper: bool| -> Result<()> {
        if !identifier(name, upper) || !names.insert(name.to_string()) {
            return Err(fail(format!("invalid or duplicate Cinch name `{name}`")));
        }
        Ok(())
    };
    for import in &bindings.imports {
        register(&import.name, false)?;
    }
    for name in &bindings.opaque_types {
        validate_opaque_name(name)?;
        register(name, true)?;
    }
    for f in &bindings.functions {
        register(&f.name, false)?;
    }
    for c in &bindings.classes {
        validate_opaque_name(&c.name)?;
        register(&c.name, true)?;
        register(&c.constructor, false)?;
        register(&c.destructor, false)?;
        for m in &c.methods {
            register(&m.name, false)?;
        }
    }
    let mut cinch = format!("-- Generated native bindings; direct imports use the declared ABI contract.\nmodule {} where\n  export {}\n", bindings.module, names.into_iter().collect::<Vec<_>>().join(", "));
    let mut cpp = format!("// Generated by cinch cxx. Do not let exceptions cross the C ABI.\n#include <cstdint>\n#include <exception>\n#include \"{}\"\n", bindings.header);
    if !needs_cpp {
        cpp.clear();
    }
    for name in &bindings.opaque_types {
        cinch.push_str(&format!("  opaque type {name} :: Type\n"));
    }
    for import in &bindings.imports {
        cinch.push_str(&direct_declaration(import)?);
    }
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
        if !cpp.is_empty() {
            std::fs::write(
                dir.join(format!("{}.cpp", spec.module.replace('.', "_"))),
                cpp,
            )
            .map_err(|e| fail(e.to_string()))?;
        }
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

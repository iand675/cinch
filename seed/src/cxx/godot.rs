//! Godot values cross the C ABI through scoped borrows and linear owners.
use super::{direct_declaration, fail, Bindings, DirectImport};
use crate::Result;

#[derive(Clone, Copy)]
pub struct Type {
    pub name: &'static str,
    pub cpp: &'static str,
    pub handle: &'static str,
    pub record: bool,
    pub kind: &'static str,
}
pub const TYPES: &[Type] = &[
    Type {
        name: "String",
        cpp: "godot::String",
        handle: "GDString",
        record: false,
        kind: "STRING",
    },
    Type {
        name: "StringName",
        cpp: "godot::StringName",
        handle: "GDStringName",
        record: false,
        kind: "STRING_NAME",
    },
    Type {
        name: "Vector2",
        cpp: "godot::Vector2",
        handle: "GDVector2Handle",
        record: true,
        kind: "VECTOR2",
    },
    Type {
        name: "Vector3",
        cpp: "godot::Vector3",
        handle: "GDVector3Handle",
        record: true,
        kind: "VECTOR3",
    },
    Type {
        name: "Vector4",
        cpp: "godot::Vector4",
        handle: "GDVector4Handle",
        record: true,
        kind: "VECTOR4",
    },
    Type {
        name: "Vector2i",
        cpp: "godot::Vector2i",
        handle: "GDVector2iHandle",
        record: true,
        kind: "VECTOR2I",
    },
    Type {
        name: "Vector3i",
        cpp: "godot::Vector3i",
        handle: "GDVector3iHandle",
        record: true,
        kind: "VECTOR3I",
    },
    Type {
        name: "Vector4i",
        cpp: "godot::Vector4i",
        handle: "GDVector4iHandle",
        record: true,
        kind: "VECTOR4I",
    },
    Type {
        name: "Color",
        cpp: "godot::Color",
        handle: "GDColorHandle",
        record: true,
        kind: "COLOR",
    },
    Type {
        name: "Variant",
        cpp: "godot::Variant",
        handle: "GDVariant",
        record: false,
        kind: "NIL",
    },
    Type {
        name: "Array",
        cpp: "godot::Array",
        handle: "GDArray",
        record: false,
        kind: "ARRAY",
    },
    Type {
        name: "Dictionary",
        cpp: "godot::Dictionary",
        handle: "GDDictionary",
        record: false,
        kind: "DICTIONARY",
    },
];
pub fn ty(s: &str) -> Option<Type> {
    TYPES.iter().find(|t| t.name == s).copied()
}
pub fn rich(s: &str) -> bool {
    ty(s).is_some() || s == "Bool" || s == "Object"
}
pub fn low(s: &str, result: bool) -> String {
    if let Some(t) = ty(s) {
        if result {
            format!("Owned GDHeap {}", t.handle)
        } else {
            format!("Borrowed {}", t.handle)
        }
    } else if s == "Bool" {
        "I32".into()
    } else if s == "Object" {
        "MaybeNull (RawPtr GDObject)".into()
    } else {
        s.into()
    }
}
pub fn logical(s: &str, result: bool) -> String {
    if let Some(t) = ty(s) {
        if t.record {
            format!("GD{}", t.name)
        } else if s == "String" || s == "StringName" {
            "String".into()
        } else {
            low(s, result)
        }
    } else if s == "Object" {
        low(s, result)
    } else {
        s.into()
    }
}
pub fn read(s: &str, value: &str) -> String {
    match ty(s) {
        Some(t) if t.record || s == "String" || s == "StringName" => {
            format!("gdRead{} {value}", t.name)
        }
        _ if s == "Bool" => format!("gdWordBool {value}"),
        _ => value.into(),
    }
}
pub fn write(s: &str, value: &str) -> String {
    match ty(s) {
        Some(t) if t.record || s == "String" || s == "StringName" => {
            format!("gdWrite{} {value}", t.name)
        }
        _ if s == "Bool" => format!("gdBoolWord {value}"),
        _ => value.into(),
    }
}
pub fn needs_copy(s: &str) -> bool {
    ty(s).is_some_and(|t| t.record || s == "String" || s == "StringName")
}
pub fn variant_kind(s: &str) -> Result<&'static str> {
    if let Some(t) = ty(s) {
        Ok(t.kind)
    } else {
        Ok(match s {
            "Bool" => "BOOL",
            "Object" => "OBJECT",
            "F32" | "F64" => "FLOAT",
            "Unit" => "NIL",
            "I8" | "I16" | "I32" | "I64" | "U8" | "U16" | "U32" | "U64" => "INT",
            _ => return Err(fail(format!("{s} has no Godot Variant conversion"))),
        })
    }
}
pub fn components(t: Type) -> Vec<&'static str> {
    if t.name == "Color" {
        vec!["r", "g", "b", "a"]
    } else {
        let n = if t.name.contains('2') {
            2
        } else if t.name.contains('3') {
            3
        } else {
            4
        };
        vec!["x", "y", "z", "w"][..n].to_vec()
    }
}
pub fn literal(s: &str) -> String {
    format!(
        "\"{}\"",
        s.as_bytes()
            .iter()
            .map(|b| format!("\\x{b:02x}"))
            .collect::<String>()
    )
}
pub fn initial(ty_name: &str, value: &serde_json::Value) -> Result<String> {
    if ty_name == "Bool" {
        return if value.is_null() {
            Ok("false".into())
        } else {
            value
                .as_bool()
                .map(|b| b.to_string())
                .ok_or_else(|| fail("Bool initializer must be boolean"))
        };
    }
    if ty_name == "Object" {
        return if value.is_null() {
            Ok("nullptr".into())
        } else {
            Err(fail("Object initializers must be null"))
        };
    }
    let t = ty(ty_name).ok_or_else(|| fail("unknown Godot field type"))?;
    if t.record {
        let fields = components(t);
        let values = if value.is_null() {
            vec![serde_json::json!(0); fields.len()]
        } else {
            value
                .as_array()
                .filter(|v| v.len() == fields.len())
                .cloned()
                .ok_or_else(|| fail("vector initializer must have exactly its component count"))?
        };
        let ints = t.name.ends_with('i');
        let mut args = Vec::new();
        for v in values {
            if ints {
                let n = v
                    .as_i64()
                    .filter(|n| i32::try_from(*n).is_ok())
                    .ok_or_else(|| fail("integer vector components must fit I32"))?;
                args.push(n.to_string());
            } else {
                let n = v
                    .as_f64()
                    .filter(|n| n.is_finite() && (*n as f32).is_finite())
                    .ok_or_else(|| fail("vector components must be finite real_t values"))?;
                args.push(format!("{n:e}"));
            }
        }
        return Ok(format!("{}({})", t.cpp, args.join(", ")));
    }
    if ty_name == "String" || ty_name == "StringName" {
        let s = if value.is_null() {
            ""
        } else {
            value
                .as_str()
                .ok_or_else(|| fail("string initializer must be text"))?
        };
        let expr = format!("godot::String::utf8({}, {})", literal(s), s.len());
        return Ok(if ty_name == "StringName" {
            format!("godot::StringName({expr})")
        } else {
            expr
        });
    }
    let expr = json_variant(value)?;
    Ok(match ty_name {
        "Variant" => expr,
        "Array" if value.is_null() => "godot::Array()".into(),
        "Dictionary" if value.is_null() => "godot::Dictionary()".into(),
        "Array" if value.is_array() => format!("godot::Array({expr})"),
        "Dictionary" if value.is_object() => format!("godot::Dictionary({expr})"),
        _ => return Err(fail("collection initializer has the wrong JSON shape")),
    })
}
fn json_variant(v: &serde_json::Value) -> Result<String> {
    Ok(match v {
        serde_json::Value::Null => "godot::Variant()".into(),
        serde_json::Value::Bool(b) => format!("godot::Variant({b})"),
        serde_json::Value::Number(n) => {
            if let Some(n) = n.as_i64() {
                if n == i64::MIN {
                    "godot::Variant(INT64_MIN)".into()
                } else {
                    format!("godot::Variant(int64_t({n}LL))")
                }
            } else if n.as_u64().is_some() {
                return Err(fail("Godot integer initializers must fit signed I64"));
            } else if let Some(f) = n.as_f64().filter(|f| f.is_finite()) {
                format!("godot::Variant(double({f:e}))")
            } else {
                return Err(fail("invalid Variant number"));
            }
        }
        serde_json::Value::String(s) => format!(
            "godot::Variant(godot::String::utf8({}, {}))",
            literal(s),
            s.len()
        ),
        serde_json::Value::Array(a) => {
            let mut x = "([](){ godot::Array a;".to_string();
            for v in a {
                x.push_str(&format!("a.append({});", json_variant(v)?));
            }
            x.push_str("return godot::Variant(a); }())");
            x
        }
        serde_json::Value::Object(d) => {
            let mut x = "([](){ godot::Dictionary d;".to_string();
            for (k, v) in d {
                x.push_str(&format!(
                    "d[{}] = {};",
                    json_variant(&serde_json::json!(k))?,
                    json_variant(v)?
                ));
            }
            x.push_str("return godot::Variant(d); }())");
            x
        }
    })
}
#[derive(Default)]
pub struct Bridge {
    pub api: String,
    pub cpp: String,
    pub names: Vec<(String, bool)>,
}
impl Bridge {
    fn foreign(
        &mut self,
        prefix: &str,
        name: &str,
        params: &[&str],
        result: &str,
        body: &str,
        cparams: &str,
        cret: &str,
    ) -> Result<()> {
        self.names.push((name.into(), false));
        let sig = if params.is_empty() {
            "Unit".into()
        } else {
            params.join(" -> ")
        };
        self.api.push_str(&direct_declaration(&DirectImport {
            name: name.into(),
            symbol: format!("{prefix}_{name}"),
            signature: format!("{sig} ->{{Foreign Cxx, IO}} {result}"),
        })?);
        self.cpp.push_str(&format!(
            "extern \"C\" {cret} {prefix}_{name}({cparams}) noexcept {{ {body} }}\n"
        ));
        Ok(())
    }
    fn function(&mut self, name: &str, params: &[&str], result: &str, body: &str) {
        self.names.push((name.into(), false));
        let sig = if params.is_empty() {
            "Unit".into()
        } else {
            params.join(" -> ")
        };
        self.api.push_str(&format!(
            "  {name} :: {sig} ->{{Foreign Cxx, IO, Diverge}} {result}\n  {body}\n"
        ));
    }
}
pub fn bridge(b: &Bindings) -> Result<Bridge> {
    let mut g = Bridge::default();
    if b.godot.is_none() {
        return Ok(g);
    }
    let prefix = format!("cinch_gd_{}", b.module.replace('.', "_"));
    g.cpp=String::from("\n#include <godot_cpp/variant/variant.hpp>\n#include <godot_cpp/variant/string.hpp>\n#include <godot_cpp/variant/string_name.hpp>\n#include <godot_cpp/variant/array.hpp>\n#include <godot_cpp/variant/dictionary.hpp>\n#include <godot_cpp/variant/utility_functions.hpp>\n#include <godot_cpp/core/object.hpp>\n#include <godot_cpp/classes/object.hpp>\n#include <godot_cpp/variant/callable.hpp>\n#include <cstdint>\n#include <vector>\n#include <limits>\nextern \"C\" [[noreturn]] void cinch_panic(const unsigned char*, int64_t);\nnamespace cinch_gd_bridge {\n[[noreturn]] inline void fault() { static const unsigned char msg[]=\"Godot bridge: invalid type, index, or object\"; cinch_panic(msg,sizeof(msg)-1); }\ntemplate<class T> T& ref(void* p) { if(!p) fault(); return *static_cast<T*>(p); }\ninline void index(int64_t i,int64_t n) { if(i<0||i>=n) fault(); }\ninline godot::Variant& variant(void* p, godot::Variant::Type t) { auto& v=ref<godot::Variant>(p); if(v.get_type()!=t) fault(); return v; }\n}\n");
    g.names.push(("GDObject".into(), true));
    g.api.push_str("  opaque type GDObject :: Type\n");
    for t in TYPES {
        g.names.push((t.handle.into(), true));
        g.api
            .push_str(&format!("  opaque type {} :: Type\n", t.handle));
        let owned = format!("Owned GDHeap {}", t.handle);
        let borrowed = format!("Borrowed {}", t.handle);
        let stem = format!("gd{}", t.name);
        g.foreign(
            &prefix,
            &format!("{stem}Delete"),
            &[&owned],
            "Unit",
            &format!("delete static_cast<{}*>(p);", t.cpp),
            "void* p",
            "void",
        )?;
        g.foreign(
            &prefix,
            &format!("{stem}Copy"),
            &[&borrowed],
            &owned,
            &format!("return new {}(cinch_gd_bridge::ref<{}>(p));", t.cpp, t.cpp),
            "void* p",
            "void*",
        )?;
        if t.record {
            let comps = components(*t);
            let scalar = if t.name.ends_with('i') { "I32" } else { "F64" };
            let cscalar = if t.name.ends_with('i') {
                "int32_t"
            } else {
                "double"
            };
            let record = format!("GD{}", t.name);
            g.names.push((record.clone(), true));
            g.api.push_str(&format!("  record {record} where\n"));
            for c in &comps {
                g.api.push_str(&format!("    {c} :: {scalar}\n"));
            }
            let params = vec![scalar; comps.len()];
            let cparams = comps
                .iter()
                .map(|c| format!("{cscalar} {c}"))
                .collect::<Vec<_>>()
                .join(", ");
            g.foreign(
                &prefix,
                &format!("{stem}New"),
                &params,
                &owned,
                &format!("return new {}({});", t.cpp, comps.join(", ")),
                &cparams,
                "void*",
            )?;
            for c in &comps {
                g.foreign(
                    &prefix,
                    &format!("{stem}{}", c.to_uppercase()),
                    &[&borrowed],
                    scalar,
                    &format!("return cinch_gd_bridge::ref<{}>(p).{c};", t.cpp),
                    "void* p",
                    cscalar,
                )?;
            }
            let mut read_body = format!("gdRead{} p = ", t.name);
            for c in &comps {
                read_body.push_str(&format!("let {c} = {stem}{} p in ", c.to_uppercase()));
            }
            read_body.push_str(&format!("{record} {}", comps.join(" ")));
            g.function(
                &format!("gdRead{}", t.name),
                &[&borrowed],
                &record,
                &read_body,
            );
            g.function(
                &format!("gdWrite{}", t.name),
                &[&record],
                &owned,
                &format!(
                    "gdWrite{} p = {stem}New {}",
                    t.name,
                    comps
                        .iter()
                        .map(|c| format!("p.{c}"))
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
            );
        }
    }
    // Unicode is copied as validated scalar values, including embedded NUL.
    g.foreign(
        &prefix,
        "gdStringNew",
        &[],
        "Owned GDHeap GDString",
        "return new godot::String;",
        "",
        "void*",
    )?;
    g.foreign(
        &prefix,
        "gdStringLength",
        &["Borrowed GDString"],
        "I64",
        "return cinch_gd_bridge::ref<godot::String>(p).length();",
        "void* p",
        "int64_t",
    )?;
    g.foreign(&prefix,"gdStringChar",&["Borrowed GDString","I64"],"Char","auto& s=cinch_gd_bridge::ref<godot::String>(p); cinch_gd_bridge::index(i,s.length()); uint32_t c=s.unicode_at(i); return c>0x10ffff||(c>=0xd800&&c<=0xdfff)?0xfffd:c;","void* p,int64_t i","uint32_t")?;
    g.foreign(&prefix,"gdStringAppend",&["Borrowed GDString","Char"],"Unit","auto& s=cinch_gd_bridge::ref<godot::String>(p); if(c>0x10ffff||(c>=0xd800&&c<=0xdfff)) cinch_gd_bridge::fault(); int64_t n=s.length(); if(s.resize(n+2)!=godot::OK)cinch_gd_bridge::fault(); auto* data=s.ptrw(); data[n]=c; data[n+1]=0;","void* p,uint32_t c","void")?;
    g.function("gdReadString",&["Borrowed GDString"],"String","gdReadString p = let n = gdStringLength p in let cs = gdReadChars p 0 n in GodotString.pack cs");
    g.function("gdReadChars",&["Borrowed GDString","I64","I64"],"List Char","gdReadChars p i n = if i == n then Nil else let c = gdStringChar p i in let cs = gdReadChars p (wrapping-add-i64 i 1) n in Cons c cs");
    g.function("gdWriteString",&["String"],"Owned GDHeap GDString","gdWriteString text = let p = gdStringNew () in let u = gdWriteChars p (GodotString.unpack text) in p");
    g.function("gdWriteChars",&["Borrowed GDString","List Char"],"Unit","gdWriteChars p cs = case cs of { Nil -> (); Cons c rest -> let u = gdStringAppend p c in gdWriteChars p rest }");
    g.foreign(
        &prefix,
        "gdStringNameFromString",
        &["Borrowed GDString"],
        "Owned GDHeap GDStringName",
        "return new godot::StringName(cinch_gd_bridge::ref<godot::String>(p));",
        "void* p",
        "void*",
    )?;
    g.foreign(
        &prefix,
        "gdStringNameToString",
        &["Borrowed GDStringName"],
        "Owned GDHeap GDString",
        "return new godot::String(cinch_gd_bridge::ref<godot::StringName>(p));",
        "void* p",
        "void*",
    )?;
    g.function("gdReadStringName",&["Borrowed GDStringName"],"String","gdReadStringName p = let text = gdStringNameToString p in let s = gdReadString text in let u = gdStringDelete text in s");
    g.function("gdWriteStringName",&["String"],"Owned GDHeap GDStringName","gdWriteStringName s = let text = gdWriteString s in let p = gdStringNameFromString text in let u = gdStringDelete text in p");
    g.names
        .extend([("gdBoolWord".into(), false), ("gdWordBool".into(), false)]);
    g.api.push_str("  gdBoolWord :: Bool -> I32\n  gdBoolWord b = if b then 1 else 0\n  gdWordBool :: I32 -> Bool\n  gdWordBool n = if n == 0 then False else True\n");
    g.foreign(
        &prefix,
        "gdVariantType",
        &["Borrowed GDVariant"],
        "I32",
        "return cinch_gd_bridge::ref<godot::Variant>(p).get_type();",
        "void* p",
        "int32_t",
    )?;
    g.foreign(
        &prefix,
        "gdVariantNil",
        &[],
        "Owned GDHeap GDVariant",
        "return new godot::Variant;",
        "",
        "void*",
    )?;
    for (name, ty, ct, expr, kind) in [
        ("Int", "I64", "int64_t", "v", "INT"),
        ("Float", "F64", "double", "v", "FLOAT"),
        ("BoolWord", "I32", "int32_t", "v!=0", "BOOL"),
    ] {
        g.foreign(
            &prefix,
            &format!("gdVariant{name}"),
            &[ty],
            "Owned GDHeap GDVariant",
            &format!("return new godot::Variant({expr});"),
            &format!("{ct} v"),
            "void*",
        )?;
        g.foreign(
            &prefix,
            &format!("gdVariantAs{name}"),
            &["Borrowed GDVariant"],
            ty,
            &format!(
                "return static_cast<{}>(cinch_gd_bridge::variant(p,godot::Variant::{kind}));",
                if name == "BoolWord" { "bool" } else { ct }
            ),
            "void* p",
            ct,
        )?;
    }
    for t in TYPES.iter().filter(|t| t.name != "Variant") {
        let borrowed = format!("Borrowed {}", t.handle);
        let owned = format!("Owned GDHeap {}", t.handle);
        g.foreign(
            &prefix,
            &format!("gdVariant{}", t.name),
            &[&borrowed],
            "Owned GDHeap GDVariant",
            &format!(
                "return new godot::Variant(cinch_gd_bridge::ref<{}>(p));",
                t.cpp
            ),
            "void* p",
            "void*",
        )?;
        g.foreign(
            &prefix,
            &format!("gdVariantAs{}", t.name),
            &["Borrowed GDVariant"],
            &owned,
            &format!(
                "return new {}(static_cast<{}>(cinch_gd_bridge::variant(p,godot::Variant::{})));",
                t.cpp, t.cpp, t.kind
            ),
            "void* p",
            "void*",
        )?;
    }
    for (name, cpp, handle) in [
        ("Array", "godot::Array", "GDArray"),
        ("Dictionary", "godot::Dictionary", "GDDictionary"),
    ] {
        let borrowed = format!("Borrowed {handle}");
        let owned = format!("Owned GDHeap {handle}");
        g.foreign(
            &prefix,
            &format!("gd{name}New"),
            &[],
            &owned,
            &format!("return new {cpp};"),
            "",
            "void*",
        )?;
        g.foreign(
            &prefix,
            &format!("gd{name}Size"),
            &[&borrowed],
            "I64",
            &format!("return cinch_gd_bridge::ref<{cpp}>(p).size();"),
            "void* p",
            "int64_t",
        )?;
        g.foreign(
            &prefix,
            &format!("gd{name}Clear"),
            &[&borrowed],
            "Unit",
            &format!("cinch_gd_bridge::ref<{cpp}>(p).clear();"),
            "void* p",
            "void",
        )?;
        g.foreign(
            &prefix,
            &format!("gd{name}Duplicate"),
            &[&borrowed],
            &owned,
            &format!("return new {cpp}(cinch_gd_bridge::ref<{cpp}>(p).duplicate(true));"),
            "void* p",
            "void*",
        )?;
    }
    g.foreign(&prefix,"gdArrayGet",&["Borrowed GDArray","I64"],"Owned GDHeap GDVariant","auto& a=cinch_gd_bridge::ref<godot::Array>(p);cinch_gd_bridge::index(i,a.size());return new godot::Variant(a[i]);","void* p,int64_t i","void*")?;
    g.foreign(&prefix,"gdArraySet",&["Borrowed GDArray","I64","Borrowed GDVariant"],"Unit","auto& a=cinch_gd_bridge::ref<godot::Array>(p);cinch_gd_bridge::index(i,a.size());a[i]=cinch_gd_bridge::ref<godot::Variant>(v);","void* p,int64_t i,void* v","void")?;
    g.foreign(
        &prefix,
        "gdArrayAppend",
        &["Borrowed GDArray", "Borrowed GDVariant"],
        "Unit",
        "cinch_gd_bridge::ref<godot::Array>(p).append(cinch_gd_bridge::ref<godot::Variant>(v));",
        "void* p,void* v",
        "void",
    )?;
    g.foreign(&prefix,"gdArrayRemove",&["Borrowed GDArray","I64"],"Unit","auto& a=cinch_gd_bridge::ref<godot::Array>(p);cinch_gd_bridge::index(i,a.size());a.remove_at(i);","void* p,int64_t i","void")?;
    g.foreign(&prefix,"gdDictionaryHas",&["Borrowed GDDictionary","Borrowed GDVariant"],"I32","return cinch_gd_bridge::ref<godot::Dictionary>(p).has(cinch_gd_bridge::ref<godot::Variant>(k));","void* p,void* k","int32_t")?;
    g.foreign(&prefix,"gdDictionaryGet",&["Borrowed GDDictionary","Borrowed GDVariant"],"Owned GDHeap GDVariant","auto& d=cinch_gd_bridge::ref<godot::Dictionary>(p);auto& key=cinch_gd_bridge::ref<godot::Variant>(k);if(!d.has(key))cinch_gd_bridge::fault();return new godot::Variant(d[key]);","void* p,void* k","void*")?;
    g.foreign(&prefix,"gdDictionarySet",&["Borrowed GDDictionary","Borrowed GDVariant","Borrowed GDVariant"],"Unit","cinch_gd_bridge::ref<godot::Dictionary>(p)[cinch_gd_bridge::ref<godot::Variant>(k)]=cinch_gd_bridge::ref<godot::Variant>(v);","void* p,void* k,void* v","void")?;
    g.foreign(&prefix,"gdDictionaryRemove",&["Borrowed GDDictionary","Borrowed GDVariant"],"I32","return cinch_gd_bridge::ref<godot::Dictionary>(p).erase(cinch_gd_bridge::ref<godot::Variant>(k));","void* p,void* k","int32_t")?;
    g.foreign(
        &prefix,
        "gdDictionaryKeys",
        &["Borrowed GDDictionary"],
        "Owned GDHeap GDArray",
        "return new godot::Array(cinch_gd_bridge::ref<godot::Dictionary>(p).keys());",
        "void* p",
        "void*",
    )?;
    g.function("gdValueString", &["String"], "Owned GDHeap GDVariant", "gdValueString text = let s = gdWriteString text in let v = gdVariantString s in let u = gdStringDelete s in v");
    g.function("gdReadValueString", &["Borrowed GDVariant"], "String", "gdReadValueString value = let s = gdVariantAsString value in let text = gdReadString s in let u = gdStringDelete s in text");
    g.function(
        "gdVariantBool",
        &["Bool"],
        "Owned GDHeap GDVariant",
        "gdVariantBool b = gdVariantBoolWord (gdBoolWord b)",
    );
    g.function(
        "gdVariantAsBool",
        &["Borrowed GDVariant"],
        "Bool",
        "gdVariantAsBool v = gdWordBool (gdVariantAsBoolWord v)",
    );
    let obj = "MaybeNull (RawPtr GDObject)";
    g.foreign(
        &prefix,
        "gdObjectId",
        &[obj],
        "U64",
        "if(!p)return 0;return static_cast<godot::Object*>(p)->get_instance_id();",
        "void* p",
        "uint64_t",
    )?;
    g.foreign(
        &prefix,
        "gdObjectFromId",
        &["U64"],
        obj,
        "return godot::ObjectDB::get_instance(id);",
        "uint64_t id",
        "void*",
    )?;
    g.foreign(
        &prefix,
        "gdVariantObject",
        &[obj],
        "Owned GDHeap GDVariant",
        "return new godot::Variant(static_cast<godot::Object*>(p));",
        "void* p",
        "void*",
    )?;
    g.foreign(&prefix, "gdVariantAsObject", &["Borrowed GDVariant"], obj, "auto& v=cinch_gd_bridge::ref<godot::Variant>(p);if(v.get_type()==godot::Variant::NIL)return nullptr;return static_cast<godot::Object*>(cinch_gd_bridge::variant(p,godot::Variant::OBJECT));", "void* p", "void*")?;
    g.foreign(&prefix, "gdObjectCallRaw", &[obj,"Borrowed GDString","Borrowed GDArray"], "Owned GDHeap GDVariant", "auto* o=static_cast<godot::Object*>(p);if(!o)cinch_gd_bridge::fault();auto& a=cinch_gd_bridge::ref<godot::Array>(args);std::vector<godot::Variant> values;std::vector<const godot::Variant*> pointers;for(int64_t i=0;i<a.size();++i)values.push_back(a[i]);for(auto& v:values)pointers.push_back(&v);godot::Variant object(o);GDExtensionCallError e{};godot::Variant out;object.callp(godot::StringName(cinch_gd_bridge::ref<godot::String>(method)),pointers.data(),pointers.size(),out,e);if(e.error!=GDEXTENSION_CALL_OK)cinch_gd_bridge::fault();return new godot::Variant(out);", "void* p,void* method,void* args", "void*")?;
    g.foreign(&prefix, "gdObjectGetRaw", &[obj,"Borrowed GDString"], "Owned GDHeap GDVariant", "auto* o=static_cast<godot::Object*>(p);if(!o)cinch_gd_bridge::fault();bool valid=false;auto v=godot::Variant(o).get_named(godot::StringName(cinch_gd_bridge::ref<godot::String>(key)),valid);if(!valid)cinch_gd_bridge::fault();return new godot::Variant(v);", "void* p,void* key", "void*")?;
    g.foreign(&prefix, "gdObjectSetRaw", &[obj,"Borrowed GDString","Borrowed GDVariant"], "Unit", "auto* o=static_cast<godot::Object*>(p);if(!o)cinch_gd_bridge::fault();bool valid=false;godot::Variant object(o);object.set_named(godot::StringName(cinch_gd_bridge::ref<godot::String>(key)),cinch_gd_bridge::ref<godot::Variant>(value),valid);if(!valid)cinch_gd_bridge::fault();", "void* p,void* key,void* value", "void")?;
    for (name, extra, result) in [
        ("Call", "Borrowed GDArray", "Owned GDHeap GDVariant"),
        ("Get", "", "Owned GDHeap GDVariant"),
        ("Set", "Borrowed GDVariant", "Unit"),
    ] {
        let params = if extra.is_empty() {
            vec![obj, "String"]
        } else {
            vec![obj, "String", extra]
        };
        let (arg, pass) = if extra.is_empty() {
            ("", "")
        } else {
            (" value", " value")
        };
        g.function(&format!("gdObject{name}"), &params,result,&format!("gdObject{name} object key{arg} = let text = gdWriteString key in let out = gdObject{name}Raw object text{pass} in let u = gdStringDelete text in out"));
    }
    for (name, action) in [
        (
            "Connect",
            "return s->connect(sn,godot::Callable(t,mn),flags);",
        ),
        (
            "Disconnect",
            "s->disconnect(sn,godot::Callable(t,mn)); return 0;",
        ),
        (
            "Connected",
            "return s->is_connected(sn,godot::Callable(t,mn));",
        ),
    ] {
        let body=format!("auto* s=static_cast<godot::Object*>(source);auto* t=static_cast<godot::Object*>(target);if(!s||!t)cinch_gd_bridge::fault();godot::StringName sn(cinch_gd_bridge::ref<godot::String>(signal));godot::StringName mn(cinch_gd_bridge::ref<godot::String>(method));{action}");
        g.foreign(
            &prefix,
            &format!("gdSignal{name}Raw"),
            &[obj, "Borrowed GDString", obj, "Borrowed GDString", "I64"],
            "I64",
            &body,
            "void* source,void* signal,void* target,void* method,int64_t flags",
            "int64_t",
        )?;
        g.function(&format!("gdSignal{name}"),&[obj,"String",obj,"String","I64"],"I64",&format!("gdSignal{name} source signal receiver method flags = let sn = gdWriteString signal in let mn = gdWriteString method in let code = gdSignal{name}Raw source sn receiver mn flags in let u = gdStringDelete sn in let v = gdStringDelete mn in code"));
    }
    Ok(g)
}

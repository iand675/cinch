//! The library declarations the compiler refers to by name.
//!
//! A source-declared type or class is identified by its declaring module
//! and its name (`Data.Core.List`, `Algebra.Arithmetic.Add`), and by
//! nothing else. Where the compiler itself must name one (the type of a
//! list literal, the result of a builtin, the class a literal resolves
//! through), it names it here, fully qualified, and every other module
//! uses the constant. A bare spelling such as `"List"` is never an
//! identity: it is surface syntax, and which declaration it reaches is
//! decided by the scope it is written in.
//!
//! The home module in each constant is the module whose source declares
//! the name. `tests::every_known_name_is_declared_by_the_embedded_family`
//! parses the embedded sources and fails, by constant, when a
//! declaration has moved or been renamed, so a stale entry is reported
//! here instead of as a type mismatch in an unrelated program.
//!
//! Types installed by `install_builtins` under the owner `<builtin>`
//! (`Vec`, `Io`, `Task`, the scalars and the rest) have no declaring
//! source and are not listed: their identities are their bare names.

macro_rules! known {
    ($(#[$group:meta])* $all:ident: $($(#[$doc:meta])* $name:ident = $identity:literal;)*) => {
        $($(#[$doc])* pub const $name: &str = $identity;)*
        $(#[$group])*
        pub const $all: &[&str] = &[$($name),*];
    };
}

known! {
    /// Every known type, for the declaration test.
    TYPES:
    /// The type of a list literal and of every builtin that takes or
    /// returns a list.
    LIST = "Data.Core.List";
    OPTION = "Data.Core.Option";
    RESULT = "Data.Core.Result";
    /// The type of a string literal.
    STRING = "Data.String.String";
    ORDERING = "Data.Ordering.Ordering";
    /// The type of a `json` data import.
    JSON = "Data.Json.Json";
    /// The step form the `vecStreamStep` builtin produces.
    STEP = "Data.Producer.Step";
    /// The type a map literal defaults to.
    MAP = "Data.Map.Map";
}

known! {
    /// Every known class, for the declaration test.
    CLASSES:
    EQ = "Relation.Eq.Eq";
    ORD = "Relation.Ord.Ord";
    /// The class a list literal resolves through.
    FROM_LIST = "Data.FromList.FromList";
    /// The class a map literal resolves through.
    FROM_MAP = "Data.FromMap.FromMap";
    /// The sequencing providers a `do` block selects between.
    DIRECT_SEQUENCING = "Effect.Sequencing.DirectSequencing";
    RESULT_SEQUENCING = "Effect.Sequencing.ResultSequencing";
    OPTION_SEQUENCING = "Effect.Sequencing.OptionSequencing";
}

known! {
    /// Every known constructor, for the declaration test.
    CONSTRUCTORS:
    /// The representation constructor of a string constant.
    STRING_BUFFER = "Data.String.StrBuf";
    STEP_DONE = "Data.Producer.Done";
    STEP_YIELD = "Data.Producer.Yield";
    ORDERING_LT = "Data.Ordering.LT";
    ORDERING_EQ = "Data.Ordering.EQ";
    ORDERING_GT = "Data.Ordering.GT";
}

known! {
    /// Every known value, for the declaration test.
    VALUES:
    /// The keyed three-way comparison whose checked body licenses a
    /// keyed ordering contract.
    COMPARING = "Relation.Ord.comparing";
}

/// The declaring module and the name of a qualified identity.
pub fn split(identity: &str) -> (&str, &str) {
    identity.rsplit_once('.').expect("a known identity is `Owner.Name`")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Decl;
    use std::collections::HashSet;

    #[derive(Default)]
    struct Declared {
        types: HashSet<String>,
        classes: HashSet<String>,
        constructors: HashSet<String>,
        values: HashSet<String>,
    }

    fn collect(decl: &Decl, owner: &str, out: &mut Declared) {
        let qualify = |name: &str| format!("{owner}.{name}");
        match decl {
            Decl::Attributed { inner, .. } | Decl::Conditional { inner, .. } => collect(inner, owner, out),
            Decl::Data(data) => {
                out.types.insert(qualify(&data.name.text));
                for constructor in &data.ctors {
                    out.constructors.insert(qualify(&constructor.name.text));
                }
            }
            Decl::Record(record) | Decl::Codata(record) => {
                out.types.insert(qualify(&record.name.text));
            }
            Decl::Refine(refine) => {
                out.types.insert(qualify(&refine.name.text));
            }
            Decl::OpaqueType(opaque) => {
                out.types.insert(qualify(&opaque.name.text));
            }
            Decl::Signature(signature) => {
                out.classes.insert(qualify(&signature.name.text));
            }
            Decl::ValueSig { name, .. } => {
                out.values.insert(qualify(&name.text));
            }
            _ => {}
        }
    }

    #[test]
    fn every_known_name_is_declared_by_the_embedded_family() {
        let family = crate::driver::parse_prelude(&[]).expect("the embedded family parses");
        let mut declared = Declared::default();
        for module in &family.modules {
            for decl in &module.decls {
                collect(decl, &module.name.text, &mut declared);
            }
        }
        let mut missing = Vec::new();
        for (kind, names, found) in [
            ("type", TYPES, &declared.types),
            ("class", CLASSES, &declared.classes),
            ("constructor", CONSTRUCTORS, &declared.constructors),
            ("value", VALUES, &declared.values),
        ] {
            for name in names {
                if !found.contains(*name) {
                    missing.push(format!("{kind} {name}"));
                }
            }
        }
        assert!(
            missing.is_empty(),
            "compiler/src/known.rs names declarations the embedded family does not declare: {}",
            missing.join(", ")
        );
    }

    #[test]
    fn a_known_identity_splits_at_its_declaring_module() {
        assert_eq!(split(LIST), ("Data.Core", "List"));
        assert_eq!(split(DIRECT_SEQUENCING), ("Effect.Sequencing", "DirectSequencing"));
    }
}

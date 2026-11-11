//! How a nominal name is written.
//!
//! A checked type names its declaration by identity (`Data.Core.List`,
//! `Main.Tree`), and `Display` writes exactly that: an interface, a Core
//! dump and every other text a program reads back need the identity. A
//! diagnostic is read by a person who wrote `List` and `Tree`, so it
//! writes the bare name, and the identity only where the bare name would
//! not say which declaration is meant.
//!
//! The rule is local to one diagnostic. When two different identities in
//! the message share a bare name, each of them is written in full; every
//! other name is written bare. [`diagnostic!`](crate::diagnostic) applies
//! the rule to the values it is given, so the ambiguity is decided over
//! exactly the names the message prints, and no rendering depends on
//! state outside its own arguments.
//!
//! The rule does not consult the scope of the module being reported on:
//! the unifier renders most mismatches and has no scope. A message that
//! names one `Option` therefore prints it bare even when the reporting
//! module declares an `Option` of its own.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

/// How one rendering writes nominal identities.
#[derive(Clone, Copy)]
pub enum Spelling<'a> {
    /// The identity itself: the form of every machine-read text.
    Identity,
    /// A diagnostic's spelling, decided over the names of one message.
    Diagnostic(&'a Names),
}

impl<'a> Spelling<'a> {
    /// `value` as this rendering writes it.
    pub fn of<T: Spell + ?Sized>(self, value: &'a T) -> Spelled<'a, T> {
        Spelled(value, self)
    }

    /// Write the nominal `identity` of a type or a class.
    pub fn nominal(self, f: &mut fmt::Formatter<'_>, identity: &str) -> fmt::Result {
        match self {
            Spelling::Identity => f.write_str(identity),
            Spelling::Diagnostic(names) => f.write_str(names.spell(identity)),
        }
    }
}

/// A value whose rendering mentions nominal identities, or a plain value
/// printed beside one.
pub trait Spell {
    fn spell(&self, f: &mut fmt::Formatter<'_>, spelling: Spelling<'_>) -> fmt::Result;
}

impl<T: Spell + ?Sized> Spell for &T {
    fn spell(&self, f: &mut fmt::Formatter<'_>, spelling: Spelling<'_>) -> fmt::Result {
        (**self).spell(f, spelling)
    }
}

impl<T: Spell + ?Sized> Spell for Box<T> {
    fn spell(&self, f: &mut fmt::Formatter<'_>, spelling: Spelling<'_>) -> fmt::Result {
        (**self).spell(f, spelling)
    }
}

/// Values that mention no nominal identity print as they always do.
macro_rules! plain {
    ($($ty:ty),* $(,)?) => {
        $(impl Spell for $ty {
            fn spell(&self, f: &mut fmt::Formatter<'_>, _: Spelling<'_>) -> fmt::Result {
                fmt::Display::fmt(self, f)
            }
        })*
    };
}

plain!(
    str, String, char, bool, usize, u8, u16, u32, u64, i32, i64,
    crate::intern::Interned, crate::lexer::Span, crate::core::GlobalId,
    crate::types::Scalar, crate::types::Mult, crate::types::EffRow, crate::types::EffLabel,
    crate::types::Sort, crate::types::IdxCmp, crate::types::IdxOp, crate::types::PredFn,
    crate::types::PtrRole,
);

/// A value paired with the rendering that writes it.
pub struct Spelled<'a, T: ?Sized>(&'a T, Spelling<'a>);

impl<T: Spell + ?Sized> fmt::Display for Spelled<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.spell(f, self.1)
    }
}

/// The identity of a type or a class held as text (a `TypeKind`'s
/// identity, a class key), as one argument of a diagnostic.
pub struct Nominal<'a>(pub &'a str);

impl Spell for Nominal<'_> {
    fn spell(&self, f: &mut fmt::Formatter<'_>, spelling: Spelling<'_>) -> fmt::Result {
        spelling.nominal(f, self.0)
    }
}

/// `items` written in order with `separator` between them, as one
/// argument of a diagnostic.
pub struct Listed<'a, T>(pub &'a [T], pub &'a str);

impl<T: Spell> Spell for Listed<'_, T> {
    fn spell(&self, f: &mut fmt::Formatter<'_>, spelling: Spelling<'_>) -> fmt::Result {
        for (index, item) in self.0.iter().enumerate() {
            if index > 0 {
                f.write_str(self.1)?;
            }
            item.spell(f, spelling)?;
        }
        Ok(())
    }
}

/// Every fact in scope, written with `separator` between them, as one
/// argument of a diagnostic: the `Prop`-typed binders by name, then the
/// predicates of refined locals. A diagnostic that prints a variable
/// number of facts (a failed proof search lists them all) needs them to
/// share the message's names, which a `format!`-built string cannot do.
pub struct Facts<'a> {
    /// The `Prop`-typed binders: name, proposition, and the rest of the
    /// evidence tuple the checker carries.
    pub items: &'a [(String, crate::types::PTerm, crate::core::LocalId, Vec<String>)],
    /// The predicates of refined locals in scope, stated of the local.
    pub refined: &'a [crate::types::PTerm],
    pub separator: &'a str,
}

impl Spell for Facts<'_> {
    fn spell(&self, f: &mut fmt::Formatter<'_>, spelling: Spelling<'_>) -> fmt::Result {
        for (index, (name, proposition, _, _)) in self.items.iter().enumerate() {
            if index > 0 {
                f.write_str(self.separator)?;
            }
            write!(f, "{name}: {}", spelling.of(proposition))?;
        }
        for (offset, predicate) in self.refined.iter().enumerate() {
            if !self.items.is_empty() || offset > 0 {
                f.write_str(self.separator)?;
            }
            predicate.spell(f, spelling)?;
        }
        Ok(())
    }
}

/// The bare name of a type or class identity: the text after the
/// declaring module. A `<builtin>` identity has no module and is its own
/// bare name.
pub fn bare_name(identity: &str) -> &str {
    identity.rsplit_once('.').map_or(identity, |(_, name)| name)
}

/// The nominal identities one diagnostic mentions, by bare name.
#[derive(Default)]
pub struct Names {
    seen: RefCell<HashMap<String, Vec<String>>>,
}

impl Names {
    /// Record `identity` and answer how it is written: bare while it is
    /// the only identity recorded under its bare name.
    fn spell<'a>(&self, identity: &'a str) -> &'a str {
        let bare = bare_name(identity);
        let mut seen = self.seen.borrow_mut();
        let identities = seen.entry(bare.to_string()).or_default();
        if !identities.iter().any(|known| known == identity) {
            identities.push(identity.to_string());
        }
        if identities.len() > 1 { identity } else { bare }
    }

    /// Render `message`, whose arguments were built with
    /// `Spelling::Diagnostic(self)`. The message is written twice: the
    /// first pass is discarded and only records every identity the
    /// message mentions, so the second knows which bare names are shared
    /// before it writes the first of them.
    pub fn render(&self, message: fmt::Arguments<'_>) -> String {
        struct Discard;
        impl fmt::Write for Discard {
            fn write_str(&mut self, _: &str) -> fmt::Result {
                Ok(())
            }
        }
        let _ = fmt::write(&mut Discard, message);
        message.to_string()
    }
}

/// `format!` for a diagnostic: every positional argument is written with
/// the diagnostic spelling of this one message. Arguments are positional
/// so that each of them passes through the rule; a name captured inline
/// in the format string is written by its own `Display`.
#[macro_export]
macro_rules! diagnostic {
    ($format:literal $(, $argument:expr)* $(,)?) => {{
        let names = $crate::spelling::Names::default();
        let message = names.render(format_args!(
            $format
            $(, $crate::spelling::Spelling::Diagnostic(&names).of(&$argument))*
        ));
        message
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ClassApp, Ty};

    fn adt(identity: &str, arguments: Vec<Ty>) -> Ty {
        Ty::Adt(identity.into(), arguments)
    }

    #[test]
    fn an_unambiguous_name_prints_bare() {
        let expected = adt(crate::known::OPTION, vec![adt("Main.Tree", Vec::new())]);
        let found = adt("Main.Tree", Vec::new());
        assert_eq!(
            diagnostic!("type mismatch: expected {}, got {}", expected, found),
            "type mismatch: expected Option Tree, got Tree"
        );
    }

    #[test]
    fn two_identities_with_one_bare_name_both_print_qualified() {
        let base = adt(crate::known::STEP, vec![Ty::Nat, Ty::Nat]);
        let local = adt("Main.Step", Vec::new());
        assert_eq!(
            diagnostic!("type mismatch: expected {}, got {}", base, local),
            "type mismatch: expected Data.Producer.Step Nat Nat, got Main.Step"
        );
        // The same identity twice is one name, and stays bare.
        assert_eq!(
            diagnostic!("type mismatch: expected {}, got {}", base, adt(crate::known::STEP, vec![Ty::Int, Ty::Nat])),
            "type mismatch: expected Step Nat Nat, got Step Int Nat"
        );
    }

    #[test]
    fn a_clash_inside_one_type_qualifies_both_occurrences() {
        let pair = Ty::Tuple(vec![adt("A.Step", Vec::new()), adt("B.Step", Vec::new())]);
        assert_eq!(diagnostic!("cannot match {}", pair), "cannot match (A.Step, B.Step)");
    }

    #[test]
    fn a_class_and_its_arguments_follow_the_same_rule() {
        let class = ClassApp { class: crate::known::EQ.into(), args: vec![adt("Main.Tree", Vec::new())] };
        assert_eq!(diagnostic!("no instance for {}", class), "no instance for Eq Tree");
        let other = ClassApp { class: "Main.Eq".into(), args: vec![Ty::Int] };
        assert_eq!(
            diagnostic!("{} is not {}", class, other),
            "Relation.Eq.Eq Tree is not Main.Eq Int"
        );
    }

    #[test]
    fn a_builtin_identity_is_its_own_bare_name() {
        let vector = adt("Vec", vec![Ty::Nat]);
        let local = adt("Main.Vec", Vec::new());
        assert_eq!(diagnostic!("{} against {}", vector, local), "Vec Nat against Main.Vec");
    }

    #[test]
    fn display_keeps_the_identity() {
        let list = adt(crate::known::LIST, vec![adt("Main.Tree", Vec::new())]);
        assert_eq!(list.to_string(), "Data.Core.List Main.Tree");
    }

    #[test]
    fn a_listed_argument_shares_the_message_names() {
        let classes = [
            ClassApp { class: "A.Run".into(), args: Vec::new() },
            ClassApp { class: "B.Run".into(), args: Vec::new() },
        ];
        assert_eq!(diagnostic!("ambiguous ({})", Listed(&classes, ", ")), "ambiguous (A.Run, B.Run)");
    }
}

//! Preserve strict evaluation boundaries in curried application spines.
//!
//! A known unit can collect arguments up to its runtime arity: those
//! applications only construct partial closures. Once saturated, its body
//! must run before an excess argument is evaluated. An unknown function
//! may saturate at any argument, so each application is a separate stage.
//! This runs before wide edges and reference counting, which must plan
//! representations and ownership for the same evaluation order as emission.

use crate::core::{child_slots, child_slots_imm, Core, Program};
use std::borrow::Cow;
use std::collections::HashMap;

/// Runtime arities available without evaluating a function value.
struct Arities(HashMap<String, usize>);

impl Arities {
    /// Include separately compiled declarations and foreign wrappers too.
    fn of(program: &Program) -> Self {
        let mut arities: HashMap<_, _> = program
            .fns
            .iter()
            .map(|f| (f.id.0.clone(), f.params.len()))
            .collect();
        arities.extend(
            program
                .foreigns
                .iter()
                .map(|(name, plan)| (name.clone(), plan.params.len())),
        );
        Self(arities)
    }

    /// The first argument count at which this head may execute a body.
    fn stage(&self, head: &Core) -> usize {
        let arity = match head {
            Core::Global(id, _) | Core::Ctor(id, _) => self.0.get(&id.0).copied(),
            Core::Foreign(name) => self.0.get(name).copied(),
            Core::Prim(op) => Some(op.arity()),
            Core::Lam { fn_id, env, .. } => {
                self.0.get(&fn_id.0).and_then(|n| n.checked_sub(env.len()))
            }
            _ => None,
        };
        // A zero-arity reference evaluates its body as the head itself;
        // the arity of the function it returns is unknown here.
        arity.filter(|n| *n != 0).unwrap_or(1)
    }

    /// Whether any application needs a boundary, including its head.
    fn needed(&self, e: &Core) -> bool {
        if let Core::App(head, args) = e {
            if args.len() > self.stage(head) || self.needed(head) {
                return true;
            }
        }
        child_slots_imm(e)
            .into_iter()
            .any(|child| self.needed(child))
    }

    /// Split a spine after saturation, then stage its unknown result.
    fn insert(&self, e: &mut Core) {
        if let Core::App(head, _) = e {
            self.insert(head);
        }
        for child in child_slots(e) {
            self.insert(child);
        }
        let Core::App(head, args) = e else { return };
        let stage = self.stage(head);
        if args.len() <= stage {
            return;
        }
        let excess = args.split_off(stage);
        let mut result = std::mem::replace(e, Core::Unit);
        for argument in excess {
            result = Core::App(Box::new(result), vec![argument]);
        }
        *e = result;
    }
}

/// Insert strict call stages before representation and ownership planning.
pub(crate) fn insert_boundaries(program: &mut Program) {
    let arities = Arities::of(program);
    for f in &mut program.fns {
        arities.insert(&mut f.body);
    }
}

/// Stage checked Core emitted directly, borrowing an already staged program.
/// This is idempotent, including after reference counting has run.
pub(crate) fn with_boundaries(program: &Program) -> Cow<'_, Program> {
    let arities = Arities::of(program);
    if !program.fns.iter().any(|f| arities.needed(&f.body)) {
        return Cow::Borrowed(program);
    }
    let mut staged = program.clone();
    for f in &mut staged.fns {
        arities.insert(&mut f.body);
    }
    Cow::Owned(staged)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{GlobalId, LocalId, PrimOp};
    use crate::repr::Class;

    /// Build a direct reference with no representation parameters.
    fn global(name: &str) -> Core {
        Core::Global(GlobalId(name.into()), vec![])
    }

    /// Known saturated and partial applications retain their direct path.
    #[test]
    fn known_calls_only_split_after_saturation() {
        let arities = Arities(HashMap::from([("producer".into(), 2)]));
        for count in [1, 2] {
            let mut e = Core::App(Box::new(global("producer")), vec![Core::Unit; count]);
            let original = e.clone();
            arities.insert(&mut e);
            assert_eq!(format!("{e:?}"), format!("{original:?}"));
        }
        let mut e = Core::App(Box::new(global("producer")), vec![Core::Unit; 4]);
        arities.insert(&mut e);
        let Core::App(last, args) = &e else {
            panic!("last stage")
        };
        assert_eq!(args.len(), 1);
        let Core::App(next, args) = &**last else {
            panic!("next stage")
        };
        assert_eq!(args.len(), 1);
        let Core::App(head, args) = &**next else {
            panic!("producer stage")
        };
        assert_eq!(args.len(), 2);
        assert!(matches!(&**head, Core::Global(id, _) if id.0 == "producer"));
        assert!(!arities.needed(&e));
    }

    /// An unknown function may execute between every pair of arguments.
    #[test]
    fn unknown_calls_preserve_each_application_boundary() {
        let arities = Arities(HashMap::new());
        let mut e = Core::App(
            Box::new(Core::Var(LocalId(0))),
            vec![Core::Int(1), Core::Int(2)],
        );
        arities.insert(&mut e);
        let Core::App(head, args) = &e else {
            panic!("second stage")
        };
        assert!(matches!(args.as_slice(), [Core::Int(2)]));
        let Core::App(_, args) = &**head else {
            panic!("first stage")
        };
        assert!(matches!(args.as_slice(), [Core::Int(1)]));
        assert!(!arities.needed(&e));
    }

    /// Constructor units and inline primitive declarations publish their
    /// real runtime parameters; neither is treated as an unknown function.
    #[test]
    fn constructor_and_primitive_arities_keep_saturated_spines() {
        let mut program = crate::driver::check_sources(
            "module Main where\n  data Pair = Pair Nat Nat\n  pack :: Nat -> Nat -> Pair\n  pack first second = Pair first second\n  main :: Int\n  main = 0\n",
            "application-arities.cinch",
        ).expect("constructor arity source");
        let constructor = program
            .fns
            .iter()
            .find(|f| f.ctor.is_some() && f.id.0 == "Main.Pair")
            .expect("published constructor unit")
            .id
            .clone();
        let inline = program
            .fns
            .iter_mut()
            .find(|f| f.id.0 == "Main.pack")
            .expect("declared unit");
        inline.meta.prim_body = true;
        let arities = Arities::of(&program);
        for head in [
            Core::Ctor(constructor, vec![]),
            global("Main.pack"),
            Core::Prim(PrimOp::EqBool),
        ] {
            assert_eq!(arities.stage(&head), 2);
            let mut e = Core::App(Box::new(head), vec![Core::Unit; 2]);
            arities.insert(&mut e);
            let Core::App(_, args) = e else {
                panic!("saturated call")
            };
            assert_eq!(args.len(), 2);
        }
    }

    /// A captured lambda saturates after its uncaptured runtime parameters.
    #[test]
    fn captured_lambda_subtracts_environment_from_arity() {
        let arities = Arities(HashMap::from([("lambda".into(), 4)]));
        let head = Core::Lam {
            fn_id: GlobalId("lambda".into()),
            env: vec![Core::Unit; 2],
            classes: vec![],
        };
        assert_eq!(arities.stage(&head), 2);
        let mut e = Core::App(Box::new(head), vec![Core::Unit; 3]);
        arities.insert(&mut e);
        let Core::App(head, args) = &e else {
            panic!("excess call")
        };
        assert_eq!(args.len(), 1);
        let Core::App(_, args) = &**head else {
            panic!("lambda call")
        };
        assert_eq!(args.len(), 2);
        assert!(!arities.needed(&e));
    }

    /// A dictionary method value and a nullary producer's result have no
    /// known runtime arity, even when their types contain several arrows.
    #[test]
    fn dictionary_and_nullary_results_keep_dynamic_boundaries() {
        let arities = Arities(HashMap::from([("producer".into(), 0)]));
        let dictionary = Core::InstDict(GlobalId("dictionary".into()), vec![], vec![]);
        for head in [
            Core::DictProj(Box::new(dictionary), 0, None, Class::Obj),
            global("producer"),
        ] {
            let mut e = Core::App(Box::new(head), vec![Core::Unit; 2]);
            arities.insert(&mut e);
            let Core::App(head, args) = &e else {
                panic!("second call")
            };
            assert_eq!(args.len(), 1);
            let Core::App(_, args) = &**head else {
                panic!("first call")
            };
            assert_eq!(args.len(), 1);
            assert!(!arities.needed(&e));
        }
    }
}

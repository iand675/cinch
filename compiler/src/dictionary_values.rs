//! Close instance methods over initialized strict value slots, never over
//! their owning dictionary. This preserves sharing without RC self cycles.
use crate::core::{Core, FnDef, GlobalId, LocalId, Program};
use crate::repr::Class;
use crate::{Error, Result};
use std::collections::{BTreeMap, BTreeSet, HashMap};

fn visit(e: &Core, f: &mut dyn FnMut(&Core)) {
    f(e);
    match e {
        Core::DictProj(x, ..) | Core::Proj(x, _) | Core::Demand(x, _) | Core::BoxGet(x, _) => visit(x, f),
        Core::InstDict(_, xs, _) | Core::Lam { env: xs, .. } => for x in xs { visit(x, f); },
        Core::App(h, xs) => { visit(h, f); for x in xs { visit(x, f); } },
        Core::Let(_, _, x, b) => { visit(x, f); visit(b, f); },
        Core::LazyInit(_, x) | Core::BoxInit(_, x, _) | Core::RefSet(_, x) | Core::Dup(_, _, x) | Core::Drop(_, _, x) => visit(x, f),
        Core::If(c, t, n) => { visit(c, f); visit(&t.body, f); visit(&n.body, f); },
        Core::Case { scrut, alts } => { visit(scrut, f); for a in alts { visit(&a.body, f); } },
        Core::Quote { holes, body, .. } => { for h in holes { visit(h, f); } if let Some(b) = body { visit(b, f); } },
        Core::DeclsQuote { holes, .. } => for h in holes { visit(h, f); },
        _ => {},
    }
}
fn children(e: &mut Core, f: &mut dyn FnMut(&mut Core) -> Result<()>) -> Result<()> {
    match e {
        Core::DictProj(x, ..) | Core::Proj(x, _) | Core::Demand(x, _) | Core::BoxGet(x, _) => f(x)?,
        Core::InstDict(_, xs, _) | Core::Lam { env: xs, .. } => for x in xs { f(x)?; },
        Core::App(h, xs) => { f(h)?; for x in xs { f(x)?; } },
        Core::Let(_, _, x, b) => { f(x)?; f(b)?; },
        Core::LazyInit(_, x) | Core::BoxInit(_, x, _) | Core::RefSet(_, x) | Core::Dup(_, _, x) | Core::Drop(_, _, x) => f(x)?,
        Core::If(c, t, n) => { f(c)?; f(&mut t.body)?; f(&mut n.body)?; },
        Core::Case { scrut, alts } => { f(scrut)?; for a in alts { f(&mut a.body)?; } },
        Core::Quote { holes, body, .. } => { for h in holes { f(h)?; } if let Some(b) = body { f(b)?; } },
        Core::DeclsQuote { holes, .. } => for h in holes { f(h)?; },
        _ => {},
    }
    Ok(())
}

struct Factory {
    name: String,
    methods: Vec<String>,
    strict: Vec<bool>,
    classes: Vec<Class>,
}
fn self_projection(e: &Core) -> Option<(&str, usize, &[Core], &[Class])> {
    match e {
        Core::DictProj(d, slot, ..) => match &**d {
            Core::InstDict(g, args, classes) => g.0.strip_suffix("#self").map(|name| (name, *slot, args.as_slice(), classes.as_slice())),
            _ => None,
        },
        _ => None,
    }
}
fn hidden_args(slots: &[usize], environment: &BTreeMap<usize, LocalId>) -> Result<Vec<Core>> {
    slots.iter().map(|slot| environment.get(slot).copied().map(Core::Var)
        .ok_or_else(|| Error::plain(format!("dictionary value slot {slot} is absent from its initialized environment")))).collect()
}
fn rewrite(e: &mut Core, owner: Option<usize>, factories: &[Factory], owners: &HashMap<String, usize>, dependencies: &HashMap<String, Vec<usize>>, environment: &BTreeMap<usize, LocalId>) -> Result<()> {
    if let Some((name, slot, args, classes)) = self_projection(e) {
        let factory = owner.and_then(|i| factories.get(i)).filter(|f| f.name == name)
            .ok_or_else(|| Error::plain(format!("dictionary self projection has no actual declaring instance: {name}")))?;
        let method = factory.methods.get(slot).ok_or_else(|| Error::plain("dictionary self slot is absent from the actual instance"))?;
        if factory.strict[slot] {
            if matches!(e, Core::DictProj(_, _, Some(site), _) if !site.own.is_empty()) {
                return Err(Error::plain("self projection of a polymorphic value field needs representation-indexed environment lowering"));
            }
            *e = hidden_args(&[slot], environment)?.remove(0);
        } else {
            let mut prefix = hidden_args(&dependencies[method], environment)?;
            prefix.extend_from_slice(args);
            let mut method_classes = classes.to_vec();
            if let Core::DictProj(_, _, Some(site), _) = e { method_classes.extend_from_slice(&site.own); }
            let head = Core::Global(GlobalId(method.clone()), method_classes);
            *e = if prefix.is_empty() { head } else { Core::App(Box::new(head), prefix) };
        }
        return Ok(());
    }
    match e {
        Core::InstDict(g, _, _) if g.0.ends_with("#self") => {
            let name = g.0.strip_suffix("#self").unwrap();
            let factory = owner.and_then(|i| factories.get(i)).filter(|f| f.name == name)
                .ok_or_else(|| Error::plain("dictionary self evidence has no declaring instance"))?;
            if factory.strict.iter().any(|v| *v) {
                return Err(Error::plain("materializing self evidence with strict value fields needs a shared dictionary environment"));
            }
            g.0 = name.to_string();
        },
        Core::Lam { fn_id, env, .. } => {
            if owners.get(&fn_id.0).copied() == owner && owner.is_some() {
                let mut prefix = hidden_args(&dependencies[&fn_id.0], environment)?;
                for value in env.iter_mut() { rewrite(value, owner, factories, owners, dependencies, environment)?; }
                prefix.append(env);
                *env = prefix;
                return Ok(());
            }
        },
        Core::Global(g, classes) if dependencies.get(&g.0).is_some_and(|d| !d.is_empty()) => {
            if owner.is_none() || owners.get(&g.0).copied() != owner {
                return Err(Error::plain("a private instance method needs its initialized value environment"));
            }
            let prefix = hidden_args(&dependencies[&g.0], environment)?;
            *e = Core::App(Box::new(Core::Global(g.clone(), classes.clone())), prefix);
            return Ok(());
        },
        _ => {},
    }
    children(e, &mut |child| rewrite(child, owner, factories, owners, dependencies, environment))
}

pub(crate) fn lower(program: &mut Program) -> Result<()> {
    let original: HashMap<String, &FnDef> = program.fns.iter().map(|f| (f.id.0.clone(), f)).collect();
    let factories: Vec<Factory> = program.instances.iter().map(|instance| Factory {
        name: instance.id.0.clone(),
        methods: instance.methods.iter().map(|m| m.0.clone()).collect(),
        strict: instance.methods.iter().map(|m| original.get(&m.0).is_some_and(|f| f.arity() == instance.subdict_count)).collect(),
        classes: instance.methods.iter().map(|m| original.get(&m.0).and_then(|f| f.repr.result).unwrap_or(Class::Raw)).collect(),
    }).collect();
    let mut owners = HashMap::new();
    for f in &program.fns {
        let mut best: Option<(usize, usize)> = None;
        for (index, factory) in factories.iter().enumerate() {
            for method in &factory.methods {
                if (f.id.0 == *method || f.id.0.starts_with(&format!("{method}."))) && best.is_none_or(|(_, len)| method.len() > len) {
                    best = Some((index, method.len()));
                }
            }
        }
        if let Some((index, _)) = best { owners.insert(f.id.0.clone(), index); }
    }
    let mut sets: HashMap<String, BTreeSet<usize>> = program.fns.iter().map(|f| (f.id.0.clone(), BTreeSet::new())).collect();
    loop {
        let mut changed = false;
        for f in &program.fns {
            let Some(&owner) = owners.get(&f.id.0) else { continue; };
            let factory = &factories[owner];
            let mut found = sets[&f.id.0].clone();
            visit(&f.body, &mut |e| {
                if let Some((name, slot, _, _)) = self_projection(e) {
                    if name == factory.name {
                        if let Some(method) = factory.methods.get(slot) {
                            if factory.strict[slot] { found.insert(slot); }
                            else { found.extend(sets[method].iter().copied()); }
                        }
                    }
                }
                let callee = match e { Core::Lam { fn_id, .. } | Core::Global(fn_id, _) => Some(&fn_id.0), _ => None };
                if let Some(callee) = callee.filter(|c| owners.get(*c) == Some(&owner)) {
                    found.extend(sets[callee].iter().copied());
                }
            });
            if found != sets[&f.id.0] { sets.insert(f.id.0.clone(), found); changed = true; }
        }
        if !changed { break; }
    }
    for factory in &factories {
        let mut pending: BTreeSet<usize> = factory.strict.iter().enumerate().filter_map(|(i, strict)| strict.then_some(i)).collect();
        let mut initialized = BTreeSet::new();
        while !pending.is_empty() {
            let next = pending.iter().copied().find(|slot| sets[&factory.methods[*slot]].is_subset(&initialized));
            let Some(next) = next else { return Err(Error::plain(format!("cyclic strict dictionary values in {} need recursive environment lowering", factory.name))); };
            pending.remove(&next); initialized.insert(next);
        }
    }
    let dependencies: HashMap<String, Vec<usize>> = sets.into_iter().map(|(name, values)| (name, values.into_iter().collect())).collect();
    for f in &mut program.fns {
        let owner = owners.get(&f.id.0).copied();
        let slots = &dependencies[&f.id.0];
        let count = slots.len();
        let environment: BTreeMap<usize, LocalId> = slots.iter().enumerate().map(|(i, slot)| (*slot, LocalId(i))).collect();
        if count > 0 {
            let mut map = HashMap::new();
            crate::core::walk_locals(&f.body, &mut |id| { map.insert(id, LocalId(id.0 + count)); });
            f.body = crate::core::rename_core_locals(&f.body, &map);
            let factory = &factories[owner.unwrap()];
            let mut classes: Vec<Class> = slots.iter().map(|slot| factory.classes[*slot]).collect();
            classes.append(&mut f.repr.params); f.repr.params = classes;
            let mut names: Vec<String> = slots.iter().map(|slot| format!("$dictionary-value{slot}")).collect();
            names.append(&mut f.params); f.params = names;
            for (i, _) in &mut f.meta.linear { *i += count; }
            for i in &mut f.meta.pure_arrows { *i += count; }
            for (i, _) in &mut f.meta.dicts { *i += count; }
            let mut params = vec![Default::default(); count]; params.append(&mut f.meta.contract.params); f.meta.contract.params = params;
        }
        f.repr.dictionary_values = slots.clone();
        rewrite(&mut f.body, owner, &factories, &owners, &dependencies, &environment)?;
    }
    Ok(())
}

//! Declaration-group capabilities. Public spellings select a member of an
//! already possessed group; only opaque identities select its references.
use crate::ast::*;
use crate::{Error, Result};
use std::collections::HashMap;

fn binding_sites(decls: &mut [Decl], visit: &mut impl FnMut(&mut Name, &[RefKind])) {
    use RefKind::{Type, Value};
    for decl in decls {
        match decl {
            Decl::Attributed { inner, .. } => binding_sites(std::slice::from_mut(inner), visit),
            Decl::ValueSig { name, .. } => visit(name, &[Value]),
            Decl::Equation { eq, .. } => visit(&mut eq.name, &[Value]),
            Decl::Data(data) => {
                visit(&mut data.name, &[Type]);
                for ctor in &mut data.ctors { visit(&mut ctor.name, &[Value]); }
            }
            Decl::Record(record) => visit(&mut record.name, &[Type, Value]),
            Decl::Codata(record) => visit(&mut record.name, &[Type]),
            Decl::OpaqueType(opaque) => visit(&mut opaque.name, &[Type]),
            Decl::Refine(refine) => visit(&mut refine.name, &[Type, Value]),
            Decl::Foreign(foreign) => visit(&mut foreign.name, &[Value]),
            Decl::Prim(primitive) => visit(&mut primitive.name, &[Value]),
            Decl::Effect(effect) => {
                visit(&mut effect.name, &[RefKind::Effect]);
                for (name, _) in &mut effect.ops { visit(name, &[Value]); }
            }
            // Other declaration namespaces retain their existing quotation
            // contract until their member and binder scopes are implemented.
            _ => {}
        }
    }
}

/// Bind a quotation's declarations before preparing its body or inserting
/// captured trees. Signatures/equations share a handle; record type and
/// constructor occupy two namespaces with the same public name.
pub(crate) fn bind_group(decls: &mut [Decl]) -> Vec<(RefKind, Name)> {
    let mut members: HashMap<(RefKind, String), Name> = HashMap::new();
    let mut old = HashMap::new();
    binding_sites(decls, &mut |name, kinds| {
        let binding = kinds.iter().find_map(|kind| members.get(&(*kind, name.text.clone())))
            .and_then(|name| name.declaration_binding()).cloned()
            .unwrap_or_else(LocalBinding::fresh);
        if let Some(previous) = name.declaration_binding() { old.insert(previous.clone(), binding.clone()); }
        *name = name.with_declaration_binding(binding);
        for kind in kinds { members.insert((*kind, name.text.clone()), name.clone()); }
    });
    remap_group(decls, &old);
    members.into_iter().map(|((kind, _), name)| (kind, name)).collect()
}

/// Each insertion or instantiation gets new owned handles. Captures of a
/// different group are absent from this map and keep their original owner.
pub(crate) fn freshen_group(decls: &mut [Decl]) {
    instantiate_group(decls);
}

fn instantiate_group(decls: &mut [Decl]) -> HashMap<LocalBinding, LocalBinding> {
    let mut mapping = HashMap::new();
    binding_sites(decls, &mut |name, _| {
        if let Some(binding) = name.declaration_binding() {
            mapping.entry(binding.clone()).or_insert_with(LocalBinding::fresh);
        }
    });
    remap_group(decls, &mapping);
    mapping
}

fn remap_group(decls: &mut [Decl], mapping: &HashMap<LocalBinding, LocalBinding>) {
    visit_names(decls, &mut |name, _| {
        if let Some(binding) = name.declaration_binding().and_then(|old| mapping.get(old)) {
            *name = name.with_declaration_binding(binding.clone());
        }
    });
}

pub(crate) fn rename_group(decls: &[Decl], source: &str, target: &str) -> Result<Vec<Decl>> {
    let mut decls = decls.to_vec();
    freshen_group(&mut decls);
    let mut selected = Vec::new();
    binding_sites(&mut decls, &mut |name, _| {
        if name.text == source {
            if let Some(binding) = name.declaration_binding() { selected.push(binding.clone()); }
        }
    });
    if selected.is_empty() {
        return Err(Error::plain(format!("`{source}` is not an owned member of this declaration group")));
    }
    visit_names(&mut decls, &mut |name, _| {
        if name.declaration_binding().is_some_and(|binding| selected.contains(binding)) {
            name.text = target.to_string();
        }
    });
    Ok(decls)
}

pub(crate) fn group_members(decls: &[Decl]) -> Vec<(RefKind, Name)> {
    let mut members = Vec::new();
    binding_sites(&mut decls.to_vec(), &mut |name, kinds| {
        if name.declaration_binding().is_some() {
            for kind in kinds { members.push((*kind, name.clone())); }
        }
    });
    members
}

/// Substitute an owned placeholder with members of another possessed group.
/// The resulting references retain the context's handles; composing both
/// groups before emit freshens their anchors and cross-references together.
pub(crate) fn link_group(source: &str, target: &str, context: &[Decl], template: &[Decl]) -> Result<Vec<Decl>> {
    let mut result = template.to_vec();
    let instantiated = instantiate_group(&mut result);
    let source_members: Vec<_> = group_members(&result).into_iter().filter(|(_, name)| name.text == source).collect();
    if source_members.is_empty() { return Err(Error::plain(format!("`{source}` is not an owned member of this declaration group"))); }
    let mut target_members = HashMap::new();
    for (kind, mut name) in group_members(context).into_iter().filter(|(_, name)| name.text == target) {
        // Context and template can share owned declarations. In that case
        // their targets must follow the copied anchors, rather than retain
        // handles belonging to the uninstantiated input group.
        if let Some(binding) = name.declaration_binding().and_then(|old| instantiated.get(old)) {
            name = name.with_declaration_binding(binding.clone());
        }
        if target_members.get(&kind).is_some_and(|previous: &Name| previous.declaration_binding() != name.declaration_binding()) {
            return Err(Error::plain(format!("ambiguous owned declaration member `{target}`")));
        }
        target_members.insert(kind, name);
    }
    let mut mapping = HashMap::new();
    let mut selected = Vec::new();
    for (kind, name) in source_members {
        let target = target_members.get(&kind).ok_or_else(|| Error::plain(format!("`{target}` has no owned {kind:?} member for `{source}`")))?;
        let binding = name.declaration_binding().unwrap().clone();
        selected.push(binding.clone());
        mapping.insert((binding, kind), target.clone());
    }
    let original = group_members(&result);
    remove_members(&mut result, &selected);
    visit_names(&mut result, &mut |name, kind| {
        let Some(binding) = name.declaration_binding() else { return };
        let destination = mapping.get(&(binding.clone(), kind)).or_else(|| {
            // Export names can denote a type without a value of that name.
            let mut candidates = mapping.iter().filter(|((old, _), _)| old == binding).map(|(_, name)| name);
            let only = candidates.next()?;
            if candidates.next().is_none() { Some(only) } else { None }
        });
        if let Some(destination) = destination {
            let mut replacement = destination.with_declaration_binding(destination.declaration_binding().unwrap().clone());
            replacement.span = name.span;
            *name = replacement;
        }
    });
    let remaining = group_members(&result);
    let removed: Vec<_> = original.iter().filter(|(_, old)| !remaining.iter().any(|(_, new)| new.declaration_binding() == old.declaration_binding()))
        .filter_map(|(_, name)| name.declaration_binding().cloned()).collect();
    let mut dangling = None;
    visit_names(&mut result, &mut |name, _| {
        if name.declaration_binding().is_some_and(|binding| removed.contains(binding)) {
            dangling = Some(Error::at(name.span, format!("linking `{source}` leaves an unbound owned member `{}`", name.text)));
        }
    });
    if let Some(error) = dangling { return Err(error); }
    Ok(result)
}

/// Replace an owned opaque type slot with an already captured source tree.
/// The slot grants authority only over its own references; names inside the
/// replacement retain the caller's scope, including private nominal types.
pub(crate) fn specialize_type(source: &str, replacement: &Type, template: &[Decl]) -> Result<Vec<Decl>> {
    let mut result = template.to_vec();
    let instantiated = instantiate_group(&mut result);
    let mut replacement = replacement.clone();
    remap_type(&mut replacement, &instantiated);
    fn slots(decls: &[Decl], source: &str, found: &mut Vec<LocalBinding>) -> Result<()> {
        for decl in decls {
            match decl {
                Decl::Attributed { inner, .. } => slots(std::slice::from_ref(inner), source, found)?,
                Decl::OpaqueType(opaque) if opaque.name.text == source => {
                    if !opaque.params.is_empty() {
                        return Err(Error::at(opaque.span, "a declaration type slot must have no parameters"));
                    }
                    if let Some(binding) = opaque.name.declaration_binding() { found.push(binding.clone()); }
                }
                _ => {}
            }
        }
        Ok(())
    }
    let mut selected = Vec::new();
    slots(&result, source, &mut selected)?;
    if selected.is_empty() {
        return Err(Error::plain(format!("`{source}` is not an owned opaque type slot")));
    }
    remove_members(&mut result, &selected);
    for decl in &mut result {
        decl.visit_types_mut(&mut |ty| {
            if let Type::Con(path, _) = ty {
                if path.segments.last().and_then(Name::declaration_binding).is_some_and(|binding| selected.contains(binding)) {
                    *ty = replacement.clone();
                }
            }
        });
    }
    let mut dangling = None;
    visit_names(&mut result, &mut |name, _| {
        if name.declaration_binding().is_some_and(|binding| selected.contains(binding)) {
            dangling = Some(Error::at(name.span, "a specialized type slot cannot remain in a declaration name or export"));
        }
    });
    if let Some(error) = dangling { return Err(error); }
    Ok(result)
}

fn remap_type(ty: &mut Type, mapping: &HashMap<LocalBinding, LocalBinding>) {
    ty.visit_mut(&mut |node| type_node(node, &mut |name, _| {
        if let Some(binding) = name.declaration_binding().and_then(|old| mapping.get(old)) {
            *name = name.with_declaration_binding(binding.clone());
        }
    }));
}

pub(crate) fn member(decls: &[Decl], spelling: &str, kind: RefKind) -> Result<Name> {
    let mut selected: Option<Name> = None;
    for (_, name) in group_members(decls).into_iter().filter(|(namespace, name)| *namespace == kind && name.text == spelling) {
        if selected.as_ref().is_some_and(|old| old.declaration_binding() != name.declaration_binding()) {
            return Err(Error::plain(format!("ambiguous owned declaration member `{spelling}`")));
        }
        selected = Some(name);
    }
    selected.map(|name| name.with_declaration_binding(name.declaration_binding().unwrap().clone()))
        .ok_or_else(|| Error::plain(format!("`{spelling}` has no owned {kind:?} declaration member")))
}

pub(crate) fn signature_type(spelling: &str, group: &[Decl]) -> Result<Type> {
    let selected = member(group, spelling, RefKind::Value)?;
    fn find(decls: &[Decl], selected: &Name, result: &mut Option<Type>) -> Result<()> {
        for decl in decls {
            match decl {
                Decl::Attributed { inner, .. } => find(std::slice::from_ref(inner), selected, result)?,
                Decl::ValueSig { name, ty, .. } if name.declaration_binding() == selected.declaration_binding() => {
                    if result.is_some() { return Err(Error::at(name.span, "ambiguous owned declaration signature")); }
                    *result = Some(ty.clone());
                }
                _ => {}
            }
        }
        Ok(())
    }
    let mut result = None;
    find(group, &selected, &mut result)?;
    result.ok_or_else(|| Error::plain(format!("`{spelling}` has no owned value signature")))
}

/// Extend a possessed data declaration. The supplied tree retains captured
/// references; references shared with the group follow its fresh anchors.
pub(crate) fn add_constructor(data: &str, constructor: Name, ty: &Type, group: &[Decl]) -> Result<Vec<Decl>> {
    let selected = member(group, data, RefKind::Type)?;
    let mut result = group.to_vec();
    let instantiated = instantiate_group(&mut result);
    let selected = instantiated.get(selected.declaration_binding().unwrap()).unwrap();
    let mut ty = ty.clone();
    remap_type(&mut ty, &instantiated);
    fn add(decls: &mut [Decl], selected: &LocalBinding, constructor: &Name, ty: &Type) -> bool {
        for decl in decls {
            match decl {
                Decl::Attributed { inner, .. } => if add(std::slice::from_mut(inner), selected, constructor, ty) { return true; },
                Decl::Data(data) if data.name.declaration_binding() == Some(selected) => {
                    data.ctors.push(CtorDecl { attrs: Vec::new(), name: constructor.with_declaration_binding(LocalBinding::fresh()), ty: ty.clone(), span: constructor.span });
                    return true;
                }
                _ => {}
            }
        }
        false
    }
    if !add(&mut result, selected, &constructor, &ty) {
        return Err(Error::plain("Syntax.addConstructor requires an owned data declaration"));
    }
    Ok(result)
}

pub(crate) fn export_member(spelling: &str, group: &[Decl]) -> Result<Vec<Decl>> {
    let mut result = group.to_vec();
    freshen_group(&mut result);
    let selected = match group_members(&result).iter().find(|(kind, name)| *kind == RefKind::Type && name.text == spelling) {
        Some(_) => member(&result, spelling, RefKind::Type)?,
        None => member(&result, spelling, RefKind::Value)?,
    };
    result.push(Decl::Export(ExportDecl { span: selected.span, items: vec![ExportItem::Name(selected)] }));
    Ok(result)
}

/// Add native clauses to an owned signature, retaining every body/pattern
/// binding. A source signature cannot be silently discarded by this operation.
pub(crate) fn add_equations(source: &str, target: &str, context: &[Decl], addition: &[Decl]) -> Result<Vec<Decl>> {
    let selected = member(addition, source, RefKind::Value)?;
    let destination = member(context, target, RefKind::Value)?;
    fn inspect(decls: &[Decl], selected: &Name, equations: &mut usize, signature: &mut bool, other: &mut bool) {
        let same = |name: &Name| name.declaration_binding() == selected.declaration_binding();
        for decl in decls {
            match decl {
                Decl::Attributed { inner, .. } => inspect(std::slice::from_ref(inner), selected, equations, signature, other),
                Decl::ValueSig { name, .. } if same(name) => *signature = true,
                Decl::Equation { eq, .. } if same(&eq.name) => *equations += 1,
                Decl::Data(data) if data.ctors.iter().any(|ctor| same(&ctor.name)) => *other = true,
                Decl::Record(record) if same(&record.name) => *other = true,
                _ => {}
            }
        }
    }
    let (mut equations, mut signature, mut other) = (0, false, false);
    inspect(addition, &selected, &mut equations, &mut signature, &mut other);
    if signature { return Err(Error::plain("Syntax.addEquations cannot discard a source signature")); }
    if equations == 0 || other { return Err(Error::plain("Syntax.addEquations needs an equation-only source member")); }
    let (mut equations, mut signature, mut other) = (0, false, false);
    inspect(context, &destination, &mut equations, &mut signature, &mut other);
    if !signature || other { return Err(Error::plain("Syntax.addEquations requires an owned target signature")); }
    let mut addition = addition.to_vec();
    visit_names(&mut addition, &mut |name, kind| {
        if kind == RefKind::Value && name.declaration_binding() == selected.declaration_binding() {
            let span = name.span;
            *name = destination.with_declaration_binding(destination.declaration_binding().unwrap().clone());
            name.span = span;
        }
    });
    let mut result = context.to_vec();
    result.extend(addition);
    freshen_group(&mut result);
    Ok(result)
}

/// Turn possessed ordinary clauses into local code. The source signature is
/// retained as an annotation, and recursive uses acquire a fresh lexical
/// handle. Other captures and owned-group references retain their identities.
pub(crate) fn function_code(spelling: &str, group: &[Decl]) -> Result<Term> {
    let selected = member(group, spelling, RefKind::Value)?;
    fn collect(decls: &[Decl], selected: &Name, attributed: bool, result: &mut Vec<Decl>) -> Result<()> {
        for decl in decls {
            match decl {
                Decl::Attributed { inner, .. } => collect(std::slice::from_ref(inner), selected, true, result)?,
                Decl::ValueSig { name, modifier, owned_attrs, .. } if name.declaration_binding() == selected.declaration_binding() => {
                    if attributed { return Err(Error::at(name.span, "Syntax.functionCode cannot discard declaration attributes")); }
                    if modifier.is_some() || !owned_attrs.is_empty() { return Err(Error::at(name.span, "Syntax.functionCode requires ordinary function clauses")); }
                    result.push(decl.clone());
                }
                Decl::Equation { modifier, eq } if eq.name.declaration_binding() == selected.declaration_binding() => {
                    if attributed || !eq.attrs.is_empty() { return Err(Error::at(eq.span, "Syntax.functionCode cannot discard declaration attributes")); }
                    if modifier.is_some() { return Err(Error::at(eq.span, "Syntax.functionCode requires ordinary function clauses")); }
                    result.push(decl.clone());
                }
                _ => {}
            }
        }
        Ok(())
    }
    let mut definitions = Vec::new();
    collect(group, &selected, false, &mut definitions)?;
    let equation_count = definitions.iter().filter(|decl| matches!(decl, Decl::Equation { .. })).count();
    if equation_count == 0 { return Err(Error::plain("Syntax.functionCode needs owned function equations")); }
    let mut references = 0;
    visit_names(&mut definitions, &mut |name, kind| {
        if kind == RefKind::Value && name.declaration_binding() == selected.declaration_binding() { references += 1; }
    });
    // Each definition and the one signature contributes a binding occurrence.
    let recursive = references > equation_count + 1;
    let local = Name::bound(spelling, selected.span, LocalBinding::fresh());
    visit_names(&mut definitions, &mut |name, kind| {
        if kind == RefKind::Value && name.declaration_binding() == selected.declaration_binding() {
            let span = name.span;
            *name = local.clone();
            name.span = span;
        }
    });
    let mut signature = None;
    let mut equations = Vec::new();
    for decl in definitions {
        match decl {
            Decl::ValueSig { ty, .. } => {
                if signature.is_some() { return Err(Error::plain("ambiguous owned declaration signature")); }
                signature = Some(ty);
            }
            Decl::Equation { eq, .. } => equations.push(eq),
            _ => unreachable!(),
        }
    }
    let signature = signature.ok_or_else(|| Error::plain("Syntax.functionCode requires an owned signature"))?;
    let width = equations[0].pats.len();
    if equations.iter().any(|equation| equation.pats.len() != width) {
        return Err(Error::plain("function clauses must have the same number of patterns"));
    }
    let span = selected.span;
    let arguments: Vec<_> = (0..width).map(|index| Name::bound(format!("argument{index}"), span, LocalBinding::fresh())).collect();
    let terms: Vec<_> = arguments.iter().map(|name| Term::Var(VarPath { quals: Vec::new(), name: name.clone() })).collect();
    let scrutinee = match width {
        0 => Term::Unit(span), 1 => terms.into_iter().next().unwrap(), _ => Term::Tuple(terms, span),
    };
    let mut alternatives = Vec::new();
    for equation in equations {
        let body = if equation.locals.is_empty() { equation.body } else {
            Term::Let(equation.locals, Box::new(equation.body), equation.span)
        };
        let pat = match width {
            0 => Pat::Unit(equation.span), 1 => equation.pats.into_iter().next().unwrap(), _ => Pat::Tuple(equation.pats, equation.span),
        };
        alternatives.push(Alt { attrs: Vec::new(), pat, guard: None, guard_attrs: Vec::new(), body, span: equation.span });
    }
    let body = Term::Case(Box::new(scrutinee), alternatives, span);
    let patterns = arguments.into_iter().map(|name| Pat::Var(name.clone(), name.span)).collect::<Vec<_>>();
    if recursive {
        if matches!(signature, Type::Forall(..)) {
            // The enclosing universal annotation checks the recursive group
            // with rigid parameters before any concrete use is instantiated.
            // Its recursion stays monomorphic within that one checked body.
            return Ok(Term::Ann(Box::new(Term::Letrec(vec![
                LocalDecl::Eq { name: local.clone(), pats: patterns, body, span },
            ], Box::new(Term::Var(VarPath { quals: Vec::new(), name: local })), span)), signature, span));
        }
        Ok(Term::Letrec(vec![
            LocalDecl::Sig { name: local.clone(), ty: signature, span },
            LocalDecl::Eq { name: local.clone(), pats: patterns, body, span },
        ], Box::new(Term::Var(VarPath { quals: Vec::new(), name: local })), span))
    } else {
        let function = if width == 0 { body } else { Term::Lam(patterns, Box::new(body), span) };
        Ok(Term::Ann(Box::new(function), signature, span))
    }
}

fn remove_members(decls: &mut Vec<Decl>, selected: &[LocalBinding]) {
    let chosen = |name: &Name| name.declaration_binding().is_some_and(|binding| selected.contains(binding));
    decls.retain_mut(|declaration| {
        match declaration {
            Decl::Attributed { inner, .. } => {
                let mut nested = vec![(**inner).clone()];
                remove_members(&mut nested, selected);
                if let Some(declaration) = nested.pop() { **inner = declaration; true } else { false }
            }
            Decl::Data(data) => {
                if chosen(&data.name) { return false; }
                data.ctors.retain(|ctor| !chosen(&ctor.name));
                true
            }
            Decl::Record(record) | Decl::Codata(record) => !chosen(&record.name),
            Decl::OpaqueType(opaque) => !chosen(&opaque.name),
            Decl::Refine(refine) => !chosen(&refine.name),
            Decl::Foreign(foreign) => !chosen(&foreign.name),
            Decl::Prim(primitive) => !chosen(&primitive.name),
            Decl::Effect(effect) => {
                if chosen(&effect.name) { return false; }
                effect.ops.retain(|(name, _)| !chosen(name)); true
            }
            Decl::ValueSig { name, .. } => !chosen(name),
            Decl::Equation { eq, .. } => !chosen(&eq.name),
            _ => true,
        }
    });
}

// This walk changes no spelling unless its caller possesses that exact
// declaration handle. Field labels and ordinary binders grant no authority.
fn path(path: &mut ConPath, kind: RefKind, visit: &mut impl FnMut(&mut Name, RefKind)) {
    if let Some(name) = path.segments.last_mut() { visit(name, kind); }
}

fn type_node(ty: &mut Type, visit: &mut impl FnMut(&mut Name, RefKind)) {
    match ty {
        Type::AppRow(_, row, _) | Type::Fun { eff: Some(row), .. } | Type::DepFun(_, _, _, Some(row), _, _) => {
            for (label, _) in &mut row.labels { path(label, RefKind::Effect, visit); }
        }
        Type::Con(con, _) => path(con, RefKind::Type, visit),
        Type::Var(name, _) => visit(name, RefKind::Value),
        Type::Infix(_, ops, _) => for name in ops { visit(name, RefKind::Value); },
        Type::Using(obligations, _, _) => for obligation in obligations {
            match obligation {
                Oblig::Class(class) => path(&mut class.class, RefKind::Type, visit),
                Oblig::Preserves { method, .. } => visit(method, RefKind::Value),
                Oblig::Pred(_) => {}
            }
        },
        _ => {}
    }
}

pub(crate) fn visit_names(decls: &mut [Decl], visit: &mut impl FnMut(&mut Name, RefKind)) {
    for decl in decls {
        decl.visit_types_mut(&mut |ty| type_node(ty, visit));
        declaration(decl, visit);
    }
}

fn pattern(pat: &mut Pat, visit: &mut impl FnMut(&mut Name, RefKind)) {
    match pat {
        Pat::Con(con, pats, _) => {
            path(con, RefKind::Value, visit);
            for pat in pats { pattern(pat, visit); }
        }
        Pat::Record(con, fields, _) => {
            path(con, RefKind::Value, visit);
            for (_, pat) in fields { pattern(pat, visit); }
        }
        Pat::As(_, pat, _) => pattern(pat, visit),
        Pat::Tuple(pats, _) | Pat::List(pats, _) => for pat in pats { pattern(pat, visit); },
        Pat::Var(..) | Pat::Wild(..) | Pat::Int(..) | Pat::Sym(..) | Pat::Unit(..) => {}
    }
}

fn local(decl: &mut LocalDecl, visit: &mut impl FnMut(&mut Name, RefKind)) {
    match decl {
        LocalDecl::Sig { .. } => {}
        LocalDecl::Eq { pats, body, .. } => {
            for pat in pats { pattern(pat, visit); }
            term(body, visit);
        }
        LocalDecl::Bind { pat, body, .. } | LocalDecl::Lazy { pat, body, .. } => {
            pattern(pat, visit);
            term(body, visit);
        }
    }
}

fn statement(stmt: &mut DoStmt, visit: &mut impl FnMut(&mut Name, RefKind)) {
    match stmt {
        DoStmt::Bind { pat, term: body, .. } => { pattern(pat, visit); term(body, visit); }
        DoStmt::Expr { term: body, .. } => term(body, visit),
        DoStmt::Let { decls, .. } => for decl in decls { local(decl, visit); },
    }
}

fn equation(eq: &mut Equation, visit: &mut impl FnMut(&mut Name, RefKind)) {
    visit(&mut eq.name, RefKind::Value);
    for pat in &mut eq.pats { pattern(pat, visit); }
    for decl in &mut eq.locals { local(decl, visit); }
    term(&mut eq.body, visit);
}

fn term(body: &mut Term, visit: &mut impl FnMut(&mut Name, RefKind)) {
    match body {
        Term::Var(var) => visit(&mut var.name, RefKind::Value),
        Term::Con(con) => path(con, RefKind::Value, visit),
        Term::App(head, args, _) => { term(head, visit); for arg in args { term(arg, visit); } }
        Term::Lam(pats, body, _) => { for pat in pats { pattern(pat, visit); } term(body, visit); }
        Term::Ann(body, _, _) | Term::Neg(body, _) | Term::Proj(body, _, _)
        | Term::CpsWrap(body, _) | Term::Quote { body, .. } | Term::Splice { body, .. }
        | Term::Forget { value: body, .. } => term(body, visit),
        Term::Let(decls, body, _) | Term::Letrec(decls, body, _) => {
            for decl in decls { local(decl, visit); } term(body, visit);
        }
        Term::If { cond, then, els, .. } => { term(cond, visit); term(then, visit); term(els, visit); }
        Term::Case(scrutinee, alts, _) => {
            term(scrutinee, visit);
            for alt in alts {
                pattern(&mut alt.pat, visit);
                if let Some(guard) = &mut alt.guard { term(guard, visit); }
                term(&mut alt.body, visit);
            }
        }
        Term::Narrow { value, witness, .. } => { term(value, visit); term(witness, visit); }
        Term::Infix(terms, ops, _) => {
            for name in ops { visit(name, RefKind::Value); }
            for body in terms { term(body, visit); }
        }
        Term::Tuple(terms, _) | Term::List(terms, _) => for body in terms { term(body, visit); },
        Term::Map(entries, _) => for (key, value) in entries { term(key, visit); term(value, visit); },
        Term::Range(start, step, end, _) => {
            term(start, visit); if let Some(step) = step { term(step, visit); } term(end, visit);
        }
        Term::Record(con, fields, _) => {
            path(con, RefKind::Value, visit); for (_, value) in fields { term(value, visit); }
        }
        Term::Do(stmts, _) => for stmt in stmts { statement(stmt, visit); },
        Term::Handle(body, handler, _) => { term(body, visit); term(handler, visit); }
        Term::Handler(handler) => {
            path(&mut handler.effect, RefKind::Effect, visit);
            pattern(&mut handler.ret_pat, visit); term(&mut handler.ret_body, visit);
            for clause in &mut handler.clauses {
                visit(&mut clause.op, RefKind::Value);
                for pat in &mut clause.pats { pattern(pat, visit); }
                term(&mut clause.body, visit);
            }
        }
        Term::With { value, body, .. } => { term(value, visit); term(body, visit); }
        Term::CapApply { head, supplies, .. } => {
            term(head, visit); for (_, value) in supplies { term(value, visit); }
        }
        Term::QuoteDecls { decls, .. } => for decl in decls { declaration(decl, visit); },
        Term::MacroCall { path, args, .. } => {
            visit(&mut path.name, RefKind::Value);
            if let Some(names) = &mut args.lexical {
                for name in names { visit(name, RefKind::Value); }
            }
            if let Some(declarations) = &mut args.declarations {
                for (kind, name) in declarations { visit(name, *kind); }
            }
        }
        Term::Int(..) | Term::Str(..) | Term::Char(..) | Term::Sym(..) | Term::Unit(..)
        | Term::Hole(..) | Term::SeqOp(..) | Term::SpliceHole { .. } => {}
    }
}

fn declaration(decl: &mut Decl, visit: &mut impl FnMut(&mut Name, RefKind)) {
    match decl {
        Decl::Attributed { inner, .. } => declaration(inner, visit),
        Decl::NameSpliced { name_tm, inner } => { term(name_tm, visit); declaration(inner, visit); }
        Decl::ValueSig { name, .. } => visit(name, RefKind::Value),
        Decl::Equation { eq, .. } => equation(eq, visit),
        Decl::Data(data) => {
            visit(&mut data.name, RefKind::Type);
            for ctor in &mut data.ctors { visit(&mut ctor.name, RefKind::Value); }
        }
        Decl::Record(record) | Decl::Codata(record) => visit(&mut record.name, RefKind::Type),
        Decl::Signature(signature) => {
            visit(&mut signature.name, RefKind::Type);
            for (name, _) in signature.methods.iter_mut().chain(&mut signature.laws) { visit(name, RefKind::Value); }
            for eq in &mut signature.defaults { equation(eq, visit); }
            for decl in &mut signature.member_decls { declaration(decl, visit); }
        }
        Decl::Instance(instance) => {
            visit(&mut instance.name, RefKind::Value);
            for eq in &mut instance.methods { equation(eq, visit); }
        }
        Decl::Foreign(foreign) => visit(&mut foreign.name, RefKind::Value),
        Decl::OpaqueType(opaque) => visit(&mut opaque.name, RefKind::Type),
        Decl::Refine(refine) => visit(&mut refine.name, RefKind::Type),
        Decl::Effect(effect) => {
            visit(&mut effect.name, RefKind::Effect);
            for (name, _) in &mut effect.ops { visit(name, RefKind::Value); }
        }
        Decl::Export(export) => for item in &mut export.items {
            match item {
                ExportItem::Name(name) => visit(name, RefKind::Value),
                ExportItem::Member(con) => path(con, RefKind::Value, visit),
                ExportItem::Module(_) => {}
            }
        },
        Decl::Fixity(fixity) => for name in &mut fixity.ops { visit(name, RefKind::Value); },
        Decl::Generate { stmts, .. } => for stmt in stmts { statement(stmt, visit); },
        Decl::Rewrite(rewrite) => {
            term(&mut rewrite.match_tm, visit); term(&mut rewrite.replace_tm, visit);
            if let Some(require) = &mut rewrite.require { term(require, visit); }
            term(&mut rewrite.justify, visit);
        }
        Decl::Upgrade(upgrade) => {
            path(&mut upgrade.old, RefKind::Type, visit); path(&mut upgrade.new, RefKind::Type, visit);
            if let Some(body) = &mut upgrade.migrate { term(body, visit); }
            if let Some(body) = &mut upgrade.related { term(body, visit); }
        }
        Decl::Contract(contract) => {
            visit(&mut contract.target, RefKind::Value);
            for clause in &mut contract.clauses {
                match clause {
                    ContractClause::Formula { body, .. } | ContractClause::AssumeProp { body } => term(body, visit),
                    ContractClause::AssumeNamed { .. } => {}
                }
            }
        }
        Decl::Prim(prim) => {
            visit(&mut prim.name, RefKind::Value);
            for body in &mut prim.bodies {
                if let Some(body) = &mut body.when { term(body, visit); }
                if let Some(body) = &mut body.justify { term(body, visit); }
            }
        },
        Decl::Import(..) | Decl::ImportData(..) => {}
    }
}

#[cfg(test)]
mod owned_effect_tests {
    use super::*;

    #[test]
    fn effect_and_type_names_with_the_same_spelling_have_distinct_handles() {
        let span = Span { line: 1, col: 1 };
        let mut declarations = vec![
            Decl::Data(DataDecl { name: Name::new("Event", span), params: vec![], ctors: vec![], span }),
            Decl::Effect(EffectDecl { name: Name::new("Event", span), params: vec![], ops: vec![], span }),
        ];
        let members = bind_group(&mut declarations);
        let datatype = members.iter().find(|(kind, _)| *kind == RefKind::Type).unwrap().1.declaration_binding().unwrap();
        let effect = members.iter().find(|(kind, _)| *kind == RefKind::Effect).unwrap().1.declaration_binding().unwrap();
        assert_ne!(datatype, effect);
        let renamed = rename_group(&declarations, "Event", "Renamed").unwrap();
        let members = group_members(&renamed);
        assert_eq!(members.len(), 2);
        assert!(members.iter().all(|(_, name)| name.text == "Renamed"));
        assert_ne!(members[0].1.declaration_binding(), members[1].1.declaration_binding());
    }
}

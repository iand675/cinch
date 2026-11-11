// Canonical default clauses carry lexical binders and the defining identities
// selected by the checker. The importing compiler rechecks the body at the
// method contract.
use crate::interface::{JsonValue as DefaultJson, MethodDefault};

fn default_node(tag: &str, items: impl IntoIterator<Item = DefaultJson>) -> DefaultJson {
    DefaultJson::Arr(std::iter::once(tag.into()).chain(items).collect())
}

/// The identity spelling every interface, object and symbol name agrees on
/// (`bootstrap/Bootstrap.Nominal.cinch`): `[A-Za-z0-9_]` is itself, and every
/// other scalar is the lowercase two-digit hex of each of its UTF-8 bytes,
/// introduced by `$`. `$` is outside the plain class, so it spells `$24`.
/// `$` always starts an escape and a component separator is never inside an
/// escape, which makes the spelling injective and reversible.
fn encode_identity(identity: &str, encoded: &mut String) {
    use std::fmt::Write;
    for character in identity.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            encoded.push(character);
        } else {
            let mut buffer = [0; 4];
            for byte in character.encode_utf8(&mut buffer).as_bytes() {
                write!(encoded, "${byte:02x}").expect("writing to String cannot fail");
            }
        }
    }
}

fn default_global(identity: &str) -> String {
    let mut encoded = String::from("i.");
    encode_identity(identity, &mut encoded);
    encoded
}

impl Checker {
    fn portable_default_body(&self, equations: &[ast::Equation]) -> Option<MethodDefault> {
        let clauses = equations
            .iter()
            .map(|equation| {
                if !equation.attrs.is_empty() || !equation.locals.is_empty() || equation.guard.is_some() {
                    return None;
                }
                let mut locals = Vec::new();
                let patterns = equation
                    .pats
                    .iter()
                    .map(|pat| self.default_pattern(pat, &mut locals))
                    .collect::<Option<Vec<_>>>()?;
                let body = self.default_term(&equation.body, &locals)?;
                Some(DefaultJson::Arr(vec![DefaultJson::Arr(patterns), body]))
            })
            .collect::<Option<Vec<_>>>()?;
        MethodDefault::from_value(DefaultJson::Arr(clauses)).ok()
    }

    fn portable_law_syntax(&self, source: &ast::Type, checked: &Ty, binders: &[&str]) -> Option<DefaultJson> {
        match (source, checked) {
            (ast::Type::Forall(parameters, body, _), _) => {
                let mut scope = binders.to_vec();
                scope.extend(parameters.iter().map(|parameter| parameter.name.text.as_str()));
                let parameters = parameters.iter().map(|parameter| {
                    let kind = match &parameter.ann {
                        Some(annotation) => self.default_type(annotation)?,
                        None => DefaultJson::Null,
                    };
                    Some(DefaultJson::Arr(vec![
                        parameter.name.text.as_str().into(), DefaultJson::Bool(parameter.implicit), kind,
                    ]))
                }).collect::<Option<Vec<_>>>()?;
                Some(default_node("LawForall", [DefaultJson::Arr(parameters), self.portable_law_syntax(body, checked, &scope)?]))
            }
            (ast::Type::DepFun(name, carrier, None, None, body, _), Ty::Pi { var, cod, .. }) if var == &name.text => {
                let mut scope = binders.to_vec();
                scope.push(var);
                Some(default_node("LawArrow", [
                    var.as_str().into(), self.default_type(carrier)?, self.portable_law_syntax(body, cod, &scope)?,
                ]))
            }
            (ast::Type::Fun { arg, mult: None, eff: None, res, .. }, Ty::Fun(_, _, _, cod)) => {
                Some(default_node("LawArrow", [
                    DefaultJson::Null, self.default_type(arg)?, self.portable_law_syntax(res, cod, binders)?,
                ]))
            }
            (_, Ty::Prp(proposition)) => match proposition.as_ref() {
                PTerm::Eq(left, right) => Some(default_node("LawEquation", [
                    self.portable_law_operand(left, binders)?, self.portable_law_operand(right, binders)?,
                ])),
                _ => None,
            },
            _ => None,
        }
    }

    fn portable_law_operand(&self, operand: &ITerm, binders: &[&str]) -> Option<DefaultJson> {
        match operand {
            ITerm::Var(name) if binders.contains(&name.as_str()) =>
                Some(default_node("EVar", [name.as_str().into()])),
            ITerm::Num(value) => Some(default_node("ELitInt", [value.as_str().into()])),
            ITerm::Fn(name) => self.portable_law_global(name),
            ITerm::Inst(call) if call.arguments.is_empty() => match &call.head {
                IndexHead::Function(name) => self.portable_law_global(name),
                _ => None,
            },
            ITerm::App(function, arguments) => {
                let mut body = self.portable_law_operand(function, binders)?;
                for argument in arguments {
                    body = default_node("EApp", [body, self.portable_law_operand(argument, binders)?]);
                }
                Some(body)
            }
            ITerm::Lam(name, body) if !name.contains('.') => {
                let mut scope = binders.to_vec();
                scope.push(name);
                let patterns = DefaultJson::Arr(vec![default_node("PVar", [name.as_str().into()])]);
                Some(default_node("ELam", [patterns, self.portable_law_operand(body, &scope)?]))
            }
            _ => None,
        }
    }

    fn portable_law_global(&self, identity: &str) -> Option<DefaultJson> {
        let (owner, name) = identity.rsplit_once('.')?;
        let value = self.qualified.get(&(owner.to_string(), name.to_string()))?.clone();
        let constructor = matches!(value.kind, ValueKind::Ctor { .. } | ValueKind::BoolLit(_));
        let name = self.default_value(value)?;
        if constructor {
            Some(default_node("ECon", [name.into(), default_node("EArgsEnd", [])]))
        } else {
            Some(default_node("EVar", [name.into()]))
        }
    }

    fn default_value(&self, value: ValueInfo) -> Option<String> {
        match value.kind {
            ValueKind::BoolLit(truth) => Some(if truth { "True" } else { "False" }.into()),
            ValueKind::Method { idx } => {
                let (_, class) = value.scheme.ctx.first()?;
                let signature = self.qualified_sigs.values().find(|signature| {
                    signature.module == value.module
                        && self.signature_key(&signature.module, &signature.name) == class.class
                        && signature
                            .methods
                            .get(idx)
                            .is_some_and(|(name, _)| name == &value.name)
                })?;
                Some(default_global(&format!(
                    "{}.{}.{}",
                    signature.module, signature.name, value.name
                )))
            }
            ValueKind::TopFn
            | ValueKind::Ctor { .. }
            | ValueKind::Foreign { .. }
            | ValueKind::Prim(_)
                if value.module != "<builtin>" =>
            {
                Some(default_global(&format!("{}.{}", value.module, value.name)))
            }
            _ => None,
        }
    }

    fn default_pattern<'a>(
        &self,
        pattern: &'a ast::Pat,
        locals: &mut Vec<&'a str>,
    ) -> Option<DefaultJson> {
        match pattern {
            ast::Pat::Var(name, _) => {
                if name.local_binding().is_some() || name.text.contains('.') {
                    return None;
                }
                locals.push(&name.text);
                Some(default_node("PVar", [name.text.as_str().into()]))
            }
            ast::Pat::Wild(_) => Some(default_node("PWild", [])),
            ast::Pat::Int(text, _) => Some(default_node("PLitInt", [text.into()])),
            ast::Pat::As(name, inner, _)
                if name.local_binding().is_none() && !name.text.contains('.') =>
            {
                locals.push(&name.text);
                Some(default_node(
                    "PAs",
                    [(&name.text).into(), self.default_pattern(inner, locals)?],
                ))
            }
            ast::Pat::Con(path, fields, span) => {
                let name = self.default_value(self.resolve_con(path, *span).ok()?)?;
                let mut tail = default_node("PArgsEnd", []);
                for field in fields.iter().rev() {
                    tail = default_node("PArg", [self.default_pattern(field, locals)?, tail]);
                }
                Some(default_node("PCon", [name.into(), tail]))
            }
            _ => None,
        }
    }

    fn default_term(&self, term: &ast::Term, locals: &[&str]) -> Option<DefaultJson> {
        match term {
            ast::Term::Var(path) => {
                if path.name.local_binding().is_some() {
                    return None;
                }
                let unqualified_local = path.quals.is_empty()
                    && path.name.resolved_binding().is_none()
                    && path.name.declaration_binding().is_none()
                    && path.name.scoped_reference().is_none()
                    && locals.contains(&path.name.text.as_str());
                let name = if unqualified_local {
                    path.name.text.clone()
                } else {
                    self.default_value(self.resolve_var(path, path.span()).ok()?)?
                };
                Some(default_node("EVar", [name.into()]))
            }
            ast::Term::Con(path) => {
                let name = self.default_value(self.resolve_con(path, path.span()).ok()?)?;
                Some(default_node(
                    "ECon",
                    [name.into(), default_node("EArgsEnd", [])],
                ))
            }
            ast::Term::Unit(_) => Some(default_node("EUnit", [])),
            ast::Term::Int(text, _) => Some(default_node("ELitInt", [text.into()])),
            ast::Term::Neg(value, _) => {
                Some(default_node("ENeg", [self.default_term(value, locals)?]))
            }
            ast::Term::Ann(value, annotation, _) => Some(default_node(
                "EAnn",
                [
                    self.default_term(value, locals)?,
                    self.default_type(annotation)?,
                ],
            )),
            ast::Term::Lam(patterns, body, _) => {
                let mut scope = locals.to_vec();
                let patterns = patterns
                    .iter()
                    .map(|pattern| self.default_pattern(pattern, &mut scope))
                    .collect::<Option<Vec<_>>>()?;
                Some(default_node(
                    "ELam",
                    [DefaultJson::Arr(patterns), self.default_term(body, &scope)?],
                ))
            }
            ast::Term::App(function, arguments, _) => {
                let mut result = self.default_term(function, locals)?;
                for argument in arguments {
                    result = default_node("EApp", [result, self.default_term(argument, locals)?]);
                }
                Some(result)
            }
            ast::Term::Case(subject, alternatives, _) => {
                let mut tail = default_node("EAltsEnd", []);
                for alternative in alternatives.iter().rev() {
                    if !alternative.attrs.is_empty() || alternative.guard.is_some() {
                        return None;
                    }
                    let mut scope = locals.to_vec();
                    let pattern = self.default_pattern(&alternative.pat, &mut scope)?;
                    let body = self.default_term(&alternative.body, &scope)?;
                    tail = default_node("EAlt", [pattern, body, tail]);
                }
                Some(default_node(
                    "ECase",
                    [self.default_term(subject, locals)?, tail],
                ))
            }
            ast::Term::If {
                cond,
                then,
                els,
                cond_attrs,
                then_attrs,
                else_attrs,
                ..
            } if cond_attrs.is_empty() && then_attrs.is_empty() && else_attrs.is_empty() => {
                Some(default_node(
                    "EIf",
                    [
                        self.default_term(cond, locals)?,
                        self.default_term(then, locals)?,
                        self.default_term(els, locals)?,
                    ],
                ))
            }
            _ => None,
        }
    }

    fn default_type(&self, annotation: &ast::Type) -> Option<DefaultJson> {
        match annotation {
            ast::Type::Var(name, _) => Some(default_node("TName", [(&name.text).into()])),
            ast::Type::Con(path, _) => {
                let kind = self.lookup_type_path(path).ok()?;
                let identity = if kind.module == "<builtin>" {
                    kind.name
                } else {
                    default_global(&kind.identity)
                };
                Some(default_node("TName", [identity.into()]))
            }
            ast::Type::Unit(_) => Some(default_node("TName", ["Unit".into()])),
            ast::Type::App(function, argument, _) => Some(default_node(
                "TApp",
                [
                    self.default_type(function)?,
                    default_node(
                        "TArg",
                        [self.default_type(argument)?, default_node("TArgsEnd", [])],
                    ),
                ],
            )),
            ast::Type::Fun {
                arg,
                res,
                mult: None,
                eff: None,
                ..
            } => Some(default_node(
                "TFun",
                [self.default_type(arg)?, self.default_type(res)?],
            )),
            _ => None,
        }
    }
}

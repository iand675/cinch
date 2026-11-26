// First-class function promotion and zero-cost `Sing` type helpers for `Checker`.
//
// Ordinary pure functions are compiled once into compact `PExpr` bodies for
// native Rust compile-time evaluation (`crate::promote`), eliminating the need
// for duplicate type families, defunctionalization symbols, or `sFoo` mirrors.

impl Checker {
    /// Attempt to compile an ordinary top-level Cinch function into a
    /// `PromotedFn` and register it on `self.uni`. If the function is impure
    /// or uses non-promotable constructs, records the exact diagnostic reason
    /// in `self.promoted_rejections` so any type-level use reports why.
    fn try_register_promoted_fn(
        &mut self,
        module: &str,
        name: &str,
        scheme: &Scheme,
        eqs: &[ast::Equation],
    ) {
        let identity = format!("{module}.{name}");
        // Do not overwrite native prelude fast paths if already built in.
        if self.uni.promoted_fn(&identity).is_some() {
            return;
        }
        if eqs.is_empty() {
            return;
        }
        // Check purity and unroll curried function arrows.
        let mut param_tys = Vec::new();
        let mut cur = &scheme.ty;
        loop {
            match cur {
                Ty::Fun(dom, _, eff, cod) => {
                    if !eff.is_pure() {
                        self.promoted_rejections.insert(
                            identity,
                            format!("function performs effect `{}`", self.display_eff(eff)),
                        );
                        return;
                    }
                    param_tys.push((**dom).clone());
                    cur = cod;
                }
                Ty::Pi { dom, eff, cod, .. } => {
                    if !eff.is_pure() {
                        self.promoted_rejections.insert(
                            identity,
                            format!("function performs effect `{}`", self.display_eff(eff)),
                        );
                        return;
                    }
                    param_tys.push((**dom).clone());
                    cur = cod;
                }
                _ => break,
            }
        }
        let ret_ty = cur.clone();
        if matches!(ret_ty, Ty::Prp(_) | Ty::Sort(_)) {
            return;
        }
        if !scheme.ctx.is_empty() {
            self.promoted_rejections.insert(
                identity,
                "typeclass-constrained functions are not yet promotable to the type level".into(),
            );
            return;
        }
        // Ensure every equation has the same pattern arity <= param_tys.len().
        let pat_arity = eqs[0].pats.len();
        if eqs.iter().any(|e| e.pats.len() != pat_arity) || pat_arity > param_tys.len() {
            return;
        }
        let effective_param_tys = param_tys[..pat_arity].to_vec();
        let effective_ret_ty = if pat_arity == param_tys.len() {
            ret_ty.clone()
        } else {
            let mut r = ret_ty.clone();
            for p in param_tys[pat_arity..].iter().rev() {
                r = Ty::fun(p.clone(), r);
            }
            r
        };
        let returns_idx = matches!(
            effective_ret_ty,
            Ty::Nat | Ty::Int | Ty::Prim(_)
        );

        match self.compile_promoted_equations(module, eqs, pat_arity) {
            Ok(body) => {
                self.promoted_rejections.remove(&identity);
                self.uni.register_promoted_fn(crate::promote::PromotedFn {
                    identity,
                    type_params: scheme.params.clone(),
                    param_tys: effective_param_tys,
                    ret_ty: effective_ret_ty,
                    returns_idx,
                    body: crate::promote::PromotedBody::Expr(std::sync::Arc::new(body)),
                });
            }
            Err(reason) => {
                self.promoted_rejections.insert(identity, reason);
            }
        }
    }

    fn display_eff(&self, eff: &EffRow) -> String {
        if eff.labels.is_empty() {
            "effect".to_string()
        } else {
            eff.labels
                .iter()
                .map(|l| l.head.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    fn compile_promoted_equations(
        &self,
        module: &str,
        eqs: &[ast::Equation],
        arity: usize,
    ) -> std::result::Result<crate::promote::PExpr, String> {
        use crate::promote::{PCaseArm, PExpr, PPat};

        // Fast path: single unguarded equation with simple variable/wildcard patterns.
        if eqs.len() == 1
            && eqs[0].guard.is_none()
            && eqs[0]
                .pats
                .iter()
                .all(|p| matches!(p, ast::Pat::Var(..) | ast::Pat::Wild(_)))
        {
            let mut locals = Vec::with_capacity(arity);
            for (i, p) in eqs[0].pats.iter().enumerate() {
                match p {
                    ast::Pat::Var(n, _) => locals.push(n.text.clone()),
                    ast::Pat::Wild(_) => locals.push(format!("$_wild_{i}")),
                    _ => unreachable!(),
                }
            }
            return self.compile_promoted_body_with_locals(
                module,
                &eqs[0].locals,
                &eqs[0].body,
                &mut locals,
            );
        }

        // General path: compile multiple equations / structural patterns into a `Case`.
        let mut base_locals = Vec::with_capacity(arity);
        for i in 0..arity {
            base_locals.push(format!("$_arg_{i}"));
        }
        let scrut = if arity == 0 {
            PExpr::Unit
        } else if arity == 1 {
            PExpr::Var(0)
        } else {
            PExpr::Tuple((0..arity).map(PExpr::Var).collect())
        };
        let mut arms = Vec::with_capacity(eqs.len());
        for eq in eqs {
            let mut locals = base_locals.clone();
            let pat = if arity == 0 {
                PPat::Unit
            } else if arity == 1 {
                self.compile_promoted_pat(module, &eq.pats[0], &mut locals)?
            } else {
                let mut ps = Vec::with_capacity(arity);
                for p in &eq.pats {
                    ps.push(self.compile_promoted_pat(module, p, &mut locals)?);
                }
                PPat::Tuple(ps)
            };
            let guard = match &eq.guard {
                Some(g) => Some(self.compile_promoted_term(module, g, &mut locals)?),
                None => None,
            };
            let body =
                self.compile_promoted_body_with_locals(module, &eq.locals, &eq.body, &mut locals)?;
            arms.push(PCaseArm { pat, guard, body });
        }
        Ok(PExpr::Case {
            scrut: Box::new(scrut),
            arms,
        })
    }

    fn compile_promoted_body_with_locals(
        &self,
        module: &str,
        decls: &[ast::LocalDecl],
        body: &ast::Term,
        locals: &mut Vec<String>,
    ) -> std::result::Result<crate::promote::PExpr, String> {
        use crate::promote::{PCaseArm, PExpr};
        if decls.is_empty() {
            return self.compile_promoted_term(module, body, locals);
        }
        let first = &decls[0];
        let rest = &decls[1..];
        match first {
            ast::LocalDecl::Sig { .. } => {
                self.compile_promoted_body_with_locals(module, rest, body, locals)
            }
            ast::LocalDecl::Eq { name, pats, body: rhs, .. } => {
                let val_expr = if pats.is_empty() {
                    self.compile_promoted_term(module, rhs, locals)?
                } else {
                    self.compile_promoted_lambda(module, pats, rhs, locals)?
                };
                locals.push(name.text.clone());
                let inner = self.compile_promoted_body_with_locals(module, rest, body, locals);
                locals.pop();
                Ok(PExpr::Let {
                    value: Box::new(val_expr),
                    body: Box::new(inner?),
                })
            }
            ast::LocalDecl::Bind { pat, body: rhs, .. }
            | ast::LocalDecl::Lazy { pat, body: rhs, .. } => {
                let val_expr = self.compile_promoted_term(module, rhs, locals)?;
                if let ast::Pat::Var(n, _) = pat {
                    locals.push(n.text.clone());
                    let inner = self.compile_promoted_body_with_locals(module, rest, body, locals);
                    locals.pop();
                    Ok(PExpr::Let {
                        value: Box::new(val_expr),
                        body: Box::new(inner?),
                    })
                } else {
                    let saved = locals.len();
                    let ppat = self.compile_promoted_pat(module, pat, locals)?;
                    let inner = self.compile_promoted_body_with_locals(module, rest, body, locals);
                    locals.truncate(saved);
                    Ok(PExpr::Case {
                        scrut: Box::new(val_expr),
                        arms: vec![PCaseArm {
                            pat: ppat,
                            guard: None,
                            body: inner?,
                        }],
                    })
                }
            }
        }
    }

    fn compile_promoted_lambda(
        &self,
        module: &str,
        pats: &[ast::Pat],
        body: &ast::Term,
        locals: &mut Vec<String>,
    ) -> std::result::Result<crate::promote::PExpr, String> {
        use crate::promote::{PCaseArm, PExpr, PPat};
        let saved = locals.len();
        let simple = pats
            .iter()
            .all(|p| matches!(p, ast::Pat::Var(..) | ast::Pat::Wild(_)));
        if simple {
            for (i, p) in pats.iter().enumerate() {
                match p {
                    ast::Pat::Var(n, _) => locals.push(n.text.clone()),
                    ast::Pat::Wild(_) => locals.push(format!("$_lwild_{i}")),
                    _ => unreachable!(),
                }
            }
            let b = self.compile_promoted_term(module, body, locals);
            locals.truncate(saved);
            Ok(PExpr::Lam {
                arity: pats.len(),
                body: std::sync::Arc::new(b?),
            })
        } else {
            let start = locals.len();
            for i in 0..pats.len() {
                locals.push(format!("$_larg_{i}"));
            }
            let scrut = if pats.len() == 1 {
                PExpr::Var(start)
            } else {
                PExpr::Tuple((start..start + pats.len()).map(PExpr::Var).collect())
            };
            let pat = if pats.len() == 1 {
                self.compile_promoted_pat(module, &pats[0], locals)?
            } else {
                let mut ps = Vec::with_capacity(pats.len());
                for p in pats {
                    ps.push(self.compile_promoted_pat(module, p, locals)?);
                }
                PPat::Tuple(ps)
            };
            let b = self.compile_promoted_term(module, body, locals);
            locals.truncate(saved);
            Ok(PExpr::Lam {
                arity: pats.len(),
                body: std::sync::Arc::new(PExpr::Case {
                    scrut: Box::new(scrut),
                    arms: vec![PCaseArm {
                        pat,
                        guard: None,
                        body: b?,
                    }],
                }),
            })
        }
    }

    fn compile_promoted_pat(
        &self,
        module: &str,
        pat: &ast::Pat,
        locals: &mut Vec<String>,
    ) -> std::result::Result<crate::promote::PPat, String> {
        use crate::promote::PPat;
        match pat {
            ast::Pat::Wild(_) => Ok(PPat::Wild),
            ast::Pat::Var(n, _) => {
                locals.push(n.text.clone());
                Ok(PPat::Bind)
            }
            ast::Pat::As(n, inner, _) => {
                locals.push(n.text.clone());
                let p = self.compile_promoted_pat(module, inner, locals)?;
                Ok(PPat::As(Box::new(p)))
            }
            ast::Pat::Unit(_) => Ok(PPat::Unit),
            ast::Pat::Int(s, _) => {
                if s.starts_with('-') {
                    s.parse::<i128>()
                        .map(PPat::Int)
                        .map_err(|_| format!("integer literal `{s}` out of range for promotion"))
                } else {
                    s.parse::<u128>()
                        .map(PPat::Nat)
                        .map_err(|_| format!("natural literal `{s}` out of range for promotion"))
                }
            }
            ast::Pat::Tuple(items, _) => {
                let mut ps = Vec::with_capacity(items.len());
                for item in items {
                    ps.push(self.compile_promoted_pat(module, item, locals)?);
                }
                Ok(PPat::Tuple(ps))
            }
            ast::Pat::List(items, _) => {
                let mut acc = PPat::Ctor {
                    identity: crate::types::PROMOTED_NIL.to_string(),
                    args: Vec::new(),
                };
                // Bind left-to-right! Build pattern list first in forward order.
                let mut compiled = Vec::with_capacity(items.len());
                for item in items {
                    compiled.push(self.compile_promoted_pat(module, item, locals)?);
                }
                for item in compiled.into_iter().rev() {
                    acc = PPat::Ctor {
                        identity: crate::types::PROMOTED_CONS.to_string(),
                        args: vec![item, acc],
                    };
                }
                Ok(acc)
            }
            ast::Pat::Con(cp, args, _) => {
                let parts = cp.parts();
                if parts.as_slice() == ["True"] && args.is_empty() {
                    return Ok(PPat::Bool(true));
                }
                if parts.as_slice() == ["False"] && args.is_empty() {
                    return Ok(PPat::Bool(false));
                }
                let ci = self
                    .lookup_promoted_ctor_path(cp)
                    .ok_or_else(|| format!("unknown constructor `{}` in promoted pattern", parts.join(".")))?;
                let mut ps = Vec::with_capacity(args.len());
                for a in args {
                    ps.push(self.compile_promoted_pat(module, a, locals)?);
                }
                Ok(PPat::Ctor {
                    identity: ci.identity,
                    args: ps,
                })
            }
            ast::Pat::Cons(head, tail, _) => {
                let h = self.compile_promoted_pat(module, head, locals)?;
                let t = self.compile_promoted_pat(module, tail, locals)?;
                Ok(PPat::Ctor {
                    identity: crate::types::PROMOTED_CONS.to_string(),
                    args: vec![h, t],
                })
            }
            ast::Pat::Str(_, _) => Err("string patterns are not supported in promoted functions".into()),
            ast::Pat::Sym(_, _) => Err("symbol patterns are not supported in promoted functions".into()),
            ast::Pat::Record(_, _, _) => {
                Err("record field patterns are not yet supported in promoted functions".into())
            }
        }
    }

    fn promoted_binop_of(name: &str) -> Option<crate::promote::PBinOp> {
        use crate::promote::PBinOp;
        Some(match name {
            "+" | "nat-add" | "int-add" => PBinOp::Add,
            "-" | "nat-sub" | "int-sub" | "checked-sub" => PBinOp::Sub,
            "*" | "nat-mul" | "int-mul" => PBinOp::Mul,
            "/" | "nat-div" | "int-div" => PBinOp::Div,
            "%" | "nat-rem" | "int-rem" => PBinOp::Rem,
            "==" | "string-eq" => PBinOp::Eq,
            "/=" => PBinOp::Ne,
            "<" => PBinOp::Lt,
            "<=" => PBinOp::Le,
            ">" => PBinOp::Gt,
            ">=" => PBinOp::Ge,
            "&&" => PBinOp::And,
            "||" => PBinOp::Or,
            "string-concat" => PBinOp::StrConcat,
            "++" => PBinOp::ListAppend,
            _ => return None,
        })
    }

    fn compile_promoted_term(
        &self,
        module: &str,
        term: &ast::Term,
        locals: &mut Vec<String>,
    ) -> std::result::Result<crate::promote::PExpr, String> {
        use crate::promote::{NativeBuiltin, PCaseArm, PExpr, PUnaryOp};
        match term {
            ast::Term::Unit(_) => Ok(PExpr::Unit),
            ast::Term::Int(s, _) => {
                if s.starts_with('-') {
                    s.parse::<i128>()
                        .map(PExpr::Int)
                        .map_err(|_| format!("integer literal `{s}` out of range"))
                } else {
                    s.parse::<u128>()
                        .map(PExpr::Nat)
                        .map_err(|_| format!("natural literal `{s}` out of range"))
                }
            }
            ast::Term::Str(s, _) => Ok(PExpr::Str(s.clone())),
            ast::Term::Neg(inner, _) => Ok(PExpr::UnaryOp {
                op: PUnaryOp::Neg,
                arg: Box::new(self.compile_promoted_term(module, inner, locals)?),
            }),
            ast::Term::Ann(inner, _, _) => self.compile_promoted_term(module, inner, locals),
            ast::Term::Tuple(items, _) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(self.compile_promoted_term(module, item, locals)?);
                }
                Ok(PExpr::Tuple(out))
            }
            ast::Term::List(items, _) => {
                let mut acc = PExpr::Ctor {
                    identity: crate::types::PROMOTED_NIL.to_string(),
                    args: Vec::new(),
                };
                for item in items.iter().rev() {
                    let e = self.compile_promoted_term(module, item, locals)?;
                    acc = PExpr::Ctor {
                        identity: crate::types::PROMOTED_CONS.to_string(),
                        args: vec![e, acc],
                    };
                }
                Ok(acc)
            }
            ast::Term::Con(cp) => {
                let parts = cp.parts();
                if parts.as_slice() == ["True"] {
                    return Ok(PExpr::Bool(true));
                }
                if parts.as_slice() == ["False"] {
                    return Ok(PExpr::Bool(false));
                }
                let ci = self
                    .lookup_promoted_ctor_path(cp)
                    .ok_or_else(|| format!("unknown constructor `{}`", parts.join(".")))?;
                if ci.field_tys.is_empty() {
                    Ok(PExpr::Ctor {
                        identity: ci.identity,
                        args: Vec::new(),
                    })
                } else {
                    // Unapplied constructor used as a first-class function value:
                    // eta-expand into a tiny closure `\a0 .. ak -> Ctor(a0..ak)`.
                    let arity = ci.field_tys.len();
                    let base = locals.len();
                    let body = PExpr::Ctor {
                        identity: ci.identity,
                        args: (base..base + arity).map(PExpr::Var).collect(),
                    };
                    Ok(PExpr::Lam {
                        arity,
                        body: std::sync::Arc::new(body),
                    })
                }
            }
            ast::Term::Var(vp) => {
                let name = &vp.name.text;
                if vp.quals.is_empty() {
                    if let Some(idx) = locals.iter().rposition(|l| l == name) {
                        return Ok(PExpr::Var(idx));
                    }
                    if name == "not" {
                        return Ok(PExpr::Native(NativeBuiltin::Not));
                    }
                    if let Some(op) = Self::promoted_binop_of(name) {
                        return Ok(PExpr::Native(NativeBuiltin::BinOp(op)));
                    }
                }
                let qual_str = if vp.quals.is_empty() {
                    None
                } else {
                    Some(
                        vp.quals
                            .iter()
                            .map(|q| q.text.as_str())
                            .collect::<Vec<_>>()
                            .join("."),
                    )
                };
                if let Ok(v) = self.lookup_value(qual_str.as_deref(), name) {
                    if matches!(v.kind, ValueKind::EffectOp { .. }) {
                        return Err(format!("calls effect operation `{name}`"));
                    }
                    let id = format!("{}.{}", v.module, v.name);
                    if let Some(reason) = self.promoted_rejections.get(&id) {
                        return Err(format!("calls unpromotable function `{name}` ({reason})"));
                    }
                    return Ok(PExpr::FnRef(id));
                }
                // Forward reference to a same-module function whose header has not been
                // registered yet; its canonical identity is `<module>.<name>`.
                if vp.quals.is_empty() {
                    return Ok(PExpr::FnRef(format!("{module}.{name}")));
                }
                Err(format!("unresolved function `{name}`"))
            }
            ast::Term::App(head, args, _) => {
                // Flatten nested curried applications: `((f a) b)` -> `(f, [a, b])`.
                let mut all_args: Vec<&ast::Term> = Vec::new();
                let mut cur_head: &ast::Term = head;
                all_args.extend(args.iter());
                while let ast::Term::App(inner_h, inner_a, _) = cur_head {
                    let mut next = Vec::with_capacity(inner_a.len() + all_args.len());
                    next.extend(inner_a.iter());
                    next.extend(all_args);
                    all_args = next;
                    cur_head = inner_h;
                }
                // Direct binary operator fast path!
                if let ast::Term::Var(vp) = cur_head {
                    if vp.quals.is_empty() && locals.iter().all(|l| l != &vp.name.text) {
                        if vp.name.text == "not" && all_args.len() == 1 {
                            return Ok(PExpr::UnaryOp {
                                op: PUnaryOp::Not,
                                arg: Box::new(self.compile_promoted_term(
                                    module,
                                    all_args[0],
                                    locals,
                                )?),
                            });
                        }
                        if let Some(op) = Self::promoted_binop_of(&vp.name.text) {
                            if all_args.len() == 2 {
                                let l = self.compile_promoted_term(module, all_args[0], locals)?;
                                let r = self.compile_promoted_term(module, all_args[1], locals)?;
                                return Ok(PExpr::BinOp {
                                    op,
                                    lhs: Box::new(l),
                                    rhs: Box::new(r),
                                });
                            }
                        }
                    }
                }
                // Direct constructor application!
                if let ast::Term::Con(cp) = cur_head {
                    let ci = self
                        .lookup_promoted_ctor_path(cp)
                        .ok_or_else(|| format!("unknown constructor `{}`", cp.parts().join(".")))?;
                    if all_args.len() == ci.field_tys.len() {
                        let mut cargs = Vec::with_capacity(all_args.len());
                        for a in all_args {
                            cargs.push(self.compile_promoted_term(module, a, locals)?);
                        }
                        return Ok(PExpr::Ctor {
                            identity: ci.identity,
                            args: cargs,
                        });
                    }
                }
                let f = self.compile_promoted_term(module, cur_head, locals)?;
                let mut cargs = Vec::with_capacity(all_args.len());
                for a in all_args {
                    cargs.push(self.compile_promoted_term(module, a, locals)?);
                }
                Ok(PExpr::App {
                    func: Box::new(f),
                    args: cargs,
                })
            }
            ast::Term::If { cond, then, els, .. } => Ok(PExpr::If {
                cond: Box::new(self.compile_promoted_term(module, cond, locals)?),
                then_br: Box::new(self.compile_promoted_term(module, then, locals)?),
                else_br: Box::new(self.compile_promoted_term(module, els, locals)?),
            }),
            ast::Term::Case(scrut, alts, _) => {
                let s = self.compile_promoted_term(module, scrut, locals)?;
                let mut arms = Vec::with_capacity(alts.len());
                for alt in alts {
                    let saved = locals.len();
                    let pat = self.compile_promoted_pat(module, &alt.pat, locals)?;
                    let guard = match &alt.guard {
                        Some(g) => Some(self.compile_promoted_term(module, g, locals)?),
                        None => None,
                    };
                    let body = self.compile_promoted_term(module, &alt.body, locals)?;
                    locals.truncate(saved);
                    arms.push(PCaseArm { pat, guard, body });
                }
                Ok(PExpr::Case {
                    scrut: Box::new(s),
                    arms,
                })
            }
            ast::Term::Lam(pats, body, _) => self.compile_promoted_lambda(module, pats, body, locals),
            ast::Term::Let(decls, body, _) => {
                self.compile_promoted_body_with_locals(module, decls, body, locals)
            }
            ast::Term::Do(_, _) => Err("`do` effect blocks cannot be promoted to the type level".into()),
            ast::Term::Handle(_, _, _) | ast::Term::Handler(_) => {
                Err("effect handlers cannot be promoted to the type level".into())
            }
            ast::Term::With { .. } => {
                Err("capability `with` blocks cannot be promoted to the type level".into())
            }
            ast::Term::Float(_, _) => {
                Err("floating-point literals cannot be promoted to exact type-level values".into())
            }
            _ => Err("expression form is not promotable to the type level".into()),
        }
    }

    /// Look up a promoted function by bare or qualified name. Returns `Err` if
    /// the name refers to an existing function that was explicitly rejected
    /// from promotion (e.g., because it performs `IO` or calls a foreign FFI).
    fn lookup_promoted_fn(
        &self,
        qual: Option<&str>,
        name: &str,
        span: ast::Span,
    ) -> Result<Option<crate::promote::PromotedFn>> {
        let Ok(v) = self.lookup_value(qual, name) else {
            return Ok(None);
        };
        if matches!(v.kind, ValueKind::EffectOp { .. }) {
            return Err(Error::at(
                span,
                format!("cannot promote effect operation `{name}` to the type level"),
            ));
        }
        let id = format!("{}.{}", v.module, v.name);
        if let Some(pfn) = self.uni.promoted_fn(&id) {
            return Ok(Some(pfn.clone()));
        }
        if let Some(reason) = self.promoted_rejections.get(&id) {
            return Err(Error::at(
                span,
                format!("cannot promote function `{name}` to the type level: {reason}"),
            ));
        }
        if matches!(v.kind, ValueKind::TopFn) {
            // Check if the function's scheme itself is impure (e.g. from another
            // module or a precompiled package).
            let mut cur = &v.scheme.ty;
            while let Ty::Fun(_, _, eff, cod) | Ty::Pi { eff, cod, .. } = cur {
                if !eff.is_pure() {
                    return Err(Error::at(
                        span,
                        format!(
                            "cannot promote function `{name}` to the type level: function performs effect `{}`",
                            self.display_eff(eff)
                        ),
                    ));
                }
                cur = cod;
            }
        }
        Ok(None)
    }

    /// Compile a type-level lambda `\x -> body` (e.g. inside `map (\x -> x * 2) [1, 2, 3]`)
    /// directly into a `PVal::Closure` for native Rust evaluation.
    fn compile_type_level_lam(
        &self,
        ty: &ast::Type,
        outer_vars: &[String],
    ) -> Result<crate::promote::PVal> {
        use crate::promote::PVal;
        let mut params = Vec::new();
        let mut cur = ty;
        while let ast::Type::Lam(n, body, _) = cur {
            params.push(n.text.clone());
            cur = body;
        }
        let arity = params.len();
        let body = self.compile_promoted_type_as_pexpr(cur, &params, outer_vars)?;
        Ok(PVal::Closure {
            arity,
            env: Vec::new(),
            body: std::sync::Arc::new(body),
        })
    }

    fn compile_promoted_type_as_pexpr(
        &self,
        ty: &ast::Type,
        lam_params: &[String],
        outer_vars: &[String],
    ) -> Result<crate::promote::PExpr> {
        use crate::promote::{NativeBuiltin, PExpr, PUnaryOp};
        match ty {
            ast::Type::Num(s, span) => s
                .parse::<u128>()
                .map(PExpr::Nat)
                .map_err(|_| Error::at(*span, format!("invalid numeral `{s}` in type-level lambda"))),
            ast::Type::StrLit(s, _) => Ok(PExpr::Str(s.clone())),
            ast::Type::Unit(_) => Ok(PExpr::Unit),
            ast::Type::Var(n, span) => {
                if let Some(idx) = lam_params.iter().rposition(|p| p == &n.text) {
                    return Ok(PExpr::Var(idx));
                }
                if n.text == "not" {
                    return Ok(PExpr::Native(NativeBuiltin::Not));
                }
                if let Some(op) = Self::promoted_binop_of(&n.text) {
                    return Ok(PExpr::Native(NativeBuiltin::BinOp(op)));
                }
                if let Some(pfn) = self.lookup_promoted_fn(None, &n.text, *span)? {
                    return Ok(PExpr::FnRef(pfn.identity));
                }
                Err(Error::at(
                    *span,
                    format!("unknown variable `{}` in type-level lambda", n.text),
                ))
            }
            ast::Type::Con(p, span) => {
                let parts = p.parts();
                if parts.as_slice() == ["True"] {
                    return Ok(PExpr::Bool(true));
                }
                if parts.as_slice() == ["False"] {
                    return Ok(PExpr::Bool(false));
                }
                if let Some(ci) = self.lookup_promoted_ctor_path(p) {
                    return Ok(PExpr::Ctor {
                        identity: ci.identity,
                        args: Vec::new(),
                    });
                }
                Err(Error::at(
                    *span,
                    format!("unsupported constructor `{}` in type-level lambda", parts.join(".")),
                ))
            }
            ast::Type::Tuple(items, _) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(self.compile_promoted_type_as_pexpr(item, lam_params, outer_vars)?);
                }
                Ok(PExpr::Tuple(out))
            }
            ast::Type::List(items, _) => {
                let mut acc = PExpr::Ctor {
                    identity: crate::types::PROMOTED_NIL.to_string(),
                    args: Vec::new(),
                };
                for item in items.iter().rev() {
                    let e = self.compile_promoted_type_as_pexpr(item, lam_params, outer_vars)?;
                    acc = PExpr::Ctor {
                        identity: crate::types::PROMOTED_CONS.to_string(),
                        args: vec![e, acc],
                    };
                }
                Ok(acc)
            }
            ast::Type::Lam(n, body, _) => {
                let mut inner_params = lam_params.to_vec();
                inner_params.push(n.text.clone());
                let b = self.compile_promoted_type_as_pexpr(body, &inner_params, outer_vars)?;
                Ok(PExpr::Lam {
                    arity: 1,
                    body: std::sync::Arc::new(b),
                })
            }
            ast::Type::App(_, _, _) => {
                let mut args = Vec::new();
                let mut cur = ty;
                while let ast::Type::App(h, a, _) = cur {
                    args.push(a.as_ref());
                    cur = h.as_ref();
                }
                args.reverse();
                if let ast::Type::Var(n, _) = cur {
                    if n.text == "not" && args.len() == 1 {
                        return Ok(PExpr::UnaryOp {
                            op: PUnaryOp::Not,
                            arg: Box::new(self.compile_promoted_type_as_pexpr(
                                args[0],
                                lam_params,
                                outer_vars,
                            )?),
                        });
                    }
                    if let Some(op) = Self::promoted_binop_of(&n.text) {
                        if args.len() == 2 {
                            let l = self.compile_promoted_type_as_pexpr(args[0], lam_params, outer_vars)?;
                            let r = self.compile_promoted_type_as_pexpr(args[1], lam_params, outer_vars)?;
                            return Ok(PExpr::BinOp {
                                op,
                                lhs: Box::new(l),
                                rhs: Box::new(r),
                            });
                        }
                    }
                }
                if let ast::Type::Con(p, _) = cur {
                    if let Some(ci) = self.lookup_promoted_ctor_path(p) {
                        if args.len() == ci.field_tys.len() {
                            let mut cargs = Vec::with_capacity(args.len());
                            for a in args {
                                cargs.push(self.compile_promoted_type_as_pexpr(
                                    a,
                                    lam_params,
                                    outer_vars,
                                )?);
                            }
                            return Ok(PExpr::Ctor {
                                identity: ci.identity,
                                args: cargs,
                            });
                        }
                    }
                }
                let f = self.compile_promoted_type_as_pexpr(cur, lam_params, outer_vars)?;
                let mut cargs = Vec::with_capacity(args.len());
                for a in args {
                    cargs.push(self.compile_promoted_type_as_pexpr(a, lam_params, outer_vars)?);
                }
                Ok(PExpr::App {
                    func: Box::new(f),
                    args: cargs,
                })
            }
            _ => Err(Error::at(
                ty.span(),
                "unsupported expression in type-level lambda",
            )),
        }
    }

    /// Convert and evaluate a promoted function call in type or index position.
    fn conv_promoted_fn_call(
        &self,
        pfn: &crate::promote::PromotedFn,
        args: &[ast::Type],
        vars: &[String],
        span: ast::Span,
    ) -> Result<Ty> {
        use crate::promote::{PVal, PROMOTED_FNREF_PREFIX, PROMOTED_FN_PREFIX};
        if args.len() > pfn.arity() && !matches!(pfn.ret_ty, Ty::Fun(..)) {
            return Err(Error::at(
                span,
                format!(
                    "promoted function `{}` expects at most {} argument{}, got {}",
                    pfn.identity,
                    pfn.arity(),
                    if pfn.arity() == 1 { "" } else { "s" },
                    args.len()
                ),
            ));
        }

        let mut has_closure = false;
        let mut pvals: Vec<PVal> = Vec::with_capacity(args.len());
        let mut conv_tys: Vec<Ty> = Vec::with_capacity(args.len());

        for (i, arg) in args.iter().enumerate() {
            let param_hint = pfn.param_tys.get(i);
            // First-class lambda argument (`map (\x -> x * 2) [1, 2, 3]`)!
            if matches!(arg, ast::Type::Lam(..)) {
                let closure = self.compile_type_level_lam(arg, vars)?;
                has_closure = true;
                pvals.push(closure);
                continue;
            }
            let ty = match param_hint {
                Some(Ty::Fun(..)) => {
                    // Expecting a promoted function value (bare ref `double` or partial app `add 10`).
                    if let ast::Type::Var(n, vspan) = arg {
                        if !vars.contains(&n.text) {
                            if let Some(op) = Self::promoted_binop_of(&n.text) {
                                has_closure = true;
                                pvals.push(PVal::Native(crate::promote::NativeBuiltin::BinOp(op)));
                                continue;
                            }
                            if let Some(callee) = self.lookup_promoted_fn(None, &n.text, *vspan)? {
                                let t = Ty::Adt(
                                    format!("{PROMOTED_FNREF_PREFIX}{}", callee.identity).into(),
                                    Vec::new(),
                                );
                                pvals.push(PVal::from_ty(&t));
                                conv_tys.push(t);
                                continue;
                            }
                        }
                    }
                    self.conv_promoted(arg, vars, None)?
                }
                Some(Ty::Bool) => self.conv_promoted(arg, vars, Some(&Ty::Bool))?,
                Some(ek) if Self::is_scalar_idx_ty(ek) => {
                    Ty::Idx(Box::new(self.conv_idx(arg, vars, ek)?))
                }
                Some(ek @ (Ty::Adt(..) | Ty::Tuple(..))) => {
                    // If the expected type mentions polymorphic type vars of `pfn`
                    // (e.g. `List a`), don't over-constrain element kind.
                    let concrete_ek = if pfn.type_params.is_empty() {
                        Some(ek)
                    } else {
                        None
                    };
                    self.conv_promoted(arg, vars, concrete_ek)?
                }
                _ => self.conv_promoted(arg, vars, None)?,
            };
            let zty = self.uni.zonk(&ty);
            pvals.push(PVal::from_ty(&zty));
            conv_tys.push(zty);
        }

        if has_closure {
            if pfn.returns_idx && args.len() == pfn.arity() {
                if let Some(it) = self.uni.promoted_env().eval_pvals_to_iterm(&pfn.identity, pvals) {
                    return Ok(Ty::Idx(Box::new(it)));
                }
            } else if let Some(ty) = self.uni.promoted_env().eval_pvals_to_ty(&pfn.identity, pvals) {
                return Ok(self.uni.zonk(&ty));
            }
            return Err(Error::at(
                span,
                format!(
                    "type-level higher-order call `{}` with lambda could not be reduced (its argument is symbolic or exceeded evaluation fuel)",
                    pfn.identity
                ),
            ));
        }

        let fn_head = format!("{PROMOTED_FN_PREFIX}{}", pfn.identity);
        let raw = if pfn.returns_idx && args.len() == pfn.arity() {
            Ty::Idx(Box::new(ITerm::TyFamily(fn_head.into(), conv_tys)))
        } else {
            Ty::Adt(fn_head.into(), conv_tys)
        };
        Ok(self.uni.zonk(&raw))
    }

    /// Infer the runtime carrier type `T` for a built-in zero-cost `Sing x`
    /// type (`Ty::Adt("Sing", [T, x])`). Rejects erased kinds containing `Type`
    /// or `Prop`.
    fn infer_sing_carrier(
        &self,
        val_ty: &Ty,
        span: ast::Span,
    ) -> Result<Ty> {
        let z = self.uni.zonk(val_ty);
        let carrier = self.infer_sing_carrier_inner(&z, span)?;
        if Self::ty_contains_erased_universe(&carrier) {
            return Err(Error::at(
                span,
                crate::diagnostic!(
                    "cannot form a runtime singleton `Sing` for kind {}: `Type` and `Prop` are erased at runtime",
                    carrier
                ),
            ));
        }
        Ok(carrier)
    }

    fn ty_contains_erased_universe(ty: &Ty) -> bool {
        match ty {
            Ty::Sort(_) | Ty::Prp(_) => true,
            Ty::Adt(_, args) | Ty::Tuple(args) => args.iter().any(Self::ty_contains_erased_universe),
            Ty::App(h, args) => {
                Self::ty_contains_erased_universe(h)
                    || args.iter().any(Self::ty_contains_erased_universe)
            }
            Ty::Fun(a, _, _, b) => {
                Self::ty_contains_erased_universe(a) || Self::ty_contains_erased_universe(b)
            }
            _ => false,
        }
    }

    fn infer_sing_carrier_inner(&self, z: &Ty, span: ast::Span) -> Result<Ty> {
        match z {
            Ty::Unit => Ok(Ty::Unit),
            Ty::Idx(it) => self.infer_idx_sing_carrier(it, span),
            Ty::Param(name) => {
                if let Some(pk) = self.declared_parameter_kind(name.as_str()) {
                    match pk {
                        ParamKind::Data(dty) => return Ok(dty),
                        ParamKind::Val(vty) => return Ok(vty),
                        ParamKind::Type => {
                            return Err(Error::at(
                                span,
                                format!(
                                    "cannot form `Sing {name}`: `{name}` has kind `Type`; annotate its data kind (e.g. `({name} :: Nat)` or `({name} :: List Nat)`)"
                                ),
                            ));
                        }
                        _ => {}
                    }
                }
                if let Some((_, carrier)) = self
                    .index_carriers
                    .borrow()
                    .iter()
                    .rev()
                    .find(|(b, _)| b == name.as_str())
                {
                    return Ok(carrier.clone());
                }
                // Check GADT/singleton skolem carrier table if present.
                if let Some(carrier) = self.sing_skolem_carriers.get(name.as_str()) {
                    return Ok(carrier.clone());
                }
                Err(Error::at(
                    span,
                    format!(
                        "cannot infer data kind for `Sing {name}`: annotate `{name}` with a data kind (e.g. `forall ({name} :: List Nat). Sing {name} -> ...`)"
                    ),
                ))
            }
            Ty::Tuple(items) => {
                let mut carriers = Vec::with_capacity(items.len());
                for item in items {
                    carriers.push(self.infer_sing_carrier_inner(item, span)?);
                }
                Ok(Ty::Tuple(carriers))
            }
            Ty::Adt(id, args) => {
                let s = id.as_str();
                if s == crate::promote::PROMOTED_TRUE || s == crate::promote::PROMOTED_FALSE {
                    return Ok(Ty::Bool);
                }
                if s.starts_with('"') && s.ends_with('"') && args.is_empty() {
                    return Ok(Ty::Adt(crate::known::STRING.into(), Vec::new()));
                }
                if s == crate::types::PROMOTED_NIL {
                    return Ok(Ty::Adt(crate::known::LIST.into(), vec![Ty::Nat]));
                }
                if s == crate::types::PROMOTED_CONS && args.len() == 2 {
                    let elem = self.infer_sing_carrier_inner(&args[0], span)?;
                    return Ok(Ty::Adt(crate::known::LIST.into(), vec![elem]));
                }
                if let Some(ctor_id) = s.strip_prefix("ctor:") {
                    if let Some((mod_part, ctor_name)) = ctor_id.rsplit_once('.') {
                        if let Ok(val) = self.lookup_value(Some(mod_part), ctor_name) {
                            if let ValueKind::Ctor { field_tys, ty_params, .. } = &val.kind {
                                let mut cur = &val.scheme.ty;
                                while let Ty::Fun(_, _, _, cod) | Ty::Pi { cod, .. } = cur {
                                    cur = cod;
                                }
                                let mut env = HashMap::new();
                                for (ft, arg) in field_tys.iter().zip(args.iter()) {
                                    if let Ok(ac) = self.infer_sing_carrier_inner(arg, span) {
                                        Self::match_carrier_params(ft, &ac, ty_params, &mut env);
                                    }
                                }
                                return Ok(Self::subst_ty(cur, &env));
                            }
                        }
                    }
                }
                if let Some(fn_id) = s.strip_prefix(crate::promote::PROMOTED_FN_PREFIX) {
                    if let Some(pfn) = self.uni.promoted_fn(fn_id) {
                        let mut env = HashMap::new();
                        for (pt, arg) in pfn.param_tys.iter().zip(args.iter()) {
                            if let Ok(ac) = self.infer_sing_carrier_inner(arg, span) {
                                Self::match_carrier_params(pt, &ac, &pfn.type_params, &mut env);
                            }
                        }
                        return Ok(Self::subst_ty(&pfn.ret_ty, &env));
                    }
                }
                if self.idx_type_families.contains(s) {
                    return Ok(Ty::Nat);
                }
                if let Some(ret_k) = self.data_type_family_returns.get(s) {
                    return Ok(ret_k.clone());
                }
                Err(Error::at(
                    span,
                    crate::diagnostic!(
                        "cannot form runtime singleton `Sing ({})`: expected a promoted data value, literal, or promoted function call",
                        z
                    ),
                ))
            }
            _ => Err(Error::at(
                span,
                crate::diagnostic!(
                    "cannot form runtime singleton `Sing ({})`: expected a promoted data value, literal, or promoted function call",
                    z
                ),
            )),
        }
    }

    fn infer_idx_sing_carrier(&self, it: &ITerm, span: ast::Span) -> Result<Ty> {
        match it {
            ITerm::Num(s) => {
                if s.as_str().starts_with('-') {
                    Ok(Ty::Int)
                } else {
                    Ok(Ty::Nat)
                }
            }
            ITerm::Var(name) => {
                if let Some(pk) = self.declared_parameter_kind(name.as_str()) {
                    match pk {
                        ParamKind::Val(vty) | ParamKind::Data(vty) => return Ok(vty),
                        _ => {}
                    }
                }
                if let Some((_, carrier)) = self
                    .index_carriers
                    .borrow()
                    .iter()
                    .rev()
                    .find(|(b, _)| b == name.as_str())
                {
                    return Ok(carrier.clone());
                }
                if let Some(carrier) = self.sing_skolem_carriers.get(name.as_str()) {
                    return Ok(carrier.clone());
                }
                Ok(Ty::Nat)
            }
            ITerm::Bin(_, a, b) => {
                let ca = self.infer_idx_sing_carrier(a, span)?;
                if ca == Ty::Int {
                    return Ok(Ty::Int);
                }
                self.infer_idx_sing_carrier(b, span)
            }
            ITerm::TyFamily(name, args) => {
                if let Some(fn_id) = name.as_str().strip_prefix(crate::promote::PROMOTED_FN_PREFIX) {
                    if let Some(pfn) = self.uni.promoted_fn(fn_id) {
                        let mut env = HashMap::new();
                        for (pt, arg) in pfn.param_tys.iter().zip(args.iter()) {
                            if let Ok(ac) = self.infer_sing_carrier_inner(arg, span) {
                                Self::match_carrier_params(pt, &ac, &pfn.type_params, &mut env);
                            }
                        }
                        return Ok(Self::subst_ty(&pfn.ret_ty, &env));
                    }
                }
                Ok(Ty::Nat)
            }
            _ => Ok(Ty::Nat),
        }
    }

    fn match_carrier_params(
        pat: &Ty,
        actual: &Ty,
        params: &[String],
        env: &mut HashMap<String, Ty>,
    ) {
        match (pat, actual) {
            (Ty::Param(p), _) if params.iter().any(|k| k == p.as_str()) => {
                env.entry(p.to_string()).or_insert_with(|| actual.clone());
            }
            (Ty::Adt(n1, a1), Ty::Adt(n2, a2)) if n1 == n2 && a1.len() == a2.len() => {
                for (x, y) in a1.iter().zip(a2.iter()) {
                    Self::match_carrier_params(x, y, params, env);
                }
            }
            (Ty::Tuple(a1), Ty::Tuple(a2)) if a1.len() == a2.len() => {
                for (x, y) in a1.iter().zip(a2.iter()) {
                    Self::match_carrier_params(x, y, params, env);
                }
            }
            _ => {}
        }
    }
}

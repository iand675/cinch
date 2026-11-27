// A native record with unpacked fields keeps its source constructor as a
// function. Its private constructor owns the flattened payload, so ordinary
// constructor specialization, reference counting and reuse see the real slots.
#[derive(Clone)]
struct UnpackedRecord {
    fields: Vec<UnpackedField>,
}

#[derive(Clone)]
struct UnpackedField {
    name: String,
    // None denotes an ordinary slot, Some the constructor used to box out.
    ctor: Option<ECore>,
    slots: Vec<Ty>,
}

impl Checker {
    fn unpacked_record(&self, info: &TypeKind) -> Option<&UnpackedRecord> {
        if self.unpacked_records.is_empty() || info.fields.is_none() {
            return None;
        }
        self.unpacked_records.get(&fn_id(&info.module, &info.name))
    }

    fn is_name_exported_in(&self, module: &str, name: &str) -> bool {
        match self.module_exports.get(module) {
            Some(items) if !items.is_empty() => items.iter().any(|item| match item {
                ast::ExportItem::Name(n) => n.text == name,
                ast::ExportItem::Member(p) => p.parts().last().copied() == Some(name),
                _ => false,
            }),
            _ => true,
        }
    }

    fn is_single_ctor_uniform_product(&self, ty: &Ty) -> bool {
        let Ty::Adt(n, _) = ty else { return false };
        let Ok(kind) = self.lookup_known_type(n) else { return false };
        let key = (kind.module.clone(), kind.name.clone());
        kind.ctors.len() == 1
            && (kind.data_family || kind.fields.is_some())
            && !self.layout_recipes.contains_key(&key)
            && !self.niche_plans.contains_key(&key)
            && !self.codata_types.contains(&key)
    }

    fn try_auto_unpack_field(
        &self,
        module: &str,
        rec_id: &GlobalId,
        rec_exported: bool,
        field: &str,
        ty: &Ty,
    ) -> Option<UnpackedField> {
        let Ty::Adt(n, args) = ty else { return None };
        let kind = self.lookup_known_type(n).ok()?;
        if kind.module != module {
            return None;
        }
        let [(cname, _)] = kind.ctors.as_slice() else { return None };
        let key = (kind.module.clone(), kind.name.clone());
        let ctor = fn_id(&kind.module, cname);
        if ctor == *rec_id {
            return None;
        }
        if rec_exported && !self.is_name_exported_in(module, cname) {
            return None;
        }
        if (!kind.data_family && kind.fields.is_none())
            || kind.param_kinds.iter().any(|p| !matches!(p, ParamKind::Type))
            || self.layout_recipes.contains_key(&key)
            || self.niche_plans.contains_key(&key)
            || self.codata_types.contains(&key)
            || self.unpack_requests.contains_key(&ctor)
            || self.unpacked_records.contains_key(&ctor)
        {
            return None;
        }
        let v = self.qualified.get(&(kind.module, cname.clone()))?;
        let ValueKind::Ctor { field_tys, field_erased, ty_params, .. } = &v.kind else { return None };
        let subst: HashMap<_, _> = ty_params.iter().cloned().zip(args.iter().cloned()).collect();
        let slots: Vec<_> = field_tys
            .iter()
            .zip(field_erased)
            .filter(|(_, erased)| !**erased)
            .map(|(t, _)| Self::subst_ty(t, &subst))
            .collect();
        if slots.len() < 2 || slots.len() > 4 {
            return None;
        }
        if slots.iter().any(|s| matches!(s, Ty::Prim(Scalar::I128 | Scalar::U128))) {
            return None;
        }
        // Require leaf product: guarantees non-recursive and declaration-order independent.
        if slots.iter().any(|s| self.is_single_ctor_uniform_product(s)) {
            return None;
        }
        let instantiation = v.scheme.params.iter().map(|p| subst.get(p).cloned()).collect::<Option<Vec<_>>>()?;
        Some(UnpackedField { name: field.to_owned(), ctor: Some(ECore::Ctor(ctor, instantiation)), slots })
    }

    fn compute_unpack_plans(&mut self) -> Result<()> {
        for (record, requested) in self.unpack_requests.clone() {
            let (module, name) = record.0.rsplit_once('.').expect("qualified constructor");
            let info = &self.qualified[&(module.to_string(), name.to_string())];
            let ValueKind::Ctor { field_binders, field_tys, field_erased, .. } = &info.kind else { unreachable!() };
            let mut fields = Vec::new();
            for ((name, ty), erased) in field_binders.iter().zip(field_tys).zip(field_erased) {
                let request = requested.get(name);
                if *erased {
                    if let Some(span) = request {
                        return Err(Error::at(*span, format!("cannot unpack erased field {record}.{name}")));
                    }
                    continue;
                }
                let field = if let Some(span) = request {
                    self.unpack_field(name, ty, *span)?
                } else {
                    UnpackedField { name: name.clone(), ctor: None, slots: vec![ty.clone()] }
                };
                fields.push(field);
            }
            self.unpacked_records.insert(record, UnpackedRecord { fields });
        }
        for (module, name, _span) in self.auto_unpack_candidates.clone() {
            let key = (module.clone(), name.clone());
            let record = fn_id(&module, &name);
            if self.layout_recipes.contains_key(&key)
                || self.niche_plans.contains_key(&key)
                || self.codata_types.contains(&key)
                || self.unpack_requests.contains_key(&record)
                || self.unpacked_records.contains_key(&record)
            {
                continue;
            }
            let Some(info) = self.qualified.get(&(module.clone(), name.clone())).cloned() else {
                continue;
            };
            let ValueKind::Ctor { field_binders, field_tys, field_erased, .. } = &info.kind else {
                continue;
            };
            let rec_exported = self.is_name_exported_in(&module, &name);
            let nounpack = self.nounpack_requests.get(&record).cloned().unwrap_or_default();
            let mut fields = Vec::new();
            let mut unpacked_any = false;
            let mut total_slots = 0usize;
            for ((fname, ty), erased) in field_binders.iter().zip(field_tys).zip(field_erased) {
                if *erased {
                    continue;
                }
                if !nounpack.contains(fname) {
                    if let Some(uf) = self.try_auto_unpack_field(&module, &record, rec_exported, fname, ty) {
                        total_slots += uf.slots.len();
                        fields.push(uf);
                        unpacked_any = true;
                        continue;
                    }
                }
                total_slots += 1;
                fields.push(UnpackedField { name: fname.clone(), ctor: None, slots: vec![ty.clone()] });
            }
            if unpacked_any && total_slots <= 12 {
                self.unpacked_records.insert(record, UnpackedRecord { fields });
            }
        }
        Ok(())
    }

    fn unpack_field(&self, field: &str, ty: &Ty, span: ast::Span) -> Result<UnpackedField> {
        let invalid = || Error::at(span, format!("unpack on {field} requires a monomorphic single-constructor native product"));
        let Ty::Adt(n, args) = ty else { return Err(invalid()); };
        let kind = self.lookup_known_type(n)?;
        let [(name, _)] = kind.ctors.as_slice() else { return Err(invalid()); };
        let key = (kind.module.clone(), kind.name.clone());
        let ctor = fn_id(&kind.module, name);
        if (!kind.data_family && kind.fields.is_none())
            || kind.param_kinds.iter().any(|p| !matches!(p, ParamKind::Type))
            || self.layout_recipes.contains_key(&key)
            || self.codata_types.contains(&key)
            || self.unpack_requests.contains_key(&ctor)
        {
            return Err(invalid());
        }
        let v = &self.qualified[&(kind.module, name.clone())];
        let ValueKind::Ctor { field_tys, field_erased, ty_params, .. } = &v.kind else { return Err(invalid()); };
        let subst: HashMap<_, _> = ty_params.iter().cloned().zip(args.iter().cloned()).collect();
        let slots: Vec<_> = field_tys.iter().zip(field_erased)
            .filter(|(_, erased)| !**erased)
            .map(|(t, _)| Self::subst_ty(t, &subst))
            .collect();
        let instantiation = v.scheme.params.iter().map(|p| subst[p].clone()).collect();
        if slots.is_empty() { return Err(invalid()); }
        Ok(UnpackedField { name: field.to_owned(), ctor: Some(ECore::Ctor(ctor, instantiation)), slots })
    }
}

impl UnpackedRecord {
    fn project(&self, index: usize, record: LocalId) -> ECore {
        let field = &self.fields[index];
        let mut before: Vec<_> = self.fields[..index].iter().flat_map(|f| f.slots.iter().cloned()).collect();
        let mut values = Vec::new();
        for ty in &field.slots {
            let slot = ProjField::slot(before.len(), Class::Raw);
            values.push(ECore::Proj(Box::new(ECore::Var(record)), slot, ty.clone(), before.clone()));
            before.push(ty.clone());
        }
        match &field.ctor {
            Some(ctor) => app_spine(ctor.clone(), values),
            None => values.pop().expect("ordinary field"),
        }
    }

    fn pattern(&self, logical: Vec<LocalId>, body: ECore, ck: &mut Checker) -> (Vec<LocalId>, ECore) {
        let mut physical = Vec::new();
        let mut bindings = Vec::new();
        for (field, local) in self.fields.iter().zip(logical) {
            if let Some(ctor) = &field.ctor {
                let mut args = Vec::new();
                for ty in &field.slots {
                    let slot = ck.fresh_local();
                    ck.note_local_ty(slot, ty);
                    physical.push(slot);
                    args.push(ECore::Var(slot));
                }
                bindings.push((local, app_spine(ctor.clone(), args)));
            } else {
                physical.push(local);
            }
        }
        (physical, wrap_lets(bindings, body))
    }

    fn storage_unit(id: &GlobalId) -> GlobalId {
        GlobalId(format!("{}$unpacked", id.0))
    }

    fn units(&self, id: GlobalId, params: Vec<Class>, ck: &Checker) -> (FnDef, FnDef) {
        let storage = Self::storage_unit(&id);
        let mut classes = Vec::new();
        let mut args = Vec::new();
        let mut cases = Vec::new();
        let mut next = params.len();
        for (i, field) in self.fields.iter().enumerate() {
            if field.ctor.is_some() {
                let mut locals = Vec::new();
                let mut inner = Vec::new();
                for ty in &field.slots {
                    let local = LocalId(next);
                    next += 1;
                    let class = crate::repr::repr_of(ty, &[], ck, &mut |_| {});
                    classes.push(class);
                    inner.push(class);
                    args.push(Core::Var(local));
                    locals.push(local);
                }
                cases.push((LocalId(i), CPat::Con { tag: 0, fields: locals, classes: inner }));
            } else {
                classes.push(params[i]);
                args.push(Core::Var(LocalId(i)));
            }
        }
        let mut body = Core::App(Box::new(Core::Ctor(storage.clone(), vec![])), args);
        for (local, pat) in cases.into_iter().rev() {
            body = Core::Case { scrut: Box::new(Core::Var(local)), alts: vec![CAlt::new(pat, body)] };
        }
        let storage_unit = FnDef {
            id: storage,
            params: (0..classes.len()).map(|i| format!("f{i}")).collect(),
            body: Core::Unit,
            prim: None,
            ctor: Some((0, classes.len())),
            hints: Default::default(),
            meta: Default::default(),
            repr: crate::core::UnitRepr {
                params: classes.clone(),
                fields: classes,
                data: Some(id.0.clone()),
                result: Some(Class::Obj),
                ..Default::default()
            },
        };
        let builder = FnDef {
            id,
            params: (0..params.len()).map(|i| format!("f{i}")).collect(),
            body,
            prim: None,
            ctor: None,
            hints: Default::default(),
            meta: Default::default(),
            repr: crate::core::UnitRepr { params, result: Some(Class::Obj), ..Default::default() },
        };
        (builder, storage_unit)
    }

    fn layout(&self, ck: &Checker) -> crate::interface::AbiLayout {
        let mut offset = 0;
        let mut fields = Vec::new();
        let mut unpacked = Vec::new();
        for field in &self.fields {
            let words: usize = field.slots.iter()
                .map(|t| crate::repr::repr_of(t, &[], ck, &mut |_| {}).words())
                .sum();
            let size = words as u64 * WORD_SIZE;
            fields.push(crate::interface::AbiField { name: field.name.clone(), offset, size, align: WORD_ALIGN });
            offset += size;
            if field.ctor.is_some() { unpacked.push(field.name.as_str()); }
        }
        crate::interface::AbiLayout {
            name: String::new(),
            repr: format!("unpack({})", unpacked.join(",")),
            size: offset,
            align: WORD_ALIGN,
            fields,
            discriminant: None,
        }
    }
}

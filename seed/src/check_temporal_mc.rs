// ============ TLA+-like Explicit-State Machines, Bounded Model Checker, and Stuttering Refinement ============
//
// Prototype vertical slice integrating:
// - Explicit-state coalgebras (`machine Name where { state = S; init = e; actions = [...]; ... }`)
// - Stuttering-invariant temporal contracts (`always`, `deadlock_free`, `eventually`, `leads_to`, `recur`, `stable`)
// - Type-directed finite domain enumeration (`Bool`, finite ADT enums, refined `InRange`/`Fin` integers)
// - Automatic sound symmetry reduction (verified via absence of `Ord` on symmetric nominal types)
// - Static field-footprint Partial Order Reduction (POR)
// - Tarjan SCC + Emerson-Lei fair-SCC lasso counterexample finder (`WF` and `SF`)
// - Checked stuttering-aware simulation refinement (`refines Concrete Abstract where { view = f; [measure = r] }`)

/// Canonical hashable/comparable structural key for a compile-time `Value`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum StateKey {
    Word(i64),
    Big { nat: bool, digits: String },
    Bool(bool),
    Unit,
    Sym(String),
    Name(String),
    Str(String),
    Adt { ctor: String, tag: usize, fields: Vec<StateKey> },
    Bytes(Vec<u8>),
}

impl StateKey {
    fn from_value(v: &Value) -> Result<Self> {
        match v {
            Value::Word(w) => Ok(StateKey::Word(*w)),
            Value::Big { nat, digits } => Ok(StateKey::Big {
                nat: *nat,
                digits: digits.clone(),
            }),
            Value::Bool(b) => Ok(StateKey::Bool(*b)),
            Value::Unit => Ok(StateKey::Unit),
            Value::Sym(s) => Ok(StateKey::Sym(s.clone())),
            Value::Name(s) => Ok(StateKey::Name(s.clone())),
            Value::Str(s) => Ok(StateKey::Str(s.clone())),
            Value::Bytes(b) => Ok(StateKey::Bytes(b.borrow().clone())),
            Value::Adt { ctor, tag, fields } => {
                let mut out = Vec::with_capacity(fields.len());
                for f in fields {
                    out.push(StateKey::from_value(f)?);
                }
                Ok(StateKey::Adt {
                    ctor: ctor.clone(),
                    tag: *tag,
                    fields: out,
                })
            }
            other => Err(Error::plain(format!(
                "temporal model checker: state value `{}` is not inspectable/hashable data (use explicit-state coalgebra records rather than opaque closures)",
                value_kind_name(other)
            ))),
        }
    }

    /// First-appearance symmetry canonicalization over symmetric ADT constructor families.
    /// When a type `T` has no `Ord` instance (verified against `Checker::instance_list`),
    /// permuting its nullary constructors or wrapped token values preserves all symmetric
    /// temporal properties.
    fn canonicalize_symmetry(&self, sym_ctors: &HashMap<String, Vec<String>>) -> Self {
        if sym_ctors.is_empty() {
            return self.clone();
        }
        let mut maps: HashMap<String, HashMap<StateKey, StateKey>> = HashMap::new();
        self.canon_rec(sym_ctors, &mut maps)
    }

    fn canon_rec(
        &self,
        sym_ctors: &HashMap<String, Vec<String>>,
        maps: &mut HashMap<String, HashMap<StateKey, StateKey>>,
    ) -> Self {
        match self {
            StateKey::Adt { ctor, tag, fields } => {
                // Check if this constructor belongs to a declared symmetric type family.
                for (family, members) in sym_ctors {
                    if members.iter().any(|m| m == ctor) {
                        if fields.is_empty() {
                            // Nullary enum constructor of a symmetric type (e.g. `NodeA | NodeB`).
                            let entry = maps.entry(family.clone()).or_default();
                            if let Some(mapped) = entry.get(self) {
                                return mapped.clone();
                            }
                            let idx = entry.len();
                            let target_ctor = members.get(idx).cloned().unwrap_or_else(|| ctor.clone());
                            let mapped = StateKey::Adt {
                                ctor: target_ctor,
                                tag: idx,
                                fields: Vec::new(),
                            };
                            entry.insert(self.clone(), mapped.clone());
                            return mapped;
                        } else if fields.len() == 1 {
                            // Single-field nominal wrapper (e.g. `data JobId = JobId Nat`).
                            let entry = maps.entry(family.clone()).or_default();
                            if let Some(mapped) = entry.get(self) {
                                return mapped.clone();
                            }
                            let idx = entry.len() as i64;
                            let mapped_field = match &fields[0] {
                                StateKey::Big { nat, .. } => StateKey::Big {
                                    nat: *nat,
                                    digits: idx.to_string(),
                                },
                                _ => StateKey::Word(idx),
                            };
                            let mapped = StateKey::Adt {
                                ctor: ctor.clone(),
                                tag: *tag,
                                fields: vec![mapped_field],
                            };
                            entry.insert(self.clone(), mapped.clone());
                            return mapped;
                        }
                    }
                }
                StateKey::Adt {
                    ctor: ctor.clone(),
                    tag: *tag,
                    fields: fields.iter().map(|f| f.canon_rec(sym_ctors, maps)).collect(),
                }
            }
            other => other.clone(),
        }
    }
}

/// Human-readable Cinch rendering of a `Value` for counterexample traces.
fn format_mc_value(v: &Value, record_fields: &HashMap<String, Vec<String>>) -> String {
    match v {
        Value::Word(w) => w.to_string(),
        Value::Big { digits, .. } => digits.clone(),
        Value::Bool(b) => if *b { "True".into() } else { "False".into() },
        Value::Unit => "()".into(),
        Value::Sym(s) => format!("'{s}"),
        Value::Name(s) => format!("`{s}"),
        Value::Str(s) => format!("{s:?}"),
        Value::Bytes(b) => format!("Bytes({} bytes)", b.borrow().len()),
        Value::Adt { ctor, tag, fields } => {
            let short = ctor.rsplit('.').next().unwrap_or(ctor);
            if short == "Nil" && fields.is_empty() {
                return "[]".into();
            }
            if short == "Cons" && fields.len() == 2 {
                let mut items = Vec::new();
                let mut cur = v;
                let mut ok = true;
                while let Value::Adt { ctor: c, tag: t, fields: fs } = cur {
                    let cs = c.rsplit('.').next().unwrap_or(c);
                    if (*t == 0 || cs == "Nil") && fs.is_empty() {
                        break;
                    } else if (*t == 1 || cs == "Cons") && fs.len() == 2 {
                        items.push(format_mc_value(&fs[0], record_fields));
                        cur = &fs[1];
                    } else {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    return format!("[{}]", items.join(", "));
                }
            }
            if let Some(names) = record_fields.get(ctor).or_else(|| record_fields.get(short)) {
                if names.len() == fields.len() && !names.is_empty() {
                    let parts: Vec<String> = names
                        .iter()
                        .zip(fields)
                        .map(|(n, f)| format!("{n} = {}", format_mc_value(f, record_fields)))
                        .collect();
                    return format!("{short} {{ {} }}", parts.join(", "));
                }
            }
            if fields.is_empty() {
                if short.is_empty() {
                    format!("#{tag}")
                } else {
                    short.to_string()
                }
            } else {
                let parts: Vec<String> = fields
                    .iter()
                    .map(|f| format_mc_value(f, record_fields))
                    .collect();
                format!("({short} {})", parts.join(" "))
            }
        }
        other => format!("<{}>", value_kind_name(other)),
    }
}

/// Static field-access footprint of a machine action over the state record
/// (used to compute action independence for Partial Order Reduction).
#[derive(Clone, Debug, Default)]
struct ActionFootprint {
    reads: HashSet<usize>,
    writes: HashSet<usize>,
    opaque: bool,
}

impl ActionFootprint {
    fn independent_of(&self, other: &ActionFootprint) -> bool {
        if self.opaque || other.opaque {
            return false;
        }
        let w1_hits_2 = self
            .writes
            .iter()
            .any(|f| other.reads.contains(f) || other.writes.contains(f));
        let w2_hits_1 = other
            .writes
            .iter()
            .any(|f| self.reads.contains(f) || self.writes.contains(f));
        !w1_hits_2 && !w2_hits_1
    }
}

#[derive(Clone, Debug)]
enum ActionResultKind {
    Direct,
    Option,
    List,
}

#[derive(Clone, Debug)]
struct CompiledAction {
    name: String,
    fn_id: GlobalId,
    /// Optional finite argument domain for parameterized actions (`Arg -> State -> ...`).
    arg_domain: Option<Vec<(String, Value)>>,
    result_kind: ActionResultKind,
    footprint: ActionFootprint,
}

struct ExploredMachine {
    module: String,
    name: String,
    state_ty: Ty,
    init_val: Value,
    actions: Vec<CompiledAction>,
    fair: HashSet<usize>,
    strong_fair: HashSet<usize>,
    sym_ctors: HashMap<String, Vec<String>>,
    /// Concrete representative `Value` for each explored state index (0 = init).
    states: Vec<Value>,
    /// Canonical key per state index.
    keys: Vec<StateKey>,
    /// Map from canonical key to state index.
    index: HashMap<StateKey, usize>,
    /// Full transition graph: `edges[u]` = list of `(action_idx, display_label, v)`.
    edges: Vec<Vec<(usize, String, usize)>>,
    /// Shortest-path BFS parent pointers for counterexample trace reconstruction.
    parent: Vec<Option<(usize, String)>>,
    /// Number of states deduplicated specifically by symmetry canonicalization.
    symmetry_merged: usize,
    /// Number of transitions that POR could commute (reported in verification receipt).
    por_commuting_pairs: usize,
    /// Whether exploration hit `max_states`.
    capped: bool,
    /// Record field names for pretty-printing traces.
    record_fields: HashMap<String, Vec<String>>,
    /// Whether any liveness contract was declared on this machine.
    has_liveness: bool,
}

impl ExploredMachine {
    fn trace_to(&self, target: usize) -> Vec<String> {
        let mut chain = Vec::new();
        let mut cur = target;
        while let Some((prev, label)) = &self.parent[cur] {
            chain.push((label.clone(), cur));
            cur = *prev;
        }
        chain.reverse();
        let mut lines = vec![format!(
            "  State 0 (init): {}",
            format_mc_value(&self.states[0], &self.record_fields)
        )];
        for (step, (label, s_idx)) in chain.into_iter().enumerate() {
            lines.push(format!("  --[{label}]-->"));
            lines.push(format!(
                "  State {}: {}",
                step + 1,
                format_mc_value(&self.states[s_idx], &self.record_fields)
            ));
        }
        lines
    }

    fn enabled_actions(&self, s: usize) -> HashSet<usize> {
        self.edges[s].iter().map(|(a, _, _)| *a).collect()
    }
}

/// Collect static record-field reads (`Core::Proj`) and writes (fields differing from
/// a pass-through projection in record construction) on a compiled Core function.
/// Tracks `let t = s.slot` pass-through bindings so unchanged record copy fields
/// (`Sys { owner = s.owner, left = 1, right = s.right }`) are not spuriously counted
/// as reads or writes, enabling exact footprint Partial Order Reduction.
fn analyze_core_footprint(
    body: &Core,
    state_param: Option<LocalId>,
    state_ctor: Option<&str>,
    passthrough: &mut HashMap<LocalId, usize>,
    out: &mut ActionFootprint,
) {
    match body {
        Core::Var(l) => {
            if let Some(&slot) = passthrough.get(l) {
                // Used outside a pass-through slot of the state record ctor -> real read.
                out.reads.insert(slot);
            }
        }
        Core::Proj(scrut, field) => {
            if state_param.is_none() || matches!(&**scrut, Core::Var(l) if Some(*l) == state_param) {
                out.reads.insert(field.slot);
            }
            analyze_core_footprint(scrut, state_param, state_ctor, passthrough, out);
        }
        Core::App(f, args) => {
            if let Core::Ctor(cid, _) = &**f {
                let is_state_ctor = state_ctor.map_or(true, |sc| cid.0 == sc || cid.0.ends_with(&format!(".{sc}")));
                if is_state_ctor {
                    for (slot, arg) in args.iter().enumerate() {
                        let unchanged_proj = matches!(
                            arg,
                            Core::Proj(scrut, pf)
                                if pf.slot == slot
                                    && (state_param.is_none()
                                        || matches!(&**scrut, Core::Var(l) if Some(*l) == state_param))
                        );
                        let unchanged_var = matches!(
                            arg,
                            Core::Var(l) if passthrough.get(l) == Some(&slot)
                        );
                        if !unchanged_proj && !unchanged_var {
                            out.writes.insert(slot);
                            analyze_core_footprint(arg, state_param, state_ctor, passthrough, out);
                        }
                    }
                    return;
                }
            }
            if let Core::Global(g, _) = &**f {
                let short = g.0.rsplit('.').next().unwrap_or(&g.0);
                if !matches!(short, "Some" | "None" | "Cons" | "Nil")
                    && args.iter().any(|a| matches!(a, Core::Var(l) if Some(*l) == state_param))
                {
                    out.opaque = true;
                }
            }
            analyze_core_footprint(f, state_param, state_ctor, passthrough, out);
            for a in args {
                analyze_core_footprint(a, state_param, state_ctor, passthrough, out);
            }
        }
        Core::Let(v, _, rhs, body) => {
            // Check for `let v = s.slot` pass-through candidate: defer marking `slot` as read
            // until `v` is actually used outside slot `slot` of the state constructor.
            if let Core::Proj(scrut, pf) = &**rhs {
                if state_param.is_none() || matches!(&**scrut, Core::Var(l) if Some(*l) == state_param) {
                    passthrough.insert(*v, pf.slot);
                    analyze_core_footprint(body, state_param, state_ctor, passthrough, out);
                    passthrough.remove(v);
                    return;
                }
            }
            analyze_core_footprint(rhs, state_param, state_ctor, passthrough, out);
            analyze_core_footprint(body, state_param, state_ctor, passthrough, out);
        }
        Core::If(c, t, e) => {
            analyze_core_footprint(c, state_param, state_ctor, passthrough, out);
            analyze_core_footprint(&t.body, state_param, state_ctor, passthrough, out);
            analyze_core_footprint(&e.body, state_param, state_ctor, passthrough, out);
        }
        Core::Case { scrut, alts } => {
            analyze_core_footprint(scrut, state_param, state_ctor, passthrough, out);
            for alt in alts {
                analyze_core_footprint(&alt.body, state_param, state_ctor, passthrough, out);
            }
        }
        Core::Lam { env, .. } => {
            for e in env {
                analyze_core_footprint(e, state_param, state_ctor, passthrough, out);
            }
        }
        _ => {}
    }
}

/// Tarjan's Strongly Connected Components over an induced subgraph `allowed`.
fn tarjan_sccs(
    n: usize,
    edges: &[Vec<(usize, String, usize)>],
    allowed: &[bool],
) -> Vec<Vec<usize>> {
    struct State<'a> {
        edges: &'a [Vec<(usize, String, usize)>],
        allowed: &'a [bool],
        index: usize,
        indices: Vec<Option<usize>>,
        lowlink: Vec<usize>,
        on_stack: Vec<bool>,
        stack: Vec<usize>,
        sccs: Vec<Vec<usize>>,
    }

    fn strongconnect(v: usize, st: &mut State<'_>) {
        st.indices[v] = Some(st.index);
        st.lowlink[v] = st.index;
        st.index += 1;
        st.stack.push(v);
        st.on_stack[v] = true;

        for &(_, _, w) in &st.edges[v] {
            if !st.allowed[w] {
                continue;
            }
            if st.indices[w].is_none() {
                strongconnect(w, st);
                st.lowlink[v] = st.lowlink[v].min(st.lowlink[w]);
            } else if st.on_stack[w] {
                st.lowlink[v] = st.lowlink[v].min(st.indices[w].unwrap());
            }
        }

        if st.lowlink[v] == st.indices[v].unwrap() {
            let mut comp = Vec::new();
            while let Some(w) = st.stack.pop() {
                st.on_stack[w] = false;
                comp.push(w);
                if w == v {
                    break;
                }
            }
            st.sccs.push(comp);
        }
    }

    let mut st = State {
        edges,
        allowed,
        index: 0,
        indices: vec![None; n],
        lowlink: vec![0; n],
        on_stack: vec![false; n],
        stack: Vec::new(),
        sccs: Vec::new(),
    };
    for v in 0..n {
        if allowed[v] && st.indices[v].is_none() {
            strongconnect(v, &mut st);
        }
    }
    st.sccs
}

/// Find a fair violating cycle (or fair deadlocked state) inside `allowed` reachable from `starts`.
/// Uses Emerson-Lei iterative fair-SCC filtering for TLA+ Weak Fairness (`WF`) and Strong Fairness (`SF`).
/// Returns `Some((entry_state, cycle_edges))` where `cycle_edges` is empty for a fair deadlock
/// or a non-empty closed cycle `[(label, next_state), ...]` starting and ending at `entry_state`.
fn find_fair_violating_lasso(
    machine: &ExploredMachine,
    starts: &[usize],
    allowed_init: &[bool],
    weak_fair: &HashSet<usize>,
    strong_fair: &HashSet<usize>,
) -> Option<(usize, Vec<(String, usize)>)> {
    let n = machine.states.len();
    // Compute states reachable from `starts` staying strictly inside `allowed_init`.
    let mut reachable = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    for &s in starts {
        if allowed_init[s] && !reachable[s] {
            reachable[s] = true;
            queue.push_back(s);
        }
    }
    while let Some(u) = queue.pop_front() {
        for &(_, _, v) in &machine.edges[u] {
            if allowed_init[v] && !reachable[v] {
                reachable[v] = true;
                queue.push_back(v);
            }
        }
    }

    // 1. Check deadlocked states inside `reachable`: a deadlocked state has no enabled
    // actions, so WF and SF are vacuously satisfied — making it a valid stuttering end state.
    for s in 0..n {
        if reachable[s] && machine.edges[s].is_empty() {
            return Some((s, Vec::new()));
        }
    }

    // 2. Iteratively filter SCCs in `reachable` until we find a non-trivial SCC that
    // satisfies every WF and SF obligation (Emerson-Lei algorithm).
    let mut worklist: Vec<Vec<usize>> = tarjan_sccs(n, &machine.edges, &reachable);
    while let Some(scc) = worklist.pop() {
        let in_scc: HashSet<usize> = scc.iter().copied().collect();
        // A single state without a self-loop is not a cycle.
        let has_internal_edge = scc.iter().any(|&u| {
            machine.edges[u].iter().any(|&(_, _, v)| in_scc.contains(&v))
        });
        if !has_internal_edge {
            continue;
        }
        // Actions taken on at least one edge inside this SCC.
        let mut taken_in_scc: HashSet<usize> = HashSet::new();
        for &u in &scc {
            for &(act, _, v) in &machine.edges[u] {
                if in_scc.contains(&v) {
                    taken_in_scc.insert(act);
                }
            }
        }

        // Check Weak Fairness: if `a` is enabled in EVERY state of `scc`, it must be taken in `scc`.
        let mut violated_wf = false;
        for &a in weak_fair {
            if !taken_in_scc.contains(&a) {
                let enabled_everywhere = scc.iter().all(|&u| machine.enabled_actions(u).contains(&a));
                if enabled_everywhere {
                    violated_wf = true;
                    break;
                }
            }
        }
        if violated_wf {
            // No sub-SCC of `scc` can avoid `a` being enabled everywhere either, so discard `scc`.
            continue;
        }

        // Check Strong Fairness: if `a` is enabled in ANY state of `scc`, it must be taken in `scc`.
        // If not, remove all states of `scc` where `a` is enabled and re-decompose sub-SCCs!
        let mut bad_sf_action: Option<usize> = None;
        for &a in strong_fair {
            if !taken_in_scc.contains(&a)
                && scc.iter().any(|&u| machine.enabled_actions(u).contains(&a))
            {
                bad_sf_action = Some(a);
                break;
            }
        }
        if let Some(a) = bad_sf_action {
            let mut sub_allowed = vec![false; n];
            for &u in &scc {
                if !machine.enabled_actions(u).contains(&a) {
                    sub_allowed[u] = true;
                }
            }
            let subs = tarjan_sccs(n, &machine.edges, &sub_allowed);
            worklist.extend(subs);
            continue;
        }

        // This SCC satisfies EVERY WF and SF constraint! Extract a concrete closed cycle
        // inside `scc` visiting every required fair action that was taken in `scc`.
        let entry = scc[0];
        let mut cycle = Vec::new();
        // Walk a BFS shortest path inside `scc` that takes an internal edge back to `entry`.
        // First pick any edge out of `entry` inside `scc`, then BFS back to `entry`.
        if let Some((_, first_label, first_next)) = machine.edges[entry]
            .iter()
            .find(|(_, _, v)| in_scc.contains(v))
            .cloned()
        {
            cycle.push((first_label, first_next));
            if first_next != entry {
                let mut parent: HashMap<usize, (usize, String)> = HashMap::new();
                let mut q = std::collections::VecDeque::new();
                q.push_back(first_next);
                let mut seen: HashSet<usize> = HashSet::from([first_next]);
                while let Some(u) = q.pop_front() {
                    if u == entry {
                        break;
                    }
                    for (_, lbl, v) in &machine.edges[u] {
                        if in_scc.contains(v) && seen.insert(*v) {
                            parent.insert(*v, (u, lbl.clone()));
                            q.push_back(*v);
                        }
                    }
                }
                let mut tail = Vec::new();
                let mut cur = entry;
                while cur != first_next {
                    if let Some((prev, lbl)) = parent.get(&cur) {
                        tail.push((lbl.clone(), cur));
                        cur = *prev;
                    } else {
                        break;
                    }
                }
                tail.reverse();
                cycle.extend(tail);
            }
        }
        return Some((entry, cycle));
    }

    None
}

impl Checker {
    /// Check all declared `machine`s, their temporal `contract`s, and `refines`
    /// simulation witnesses after module bodies have been elaborated.
    fn check_machines_and_refines(&mut self) -> Result<()> {
        let pending_machines = std::mem::take(&mut self.pending_machines);
        let pending_refines = std::mem::take(&mut self.pending_refines);
        if pending_machines.is_empty() && pending_refines.is_empty() {
            return Ok(());
        }
        let saved_module = self.current_module.clone();
        let saved_visible = self.visible.clone();
        self.visible = self.module_names.iter().cloned().collect();

        let mut explored: HashMap<(String, String), ExploredMachine> = HashMap::new();
        for (module, decl) in &pending_machines {
            self.current_module = module.clone();
            let m = self.check_and_explore_machine(module, decl)?;
            explored.insert((module.clone(), decl.name.text.clone()), m);
        }

        // Verify all temporal `contract` declarations targeting machines.
        for ((module, name), machine) in &mut explored {
            self.current_module = module.clone();
            if let Some(contracts) = self.pending_contracts.remove(&(module.clone(), name.clone())) {
                for contract in &contracts {
                    if contract.kind == ast::ContractKind::Liveness
                        || contract.kind == ast::ContractKind::Recurrence
                        || contract.kind == ast::ContractKind::Stability
                    {
                        machine.has_liveness = true;
                    }
                    self.check_machine_contract(machine, contract)?;
                }
            }
        }

        // Verify all `refines Concrete Abstract` declarations.
        for (module, decl) in &pending_refines {
            self.current_module = module.clone();
            self.check_refines_decl(module, decl, &explored)?;
        }

        self.current_module = saved_module;
        self.visible = saved_visible;
        Ok(())
    }

    /// Check whether a type identity has an `Ord` instance anywhere in scope.
    /// Used to enforce soundness of automatic symmetry reduction: a symmetric
    /// nominal type must not be ordered.
    fn type_has_ord_instance(&self, ty_identity: &str) -> bool {
        let short = ty_identity.rsplit('.').next().unwrap_or(ty_identity);
        for inst in &self.instance_list {
            let cls = inst.target.class.rsplit('.').next().unwrap_or(&inst.target.class);
            if cls != "Ord" {
                continue;
            }
            for arg in &inst.target.args {
                if let Ty::Adt(id, _) = arg {
                    let arg_short = id.rsplit('.').next().unwrap_or(id);
                    if id.as_str() == ty_identity || arg_short == short {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Return the owning `Ty::Adt` identity of a constructor `ValueInfo`.
    fn ctor_parent_id(v: &ValueInfo) -> Option<String> {
        let mut cur = &v.scheme.ty;
        loop {
            match cur {
                Ty::Fun(_, _, _, res) | Ty::Pi { cod: res, .. } => cur = res,
                _ => break,
            }
        }
        match cur {
            Ty::Adt(parent_id, _) => Some(parent_id.to_string()),
            _ => None,
        }
    }

    /// Collect constructor names belonging to a type identity (for symmetry canonicalization).
    fn type_ctor_units(&self, ty_identity: &str) -> Vec<String> {
        let mut ctors: Vec<(usize, String)> = Vec::new();
        for vs in self.values.values() {
            for v in vs {
                if let ValueKind::Ctor { tag, .. } = &v.kind {
                    if Self::ctor_parent_id(v).as_deref() == Some(ty_identity) {
                        let unit = format!("{}.{}", v.module, v.name);
                        if !ctors.iter().any(|(_, u)| u == &unit) {
                            ctors.push((*tag, unit));
                        }
                    }
                }
            }
        }
        ctors.sort_by_key(|(tag, _)| *tag);
        ctors.into_iter().map(|(_, u)| u).collect()
    }

    /// Collect a map from record constructor unit/short name -> runtime field names
    /// for readable counterexample trace printing.
    fn collect_record_field_names(&self) -> HashMap<String, Vec<String>> {
        let mut out = HashMap::new();
        for vs in self.values.values() {
            for v in vs {
                if let ValueKind::Ctor {
                    field_binders,
                    field_erased,
                    ..
                } = &v.kind
                {
                    let runtime_names: Vec<String> = field_binders
                        .iter()
                        .zip(field_erased)
                        .filter(|(_, erased)| !**erased)
                        .map(|(n, _)| n.clone())
                        .collect();
                    if !runtime_names.is_empty() && runtime_names.iter().all(|n| !n.is_empty()) {
                        out.insert(format!("{}.{}", v.module, v.name), runtime_names.clone());
                        out.entry(v.name.clone()).or_insert(runtime_names);
                    }
                }
            }
        }
        out
    }

    /// Enumerate the finite domain of an action parameter type using Cinch's own type
    /// and refinement information (`Bool`, finite nullary ADT enums, `InRange lo hi Nat`,
    /// bounded refined integers, `Fin n`).
    fn enumerate_finite_domain(&self, ty: &Ty, span: ast::Span) -> Result<Vec<(String, Value)>> {
        let ty = self.uni.zonk(ty);
        match &ty {
            Ty::Bool => Ok(vec![
                ("False".into(), Value::Bool(false)),
                ("True".into(), Value::Bool(true)),
            ]),
            Ty::Unit => Ok(vec![("()".into(), Value::Unit)]),
            Ty::Adt(id, args) => {
                let short = id.rsplit('.').next().unwrap_or(id);
                // `Fin n` with concrete numeral `n`
                if short == "Fin" && args.len() == 1 {
                    if let Ty::Idx(it) = &args[0] {
                        if let ITerm::Num(d) = it.normalize() {
                            if let Ok(n) = d.parse::<i64>() {
                                if n <= 64 {
                                    let fin_ctor = self
                                        .values
                                        .get("Fin")
                                        .and_then(|vs| vs.first())
                                        .map(|v| format!("{}.{}", v.module, v.name))
                                        .unwrap_or_else(|| "Fin".into());
                                    return Ok((0..n)
                                        .map(|i| {
                                            (
                                                i.to_string(),
                                                Value::Adt {
                                                    ctor: fin_ctor.clone(),
                                                    tag: 0,
                                                    fields: vec![Value::Big {
                                                        nat: true,
                                                        digits: i.to_string(),
                                                    }],
                                                },
                                            )
                                        })
                                        .collect());
                                }
                            }
                        }
                    }
                }
                // Refined integer types (`refined (lo <= x && x <= hi) Nat` / `InRange lo hi Nat`)
                if let Some(rv) = self.refine_view(&ty) {
                    if let Some(base) = rv.base {
                        if let Some((lo, hi, is_nat, is_word)) =
                            self.extract_finite_int_range(&base, &rv.pred, &rv.binder)
                        {
                            if hi >= lo && (hi - lo) <= 64 {
                                let mut vals = Vec::new();
                                for v in lo..=hi {
                                    let val = if is_word {
                                        Value::Word(v as i64)
                                    } else {
                                        Value::Big {
                                            nat: is_nat,
                                            digits: v.to_string(),
                                        }
                                    };
                                    vals.push((v.to_string(), val));
                                }
                                return Ok(vals);
                            }
                        }
                    }
                }
                // Finite nullary ADT enum (e.g. `data Role = Leader | Follower` or `data Node = N0 | N1`)
                let mut ctors: Vec<(usize, String, String)> = Vec::new();
                let mut has_non_nullary = false;
                for vs in self.values.values() {
                    for v in vs {
                        if let ValueKind::Ctor {
                            tag,
                            runtime_arity,
                            ..
                        } = &v.kind
                        {
                            if Self::ctor_parent_id(v).as_deref() == Some(id.as_str()) {
                                if *runtime_arity > 0 {
                                    has_non_nullary = true;
                                } else {
                                    let unit = format!("{}.{}", v.module, v.name);
                                    if !ctors.iter().any(|(_, u, _)| u == &unit) {
                                        ctors.push((*tag, unit, v.name.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
                if !has_non_nullary && !ctors.is_empty() {
                    ctors.sort_by_key(|(t, _, _)| *t);
                    return Ok(ctors
                        .into_iter()
                        .map(|(tag, unit, name)| {
                            (
                                name,
                                Value::Adt {
                                    ctor: unit,
                                    tag,
                                    fields: Vec::new(),
                                },
                            )
                        })
                        .collect());
                }
                Err(Error::at(
                    span,
                    crate::diagnostic!(
                        "temporal machine action parameter type `{}` has no finite enumerable domain (use `Bool`, a finite enum `data`, `Fin n`, or a bounded refinement such as `InRange 0 2 Nat`)",
                        ty
                    ),
                ))
            }
            _ => Err(Error::at(
                span,
                crate::diagnostic!(
                    "temporal machine action parameter type `{}` has no finite enumerable domain",
                    ty
                ),
            )),
        }
    }

    /// Extract `[lo, hi]` bounds from a refined integer type (`InRange lo hi Nat` or
    /// `refined (x >= 0 && x < 3) Nat`, etc.).
    fn extract_finite_int_range(
        &self,
        base: &Ty,
        pred: &PTerm,
        binder: &str,
    ) -> Option<(i128, i128, bool, bool)> {
        let (is_nat, is_word) = match base {
            Ty::Nat => (true, false),
            Ty::Int => (false, false),
            Ty::Prim(s) => (!s.is_signed(), true),
            _ => return None,
        };
        let mut lo: Option<i128> = if is_nat { Some(0) } else { None };
        let mut hi: Option<i128> = None;
        for atom in pred_conjuncts(pred) {
            if let PTerm::Cmp(cmp, l, r) = atom {
                let l = l.normalize();
                let r = r.normalize();
                match (&l, &r) {
                    (ITerm::Var(v), ITerm::Num(d)) if v.as_str() == binder => {
                        let k: i128 = d.parse().ok()?;
                        match cmp {
                            IdxCmp::Lt => hi = Some(hi.map_or(k - 1, |h| h.min(k - 1))),
                            IdxCmp::Le => hi = Some(hi.map_or(k, |h| h.min(k))),
                            IdxCmp::Gt => lo = Some(lo.map_or(k + 1, |l0| l0.max(k + 1))),
                            IdxCmp::Ge => lo = Some(lo.map_or(k, |l0| l0.max(k))),
                            IdxCmp::Eq => {
                                lo = Some(lo.map_or(k, |l0| l0.max(k)));
                                hi = Some(hi.map_or(k, |h| h.min(k)));
                            }
                            IdxCmp::Ne => {}
                        }
                    }
                    (ITerm::Num(d), ITerm::Var(v)) if v.as_str() == binder => {
                        let k: i128 = d.parse().ok()?;
                        match cmp {
                            IdxCmp::Lt => lo = Some(lo.map_or(k + 1, |l0| l0.max(k + 1))),
                            IdxCmp::Le => lo = Some(lo.map_or(k, |l0| l0.max(k))),
                            IdxCmp::Gt => hi = Some(hi.map_or(k - 1, |h| h.min(k - 1))),
                            IdxCmp::Ge => hi = Some(hi.map_or(k, |h| h.min(k))),
                            IdxCmp::Eq => {
                                lo = Some(lo.map_or(k, |l0| l0.max(k)));
                                hi = Some(hi.map_or(k, |h| h.min(k)));
                            }
                            IdxCmp::Ne => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        Some((lo?, hi?, is_nat, is_word))
    }

    /// Extract successors from an action return value (`State`, `Option State`, or `List State`).
    fn unpack_action_successors(kind: &ActionResultKind, val: Value) -> Result<Vec<Value>> {
        match kind {
            ActionResultKind::Direct => Ok(vec![val]),
            ActionResultKind::Option => match val {
                Value::Adt { tag: 0, fields, .. } if fields.is_empty() => Ok(Vec::new()),
                Value::Adt { tag: 1, mut fields, .. } if fields.len() == 1 => Ok(vec![fields.remove(0)]),
                other => Err(Error::plain(format!(
                    "temporal machine action returned unexpected Option value: {}",
                    value_kind_name(&other)
                ))),
            },
            ActionResultKind::List => {
                let mut out = Vec::new();
                let mut cur = val;
                loop {
                    match cur {
                        Value::Adt { tag: 0, fields, .. } if fields.is_empty() => break,
                        Value::Adt { tag: 1, mut fields, .. } if fields.len() == 2 => {
                            let tail = fields.pop().unwrap();
                            let head = fields.pop().unwrap();
                            out.push(head);
                            cur = tail;
                        }
                        other => {
                            return Err(Error::plain(format!(
                                "temporal machine action returned unexpected List value: {}",
                                value_kind_name(&other)
                            )));
                        }
                    }
                }
                Ok(out)
            }
        }
    }

    /// Classify an action return type against the machine's `state_ty`.
    fn classify_action_result(&mut self, ret_ty: &Ty, state_ty: &Ty, span: ast::Span, name: &str) -> Result<ActionResultKind> {
        let ret = self.uni.zonk(ret_ty);
        if crate::types::ty_eq(&ret, state_ty) {
            return Ok(ActionResultKind::Direct);
        }
        if let Ty::Adt(id, args) = &ret {
            let short = id.rsplit('.').next().unwrap_or(id);
            if short == "Option" && args.len() == 1 && crate::types::ty_eq(&args[0], state_ty) {
                return Ok(ActionResultKind::Option);
            }
            if short == "List" && args.len() == 1 && crate::types::ty_eq(&args[0], state_ty) {
                return Ok(ActionResultKind::List);
            }
        }
        Err(Error::at(
            span,
            crate::diagnostic!(
                "machine action `{}` must return `{}`, `Option {}`, or `List {}`, found `{}`",
                name, state_ty, state_ty, state_ty, ret
            ),
        ))
    }

    /// Finalize an elaborated expression (solving deferred dictionaries, literals,
    /// index values, and hoisting lambdas into `cx.fns`) and evaluate it to a `Value`.
    fn eval_checked_ecore(&mut self, cx: &mut StageCx, prefix: &str, mut ec: ECore) -> Result<Value> {
        let sol = self.solve_wants()?;
        Self::fill_holes(&mut ec, &sol)?;
        let lits = self.resolved_literals()?;
        let idx_pends = std::mem::take(&mut self.idx_pends);
        let mut vals = Vec::with_capacity(idx_pends.len());
        for p in &idx_pends {
            vals.push(self.reify_index_value(&p.iterm, &p.scope, &p.carrier, p.span)?);
        }
        resolve_lit_words(&mut ec, &lits);
        resolve_idx_words(&mut ec, &vals);
        let mut hoisted = Vec::new();
        hoist_lambdas(&mut ec, prefix, &mut self.lam_counter, &mut hoisted);
        let own: HashSet<GlobalId> = hoisted.iter().map(|h| h.id.clone()).collect();
        let mut lcx = LowerCx::compile_time(self, &[], &own);
        for h in hoisted {
            let fndef = finalize_fn(
                h.id.clone(),
                h.params,
                h.body,
                None,
                None,
                crate::core::FnHints::default(),
                crate::core::FnMeta::default(),
                &mut lcx,
            )?;
            cx.fns.insert(h.id.0, fndef);
        }
        let core = lower(&ec, &lcx);
        lcx.finish()?;
        self.eval_core(cx, &core, &mut Vec::new())
    }

    /// Elaborate, validate, and BFS-explore one `machine` declaration.
    fn check_and_explore_machine(&mut self, module: &str, decl: &ast::MachineDecl) -> Result<ExploredMachine> {
        let state_ty = self.conv_type(&decl.state, &[])?;
        let state_ty = self.uni.zonk(&state_ty);

        // Elaborate and evaluate `init :: state_ty`.
        let mut env = Env::default();
        let init_ecore = self.check(&mut env, &decl.init, &state_ty)?;
        let scx = self.fold_scx.clone().unwrap_or_default();
        let mut cx = StageCx::new(&scx);
        let init_prefix = format!("{module}.machine.{}.init", decl.name.text);
        let init_val = self.eval_checked_ecore(&mut cx, &init_prefix, init_ecore).map_err(|e| {
            Error::at(
                decl.init.span(),
                format!("machine `{}`: failed to evaluate `init`: {e}", decl.name.text),
            )
        })?;

        // Validate optional `observe :: State -> Obs` (coalgebraic observation projection).
        if let Some(obs_tm) = &decl.observe {
            let obs_out = self.uni.fresh();
            let expected_obs = Ty::fun_eff(state_ty.clone(), EffRow::pure(), obs_out);
            let mut obs_env = Env::default();
            self.check(&mut obs_env, obs_tm, &expected_obs).map_err(|e| {
                Error::at(
                    obs_tm.span(),
                    format!("machine `{}`: `observe` must be a pure function `{state_ty} -> Obs`: {e}", decl.name.text),
                )
            })?;
        }

        // Validate symmetry types: sound by parametricity ONLY when no `Ord` instance exists.
        let mut sym_ctors: HashMap<String, Vec<String>> = HashMap::new();
        for sym_path in &decl.symmetry {
            let tk = self.lookup_type_path(sym_path)?;
            if self.type_has_ord_instance(&tk.identity) {
                return Err(Error::at(
                    sym_path.span(),
                    format!(
                        "unsound symmetry reduction on `{}`: `{}` has an `Ord` instance, which breaks permutation symmetry (remove `Ord` or drop `{}` from `symmetry`)",
                        tk.identity, tk.identity, sym_path.parts().join(".")
                    ),
                ));
            }
            let ctors = self.type_ctor_units(&tk.identity);
            sym_ctors.insert(tk.identity.clone(), ctors);
        }

        // Validate and compile every action in `actions = [...]`.
        let mut compiled_actions: Vec<CompiledAction> = Vec::new();
        for act_name in &decl.actions {
            let info = self.lookup_value(Some(module), &act_name.text).or_else(|_| {
                self.lookup_value(None, &act_name.text)
            }).map_err(|_| {
                Error::at(
                    act_name.span,
                    format!("machine `{}`: unknown action `{}`", decl.name.text, act_name.text),
                )
            })?;
            let action_id = fn_id(&info.module, &info.name);
            let (ty, _, _) = self.instantiate(&info.scheme);
            let ty = self.uni.zonk(&ty);
            let (arg_domain, result_kind) = match &ty {
                Ty::Fun(dom, _, eff, cod) => {
                    if !eff.is_pure() {
                        return Err(Error::at(
                            act_name.span,
                            crate::diagnostic!(
                                "machine `{}`: action `{}` must be pure for model checking, found effect row `{}`",
                                decl.name.text, act_name.text, eff
                            ),
                        ));
                    }
                    let dom_z = self.uni.zonk(dom);
                    if crate::types::ty_eq(&dom_z, &state_ty) {
                        let rk = self.classify_action_result(cod, &state_ty, act_name.span, &act_name.text)?;
                        (None, rk)
                    } else if let Ty::Fun(dom2, _, eff2, cod2) = self.uni.zonk(cod) {
                        if !eff2.is_pure() {
                            return Err(Error::at(
                                act_name.span,
                                crate::diagnostic!(
                                    "machine `{}`: action `{}` must be pure, found effect row `{}`",
                                    decl.name.text, act_name.text, eff2
                                ),
                            ));
                        }
                        let dom2_z = self.uni.zonk(&dom2);
                        if !crate::types::ty_eq(&dom2_z, &state_ty) {
                            return Err(Error::at(
                                act_name.span,
                                crate::diagnostic!(
                                    "machine `{}`: parameterized action `{}` must take `{}` as its state argument, found `{}`",
                                    decl.name.text, act_name.text, state_ty, dom2_z
                                ),
                            ));
                        }
                        let domain = self.enumerate_finite_domain(&dom_z, act_name.span)?;
                        let rk = self.classify_action_result(&cod2, &state_ty, act_name.span, &act_name.text)?;
                        (Some(domain), rk)
                    } else {
                        return Err(Error::at(
                            act_name.span,
                            crate::diagnostic!(
                                "machine `{}`: action `{}` must have type `{} -> Option {}` (or `List {}` / `{}` / `Arg -> {} -> ...`), found `{}`",
                                decl.name.text, act_name.text, state_ty, state_ty, state_ty, state_ty, state_ty, ty
                            ),
                        ));
                    }
                }
                _ => {
                    return Err(Error::at(
                        act_name.span,
                        crate::diagnostic!(
                            "machine `{}`: action `{}` must be a function over `{}`, found `{}`",
                            decl.name.text, act_name.text, state_ty, ty
                        ),
                    ));
                }
            };

            // Compute static field footprint for POR analysis.
            self.ensure_fn(&mut cx, &action_id.0)?;
            let mut footprint = ActionFootprint::default();
            if let Some(fndef) = cx.fns.get(&action_id.0) {
                let state_param = fndef.params.len().checked_sub(1).map(LocalId);
                let state_ctor = match &state_ty {
                    Ty::Adt(id, _) => Some(id.as_str()),
                    _ => None,
                };
                let mut passthrough = HashMap::new();
                analyze_core_footprint(&fndef.body, state_param, state_ctor, &mut passthrough, &mut footprint);
            } else {
                footprint.opaque = true;
            }

            compiled_actions.push(CompiledAction {
                name: act_name.text.clone(),
                fn_id: action_id,
                arg_domain,
                result_kind,
                footprint,
            });
        }

        // Validate `fair` and `strong_fair` action lists.
        let mut fair = HashSet::new();
        for f in &decl.fair {
            let Some(idx) = compiled_actions.iter().position(|a| a.name == f.text) else {
                return Err(Error::at(
                    f.span,
                    format!(
                        "machine `{}`: `fair` names unknown action `{}` (not in `actions`)",
                        decl.name.text, f.text
                    ),
                ));
            };
            fair.insert(idx);
        }
        let mut strong_fair = HashSet::new();
        for f in &decl.strong_fair {
            let Some(idx) = compiled_actions.iter().position(|a| a.name == f.text) else {
                return Err(Error::at(
                    f.span,
                    format!(
                        "machine `{}`: `strong_fair` names unknown action `{}` (not in `actions`)",
                        decl.name.text, f.text
                    ),
                ));
            };
            strong_fair.insert(idx);
        }

        // Count independent action pairs discovered by static field-footprint analysis (POR).
        let mut por_commuting_pairs = 0usize;
        for i in 0..compiled_actions.len() {
            for j in (i + 1)..compiled_actions.len() {
                if compiled_actions[i].footprint.independent_of(&compiled_actions[j].footprint) {
                    por_commuting_pairs += 1;
                }
            }
        }

        // BFS state-space exploration with symmetry canonicalization.
        let max_states = decl.max_states.unwrap_or(4096);
        let init_raw_key = StateKey::from_value(&init_val)?;
        let init_key = init_raw_key.canonicalize_symmetry(&sym_ctors);

        let mut states = vec![init_val.clone()];
        let mut keys = vec![init_key.clone()];
        let mut index = HashMap::from([(init_key, 0usize)]);
        let mut raw_seen = HashSet::from([init_raw_key]);
        let mut edges: Vec<Vec<(usize, String, usize)>> = vec![Vec::new()];
        let mut parent: Vec<Option<(usize, String)>> = vec![None];
        let mut queue = std::collections::VecDeque::from([0usize]);
        let mut symmetry_merged = 0usize;
        let mut capped = false;

        while let Some(u) = queue.pop_front() {
            let state_u = states[u].clone();
            let mut out_edges = Vec::new();
            for (act_idx, act) in compiled_actions.iter().enumerate() {
                let invocations: Vec<(String, Vec<Value>)> = match &act.arg_domain {
                    None => vec![(act.name.clone(), vec![state_u.clone()])],
                    Some(dom) => dom
                        .iter()
                        .map(|(arg_str, arg_val)| {
                            (
                                format!("{}({arg_str})", act.name),
                                vec![arg_val.clone(), state_u.clone()],
                            )
                        })
                        .collect(),
                };
                for (label, call_args) in invocations {
                    let res = self.eval_fn(&mut cx, &act.fn_id, call_args).map_err(|e| {
                        Error::at(
                            decl.span,
                            format!(
                                "machine `{}`: action `{label}` failed during exploration: {e}",
                                decl.name.text
                            ),
                        )
                    })?;
                    let succs = Self::unpack_action_successors(&act.result_kind, res)?;
                    for succ in succs {
                        let raw_key = StateKey::from_value(&succ)?;
                        let canon_key = raw_key.canonicalize_symmetry(&sym_ctors);
                        let is_new_raw = raw_seen.insert(raw_key);
                        let v = if let Some(&existing) = index.get(&canon_key) {
                            if is_new_raw && !sym_ctors.is_empty() {
                                symmetry_merged += 1;
                            }
                            existing
                        } else {
                            if states.len() >= max_states {
                                capped = true;
                                continue;
                            }
                            let new_idx = states.len();
                            states.push(succ);
                            keys.push(canon_key.clone());
                            index.insert(canon_key, new_idx);
                            edges.push(Vec::new());
                            parent.push(Some((u, label.clone())));
                            queue.push_back(new_idx);
                            new_idx
                        };
                        out_edges.push((act_idx, label.clone(), v));
                    }
                }
            }
            edges[u] = out_edges;
        }

        let record_fields = self.collect_record_field_names();
        Ok(ExploredMachine {
            module: module.to_string(),
            name: decl.name.text.clone(),
            state_ty,
            init_val,
            actions: compiled_actions,
            fair,
            strong_fair,
            sym_ctors,
            states,
            keys,
            index,
            edges,
            parent,
            symmetry_merged,
            por_commuting_pairs,
            capped,
            record_fields,
            has_liveness: false,
        })
    }

    /// Evaluate a predicate term `P :: State -> Bool` on every explored state of `machine`.
    fn eval_machine_predicate(
        &mut self,
        machine: &ExploredMachine,
        term: &ast::Term,
        contract: &ast::ContractDecl,
    ) -> Result<Vec<bool>> {
        let pred_ty = Ty::fun_eff(machine.state_ty.clone(), EffRow::pure(), Ty::Bool);
        let mut env = Env::default();
        let pred_ecore = self.check(&mut env, term, &pred_ty).map_err(|e| {
            Error::at(
                term.span(),
                format!(
                    "temporal contract `{}` on machine `{}`: predicate must have type `{} -> Bool`: {e}",
                    contract.name.text, machine.name, machine.state_ty
                ),
            )
        })?;
        let scx = self.fold_scx.clone().unwrap_or_default();
        let mut cx = StageCx::new(&scx);
        let pred_prefix = format!("{}.contract.{}", machine.module, contract.name.text);
        let pred_val = self.eval_checked_ecore(&mut cx, &pred_prefix, pred_ecore)?;
        let mut results = Vec::with_capacity(machine.states.len());
        for s in &machine.states {
            let b = self.apply_value(&mut cx, pred_val.clone(), vec![s.clone()])?;
            match b {
                Value::Bool(flag) => results.push(flag),
                other => {
                    return Err(Error::at(
                        term.span(),
                        format!(
                            "temporal contract `{}`: predicate did not evaluate to Bool (got {})",
                            contract.name.text,
                            value_kind_name(&other)
                        ),
                    ));
                }
            }
        }
        Ok(results)
    }

    /// Verify one temporal `contract` attached to an explored `machine`.
    fn check_machine_contract(
        &mut self,
        machine: &ExploredMachine,
        contract: &ast::ContractDecl,
    ) -> Result<()> {
        let mut weak_fair = machine.fair.clone();
        let mut strong_fair = machine.strong_fair.clone();
        let mut assumption_rows: Vec<String> = Vec::new();
        for &a in &machine.fair {
            assumption_rows.push(format!("weak_fair {}", machine.actions[a].name));
        }
        for &a in &machine.strong_fair {
            assumption_rows.push(format!("strong_fair {}", machine.actions[a].name));
        }
        let mut main_clause: Option<&ast::ContractClause> = None;
        for cl in &contract.clauses {
            match cl {
                ast::ContractClause::Formula { .. }
                | ast::ContractClause::LeadsTo { .. }
                | ast::ContractClause::DeadlockFree { .. } => {
                    main_clause = Some(cl);
                }
                ast::ContractClause::AssumeNamed { text, .. } => {
                    assumption_rows.push(format!("assume \"{text}\""));
                }
                ast::ContractClause::AssumeProp { body } => {
                    assumption_rows.push(format!("assume ({})", render_formula_term(body)));
                }
                ast::ContractClause::AssumeFair { strong, actions, .. } => {
                    for a in actions {
                        let Some(idx) = machine.actions.iter().position(|act| act.name == a.text) else {
                            return Err(Error::at(
                                a.span,
                                format!(
                                    "contract `{}` on `{}`: unknown fair action `{}`",
                                    contract.name.text, machine.name, a.text
                                ),
                            ));
                        };
                        if *strong {
                            strong_fair.insert(idx);
                            assumption_rows.push(format!("strong_fair {}", a.text));
                        } else {
                            weak_fair.insert(idx);
                            assumption_rows.push(format!("weak_fair {}", a.text));
                        }
                    }
                }
            }
        }
        let main_clause = main_clause.expect("parser guarantees one main formula clause");
        let display = format!("{}.{}", machine.module, machine.name);
        let total_edges: usize = machine.edges.iter().map(|e| e.len()).sum();
        let scope_note = if machine.capped {
            format!(
                "bounded/capped at {} states, {} transitions, symmetry_merged: {}, por_independent_pairs: {}",
                machine.states.len(),
                total_edges,
                machine.symmetry_merged,
                machine.por_commuting_pairs
            )
        } else {
            format!(
                "{} states, {} transitions, symmetry_merged: {}, por_independent_pairs: {}",
                machine.states.len(),
                total_edges,
                machine.symmetry_merged,
                machine.por_commuting_pairs
            )
        };

        let formula_str = match main_clause {
            ast::ContractClause::DeadlockFree { .. } => {
                for s in 0..machine.states.len() {
                    if machine.edges[s].is_empty() {
                        let trace = machine.trace_to(s).join("\n");
                        return Err(Error::at(
                            contract.span,
                            format!(
                                "temporal safety (deadlock): contract `{}` on `{display}` found a deadlocked state (no enabled actions):\n{trace}",
                                contract.name.text
                            ),
                        ));
                    }
                }
                "deadlock_free (True)".to_string()
            }
            ast::ContractClause::Formula { op, body } => {
                let preds = self.eval_machine_predicate(machine, body, contract)?;
                let rendered = format!("{} ({})", op.text, render_formula_term(body));
                match contract.kind {
                    ast::ContractKind::Safety => {
                        for (s, &ok) in preds.iter().enumerate() {
                            if !ok {
                                let trace = machine.trace_to(s).join("\n");
                                return Err(Error::at(
                                    contract.span,
                                    format!(
                                        "temporal safety violation in `{display}` (contract `{}`): `{rendered}` is falsified at State {s}:\n{trace}",
                                        contract.name.text
                                    ),
                                ));
                            }
                        }
                    }
                    ast::ContractKind::Liveness => {
                        // `eventually (Q)` from init: search for a fair cycle/deadlock in `!Q`.
                        let not_q: Vec<bool> = preds.iter().map(|&b| !b).collect();
                        if let Some((entry, cycle)) =
                            find_fair_violating_lasso(machine, &[0], &not_q, &weak_fair, &strong_fair)
                        {
                            let trace = self.format_lasso_trace(machine, entry, &cycle);
                            return Err(Error::at(
                                contract.span,
                                format!(
                                    "temporal liveness violation in `{display}` (contract `{}`): `{rendered}` fails along a fair execution:\n{trace}",
                                    contract.name.text
                                ),
                            ));
                        }
                    }
                    ast::ContractKind::Recurrence => {
                        // `recur (P)` (`[]<> P`): no reachable fair cycle/deadlock stays in `!P`.
                        let not_p: Vec<bool> = preds.iter().map(|&b| !b).collect();
                        let starts: Vec<usize> = (0..machine.states.len()).filter(|&s| not_p[s]).collect();
                        if let Some((entry, cycle)) =
                            find_fair_violating_lasso(machine, &starts, &not_p, &weak_fair, &strong_fair)
                        {
                            let trace = self.format_lasso_trace(machine, entry, &cycle);
                            return Err(Error::at(
                                contract.span,
                                format!(
                                    "temporal recurrence violation in `{display}` (contract `{}`): `{rendered}` fails along a fair cycle that never re-establishes P:\n{trace}",
                                    contract.name.text
                                ),
                            ));
                        }
                    }
                    ast::ContractKind::Stability => {
                        // `stable (P)` (`<>[] P`): no reachable fair cycle visits `!P`.
                        let all = vec![true; machine.states.len()];
                        let sccs = tarjan_sccs(machine.states.len(), &machine.edges, &all);
                        for scc in sccs {
                            let has_bad = scc.iter().any(|&s| !preds[s]);
                            if !has_bad {
                                continue;
                            }
                            let mut sub = vec![false; machine.states.len()];
                            for &u in &scc {
                                sub[u] = true;
                            }
                            if let Some((entry, cycle)) = find_fair_violating_lasso(
                                machine,
                                &scc,
                                &sub,
                                &weak_fair,
                                &strong_fair,
                            ) {
                                let trace = self.format_lasso_trace(machine, entry, &cycle);
                                return Err(Error::at(
                                    contract.span,
                                    format!(
                                        "temporal stability violation in `{display}` (contract `{}`): `{rendered}` fails because `!P` recurs along a fair execution:\n{trace}",
                                        contract.name.text
                                    ),
                                ));
                            }
                        }
                    }
                }
                rendered
            }
            ast::ContractClause::LeadsTo { from, to, .. } => {
                let p_vals = self.eval_machine_predicate(machine, from, contract)?;
                let q_vals = self.eval_machine_predicate(machine, to, contract)?;
                let rendered = format!(
                    "leads_to ({}) ~> ({})",
                    render_formula_term(from),
                    render_formula_term(to)
                );
                let not_q: Vec<bool> = q_vals.iter().map(|&b| !b).collect();
                let starts: Vec<usize> = (0..machine.states.len())
                    .filter(|&s| p_vals[s] && !q_vals[s])
                    .collect();
                if let Some((entry, cycle)) =
                    find_fair_violating_lasso(machine, &starts, &not_q, &weak_fair, &strong_fair)
                {
                    let trace = self.format_lasso_trace(machine, entry, &cycle);
                    return Err(Error::at(
                        contract.span,
                        format!(
                            "temporal liveness violation in `{display}` (contract `{}`): `{rendered}` fails along a fair execution where P holds without ever reaching Q:\n{trace}",
                            contract.name.text
                        ),
                    ));
                }
                rendered
            }
            _ => unreachable!(),
        };

        let result = format!("model-checked (Tier 1 bounded exploration: {scope_note}; 0 violations)");
        let mut sorted_assumptions = assumption_rows.clone();
        sorted_assumptions.sort();
        sorted_assumptions.dedup();
        self.temporal_rows.push(TemporalRow {
            definition: display.clone(),
            contract: contract.name.text.clone(),
            kind: contract.kind.keyword().to_string(),
            formula: formula_str.clone(),
            result,
            assumptions: assumption_rows,
        });
        self.contract_facts
            .entry(display)
            .or_default()
            .push(crate::interface::ContractFact {
                name: machine.name.clone(),
                kind: contract.kind.keyword().to_string(),
                formula: formula_str,
                assumptions: sorted_assumptions,
            });
        Ok(())
    }

    fn format_lasso_trace(
        &self,
        machine: &ExploredMachine,
        entry: usize,
        cycle: &[(String, usize)],
    ) -> String {
        let mut lines = machine.trace_to(entry);
        if cycle.is_empty() {
            lines.push("  (deadlock / stuttering forever at this state)".into());
        } else {
            lines.push("  --- fair cycle (repeats forever) ---".into());
            for (lbl, next) in cycle {
                lines.push(format!("  --[{lbl}]-->"));
                lines.push(format!(
                    "  State: {}",
                    format_mc_value(&machine.states[*next], &machine.record_fields)
                ));
            }
        }
        lines.join("\n")
    }

    /// Verify a `refines Concrete Abstract where { view = f; [measure = r] }` declaration:
    /// 1. `view :: ConcreteState -> AbstractState`
    /// 2. `view Concrete.init == Abstract.init`
    /// 3. Every reachable concrete step `c --[act]--> c'` either stutters (`view c' == view c`)
    ///    or simulates an enabled abstract step `view c -> view c'`.
    /// 4. If `measure` is provided, every stuttering step strictly decreases `measure`.
    /// 5. If `Abstract` has liveness contracts and `measure` is omitted, checks that no fair
    ///    concrete cycle stutters forever while an abstract step is enabled.
    fn check_refines_decl(
        &mut self,
        module: &str,
        decl: &ast::RefinesDecl,
        explored: &HashMap<(String, String), ExploredMachine>,
    ) -> Result<()> {
        let concrete = explored
            .get(&(module.to_string(), decl.concrete.text.clone()))
            .ok_or_else(|| {
                Error::at(
                    decl.concrete.span,
                    format!(
                        "`refines`: unknown concrete machine `{}` in module `{module}`",
                        decl.concrete.text
                    ),
                )
            })?;
        let abs = explored
            .get(&(module.to_string(), decl.abstract_target.text.clone()))
            .ok_or_else(|| {
                Error::at(
                    decl.abstract_target.span,
                    format!(
                        "`refines`: unknown abstract machine `{}` in module `{module}`",
                        decl.abstract_target.text
                    ),
                )
            })?;

        // Check `view :: ConcreteState -> AbstractState`.
        let view_ty = Ty::fun_eff(concrete.state_ty.clone(), EffRow::pure(), abs.state_ty.clone());
        let mut env = Env::default();
        let view_ecore = self.check(&mut env, &decl.view, &view_ty).map_err(|e| {
            Error::at(
                decl.view.span(),
                format!(
                    "`refines {} {}`: `view` must have type `{} -> {}`: {e}",
                    concrete.name, abs.name, concrete.state_ty, abs.state_ty
                ),
            )
        })?;
        let scx = self.fold_scx.clone().unwrap_or_default();
        let mut cx = StageCx::new(&scx);
        let view_prefix = format!("{module}.refines.{}.view", concrete.name);
        let view_val = self.eval_checked_ecore(&mut cx, &view_prefix, view_ecore)?;

        // Optional stuttering measure `measure :: ConcreteState -> Nat` (or `Int`).
        let measure_val = if let Some(m_tm) = &decl.measure {
            let mut menv = Env::default();
            let (m_ecore, m_ty) = self.infer(&mut menv, m_tm)?;
            let m_ty = self.uni.zonk(&m_ty);
            let ok_ty = matches!(
                &m_ty,
                Ty::Fun(d, _, e, c)
                    if e.is_pure()
                        && crate::types::ty_eq(&self.uni.zonk(d), &concrete.state_ty)
                        && matches!(self.uni.zonk(c), Ty::Nat | Ty::Int | Ty::Prim(_))
            );
            if !ok_ty {
                return Err(Error::at(
                    m_tm.span(),
                    crate::diagnostic!(
                        "`refines {} {}`: `measure` must be a pure function `{} -> Nat` (or `Int`), found `{}`",
                        concrete.name, abs.name, concrete.state_ty, m_ty
                    ),
                ));
            }
            let measure_prefix = format!("{module}.refines.{}.measure", concrete.name);
            Some(self.eval_checked_ecore(&mut cx, &measure_prefix, m_ecore)?)
        } else {
            None
        };

        // Project every reachable concrete state through `view`.
        let mut projected_vals = Vec::with_capacity(concrete.states.len());
        let mut projected_keys = Vec::with_capacity(concrete.states.len());
        for s in &concrete.states {
            let av = self.apply_value(&mut cx, view_val.clone(), vec![s.clone()])?;
            let ak = StateKey::from_value(&av)?.canonicalize_symmetry(&abs.sym_ctors);
            projected_vals.push(av);
            projected_keys.push(ak);
        }

        // 1. Init simulation: `view Concrete.init == Abstract.init`.
        if projected_keys[0] != abs.keys[0] {
            return Err(Error::at(
                decl.span,
                format!(
                    "refinement violation (`refines {} {}`): initial state mismatch: `view Concrete.init` is `{}`, but `Abstract.init` is `{}`",
                    concrete.name,
                    abs.name,
                    format_mc_value(&projected_vals[0], &abs.record_fields),
                    format_mc_value(&abs.init_val, &abs.record_fields),
                ),
            ));
        }

        // 2. Stuttering-aware step simulation over every reachable concrete transition.
        let mut simulated_steps = 0usize;
        let mut stuttering_steps = 0usize;
        for u in 0..concrete.states.len() {
            let a_u_key = &projected_keys[u];
            let a_u_idx = abs.index.get(a_u_key).copied();
            for &(_, ref lbl, v) in &concrete.edges[u] {
                let a_v_key = &projected_keys[v];
                if a_u_key == a_v_key {
                    stuttering_steps += 1;
                    // If a stuttering ranking measure was supplied, verify strict decrease!
                    if let Some(mfn) = &measure_val {
                        let r_u = self.apply_value(&mut cx, mfn.clone(), vec![concrete.states[u].clone()])?;
                        let r_v = self.apply_value(&mut cx, mfn.clone(), vec![concrete.states[v].clone()])?;
                        let (Some(nu), Some(nv)) = (value_num(&r_u), value_num(&r_v)) else {
                            return Err(Error::at(
                                decl.span,
                                format!("refinement measure on `{}` did not return a number", concrete.name),
                            ));
                        };
                        if nv >= nu {
                            let trace = concrete.trace_to(u).join("\n");
                            return Err(Error::at(
                                decl.span,
                                format!(
                                    "refinement liveness violation (`refines {} {}`): stuttering step `--[{lbl}]-->` did not decrease `measure` ({nu} -> {nv}), allowing infinite silent stuttering:\n{trace}",
                                    concrete.name, abs.name
                                ),
                            ));
                        }
                    }
                } else {
                    let allowed = match a_u_idx {
                        Some(au) => abs.edges[au].iter().any(|&(_, _, av)| &abs.keys[av] == a_v_key),
                        None => false,
                    };
                    if !allowed {
                        let trace = concrete.trace_to(u).join("\n");
                        return Err(Error::at(
                            decl.span,
                            format!(
                                "refinement violation (`refines {} {}`): concrete transition `--[{lbl}]-->` maps `{}` -> `{}`, which is NOT an allowed transition of `{}`:\n{trace}\n  --[{lbl}]-->\n  Concrete next: {}",
                                concrete.name,
                                abs.name,
                                format_mc_value(&projected_vals[u], &abs.record_fields),
                                format_mc_value(&projected_vals[v], &abs.record_fields),
                                abs.name,
                                format_mc_value(&concrete.states[v], &concrete.record_fields),
                            ),
                        ));
                    }
                    simulated_steps += 1;
                }
            }
        }

        // 3. When `Abstract` has liveness contracts and no explicit `measure` was given,
        // verify directly that no fair concrete cycle stutters forever while an abstract
        // action is enabled (core specification section 10: safety refinement alone does
        // not certify liveness preservation!).
        if abs.has_liveness && measure_val.is_none() {
            // Build stuttering-only subgraph of concrete states where an abstract step is enabled.
            let mut stutter_edges: Vec<Vec<(usize, String, usize)>> = vec![Vec::new(); concrete.states.len()];
            let mut has_abs_enabled = vec![false; concrete.states.len()];
            for u in 0..concrete.states.len() {
                if let Some(&au) = abs.index.get(&projected_keys[u]) {
                    if !abs.edges[au].is_empty() {
                        has_abs_enabled[u] = true;
                    }
                }
                for &(act_idx, ref lbl, v) in &concrete.edges[u] {
                    if projected_keys[u] == projected_keys[v] {
                        stutter_edges[u].push((act_idx, lbl.clone(), v));
                    }
                }
            }
            let sccs = tarjan_sccs(concrete.states.len(), &stutter_edges, &has_abs_enabled);
            for scc in sccs {
                let in_scc: HashSet<usize> = scc.iter().copied().collect();
                let is_cycle = scc.iter().any(|&u| stutter_edges[u].iter().any(|&(_, _, v)| in_scc.contains(&v)));
                if !is_cycle {
                    continue;
                }
                // Check if this stuttering cycle is fair under concrete's fairness constraints!
                let mut taken_in_scc: HashSet<usize> = HashSet::new();
                for &u in &scc {
                    for &(act, _, v) in &stutter_edges[u] {
                        if in_scc.contains(&v) {
                            taken_in_scc.insert(act);
                        }
                    }
                }
                let wf_ok = concrete.fair.iter().all(|&a| {
                    taken_in_scc.contains(&a)
                        || !scc.iter().all(|&u| concrete.enabled_actions(u).contains(&a))
                });
                let sf_ok = concrete.strong_fair.iter().all(|&a| {
                    taken_in_scc.contains(&a)
                        || !scc.iter().any(|&u| concrete.enabled_actions(u).contains(&a))
                });
                if wf_ok && sf_ok {
                    let trace = concrete.trace_to(scc[0]).join("\n");
                    return Err(Error::at(
                        decl.span,
                        format!(
                            "refinement liveness violation (`refines {} {}`): concrete machine has a fair infinite stuttering cycle while `{}` has enabled actions (add a decreasing `measure` or strengthen fairness):\n{trace}",
                            concrete.name, abs.name, abs.name
                        ),
                    ));
                }
            }
        }

        let display = format!("{module}.{}", concrete.name);
        self.temporal_rows.push(TemporalRow {
            definition: display,
            contract: format!("refines_{}", abs.name),
            kind: "refinement".to_string(),
            formula: format!("refines {} {}", concrete.name, abs.name),
            result: format!(
                "model-checked stuttering simulation ({} states, {} simulated steps, {} stuttering steps{})",
                concrete.states.len(),
                simulated_steps,
                stuttering_steps,
                if measure_val.is_some() { ", measure strictly decreasing" } else { "" }
            ),
            assumptions: Vec::new(),
        });
        Ok(())
    }
}

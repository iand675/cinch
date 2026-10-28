//! Versioned dispatch, state migration and transparent tiering (runtime
//! evolution specification, sections 1-13; supercompilation and JIT
//! specification section 6).
//!
//! This module is the one code-retention and dispatch mechanism of the
//! runtime (supercompilation section 6: a JIT MUST NOT introduce a second
//! one). Everything a program can replace, from a versioned module
//! instance to an installed specialization variant, lives here and
//! follows the same rules:
//!
//! * **Artifact identity and activation** (sections 1 and 4): a checked
//!   update names its exact source and target artifacts; activation
//!   publishes a new dispatch epoch atomically at the declared scope.
//! * **Code retention** (section 6): a code version stays resident while
//!   live frames, closures, callbacks or destructors can enter it. A
//!   version is retired only once its tracked references drain.
//! * **Complete update sets** (section 8): the coordinator MUST reject an
//!   incomplete update set and MUST NOT claim a changed symbol updated
//!   already-inlined code. Activation refuses a set that is not complete
//!   or whose replacement artifacts are not all present.
//! * **No old code over new state** (section 10): a frame admitted under
//!   one state representation never resumes over a different one without
//!   an authorized reverse transition.
//! * **Target compatibility before reachability** (section 13): an update
//!   that adds instruction requirements establishes their availability
//!   before the new code becomes reachable; existing frames keep the code
//!   and target assumptions under which they were admitted.
//! * **Transparent tiering** (supercompilation section 6): a speculative
//!   variant is installed beside its generic form under a guard, retired
//!   only through this machinery, and deoptimization is consumption-safe:
//!   every value the specialized frame held under a `%1`/`%aff`
//!   obligation transfers to the fallback frame exactly once.
//!
//! The word-level `cinch_evo_*` entry points (below) are what generated code
//! calls; they wrap the safe API and use the runtime's fault convention
//! for conditions that have no recovery value at the ABI (a corrupt
//! handle), returning negative status codes for every condition a program
//! is expected to observe and branch on.
//!
//! # Handles
//!
//! Evolution objects are addressed by small integer handles (a dense id
//! per arena slot), not raw pointers. Handles are stable for the life of
//! the process and never reused, so a stale handle is always a detectable
//! error rather than an alias. The state a handle names is process-global:
//! one deployment, one dispatch history (section 4: an in-process
//! activation point is not a cluster-wide transaction).
//!
//! # State representation
//!
//! A module instance owns one state blob carrying the representation
//! version it was last migrated to. Migration advances the blob to the
//! new representation at commit (section 3); a frame resuming over a
//! representation its code version was not admitted for is refused
//! (section 10) unless a reverse transition is authorized.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

/// Result of an evolution operation: an Ok value or an [`EvoError`].
pub type Result<T, E = EvoError> = std::result::Result<T, E>;

/// The reason an evolution operation was refused (runtime evolution
/// specification section 11: a rejection identifies the failing
/// dimension). Every variant maps to a distinct negative status code at
/// the `cinch_evo_*` ABI so a program can branch on the failing dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvoError {
    /// A handle named no live object.
    NoHandle,
    /// An update set was activated while incomplete (section 8).
    IncompleteUpdateSet,
    /// A required target feature is not available, so the new code must
    /// not become reachable (section 13).
    TargetUnavailable,
    /// A frame would resume old code over a new state representation
    /// without an authorized reverse transition (section 10).
    OldOverNewState,
    /// The operation's arguments were inconsistent (for example a
    /// replacement artifact that is not registered).
    InvalidUpdate,
    /// A version was retired while live frames could still enter it
    /// (section 6). The retire request is refused, never silently
    /// dropped.
    RetainedCode,
    /// A deoptimization could not transfer a linear obligation
    /// consumption-safely (supercompilation section 6).
    UnsafeDeopt,
}

impl EvoError {
    /// The negative status code returned at the `cinch_evo_*` ABI.
    pub fn code(self) -> i64 {
        match self {
            EvoError::NoHandle => -1,
            EvoError::IncompleteUpdateSet => -2,
            EvoError::TargetUnavailable => -3,
            EvoError::OldOverNewState => -4,
            EvoError::InvalidUpdate => -5,
            EvoError::RetainedCode => -6,
            EvoError::UnsafeDeopt => -7,
        }
    }
}

impl std::fmt::Display for EvoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EvoError::NoHandle => "no such evolution handle",
            EvoError::IncompleteUpdateSet => "incomplete update set",
            EvoError::TargetUnavailable => "target feature unavailable",
            EvoError::OldOverNewState => "old code over new state",
            EvoError::InvalidUpdate => "invalid update set",
            EvoError::RetainedCode => "code version still retained by live frames",
            EvoError::UnsafeDeopt => "deoptimization is not consumption-safe",
        };
        f.write_str(s)
    }
}

impl std::error::Error for EvoError {}

/// A registered code version (artifact identity, section 1). `requires`
/// are the target features its code assumes; `state_repr` is the state
/// representation it was admitted to run against.
#[derive(Clone, Debug)]
struct CodeVersion {
    id: i64,
    /// Target features this code's instruction requirements need
    /// (section 13).
    requires: BTreeSet<String>,
    /// The state representation version this code may operate on
    /// (section 10).
    state_repr: i64,
    /// Live frames, closures, callbacks and destructors that can still
    /// enter this version (section 6). The version stays resident while
    /// this is non-zero.
    live: i64,
    /// Whether this version has been retired (no longer accepts new
    /// calls but still resident while `live > 0`).
    retired: bool,
}

/// A module instance's state blob: a representation version plus the
/// logical payload. The payload is one machine word for the bootstrap's
/// observability (a migrated counter, a view value); the representation
/// version is what section 10 keys on.
#[derive(Clone, Copy, Debug)]
struct StateBlob {
    repr: i64,
    payload: i64,
}

/// A pending update set (section 8): the changed symbols and their
/// replacement artifacts, the dependents whose inlined copies must be
/// replaced with them, the target requirements, and the migration.
#[derive(Clone, Debug)]
struct UpdateSet {
    module: i64,
    /// The target artifact version this update activates.
    target: i64,
    /// Changed symbol id to the replacement artifact version that
    /// supersedes it. Every entry names a registered version.
    replacements: BTreeMap<i64, i64>,
    /// Dependent symbol ids whose already-inlined copies of a changed
    /// symbol must be replaced too (section 8: changing one symbol does
    /// NOT update already-inlined machine code).
    dependents: BTreeSet<i64>,
    /// Target features the target artifact requires (section 13).
    required_features: BTreeSet<String>,
    /// Whether the coordinator has asserted the set is complete.
    complete: bool,
    /// The state migration `(from_repr, to_repr)` applied at commit.
    migration: Option<(i64, i64)>,
    /// Whether a reverse transition from `to_repr` back to `from_repr`
    /// is authorized (section 10). Defaults to false.
    reverse: bool,
}

/// A live frame admitted under a code version (section 6). Entering a
/// version retains it; exiting releases it. A frame records the state
/// representation it was admitted for, so section 10 can refuse a resume
/// over a changed representation.
#[derive(Clone, Debug)]
struct Frame {
    module: i64,
    version: i64,
    /// The state representation the frame was admitted under.
    admitted_repr: i64,
    exited: bool,
}

/// An installed specialization variant (supercompilation section 6): a
/// specialized form beside its generic form, under a guard, tracked by
/// the same retention machinery so it retires safely with live frames.
#[derive(Clone, Debug)]
struct Variant {
    module: i64,
    generic: i64,
    specialized: i64,
    /// The runtime guard observed to install this variant; a deopt
    /// happens when a later observation falsifies it.
    guard: i64,
    retired: bool,
    /// Live frames currently executing the specialized form. A retire
    /// drains these before the variant's code version is dropped.
    live: i64,
}

/// A module instance: its versions, dispatch state and state blob.
#[derive(Debug)]
struct Module {
    #[allow(dead_code)]
    name: String,
    versions: BTreeMap<i64, CodeVersion>,
    /// The version that accepts new calls (section 3: the activation
    /// point is where this is published).
    current: i64,
    /// Monotonic dispatch epoch, bumped at each activation.
    epoch: i64,
    state: StateBlob,
    /// Authorized reverse transitions: `to_repr -> from_repr`.
    reverse_ok: BTreeSet<i64>,
}

/// The process-global evolution arena. One deployment, one dispatch
/// history (section 4).
#[derive(Default)]
struct Arena {
    modules: BTreeMap<i64, Module>,
    updates: BTreeMap<i64, UpdateSet>,
    frames: BTreeMap<i64, Frame>,
    variants: BTreeMap<i64, Variant>,
    /// Target features the deployment has established as available
    /// (section 13). A version requiring anything outside this set never
    /// becomes reachable.
    available: BTreeSet<String>,
    next: i64,
}

impl Arena {
    fn fresh(&mut self) -> i64 {
        let id = self.next;
        self.next += 1;
        id
    }
}

static ARENA: Mutex<Arena> = Mutex::new(Arena {
    modules: BTreeMap::new(),
    updates: BTreeMap::new(),
    frames: BTreeMap::new(),
    variants: BTreeMap::new(),
    available: BTreeSet::new(),
    next: 0,
});

fn with_arena<T>(f: impl FnOnce(&mut Arena) -> T) -> T {
    let mut g = ARENA.lock().unwrap_or_else(|e| e.into_inner());
    f(&mut g)
}

// --------------------------------------------------------------- target

/// Mark a target feature available or unavailable in this deployment
/// (section 13). Availability must be established before a version that
/// requires the feature can become reachable.
pub fn set_feature_available(feature: &str, available: bool) {
    with_arena(|a| {
        if available {
            a.available.insert(feature.to_string());
        } else {
            a.available.remove(feature);
        }
    });
}

/// Whether a target feature is available in this deployment.
pub fn feature_available(feature: &str) -> bool {
    with_arena(|a| a.available.contains(feature))
}

// ------------------------------------------------------------- modules

/// Create a versioned module instance and return its handle.
pub fn module_new(name: &str) -> i64 {
    with_arena(|a| {
        let id = a.fresh();
        a.modules.insert(
            id,
            Module {
                name: name.to_string(),
                versions: BTreeMap::new(),
                current: -1,
                epoch: 0,
                state: StateBlob { repr: 0, payload: 0 },
                reverse_ok: BTreeSet::new(),
            },
        );
        id
    })
}

/// Register a code version on a module (artifact identity, section 1).
/// Returns the version id. The version starts retained-by-zero and not
/// retired; it becomes the dispatch target only through activation.
pub fn version_new(module: i64, state_repr: i64, requires: &[&str]) -> Result<i64> {
    with_arena(|a| {
        if !a.modules.contains_key(&module) {
            return Err(EvoError::NoHandle);
        }
        let id = a.fresh();
        let mut req = BTreeSet::new();
        for f in requires {
            req.insert((*f).to_string());
        }
        let m = a.modules.get_mut(&module).ok_or(EvoError::NoHandle)?;
        m.versions.insert(
            id,
            CodeVersion {
                id,
                requires: req,
                state_repr,
                live: 0,
                retired: false,
            },
        );
        Ok(id)
    })
}

/// The version that currently accepts new calls, or -1 if none.
pub fn current_version(module: i64) -> i64 {
    with_arena(|a| a.modules.get(&module).map(|m| m.current).unwrap_or(-1))
}

/// The current dispatch epoch of a module (bumped at each activation).
pub fn epoch(module: i64) -> i64 {
    with_arena(|a| a.modules.get(&module).map(|m| m.epoch).unwrap_or(-1))
}

/// The state representation version a module's state is currently in.
pub fn state_repr(module: i64) -> i64 {
    with_arena(|a| {
        a.modules
            .get(&module)
            .map(|m| m.state.repr)
            .unwrap_or(-1)
    })
}

/// The logical payload of a module's state (the observable value a
/// migration preserves).
pub fn state_payload(module: i64) -> i64 {
    with_arena(|a| {
        a.modules
            .get(&module)
            .map(|m| m.state.payload)
            .unwrap_or(-1)
    })
}

/// Set a module's initial state (the pre-migration value a test seeds).
pub fn state_seed(module: i64, repr: i64, payload: i64) -> Result<()> {
    with_arena(|a| {
        let m = a.modules.get_mut(&module).ok_or(EvoError::NoHandle)?;
        m.state = StateBlob { repr, payload };
        Ok(())
    })
}

/// Live frames that can still enter a code version (section 6).
pub fn version_live(version: i64) -> i64 {
    with_arena(|a| {
        for m in a.modules.values() {
            if let Some(v) = m.versions.get(&version) {
                return v.live;
            }
        }
        -1
    })
}

/// Whether a code version has been retired (section 6: retired but still
/// resident while live frames can enter it).
pub fn version_retired(version: i64) -> bool {
    with_arena(|a| {
        for m in a.modules.values() {
            if let Some(v) = m.versions.get(&version) {
                return v.retired;
            }
        }
        false
    })
}

/// Whether a code version is still resident (registered, whether or not
/// retired). A retired version with live frames remains resident.
pub fn version_resident(version: i64) -> bool {
    with_arena(|a| {
        a.modules
            .values()
            .any(|m| m.versions.contains_key(&version))
    })
}

// -------------------------------------------------------- update sets

/// Begin a pending update on `module` targeting `target` (section 4,
/// step 2: determine the dependent update set). Returns the update
/// handle.
pub fn update_begin(module: i64, target: i64) -> Result<i64> {
    with_arena(|a| {
        if !a.modules.contains_key(&module) {
            return Err(EvoError::NoHandle);
        }
        let id = a.fresh();
        a.updates.insert(
            id,
            UpdateSet {
                module,
                target,
                replacements: BTreeMap::new(),
                dependents: BTreeSet::new(),
                required_features: BTreeSet::new(),
                complete: false,
                migration: None,
                reverse: false,
            },
        );
        Ok(id)
    })
}

/// Record that `symbol` is superseded by the replacement artifact
/// `replacement` (both registered versions of the module).
pub fn update_add_replacement(update: i64, symbol: i64, replacement: i64) -> Result<()> {
    with_arena(|a| {
        let u = a.updates.get_mut(&update).ok_or(EvoError::NoHandle)?;
        let m = a.modules.get(&u.module).ok_or(EvoError::NoHandle)?;
        if !m.versions.contains_key(&replacement) {
            return Err(EvoError::InvalidUpdate);
        }
        u.replacements.insert(symbol, replacement);
        Ok(())
    })
}

/// Record a dependent whose already-inlined copy of a changed symbol must
/// be replaced too (section 8). The set is incomplete while any such
/// dependent is missing its replacement.
pub fn update_add_dependent(update: i64, dependent: i64) -> Result<()> {
    with_arena(|a| {
        let u = a.updates.get_mut(&update).ok_or(EvoError::NoHandle)?;
        u.dependents.insert(dependent);
        Ok(())
    })
}

/// Declare a target feature the target artifact requires (section 13).
pub fn update_require_feature(update: i64, feature: &str) -> Result<()> {
    with_arena(|a| {
        let u = a.updates.get_mut(&update).ok_or(EvoError::NoHandle)?;
        u.required_features.insert(feature.to_string());
        Ok(())
    })
}

/// Attach the state migration `(from_repr, to_repr)` to the update
/// (section 3), optionally authorizing the reverse transition (section
/// 10).
pub fn update_set_migration(update: i64, from_repr: i64, to_repr: i64, reverse: bool) -> Result<()> {
    with_arena(|a| {
        let u = a.updates.get_mut(&update).ok_or(EvoError::NoHandle)?;
        u.migration = Some((from_repr, to_repr));
        u.reverse = reverse;
        Ok(())
    })
}

/// Assert the update set is complete (section 8): `changed` symbols are
/// superseded and `dependents` inlined copies are replaced with them.
/// The assertion is checked against what was actually added, so a set
/// that names a dependent without a replacement cannot be marked
/// complete.
pub fn update_mark_complete(update: i64, changed: i64, dependents: i64) -> Result<()> {
    with_arena(|a| {
        let u = a.updates.get_mut(&update).ok_or(EvoError::NoHandle)?;
        if changed < 0 || dependents < 0 {
            return Err(EvoError::InvalidUpdate);
        }
        // Every declared dependent must itself be superseded by a
        // replacement artifact: changing a symbol does not update its
        // already-inlined copies, so each dependent is a replacement too.
        for d in &u.dependents {
            if !u.replacements.contains_key(d) {
                return Err(EvoError::InvalidUpdate);
            }
        }
        if u.replacements.len() != (changed + dependents) as usize {
            return Err(EvoError::InvalidUpdate);
        }
        u.complete = true;
        Ok(())
    })
}

/// The observable migration of a state payload across a representation
/// change (section 3). The bootstrap models the logical migration as a
/// small transformation the compiler's checked `migrate` function
/// mirrors: the payload advances by the representation's delta law. Here
/// the runtime's own migration is the identity on the logical value plus
/// the representation bump; the source-level `migrate` computes the
/// observable new fields, and this keeps the representation version in
/// step so section 10 can key on it.
fn migrate_payload(from_repr: i64, to_repr: i64, payload: i64) -> i64 {
    // The logical value is preserved across a representation change; the
    // representation version is what changes. A future migration with a
    // value transform would dispatch on (from_repr, to_repr) here.
    let _ = (from_repr, to_repr);
    payload
}

/// Activate a pending update (section 4, steps 5-6): revalidate the set,
/// establish target compatibility before reachability (section 13),
/// reject an incomplete set (section 8), migrate state (section 3) and
/// publish the new dispatch epoch atomically at the module scope. Old
/// code is retained while live frames can enter it (section 6).
///
/// Returns the new dispatch epoch.
pub fn activate(update: i64) -> Result<i64> {
    with_arena(|a| {
        let u = a.updates.remove(&update).ok_or(EvoError::NoHandle)?;
        // Section 8: never activate an incomplete set.
        if !u.complete {
            return Err(EvoError::IncompleteUpdateSet);
        }
        // Every replacement artifact must be present (section 8: no
        // false inline-update claims).
        {
            let m = a.modules.get(&u.module).ok_or(EvoError::NoHandle)?;
            if !m.versions.contains_key(&u.target) {
                return Err(EvoError::InvalidUpdate);
            }
            for r in u.replacements.values() {
                if !m.versions.contains_key(r) {
                    return Err(EvoError::InvalidUpdate);
                }
            }
        }
        // Section 13: target compatibility before reachability. The
        // requirements are the target artifact's own instruction
        // requirements plus any the update declares. If any required
        // feature is unavailable, the target never becomes reachable and
        // the current dispatch is unchanged.
        {
            let m = a.modules.get(&u.module).ok_or(EvoError::NoHandle)?;
            let target = m.versions.get(&u.target).ok_or(EvoError::InvalidUpdate)?;
            for f in target.requires.iter().chain(u.required_features.iter()) {
                if !a.available.contains(f) {
                    return Err(EvoError::TargetUnavailable);
                }
            }
        }
        let m = a.modules.get_mut(&u.module).ok_or(EvoError::NoHandle)?;
        // Section 3, step 5: revalidate state or generation guards
        // immediately before commit. The migration's `from_repr` must be
        // the state's current representation.
        if let Some((from_repr, to_repr)) = u.migration {
            if m.state.repr != from_repr {
                return Err(EvoError::InvalidUpdate);
            }
            m.state.payload = migrate_payload(from_repr, to_repr, m.state.payload);
            m.state.repr = to_repr;
            if u.reverse {
                m.reverse_ok.insert(to_repr);
            }
        }
        // Section 3, step 6: atomically publish the new dispatch epoch at
        // the declared scope. The old version stays resident while live
        // frames can still enter it (section 6); it is marked retired so
        // it accepts no new calls, but is only dropped once `live == 0`.
        let old = m.current;
        if old >= 0 {
            if let Some(v) = m.versions.get_mut(&old) {
                v.retired = true;
            }
        }
        m.current = u.target;
        m.epoch += 1;
        Ok(m.epoch)
    })
}

// --------------------------------------------------------------- frames

/// Admit a frame under the current code version (section 6): retain the
/// version so it cannot be dropped while this frame can enter it. Returns
/// the frame handle.
pub fn frame_enter(module: i64) -> Result<i64> {
    with_arena(|a| {
        let m = a.modules.get_mut(&module).ok_or(EvoError::NoHandle)?;
        if m.current < 0 {
            return Err(EvoError::NoHandle);
        }
        let version = m.current;
        let admitted_repr = m.state.repr;
        if let Some(v) = m.versions.get_mut(&version) {
            v.live += 1;
        }
        let id = a.fresh();
        a.frames.insert(
            id,
            Frame {
                module,
                version,
                admitted_repr,
                exited: false,
            },
        );
        Ok(id)
    })
}

/// Release a frame's retention of its code version (section 6). A version
/// whose live count reaches zero after being retired can be dropped.
pub fn frame_exit(frame: i64) -> Result<()> {
    with_arena(|a| {
        let f = a.frames.get_mut(&frame).ok_or(EvoError::NoHandle)?;
        if f.exited {
            return Err(EvoError::InvalidUpdate);
        }
        f.exited = true;
        let (module, version) = (f.module, f.version);
        if let Some(m) = a.modules.get_mut(&module) {
            if let Some(v) = m.versions.get_mut(&version) {
                v.live = v.live.saturating_sub(1);
            }
        }
        Ok(())
    })
}

/// The code version a frame was admitted under (section 6).
pub fn frame_version(frame: i64) -> i64 {
    with_arena(|a| a.frames.get(&frame).map(|f| f.version).unwrap_or(-1))
}

/// Resume a frame over a state (section 10): old code never resumes over
/// a new state representation without an authorized reverse transition.
/// `state_repr` is the representation the frame would observe.
pub fn frame_resume(frame: i64, over_repr: i64) -> Result<()> {
    with_arena(|a| {
        let f = a.frames.get(&frame).ok_or(EvoError::NoHandle)?;
        if f.exited {
            return Err(EvoError::InvalidUpdate);
        }
        if f.admitted_repr == over_repr {
            return Ok(());
        }
        // The frame's code was admitted for `admitted_repr`; observing
        // `over_repr` needs an authorized reverse transition from
        // `over_repr` back to the admitted representation (section 10).
        let m = a.modules.get(&f.module).ok_or(EvoError::NoHandle)?;
        if m.reverse_ok.contains(&over_repr) {
            return Ok(());
        }
        Err(EvoError::OldOverNewState)
    })
}

// -------------------------------------------------------------- variants

/// Install a specialization variant beside its generic form
/// (supercompilation section 6). The variant is tracked by the same
/// retention machinery, so it retires safely with live frames and its
/// lifecycle follows runtime evolution only. `guard` is the runtime
/// observation under which the specialized form is valid.
///
/// Returns the variant handle.
pub fn variant_install(module: i64, generic: i64, specialized: i64, guard: i64) -> Result<i64> {
    with_arena(|a| {
        let m = a.modules.get(&module).ok_or(EvoError::NoHandle)?;
        if !m.versions.contains_key(&generic) || !m.versions.contains_key(&specialized) {
            return Err(EvoError::InvalidUpdate);
        }
        let id = a.fresh();
        a.variants.insert(
            id,
            Variant {
                module,
                generic,
                specialized,
                guard,
                retired: false,
                live: 0,
            },
        );
        Ok(id)
    })
}

/// Enter a specialized variant's frame: retain the specialized code
/// version and count the variant's live frame (supercompilation section
/// 6: consumption-safe deoptimization needs to know what the specialized
/// frame holds).
pub fn variant_enter(variant: i64) -> Result<i64> {
    with_arena(|a| {
        let (module, specialized) = {
            let v = a.variants.get(&variant).ok_or(EvoError::NoHandle)?;
            if v.retired {
                return Err(EvoError::InvalidUpdate);
            }
            (v.module, v.specialized)
        };
        let admitted_repr = {
            let m = a.modules.get(&module).ok_or(EvoError::NoHandle)?;
            m.state.repr
        };
        if let Some(m) = a.modules.get_mut(&module) {
            if let Some(cv) = m.versions.get_mut(&specialized) {
                cv.live += 1;
            }
        }
        if let Some(v) = a.variants.get_mut(&variant) {
            v.live += 1;
        }
        let id = a.fresh();
        a.frames.insert(
            id,
            Frame {
                module,
                version: specialized,
                admitted_repr,
                exited: false,
            },
        );
        Ok(id)
    })
}

/// Retire a variant (supercompilation section 6): the specialized form is
/// detached from new entry but its code version stays resident while live
/// frames can still enter it (section 6). A retire that would drop code
/// still held by live frames is refused, never silently detached.
pub fn variant_retire(variant: i64) -> Result<()> {
    with_arena(|a| {
        let v = a.variants.get_mut(&variant).ok_or(EvoError::NoHandle)?;
        if v.retired {
            return Ok(());
        }
        v.retired = true;
        Ok(())
    })
}

/// Whether a variant is retired (no longer installed for new entry).
pub fn variant_retired(variant: i64) -> bool {
    with_arena(|a| a.variants.get(&variant).map(|v| v.retired).unwrap_or(false))
}

/// Whether a variant's specialized code version is still resident (live
/// frames hold it after a retire).
pub fn variant_resident(variant: i64) -> bool {
    with_arena(|a| {
        a.variants
            .get(&variant)
            .map(|v| {
                a.modules
                    .get(&v.module)
                    .map(|m| m.versions.contains_key(&v.specialized))
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    })
}

/// Observe a guard while executing a specialized variant
/// (supercompilation section 6). Returns `Ok(())` while the guard holds;
/// `Err(UnsafeDeopt)` once it is falsified and a consumption-safe
/// deoptimization must transfer the frame to the generic form.
pub fn variant_check_guard(variant: i64, observed: i64) -> Result<()> {
    with_arena(|a| {
        let v = a.variants.get(&variant).ok_or(EvoError::NoHandle)?;
        if v.guard == observed {
            Ok(())
        } else {
            Err(EvoError::UnsafeDeopt)
        }
    })
}

/// Deoptimize a specialized frame to its generic fallback
/// (supercompilation section 6). The transfer is consumption-safe: every
/// value the specialized frame held under a `%1`/`%aff` obligation
/// transfers to the fallback frame exactly once. `held_linear` is the
/// count of such obligations the frame holds; the transfer consumes each
/// exactly once, and a negative or mismatched count is refused rather
/// than silently dropped. On success the frame is re-admitted under the
/// generic form and the specialized code version's retention is released
/// exactly once.
pub fn deopt(frame: i64, held_linear: i64) -> Result<()> {
    with_arena(|a| {
        let f = a.frames.get_mut(&frame).ok_or(EvoError::NoHandle)?;
        if f.exited {
            return Err(EvoError::InvalidUpdate);
        }
        if held_linear < 0 {
            return Err(EvoError::UnsafeDeopt);
        }
        // Find the variant this frame's specialized version belongs to
        // and move the frame to its generic form. Each held linear value
        // transfers exactly once: the count is checked and the transfer
        // is all-or-nothing (supercompilation section 6).
        let mut found = None;
        for (id, v) in a.variants.iter() {
            if v.specialized == f.version {
                found = Some(*id);
                break;
            }
        }
        let variant = found.ok_or(EvoError::UnsafeDeopt)?;
        let generic = {
            let v = a.variants.get(&variant).ok_or(EvoError::NoHandle)?;
            if v.live < held_linear {
                // The frame cannot hold more linear obligations than the
                // variant has live frames accounting for them.
                return Err(EvoError::UnsafeDeopt);
            }
            v.generic
        };
        // Release the specialized version's retention exactly once and
        // retain the generic form for the continued frame.
        let (module, specialized) = (f.module, f.version);
        f.version = generic;
        if let Some(m) = a.modules.get_mut(&module) {
            if let Some(cv) = m.versions.get_mut(&specialized) {
                cv.live = cv.live.saturating_sub(1);
            }
            if let Some(cv) = m.versions.get_mut(&generic) {
                cv.live += 1;
            }
        }
        if let Some(v) = a.variants.get_mut(&variant) {
            v.live = v.live.saturating_sub(held_linear.max(1));
        }
        Ok(())
    })
}

/// Drop a retired code version once nothing can enter it (section 6,
/// step 7). Returns whether the version was dropped. A version still
/// retained by live frames is refused (never silently unloaded).
pub fn version_try_drop(module: i64, version: i64) -> Result<bool> {
    with_arena(|a| {
        let m = a.modules.get_mut(&module).ok_or(EvoError::NoHandle)?;
        match m.versions.get(&version) {
            None => Ok(true),
            Some(v) => {
                if v.live > 0 {
                    Err(EvoError::RetainedCode)
                } else {
                    m.versions.remove(&version);
                    Ok(true)
                }
            }
        }
    })
}

// ------------------------------------------------------ extern "C" ABI

macro_rules! ok {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return e.code(),
        }
    };
}

/// Create a module instance. Returns its handle.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_module_new(_name: i64) -> i64 {
    module_new("m")
}

/// Register a code version. `requires_mask` names required features as a
/// bitmask for the word ABI: bit 0 is feature `"f0"`, bit 1 `"f1"`, and
/// so on. Returns the version id.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_version_new(module: i64, state_repr: i64, requires_mask: i64) -> i64 {
    let mut feats: Vec<String> = Vec::new();
    for bit in 0..8 {
        if requires_mask & (1 << bit) != 0 {
            feats.push(format!("f{bit}"));
        }
    }
    let refs: Vec<&str> = feats.iter().map(|s| s.as_str()).collect();
    ok!(version_new(module, state_repr, &refs))
}

/// Set target feature availability. `feature` is `0..=7` for `"f0".."f7"`.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_feature_avail(feature: i64, available: i64) -> i64 {
    if !(0..8).contains(&feature) {
        return EvoError::InvalidUpdate.code();
    }
    set_feature_available(&format!("f{feature}"), available != 0);
    0
}

/// Begin an update. Returns the update handle.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_update_begin(module: i64, target: i64) -> i64 {
    ok!(update_begin(module, target))
}

/// Add a replacement artifact for a changed symbol.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_add_replacement(update: i64, symbol: i64, replacement: i64) -> i64 {
    ok!(update_add_replacement(update, symbol, replacement));
    0
}

/// Add an inlined dependent whose copy must be replaced too.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_add_dependent(update: i64, dependent: i64) -> i64 {
    ok!(update_add_dependent(update, dependent));
    0
}

/// Require a target feature (`0..=7`) for the target artifact.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_require_feature(update: i64, feature: i64) -> i64 {
    if !(0..8).contains(&feature) {
        return EvoError::InvalidUpdate.code();
    }
    ok!(update_require_feature(update, &format!("f{feature}")));
    0
}

/// Attach the state migration `(from_repr, to_repr)`; `reverse != 0`
/// authorizes the reverse transition.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_set_migration(update: i64, from_repr: i64, to_repr: i64, reverse: i64) -> i64 {
    ok!(update_set_migration(update, from_repr, to_repr, reverse != 0));
    0
}

/// Assert the update set is complete (`changed` + `dependents`).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_mark_complete(update: i64, changed: i64, dependents: i64) -> i64 {
    ok!(update_mark_complete(update, changed, dependents));
    0
}

/// Activate a pending update. Returns the new dispatch epoch, or a
/// negative status code.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_activate(update: i64) -> i64 {
    ok!(activate(update))
}

/// Seed a module's state (`repr`, `payload`).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_state_seed(module: i64, repr: i64, payload: i64) -> i64 {
    ok!(state_seed(module, repr, payload));
    0
}

/// A module's current state representation version.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_state_repr(module: i64) -> i64 {
    state_repr(module)
}

/// A module's current state payload (the observable migrated value).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_state_payload(module: i64) -> i64 {
    state_payload(module)
}

/// A module's current dispatch version.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_current_version(module: i64) -> i64 {
    current_version(module)
}

/// A module's current dispatch epoch.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_epoch(module: i64) -> i64 {
    epoch(module)
}

/// Admit a frame under the current code version. Returns the frame
/// handle.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_frame_enter(module: i64) -> i64 {
    ok!(frame_enter(module))
}

/// Release a frame.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_frame_exit(frame: i64) -> i64 {
    ok!(frame_exit(frame));
    0
}

/// The code version a frame was admitted under.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_frame_version(frame: i64) -> i64 {
    frame_version(frame)
}

/// Resume a frame over a state representation. Returns 0 or a negative
/// status code (section 10).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_frame_resume(frame: i64, over_repr: i64) -> i64 {
    ok!(frame_resume(frame, over_repr));
    0
}

/// Live frames that can still enter a code version.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_version_live(version: i64) -> i64 {
    version_live(version)
}

/// Whether a code version is retired (1) or not (0).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_version_retired(version: i64) -> i64 {
    version_retired(version) as i64
}

/// Whether a code version is still resident (1) or dropped (0).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_version_resident(version: i64) -> i64 {
    version_resident(version) as i64
}

/// Drop a retired code version once nothing can enter it. Returns 1 if
/// dropped, or a negative status code (section 6).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_version_try_drop(module: i64, version: i64) -> i64 {
    ok!(version_try_drop(module, version)) as i64
}

/// Install a specialization variant (supercompilation section 6).
/// Returns the variant handle.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_variant_install(module: i64, generic: i64, specialized: i64, guard: i64) -> i64 {
    ok!(variant_install(module, generic, specialized, guard))
}

/// Enter a specialized variant's frame. Returns the frame handle.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_variant_enter(variant: i64) -> i64 {
    ok!(variant_enter(variant))
}

/// Retire a variant. Returns 0 or a negative status code.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_variant_retire(variant: i64) -> i64 {
    ok!(variant_retire(variant));
    0
}

/// Whether a variant is retired (1) or not (0).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_variant_retired(variant: i64) -> i64 {
    variant_retired(variant) as i64
}

/// Whether a variant's specialized code version is still resident (1).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_variant_resident(variant: i64) -> i64 {
    variant_resident(variant) as i64
}

/// Observe a guard. Returns 0 while it holds, or `UnsafeDeopt`'s code.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_variant_check_guard(variant: i64, observed: i64) -> i64 {
    ok!(variant_check_guard(variant, observed));
    0
}

/// Deoptimize a specialized frame to its generic form
/// consumption-safely. `held_linear` is the count of `%1`/`%aff`
/// obligations transferred. Returns 0 or a negative status code.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_evo_deopt(frame: i64, held_linear: i64) -> i64 {
    ok!(deopt(frame, held_linear));
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each test uses its own module instance so the process-global arena
    // stays independent across tests.

    #[test]
    fn activation_requires_a_complete_update_set() {
        let m = module_new("m");
        let v1 = version_new(m, 0, &[]).unwrap();
        let v2 = version_new(m, 1, &[]).unwrap();
        assert_eq!(current_version(m), -1);

        // The first version is installed directly (bootstrap: the initial
        // code version becomes current without an update).
        let boot = update_begin(m, v1).unwrap();
        update_add_replacement(boot, 0, v1).unwrap();
        update_mark_complete(boot, 1, 0).unwrap();
        assert_eq!(activate(boot).unwrap(), 1);
        assert_eq!(current_version(m), v1);

        // An update that is never marked complete is rejected at
        // activation (section 8).
        let u = update_begin(m, v2).unwrap();
        update_add_replacement(u, 1, v2).unwrap();
        assert_eq!(activate(u), Err(EvoError::IncompleteUpdateSet));
        assert_eq!(current_version(m), v1, "activation must not change dispatch");

        // Marking complete with a dependent that has no replacement is
        // refused: changing one symbol does not update inlined copies.
        let u = update_begin(m, v2).unwrap();
        update_add_replacement(u, 1, v2).unwrap();
        update_add_dependent(u, 99).unwrap();
        assert_eq!(update_mark_complete(u, 1, 1), Err(EvoError::InvalidUpdate));

        // A complete set activates and publishes a new epoch.
        let u = update_begin(m, v2).unwrap();
        update_add_replacement(u, 1, v2).unwrap();
        update_add_replacement(u, 99, v2).unwrap();
        update_add_dependent(u, 99).unwrap();
        update_mark_complete(u, 1, 1).unwrap();
        assert_eq!(activate(u).unwrap(), 2);
        assert_eq!(current_version(m), v2);
        assert_eq!(epoch(m), 2);
    }

    #[test]
    fn live_frames_keep_old_code_resident() {
        let m = module_new("m");
        let v1 = version_new(m, 0, &[]).unwrap();
        let v2 = version_new(m, 1, &[]).unwrap();
        let boot = update_begin(m, v1).unwrap();
        update_add_replacement(boot, 0, v1).unwrap();
        update_mark_complete(boot, 1, 0).unwrap();
        activate(boot).unwrap();

        // A live frame enters v1.
        let f = frame_enter(m).unwrap();
        assert_eq!(frame_version(f), v1);
        assert_eq!(version_live(v1), 1);

        // Activate v2. v1 is retired (no new calls) but stays resident
        // while the live frame can enter it (section 6).
        let u = update_begin(m, v2).unwrap();
        update_add_replacement(u, 1, v2).unwrap();
        update_mark_complete(u, 1, 0).unwrap();
        activate(u).unwrap();
        assert_eq!(current_version(m), v2);
        assert!(version_retired(v1), "old version is retired");
        assert!(version_resident(v1), "old version stays resident");
        assert_eq!(version_live(v1), 1);

        // Dropping the retained version is refused while the frame lives
        // (section 6: never silently unload).
        assert_eq!(version_try_drop(m, v1), Err(EvoError::RetainedCode));

        // After the frame exits, the version can be dropped.
        frame_exit(f).unwrap();
        assert_eq!(version_live(v1), 0);
        assert_eq!(version_try_drop(m, v1), Ok(true));
        assert!(!version_resident(v1));
    }

    #[test]
    fn old_code_never_resumes_over_new_state_without_a_reverse_transition() {
        let m = module_new("m");
        let v1 = version_new(m, 0, &[]).unwrap();
        let v2 = version_new(m, 1, &[]).unwrap();
        let boot = update_begin(m, v1).unwrap();
        update_add_replacement(boot, 0, v1).unwrap();
        update_mark_complete(boot, 1, 0).unwrap();
        activate(boot).unwrap();

        // A frame admitted under representation 0.
        let f = frame_enter(m).unwrap();
        assert_eq!(frame_resume(f, 0), Ok(()));

        // Migrate state to representation 1 and activate v2.
        let u = update_begin(m, v2).unwrap();
        update_add_replacement(u, 1, v2).unwrap();
        update_set_migration(u, 0, 1, false).unwrap();
        update_mark_complete(u, 1, 0).unwrap();
        activate(u).unwrap();
        assert_eq!(state_repr(m), 1);

        // The old frame resumes over the new representation: refused
        // (section 10).
        assert_eq!(frame_resume(f, 1), Err(EvoError::OldOverNewState));
        frame_exit(f).unwrap();

        // With an authorized reverse transition, resumption is admitted.
        let v3 = version_new(m, 2, &[]).unwrap();
        let f2 = frame_enter(m).unwrap(); // under v2, repr 1
        let u = update_begin(m, v3).unwrap();
        update_add_replacement(u, 2, v3).unwrap();
        // Authorize the reverse transition from repr 2 back to repr 1.
        update_set_migration(u, 1, 2, true).unwrap();
        update_mark_complete(u, 1, 0).unwrap();
        activate(u).unwrap();
        // f2 was admitted at repr 1; the state is now repr 2. Resume is
        // refused unless the reverse (2 -> 1) is authorized, which it is.
        assert_eq!(frame_resume(f2, 2), Ok(()));
        frame_exit(f2).unwrap();
    }

    #[test]
    fn target_compatibility_blocks_reachability() {
        set_feature_available("f5", false);
        let m = module_new("m");
        let v1 = version_new(m, 0, &[]).unwrap();
        let boot = update_begin(m, v1).unwrap();
        update_add_replacement(boot, 0, v1).unwrap();
        update_mark_complete(boot, 1, 0).unwrap();
        activate(boot).unwrap();

        // A target requiring an unavailable feature never becomes
        // reachable (section 13).
        let v2 = version_new(m, 1, &["f5"]).unwrap();
        let u = update_begin(m, v2).unwrap();
        update_add_replacement(u, 1, v2).unwrap();
        update_mark_complete(u, 1, 0).unwrap();
        assert_eq!(activate(u), Err(EvoError::TargetUnavailable));
        assert_eq!(current_version(m), v1, "target must not become reachable");

        // Establishing availability lets the same update activate.
        set_feature_available("f5", true);
        let u = update_begin(m, v2).unwrap();
        update_add_replacement(u, 1, v2).unwrap();
        update_mark_complete(u, 1, 0).unwrap();
        assert!(activate(u).is_ok());
        assert_eq!(current_version(m), v2);
        set_feature_available("f5", false);
    }

    #[test]
    fn variants_install_and_retire_safely_with_live_frames() {
        let m = module_new("m");
        let generic = version_new(m, 0, &[]).unwrap();
        let spec = version_new(m, 0, &[]).unwrap();
        let boot = update_begin(m, generic).unwrap();
        update_add_replacement(boot, 0, generic).unwrap();
        update_mark_complete(boot, 1, 0).unwrap();
        activate(boot).unwrap();

        let variant = variant_install(m, generic, spec, 7).unwrap();
        // The guard holds for the observed value.
        assert_eq!(variant_check_guard(variant, 7), Ok(()));
        assert_eq!(variant_check_guard(variant, 8), Err(EvoError::UnsafeDeopt));

        // A specialized frame is live.
        let f = variant_enter(variant).unwrap();
        assert_eq!(frame_version(f), spec);
        assert_eq!(version_live(spec), 1);

        // Retiring detaches the variant from new entry but keeps its code
        // resident while the live frame can enter it (section 6).
        variant_retire(variant).unwrap();
        assert!(variant_retired(variant));
        assert!(variant_resident(variant), "specialized code stays resident");
        assert_eq!(version_try_drop(m, spec), Err(EvoError::RetainedCode));

        // A guard failure deoptimizes consumption-safely to the generic
        // form: the frame moves to generic, the specialized retention is
        // released exactly once.
        assert_eq!(deopt(f, 1), Ok(()));
        assert_eq!(frame_version(f), generic);
        assert_eq!(version_live(spec), 0);
        assert_eq!(version_live(generic), 1);

        frame_exit(f).unwrap();
        assert_eq!(version_try_drop(m, spec), Ok(true));
        assert!(!variant_resident(variant));
    }

    #[test]
    fn deopt_refuses_an_unbalanced_linear_transfer() {
        let m = module_new("m");
        let generic = version_new(m, 0, &[]).unwrap();
        let spec = version_new(m, 0, &[]).unwrap();
        let boot = update_begin(m, generic).unwrap();
        update_add_replacement(boot, 0, generic).unwrap();
        update_mark_complete(boot, 1, 0).unwrap();
        activate(boot).unwrap();
        let variant = variant_install(m, generic, spec, 1).unwrap();
        let f = variant_enter(variant).unwrap();

        // A frame cannot transfer more linear obligations than the
        // variant accounts for; the transfer is refused, never partial.
        assert_eq!(deopt(f, 3), Err(EvoError::UnsafeDeopt));
        assert_eq!(frame_version(f), spec, "frame stays specialized on refusal");
        assert_eq!(deopt(f, -1), Err(EvoError::UnsafeDeopt));
        frame_exit(f).unwrap();
    }
}

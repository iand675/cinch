//! Prelude split tests: the `Prelude` facade over the scheme homes
//! (the prelude split, `compiler/prelude/**`).
//!
//! The facade's contract is the compatibility rule (D161-D174's
//! integration report): its surface is exactly the pre-split implicit
//! surface, and the negative space is real. These tests pin both
//! halves:
//!
//!   facade coverage        the facade's export list and the checked
//!                          surface agree name for name, and every one
//!                          of those names resolves through the
//!                          implicit `Prelude` layer in user code;
//!   negative curation      `invert`, `show` and `hash` are NOT
//!                          reachable through `Prelude`; they resolve
//!                          only with an explicit import of their home;
//!   home imports           `import Data.List`, `import Algebra.Group`,
//!                          `import Text.Codec` and the rest work from
//!                          a module beside the entry file;
//!   parked instances       the instance families that moved into the
//!                          types' homes at the split (D159 is over)
//!                          are invisible as such and behave exactly as
//!                          before (`<>` on lists, `map` on options,
//!                          `show`/`encode` round trips);
//!   family laws            the law shapes the family headers document
//!                          over sample values: `Group` inverse laws
//!                          over the additive monoid, `Semiring` laws
//!                          on `Nat`/`Int`, `Data.Set` insert/lookup/
//!                          union behavior, `Data.Hash`/`Data.HashMap`
//!                          consistency and round trips, `Text.Show`'s
//!                          rendering, `Text.Codec`'s round trip for
//!                          the D65 set.
//!
//! Every law program is a real `Main` compiled and run through the
//! same driver the CLI uses; the families' classes are reached with
//! explicit imports of their homes, since the curated facade names
//! only the pre-split surface.

use cinch::driver::{
    check_sources, check_sources_ifaces, driver, run_source, Mode, PRELUDE_EARLY, PRELUDE_FACADE,
    PRELUDE_LATE,
};
use cinch::interface::Iface;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

// ----------------------------------------------------------------
// Shared helpers
// ----------------------------------------------------------------

fn expect_exit(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn write_project(files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cinch_prelude_split_{}_{}",
        std::process::id(),
        files.as_ptr() as usize
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write module");
    }
    dir
}

fn run_entry(dir: &Path, entry: &str) -> Result<i32, String> {
    driver(Mode::Run, &dir.join(entry), None).map_err(|e| e.to_string())
}

fn check_entry(dir: &Path, entry: &str) -> Result<(), String> {
    driver(Mode::Check, &dir.join(entry), None)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

// ----------------------------------------------------------------
// The facade surface: the export list and the checked surface
// ----------------------------------------------------------------

/// The names one module's `export` declarations name: `export a, b,
/// (op)` items as written, with the parenthesized operator items
/// spelling their operator (the forced export grammar, D161-D174).
fn export_names(src: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for line in src.lines() {
        let Some(rest) = line.trim_start().strip_prefix("export ") else {
            continue;
        };
        if rest.trim_start().starts_with("module ") {
            continue;
        }
        for item in rest.split(',') {
            let item = item.trim();
            let bare = item
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .unwrap_or(item);
            names.insert(bare.rsplit('.').next().unwrap_or(bare).to_string());
        }
    }
    names
}

/// The modules a source re-exports with `export module X` (D146).
fn reexported_modules(src: &str) -> Vec<String> {
    src.lines()
        .filter_map(|line| {
            line.trim_start()
                .strip_prefix("export module ")
                .map(|m| m.trim().to_string())
        })
        .collect()
}

/// The embedded scheme homes by module name, in the driver's own
/// tables (so the test tracks the shipped family, not a copy).
fn home_sources() -> BTreeMap<String, &'static str> {
    PRELUDE_EARLY
        .iter()
        .chain(PRELUDE_LATE.iter())
        .map(|(path, src)| (path.trim_end_matches(".cinch").replace('/', "."), *src))
        .collect()
}

/// The names the facade's export list declares: its own per-name
/// items plus every name of each re-exported home's export list.
fn facade_export_names() -> BTreeSet<String> {
    let homes = home_sources();
    let mut names = export_names(PRELUDE_FACADE);
    for module in reexported_modules(PRELUDE_FACADE) {
        if module == "Prelude.Generated" {
            // The generated WIT adapter vocabulary: its names come
            // from the `.wit` adapters beside the entry, and these
            // tests compile without one (empty surface here).
            continue;
        }
        let src = homes
            .get(&module)
            .unwrap_or_else(|| panic!("the facade re-exports {module}, which has no embedded home"));
        for n in export_names(src) {
            names.insert(n);
        }
    }
    names
}

fn prelude_iface() -> Iface {
    let (_program, ifaces) =
        check_sources_ifaces("module Main where\n  main :: Int\n  main = 0\n", "<test>")
            .expect("the trivial program checks");
    ifaces
        .into_iter()
        .find(|i| i.module == "Prelude")
        .expect("the prelude facade publishes an interface")
}

/// The names the checked surface records for the facade: values,
/// types and their constructors, signatures and their methods (the
/// contract projection's view of D143's named set).
fn checked_surface_names(iface: &Iface) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for v in &iface.exports.values {
        names.insert(v.name.clone());
    }
    for t in &iface.exports.types {
        names.insert(t.name.clone());
        for c in &t.ctors {
            names.insert(c.name.clone());
        }
    }
    for s in &iface.exports.signatures {
        names.insert(s.name.clone());
        for m in &s.methods {
            names.insert(m.name.clone());
        }
    }
    names
}

// ----------------------------------------------------------------
// Name-resolution probes
// ----------------------------------------------------------------

/// One probe: a definition in a `Main` that sees only the implicit
/// `Prelude` base layer, whose signature and body force `name` to
/// resolve and type-check at its own shape.
struct Probe {
    name: String,
    kind: &'static str,
    body: String,
}

/// The reference spelling of a name: operators take their parenthesized
/// form (`(==)`), ordinary names are bare.
fn ref_of(name: &str) -> String {
    if name.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
        name.to_string()
    } else {
        format!("({name})")
    }
}

/// The probes whose signature text is restated from the home
/// declarations: the interface's canonical scheme rendering flattens
/// nested type applications (`Option (Producer (TakeSt s) a)` renders
/// as `Option Producer TakeSt s a`, `Ref region (Vec n U32)` as `Ref
/// region Vec n U32`), so splicing the rendered scheme does not parse
/// back for these seven names. Each signature below is the home
/// declaration's own text; a rendering fix would let the table drop
/// away.
const RESTATED: &[(&str, &str, &str)] = &[
    (
        "detect-vec-u32",
        "Unit -> Option (VecU32Target AVX2_256 EightLanes)",
        "detect-vec-u32",
    ),
    (
        "prod-take",
        "forall s a. Nat -> Producer s a -> Producer (TakeSt s) a",
        "prod-take",
    ),
    (
        "prod-take-step",
        "forall s a. (s -> Step s a) -> TakeSt s -> Step (TakeSt s) a",
        "prod-take-step",
    ),
    (
        "scalar-sum-wrap32",
        "forall n region. Ref region (Vec n U32) -> U32",
        "scalar-sum-wrap32",
    ),
    (
        "simd-sum-wrap32",
        "forall t w n region. (vectors :: VecU32Target t w) -> Ref region (Vec n U32) -> U32",
        "simd-sum-wrap32",
    ),
    (
        "vec-stream",
        "forall n a region. Ref region (Vec n a) -> Producer (Ref region (Vec n a)) a",
        "vec-stream",
    ),
    (
        "scalar-to-simd",
        "forall t w n region. using (dict :: SumProofs) => (vectors :: VecU32Target t w) -> (xs :: Ref region (Vec n U32)) -> Eq (scalar-sum-wrap32 xs) (simd-sum-wrap32 vectors xs)",
        "scalar-to-simd",
    ),
];

/// One probe per surface name, keyed by the name's contract kind:
/// values and methods resolve at their scheme, types are consumed at
/// their full application, constructors build their type, and class
/// names appear in constraint position.
fn probes(iface: &Iface) -> Vec<Probe> {
    let mut out: Vec<Probe> = Vec::new();
    let mut push = |name: &str, kind: &'static str, sig: &str, args: &[&str], rhs: &str| {
        let n = format!("probe{}", out.len());
        let params = if args.is_empty() {
            String::new()
        } else {
            format!(" {}", args.join(" "))
        };
        out.push(Probe {
            name: name.to_string(),
            kind,
            body: format!("  {n} :: {sig}\n  {n}{params} = {rhs}\n"),
        });
    };
    let restated: BTreeMap<&str, (&str, &str)> = RESTATED
        .iter()
        .map(|(name, sig, rhs)| (*name, (*sig, *rhs)))
        .collect();

    for v in &iface.exports.values {
        let (sig, rhs) = match restated.get(v.name.as_str()) {
            Some((sig, rhs)) => (*sig, (*rhs).to_string()),
            None => (v.scheme.as_str(), ref_of(&v.name)),
        };
        push(&v.name, "value", sig, &[], &rhs);
    }
    for t in &iface.exports.types {
        if iface.exports.signatures.iter().any(|s| s.name == t.name) {
            // A class name also registers as its dictionary type; the
            // name probes once, in constraint position below.
            continue;
        }
        let params: Vec<String> = (0..t.params.len()).map(|i| format!("t{i}")).collect();
        let app = if params.is_empty() {
            t.name.clone()
        } else {
            format!("{} {}", t.name, params.join(" "))
        };
        let forall = if params.is_empty() {
            String::new()
        } else {
            format!("forall {}. ", params.join(" "))
        };
        push(&t.name, "type", &format!("{forall}{app} -> {app}"), &["x"], "x");
        for c in &t.ctors {
            // The constructor at its full field type, applied to its
            // fields (an erased-field constructor must be fully
            // applied). The fields and result mention the type's own
            // parameter names, so the probe foralls exactly those.
            let fields: Vec<String> = c.fields.iter().map(|f| format!("({f})")).collect();
            let arrows = if fields.is_empty() {
                String::new()
            } else {
                format!("{} -> ", fields.join(" -> "))
            };
            let ty_params: Vec<&str> = t.params.iter().map(|p| p.as_str()).collect();
            let sig = if ty_params.is_empty() {
                format!("{arrows}{app}")
            } else {
                let result = format!("{} {}", t.name, ty_params.join(" "));
                format!("forall {}. {arrows}{result}", ty_params.join(" "))
            };
            let args: Vec<String> = (0..c.fields.len()).map(|i| format!("y{i}")).collect();
            let rhs = if args.is_empty() {
                ref_of(&c.name)
            } else {
                format!("{} {}", ref_of(&c.name), args.join(" "))
            };
            push(&c.name, "ctor", &sig, &args.iter().map(|a| a.as_str()).collect::<Vec<_>>(), &rhs);
        }
    }
    for s in &iface.exports.signatures {
        let params: Vec<String> = (0..s.params.len()).map(|i| format!("t{i}")).collect();
        let app = if params.is_empty() {
            s.name.clone()
        } else {
            format!("{} {}", s.name, params.join(" "))
        };
        let forall = if params.is_empty() {
            String::new()
        } else {
            format!("forall {}. ", params.join(" "))
        };
        push(
            &s.name,
            "class",
            &format!("{forall}using (d :: {app}) => Unit -> Unit"),
            &["u"],
            "u",
        );
    }
    // Method names resolve once each. `pure` and `bind` serve the
    // three sequencing providers under one name (D22: the first
    // registered provider answers bare uses), so the first scheme wins.
    let mut seen_methods: BTreeSet<String> = BTreeSet::new();
    for s in &iface.exports.signatures {
        for m in &s.methods {
            if !seen_methods.insert(m.name.clone()) {
                continue;
            }
            let (sig, rhs) = match restated.get(m.name.as_str()) {
                Some((sig, rhs)) => (*sig, (*rhs).to_string()),
                None => (m.scheme.as_str(), ref_of(&m.name)),
            };
            push(&m.name, "method", sig, &[], &rhs);
        }
    }
    out
}

#[test]
fn facade_export_list_is_the_checked_surface() {
    let iface = prelude_iface();
    let checked = checked_surface_names(&iface);
    let declared = facade_export_names();
    assert_eq!(
        declared, checked,
        "D143's surface rule: the facade's export list and the checked surface must name the same set"
    );
    assert!(
        checked.len() >= 300,
        "the surface enumeration looks empty: {} names",
        checked.len()
    );
    for name in [
        "nat-add",
        "Cons",
        "==",
        "pure",
        "map",
        "char-string",
        "fetch",
        "String",
        // The deliberate P24 addition (primitive types specification
        // section 9 puts both in the prelude).
        "intern",
        "sym-name",
    ] {
        assert!(checked.contains(name), "the pre-split surface should contain {name}");
    }
    for name in ["invert", "show", "hash", "encode", "decode", "add", "zero", "set-insert"] {
        assert!(
            !checked.contains(name),
            "{name} belongs to a new family and must stay out of the curated facade"
        );
    }
}

#[test]
fn every_facade_surface_name_resolves_through_prelude() {
    let iface = prelude_iface();
    let surface = checked_surface_names(&iface);
    let ps = probes(&iface);
    let covered: BTreeSet<String> = ps.iter().map(|p| p.name.clone()).collect();
    assert_eq!(
        covered, surface,
        "every surface name needs a probe (and no probe may name a stranger)"
    );

    let mut failures: Vec<String> = Vec::new();
    for kind in ["value", "type", "ctor", "class", "method"] {
        let mut src = String::from("module Main where\n");
        for p in ps.iter().filter(|p| p.kind == kind) {
            src.push_str(&p.body);
        }
        src.push_str("  main :: Int\n  main = 0\n");
        let batch = check_sources(&src, "<test>");
        if batch.is_ok() {
            continue;
        }
        // The batch failed: name every failing probe, not just the
        // first, so the report points at the whole broken set.
        let mut localized = false;
        for p in ps.iter().filter(|p| p.kind == kind) {
            let mut one = String::from("module Main where\n");
            one.push_str(&p.body);
            one.push_str("  main :: Int\n  main = 0\n");
            if let Err(e) = check_sources(&one, "<test>") {
                localized = true;
                failures.push(format!("{} ({}) :: {}", p.name, p.kind, e));
            }
        }
        if !localized {
            failures.push(format!(
                "{kind} batch :: {}",
                batch.err().expect("the batch failed")
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "surface names unreachable through `Prelude`:\n{}",
        failures.join("\n")
    );
}

#[test]
fn private_ctors_are_unreachable_through_an_import() {
    // D298/D308's surface rule: `Data.String`'s representation
    // constructor `StrBuf` is module-private, so an import cannot
    // reach it in pattern or term position; the type stays fully
    // usable through its conversions (the round trips live in
    // tests/string_literals.rs).
    let pat = "module Main where\n\
         \x20 import Data.String\n\
         \x20 leak :: String -> Nat\n\
         \x20 leak s = case s of\n\
         \x20   StrBuf b -> 1\n\
         \x20 main :: Int\n\
         \x20 main = 0\n";
    match check_sources(pat, "<test>") {
        Ok(_) => panic!("StrBuf pattern: expected a not-exported refusal"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("not exported") && msg.contains("StrBuf"),
                "StrBuf pattern: expected the named refusal, got {msg}"
            );
        }
    }
    let term = "module Main where\n\
         \x20 import Data.String\n\
         \x20 leak :: Bytes -> String\n\
         \x20 leak b = StrBuf b\n\
         \x20 main :: Int\n\
         \x20 main = 0\n";
    match check_sources(term, "<test>") {
        Ok(_) => panic!("StrBuf term: expected a not-exported refusal"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("not exported") && msg.contains("StrBuf"),
                "StrBuf term: expected the named refusal, got {msg}"
            );
        }
    }
    // The single-constructor type-name alias is construction too
    // (D308): `String b` must not reach the private constructor
    // through the type's exported name.
    let alias = "module Main where\n\
         \x20 import Data.String\n\
         \x20 leak :: Bytes -> String\n\
         \x20 leak b = String b\n\
         \x20 main :: Int\n\
         \x20 main = 0\n";
    match check_sources(alias, "<test>") {
        Ok(_) => panic!("String alias: expected a refusal"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("String"),
                "String alias: expected a named refusal, got {msg}"
            );
        }
    }
}

#[test]
fn curated_names_are_unreachable_through_prelude() {
    // The negative space is real (the facade's header): `invert`,
    // `show` and `hash` belong to families the facade does not
    // re-export, so a bare use does not resolve.
    for (name, probe) in [
        ("invert", "  probe :: Int\n  probe = invert (0 - 1)\n"),
        ("show", "  probe :: String\n  probe = show (5 :: Nat)\n"),
        ("hash", "  probe :: Nat\n  probe = hash (5 :: Nat)\n"),
    ] {
        let src = format!("module Main where\n{probe}  main :: Int\n  main = 0\n");
        match check_sources(&src, "<test>") {
            Ok(_) => panic!("{name}: expected an unbound-name error through `Prelude`"),
            Err(e) => {
                let msg = e.to_string();
                assert!(
                    msg.contains("unknown name") && msg.contains(name),
                    "{name}: expected an unbound-name error, got {msg}"
                );
            }
        }
    }
    // Each name resolves through its home, and works there.
    expect_exit(
        "invert-home",
        "module Main where\n\
         \x20 import Algebra.Group\n\
         \x20 probe :: Int\n\
         \x20 probe = invert (0 - 1)\n\
         \x20 main :: Int\n\
         \x20 main = if probe == 1 then 0 else 1\n",
        0,
    );
    expect_exit(
        "show-home",
        "module Main where\n\
         \x20 import Text.Show\n\
         \x20 probe :: String\n\
         \x20 probe = show (5 :: Nat)\n\
         \x20 main :: Int\n\
         \x20 main = if probe == nat-to-string 5 then 0 else 1\n",
        0,
    );
    expect_exit(
        "hash-home",
        "module Main where\n\
         \x20 import Data.Hash\n\
         \x20 probe :: Nat\n\
         \x20 probe = hash (5 :: Nat)\n\
         \x20 main :: Int\n\
         \x20 main = if probe == 5 then 0 else 1\n",
        0,
    );
}

#[test]
fn home_imports_resolve_across_files() {
    // A library module beside the entry imports the scheme homes
    // directly (`import Data.List`, `import Algebra.Group`, `import
    // Text.Codec`, ...) and the entry consumes its results.
    let dir = write_project(&[
        (
            "Lib.cinch",
            "module Lib where\n\
             \x20 import Data.List\n\
             \x20 import Algebra.Group\n\
             \x20 import Algebra.Semiring\n\
             \x20 import Data.Set\n\
             \x20 import Data.Hash\n\
             \x20 import Data.HashMap\n\
             \x20 import Text.Codec\n\
             \x20 import Text.Show\n\
             \n\
             \x20 lib-invert :: Int -> Int\n\
             \x20 lib-invert x = invert x\n\
             \n\
             \x20 lib-add :: Nat\n\
             \x20 lib-add = add 2 3\n\
             \n\
             \x20 lib-set-size :: Nat\n\
             \x20 lib-set-size = set-size (set-insert 1 (set-insert 1 set-empty))\n\
             \n\
             \x20 lib-map-lookup :: Option Nat\n\
             \x20 lib-map-lookup = hash-map-lookup 1 (hash-map-insert 1 2 hash-map-empty)\n\
             \n\
             \x20 lib-hash :: Nat\n\
             \x20 lib-hash = hash (7 :: Nat)\n\
             \n\
             \x20 lib-codec :: Option I64\n\
             \x20 lib-codec = decode (encode (9 :: I64))\n\
             \n\
             \x20 lib-show :: String\n\
             \x20 lib-show = show (5 :: Nat)\n\
             \n\
             \x20 lib-map-list :: List Nat\n\
             \x20 lib-map-list = map (\\x -> x + 1) (Cons 1 (Cons 2 Nil))\n",
        ),
        (
            "Main.cinch",
            "module Main where\n\
             \x20 import Lib\n\
             \n\
             \x20 eq-list :: forall a. using (ea :: Eq a) => List a -> List a -> Bool\n\
             \x20 eq-list a b = case a of\n\
             \x20   Nil -> case b of\n\
             \x20     Nil -> True\n\
             \x20     Cons _ _ -> False\n\
             \x20   Cons x xs -> case b of\n\
             \x20     Nil -> False\n\
             \x20     Cons y ys -> if x == y then eq-list xs ys else False\n\
             \n\
             \x20 eq-opt :: forall a. using (ea :: Eq a) => Option a -> Option a -> Bool\n\
             \x20 eq-opt x y = case x of\n\
             \x20   None -> case y of\n\
             \x20     None -> True\n\
             \x20     Some _ -> False\n\
             \x20   Some p -> case y of\n\
             \x20     None -> False\n\
             \x20     Some q -> p == q\n\
             \n\
             \x20 main :: Int\n\
             \x20 main =\n\
             \x20   if Lib.lib-invert (0 - 3) == 3 then\n\
             \x20     (if Lib.lib-add == 5 then\n\
             \x20       (if Lib.lib-set-size == 1 then\n\
             \x20         (if eq-opt Lib.lib-map-lookup (Some 2) then\n\
             \x20           (if Lib.lib-hash == 7 then\n\
             \x20             (if eq-opt Lib.lib-codec (Some 9) then\n\
             \x20               (if Lib.lib-show == nat-to-string 5 then\n\
             \x20                 (if eq-list Lib.lib-map-list (Cons 2 (Cons 3 Nil)) then 0 else 8)\n\
             \x20               else 7)\n\
             \x20             else 6)\n\
             \x20           else 5)\n\
             \x20         else 4)\n\
             \x20       else 3)\n\
             \x20     else 2)\n\
             \x20   else 1\n",
        ),
    ]);
    check_entry(&dir, "Main.cinch").expect("the multi-file project checks");
    let code = run_entry(&dir, "Main.cinch").expect("the multi-file project runs");
    assert_eq!(code, 0, "home imports across files should run clean");
}

// ----------------------------------------------------------------
// Parked-instance invisibility (D159's deviation is over)
// ----------------------------------------------------------------

#[test]
fn parked_semigroup_instances_behave_identically() {
    // The instance families that moved into the types' homes at the
    // split: the program imports ONLY the class home, and the
    // carriers' instances fire as module-owned facts (never names),
    // with their pre-split behavior.
    expect_exit(
        "parked-semigroup",
        "module Main where\n\
         \x20 import Algebra.Semigroup\n\
         \n\
         \x20 infixr 6 <>\n\
         \n\
         \x20 eq-list :: forall a. using (ea :: Eq a) => List a -> List a -> Bool\n\
         \x20 eq-list a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons _ _ -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then eq-list xs ys else False\n\
         \n\
         \x20 eq-opt :: forall a. using (ea :: Eq a) => Option a -> Option a -> Bool\n\
         \x20 eq-opt x y = case x of\n\
         \x20   None -> case y of\n\
         \x20     None -> True\n\
         \x20     Some _ -> False\n\
         \x20   Some p -> case y of\n\
         \x20     None -> False\n\
         \x20     Some q -> p == q\n\
         \n\
         \x20 ord-tag :: Ordering -> Nat\n\
         \x20 ord-tag o = case o of\n\
         \x20   LT -> 0\n\
         \x20   EQ -> 1\n\
         \x20   GT -> 2\n\
         \n\
         \x20 eq-ord :: Ordering -> Ordering -> Bool\n\
         \x20 eq-ord x y = ord-tag x == ord-tag y\n\
         \n\
         \x20 str-a :: String\n\
         \x20 str-a = case u32-to-char 97 of\n\
         \x20   None -> empty-string\n\
         \x20   Some c -> char-string c\n\
         \n\
         \x20 str-b :: String\n\
         \x20 str-b = case u32-to-char 98 of\n\
         \x20   None -> empty-string\n\
         \x20   Some c -> char-string c\n\
         \n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   if eq-list (Cons (1 :: Nat) (Cons 2 Nil) <> Cons 3 Nil) (Cons 1 (Cons 2 (Cons 3 Nil))) then\n\
         \x20     (if eq-opt (Some (1 :: Nat) <> None) (Some 1) && eq-opt (None <> Some 2) (Some 2) && eq-opt (Some 1 <> Some 2) (Some 1) then\n\
         \x20       (if str-a <> str-b == string-concat str-a str-b then\n\
         \x20         (if eq-ord (LT <> EQ) LT && eq-ord (EQ <> GT) GT && eq-ord (LT <> GT) LT && eq-ord (EQ <> EQ) EQ then 0 else 4)\n\
         \x20       else 3)\n\
         \x20     else 2)\n\
         \x20   else 1\n",
        0,
    );
}

#[test]
fn parked_functor_instances_behave_identically() {
    // The `Functor` instances of `Option` and `List` live in their
    // types' homes now. The explicit `import Category.Functor` shadows
    // the facade's concrete `map` (D145), so the calls below exercise
    // the class method over the moved instances.
    expect_exit(
        "parked-functor",
        "module Main where\n\
         \x20 import Category.Functor\n\
         \n\
         \x20 eq-list :: forall a. using (ea :: Eq a) => List a -> List a -> Bool\n\
         \x20 eq-list a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons _ _ -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then eq-list xs ys else False\n\
         \n\
         \x20 eq-opt :: forall a. using (ea :: Eq a) => Option a -> Option a -> Bool\n\
         \x20 eq-opt x y = case x of\n\
         \x20   None -> case y of\n\
         \x20     None -> True\n\
         \x20     Some _ -> False\n\
         \x20   Some p -> case y of\n\
         \x20     None -> False\n\
         \x20     Some q -> p == q\n\
         \n\
         \x20 inc :: Nat -> Nat\n\
         \x20 inc x = x + 1\n\
         \n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   if eq-opt (map inc (Some 1)) (Some 2) then\n\
         \x20     (if eq-opt (map inc None) (None :: Option Nat) then\n\
         \x20       (if eq-list (map inc (Cons 1 (Cons 2 Nil))) (Cons 2 (Cons 3 Nil)) then\n\
         \x20         (if eq-opt ((<$>) inc (Some 3)) (Some 4) then 0 else 4)\n\
         \x20       else 3)\n\
         \x20     else 2)\n\
         \x20   else 1\n",
        0,
    );
}

#[test]
fn parked_show_and_codec_instances_round_trip() {
    // The `Show` and `Codec` instance families in their homes
    // (Text.Show's conversions, Data.Machine's `Codec I64` and
    // Data.Bool's `Codec Bool`): the round trips and renderings behave
    // exactly as documented.
    expect_exit(
        "parked-show-codec",
        "module Main where\n\
         \x20 import Text.Show\n\
         \x20 import Text.Codec\n\
         \n\
         \x20 eq-opt :: forall a. using (ea :: Eq a) => Option a -> Option a -> Bool\n\
         \x20 eq-opt x y = case x of\n\
         \x20   None -> case y of\n\
         \x20     None -> True\n\
         \x20     Some _ -> False\n\
         \x20   Some p -> case y of\n\
         \x20     None -> False\n\
         \x20     Some q -> p == q\n\
         \n\
         \x20 char-show :: Bool\n\
         \x20 char-show = case u32-to-char 65 of\n\
         \x20   None -> True\n\
         \x20   Some c -> show c == char-string c\n\
         \n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   if show (42 :: Nat) == nat-to-string 42 then\n\
         \x20     (if show ((0 - 7) :: Int) == int-to-string (0 - 7) then\n\
         \x20       (if char-show then\n\
         \x20         (if eq-opt (decode (encode (255 :: I64))) (Some (255 :: I64)) then\n\
         \x20           (if eq-opt (decode (encode True)) (Some True) then 0 else 5)\n\
         \x20         else 4)\n\
         \x20       else 3)\n\
         \x20     else 2)\n\
         \x20   else 1\n",
        0,
    );
}

// ----------------------------------------------------------------
// The families' law shapes over samples
// ----------------------------------------------------------------

#[test]
fn group_inverse_laws_hold_over_the_additive_monoid() {
    // Algebra.Group's documented law shape (its header), stated over
    // the semiring's additive monoid `add`/`zero` (the form the header
    // prescribes for `Int`; `Nat` gets no `Group`, its subtraction
    // truncates), over the sample integers below.
    expect_exit(
        "group-inverse-laws",
        r#"module Main where

  import Algebra.Group
  import Algebra.Semiring

  inverse-laws :: forall a. using (ga :: Group a, sa :: Semiring a, ea :: Eq a) => a -> Bool
  inverse-laws x = add (invert x) x == zero && add x (invert x) == zero

  sample :: Nat -> Int
  sample t =
    if t == 0 then 0
    else if t == 1 then 1
    else if t == 2 then 0 - 1
    else if t == 3 then 7
    else if t == 4 then 0 - 7
    else if t == 5 then 42
    else 0 - 42

  run :: Nat -> Bool
  run n =
    if n == 0 then True
    else if inverse-laws (sample (n - 1)) then run (n - 1)
    else False

  main :: Int
  main = if run 7 then 0 else 1
"#,
        0,
    );
}

#[test]
fn semiring_laws_hold_on_nat_and_int() {
    // Algebra.Semiring's documented law list (its header) over every
    // ordered triple of the sample sets: addition and multiplication
    // associate, addition commutes, `zero` and `one` are the
    // identities, multiplication distributes over addition on both
    // sides, and `zero` annihilates.
    expect_exit(
        "semiring-laws",
        r#"module Main where

  import Algebra.Semiring

  laws :: forall a. using (sa :: Semiring a, ea :: Eq a) => a -> a -> a -> Bool
  laws x y z =
    add (add x y) z == add x (add y z) &&
    add x y == add y x &&
    add zero x == x && add x zero == x &&
    mul (mul x y) z == mul x (mul y z) &&
    mul one x == x && mul x one == x &&
    mul x (add y z) == add (mul x y) (mul x z) &&
    mul (add y z) x == add (mul y x) (mul z x) &&
    mul zero x == zero && mul x zero == zero

  nat-sample :: Nat -> Nat
  nat-sample t = if t == 0 then 0 else if t == 1 then 1 else if t == 2 then 2 else 5

  int-sample :: Nat -> Int
  int-sample t = if t == 0 then 0 else if t == 1 then 1 else if t == 2 then 0 - 1 else 5

  run-nat :: Nat -> Bool
  run-nat n =
    if n == 0 then True
    else
      let m = n - 1 in
      if laws (nat-sample (m / 16)) (nat-sample ((m / 4) % 4)) (nat-sample (m % 4)) then run-nat m
      else False

  run-int :: Nat -> Bool
  run-int n =
    if n == 0 then True
    else
      let m = n - 1 in
      if laws (int-sample (m / 16)) (int-sample ((m / 4) % 4)) (int-sample (m % 4)) then run-int m
      else False

  main :: Int
  main = if run-nat 64 then (if run-int 64 then 0 else 2) else 1
"#,
        0,
    );
}

#[test]
fn set_insert_lookup_and_union_keep_one_copy() {
    // Data.Set's documented behavior (its header and the split
    // report): a repeated element keeps one copy, insert/lookup round
    // trip, and the union (test-local; the family ships no
    // `set-union`) keeps one copy per element with the tree's
    // ascending listing.
    expect_exit(
        "set-behavior",
        r#"module Main where

  import Data.Set

  eq-list :: forall a. using (ea :: Eq a) => List a -> List a -> Bool
  eq-list a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if x == y then eq-list xs ys else False

  sample-list :: Nat -> List Nat
  sample-list t =
    if t == 0 then Nil
    else if t == 1 then Cons 3 Nil
    else if t == 2 then Cons 1 (Cons 2 Nil)
    else Cons 2 (Cons 3 (Cons 3 Nil))

  insert-all :: List Nat -> Set Nat -> Set Nat
  insert-all xs s = case xs of
    Nil -> s
    Cons y ys -> insert-all ys (set-insert y s)

  set-union :: Set Nat -> Set Nat -> Set Nat
  set-union a b = insert-all (set-to-list a) b

  dup-law :: Nat -> Bool
  dup-law n =
    if n == 0 then True
    else
      let m = n - 1 in
      let s = set-from-list (sample-list (m / 5)) in
      let one = set-insert (m % 5) s in
      let two = set-insert (m % 5) one in
      if set-size two == set-size one && eq-list (set-to-list two) (set-to-list one) then dup-law m
      else False

  lookup-law :: Nat -> Bool
  lookup-law n =
    if n == 0 then True
    else
      let m = n - 1 in
      let s = set-from-list (sample-list (m / 5)) in
      if set-member (m % 5) (set-insert (m % 5) s) then lookup-law m else False

  union-law :: Nat -> Bool
  union-law n =
    if n == 0 then True
    else
      let m = n - 1 in
      let a = set-from-list (sample-list ((m / 20) % 4)) in
      let b = set-from-list (sample-list ((m / 5) % 4)) in
      if set-member (m % 5) (set-union a b) == (set-member (m % 5) a || set-member (m % 5) b) then union-law m
      else False

  main :: Int
  main =
    if dup-law 20 then
      (if lookup-law 20 then
        (if union-law 80 then
          (if eq-list (set-to-list (set-union (set-from-list (Cons (1 :: Nat) (Cons 2 Nil))) (set-from-list (Cons 2 (Cons 3 Nil))))) (Cons 1 (Cons 2 (Cons 3 Nil))) then
            (if set-size (set-from-list (Cons (2 :: Nat) (Cons 1 (Cons 2 (Cons 1 Nil))))) == 2 then
              (if set-member (3 :: Nat) (set-insert 3 set-empty) && not (set-member (4 :: Nat) (set-insert 3 set-empty)) then 0 else 6)
            else 5)
          else 4)
        else 3)
      else 2)
    else 1
"#,
        0,
    );
}

#[test]
fn hash_consistency_and_hashmap_round_trips() {
    // Data.Hash's law (its header): equal keys hash alike. Data.HashMap
    // (its header): insert/lookup round trips, an equal key replaces in
    // place, an earlier pair wins a repeated key in
    // `hash-map-from-pairs`, and a hash collision degrades the bucket
    // to a scan without losing either key.
    expect_exit(
        "hash-family",
        r#"module Main where

  import Data.Hash
  import Data.HashMap

  eq-opt :: forall a. using (ea :: Eq a) => Option a -> Option a -> Bool
  eq-opt x y = case x of
    None -> case y of
      None -> True
      Some _ -> False
    Some p -> case y of
      None -> False
      Some q -> p == q

  hash-agrees :: forall a. using (ha :: Hash a, ea :: Eq a) => a -> a -> Bool
  hash-agrees x y = if x == y then hash x == hash y else True

  run-nat :: Nat -> Bool
  run-nat n =
    if n == 0 then True
    else
      let m = n - 1 in
      if hash-agrees (nat-mul m 3) (nat-add m (nat-add m m)) then run-nat m else False

  run-int :: Nat -> Bool
  run-int n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = int-from-i64 (nat-to-i64 m) in
      if hash-agrees (int-add x (int-neg 7)) (int-sub x 7) then run-int m else False

  bool-char-string :: Bool
  bool-char-string =
    hash-agrees True (prim-eq-bool True True) &&
    char-hashes-agree &&
    hash-agrees empty-string (string-concat empty-string empty-string)

  char-hashes-agree :: Bool
  char-hashes-agree = case u32-to-char 65 of
    None -> True
    Some c -> case u32-to-char (wrapping-add-u32 33 32) of
      None -> True
      Some d -> hash-agrees c d

  round-trip :: forall k v. using (hk :: Hash k, ek :: Eq k, ev :: Eq v) => k -> v -> HashMap k v -> Bool
  round-trip k v m = eq-opt (hash-map-lookup k (hash-map-insert k v m)) (Some v)

  insert-replaces :: forall k v. using (hk :: Hash k, ek :: Eq k, ev :: Eq v) => k -> v -> v -> Bool
  insert-replaces k v1 v2 = eq-opt (hash-map-lookup k (hash-map-insert k v2 (hash-map-insert k v1 hash-map-empty))) (Some v2)

  first-pair-wins :: forall k v. using (hk :: Hash k, ek :: Eq k, ev :: Eq v) => k -> v -> v -> Bool
  first-pair-wins k v1 v2 = eq-opt (hash-map-lookup k (hash-map-from-pairs (Cons (k, v1) (Cons (k, v2) Nil)))) (Some v1)

  collision-scan :: Bool
  collision-scan =
    let m = hash-map-insert (8 :: Nat) 80 (hash-map-insert 0 100 hash-map-empty) in
    eq-opt (hash-map-lookup 8 m) (Some 80) && eq-opt (hash-map-lookup 0 m) (Some 100)

  absent :: Bool
  absent = eq-opt (hash-map-lookup (9 :: Nat) (hash-map-empty :: HashMap Nat Nat)) None

  main :: Int
  main =
    if run-nat 16 then
      (if run-int 16 then
        (if bool-char-string then
          (if round-trip (3 :: Nat) 30 hash-map-empty then
            (if insert-replaces (3 :: Nat) 1 2 then
              (if first-pair-wins (3 :: Nat) 1 2 then
                (if collision-scan then
                  (if absent then 0 else 8)
                else 7)
              else 6)
            else 5)
          else 4)
        else 3)
      else 2)
    else 1
"#,
        0,
    );
}

#[test]
fn show_is_the_documented_rendering() {
    // Text.Show's law shape (its header): the class rendering agrees
    // with the concrete conversions, and a `String` renders as itself.
    expect_exit(
        "show-rendering",
        r#"module Main where

  import Text.Show

  nat-agrees :: Nat -> Bool
  nat-agrees n = show n == nat-to-string n

  int-agrees :: Int -> Bool
  int-agrees x = show x == int-to-string x

  char-agrees :: Nat -> Bool
  char-agrees t = case u32-to-char (if t == 0 then 65 else if t == 1 then 90 else 97) of
    None -> True
    Some c -> show c == char-string c

  string-is-raw :: String -> Bool
  string-is-raw s = show s == s

  char-string-raw :: Bool
  char-string-raw = case u32-to-char 65 of
    None -> True
    Some c -> string-is-raw (char-string c)

  nat-sample :: Nat -> Nat
  nat-sample t = if t == 0 then 0 else if t == 1 then 7 else if t == 2 then 9 else if t == 3 then 10 else if t == 4 then 42 else 12345

  int-sample :: Nat -> Int
  int-sample t = if t == 0 then 0 else if t == 1 then 0 - 1 else if t == 2 then 42 else 0 - 42

  run :: Nat -> Bool
  run n =
    if n == 0 then True
    else
      let m = n - 1 in
      if nat-agrees (nat-sample (m % 6)) && int-agrees (int-sample (m % 4)) && char-agrees (m % 3) then run m
      else False

  main :: Int
  main =
    if run 60 then
      (if string-is-raw empty-string && char-string-raw then 0 else 2)
    else 1
"#,
        0,
    );
}

#[test]
fn codec_round_trips_the_d65_set() {
    // Text.Codec's law (its header): `decode (encode x) == Some x` for
    // the D65 covered set (`I64`, Data.Machine; `Bool`, Data.Bool),
    // with the documented column formats (an `I64` is 8 bytes
    // little-endian, a `Bool` one byte 0 or 1) and `None` on a
    // malformed column.
    expect_exit(
        "codec-round-trip",
        r#"module Main where

  import Text.Codec

  eq-opt :: forall a. using (ea :: Eq a) => Option a -> Option a -> Bool
  eq-opt x y = case x of
    None -> case y of
      None -> True
      Some _ -> False
    Some p -> case y of
      None -> False
      Some q -> p == q

  decode-i64 :: Bytes -> Option I64
  decode-i64 b = decode b

  decode-bool :: Bytes -> Option Bool
  decode-bool b = decode b

  sample :: Nat -> I64
  sample t =
    if t == 0 then 0
    else if t == 1 then 1
    else if t == 2 then wrapping-neg-i64 1
    else if t == 3 then wrapping-shl-i64 1 8
    else if t == 4 then wrapping-neg-i64 (wrapping-shl-i64 1 40)
    else wrapping-shl-i64 1 56

  run-i64 :: Nat -> Bool
  run-i64 n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = sample m in
      if eq-opt (decode-i64 (encode x)) (Some x) then run-i64 m else False

  bool-round-trip :: Bool
  bool-round-trip =
    eq-opt (decode-bool (encode True)) (Some True) &&
    eq-opt (decode-bool (encode False)) (Some False)

  format :: Bool
  format =
    bytes-len (encode (1 :: I64)) == 8 &&
    bytes-get (encode (wrapping-shl-i64 1 56)) 7 == 1 &&
    bytes-get (encode (wrapping-shl-i64 1 56)) 0 == 0 &&
    bytes-len (encode True) == 1 &&
    bytes-get (encode True) 0 == 1 &&
    bytes-get (encode False) 0 == 0

  malformed :: Bool
  malformed =
    let b = bytes-new 1 in
    let u = bytes-set b 0 2 in
    eq-opt (decode-i64 (bytes-new 3)) (None :: Option I64) &&
    eq-opt (decode-i64 (bytes-new 9)) (None :: Option I64) &&
    eq-opt (decode-bool (bytes-new 2)) (None :: Option Bool) &&
    eq-opt (decode-bool b) (None :: Option Bool)

  main :: Int
  main =
    if run-i64 6 then
      (if bool-round-trip then
        (if format then
          (if malformed then 0 else 4)
        else 3)
      else 2)
    else 1
"#,
        0,
    );
}

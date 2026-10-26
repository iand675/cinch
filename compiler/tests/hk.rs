//! A7 acceptance: the higher-kinded surface (var-headed type
//! application with beta-reducing substitution, partially applied type
//! constructors as class arguments, the `(->)` former, kind binders and
//! inference, dotted module names), the capture-avoiding instance
//! context, and the nullary class method round trip.
//!
//! The committed fixtures under `fixtures/hk/` are the acceptance:
//! `hk_surface_fixture.lang` is the full Functor/Applicative/Monad/
//! Alternative/Compose surface plus the four instance families and the
//! `Category.FamilyLaws` polymorphic law statements (it CHECKS),
//! `lib_category_check.lang` is the shipped family end to end (it
//! RUNS), and probe1-probe17 map each gap to its checker site. Probe
//! expectations: 1/2/5/6 flipped from rejection to acceptance at this
//! slice; 3/4/9/11/13/15/16 are controls that stay green; 8/12/14 are
//! the dotted imports (single-file, multi-file, aliased) and resolve
//! after the dotted-name fix; 7/10/17 keep their documented forms (7:
//! an instance head variable must be a declared instance parameter; 10:
//! `do` parses, the probe's own type error stays; 17: an unimported
//! name is unknown).

use safelang::driver::{check_sources, driver, run_source, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const HK_SURFACE: &str = include_str!("fixtures/hk/hk_surface_fixture.lang");
const LIB_CATEGORY_CHECK: &str = include_str!("fixtures/hk/lib_category_check.lang");
const PROBE1: &str = include_str!("fixtures/hk/probe1.lang");
const PROBE2: &str = include_str!("fixtures/hk/probe2.lang");
const PROBE3: &str = include_str!("fixtures/hk/probe3.lang");
const PROBE4: &str = include_str!("fixtures/hk/probe4.lang");
const PROBE5: &str = include_str!("fixtures/hk/probe5.lang");
const PROBE6: &str = include_str!("fixtures/hk/probe6.lang");
const PROBE7: &str = include_str!("fixtures/hk/probe7.lang");
const PROBE8: &str = include_str!("fixtures/hk/probe8.lang");
const PROBE9: &str = include_str!("fixtures/hk/probe9.lang");
const PROBE10: &str = include_str!("fixtures/hk/probe10.lang");
const PROBE11: &str = include_str!("fixtures/hk/probe11.lang");
const PROBE13: &str = include_str!("fixtures/hk/probe13.lang");
const PROBE14: &str = include_str!("fixtures/hk/probe14.lang");
const PROBE15: &str = include_str!("fixtures/hk/probe15.lang");
const PROBE16: &str = include_str!("fixtures/hk/probe16.lang");
const PROBE17: &str = include_str!("fixtures/hk/probe17.lang");
const MULTI_FUNCTOR: &str = include_str!("fixtures/hk/multi/Functor.lang");
const MULTI_MAIN: &str = include_str!("fixtures/hk/multi/Main.lang");

fn expect_ok(name: &str, src: &str) {
    if let Err(e) = check_sources(src, name) {
        panic!("{name}: expected check to pass, got: {e}");
    }
}

fn expect_reject(name: &str, needle: &str, src: &str) {
    match check_sources(src, name) {
        Ok(_) => panic!("{name}: expected rejection containing `{needle}`, but it checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: rejection `{msg}` does not contain `{needle}`"
            );
        }
    }
}

#[test]
fn hk_surface_fixture_checks() {
    expect_ok("hk_surface_fixture.lang", HK_SURFACE);
}

#[test]
fn lib_category_check_runs() {
    let code = run_source(LIB_CATEGORY_CHECK).expect("lib_category_check compiles");
    assert_eq!(code, 0, "lib_category_check exit code");
}

// Flipped probes: rejected before this slice, accepted now.

#[test]
fn probe1_class_method_var_headed_application() {
    expect_ok("probe1.lang", PROBE1);
}

#[test]
fn probe2_ctor_field_var_headed_application() {
    expect_ok("probe2.lang", PROBE2);
}

#[test]
fn probe5_capitalized_sig_param() {
    expect_ok("probe5.lang", PROBE5);
}

#[test]
fn probe6_kind_annotated_data_params() {
    expect_ok("probe6.lang", PROBE6);
}

// Control probes: green before and after.

#[test]
fn probe3_applied_class_arg_control() {
    expect_ok("probe3.lang", PROBE3);
}

#[test]
fn probe4_function_typed_methods_control() {
    expect_ok("probe4.lang", PROBE4);
}

#[test]
fn probe9_single_segment_import_control() {
    expect_ok("probe9.lang", PROBE9);
}

#[test]
fn probe11_instance_params_control() {
    expect_ok("probe11.lang", PROBE11);
}

#[test]
fn probe13_do_with_provider_control() {
    expect_ok("probe13.lang", PROBE13);
}

#[test]
fn probe15_module_type_ctor_names_control() {
    expect_ok("probe15.lang", PROBE15);
}

#[test]
fn probe16_char_nat_literals_control() {
    expect_ok("probe16.lang", PROBE16);
}

#[test]
fn probe10_do_parses_probe_type_error_stays() {
    // The probe's documented form: `do` parses (the block reached the
    // type checker); the probe author's own type error remains.
    expect_reject("probe10.lang", "type mismatch", PROBE10);
}

// Documented rejection forms.

#[test]
fn probe7_instance_head_variable_must_be_declared() {
    expect_reject("probe7.lang", "unknown type variable a", PROBE7);
}

#[test]
fn probe17_unimported_name_stays_unknown() {
    expect_reject("probe17.lang", "unknown name get-const", PROBE17);
}

// Dotted module names: the imports key on the full dotted name.

#[test]
fn probe8_dotted_import_single_file() {
    expect_ok("probe8.lang", PROBE8);
}

#[test]
fn probe14_dotted_import_aliased() {
    expect_ok("probe14.lang", PROBE14);
}

#[test]
fn probe12_dotted_import_multi_file() {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "safelang_hkmulti_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Functor.lang"), MULTI_FUNCTOR).expect("write module");
    std::fs::write(dir.join("Main.lang"), MULTI_MAIN).expect("write module");
    let code = match driver(Mode::Check, &dir.join("Main.lang"), None) {
        Ok(code) => code,
        Err(e) => panic!("probe12 (multi/): compilation failed: {e}"),
    };
    assert_eq!(code, 0, "probe12 (multi/) check exit code");
}

// The slice's semantic acceptance items.

#[test]
fn var_headed_beta_reduction() {
    // `tm :: forall a. f (f a) -> f a` at `f := Option` substitutes to
    // `Option (Option a) -> Option a` (the head's image collects its
    // arguments), and the instance body checks against the reduced
    // type.
    expect_ok(
        "hk-beta.lang",
        "module Main where\n\
         \x20 signature Twice (f :: Type -> Type) where\n\
         \x20   tm :: forall a. f (f a) -> f a\n\
         \x20 implicit instance Twice_Option :: Twice Option where\n\
         \x20   tm oo = case oo of\n\
         \x20     None -> None\n\
         \x20     Some o -> o\n\
         \x20 main :: Int\n\
         \x20 main = case tm (Some (Some (1 :: Int))) of\n\
         \x20   None -> 1\n\
         \x20   Some n -> if n == 1 then 0 else 2\n",
    );
    let code = run_source(
        "module Main where\n\
         \x20 signature Twice (f :: Type -> Type) where\n\
         \x20   tm :: forall a. f (f a) -> f a\n\
         \x20 implicit instance Twice_Option :: Twice Option where\n\
         \x20   tm oo = case oo of\n\
         \x20     None -> None\n\
         \x20     Some o -> o\n\
         \x20 main :: Int\n\
         \x20 main = case tm (Some (Some (1 :: Int))) of\n\
         \x20   None -> 1\n\
         \x20   Some n -> if n == 1 then 0 else 2\n",
    )
    .expect("beta program compiles");
    assert_eq!(code, 0, "beta program exit code");
}

#[test]
fn kind_inference_matches_annotation() {
    // `K1 (f :: Type -> Type)` and the un-annotated `K2 f` infer the
    // same kind from the same method shape.
    expect_ok(
        "hk-kinds.lang",
        "module Main where\n\
         \x20 signature K1 (f :: Type -> Type) where\n\
         \x20   m1 :: forall a. f a -> Int\n\
         \x20 signature K2 f where\n\
         \x20   m2 :: forall a. f a -> Int\n\
         \x20 data Id2 a where\n\
         \x20   Id2 :: a -> Id2 a\n\
         \x20 implicit instance K1_Id2 :: K1 Id2 where\n\
         \x20   m1 x = 0\n\
         \x20 implicit instance K2_Id2 :: K2 Id2 where\n\
         \x20   m2 x = 0\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    );
}

#[test]
fn capture_avoiding_instance_context() {
    // The `sa` case: `using (sa :: Semi a) => Semi (Dual a)` with the
    // instance parameter named like the class parameter. The context
    // must stay `sa :: Semi a`, so the body's `comb q p` at `a`
    // resolves through `sa` while the equation head is the instance
    // itself.
    let src = "module Main where\n\
         \x20 data Dual a where\n\
         \x20   Dual :: a -> Dual a\n\
         \x20 signature Semi a where\n\
         \x20   comb :: a -> a -> a\n\
         \x20 implicit instance Semi_Int :: Semi Int where\n\
         \x20   comb x y = x + y\n\
         \x20 implicit instance Semi_Dual a :: using (sa :: Semi a) => Semi (Dual a) where\n\
         \x20   comb x y = case x of\n\
         \x20     Dual p -> case y of\n\
         \x20       Dual q -> Dual (comb q p)\n\
         \x20 main :: Int\n\
         \x20 main = case comb (Dual (1 :: Int)) (Dual 2) of\n\
         \x20   Dual n -> if n == 3 then 0 else 1\n";
    expect_ok("hk-capture.lang", src);
    let code = run_source(src).expect("capture program compiles");
    assert_eq!(code, 0, "capture program exit code");
}

#[test]
fn nullary_method_round_trip() {
    // A class method with no parameters (`mempty :: a`'s shape): the
    // dictionary carries the value itself.
    let src = "module Main where\n\
         \x20 signature Pick a where\n\
         \x20   pick :: a\n\
         \x20 implicit instance Pick_Int :: Pick Int where\n\
         \x20   pick = 7\n\
         \x20 main :: Int\n\
         \x20 main = if pick == (7 :: Int) then 0 else 1\n";
    expect_ok("hk-nullary.lang", src);
    let code = run_source(src).expect("nullary program compiles");
    assert_eq!(code, 0, "nullary program exit code");
}

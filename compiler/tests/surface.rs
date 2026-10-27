//! The P19 language surface acceptance table: collection literals and
//! their compile-time fold (surface grammar local amendments section 10),
//! exports and re-exports (section 11), import renaming and selection
//! (section 11), and the file-type converters (section 11). These tests
//! drive the real CLI pipeline over temp multi-file projects.

use safelang::driver::{driver, Mode};
use std::path::{Path, PathBuf};

fn write_project(files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "safelang_surfacetest_{}_{}",
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

/// The manifest's `literal_folds` boundary: `(kind, folded)` per site.
fn literal_folds(dir: &Path) -> Vec<(String, bool)> {
    let text = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("manifest");
    let mut out = Vec::new();
    let start = text.find("\"literal_folds\"").expect("literal_folds section");
    let end = text[start..].find("\"data_imports\"").map(|i| i + start).unwrap_or(text.len());
    let mut rest = &text[start..end];
    while let Some(i) = rest.find("\"kind\": \"") {
        let kind_start = i + "\"kind\": \"".len();
        let kind_end = rest[kind_start..].find('"').unwrap() + kind_start;
        let kind = rest[kind_start..kind_end].to_string();
        let tail = &rest[kind_end..];
        let f = tail.find("\"folded\": ").unwrap();
        let folded = tail[f..].starts_with("\"folded\": true");
        out.push((kind, folded));
        rest = &rest[kind_end..];
    }
    out
}

fn manifest_has(dir: &Path, needle: &str) -> bool {
    std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .map(|t| t.contains(needle))
        .unwrap_or(false)
}

fn iface_value_names(dir: &Path, module: &str) -> Vec<String> {
    let text = std::fs::read_to_string(dir.join(format!("build/{module}.sliface")))
        .expect("interface");
    let mut out = Vec::new();
    let mut rest = text.as_str();
    while let Some(i) = rest.find("\"name\": \"") {
        let start = i + "\"name\": \"".len();
        let end = rest[start..].find('"').unwrap() + start;
        out.push(rest[start..end].to_string());
        rest = &rest[end..];
    }
    out
}

// ---------------------------------------- D105/D110: collection literals

#[test]
fn list_literal_folds_to_a_constant_and_a_runtime_call_stays() {
    // All-constant elements fold (the boundary records it); an element
    // that is a runtime value keeps its `from-list` call.
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 xs :: List Nat\n\
         \x20 xs = [1, 2, 3]\n\
         \x20 pick :: Nat -> List Nat\n\
         \x20 pick n = [n, 2]\n\
         \x20 main :: Int\n\
         \x20 main = if length xs == 3 && length (pick 7) == 2 then 0 else 1\n",
    )]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
    let folds = literal_folds(&dir);
    assert_eq!(
        folds,
        vec![("list".to_string(), true), ("list".to_string(), false)],
        "the constant literal folds and the runtime one does not"
    );
}

#[test]
fn bytes_literal_folds_and_map_literal_round_trips() {
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 buf :: Bytes\n\
         \x20 buf = [1, 2, 3]\n\
         \x20 m :: Map Nat Nat\n\
         \x20 m = {1 => 10, 2 => 20}\n\
         \x20 main :: Int\n\
         \x20 main = case map-lookup 2 m of\n\
         \x20   None -> 1\n\
         \x20   Some v -> if v == 20 && bytes-len buf == 3 then 0 else 2\n",
    )]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
    let folds = literal_folds(&dir);
    assert!(
        folds.iter().any(|(k, f)| k == "list" && *f),
        "the bytes list literal folds to a constant buffer: {folds:?}"
    );
    assert!(
        folds.iter().any(|(k, f)| k == "map" && *f),
        "the map literal folds to its constant tree: {folds:?}"
    );
}

#[test]
fn map_literal_defaults_and_wrong_instance_are_reported() {
    // An unconstrained `{k => v}` defaults to `Map` (decision D142).
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 main :: Int\n\
         \x20 main = let m = {1 => 42} in case map-lookup 1 m of\n\
         \x20   None -> 1\n\
         \x20   Some v -> if v == 42 then 0 else 2\n",
    )]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
    // A list literal at `Map` has no `FromList` instance.
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 bad :: Map Nat Nat\n\
         \x20 bad = [1, 2]\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    )]);
    let err = check_entry(&dir, "Main.lang").expect_err("wrong instance");
    assert!(err.contains("no instance for FromList"), "{err}");
}

#[test]
fn map_literal_keys_and_values_resolve_infix_chains() {
    // An infix chain in a map key or value resolves in place through
    // the `resolve_term` map arm (as the range literal does); before
    // the arm existed the chain reached the checker unresolved.
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 m :: Map Nat Nat\n\
         \x20 m = {1 + 2 => 3}\n\
         \x20 bump :: Nat -> Map Nat Nat\n\
         \x20 bump n = {n + 1 => n}\n\
         \x20 main :: Int\n\
         \x20 main = case map-lookup 3 m of\n\
         \x20   None -> 1\n\
         \x20   Some v -> case map-lookup 8 (bump 7) of\n\
         \x20     None -> 2\n\
         \x20     Some w -> if v == 3 && w == 7 then 0 else 3\n",
    )]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
}

// ---------------------------------------- the range literal (Data.Enum)

#[test]
fn range_literals_desugar_through_the_enum_class() {
    // `[e1..e2]` is the step-one enumeration and `[e1, e2 .. e3]` the
    // stepped one, both inclusive, through the `Enum` class: exact
    // sequences over `Int`, a descending step, the empty descending
    // case, and the `Nat` instance.
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 xs :: List Int\n\
         \x20 xs = [1..10]\n\
         \x20 ns :: List Nat\n\
         \x20 ns = [2..4]\n\
         \x20 stepped :: Int -> Int -> Int -> List Int\n\
         \x20 stepped a b z = [a, b .. z]\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> (case b of Nil -> True; Cons _ _ -> False)\n\
         \x20   Cons x xr -> (case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y yr -> if x == y then same xr yr else False)\n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   if same xs [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] then\n\
         \x20     (if same (stepped 1 4 10) [1, 4, 7, 10] then\n\
         \x20       (if same (stepped 10 8 0) [10, 8, 6, 4, 2, 0] then\n\
         \x20         (if length ns == 3 && same [3..1] [] then 0 else 4)\n\
         \x20       else 3)\n\
         \x20     else 2)\n\
         \x20   else 1\n",
    )]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
}

#[test]
fn range_literal_folds_when_stage_known() {
    // Constant endpoints fold through the D92 evaluator to a constant
    // value (the boundary records it); runtime endpoints keep the class
    // calls (surface grammar local amendments section 10's rule).
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 xs :: List Int\n\
         \x20 xs = [1..3]\n\
         \x20 pick :: Int -> List Int\n\
         \x20 pick n = [n..n + 2]\n\
         \x20 main :: Int\n\
         \x20 main = if length xs == 3 && length (pick 7) == 3 then 0 else 1\n",
    )]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
    let folds = literal_folds(&dir);
    assert_eq!(
        folds,
        vec![("range".to_string(), true), ("range".to_string(), false)],
        "the constant range folds and the runtime one does not"
    );
}

#[test]
fn range_without_an_enum_instance_is_reported() {
    // A range at a type with no `Enum` instance names the missing
    // instance (the wrong-instance negative of the class desugar).
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 data Toggle where\n\
         \x20   Off :: Toggle\n\
         \x20   On :: Toggle\n\
         \x20 bad :: List Toggle\n\
         \x20 bad = [Off..On]\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    )]);
    let err = check_entry(&dir, "Main.lang").expect_err("wrong instance");
    assert!(err.contains("no instance for Enum"), "{err}");
}

// ---------------------------------------- D107: exports and re-exports

#[test]
fn export_list_is_the_surface_and_re_exports_ride_it() {
    // With `export` declarations the surface is exactly the listed names.
    let dir = write_project(&[
        (
            "Conf.lang",
            "module Conf where\n\
             \x20 export mine\n\
             \x20 mine :: Int\n\
             \x20 mine = 1\n\
             \x20 secret :: Int\n\
             \x20 secret = 2\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Conf (mine)\n\
             \x20 main :: Int\n\
             \x20 main = if mine == 1 then 0 else 1\n",
        ),
    ]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
    let names = iface_value_names(&dir, "Conf");
    assert!(names.contains(&"mine".to_string()), "{names:?}");
    assert!(!names.contains(&"secret".to_string()), "{names:?}");
    // An unexported name is not importable.
    let dir = write_project(&[
        (
            "Conf.lang",
            "module Conf where\n\
             \x20 export mine\n\
             \x20 mine :: Int\n\
             \x20 mine = 1\n\
             \x20 secret :: Int\n\
             \x20 secret = 2\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Conf (secret)\n\
             \x20 main :: Int\n\
             \x20 main = secret\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("hidden member");
    assert!(err.contains("has no member `secret`"), "{err}");
    // `export module X` re-exports everything imported from X, and the
    // exporting module's own listed name with it.
    let dir = write_project(&[
        (
            "Lib.lang",
            "module Lib where\n\
             \x20 pub :: Int\n\
             \x20 pub = 1\n\
             \x20 hidden :: Int\n\
             \x20 hidden = 2\n",
        ),
        (
            "Conf.lang",
            "module Conf where\n\
             \x20 export module Lib, mine\n\
             \x20 import Lib\n\
             \x20 mine :: Int\n\
             \x20 mine = pub + 10\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Conf (mine, pub, hidden)\n\
             \x20 main :: Int\n\
             \x20 main = if mine == 11 && pub == 1 && hidden == 2 then 0 else 1\n",
        ),
    ]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
    let names = iface_value_names(&dir, "Conf");
    assert!(names.contains(&"mine".to_string()), "{names:?}");
    assert!(names.contains(&"pub".to_string()), "{names:?}");
    assert!(names.contains(&"hidden".to_string()), "{names:?}");
}

#[test]
fn named_re_export_is_reachable_by_its_exported_name() {
    let dir = write_project(&[
        (
            "Lib.lang",
            "module Lib where\n\
             \x20 parse :: Int -> Int\n\
             \x20 parse x = x + 1\n",
        ),
        (
            "Conf.lang",
            "module Conf where\n\
             \x20 export Lib.parse\n\
             \x20 import Lib\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Conf (parse)\n\
             \x20 main :: Int\n\
             \x20 main = if parse 1 == 2 then 0 else 1\n",
        ),
    ]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
}

// ---------------------------------------- D108: import renaming/selection

#[test]
fn import_alias_hides_bare_names_and_renames_the_qualifier() {
    let dir = write_project(&[
        (
            "Lib.lang",
            "module Lib where\n\
             \x20 greet :: Int -> Int\n\
             \x20 greet x = x + 1\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Lib as L\n\
             \x20 main :: Int\n\
             \x20 main = if L.greet 1 == 2 then 0 else 1\n",
        ),
    ]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
    // The bare name is not in scope.
    let dir = write_project(&[
        (
            "Lib.lang",
            "module Lib where\n\
             \x20 greet :: Int -> Int\n\
             \x20 greet x = x + 1\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Lib as L\n\
             \x20 main :: Int\n\
             \x20 main = greet 1\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("bare name hidden");
    assert!(err.contains("unknown name greet"), "{err}");
}

#[test]
fn member_selection_renames_and_hides_the_unselected() {
    let dir = write_project(&[
        (
            "Lib.lang",
            "module Lib where\n\
             \x20 one :: Int\n\
             \x20 one = 1\n\
             \x20 two :: Int\n\
             \x20 two = 2\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Lib (one as uno, two)\n\
             \x20 main :: Int\n\
             \x20 main = if uno + two == 3 then 0 else 1\n",
        ),
    ]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
    // The unrenamed spelling of the renamed member is not in scope.
    let dir = write_project(&[
        (
            "Lib.lang",
            "module Lib where\n\
             \x20 one :: Int\n\
             \x20 one = 1\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Lib (one as uno)\n\
             \x20 main :: Int\n\
             \x20 main = one\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("renamed member");
    assert!(err.contains("unknown name one"), "{err}");
}

#[test]
fn duplicate_unrenamed_imported_names_error_names_both_sources() {
    let dir = write_project(&[
        (
            "A.lang",
            "module A where\n\
             \x20 shared :: Int\n\
             \x20 shared = 1\n",
        ),
        (
            "B.lang",
            "module B where\n\
             \x20 shared :: Int\n\
             \x20 shared = 2\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import A\n\
             \x20 import B\n\
             \x20 main :: Int\n\
             \x20 main = shared\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("duplicate names");
    assert!(
        err.contains("imported from both A and B"),
        "the error names both sources: {err}"
    );
    // Renaming one import is the fix.
    let dir = write_project(&[
        (
            "A.lang",
            "module A where\n\
             \x20 shared :: Int\n\
             \x20 shared = 1\n",
        ),
        (
            "B.lang",
            "module B where\n\
             \x20 shared :: Int\n\
             \x20 shared = 2\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import A\n\
             \x20 import B (shared as theirs)\n\
             \x20 main :: Int\n\
             \x20 main = if shared + theirs == 3 then 0 else 1\n",
        ),
    ]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
}

// ---------------------------------------- D109: file-type converters

#[test]
fn json_txt_and_bytes_converters_bind_and_record_provenance() {
    let dir = write_project(&[
        ("config.json", "{\"port\": 8080, \"name\": \"svc\"}"),
        ("note.txt", "hello"),
        ("blob.bytes", "\x01\x02\x03"),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import data \"config.json\" as config\n\
             \x20 import data \"note.txt\" as note\n\
             \x20 import data \"blob.bytes\" as blob\n\
             \x20 sizes :: Int\n\
             \x20 sizes = if bytes-len blob == 3 && bytes-len (string-to-bytes note) == 5 then 0 else 100\n\
             \x20 main :: Int\n\
             \x20 main = sizes + (case config of\n\
             \x20   JObj k v rest -> case rest of\n\
             \x20     JObj k2 v2 rest2 -> 0\n\
             \x20     JObjEnd -> 1\n\
             \x20     JNull -> 2\n\
             \x20     JBool b -> 3\n\
             \x20     JNum n -> 4\n\
             \x20     JFloat f -> 5\n\
             \x20     JStr s -> 6\n\
             \x20     JArr a t -> 7\n\
             \x20     JArrEnd -> 8\n\
             \x20   JObjEnd -> 9\n\
             \x20   JNull -> 9\n\
             \x20   JBool b -> 9\n\
             \x20   JNum n -> 9\n\
             \x20   JFloat f -> 9\n\
             \x20   JStr s -> 9\n\
             \x20   JArr a t -> 9\n\
             \x20   JArrEnd -> 9)\n",
        ),
    ]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
    // Provenance: converter identity and file content hash per binding.
    assert!(manifest_has(&dir, "\"converter\": \"json\""), "json provenance");
    assert!(manifest_has(&dir, "\"converter\": \"txt\""), "txt provenance");
    assert!(manifest_has(&dir, "\"converter\": \"bytes\""), "bytes provenance");
    assert!(manifest_has(&dir, "\"hash\": \""), "content hashes recorded");
    assert!(!manifest_has(&dir, "hello"), "contents stay out of the manifest");
}

#[test]
fn json_decodes_a_declared_record_type() {
    let dir = write_project(&[
        ("config.json", "{\"port\": 8080, \"debug\": true}"),
        (
            "Main.lang",
            "module Main where\n\
             \x20 record Config where\n\
             \x20   port :: Nat\n\
             \x20   debug :: Bool\n\
             \x20 config :: Config\n\
             \x20 import data \"config.json\" as config\n\
             \x20 main :: Int\n\
             \x20 main = case config of\n\
             \x20   Config { port = p, debug = b } -> if p == 8080 && b then 0 else 1\n",
        ),
    ]);
    assert_eq!(run_entry(&dir, "Main.lang").expect("run"), 0);
}

#[test]
fn converter_failures_name_the_file_and_the_converter() {
    let dir = write_project(&[
        ("config.json", "{not json"),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import data \"config.json\" as config\n\
             \x20 main :: Int\n\
             \x20 main = 0\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("bad json");
    assert!(
        err.contains("`json` converter") && err.contains("config.json"),
        "the error names the file and the converter: {err}"
    );
    let dir = write_project(&[
        ("thing.qoi", "x"),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import data \"thing.qoi\" as thing\n\
             \x20 main :: Int\n\
             \x20 main = 0\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("unknown extension");
    assert!(err.contains("no converter"), "{err}");
}

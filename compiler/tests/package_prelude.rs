//! Package-level behaviour the compiler now enforces: a module another
//! package owns is importable only when that package exposed it, a
//! package's own `prelude` is injected as a real dependency, and a named
//! prelude that nothing declares is refused at load time
//! (design/base-as-a-package.md sections 3 and 4.2).

use std::path::{Path, PathBuf};

fn scratch(tag: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("cinch-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("scratch dir");
    path
}

fn write(dir: &Path, files: &[(&str, &str)]) {
    for (name, text) in files {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("parent dir");
        }
        std::fs::write(&path, text).expect("write fixture");
    }
}

fn check(dir: &Path, entry: &str) -> Result<(), String> {
    cinch::driver::driver(cinch::driver::Mode::Check, &dir.join(entry), None)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

const LIB_MANIFEST: &str = r#"{
  "schema": 2,
  "name": "acme/lib",
  "version": "0.1.0",
  "modules": [
    { "name": "Api", "path": "Api.cinch" },
    { "name": "Internal", "path": "Internal.cinch", "exposed": false }
  ],
  "order": ["Api", "Internal"]
}"#;

fn library(dir: &Path) {
    write(dir, &[
        (
            "vendor/acme/lib/0.1.0/.cinchpkg",
            LIB_MANIFEST,
        ),
        (
            "vendor/acme/lib/0.1.0/Api.cinch",
            "module Api where\n  export greeting\n\n  greeting :: Int\n  greeting = 7\n",
        ),
        (
            "vendor/acme/lib/0.1.0/Internal.cinch",
            "module Internal where\n  secret :: Int\n  secret = 8\n",
        ),
    ]);
}

#[test]
fn an_exposed_module_is_importable_and_an_internal_one_is_not() {
    let dir = scratch("pkg-exposed");
    library(&dir);
    write(
        &dir,
        &[(
            "Main.cinch",
            "module Main where\n  import Api\n\n  main :: Int\n  main = greeting - 7\n",
        )],
    );
    check(&dir, "Main.cinch").expect("an exposed module imports");

    write(
        &dir,
        &[(
            "Main2.cinch",
            "module Main2 where\n  import Internal\n\n  main :: Int\n  main = secret - 8\n",
        )],
    );
    let err = check(&dir, "Main2.cinch").expect_err("an internal module is refused");
    assert!(
        err.contains("is not exposed"),
        "the refusal names the exposure rule: {err}"
    );
    assert!(err.contains("acme/lib"), "and the package: {err}");
}

const PRELUDE_MANIFEST: &str = r#"{
  "schema": 2,
  "name": "acme/app",
  "version": "0.1.0",
  "prelude": "Acme.Prelude",
  "modules": [
    { "name": "Acme.Prelude", "path": "Acme.Prelude.cinch" },
    { "name": "Main", "path": "Main.cinch" }
  ],
  "order": ["Acme.Prelude", "Main"]
}"#;

#[test]
fn a_packages_prelude_is_an_implicit_import() {
    let dir = scratch("pkg-prelude");
    write(
        &dir,
        &[
            (".cinchpkg", PRELUDE_MANIFEST),
            (
                "Acme.Prelude.cinch",
                "module Acme.Prelude where\n  import Prelude\n  export module Prelude\n\n  export helper\n\n  helper :: Int -> Int\n  helper n = n + 1\n",
            ),
            (
                "Main.cinch",
                "module Main where\n\n  main :: Int\n  main = helper 41 - 42\n",
            ),
        ],
    );
    check(&dir, "Main.cinch").expect("the injected prelude supplies helper and the operators");
}

#[test]
fn an_undeclared_prelude_is_refused_at_load() {
    let dir = scratch("pkg-bad-prelude");
    let manifest = PRELUDE_MANIFEST.replace(
        "\"prelude\": \"Acme.Prelude\"",
        "\"prelude\": \"Nope.Missing\"",
    );
    write(
        &dir,
        &[
            (".cinchpkg", manifest.as_str()),
            (
                "Acme.Prelude.cinch",
                "module Acme.Prelude where\n  import Prelude\n  export module Prelude\n",
            ),
            (
                "Main.cinch",
                "module Main where\n\n  main :: Int\n  main = 0\n",
            ),
        ],
    );
    let err = check(&dir, "Main.cinch").expect_err("a prelude nobody declares is refused");
    assert!(err.contains("prelude `Nope.Missing`"), "{err}");
}

#[test]
fn two_paths_to_one_declaration_are_one_name() {
    let dir = scratch("pkg-diamond");
    write(
        &dir,
        &[
            ("A.cinch", "module A where\n  x :: Int\n  x = 5\n"),
            (
                "B.cinch",
                "module B where\n  import A\n  export module A\n",
            ),
            (
                "Main.cinch",
                "module Main where\n  import A\n  import B\n\n  main :: Int\n  main = x - 5\n",
            ),
        ],
    );
    check(&dir, "Main.cinch").expect("re-export plus direct import is one name");

    write(
        &dir,
        &[("C.cinch", "module C where\n  x :: Int\n  x = 9\n")],
    );
    write(
        &dir,
        &[(
            "Main2.cinch",
            "module Main2 where\n  import A\n  import C\n\n  main :: Int\n  main = x\n",
        )],
    );
    let err = check(&dir, "Main2.cinch").expect_err("two declarations under one name collide");
    assert!(
        err.contains("is imported from both A and C"),
        "the plain two-module message keeps its shape: {err}"
    );
}

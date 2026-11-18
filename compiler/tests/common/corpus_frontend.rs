//! Select the public CLI and check the artifact identities recorded by corpus runners.
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    env,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

pub enum Frontend {
    Rust(PathBuf),
    Native(PathBuf),
}

impl Frontend {
    pub fn selected(rust_runtime: &Path) -> Self {
        let native = env::var_os("CINCH_TEST_STANDALONE").map(PathBuf::from);
        let selectors = [
            ("CINCH_TEST_INTERFACES", "base-interfaces.json"),
            ("CINCH_TEST_OBJECTS", "objects.json"),
            ("CINCH_TEST_RUNTIME_ARCHIVE", "libcinchruntime.a"),
        ];
        if let Some(directory) = native {
            assert!(env::var_os("CINCH_TEST_COMPILER").is_some());
            for (selector, filename) in selectors {
                let selected =
                    env::var_os(selector).unwrap_or_else(|| panic!("missing {selector}"));
                assert_eq!(
                    fs::canonicalize(selected).unwrap(),
                    directory.join(filename).canonicalize().unwrap()
                );
            }
            Self::Native(directory)
        } else {
            for selector in selectors
                .map(|(selector, _)| selector)
                .into_iter()
                .chain(["CINCH_TEST_COMPILER"])
            {
                assert!(
                    env::var_os(selector).is_none(),
                    "{selector} requires a matched CINCH_TEST_STANDALONE for corpus tests"
                );
            }
            Self::Rust(rust_runtime.to_owned())
        }
    }

    pub fn arguments(&self) -> Vec<OsString> {
        match self {
            Self::Rust(runtime) => vec![
                "--rust".into(),
                env!("CARGO_BIN_EXE_cinch").into(),
                "--rust-runtime".into(),
                runtime.into(),
                "--initialize-rust-runtime".into(),
            ],
            Self::Native(directory) => vec!["--native".into(), directory.into()],
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Rust(_) => "rust",
            Self::Native(_) => "native",
        }
    }

    pub fn check_identity(&self, report: &Value) {
        assert_eq!(report["frontends"].as_object().unwrap().len(), 1);
        let identity = &report["frontends"][self.label()];
        match self {
            Self::Rust(runtime) => {
                let init = &report["runtime_initialization"];
                assert_eq!(init["archive_path"].as_str(), runtime.to_str());
                assert!(init["archive_sha256"].is_string());
                assert_eq!(identity["runtime_sha256"], init["archive_sha256"]);
                assert_eq!(init["snapshot_sha256"], init["archive_sha256"]);
            }
            Self::Native(directory) => {
                for (key, file) in [
                    ("compiler_sha256", "cinch"),
                    ("runtime_sha256", "libcinchruntime.a"),
                    ("base-interfaces.json", "base-interfaces.json"),
                    ("objects.json", "objects.json"),
                ] {
                    let mut digest = Sha256::new();
                    io::copy(
                        &mut fs::File::open(directory.join(file)).unwrap(),
                        &mut digest,
                    )
                    .unwrap();
                    assert_eq!(identity[key], format!("{:x}", digest.finalize()), "{file}");
                }
                let objects: Value =
                    serde_json::from_slice(&fs::read(directory.join("objects.json")).unwrap())
                        .unwrap();
                assert!(objects["object_sha256"].is_string());
                assert_eq!(identity["provider_sha256"], objects["object_sha256"]);
            }
        }
    }
}

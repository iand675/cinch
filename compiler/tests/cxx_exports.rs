//! Exercise the complete foreign-export path from Cinch to a real C++ host.
use cinch::cxx::{self, Bindings};
use std::{path::Path, process::Command};

fn example() -> Bindings {
    serde_json::from_str(include_str!("../../examples/cxx/exports/bindings.json")).unwrap()
}
fn copy_example(from: &Path, to: &Path, files: &[&str]) {
    for file in files {
        std::fs::copy(from.join(file), to.join(file)).unwrap();
    }
}
fn run(command: &mut Command) -> std::process::Output {
    let output = command
        .output()
        .unwrap_or_else(|e| panic!("{command:?}: {e}"));
    assert!(
        output.status.success(),
        "{command:?}: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn cpp_classes_call_cinch_keep_independent_state_and_destroy_once() {
    let dir = tempfile::tempdir().unwrap();
    let source = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../examples/cxx/exports"
    ));
    copy_example(
        source,
        dir.path(),
        &[
            "bindings.json",
            "Logic.cinch",
            "Lifecycle.cpp",
            "host.cpp",
            "host.c",
        ],
    );
    let out = dir.path().join("build");
    assert_eq!(
        cxx::cli(&[
            "build".into(),
            dir.path()
                .join("bindings.json")
                .to_string_lossy()
                .into_owned(),
            "--out-dir".into(),
            out.to_string_lossy().into_owned()
        ]),
        0
    );
    let host = out.join("host");
    run(
        Command::new(std::env::var("CINCH_CXX").unwrap_or_else(|_| "c++".into()))
            .arg("-std=c++17")
            .arg(dir.path().join("host.cpp"))
            .arg("-I")
            .arg(&out)
            .arg("-L")
            .arg(&out)
            .arg("-lHostApi")
            .arg(format!("-Wl,-rpath,{}", out.display()))
            .arg("-o")
            .arg(&host),
    );
    let host_run = run(Command::new(&host).env("CINCH_ALLOC_STATS", "1"));
    let stderr = String::from_utf8_lossy(&host_run.stderr);
    let stats = stderr
        .lines()
        .find(|line| line.starts_with("cinch-alloc:"))
        .expect("the exported heap computation must exercise allocation");
    assert!(
        stats.split_whitespace().any(|word| word == "live=0"),
        "{stats}"
    );
    assert!(!stats.contains(" peak=0 "), "{stats}");
    if cfg!(target_os = "linux") {
        let dynamic = run(Command::new("readelf")
            .arg("-d")
            .arg(out.join("libHostApi.so")));
        assert!(!String::from_utf8_lossy(&dynamic.stdout).contains("TEXTREL"));
        let symbols = run(Command::new("nm")
            .args(["-D", "--defined-only"])
            .arg(out.join("libHostApi.so")));
        let symbols = String::from_utf8_lossy(&symbols.stdout);
        assert!(symbols.contains("cinch_export_HostApi_Counter_add"));
        assert!(
            !symbols.lines().any(|line| line.ends_with(" cinch_alloc")),
            "each library must keep its runtime private"
        );
    }
    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(out.join("build/HostApiLibrary.manifest.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(manifest["link"]["shared"], true);
    assert_eq!(
        manifest["link"]["native_objects"].as_array().unwrap().len(),
        2
    );
    assert_eq!(
        manifest["link"]["native_objects"][0]["hash"]
            .as_str()
            .unwrap(),
        cinch::interface::content_hash_bytes(&std::fs::read(out.join("native-0.o")).unwrap())
    );
    let c_host = out.join("c-host");
    run(Command::new("cc")
        .arg(dir.path().join("host.c"))
        .arg("-I")
        .arg(&out)
        .arg("-L")
        .arg(&out)
        .arg("-lHostApi")
        .arg(format!("-Wl,-rpath,{}", out.display()))
        .arg("-o")
        .arg(&c_host));
    run(&mut Command::new(&c_host));
    // Failed relinks must preserve the working library for a running host.
    let library = out.join(if cfg!(target_os = "macos") {
        "libHostApi.dylib"
    } else {
        "libHostApi.so"
    });
    let good_bytes = std::fs::read(&library).unwrap();
    let bad = out.join("bad.o");
    std::fs::write(&bad, "invalid object bytes").unwrap();
    let opts = cinch::driver::DriverOptions {
        shared: true,
        native_objects: vec![bad],
        ..Default::default()
    };
    assert!(cinch::driver::driver_opts(
        cinch::driver::Mode::Build,
        &out.join("HostApiLibrary.cinch"),
        Some(&library),
        &opts
    )
    .is_err());
    assert_eq!(std::fs::read(&library).unwrap(), good_bytes);
    run(&mut Command::new(&host));
}

#[test]
fn export_metadata_rejects_unsafe_or_ambiguous_shapes_before_output() {
    let json = serde_json::to_value(
        serde_json::from_str::<serde_json::Value>(include_str!(
            "../../examples/cxx/exports/bindings.json"
        ))
        .unwrap(),
    )
    .unwrap();
    let mut invalid = json.clone();
    invalid["export_classes"][0]["fields"][0]["type"] = "I8".into();
    invalid["export_classes"][0]["fields"][0]["initial"] = 1000.into();
    assert!(cxx::generate(&serde_json::from_value::<Bindings>(invalid).unwrap()).is_err());
    let mut invalid = json.clone();
    invalid["export_classes"][0]["methods"][0]["name"] = "get_count".into();
    assert!(cxx::generate(&serde_json::from_value::<Bindings>(invalid).unwrap()).is_err());
    let mut invalid = json.clone();
    invalid["export_classes"][0]["methods"][0]["name"] = "create".into();
    assert!(cxx::generate(&serde_json::from_value::<Bindings>(invalid).unwrap()).is_err());
    let mut invalid = json;
    invalid["exports"][0]["function"] = "Logic.add; injected".into();
    assert!(cxx::generate(&serde_json::from_value::<Bindings>(invalid).unwrap()).is_err());
    let mut named = example();
    named.exports[0].symbol = Some("native_add".into());
    let generated = cinch::cxx::exports::extra_files(&named).unwrap();
    assert!(generated["HostApiExports.cinch"].contains("foreign export ccall \"native_add\""));
    assert!(generated["HostApi.h"].contains("native_add("));
    let (api, _) = cxx::generate(&example()).unwrap();
    assert!(api.contains("getCounterCount"));
}

#[test]
fn shared_build_rejects_main_and_linker_options_as_native_objects() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Main.cinch");
    std::fs::write(&path, "module Main where\n  main :: Int\n  main = 0\n").unwrap();
    let opts = cinch::driver::DriverOptions {
        shared: true,
        ..Default::default()
    };
    let error = cinch::driver::driver_opts(
        cinch::driver::Mode::Build,
        &path,
        Some(&dir.path().join("libbad.so")),
        &opts,
    )
    .unwrap_err();
    assert!(error.to_string().contains("without main"), "{error}");
    let opts = cinch::driver::DriverOptions {
        native_objects: vec!["-Wl,-shared".into()],
        ..Default::default()
    };
    let error =
        cinch::driver::driver_opts(cinch::driver::Mode::Build, &path, None, &opts).unwrap_err();
    assert!(error.to_string().contains("not a linker option"), "{error}");
}

#[test]
#[ignore = "requires CINCH_GODOT, CINCH_GODOT_CPP and SCons; builds and runs a real GDExtension"]
fn godot_registers_classes_calls_cinch_and_releases_instances() {
    let godot = std::env::var("CINCH_GODOT").expect("CINCH_GODOT executable");
    let sdk = std::env::var("CINCH_GODOT_CPP").expect("CINCH_GODOT_CPP checkout");
    let dir = tempfile::tempdir().unwrap();
    let source = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/godot"));
    copy_example(
        source,
        dir.path(),
        &[
            "bindings.json",
            "GameLogic.cinch",
            "Lifecycle.cpp",
            "project.godot",
            "main.tscn",
            "test.gd",
        ],
    );
    let out = dir.path().join("build");
    assert_eq!(
        cxx::cli(&[
            "build".into(),
            dir.path()
                .join("bindings.json")
                .to_string_lossy()
                .into_owned(),
            "--out-dir".into(),
            out.to_string_lossy().into_owned(),
            "--godot-cpp".into(),
            sdk
        ]),
        0
    );
    run(Command::new(&godot)
        .args(["--headless", "--editor", "--import", "--path"])
        .arg(dir.path()));
    let output = run(Command::new(&godot)
        .args(["--headless", "--path"])
        .arg(dir.path())
        .args(["--script", "test.gd", "--quit-after", "120"]));
    assert!(String::from_utf8_lossy(&output.stdout).contains("CINCH_GODOT_OK"));
}

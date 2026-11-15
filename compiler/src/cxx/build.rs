//! Repeatable native library builds for generated imports and exports.
use super::{exports, fail, Bindings};
use crate::{
    driver::{self, DriverOptions, Mode},
    Result,
};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    process::Command,
};

const GODOT_CPP_TAG: &str = "godot-4.5-stable";
const GODOT_CPP_REV: &str = "e83fd0904c13356ed1d4c3d09f8bb9132bdc6b77";

fn io<T>(value: std::io::Result<T>) -> Result<T> {
    value.map_err(|e| fail(e.to_string()))
}
fn run(command: &mut Command) -> Result<()> {
    let status = io(command.status())?;
    if status.success() {
        Ok(())
    } else {
        Err(fail(format!("{command:?} failed: {status}")))
    }
}

pub fn cli(args: &[String]) -> i32 {
    let args = args.to_vec();
    match std::thread::Builder::new()
        .name("cinch-native-build".into())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || build(&args))
    {
        Ok(thread) => match thread.join() {
            Ok(Ok(_)) => 0,
            Ok(Err(e)) => {
                eprintln!("{e}");
                2
            }
            Err(_) => {
                eprintln!("C++ library build panicked");
                2
            }
        },
        Err(e) => {
            eprintln!("cannot start native build: {e}");
            2
        }
    }
}

/// Generate, compile C++ glue, check Cinch implementation modules, and link
/// one self-contained shared library. No shell interpolation or hand linking.
pub fn build(args: &[String]) -> Result<PathBuf> {
    if args.len() < 3 || args[1] != "--out-dir" {
        return Err(fail(
            "usage: cinch cxx build bindings.json --out-dir DIR [--godot-cpp DIR]",
        ));
    }
    let spec_path = io(std::fs::canonicalize(&args[0]))?;
    let root = spec_path.parent().unwrap();
    let mut spec: Bindings = serde_json::from_str(&io(std::fs::read_to_string(&spec_path))?)
        .map_err(|e| fail(e.to_string()))?;
    super::script::load_sources(&mut spec, root)?;
    if spec.exports.is_empty() && spec.export_classes.is_empty() {
        return Err(fail("cxx build needs exported functions or classes"));
    }
    if spec.sources.is_empty() {
        return Err(fail(
            "cxx build needs sources containing the Cinch implementations",
        ));
    }
    if !cfg!(any(target_os = "linux", target_os = "macos")) {
        return Err(fail(
            "native library builds currently support Linux and macOS",
        ));
    }
    let mut sdk_override = None;
    if args.len() != 3 {
        if args.len() == 5 && args[3] == "--godot-cpp" {
            sdk_override = Some(PathBuf::from(&args[4]));
        } else {
            return Err(fail("unknown cxx build option"));
        }
    }
    if sdk_override.is_some() && spec.godot.is_none() {
        return Err(fail("--godot-cpp requires Godot output"));
    }
    // Validate every artifact before creating the output directory.
    let (api, cpp) = super::generate(&spec)?;
    let extra = exports::extra_files(&spec)?;
    let mut implementations = String::new();
    for source in &spec.sources {
        // Compile exactly the source revision embedded in each script descriptor.
        let text = match spec.script_sources.get(source) {
            Some(snapshot) => snapshot.clone(),
            None => io(std::fs::read_to_string(root.join(source)))?,
        };
        implementations.push_str(&text);
        implementations.push('\n');
    }
    io(std::fs::create_dir_all(&args[2]))?;
    let out = io(std::fs::canonicalize(&args[2]))?;
    if out == root {
        return Err(fail(
            "use a separate output directory, not the authored source directory",
        ));
    }
    if spec.godot.is_some() && !out.starts_with(root) {
        return Err(fail(
            "Godot output must be inside the project directory containing bindings.json",
        ));
    }
    let manifest_path = out.join(".cinchpkg");
    if manifest_path.exists() {
        let old: serde_json::Value =
            serde_json::from_str(&io(std::fs::read_to_string(&manifest_path))?)
                .map_err(|e| fail(e.to_string()))?;
        if old["name"] != "cinch/generated-library" {
            return Err(fail("output directory contains an authored package manifest; choose a generated build directory"));
        }
    }
    let stem = spec.module.replace('.', "_");
    for (path, text) in &extra {
        let path = out.join(path);
        io(std::fs::create_dir_all(path.parent().unwrap()))?;
        io(std::fs::write(path, text))?;
    }
    let export_source = extra
        .get(&format!("{}Exports.cinch", spec.module.replace('.', "/")))
        .ok_or_else(|| fail("missing generated export module"))?;
    // The explicit source set forms one checked compilation; the source files
    // stay authored and error locations in this generated unit are inspectable.
    let library_source = out.join(format!("{stem}Library.cinch"));
    io(std::fs::write(
        &library_source,
        format!("{api}\n{implementations}\n{export_source}"),
    ))?;
    let cpp_path = out.join(format!("{stem}.cpp"));
    io(std::fs::write(&cpp_path, cpp))?;
    let cxx = std::env::var("CINCH_CXX").unwrap_or_else(|_| "c++".into());
    let mut includes = vec![out.clone(), root.to_path_buf()];
    let mut cpp_defines = Vec::<String>::new();
    let mut objects = Vec::new();
    let mut sdk_record = serde_json::Value::Null;
    let mut sdk_minimum = String::from("4.5");
    if let Some(godot) = &spec.godot {
        let sdk = match sdk_override.or_else(|| godot.godot_cpp.as_ref().map(|p| root.join(p))) {
            Some(path) => io(std::fs::canonicalize(path))?,
            None => {
                let path = out.join("deps/godot-cpp");
                if !path.exists() {
                    io(std::fs::create_dir_all(path.parent().unwrap()))?;
                    run(Command::new("git")
                        .args([
                            "clone",
                            "--depth",
                            "1",
                            "--branch",
                            GODOT_CPP_TAG,
                            "https://github.com/godotengine/godot-cpp.git",
                        ])
                        .arg(&path))?;
                }
                let revision = git_revision(&path)?;
                if revision != GODOT_CPP_REV {
                    return Err(fail("cached Godot SDK does not match the pinned revision; supply --godot-cpp for a custom checkout"));
                }
                path
            }
        };
        let api: serde_json::Value = serde_json::from_str(&io(std::fs::read_to_string(
            sdk.join("gdextension/extension_api.json"),
        ))?)
        .map_err(|e| fail(e.to_string()))?;
        if api["header"]["version_major"] != 4 {
            return Err(fail("Godot output requires a Godot 4 SDK"));
        }
        let minor = api["header"]["version_minor"]
            .as_u64()
            .ok_or_else(|| fail("Godot SDK has no version"))?;
        if minor < 5 {
            return Err(fail("Godot output requires SDK 4.5 or newer"));
        }
        sdk_minimum = format!("4.{minor}");
        let mut enabled = spec
            .export_classes
            .iter()
            .filter_map(|c| c.base.as_ref())
            .map(|b| b.rsplit("::").next().unwrap().to_string())
            .collect::<BTreeSet<_>>();
        enabled.extend(["OS", "Engine"].into_iter().map(str::to_string));
        // Include source editing even when the project only exports native classes.
        {
            enabled.extend(
                [
                    "ScriptLanguageExtension",
                    "ScriptExtension",
                    "ResourceFormatLoader",
                    "ResourceFormatSaver",
                    "ResourceLoader",
                    "ResourceSaver",
                    "EditorInterface",
                    "EditorPlugin",
                    "ScriptEditor",
                    "EditorSyntaxHighlighter",
                    "TextEdit",
                    "CodeEdit",
                ]
                .into_iter()
                .map(str::to_string),
            );
        }
        let profile = out.join("godot-build-profile.json");
        io(std::fs::write(
            &profile,
            serde_json::json!({"enabled_classes":enabled}).to_string(),
        ))?;
        let platform = if cfg!(target_os = "macos") {
            "macos"
        } else {
            "linux"
        };
        let arch = if cfg!(target_arch = "aarch64") {
            "arm64"
        } else {
            "x86_64"
        };
        let jobs = std::thread::available_parallelism()
            .map(|v| v.get().min(4))
            .unwrap_or(1);
        let mut build = Command::new("scons");
        build
            .current_dir(&sdk)
            .arg(format!("platform={platform}"))
            .arg(format!("arch={arch}"))
            .args([
                "target=template_debug",
                "precision=single",
                "debug_symbols=no",
                "threads=yes",
                "use_hot_reload=no",
                "dev_build=no",
            ])
            .arg(format!("build_profile={}", profile.display()))
            .arg(format!("-j{jobs}"));
        run(&mut build)?;
        // Keep the adapter's header ABI identical to this SDK build, including
        // the platform/thread configuration. Hot reload is disabled on both.
        cpp_defines.extend(
            [
                "GDEXTENSION",
                "THREADS_ENABLED",
                "DEBUG_ENABLED",
                "NDEBUG",
                "UNIX_ENABLED",
            ]
            .into_iter()
            .map(str::to_string),
        );
        cpp_defines.push(if cfg!(target_os = "macos") {
            "MACOS_ENABLED".into()
        } else {
            "LINUX_ENABLED".into()
        });
        includes.extend([
            sdk.join("include"),
            sdk.join("gen/include"),
            sdk.join("gdextension"),
        ]);
        let archive = sdk.join(format!(
            "bin/libgodot-cpp.{platform}.template_debug.{arch}.a"
        ));
        if !archive.is_file() {
            return Err(fail(format!(
                "Godot SDK did not produce {}",
                archive.display()
            )));
        }
        objects.push(archive);
        sdk_record = serde_json::json!({"path":sdk, "revision":git_revision(&sdk)?, "profile":enabled, "platform":platform, "arch":arch});
    }
    let mut native_sources = vec![cpp_path];
    native_sources.extend(spec.native_sources.iter().map(|p| root.join(p)));
    // Object order matters for archives: the adapters precede the SDK.
    let mut native_objects = Vec::new();
    for (i, source) in native_sources.iter().enumerate() {
        let object = out.join(format!("native-{i}.o"));
        let mut compile = Command::new(&cxx);
        compile
            .args(["-std=c++17", "-fPIC", "-fvisibility=hidden", "-c"])
            .arg(source)
            .arg("-o")
            .arg(&object);
        for define in &cpp_defines {
            compile.arg(format!("-D{define}"));
        }
        for include in &includes {
            compile.arg("-I").arg(include);
        }
        run(&mut compile)?;
        native_objects.push(object);
    }
    native_objects.extend(objects);
    let mut libs = spec.libraries.clone();
    libs.push(if cfg!(target_os = "macos") {
        "c++".into()
    } else {
        "stdc++".into()
    });
    let manifest = serde_json::json!({"name":"cinch/generated-library", "version":"0.1.0", "modules":[], "order":[], "link":{"libs":libs}});
    io(std::fs::write(out.join(".cinchpkg"), manifest.to_string()))?;
    let library = out.join(format!(
        "lib{stem}.{}",
        if cfg!(target_os = "macos") {
            "dylib"
        } else {
            "so"
        }
    ));
    let opts = DriverOptions {
        shared: true,
        native_objects,
        opt_level: Some(2),
        ..DriverOptions::default()
    };
    driver::driver_opts(Mode::Build, &library_source, Some(&library), &opts)?;
    if let Some(godot) = &spec.godot {
        let relative = library.strip_prefix(root).map_err(|_| {
            fail("Godot output must be inside the project directory containing bindings.json")
        })?;
        let resource = format!("res://{}", relative.to_string_lossy().replace('\\', "/"));
        let platform = if cfg!(target_os = "macos") {
            "macos"
        } else {
            "linux"
        };
        let arch = if cfg!(target_arch = "aarch64") {
            "arm64"
        } else {
            "x86_64"
        };
        let quoted = serde_json::to_string(&resource).map_err(|e| fail(e.to_string()))?;
        io(std::fs::write(out.join(format!("{stem}.gdextension")), format!("[configuration]\nentry_symbol = \"{}\"\ncompatibility_minimum = \"{sdk_minimum}\"\nreloadable = false\n\n[libraries]\n{platform}.debug.{arch} = {quoted}\n{platform}.release.{arch} = {quoted}\n", godot.entry_symbol)))?;
    }
    let source_inputs = spec
        .sources
        .iter()
        .chain(spec.native_sources.iter())
        .map(|p| {
            let bytes = match spec.script_sources.get(p) {
                Some(snapshot) => snapshot.as_bytes().to_vec(),
                None => io(std::fs::read(root.join(p)))?,
            };
            Ok(serde_json::json!({"path":p, "hash":crate::interface::content_hash_bytes(&bytes)}))
        })
        .collect::<Result<Vec<_>>>()?;
    if spec.godot.is_some() {
        // Load the extension during engine initialization, before the first
        // editor scan. Dynamic first-discovery can race editor doc generation
        // on Godot 4.6. Preserve other extensions and replace the list atomically.
        let cache = root.join(".godot");
        io(std::fs::create_dir_all(&cache))?;
        let list = cache.join("extension_list.cfg");
        let mut text = if list.exists() {
            io(std::fs::read_to_string(&list))?
        } else {
            String::new()
        };
        let path = out.join(format!("{stem}.gdextension"));
        let entry = format!(
            "res://{}",
            path.strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/")
        );
        if !text.lines().any(|line| line.trim() == entry) {
            if !text.is_empty() && !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(&entry);
            text.push('\n');
            let mut staged = io(tempfile::NamedTempFile::new_in(&cache))?;
            use std::io::Write;
            io(staged.write_all(text.as_bytes()))?;
            staged.persist(&list).map_err(|e| fail(e.to_string()))?;
        }
    }
    // Record the selected SDK, target, tools and authored inputs beside the
    // artifact. This is build evidence, not an assertion of ABI verification.
    io(std::fs::write(
        out.join("native-build.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "generator":env!("CARGO_PKG_VERSION"), "bindings":spec_path, "sources":spec.sources,
            "native_sources":spec.native_sources, "inputs":source_inputs, "cxx":cxx, "cpp_defines":cpp_defines, "godot_cpp":sdk_record,
            "library":library, "target":driver::target_profile(&opts)?.triple
        }))
        .map_err(|e| fail(e.to_string()))?,
    ))?;
    eprintln!("built {}", library.display());
    Ok(library)
}

fn git_revision(path: &Path) -> Result<String> {
    let output = io(Command::new("git")
        .current_dir(path)
        .args(["rev-parse", "HEAD"])
        .output())?;
    if !output.status.success() {
        return Err(fail("could not identify the Godot SDK checkout"));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().into())
}

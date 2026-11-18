#!/usr/bin/env python3
"""Run the preserved portable targets through the compiler built at Stage3."""
from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import fcntl
import json
import os
import tempfile
from pathlib import Path
import shutil
import subprocess
import sys

import bootstrap
from bootstrap_checklist import matches_digest, published_stage3_status


def publish_interfaces(manifest, source, prefix):
    build = source.parents[2] / "build"
    build.mkdir(exist_ok=True)
    for name, _, path in bootstrap.read_json(manifest)["interfaces"]:
        shutil.copyfile(path, build / (name + ".cinchif"))
    for path in prefix.parent.glob(prefix.name + ".*.cinchif"):
        shutil.copyfile(path, build / path.name.removeprefix(prefix.name + "."))


# Target profile flags the staged compiler accepts in any position before its
# paths. The selectors in compiler/tests/common put them after the mode word.
# `--no-reclaim` is one of them: it is a word, not a path, so a caller that
# leaves it in the argument list would hand it to the compiler as a source.
PROFILE_FLAGS = ("--rc-debug", "--no-reclaim")


def compile_client(compiler, objects, runtime, args, *, ir_only=False):
    """Adapt the shared Rust test harness to the staged frontend and linker."""
    env = bootstrap.environment()
    env.update(CINCH_RC_DEBUG="0", CINCH_LEAK_CHECK="1")
    api = env.get("CINCH_TEST_API_EMIT_INTERFACES") == "1"
    emit = "--emit-ir-only" if ir_only and not api and env.get("CINCH_TEST_DISCARD_INTERFACES") == "1" else "--emit-ir"
    profile = [arg for arg in args if arg in PROFILE_FLAGS]
    args = [arg for arg in args if arg not in PROFILE_FLAGS]
    if args[0:1] == ["--interfaces"] and args[2:3] == ["--objects"] and args[4:5] == ["run"]:
        return subprocess.call([compiler, *profile, *args], env=env)
    if args[0:1] in (["--emit-ir-with-interfaces"], ["--check"], ["--check-types"]):
        return subprocess.call([compiler, *profile, *args], env=env)
    if args[0:1] == ["--emit-object"]:
        return subprocess.call([compiler, args[0], *profile, *args[1:]], env=env)
    if args[0:1] == ["--emit-ir"]:
        flags = [] if "--objects" in args else ["--objects", objects]
        return subprocess.call([compiler, emit, *profile, *flags, *args[1:]], env=env)
    if len(args) != 7 or args[:2] != ["--compile-only", "--interfaces"]:
        return 1
    manifest, source, ir, exe, report = map(Path, args[2:])
    mode = "--emit-ir-with-interfaces" if api else emit
    code = subprocess.call([compiler, mode, *profile, "--interfaces", manifest, "--objects", objects, source, ir], env=env)
    if code:
        return code
    if api:
        publish_interfaces(manifest, source, ir)
    configuration = bootstrap.read_json(Path(objects))
    link = [env.get("CINCH_CLANG", "clang"), ir, configuration["object"], runtime, "-o", exe]
    if bootstrap.HOST["system"] == "Linux":
        link.extend(["-ldl", "-lpthread", "-lm"])
    code = subprocess.call(link, env=env)
    report.write_text(f"clang-exit: {code}\n")
    build_manifest = ir.with_name(ir.name + ".manifest.json")
    if code == 0 and api and build_manifest.is_file():
        entry = bootstrap.read_json(build_manifest)["entry"]
        if not isinstance(entry, str) or not entry or Path(entry).name != entry or entry in (".", ".."):
            raise ValueError("invalid entry name in the staged build manifest")
        shutil.copyfile(build_manifest, source.parents[2] / "build" / (entry + ".manifest.json"))
    return code


def stage3_artifacts():
    stage = bootstrap.BUILD / "stage3"
    receipt = bootstrap.read_json(stage / "stage.json")
    inputs = receipt.get("inputs", {})
    provider = inputs.get("library_provider", {})
    if provider.get("kind") == "current-base-artifact":
        if bootstrap.LIBRARY_ARTIFACT is None:
            raise bootstrap.BootstrapError("this Stage3 requires its explicit --library-artifact receipt")
        library_matches = provider == bootstrap.runtime_provider()["identity"]
    else:
        library_matches = (bootstrap.LIBRARY_ARTIFACT is None
                           and provider.get("producer") == bootstrap.read_json(bootstrap.ROOT / "bootstrap/library-provider.lock.json")
                           and provider.get("roots_sha256") == bootstrap.file_hash(bootstrap.ROOT / "bootstrap/library-object-roots.json"))
    if (inputs.get("source_sha256") != bootstrap.source_hash(bootstrap.collect_compiler_sources())
            or inputs.get("built_by_stage") != 2 or inputs.get("protocol") != bootstrap.PROTOCOL
            or inputs.get("host") != bootstrap.HOST or not bootstrap.cached(stage, inputs)
            or not library_matches):
        raise bootstrap.BootstrapError("Stage3 frontend tests require the current checked compiler and library")
    compiler, manifest, objects = (stage / name for name in ("cinch", "base-interfaces.json", "objects.json"))
    configuration = bootstrap.read_json(objects)
    runtime = Path(configuration["metadata"]).with_name("libcinchruntime.a")
    hashes = {runtime.parent / name: digest for name, digest in provider["artifacts"].items()}
    hashes.update({
        bootstrap.BUILD / "stage2/cinch": inputs["builder_sha256"],
        compiler: receipt["executable_sha256"],
        manifest: inputs["interface_set_sha256"],
        objects: inputs["object_configuration_sha256"],
        Path(configuration["object"]): configuration["object_sha256"],
        Path(configuration["metadata"]): configuration["metadata_sha256"],
        runtime: inputs["runtime_archive_sha256"],
    })
    if (inputs["builder_sha256"] != receipt["executable_sha256"]
            or any(bootstrap.file_hash(path) != digest for path, digest in hashes.items())):
        raise bootstrap.BootstrapError("Stage3 frontend test artifacts changed")
    return compiler, manifest, objects, runtime, hashes


def test_target_inputs(directory, objects, triple):
    """Build only the base library for the requested target using its pinned producer."""
    from bootstrap_objects import ENTRY, admitted_interfaces, source_snapshot

    supported = {"aarch64-apple-darwin", "x86_64-apple-darwin", "x86_64-unknown-linux-gnu",
                 "aarch64-unknown-linux-gnu", "wasm32-unknown-unknown"}
    if triple not in supported:
        raise ValueError(f"unsupported target {triple!r}")
    configuration = bootstrap.read_json(Path(objects))
    pinned = bootstrap.runtime_provider()
    if (pinned["identity"].get("kind") == "current-base-artifact"
            or configuration != pinned["configuration"]):
        raise ValueError("cross compilation requires a pinned base library producer")
    native = pinned["object"].parent
    producer = native / "cinch"
    roots = native / "roots.json"
    output = Path(directory) / "target-libraries" / configuration["object_sha256"] / triple
    project = output / "library"
    project.mkdir(parents=True, exist_ok=True)
    manifest, config = output / "interfaces.json", output / "objects.json"
    with (output / ".lock").open("a") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        if not config.exists():
            source = project / "CinchBaseObject.cinch"
            source.write_bytes(ENTRY)
            code = bootstrap.run([producer, "emit-object", source, "--target", triple,
                                  "--object-roots", roots, "-o", output / "library.o"],
                                 project, output / "build.log")
            if code:
                raise ValueError(f"building {triple} base library failed: {output / 'build.log'}")
            metadata = output / "library.abi.json"
            document = bootstrap.read_json(metadata)
            if document.get("target") != triple:
                raise ValueError("base library producer emitted a different target")
            snapshot = source_snapshot(bootstrap, bootstrap.library_provider_lock())
            package = bootstrap.read_json(snapshot / "compiler/prelude/cinch-base.cinchpkg")
            bootstrap.write_json(manifest, admitted_interfaces(bootstrap, document, package, output))
            library = output / "library.o"
            bootstrap.write_json(config, dict(schema=1, target=triple, object=str(library),
                                              metadata=str(metadata), object_sha256=bootstrap.file_hash(library),
                                              metadata_sha256=bootstrap.file_hash(metadata)))
    print(json.dumps({"interfaces": str(manifest), "objects": str(config)}))
    return 0


def supports_source_mode(compiler, mode):
    # An older stage refuses the option (1). A supporting stage reaches the
    # source read (2). Probe once per runner, using no interface or source.
    with tempfile.TemporaryDirectory() as directory:
        return subprocess.run([compiler, mode, "missing.cinch", "unused.ll"],
                              cwd=directory, env=bootstrap.environment()).returncode == 2


def supports_check_types(compiler):
    return supports_source_mode(compiler, "--check-types")


def standalone_inputs(directory, compiler, manifest, objects, runtime):
    """Expose the selected artifacts to public CLI corpus runners."""
    directory.mkdir(exist_ok=True)
    for name, source in (("cinch", compiler), ("base-interfaces.json", manifest),
                         ("objects.json", objects), ("libcinchruntime.a", runtime)):
        path = directory / name
        if path.is_symlink() and path.resolve() == source.resolve(strict=True):
            continue
        path.symlink_to(source.resolve(strict=True))
    return str(directory)


def write_wrapper(wrapper, compiler, objects, runtime):
    ir_only = supports_source_mode(compiler, "--emit-ir-only")
    wrapper.write_text(
        "#!/usr/bin/env python3\nimport sys\n"
        f"sys.path.insert(0, {str(Path(__file__).resolve().parent)!r})\n"
        "from verify_stage3 import compile_client, test_target_inputs\n"
        f"compiler={str(compiler)!r}\n"
        "if len(sys.argv) == 3 and sys.argv[1] == '--test-target-inputs':\n"
        f"    raise SystemExit(test_target_inputs({str(wrapper.parent.resolve())!r}, {str(objects)!r}, sys.argv[2]))\n"
        f"raise SystemExit(compile_client(compiler, {str(objects)!r}, {str(runtime)!r}, sys.argv[1:], ir_only={ir_only!r}))\n"
    )
    wrapper.chmod(0o755)


def run_target(target, api, evidence, cargo_target, env, interface_sha, native_assertions):
    compiler = bootstrap.ROOT / "compiler"
    invocations = evidence / "invocations" / target
    log = evidence / (target + ".log")
    command = ["cargo", "test", "--locked", "--target-dir", str(cargo_target), "--test", target,
               "--", "--include-ignored", "--nocapture", "--test-threads=1"]
    fixtures = evidence / "fixtures" / target
    fixtures.mkdir()
    env = dict(env, TMPDIR=str(fixtures), CINCH_TEST_INVOCATIONS=str(invocations),
               CINCH_TEST_API_EMIT_INTERFACES=str(int(api)))
    with log.open("wb") as output:
        code = subprocess.run(command, cwd=compiler, env=env, stdout=output, stderr=subprocess.STDOUT).returncode
    receipts = {path.name: bootstrap.file_hash(path) for path in sorted(invocations.glob("*.json"))}
    row = dict(name=target, exit_code=code, status="failed" if code else "passed",
               log=str(log), log_sha256=bootstrap.file_hash(log),
               source=str(compiler / "tests" / (target + ".rs")), invocation_receipts=receipts)
    row["coverage"], row["detail"] = published_stage3_status(row, evidence, interface_sha, native_assertions)
    return row


def verify(evidence, cargo_target, jobs=1):
    if jobs < 1:
        raise ValueError("Stage3 verification needs at least one worker")
    scope_path = bootstrap.ROOT / "bootstrap/current-frontend-acceptance-scope.json"
    scope = bootstrap.read_json(scope_path)
    api = scope["interface_api_targets"]
    targets = scope["portable_targets"] + api
    required = scope["original_targets"] + scope["new_targets_beyond_original"]
    if (not targets or len(targets) != len(required) or len(set(required)) != len(required)
            or set(targets) != set(required) or len(targets) != scope["current_total_targets"]):
        raise ValueError("Stage3 test scope must cover every preserved portable target exactly once")
    native_assertions = scope.get("native_api_assertions", {})
    for target, review in native_assertions.items():
        test = bootstrap.ROOT / "compiler/tests" / (target + ".rs")
        if target not in targets or not matches_digest(test, review.get("source_sha256")):
            raise ValueError(f"Stage3 direct Rust assertions need a source review for {target}")
    compiler, manifest, objects, runtime, hashes = stage3_artifacts()
    evidence.mkdir(parents=True, exist_ok=False)
    fixtures = evidence / "fixtures"
    fixtures.mkdir()
    wrapper = evidence / "frontend.py"
    write_wrapper(wrapper, compiler, objects, runtime)
    hashes[wrapper] = bootstrap.file_hash(wrapper)
    env = {name: value for name, value in bootstrap.environment().items() if not name.startswith("CINCH_TEST_")}
    env.update(CINCH_TEST_STANDALONE=standalone_inputs(evidence / "standalone", compiler, manifest, objects, runtime),
               CINCH_TEST_CHECK_TYPES="1" if supports_check_types(compiler) else "0",
               CINCH_TEST_COMPILER=str(wrapper), CINCH_TEST_TARGET_INPUTS=str(wrapper), CINCH_TEST_INTERFACES=str(manifest),
               CINCH_TEST_OBJECTS=str(objects), CINCH_TEST_RUNTIME_ARCHIVE=str(runtime),
               CINCH_TEST_SOURCE_ROOT=str(bootstrap.ROOT / "compiler"), TMPDIR=str(fixtures), PYTHONDONTWRITEBYTECODE="1")
    report = {
        "stage": 3, "compiler_sha256": hashes[compiler], "interfaces_sha256": hashes[manifest],
        "scope_sha256": bootstrap.file_hash(scope_path), "total_targets": len(targets), "rows": [], "status": "running"}
    report_path = evidence / "results.json"
    bootstrap.write_json(report_path, report)
    completed = {}
    with ThreadPoolExecutor(max_workers=jobs) as pool:
        futures = [pool.submit(run_target, target, target in api, evidence, cargo_target, env,
                               hashes[manifest], native_assertions) for target in targets]
        for future in as_completed(futures):
            row = future.result()
            completed[row["name"]] = row
            report["rows"] = [completed[name] for name in targets if name in completed]
            bootstrap.write_json(report_path, report)
            print(f"{row['name']}: {row['coverage']} ({row['detail']})", flush=True)
    if (bootstrap.file_hash(scope_path) != report["scope_sha256"]
            or any(bootstrap.file_hash(path) != digest for path, digest in hashes.items())):
        raise bootstrap.BootstrapError("Stage3 frontend test artifacts changed during the suite")
    passed = all(row["coverage"] == "verified" for row in report["rows"])
    report["status"] = "passed" if passed else "failed"
    bootstrap.write_json(report_path, report)
    return 0 if passed else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("-j", "--jobs", type=int, default=min(4, os.cpu_count() or 1),
                        help="maximum concurrent test targets (default: up to four)")
    parser.add_argument("--evidence", type=Path, required=True)
    parser.add_argument("--cargo-target", type=Path, required=True)
    parser.add_argument("--library-artifact", type=Path,
                        help="explicit current-source base receipt used to build the selected Stage3")
    args = parser.parse_args()
    bootstrap.LIBRARY_ARTIFACT = args.library_artifact.resolve() if args.library_artifact is not None else None
    return verify(args.evidence.resolve(), args.cargo_target.resolve(), args.jobs)


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ValueError, OSError, KeyError, bootstrap.BootstrapError) as error:
        print(f"Stage3 verification: {error}", file=sys.stderr)
        raise SystemExit(2)

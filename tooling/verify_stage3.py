#!/usr/bin/env python3
"""Run the preserved portable targets through the compiler built at Stage3."""
from __future__ import annotations

import argparse
import os
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


def compile_client(compiler, objects, runtime, args):
    """Adapt the shared Rust test harness to the staged frontend and linker."""
    env = bootstrap.environment()
    env.update(CINCH_RC_DEBUG="0", CINCH_LEAK_CHECK="1")
    api = env.get("CINCH_TEST_API_EMIT_INTERFACES") == "1"
    if args[0:1] == ["--interfaces"] and args[2:3] == ["--objects"] and args[4:5] == ["run"]:
        return subprocess.call([compiler, *args], env=env)
    if args[0:1] == ["--check"] and len(args) == 5 and args[1] == "--interfaces":
        mode = "--emit-interfaces" if api else "--check"
        code = subprocess.call([compiler, mode, *args[1:]], env=env)
        if code == 0 and api:
            publish_interfaces(*(Path(value) for value in args[2:]))
        return code
    if args[0:1] == ["--emit-object"]:
        return subprocess.call([compiler, *args], env=env)
    if args[0:1] == ["--emit-ir"]:
        flags = [] if "--objects" in args else ["--objects", objects]
        return subprocess.call([compiler, args[0], *flags, *args[1:]], env=env)
    if len(args) != 7 or args[:2] != ["--compile-only", "--interfaces"]:
        return 1
    manifest, source, ir, exe, report = map(Path, args[2:])
    mode = "--emit-ir-with-interfaces" if api else "--emit-ir"
    code = subprocess.call([compiler, mode, "--interfaces", manifest, "--objects", objects, source, ir], env=env)
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
    return code


def stage3_artifacts():
    stage = bootstrap.BUILD / "stage3"
    receipt = bootstrap.read_json(stage / "stage.json")
    inputs = receipt.get("inputs", {})
    provider = inputs.get("library_provider", {})
    if (inputs.get("source_sha256") != bootstrap.source_hash(bootstrap.collect_compiler_sources())
            or inputs.get("built_by_stage") != 2 or inputs.get("protocol") != bootstrap.PROTOCOL
            or inputs.get("host") != bootstrap.HOST or not bootstrap.cached(stage, inputs)
            or provider.get("producer") != bootstrap.read_json(bootstrap.ROOT / "bootstrap/library-provider.lock.json")
            or provider.get("roots_sha256") != bootstrap.file_hash(bootstrap.ROOT / "bootstrap/library-object-roots.json")):
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


def write_wrapper(wrapper, compiler, objects, runtime):
    wrapper.write_text(
        "#!/usr/bin/env python3\nimport sys\n"
        f"sys.path.insert(0, {str(Path(__file__).resolve().parent)!r})\n"
        "from verify_stage3 import compile_client\n"
        f"compiler={str(compiler)!r}\n"
        f"raise SystemExit(compile_client(compiler, {str(objects)!r}, {str(runtime)!r}, sys.argv[1:]))\n"
    )
    wrapper.chmod(0o755)


def run_target(target, api, evidence, cargo_target, env, interface_sha, native_assertions):
    compiler = bootstrap.ROOT / "compiler"
    invocations = evidence / "invocations" / target
    log = evidence / (target + ".log")
    command = ["cargo", "test", "--locked", "--target-dir", str(cargo_target), "--test", target,
               "--", "--include-ignored", "--nocapture", "--test-threads=1"]
    env = dict(env, CINCH_TEST_INVOCATIONS=str(invocations), CINCH_TEST_API_EMIT_INTERFACES=str(int(api)))
    with log.open("wb") as output:
        code = subprocess.run(command, cwd=compiler, env=env, stdout=output, stderr=subprocess.STDOUT).returncode
    receipts = {path.name: bootstrap.file_hash(path) for path in sorted(invocations.glob("*.json"))}
    row = dict(name=target, exit_code=code, status="failed" if code else "passed",
               log=str(log), log_sha256=bootstrap.file_hash(log),
               source=str(compiler / "tests" / (target + ".rs")), invocation_receipts=receipts)
    row["coverage"], row["detail"] = published_stage3_status(row, evidence, interface_sha, native_assertions)
    return row


def verify(evidence, cargo_target):
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
    env.update(CINCH_TEST_COMPILER=str(wrapper), CINCH_TEST_INTERFACES=str(manifest),
               CINCH_TEST_OBJECTS=str(objects), CINCH_TEST_RUNTIME_ARCHIVE=str(runtime),
               CINCH_TEST_SOURCE_ROOT=str(bootstrap.ROOT / "compiler"), TMPDIR=str(fixtures), PYTHONDONTWRITEBYTECODE="1")
    report = {
        "stage": 3, "compiler_sha256": hashes[compiler], "interfaces_sha256": hashes[manifest],
        "scope_sha256": bootstrap.file_hash(scope_path), "total_targets": len(targets), "rows": [], "status": "running"}
    report_path = evidence / "results.json"
    bootstrap.write_json(report_path, report)
    for target in targets:
        row = run_target(target, target in api, evidence, cargo_target, env, hashes[manifest],
                         native_assertions)
        report["rows"].append(row)
        bootstrap.write_json(report_path, report)
        print(f"{target}: {row['coverage']} ({row['detail']})", flush=True)
    if (bootstrap.file_hash(scope_path) != report["scope_sha256"]
            or any(bootstrap.file_hash(path) != digest for path, digest in hashes.items())):
        raise bootstrap.BootstrapError("Stage3 frontend test artifacts changed during the suite")
    passed = all(row["coverage"] == "verified" for row in report["rows"])
    report["status"] = "passed" if passed else "failed"
    bootstrap.write_json(report_path, report)
    return 0 if passed else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--evidence", type=Path, required=True)
    parser.add_argument("--cargo-target", type=Path, required=True)
    args = parser.parse_args()
    return verify(args.evidence.resolve(), args.cargo_target.resolve())


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (ValueError, OSError, KeyError, bootstrap.BootstrapError) as error:
        print(f"Stage3 verification: {error}", file=sys.stderr)
        raise SystemExit(2)

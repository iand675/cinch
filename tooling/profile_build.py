#!/usr/bin/env python3
"""Profile Cinch build cost and write bootstrap/speed-baseline.json.

Three measurements, one JSON object, stable keys so later runs diff:

  chain    a clean stage ladder. Per stage: wall, the cargo part, the
           frontend part (the Cinch compiler's own run), the clang part, and
           a -ftime-trace summary of a re-link of that stage's IR.
  check    `cinch check` and `cinch emit-ir` latency on one source, median
           and p95 over N runs, plus a `sample` profile of the longest run.
  targets  tooling/run_stage_targets.py at stage 1 and stage 3. Per target:
           wall, tests passed, tests failed.

Nothing here writes the tree it reads: generated stages live in
build/bootstrap (the ladder's own cache, rewritten from clean per chain run)
and every other artifact lives in build/bootstrap-profile, which the
`/build/bootstrap*/` ignore rule already covers.

The frontend number is a residual: a stage invocation is
`wall = seed cargo builds + frontend + clang`, where the cargo part is the
`Finished in` line(s) cargo writes to the logs of that invocation and the
clang part is measured by a wrapper around CINCH_CLANG. The residual is the
builder process itself, which is the frontend time the ladder is paying for.
"""
from __future__ import annotations

import argparse
import bisect
import json
import os
import platform
import re
import shutil
import statistics
import subprocess
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import shard_ir  # noqa: E402  (the sibling module the stage link also uses)

ROOT = Path(__file__).resolve().parents[1]
TOOLING = ROOT / "tooling"
BOOTSTRAP = TOOLING / "bootstrap.py"
RUNNER = TOOLING / "run_stage_targets.py"
SCRATCH = ROOT / "build" / "bootstrap-profile"  # matched by the /build/bootstrap*/ ignore rule
DEFAULT_BUILD = ROOT / "build" / "bootstrap"
BASELINE = ROOT / "bootstrap" / "speed-baseline.json"
TOOLKIT = ("Lex", "Scan", "Parse", "Json", "Builder")

# cargo reports its own elapsed time; the stage logs carry it verbatim.
CARGO_FINISH = re.compile(r"^\s*Finished\b.*?\bin (?:(?P<m>[0-9]+)m ?)?(?P<s>[0-9.]+)s\s*$", re.M)
# `sample` indents one call-graph level per two columns, starting at four,
# and draws its group markers (`+`, `!`, `:`, `|`) in the gutter *before*
# the count: one marker per two columns of indent, so a frame four levels
# deep reads `      !   | + ! 5763 symbol`. The marker run is therefore part
# of the indent, not one optional prefix; matching only a single marker
# drops every frame deeper than the first and silently flattens the graph.
SAMPLE_LINE = re.compile(r"^(?P<indent>[ +!:|]*?)(?P<count>[0-9]+)\s+(?P<rest>.*)$")


def iso_now() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%S%z")


def host() -> dict:
    def sysctl(name: str) -> str:
        try:
            return subprocess.run(["sysctl", "-n", name], capture_output=True, text=True).stdout.strip()
        except OSError:
            return ""

    return {
        "cores": os.cpu_count(),
        "machine": platform.machine(),
        "model": sysctl("machdep.cpu.brand_string") or sysctl("hw.model"),
        "os": platform.platform(),
        "clang": clang_version(),
    }


def clang_version() -> str:
    clang = shutil.which("clang")
    if clang is None:
        return ""
    out = subprocess.run([clang, "--version"], capture_output=True, text=True).stdout
    return out.splitlines()[0] if out else ""


def cargo_seconds(text: str) -> float:
    """Sum every cargo build duration a log records. One line per build."""
    total = 0.0
    for match in CARGO_FINISH.finditer(text):
        total += 60 * int(match.group("m") or 0) + float(match.group("s"))
    return total


def git_dir() -> str:
    """The git store the pinned seed revision is extracted from.

    A jj workspace has no .git of its own; its history is the colocated
    repository's store, which `jj git root` names.
    """
    for command in (["git", "-c", "core.fsmonitor=false", "rev-parse", "--absolute-git-dir"],
                    ["jj", "git", "root"]):
        try:
            result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        except OSError:
            continue
        if result.returncode == 0 and result.stdout.strip():
            return result.stdout.strip()
    raise SystemExit("no git store for the pinned seed: git and jj both failed in " + str(ROOT))


def stage_env(**extra) -> dict:
    """The environment the ladder's own tools run in: inherited, plus GIT_DIR."""
    env = os.environ.copy()
    env["GIT_DIR"] = git_dir()
    env.update(extra)
    return env


def call(argv, *, env=None, cwd=None, log=None) -> tuple[int, float]:
    """Run one command and return its exit code and wall time."""
    argv = [str(a) for a in argv]
    started = time.perf_counter()
    if log is None:
        code = subprocess.call(argv, cwd=cwd, env=env)
    else:
        log.parent.mkdir(parents=True, exist_ok=True)
        with log.open("ab") as stream:
            code = subprocess.call(argv, cwd=cwd, env=env, stdout=stream, stderr=subprocess.STDOUT)
    return code, time.perf_counter() - started


def write_json(path: Path, value) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=1, sort_keys=False) + "\n")


def nearest_rank(values: list[float], fraction: float) -> float:
    ordered = sorted(values)
    index = max(0, min(len(ordered) - 1, int(-(-len(ordered) * fraction // 1)) - 1))
    return ordered[index]


def clang_wrapper() -> Path:
    """A CINCH_CLANG stand-in that records each clang run it forwards.

    The stage builders measure one clang per link, and the seed's build mode
    links its IR through this same variable, so every clang second a stage
    spends is attributed to that stage without re-deriving the link flags.
    """
    directory = SCRATCH / "clang"
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / "clang.py"
    real = shutil.which("clang") or "/usr/bin/clang"
    path.write_text(
        "#!/usr/bin/env python3\n"
        "import json, os, subprocess, sys, time\n"
        f"REAL = {real!r}\n"
        "started = time.perf_counter()\n"
        "code = subprocess.call([REAL] + sys.argv[1:])\n"
        "elapsed = time.perf_counter() - started\n"
        "with open(os.environ['CINCH_PROFILE_CLANG_LOG'], 'a') as log:\n"
        "    log.write(json.dumps({'s': elapsed, 'rc': code}) + '\\n')\n"
        "sys.exit(code)\n"
    )
    path.chmod(0o755)
    return path


def summarize_trace(path: Path) -> dict | None:
    """Split a clang -ftime-trace JSON into the phases this ladder cares about.

    Named phases come from the `Total <name>` roll-up entries clang appends,
    because those are the ones comparable across differently shaped modules.
    """
    try:
        events = json.loads(path.read_text()).get("traceEvents", [])
    except (OSError, json.JSONDecodeError):
        return None
    totals = {}
    for event in events:
        name, duration = event.get("name", ""), event.get("dur")
        if duration is None:
            continue
        if name.startswith("Total "):
            totals.setdefault(name[len("Total "):], duration / 1e6)
        elif name in ("ExecuteCompiler", "Frontend", "Optimizer", "CodeGenPasses"):
            totals.setdefault(name, duration / 1e6)
    summary = {phase: round(totals.get(phase, 0.0), 3)
               for phase in ("ExecuteCompiler", "Frontend", "Optimizer", "CodeGenPasses")}
    top = sorted(totals.items(), key=lambda item: item[1], reverse=True)[:5]
    summary["top_total_s"] = [[name, round(seconds, 3)] for name, seconds in top]
    return summary


def stage_ir(stage: int, build: Path) -> tuple[Path | None, float]:
    """The stage's own IR, re-emitted for stage 1 because the seed discards it."""
    ir = build / f"stage{stage}" / "compiler.ll"
    if ir.is_file():
        return ir, 0.0
    seed, bundle = build / "stage0" / "cinch", build / f"stage{stage}" / "compiler.cinch"
    if stage != 1 or not (seed.is_file() and bundle.is_file()):
        return None, 0.0
    out = SCRATCH / "ir" / "stage1.ll"
    out.parent.mkdir(parents=True, exist_ok=True)
    code, elapsed = call([seed, "emit-ir", bundle, "-o", out], env=os.environ.copy(), cwd=out.parent)
    return (out if code == 0 else None), elapsed


def relink(stage: int, build: Path, ir: Path | None) -> tuple[dict | None, float | None]:
    """Re-link the stage IR under clang's own time trace."""
    if ir is None:
        return None, None
    directory = build / f"stage{stage}"
    objects = directory / "objects.json"
    object_path = None
    if objects.is_file():
        object_path = json.loads(objects.read_text()).get("object")
    runtime = next(iter(sorted(build.glob("library-provider/*/libcinchruntime.a"))), None)
    if runtime is None:
        runtime = next(iter(sorted(build.glob("seed-sources/*/compiler/runtime/target/release/libcinchruntime.a"))), None)
    out = SCRATCH / "relink" / f"stage{stage}.out"
    out.parent.mkdir(parents=True, exist_ok=True)
    trace = out.with_name(out.name + ".trace.json")
    trace.unlink(missing_ok=True)
    command = [shutil.which("clang") or "clang", "-O2", "-Wno-override-module",
               f"-ftime-trace={trace}", str(ir)]
    if object_path:
        command.append(object_path)
    if runtime is not None:
        command.append(str(runtime))
    command += ["-o", str(out)]
    if platform.system() == "Linux":
        command += ["-ldl", "-lpthread", "-lm"]
    code, elapsed = call(command)
    if code != 0:
        return None, round(elapsed, 3)
    return summarize_trace(trace), round(elapsed, 3)


def chain(stage: int, jobs: int, build: Path, clean: bool = True) -> list[dict]:
    """Build stages 0..N from clean, one builder invocation per stage."""
    wrapper = clang_wrapper()
    calls = SCRATCH / "clang-calls.jsonl"
    calls.unlink(missing_ok=True)
    if clean and build.exists():
        shutil.rmtree(build)
    env = stage_env(CINCH_BOOTSTRAP_BUILD=str(build), CINCH_CLANG=str(wrapper),
                    CINCH_PROFILE_CLANG_LOG=str(calls), CARGO_BUILD_JOBS=str(jobs))
    rows: list[dict] = []
    consumed = 0
    for n in range(stage + 1):
        started = time.time()
        code, wall_s = call([sys.executable, str(BOOTSTRAP), "build", "--stage", str(n)],
                            env=env, cwd=ROOT, log=SCRATCH / "chain.log")
        if code:
            raise SystemExit(f"stage {n} failed (exit {code}); see build/bootstrap-profile/chain.log")
        # Every cargo build that stage ran appended a `Finished in` line to a
        # log under the build directory; nothing earlier is rewritten.
        seed_s = sum(cargo_seconds(path.read_text(errors="replace"))
                     for path in build.rglob("*.log")
                     if path.is_file() and path.stat().st_mtime >= started)
        recorded = calls.read_text().splitlines() if calls.exists() else []
        clang_s = sum(json.loads(line)["s"] for line in recorded[consumed:])
        consumed = len(recorded)
        if n == 0:
            # Stage 0 is the seed's cargo build and nothing else.
            seed_s = wall_s
        row = {"stage": n, "wall_s": round(wall_s, 3), "seed_s": round(seed_s, 3),
               "clang_s": round(clang_s, 3),
               "frontend_s": round(max(0.0, wall_s - seed_s - clang_s), 3)}
        ir, ir_s = stage_ir(n, build)
        row["ir_s"] = round(ir_s, 3) if ir_s else None
        row["clang_trace"], row["relink_s"] = relink(n, build, ir)
        rows.append(row)
        print(f"stage{n}: wall {row['wall_s']:.1f}s seed {row['seed_s']:.1f}s "
              f"frontend {row['frontend_s']:.1f}s clang {row['clang_s']:.1f}s", flush=True)
    return rows


def symbol_table(binary: Path) -> list[tuple[int, str]] | None:
    """Every defined text symbol of `binary`, ascending by address.

    `sample` leaves a frame unresolved when its stack walk starts inside a
    function it cannot name from the symbol table's own extent records: on
    this tree that was 42% of self samples, all of them real compiler code.
    The addresses it prints are slide-relative (`load address 0x104dac000 +
    0x66da58`), so subtracting the slide from the printed base recovers the
    link-time address `nm` lists.
    """
    try:
        result = subprocess.run(["nm", "-n", str(binary)], capture_output=True, text=True)
    except OSError:
        return None
    if result.returncode != 0:
        return None
    symbols = []
    for line in result.stdout.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[1] in ("t", "T"):
            try:
                symbols.append((int(parts[0], 16), parts[2]))
            except ValueError:
                continue
    symbols.sort()
    return symbols or None


def resolve_frame(symbol: str, symbols: list[tuple[int, str]] | None) -> str:
    """Name an unresolved `sample` frame from its printed load address."""
    if symbols is None or not symbol.startswith("???"):
        return symbol
    match = re.search(r"load address (0x[0-9a-f]+) \+ (0x[0-9a-f]+)", symbol)
    if match is None:
        return symbol
    # The printed pair is the image's in-memory base and the offset inside
    # it, so the offset is also the link-time address measured from the base
    # the binary was linked at (the first symbol's page, the Mach-O header).
    # `nm` lists link-time addresses, so the two agree without the slide.
    address = int(match.group(2), 16) + (symbols[0][0] & ~0xFFF)
    index = bisect.bisect_right([a for a, _ in symbols], address) - 1
    return symbols[index][1].lstrip("_") if index >= 0 else symbol


def sample_profile(argv: list[str], *, cwd: Path, seconds: int) -> dict:
    """Sample a running process, then rank self and inclusive symbols."""
    directory = SCRATCH / "sample"
    directory.mkdir(parents=True, exist_ok=True)
    report = directory / "profile.txt"
    report.unlink(missing_ok=True)
    with open(directory / "run.log", "wb") as stream:
        process = subprocess.Popen([str(a) for a in argv], cwd=cwd, stdout=stream, stderr=stream)
    # sample needs the pid to exist; the compiler starts in tens of ms.
    time.sleep(0.05)
    result = subprocess.run(["sample", str(process.pid), str(seconds), "-mayDie", "-file", str(report)],
                            capture_output=True, text=True)
    process.wait()
    if not report.is_file():
        return {"error": (result.stderr or result.stdout).strip()[:400]}
    return parse_sample(report.read_text(errors="replace"), symbols=symbol_table(Path(argv[0])))


def parse_sample(text: str, top: int = 15, symbols: list[tuple[int, str]] | None = None) -> dict:
    """Self and inclusive sample counts from `sample`'s text call graph.

    `sample` prints one line per stack position; a node's count is the number
    of samples whose stack contains that frame at that position, so a node's
    self count is its count minus its children's.
    """
    lines = text.splitlines()
    try:
        begin = lines.index("Call graph:") + 1
    except ValueError:
        return {"error": "no call graph in sample output"}
    end = len(lines)
    for index in range(begin, len(lines)):
        if lines[index].startswith(("Total number in stack", "Sort by top of stack", "Binary Images:")):
            end = index
            break
    nodes = []
    for line in lines[begin:end]:
        match = SAMPLE_LINE.match(line)
        if match is None:
            continue
        # The count's column is the depth: `sample` indents two columns per
        # level from column four, and its `+`/`!`/`:` group markers sit to the
        # left of the count instead of inside the indent.
        depth = max(0, (match.start("count") - 4) // 2)
        # The frame is the symbol name only; `sample` pads it away from the
        # `(in image) + offset [address]` suffix with two or more spaces.
        frame = match.group("rest")
        symbol = re.split(r"\s{2,}", frame)[0].strip()
        if symbol.startswith("???"):
            symbol = resolve_frame(frame, symbols)
        nodes.append({"depth": depth, "count": int(match.group("count")), "symbol": symbol,
                      "children": 0})
    if not nodes:
        return {"error": "empty call graph"}
    total = sum(node["count"] for node in nodes if node["depth"] == 0) or nodes[0]["count"]
    selfs: dict[str, int] = {}
    inclusive: dict[str, int] = {}
    stack: list[dict] = []
    for node in nodes:
        while stack and stack[-1]["depth"] >= node["depth"]:
            closed = stack.pop()
            own = closed["count"] - closed["children"]
            selfs[closed["symbol"]] = selfs.get(closed["symbol"], 0) + own
            if stack:
                stack[-1]["children"] += closed["count"]
        inclusive[node["symbol"]] = max(inclusive.get(node["symbol"], 0), node["count"])
        stack.append(node)
    while stack:
        closed = stack.pop()
        own = closed["count"] - closed["children"]
        selfs[closed["symbol"]] = selfs.get(closed["symbol"], 0) + own
        if stack:
            stack[-1]["children"] += closed["count"]

    def ranked(counts: dict[str, int]) -> list[list]:
        return [[name, round(100.0 * count / total, 1)]
                for name, count in sorted(counts.items(), key=lambda item: item[1], reverse=True)[:top]]

    return {"samples": total, "self": ranked(selfs), "inclusive": ranked(inclusive)}


def compiler_cli(compiler: Path) -> str:
    """Which command line this compiler speaks.

    The Rust seed documents `cinch check FILE`; the Cinch-built stages parse
    the ladder's positional interface (`--check SOURCE DIAGNOSTICS`) and print
    no usage of their own, so the difference shows only in the no-argument
    output.
    """
    result = subprocess.run([str(compiler)], capture_output=True, text=True)
    return "seed" if "cinch check" in (result.stdout + result.stderr) else "pipeline"


def check_commands(compiler: Path, cli: str, source: Path, ir: Path, diag: Path) -> dict:
    """The two commands to time, with the manifests a staged compiler needs.

    A staged compiler has no embedded prelude: it resolves the base family
    through the interface manifest its own build wrote beside it.
    """
    if cli == "seed":
        return {"check": [compiler, "check", source],
                "emit-ir": [compiler, "emit-ir", source, "-o", ir]}
    manifest = compiler.parent / "base-interfaces.json"
    objects = compiler.parent / "objects.json"
    if not (manifest.is_file() and objects.is_file()):
        raise SystemExit(f"{compiler} has no base-interfaces.json/objects.json beside it; "
                         "run `profile_build.py chain` first")
    return {"check": [compiler, "--check", "--interfaces", manifest, source, diag],
            "emit-ir": [compiler, "--emit-ir", "--interfaces", manifest, "--objects", objects,
                        source, ir]}


def check(source: Path, *, compiler: Path, repeat: int, tag: str) -> dict:
    """Latency of check and emit-ir on one source, then sample the slow one."""
    work = SCRATCH / "check" / tag
    work.mkdir(parents=True, exist_ok=True)
    local = work / source.name
    shutil.copyfile(source, local)
    cli = compiler_cli(compiler)
    entry = {"file": str(source), "compiler": str(compiler), "cli": cli, "repeat": repeat}
    commands = check_commands(compiler, cli, local, work / "out.ll", work / "diag")
    timings = {}
    for mode in ("check", "emit-ir"):
        runs = []
        for _ in range(repeat):
            code, elapsed = call(commands[mode], cwd=work)
            if code:
                raise SystemExit(f"{compiler} {mode} failed (exit {code}) on {source}: "
                                 f"see {work}")
            runs.append(elapsed)
        key = mode.replace("-", "_")
        timings[key] = {"runs_s": [round(run, 3) for run in runs],
                        "median_s": round(statistics.median(runs), 3),
                        "p95_s": round(nearest_rank(runs, 0.95), 3),
                        "max_s": round(max(runs), 3)}
    entry.update(timings)
    slow = "emit-ir" if timings["emit_ir"]["max_s"] >= timings["check"]["max_s"] else "check"
    profile = sample_profile(commands[slow], cwd=work, seconds=10)
    profile["mode"] = slow
    entry["sample"] = profile
    print(f"check {source.name}: check {entry['check']['median_s']}s "
          f"emit-ir {entry['emit_ir']['median_s']}s sampled {slow}", flush=True)
    return entry


def targets(listed: list[str] | None, *, jobs: int, build: Path, stages=(1, 3)) -> tuple[list[dict], list[dict]]:
    """Per-target test cost at each ladder stage, plus each stage's suite wall."""
    env = stage_env(CINCH_BOOTSTRAP_BUILD=str(build))
    rows: list[dict] = []
    suites: list[dict] = []
    for stage in stages:
        out = SCRATCH / "targets" / f"stage{stage}"
        if out.exists():
            shutil.rmtree(out)
        command = [sys.executable, str(RUNNER), "--stage", str(stage), "--out", str(out),
                   "--jobs", str(jobs), "--cargo-target", str(SCRATCH / "targets" / "cargo")]
        command += ["--all"] if listed is None else listed
        code, suite_wall = call(command, env=env, cwd=ROOT,
                                log=SCRATCH / "targets" / f"stage{stage}.log")
        summary = read_summary(out / "summary.json")
        for row in summary:
            log = out / f"{row['target']}.log"
            rows.append({"stage": stage, "target": row["target"], "exit": row["exit"],
                         "passed": row["passed"], "failed": len(row["failed"]),
                         "wall_s": log_wall(log)})
        suites.append({"stage": stage, "suite_wall_s": round(suite_wall, 3),
                       "exit": code, "targets": len(summary)})
        print(f"stage{stage} targets: {len(summary)} in {suite_wall:.1f}s", flush=True)
    return rows, suites


def read_summary(path: Path) -> list[dict]:
    if not path.is_file():
        return []
    return json.loads(path.read_text())


def log_wall(path: Path) -> float | None:
    """One target's elapsed time, from the log the runner creates and closes.

    The runner opens `<target>.log` before the run and writes through it until
    the run ends, so birth to mtime is that target's wall time, contention
    included.
    """
    try:
        info = path.stat()
    except OSError:
        return None
    birth = getattr(info, "st_birthtime", None)
    return round(info.st_mtime - birth, 3) if birth else None


def toolkit_bundle() -> Path:
    """The P30 toolkit as one file, the way the ladder bundles its sources.

    The five modules have no entry point of their own, and emit-ir needs one,
    so a `Main` that imports them closes the bundle.
    """
    out = SCRATCH / "sources" / "toolkit.cinch"
    out.parent.mkdir(parents=True, exist_ok=True)
    parts = [f"-- source: {name}\n".encode() + (ROOT / "compiler" / "prelude" / "Text" / f"{name}.cinch").read_bytes()
             for name in TOOLKIT]
    imports = "".join(f"  import Text.{name} as Bundle{name}\n" for name in TOOLKIT)
    parts.append(("module Main where\n" + imports + "  main :: Int\n  main = 0\n").encode())
    out.write_bytes(b"\n\n".join(parts) + b"\n")
    return out


def workload(name: str) -> Path:
    """One of the four shapes the shard count is measured against.

    The stage link's cost is dominated by how much independent work the
    module hands clang, and shard count trades link wall time against
    cross-shard inlining. These are the four shapes section 12 names:

    `many-small`  forty tiny modules with no imports between them, so the
                  backend has breadth and the frontend has nothing to
                  resolve.
    `wide-graph`  one root importing forty leaves, each leaf importing four
                  shared bases: breadth with a fan-in.
    `deep-chain`  forty modules importing the previous one, a pure depth
                  test with no breadth at all.
    `huge-module` one generated module of 200 top-level definitions, the
                  single-translation-unit case a shard split is meant to
                  fix. It is 200 and not 1200 because the frontend is
                  superlinear in definition count on this tree: `emit-ir`
                  over this shape takes 6.0 s at 100, 16.3 s at 200 and
                  over 300 s at 1200, so a larger instance measures the
                  frontend's scaling rather than the backend's sharding.

    Each is written under `build/bootstrap-profile/sources`, so measuring
    never touches the tree.
    """
    directory = SCRATCH / "sources" / "workloads"
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / f"{name}.cinch"
    parts: list[bytes] = []
    if name == "many-small":
        for index in range(40):
            parts.append(f"module ManySmall{index} where\n  export value{index}\n"
                         f"  value{index} :: Int\n  value{index} = {index}\n".encode())
        imports = "".join(f"  import ManySmall{index} as Small{index}\n" for index in range(40))
        # `main` reads every leaf, so the reachability sweep keeps all forty
        # bodies: a workload the emitter prunes measures nothing.
        total = " + ".join(f"Small{index}.value{index}" for index in range(40))
        parts.append(("module Main where\n" + imports +
                      f"  main :: Int\n  main = {total}\n").encode())
    elif name == "wide-graph":
        for base in range(4):
            parts.append(f"module Base{base} where\n  export base{base}\n"
                         f"  base{base} :: Int\n  base{base} = {base}\n".encode())
        for leaf in range(40):
            uses = "".join(f" + Base{b}.base{b}" for b in range(4))
            imports = "".join(f"  import Base{b} as Base{b}\n" for b in range(4))
            parts.append(f"module Leaf{leaf} where\n{imports}"
                         f"  export leaf{leaf}\n  leaf{leaf} :: Int\n"
                         f"  leaf{leaf} = {leaf}{uses}\n".encode())
        imports = "".join(f"  import Leaf{leaf} as Leaf{leaf}\n" for leaf in range(40))
        total = " + ".join(f"Leaf{leaf}.leaf{leaf}" for leaf in range(40))
        parts.append(("module Main where\n" + imports +
                      f"  main :: Int\n  main = {total}\n").encode())
    elif name == "deep-chain":
        parts.append(b"module Chain0 where\n  export chain0\n  chain0 :: Int\n  chain0 = 0\n")
        for index in range(1, 40):
            parts.append(f"module Chain{index} where\n  import Chain{index - 1} as Prior\n"
                         f"  export chain{index}\n  chain{index} :: Int\n"
                         f"  chain{index} = Prior.chain{index - 1} + 1\n".encode())
        parts.append(("module Main where\n  import Chain39 as Top\n"
                      "  main :: Int\n  main = Top.chain39\n").encode())
    elif name == "huge-module":
        # One module, no imports: every definition is reachable from `main`,
        # so this is the single-translation-unit case a shard split targets.
        # Nothing is exported: an `export` per definition routes the module
        # through the interface producer, which is a different measurement.
        body = "".join(f"  def{index} :: Int -> Int\n"
                       f"  def{index} x = x + {index}\n" for index in range(200))
        total = " + ".join(f"def{index} 1" for index in range(200))
        parts.append(f"module Main where\n{body}  main :: Int\n  main = {total}\n".encode())
    else:
        raise SystemExit(f"unknown workload {name}")
    path.write_bytes(b"\n\n".join(parts) + b"\n")
    return path


def shard_sweep(compiler: Path, *, jobs: list[int], workloads: list[str]) -> list[dict]:
    """Time `emit-ir` at each shard count over each workload.

    The shard count is a stage input, so this measures the two halves
    separately: the frontend, which `emit-ir` spends almost all of, and the
    backend, which is `clang` over the emitted module split K ways. Both are
    recorded per (workload, K) so a change to either shows as a column.
    """
    rows: list[dict] = []
    cli = compiler_cli(compiler)
    for name in workloads:
        source = workload(name)
        work = SCRATCH / "sweep" / name
        work.mkdir(parents=True, exist_ok=True)
        local = work / source.name
        shutil.copyfile(source, local)
        ir = work / "out.ll"
        commands = check_commands(compiler, cli, local, ir, work / "diag")
        code, emit_s = call(commands["emit-ir"], cwd=work)
        if code:
            rows.append({"workload": name, "error": f"emit-ir exited {code}"})
            continue
        ir_lines = sum(1 for _ in ir.open()) if ir.is_file() else 0
        for count in jobs:
            entry = {"workload": name, "jobs": count, "ir_lines": ir_lines,
                     "frontend_s": round(emit_s, 3)}
            # K=1 compiles the module as it stands; K>1 cuts it the way the
            # stage link does, so the backend number is the same work split
            # differently rather than a different program.
            if count == 1:
                shards = [ir]
            else:
                shards = shard_ir.write_shards(ir, work, count, stem=f"sweep{count}")
            objects = [work / f"sweep{count}.shard{index}.o" for index in range(len(shards))]
            clang = shutil.which("clang") or "clang"
            started = time.perf_counter()
            failure, log = shard_ir.compile_shards(shards, objects, clang, 2, count, work)
            entry["backend_s"] = round(time.perf_counter() - started, 3)
            entry["backend_ok"] = failure == 0
            if failure:
                entry["backend_log"] = log.strip().splitlines()[-1][:200] if log.strip() else ""
            rows.append(entry)
            print(f"{name} K={count}: emit {entry['frontend_s']:.1f}s "
                  f"backend {entry['backend_s']:.1f}s over {len(shards)} shards", flush=True)
    return rows


def summarize(baseline: dict) -> str:
    lines = []
    lines.append(f"baseline {baseline['date']} on {baseline['host']['model']} ({baseline['host']['cores']} cores)")
    lines.append("")
    lines.append("stage  wall_s  seed_s  frontend_s  clang_s  ExecuteCompiler_s")
    for row in baseline["chain"]:
        total = (row["clang_trace"] or {}).get("ExecuteCompiler", 0.0)
        lines.append(f"{row['stage']:5d} {row['wall_s']:7.1f} {row['seed_s']:7.1f} "
                     f"{row['frontend_s']:11.1f} {row['clang_s']:8.1f} {total:19.1f}")
    lines.append("")
    for entry in baseline["check"]:
        lines.append(f"{Path(entry['file']).name}: check median {entry['check']['median_s']}s "
                     f"p95 {entry['check']['p95_s']}s, emit-ir median {entry['emit_ir']['median_s']}s "
                     f"p95 {entry['emit_ir']['p95_s']}s")
    lines.append("")
    for suite in baseline.get("target_suites", []):
        spent = sum(row["wall_s"] or 0.0 for row in baseline["targets"] if row["stage"] == suite["stage"])
        failed = sum(row["failed"] for row in baseline["targets"] if row["stage"] == suite["stage"])
        lines.append(f"stage{suite['stage']} targets: {suite['targets']} in {suite['suite_wall_s']}s "
                     f"(summed per-target {spent:.1f}s, failed tests {failed})")
    return "\n".join(lines)


def run_all(args) -> dict:
    build = args.build.resolve()
    baseline = {"schema": 1, "date": iso_now(), "host": host(), "jobs": args.jobs,
                "chain": [], "check": [], "targets": [], "target_suites": [], "sweep": []}
    write_json(BASELINE, baseline)
    baseline["chain"] = chain(args.stage, args.jobs, build)
    write_json(BASELINE, baseline)
    sources = [toolkit_bundle(), build / "stage1" / "compiler.cinch"]
    for source in sources:
        baseline["check"].append(check(source, compiler=build / f"stage{args.stage}" / "cinch",
                                       repeat=args.repeat, tag=source.stem))
    write_json(BASELINE, baseline)
    baseline["sweep"] = shard_sweep(build / f"stage{args.stage}" / "cinch", jobs=[1, 2, 4, 8],
                                    workloads=["many-small", "wide-graph", "deep-chain", "huge-module"])
    write_json(BASELINE, baseline)
    baseline["targets"], baseline["target_suites"] = targets(None, jobs=args.jobs, build=build)
    write_json(BASELINE, baseline)
    print(summarize(baseline), flush=True)
    print(f"\nwrote {BASELINE.relative_to(ROOT)}", flush=True)
    return baseline


def main() -> int:
    parser = argparse.ArgumentParser(
        prog="profile_build.py",
        description="Measure the Cinch ladder and write bootstrap/speed-baseline.json.",
        epilog="Every subcommand leaves its artifacts under build/bootstrap-profile. "
               "The stage ladder is serial by construction (bootstrap.py pins CARGO_BUILD_JOBS=1), "
               "so --jobs only reaches the targets runner.")
    parser.add_argument("--build", type=Path, default=DEFAULT_BUILD,
                        help="stage build directory (default build/bootstrap)")
    commands = parser.add_subparsers(dest="command", required=True)

    ladder = commands.add_parser("chain", help="build stages 0..N from clean and time each")
    ladder.add_argument("--stage", type=int, default=3, choices=range(4))
    ladder.add_argument("--jobs", type=int, default=1)

    one = commands.add_parser("check", help="time check and emit-ir on one source")
    one.add_argument("source", type=Path)
    one.add_argument("--repeat", type=int, default=5)
    one.add_argument("--compiler", type=Path, default=None)
    one.add_argument("--stage", type=int, default=3, choices=range(4),
                     help="stage compiler to use when --compiler is absent")

    sweep = commands.add_parser("sweep", help="time emit-ir and the sharded backend at -j 1,2,4,8")
    sweep.add_argument("--jobs", type=int, nargs="+", default=[1, 2, 4, 8],
                       help="shard counts to sweep (default 1 2 4 8)")
    sweep.add_argument("--workloads", nargs="+", default=None,
                       help="any of many-small, wide-graph, deep-chain, huge-module")
    sweep.add_argument("--compiler", type=Path, default=None)
    sweep.add_argument("--stage", type=int, default=3, choices=range(4),
                       help="stage compiler to use when --compiler is absent")

    many = commands.add_parser("targets", help="per-target test wall time at stages 1 and 3")
    many.add_argument("targets", nargs="*")
    many.add_argument("--jobs", type=int, default=4)

    everything = commands.add_parser("all", help="chain, check, targets, then bootstrap/speed-baseline.json")
    everything.add_argument("--stage", type=int, default=3, choices=range(4))
    everything.add_argument("--jobs", type=int, default=4)
    everything.add_argument("--repeat", type=int, default=5)

    args = parser.parse_args()
    SCRATCH.mkdir(parents=True, exist_ok=True)
    build = args.build.resolve()
    if args.command == "chain":
        print(json.dumps({"schema": 1, "date": iso_now(), "host": host(),
                          "chain": chain(args.stage, args.jobs, build)}, indent=1))
    elif args.command == "check":
        compiler = args.compiler or build / f"stage{args.stage}" / "cinch"
        if not compiler.is_file():
            parser.error(f"no compiler at {compiler}; run `profile_build.py chain` first")
        print(json.dumps(check(args.source.resolve(), compiler=compiler, repeat=args.repeat,
                               tag=args.source.stem), indent=1))
    elif args.command == "sweep":
        compiler = args.compiler or build / f"stage{args.stage}" / "cinch"
        if not compiler.is_file():
            parser.error(f"no compiler at {compiler}; run `profile_build.py chain` first")
        names = args.workloads or ["many-small", "wide-graph", "deep-chain", "huge-module"]
        print(json.dumps({"sweep": shard_sweep(compiler, jobs=args.jobs, workloads=names)}, indent=1))
    elif args.command == "targets":
        rows, suites = targets(args.targets or None, jobs=args.jobs, build=build)
        print(json.dumps({"targets": rows, "target_suites": suites}, indent=1))
    else:
        run_all(args)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

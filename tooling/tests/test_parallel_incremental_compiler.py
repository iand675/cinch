import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import bootstrap


def find_stage_compiler() -> Path | None:
    for stage in ("stage2", "stage1"):
        candidate = bootstrap.BUILD / stage / "cinch"
        if candidate.is_file() and os.access(candidate, os.X_OK):
            return candidate
    return None


def find_base_provider() -> tuple[Path, dict, Path] | None:
    """Return (package_cpk, config_dict, runtime_a) if a verified library provider is cached."""
    lp_root = bootstrap.BUILD / "library-provider"
    if not lp_root.is_dir():
        return None
    try:
        p = bootstrap.runtime_provider()
        try:
            rt = bootstrap.runtime_archive()
        except Exception:
            rt = p["runtime"]
        return p["interface_package"], p["configuration"], rt
    except Exception:
        return None


def sample_program(f1_body: str = "x + 1", f2_sig: str = "Int -> Int", f2_body: str = "x * 2", comment: str = "-- v1") -> str:
    return f"""{comment}
module HelperA where
  export ha1, ha2, ha3

  ha1 :: Int -> Int
  ha1 x = {f1_body}

  ha2 :: {f2_sig}
  ha2 x = {f2_body}

  ha3 :: Int -> Int
  ha3 x = x + 3

module HelperB where
  export hb1, hb2, hb3

  hb1 :: Int -> Int
  hb1 x = x + 10

  hb2 :: Int -> Int
  hb2 x = x + 20

  hb3 :: Int -> Int
  hb3 x = x + 30

module Main where
  import HelperA (ha1, ha2, ha3)
  import HelperB (hb1, hb2, hb3)

  combine :: Int -> Int
  combine x = ha1 x + ha2 x + ha3 x + hb1 x + hb2 x + hb3 x

  main :: Int
  main = combine 4
"""


class ParallelIncrementalUnitTests(unittest.TestCase):
    def test_parallel_and_incremental_modules_in_compiler_sources(self):
        sources = bootstrap.collect_compiler_sources()
        self.assertIn("Cinch.Parallel.cinch", sources)
        self.assertIn("Cinch.Incremental.cinch", sources)
        self.assertIn("Cinch.Borrow.cinch", sources)
        self.assertIn("Cinch.Ownership.cinch", sources)
        main_src = sources["Main.cinch"].decode("utf-8")
        self.assertIn("--serve-stream", main_src)
        self.assertIn("--compile-profile", main_src)
        self.assertIn("--latency", main_src)
        self.assertIn("--throughput", main_src)
        self.assertIn("--jobs", main_src)


class ParallelIncrementalStageIntegration(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.compiler = find_stage_compiler()
        if cls.compiler is None:
            raise unittest.SkipTest("self-hosted stage1/stage2 compiler not built")

    def _run(self, args: list[str], cwd: Path) -> subprocess.CompletedProcess[str]:
        env = os.environ.copy()
        local_bin = str(Path.home() / ".local/bin")
        if local_bin not in env.get("PATH", ""):
            env["PATH"] = f"{local_bin}:{env.get('PATH', '')}"
        return subprocess.run(
            [str(self.compiler), *args],
            cwd=cwd,
            env=env,
            capture_output=True,
            text=True,
            timeout=120,
        )

    def test_parallel_worker_counts_produce_byte_identical_ir(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            src = root / "Main.cinch"
            src.write_text(sample_program())
            digests = {}
            for workers, strategy_flag in [
                ("1", "--throughput"),
                ("2", "--throughput"),
                ("4", "--throughput"),
                ("1", "--latency"),
                ("2", "--latency"),
                ("2", "--compile-profile"),
            ]:
                out = root / f"out_{workers}_{strategy_flag.lstrip('-')}.ll"
                extra = [strategy_flag, "latency"] if strategy_flag == "--compile-profile" else [strategy_flag]
                res = self._run(["-j", workers, *extra, "--emit-ir-only", str(src), str(out)], root)
                self.assertEqual(res.returncode, 0, f"failed for -j {workers} {extra}: {res.stderr}")
                digests[(workers, strategy_flag)] = hashlib.sha256(out.read_bytes()).hexdigest()
            unique = set(digests.values())
            self.assertEqual(len(unique), 1, f"IR differed across parallel/strategy settings: {digests}")

    def test_invalid_parallel_and_profile_flags_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            src = root / "Main.cinch"
            src.write_text(sample_program())
            out = root / "out.ll"
            for bad in [
                ["-j", "0", "--emit-ir-only", str(src), str(out)],
                ["-j", "abc", "--emit-ir-only", str(src), str(out)],
                ["--compile-profile", "unknown", "--emit-ir-only", str(src), str(out)],
                ["--latency", "--throughput", "--emit-ir-only", str(src), str(out)],
            ]:
                res = self._run(bad, root)
                self.assertEqual(res.returncode, 1, f"expected bad-args exit 1 for {bad}, got {res.returncode}")

    def test_serve_stream_and_serve_ir_incremental_bytecode_equivalence(self):
        provider = find_base_provider()
        if provider is None:
            self.skipTest("packed base provider not present in build/bootstrap")
        pkg, cfg_dict, runtime = provider
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            cfg = root / "object.json"
            cfg.write_text(json.dumps(cfg_dict, sort_keys=True))
            src = root / "Main.cinch"
            ir = root / "stream.ll"
            src.write_text(sample_program())

            with subprocess.Popen(
                [
                    str(self.compiler),
                    "--serve-stream",
                    "-j",
                    "2",
                    "--interfaces",
                    str(pkg),
                    "--objects",
                    str(cfg),
                    str(src),
                    str(ir),
                ],
                cwd=root,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1,
            ) as proc:
                try:
                    assert proc.stdout is not None and proc.stdin is not None
                    ready = json.loads(proc.stdout.readline())
                    self.assertEqual(ready["event"], "ready")
                    self.assertEqual(ready["strategy"], "latency")
                    self.assertEqual(ready["workers"], 2)

                    # 1. Cold full build
                    proc.stdin.write("rebuild\n")
                    proc.stdin.flush()
                    plan1 = json.loads(proc.stdout.readline())
                    done1 = json.loads(proc.stdout.readline())
                    self.assertEqual(plan1["event"], "plan")
                    self.assertEqual(plan1["kind"], "full")
                    self.assertEqual(done1["event"], "done")
                    self.assertEqual(done1["status"], 0)
                    self.assertEqual(done1["kind"], "full")
                    self.assertGreater(done1["emitted_units"], 0)
                    cold_ir = ir.read_bytes()

                    # 2. Comment-only edit -> noop plan, 0 rechecked, 0 emitted, byte-identical IR
                    src.write_text(sample_program(comment="-- v2 comment only"))
                    proc.stdin.write("rebuild\n")
                    proc.stdin.flush()
                    plan2 = json.loads(proc.stdout.readline())
                    done2 = json.loads(proc.stdout.readline())
                    self.assertEqual(plan2["kind"], "noop")
                    self.assertEqual(plan2["dirty"], 0)
                    self.assertEqual(done2["kind"], "noop")
                    self.assertEqual(done2["rechecked"], 0)
                    self.assertEqual(done2["emitted_units"], 0)
                    self.assertEqual(ir.read_bytes(), cold_ir)

                    # 3. Body-only edit to ha1 -> body-only plan, 1 rechecked, reused bodies & units
                    src.write_text(sample_program(f1_body="x + 99"))
                    proc.stdin.write("rebuild\n")
                    proc.stdin.flush()
                    plan3 = json.loads(proc.stdout.readline())
                    done3 = json.loads(proc.stdout.readline())
                    self.assertEqual(plan3["kind"], "body-only")
                    self.assertEqual(plan3["dirty"], 1)
                    self.assertGreaterEqual(plan3["reused"], 6)
                    self.assertEqual(done3["status"], 0)
                    self.assertEqual(done3["kind"], "body-only")
                    self.assertEqual(done3["rechecked"], 1)
                    self.assertGreaterEqual(done3["reused_bodies"], 6)
                    self.assertGreater(done3["reused_units"], 0)
                    incr_ir = ir.read_bytes()

                    # Must be byte-identical to a fresh cold compile of the edited source!
                    fresh_ir = root / "fresh_body_edit.ll"
                    fresh = self._run(
                        [
                            "--emit-ir-only",
                            "--interfaces",
                            str(pkg),
                            "--objects",
                            str(cfg),
                            str(src),
                            str(fresh_ir),
                        ],
                        root,
                    )
                    self.assertEqual(fresh.returncode, 0, fresh.stderr)
                    self.assertEqual(incr_ir, fresh_ir.read_bytes(), "incremental body-only IR must equal cold IR byte-for-byte")

                    # 3b. Signature change -> falls back cleanly to full rebuild
                    sig_changed = (
                        sample_program(f1_body="x + 99", f2_sig="Int -> Int -> Int")
                        .replace("  ha2 x = x * 2", "  ha2 x y = x * 2 + y")
                        .replace("ha2 x +", "ha2 x 0 +")
                    )
                    src.write_text(sig_changed)
                    proc.stdin.write("rebuild\n")
                    proc.stdin.flush()
                    plan_sig = json.loads(proc.stdout.readline())
                    done_sig = json.loads(proc.stdout.readline())
                    self.assertEqual(plan_sig["kind"], "full")
                    self.assertEqual(done_sig["status"], 0)

                    # Restore original signature and verify subsequent body-only edit works again
                    src.write_text(sample_program(f1_body="x + 99"))
                    proc.stdin.write("rebuild\n")
                    proc.stdin.flush()
                    _ = json.loads(proc.stdout.readline())
                    done_restored = json.loads(proc.stdout.readline())
                    self.assertEqual(done_restored["status"], 0)
                    self.assertEqual(ir.read_bytes(), fresh_ir.read_bytes())

                    # 4. Dynamic mode / workers / invalidate / stats commands
                    proc.stdin.write("mode throughput\n")
                    proc.stdin.flush()
                    st_tp = json.loads(proc.stdout.readline())
                    self.assertEqual(st_tp["event"], "stats")
                    self.assertEqual(st_tp["strategy"], "throughput")

                    proc.stdin.write("workers 1\n")
                    proc.stdin.flush()
                    st_w1 = json.loads(proc.stdout.readline())
                    self.assertEqual(st_w1["workers"], 1)

                    proc.stdin.write("invalidate\n")
                    proc.stdin.flush()
                    st_inv = json.loads(proc.stdout.readline())
                    self.assertEqual(st_inv["cached_bodies"], 0)
                    self.assertEqual(st_inv["cached_units"], 0)

                    # 5. Type error followed by clean recovery
                    src.write_text(sample_program(f1_body="x + True"))
                    proc.stdin.write("rebuild\n")
                    proc.stdin.flush()
                    plan_err = json.loads(proc.stdout.readline())
                    err_ev = json.loads(proc.stdout.readline())
                    self.assertEqual(plan_err["event"], "plan")
                    self.assertEqual(err_ev["event"], "error")
                    self.assertEqual(err_ev["status"], 4)

                    src.write_text(sample_program(f1_body="x + 99"))
                    proc.stdin.write("rebuild\n")
                    proc.stdin.flush()
                    plan_rec = json.loads(proc.stdout.readline())
                    done_rec = json.loads(proc.stdout.readline())
                    self.assertEqual(plan_rec["kind"], "full")
                    self.assertEqual(done_rec["status"], 0)
                    self.assertEqual(ir.read_bytes(), fresh_ir.read_bytes())

                    proc.stdin.write("quit\n")
                    proc.stdin.flush()
                    self.assertEqual(proc.wait(timeout=30), 0)
                finally:
                    if proc.poll() is None:
                        proc.kill()
                        proc.wait()

            # 6. Backward-compatible --serve-ir protocol check
            serve_ir = root / "serve.ll"
            src.write_text(sample_program())
            with subprocess.Popen(
                [
                    str(self.compiler),
                    "--serve-ir",
                    "-j",
                    "2",
                    "--interfaces",
                    str(pkg),
                    "--objects",
                    str(cfg),
                    str(src),
                    str(serve_ir),
                ],
                cwd=root,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1,
            ) as p2:
                try:
                    assert p2.stdout is not None and p2.stdin is not None
                    self.assertEqual(p2.stdout.readline(), "ready\n")
                    p2.stdin.write("\n")
                    p2.stdin.flush()
                    self.assertEqual(p2.stdout.readline(), "0\n")
                    src.write_text(sample_program(f1_body="x + 99"))
                    p2.stdin.write("\n")
                    p2.stdin.flush()
                    self.assertEqual(p2.stdout.readline(), "0\n")
                    self.assertEqual(serve_ir.read_bytes(), fresh_ir.read_bytes())
                    p2.stdin.close()
                    self.assertEqual(p2.wait(timeout=30), 0)
                finally:
                    if p2.poll() is None:
                        p2.kill()
                        p2.wait()

            # 7. Native build/run under --latency (-O0) and --throughput (-O2)
            for tag, flag in [("lat", "--latency"), ("tp", "--throughput")]:
                prefix = root / f"native_{tag}"
                res = self._run(
                    [
                        flag,
                        "-j",
                        "2",
                        "--interfaces",
                        str(pkg),
                        "--objects",
                        str(cfg),
                        "run",
                        "--clang",
                        os.environ.get("CINCH_CLANG", "clang-19"),
                        str(runtime),
                        str(prefix),
                        str(src),
                    ],
                    root,
                )
                # combine 4 with f1_body = x + 99:
                # ha1 4 = 103, ha2 4 = 8, ha3 4 = 7, hb1 4 = 14, hb2 4 = 24, hb3 4 = 34 => sum = 190
                self.assertEqual(res.returncode, 190, f"native {flag} run failed: {res.stderr}")

    def test_ownership_analyses_single_threaded_and_rc_debug(self):
        provider = find_base_provider()
        if provider is None:
            self.skipTest("cached base library provider not built")
        pkg, config_dict, runtime = provider
        clang = os.environ.get("CINCH_CLANG", "clang-19")
        base_obj = config_dict["object"]
        program = """module Main where
  data Shade where
    Light :: Shade
    Dark :: Shade
    Muted :: Shade

  data Pair where
    MkPair :: Int -> Int -> Pair

  data RB where
    Red :: Int -> Int -> RB
    Black :: Int -> Int -> RB

  data IntList where
    INil :: IntList
    ICons :: Int -> IntList -> IntList

  shade-code :: Shade -> Int
  shade-code s = case s of
    Light -> 1
    Dark -> 2
    Muted -> 3

  read-pair :: Pair -> Int
  read-pair p = case p of
    MkPair a b -> a + b

  use-pair :: Pair -> Int -> Int
  use-pair p k = case p of
    MkPair a b -> a + b + k

  apply-twice :: (Int -> Int) -> Int -> Int
  apply-twice f n = f n + f (n + 1)

  test-partial :: Int -> Int
  test-partial n =
    let p = MkPair n (n + 10) in
    let f = use-pair p in
    apply-twice f 5

  test-frame :: Int -> Int
  test-frame n =
    let p1 = MkPair n (n + 1) in
    let p2 = MkPair (n + 2) (n + 3) in
    read-pair p1 + (case p2 of MkPair a b -> a * 2 + b)

  flip-rb :: RB -> RB
  flip-rb x = case x of
    Red a b -> Black (a + 1) (b + 2)
    Black a b -> Red (a + 3) (b + 4)

  rb-sum :: RB -> Int
  rb-sum x = case x of
    Red a b -> a + b
    Black a b -> a * 2 + b

  insert-sorted :: Int -> IntList -> IntList
  insert-sorted x xs = case xs of
    INil -> ICons x INil
    ICons h t ->
      if x < h then ICons x xs
      else ICons h (insert-sorted x t)

  filter-inc :: IntList -> IntList
  filter-inc xs = case xs of
    INil -> INil
    ICons h t ->
      let next = filter-inc t in
      if h <= 5 then ICons (h + 10) next else next

  mk-list :: Int -> IntList -> IntList
  mk-list n acc =
    if n <= 0 then acc
    else mk-list (n - 1) (ICons n acc)

  sum-list :: IntList -> Int -> Int
  sum-list xs acc = case xs of
    INil -> acc
    ICons h t -> sum-list t (acc + h)

  scratch-phase :: Int -> Int
  scratch-phase n =
    let xs = mk-list n INil in
    let ys = filter-inc xs in
    sum-list ys 0

  main :: Int
  main =
    let u = flip-rb (Red 7 8) in
    let sh = Black 7 12 in
    let s1 = flip-rb sh in
    let s2 = flip-rb sh in
    let sorted = insert-sorted 3 (insert-sorted 1 (insert-sorted 4 (insert-sorted 2 INil))) in
    let total = shade-code Light + shade-code Dark + shade-code Muted
              + test-partial 7 + test-frame 10
              + rb-sum u + rb-sum s1 + rb-sum s2
              + sum-list sorted 0 + scratch-phase 200 in
    if total == 276 then 0 else total
"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            cfg = root / "objects.json"
            cfg.write_text(json.dumps(config_dict))
            src = root / "Main.cinch"
            src.write_text(program)
            for rc_debug in (False, True):
                ll = root / ("st_dbg.ll" if rc_debug else "st_opt.ll")
                exe = root / ("st_dbg" if rc_debug else "st_opt")
                args = ["--emit-ir"]
                if rc_debug:
                    args.append("--rc-debug")
                args.extend(["--interfaces", str(pkg), "--objects", str(cfg), str(src), str(ll)])
                r = self._run(args, root)
                self.assertEqual(r.returncode, 0, f"emit failed (rc_debug={rc_debug}): {r.stderr}")
                ir = ll.read_text()
                self.assertGreater(ir.count('#b"('), 0)
                self.assertGreater(ir.count("resetuniq"), 0)
                self.assertGreater(ir.count("reusehit"), 0)
                self.assertGreater(ir.count("%fslot"), 0)
                self.assertGreater(ir.count("call i64 @cinch_arena_enter_phase("), 0)
                self.assertEqual(ir.count("load atomic i64"), 0)
                subprocess.run(
                    [clang, "-O0" if rc_debug else "-O2", str(ll), base_obj, str(runtime),
                     "-o", str(exe), "-ldl", "-lpthread", "-lm"],
                    check=True,
                )
                run_env = os.environ.copy()
                run_env["CINCH_LEAK_CHECK"] = "1"
                run_env["CINCH_ALLOC_STATS"] = "1"
                if rc_debug:
                    run_env["CINCH_RC_DEBUG"] = "1"
                rr = subprocess.run([str(exe)], env=run_env, capture_output=True, text=True, timeout=30)
                self.assertEqual(rr.returncode, 0, f"st run failed (rc_debug={rc_debug}): {rr.stderr}")
                self.assertIn("live=0 immortal=0 leaked=0", rr.stderr)
                self.assertIn("arena-blocks=", rr.stderr)

    def test_ownership_analyses_concurrent_par_both(self):
        provider = find_base_provider()
        if provider is None:
            self.skipTest("cached base library provider not built")
        pkg, config_dict, runtime = provider
        clang = os.environ.get("CINCH_CLANG", "clang-19")
        base_obj = config_dict["object"]
        program = """module Main where
  data Pair where
    MkPair :: Int -> Int -> Pair

  data IntList where
    INil :: IntList
    ICons :: Int -> IntList -> IntList

  read-pair :: Pair -> Int
  read-pair p = case p of
    MkPair a b -> a + b

  mk-list :: Int -> IntList -> IntList
  mk-list n acc =
    if n <= 0 then acc
    else mk-list (n - 1) (ICons n acc)

  sum-list :: IntList -> Int -> Int
  sum-list xs acc = case xs of
    INil -> acc
    ICons h t -> sum-list t (acc + h)

  scratch-phase :: Int -> Int
  scratch-phase n =
    let p = MkPair n (n + 1) in
    let xs = mk-list n INil in
    read-pair p + sum-list xs 0

  main :: Int
  main =
    case parBoth (\\() -> scratch-phase 150) (\\() -> scratch-phase 250) of
      (a, b) -> if a + b == 43502 then 0 else a + b
"""
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            cfg = root / "objects.json"
            cfg.write_text(json.dumps(config_dict))
            src = root / "Main.cinch"
            src.write_text(program)
            for rc_debug in (False, True):
                ll = root / ("mt_dbg.ll" if rc_debug else "mt_opt.ll")
                exe = root / ("mt_dbg" if rc_debug else "mt_opt")
                args = ["--emit-ir"]
                if rc_debug:
                    args.append("--rc-debug")
                args.extend(["--interfaces", str(pkg), "--objects", str(cfg), str(src), str(ll)])
                r = self._run(args, root)
                self.assertEqual(r.returncode, 0, f"MT emit failed (rc_debug={rc_debug}): {r.stderr}")
                ir = ll.read_text()
                self.assertGreater(ir.count("load atomic i64"), 0)
                self.assertGreater(ir.count("call i64 @cinch_arena_enter_phase("), 0)
                self.assertGreater(ir.count("%fslot"), 0)
                subprocess.run(
                    [clang, "-O0" if rc_debug else "-O2", str(ll), base_obj, str(runtime),
                     "-o", str(exe), "-ldl", "-lpthread", "-lm"],
                    check=True,
                )
                run_env = os.environ.copy()
                run_env["CINCH_LEAK_CHECK"] = "1"
                run_env["CINCH_ALLOC_STATS"] = "1"
                if rc_debug:
                    run_env["CINCH_RC_DEBUG"] = "1"
                rr = subprocess.run([str(exe)], env=run_env, capture_output=True, text=True, timeout=30)
                self.assertEqual(rr.returncode, 0, f"MT run failed (rc_debug={rc_debug}): {rr.stderr}")
                self.assertIn("live=0 immortal=0 leaked=0", rr.stderr)
                self.assertIn("arena-blocks=", rr.stderr)


if __name__ == "__main__":
    unittest.main()

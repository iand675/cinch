from __future__ import annotations

import csv
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import bootstrap


def find_stage_compiler() -> Path | None:
    for stage in ("stage3", "stage2", "stage1"):
        candidate = bootstrap.BUILD / stage / "cinch"
        if candidate.is_file() and os.access(candidate, os.X_OK):
            return candidate
    return None


def find_base_provider() -> tuple[Path, dict, Path] | None:
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


def clang_bin() -> str:
    return os.environ.get("CINCH_CLANG") or shutil.which("clang-19") or "clang"


class BuiltinBenchRunnerUnitTests(unittest.TestCase):
    def test_runner_module_and_cli_flags_present(self):
        sources = bootstrap.collect_compiler_sources()
        self.assertIn("Cinch.BenchRunner.cinch", sources)
        self.assertIn("Effect.Benchmark.cinch", sources)
        main_src = sources["Main.cinch"].decode("utf-8")
        self.assertIn('"bench"', main_src)


class BuiltinBenchRunnerIntegrationTests(unittest.TestCase):
    def setUp(self):
        self.compiler = find_stage_compiler()
        self.provider = find_base_provider()
        if self.compiler is None or self.provider is None:
            self.skipTest("built stage compiler or cached library-provider not available")
        self.pkg, self.cfg, self.runtime = self.provider

    def _run_cinch(
        self, td: str, extra_flags: list[str], subcmd_args: list[str], cwd: Path | None = None
    ) -> subprocess.CompletedProcess:
        obj_cfg = Path(td) / "objects.json"
        obj_cfg.write_text(json.dumps(self.cfg), encoding="utf-8")
        cmd = [
            str(self.compiler),
            "--interfaces", str(self.pkg),
            "--objects", str(obj_cfg),
            *extra_flags,
            *subcmd_args,
        ]
        return subprocess.run(
            cmd, capture_output=True, text=True, timeout=180, cwd=str(cwd or bootstrap.ROOT)
        )

    def test_help_flag(self):
        with tempfile.TemporaryDirectory() as td:
            proc = self._run_cinch(td, [], ["bench", "--help"])
            self.assertEqual(proc.returncode, 0, proc.stdout + proc.stderr)
            self.assertIn("cinch bench", proc.stdout)
            self.assertIn("--save-baseline", proc.stdout)
            self.assertIn("--html-report", proc.stdout)

    def test_direct_bencher_criterion_group_reports_and_leak_check(self):
        src = """module FibBench where
  import Data.Int
  import Data.Nat
  import Data.List
  import Effect.Benchmark

  export fib-iter

  fib-iter-go :: Nat -> Int -> Int -> Int
  fib-iter-go n a b = if n == 0 then a else fib-iter-go (saturating-sub n 1) b (a + b)

  fib-iter :: Nat -> Int
  fib-iter n = fib-iter-go n 0 1

  #[bench_only]
  bench-input-n :: Nat
  bench-input-n = 25

  #[bench(group = "fib", tag = "fast", throughput_elements = 25)]
  bench-direct :: Unit -> Int
  bench-direct () = fib-iter (black-box bench-input-n)

  #[bench(group = "fib", tag = "bencher")]
  bench-with-bencher :: forall r. using (io :: Io r) => Bencher ->{IO r} BenchOutcome
  bench-with-bencher b =
    let n = bench-input-n in
    bench-iter-pure (bencher-with-throughput (ThroughputBytes 64) b) (\\() -> fib-iter (black-box n))

  #[bench(tag = "batched")]
  bench-batched :: forall r. using (io :: Io r) => Bencher ->{IO r} BenchOutcome
  bench-batched b =
    bench-iter-batched-pure b SmallInput (\\() -> bench-input-n) (\\n -> fib-iter n)

  #[bench(tag = "suite")]
  bench-criterion-group :: forall r. using (io :: Io r) => Criterion ->{IO r} List BenchOutcome
  bench-criterion-group c =
    let g = group-with-throughput (ThroughputElements 1) (benchmark-group c "scaling") in
    group-bench-inputs g "fib" [("10", 10), ("20", 20)] (\\b n ->
      bench-iter-pure b (\\() -> fib-iter (black-box n)))

  #[bench]
  #[ignore("slow benchmark")]
  bench-ignored :: Unit -> Int
  bench-ignored () = fib-iter 40
"""
        with tempfile.TemporaryDirectory() as td:
            src_path = Path(td) / "FibBench.cinch"
            src_path.write_text(src, encoding="utf-8")
            prefix = str(Path(td) / "fib_bench")
            json_out = str(Path(td) / "report.json")
            csv_out = str(Path(td) / "report.csv")
            html_out = str(Path(td) / "report.html")

            # --list-json
            p_list = self._run_cinch(
                td, [], ["bench", "--list-json", str(self.runtime), prefix, str(src_path)]
            )
            self.assertEqual(p_list.returncode, 0, p_list.stdout + p_list.stderr)
            doc = json.loads(p_list.stdout)
            self.assertEqual(doc["count"], 5)
            by_name = {b["name"]: b for b in doc["benchmarks"]}
            self.assertEqual(by_name["FibBench.bench-direct"]["kind"], "direct")
            self.assertEqual(by_name["FibBench.bench-direct"]["group"], "fib")
            self.assertEqual(by_name["FibBench.bench-with-bencher"]["kind"], "bencher")
            self.assertEqual(by_name["FibBench.bench-criterion-group"]["kind"], "criterion-group")
            self.assertTrue(by_name["FibBench.bench-ignored"]["ignored"])

            # Filter expression -E
            p_filt = self._run_cinch(
                td, [],
                ["bench", "--list", "-E", "group(fib) and not tag(bencher)", str(self.runtime), prefix, str(src_path)],
            )
            self.assertEqual(p_filt.returncode, 0, p_filt.stdout + p_filt.stderr)
            self.assertIn("FibBench.bench-direct", p_filt.stdout)
            self.assertNotIn("FibBench.bench-with-bencher", p_filt.stdout)

            # Run quick benchmark suite with JSON/CSV/HTML reports + leak check!
            p_run = self._run_cinch(
                td, [],
                [
                    "bench", "--clang", clang_bin(), "--quick",
                    "--warm-up-time", "30", "--measurement-time", "80", "--sample-size", "12",
                    "--nresamples", "80", "--leak-check",
                    "--json-report", json_out, "--csv-report", csv_out, "--html-report", html_out,
                    str(self.runtime), prefix, str(src_path),
                ],
            )
            self.assertEqual(p_run.returncode, 0, f"stdout:\n{p_run.stdout}\nstderr:\n{p_run.stderr}")
            self.assertIn("5 benchmarks measured", p_run.stdout)
            self.assertIn("1 skipped", p_run.stdout)
            self.assertIn("fib/FibBench.bench-direct", p_run.stdout)
            self.assertIn("scaling/fib/10", p_run.stdout)
            self.assertIn("scaling/fib/20", p_run.stdout)
            self.assertIn("thrpt:", p_run.stdout)
            self.assertIn("R^2", p_run.stdout)

            # Verify JSON report structure
            jdoc = json.loads(Path(json_out).read_text(encoding="utf-8"))
            self.assertEqual(jdoc["schema"], 1)
            self.assertEqual(jdoc["summary"]["benchmarks"], 5)
            self.assertEqual(jdoc["summary"]["skipped"], 1)
            names = [b["name"] for b in jdoc["benchmarks"]]
            self.assertIn("fib/FibBench.bench-direct", names)
            self.assertIn("scaling/fib/10", names)
            self.assertIn("scaling/fib/20", names)
            for b in jdoc["benchmarks"]:
                self.assertGreaterEqual(b["samples"], 12)
                self.assertGreater(b["total_iterations"], 0)
                self.assertLessEqual(b["median"]["lower_ns"], b["median"]["upper_ns"])

            # Verify CSV report
            rows = list(csv.DictReader(io.StringIO(Path(csv_out).read_text(encoding="utf-8"))))
            self.assertEqual(len(rows), 5)
            self.assertIn("median_ns", rows[0])
            self.assertIn("r_squared", rows[0])

            # Verify HTML report with SVG chart
            html = Path(html_out).read_text(encoding="utf-8")
            self.assertIn("Cinch Criterion Benchmark Report", html)
            self.assertIn("<svg", html)
            self.assertIn("scaling/fib/20", html)

    def test_baseline_save_compare_and_fail_on_regression(self):
        fast_src = """module RegBench where
  import Data.Nat

  spin :: Nat -> Nat -> Nat
  spin n acc = if n == 0 then acc else spin (saturating-sub n 1) (acc + 1)

  #[bench]
  bench-work :: Unit -> Nat
  bench-work () = spin 2000 0
"""
        slow_src = """module RegBench where
  import Data.Nat

  spin :: Nat -> Nat -> Nat
  spin n acc = if n == 0 then acc else spin (saturating-sub n 1) (acc + 1)

  #[bench]
  bench-work :: Unit -> Nat
  bench-work () = spin 60000 0
"""
        with tempfile.TemporaryDirectory() as td:
            bdir = str(Path(td) / "baselines")
            src_path = Path(td) / "RegBench.cinch"
            prefix = str(Path(td) / "reg_bench")

            # 1. Save fast baseline
            src_path.write_text(fast_src, encoding="utf-8")
            p_save = self._run_cinch(
                td, [],
                [
                    "bench", "--clang", clang_bin(), "--quick",
                    "--warm-up-time", "20", "--measurement-time", "60", "--sample-size", "15",
                    "--nresamples", "80",
                    "--baseline-dir", bdir, "--save-baseline", "main",
                    str(self.runtime), prefix, str(src_path),
                ],
            )
            self.assertEqual(p_save.returncode, 0, p_save.stdout + p_save.stderr)
            self.assertTrue((Path(bdir) / "main.json").is_file())

            # 2. Compare deliberate 30x regression with --fail-on-regression -> exit code 3
            src_path.write_text(slow_src, encoding="utf-8")
            p_reg = self._run_cinch(
                td, [],
                [
                    "bench", "--clang", clang_bin(), "--quick",
                    "--warm-up-time", "20", "--measurement-time", "60", "--sample-size", "15",
                    "--nresamples", "80",
                    "--baseline-dir", bdir, "--baseline", "main", "--fail-on-regression",
                    str(self.runtime), prefix, str(src_path),
                ],
            )
            self.assertEqual(p_reg.returncode, 3, f"expected exit 3 on regression:\n{p_reg.stdout}\n{p_reg.stderr}")
            self.assertIn("Performance has regressed", p_reg.stdout)

    def test_release_elimination_and_bench_boundary(self):
        release_src = """module Prod where
  import Data.Int

  #[bench_only]
  secret-bench-marker :: String
  secret-bench-marker = "BENCH_ONLY_SECRET_SHOULD_NOT_SURVIVE"

  #[cfg(bench)]
  bench-helper :: Int -> Int
  bench-helper x = x + 1

  #[bench]
  bench-add :: Unit -> Int
  bench-add () = bench-helper 41

  main :: Int
  main = 0
"""
        boundary_src = """module BadProd where
  import Data.Int

  #[bench_only]
  helper :: Int
  helper = 99

  main :: Int
  main = helper
"""
        with tempfile.TemporaryDirectory() as td:
            # Release build strips #[bench], #[bench_only], #[cfg(bench)]
            r_path = Path(td) / "Prod.cinch"
            r_path.write_text(release_src, encoding="utf-8")
            ir_path = Path(td) / "prod.ll"
            p_rel = self._run_cinch(
                td, ["--release", "--emit-ir-only"], [str(r_path), str(ir_path)]
            )
            self.assertEqual(p_rel.returncode, 0, p_rel.stdout + p_rel.stderr)
            ir_text = ir_path.read_text(encoding="utf-8")
            self.assertNotIn("BENCH_ONLY_SECRET_SHOULD_NOT_SURVIVE", ir_text)
            self.assertNotIn("bench-helper", ir_text)
            self.assertNotIn("bench-add", ir_text)

            # Production code calling #[bench_only] is refused
            b_path = Path(td) / "BadProd.cinch"
            b_path.write_text(boundary_src, encoding="utf-8")
            bad_prefix = str(Path(td) / "bad_build")
            p_bad = self._run_cinch(
                td, [], ["build", "--clang", clang_bin(), str(self.runtime), bad_prefix, str(b_path)]
            )
            self.assertNotEqual(p_bad.returncode, 0, "expected bench boundary refusal")
            diag_file = Path(bad_prefix + ".ll.diagnostics")
            diag_text = diag_file.read_text(encoding="utf-8") if diag_file.is_file() else (p_bad.stdout + p_bad.stderr)
            self.assertIn("bench-only", diag_text)


if __name__ == "__main__":
    unittest.main()

import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest
import xml.etree.ElementTree as ET

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
    return os.environ.get("CINCH_CLANG", "clang-19")


class BuiltinTestRunnerUnitTests(unittest.TestCase):
    def test_runner_module_and_cli_flags_present(self):
        sources = bootstrap.collect_compiler_sources()
        self.assertIn("Cinch.TestRunner.cinch", sources)
        main_src = sources["Main.cinch"].decode("utf-8")
        self.assertIn('FlagSpec "release"', main_src)
        self.assertIn('"test"', main_src)


class BuiltinTestRunnerIntegrationTests(unittest.TestCase):
    def setUp(self):
        self.compiler = find_stage_compiler()
        self.provider = find_base_provider()
        if self.compiler is None or self.provider is None:
            self.skipTest("built stage compiler or cached library-provider not available")
        self.pkg, self.cfg, self.runtime = self.provider

    def _run_cinch(self, td: str, extra_flags: list[str], subcmd_args: list[str]) -> subprocess.CompletedProcess:
        obj_cfg = Path(td) / "objects.json"
        obj_cfg.write_text(json.dumps(self.cfg), encoding="utf-8")
        cmd = [
            str(self.compiler),
            "--interfaces", str(self.pkg),
            "--objects", str(obj_cfg),
            *extra_flags,
            *subcmd_args,
        ]
        return subprocess.run(cmd, capture_output=True, text=True, timeout=180)

    def test_all_return_types_private_tests_io_and_leak_check(self):
        effect_test_src = (bootstrap.ROOT / "seed" / "prelude" / "Effect" / "Test.cinch").read_text(encoding="utf-8")
        src = effect_test_src + """

module Math where
  import Data.Int
  import Data.Bool
  import Data.Result
  import Data.String
  import Effect.Test (Check, assert, assert-eq)

  export add

  add :: Int -> Int -> Int
  add x y = x + y

  #[test_only]
  expected-sum :: Int
  expected-sum = 42

  #[test]
  test-int :: Unit -> Int
  test-int () = if add 20 22 == expected-sum then 0 else 1

  #[test]
  #[tag("fast")]
  test-bool :: Unit -> Bool
  test-bool () = add 2 3 == 5

  #[test(tag = "fast")]
  test-unit :: Unit -> Unit
  test-unit () = ()

  #[test]
  test-result :: Unit -> Result String Unit
  test-result () = if add 1 1 == 2 then Ok () else Err "bad math"

  #[test]
  test-check :: Unit -> Check
  test-check () = assert-eq "sum" (add 10 5) 15

  #[test]
  test-check-list :: Unit -> List Check
  test-check-list () = [assert "pos" (add 1 2 == 3), assert-eq "eq" (add 4 5) 9]

  #[test(tag = "io")]
  test-io :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  test-io () = case ioWrite (string-to-bytes "hello from io test\\n") of
    _ -> 0
"""
        with tempfile.TemporaryDirectory() as td:
            src_path = Path(td) / "Math.cinch"
            src_path.write_text(src, encoding="utf-8")
            prefix = str(Path(td) / "math_suite")
            proc = self._run_cinch(
                td,
                [],
                ["test", "--clang", clang_bin(), "-j", "4", "--leak-check", str(self.runtime), prefix, str(src_path)],
            )
            self.assertEqual(proc.returncode, 0, f"stdout:\n{proc.stdout}\nstderr:\n{proc.stderr}")
            self.assertIn("7 tests run: 7 passed, 0 failed", proc.stdout)
            for name in ("Math.test-int", "Math.test-bool", "Math.test-unit", "Math.test-result",
                         "Math.test-check", "Math.test-check-list", "Math.test-io"):
                self.assertIn(name, proc.stdout)

    def test_list_filter_expressions_ignored_and_partitions(self):
        src = """module Alpha where
  import Data.Bool

  #[test(tag = "fast")]
  t-one :: Unit -> Bool
  t-one () = True

  #[test(tag = "slow", serial)]
  t-two :: Unit -> Bool
  t-two () = True

  #[test]
  #[ignore="waiting on upstream"]
  t-ignored :: Unit -> Bool
  t-ignored () = True

module Beta where
  import Data.Bool

  #[test(tag = "fast")]
  t-three :: Unit -> Bool
  t-three () = True

  #[test(should_fail)]
  t-four :: Unit -> Unit
  t-four () = ()
"""
        with tempfile.TemporaryDirectory() as td:
            src_path = Path(td) / "Suite.cinch"
            src_path.write_text(src, encoding="utf-8")
            prefix = str(Path(td) / "suite")

            # --list-json
            p_json = self._run_cinch(td, [], ["test", "--list-json", str(self.runtime), prefix, str(src_path)])
            self.assertEqual(p_json.returncode, 0, p_json.stdout + p_json.stderr)
            doc = json.loads(p_json.stdout)
            self.assertEqual(doc["count"], 5)
            by_name = {t["name"]: t for t in doc["tests"]}
            self.assertTrue(by_name["Alpha.t-ignored"]["ignored"])
            self.assertEqual(by_name["Alpha.t-ignored"]["ignore_reason"], "waiting on upstream")
            self.assertTrue(by_name["Alpha.t-two"]["serial"])
            self.assertEqual(by_name["Alpha.t-two"]["tags"], ["slow"])
            self.assertTrue(by_name["Beta.t-four"]["should_fail"])

            # Filter expression: module(Alpha) and not tag(slow) and not ignored()
            p_filt = self._run_cinch(
                td, [],
                ["test", "--list", "-E", "module(Alpha) and not tag(slow) and not ignored()", str(self.runtime), prefix, str(src_path)],
            )
            self.assertEqual(p_filt.returncode, 0, p_filt.stdout + p_filt.stderr)
            self.assertIn("Alpha.t-one", p_filt.stdout)
            self.assertNotIn("Alpha.t-two", p_filt.stdout)
            self.assertNotIn("Alpha.t-ignored", p_filt.stdout)
            self.assertNotIn("Beta.t-three", p_filt.stdout)

            # Set difference: all() - tag(fast)
            p_diff = self._run_cinch(
                td, [],
                ["test", "--list", "-E", "all() - tag(fast)", str(self.runtime), prefix, str(src_path)],
            )
            self.assertEqual(p_diff.returncode, 0, p_diff.stdout + p_diff.stderr)
            self.assertNotIn("Alpha.t-one", p_diff.stdout)
            self.assertNotIn("Beta.t-three", p_diff.stdout)
            self.assertIn("Alpha.t-two", p_diff.stdout)
            self.assertIn("Beta.t-four", p_diff.stdout)

            # Partitions count:1/2 and count:2/2 are disjoint and cover all 5 tests
            p1 = self._run_cinch(td, [], ["test", "--list-json", "--partition", "count:1/2", str(self.runtime), prefix, str(src_path)])
            p2 = self._run_cinch(td, [], ["test", "--list-json", "--partition", "count:2/2", str(self.runtime), prefix, str(src_path)])
            self.assertEqual(p1.returncode, 0)
            self.assertEqual(p2.returncode, 0)
            s1 = {t["name"] for t in json.loads(p1.stdout)["tests"]}
            s2 = {t["name"] for t in json.loads(p2.stdout)["tests"]}
            self.assertEqual(s1 & s2, set())
            self.assertEqual(s1 | s2, set(by_name.keys()))

            # --run-ignored only runs only Alpha.t-ignored
            p_ign = self._run_cinch(
                td, [],
                ["test", "--clang", clang_bin(), "--run-ignored", "only", str(self.runtime), prefix, str(src_path)],
            )
            self.assertEqual(p_ign.returncode, 0, p_ign.stdout + p_ign.stderr)
            self.assertIn("1 tests run: 1 passed, 0 failed", p_ign.stdout)
            self.assertIn("Alpha.t-ignored", p_ign.stdout)

    def test_isolation_should_fail_failures_junit_and_json_reports(self):
        effect_test_src = (bootstrap.ROOT / "seed" / "prelude" / "Effect" / "Test.cinch").read_text(encoding="utf-8")
        src = effect_test_src + """

module Isolation where
  import Data.Result
  import Data.String (string-to-bytes)
  import Effect.Test (Check, assert-eq)

  #[test]
  test-ok :: Unit -> Bool
  test-ok () = True

  #[test]
  test-fail-result :: Unit -> Result String Unit
  test-fail-result () = Err "boom-result-reason"

  #[test]
  test-fail-check :: Unit -> Check
  test-fail-check () = assert-eq "arith" 3 (1 + 1)

  #[test(should_fail = "expected-failure-marker")]
  test-expected-failure :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  test-expected-failure () = case ioWrite (string-to-bytes "expected-failure-marker\\n") of
    _ -> 1

  #[test(should_fail)]
  test-unexpected-success :: Unit -> Int
  test-unexpected-success () = 0

  #[test]
  #[ignore("deferred")]
  test-skipped :: Unit -> Bool
  test-skipped () = False
"""
        with tempfile.TemporaryDirectory() as td:
            src_path = Path(td) / "Isolation.cinch"
            src_path.write_text(src, encoding="utf-8")
            prefix = str(Path(td) / "iso")
            junit_path = Path(td) / "junit.xml"
            json_path = Path(td) / "report.json"

            proc = self._run_cinch(
                td, [],
                [
                    "test", "--clang", clang_bin(), "-j", "3",
                    "--junit", str(junit_path),
                    "--json-report", str(json_path),
                    str(self.runtime), prefix, str(src_path),
                ],
            )
            self.assertEqual(proc.returncode, 1, proc.stdout + proc.stderr)
            self.assertIn("5 tests run: 2 passed, 3 failed, 1 skipped", proc.stdout)
            self.assertIn("boom-result-reason", proc.stdout)
            self.assertIn("FAIL [arith]: expected 3, got 2", proc.stdout)
            self.assertIn("expected failure, but test exited 0", proc.stdout)

            # Validate JUnit XML
            root = ET.fromstring(junit_path.read_text(encoding="utf-8"))
            self.assertEqual(root.attrib["tests"], "6")
            self.assertEqual(root.attrib["failures"], "3")
            self.assertEqual(root.attrib["skipped"], "1")

            # Validate JSON report
            rep = json.loads(json_path.read_text(encoding="utf-8"))
            self.assertEqual(rep["summary"]["run"], 5)
            self.assertEqual(rep["summary"]["passed"], 2)
            self.assertEqual(rep["summary"]["failed"], 3)
            self.assertEqual(rep["summary"]["skipped"], 1)

    def test_timeout_and_flaky_retries(self):
        with tempfile.TemporaryDirectory() as td:
            counter_file = Path(td) / "attempts.txt"
            src = f"""module Timing where
  import Data.Path (path-from-string)
  import Data.String (string-to-bytes, string-from-bytes, string-compare)
  import Effect.Fs (fs-read, fs-write)

  spin :: Int ->{{Diverge}} Int
  spin n = spin (n + 1)

  #[test(timeout = 150)]
  test-hang :: Unit ->{{Diverge}} Int
  test-hang () = spin 0

  #[test(retries = 2)]
  test-flaky :: forall r. using (io :: Io r) => Unit ->{{IO r}} Int
  test-flaky () =
    let p = path-from-string "{counter_file}" in
    case fs-read p of
      Err _ -> case fs-write p (string-to-bytes "1") of _ -> 1
      Ok b -> 0
"""
            src_path = Path(td) / "Timing.cinch"
            src_path.write_text(src, encoding="utf-8")
            prefix = str(Path(td) / "timing")

            # 1) Test flaky retry passes on attempt 2/3 -> exit 0!
            p_flaky = self._run_cinch(
                td, [],
                ["test", "--clang", clang_bin(), str(self.runtime), prefix, str(src_path), "test-flaky"],
            )
            self.assertEqual(p_flaky.returncode, 0, p_flaky.stdout + p_flaky.stderr)
            self.assertIn("FLAKY", p_flaky.stdout)
            self.assertIn("(try 2/3)", p_flaky.stdout)
            self.assertIn("1 passed (1 flaky)", p_flaky.stdout)

            # 2) Test hard timeout kills infinite CPU loop promptly
            t0 = time.monotonic()
            p_to = self._run_cinch(
                td, [],
                ["test", "--clang", clang_bin(), str(self.runtime), prefix + "_to", str(src_path), "test-hang"],
            )
            elapsed = time.monotonic() - t0
            self.assertEqual(p_to.returncode, 1, p_to.stdout + p_to.stderr)
            self.assertIn("TIMEOUT", p_to.stdout)
            self.assertIn("1 timed out", p_to.stdout)
            self.assertLess(elapsed, 15.0, "timed-out test did not terminate promptly")

    def test_release_elimination_and_test_boundary_enforcement(self):
        # 1) Production code referencing a #[test_only] helper must be rejected.
        bad_src = """module BadBoundary where
  #[test_only]
  secret-helper :: Int -> Int
  secret-helper x = x + 99

  main :: Int
  main = secret-helper 1
"""
        with tempfile.TemporaryDirectory() as td:
            bad_path = Path(td) / "BadBoundary.cinch"
            bad_path.write_text(bad_src, encoding="utf-8")
            bad_prefix = str(Path(td) / "bad_build")
            p_bad = self._run_cinch(
                td, [],
                ["build", "--clang", clang_bin(), str(self.runtime), bad_prefix, str(bad_path)],
            )
            self.assertNotEqual(p_bad.returncode, 0, "expected test-only boundary violation to fail")
            diag_file = Path(bad_prefix + ".ll.diagnostics")
            diag_text = diag_file.read_text(encoding="utf-8") if diag_file.is_file() else (p_bad.stdout + p_bad.stderr)
            self.assertIn("cannot reference test-only definition", diag_text)

        # 2) Clean program with #[test], #[test_only], and #[cfg(test)] helpers + unique sentinels.
        prod_src = """module ProdApp where
  import Algebra.Semigroup
  import Data.String (string-to-bytes)

  compute :: Int -> Int
  compute x = x * 2 + 1

  #[test_only]
  test-only-sentinel-helper :: String
  test-only-sentinel-helper = "SENTINEL_TEST_ONLY_SECRET_9911"

  #[cfg(test)]
  cfg-test-sentinel-helper :: String
  cfg-test-sentinel-helper = "SENTINEL_CFG_TEST_SECRET_7733"

  #[test]
  test-compute-works :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  test-compute-works () =
    case ioWrite (string-to-bytes (test-only-sentinel-helper <> cfg-test-sentinel-helper <> "SENTINEL_TEST_CASE_SECRET_8822")) of
      _ -> if compute 20 == 41 then 0 else 1

  main :: Int
  main = if compute 20 == 41 then 0 else 1
"""
        with tempfile.TemporaryDirectory() as td:
            prod_path = Path(td) / "ProdApp.cinch"
            prod_path.write_text(prod_src, encoding="utf-8")
            prefix = str(Path(td) / "prod_release")

            # Build in --release mode
            p_rel = self._run_cinch(
                td, ["--release"],
                ["build", "--clang", clang_bin(), str(self.runtime), prefix, str(prod_path)],
            )
            self.assertEqual(p_rel.returncode, 0, f"release build failed:\n{p_rel.stdout}\n{p_rel.stderr}")

            ir_text = Path(prefix + ".ll").read_text(encoding="utf-8")
            exe_bytes = Path(prefix + ".exe").read_bytes()

            for forbidden in (
                "SENTINEL_TEST_ONLY_SECRET_9911",
                "SENTINEL_CFG_TEST_SECRET_7733",
                "SENTINEL_TEST_CASE_SECRET_8822",
                "test-compute-works",
                "test-only-sentinel-helper",
                "cfg-test-sentinel-helper",
            ):
                self.assertNotIn(forbidden, ir_text, f"found `{forbidden}` in --release LLVM IR!")
                self.assertNotIn(forbidden.encode("utf-8"), exe_bytes, f"found `{forbidden}` in --release native binary!")

            # Production binary executes cleanly
            run_prod = subprocess.run([prefix + ".exe"], capture_output=True, text=True, timeout=30)
            self.assertEqual(run_prod.returncode, 0)

            # Same file STILL runs its #[test] cleanly via `cinch test`!
            p_test = self._run_cinch(
                td, [],
                ["test", "--clang", clang_bin(), str(self.runtime), str(Path(td) / "prod_test"), str(prod_path)],
            )
            self.assertEqual(p_test.returncode, 0, p_test.stdout + p_test.stderr)
            self.assertIn("1 tests run: 1 passed, 0 failed", p_test.stdout)
            self.assertIn("ProdApp.test-compute-works", p_test.stdout)

    def test_help_and_fail_fast(self):
        # 1) `cinch test --help` succeeds and prints usage
        p_help = subprocess.run([str(self.compiler), "test", "--help"], capture_output=True, text=True, timeout=30)
        self.assertEqual(p_help.returncode, 0, p_help.stdout + p_help.stderr)
        self.assertIn("Usage:", p_help.stdout)
        self.assertIn("--filter-expr", p_help.stdout)
        self.assertIn("--junit", p_help.stdout)

        # 2) `--fail-fast` with `-j 1` aborts after the first failing test
        src = """module FailFast where
  import Data.Bool

  #[test]
  a-first-fail :: Unit -> Bool
  a-first-fail () = False

  #[test]
  b-second-never-runs :: Unit -> Bool
  b-second-never-runs () = True

  #[test]
  c-third-never-runs :: Unit -> Bool
  c-third-never-runs () = True
"""
        with tempfile.TemporaryDirectory() as td:
            src_path = Path(td) / "FailFast.cinch"
            src_path.write_text(src, encoding="utf-8")
            prefix = str(Path(td) / "ff")
            p_ff = self._run_cinch(
                td, [],
                ["test", "--clang", clang_bin(), "-j", "1", "--fail-fast", str(self.runtime), prefix, str(src_path)],
            )
            self.assertEqual(p_ff.returncode, 1, p_ff.stdout + p_ff.stderr)
            self.assertIn("FailFast.a-first-fail", p_ff.stdout)
            self.assertIn("1 tests run: 0 passed, 1 failed", p_ff.stdout)
            self.assertNotIn("FailFast.b-second-never-runs", p_ff.stdout)


if __name__ == "__main__":
    unittest.main()

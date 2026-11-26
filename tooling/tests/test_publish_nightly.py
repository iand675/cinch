import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import publish_nightly as nightly


REPO = "iand675/cinch"
REVISION = "a" * 40


def build(run_id=42, **changes):
    return {
        "id": run_id, "repository": {"full_name": REPO},
        "head_repository": {"full_name": REPO}, "head_branch": "main",
        "path": nightly.WORKFLOW, "event": "push", "status": "completed",
        "conclusion": "success", "head_sha": REVISION,
        "created_at": "2026-11-23T23:59:00Z", **changes,
    }


class NightlyPublication(unittest.TestCase):
    def distribution(self, directory, system):
        directory.mkdir(parents=True)
        binary = f"cinch-{system}"
        content = f"checked {system}".encode()
        digest = hashlib.sha256(content).hexdigest()
        (directory / binary).write_bytes(content)
        (directory / (binary + ".sha256")).write_text(f"{digest}  {binary}\n")
        # Cinch.Host varies per system, so bundle hashes must not be
        # compared across architectures even when the revision is identical.
        (directory / f"build-{system}.json").write_text(json.dumps({
            "revision": REVISION, "system": system, "stage": 3,
            "sha256": digest, "compiler_source_sha256": hashlib.sha256(system.encode()).hexdigest(),
        }))
        (directory / "README.md").write_text("compiler usage\n")

    def download(self, *args):
        self.assertEqual(args[:2], ("run", "download"))
        self.assertEqual(args[2], 42)
        name = args[args.index("--name") + 1]
        system = next(s for s in nightly.SYSTEMS if name == f"cinch-{s}-{REVISION}")
        self.distribution(Path(args[args.index("--dir") + 1]), system)
        return ""

    def test_run_must_be_successful_main_in_the_expected_repository_and_workflow(self):
        nightly.validate_run(REPO, build())
        for changes in (
            {"head_branch": "branch"}, {"event": "pull_request"},
            {"conclusion": "failure"}, {"status": "in_progress"},
            {"head_repository": {"full_name": "fork/cinch"}},
            {"repository": {"full_name": "other/cinch"}}, {"path": "other.yml"},
            {"head_sha": "invalid"}, {"id": True},
        ):
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                nightly.validate_run(REPO, build(**changes))

    def test_selection_skips_incomplete_and_expired_sets(self):
        runs = [build(44), build(43), build(42)]
        def records(repo, resource, field):
            if field == "workflow_runs":
                return iter(runs)
            run_id = int(resource.split("/")[2])
            artifacts = [{"name": n, "expired": run_id == 43} for n in nightly.artifact_names(build())]
            return iter(artifacts[:2] if run_id == 44 else artifacts)
        with patch.object(nightly, "pages", side_effect=records):
            self.assertEqual(nightly.select_run(REPO)["id"], 42)

    def test_explicit_run_never_silently_falls_back(self):
        with patch.object(nightly, "api", return_value=build()), \
                patch.object(nightly, "complete_artifacts", return_value=False), \
                patch.object(nightly, "pages") as listing:
            with self.assertRaisesRegex(ValueError, "all three"):
                nightly.select_run(REPO, 42)
            listing.assert_not_called()

    def test_no_complete_build_is_an_error(self):
        with patch.object(nightly, "pages", return_value=iter([])):
            with self.assertRaisesRegex(ValueError, "no successful main build"):
                nightly.select_run(REPO)

    def test_artifact_listing_is_paginated(self):
        first = [{"name": "unrelated", "expired": False}] * 100
        last = [{"name": n, "expired": False} for n in nightly.artifact_names(build())]
        with patch.object(nightly, "api", side_effect=[{"artifacts": first}, {"artifacts": last}]) as api:
            self.assertTrue(nightly.complete_artifacts(REPO, build()))
        self.assertIn("page=2", api.call_args.args[1])

    def test_duplicate_platform_artifacts_are_not_complete(self):
        rows = [{"name": n, "expired": False} for n in nightly.artifact_names(build())]
        with patch.object(nightly, "pages", return_value=iter([*rows, rows[0]])):
            self.assertFalse(nightly.complete_artifacts(REPO, build()))

    def test_receipts_checksums_and_files_are_verified(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "dist"
            system = nightly.SYSTEMS[0]
            self.distribution(root, system)
            nightly.verify_distribution(root, system, REVISION)
            receipt = root / f"build-{system}.json"
            original = json.loads(receipt.read_text())
            for key, value in (("revision", "b" * 40), ("system", "aarch64-linux"),
                               ("stage", 2), ("stage", True), ("sha256", "0" * 64),
                               ("compiler_source_sha256", "bad")):
                with self.subTest(key=key, value=value):
                    receipt.write_text(json.dumps({**original, key: value}))
                    with self.assertRaises(ValueError):
                        nightly.verify_distribution(root, system, REVISION)
            receipt.write_text(json.dumps(original))
            binary = root / f"cinch-{system}"
            binary.write_bytes(b"corrupted binary")
            with self.assertRaisesRegex(ValueError, "checksum"):
                nightly.verify_distribution(root, system, REVISION)
            binary.unlink()
            binary.symlink_to(receipt)
            with self.assertRaisesRegex(ValueError, "non-regular"):
                nightly.verify_distribution(root, system, REVISION)

    def test_prepare_uses_source_build_identity_after_midnight(self):
        with tempfile.TemporaryDirectory() as temp, patch.object(nightly, "gh", side_effect=self.download):
            first = nightly.prepare(REPO, build(), Path(temp) / "first")
            second = nightly.prepare(REPO, build(), Path(temp) / "second")
            self.assertEqual(first, second)
            self.assertEqual(first["tag"], f"nightly-20261123-{REVISION[:12]}-42")
            self.assertEqual(first["title"], f"Cinch nightly 2026-11-23 ({REVISION[:8]})")
            self.assertIn("/actions/runs/42", (Path(temp) / "first/notes.md").read_text())
            release = Path(temp) / "first/release"
            self.assertEqual(len(list(release.iterdir())), 11)
            self.assertEqual(len((release / "SHA256SUMS").read_text().splitlines()), 3)
            # Interpret an offset timestamp as UTC before deriving the date.
            self.assertEqual(str(nightly.validate_run(REPO, build(created_at="2026-11-24T00:59:00+01:00"))),
                             "2026-11-23")

    def test_no_release_mutation_before_every_distribution_is_verified(self):
        def corrupt_last(*args):
            self.download(*args)
            if "aarch64-darwin" in args[args.index("--name") + 1]:
                root = Path(args[args.index("--dir") + 1])
                (root / "cinch-aarch64-darwin").write_bytes(b"corrupt")
        with tempfile.TemporaryDirectory() as temp, patch.object(nightly, "gh", side_effect=corrupt_last) as gh:
            with self.assertRaisesRegex(ValueError, "checksum"):
                nightly.prepare(REPO, build(), Path(temp) / "work")
            self.assertTrue(all(call.args[0] == "run" for call in gh.call_args_list))
            self.assertFalse((Path(temp) / "work/release").exists())

    def test_preparation_is_read_only_by_default(self):
        with tempfile.TemporaryDirectory() as temp, patch.object(sys, "argv", [
                "publish_nightly.py", "--repo", REPO, "--out", str(Path(temp) / "work")]), \
                patch.object(nightly, "select_run", return_value=build()), \
                patch.object(nightly, "gh", side_effect=self.download), \
                patch.object(nightly, "publish") as publish:
            nightly.main()
            publish.assert_not_called()

    def test_draft_resumes_and_public_release_is_unchanged(self):
        with tempfile.TemporaryDirectory() as temp, patch.object(nightly, "gh", side_effect=self.download):
            root = Path(temp) / "work"
            identity = nightly.prepare(REPO, build(), root)
            existing = {"target_commitish": REVISION, "draft": True}
            with patch.object(nightly, "api", return_value=existing), patch.object(nightly, "gh") as gh:
                nightly.publish(REPO, root, identity)
                self.assertEqual([call.args[:2] for call in gh.call_args_list],
                                 [("release", "upload"), ("release", "edit")])
            existing.update(draft=False, assets=[{"name": p.name} for p in (root / "release").iterdir()])
            with patch.object(nightly, "api", return_value=existing), patch.object(nightly, "gh") as gh:
                nightly.publish(REPO, root, identity)
                gh.assert_not_called()
            for changed in ({**existing, "target_commitish": "b" * 40}, {**existing, "assets": []}):
                with patch.object(nightly, "api", return_value=changed), patch.object(nightly, "gh") as gh:
                    with self.assertRaises(ValueError):
                        nightly.publish(REPO, root, identity)
                    gh.assert_not_called()

    def test_only_a_confirmed_404_creates_a_new_draft(self):
        with tempfile.TemporaryDirectory() as temp, patch.object(nightly, "gh", side_effect=self.download):
            root = Path(temp) / "work"
            identity = nightly.prepare(REPO, build(), root)
            for error_text in ("HTTP 503", "connection timeout", "(HTTP 404)"):
                error = subprocess.CalledProcessError(1, ["gh"], stderr=error_text)
                with patch.object(nightly, "api", side_effect=error), patch.object(nightly, "gh") as gh:
                    if "404" in error_text:
                        nightly.publish(REPO, root, identity)
                        self.assertEqual([call.args[:2] for call in gh.call_args_list],
                                         [("release", "create"), ("release", "upload"), ("release", "edit")])
                        self.assertIn("--draft", gh.call_args_list[0].args)
                        self.assertIn(REVISION, gh.call_args_list[0].args)
                    else:
                        with self.assertRaises(subprocess.CalledProcessError):
                            nightly.publish(REPO, root, identity)
                        gh.assert_not_called()


if __name__ == "__main__":
    unittest.main()

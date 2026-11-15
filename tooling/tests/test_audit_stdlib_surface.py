"""Coverage inventory controls: target deduplication and honest drift detection."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SOURCE = Path(__file__).resolve().parents[1] / "audit_stdlib_surface.py"
SPEC = importlib.util.spec_from_file_location("audit_stdlib_surface", SOURCE)
AUDIT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(AUDIT)


class SurfaceAuditTests(unittest.TestCase):
    """Package presence must not silently promote or stale a coverage claim."""

    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        (self.root / "api").mkdir()
        self.output = self.root / "surface.json"

    def invoke(self, *options):
        """Supply an isolated SDK's package inventory without installing Go in CI."""
        def command(_go, *args):
            if args == ("env", "GOROOT"):
                return str(self.root)
            if args == ("list", "std"):
                return "io\nmissing\ninternal/test\npath/internal/test\nvendor/something\n"
            if args == ("version",):
                return "go version controlled-test linux/amd64"
            self.fail(f"unexpected SDK request {args}")
        with patch.object(AUDIT, "command", command), patch("sys.argv", ["audit", "--go", "/unused/go", "--output", str(self.output), *options]):
            AUDIT.main()

    def test_declarations_deduplicate_targets_and_remove_issue_annotations(self):
        (self.root / "api/go1.txt").write_text("pkg io, func Copy(Writer, Reader) (int64, error)\npkg io (linux-amd64), func Copy(Writer, Reader) (int64, error)\n")
        (self.root / "api/go1.27.txt").write_text("pkg io, func NewOperation() #12345\n")
        self.assertEqual(AUDIT.declarations(self.root)["io"], ["func Copy(Writer, Reader) (int64, error)", "func NewOperation()"])

    def test_check_rejects_declaration_drift_instead_of_rewriting_the_record(self):
        self.invoke()
        (self.root / "api/go1.txt").write_text("pkg io, func NewOperation()\n")
        before = self.output.read_bytes()
        with self.assertRaisesRegex(AssertionError, "io"):
            self.invoke("--check")
        self.assertEqual(self.output.read_bytes(), before)

    def test_generation_preserves_reviews_without_claiming_unmapped_packages_complete(self):
        self.invoke()
        report = json.loads(self.output.read_text())
        self.assertEqual([entry["reference_package"] for entry in report["packages"]], ["io", "missing"])
        self.assertEqual([entry["status"] for entry in report["packages"]], ["partial", "unreviewed"])
        report["packages"][0]["remaining"] = ["pipe cancellation"]
        self.output.write_text(json.dumps(report))
        self.invoke()
        self.assertEqual(json.loads(self.output.read_text())["packages"][0]["remaining"], ["pipe cancellation"])


if __name__ == "__main__":
    unittest.main()

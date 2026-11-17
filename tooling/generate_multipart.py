#!/usr/bin/env python3
"""Generate pinned public multipart observations and check independent scalar models.

SDK access is confined to this generator. Committed JSON is sufficient for later
consumer CI. Exact compiler, source and SDK identities accompany each generation.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

#: Repository root for explicitly selected SDK fixture generation.
ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify complete immutable input bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def quoted(value: bytes) -> bytes:
    """Escape disposition bytes independently of the public reference."""
    return value.replace(b"\\", b"\\\\").replace(b'"', b'\\"').replace(b"\r", b"%0D").replace(b"\n", b"%0A")


def disposition(name: bytes, filename: bytes | None = None) -> bytes:
    """Build exact field/file disposition bytes with the historical escaping."""
    out = b'form-data; name="' + quoted(name) + b'"'
    return out if filename is None else out + b'; filename="' + quoted(filename) + b'"'


def boundary_error(value: bytes) -> str:
    """Validate the complete boundary byte range without Unicode categories."""
    if not 1 <= len(value) <= 70:
        return "mime: invalid boundary length"
    allowed = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'()+_,-./:=? "
    if value[-1:] == b" " or any(byte not in allowed for byte in value):
        return "mime: invalid boundary character"
    return ""


def content_type(boundary: bytes) -> bytes:
    """Render explicit-boundary media syntax without a media formatter dependency."""
    if any(byte in b'()<>@,;:\\"/[]?= ' for byte in boundary):
        boundary = b'"' + boundary + b'"'
    return b"multipart/form-data; boundary=" + boundary


class ScalarWriter:
    """Independent state machine for confirmed output, aliases and repeatable finish."""

    def __init__(self, row: dict) -> None:
        """Create a deterministic sink and writer state with no package callbacks."""
        self.limits = row["limits"] or []
        self.failures = row["failures"]
        self.trace: list[dict] = []
        self.output = bytearray()
        self.boundary = b"a b"
        self.current: dict | None = None

    def offer(self, value: bytes) -> tuple[int, str]:
        """Accept exactly one scripted prefix and an independent simultaneous error."""
        call = len(self.trace)
        limit = self.limits[call] if call < len(self.limits) else -1
        count = min(limit, len(value)) if limit >= 0 else len(value)
        error = self.failures.get(str(call), "")
        self.trace.append({"offer": value.hex(), "count": count, "error": error})
        self.output.extend(value[:count])
        return count, error

    def create(self, headers: dict[bytes, list[bytes]]) -> tuple[dict | None, str]:
        """Finish old body before the one header callback, retaining failed old state."""
        if self.current is not None:
            self.current["closed"] = True
            if self.current["error"]:
                return None, self.current["error"]
        prefix = b"\r\n--" if self.current is not None else b"--"
        wire = prefix + self.boundary + b"\r\n"
        for key, values in sorted(headers.items()):
            for value in values:
                wire += key + b": " + value + b"\r\n"
        wire += b"\r\n"
        count, error = self.offer(wire)
        if not error and count != len(wire):
            error = "short write"
        if error:
            return None, error
        self.current = {"closed": False, "error": ""}
        return self.current, ""

    def write(self, part: dict, value: bytes) -> tuple[int, str]:
        """Use actual shared part identity and retain its latest nonempty failure."""
        if part["closed"]:
            return 0, "multipart: can't write to finished part"
        count, error = self.offer(value)
        if error:
            part["error"] = error
        return count, error

    def finish(self) -> str:
        """Repeat footer output without synthesizing an error on short success."""
        if self.current is not None:
            self.current["closed"] = True
            if self.current["error"]:
                return self.current["error"]
            self.current = None
        return self.offer(b"\r\n--" + self.boundary + b"--\r\n")[1]

    def set_boundary(self, boundary: bytes) -> str:
        """Preserve the state-before-length-before-character refusal order."""
        if self.current is not None:
            return "mime: SetBoundary called after write"
        error = boundary_error(boundary)
        if not error:
            self.boundary = boundary
        return error

    def scenario(self) -> list[dict]:
        """Replay the public generator's operations through a separate implementation."""
        actions = [{"operation": "set", "error": ""}]
        first, error = self.create({b"z-raw": [b"last", b"again"], b"A": [b"one"], b"empty": [], b"": [b"\0"]})
        actions.append({"operation": "create", "error": error})
        if first is not None:
            for operation, value in [("write", b"body"), ("zero-write", b"")]:
                count, error = self.write(first, value)
                actions.append({"operation": operation, "count": count, "error": error})
        _, error = self.create({b"Content-Disposition": [disposition(b'na\r\n"\\me', b"path/file")], b"Content-Type": [b"application/octet-stream"]})
        actions.append({"operation": "replace", "error": error})
        if first is not None:
            count, error = self.write(first, b"stale")
            actions.append({"operation": "stale", "count": count, "error": error})
        actions.append({"operation": "set-live", "error": self.set_boundary(b"new")})
        actions.extend({"operation": "close", "error": self.finish()} for _ in range(2))
        actions.append({"operation": "set-after-close", "error": self.set_boundary(b"new")})
        part, error = self.create({b"Content-Disposition": [disposition(b"after")]})
        if not error:
            _, error = self.write(part, b"value")
        actions.append({"operation": "field-after-close", "error": error})
        return actions


def verify(data: dict) -> dict:
    """Cross-check byte codecs and every scripted writer effect independently."""
    for row in data["boundaries"]:
        value = bytes.fromhex(row["input"])
        error = boundary_error(value)
        assert row["error"] == error, row
        assert row["content_type"] == ("" if error else content_type(value).hex()), row
    for row in data["dispositions"]:
        assert row["output"] == disposition(bytes.fromhex(row["name"]), bytes.fromhex(row["filename"])).hex(), row
    for row in data["writers"]:
        writer = ScalarWriter(row)
        assert writer.scenario() == row["actions"], row
        assert writer.trace == row["trace"], row
        assert writer.output.hex() == row["output"], row
        assert writer.boundary.hex() == row["boundary"], row
        assert content_type(writer.boundary).hex() == row["content_type"], row
    assert bytes.fromhex(data["cache"]["name"]) == b"edited"
    assert bytes.fromhex(data["cache"]["late_name"]) == b"edited"
    assert bytes.fromhex(data["cache"]["filename"]) == b"after"
    assert bytes.fromhex(data["cache"]["old_body"]) == b""
    assert bytes.fromhex(data["cache"]["next_body"]) == b"last"
    for row in data["readers"]:
        for part in row["parts"]:
            assert "".join(read["output"] for read in part["reads"]) == part["body"]
    interrupted = [row for row in data["forms"] if row.get("interrupted_close_count")]
    assert len(interrupted) == 1
    assert interrupted[0]["interrupted_close_count"] == 1 and interrupted[0]["form_present"]
    assert interrupted[0]["error"] == "close <scratch>: bad file descriptor"
    assert all(error == "open <scratch>: no such file or directory" for error in interrupted[0]["reopened"])
    return {"boundary": len(data["boundaries"]), "disposition": len(data["dispositions"]),
            "writer_scenarios": len(data["writers"]), "reader_observations": len(data["readers"]),
            "form_observations": len(data["forms"]), "cache_controls": 1,
            "independent_scalar_scope": "all boundaries/dispositions/writer callback traces, reader prefix accounting, lazy-cache and final-close form/error observations"}


def main() -> None:
    """Run one explicit SDK generator or independently verify already committed bytes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--go", type=Path)
    parser.add_argument("--output", type=Path, default=ROOT / "compiler/tests/fixtures/multipart-vectors.json")
    parser.add_argument("--cache", type=Path, default=Path("/tmp/cinch-mime-go-cache"))
    parser.add_argument("--verify-only", action="store_true")
    parser.add_argument("--cpu", type=int)
    args = parser.parse_args()
    if args.cpu is not None:
        os.sched_setaffinity(0, {args.cpu})
    source = ROOT / "tooling/data/multipart-reference.go"
    args.output.parent.mkdir(parents=True, exist_ok=True)
    if args.verify_only:
        counts = verify(json.loads(args.output.read_text()))
        print(json.dumps(counts, sort_keys=True))
        return
    metadata = {"generator_sha256": digest(source), "scalar_verifier_sha256": digest(Path(__file__))}
    if not args.verify_only:
        if args.go is None:
            parser.error("generation requires an explicitly selected pinned --go")
        env = {**os.environ, "GOTOOLCHAIN": "local", "GOMAXPROCS": "1", "GOCACHE": str(args.cache)}
        command = [str(args.go), "run", str(source)]
        result = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, check=True)
        args.output.write_bytes(result.stdout)
        metadata.update({"command": command, "go_sha256": digest(args.go), "stderr": result.stderr.decode(),
                         "sdk_version": (args.go.parent.parent / "VERSION").read_text()})
    data = json.loads(args.output.read_text())
    metadata.update({"vectors_sha256": digest(args.output), "counts": verify(data)})
    args.output.with_suffix(".provenance.json").write_text(json.dumps(metadata, indent=2) + "\n")
    print(json.dumps(metadata["counts"], sort_keys=True))


if __name__ == "__main__":
    main()

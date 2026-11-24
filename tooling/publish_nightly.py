#!/usr/bin/env python3
"""Promote a complete, checked main build without starting compiler jobs.

Preparation is read-only on GitHub. Only --publish creates or updates a release.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess


SYSTEMS = ("x86_64-linux", "aarch64-linux", "aarch64-darwin")
WORKFLOW = ".github/workflows/ladder-cache.yml"


def gh(*args):
    return subprocess.run(["gh", *map(str, args)], check=True, text=True,
                          capture_output=True).stdout


def api(repo, resource):
    return json.loads(gh("api", "--method", "GET", f"repos/{repo}/{resource}"))


def pages(repo, resource, field):
    separator = "&" if "?" in resource else "?"
    page = 1
    while True:
        rows = api(repo, f"{resource}{separator}per_page=100&page={page}")[field]
        yield from rows
        if len(rows) < 100:
            return
        page += 1


def validate_run(repo, run):
    if (run.get("repository", {}).get("full_name") != repo
            or run.get("head_repository", {}).get("full_name") != repo
            or run.get("head_branch") != "main"
            or run.get("path") != WORKFLOW
            or run.get("event") not in ("push", "schedule", "workflow_dispatch")
            or run.get("status") != "completed" or run.get("conclusion") != "success"):
        raise ValueError("publication requires a successful main Compiler distributions run in this repository")
    if (type(run.get("id")) is not int or run["id"] <= 0
            or not re.fullmatch(r"[0-9a-f]{40}", run.get("head_sha", ""))):
        raise ValueError("invalid build identity")
    created = datetime.fromisoformat(run["created_at"].replace("Z", "+00:00"))
    if created.tzinfo is None:
        raise ValueError("build timestamp must include a timezone")
    return created.astimezone(timezone.utc).date()


def artifact_names(run):
    return [f"cinch-{system}-{run['head_sha']}" for system in SYSTEMS]


def complete_artifacts(repo, run):
    rows = list(pages(repo, f"actions/runs/{run['id']}/artifacts", "artifacts"))
    return all(sum(row["name"] == name and not row.get("expired", True)
                   for row in rows) == 1 for name in artifact_names(run))


def select_run(repo, run_id=None):
    if run_id is not None:
        run = api(repo, f"actions/runs/{run_id}")
        validate_run(repo, run)
        if run["id"] != run_id or not complete_artifacts(repo, run):
            raise ValueError(f"build {run_id} does not have all three unexpired distributions")
        return run
    # GitHub lists newest runs first. Reject branch/PR runs even if an API
    # filter changes, and skip old successful runs whose artifacts expired.
    for run in pages(repo, "actions/workflows/ladder-cache.yml/runs?branch=main&status=success",
                     "workflow_runs"):
        try:
            validate_run(repo, run)
        except ValueError:
            continue
        if complete_artifacts(repo, run):
            return run
    raise ValueError("no successful main build has all three unexpired distributions")


def required_file(directory, name):
    path = directory / name
    if path.is_symlink() or not path.is_file():
        raise ValueError(f"missing or non-regular distribution file: {path}")
    return path


def verify_distribution(directory, system, revision):
    name = f"cinch-{system}"
    binary = required_file(directory, name)
    hasher = hashlib.sha256()
    with binary.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            hasher.update(chunk)
    digest = hasher.hexdigest()
    checksum = required_file(directory, name + ".sha256")
    if checksum.read_text() != f"{digest}  {name}\n":
        raise ValueError(f"invalid checksum for {system}")
    receipt = required_file(directory, f"build-{system}.json")
    metadata = json.loads(receipt.read_text())
    if (metadata.get("revision") != revision or metadata.get("system") != system
            or type(metadata.get("stage")) is not int or metadata["stage"] != 3
            or metadata.get("sha256") != digest
            or not re.fullmatch(r"[0-9a-f]{64}", metadata.get("compiler_source_sha256", ""))):
        raise ValueError(f"invalid build receipt for {system}")
    return binary, checksum, receipt


def prepare(repo, run, directory):
    day = validate_run(repo, run)
    directory.mkdir(parents=True, exist_ok=False)
    revision = run["head_sha"]
    files = []
    downloads = directory / "artifacts"
    for system, name in zip(SYSTEMS, artifact_names(run)):
        destination = downloads / name
        gh("run", "download", run["id"], "--repo", repo,
           "--name", name, "--dir", destination)
        checked = verify_distribution(destination, system, revision)
        files.extend(checked)
    # Compiler bundles include a generated Bootstrap.Host module, so their
    # source hashes differ across systems. The run revision must match instead.
    readme = required_file(downloads / artifact_names(run)[0], "README.md")
    release = directory / "release"
    release.mkdir()
    for source in [*files, readme]:
        shutil.copyfile(source, release / source.name)
    (release / "SHA256SUMS").write_text("".join(
        (release / f"cinch-{system}.sha256").read_text() for system in SYSTEMS))
    # All identity fields belong to the source build, including its UTC date.
    # Retrying after midnight or in another publisher run resumes the same tag.
    identity = {
        "tag": f"nightly-{day:%Y%m%d}-{revision[:12]}-{run['id']}",
        "title": f"Cinch nightly {day} ({revision[:8]})",
        "revision": revision,
        "build_run_id": run["id"],
    }
    (directory / "identity.json").write_text(json.dumps(identity, indent=2) + "\n")
    (directory / "notes.md").write_text(
        f"Native compiler distributions built from {revision}.\n\n"
        "All three platforms passed the bootstrap fixed point, stage smokes, "
        "interface consumption and changed-source checks.\n\n"
        "Download the native Stage 3 executable for your platform and make it executable "
        "with chmod +x. See README.md for usage. Each binary has a SHA-256 sidecar; "
        "SHA256SUMS covers all three. Linux binaries are statically linked; "
        "macOS uses only system libraries.\n\n"
        f"Build: https://github.com/{repo}/actions/runs/{run['id']}\n")
    return identity


def publish(repo, directory, identity):
    tag = identity["tag"]
    try:
        existing = api(repo, f"releases/tags/{tag}")
    except subprocess.CalledProcessError as error:
        if "(HTTP 404)" not in error.stderr:
            raise
        existing = None
    if existing is not None:
        if existing["target_commitish"] != identity["revision"]:
            raise ValueError("existing nightly tag targets a different revision")
        if not existing["draft"]:
            names = {asset["name"] for asset in existing["assets"]}
            if not {path.name for path in (directory / "release").iterdir()} <= names:
                raise ValueError("existing public nightly is incomplete")
            print(f"Already published: {tag}")
            return
    else:
        gh("release", "create", tag, "--repo", repo, "--target", identity["revision"],
           "--draft", "--prerelease", "--title", identity["title"],
           "--notes-file", directory / "notes.md")
    gh("release", "upload", tag, "--repo", repo,
       *sorted((directory / "release").iterdir()), "--clobber")
    gh("release", "edit", tag, "--repo", repo, "--draft=false", "--prerelease", "--latest=false")


def positive_id(value):
    if not re.fullmatch(r"[1-9][0-9]*", value):
        raise argparse.ArgumentTypeError("build run ID must be a positive integer")
    return int(value)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", default=os.environ.get("GH_REPO"), required=not os.environ.get("GH_REPO"))
    parser.add_argument("--run-id", type=positive_id, help="otherwise select the latest complete successful main build")
    parser.add_argument("--out", type=Path, default=Path("build/nightly-publication"))
    parser.add_argument("--publish", action="store_true", help="create/update the release after verification")
    args = parser.parse_args()
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", args.repo):
        parser.error("repository must be owner/name")
    try:
        run = select_run(args.repo, args.run_id)
        identity = prepare(args.repo, run, args.out)
        print(f"Verified build {run['id']}: {identity['tag']}")
        if args.publish:
            publish(args.repo, args.out, identity)
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"nightly publication: {error}\n{getattr(error, 'stderr', '') or ''}")


if __name__ == "__main__":
    main()

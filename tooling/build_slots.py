"""A cross-process build semaphore, so several lanes can build at once.

Every lane builds in its own tree, so nothing needs to be serialised for
correctness. What must be bounded is how many *heavy* processes exist at
once: each stage link runs several clang shards, and each test target runs a
compiler, so nine lanes at their own default parallelism oversubscribe the
machine and every measurement becomes noise.

The semaphore is a directory of lock files. A slot is taken by creating and
flocking one of them; the kernel drops the lock when the holder exits, so a
killed build cannot leak a slot. Waiting is a poll with a short sleep, which
is enough here because a slot is held for seconds to minutes, not for
microseconds.

Environment:
  CINCH_BUILD_SLOTS      how many holders may run at once (default below)
  CINCH_BUILD_SLOTS_DIR  where the lock files live (default under the system
                         temp directory, keyed by the machine's user only)
"""

from __future__ import annotations

import contextlib
import fcntl
import os
import time
from pathlib import Path

# The default leaves one core of slack on an 18-core machine once nine lanes
# are running: the point of the semaphore is to bound concurrent heavy
# processes, not to bound a single process's shard parallelism.
DEFAULT_SLOTS = 6


def slot_count() -> int:
    raw = (os.environ.get("CINCH_BUILD_SLOTS") or "").strip()
    if not raw:
        return DEFAULT_SLOTS
    try:
        value = int(raw)
    except ValueError:
        raise SystemExit("CINCH_BUILD_SLOTS must be a positive integer")
    if value < 1:
        raise SystemExit("CINCH_BUILD_SLOTS must be a positive integer")
    return value


# A slot taken for the rest of the process's life, held here so the handle
# outlives the call that took it.
_HELD: list = []


def hold(owner: str = "", poll: float = 0.25):
    """Take a build slot and keep it until this process exits.

    A command line tool wants the slot for its whole run, not for a block:
    the kernel drops the lock on exit, so a killed build cannot leak one.
    """
    if _HELD:
        return
    count = slot_count()
    directory = slot_directory()
    while True:
        for index in range(count):
            path = directory / f"{index}.lock"
            handle = path.open("a+")
            try:
                fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except OSError:
                handle.close()
                continue
            if owner:
                handle.seek(0)
                handle.truncate()
                handle.write(f"{owner} {os.getpid()}\n")
                handle.flush()
            _HELD.append(handle)
            return
        time.sleep(poll)


def slot_directory() -> Path:
    raw = os.environ.get("CINCH_BUILD_SLOTS_DIR")
    if raw:
        directory = Path(raw)
    else:
        import tempfile

        directory = Path(tempfile.gettempdir()) / "cinch-build-slots"
    directory.mkdir(parents=True, exist_ok=True)
    return directory


@contextlib.contextmanager
def slot(owner: str = "", poll: float = 0.25):
    """Hold one build slot for the duration of the block.

    `owner` is only a label written into the lock file, so a stuck build can
    be attributed to a lane without guessing from pids alone.
    """
    count = slot_count()
    directory = slot_directory()
    while True:
        for index in range(count):
            path = directory / f"{index}.lock"
            handle = path.open("a+")
            try:
                fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except OSError:
                handle.close()
                continue
            if owner:
                handle.seek(0)
                handle.truncate()
                handle.write(f"{owner} {os.getpid()}\n")
                handle.flush()
            try:
                yield
            finally:
                fcntl.flock(handle, fcntl.LOCK_UN)
                handle.close()
            return
        time.sleep(poll)


def status() -> list[tuple[int, str]]:
    """Read what the slots currently hold, for a stuck-build diagnosis."""
    directory = slot_directory()
    held = []
    for index in range(slot_count()):
        path = directory / f"{index}.lock"
        if not path.is_file():
            continue
        handle = path.open("a+")
        try:
            fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError:
            handle.seek(0)
            held.append((index, handle.read().strip()))
        else:
            fcntl.flock(handle, fcntl.LOCK_UN)
        finally:
            handle.close()
    return held

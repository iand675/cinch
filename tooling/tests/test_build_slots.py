import os
import sys
import tempfile
import threading
import unittest
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import build_slots


class BuildSlotAdmission(unittest.TestCase):
    def test_next_target_waits_behind_a_lane_already_polling(self):
        polling = threading.Event()
        resume = threading.Event()
        next_target_started = threading.Event()
        order = []

        def poll(_):
            polling.set()
            self.assertTrue(resume.wait(5))

        def work(name):
            if name == "next target":
                next_target_started.set()
            with build_slots.slot(name):
                order.append(name)

        with tempfile.TemporaryDirectory() as root, patch.dict(os.environ, {
            "CINCH_BUILD_SLOTS_DIR": root, "CINCH_BUILD_SLOTS": "1",
        }), patch.object(build_slots.time, "sleep", poll), ThreadPoolExecutor(2) as pool:
            with build_slots.slot("running target"):
                lane = pool.submit(work, "waiting lane")
                self.assertTrue(polling.wait(5))
            next_target = pool.submit(work, "next target")
            try:
                self.assertTrue(next_target_started.wait(5))
                self.assertFalse(next_target.done())
            finally:
                resume.set()
            lane.result(5)
            next_target.result(5)
            self.assertEqual(order, ["waiting lane", "next target"])
            self.assertEqual(build_slots.status(), [])

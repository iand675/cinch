import sys
import unittest
import os
import tempfile
from unittest.mock import patch
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import shard_ir  # noqa: E402


class ClangWrapperTests(unittest.TestCase):
    def test_target_in_a_path_does_not_disable_shards(self):
        with tempfile.TemporaryDirectory(prefix="bootstrap-target-") as directory:
            ir = Path(directory) / "program.ll"
            ir.write_text("define i32 @main() {\n  ret i32 0\n}\n")
            args = [str(ir), "-o", str(ir.with_suffix(""))]
            with patch.dict(os.environ, CINCH_CLANG_REAL="clang", CINCH_BOOTSTRAP_JOBS="2",
                            CINCH_SHARD_MIN_BYTES="1"), \
                    patch.object(shard_ir, "compile_shards", return_value=(0, "")) as compile_shards, \
                    patch.object(shard_ir.subprocess, "run") as link:
                link.return_value.returncode = 0
                self.assertEqual(shard_ir.clang_main(args), 0)
                compile_shards.assert_called_once()
                compile_shards.reset_mock()
                for flags in (["-target", "wasm32"], ["--target=wasm32"], ["-arch", "arm64"]):
                    self.assertEqual(shard_ir.clang_main([*flags, *args]), 0)
                    link.assert_called_with(["clang", *flags, *args])
                compile_shards.assert_not_called()


class ModuleTests(unittest.TestCase):
    def test_invalid_later_shard_preserves_previous_outputs(self):
        ir = "define i32 @A.f() {\n  ret i32 1\n}\ndefine i32 @B.f() {\n  ret i32 2\n}\n"
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "program.ll"
            path.write_text(ir)
            outputs = shard_ir.write_shards(path, path.parent, 2)
            expected = [output.read_bytes() for output in outputs]
            path.write_text(ir.replace("ret i32 1", "ret i32 3").replace(
                "ret i32 2", "%x = call i32 @missing()\n  ret i32 %x"))
            with self.assertRaisesRegex(shard_ir.ShardError, "undefined symbol missing"):
                shard_ir.write_shards(path, path.parent, 2)
            self.assertEqual([output.read_bytes() for output in outputs], expected)

    def test_function_boundaries_and_multiline_units(self):
        for newline in ("\n", "\r\n", "\r"):
            with self.subTest(newline=newline):
                body = newline.join(["define i32 @main() {", "  ret i32 0", "}", ""])
                declaration = newline.join(["declare i32 @call(", "  i32)", ""])
                module = shard_ir.parse_module(declaration + body + "!0 = !{}")
                self.assertEqual(module.functions[0].text, body)
                self.assertEqual(module.declares["call"], declaration)
                self.assertEqual(module.metadata, ["!0 = !{}"])
                with self.assertRaisesRegex(shard_ir.ShardError, "line 4 has no closing brace"):
                    shard_ir.parse_module(declaration + newline + body.split("}")[0])


class DeclarationTests(unittest.TestCase):
    def test_function_data_stays_with_the_definition(self):
        # An apply stub carries its tailcc twin's address in prefix data; a
        # declaration of the stub in another shard must not, or it names a
        # symbol that shard neither defines nor declares.
        define = ('define i64 @"X.app"(ptr %self, i64 %argc, ptr %argv) prefix <{ i64, i64 }> '
                  '<{ i64 4848492524724309041, i64 ptrtoint (ptr @"X.tapp" to i64) }> #3 {\n  ret i64 0\n}\n')
        self.assertEqual(shard_ir.declaration_for(define), 'declare i64 @"X.app"(ptr %self, i64 %argc, ptr %argv) #3\n')

    def test_quoted_symbols_in_function_data_may_spell_brackets(self):
        define = ('define i64 @"Ord_Nat.<=.app"(ptr %self, i64 %argc, ptr %argv) prefix <{ i64, i64 }> '
                  '<{ i64 1, i64 ptrtoint (ptr @"Ord_Nat.<=.tapp" to i64) }> #2 {\n  ret i64 0\n}\n')
        self.assertEqual(shard_ir.declaration_for(define), 'declare i64 @"Ord_Nat.<=.app"(ptr %self, i64 %argc, ptr %argv) #2\n')

    def test_linkage_is_removed_and_attributes_kept(self):
        self.assertEqual(shard_ir.declaration_for("define internal i64 @f(i64 %x) #1 {\n}\n"), "declare i64 @f(i64 %x) #1\n")


if __name__ == "__main__":
    unittest.main()


def _module(bodies):
    """Twelve one-function groups and a string constant, each calling the next."""
    names = [f"M{k}.f" for k in range(12)]
    parts = ['@text = private constant [2 x i8] c"a\\00"\n']
    for k, name in enumerate(names):
        callee = names[(k + 1) % len(names)]
        parts.append(f"define i32 @{name}(i32 %x) {{\n  %y = call i32 @{callee}(i32 %x)\n"
                     f"  ret i32 {bodies.get(name, k)}\n}}\n")
    return "".join(parts)


class StablePartitionTests(unittest.TestCase):
    def test_a_body_edit_changes_one_shard(self):
        before = shard_ir.split_module(_module({}), 8, stable=True)
        after = shard_ir.split_module(_module({"M5.f": 99}), 8, stable=True)
        self.assertEqual(len(before), 8)
        self.assertEqual(sum(a != b for a, b in zip(before, after)), 1)
        # The balanced partition is still what an unmarked split gets.
        self.assertEqual(shard_ir.split_module(_module({}), 8),
                         shard_ir.split_module(_module({}), 8, stable=False))

    def test_module_level_content_has_its_own_shard(self):
        shards = shard_ir.split_module(_module({}), 8, stable=True)
        self.assertIn("@text = private constant", shards[0])
        self.assertNotIn("define", shards[0])
        defined = sorted(name for shard in shards for name in
                         (unit.name for unit in shard_ir.parse_module(shard).functions))
        self.assertEqual(defined, sorted(f"M{k}.f" for k in range(12)))

    def test_one_shard_cannot_be_stable(self):
        with self.assertRaisesRegex(shard_ir.ShardError, "at least two"):
            shard_ir._stable_assignment(shard_ir.parse_module(_module({})), 1)


class ObjectCacheTests(unittest.TestCase):
    def compile(self, directory, texts, cache_directory, flags=()):
        """Compile shard texts with a counting stand-in for clang."""
        clang = directory / "clang"
        clang.write_text("#!/bin/sh\n"
                         'if [ "$1" = --version ]; then echo fake clang 1; exit 0; fi\n'
                         f'echo run >> "{directory}/calls"\n'
                         'while [ "$1" != -c ]; do shift; done\n'
                         'cp "$2" "$4"\n')
        clang.chmod(0o755)
        shards, objects = [], []
        for k, text in enumerate(texts):
            shards.append(directory / f"s{k}.ll")
            shards[-1].write_text(text)
            objects.append(directory / f"s{k}.o")
            objects[-1].unlink(missing_ok=True)
        with patch.dict(os.environ, CINCH_SHARD_CACHE=str(cache_directory)):
            cache = shard_ir.object_cache(str(clang), tuple(flags))
        code, log = shard_ir.compile_shards(shards, objects, str(clang), 2, 2, directory,
                                            None, tuple(flags), cache)
        self.assertEqual(code, 0, log)
        calls = directory / "calls"
        count = len(calls.read_text().splitlines()) if calls.exists() else 0
        calls.unlink(missing_ok=True)
        return count, [obj.read_text() for obj in objects]

    def test_unchanged_shards_are_reused_and_changed_ones_compiled(self):
        with tempfile.TemporaryDirectory() as name:
            directory = Path(name)
            cache = directory / "cache"
            self.assertEqual(self.compile(directory, ["a", "b", "c"], cache), (3, ["a", "b", "c"]))
            self.assertEqual(self.compile(directory, ["a", "b", "c"], cache), (0, ["a", "b", "c"]))
            self.assertEqual(self.compile(directory, ["a", "B", "c"], cache), (1, ["a", "B", "c"]))
            # Different flags are a different object.
            self.assertEqual(self.compile(directory, ["a"], cache, ("-fno-inline",))[0], 1)

    def test_profile_compiles_never_use_the_cache(self):
        with tempfile.TemporaryDirectory() as name:
            directory = Path(name)
            flags = ("-fprofile-use=x.profdata",)
            self.assertEqual(self.compile(directory, ["a"], directory / "cache", flags)[0], 1)
            self.assertEqual(self.compile(directory, ["a"], directory / "cache", flags)[0], 1)
            self.assertFalse((directory / "cache").exists())

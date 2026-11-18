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

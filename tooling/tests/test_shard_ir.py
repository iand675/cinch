import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import shard_ir  # noqa: E402


class DeclarationTests(unittest.TestCase):
    def test_function_data_stays_with_the_definition(self):
        # An apply stub carries its tailcc twin's address in prefix data; a
        # declaration of the stub in another shard must not, or it names a
        # symbol that shard neither defines nor declares.
        define = ('define i64 @"X.app"(ptr %self, i64 %argc, ptr %argv) prefix <{ i64, i64 }> '
                  '<{ i64 4848492524724309041, i64 ptrtoint (ptr @"X.tapp" to i64) }> #3 {\n  ret i64 0\n}\n')
        self.assertEqual(shard_ir.declaration_for(define), 'declare i64 @"X.app"(ptr %self, i64 %argc, ptr %argv) #3\n')

    def test_linkage_is_removed_and_attributes_kept(self):
        self.assertEqual(shard_ir.declaration_for("define internal i64 @f(i64 %x) #1 {\n}\n"), "declare i64 @f(i64 %x) #1\n")


if __name__ == "__main__":
    unittest.main()

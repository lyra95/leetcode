import unittest


class Solution:
    def convert(self, s: str, numRows: int) -> str:
        if numRows == 1:
            return s
        n = (numRows - 1) * 2
        q = len(s)//n
        ans = ""
        for r in range(numRows):
            for k in range(q+2):
                ans += self.try_get(s, k*n + r)
                if r % (numRows-1) != 0:
                    ans += self.try_get(s, k*n + n - r)
        return ans

    @staticmethod
    def try_get(s: str, i: int) -> str:
        ans = ""
        if i < len(s):
            ans = s[i]
        return ans


class Test(unittest.TestCase):
    def test_smoke(self):
        self.assertEqual(
            Solution().convert("PAYPALISHIRING", 4),
            "PINALSIGYAHRPI")

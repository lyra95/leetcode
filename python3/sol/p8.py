import unittest
from enum import Enum


class Sign(Enum):
    PLUS = 1
    MINUS = 2


class Solution:
    def myAtoi(self, s: str) -> int:
        n = len(s)
        overflow = 2**31
        cur = Solution.trim_leading_whitespace(s)
        if cur >= n:
            return 0
        sign, cur = Solution.get_sign_and_move_cur(cur, s)
        if cur >= n:
            return 0
        cur = Solution.trim_leading_zeros(cur, s)
        if cur >= n:
            return 0
        digits = Solution.read_until_not_numeric(cur, s)

        result = Solution.add_up(digits)
        if sign == Sign.MINUS:
            if result > overflow:
                return -overflow
            result = -result
        if result >= overflow:
            return overflow - 1
        return result

    @staticmethod
    def trim_leading_whitespace(s: str) -> int:
        cur = 0
        for c in s:
            if c != ' ':
                break
            if c in "+-":
                break
            cur += 1

        return cur

    @staticmethod
    def get_sign_and_move_cur(cur: int, s: str):
        c = s[cur]
        if c == '+':
            return (Sign.PLUS, cur + 1)
        if c == '-':
            return (Sign.MINUS, cur + 1)
        return (Sign.PLUS, cur)

    @staticmethod
    def read_until_not_numeric(cur: int, s: str) -> list[int]:
        result = []
        for i in range(cur, len(s)):
            c = s[i]
            if c.isnumeric():
                result.append(int(c))
            else:
                break
            if len(result) > 10:
                break
        return result

    @staticmethod
    def trim_leading_zeros(cur: int, s: str) -> int:
        for i in range(cur, len(s)):
            if s[i] != '0':
                return i
        return len(s)

    @staticmethod
    def add_up(digits: list[int]) -> int:
        n = len(digits)
        acc = 0
        for i in range(n):
            acc += digits[n-1-i] * (10 ** i)
        return acc


class Test(unittest.TestCase):
    def test_trim(self):
        self.assertEqual(Solution.trim_leading_whitespace("    -"), 4)

    def test_get_sign_and_move_cur(self):
        self.assertEqual(
                Solution.get_sign_and_move_cur(4, "    -1"),
                (Sign.MINUS, 5))
        self.assertEqual(
                Solution.get_sign_and_move_cur(4, "    1"),
                (Sign.PLUS, 4))

    def test_all(self):
        s = Solution()
        self.assertEqual(s.myAtoi("    -1234"), -1234)
        self.assertEqual(s.myAtoi("    -0000asdf"), 0)

import unittest


class StringWithCursor:
    s: str
    cur: int
    is_plus: bool
    MAX = 2**31

    def __init__(self, s: str):
        self.s = s
        self.cur = 0
        self.is_plus = True

    def atoi(self):
        n = len(self.s)
        self.__trim_leading_whitespaces()
        if self.cur >= n:
            return 0
        self.__get_sign_and_move_cur()
        if self.cur >= n:
            return 0
        self.__trim_leading_zeros()
        if self.cur >= n:
            return 0
        digits = self.__read_until_not_numeric()
        result = self.__sum_up(digits)

        if self.is_plus and result > self.MAX - 1:
            return self.MAX - 1

        if not self.is_plus and result > self.MAX:
            return -self.MAX

        return - (-1) ** self.is_plus * result

    def __trim_leading_whitespaces(self):
        for c in self.s:
            if c == ' ':
                self.cur += 1
            else:
                return

    def __get_sign_and_move_cur(self):
        first = self.s[self.cur]
        if first == '-':
            self.is_plus = False
            self.cur += 1
            return
        if first == '+':
            self.cur += 1
            return

    def __trim_leading_zeros(self):
        for i in range(self.cur, len(self.s)):
            if self.s[i] != '0':
                self.cur = i
                return
        self.cur = len(self.s)

    def __read_until_not_numeric(self) -> list[int]:
        result = []
        for i in range(self.cur, len(self.s)):
            c = self.s[i]
            if c.isnumeric():
                result.append(int(c))
            else:
                break
            if len(result) > 10:
                break
        return result

    @staticmethod
    def __sum_up(digits: list[int]):
        n = len(digits)
        acc = 0
        for i in range(n):
            acc += digits[n-1-i] * (10 ** i)
        return acc


class Solution:
    def myAtoi(self, s: str) -> int:
        x = StringWithCursor(s)
        return x.atoi()


class Test(unittest.TestCase):
    def test_all(self):
        s = Solution()
        self.assertEqual(-1234, s.myAtoi("    -1234"))
        self.assertEqual(0, s.myAtoi("    -0000asdf"))

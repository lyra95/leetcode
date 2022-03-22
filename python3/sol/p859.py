import unittest


class Solution:
    @staticmethod
    def has_duplicate(s: str) -> bool:
        chars = {}
        for c in s:
            chars[c] = chars.setdefault(c, 0) + 1
        return any(v > 1 for v in chars.values())

    def buddyStrings(self, s: str, goal: str) -> bool:
        n = len(s)
        if len(goal) != n:
            return False
        count = 0
        indices = [-1, -1]
        for i in range(n):
            if s[i] != goal[i]:
                if count > 1:
                    return False
                indices[count] = i
                count += 1
        if count == 0:
            return Solution.has_duplicate(s)
        if count == 1:
            return False
        i, j = indices[0], indices[1]
        if s[i] == goal[j] and s[j] == goal[i]:
            return True
        return False


class Test(unittest.TestCase):
    def test_true(self):
        sol = Solution()
        self.assertTrue(sol.buddyStrings("ab", "ba"))
        self.assertTrue(sol.buddyStrings("aa", "aa"))

    def test_false(self):
        sol = Solution()
        self.assertFalse(sol.buddyStrings("ab", "ab"))

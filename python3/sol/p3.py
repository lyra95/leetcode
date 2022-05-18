import unittest


class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        n = len(s)
        if n <= 1:
            return n
        window: set[str] = set(s[0])
        start = 0
        end = 1

        ans = 0
        while end < n:
            if s[end] not in window:
                window.add(s[end])
                end += 1
                continue
            ans = max(ans, len(window))
            while s[start] != s[end]:
                window.remove(s[start])
                start += 1
            start += 1
            end += 1

        return max(ans, len(window))


class Test(unittest.TestCase):
    s = Solution()

    def test_it_works(self):
        self.assertEqual(self.s.lengthOfLongestSubstring("abcabcbb"), 3)
        self.assertEqual(self.s.lengthOfLongestSubstring("bbbbb"), 1)
        self.assertEqual(self.s.lengthOfLongestSubstring("pwwkew"), 3)

    def test_edge_case(self):
        self.assertEqual(self.s.lengthOfLongestSubstring(" "), 1)

    def test_failed(self):
        self.assertEqual(self.s.lengthOfLongestSubstring("au"), 2)

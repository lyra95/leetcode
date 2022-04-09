import unittest


class Solution:

    def longestCommonPrefix(self, strs: list[str]) -> str:
        if len(strs) == 0:
            return ""
        answer = strs[0]
        for word in strs:
            answer = Solution.__get_common_prefix(answer, word)
        return answer

    @staticmethod
    def __get_common_prefix(word1: str, word2: str):
        common = ""
        n = min(len(word1), len(word2))
        for i in range(n):
            if word1[i] == word2[i]:
                common += word1[i]
            else:
                break
        return common


class Test(unittest.TestCase):

    def test_should_pass(self):
        strs1 = ["flower", "flow", "flight"]
        strs2 = ["dog", "racecar", "car"]
        sol = Solution()
        result1 = sol.longestCommonPrefix(strs1)
        result2 = sol.longestCommonPrefix(strs2)
        self.assertEqual("fl", result1)
        self.assertEqual("", result2)

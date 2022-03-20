import unittest

class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        visited = {}
        for i, cur in enumerate(nums):
            extra = target - cur

            if extra in visited:
                return [i, visited[extra]]
            
            visited[cur] = i
        raise ValueError

    def _twoSum(self, nums: list[int], target: int) -> list[int]:
        n = len(nums)
        for i in range(n):
            for j in range(i+1,n):
                sum = nums[i]+nums[j]
                if sum==target:
                    return [i,j]
        raise ValueError


class Test(unittest.TestCase):
    
    def test_should_pass(self):
        nums = [1,2,3,4]
        target = 7

        sol = Solution()
        result = sol.twoSum(nums, target)
        self.assertEqual([2,3], sorted(result))
    

    def test_should_raise(self):
        nums = [1,2,3,4]
        target = 8

        sol = Solution()
        self.assertRaises(ValueError, lambda: sol.twoSum(nums, target))


if __name__ == '__main__':
    unittest.main()

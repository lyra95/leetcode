using FluentAssertions;
using Xunit;

namespace csharp;

[LeetCodeMetadata(Number = 46, Title = "Permutations", Url = "https://leetcode.com/problems/permutations/")]
public class Permutations
{
    public IList<IList<int>> Permute(int[] nums)
    {
        return GetEnumerable(nums.ToList()).ToList();
    }

    private IEnumerable<IList<int>> GetEnumerable(IList<int> current, int pos = 0)
    {
        var n = current.Count;
        
        if (pos >= n)
        {
            yield return current;
            yield break;
        }
        
        for (var i = pos; i < n; i++)
        {
            var child = GetEnumerable(Swap(current, pos, i), pos + 1);
            foreach (var leaf in child)
            {
                yield return leaf;
            }
        }
    }

    private IList<int> Swap(IList<int> list, int i, int j)
    {
        var result = list.ToList();
        
        if (i == j)
        {
            return result;
        }

        (result[i], result[j]) = (result[j], result[i]);
        return result.ToList();
    }

    [Fact]
    public void SmokeTest()
    {
        var x = GetEnumerable(new List<int>(){ 4, 3, 2, 1 }).ToList();
        x.Count().Should().Be(24);
    }
}

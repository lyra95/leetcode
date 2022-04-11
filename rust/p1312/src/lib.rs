pub struct Solution {}
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let n = s.len();

        if n <= 1 {
            return 0;
        }

        let reversed: Vec<u8> = s.as_bytes().iter().rev().map(|c| *c).collect();
        let memo = Solution::lcs(s.as_bytes(), &reversed);
        let pos = Solution::backtracking(&memo);
        let mut count = 0;
        for (i, j) in pos {
            if i + j == n + 1 {
                count += 1;
                return n as i32 - 1 - 2 * (count - 1);
            }
            if i + j > n + 1 {
                return n as i32 - 2 * count;
            }
            count += 1;
        }
        panic!();
    }

    pub fn lcs(a: &[u8], b: &[u8]) -> Vec<Vec<i32>> {
        let mut memoization: Vec<Vec<i32>> = Vec::new();
        for _ in 0..a.len() + 1 {
            memoization.push(vec![-1; b.len() + 1]);
        }

        Solution::lcs_recursive(a.len(), b.len(), &mut memoization, a, b);
        memoization
    }

    fn lcs_recursive(
        i: usize,
        j: usize,
        memoization: &mut Vec<Vec<i32>>,
        left: &[u8],
        right: &[u8],
    ) -> i32 {
        if memoization[i][j] != -1 {
            return memoization[i][j];
        }
        if i == 0 || j == 0 {
            memoization[i][j] = 0;
            return 0;
        }

        if left[i - 1] == right[j - 1] {
            let temp = Solution::lcs_recursive(i - 1, j - 1, memoization, left, right) + 1;
            memoization[i][j] = temp;
            return temp;
        }

        let temp1 = Solution::lcs_recursive(i, j - 1, memoization, left, right);
        let temp2 = Solution::lcs_recursive(i - 1, j, memoization, left, right);
        memoization[i][j] = temp1.max(temp2);
        memoization[i][j]
    }

    fn backtracking(memo: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
        let n = memo.len();
        let m = memo[0].len();
        let mut i = n - 1;
        let mut j = m - 1;
        let mut cur = memo[n - 1][m - 1];

        let mut pos: Vec<(usize, usize)> = Vec::new();
        while cur > 0 {
            if memo[i - 1][j - 1] == cur {
                i -= 1;
                j -= 1;
                continue;
            }

            if memo[i - 1][j] == cur {
                i -= 1;
                continue;
            }

            if memo[i][j - 1] == cur {
                j -= 1;
                continue;
            }

            if memo[i - 1][j - 1] + 1 == cur {
                pos.push((i, j));
                i -= 1;
                j -= 1;
                cur -= 1;
                continue;
            }
        }
        pos.reverse();
        return pos;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::min_insertions(String::from("zzazz")), 0);
        assert_eq!(Solution::min_insertions(String::from("mbadm")), 2);
        assert_eq!(Solution::min_insertions(String::from("leetcode")), 5);
        assert_eq!(Solution::min_insertions(String::from("zjveiiwvc")), 5);
    }
}

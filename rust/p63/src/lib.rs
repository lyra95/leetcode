pub struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid.first().unwrap().len();
        let mut memo = vec![vec![0; n]; m];

        Solution::dynamic(&obstacle_grid, &mut memo, m - 1, n - 1)
    }

    fn dynamic(grid: &[Vec<i32>], memo: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        if i == 0 && j == 0 {
            return 1 - grid[0][0];
        }

        if grid[i][j] == 1 {
            return 0;
        }

        if memo[i][j] != 0 {
            return memo[i][j];
        }

        if i == 0 {
            memo[i][j] = Solution::dynamic(grid, memo, 0, j - 1);
        } else if j == 0 {
            memo[i][j] = Solution::dynamic(grid, memo, i - 1, 0);
        } else {
            memo[i][j] =
                Solution::dynamic(grid, memo, i - 1, j) + Solution::dynamic(grid, memo, i, j - 1);
        }

        memo[i][j]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 2);
    }
}

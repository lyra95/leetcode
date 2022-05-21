pub struct Solution {}

#[derive(Clone, PartialEq, Copy)]
enum Status {
    Unvisited,
    Unreachable,
    Visited(i32),
}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut memo: Vec<Status> = vec![Status::Unvisited; amount as usize + 1];
        let ans = Solution::dynamic(&coins, &mut memo, amount as usize);
        if let Status::Visited(v) = ans {
            return v;
        }
        -1
    }

    fn dynamic(coins: &[i32], memo: &mut Vec<Status>, amount: usize) -> Status {
        if memo[amount] != Status::Unvisited {
            return memo[amount];
        }

        if amount == 0 {
            memo[amount] = Status::Visited(0);
            return memo[amount];
        }

        if coins.contains(&(amount as i32)) {
            memo[amount] = Status::Visited(1);
            return memo[amount];
        }

        let mut min = Status::Unreachable;
        for coin in coins {
            let coin = *coin as usize;
            if amount < coin {
                continue;
            }
            let prev = Solution::dynamic(coins, memo, amount - coin);
            if let Status::Visited(v) = prev {
                if let Status::Visited(w) = min {
                    min = Status::Visited(i32::min(v, w));
                } else {
                    min = prev;
                }
            }
        }

        if let Status::Visited(w) = min {
            min = Status::Visited(w + 1);
        }

        memo[amount] = min;
        min
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
}

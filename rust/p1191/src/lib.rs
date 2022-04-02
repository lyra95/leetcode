pub struct Solution {}

impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return Solution::max_subarray_sum(arr.iter());
        }
        let sum = arr.iter().sum();
        let doubled = arr.iter().chain(arr.iter());
        Solution::handle_overflow(
            std::cmp::max(sum, 0),
            k - 2,
            Solution::max_subarray_sum(doubled),
        )
    }

    fn max_subarray_sum<'a>(iter: impl Iterator<Item = &'a i32>) -> i32 {
        let mut max = 0;
        let mut cur = 0;
        for ele in iter {
            cur += ele;
            if cur > max {
                max = cur;
            }
            if cur < 0 {
                cur = 0;
            }
        }
        max
    }

    fn handle_overflow(a: i32, b: i32, c: i32) -> i32 {
        let mut result = 0;
        for _ in 0..b {
            result = (result + a) % 1000000007;
        }

        (result + c) % 1000000007
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(9, Solution::k_concatenation_max_sum(vec![1, 2], 3));
        assert_eq!(2, Solution::k_concatenation_max_sum(vec![1, -2, 1], 5));
        assert_eq!(0, Solution::k_concatenation_max_sum(vec![-1, -2], 7));
    }

    #[test]
    fn overflow() {
        assert_eq!(
            999999937,
            Solution::k_concatenation_max_sum(
                vec![10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000],
                100000
            )
        );
    }
}

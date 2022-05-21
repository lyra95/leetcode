pub struct Solution {}

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut check = false;
        let mut i = 0;
        while i < nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                if check {
                    return false;
                }
                if Solution::is_ok(&nums, i) {
                    check = true;
                    i += 1;
                    continue;
                }
                if Solution::is_ok(&nums, i + 1) {
                    check = true;
                    i += 2;
                    continue;
                }
                return false;
            } else {
                i += 1;
            }
        }
        true
    }

    fn is_ok(nums: &[i32], j: usize) -> bool {
        if j == 0 {
            return true;
        }
        !Solution::left_is_bigger(nums.get(j - 1), nums.get(j + 1))
    }

    fn left_is_bigger(x: Option<&i32>, y: Option<&i32>) -> bool {
        if x.is_none() || y.is_none() {
            return false;
        }

        let x_ = x.unwrap();
        let y_ = y.unwrap();

        *x_ > *y_
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
        assert_eq!(Solution::check_possibility(vec![1, 2, 4, 5, 3]), true);
        assert_eq!(Solution::check_possibility(vec![5, 7, 1, 8]), true);
    }
}

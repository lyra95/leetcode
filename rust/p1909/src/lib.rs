pub struct Solution {}

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        assert!(nums.len() > 1);
        let mut i = 0;
        let mut j = 1;

        let mut padded = vec![0];
        padded.extend(nums.iter());
        let mut removed = false;
        while j < padded.len() {
            if padded[i] < padded[j] {
                i = j;
                j += 1;
                continue;
            }
            if padded[i] >= padded[j] {
                if removed {
                    return false;
                }

                removed = true;
                if padded[i - 1] < padded[j] {
                    i = j;
                    j += 1;
                    continue;
                }

                j += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn should_true() {
        assert!(Solution::can_be_increasing(vec![100, 21, 100]));
        assert!(Solution::can_be_increasing(vec![105, 924, 32, 968]));
        assert!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]));
    }

    #[test]
    fn should_false() {
        assert!(!Solution::can_be_increasing(vec![2, 3, 1, 2]));
        assert!(!Solution::can_be_increasing(vec![1, 1, 1]));
    }
}

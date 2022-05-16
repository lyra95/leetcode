use crate::IsOkayResult::{MoreToLeft, MoreToRight, Okay};

pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len() + nums2.len();
        let f = Finder {
            sorted_first: nums1,
            sorted_second: nums2,
        };
        let mid = n / 2;
        if n % 2 == 0 {
            let a = f.find_smallest_k_th(mid - 1).unwrap();
            let b = f.find_smallest_k_th(mid).unwrap();
            return (a + b) as f64 / 2f64;
        }
        f.find_smallest_k_th(mid).unwrap() as f64
    }
}

pub struct Finder {
    sorted_first: Vec<i32>,
    sorted_second: Vec<i32>,
}

impl Finder {
    pub fn find_smallest_k_th(&self, k: usize) -> Option<i32> {
        if let Some(v) = Self::try_first(
            &self.sorted_first,
            &self.sorted_second,
            k,
            0,
            k as isize + 1,
        ) {
            Some(v)
        } else {
            Self::try_first(
                &self.sorted_second,
                &self.sorted_first,
                k,
                0,
                k as isize + 1,
            )
        }
    }

    fn try_first(
        sorted_first: &[i32],
        sorted_second: &[i32],
        k: usize,
        start: isize,
        end: isize,
    ) -> Option<i32> {
        let diff = end - start;
        if diff <= 0 {
            return None;
        }

        let first = diff / 2 + start;
        let second = k as isize - 1 - first;

        match Self::is_okay(sorted_first, sorted_second, first, second) {
            Okay(v) => Some(v),
            MoreToLeft => {
                if diff == 1 {
                    None
                } else {
                    Self::try_first(sorted_first, sorted_second, k, start, first)
                }
            }
            MoreToRight => {
                if diff == 1 {
                    None
                } else {
                    Self::try_first(sorted_first, sorted_second, k, first, end)
                }
            }
        }
    }

    fn is_okay(
        sorted_first: &[i32],
        sorted_second: &[i32],
        first: isize,
        second: isize,
    ) -> IsOkayResult {
        let v1 = Self::try_get(sorted_first, first);
        let v2 = Self::try_get(sorted_second, second);
        let v3 = Self::try_get(sorted_second, second + 1);

        if v2 > v1 {
            MoreToRight
        } else if v1 > v3 {
            MoreToLeft
        } else {
            // v2 <= v1 <= v3
            Okay(v1)
        }
    }

    fn try_get(nums: &[i32], idx: isize) -> i32 {
        if idx < 0 {
            return Finder::MIN;
        }

        return if let Some(v) = nums.get(idx as usize) {
            *v
        } else {
            Finder::MAX
        };
    }

    const MAX: i32 = 10i32.pow(6);
    const MIN: i32 = -(10i32.pow(6));
}

enum IsOkayResult {
    Okay(i32),
    MoreToLeft,
    MoreToRight,
}

#[cfg(test)]
mod tests {
    use crate::{Finder, Solution};

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2f64
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![99]),
            3f64
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![-1, 3]),
            1.5f64
        );
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![]), 1f64);
    }

    #[test]
    fn finder() {
        let f = Finder {
            sorted_first: vec![1, 3],
            sorted_second: vec![2, 4],
        };

        assert_eq!(f.find_smallest_k_th(0), Some(1));
        assert_eq!(f.find_smallest_k_th(1), Some(2));
        assert_eq!(f.find_smallest_k_th(2), Some(3));
    }
}

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            let n_area = (right - left) as i32 * std::cmp::min(height[left], height[right]);
            area = std::cmp::max(area, n_area);
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
    }
}

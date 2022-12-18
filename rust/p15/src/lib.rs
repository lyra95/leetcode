use std::collections::HashSet;

pub struct Solution {}

#[derive(Eq, PartialEq, Hash)]
struct OrderedTuple(i32, i32);

impl OrderedTuple {
    pub fn new(a: i32, b: i32) -> OrderedTuple {
        if a < b {
            return OrderedTuple(a, b);
        }

        OrderedTuple(b, a)
    }
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return Vec::new();
        }

        let mut result: HashSet<OrderedTuple> = HashSet::new();

        let mut nums = nums;
        let n = nums.len();
        nums.sort();
        let max = nums.last().unwrap();
        for i in 0..n {
            for j in i + 1..n {
                let first = nums[i];
                let second = nums[j];
                let third = -first - second;

                if third < second {
                    continue;
                }

                if third > *max {
                    continue;
                }

                let t = OrderedTuple::new(first, second);
                if result.contains(&t) {
                    continue;
                }

                if nums[j + 1..n].binary_search(&third).is_ok() {
                    result.insert(t);
                }
            }
        }

        result
            .into_iter()
            .map(|t| vec![t.0, t.1, -t.0 - t.1])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::three_sum(vec![1, 2, -2, -1]).is_empty());
    }
}

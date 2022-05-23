use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mapper = |x: &String| {
            let mut zeros = 0;
            let mut ones = 0;
            for c in x.chars() {
                match c {
                    '0' => zeros += 1,
                    '1' => ones += 1,
                    _ => continue,
                }
            }

            (zeros, ones)
        };

        let mut bins: Vec<(i32, i32)> = strs.iter().clone().map(mapper).collect();

        let compare_zero = |a: &(i32, i32), b: &(i32, i32)| match (a.0 + a.1).cmp(&(b.0 + b.1)) {
            Ordering::Equal => a.0.cmp(&b.0),
            order => order,
        };

        let compare_one = |a: &(i32, i32), b: &(i32, i32)| match (a.0 + a.1).cmp(&(b.0 + b.1)) {
            Ordering::Equal => a.1.cmp(&b.1),
            order => order,
        };

        if m >= n {
            bins.sort_unstable_by(compare_zero);
        } else {
            bins.sort_unstable_by(compare_one);
        }
        let mut count = 0;
        let mut cur = (0, 0);
        while let Some(e) = bins.get(count) {
            let n_cur = (cur.0+e.0,cur.1+e.1);
            if n_cur.0 > m || n_cur.1 > n {
                break;
            }
            count += 1;
            cur.0 += e.0;
            cur.1 += e.1;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let strs = ["111","1000","1000","1000"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::find_max_form(strs, 9, 3), 3);
    }
}

pub struct Solution {}

const OVERFLOW: i64 = 2_i64.pow(31);
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let trimmed = s.trim_start();
        if trimmed.is_empty() {
            return 0;
        }

        let mut is_negative = false;
        let mut slice = trimmed;
        if trimmed.starts_with('-') {
            is_negative = true;
            slice = trimmed.strip_prefix('-').unwrap();
        } else if trimmed.starts_with('+') {
            slice = trimmed.strip_prefix('+').unwrap();
        }

        slice = slice.trim_start_matches('0');
        let mut digits = Vec::new();
        for c in slice.chars() {
            if !c.is_numeric() {
                break;
            }
            digits.push(c.to_digit(10).unwrap());
            if digits.len() > 10 {
                break;
            }
        }

        let n = digits.len();
        let mut acc = 0;
        for (i, d) in digits.iter().enumerate() {
            acc += (*d as i64) * 10_i64.pow((n - i - 1) as u32);
        }

        if is_negative && acc >= OVERFLOW {
            return -OVERFLOW as i32;
        }

        if !is_negative && acc >= OVERFLOW {
            return (OVERFLOW - 1) as i32;
        }

        if is_negative {
            return -acc as i32;
        }

        acc as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::my_atoi(String::from("42")), 42);
    }
}

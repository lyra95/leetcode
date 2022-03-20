pub struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }
        
        let v = Solution::to_reversed_vector(x);
        
        Solution::is_palindrome_as_vec(&v)
    }

    pub fn to_reversed_vector(mut x: i32) -> Vec<i32> {
        if x < 0 {
            panic!("x must be non-negative")
        }

        let mut v: Vec<i32> = Vec::new();
        while x >= 10 {
            let d = x / 10;
            let r = x - d * 10;
            v.push(r);
            x = d;
        };

        v.push(x);
        v
    }
    
    pub fn is_palindrome_as_vec(v : &[i32]) -> bool {
        let n = v.len();
        for i in 0..n {
            let j = n - 1 - i;
            if j <= i {
                break;
            }

            if v[i] != v[j] {
                return false
            }
        }
        true
    }

    pub fn _is_palindrome(x: i32) -> bool {
        let s: String = x.to_string();
        s.chars().rev().collect::<String>() == s
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_convert_to_vec() {
        let x = 123;
        let expected = vec![3, 2, 1];

        assert_eq!(expected, Solution::to_reversed_vector(x))
    }

    #[test]
    fn should_convert_to_vec_when_zero() {
        let x = 0;
        let expected = vec![0];

        assert_eq!(expected, Solution::to_reversed_vector(x))
    }

    #[test]
    #[should_panic]
    fn should_panic_when_negative() {
        let x = -1;
        Solution::to_reversed_vector(x);
    }

    #[test]
    fn should_true_on_palindrome() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn should_false_on_palindrome() {
        assert!(!Solution::is_palindrome(-121));
    }
}

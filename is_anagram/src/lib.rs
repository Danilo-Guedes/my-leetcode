pub struct Solution;

const MAX_LETTERS_SIZE: usize = 26;

impl Solution {
    pub fn is_anagram(first: String, second: String) -> bool {
        if first.len() != second.len() {
            return false;
        }
        let mut counts = [0i32; MAX_LETTERS_SIZE]; // on the stack, no heap allocation

        for (a, b) in first.bytes().zip(second.bytes()) {
            counts[(a - b'a') as usize] += 1;
            counts[(b - b'a') as usize] -= 1;
        }

        !counts.iter().any(|n| *n != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anagram_found_returns_true() {
        let str1 = "racecar".to_string();
        let str2 = "carrace".to_string();

        assert_eq!(Solution::is_anagram(str1, str2), true);
    }

    #[test]
    fn anagram_found_single_letter_returns_true() {
        let str1 = "x".to_string();
        let str2 = "x".to_string();

        assert_eq!(Solution::is_anagram(str1, str2), true);
    }

    #[test]
    fn anagram_not_found_returns_false() {
        let str1 = "jar".to_string();
        let str2 = "jam".to_string();

        assert_eq!(Solution::is_anagram(str1, str2), false);
    }
}

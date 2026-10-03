use std::collections::HashMap;

pub struct Solution;

const MAX_LETTERS_SIZE: usize = 26;

impl Solution {
    pub fn is_anagram(first: String, second: String) -> bool {
        if first.len() != second.len() {
            return false;
        }
        let mut first_hash = HashMap::with_capacity(MAX_LETTERS_SIZE);
        let mut second_hash = HashMap::with_capacity(MAX_LETTERS_SIZE);

        for ch in first.chars() {
            first_hash
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        for ch in second.chars() {
            second_hash
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        first_hash == second_hash
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

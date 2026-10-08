use std::collections::HashMap;

pub struct Solution;

const MAX_LETTERS_SIZE: usize = 26;

impl Solution {
    pub fn is_anagram_sort(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        s_chars.sort();

        let mut t_chars: Vec<char> = t.chars().collect();
        t_chars.sort();

        s_chars == t_chars
    }

    pub fn is_anagram_hashmap(first: String, second: String) -> bool {
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

    pub fn is_anagram_array(first: String, second: String) -> bool {
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
    fn anagram_found_returns_true_sort() {
        let str1 = "racecar".to_string();
        let str2 = "carrace".to_string();

        assert_eq!(Solution::is_anagram_sort(str1, str2), true);
    }

    #[test]
    fn anagram_found_single_letter_returns_true_sort() {
        let str1 = "x".to_string();
        let str2 = "x".to_string();

        assert_eq!(Solution::is_anagram_sort(str1, str2), true);
    }

    #[test]
    fn anagram_not_found_returns_false_sort() {
        let str1 = "jar".to_string();
        let str2 = "jam".to_string();

        assert_eq!(Solution::is_anagram_sort(str1, str2), false);
    }

    #[test]
    fn anagram_found_returns_true_hashmap() {
        let str1 = "racecar".to_string();
        let str2 = "carrace".to_string();

        assert_eq!(Solution::is_anagram_hashmap(str1, str2), true);
    }

    #[test]
    fn anagram_found_single_letter_returns_true_hashmap() {
        let str1 = "x".to_string();
        let str2 = "x".to_string();

        assert_eq!(Solution::is_anagram_hashmap(str1, str2), true);
    }

    #[test]
    fn anagram_not_found_returns_false_hashmap() {
        let str1 = "jar".to_string();
        let str2 = "jam".to_string();

        assert_eq!(Solution::is_anagram_hashmap(str1, str2), false);
    }

    #[test]
    fn anagram_found_returns_true_array() {
        let str1 = "racecar".to_string();
        let str2 = "carrace".to_string();

        assert_eq!(Solution::is_anagram_array(str1, str2), true);
    }

    #[test]
    fn anagram_found_single_letter_returns_true_array() {
        let str1 = "x".to_string();
        let str2 = "x".to_string();

        assert_eq!(Solution::is_anagram_array(str1, str2), true);
    }

    #[test]
    fn anagram_not_found_returns_false_array() {
        let str1 = "jar".to_string();
        let str2 = "jam".to_string();

        assert_eq!(Solution::is_anagram_array(str1, str2), false);
    }
}

use std::collections::HashMap;

#[allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        s_chars.sort();

        let mut t_chars: Vec<char> = t.chars().collect();
        t_chars.sort();

        s_chars == t_chars
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

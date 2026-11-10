// You are given two strings s1 and s2.
//
// Return true if s2 contains a permutation of s1, or false otherwise. That means if a permutation of s1 exists as a substring of s2, then return true.
//
// Both strings only contain lowercase letters.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s2_chars: Vec<char> = s2.chars().collect();

        let mut s1_hashmap: HashMap<char, i32> = HashMap::new();

        for letter in s1.chars() {
            *s1_hashmap.entry(letter).or_insert(0) += 1;
        }

        let mut s2_hashmap: HashMap<char, i32> = HashMap::new();

        let mut left = 0;

        for (right, &letter) in s2_chars.iter().enumerate() {
            *s2_hashmap.entry(letter).or_insert(0) += 1;

            let current_window = right - left + 1;

            if current_window > s1.len() {
                let letter_in_window = s2_hashmap.get_mut(&s2_chars[left]).unwrap();

                if *letter_in_window > 1 {
                    *letter_in_window -= 1;
                } else {
                    s2_hashmap.remove(&s2_chars[left]);
                }
                left += 1
            }

            if s1_hashmap == s2_hashmap {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = "abc".to_string();
        let s2 = "lecabee".to_string();
        let result = Solution::check_inclusion(input, s2);
        assert_eq!(result, true);
    }

    #[test]
    fn example_two() {
        let input = "abc".to_string();
        let s2 = "lecaabee".to_string();
        let result = Solution::check_inclusion(input, s2);
        assert_eq!(result, false);
    }
}

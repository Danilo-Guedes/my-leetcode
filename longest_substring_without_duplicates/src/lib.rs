// Given a string s, find the length of the longest substring without duplicate characters.
//
// A substring is a contiguous sequence of characters within a string.
//
// Example 1:
//
// Input: s = "zxyzxyz"
//
// Output: 3
//
// Explanation: The string "xyz" is the longest without duplicate characters.
//
// Example 2:
//
// Input: s = "xxxx"
//
// Output: 1
//
// Constraints:
//
// 0 <= s.length <= 1000
// s may consist of printable ASCII characters.

use std::cmp::max;
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut hashset = HashSet::new();

        let mut left = 0;
        let mut longest = 0;

        for (right, &letter) in chars.iter().enumerate() {
            // shrink the window from the left until `letter` is no longer in it
            while hashset.contains(&letter) {
                hashset.remove(&chars[left]);
                left += 1;
            }

            hashset.insert(letter);
            longest = max(longest, right - left + 1);
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = "zxyzxyz".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let input = "xxxx".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_three() {
        let input = "pwwkew".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_four() {
        let input = "abc".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_five() {
        let input = "aab".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_six() {
        let input = "dvdf".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_seven() {
        let input = "abcad".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_eight() {
        let input = "abba".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn empty_string() {
        let input = "".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 0);
    }
}

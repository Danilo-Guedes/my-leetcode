// You are given a string s consisting of only uppercase english characters and an integer k.
// You can choose up to k characters of the string and replace them with any other uppercase
// English character.
//
// After performing at most k replacements, return the length of the longest substring which
// contains only one distinct character.
//
// Example 1:
//
// Input: s = "XYYX", k = 2
//
// Output: 4
//
// Explanation: Either replace the 'X's with 'Y's, or replace the 'Y's with 'X's.
//
// Example 2:
//
// Input: s = "AAABABB", k = 1
//
// Output: 5
//
// Constraints:
//
// 1 <= s.length <= 1000
// 0 <= k <= s.length

use std::{cmp::max, collections::HashMap};

pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut hashmap: HashMap<char, i32> = HashMap::new();

        let mut left = 0;
        let mut longest = 0;

        for (right, &letter) in chars.iter().enumerate() {
            *hashmap.entry(letter).or_insert(0) += 1;

            // while (right - left + 1) as i32 - *hashmap.values().max().unwrap_or(&0) > k {
            //     *hashmap.get_mut(&chars[left]).unwrap() -= 1;
            //     left += 1;
            // }

            // SAME LOGIC AS ABOVE BUIT WRITE DIFFERENTLY
            loop {
                let window_len = (right - left + 1) as i32;
                let max_freq = *hashmap.values().max().unwrap_or(&0);
                if window_len - max_freq <= k {
                    break;
                }
                *hashmap.get_mut(&chars[left]).unwrap() -= 1;
                left += 1;
            }

            longest = max(longest, (right - left + 1) as i32)
        }

        longest
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = "XYYX".to_string();
        let k = 2;
        let result = Solution::character_replacement(input, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let input = "AAABABB".to_string();
        let k = 1;
        let result = Solution::character_replacement(input, k);
        assert_eq!(result, 5);
    }
}

// Given a string s, return true if it is a palindrome, otherwise return false.
//
// A palindrome is a string that reads the same forward and backward. It is also
// case-insensitive and ignores all non-alphanumeric characters.
//
// Example 1:
//
// Input: s = "Was it a car or a cat I saw?"
//
// Output: true
//
// Explanation: After considering only alphanumerical characters we have "wasitacaroracatisaw",
// which is a palindrome.
//
// Example 2:
//
// Input: s = "tab a cat"
//
// Output: false
//
// Explanation: "tabacat" is not a palindrome.
//
// Constraints:
//
// 1 <= s.length <= 1000
// s is made up of only printable ASCII characters.

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<char>>();

        let mut ending_idx = chars.len().saturating_sub(1);

        for letter in chars.iter() {
            if *letter == chars[ending_idx] {
                ending_idx = ending_idx.saturating_sub(1);
                continue;
            } else {
                return false;
            }
        }

        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_one() {
        let input = String::from("Was it a car or a cat I saw?");
        let result = Solution::is_palindrome(input);
        assert_eq!(result, true);
    }

    #[test]
    fn it_works_two() {
        let input = String::from("tab a cat");
        let result = Solution::is_palindrome(input);
        assert_eq!(result, false);
    }

    #[test]
    fn empty_after_filter_is_palindrome() {
        assert_eq!(Solution::is_palindrome(String::from(" ")), true);
    }
}

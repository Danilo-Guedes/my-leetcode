//! # Valid Anagram (LeetCode 242 / NeetCode 150)
//!
//! Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`:
//! same letters, same number of times each, in any order.
//!
//! Three versions were written, each one improving on the previous one.
//!
//! ## v1: sort and compare — O(n log n) time, O(n) space
//!
//! Sort the characters of both strings. Anagrams end up as the same sequence.
//! `Vec`'s `==` also checks the length, so no manual check is needed.
//!
//! ```rust,ignore
//! pub fn is_anagram(s: String, t: String) -> bool {
//!     let mut s_chars: Vec<char> = s.chars().collect();
//!     s_chars.sort();
//!
//!     let mut t_chars: Vec<char> = t.chars().collect();
//!     t_chars.sort();
//!
//!     s_chars == t_chars
//! }
//! ```
//!
//! Notes:
//! - `String` has no `sort`, because sorting UTF-8 bytes in place could break characters.
//!   Collect into a `Vec` first.
//! - `sort()` works in place and returns `()`, so the `Vec` must be `mut`.
//! - A hand-written `zip` loop stops at the shorter list, so `"ab"` vs `"abc"` would
//!   wrongly match. `==` doesn't have that problem.
//!
//! ## v2: count with HashMaps — O(n) time, O(1) space (at most 26 keys)
//!
//! Count how many times each character appears in each string, then compare the maps.
//!
//! ```rust,ignore
//! pub fn is_anagram(first: String, second: String) -> bool {
//!     if first.len() != second.len() {
//!         return false;
//!     }
//!     let mut first_hash = HashMap::with_capacity(MAX_LETTERS_SIZE);
//!     let mut second_hash = HashMap::with_capacity(MAX_LETTERS_SIZE);
//!
//!     for ch in first.chars() {
//!         first_hash
//!             .entry(ch)
//!             .and_modify(|counter| *counter += 1)
//!             .or_insert(1);
//!     }
//!
//!     for ch in second.chars() {
//!         second_hash
//!             .entry(ch)
//!             .and_modify(|counter| *counter += 1)
//!             .or_insert(1);
//!     }
//!
//!     first_hash == second_hash
//! }
//! ```
//!
//! Notes:
//! - `HashSet` doesn't work here: it only stores keys, and counting needs key -> count.
//! - Entry API: `*map.entry(k).or_insert(0) += 1` does the same as `and_modify` + `or_insert(1)`.
//! - `with_capacity(26)` allocates once. `new()` grows step by step (3 -> 7 -> 14 -> 28),
//!   rehashing each time.
//! - It's O(n), but it wasn't faster than v1 on LeetCode. Each operation pays for hashing
//!   (SipHash) and probing, while sorting runs over contiguous, cache-friendly memory.
//!   Big O describes how cost scales, not which version is faster at a given size.
//!   LeetCode times are also noisy: the same code got 6ms and then 4ms.
//!
//! ## v3: one `[i32; 26]` counter over bytes — O(n) time, O(1) space (current)
//!
//! One pass over both strings together: +1 for each letter of `first`, -1 for each letter
//! of `second`. They're anagrams if every counter ends at 0.
//!
//! Notes:
//! - `.bytes()` gives each letter as a `u8` (fine because the input is lowercase ASCII).
//!   Avoid `ch as u8`: it silently cuts off non-ASCII chars.
//! - `b'a'` is the byte 97. `(x - b'a') as usize` maps `'a'..='z'` to indexes `0..26`.
//!   Always subtract `b'a'` (the letter), not the variable's name, e.g. `b'b'` is 98.
//! - The counter must be signed (`i32`): it can go negative in the middle of the loop.
//! - The length check matters because `zip` stops at the shorter string.
//! - No heap allocation, no hashing: the array lives on the stack.

pub struct Solution;

const MAX_LETTERS_SIZE: usize = 26;

impl Solution {
    /// v3: one `[i32; 26]` counter, +1 for `first` and -1 for `second`. See the module docs.
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

// Given an array of integers nums, return the length of the longest consecutive sequence
// of elements that can be formed.
//
// A consecutive sequence is a sequence of elements in which each element is exactly 1
// greater than the previous element. The elements do not have to be consecutive in the
// original array.
//
// You must write an algorithm that runs in O(n) time.
//
// Example 1:
//
// Input: nums = [2,20,4,10,3,4,5]
//
// Output: 4
//
// Explanation: The longest consecutive sequence is [2, 3, 4, 5].
//
// Example 2:
//
// Input: nums = [0,3,2,5,4,6,1,1]
//
// Output: 7
//
// Constraints:
//
// 0 <= nums.length <= 1000
// -10^9 <= nums[i] <= 10^9

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive_sorting_version(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut sorted_nums = nums.clone();

        sorted_nums.sort();

        let mut acc = 1;
        let mut longest_run = 0;

        for i in 0..nums.len() - 1 {
            if sorted_nums[i + 1] - sorted_nums[i] == 0 {
                continue;
            } else if sorted_nums[i + 1] - sorted_nums[i] == 1 {
                acc += 1
            } else {
                if acc > longest_run {
                    longest_run = acc;
                }
                acc = 1;
            }
        }

        if acc > longest_run {
            longest_run = acc;
        }

        longest_run
    }

    pub fn longest_consecutive_hashset_version(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let hash: HashSet<i32> = nums.into_iter().collect();

        let mut longest_run = 0;

        for &num in &hash {
            if hash.contains(&(num - 1)) {
                continue;
            }

            let mut acc = 1;
            let mut current = num;

            while hash.contains(&(current + 1)) {
                acc += 1;
                current += 1;
            }

            if acc > longest_run {
                longest_run = acc;
            }
        }

        longest_run
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_first_example_sorting_version() {
        let input = vec![2, 20, 4, 10, 3, 4, 5];
        let result = Solution::longest_consecutive_sorting_version(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_second_example_sorting_version() {
        let input = vec![0, 3, 2, 5, 4, 6, 1, 1];
        let result = Solution::longest_consecutive_sorting_version(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_works_third_example_sorting_version() {
        let input = vec![1, 2, 10, 11, 12];
        let result = Solution::longest_consecutive_sorting_version(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_fourth_example_sorting_version() {
        let input = vec![-3, -1, -2, 5];
        let result = Solution::longest_consecutive_sorting_version(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_fifth_example_sorting_version() {
        let input = vec![-1, 0, 1];
        let result = Solution::longest_consecutive_sorting_version(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_sixth_example_sorting_version() {
        let input = vec![];
        let result = Solution::longest_consecutive_sorting_version(input);
        assert_eq!(result, 0);
    }

    //HASHSET VERSION

    #[test]
    fn it_works_first_example_hashset_version() {
        let input = vec![2, 20, 4, 10, 3, 4, 5];
        let result = Solution::longest_consecutive_hashset_version(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_second_example_hashset_version() {
        let input = vec![0, 3, 2, 5, 4, 6, 1, 1];
        let result = Solution::longest_consecutive_hashset_version(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_works_third_example_hashset_version() {
        let input = vec![1, 2, 10, 11, 12];
        let result = Solution::longest_consecutive_hashset_version(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_fourth_example_hashset_version() {
        let input = vec![-3, -1, -2, 5];
        let result = Solution::longest_consecutive_hashset_version(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_fifth_example_hashset_version() {
        let input = vec![-1, 0, 1];
        let result = Solution::longest_consecutive_hashset_version(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_sixth_example_hashset_version() {
        let input = vec![];
        let result = Solution::longest_consecutive_hashset_version(input);
        assert_eq!(result, 0);
    }
}

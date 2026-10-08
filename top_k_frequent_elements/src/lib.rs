// Given an integer array nums and an integer k, return the k most frequent elements within the array.
//
// The test cases are generated such that the answer is always unique.
//
// You may return the output in any order.
//
// Example 1:
//
// Input: nums = [1,2,2,3,3,3], k = 2
//
// Output: [2,3]

// Constraints:
//
// 1 <= nums.length <= 10^4.
// -1000 <= nums[i] <= 1000
// 1 <= k <= number of distinct elements in nums.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for num in nums {
            if let Some(found) = hash.get_mut(&num) {
                *found += 1;
            } else {
                hash.insert(num, 1);
            }
        }

        let mut accumulator: Vec<(i32, i32)> = Vec::new();

        for (key, count) in hash.iter() {
            accumulator.push((*count, *key));
        }

        accumulator.sort_by(|a, b| b.cmp(a));

        accumulator
            .iter()
            .take(k as usize)
            .map(|tup| tup.1)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_example_one() {
        let nums = vec![1, 2, 2, 3, 3, 3];

        let k = 2;

        let mut expected = vec![2, 3];

        let mut result = Solution::top_k_frequent(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_two() {
        let nums = vec![7, 7];

        let k = 1;

        let mut expected = vec![7];

        let mut result = Solution::top_k_frequent(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }
}

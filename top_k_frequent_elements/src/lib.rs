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

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct Solution;

impl Solution {
    // How to read Big-O here:
    //   n = nums.len()
    //   m = number of DISTINCT values in nums (always m <= n)
    //   k = how many we return (k <= m)
    // Recipe: (1) find the loops, (2) count how many times each runs,
    // (3) multiply by the cost of one iteration, (4) add the steps,
    // (5) keep only the biggest term and drop constants.
    pub fn top_k_frequent_naive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // STEP 1 - allocate the map.
        // Time: ~O(1) to request memory, not a loop over the elements.
        // Space: O(n) reserved up front (more than the m slots we really need).
        let mut hash: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        // STEP 2 - count occurrences.
        // The loop runs n times. Each get_mut/insert on a HashMap is O(1) on
        // average (hash the key, jump to the bucket).
        // n iterations * O(1) each = O(n)
        for num in nums {
            if let Some(found) = hash.get_mut(&num) {
                *found += 1;
            } else {
                hash.insert(num, 1);
            }
        }

        // STEP 3 - copy the map into a Vec of (count, num).
        // The map has m entries, so the loop runs m times. push() is O(1)
        // "amortized": the Vec sometimes doubles its capacity, but averaged over
        // all pushes each one is still O(1).
        // m iterations * O(1) = O(m). Space: O(m) for the new Vec.
        let mut accumulator: Vec<(i32, i32)> = Vec::new();

        for (key, count) in hash.iter() {
            accumulator.push((*count, *key));
        }

        // STEP 4 - sort the m tuples.
        // Comparison sorts (Rust uses merge-sort/quicksort variants) cost
        // O(m log m): roughly log m "rounds", and each round touches all m items.
        // No comparison sort can do better than O(m log m) in general.
        accumulator.sort_by(|a, b| b.cmp(a));

        // STEP 5 - take the first k and keep only the number.
        // Iterators are lazy: take(k) stops after k items, so map runs k times
        // and collect pushes k items. -> O(k)
        accumulator
            .iter()
            .take(k as usize)
            .map(|tup| tup.1)
            .collect()

        // TOTAL - add the steps:
        //   O(1) + O(n) + O(m) + O(m log m) + O(k)
        // Drop the smaller terms (k <= m, and m <= m log m):
        //   = O(n + m log m)
        // Worst case: every number is distinct, so m = n:
        //   = O(n + n log n) = O(n log n)   <- the sort dominates
        // Space: O(m) for the map + O(m) for the Vec = O(m), worst case O(n).
    }

    pub fn top_k_frequent_bheap(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::with_capacity(2_000);

        for num in nums {
            hash.entry(num).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut bheap: BinaryHeap<Reverse<(i32, i32)>> =
            BinaryHeap::with_capacity((k + 1) as usize);

        for (key, count) in hash.iter() {
            bheap.push(Reverse((*count, *key)));

            if bheap.len() > k as usize {
                bheap.pop().unwrap();
            }
        }

        bheap.into_iter().map(|Reverse((_, val))| val).collect()
    }

    pub fn top_k_frequent_bucket(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::with_capacity(2_000);

        let nums_len = nums.len();

        for num in nums {
            hash.entry(num).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut bucket: Vec<Vec<i32>> = vec![Vec::new(); nums_len + 1];

        for (key, count) in hash.iter() {
            bucket[*count as usize].push(*key);
        }

        bucket
            .into_iter()
            .rev()
            .flatten()
            .take(k as usize)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_example_one_naive() {
        let nums = vec![1, 2, 2, 3, 3, 3];

        let k = 2;

        let mut expected = vec![2, 3];

        let mut result = Solution::top_k_frequent_naive(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_two_naive() {
        let nums = vec![7, 7];

        let k = 1;

        let mut expected = vec![7];

        let mut result = Solution::top_k_frequent_naive(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_with_tied_counts_naive() {
        let nums = vec![1, 1, 2, 2, 3];

        let k = 2;

        let mut expected = vec![1, 2];

        let mut result = Solution::top_k_frequent_naive(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_with_negative_numbers_naive() {
        let nums = vec![-1, -1, -1, 2, 2, -3, 4];

        let k = 2;

        let mut expected = vec![-1, 2];

        let mut result = Solution::top_k_frequent_naive(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_one_bheap() {
        let nums = vec![1, 2, 2, 3, 3, 3];

        let k = 2;

        let mut expected = vec![2, 3];

        let mut result = Solution::top_k_frequent_bheap(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_two_bheap() {
        let nums = vec![7, 7];

        let k = 1;

        let mut expected = vec![7];

        let mut result = Solution::top_k_frequent_bheap(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_with_tied_counts_bheap() {
        let nums = vec![1, 1, 2, 2, 3];

        let k = 2;

        let mut expected = vec![1, 2];

        let mut result = Solution::top_k_frequent_bheap(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_with_negative_numbers_bheap() {
        let nums = vec![-1, -1, -1, 2, 2, -3, 4];

        let k = 2;

        let mut expected = vec![-1, 2];

        let mut result = Solution::top_k_frequent_bheap(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_one_bucket() {
        let nums = vec![1, 2, 2, 3, 3, 3];

        let k = 2;

        let mut expected = vec![2, 3];

        let mut result = Solution::top_k_frequent_bucket(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_two_bucket() {
        let nums = vec![7, 7];

        let k = 1;

        let mut expected = vec![7];

        let mut result = Solution::top_k_frequent_bucket(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_with_tied_counts_bucket() {
        let nums = vec![1, 1, 2, 2, 3];

        let k = 2;

        let mut expected = vec![1, 2];

        let mut result = Solution::top_k_frequent_bucket(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_with_negative_numbers_bucket() {
        let nums = vec![-1, -1, -1, 2, 2, -3, 4];

        let k = 2;

        let mut expected = vec![-1, 2];

        let mut result = Solution::top_k_frequent_bucket(nums, k);

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }
}

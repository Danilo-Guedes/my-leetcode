// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] where
// nums[i] + nums[j] + nums[k] == 0, and the indices i, j and k are all distinct.
//
// The output should not contain any duplicate triplets. You may return the output and
// the triplets in any order.
//
// Example 1:
//
// Input: nums = [-1,0,1,2,-1,-4]
//
// Output: [[-1,-1,2],[-1,0,1]]
//
// Explanation:
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
//
// Example 2:
//
// Input: nums = [0,1,1]
//
// Output: []
//
// Example 3:
//
// Input: nums = [0,0,0]
//
// Output: [[0,0,0]]
//
// Constraints:
//
// 3 <= nums.length <= 1000
// -10^5 <= nums[i] <= 10^5

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut results: Vec<Vec<i32>> = vec![];

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = (nums.len() - 1) as usize;

            while j < k {
                let value = nums[i] + nums[j] + nums[k];

                if value == 0 {
                    results.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;

                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                }

                if value > 0 {
                    k -= 1;
                }

                if value < 0 {
                    j += 1;
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_example_one() {
        let input = vec![-1, 0, 1, 2, -1, -4];
        let result = Solution::three_sum(input);
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_two() {
        let input = vec![0, 1, 1];
        let result = Solution::three_sum(input);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_three() {
        let input = vec![0, 0, 0];
        let result = Solution::three_sum(input);
        let expected = vec![vec![0, 0, 0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_skips_duplicate_j() {
        let input = vec![1, 2, 0, 1, 0, 0, 0, 0];
        let result = Solution::three_sum(input);
        let expected = vec![vec![0, 0, 0]];
        assert_eq!(result, expected);
    }
}

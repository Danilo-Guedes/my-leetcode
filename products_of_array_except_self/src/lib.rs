// Given an integer array nums, return an array output where output[i] is the product of
// all the elements of nums except nums[i].
//
// Each product is guaranteed to fit in a 32-bit integer.
//
// Follow-up: Could you solve it in O(n) time without using the division operation?
//
// Example 1:
//
// Input: nums = [1,2,4,6]
//
// Output: [48,24,12,8]
//
// Example 2:
//
// Input: nums = [-1,0,1,2,3]
//
// Output: [0,-6,0,0,0]
//
// Constraints:
//
// 2 <= nums.length <= 1000
// -20 <= nums[i] <= 20

pub struct Solution;

impl Solution {
    pub fn product_except_self_simple_loop(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for idx in 0..nums.len() {
            let mut accumulator = 1;

            for (inner_idx, num) in nums.iter().enumerate() {
                if inner_idx == idx {
                    continue;
                } else {
                    accumulator = accumulator * num;
                }
            }

            result.push(accumulator);
        }

        result
    }

    pub fn product_except_self_optimal(nums: Vec<i32>) -> Vec<i32> {
        let mut before_column: Vec<i32> = vec![1; nums.len()];

        for idx in 1..nums.len() {
            before_column[idx] = before_column[idx - 1] * nums[idx - 1];
        }

        let mut after_column: Vec<i32> = vec![1; nums.len()];

        for idx in (0..nums.len() - 1).rev() {
            after_column[idx] = after_column[idx + 1] * nums[idx + 1];
        }

        before_column
            .iter()
            .zip(after_column)
            .map(|(bef, aft)| bef * aft)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_example_one_simple_loop() {
        let input = vec![1, 2, 4, 6];

        let result = Solution::product_except_self_simple_loop(input);

        let expected = vec![48, 24, 12, 8];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_two_simple_loop() {
        let input = vec![-1, 0, 1, 2, 3];

        let result = Solution::product_except_self_simple_loop(input);

        let expected = vec![0, -6, 0, 0, 0];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_one_optimal() {
        let input = vec![1, 2, 4, 6];

        let result = Solution::product_except_self_optimal(input);

        let expected = vec![48, 24, 12, 8];

        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_example_two_optimal() {
        let input = vec![-1, 0, 1, 2, 3];

        let result = Solution::product_except_self_optimal(input);

        let expected = vec![0, -6, 0, 0, 0];

        assert_eq!(result, expected);
    }
}

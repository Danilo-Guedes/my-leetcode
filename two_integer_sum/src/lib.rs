// Given an array of integers nums and an integer target,
// return the indices i and j such that nums[i] + nums[j] == target and i != j.

// You may assume that every input has exactly one pair of indices i and j that satisfy the condition.
//
// Return the answer with the smaller index first.

// Constraints:
//
// 2 <= nums.length <= 1000
// -10,000,000 <= nums[i] <= 10,000,000
// -10,000,000 <= target <= 10,000,000

pub struct Solution;

impl Solution {
    pub fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if j == i {
                    continue;
                }

                if (nums[i] + nums[j]) == target && i < j {
                    result.push(i as i32);
                    result.push(j as i32);
                }
            }
        }

        result
    }

    pub fn two_sum_early_return(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len().saturating_sub(1) {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_a_valid_solution_one_naive() {
        let input = vec![3, 4, 5, 6];
        let target = 7;

        let result = Solution::two_sum_naive(input, target);
        assert_eq!(result, [0, 1]); //nums[0] + nums[1] == 7
    }

    #[test]
    fn it_finds_a_valid_solution_two_naive() {
        let input = vec![4, 5, 6];
        let target = 10;

        let result = Solution::two_sum_naive(input, target);
        assert_eq!(result, [0, 2]); //nums[0] + nums[2] == 10
    }

    #[test]
    fn it_finds_a_valid_solution_three_naive() {
        let input = vec![5, 5];
        let target = 10;

        let result = Solution::two_sum_naive(input, target);
        assert_eq!(result, [0, 1]); //nums[0] + nums[1] == 10
    }

    #[test]
    fn it_finds_a_valid_solution_one_early_return() {
        let input = vec![3, 4, 5, 6];
        let target = 7;

        let result = Solution::two_sum_early_return(input, target);
        assert_eq!(result, [0, 1]); //nums[0] + nums[1] == 7
    }

    #[test]
    fn it_finds_a_valid_solution_two_early_return() {
        let input = vec![4, 5, 6];
        let target = 10;

        let result = Solution::two_sum_early_return(input, target);
        assert_eq!(result, [0, 2]); //nums[0] + nums[2] == 10
    }

    #[test]
    fn it_finds_a_valid_solution_three_early_return() {
        let input = vec![5, 5];
        let target = 10;

        let result = Solution::two_sum_early_return(input, target);
        assert_eq!(result, [0, 1]); //nums[0] + nums[1] == 10
    }
}

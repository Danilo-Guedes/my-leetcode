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

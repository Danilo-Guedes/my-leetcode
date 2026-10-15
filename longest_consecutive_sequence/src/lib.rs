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
}

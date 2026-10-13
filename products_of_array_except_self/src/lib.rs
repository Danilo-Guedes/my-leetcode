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
    fn it_works_example_two() {
        let input = vec![-1, 0, 1, 2, 3];

        let result = Solution::product_except_self_simple_loop(input);

        let expected = vec![0, -6, 0, 0, 0];

        assert_eq!(result, expected);
    }
}

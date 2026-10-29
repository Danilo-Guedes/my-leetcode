use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn trap_naive_loops(height: Vec<i32>) -> i32 {
        let mut accum = 0;

        if height.len() < 3 {
            return 0;
        }

        for idx in 1..height.len() - 1 {
            let mut left_most_taller = 0;
            let mut right_most_taller = 0;

            for left_idx in 0..idx {
                left_most_taller = max(height[left_idx], left_most_taller);
            }

            for right_idx in idx + 1..height.len() {
                right_most_taller = max(height[right_idx], right_most_taller);
            }

            let water_vol = max(min(left_most_taller, right_most_taller) - height[idx], 0);

            accum += water_vol;
        }

        accum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_naive_loops() {
        let input = vec![0, 2, 0, 3, 1, 0, 1, 3, 2, 1];
        let result = Solution::trap_naive_loops(input);
        assert_eq!(result, 9);
    }

    #[test]
    fn tall_edges_naive_loops() {
        assert_eq!(Solution::trap_naive_loops(vec![3, 0, 2, 0, 4]), 7);
    }

    #[test]
    fn staircase_holds_nothing_naive_loops() {
        assert_eq!(Solution::trap_naive_loops(vec![0, 1, 2, 3]), 0);
    }
}

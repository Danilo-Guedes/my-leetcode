use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // todo!()

        // min(height[l], height[r]) - height[i]

        let mut accum = 0;

        if height.len() < 3 {
            return 0;
        }

        for idx in 1..height.len() - 1 {
            let mut left_idx = idx.saturating_sub(1);
            let mut right_idx = idx.saturating_add(1);
            let mut left_most_taller = 0;
            let mut right_most_taller = 0;

            if height[left_idx] < height[idx] || height[right_idx] < height[idx] {
                continue;
            }

            while left_idx > 0 {
                left_most_taller = max(height[left_idx - 1], height[left_idx]);
                left_idx = left_idx - 1;
            }

            while right_idx < height.len() - 1 {
                right_most_taller = max(height[right_idx - 1], height[right_idx]);
                right_idx = right_idx + 1;
            }

            let water_vol = max(min(left_most_taller, right_most_taller) - height[idx], 0);

            println!("{idx}: {water_vol}");

            accum += water_vol;
        }

        accum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![0, 2, 0, 3, 1, 0, 1, 3, 2, 1];
        let result = Solution::trap(input);
        assert_eq!(result, 9);
    }
}

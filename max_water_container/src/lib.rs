use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        // water = min(height_left, height_right) * (right_index - left_index)

        let mut i = 0;
        let mut j = (heights.len() - 1) as usize;
        let mut final_water_volume = 0;

        while i < j {
            let left_height = heights[i];
            let right_height = heights[j];
            let current_water_vol = min(left_height, right_height) * (j - i) as i32;

            final_water_volume = max(current_water_vol, final_water_volume);

            // now the move of the

            if left_height < right_height {
                i += 1;
            } else {
                j -= 1;
            }
        }
        final_water_volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = vec![1, 7, 2, 5, 4, 7, 3, 6];
        let result = Solution::max_area(input);
        let expected = 36;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two() {
        let input = vec![2, 2, 2];
        let result = Solution::max_area(input);
        let expected = 4;
        assert_eq!(result, expected);
    }
}

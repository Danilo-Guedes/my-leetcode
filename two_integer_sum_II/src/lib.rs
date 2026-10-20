pub struct Solution;

impl Solution {
    pub fn two_sum_ii_forloop(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            for j in (i + 1..numbers.len()).rev() {
                if numbers[i] + numbers[j] == target {
                    return vec![(i + 1) as i32, (j + 1) as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_ii_two_pointers(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0 as usize;
        let mut j = (numbers.len().saturating_sub(1)) as usize;

        while j > i {
            let sum = numbers[i] + numbers[j];

            // three cases:

            // sum == target -> return vec
            if sum == target {
                return vec![(i + 1) as i32, (j + 1) as i32];
            }
            // sum >  target -> move j -
            if sum > target {
                j -= 1;
            }
            // sum <  target -> move i +
            if sum < target {
                i += 1;
            }
        }
        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_forloop() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![1, 2];
        let result = Solution::two_sum_ii_forloop(input, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_forloop() {
        let input = vec![1, 2, 3, 4];
        let target = 7;
        let expected = vec![3, 4];
        let result = Solution::two_sum_ii_forloop(input, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_one_two_pointers() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![1, 2];
        let result = Solution::two_sum_ii_two_pointers(input, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_two_pointers() {
        let input = vec![1, 2, 3, 4];
        let target = 7;
        let expected = vec![3, 4];
        let result = Solution::two_sum_ii_two_pointers(input, target);
        assert_eq!(result, expected);
    }
}

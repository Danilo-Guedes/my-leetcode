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
}

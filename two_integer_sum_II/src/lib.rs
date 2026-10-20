pub struct Solution;

impl Solution {
    pub fn two_sum_ii_forloop(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        for i in 0..numbers.len() {
            // let mut reverse_idx = usize::from(numbers.len() - 1);
            // let current = numbers[i];

            for j in (i + 1..numbers.len()).rev() {
                if numbers[i] + numbers[j] == target {
                    result.push((i + 1) as i32);
                    result.push((j + 1) as i32);
                }
            }
        }

        result
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

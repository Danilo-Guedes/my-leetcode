use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut hash = HashSet::with_capacity(nums.len());

        nums.into_iter().any(|n| !hash.insert(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contain_duplicates_should_return_false() {
        let list_of_num = vec![1, 2, 3, 4];

        assert_eq!(Solution::has_duplicate(list_of_num), false)
    }

    #[test]
    fn contain_duplicates_should_return_true() {
        let list_of_num = vec![1, 2, 3, 3];

        assert_eq!(Solution::has_duplicate(list_of_num), true)
    }
}

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn has_duplicate_hashmap(nums: Vec<i32>) -> bool {
        let mut hash = HashMap::new();

        for num in nums {
            if hash.contains_key(&num) {
                let found = hash.get_mut(&num).unwrap();
                *found += 1;
            } else {
                hash.insert(num, 1);
            }
        }

        let mut values = hash.values();

        values.any(|&n| n > 1)
    }

    pub fn has_duplicate_hashset(nums: Vec<i32>) -> bool {
        let mut hash = HashSet::new();

        for num in nums {
            if hash.contains(&num) {
                return true;
            } else {
                hash.insert(num);
            }
        }

        false
    }

    pub fn has_duplicate_hashset_capacity(nums: Vec<i32>) -> bool {
        let mut hash = HashSet::with_capacity(nums.len());

        nums.into_iter().any(|n| !hash.insert(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contain_duplicates_should_return_false_hashmap() {
        let list_of_num = vec![1, 2, 3, 4];

        assert_eq!(Solution::has_duplicate_hashmap(list_of_num), false)
    }

    #[test]
    fn contain_duplicates_should_return_true_hashmap() {
        let list_of_num = vec![1, 2, 3, 3];

        assert_eq!(Solution::has_duplicate_hashmap(list_of_num), true)
    }

    #[test]
    fn contain_duplicates_should_return_false_hashset() {
        let list_of_num = vec![1, 2, 3, 4];

        assert_eq!(Solution::has_duplicate_hashset(list_of_num), false)
    }

    #[test]
    fn contain_duplicates_should_return_true_hashset() {
        let list_of_num = vec![1, 2, 3, 3];

        assert_eq!(Solution::has_duplicate_hashset(list_of_num), true)
    }

    #[test]
    fn contain_duplicates_should_return_false_hashset_capacity() {
        let list_of_num = vec![1, 2, 3, 4];

        assert_eq!(Solution::has_duplicate_hashset_capacity(list_of_num), false)
    }

    #[test]
    fn contain_duplicates_should_return_true_hashset_capacity() {
        let list_of_num = vec![1, 2, 3, 3];

        assert_eq!(Solution::has_duplicate_hashset_capacity(list_of_num), true)
    }
}

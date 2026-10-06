use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash: HashMap<String, Vec<String>> = HashMap::new();

        for orig_val in strs {
            let mut ordered_chars: Vec<_> = orig_val.chars().collect();
            ordered_chars.sort();

            let key = String::from_iter::<Vec<_>>(ordered_chars.iter().collect());

            if let Some(group) = hash.get_mut(&key) {
                group.push(orig_val);
            } else {
                hash.insert(key, vec![orig_val]);
            }
        }

        let values: Vec<Vec<String>> = hash.into_values().collect();

        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_example_one() {
        let input: Vec<String> = vec![
            "act".to_string(),
            "pots".to_string(),
            "tops".to_string(),
            "cat".to_string(),
            "stop".to_string(),
            "hat".to_string(),
        ];
        let result = Solution::group_anagrams(input);

        let expected = vec![
            vec!["hat".to_string()],
            vec!["act".to_string(), "cat".to_string()],
            vec!["stop".to_string(), "pots".to_string(), "tops".to_string()],
        ];
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn it_works_example_two() {
        let input: Vec<String> = vec!["x".to_string()];
        let result = Solution::group_anagrams(input);

        let expected = vec![vec!["x".to_string()]];
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn it_works_example_three() {
        let input: Vec<String> = vec!["".to_string()];
        let result = Solution::group_anagrams(input);

        let expected = vec![vec!["".to_string()]];
        assert_eq!(normalize(result), normalize(expected));
    }

    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in groups.iter_mut() {
            group.sort(); // order inside each group no longer matters
        }
        groups.sort(); // order of the groups no longer matters
        groups
    }
}

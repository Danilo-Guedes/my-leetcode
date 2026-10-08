use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams_contains_key(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash: HashMap<String, Vec<String>> = HashMap::new();

        for orig_val in strs {
            let mut ordered_chars: Vec<_> = orig_val.chars().collect();
            ordered_chars.sort();

            let key = String::from_iter::<Vec<_>>(ordered_chars.iter().collect());

            if hash.contains_key(&key) {
                //found

                let found_value = hash.get_mut(&key).unwrap();

                found_value.push(orig_val);
            } else {
                // didn' find, add it

                hash.insert(key, vec![orig_val]);
            }
        }

        let values: Vec<Vec<String>> = hash.into_values().collect();

        values
    }

    pub fn group_anagrams_if_let(strs: Vec<String>) -> Vec<Vec<String>> {
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
    fn it_works_example_one_contains_key() {
        let input: Vec<String> = vec![
            "act".to_string(),
            "pots".to_string(),
            "tops".to_string(),
            "cat".to_string(),
            "stop".to_string(),
            "hat".to_string(),
        ];
        let result = Solution::group_anagrams_contains_key(input);

        let expected = vec![
            vec!["hat".to_string()],
            vec!["act".to_string(), "cat".to_string()],
            vec!["stop".to_string(), "pots".to_string(), "tops".to_string()],
        ];
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn it_works_example_two_contains_key() {
        let input: Vec<String> = vec!["x".to_string()];
        let result = Solution::group_anagrams_contains_key(input);

        let expected = vec![vec!["x".to_string()]];
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn it_works_example_three_contains_key() {
        let input: Vec<String> = vec!["".to_string()];
        let result = Solution::group_anagrams_contains_key(input);

        let expected = vec![vec!["".to_string()]];
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn it_works_example_one_if_let() {
        let input: Vec<String> = vec![
            "act".to_string(),
            "pots".to_string(),
            "tops".to_string(),
            "cat".to_string(),
            "stop".to_string(),
            "hat".to_string(),
        ];
        let result = Solution::group_anagrams_if_let(input);

        let expected = vec![
            vec!["hat".to_string()],
            vec!["act".to_string(), "cat".to_string()],
            vec!["stop".to_string(), "pots".to_string(), "tops".to_string()],
        ];
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn it_works_example_two_if_let() {
        let input: Vec<String> = vec!["x".to_string()];
        let result = Solution::group_anagrams_if_let(input);

        let expected = vec![vec!["x".to_string()]];
        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn it_works_example_three_if_let() {
        let input: Vec<String> = vec!["".to_string()];
        let result = Solution::group_anagrams_if_let(input);

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

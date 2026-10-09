pub struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut enconded_acc = String::new();

        for word in strs {
            enconded_acc.push_str(&format!("{}#{}", word.len(), word));
        }

        enconded_acc
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut decoded_acc: Vec<String> = vec![];

        let mut i = 0;

        while let Some(relative_hash_idx) = s[i..].find('#') {
            let hash_idx = i + relative_hash_idx;
            let word_size = s[i..hash_idx].parse::<usize>().unwrap();

            let begin = hash_idx + 1;
            let end = begin + word_size;

            let found_word = s[begin..end].to_string();

            decoded_acc.push(found_word);

            i = end;
        }

        decoded_acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_naive() {
        let input: Vec<String> = vec!["Hello".to_string(), "World".to_string()];

        let encoded_string = Solution::encode(input.clone());
        let decoded_string = Solution::decode(encoded_string);

        assert_eq!(decoded_string, input.clone());
    }

    #[test]
    fn it_works_with_hash_inside_words() {
        let input: Vec<String> = vec!["a#b".to_string(), "".to_string(), "##".to_string()];

        let encoded_string = Solution::encode(input.clone());
        let decoded_string = Solution::decode(encoded_string);

        assert_eq!(decoded_string, input);
    }
}

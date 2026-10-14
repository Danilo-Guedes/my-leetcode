use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku_hashset(board: Vec<Vec<char>>) -> bool {
        let mut lines_hash: HashSet<String> = HashSet::new();
        let mut col_hash: HashSet<String> = HashSet::new();
        let mut square_hash: HashSet<String> = HashSet::new();

        for (line_idx, col) in board.iter().enumerate() {
            for (col_idx, val) in col.iter().enumerate() {
                if *val == '.' {
                    continue;
                }

                if !lines_hash.insert(format!("{line_idx}-{val}")) {
                    return false;
                }
                if !col_hash.insert(format!("{col_idx}-{val}")) {
                    return false;
                }

                if !square_hash.insert(format!("{}-{}-{val}", (line_idx / 3), (col_idx / 3))) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validate_a_correct_hashset() {
        let input: Vec<Vec<char>> = vec![
            vec!['1', '2', '.', '.', '3', '.', '.', '.', '.'],
            vec!['4', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '.', '3'],
            vec!['5', '.', '.', '.', '6', '.', '.', '.', '4'],
            vec!['.', '.', '.', '8', '.', '3', '.', '.', '5'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '.', '.', '.', '.', '.', '2', '.', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '8'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let result = Solution::is_valid_sudoku_hashset(input);
        assert_eq!(result, true);
    }

    #[test]
    fn it_reject_an_incorrect_hashset() {
        let input: Vec<Vec<char>> = vec![
            vec!['1', '2', '.', '.', '3', '.', '.', '.', '.'],
            vec!['4', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '9', '1', '.', '.', '.', '.', '.', '3'],
            vec!['5', '.', '.', '.', '6', '.', '.', '.', '4'],
            vec!['.', '.', '.', '8', '.', '3', '.', '.', '5'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '.', '.', '.', '.', '.', '2', '.', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '8'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let result = Solution::is_valid_sudoku_hashset(input);
        assert_eq!(result, false);
    }
}

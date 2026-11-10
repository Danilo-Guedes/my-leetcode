// You are given an integer array prices where prices[i] is the price of NeetCoin on the ith day.
//
// You may choose a single day to buy one NeetCoin and choose a different day in the future to sell it.
//
// Return the maximum profit you can achieve. You may choose to not make any transactions,
// in which case the profit would be 0.
//
// Example 1:
//
// Input: prices = [10,1,5,6,7,1]
//
// Output: 6
//
// Explanation: Buy prices[1] and sell prices[4], profit = 7 - 1 = 6.
//
// Example 2:
//
// Input: prices = [10,8,7,5,2]
//
// Output: 0
//
// Explanation: No profitable transactions can be made, thus the max profit is 0.
//
// Constraints:
//
// 1 <= prices.length <= 100
// 0 <= prices[i] <= 100

use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn max_profit_naive_loops(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;

        for i in 0..prices.len() {
            let sell_price = prices[i];
            let mut best_buy_price = i32::MAX;

            for j in 0..=i {
                best_buy_price = min(prices[j], best_buy_price);
            }

            max_profit = max(sell_price - best_buy_price, max_profit);
        }

        max_profit
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut best_buy_price = i32::MAX;

        for i in 0..prices.len() {
            let sell_price = prices[i];

            best_buy_price = min(prices[i], best_buy_price);

            max_profit = max(sell_price - best_buy_price, max_profit);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_naive_loops() {
        let input = vec![10, 1, 5, 6, 7, 1];
        let result = Solution::max_profit_naive_loops(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_two_naive_loops() {
        let input = vec![10, 8, 7, 5, 2];
        let result = Solution::max_profit_naive_loops(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn two_days_sell_on_last_day_naive_loops() {
        assert_eq!(Solution::max_profit_naive_loops(vec![1, 5]), 4);
    }

    #[test]
    fn best_sell_is_last_day_naive_loops() {
        assert_eq!(Solution::max_profit_naive_loops(vec![3, 1, 9]), 8);
    }

    #[test]
    fn global_min_after_global_max_naive_loops() {
        assert_eq!(Solution::max_profit_naive_loops(vec![2, 10, 1, 3]), 8);
    }

    #[test]
    fn single_day_naive_loops() {
        assert_eq!(Solution::max_profit_naive_loops(vec![7]), 0);
    }

    #[test]
    fn example_one() {
        let input = vec![10, 1, 5, 6, 7, 1];
        let result = Solution::max_profit(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_two() {
        let input = vec![10, 8, 7, 5, 2];
        let result = Solution::max_profit(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn two_days_sell_on_last_day() {
        assert_eq!(Solution::max_profit(vec![1, 5]), 4);
    }

    #[test]
    fn best_sell_is_last_day() {
        assert_eq!(Solution::max_profit(vec![3, 1, 9]), 8);
    }

    #[test]
    fn global_min_after_global_max() {
        assert_eq!(Solution::max_profit(vec![2, 10, 1, 3]), 8);
    }

    #[test]
    fn single_day() {
        assert_eq!(Solution::max_profit(vec![7]), 0);
    }
}

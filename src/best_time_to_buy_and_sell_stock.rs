// # Best Time to Buy and Sell Stock
//
// You are given an array prices where prices[i] is the price of a given stock on the ith day.
// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
//
// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

use std::{env::current_dir, ops::IndexMut};

fn solution(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut profits: Vec<i32> = Vec::new();

    for idx in 0..prices.len() {
        profits.push(0);
        let current_pivot = prices[idx];
        for idx2 in idx..prices.len() {
            if prices[idx2] - current_pivot > profits[idx] {
                profits[idx] = prices[idx2] - current_pivot;
            }
        }
    }
    profits.iter().max().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(solution(prices), 5);
    }

    #[test]
    fn example_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(solution(prices), 0);
    }
}

// # Best Time to Buy and Sell Stock II
// You are given an integer array prices where prices[i] is the price of a given stock on the ith day.
//
// On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.
//
// Find and return the maximum profit you can achieve.

fn solution(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut buy_price = prices[0];

    for price in prices {
        if price > buy_price {
            profit += price - buy_price;
            buy_price = price
        } else { buy_price = price }
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(solution(prices), 7);
    }

    #[test]
    fn example_2() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(solution(prices), 4);
    }

    #[test]
    fn example_3() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(solution(prices), 0);
    }

    #[test]
    fn example_4() {
        let prices = vec![1];
        assert_eq!(solution(prices), 0);
    }
}
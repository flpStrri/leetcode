// # Best Time to Buy and Sell Stock
//
// You are given an array prices where prices[i] is the price of a given stock on the ith day.
// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
//
// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

fn solution(prices: Vec<i32>) -> i32 {
    let mut buy_price: i32 = prices[0];
    let mut profit: i32 = 0;

    for price in prices {
        if price < buy_price {
            buy_price = price;
        } else if price - buy_price > profit {
            profit = price - buy_price;
        }
    }

    profit
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

    #[test]
    fn example_3() {
        let prices = vec![7, 3, 8, 1, 3];
        assert_eq!(solution(prices), 5)
    }
}

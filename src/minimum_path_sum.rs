// # Minimum Path Sum
// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.
//
// Note: You can only move either down or right at any point in time.

pub fn solution(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = grid[0][0];

    for row in 0..m {
        for col in 0..n {
            if row + col == 0 {
                continue;
            };
            let upper_row = if row == 0 { i32::MAX } else { dp[row - 1][col] };
            let upper_col = if col == 0 { i32::MAX } else { dp[row][col - 1] };

            dp[row][col] = std::cmp::min(upper_row, upper_col) + grid[row][col];
        }
    }

    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(solution(grid), 7);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(solution(grid), 12);
    }
}

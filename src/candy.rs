// # Candy
// There are `n` children standing in a line.
// Each child is assigned a rating value given in the integer array ratings.
//
// You are giving candies to these children subjected to the following requirements:
// - Each child must have at least one candy.
// - Children with a higher rating get more candies than their neighbors.
//
// Return the minimum number of candies you need to have to distribute the candies to the children.

pub fn solution(ratings: Vec<i32>) -> i32 {
    let child_count = ratings.len();
    if child_count <= 1 {
        return child_count as i32;
    }

    let mut candies = vec![1; ratings.len()];
    for forward_pass_rating_idx in 1..child_count {
        if ratings[forward_pass_rating_idx] > ratings[forward_pass_rating_idx - 1] {
            candies[forward_pass_rating_idx] = candies[forward_pass_rating_idx - 1] + 1;
        }
    }
    for backward_pass_rating_idx in (0..child_count - 1).rev() {
        if ratings[backward_pass_rating_idx] > ratings[backward_pass_rating_idx + 1] {
            candies[backward_pass_rating_idx] = std::cmp::max(
                candies[backward_pass_rating_idx],
                candies[backward_pass_rating_idx + 1] + 1,
            );
        }
    }

    candies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let ratings = vec![1, 0, 2];
        assert_eq!(solution(ratings), 5);
    }

    #[test]
    fn example_2() {
        let ratings = vec![1, 2, 2];
        assert_eq!(solution(ratings), 4);
    }

    #[test]
    fn example_3() {
        let ratings = vec![1, 3, 2, 2, 1];
        assert_eq!(solution(ratings), 7)
    }

    #[test]
    fn example_4() {
        let ratings = vec![1, 2, 87, 87, 87, 2, 1];
        assert_eq!(solution(ratings), 13)
    }
}

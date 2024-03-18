// # Majority Element
// Given an array nums of size n, return the majority element.
//
// The majority element is the element that appears more than ⌊n / 2⌋ times.
// You may assume that the majority element always exists in the array.

pub fn solution(nums: Vec<i32>) -> i32 {
    let mut map = std::collections::HashMap::new();

    for num in &nums {
        map.entry(num)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let majority_hurdle = &nums.len() / 2;
    for (num, count) in map {
        if count > majority_hurdle {
            return *num;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![3, 2, 3];
        assert_eq!(solution(nums), 3);
    }

    #[test]
    fn example_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(solution(nums), 2);
    }
}

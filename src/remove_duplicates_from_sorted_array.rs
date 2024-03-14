// # Remove Duplicates from Sorted Array
//
// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once.
// The relative order of the elements should be kept the same.
// Then return the number of unique elements in nums.
//
// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
// - Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially.
//   The remaining elements of nums are not important as well as the size of nums.
// - Return k.

pub fn solution(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut next_non_duplicate_position: i32 = 1;
    println!("initial: {:?}", nums);
    for idx in 1..nums.len() {
        if nums[idx] != nums[next_non_duplicate_position as usize - 1] {
            println!(
                "{} from idx {} moved to {}",
                nums[idx], idx, next_non_duplicate_position
            );
            nums[next_non_duplicate_position as usize] = nums[idx];
            next_non_duplicate_position += 1;
        }
        println!("{:?}", nums);
    }

    next_non_duplicate_position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(solution(&mut nums), 2);
        assert_eq!(nums[..2], vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(solution(&mut nums), 5);
        assert_eq!(nums[..5], vec![0, 1, 2, 3, 4]);
    }
}

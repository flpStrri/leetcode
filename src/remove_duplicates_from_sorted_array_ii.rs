// # Remove Duplicates from Sorted Array II
// Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice.
// The relative order of the elements should be kept the same.
//
// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums.
// More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result.
// It does not matter what you leave beyond the first k elements.
//
// Return k after placing the final result in the first k slots of nums.
//
// Do not allocate extra space for another array.
// You must do this by modifying the input array in-place with O(1) extra memory.

pub fn solution(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut next_non_duplicate_position: i32 = 1;
    let mut current_duplication_count: i32 = 1;

    for idx in 1..nums.len() {
        if nums[idx] != nums[next_non_duplicate_position as usize - 1] {
            nums[next_non_duplicate_position as usize] = nums[idx];
            next_non_duplicate_position += 1;
            current_duplication_count = 1;
        } else if current_duplication_count < 2 {
            nums[next_non_duplicate_position as usize] = nums[idx];
            next_non_duplicate_position += 1;
            current_duplication_count += 1;
        }
    }
    next_non_duplicate_position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(solution(&mut nums), 5);
        assert_eq!(nums[..5], vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(solution(&mut nums), 7);
        assert_eq!(nums[..7], vec![0, 0, 1, 1, 2, 3, 3]);
    }
}

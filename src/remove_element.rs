// # Remove Element
// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
// The order of the elements may be changed.
// Then return the number of elements in nums which are not equal to val.
//
// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:
//
// - Change the array nums such that the first k elements of nums contain the elements which are not equal to val.
//   The remaining elements of nums are not important as well as the size of nums.
// - Return k.

pub fn solution(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut next_non_target_position: i32 = 0;
    for idx in 0..nums.len() {
        if nums[idx] != val {
            nums[next_non_target_position as usize] = nums[idx];
            next_non_target_position += 1
        }
    }
    next_non_target_position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(solution(&mut nums, val), 2);
        assert_eq!(nums[..2], vec![2, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        assert_eq!(solution(&mut nums, val), 5);
        assert_eq!(nums[..5], vec![0, 1, 3, 0, 4]);
    }
}

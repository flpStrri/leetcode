// # Rotate Array
// Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let nums_len = nums.len();
    if nums_len <= 1 {
        return;
    }

    let k = (k as usize) % nums_len;
    if k == 0 {
        return;
    }

    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        rotate(&mut nums, k);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }
}

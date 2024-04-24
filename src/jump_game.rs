// Jump Game
// You are given an integer array nums.
// You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.
//
// Return true if you can reach the last index, or false otherwise.

fn solution(nums: Vec<i32>) -> bool {
    let mut jump_potential: i32 = nums[0];

    for jump_size in nums {
        if jump_potential < 0 {
            return false;
        }

        if jump_size > jump_potential {
            jump_potential = jump_size
        }

        jump_potential -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(solution(nums), true);
    }

    #[test]
    fn example_2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(solution(nums), false);
    }
}

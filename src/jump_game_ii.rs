// # Jump Game II
// You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].

// Each element nums[i] represents the maximum length of a forward jump from index i.
// In other words, if you are at nums[i], you can jump to any nums[i + j] where:
// - 0 <= j <= nums[i] and
// - i + j < n
//
// Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].

pub fn solution(nums: Vec<i32>) -> i32 {
    let final_position = (nums.len() - 1) as i32;

    let mut jump_count: i32 = 0;
    let mut farthest_position: i32 = 0;
    let mut farthest_jump : i32 = 0;

    if final_position == 0 {
        return 0
    }

    for position in 0..nums.len() {
        farthest_jump = i32::max(farthest_jump, position as i32 + nums[position]);

        if farthest_jump >= final_position {
            jump_count += 1;
            break
        }

        if position as i32 == farthest_position {
            farthest_position = farthest_jump;
            jump_count += 1;
            farthest_jump = 0;
        }

    }
    jump_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(solution(nums), 2);
    }

    #[test]
    fn example_2() {
        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(solution(nums), 2);
    }

    #[test]
    fn example_3() {
        let nums = vec![2,1];
        assert_eq!(solution(nums), 1)
    }

    #[test]
    fn example_4() {
        let nums = vec![1,1,1,1];
        assert_eq!(solution(nums), 3)
    }

    #[test]
    fn example_5() {
        let nums = vec![3,4,3,2,5,4,3];
        assert_eq!(solution(nums), 3)
    }

    #[test]
    fn example_6() {
        let nums = vec![2,9,6,5,7,0,7,2,7,9,3,2,2,5,7,8,1,6,6,6,3,5,2,2,6,3];
        assert_eq!(solution(nums), 5)
    }

    #[test]
    fn example_7() {
        let nums = vec![0];
        assert_eq!(solution(nums), 0)
    }
}
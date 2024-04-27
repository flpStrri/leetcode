// # Product of Array Except Self
//
// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
//
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//
// You must write an algorithm that runs in O(n) time and without using the division operation.

pub fn solution(nums: Vec<i32>) -> Vec<i32> {
    let nums_size = nums.len();
    let mut prefix = vec![0; nums_size];
    let mut suffix = vec![0; nums_size];

    let mut answer = vec![1; nums_size];
    prefix[0] = 1;
    suffix[nums_size - 1] = 1;

    for idx in 1..nums_size {
        let prefix_index = idx;
        let suffix_index = nums_size - 1 - idx;

        prefix[prefix_index] = prefix[prefix_index - 1] * nums[prefix_index - 1];
        suffix[suffix_index] = suffix[suffix_index + 1] * nums[suffix_index + 1];

        answer[prefix_index] *= prefix[prefix_index];
        answer[suffix_index] *= suffix[suffix_index];
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3, 4];
        let result = solution(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }

    #[test]
    fn example_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let result = solution(nums);
        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }
}

// # Merge Sorted Array
// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
//
// Merge `nums1` and `nums2` into a single array sorted in non-decreasing order.
//
// The final sorted array should **not** be returned by the function, but instead be stored inside the array `nums1`.
// To accommodate this, `nums1` has a length of `m + n`, where the first `m` elements denote the elements that should be merged, and the last `n` elements are set to `0` and should be ignored. `nums2` has a length of `n`.
//
// ## Constraints:
// - `nums1.length == m + n`
// - `nums2.length == n`
// - `0 <= m, n <= 200`
// - `1 <= m + n <= 200`
// - `-109 <= nums1[i], nums2[j] <= 109`

fn solution(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    if n == 0 {
        return;
    }

    let mut nums2_index = n;
    let mut nums1_index = m;

    let mut last_num_index = nums1.len() - 1;

    while nums2_index > 0 {
        if nums1_index > 0 && nums1[(nums1_index - 1) as usize] > nums2[(nums2_index - 1) as usize]
        {
            nums1[last_num_index] = nums1[(nums1_index - 1) as usize];
            nums1_index -= 1;
        } else {
            nums1[last_num_index] = nums2[(nums2_index - 1) as usize];
            nums2_index -= 1;
        }
        last_num_index -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        solution(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1[..6], vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        solution(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1[..1], vec![1]);
    }

    // #[test]
    // fn example_3() {
    //     let mut nums1 = vec![0];
    //     let m = 0;
    //     let mut nums2 = vec![1];
    //     let n = 1;
    //     solution(&mut nums1, m, &mut nums2, n);
    //     assert_eq!(nums1[..1], vec![1]);
    // }
}

// # Trapping Rain Water
// Given `n` non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

pub fn solution(height: Vec<i32>) -> i32 {
    let max_left_block = calculate_left_water_blocks(&height);
    let max_right_block = calculate_right_water_blocks(&height);

    calculate_trapped_water(&height, &max_left_block, &max_right_block)
}

fn calculate_left_water_blocks(height: &[i32]) -> Vec<i32> {
    let mut max_left_block = vec![0; height.len()];
    for idx in 1..height.len() {
        max_left_block[idx] = i32::max(height[idx-1], max_left_block[idx-1])
    }
    max_left_block
}

fn calculate_right_water_blocks(height: &[i32]) -> Vec<i32> {
    let mut max_right_block = vec![0; height.len()];
    for idx in (0..height.len() - 1).rev() {
        max_right_block[idx] = i32::max(height[idx + 1], max_right_block[idx + 1])
    }
    max_right_block
}

fn calculate_trapped_water(height: &[i32], max_left_block: &[i32], max_right_block: &[i32]) -> i32 {
    let mut trapped_water = 0;
    for (idx, elevation) in height.iter().enumerate() {
        let water_level = i32::min(max_left_block[idx], max_right_block[idx]);


        if water_level < *elevation {
            continue;
        }

        trapped_water += water_level - elevation;
    }
    trapped_water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solution(vec!(0,1,0,2,1,0,1,3,2,1,2,1)), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(solution(vec!(4,2,0,3,2,5)), 9);
    }
}
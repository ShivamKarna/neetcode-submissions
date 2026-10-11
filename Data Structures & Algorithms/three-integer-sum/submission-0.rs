impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut result = Vec::new();

        for i in 0..nums.len().saturating_sub(2) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] > 0 {
                break;
            }
            if nums[i] + nums[i + 1] + nums[i + 2] > 0 {
                break;
            }

            if nums[i] + nums[nums.len() - 2] + nums[nums.len() - 1] < 0 {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;
            let new_target = 0 - nums[i];

            while left < right {
                if nums[left] + nums[right] == new_target {
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if nums[left] + nums[right] < new_target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        result
    }
}

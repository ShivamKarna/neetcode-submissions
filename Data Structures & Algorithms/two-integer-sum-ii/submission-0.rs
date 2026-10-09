impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() -1);

        while left < right {
            let sum = numbers[left] + numbers[right];

            match sum.cmp(&target) {
                std::cmp::Ordering::Equal =>{
                    return vec![(left+1) as i32, (right +1 ) as i32];
                },
                std::cmp::Ordering::Less =>left += 1,
                std::cmp::Ordering::Greater =>right -= 1,
            }
        }

        unreachable!("The function gurantees a unique solution!");
    }
}

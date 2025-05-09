pub mod solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;
        let mut distance = i32::MAX;
        let mut nums = nums;
        let len = nums.len();
        nums.sort_unstable();
        for (idx, value) in nums.iter().enumerate() {
            let (mut left, mut right) = (idx + 1, len - 1);
            while left < right {
                let sum = value + nums[left] + nums[right];
                if sum == target {
                    return target;
                }
                let current_distance = i32::abs(target - sum);
                if distance > current_distance {
                    distance = current_distance;
                    res = sum;
                }
                if sum > target {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::three_sum_closest;

        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}

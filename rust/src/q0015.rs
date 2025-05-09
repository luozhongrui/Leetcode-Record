pub mod solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        let len = nums.len();
        while i < len {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                if let 0 = nums[i] + nums[left] + nums[right] {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                };
                match nums[i] + nums[left] + nums[right] {
                    sum if sum > 0 => {
                        while right > left && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        right -= 1;
                    }
                    _ => {
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        left += 1;
                    }
                }
            }
            while i < len - 1 && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::three_sum;
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}

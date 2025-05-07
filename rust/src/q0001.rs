pub mod solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::<i32, i32>::new();
        for (idx, &val) in nums.iter().enumerate() {
            if let Some(&preidx) = map.get(&(target - val)) {
                return vec![preidx, idx as i32];
            } else {
                map.insert(val, idx as i32);
            }
        }
        panic!("incorrect testcase");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::solution::two_sum;
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6));
    }
}

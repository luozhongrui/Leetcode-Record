pub mod solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut iter = height.iter().enumerate();
        let (mut left, mut right) = (iter.next(), iter.next_back());
        while let (Some((i, h1)), Some((j, h2))) = (left, right) {
            res = res.max(h1.min(h2) * (j - i) as i32);
            if h1 < h2 {
                left = iter.next();
            } else {
                right = iter.next_back();
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
        use solution::max_area;
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}

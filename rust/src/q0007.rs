pub mod solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut res = 0;
        let (min, max) = (i32::MIN / 10, i32::MAX / 10);
        while x != 0 {
            if (res > 0 && res > max) || (res < 0 && res < min) {
                return 0;
            }
            res = res * 10 + x % 10;
            x /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::reverse;
        assert_eq!(123, reverse(321));
        assert_eq!(-123, reverse(-321));
        assert_eq!(21, reverse(120));
    }
}

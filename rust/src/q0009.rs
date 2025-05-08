pub mod solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x = x as i64;
        if x < 0 {
            return false;
        }
        let prev = x;
        let mut res = 0;
        while x != 0 {
            res = res * 10 + x % 10;
            x /= 10;
        }
        res == prev
        // 转换为字符串反转判等
        // x.to_string().chars().rev().eq(x.to_string().chars())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::is_palindrome;
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
    }
}

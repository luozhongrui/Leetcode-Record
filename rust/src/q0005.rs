pub mod solution {
    /// 思路:枚举中点向两边扩散
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let arr: Vec<char> = s.chars().collect();
        let (mut start, mut end) = (0i32, 0i32);
        let len = arr.len() - 1;
        for idx in 0..len {
            let idx = idx as i32;
            let mut callback = |mut min, mut max| {
                while min >= 0 && max <= len as i32 {
                    if arr[min as usize] != arr[max as usize] {
                        return;
                    }
                    if (max - min) > (end - start) {
                        (start, end) = (min, max);
                    }
                    min -= 1;
                    max += 1;
                }
            };
            // 处理偶数长度
            callback(idx, idx + 1);
            // 处理奇数长度
            callback(idx, idx);
        }
        arr[start as usize..=end as usize].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::longest_palindrome;
        assert_eq!("bb".to_string(), longest_palindrome("cbbd".to_string()));
        assert_eq!("bab".to_string(), longest_palindrome("babad".to_string()));
        assert_eq!("".to_string(), longest_palindrome("".to_string()));
        assert_eq!("a".to_string(), longest_palindrome("a".to_string()));
        assert_eq!("aa".to_string(), longest_palindrome("aa".to_string()));
        assert_eq!("a".to_string(), longest_palindrome("ab".to_string()));
    }
}

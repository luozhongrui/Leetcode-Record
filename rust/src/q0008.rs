pub mod solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut res: i64 = 0;
        // 标志位，用来表示是否开始计算数字
        let mut flag = false;
        let mut nagative = false;
        let s = s.trim();
        for c in s.chars() {
            match c {
                num @ '0'..='9' => {
                    flag = true;
                    let num = num.to_digit(10).unwrap() as i64;
                    res = res * 10 + num;
                    // 处理越界
                    if nagative && -res < i32::MIN as i64 {
                        return i32::MIN;
                    }
                    if !nagative && res > i32::MAX as i64 {
                        return i32::MAX;
                    }
                }
                // '+' '-' 只该最先出现一次，之后出现也是非法字符，直接break
                '+' => {
                    if flag {
                        break;
                    }
                    flag = true;
                }
                '-' => {
                    if flag {
                        break;
                    }
                    flag = true;
                    nagative = true;
                }
                _ => {
                    break;
                }
            }
        }
        if nagative {
            res = -res;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::my_atoi;
        assert_eq!(my_atoi("1337c0d3".to_string()), 1337);
        assert_eq!(my_atoi(" -042".to_string()), -42);
        assert_eq!(my_atoi("0-1".to_string()), 0);
        assert_eq!(my_atoi("words and 987".to_string()), 0);
    }
}

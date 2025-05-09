pub mod solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let map: std::collections::HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .into_iter()
        .collect();
        let chars: Vec<_> = s.chars().collect();
        for idx in 0..chars.len() {
            if idx == chars.len() - 1 {
                res = res + map.get(chars.get(idx).unwrap()).unwrap();
            } else {
                let prev_value = map.get(chars.get(idx).unwrap()).unwrap();
                let next_value = map.get(chars.get(idx + 1).unwrap()).unwrap();
                res = res
                    + if prev_value >= next_value {
                        *prev_value
                    } else {
                        -*prev_value
                    };
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
        use solution::roman_to_int;
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}

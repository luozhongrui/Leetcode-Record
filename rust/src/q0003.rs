pub mod solution {
    /// # 题解思路
    /// 迭代字符串，维持一个到当前字符不重复的序列，并不断取最大值
    /// 维持方法是每次又重复字符进来就让起始位置移动到和这个字符相同的字符位置
    /// 解法1:使用一个HashSet，需要两层循环，不过内层循环只会做n次，因此算法复杂度还是O(n)
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        let mut res = 0;
        let mut idx = 0;
        let mut set: HashSet<char> = HashSet::new();
        for c in s.chars() {
            if set.contains(&c) {
                for prev in s.chars().skip(idx) {
                    idx += 1;
                    set.remove(&prev);
                    if prev == c {
                        break;
                    }
                }
            }
            set.insert(c);
            res = res.max(set.len());
        }
        res as i32
    }

    /// 解法2:由于维持不重复序列每次都要删除到前一个相同字符的位置，因此可以使用HashMap来记录位置，这样就不需要两层循环
    pub fn length_of_longest_substring_2(s: String) -> i32 {
        use std::collections::HashMap;
        let mut res = 0;
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut start = -1;
        for (idx, c) in s.chars().enumerate() {
            if let Some(last) = map.insert(c, idx as i32) {
                start = i32::max(start, last);
            }
            res = res.max(idx as i32 - start);
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::length_of_longest_substring;
        use solution::length_of_longest_substring_2;
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring_2("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring_2("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring_2("pwwkew".to_string()), 3);
    }
}

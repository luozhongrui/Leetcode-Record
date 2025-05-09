pub mod solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let map = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
        for c in digits.chars() {
            let chars = map.get(&c).unwrap().chars().collect::<Vec<_>>();
            let tmp = res;
            res = vec![];
            for c in chars.iter() {
                for mut prev in tmp.clone() {
                    prev.push(*c);
                    res.push(prev);
                }
            }
            if res.is_empty() {
                res = chars.into_iter().map(|c| c.to_string()).collect();
            }
        }
        res.sort();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::letter_combinations;
        assert_eq!(
            letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}

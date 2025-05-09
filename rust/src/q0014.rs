pub mod solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = "".to_string();
        let mut iters: Vec<_> = strs.iter().map(|str| str.chars().into_iter()).collect();
        let Some(mut first_iter) = iters.pop() else {
            return res;
        };
        'out: while let Some(c) = first_iter.next() {
            for iter in &mut iters {
                let Some(oc) = iter.next() else {
                    break 'out;
                };
                if oc != c {
                    break 'out;
                }
            }
            res.push(c);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use solution::longest_common_prefix;
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }
}

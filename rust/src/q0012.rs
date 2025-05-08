pub mod solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut tuples = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut res = "".to_string();
        while num > 0 {
            for tuple in tuples.iter() {
                while num >= tuple.0 {
                    num -= tuple.0;
                    res.push_str(tuple.1);
                }
            }
            tuples.remove(0);
        }
        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        use solution::int_to_roman;
        assert_eq!(int_to_roman(3749), "MMMDCCXLIX".to_string());
        assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
    }
}

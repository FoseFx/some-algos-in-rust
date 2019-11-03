
/// Excel columns follow the following pattern: column 1 is named "A", column 2 is "B", column 26 is "Z", column 27 is "AA" and so forth
/// Generates an excel column name for a given u32
pub fn column_name(number: u32) -> String {
    let mut number = number - 1; // 0 indexed
    let mut string = "".to_string(); // this will be returned
    loop {
        let quotient = number / 26;
        let remainder = (number % 26) as u8;
        const A: u8 = 65; // ASCII value for an A
        string = format!("{}{}", char::from(A + remainder), string);
        if quotient == 0 {
            return string;
        }
        number = quotient - 1;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_column_name() {
        assert_eq!(column_name(26), "Z");
        assert_eq!(column_name(51), "AY");
        assert_eq!(column_name(52), "AZ");
        assert_eq!(column_name(676), "YZ");
        assert_eq!(column_name(702), "ZZ");
        assert_eq!(column_name(704), "AAB");
    }
}
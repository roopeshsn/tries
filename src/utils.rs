fn decimal_to_binary(decimal_number: i32) -> String {
    return format!("{:b}", decimal_number)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn convert_decimal_to_binary() {
        assert_eq!(decimal_to_binary(10), "1010");
    }
}

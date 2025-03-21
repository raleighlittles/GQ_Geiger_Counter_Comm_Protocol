/// Takes a number, converts it to a percentage of the maximum representable by that data type.
/// For example, if this were an 8-bit integer, and you gave me "54" as input, I would return 42, 
/// because 54 is 42% of 127 (the largest signed integer representable with 8 bits)
pub fn convert_to_percentage_of_max(base_value: u16) -> i16 {
    return (((base_value as f32 / i16::MAX as f32) * 100.0).round()) as i16;
}

#[cfg(test)]
mod helpers_test {
    use super::*;

    #[test]
    fn test_convert_to_percentage_of_max() {
        assert_eq!(convert_to_percentage_of_max(5000), 15);
    }
}

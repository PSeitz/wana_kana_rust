use crate::constants::{UPPERCASE_END, UPPERCASE_START};
use crate::utils::is_char_in_range::*;

/// Tests if char is in English unicode uppercase range
#[inline]
pub fn is_char_upper_case(char: char) -> bool {
    is_char_in_range(char, UPPERCASE_START, UPPERCASE_END)
}

#[test]
fn is_char_upper_case_test() {
    assert_eq!(is_char_upper_case('A'), true);
    assert_eq!(is_char_upper_case('D'), true);
    assert_eq!(is_char_upper_case('-'), false);
    assert_eq!(is_char_upper_case('ー'), false);
    assert_eq!(is_char_upper_case('a'), false);
    assert_eq!(is_char_upper_case('d'), false);
}

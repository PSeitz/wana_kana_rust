#[allow(dead_code)]

/// Converts all fullwidth roman letters in string to proper ASCII
///
/// `text` Full Width roman letters
///
use crate::constants::UPPERCASE_START;
use std;
use crate::utils::is_char_in_range::*;

pub const LOWERCASE_START: u32 = 0x61;
//pub const LOWERCASE_END: u32 = 0x7A;

#[allow(dead_code)]
pub const LOWERCASE_FULLWIDTH_START: u32 = 0xFF41;
#[allow(dead_code)]
pub const LOWERCASE_FULLWIDTH_END: u32 = 0xFF5A;
#[allow(dead_code)]
pub const UPPERCASE_FULLWIDTH_START: u32 = 0xFF21;
#[allow(dead_code)]
pub const UPPERCASE_FULLWIDTH_END: u32 = 0xFF3A;

#[allow(dead_code)]
pub fn convert_fullwidth_chars_to_ascii(text: &str) -> String {
    let ascii_chars = text.chars().map(|char| {
        let code = char as u32;
        let lower = is_char_in_range(char, LOWERCASE_FULLWIDTH_START, LOWERCASE_FULLWIDTH_END);
        let upper = is_char_in_range(char, UPPERCASE_FULLWIDTH_START, UPPERCASE_FULLWIDTH_END);
        if lower {
            std::char::from_u32((code - LOWERCASE_FULLWIDTH_START) + LOWERCASE_START)
                .unwrap()
                .to_string();
        } else if upper {
            std::char::from_u32((code - UPPERCASE_FULLWIDTH_START) + UPPERCASE_START)
                .unwrap()
                .to_string();
        }
        return char;
    });
    ascii_chars.into_iter().collect()
}

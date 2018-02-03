use utils::is_char_in_range::*;

use constants::{UPPERCASE_END, UPPERCASE_START};
/**
 * Tests if char is in English unicode uppercase range
 * @param  {String} char
 * @return {Boolean}
 */
pub fn is_char_upper_case(char: char) -> bool {
    return is_char_in_range(char, UPPERCASE_START, UPPERCASE_END);
}

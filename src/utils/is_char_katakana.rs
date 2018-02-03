use constants::{KATAKANA_END, KATAKANA_START};

use utils::is_char_in_range::*;

/**
 * Tests a character. Returns true if the character is [Katakana](https://en.wikipedia.org/wiki/Katakana).
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_katakana(char: char) -> bool {
    return is_char_in_range(char, KATAKANA_START, KATAKANA_END);
}

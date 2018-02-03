
use utils::is_char_inRange::*;

use constants::{UPPERCASE_START, UPPERCASE_END};
/**
 * Tests if char is in English unicode uppercase range
 * @param  {String} char
 * @return {Boolean}
 */
pub fn is_char_upper_case(char: char) -> bool {
  
  return is_char_inRange(char, UPPERCASE_START, UPPERCASE_END);
}



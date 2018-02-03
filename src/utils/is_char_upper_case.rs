
use utils::is_char_inRange::*;
import {
  UPPERCASE_START,
  UPPERCASE_END,
} from '../constants';

/**
 * Tests if char is in English unicode uppercase range
 * @param  {String} char
 * @return {Boolean}
 */
pub fn is_char_upper_case(char: char) -> bool {
  
  return is_char_inRange(char, UPPERCASE_START, UPPERCASE_END);
}

export default is_char_upperCase;


use utils::isCharInRange::*;
import {
  UPPERCASE_START,
  UPPERCASE_END,
} from '../constants';

/**
 * Tests if char is in English unicode uppercase range
 * @param  {String} char
 * @return {Boolean}
 */
pub fn is_char_uppercase(char: char) -> bool {
  
  return isCharInRange(char, UPPERCASE_START, UPPERCASE_END);
}

export default isCharUpperCase;

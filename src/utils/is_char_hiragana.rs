
use utils::is_char_longDash::*;
use utils::is_char_inRange::*;
import {
  HIRAGANA_START,
  HIRAGANA_END,
} from '../constants';

/**
 * Tests a character. Returns true if the character is [Hiragana](https://en.wikipedia.org/wiki/Hiragana).
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_hiragana(char: char) -> bool {
  
  if (is_char_longDash(char)) return true;
  return is_char_inRange(char, HIRAGANA_START, HIRAGANA_END);
}



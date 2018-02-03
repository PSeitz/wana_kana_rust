
import isCharLongDash from './isCharLongDash';
use utils::isCharInRange::*;
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
  
  if (isCharLongDash(char)) return true;
  return isCharInRange(char, HIRAGANA_START, HIRAGANA_END);
}

export default isCharHiragana;

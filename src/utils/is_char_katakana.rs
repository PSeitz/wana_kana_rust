import {
  KATAKANA_START,
  KATAKANA_END,
} from '../constants';

use utils::isCharInRange::*;

/**
 * Tests a character. Returns true if the character is [Katakana](https://en.wikipedia.org/wiki/Katakana).
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn isCharKatakana(char: char) -> bool {
  return isCharInRange(char, KATAKANA_START, KATAKANA_END);
}

export default isCharKatakana;

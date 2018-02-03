use utils::is_char_inRange::*;
import { JAPANESE_RANGES } from '../constants';

/**
 * Tests a character. Returns true if the character is [Katakana](https://en.wikipedia.org/wiki/Katakana).
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_japanese(char: char) -> bool {
  return JAPANESE_RANGES.some(([start, end]) => is_char_inRange(char, start, end));
}



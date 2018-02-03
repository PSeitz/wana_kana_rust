
import { JA_PUNCTUATION_RANGES } from '../constants';
use utils::isCharInRange::*;

/**
 * Tests a character. Returns true if the character is considered English punctuation.
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_japanesepunctuation(char: char) -> bool {
  
  return JA_PUNCTUATION_RANGES.some(([start, end]) => isCharInRange(char, start, end));
}

export default isCharJapanesePunctuation;

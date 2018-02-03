
use utils::is_char_inRange::*;
import { ROMAJI_RANGES } from '../constants';

/**
 * Tests a character. Returns true if the character is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_romaji(char: char) -> bool {
  
  return ROMAJI_RANGES.some(([start, end]) => is_char_inRange(char, start, end));
}



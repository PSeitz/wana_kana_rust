
use utils::is_char_inRange::*;
use constants::ROMAJI_RANGES;

/**
 * Tests a character. Returns true if the character is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_romaji(char: char) -> bool {
  
  return ROMAJI_RANGES.iter().any(([start, end]) => is_char_inRange(char, start, end));
}



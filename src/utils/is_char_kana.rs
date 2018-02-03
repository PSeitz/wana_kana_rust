
use utils::is_char_hiragana::*;
use utils::is_char_katakana::*;

/**
 * Tests a character. Returns true if the character is [Hiragana](https://en.wikipedia.org/wiki/Hiragana) or [Katakana](https://en.wikipedia.org/wiki/Katakana).
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_kana(char: char) -> bool {
  
  return is_char_hiragana(char) || is_char_katakana(char);
}



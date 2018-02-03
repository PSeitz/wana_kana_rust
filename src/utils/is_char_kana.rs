
import isCharHiragana from './isCharHiragana';
import isCharKatakana from './isCharKatakana';

/**
 * Tests a character. Returns true if the character is [Hiragana](https://en.wikipedia.org/wiki/Hiragana) or [Katakana](https://en.wikipedia.org/wiki/Katakana).
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_kana(char: char) -> bool {
  
  return isCharHiragana(char) || isCharKatakana(char);
}

export default isCharKana;

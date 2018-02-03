use constants::{KATAKANA_START, HIRAGANA_START};
use utils::is_char_long_dash::*;
use utils::is_char_slash_dot::*;
use utils::is_char_hiragana::*;

/**
 * Convert [Hiragana](https://en.wikipedia.org/wiki/Hiragana) to [Katakana](https://en.wikipedia.org/wiki/Katakana)
 * Passes through any non-hiragana chars
 * @param  {String} [input=''] text input
 * @return {String} converted text
 * @example
 * hiragana_to_katakana('ひらがな')
 * // => "ヒラガナ"
 * hiragana_to_katakana('ひらがな is a type of kana')
 * // => "ヒラガナ is a type of kana"
 * @ignore
 */
fn hiragana_to_katakana(input: &str) {
  let kata = vec![];
  input.chars().for_each((char) => {
    // Short circuit to avoid incorrect codeshift for 'ー' and '・'
    if (is_char_long_dash(char) || is_char_slash_dot(char)) {
      kata.push(char);
    } else if (is_char_hiragana(char)) {
      // Shift charcode.
      let code = char as u32 + (KATAKANA_START - HIRAGANA_START);
      let kata_char = String.from_char_code(code);
      kata.push(kata_char);
    } else {
      // Pass non-hiragana chars through
      kata.push(char);
    }
  });
  return kata.join('');
}



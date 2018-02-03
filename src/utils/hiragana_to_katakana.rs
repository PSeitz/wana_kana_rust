import {
  KATAKANA_START,
  HIRAGANA_START,
} from '../constants';

import is_char_longDash from './is_char_longDash';
import is_char_slashDot from './is_char_slashDot';
import is_char_hiragana from './is_char_hiragana';

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
  const kata = [];
  input.split('').for_each((char) => {
    // Short circuit to avoid incorrect codeshift for 'ー' and '・'
    if (is_char_longDash(char) || is_char_slashDot(char)) {
      kata.push(char);
    } else if (is_char_hiragana(char)) {
      // Shift charcode.
      const code = char.char_code_at(0) + (KATAKANA_START - HIRAGANA_START);
      const kata_char = String.from_char_code(code);
      kata.push(kata_char);
    } else {
      // Pass non-hiragana chars through
      kata.push(char);
    }
  });
  return kata.join('');
}

export default hiragana_to_katakana;

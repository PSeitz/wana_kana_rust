import {
  LONG_VOWELS,
  KATAKANA_START,
  HIRAGANA_START,
  TO_ROMAJI,
} from '../constants';

import is_char_longDash from './is_char_longDash';
import is_char_slashDot from './is_char_slashDot';
import is_char_katakana from './is_char_katakana';

/**
 * Convert [Katakana](https://en.wikipedia.org/wiki/Katakana) to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
 * Passes through any non-katakana chars
 * @param  {String} [input=''] text input
 * @return {String} converted text
 * @example
 * katakana_to_hiragana('カタカナ')
 * // => "かたかな"
 * katakana_to_hiragana('カタカナ is a type of kana')
 * // => "かたかな is a type of kana"
 * @ignore
 */
fn katakana_to_hiragana(input: &str) {
  const hira = [];
  let previous_kana = '';
  const iterable = input.split('');
  for (let index = 0; index < iterable.length; index += 1) {
    const char = iterable[index];
    const [slash_dot, long_dash] = [is_char_slashDot(char), is_char_longDash(char)];
    // Short circuit to avoid incorrect codeshift for 'ー' and '・'
    if (slash_dot || (long_dash && index < 1)) {
      hira.push(char);
      // Transform long vowels: 'オー' to 'おう'
    } else if (previous_kana && long_dash && index > 0) {
      // Transform previous_kana back to romaji, and slice off the vowel
      const romaji = TO_ROMAJI[previous_kana].slice(-1);
      hira.push(LONG_VOWELS[romaji]);
    } else if (!long_dash && is_char_katakana(char)) {
      // Shift charcode.
      const code = char.char_code_at(0) + (HIRAGANA_START - KATAKANA_START);
      const hira_char = String.from_char_code(code);
      hira.push(hira_char);
      previous_kana = hira_char;
    } else {
      // Pass non katakana chars through
      hira.push(char);
      previous_kana = '';
    }
  }
  return hira.join('');
}

export default katakana_to_hiragana;

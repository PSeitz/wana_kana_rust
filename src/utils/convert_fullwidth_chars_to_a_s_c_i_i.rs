import {
  LOWERCASE_START,
  UPPERCASE_START,
  LOWERCASE_FULLWIDTH_START,
  LOWERCASE_FULLWIDTH_END,
  UPPERCASE_FULLWIDTH_START,
  UPPERCASE_FULLWIDTH_END,
} from '../constants';

use utils::is_char_inRange::*;

/**
 * Converts all fullwidth roman letters in string to proper ASCII
 * @param  {String} text Full Width roman letters
 * @return {String} ASCII
 */
fn convert_fullwidth_charstoascii(text = '') {
  const ascii_chars = [...text].map((char, index) => {
    const code = char.char_code_at(0);
    const lower = is_char_inRange(char, LOWERCASE_FULLWIDTH_START, LOWERCASE_FULLWIDTH_END);
    const upper = is_char_inRange(char, UPPERCASE_FULLWIDTH_START, UPPERCASE_FULLWIDTH_END);
    if (lower) {
      return String.from_char_code((code - LOWERCASE_FULLWIDTH_START) + LOWERCASE_START);
    } else if (upper) {
      return String.from_char_code((code - UPPERCASE_FULLWIDTH_START) + UPPERCASE_START);
    }
    return char;
  });
  return ascii_chars.join('');
}

export default convert_fullwidth_chars_to_aSCII;

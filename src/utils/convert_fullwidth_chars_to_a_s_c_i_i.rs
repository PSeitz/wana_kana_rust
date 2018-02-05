
use constants::{LOWERCASE_START, UPPERCASE_START, LOWERCASE_FULLWIDTH_START, LOWERCASE_FULLWIDTH_END, UPPERCASE_FULLWIDTH_START, UPPERCASE_FULLWIDTH_END,};

use utils::is_char_in_range::*;


///Converts all fullwidth roman letters in string to proper ASCII
///
///@param  {String} text Full Width roman letters
///

fn convert_fullwidth_charstoascii(text = '') {
  let ascii_chars = [...text].map((char, index) => {
    let code = char as u32;
    let lower = is_char_in_range(char, LOWERCASE_FULLWIDTH_START, LOWERCASE_FULLWIDTH_END);
    let upper = is_char_in_range(char, UPPERCASE_FULLWIDTH_START, UPPERCASE_FULLWIDTH_END);
    if (lower) {
      return String.from_char_code((code - LOWERCASE_FULLWIDTH_START) + LOWERCASE_START);
    } else if (upper) {
      return String.from_char_code((code - UPPERCASE_FULLWIDTH_START) + UPPERCASE_START);
    }
    return char;
  });
  return ascii_chars.join('');
}



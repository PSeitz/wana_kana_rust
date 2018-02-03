
use constants::KANA_SLASH_DOT;

/**
 * Tests if char is '・'
 * @param  {String} char
 * @return {Boolean} true if '・'
 */
pub fn is_char_slash_dot(char: char) -> bool {
  
  return char.char_code_at(0) === KANA_SLASH_DOT;
}



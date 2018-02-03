
import { KANA_SLASH_DOT } from '../constants';

/**
 * Tests if char is '・'
 * @param  {String} char
 * @return {Boolean} true if '・'
 */
pub fn is_char_slash_dot(char: char) -> bool {
  
  return char.charCodeAt(0) === KANA_SLASH_DOT;
}

export default isCharSlashDot;

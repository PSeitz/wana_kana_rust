
import { PROLONGED_SOUND_MARK } from '../constants';

/**
 * Returns true if char is 'ー'
 * @param  {String} char to test
 * @return {Boolean}
 */
pub fn is_char_longdash(char: char) -> bool {
  
  return char.char_code_at(0) === PROLONGED_SOUND_MARK;
}

export default is_char_longDash;


use constants::PROLONGED_SOUND_MARK;

/**
 * Returns true if char is 'ー'
 * @param  {String} char to test
 * @return {Boolean}
 */
pub fn is_char_longdash(char: char) -> bool {
  
  return char.char_code_at(0) === PROLONGED_SOUND_MARK;
}



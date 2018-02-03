
import is_char_englishPunctuation from './is_char_englishPunctuation';
import is_char_japanesePunctuation from './is_char_japanesePunctuation';

/**
 * Tests a character. Returns true if the character is considered Japanese or English punctuation.
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_punctuation(char: char) -> bool {
  
  return is_char_englishPunctuation(char) || is_char_japanesePunctuation(char);
}

export default is_char_punctuation;

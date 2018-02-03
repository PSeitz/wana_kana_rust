
use utils::is_char_japanese::*;

/**
 * Test if `input` only includes [Kanji](https://en.wikipedia.org/wiki/Kanji), [Kana](https://en.wikipedia.org/wiki/Kana), zenkaku punctuation, japanese symbols and numbers.”
 * @param  {String} [input=''] text
 * @return {Boolean} true if passes checks
 * @example
 * is_japanese('泣き虫')
 * // => true
 * is_japanese('あア')
 * // => true
 * is_japanese('２月1日') // Full and half-width numbers allowed
 * // => true
 * is_japanese('泣き虫。！〜＄')
 * // => true
 * is_japanese('泣き虫.!~$') // Half-width / Latin punctuation fails
 * // => false
 * is_japanese('A泣き虫')
 * // => false
 * is_japanese('A')
 * // => false
 */
pub fn is_japanese(input: &str) -> bool {
  if input.is_empty(){return false;}
  return input.chars().all(is_char_japanese);
}

export default is_japanese;

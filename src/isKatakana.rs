
use utils::isCharKatakana::*;

/**
 * Test if `input` is [Katakana](https://en.wikipedia.org/wiki/Katakana)
 * @param  {String} [input=''] text
 * @return {Boolean} true if all [Katakana](https://en.wikipedia.org/wiki/Katakana)
 * @example
 * isKatakana('ゲーム')
 * // => true
 * isKatakana('あ')
 * // => false
 * isKatakana('A')
 * // => false
 * isKatakana('あア')
 * // => false
 */
pub fn isKatakana(input: &str) -> bool {
  if input.is_empty(){return false;}
  return input.chars().all(isCharKatakana);
}

export default isKatakana;

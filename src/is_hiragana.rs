
use utils::isCharHiragana::*;

/**
 * Test if `input` is [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
 * @param  {String} [input=''] text
 * @return {Boolean} true if all [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
 * @example
 * isHiragana('げーむ')
 * // => true
 * isHiragana('A')
 * // => false
 * isHiragana('あア')
 * // => false
 */
pub fn isHiragana(input: &str) -> bool {
  if input.is_empty(){return false;}
  return input.chars().all(isCharHiragana);
}

export default isHiragana;

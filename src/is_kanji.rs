
use utils::is_char_kanji::*;

/**
 * Tests if `input` is [Kanji](https://en.wikipedia.org/wiki/Kanji) ([Japanese CJK ideographs](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs))
 * @param  {String} [input=''] text
 * @return {Boolean} true if all [Kanji](https://en.wikipedia.org/wiki/Kanji)
 * @example
 * is_kanji('刀')
 * // => true
 * is_kanji('切腹')
 * // => true
 * is_kanji('勢い')
 * // => false
 * is_kanji('あAア')
 * // => false
 * is_kanji('🐸')
 * // => false
 */
pub fn is_kanji(input: &str) -> bool {
  if input.is_empty(){return false;}
  return input.chars().all(is_char_kanji);
}


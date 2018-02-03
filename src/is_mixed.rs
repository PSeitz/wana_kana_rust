use is_kanji::*;
use is_hiragana::*;
use is_katakana::*;
use is_romaji::*;

/**
 * Test if `input` contains a mix of [Romaji](https://en.wikipedia.org/wiki/Romaji) *and* [Kana](https://en.wikipedia.org/wiki/Kana), defaults to pass through [Kanji](https://en.wikipedia.org/wiki/Kanji)
 * @param  {String} input text
 * @param  {Object} [options={ pass_kanji: true }] optional config to pass through kanji
 * @return {Boolean} true if mixed
 * @example
 * is_mixed('Abあア'))
 * // => true
 * is_mixed('お腹A'))
 * // => true
 * is_mixed('お腹A', { pass_kanji: false }))
 * // => false
 * is_mixed('ab'))
 * // => false
 * is_mixed('あア'))
 * // => false
 */
pub fn is_mixed(input: &str, options = { passkanji: true }) -> bool {
  const chars = [...input];
  let has_kanji = false;
  if (!options.pass_kanji) {
    has_kanji = chars.iter().any(is_kanji);
  }
  return (chars.iter().any(is_hiragana) || chars.iter().any(is_katakana)) && chars.iter().any(is_romaji) && !has_kanji;
}



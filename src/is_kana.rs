
use utils::is_char_kana::*;

/**
 * Test if `input` is [Kana](https://en.wikipedia.org/wiki/Kana) ([Katakana](https://en.wikipedia.org/wiki/Katakana) and/or [Hiragana](https://en.wikipedia.org/wiki/Hiragana))
 * @param  {String} [input=''] text
 * @return {Boolean} true if all [Kana](https://en.wikipedia.org/wiki/Kana)
 * @example
 * is_kana('あ')
 * // => true
 * is_kana('ア')
 * // => true
 * is_kana('あーア')
 * // => true
 * is_kana('A')
 * // => false
 * is_kana('あAア')
 * // => false
 */
pub fn is_kana(input: &str) -> bool {
  if input.is_empty(){return false;}
  return input.chars().all(is_char_kana);
}

#[test]
fn check_is_kanji() {
    assert_eq!(is_kana("あ"), true);
    assert_eq!(is_kana("ア"), true);
    assert_eq!(is_kana("あーア"), true);
    assert_eq!(is_kana("A"), false);
    assert_eq!(is_kana("あAア"), false);
}
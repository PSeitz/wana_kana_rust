use utils::is_char_hiragana::*;

/**
 * Test if `input` is [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
 * @param  {String} [input=''] text
 * @return {Boolean} true if all [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
 * @example
 * is_hiragana('げーむ')
 * // => true
 * is_hiragana('A')
 * // => false
 * is_hiragana('あア')
 * // => false
 */
pub fn is_hiragana(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    return input.chars().all(is_char_hiragana);
}

#[test]
fn check_is_hiragana() {
    assert_eq!(is_hiragana("げーむ"), true);
    assert_eq!(is_hiragana("A"), false);
    assert_eq!(is_hiragana("あア"), false);
}

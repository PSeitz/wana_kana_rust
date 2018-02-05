use utils::is_char_katakana::*;


///Test if `input` is [Katakana](https://en.wikipedia.org/wiki/Katakana)
///@param  {String} [input=''] text
///@return {Boolean} true if all [Katakana](https://en.wikipedia.org/wiki/Katakana)
///@example
///is_katakana('ゲーム')
/// => true
///is_katakana('あ')
/// => false
///is_katakana('A')
/// => false
///is_katakana('あア')
/// => false

pub fn is_katakana(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    return input.chars().all(is_char_katakana);
}

#[test]
fn check_is_katakana() {
    assert_eq!(is_katakana("ゲーム"), true);
    assert_eq!(is_katakana("あ"), false);
    assert_eq!(is_katakana("A"), false);
    assert_eq!(is_katakana("あア"), false);
}

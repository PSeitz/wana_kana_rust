
//! Test if all chars of `input` are [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
//!
//! # Examples
//! ```
//! use wana_kana::is_hiragana::*;
//! assert_eq!(is_hiragana("げーむ"), true);
//! assert_eq!(is_hiragana("A"), false);
//! assert_eq!(is_hiragana("あア"), false);
//! ```

use utils::is_char_hiragana::*;

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

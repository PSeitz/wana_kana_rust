//! Test if all chars of `input` are [Kana](https://en.wikipedia.org/wiki/Kana) ([Katakana](https://en.wikipedia.org/wiki/Katakana) and/or [Hiragana](https://en.wikipedia.org/wiki/Hiragana))
//!
//! # Examples
//! ```
//! use wana_kana::is_kana::*;
//! assert_eq!(is_kana("あ"), true);
//! assert_eq!(is_kana("ア"), true);
//! assert_eq!(is_kana("あーア"), true);
//! assert_eq!(is_kana("A"), false);
//! assert_eq!(is_kana("あAア"), false);
//! ```

use crate::utils::is_char_kana::*;

pub fn is_kana(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    return input.chars().all(is_char_kana);
}

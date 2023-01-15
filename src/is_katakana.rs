//! Test if all chars of `input` are [Katakana](https://en.wikipedia.org/wiki/Katakana)
//!
//! # Examples
//! ```
//! use wana_kana::is_katakana::*;
//! assert_eq!(is_katakana("ゲーム"), true);
//! assert_eq!(is_katakana("あ"), false);
//! assert_eq!(is_katakana("A"), false);
//! assert_eq!(is_katakana("あア"), false);
//! ```

use crate::utils::is_char_katakana::*;

pub fn is_katakana(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(is_char_katakana)
}

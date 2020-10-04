//! Test if all chars of `input` are [Katakana](https://en.wikipedia.org/wiki/Katakana)
//!
//! # Examples
//! ```
//! use wana_kana::is_katakana::*;
//! assert_eq!(is_katakana("ゲーム"), true);
//! assert_eq!(is_katakana("あ"), false);
//! assert_eq!(is_katakana("A"), false);
//! assert_eq!(is_katakana("あア"), false);
//! assert_eq!(is_katakana(""), true);
//! ```

use crate::utils::is_char_katakana::*;

pub fn is_katakana(input: &str) -> bool {
    input.chars().all(is_char_katakana)
}

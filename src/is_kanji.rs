//! Test if all chars of `input` are [Kanji](https://en.wikipedia.org/wiki/Kanji) ([Japanese CJK ideographs](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs))
//!
//! # Examples
//! ```
//! use wana_kana::is_kanji::*;
//! assert_eq!(is_kanji("刀"), true);
//! assert_eq!(is_kanji("切腹"), true);
//! assert_eq!(is_kanji("勢い"), false);
//! assert_eq!(contains_kanji("勢い"), true);
//! assert_eq!(is_kanji("あAア"), false);
//! assert_eq!(is_kanji("🐸"), false);
//! assert_eq!(contains_kanji("🐸"), false);
//! ```

use crate::utils::is_char_kanji::*;

pub fn is_kanji(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    return input.chars().all(is_char_kanji);
}

pub fn contains_kanji(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    return input.chars().any(is_char_kanji);
}


//! Test if `input` only includes [Kanji](https://en.wikipedia.org/wiki/Kanji), [Kana](https://en.wikipedia.org/wiki/Kana), zenkaku punctuation, japanese symbols and numbers.
//!
//! # Examples
//! ```
//! use wana_kana::is_japanese::*;
//! assert_eq!(is_japanese("泣き虫"), true);
//! assert_eq!(is_japanese("あア"), true);
//! assert_eq!(is_japanese("２月1日"), /* Full and half-width numbers allowed */ true );
//! assert_eq!(is_japanese("泣き虫。！〜＄"), true);
//! assert_eq!(is_japanese("泣き虫.!~$"), /* Half-width / Latin punctuation fails*/ false );
//! assert_eq!(is_japanese("A泣き虫"), false);
//! assert_eq!(is_japanese("A"), false);
//! ```

use utils::is_char_japanese::*;

pub fn is_japanese(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    return input.chars().all(is_char_japanese);
}


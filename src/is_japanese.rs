//! Test if `input` only includes [Kanji](https://en.wikipedia.org/wiki/Kanji), [Kana](https://en.wikipedia.org/wiki/Kana), zenkaku punctuation, japanese symbols and numbers.
//!
//! # Examples
//! ```
//! use wana_kana::is_japanese::*;
//! use regex::Regex;
//! assert_eq!(is_japanese("泣き虫"), true);
//! assert_eq!(is_japanese("あア"), true);
//! assert_eq!(is_japanese("２月"), true); // Zenkaku numbers allowed
//! assert_eq!(is_japanese("泣き虫。！〜＄"), true); // Zenkaku/JA punctuation
//! assert_eq!(is_japanese("泣き虫.!~$"), false); // Latin punctuation fails
//! assert_eq!(is_japanese("A"), false);
//! assert_eq!(is_japanese_with_whitelist("≪偽括弧≫", Some(&Regex::new(r"[≪≫]").unwrap())), true);
//! ```

use regex::Regex;
use crate::utils::is_char_japanese::*;

pub fn is_japanese(input: &str) -> bool {
    is_japanese_with_whitelist(input, None)
}

pub fn is_japanese_with_whitelist(input: &str, allowed: Option<&Regex>) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(|char| {
        let is_jap = is_char_japanese(char);
        if !is_jap{
        if let Some(allowed) = allowed {
                return allowed.is_match(input);
            }
        }
        is_jap
    })

}

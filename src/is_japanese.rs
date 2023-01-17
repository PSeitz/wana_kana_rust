#[cfg(feature = "enable_regex")]
use regex::Regex;

use crate::utils::is_char_japanese::*;

/// Test if `input` only includes [Kanji](https://en.wikipedia.org/wiki/Kanji), [Kana](https://en.wikipedia.org/wiki/Kana), zenkaku punctuation, japanese symbols and japanese numbers.
pub fn is_japanese(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(is_char_japanese)
}

#[cfg(feature = "enable_regex")]
/// Checks if all chars are in the japanese unicode ranges or match the provided regex
pub fn is_japanese_with_whitelist(input: &str, allowed: Option<&Regex>) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(|char| {
        let is_jap = is_char_japanese(char);
        if !is_jap {
            if let Some(allowed) = allowed {
                return allowed.is_match(input);
            }
        }
        is_jap
    })
}

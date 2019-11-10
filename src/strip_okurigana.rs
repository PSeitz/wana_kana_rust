//! Strips trailing [Okurigana](https://en.wikipedia.org/wiki/Okurigana) if `input` is a mix of [Kanji](https://en.wikipedia.org/wiki/Kanji) and [Kana](https://en.wikipedia.org/wiki/Kana)
//!
//! # Examples
//! ```
//! use wana_kana::strip_okurigana::*;
//! use wana_kana::Options;
//! assert_eq!(strip_okurigana_all("踏み込む", false), "踏み込");
//! assert_eq!(strip_okurigana_all("粘り。", true), "粘。");
//! assert_eq!(strip_okurigana_all("お祝い", false), "お祝");
//! assert_eq!(strip_okurigana_all("踏み込む", true), "踏込");
//! assert_eq!(strip_okurigana_all("お祝い", true), "祝");
//! ```

use crate::is_japanese::*;
use crate::is_kana::*;
use crate::is_kanji::*;
use crate::utils::is_char_kana::*;
use crate::utils::is_char_punctuation::*;

pub fn strip_okurigana(input: &str) -> String {
    strip_okurigana_all(input, false)
}
pub fn strip_okurigana_all(input: &str, all: bool) -> String {
    if input.is_empty() || !is_japanese(input) || is_kana(input) {
        return input.to_string();
    }

    if all {
        return input.chars().filter(|char| !is_char_kana(*char)).into_iter().collect();
    }

    // strip trailing only
    let mut reverse_chars = input.chars().rev().collect::<Vec<char>>();

    let mut i = 0;
    while i != reverse_chars.len() {
        let char = reverse_chars[i];
        if is_char_punctuation(char) {
            i += 1;
            continue;
        }

        if !is_kanji(&char.to_string()) {
            reverse_chars.remove(i);
        } else {
            break; // stop when we hit a kanji char
        }
    }

    return reverse_chars.into_iter().rev().collect();
}

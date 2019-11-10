//! Splits input into array of [Kanji](https://en.wikipedia.org/wiki/Kanji), [Hiragana](https://en.wikipedia.org/wiki/Hiragana), [Katakana](https://en.wikipedia.org/wiki/Katakana), and [Romaji](https://en.wikipedia.org/wiki/Romaji) tokens.
//!
//! [Katakana](https://en.wikipedia.org/wiki/Katakana), and [Romaji](https://en.wikipedia.org/wiki/Romaji) tokens.
//!
//! Does not split into parts of speech!
//! # Example
//! ```
//! use wana_kana::tokenize::*;
//! use wana_kana::Options;
//! let empty: Vec<String> = vec![];
//! assert_eq!(tokenize(""), empty);
//! assert_eq!(tokenize("ふふフフ"), vec!["ふふ", "フフ"]);
//! assert_eq!(tokenize("感じ"), vec!["感", "じ"]);
//! assert_eq!(tokenize("私は悲しい"), vec!["私", "は", "悲", "しい"] );
//! assert_eq!(tokenize("what the...私は「悲しい」。"), vec!["what the...", "私", "は", "「", "悲", "しい", "」。", ] );
//! ```

use itertools::Itertools;
use crate::utils::is_char_hiragana::*;
use crate::utils::is_char_japanese_punctuation::*;
use crate::utils::is_char_kanji::*;
use crate::utils::is_char_katakana::*;

fn get_type(input: char) -> &'static str {
    match input {
        input if is_char_japanese_punctuation(input) => "japanese_punctuation",
        input if is_char_kanji(input) => "kanji",
        input if is_char_hiragana(input) => "hiragana",
        input if is_char_katakana(input) => "katakana",
        _ => "romaji",
    }
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut result = vec![];
    for (_, group) in &input.chars().group_by(|elt| get_type(*elt)) {
        result.push(group.collect());
    }
    result
}

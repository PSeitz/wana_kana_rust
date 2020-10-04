//! Strips trailing [Okurigana](https://en.wikipedia.org/wiki/Okurigana) if `input` is a mix of [Kanji](https://en.wikipedia.org/wiki/Kanji) and [Kana](https://en.wikipedia.org/wiki/Kana)
//!
//! # Examples
//! ```
//! use wana_kana::trim_okurigana::*;
//! use wana_kana::Options;
//! assert_eq!(trim_okurigana("踏み込む"), "踏み込");
//! assert_eq!(trim_okurigana("使い方"), "使い方");
//! assert_eq!(trim_okurigana("申し申し"), "申し申");
//! assert_eq!(trim_okurigana("お腹"), "お腹");
//! assert_eq!(trim_okurigana("お祝い"), "お祝");
//! ```

use crate::is_japanese::*;
use crate::is_kana::*;

use crate::tokenize::*;
use crate::utils::is_char_kana::*;
use crate::utils::is_char_kanji::*;

pub fn trim_okurigana(input: &str) -> &str {
    trim_okurigana_with_opt(input, false, None)
}

pub fn is_leading_without_initial_kana(input: &str, from_start: bool) -> bool {
    from_start && !is_char_kana(input.chars().next().unwrap())
}
pub fn is_trailing_without_final_kana(input: &str, from_start: bool) -> bool {
    !from_start && !is_char_kana(input.chars().last().unwrap())
}

pub fn is_invalid_matcher(input: &str, match_kanji: Option<&str>) -> bool {
    if let Some(match_kanji) = match_kanji {
        match_kanji.chars().all(is_char_kanji)
    } else {
        is_kana(input)
    }
}

pub fn trim_okurigana_with_opt<'a>(input: &'a str, from_start: bool, match_kanji: Option<&str>) -> &'a str {
    if input.is_empty()
        || !is_japanese(input)
        || is_leading_without_initial_kana(input, from_start)
        || is_trailing_without_final_kana(input, from_start)
        || is_invalid_matcher(input, match_kanji)
    {
        return input;
    }

    let tokens = if let Some(match_kanji) = match_kanji {
        tokenize(match_kanji)
    } else {
        tokenize(input)
    };

    if from_start {
        input.trim_start_matches(tokens.get(0).unwrap())
    } else {
        input.trim_end_matches(tokens.iter().last().unwrap())
    }
}

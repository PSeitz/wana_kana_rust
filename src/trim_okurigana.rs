//! Strips trailing [Okurigana](https://en.wikipedia.org/wiki/Okurigana) if `input` is a mix of [Kanji](https://en.wikipedia.org/wiki/Kanji) and [Kana](https://en.wikipedia.org/wiki/Kana)
//!
//! # Examples
//! ```
//! use wana_kana::trim_okurigana::*;
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

#[inline]
/// Strips trailing [Okurigana](https://en.wikipedia.org/wiki/Okurigana) if `input` is a mix of [Kanji](https://en.wikipedia.org/wiki/Kanji) and [Kana](https://en.wikipedia.org/wiki/Kana)
pub fn trim_okurigana(input: &str) -> &str {
    trim_okurigana_with_opt(input, false, None)
}

pub(crate) fn is_leading_without_initial_kana(input: &str, trim_from_start: bool) -> bool {
    trim_from_start && !is_char_kana(input.chars().next().unwrap())
}

#[inline]
pub(crate) fn is_trailing_without_final_kana(input: &str, trim_from_start: bool) -> bool {
    !trim_from_start && !is_char_kana(input.chars().last().unwrap())
}

pub(crate) fn is_invalid_matcher(input: &str, match_kanji: Option<&str>) -> bool {
    if let Some(match_kanji) = match_kanji {
        match_kanji.chars().all(is_char_kanji)
    } else {
        is_kana(input)
    }
}

/// Strips trailing [Okurigana](https://en.wikipedia.org/wiki/Okurigana) if `input` is a mix of [Kanji](https://en.wikipedia.org/wiki/Kanji) and [Kana](https://en.wikipedia.org/wiki/Kana)
///
/// Description.
///
/// * `trim_from_start` - trim the start of the text. If unset will trim the end of the text
/// * `match_kanji` - In case input is all Kana, the kanji needs to be provided to know where to
/// trim
///
/// # Examples
/// ```rust
/// use wana_kana::trim_okurigana::*;
/// assert_eq!(trim_okurigana_with_opt("おはら", false, Some("お腹")), "おはら");
/// assert_eq!(trim_okurigana_with_opt("おはら", true, Some("お腹")), "はら");
/// assert_eq!(trim_okurigana_with_opt("ふみこむ", false, Some("踏み込む")), "ふみこ");
/// assert_eq!(trim_okurigana_with_opt("踏み込む", false, None), "踏み込");
/// assert_eq!(trim_okurigana_with_opt("おみまい", true, Some("お祝い")), "みまい");
/// assert_eq!(trim_okurigana_with_opt("おみまい", false, Some("お祝い")), "おみま");
/// ```
pub fn trim_okurigana_with_opt<'a>(
    input: &'a str,
    trim_from_start: bool,
    match_kanji: Option<&str>,
) -> &'a str {
    if !is_japanese(input)
        // Check already trimmed start
        || is_leading_without_initial_kana(input, trim_from_start)
         // Check already trimmed end
        || is_trailing_without_final_kana(input, trim_from_start)
        || is_invalid_matcher(input, match_kanji)
    {
        return input;
    }

    let tokens = if let Some(match_kanji) = match_kanji {
        tokenize(match_kanji)
    } else {
        tokenize(input)
    };
    dbg!(&tokens);

    if trim_from_start {
        input.trim_start_matches(&tokens[0])
    } else {
        input.trim_end_matches(tokens.iter().last().unwrap())
    }
}

//! Splits input into array of strings separated by opinionated
//! [`TokenType`](crate::tokenize::TokenType).
//!
//! [`tokenize_detailed`](crate::tokenize::tokenize_detailed) returns an
//! array containing `{ TokenType, String }` instead of `String`
//!
//! # Example
//! ```
//! use wana_kana::tokenize::*;
//! let empty: Vec<String> = vec![];
//! assert_eq!(tokenize(""), empty);
//! assert_eq!(tokenize("ふふフフ"), vec!["ふふ", "フフ"]);
//! assert_eq!(tokenize("感じ"), vec!["感", "じ"]);
//! assert_eq!(tokenize("私は悲しい"), vec!["私", "は", "悲", "しい"] );
//! ```

use itertools::Itertools;

use crate::utils::is_char_english_punctuation::*;
use crate::utils::is_char_hiragana::*;
use crate::utils::is_char_japanese::is_char_japanese;
use crate::utils::is_char_japanese_number::*;
use crate::utils::is_char_japanese_punctuation::*;
use crate::utils::is_char_kanji::*;
use crate::utils::is_char_katakana::*;
use crate::utils::is_char_latin_number::*;
use crate::utils::is_char_romaji::is_char_romaji;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The tokenizer assigns each token a `TokenType`.
///
/// In compact tokenization mode only `En`, `Ja` and `Other` exist.
pub enum TokenType {
    /// Egnlish token.
    En,
    /// Japanese token
    Ja,
    /// English number
    EnNum,
    /// Japanese number
    JaNum,
    /// English punctuation
    EnPunc,
    /// Japanese punctuation
    JaPunc,
    /// Kanji
    Kanji,
    /// Hiragana
    Hiragana,
    /// Katakana
    Katakana,
    /// Space
    Space,
    /// Other
    Other,
}

fn get_type(input: char, compact: bool) -> TokenType {
    if compact {
        match input {
            ' ' => TokenType::En,   // En Space
            '　' => TokenType::Ja, // Ja Space
            input
                if is_char_japanese_number(input)
                    || is_char_latin_number(input)
                    || is_char_english_punctuation(input)
                    || is_char_japanese_punctuation(input) =>
            {
                TokenType::Other
            }
            input if is_char_japanese(input) => TokenType::Ja,
            input if is_char_romaji(input) => TokenType::En,
            _ => TokenType::Other,
        }
    } else {
        match input {
            ' ' => TokenType::Space,   // En Space
            '　' => TokenType::Space, // Ja Space
            input if is_char_japanese_number(input) => TokenType::JaNum,
            input if is_char_latin_number(input) => TokenType::EnNum,
            input if is_char_english_punctuation(input) => TokenType::EnPunc,
            input if is_char_japanese_punctuation(input) => TokenType::JaPunc,
            input if is_char_kanji(input) => TokenType::Kanji,
            input if is_char_hiragana(input) => TokenType::Hiragana,
            input if is_char_katakana(input) => TokenType::Katakana,
            input if is_char_japanese(input) => TokenType::Ja,
            input if is_char_romaji(input) => TokenType::En,
            _ => TokenType::Other,
        }
    }
}

/// Tokenizes the text. Splits input into array of strings separated by opinionated
/// [`TokenType`](crate::tokenize::TokenType).
///
/// # Example
/// ```
/// use wana_kana::tokenize::*;
/// let empty: Vec<String> = vec![];
/// assert_eq!(tokenize(""), empty);
/// assert_eq!(tokenize("ふふフフ"), vec!["ふふ", "フフ"]);
/// assert_eq!(tokenize("感じ"), vec!["感", "じ"]);
/// assert_eq!(tokenize("私は悲しい"), vec!["私", "は", "悲", "しい"] );
/// ```
pub fn tokenize(input: &str) -> Vec<String> {
    tokenize_with_opt(input, false)
}

/// Tokenizes the text. Splits input into array of strings separated by opinionated
/// [`TokenType`](crate::tokenize::TokenType).
///
/// If `compact` is set, many same-language tokens are combined (spaces + text, kanji + kana,
/// numeral + punctuation).
pub fn tokenize_with_opt(input: &str, compact: bool) -> Vec<String> {
    let mut result = vec![];
    for (_, group) in &input.chars().group_by(|elt| get_type(*elt, compact)) {
        result.push(group.collect());
    }
    result
}

/// Tokenizes the text and returns the token for each type.
///
/// # Example
/// ```
/// use wana_kana::tokenize::*;
/// assert_eq!(
///     tokenize_detailed(
///         "5romaji here...!?漢字ひらがなカタ　カナ４「ＳＨＩＯ」。！ لنذهب",
///         true
///     ),
///     vec![
///         (TokenType::Other, "5".to_string()),
///         (TokenType::En, "romaji here".to_string()),
///         (TokenType::Other, "...!?".to_string()),
///         (TokenType::Ja, "漢字ひらがなカタ　カナ".to_string()),
///         (TokenType::Other, "４「".to_string()),
///         (TokenType::Ja, "ＳＨＩＯ".to_string()),
///         (TokenType::Other, "」。！".to_string()),
///         (TokenType::En, " ".to_string()),
///         (TokenType::Other, "لنذهب".to_string()),
///     ]
/// );
/// ```
pub fn tokenize_detailed(input: &str, compact: bool) -> Vec<(TokenType, String)> {
    let mut result = vec![];
    for (token_type, group) in &input.chars().group_by(|elt| get_type(*elt, compact)) {
        result.push((token_type, group.collect()));
    }
    result
}

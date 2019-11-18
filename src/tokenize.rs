//! Splits input into array of strings separated by opinionated token types
//!
//! `'en', 'ja', 'englishNumeral', 'japaneseNumeral','englishPunctuation', 'japanesePunctuation','kanji', 'hiragana', 'katakana', 'space', 'other'`.
//!
//! If `compact` true then many same-language tokens are combined (spaces + text, kanji + kana, numeral + punctuation).
//! `tokenize_detailed` returns an array containing `{ type, value }` instead of `'value'`
//! # Example
//! ```
//! use wana_kana::tokenize::*;
//! use wana_kana::Options;
//! let empty: Vec<String> = vec![];
//! assert_eq!(tokenize(""), empty);
//! assert_eq!(tokenize("ふふフフ"), vec!["ふふ", "フフ"]);
//! assert_eq!(tokenize("感じ"), vec!["感", "じ"]);
//! assert_eq!(tokenize("私は悲しい"), vec!["私", "は", "悲", "しい"] );
//! ```

use crate::utils::is_char_english_punctuation::*;
use crate::utils::is_char_hiragana::*;
use crate::utils::is_char_japanese::is_char_japanese;
use crate::utils::is_char_japanese_number::*;
use crate::utils::is_char_japanese_punctuation::*;
use crate::utils::is_char_kanji::*;
use crate::utils::is_char_katakana::*;
use crate::utils::is_char_latin_number::*;
use crate::utils::is_char_romaji::is_char_romaji;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    En,
    Ja,
    EnNum,
    JaNum,
    EnPunc,
    JaPunc,
    Kanji,
    Hiragana,
    Katakana,
    Space,
    Other,
}

fn get_type(input: char, compact: bool) -> TokenType {
    if compact {
        match input {
            input if is_char_japanese_number(input) => TokenType::Other,
            input if is_char_latin_number(input) => TokenType::Other,
            ' ' => TokenType::En, //En Space
            input if is_char_english_punctuation(input) => TokenType::Other,
            '　' => TokenType::Ja, //Ja Space
            input if is_char_japanese_punctuation(input) => TokenType::Other,
            input if is_char_japanese(input) => TokenType::Ja,
            input if is_char_romaji(input) => TokenType::En,
            _ => TokenType::Other,
        }
    } else {
        match input {
            ' ' => TokenType::Space,   //En Space
            '　' => TokenType::Space, //Ja Space
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

pub fn tokenize(input: &str) -> Vec<String> {
    tokenize_with_opt(input, false)
}

pub fn tokenize_with_opt(input: &str, compact: bool) -> Vec<String> {
    let mut result = vec![];
    for (_, group) in &input.chars().group_by(|elt| get_type(*elt, compact)) {
        result.push(group.collect());
    }
    result
}

pub fn tokenize_detailed(input: &str, compact: bool) -> Vec<(TokenType, String)> {
    let mut result = vec![];
    for (token_type, group) in &input.chars().group_by(|elt| get_type(*elt, compact)) {
        result.push((token_type, group.collect()));
    }
    result
}

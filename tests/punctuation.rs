#![feature(test)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

mod conversion_tables;
use conversion_tables::*;

use wana_kana::utils::is_char_english_punctuation;
use wana_kana::utils::is_char_japanese_punctuation;
use wana_kana::utils::is_char_punctuation::is_char_punctuation;

#[test]
fn is_char_english_punctuation_test() {
    assert_eq!(EN_PUNC.iter().cloned().all(is_char_english_punctuation), true);
    assert_eq!(JA_PUNC.iter().cloned().all(is_char_english_punctuation), false);
    assert_eq!(is_char_english_punctuation(' '), true);
    assert_eq!(is_char_english_punctuation('a'), false);
    assert_eq!(is_char_english_punctuation('ふ'), false);
    assert_eq!(is_char_english_punctuation('字'), false);
}

#[test]
fn is_char_punctuation_test() {
    assert_eq!(EN_PUNC.iter().cloned().all(is_char_punctuation), true);
    assert_eq!(JA_PUNC.iter().cloned().all(is_char_punctuation), true);
    assert_eq!(is_char_punctuation(' '), true);
    assert_eq!(is_char_punctuation('　'), true);
    assert_eq!(is_char_punctuation('a'), false);
    assert_eq!(is_char_punctuation('ふ'), false);
    assert_eq!(is_char_punctuation('字'), false);
}

#[test]
fn is_char_japanese_punctuation_test() {
    assert_eq!(EN_PUNC.iter().cloned().all(is_char_japanese_punctuation), false);
    assert_eq!(JA_PUNC.iter().cloned().all(is_char_japanese_punctuation), true);
    assert_eq!(is_char_japanese_punctuation('　'), true);
    assert_eq!(is_char_japanese_punctuation('?'), false);
    assert_eq!(is_char_japanese_punctuation('a'), false);
    assert_eq!(is_char_japanese_punctuation('ふ'), false);
    assert_eq!(is_char_japanese_punctuation('字'), false);
}

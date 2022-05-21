//! Convert [Katakana](https://en.wikipedia.org/wiki/Katakana) to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
//!
//! Passes through any non-katakana chars
//!
//! # Examples
//!
//! ```
//! # use wana_kana::utils::katakana_to_hiragana;
//! katakana_to_hiragana("カタカナ");
//! // =>  "かたかな"
//!
//! katakana_to_hiragana("カタカナ is a type of kana");
//! // =>  "かたかな is a type of kana"
//! ```

use crate::constants::{HIRAGANA_START, KATAKANA_START};
use crate::to_romaji::TO_ROMAJI_NODE_TREE;
use crate::utils::is_char_katakana::*;
use crate::utils::is_char_long_dash::*;
use crate::utils::is_char_slash_dot::*;
use fnv::FnvHashMap;

pub fn is_char_initial_long_dash(char: char, index: usize) -> bool {
    is_char_long_dash(char) && index == 0
}
pub fn is_char_inner_long_dash(char: char, index: usize) -> bool {
    is_char_long_dash(char) && index != 0
}
pub fn is_kana_as_symbol(char: char) -> bool {
    'ヶ' == char || 'ヵ' == char
}

lazy_static! {
    pub static ref LONG_VOWELS: FnvHashMap<char, char> = hashmap! {
        'a' => 'あ',
        'i' => 'い',
        'u' => 'う',
        'e' => 'え',
        'o' => 'う',
    };
}

/// Convert a katakana `&str` to hiragana
///
/// ```
/// # use wana_kana::utils::katakana_to_hiragana;
/// assert_eq!(katakana_to_hiragana("カタカナ"), "かたかな");
/// ```
pub fn katakana_to_hiragana(input: &str) -> String {
    katakana_to_hiragana_with_opt(input, false)
}

/// Convert a katakana iterator of `char` to hiragana
///
/// ```
/// # use wana_kana::utils::katakana_iter_to_hiragana;
/// let v = vec!['カ', 'タ', 'カ', 'ナ'];
/// assert_eq!(katakana_iter_to_hiragana(v.iter().cloned()), "かたかな");
/// ```
pub fn katakana_iter_to_hiragana<C: Iterator<Item = char>>(input_chars: C) -> String {
    katakana_iter_to_hiragana_with_opt(input_chars, false)
}

pub(crate) fn katakana_to_hiragana_with_opt(input: &str, is_destination_romaji: bool) -> String {
    katakana_iter_to_hiragana_with_opt(input.chars(), is_destination_romaji)
}

pub(crate) fn katakana_iter_to_hiragana_with_opt<C: Iterator<Item = char>>(input_chars: C, is_destination_romaji: bool) -> String {
    let mut hira = Vec::with_capacity(input_chars.size_hint().0);
    let mut previous_kana: Option<char> = None;
    let mut previous_char: Option<char> = None;
    for (index, char) in input_chars.enumerate() {
        // Short circuit to avoid incorrect codeshift for 'ー' and '・'
        if is_char_slash_dot(char) || is_char_initial_long_dash(char, index) || is_kana_as_symbol(char) {
            hira.push(char);
        // Transform long vowels: 'オー' to 'おう'
        } else if let (Some(previous_kana), true) = (previous_kana, is_char_inner_long_dash(char, index)) {
            // Transform previous_kana back to romaji, and slice off the vowel
            let romaji = TO_ROMAJI_NODE_TREE.find_transition_node(previous_kana).unwrap().output;

            let romaji = romaji
                .chars()
                .last()
                .unwrap_or_else(|| panic!("could not find kana {:?} in TO_ROMAJI map", previous_kana));
            // However, ensure 'オー' => 'おお' => 'oo' if this is a transform on the way to romaji
            if let Some(prev_char) = previous_char {
                if is_char_katakana(prev_char) && romaji == 'o' && is_destination_romaji {
                    hira.push('お');
                    continue;
                }
            }

            if let Some(hit) = LONG_VOWELS.get(&romaji) {
                hira.push(*hit);
            }
        } else if !is_char_long_dash(char) && is_char_katakana(char) {
            // Shift charcode.
            let code = char as i32 + (HIRAGANA_START as i32 - KATAKANA_START as i32) as i32;
            let hira_char = std::char::from_u32(code as u32).unwrap();
            hira.push(hira_char);
            previous_kana = Some(hira_char);
        } else {
            // Pass non katakana chars through
            hira.push(char);
            previous_kana = None;
        }
        previous_char = Some(char);
    }
    hira.into_iter().collect()
}

#[test]
fn test_katakana_to_hiragana() {
    assert_eq!(katakana_to_hiragana("カタカナ"), "かたかな");
    assert_eq!(katakana_to_hiragana("カタカナ is a type of kana"), "かたかな is a type of kana");
    assert_eq!(katakana_to_hiragana("オーオー"), "おうおう");
}

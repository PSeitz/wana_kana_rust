use crate::utils::kana_to_romaji_map::TO_ROMAJI;
use fnv::FnvHashMap;
use crate::constants::{HIRAGANA_START, KATAKANA_START};
use std;
use crate::utils::is_char_katakana::*;
use crate::utils::is_char_long_dash::*;
use crate::utils::is_char_slash_dot::*;

/// Convert [Katakana](https://en.wikipedia.org/wiki/Katakana) to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
///
/// Passes through any non-katakana chars
///
/// @param  {String} [input=''] text input
///
/// # Examples
///
/// katakana_to_hiragana('カタカナ')
///
/// // => "かたかな"
///
/// katakana_to_hiragana('カタカナ is a type of kana')
///
/// // => "かたかな is a type of kana"
///


pub fn is_char_initial_long_dash(char: char, index: usize) -> bool {
    is_char_long_dash(char) && index < 1
}
pub fn is_char_inner_long_dash(char: char, index: usize) -> bool {
    is_char_long_dash(char) && index > 0
}
pub fn is_kana_as_symbol(char: char) -> bool {
    'ヶ' == char || 'ヵ' == char
}

lazy_static!{
    pub static ref LONG_VOWELS: FnvHashMap<char, char> = hashmap! {
        'a' => 'あ',
        'i' => 'い',
        'u' => 'う',
        'e' => 'え',
        'o' => 'う',
    };
}

pub fn katakana_to_hiragana(input: &str) -> String {
    katakana_to_hiragana_with_opt(input, false)
}
pub fn katakana_to_hiragana_with_opt(input: &str, is_destination_romaji: bool) -> String {
    let mut hira = vec![];
    let mut previous_kana: Option<char> = None;
    for (index, char) in input.chars().enumerate() {
        // Short circuit to avoid incorrect codeshift for 'ー' and '・'
        if is_char_slash_dot(char) || is_char_initial_long_dash(char, index) {
            hira.push(char);
        // Transform long vowels: 'オー' to 'おう'
        } else if let (Some(previous_kana), true) = (previous_kana, is_char_inner_long_dash(char, index)) {
            // Transform previous_kana back to romaji, and slice off the vowel
            let romaji = TO_ROMAJI[&previous_kana.to_string() as &str];

            // However, ensure 'オー' => 'おお' => 'oo' if this is a transform on the way to romaji
            if let Some(prev_char) = input.chars().nth(index-1) {
                if is_char_katakana(prev_char) && romaji == "o" && is_destination_romaji {
                    hira.push('お');
                    continue;
                }
            }

            if let Some(chacha) = romaji.chars().last() {
                if let Some(hit) = LONG_VOWELS.get(&chacha) {
                    hira.push(*hit);
                }
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
    }
    hira.into_iter().collect()
}

#[test]
fn test_katakana_to_hiragana() {
    assert_eq!(katakana_to_hiragana("カタカナ"), "かたかな");
    assert_eq!(
        katakana_to_hiragana("カタカナ is a type of kana"),
        "かたかな is a type of kana"
    );
}

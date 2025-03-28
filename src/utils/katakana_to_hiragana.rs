//! Convert [Katakana](https://en.wikipedia.org/wiki/Katakana) to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
//!
//! Passes through any non-katakana chars
//!
//! # Examples
//!
//! katakana_to_hiragana('カタカナ')
//!
//! // => "かたかな"
//!
//! katakana_to_hiragana('カタカナ is a type of kana')
//!
//! // => "かたかな is a type of kana"

use fnv::FnvHashMap;

use crate::constants::{HIRAGANA_START, KATAKANA_START};
use crate::halfwidth_to_hiragana_node_tree::HALFWIDTH_KATAKANA_TO_HIRAGANA_NODE_TREE;
use crate::to_romaji::TO_ROMAJI_NODE_TREE;
use crate::utils::is_char_halfwidth_katakana::is_char_halfwidth_katakana;
use crate::utils::is_char_katakana::*;
use crate::utils::is_char_long_dash::*;
use crate::utils::is_char_slash_dot::*;

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

pub fn katakana_to_hiragana(input: &str) -> String {
    katakana_to_hiragana_with_opt(input, false)
}

pub(crate) fn katakana_to_hiragana_with_opt(input: &str, is_destination_romaji: bool) -> String {
    let mut hira = Vec::with_capacity(input.chars().count());
    let mut previous_kana: Option<char> = None;
    let mut previous_read_forward_count: usize = 0;
    let chars = input.chars().collect::<Vec<_>>();

    for (index, input_char) in input.chars().enumerate() {
        // skip if already read
        if previous_read_forward_count > 0 {
            previous_read_forward_count -= 1;
            continue;
        }
        // Short circuit to avoid incorrect codeshift for 'ー' and '・'
        if is_char_slash_dot(input_char)
            || is_char_initial_long_dash(input_char, index)
            || is_kana_as_symbol(input_char)
        {
            hira.push(input_char);
        // Transform long vowels: 'オー' to 'おう'
        } else if let (Some(previous_kana), true) =
            (previous_kana, is_char_inner_long_dash(input_char, index))
        {
            // Transform previous_kana back to romaji, and slice off the vowel
            let Some(node) = TO_ROMAJI_NODE_TREE.find_transition_node(previous_kana) else {
                hira.push(input_char);
                continue;
            };

            let romaji_opt = node.output.chars().last();
            // However, ensure 'オー' => 'おお' => 'oo' if this is a transform on the way to romaji
            if let Some(prev_char) = input.chars().nth(index - 1) {
                if is_char_katakana(prev_char) && romaji_opt == Some('o') && is_destination_romaji {
                    hira.push('お');
                    continue;
                }
            }

            if let Some(hit) = romaji_opt.and_then(|romaji| LONG_VOWELS.get(&romaji)) {
                hira.push(*hit);
            }
        } else if !is_char_long_dash(input_char) && is_char_katakana(input_char) {
            let hira_char = match input_char {
                // rare special cases
                'ヷ' => 'わ', // wa with a voiced mark
                'ヸ' => 'ゐ', // wi with a voiced mark
                'ヹ' => 'ゑ', // we with a voiced mark
                'ヺ' => 'を', // wo with a voiced mark
                _ => {
                    // Shift charcode.
                    let code = input_char as i32 + (HIRAGANA_START as i32 - KATAKANA_START as i32);
                    // the fallback shouldn't normally happen
                    std::char::from_u32(code as u32).unwrap_or(input_char)
                }
            };

            hira.push(hira_char);
            previous_kana = Some(hira_char);
        } else if is_char_halfwidth_katakana(input_char) {
            let result = HALFWIDTH_KATAKANA_TO_HIRAGANA_NODE_TREE.get(&chars[index..]);
            result.0.chars().for_each(|char| hira.push(char));
            previous_read_forward_count += result.1 - 1;
        } else {
            // Pass non katakana chars through
            hira.push(input_char);
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

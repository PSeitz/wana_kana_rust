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
use crate::utils::is_char_slash_dot::*;
use crate::utils::is_prolonged_sound::*;

pub fn is_char_initial_long_dash(char: char, index: usize) -> bool {
    is_prolonged_sound(char) && index == 0
}
pub fn is_char_inner_long_dash(char: char, index: usize) -> bool {
    is_prolonged_sound(char) && index != 0
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
    katakana_to_hiragana_with_opt(input, false, false)
}

pub(crate) fn katakana_to_hiragana_with_opt(
    input: &str,
    is_destination_romaji: bool,
    keep_prolonged_sound_mark: bool,
) -> String {
    let chars = input.chars().collect::<Vec<_>>();
    let mut hira = Vec::with_capacity(chars.len());
    let mut previous_kana: Option<char> = None;
    let mut count: usize = 0;

    while count < chars.len() {
        let input_char = chars[count];
        // Short circuit to avoid incorrect codeshift for 'ー' and '・'
        if is_char_slash_dot(input_char)
            || is_char_initial_long_dash(input_char, count)
            || is_kana_as_symbol(input_char)
        {
            if keep_prolonged_sound_mark && is_char_initial_long_dash(input_char, count) {
                // Normalize halfwidth 'ｰ' to fullwidth 'ー' so fullwidth/halfwidth inputs agree.
                hira.push('ー');
            } else {
                hira.push(input_char);
            }
        // Transform long vowels: 'オー' to 'おう'
        } else if let (Some(previous_kana), true) =
            (previous_kana, is_char_inner_long_dash(input_char, count))
        {
            // Opt-out: keep the prolonged sound mark as-is. Normalize halfwidth
            // 'ｰ' to fullwidth 'ー' so fullwidth/halfwidth inputs agree.
            if keep_prolonged_sound_mark {
                hira.push('ー');
                count += 1;
                continue;
            }

            // Transform previous_kana back to romaji, and slice off the vowel
            let Some(node) = TO_ROMAJI_NODE_TREE.find_transition_node(previous_kana) else {
                hira.push(input_char);
                count += 1;
                continue;
            };

            let romaji_opt = node.output.chars().last();
            // However, ensure 'オー' => 'おお' => 'oo' if this is a transform on the way to romaji
            if let Some(prev_char) = chars.get(count - 1) {
                if is_char_katakana(*prev_char) && romaji_opt == Some('o') && is_destination_romaji
                {
                    hira.push('お');
                    count += 1;
                    continue;
                }
            }

            if let Some(hit) = romaji_opt.and_then(|romaji| LONG_VOWELS.get(&romaji)) {
                hira.push(*hit);
            }
        } else if !is_prolonged_sound(input_char) && is_char_katakana(input_char) {
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
            let result = HALFWIDTH_KATAKANA_TO_HIRAGANA_NODE_TREE.get(&chars[count..]);
            hira.extend(result.0.chars());
            // Track the last produced kana so a following 'ー' can trigger
            // the long-vowel transformation (e.g. 'ｽｰ' => 'すう').
            previous_kana = result.0.chars().last();
            count += result.1 - 1;
        } else {
            // Pass non katakana chars through
            hira.push(input_char);
            previous_kana = None;
        }
        count += 1;
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

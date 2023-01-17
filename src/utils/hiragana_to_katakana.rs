use crate::constants::{HIRAGANA_START, KATAKANA_START};
use crate::utils::is_char_hiragana::*;
use crate::utils::is_char_long_dash::*;
use crate::utils::is_char_slash_dot::*;

/// Convert [Hiragana](https://en.wikipedia.org/wiki/Hiragana) to [Katakana](https://en.wikipedia.org/wiki/Katakana)
///
/// Passes through any non-hiragana chars
///
/// # Examples
///
///
/// hiragana_to_katakana('ひらがな')
///
/// // => "ヒラガナ"
///
/// hiragana_to_katakana('ひらがな is a type of kana')
///
/// // => "ヒラガナ is a type of kana"

pub fn hiragana_to_katakana(input: &str) -> String {
    let mut kata = vec![];
    for char in input.chars() {
        // Short circuit to avoid incorrect codeshift for 'ー' and '・'
        if is_char_long_dash(char) || is_char_slash_dot(char) {
            kata.push(char);
        } else if is_char_hiragana(char) {
            // Shift charcode.
            let code = char as i32 + (KATAKANA_START as i32 - HIRAGANA_START as i32);
            // let kata_char = String.from_char_code(code);
            let kata_char = std::char::from_u32(code as u32).unwrap();
            kata.push(kata_char);
        } else {
            // Pass non-hiragana chars through
            kata.push(char);
        }
    }
    kata.into_iter().collect()
}

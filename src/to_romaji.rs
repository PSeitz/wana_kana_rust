use constants::TO_ROMAJI;

use utils::get_chunk::*;
use utils::katakana_to_hiragana::*;
use is_katakana::*;
use options::Options;
use std;
use std::borrow::Cow;
/**
 * Convert kana to romaji
 * @param  {String} kana text input
 * @param  {DefaultOptions} [options=default_options]
 * @return {String} converted text
 * @example
 * to_romaji('ひらがな　カタカナ')
 * // => 'hiragana katakana'
 * to_romaji('ひらがな　カタカナ', { upcase_katakana: true })
 * // => 'hiragana KATAKANA'
 */
pub fn to_romaji(kana: &str) -> String {
    to_romaji_with_opt(kana, Options::default())
}
pub fn to_romaji_with_opt(kana: &str, options: Options) -> String {
    let config = options;
    let len = kana.chars().count();
    // Final output array
    let mut roma = String::with_capacity(kana.len());
    // Position in the string that is being evaluated
    let mut cursor = 0;
    let max_chunk = 2;

    let mut next_char_is_double_consonant = false;

    while cursor < len {
        let mut roma_char = None;
        let mut chunk = Cow::from("");
        let mut chunk_size = std::cmp::min(max_chunk, len - cursor);
        let mut convert_this_chunk_to_uppercase = false;
        while chunk_size > 0 {
            chunk = Cow::from(get_chunk(kana, cursor, cursor + chunk_size));
            if is_katakana(&chunk) {
                convert_this_chunk_to_uppercase = config.upcase_katakana;
                chunk = Cow::from(katakana_to_hiragana(&chunk));
            }
            // special case for small tsus
            if chunk.chars().nth(0).map(|c| c == 'っ').unwrap_or(false) && chunk_size == 1 && cursor < (len - 1) {
                next_char_is_double_consonant = true;
                roma_char = Some(Cow::from(""));
                break;
            }

            roma_char = TO_ROMAJI.get(&chunk as &str).map(|el| Cow::from(*el));

            if let &mut Some(ref mut roma_charo) = &mut roma_char {
                if next_char_is_double_consonant {
                    *roma_charo = Cow::from(roma_charo.chars().nth(0).unwrap().to_string() + &roma_charo);
                    next_char_is_double_consonant = false;
                }
                break;
            }

            chunk_size -= 1;
        }

        let mut roma_charo = roma_char.clone().unwrap_or(chunk); // Passthrough undefined values
        if convert_this_chunk_to_uppercase {
            roma_charo = Cow::from(roma_charo.to_uppercase());
        }
        roma.push_str(&roma_charo);
        cursor += std::cmp::max(chunk_size, 1);
    }
    roma
}

#[test]
fn check_to_to_romaji() {
    assert_eq!(
        to_romaji("ひらがな　カタカナ"),
        "hiragana katakana"
    );
    assert_eq!(
        to_romaji_with_opt(
            "ひらがな　カタカナ",
            Options {
                upcase_katakana: true,
                ..Default::default()
            }
        ),
        "hiragana KATAKANA"
    );
}

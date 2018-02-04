use constants::TO_ROMAJI;

use utils::get_chunk::*;
use utils::katakana_to_hiragana::*;
use is_katakana::*;
use options::Options;
use std;
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
pub fn to_romaji(kana: &str, options: Options) -> String {
    let config = options;
    let len = kana.chars().count();
    // Final output array
    let mut roma = String::new();
    // Position in the string that is being evaluated
    let mut cursor = 0;
    let max_chunk = 2;
    // let mut chunk_size = 2;
    let mut chunk = "".to_string();
    let mut roma_char = "".to_string();
    let mut next_char_is_double_consonant = false;

    while cursor < len {
        let mut chunk_size = std::cmp::min(max_chunk, len - cursor);
        let mut convert_this_chunk_to_uppercase = false;
        while chunk_size > 0 {
            chunk = get_chunk(kana, cursor, cursor + chunk_size);
            if is_katakana(&chunk) {
                convert_this_chunk_to_uppercase = config.upcase_katakana;
                chunk = katakana_to_hiragana(&chunk);
            }
            // special case for small tsus
            if chunk.chars().nth(0).unwrap() == 'っ' && chunk_size == 1 && cursor < (len - 1) {
                next_char_is_double_consonant = true;
                roma_char = "".to_string();
                break;
            }

            // roma_char = TO_ROMAJI[&chunk as &str].to_string();
            if let Some(char) = TO_ROMAJI.get(&chunk as &str) {
                roma_char = char.to_string();
                if next_char_is_double_consonant {
                    roma_char = roma_char.chars().nth(0).unwrap().to_string() + &roma_char;
                    next_char_is_double_consonant = false;
                }
                break;
            }

            chunk_size -= 1;
        }
        if roma_char == "" {
            // Passthrough undefined values
            roma_char = chunk.to_string();
        }

        if convert_this_chunk_to_uppercase {
            roma_char = roma_char.to_uppercase();
        }
        roma.push_str(&roma_char);
        cursor += std::cmp::max(chunk_size, 1);
    }
    roma
}

#[test]
fn check_to_to_romaji() {
    assert_eq!(
        to_romaji("ひらがな　カタカナ", Options::default()),
        "hiragana katakana"
    );
    assert_eq!(
        to_romaji(
            "ひらがな　カタカナ",
            Options {
                upcase_katakana: true,
                ..Default::default()
            }
        ),
        "hiragana KATAKANA"
    );
}

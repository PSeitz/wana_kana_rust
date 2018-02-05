//! Convert [Romaji](https://en.wikipedia.org/wiki/Romaji) to [Kana](https://en.wikipedia.org/wiki/Kana), lowercase text will result in [Hiragana](https://en.wikipedia.org/wiki/Hiragana) and uppercase text will result in [Katakana](https://en.wikipedia.org/wiki/Katakana).
//!
//! # Examples
//! ```
//! use wana_kana::to_kana::*;
//! use wana_kana::Options;
//! assert_eq!(to_kana_with_opt("o", Options::default()), "お");
//! assert_eq!(to_kana_with_opt("ona", Options::default()), "おな");
//! assert_eq!(to_kana_with_opt("onaji", Options::default()), "おなじ");
//! assert_eq!(to_kana_with_opt("onaji BUTTSUUJI", Options::default()), "おなじ ブッツウジ");
//! assert_eq!(to_kana_with_opt("ONAJI buttsuuji", Options::default()), "オナジ ぶっつうじ");
//! assert_eq!(to_kana_with_opt("座禅‘zazen’スタイル", Options::default()), "座禅「ざぜん」スタイル");
//! assert_eq!(to_kana_with_opt("batsuge-mu", Options {use_obsolete_kana: true, ..Default::default() } ), "ばつげーむ");
//! assert_eq!(to_kana_with_opt("!?./,~-‘’“”[](){}", Options::default()), "！？。・、〜ー「」『』［］（）｛｝");
//! assert_eq!(to_kana_with_opt("we", Options {use_obsolete_kana: true, ..Default::default() } ), "ゑ");
//! ```

use constants::{FOUR_CHAR_EDGECASES, FROM_ROMAJI, UPPERCASE_END, UPPERCASE_START};

use utils::is_char_in_range::*;
use utils::is_char_upper_case::*;
use utils::get_chunk::get_chunk;
use utils::is_char_consonant::*;
use utils::is_char_vowel::*;
use utils::hiragana_to_katakana::*;
use is_kana::*;
use options::Options;
use std;
use std::borrow::Cow;



pub fn to_kana(input: &str) -> String {
    to_kana_with_opt(input, Options::default())
}

fn lower_cow<'a>(text: &Cow<'a, str>) -> Cow<'a, str> {
    if text.chars().all(char::is_lowercase) {
        text.clone()
    } else {
        Cow::from(text.to_lowercase())
    }
}

pub fn to_kana_with_opt(input: &str, options: Options) -> String {
    let config = options;
    // Final output array containing arrays [start index of the translitterated substring, end index, kana]
    let mut kana = String::with_capacity(input.len());
    // Position in the string that is being evaluated
    let mut cursor = 0;
    let len = input.chars().count();
    let max_chunk = 3;
    // let mut chunk_size = 3;
    // Steps through the string pulling out chunks of characters. Each chunk will be evaluated
    // against the romaji to kana table. If there is no match, the last character in the chunk
    // is dropped and the chunk is reevaluated. If nothing matches, the character is assumed
    // to be invalid or punctuation or other and gets passed through.
    while cursor < len {
        let mut chunk = Cow::from("");
        let mut chunk_lc = Cow::from("");
        let mut kana_char = Cow::from("");
        let mut chunk_size = std::cmp::min(max_chunk, len - cursor);
        while chunk_size > 0 {
            chunk = Cow::from(get_chunk(input, cursor, cursor + chunk_size));
            chunk_lc = lower_cow(&chunk);
            // Handle super-rare edge cases with 4 char chunks (like ltsu, chya, shya)
            if FOUR_CHAR_EDGECASES.contains(&(&chunk_lc as &str)) && len - cursor >= 4 {
                chunk_size += 1;
                chunk = Cow::from(get_chunk(input, cursor, cursor + chunk_size));
                chunk_lc = lower_cow(&chunk);
            } else if let (Some(lc), Some(c)) = (chunk_lc.chars().nth(0), chunk.chars().nth(0)) {
                // Handle edge case of n followed by consonant
                if lc == 'n' {
                    if chunk_size == 2 {
                        // Handle edge case of n followed by a space (only if not in IME mode)
                        if !config.imemode && chunk_lc.chars().nth(1).map(|c| c == ' ').unwrap_or(false) {
                            kana_char = Cow::from("ん ");
                            break;
                        }
                        // Convert IME input of n' to "ん"
                        if config.imemode && chunk_lc == "n'" {
                            kana_char = Cow::from("ん");
                            break;
                        }
                    }
                    // Handle edge case of n followed by n and vowel
                    if chunk_lc
                        .chars()
                        .nth(1)
                        .map(|c| is_char_consonant(c, false))
                        .unwrap_or(false) && chunk_lc.chars().nth(2).map(is_char_vowel).unwrap_or(false)
                    {
                        chunk_size = 1;
                        chunk = Cow::from(get_chunk(input, cursor, cursor + chunk_size));
                        chunk_lc = lower_cow(&chunk);
                    }
                }

                // Handle case of double consonants
                if lc != 'n' && is_char_consonant(lc, true) && Some(c) == chunk.chars().nth(1) {
                    chunk_size = 1;
                    // Return katakana ッ if chunk is uppercase, otherwise return hiragana っ
                    if is_char_in_range(c, UPPERCASE_START, UPPERCASE_END) {
                        chunk_lc = Cow::from("ッ");
                        chunk = Cow::from("ッ");
                    } else {
                        chunk_lc = Cow::from("っ");
                        chunk = Cow::from("っ");
                    }
                }
            }

            if let Some(char) = FROM_ROMAJI.get(&chunk_lc as &str) {
                kana_char = Cow::from(*char);
                break;
            } else {
                kana_char = Cow::from("");
            }
            // Step down the chunk size.
            // If chunk_size was 4, step down twice.
            if chunk_size == 4 {
                chunk_size -= 2;
            } else {
                chunk_size -= 1;
            }
        }

        // Passthrough undefined values
        if kana_char == "" {
            kana_char = chunk.clone();
        }

        // Handle special cases.
        if config.use_obsolete_kana {
            if chunk_lc == "wi" {
                kana_char = Cow::from("ゐ")
            };
            if chunk_lc == "we" {
                kana_char = Cow::from("ゑ")
            };
        }

        if config.imemode && chunk_lc.chars().nth(0).map(|c| c == 'n').unwrap_or(false) {
            if input
                .chars()
                .nth(cursor + 1)
                .map(|c| c.to_string().to_lowercase() == "y")
                .unwrap_or(false)
                && input
                    .chars()
                    .nth(cursor + 2)
                    .map(|c| !is_char_vowel(c))
                    .unwrap_or(true) || cursor == len - 1
                || input
                    .chars()
                    .nth(cursor + 1)
                    .map(|c| is_kana(&c.to_string()))
                    .unwrap_or(false)
            {
                // Don't transliterate this yet.
                kana_char = Cow::from(chunk.chars().nth(0).unwrap().to_string());
            }
        }

        // Use katakana if first letter in chunk is uppercase
        if chunk
            .chars()
            .nth(0)
            .map(|c| is_char_upper_case(c))
            .unwrap_or(false)
        {
            kana_char = Cow::from(hiragana_to_katakana(&kana_char));
        }

        cursor += std::cmp::max(chunk_size, 1);

        kana.push_str(&kana_char);
    }
    return kana;
}


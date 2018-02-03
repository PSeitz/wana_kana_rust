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

/**
 * Convert [Romaji](https://en.wikipedia.org/wiki/Romaji) to [Kana](https://en.wikipedia.org/wiki/Kana), lowercase text will result in [Hiragana](https://en.wikipedia.org/wiki/Hiragana) and uppercase text will result in [Katakana](https://en.wikipedia.org/wiki/Katakana).
 * @param  {String} [input=''] text
 * @param  {DefaultOptions} [options=default_options]
 * @return {String} converted text
 * @example
 * to_kana('onaji BUTTSUUJI')
 * // => 'おなじ ブッツウジ'
 * to_kana('ONAJI buttsuuji')
 * // => 'オナジ ぶっつうじ'
 * to_kana('座禅‘zazen’スタイル')
 * // => '座禅「ざぜん」スタイル'
 * to_kana('batsuge-mu')
 * // => 'ばつげーむ'
 * to_kana('!?.:/,~-‘’“”[](){}') // Punctuation conversion
 * // => '！？。：・、〜ー「」『』［］（）｛｝'
 * to_kana('we', { use_obsolete_kana: true })
 * // => 'ゑ'
 */
pub fn to_kana(input: &str, options: Options) -> String {
    // just throw away the substring index information and just concatenate all the kana
    // return split_into_kana(input, options)
    // .iter()
    //   .map(|kana_token| kana_token[2])
    //   .into_iter().collect()
    split_into_kana(input, options)
}

pub fn split_into_kana(input: &str, options: Options) -> String {
    let config = options;
    // Final output array containing arrays [start index of the translitterated substring, end index, kana]
    let mut kana = String::new();
    // Position in the string that is being evaluated
    let mut cursor = 0;
    let len = input.len();
    let max_chunk = 3;
    let mut chunk_size = 3;
    let mut chunk = "";
    let mut chunkLC = "".to_string();

    // Steps through the string pulling out chunks of characters. Each chunk will be evaluated
    // against the romaji to kana table. If there is no match, the last character in the chunk
    // is dropped and the chunk is reevaluated. If nothing matches, the character is assumed
    // to be invalid or punctuation or other and gets passed through.
    while (cursor < len) {
        let mut kana_char = Cow::from("".to_string());
        chunk_size = std::cmp::min(max_chunk, len - cursor);
        while (chunk_size > 0) {
            chunk = get_chunk(input, cursor, cursor + chunk_size);
            chunkLC = chunk.to_lowercase();
            // Handle super-rare edge cases with 4 char chunks (like ltsu, chya, shya)
            if FOUR_CHAR_EDGECASES.contains(&(&chunkLC as &str)) && len - cursor >= 4 {
                chunk_size += 1;
                chunk = get_chunk(input, cursor, cursor + chunk_size);
                chunkLC = chunk.to_lowercase();
            } else {
                // Handle edge case of n followed by consonant
                if (chunkLC.chars().nth(0).unwrap() == 'n') {
                    if (chunk_size == 2) {
                        // Handle edge case of n followed by a space (only if not in IME mode)
                        if (!config.IMEMode && chunkLC.chars().nth(1).unwrap() == ' ') {
                            kana_char = Cow::from("ん ");
                            break;
                        }
                        // Convert IME input of n' to "ん"
                        if (config.IMEMode && chunkLC == "n'") {
                            kana_char = Cow::from("ん");
                            break;
                        }
                    }
                    // Handle edge case of n followed by n and vowel
                    if (is_char_consonant(chunkLC.chars().nth(1).unwrap(), false)
                        && is_char_vowel(chunkLC.chars().nth(2).unwrap()))
                    {
                        chunk_size = 1;
                        chunk = get_chunk(input, cursor, cursor + chunk_size);
                        chunkLC = chunk.to_lowercase();
                    }
                }

                // Handle case of double consonants
                if (chunkLC.chars().nth(0).unwrap() != 'n'
                    && is_char_consonant(chunkLC.chars().nth(0).unwrap(), true)
                    && chunk.chars().nth(0).unwrap() == chunk.chars().nth(1).unwrap())
                {
                    chunk_size = 1;
                    // Return katakana ッ if chunk is uppercase, otherwise return hiragana っ
                    if (is_char_in_range(
                        chunk.chars().nth(0).unwrap(),
                        UPPERCASE_START,
                        UPPERCASE_END,
                    )) {
                        chunkLC = "ッ".to_string();
                        chunk = "ッ";
                    } else {
                        chunkLC = "っ".to_string();
                        chunk = "っ";
                    }
                }
            }

            kana_char = Cow::from(FROM_ROMAJI[&chunkLC as &str]);
            // console.log(`${chunkLC}, ${cursor}x${chunk_size}:${chunk} => ${kana_char}`); // DEBUG
            if (kana_char != "") {
                break;
            }
            // Step down the chunk size.
            // If chunk_size was 4, step down twice.
            if (chunk_size == 4) {
                chunk_size -= 2;
            } else {
                chunk_size -= 1;
            }
        }

        // Passthrough undefined values
        if (kana_char == "") {
            kana_char = Cow::from(chunk);
        }

        // Handle special cases.
        if (config.use_obsolete_kana) {
            if (chunkLC == "wi") {
                kana_char = Cow::from("ゐ")
            };
            if (chunkLC == "we") {
                kana_char = Cow::from("ゑ")
            };
        }

        if (!!config.IMEMode && chunkLC.chars().nth(0).unwrap() == 'n') {
            if ((input
                .chars()
                .nth(cursor + 1)
                .unwrap()
                .to_string()
                .to_lowercase() == "y"
                && is_char_vowel(input.chars().nth(cursor + 2).unwrap()) == false)
                || cursor == len - 1
                || is_kana(&input.chars().nth(cursor + 1).unwrap().to_string()))
            {
                // Don't transliterate this yet.
                kana_char = Cow::from(chunk.chars().nth(0).unwrap().to_string());
            }
        }

        // Use katakana if first letter in chunk is uppercase
        if (is_char_upper_case(chunk.chars().nth(0).unwrap())) {
            kana_char = Cow::from(hiragana_to_katakana(&kana_char));
        }

        let next_cursor = cursor + (std::cmp::max(chunk_size, 1));
        // kana.push([cursor, next_cursor, kana_char]);
        kana.push_str(&kana_char);
        cursor = next_cursor;
    }
    return kana;
}


#[test]
fn check_to_kana() {
    assert_eq!(to_kana("onaji BUTTSUUJI"), "おなじ ブッツウジ");
    assert_eq!(to_kana("ONAJI buttsuuji"), "オナジ ぶっつうじ");
    assert_eq!(to_kana("座禅‘zazen’スタイル"), "座禅「ざぜん」スタイル");
    assert_eq!(to_kana("batsuge-mu"),      "ばつげーむ");
    // assert_eq!(to_kana("🐸"), false);
    // assert_eq!(to_kana("🐸"), false);
}

import {
  DEFAULT_OPTIONS,
  UPPERCASE_START,
  UPPERCASE_END,
  FOUR_CHAR_EDGECASES,
  FROM_ROMAJI,
} from './constants';

use utils::is_char_inRange::*;
use utils::is_char_upperCase::*;
use utils::get_chunk_size::*;
use utils::get_chunk::*;
use utils::is_char_consonant::*;
use utils::is_char_vowel::*;
use utils::hiragana_to_katakana::*;
import is_kana from './is_kana';

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
export fn to_kana(input: &str, options = {}) {
  // just throw away the substring index information and just concatenate all the kana
  return split_into_kana(input, options)
    .map((kana_token) => kana_token[2])
    .join('');
}

export fn split_into_kana(input: &str, options = {}) {
  const config = Object.assign({}, DEFAULT_OPTIONS, options);
  // Final output array containing arrays [start index of the translitterated substring, end index, kana]
  const kana = [];
  // Position in the string that is being evaluated
  let cursor = 0;
  const len = input.length;
  const max_chunk = 3;
  let chunk_size = 3;
  let chunk = '';
  let chunkLC = '';

  // Steps through the string pulling out chunks of characters. Each chunk will be evaluated
  // against the romaji to kana table. If there is no match, the last character in the chunk
  // is dropped and the chunk is reevaluated. If nothing matches, the character is assumed
  // to be invalid or punctuation or other and gets passed through.
  while (cursor < len) {
    let kana_char = null;
    chunk_size = get_chunk_size(max_chunk, len - cursor);
    while (chunk_size > 0) {
      chunk = get_chunk(input, cursor, cursor + chunk_size);
      chunkLC = chunk.to_lower_case();
      // Handle super-rare edge cases with 4 char chunks (like ltsu, chya, shya)
      if (FOUR_CHAR_EDGECASES.includes(chunkLC) && len - cursor >= 4) {
        chunk_size += 1;
        chunk = get_chunk(input, cursor, cursor + chunk_size);
        chunkLC = chunk.to_lower_case();
      } else {
        // Handle edge case of n followed by consonant
        if (chunkLC.char_at(0) === 'n') {
          if (chunk_size === 2) {
            // Handle edge case of n followed by a space (only if not in IME mode)
            if (!config.IMEMode && chunkLC.char_at(1) === ' ') {
              kana_char = 'ん ';
              break;
            }
            // Convert IME input of n' to "ん"
            if (config.IMEMode && chunkLC === "n'") {
              kana_char = 'ん';
              break;
            }
          }
          // Handle edge case of n followed by n and vowel
          if (
            is_char_consonant(chunkLC.char_at(1), false) &&
            is_char_vowel(chunkLC.char_at(2))
          ) {
            chunk_size = 1;
            chunk = get_chunk(input, cursor, cursor + chunk_size);
            chunkLC = chunk.to_lower_case();
          }
        }

        // Handle case of double consonants
        if (
          chunkLC.char_at(0) !== 'n' &&
          is_char_consonant(chunkLC.char_at(0)) &&
          chunk.char_at(0) === chunk.char_at(1)
        ) {
          chunk_size = 1;
          // Return katakana ッ if chunk is uppercase, otherwise return hiragana っ
          if (is_char_inRange(chunk.char_at(0), UPPERCASE_START, UPPERCASE_END)) {
            chunkLC = 'ッ';
            chunk = 'ッ';
          } else {
            chunkLC = 'っ';
            chunk = 'っ';
          }
        }
      }

      kana_char = FROM_ROMAJI[chunkLC];
      // console.log(`${chunkLC}, ${cursor}x${chunk_size}:${chunk} => ${kana_char}`); // DEBUG
      if (kana_char != null) {
        break;
      }
      // Step down the chunk size.
      // If chunk_size was 4, step down twice.
      if (chunk_size === 4) {
        chunk_size -= 2;
      } else {
        chunk_size -= 1;
      }
    }

    // Passthrough undefined values
    if (kana_char == null) {
      kana_char = chunk;
    }

    // Handle special cases.
    if (config.use_obsolete_kana) {
      if (chunkLC === 'wi') kana_char = 'ゐ';
      if (chunkLC === 'we') kana_char = 'ゑ';
    }

    if (!!config.IMEMode && chunkLC.char_at(0) === 'n') {
      if (
        (input.char_at(cursor + 1).to_lower_case() === 'y' &&
          is_char_vowel(input.char_at(cursor + 2)) === false) ||
        cursor === len - 1 ||
        is_kana(input.char_at(cursor + 1))
      ) {
        // Don't transliterate this yet.
        kana_char = chunk.char_at(0);
      }
    }

    // Use katakana if first letter in chunk is uppercase
    if (is_char_upperCase(chunk.char_at(0))) {
      kana_char = hiragana_to_katakana(kana_char);
    }

    const next_cursor = cursor + (chunk_size || 1);
    kana.push([cursor, next_cursor, kana_char]);
    cursor = next_cursor;
  }
  return kana;
}

export default to_kana;

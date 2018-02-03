use constants::DEFAULT_OPTIONS;
use utils::katakana_to_hiragana::*;
use utils::romaji_to_hiragana::*;
use is_romaji::*;
use is_mixed::*;

/**
 * Convert input to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
 * @param  {String} [input=''] text
 * @param  {DefaultOptions} [options=default_options]
 * @return {String} converted text
 * @example
 * to_hiragana('toukyou, オオサカ')
 * // => 'とうきょう、　おおさか'
 * to_hiragana('only カナ', { pass_romaji: true })
 * // => 'only かな'
 * to_hiragana('wi')
 * // => 'うぃ'
 * to_hiragana('wi', { use_obsolete_kana: true })
 * // => 'ゐ'
*/
fn to_hiragana(input: &str, options = {}) {
  const config = Object.assign({}, DEFAULT_OPTIONS, options);
  if (config.pass_romaji) return katakana_to_hiragana(input);
  if (is_romaji(input)) return romaji_to_hiragana(input, config);
  if (is_mixed(input, { pass_kanji: true })) {
    const romaji = katakana_to_hiragana(input);
    return romaji_to_hiragana(romaji, config);
  }
  return katakana_to_hiragana(input);
}



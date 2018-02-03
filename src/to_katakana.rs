use is_romaji::*;
use is_mixed::*;
use utils::hiragana_to_katakana::*;
use utils::romaji_to_hiragana::*;
use options::Options;

/**
 * Convert input to [Katakana](https://en.wikipedia.org/wiki/Katakana)
 * @param  {String} [input=''] text
 * @param  {DefaultOptions} [options=default_options]
 * @return {String} converted text
 * @example
 * to_katakana('toukyou, おおさか')
 * // => 'トウキョウ、　オオサカ'
 * to_katakana('only かな', { pass_romaji: true })
 * // => 'only カナ'
 * to_katakana('wi')
 * // => 'ウィ'
 * to_katakana('wi', { use_obsolete_kana: true })
 * // => 'ヰ'
*/
fn to_katakana(input: &str, options: Options) {
  let config = options;
  if (config.pass_romaji) return hiragana_to_katakana(input);
  if (is_romaji(input) || is_mixed(input)) {
    let romaji = romaji_to_hiragana(input, config);
    return hiragana_to_katakana(romaji);
  }
  return hiragana_to_katakana(input);
}



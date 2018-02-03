import { DEFAULT_OPTIONS } from './constants';
import is_romaji from './is_romaji';
import is_mixed from './is_mixed';
use utils::hiragana_to_katakana::*;
use utils::romaji_to_hiragana::*;

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
fn to_katakana(input: &str, options = {}) {
  const config = Object.assign({}, DEFAULT_OPTIONS, options);
  if (config.pass_romaji) return hiragana_to_katakana(input);
  if (is_romaji(input) || is_mixed(input)) {
    const romaji = romaji_to_hiragana(input, config);
    return hiragana_to_katakana(romaji);
  }
  return hiragana_to_katakana(input);
}

export default to_katakana;

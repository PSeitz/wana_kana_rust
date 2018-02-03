import to_kana from '../to_kana';

/**
 * Convert [Romaji](https://en.wikipedia.org/wiki/Romaji) to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
 * @param  {String} [input=''] text
 * @param  {Object} options used internally to pass along default options
 * @return {String} converted text
 * @example
 * romaji_to_hiragana('hiragana')
 * // => "ひらがな"
 * @ignore
 */
fn romaji_to_hiragana(input: &str, options = {}) {
  const text = input.to_lower_case(); // ensure hiragana
  return to_kana(text, options);
}

export default romaji_to_hiragana;

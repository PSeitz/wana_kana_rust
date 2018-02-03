use to_kana::to_kana;
use options::Options;
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
fn romaji_to_hiragana(input: &str, options: Options) -> String {
    let text = input.to_lowercase(); // ensure hiragana
    to_kana(&text, options)
}


use utils::is_char_kana::*;
use utils::is_char_punctuation::*;
import is_japanese from './is_japanese';
import is_kana from './is_kana';
import is_kanji from './is_kanji';

/**
 * Strips trailing [Okurigana](https://en.wikipedia.org/wiki/Okurigana) if `input` is a mix of [Kanji](https://en.wikipedia.org/wiki/Kanji) and [Kana](https://en.wikipedia.org/wiki/Kana)
 * @param  {String} input text
 * @param  {Object} [options={ all: false }] config object specifying if *all* kana should be removed, not just trailing okurigana
 * @return {String} text with okurigana removed
 * @example
 * strip_okurigana('踏み込む')
 * // => '踏み込'
 * strip_okurigana('粘り。')
 * // => '粘。'
 * strip_okurigana('お祝い')
 * // => 'お祝'
 * strip_okurigana('踏み込む', { all: true })
 * // => '踏込'
 * strip_okurigana('お祝い', { all: true })
 * // => '祝'
 */
fn strip_okurigana(input: &str, options = { all: false }) {
  if (is_empty(input) || !is_japanese(input) || is_kana(input)) return input;
  const chars = [...input];

  // strip every kana
  if (options.all) return chars.filter((char) => !is_char_kana(char)).join('');

  // strip trailing only
  const reverse_chars = chars.reverse();
  for (let i = 0, len = reverse_chars.length; i < len; i += 1) {
    const char = reverse_chars[i];
    // pass if it's punctuation
    if (is_char_punctuation(char)) {
      continue; // eslint-disable-line no-continue
    }
    // blank out if not kanji
    if (!is_kanji(char)) {
      reverse_chars[i] = '';
    } else {
      break; // stop when we hit a kanji char
    }
  }

  return reverse_chars.reverse().join('');
}

export default strip_okurigana;

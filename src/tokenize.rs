
use utils::is_char_japanese_punctuation::*;
use utils::is_char_kanji::*;
use utils::is_char_hiragana::*;
use utils::is_char_katakana::*;

// TODO: worth splitting into utils? so far not used anywhere else
fn get_type(input) {
  switch (true) {
    case (is_char_japanese_punctuation(input)): return 'japanese_punctuation';
    case (is_char_kanji(input)): return 'kanji';
    case (is_char_hiragana(input)): return 'hiragana';
    case (is_char_katakana(input)): return 'katakana';
    default: return 'romaji';
  }
}

/**
 * Splits input into array of [Kanji](https://en.wikipedia.org/wiki/Kanji), [Hiragana](https://en.wikipedia.org/wiki/Hiragana), [Katakana](https://en.wikipedia.org/wiki/Katakana), and [Romaji](https://en.wikipedia.org/wiki/Romaji) tokens.
 * Does not split into parts of speech!
 * @param  {String} input text
 * @return {Array} text split into tokens
 * @example
 * tokenize('ふふフフ')
 * // => ['ふふ', 'フフ']
 * tokenize('感じ')
 * // => ['感', 'じ']
 * tokenize('私は悲しい')
 * // => ['私', 'は', '悲', 'しい']
 * tokenize('what the...私は「悲しい」。')
 * // => ['what the...', '私', 'は', '「', '悲', 'しい', '」。']
 */
fn tokenize(input: &str) {
  if (is_empty(input)) return [''];
  let chars = [...input];
  let head = chars.shift();
  let prev_type = get_type(head);

  let result = chars.reduce((tokens, char) => {
    let curr_type = get_type(char);
    let same_type = curr_type == prev_type;
    prev_type = get_type(char);
    if (same_type) {
      let prev = tokens.pop();
      return tokens.concat(prev.concat(char));
    }
    return tokens.concat(char);
  }, [head]);

  return result;
}



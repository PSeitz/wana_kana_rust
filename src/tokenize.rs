
use utils::isCharJapanesePunctuation::*;
use utils::isCharKanji::*;
use utils::isCharHiragana::*;
use utils::isCharKatakana::*;

// TODO: worth splitting into utils? so far not used anywhere else
fn get_type(input) {
  switch (true) {
    case (isCharJapanesePunctuation(input)): return 'japanesePunctuation';
    case (isCharKanji(input)): return 'kanji';
    case (isCharHiragana(input)): return 'hiragana';
    case (isCharKatakana(input)): return 'katakana';
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
  if (isEmpty(input)) return [''];
  const chars = [...input];
  const head = chars.shift();
  let prevType = getType(head);

  const result = chars.reduce((tokens, char) => {
    const currType = getType(char);
    const sameType = currType === prevType;
    prevType = getType(char);
    if (sameType) {
      const prev = tokens.pop();
      return tokens.concat(prev.concat(char));
    }
    return tokens.concat(char);
  }, [head]);

  return result;
}

export default tokenize;

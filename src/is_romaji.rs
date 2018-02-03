
use utils::is_char_romaji::*;

/**
 * Test if `input` is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
 * @param  {String} [input=''] text
 * @return {Boolean} true if [Romaji](https://en.wikipedia.org/wiki/Romaji)
 * @example
 * is_romaji('Tōkyō and Ōsaka')
 * // => true
 * is_romaji('12a*b&c-d')
 * // => true
 * is_romaji('あアA')
 * // => false
 * is_romaji('お願い')
 * // => false
 * is_romaji('a！b&cーd') // Full-width punctuation fails
 * // => false
 */
pub fn is_romaji(input: &str) -> bool {
  if input.is_empty(){return false;}
  return input.chars().all(is_char_romaji);
}



use constants::ROMAJI_RANGES;
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
///
/// @param  {String} char character string to test
///

pub fn is_char_romaji(char: char) -> bool {
    return ROMAJI_RANGES.iter().any(|el: &[u32; 2]| is_char_in_range(char, el[0], el[1]));
}

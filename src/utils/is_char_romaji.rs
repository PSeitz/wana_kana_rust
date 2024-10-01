use crate::constants::ROMAJI_RANGES;
use crate::utils::is_char_in_range::*;

/// Tests a character. Returns true if the character is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
pub fn is_char_romaji(char: char) -> bool {
    ROMAJI_RANGES
        .iter()
        .any(|el: &[u32; 2]| is_char_in_range(char, el[0], el[1]))
}

#[test]
fn is_char_romajin_test() {
    assert!(is_char_romaji('n'));
    assert!(is_char_romaji('!'));
    assert!(!is_char_romaji('ナ'));
    assert!(!is_char_romaji('は'));
    assert!(!is_char_romaji('缶'));
}

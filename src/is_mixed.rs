use crate::utils::is_char_hiragana::is_char_hiragana;
use crate::utils::is_char_kanji::is_char_kanji;
use crate::utils::is_char_katakana::is_char_katakana;
use crate::utils::is_char_romaji::is_char_romaji;

#[inline]
/// Test if `input` contains a mix of [Romaji](https://en.wikipedia.org/wiki/Romaji) and [Kana](https://en.wikipedia.org/wiki/Kana).
/// [Kanji](https://en.wikipedia.org/wiki/Kanji) is ignored
pub fn is_mixed(input: &str) -> bool {
    is_mixed_pass_kanji(input, true)
}

/// Test if `input` contains a mix of [Romaji](https://en.wikipedia.org/wiki/Romaji) and [Kana](https://en.wikipedia.org/wiki/Kana)
#[inline]
pub fn is_mixed_pass_kanji(input: &str, pass_kanji: bool) -> bool {
    let mut has_kanji = false;
    if !pass_kanji {
        has_kanji = input.chars().any(is_char_kanji);
    }
    (input.chars().any(is_char_hiragana) || input.chars().any(is_char_katakana))
        && input.chars().any(is_char_romaji)
        && !has_kanji
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sane_defaults() {
        assert_eq!(is_mixed(""), false);
    }
    #[test]
    fn aア_is_mixed() {
        assert_eq!(is_mixed("Aア"), true);
    }
    #[test]
    fn aあ_is_mixed() {
        assert_eq!(is_mixed("Aあ"), true);
    }
    #[test]
    fn aあア_is_mixed() {
        assert_eq!(is_mixed("Aあア"), true);
    }
    #[test]
    fn number_2あア_is_not_mixed() {
        assert_eq!(is_mixed("２あア"), false);
    }
    #[test]
    fn お腹a_is_mixed() {
        assert_eq!(is_mixed("お腹A"), true);
    }
    #[test]
    fn お腹a_is_not_mixed_when_pass_kanji_false() {
        assert_eq!(is_mixed_pass_kanji("お腹A", false), false);
    }
    #[test]
    fn お腹_is_not_mixed() {
        assert_eq!(is_mixed("お腹"), false);
    }
    #[test]
    fn 腹_is_not_mixed() {
        assert_eq!(is_mixed("腹"), false);
    }
    #[test]
    fn a_is_not_mixed() {
        assert_eq!(is_mixed("A"), false);
    }
    #[test]
    fn あ_is_not_mixed() {
        assert_eq!(is_mixed("あ"), false);
    }
    #[test]
    fn ア_is_not_mixed() {
        assert_eq!(is_mixed("ア"), false);
    }
}

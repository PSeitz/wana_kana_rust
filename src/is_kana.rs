use crate::utils::is_char_kana::*;

/// Test if all chars of `input` are [Kana](https://en.wikipedia.org/wiki/Kana) ([Katakana](https://en.wikipedia.org/wiki/Katakana) and/or [Hiragana](https://en.wikipedia.org/wiki/Hiragana))
pub fn is_kana(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(is_char_kana)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sane_defaults() {
        assert_eq!(is_kana(""), false);
    }
    #[test]
    fn あ_is_kana() {
        assert_eq!(is_kana("あ"), true);
    }
    #[test]
    fn ア_is_kana() {
        assert_eq!(is_kana("ア"), true);
    }
    #[test]
    fn あア_is_kana() {
        assert_eq!(is_kana("あア"), true);
    }
    #[test]
    fn a_is_not_kana() {
        assert_eq!(is_kana("A"), false);
    }
    #[test]
    fn あaア_is_not_kana() {
        assert_eq!(is_kana("あAア"), false);
    }
    #[test]
    fn ignores_long_dash_in_mixed_kana() {
        assert_eq!(is_kana("アーあ"), true);
    }
}

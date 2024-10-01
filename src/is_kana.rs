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
        assert!(!is_kana(""));
    }
    #[test]
    fn あ_is_kana() {
        assert!(is_kana("あ"));
    }
    #[test]
    fn ア_is_kana() {
        assert!(is_kana("ア"));
    }
    #[test]
    fn あア_is_kana() {
        assert!(is_kana("あア"));
    }
    #[test]
    fn a_is_not_kana() {
        assert!(!is_kana("A"));
    }
    #[test]
    fn あaア_is_not_kana() {
        assert!(!is_kana("あAア"));
    }
    #[test]
    fn ignores_long_dash_in_mixed_kana() {
        assert!(is_kana("アーあ"));
    }
}

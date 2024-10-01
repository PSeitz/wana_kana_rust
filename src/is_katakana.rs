use crate::utils::is_char_katakana::*;

/// Test if all chars of `input` are [Katakana](https://en.wikipedia.org/wiki/Katakana)
pub fn is_katakana(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(is_char_katakana)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sane_defaults() {
        assert!(!is_katakana(""));
    }
    #[test]
    fn アア_is_katakana() {
        assert!(is_katakana("アア"));
    }
    #[test]
    fn ア_is_katakana() {
        assert!(is_katakana("ア"));
    }
    #[test]
    fn あ_is_not_katakana() {
        assert!(!is_katakana("あ"));
    }
    #[test]
    fn a_is_not_katakana() {
        assert!(!is_katakana("A"));
    }
    #[test]
    fn あア_is_not_katakana() {
        assert!(!is_katakana("あア"));
    }
    #[test]
    fn ignores_long_dash_in_katakana() {
        assert!(is_katakana("ゲーム"));
    }
}

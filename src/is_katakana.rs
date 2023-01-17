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
        assert_eq!(is_katakana(""), false);
    }
    #[test]
    fn アア_is_katakana() {
        assert_eq!(is_katakana("アア"), true);
    }
    #[test]
    fn ア_is_katakana() {
        assert_eq!(is_katakana("ア"), true);
    }
    #[test]
    fn あ_is_not_katakana() {
        assert_eq!(is_katakana("あ"), false);
    }
    #[test]
    fn a_is_not_katakana() {
        assert_eq!(is_katakana("A"), false);
    }
    #[test]
    fn あア_is_not_katakana() {
        assert_eq!(is_katakana("あア"), false);
    }
    #[test]
    fn ignores_long_dash_in_katakana() {
        assert_eq!(is_katakana("ゲーム"), true);
    }
}

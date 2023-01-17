use crate::utils::is_char_hiragana::*;

/// Test if all chars of `input` are [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
pub fn is_hiragana(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(is_char_hiragana)
}

#[test]
fn check_is_hiragana() {
    assert_eq!(is_hiragana("げーむ"), true);
    assert_eq!(is_hiragana("A"), false);
    assert_eq!(is_hiragana("あア"), false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sane_defaults() {
        assert_eq!(is_hiragana(""), false);
    }
    #[test]
    fn あ_is_hiragana() {
        assert_eq!(is_hiragana("あ"), true);
    }
    #[test]
    fn ああ_is_hiragana() {
        assert_eq!(is_hiragana("ああ"), true);
    }
    #[test]
    fn ア_is_not_hiragana() {
        assert_eq!(is_hiragana("ア"), false);
    }
    #[test]
    fn a_is_not_hiragana() {
        assert_eq!(is_hiragana("A"), false);
    }
    #[test]
    fn あア_is_not_hiragana() {
        assert_eq!(is_hiragana("あア"), false);
    }
    #[test]
    fn ignores_long_dash_in_hiragana() {
        assert_eq!(is_hiragana("げーむ"), true);
    }
}

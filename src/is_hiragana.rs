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
    assert!(is_hiragana("げーむ"));
    assert!(!is_hiragana("A"));
    assert!(!is_hiragana("あア"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sane_defaults() {
        assert!(!is_hiragana(""));
    }
    #[test]
    fn あ_is_hiragana() {
        assert!(is_hiragana("あ"));
    }
    #[test]
    fn ああ_is_hiragana() {
        assert!(is_hiragana("ああ"));
    }
    #[test]
    fn ア_is_not_hiragana() {
        assert!(!is_hiragana("ア"));
    }
    #[test]
    fn a_is_not_hiragana() {
        assert!(!is_hiragana("A"));
    }
    #[test]
    fn あア_is_not_hiragana() {
        assert!(!is_hiragana("あア"));
    }
    #[test]
    fn ignores_long_dash_in_hiragana() {
        assert!(is_hiragana("げーむ"));
    }
}

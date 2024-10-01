use crate::utils::is_char_kanji::*;

/// Test if all chars of `input` are [Kanji](https://en.wikipedia.org/wiki/Kanji) ([Japanese CJK ideographs](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs))
#[inline]
pub fn is_kanji(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(is_char_kanji)
}

/// Test if any chars of `input` are [Kanji](https://en.wikipedia.org/wiki/Kanji) ([Japanese CJK ideographs](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs))
#[inline]
pub fn contains_kanji(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().any(is_char_kanji)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sane_defaults() {
        assert!(!is_kanji(""));
        assert!(!contains_kanji(""));
    }

    #[test]
    fn 勢い_contains_kanji() {
        assert!(contains_kanji("勢い"));
    }
    #[test]
    fn hello_contains_not_kanji() {
        assert!(!contains_kanji("hello"));
    }
    #[test]
    fn 切腹_is_kanji() {
        assert!(is_kanji("切腹"));
    }
    #[test]
    fn 刀_is_kanji() {
        assert!(is_kanji("刀"));
    }
    #[test]
    fn emoji_are_not_kanji() {
        assert!(!is_kanji("🐸"));
    }
    #[test]
    fn あ_is_not_kanji() {
        assert!(!is_kanji("あ"));
    }
    #[test]
    fn ア_is_not_kanji() {
        assert!(!is_kanji("ア"));
    }
    #[test]
    fn あア_is_not_kanji() {
        assert!(!is_kanji("あア"));
    }
    #[test]
    fn a_is_not_kanji() {
        assert!(!is_kanji("A"));
    }
    #[test]
    fn あaア_is_not_kanji() {
        assert!(!is_kanji("あAア"));
    }
    #[test]
    fn number_with_kanj_is_not_kanji1() {
        assert!(!is_kanji("１２隻"));
    }
    #[test]
    fn number_with_kanj_is_not_kanji2() {
        assert!(!is_kanji("12隻"));
    }
    #[test]
    fn kanji_with_dot_is_not_kanji() {
        assert!(!is_kanji("隻。"));
    }
}

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
        assert_eq!(is_kanji(""), false);
        assert_eq!(contains_kanji(""), false);
    }

    #[test]
    fn 勢い_contains_kanji() {
        assert_eq!(contains_kanji("勢い"), true);
    }
    #[test]
    fn hello_contains_not_kanji() {
        assert_eq!(contains_kanji("hello"), false);
    }
    #[test]
    fn 切腹_is_kanji() {
        assert_eq!(is_kanji("切腹"), true);
    }
    #[test]
    fn 刀_is_kanji() {
        assert_eq!(is_kanji("刀"), true);
    }
    #[test]
    fn emoji_are_not_kanji() {
        assert_eq!(is_kanji("🐸"), false);
    }
    #[test]
    fn あ_is_not_kanji() {
        assert_eq!(is_kanji("あ"), false);
    }
    #[test]
    fn ア_is_not_kanji() {
        assert_eq!(is_kanji("ア"), false);
    }
    #[test]
    fn あア_is_not_kanji() {
        assert_eq!(is_kanji("あア"), false);
    }
    #[test]
    fn a_is_not_kanji() {
        assert_eq!(is_kanji("A"), false);
    }
    #[test]
    fn あaア_is_not_kanji() {
        assert_eq!(is_kanji("あAア"), false);
    }
    #[test]
    fn number_with_kanj_is_not_kanji1() {
        assert_eq!(is_kanji("１２隻"), false);
    }
    #[test]
    fn number_with_kanj_is_not_kanji2() {
        assert_eq!(is_kanji("12隻"), false);
    }
    #[test]
    fn kanji_with_dot_is_not_kanji() {
        assert_eq!(is_kanji("隻。"), false);
    }
}

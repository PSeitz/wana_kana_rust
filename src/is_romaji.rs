#[cfg(feature = "enable_regex")]
use regex::Regex;

use crate::utils::is_char_romaji::*;

/// Test if every char in `input` is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
pub fn is_romaji(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(is_char_romaji)
}

#[cfg(feature = "enable_regex")]
/// Test if every char in `input` is [Romaji](https://en.wikipedia.org/wiki/Romaji) or matches the
/// provided regex
pub fn is_romaji_with_whitelist(input: &str, allowed: Option<&Regex>) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(|char| {
        let is_jap = is_char_romaji(char);
        if !is_jap {
            if let Some(allowed) = allowed {
                return allowed.is_match(input);
            }
        }
        is_jap
    })
}

#[test]
fn check_is_romaji() {
    assert!(is_romaji("Tōkyō and Ōsaka"));
    assert!(is_romaji("12a*b&c-d"));
    assert!(!is_romaji("あアA"));
    assert!(!is_romaji("お願い"));
    assert!(!is_romaji("a！b&cーd"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sane_defaults() {
        assert!(!is_romaji(""));

        #[cfg(feature = "enable_regex")]
        assert_eq!(is_romaji_with_whitelist("", None), false);
    }
    #[test]
    fn a_is_romaji() {
        assert!(is_romaji("A"));
    }
    #[test]
    fn x_yz_is_romaji() {
        assert!(is_romaji("xYz"));
    }
    #[test]
    fn tōkyō_and_ōsaka_is_romaji() {
        assert!(is_romaji("Tōkyō and Ōsaka"));
    }
    #[test]
    fn あアa_is_not_romaji() {
        assert!(!is_romaji("あアA"));
    }
    #[test]
    fn お願い_is_not_romaji() {
        assert!(!is_romaji("お願い"));
    }
    #[test]
    fn 熟成_is_not_romaji() {
        assert!(!is_romaji("熟成"));
    }
    #[test]
    fn passes_latin_punctuation() {
        assert!(is_romaji("a*b&c-d"));
    }
    #[test]
    fn passes_latin_numbers() {
        assert!(is_romaji("0123456789"));
    }
    #[test]
    fn fails_zenkaku_punctuation() {
        assert!(!is_romaji("a！b&cーd"));
    }
    #[test]
    fn fails_zenkaku_latin() {
        assert!(!is_romaji("ｈｅｌｌｏ"));
    }

    #[cfg(feature = "enable_regex")]
    #[test]
    fn accepts_optional_allowed_chars() {
        assert_eq!(
            is_romaji_with_whitelist("a！b&cーd", Some(&Regex::new(r"[！ー]").unwrap())),
            true
        );
    }
}

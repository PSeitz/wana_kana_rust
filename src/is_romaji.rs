//! Test if every char in `input` is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
//!
//! # Examples
//! ```
//! use wana_kana::is_romaji::*;
//! use wana_kana::Options;
//! assert_eq!(is_romaji("Tōkyō and Ōsaka"), true);
//! assert_eq!(is_romaji("12a*b&c-d"), true);
//! assert_eq!(is_romaji("あアA"), false);
//! assert_eq!(is_romaji("お願い"), false);
//! assert_eq!(is_romaji("a！b&cーd"), false);
//! ```

use crate::utils::is_char_romaji::*;

pub fn is_romaji(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    return input.chars().all(is_char_romaji);
}

#[test]
fn check_is_romaji() {
    assert_eq!(is_romaji("Tōkyō and Ōsaka"), true);
    assert_eq!(is_romaji("12a*b&c-d"), true);
    assert_eq!(is_romaji("あアA"), false);
    assert_eq!(is_romaji("お願い"), false);
    assert_eq!(is_romaji("a！b&cーd"), false);
}

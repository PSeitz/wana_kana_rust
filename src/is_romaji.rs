//! Test if every char in `input` is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
//!
//! # Examples
//! ```
//! use wana_kana::is_romaji::*;
//! use regex::Regex;
//! assert_eq!(is_romaji("A"), true);
//! assert_eq!(is_romaji("xYz"), true);
//! assert_eq!(is_romaji("Tōkyō and Ōsaka"), true);
//! assert_eq!(is_romaji("あアA"), false);
//! assert_eq!(is_romaji("お願い"), false);
//! assert_eq!(is_romaji("熟成"), false);
//! assert_eq!(is_romaji("a*b&c-d"), true);
//! assert_eq!(is_romaji("0123456789"), true);
//! assert_eq!(is_romaji("a！b&cーd"), false);
//! assert_eq!(is_romaji("ｈｅｌｌｏ"), false);
//! assert_eq!(is_romaji_with_whitelist("a！b&cーd", Some(&Regex::new(r"[！ー]").unwrap())), true);
//! ```

use crate::utils::is_char_romaji::*;
use regex::Regex;

pub fn is_romaji(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    input.chars().all(|char| is_char_romaji(char))
}

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
    assert_eq!(is_romaji("Tōkyō and Ōsaka"), true);
    assert_eq!(is_romaji("12a*b&c-d"), true);
    assert_eq!(is_romaji("あアA"), false);
    assert_eq!(is_romaji("お願い"), false);
    assert_eq!(is_romaji("a！b&cーd"), false);
}

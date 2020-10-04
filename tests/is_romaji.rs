#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

#[cfg(feature = "enable_regex")]
use regex::Regex;
use wana_kana::is_romaji::*;

speculate! {
    it "sane defaults" {
        assert_eq!(is_romaji(""), true);
    }
    it "A is romaji" { assert_eq!(is_romaji("A"), true); }
    it "xYz is romaji" { assert_eq!(is_romaji("xYz"), true); }
    it "Tōkyō and Ōsaka is romaji" { assert_eq!(is_romaji("Tōkyō and Ōsaka"), true); }
    it "あアA is not romaji" { assert_eq!(is_romaji("あアA"), false); }
    it "お願い is not romaji" { assert_eq!(is_romaji("お願い"), false); }
    it "熟成 is not romaji" { assert_eq!(is_romaji("熟成"), false); }
    it "passes latin punctuation" { assert_eq!(is_romaji("a*b&c-d"), true); }
    it "passes latin numbers" { assert_eq!(is_romaji("0123456789"), true); }
    it "fails zenkaku punctuation" { assert_eq!(is_romaji("a！b&cーd"), false); }
    it "fails zenkaku latin" { assert_eq!(is_romaji("ｈｅｌｌｏ"), false); }

    #[cfg(feature = "enable_regex")]
    it "accepts optional allowed chars" { assert_eq!(is_romaji_with_whitelist("a！b&cーd", &Regex::new(r"[！ー]").unwrap()), true); }
}

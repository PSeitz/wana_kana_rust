#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::is_hiragana::*;


speculate!{
    it "sane defaults" {
        assert_eq!(is_hiragana(""), false);
    }
    it "あ is hiragana" { assert_eq!(is_hiragana("あ"), true); }
    it "ああ is hiragana" { assert_eq!(is_hiragana("ああ"), true); }
    it "ア is not hiragana" { assert_eq!(is_hiragana("ア"), false); }
    it "A is not hiragana" { assert_eq!(is_hiragana("A"), false); }
    it "あア is not hiragana" { assert_eq!(is_hiragana("あア"), false); }
    it "ignores long dash in hiragana" { assert_eq!(is_hiragana("げーむ"), true); }
}

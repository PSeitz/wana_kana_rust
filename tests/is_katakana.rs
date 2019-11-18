#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::is_katakana::*;

speculate! {
    it "sane defaults" {
        assert_eq!(is_katakana(""), false);
    }
    it "アア is katakana" { assert_eq!(is_katakana("アア"), true); }
    it "ア is katakana" { assert_eq!(is_katakana("ア"), true); }
    it "あ is not katakana" { assert_eq!(is_katakana("あ"), false); }
    it "A is not katakana" { assert_eq!(is_katakana("A"), false); }
    it "あア is not katakana" { assert_eq!(is_katakana("あア"), false); }
    it "ignores long dash in katakana" { assert_eq!(is_katakana("ゲーム"), true); }
}

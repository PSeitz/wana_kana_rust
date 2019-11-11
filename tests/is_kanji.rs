#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::is_kanji::*;


speculate!{
    it "sane defaults" {
        assert_eq!(is_kanji(""), false);
    }
    it "切腹 is kanji" { assert_eq!(is_kanji("切腹"), true); }
    it "刀 is kanji" { assert_eq!(is_kanji("刀"), true); }
    it "emoji are not kanji" { assert_eq!(is_kanji("🐸"), false); }
    it "あ is not kanji" { assert_eq!(is_kanji("あ"), false); }
    it "ア is not kanji" { assert_eq!(is_kanji("ア"), false); }
    it "あア is not kanji" { assert_eq!(is_kanji("あア"), false); }
    it "A is not kanji" { assert_eq!(is_kanji("A"), false); }
    it "あAア is not kanji" { assert_eq!(is_kanji("あAア"), false); }
    it "１２隻 is not kanji" { assert_eq!(is_kanji("１２隻"), false); }
    it "12隻 is not kanji" { assert_eq!(is_kanji("12隻"), false); }
    it "隻。 is not kanji" { assert_eq!(is_kanji("隻。"), false); }
}

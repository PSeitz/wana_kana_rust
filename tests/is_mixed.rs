#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::is_mixed::*;

speculate! {
    it "sane defaults" {
        assert_eq!(is_mixed(""), false);
    }
    it "Aア is mixed"{ assert_eq!(is_mixed("Aア"), true); }
    it "Aあ is mixed"{ assert_eq!(is_mixed("Aあ"), true); }
    it "Aあア is mixed"{ assert_eq!(is_mixed("Aあア"), true); }
    it "２あア is not mixed"{ assert_eq!(is_mixed("２あア"), false); }
    it "お腹A is mixed"{ assert_eq!(is_mixed("お腹A"), true); }
    it "お腹A is not mixed when { passKanji: false }"{ assert_eq!(is_mixed_pass_kanji("お腹A", false), false); }
    it "お腹 is not mixed"{ assert_eq!(is_mixed("お腹"), false); }
    it "腹 is not mixed"{ assert_eq!(is_mixed("腹"), false); }
    it "A is not mixed"{ assert_eq!(is_mixed("A"), false); }
    it "あ is not mixed"{ assert_eq!(is_mixed("あ"), false); }
    it "ア is not mixed"{ assert_eq!(is_mixed("ア"), false); }
}

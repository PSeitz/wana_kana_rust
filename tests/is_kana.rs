#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::is_kana::*;

speculate! {
    it "sane defaults" {
        assert_eq!(is_kana(""), true);
    }
    it "あ is kana" { assert_eq!(is_kana("あ"), true);}
    it "ア is kana" { assert_eq!(is_kana("ア"), true);}
    it "あア is kana" { assert_eq!(is_kana("あア"), true);}
    it "A is not kana" { assert_eq!(is_kana("A"), false);}
    it "あAア is not kana" { assert_eq!(is_kana("あAア"), false);}
    it "ignores long dash in mixed kana" { assert_eq!(is_kana("アーあ"), true);}
}

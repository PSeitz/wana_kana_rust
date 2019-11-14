#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::Options;
use wana_kana::to_kana::*;

fn with_obsolete_kana() -> Options {
    Options{ use_obsolete_kana: true, .. Default::default() }
}


speculate!{

    it "sane defaults" {
        assert_eq!(to_kana(""), "");
    }
    it "uppercase ist katakana defaults" {
        assert_eq!(to_kana("WANAKANA"), "ワナカナ");
    }


}

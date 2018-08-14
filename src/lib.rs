//!
//! ### ワナカナ <--> WanaKana <--> わなかな
//!
//! Utility library for checking and converting between Japanese characters - Kanji, Hiragana, Katakana - and Romaji (Ported from https://github.com/WaniKani/WanaKana)
//! # Examples
//! ```
//! use wana_kana::to_romaji::*;
//! use wana_kana::to_kana::*;
//! use wana_kana::to_hiragana::*;
//! use wana_kana::Options;
//! assert_eq!(to_romaji("ワナカナ"), "wanakana");
//! assert_eq!(to_hiragana("WanaKana"), "わなかな");
//! assert_eq!(to_kana("WANAKANA"), "ワナカナ");
//! ```

#![feature(plugin)]
#![feature(slice_patterns)]
#![feature(test)]
#![feature(non_ascii_idents)]
#![cfg_attr(test, plugin(stainless))]

extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate fnv;
extern crate regex;

#[cfg(test)]
extern crate test;

pub mod is_kanji;
pub mod is_kana;
pub mod is_katakana;
pub mod is_romaji;
pub mod is_japanese;
pub mod is_hiragana;
pub mod is_mixed;

pub mod to_kana;
pub mod to_katakana;
pub mod to_hiragana;
pub mod to_romaji;

pub mod strip_okurigana;
pub mod tokenize;

mod utils;
mod options;
mod constants;

pub use options::Options;


#[cfg(test)]
mod tests;

#[bench]
fn bench_kana_1(b: &mut test::Bencher) {
    b.iter(|| to_kana::to_kana("aiueosashisusesonaninunenokakikukeko"))
}
#[bench]
fn bench_kana_2(b: &mut test::Bencher) {
    b.iter(|| to_kana::to_kana("AIUEOSASHISUSESONANINUNENOKAKIKUKEKO"))
}

#[bench]
fn bench_romaji_1(b: &mut test::Bencher) {
    b.iter(|| to_romaji::to_romaji("あいうえおさしすせそなにぬねのかきくけこ"))
}
#[bench]
fn bench_romaji_2(b: &mut test::Bencher) {
    b.iter(|| to_romaji::to_romaji("アイウエオサシスセソナニヌネノカキクケコ"))
}

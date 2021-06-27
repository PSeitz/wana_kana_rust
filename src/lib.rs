//! # ワナカナ <--> WanaKana <--> わなかな
//!
//! Utility library for checking and converting between Japanese characters - Kanji,
//! Hiragana, Katakana - and Romaji (Ported from <https://github.com/WaniKani/WanaKana>)
//!
//! # Conversions
//!
//! You can use `to_*` functions in `to_*` modules, e.g. `wana_naka::to_romaji::to_romaji`
//! to do conversions, for example:
//!
//! ```
//! use wana_kana::to_romaji::*;
//! use wana_kana::to_kana::*;
//! use wana_kana::to_hiragana::*;
//! use wana_kana::Options;
//! assert_eq!(to_romaji("ワナカナ"), "wanakana");
//! assert_eq!(to_hiragana("WanaKana"), "わなかな");
//! assert_eq!(to_kana("WANAKANA"), "ワナカナ");
//! ```
//!
//! We also provide the [`ConvertJapanese`] Trait, which is implemented for `&str`,
//! that allows the same conversions as mentioned above, by calling the
//! corresponding method directly on the `&str`.
//!
//! ```
//! use wana_kana::ConvertJapanese;
//! // to kana
//! assert_eq!("o".to_kana(), "お");
//! assert_eq!("ona".to_kana(), "おな");
//! assert_eq!("onaji".to_kana(), "おなじ");
//! assert_eq!("onaji BUTTSUUJI".to_kana(), "おなじ ブッツウジ");
//! assert_eq!("ONAJI buttsuuji".to_kana(), "オナジ ぶっつうじ");
//! assert_eq!("座禅‘zazen’スタイル".to_kana(), "座禅「ざぜん」スタイル");
//! // to hiragana
//! assert_eq!("toukyou,オオサカ".to_hiragana(), "とうきょう、おおさか");
//! // to katakana
//! assert_eq!("toukyou,おおさか".to_katakana(), "トウキョウ、オオサカ");
//! // to romaji
//! assert_eq!("ひらがな　カタカナ".to_romaji(), "hiragana katakana");
//! ```
//!
//! To check whether a string is Japanese, romaji, kana, etc, use the `is_*` functions
//! in `is_*` modules, or equivalently use the [`IsJapaneseStr`] trait.
//! There are also functions to check a single `char` available in the [`utils`] module,
//! and the corresponding trait is [`IsJapaneseChar`]. Refer to the links for detailed
//! information.
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "enable_regex")]
extern crate regex;

#[macro_use]
pub mod utils;

pub mod is_hiragana;
pub mod is_japanese;
pub mod is_kana;
pub mod is_kanji;
pub mod is_katakana;
pub mod is_mixed;
pub mod is_romaji;

pub mod to_hiragana;
pub mod to_kana;
pub mod to_katakana;
pub mod to_romaji;

pub mod tokenize;
pub mod trim_okurigana;

pub mod constants;
mod options;

pub use crate::options::Options;

mod traits;
pub use traits::{ConvertJapanese, IsJapaneseChar, IsJapaneseStr};

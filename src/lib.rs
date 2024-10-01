//! # ワナカナ <--> WanaKana <--> わなかな
//!
//! Utility library for checking and converting between Japanese characters -
//! Hiragana, Katakana - and Romaji (Ported from <https://github.com/WaniKani/WanaKana>)
//!
//! # Conversions
//!
//! We provide the [`ConvertJapanese`] Trait, which is implemented for `&str`,
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
//! # Checks
//! To check whether a string is Japanese, romaji, kana, etc check the [`IsJapaneseStr`] trait.
//! There are also functions to check a single `char` in the is [`IsJapaneseChar`] trait.
//!
//! ```
//! use wana_kana::IsJapaneseStr;
//! assert_eq!("げーむ".is_hiragana(), true);
//! assert_eq!("ア".is_kana(), true);
//! assert_eq!("Tōkyō and Ōsaka".is_romaji(), true);
//! assert_eq!("切腹".is_kanji(), true);
//! assert_eq!("Aあア".is_mixed(), true);
//! assert_eq!("勢い".contains_kanji(), true);
//! ```
//!
//! # Tokenize
//!
//! [`tokenize`] Splits input into array of strings separated by opinionated TokenType.

#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "enable_regex")]
extern crate regex;

#[macro_use]
#[allow(missing_docs)]
pub mod utils;

pub(crate) mod is_hiragana;
pub(crate) mod is_japanese;
pub(crate) mod is_kana;
pub(crate) mod is_kanji;
pub(crate) mod is_katakana;
pub(crate) mod is_mixed;
pub(crate) mod is_romaji;

pub(crate) mod to_hiragana;
pub(crate) mod to_kana;
pub(crate) mod to_kana_node_tree;
pub(crate) mod to_katakana;
pub(crate) mod to_romaji;
pub(crate) mod to_romaji_node_tree;

#[cfg_attr(docsrs, doc(cfg(feature = "tokenize")))]
#[cfg(feature = "tokenize")]
pub mod tokenize;
#[cfg_attr(docsrs, doc(cfg(feature = "tokenize")))]
#[cfg(feature = "tokenize")]
pub mod trim_okurigana;

#[allow(missing_docs)]
pub mod constants;
mod options;

pub use crate::options::Options;

pub mod traits;
pub use traits::{ConvertJapanese, IsJapaneseChar, IsJapaneseStr};

#[cfg(test)]
mod proptests {
    use super::*;

    #[test]
    fn test_regression_11() {
        assert_eq!("シークヮーサー".to_hiragana(), "しいくゎあさあ");
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn to_kana_does_not_panic(input in ".*") {
            let _ = input.to_kana();
        }

        #[test]
        fn to_romaji_does_not_panic(input in ".*") {
            let _ = &input.to_romaji();
        }

        #[test]
        fn to_romaji_does_not_panic_kana_katakana_input(
            input in r"[ぁ-ゟァ-ヿー]{1,100}"
        ) {
                let _ = &input.to_romaji();
        }

        #[test]
        fn to_katakana_does_not_panic(input in ".*") {
            let _ = &input.to_katakana();
        }

        #[test]
        fn to_hiragana_does_not_panic(input in ".*") {
            let _ = &input.to_hiragana();
        }

        #[test]
        fn mixed_romaji_kana_katakana_input_does_not_panic(
            input in r"[A-Za-zぁ-ゟァ-ヿー]{1,100}"
        ) {
            let _ = input.to_kana();
            let _ = input.to_romaji();
            let _ = input.to_katakana();
            let _ = input.to_hiragana();
        }

    }
}

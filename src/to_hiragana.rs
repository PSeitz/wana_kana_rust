//! Convert input to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
//!
//! # Examples
//! ```
//! use wana_kana::to_hiragana::*;
//! use wana_kana::Options;
//! assert_eq!(to_hiragana("toukyou,オオサカ"), "とうきょう、おおさか");
//! assert_eq!(to_hiragana_with_opt("only かな", Options {pass_romaji: true, ..Default::default() } ), "only かな");
//! assert_eq!(to_hiragana("wi"), "うぃ"); assert_eq!(to_hiragana_with_opt("wi", Options {use_obsolete_kana: true, ..Default::default() } ), "ゐ");
//! ```

use crate::is_mixed::*;
use crate::is_romaji::*;
use crate::options::Options;
use crate::utils::katakana_to_hiragana::*;
use crate::utils::romaji_to_hiragana::romaji_to_hiragana;

pub fn to_hiragana(input: &str) -> String {
    to_hiragana_with_opt(input, Options::default())
}
pub fn to_hiragana_with_opt(input: &str, options: Options) -> String {
    let config = options;
    if config.pass_romaji {
        return katakana_to_hiragana(input);
    }
    if is_romaji(input) {
        return romaji_to_hiragana(input, config);
    }
    if is_mixed(input) {
        let romaji = katakana_to_hiragana(input);
        return romaji_to_hiragana(&romaji, config);
    }
    return katakana_to_hiragana(input);
}

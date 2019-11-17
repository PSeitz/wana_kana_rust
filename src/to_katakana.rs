//! Convert input to [Katakana](https://en.wikipedia.org/wiki/Katakana)
//!
//! # Examples
//! ```
//! use wana_kana::to_katakana::to_katakana_with_opt;
//! use wana_kana::to_katakana::to_katakana;
//! use wana_kana::Options;
//! assert_eq!(
//!     to_katakana("toukyou,おおさか"),
//!     "トウキョウ、オオサカ"
//! );
//! assert_eq!(
//!     to_katakana_with_opt(
//!         "only かな",
//!         Options {
//!             pass_romaji: true,
//!             ..Default::default()
//!         }
//!     ),
//!     "only カナ"
//! );
//! assert_eq!(to_katakana("wi"), "ウィ");
//! assert_eq!(to_katakana_with_opt("wi", Options {use_obsolete_kana: true, ..Default::default() }),"ヰ");
//! ```

use crate::is_mixed::*;
use crate::is_romaji::*;
use crate::options::Options;
use crate::utils::hiragana_to_katakana::*;
use crate::utils::romaji_to_hiragana::*;

pub fn to_katakana(input: &str) -> String {
    to_katakana_with_opt(input, Options::default())
}
pub fn to_katakana_with_opt(input: &str, options: Options) -> String {
    let config = options;
    if config.pass_romaji {
        hiragana_to_katakana(input)
    }else if is_romaji(input) || is_mixed(input) {
        let romaji = romaji_to_hiragana(input, config);
        hiragana_to_katakana(&romaji)
    }else{
        hiragana_to_katakana(input)
    }
    
}

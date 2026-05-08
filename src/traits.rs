//! The `wana_kana::ConvertJapanese` trait is implemented for `&str`, which allows
//! conversion between kana and romaji.
//!
//! # Examples
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
#[cfg(feature = "enable_regex")]
use regex::Regex;

use crate::Options;

/// The `wana_kana::ConvertJapanese` trait is implemented for `&str`, which allows
/// conversion between kana and romaji.
///
/// # Examples
///
/// ```
/// use wana_kana::ConvertJapanese;
/// // to kana
/// assert_eq!("o".to_kana(), "お");
/// assert_eq!("ona".to_kana(), "おな");
/// assert_eq!("onaji".to_kana(), "おなじ");
/// assert_eq!("onaji BUTTSUUJI".to_kana(), "おなじ ブッツウジ");
/// assert_eq!("ONAJI buttsuuji".to_kana(), "オナジ ぶっつうじ");
/// assert_eq!("座禅‘zazen’スタイル".to_kana(), "座禅「ざぜん」スタイル");
/// // to hiragana
/// assert_eq!("toukyou,オオサカ".to_hiragana(), "とうきょう、おおさか");
/// // to katakana
/// assert_eq!("toukyou,おおさか".to_katakana(), "トウキョウ、オオサカ");
/// // to romaji
/// assert_eq!("ひらがな　カタカナ".to_romaji(), "hiragana katakana");
/// ```
pub trait ConvertJapanese {
    /// Convert [Romaji](https://en.wikipedia.org/wiki/Romaji) to [Kana](https://en.wikipedia.org/wiki/Kana), lowercase text will result in [Hiragana](https://en.wikipedia.org/wiki/Hiragana) and uppercase text will result in [Katakana](https://en.wikipedia.org/wiki/Katakana).
    /// # Examples
    /// ```
    /// use wana_kana::ConvertJapanese;
    /// assert_eq!("o".to_kana(), "お");
    /// assert_eq!("ona".to_kana(), "おな");
    /// assert_eq!("onaji".to_kana(), "おなじ");
    /// assert_eq!("onaji BUTTSUUJI".to_kana(), "おなじ ブッツウジ");
    /// assert_eq!("ONAJI buttsuuji".to_kana(), "オナジ ぶっつうじ");
    /// assert_eq!("座禅‘zazen’スタイル".to_kana(), "座禅「ざぜん」スタイル");
    /// assert_eq!("!?./,~-‘’“”[](){}".to_kana(), "！？。・、〜ー「」『』［］（）｛｝");
    /// ```
    fn to_kana(self) -> String;

    /// Convert [Romaji](https://en.wikipedia.org/wiki/Romaji) to [Kana](https://en.wikipedia.org/wiki/Kana), lowercase text will result in [Hiragana](https://en.wikipedia.org/wiki/Hiragana) and uppercase text will result in [Katakana](https://en.wikipedia.org/wiki/Katakana).
    /// # Examples
    /// ```
    /// use wana_kana::ConvertJapanese;
    /// use wana_kana::Options;
    /// assert_eq!("batsuge-mu".to_kana_with_opt(Options {use_obsolete_kana: true, ..Default::default() } ), "ばつげーむ");
    /// assert_eq!("we".to_kana_with_opt(Options {use_obsolete_kana: true, ..Default::default() } ), "ゑ");
    /// ```
    fn to_kana_with_opt(self, options: Options) -> String;

    /// Convert input to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
    /// # Examples
    /// ```
    /// use wana_kana::ConvertJapanese;
    /// assert_eq!("toukyou,オオサカ".to_hiragana(), "とうきょう、おおさか");
    /// assert_eq!("wi".to_hiragana(), "うぃ");
    /// ```
    fn to_hiragana(self) -> String;

    /// Convert input to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
    /// # Examples
    /// ```
    /// use wana_kana::ConvertJapanese;
    /// use wana_kana::Options;
    /// assert_eq!("only かな".to_hiragana_with_opt(Options {pass_romaji: true, ..Default::default() } ), "only かな");
    /// assert_eq!("wi".to_hiragana_with_opt(Options {use_obsolete_kana: true, ..Default::default() } ), "ゐ");
    /// assert_eq!("スーパー".to_hiragana_with_opt(Options {keep_prolonged_sound_mark: true, ..Default::default() } ), "すーぱー");
    /// ```
    fn to_hiragana_with_opt(self, options: Options) -> String;

    /// Convert input to [Katakana](https://en.wikipedia.org/wiki/Katakana)
    fn to_katakana(self) -> String;

    /// Convert input to [Katakana](https://en.wikipedia.org/wiki/Katakana)
    fn to_katakana_with_opt(self, options: Options) -> String;

    /// Convert kana to romaji
    /// # Examples
    /// ```
    /// use wana_kana::ConvertJapanese;
    /// assert_eq!("ひらがな　カタカナ".to_romaji(), "hiragana katakana");
    /// ```
    fn to_romaji(self) -> String;

    /// Convert kana to romaji with Options.
    /// # Examples
    /// ```
    /// use wana_kana::ConvertJapanese;
    /// use wana_kana::Options;
    /// assert_eq!("ひらがな　カタカナ".to_romaji_with_opt(Options {upcase_katakana: true, ..Default::default() } ), "hiragana KATAKANA");
    /// ```

    fn to_romaji_with_opt(self, options: Options) -> String;
}

impl ConvertJapanese for &str {
    #[inline]
    fn to_kana(self) -> String {
        crate::to_kana::to_kana(self)
    }

    #[inline]
    fn to_kana_with_opt(self, options: Options) -> String {
        crate::to_kana::to_kana_with_opt(self, options)
    }

    #[inline]
    fn to_hiragana(self) -> String {
        crate::to_hiragana::to_hiragana(self)
    }

    #[inline]
    fn to_hiragana_with_opt(self, options: Options) -> String {
        crate::to_hiragana::to_hiragana_with_opt(self, options)
    }

    #[inline]
    fn to_katakana(self) -> String {
        crate::to_katakana::to_katakana(self)
    }

    #[inline]
    fn to_katakana_with_opt(self, options: Options) -> String {
        crate::to_katakana::to_katakana_with_opt(self, options)
    }

    #[inline]
    fn to_romaji(self) -> String {
        crate::to_romaji::to_romaji(self)
    }

    #[inline]
    fn to_romaji_with_opt(self, options: Options) -> String {
        crate::to_romaji::to_romaji_with_opt(self, options)
    }
}

/// The `wana_kana::IsJapaneseStr` trait is implemented for `&str`, which allows easy
/// checking of whether a string is fully composed of hiragana, katakana, kana,
/// kanji, Japanese, or mixed.
pub trait IsJapaneseStr {
    /// Test if all chars of `input` are [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// assert_eq!("げーむ".is_hiragana(), true);
    /// assert_eq!("A".is_hiragana(), false);
    /// assert_eq!("あア".is_hiragana(), false);
    /// ```
    fn is_hiragana(&self) -> bool;
    /// Test if all chars of `input` are [Katakana](https://en.wikipedia.org/wiki/Katakana)
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// assert_eq!("ゲーム".is_katakana(), true);
    /// assert_eq!("あ".is_katakana(), false);
    /// assert_eq!("A".is_katakana(), false);
    /// assert_eq!("あア".is_katakana(), false);
    /// ```
    fn is_katakana(&self) -> bool;
    /// Test if all chars of `input` are [Kana](https://en.wikipedia.org/wiki/Kana) ([Katakana](https://en.wikipedia.org/wiki/Katakana) and/or [Hiragana](https://en.wikipedia.org/wiki/Hiragana))
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// assert_eq!("あ".is_kana(), true);
    /// assert_eq!("ア".is_kana(), true);
    /// assert_eq!("あーア".is_kana(), true);
    /// assert_eq!("A".is_kana(), false);
    /// assert_eq!("あAア".is_kana(), false);
    /// ```
    fn is_kana(&self) -> bool;
    /// Test if every char in `input` is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// assert_eq!("A".is_romaji(), true);
    /// assert_eq!("xYz".is_romaji(), true);
    /// assert_eq!("Tōkyō and Ōsaka".is_romaji(), true);
    /// assert_eq!("あアA".is_romaji(), false);
    /// assert_eq!("お願い".is_romaji(), false);
    /// assert_eq!("熟成".is_romaji(), false);
    /// assert_eq!("a*b&c-d".is_romaji(), true);
    /// assert_eq!("0123456789".is_romaji(), true);
    /// assert_eq!("a！b&cーd".is_romaji(), false);
    /// assert_eq!("ｈｅｌｌｏ".is_romaji(), false);
    /// ```
    fn is_romaji(&self) -> bool;

    #[cfg_attr(docsrs, doc(cfg(feature = "enable_regex")))]
    #[cfg(feature = "enable_regex")]
    /// Test if every char in `input` is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization)
    /// or matches the provided regex
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// use regex::Regex;
    /// assert_eq!(
    ///     "a！b&cーd".is_romaji_with_whitelist(Some(&Regex::new(r"[！ー]").unwrap())),
    ///     true
    /// );
    /// ```
    fn is_romaji_with_whitelist(&self, allowed: Option<&Regex>) -> bool;

    /// Test if all chars of `input` are [Kanji](https://en.wikipedia.org/wiki/Kanji) ([Japanese CJK ideographs](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs))
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// assert_eq!("刀".is_kanji(), true);
    /// assert_eq!("切腹".is_kanji(), true);
    /// assert_eq!("勢い".is_kanji(), false);
    /// assert_eq!("あAア".is_kanji(), false);
    /// assert_eq!("🐸".is_kanji(), false);
    /// ```
    fn is_kanji(&self) -> bool;

    /// Test if any chars of `input` are [Kanji](https://en.wikipedia.org/wiki/Kanji) ([Japanese CJK ideographs](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs))
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// assert_eq!("勢い".contains_kanji(), true);
    /// assert_eq!("hello".contains_kanji(), false);
    /// ```
    fn contains_kanji(&self) -> bool;

    /// Test if `input` only includes [Kanji](https://en.wikipedia.org/wiki/Kanji), [Kana](https://en.wikipedia.org/wiki/Kana), zenkaku punctuation, japanese symbols and japanese numbers.
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// assert_eq!("泣き虫".is_japanese(), true);
    /// assert_eq!("あア".is_japanese(), true);
    /// assert_eq!("２月".is_japanese(), true); // Zenkaku numbers allowed
    /// assert_eq!("2月".is_japanese(), false); // Normal numbers not allowed
    /// assert_eq!("泣き虫。！〜＄".is_japanese(), true); // Zenkaku/JA punctuation
    /// assert_eq!("泣き虫.!~$".is_japanese(), false); // Latin punctuation fails
    /// assert_eq!("A".is_japanese(), false);
    /// ```
    fn is_japanese(&self) -> bool;

    #[cfg_attr(docsrs, doc(cfg(feature = "enable_regex")))]
    #[cfg(feature = "enable_regex")]
    /// Checks if all chars are in the japanese unicode ranges or match the provided regex
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// use regex::Regex;
    /// assert_eq!(
    ///     "≪偽括弧≫".is_japanese_with_whitelist(Some(&Regex::new(r"[≪≫]").unwrap())),
    ///     true
    /// );
    /// ```
    fn is_japanese_with_whitelist(self, allowed: Option<&Regex>) -> bool;

    /// Test if `input` contains a mix of [Romaji](https://en.wikipedia.org/wiki/Romaji) and [Kana](https://en.wikipedia.org/wiki/Kana),
    /// or [Kanji](https://en.wikipedia.org/wiki/Kanji)
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// assert_eq!("Aア".is_mixed(), true);
    /// assert_eq!("Aあ".is_mixed(), true);
    /// assert_eq!("Aあア".is_mixed(), true);
    /// assert_eq!("２あア".is_mixed(), false);
    /// assert_eq!("お腹A".is_mixed(), true);
    /// ```
    fn is_mixed(&self) -> bool;

    /// Test if `input` contains a mix of [Romaji](https://en.wikipedia.org/wiki/Romaji) and [Kana](https://en.wikipedia.org/wiki/Kana),
    /// or [Kanji](https://en.wikipedia.org/wiki/Kanji)
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// assert_eq!("Abあア".is_mixed_pass_kanji(true), true);
    /// assert_eq!("お腹A".is_mixed_pass_kanji(true), true);
    /// assert_eq!("お腹A".is_mixed_pass_kanji(false), false);
    /// assert_eq!("ab".is_mixed_pass_kanji(true), false);
    /// assert_eq!("あア".is_mixed_pass_kanji(true), false);
    /// ```
    fn is_mixed_pass_kanji(&self, pass_kanji: bool) -> bool;
}

impl IsJapaneseStr for &str {
    #[inline]
    fn is_hiragana(&self) -> bool {
        crate::is_hiragana::is_hiragana(self)
    }

    #[inline]
    fn is_katakana(&self) -> bool {
        crate::is_katakana::is_katakana(self)
    }

    #[inline]
    fn is_kana(&self) -> bool {
        crate::is_kana::is_kana(self)
    }

    #[inline]
    fn is_kanji(&self) -> bool {
        crate::is_kanji::is_kanji(self)
    }

    #[inline]
    fn contains_kanji(&self) -> bool {
        crate::is_kanji::contains_kanji(self)
    }

    #[inline]
    #[cfg(feature = "enable_regex")]
    fn is_japanese_with_whitelist(self, allowed: Option<&Regex>) -> bool {
        crate::is_japanese::is_japanese_with_whitelist(self, allowed)
    }

    #[inline]
    fn is_japanese(&self) -> bool {
        crate::is_japanese::is_japanese(self)
    }

    #[inline]
    fn is_romaji(&self) -> bool {
        crate::is_romaji::is_romaji(self)
    }

    #[cfg(feature = "enable_regex")]
    #[inline]
    fn is_romaji_with_whitelist(&self, allowed: Option<&Regex>) -> bool {
        crate::is_romaji::is_romaji_with_whitelist(self, allowed)
    }

    #[inline]
    fn is_mixed(&self) -> bool {
        crate::is_mixed::is_mixed(self)
    }

    #[inline]
    fn is_mixed_pass_kanji(&self, pass_kanji: bool) -> bool {
        crate::is_mixed::is_mixed_pass_kanji(self, pass_kanji)
    }
}

/// The `wana_kana::IsJapaneseChar` trait is implemented for the `char`type,
/// which allows easy checking of whether a `char` or `&str` is a Japanese character,
/// number or punctuation.
pub trait IsJapaneseChar {
    /// Tests a character. Returns true if the character is [Hiragana](https://en.wikipedia.org/wiki/Hiragana).
    fn is_hiragana(&self) -> bool;
    /// Tests a character. Returns true if the character is [Katakana](https://en.wikipedia.org/wiki/Katakana).
    fn is_katakana(&self) -> bool;
    /// Tests a character. Returns true if the character is [Hiragana](https://en.wikipedia.org/wiki/Hiragana) or [Katakana](https://en.wikipedia.org/wiki/Katakana).
    fn is_kana(&self) -> bool;
    /// Tests a character. Returns true if the character is a CJK ideograph (kanji).
    fn is_kanji(&self) -> bool;
    /// Checks if a char is in any of the japanese unicode ranges.
    fn is_japanese(&self) -> bool;
    /// Tests a character. Returns true if the character is considered a Zenkaku number(０-９)
    fn is_japanese_number(&self) -> bool;
    /// Tests a character. Returns true if the character is considered japanese punctuation.
    fn is_japanese_punctuation(&self) -> bool;
}

impl IsJapaneseChar for char {
    #[inline]
    fn is_hiragana(&self) -> bool {
        crate::utils::is_char_hiragana(*self)
    }

    #[inline]
    fn is_katakana(&self) -> bool {
        crate::utils::is_char_katakana(*self)
    }

    #[inline]
    fn is_kana(&self) -> bool {
        crate::utils::is_char_kana(*self)
    }

    #[inline]
    fn is_kanji(&self) -> bool {
        crate::utils::is_char_kanji(*self)
    }

    #[inline]
    fn is_japanese(&self) -> bool {
        crate::utils::is_char_japanese(*self)
    }

    #[inline]
    fn is_japanese_number(&self) -> bool {
        crate::utils::is_char_japanese_number(*self)
    }

    #[inline]
    fn is_japanese_punctuation(&self) -> bool {
        crate::utils::is_char_japanese_punctuation(*self)
    }
}

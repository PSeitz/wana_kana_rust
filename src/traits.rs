/// The `wana_kana::ConvertJapanese` trait is implemented for `&str`, which allows easy
/// conversion between kana and romaji with default options.
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
    fn to_hiragana(self) -> String;
    fn to_katakana(self) -> String;
    fn to_kana(self) -> String;
    fn to_romaji(self) -> String;
}

impl ConvertJapanese for &str {
    fn to_kana(self) -> String {
        crate::to_kana::to_kana(self)
    }
    fn to_hiragana(self) -> String {
        crate::to_hiragana::to_hiragana(self)
    }
    fn to_katakana(self) -> String {
        crate::to_katakana::to_katakana(self)
    }
    fn to_romaji(self) -> String {
        crate::to_romaji::to_romaji(self)
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
    /// // or `use wana_kana::traits::*`
    /// assert_eq!("げーむ".is_hiragana(), true);
    /// assert_eq!("A".is_hiragana(), false);
    /// assert_eq!("あア".is_hiragana(), false);
    /// ```
    fn is_hiragana(self) -> bool;
    /// Test if all chars of `input` are [Katakana](https://en.wikipedia.org/wiki/Katakana)
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// // or `use wana_kana::traits::*`
    /// assert_eq!("ゲーム".is_katakana(), true);
    /// assert_eq!("あ".is_katakana(), false);
    /// assert_eq!("A".is_katakana(), false);
    /// assert_eq!("あア".is_katakana(), false);
    /// ```
    fn is_katakana(self) -> bool;
    /// Test if all chars of `input` are [Kana](https://en.wikipedia.org/wiki/Kana) ([Katakana](https://en.wikipedia.org/wiki/Katakana) and/or [Hiragana](https://en.wikipedia.org/wiki/Hiragana))
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// // or `use wana_kana::traits::*`
    /// assert_eq!("あ".is_kana(), true);
    /// assert_eq!("ア".is_kana(), true);
    /// assert_eq!("あーア".is_kana(), true);
    /// assert_eq!("A".is_kana(), false);
    /// assert_eq!("あAア".is_kana(), false);
    /// ```
    fn is_kana(self) -> bool;
    /// Test if every char in `input` is [Romaji](https://en.wikipedia.org/wiki/Romaji) (allowing [Hepburn romanisation](https://en.wikipedia.org/wiki/Hepburn_romanization))
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// // or `use wana_kana::traits::*`
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
    fn is_romaji(self) -> bool;
    /// Test if all chars of `input` are [Kanji](https://en.wikipedia.org/wiki/Kanji) ([Japanese CJK ideographs](https://en.wikipedia.org/wiki/CJK_Unified_Ideographs))
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// // or `use wana_kana::traits::*`
    /// assert_eq!("刀".is_kanji(), true);
    /// assert_eq!("切腹".is_kanji(), true);
    /// assert_eq!("勢い".is_kanji(), false);
    /// assert_eq!("あAア".is_kanji(), false);
    /// assert_eq!("🐸".is_kanji(), false);
    /// ```
    fn is_kanji(self) -> bool;
    /// Test if `input` only includes [Kanji](https://en.wikipedia.org/wiki/Kanji), [Kana](https://en.wikipedia.org/wiki/Kana), zenkaku punctuation, japanese symbols and numbers.
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// // or `use wana_kana::traits::*`
    /// assert_eq!("泣き虫".is_japanese(), true);
    /// assert_eq!("あア".is_japanese(), true);
    /// assert_eq!("２月".is_japanese(), true); // Zenkaku numbers allowed
    /// assert_eq!("泣き虫。！〜＄".is_japanese(), true); // Zenkaku/JA punctuation
    /// assert_eq!("泣き虫.!~$".is_japanese(), false); // Latin punctuation fails
    /// assert_eq!("A".is_japanese(), false);
    /// ```
    fn is_japanese(self) -> bool;
    /// Test if `input` contains a mix of [Romaji](https://en.wikipedia.org/wiki/Romaji) and [Kana](https://en.wikipedia.org/wiki/Kana), defaults to pass through [Kanji](https://en.wikipedia.org/wiki/Kanji)
    ///
    /// # Examples
    /// ```
    /// use wana_kana::IsJapaneseStr;
    /// // or `use wana_kana::traits::*`
    /// assert_eq!("Aア".is_mixed(), true);
    /// assert_eq!("Aあ".is_mixed(), true);
    /// assert_eq!("Aあア".is_mixed(), true);
    /// assert_eq!("２あア".is_mixed(), false);
    /// assert_eq!("お腹A".is_mixed(), true);
    /// ```
    fn is_mixed(self) -> bool;
}

impl IsJapaneseStr for &str {
    fn is_hiragana(self) -> bool {
        crate::is_hiragana::is_hiragana(self)
    }
    fn is_katakana(self) -> bool {
        crate::is_katakana::is_katakana(self)
    }
    fn is_kana(self) -> bool {
        crate::is_kana::is_kana(self)
    }
    fn is_kanji(self) -> bool {
        crate::is_kanji::is_kanji(self)
    }
    fn is_japanese(self) -> bool {
        crate::is_japanese::is_japanese(self)
    }
    fn is_romaji(self) -> bool {
        crate::is_romaji::is_romaji(self)
    }
    fn is_mixed(self) -> bool {
        crate::is_mixed::is_mixed(self)
    }
}

/// The `wana_kana::IsJapaneseChar` trait is implemented for the `char`type,
/// which allows easy checking of whether a `char` is a Japanese character,
/// number or punctuation.
pub trait IsJapaneseChar {
    fn is_hiragana(self) -> bool;
    fn is_katakana(self) -> bool;
    fn is_kana(self) -> bool;
    fn is_kanji(self) -> bool;
    fn is_japanese(self) -> bool;
    fn is_japanese_number(self) -> bool;
    fn is_japanese_punctuation(self) -> bool;
}

impl IsJapaneseChar for char {
    fn is_hiragana(self) -> bool {
        crate::utils::is_char_hiragana(self)
    }
    fn is_katakana(self) -> bool {
        crate::utils::is_char_katakana(self)
    }
    fn is_kana(self) -> bool {
        crate::utils::is_char_kana(self)
    }
    fn is_kanji(self) -> bool {
        crate::utils::is_char_kanji(self)
    }
    fn is_japanese(self) -> bool {
        crate::utils::is_char_japanese(self)
    }
    fn is_japanese_number(self) -> bool {
        crate::utils::is_char_japanese_number(self)
    }
    fn is_japanese_punctuation(self) -> bool {
        crate::utils::is_char_japanese_punctuation(self)
    }
}

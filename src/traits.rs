/// The `wana_kana::Convert` trait is implemented for &str, which allows easy
/// conversion between kana and romaji with default options.
///
/// # Examples
///
/// ```
/// use wana_kana::Convert;
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
pub trait Convert {
    fn to_hiragana(self) -> String;
    fn to_katakana(self) -> String;
    fn to_kana(self) -> String;
    fn to_romaji(self) -> String;
}

impl Convert for &str {
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

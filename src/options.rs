#[derive(Debug, Default, Clone)]
/// Options to set.
pub struct Options {
    /// Set to true to use obsolete characters, such as ゐ and ゑ.
    pub use_obsolete_kana: bool,

    /// Set to true to pass romaji when using mixed syllabaries with to_katakana() or to_hiragana()
    /// toHiragana('only convert the katakana: ヒラガナ', { passRomaji: true })
    /// => "only convert the katakana: ひらがな"
    pub pass_romaji: bool,

    /// Set to true to convert katakana to uppercase using to_romaji()
    /// to_romaji('ひらがな カタカナ', { upcaseKatakana: true })
    pub upcase_katakana: bool,

    /// Set to true to handle conversion while it is being typed
    pub imemode: bool,
}

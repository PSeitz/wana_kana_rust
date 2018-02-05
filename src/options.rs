#[derive(Debug, Default, Clone)]
pub struct Options {
    pub use_obsolete_kana: bool,
    pub pass_romaji: bool,
    pub upcase_katakana: bool,
    pub imemode: bool,
}

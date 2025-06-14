#[macro_use]
pub(crate) mod hashmap_macro;
pub mod get_chunk;
pub mod halfwidth_katakana_to_hiragana;
pub mod hiragana_to_katakana;
pub mod is_char_consonant;
pub mod is_char_english_punctuation;
pub mod is_char_halfwidth_katakana;
pub mod is_char_hiragana;
pub mod is_char_in_range;
pub mod is_char_japanese;
pub mod is_char_japanese_number;
pub mod is_char_japanese_punctuation;
pub mod is_char_kana;
pub mod is_char_kanji;
pub mod is_char_katakana;
pub mod is_char_latin_number;
pub mod is_char_long_dash;
pub mod is_char_punctuation;
pub mod is_char_romaji;
pub mod is_char_slash_dot;
pub mod is_char_upper_case;
pub mod is_char_vowel;
pub mod katakana_to_hiragana;
pub mod romaji_to_hiragana;

pub use get_chunk::*;
pub use hiragana_to_katakana::*;
pub use is_char_consonant::*;
pub use is_char_english_punctuation::*;
pub use is_char_hiragana::*;
pub use is_char_in_range::*;
pub use is_char_japanese::*;
pub use is_char_japanese_number::*;
pub use is_char_japanese_punctuation::*;
pub use is_char_kana::*;
pub use is_char_kanji::*;
pub use is_char_katakana::*;
pub use is_char_latin_number::*;
pub use is_char_long_dash::*;
pub use is_char_punctuation::*;
pub use is_char_romaji::*;
pub use is_char_slash_dot::*;
pub use is_char_upper_case::*;
pub use is_char_vowel::*;
pub use katakana_to_hiragana::*;
pub use romaji_to_hiragana::*;

#[cfg(test)]
mod tests {
    use super::*;

    pub const JA_PUNC: [char; 18] = [
        '！', '？', '。', '：', '・', '、', '〜', 'ー', '「', '」', '『', '』', '［', '］', '（',
        '）', '｛', '｝',
    ];

    pub const EN_PUNC: [char; 18] = [
        '!', '?', '.', ':', '/', ',', '~', '-', '‘', '’', '“', '”', '[', ']', '(', ')', '{', '}',
    ];

    #[test]
    fn is_char_english_punctuation_test() {
        assert!(EN_PUNC.iter().cloned().all(is_char_english_punctuation));
        assert!(!JA_PUNC.iter().cloned().all(is_char_english_punctuation));
        assert!(is_char_english_punctuation(' '));
        assert!(!is_char_english_punctuation('a'));
        assert!(!is_char_english_punctuation('ふ'));
        assert!(!is_char_english_punctuation('字'));
    }

    #[test]
    fn is_char_punctuation_test() {
        assert!(EN_PUNC.iter().cloned().all(is_char_punctuation));
        assert!(JA_PUNC.iter().cloned().all(is_char_punctuation));
        assert!(is_char_punctuation(' '));
        assert!(is_char_punctuation('　'));
        assert!(!is_char_punctuation('a'));
        assert!(!is_char_punctuation('ふ'));
        assert!(!is_char_punctuation('字'));
    }

    #[test]
    fn is_char_japanese_punctuation_test() {
        assert!(!EN_PUNC.iter().cloned().all(is_char_japanese_punctuation));
        assert!(JA_PUNC.iter().cloned().all(is_char_japanese_punctuation));
        assert!(is_char_japanese_punctuation('　'));
        assert!(!is_char_japanese_punctuation('?'));
        assert!(!is_char_japanese_punctuation('a'));
        assert!(!is_char_japanese_punctuation('ふ'));
        assert!(!is_char_japanese_punctuation('字'));
    }
}

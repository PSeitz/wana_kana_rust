//! CharCode References
//! <http://www.rikai.com/library/kanjitables/kanji_codes.unicode.shtml>
//! <http://unicode-table.com>

pub const CJK_SYMBOLS_PUNCTUATION: [u32; 2] = [0x3000, 0x303F];
pub const KATAKANA_PUNCTUATION: [u32; 2] = [0x30FB, 0x30FC];
pub const HIRAGANA_CHARS: [u32; 2] = [0x3040, 0x309F];
pub const KATAKANA_CHARS: [u32; 2] = [0x30A0, 0x30FF];

pub const LOWERCASE_ZENKAKU_START: u32 = 0xff41;
pub const LOWERCASE_ZENKAKU_END: u32 = 0xff5a;
pub const UPPERCASE_ZENKAKU_START: u32 = 0xff21;
pub const UPPERCASE_ZENKAKU_END: u32 = 0xff3a;
pub const ZENKAKU_NUMBERS: [u32; 2] = [0xFF10, 0xFF19];
pub const ZENKAKU_PUNCTUATION_1: [u32; 2] = [0xFF01, 0xFF0F];
pub const ZENKAKU_PUNCTUATION_2: [u32; 2] = [0xFF1A, 0xFF1F];
pub const ZENKAKU_PUNCTUATION_3: [u32; 2] = [0xFF3B, 0xFF3F];
pub const ZENKAKU_PUNCTUATION_4: [u32; 2] = [0xFF5B, 0xFF60];
pub const ZENKAKU_UPPERCASE: [u32; 2] = [UPPERCASE_ZENKAKU_START, UPPERCASE_ZENKAKU_END];
pub const ZENKAKU_LOWERCASE: [u32; 2] = [LOWERCASE_ZENKAKU_START, LOWERCASE_ZENKAKU_END];
pub const ZENKAKU_SYMBOLS_CURRENCY: [u32; 2] = [0xFFE0, 0xFFEE];
pub const KANA_PUNCTUATION: [u32; 2] = [0xFF61, 0xFF65];
pub const HANKAKU_KATAKANA: [u32; 2] = [0xFF66, 0xFF9F];
pub const COMMON_CJK: [u32; 2] = [0x4E00, 0x9FFF];
pub const RARE_CJK: [u32; 2] = [0x3400, 0x4DBF];
pub const LATIN_NUMBERS: [u32; 2] = [0x0030, 0x0039];
pub const MODERN_ENGLISH: [u32; 2] = [0x0000, 0x007f];
pub const HEPBURN_MACRON_RANGES: [[u32; 2]; 5] = [
    [0x0100, 0x0101], // Ā ā
    [0x0112, 0x0113], // Ē ē
    [0x012a, 0x012b], // Ī ī
    [0x014c, 0x014d], // Ō ō
    [0x016a, 0x016b], // Ū ū
];
pub const SMART_QUOTE_RANGES: [[u32; 2]; 2] = [
    [0x2018, 0x2019], // ‘ ’
    [0x201C, 0x201D], // “ ”
];

// // pub const FULL_LATIN_RANGES = [
// //   [0x0001-0x007F],
// //   [0x0080-0x00FF],
// //   [0x0100-0x017F],
// //   [0x0180-0x024F],
// // ];

pub const JA_PUNCTUATION_RANGES: [[u32; 2]; 8] = [
    CJK_SYMBOLS_PUNCTUATION,
    KANA_PUNCTUATION,
    KATAKANA_PUNCTUATION,
    ZENKAKU_PUNCTUATION_1,
    ZENKAKU_PUNCTUATION_2,
    ZENKAKU_PUNCTUATION_3,
    ZENKAKU_PUNCTUATION_4,
    ZENKAKU_SYMBOLS_CURRENCY,
];

pub const KANA_RANGES: [[u32; 2]; 4] = [
    // const KANA_RANGES = [
    HIRAGANA_CHARS,
    KATAKANA_CHARS,
    KANA_PUNCTUATION,
    HANKAKU_KATAKANA,
];

lazy_static! {
    /// All Japanese unicode start and end ranges
    /// Includes full-width punctuation and number ranges.
    pub static ref JAPANESE_RANGES: Vec<[u32; 2]> = {
        let mut m = vec![ZENKAKU_UPPERCASE, ZENKAKU_LOWERCASE, ZENKAKU_NUMBERS, COMMON_CJK, RARE_CJK,];
        m.extend(KANA_RANGES);
        m.extend(JA_PUNCTUATION_RANGES);
        m
    };

    /// Basic Latin unicode regex, for determining Romaji + Hepburn romanisation
    /// Includes upper/lowercase long vowels like "ā, ī, ū, ē, ō"
    pub static ref ROMAJI_RANGES: Vec<[u32; 2]> = {
        let mut m = vec![MODERN_ENGLISH,];
        m.extend(HEPBURN_MACRON_RANGES);
        m
    };
    pub static ref EN_PUNCTUATION_RANGES: Vec<[u32; 2]> = {
        let mut m = vec![[0x20, 0x2F], [0x3A, 0x3F], [0x5B, 0x60], [0x7B, 0x7E],];
        m.extend(SMART_QUOTE_RANGES);
        m
    };

}

pub const UPPERCASE_START: u32 = 0x41;
pub const UPPERCASE_END: u32 = 0x5A;
pub const HIRAGANA_START: u32 = 0x3041;
pub const HIRAGANA_END: u32 = 0x3096;
pub const KATAKANA_START: u32 = 0x30A1;
pub const KATAKANA_END: u32 = 0x30FC;
pub const PROLONGED_SOUND_MARK: u32 = 0x30FC;
pub const KANA_SLASH_DOT: u32 = 0x30FB;

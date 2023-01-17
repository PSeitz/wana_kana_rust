use crate::options::Options;
use crate::to_kana_node_tree::{
    TO_KANA_NODE_TREE, TO_KANA_NODE_TREE_IMEMODE, TO_KANA_NODE_TREE_OBSOLETE,
};
use crate::utils::hiragana_to_katakana::*;

#[inline]
/// Convert [Romaji](https://en.wikipedia.org/wiki/Romaji) to [Kana](https://en.wikipedia.org/wiki/Kana), lowercase text will result in [Hiragana](https://en.wikipedia.org/wiki/Hiragana) and uppercase text will result in [Katakana](https://en.wikipedia.org/wiki/Katakana).
pub fn to_kana(input: &str) -> String {
    to_kana_with_opt(input, Options::default())
}

/// Convert [Romaji](https://en.wikipedia.org/wiki/Romaji) to [Kana](https://en.wikipedia.org/wiki/Kana), lowercase text will result in [Hiragana](https://en.wikipedia.org/wiki/Hiragana) and uppercase text will result in [Katakana](https://en.wikipedia.org/wiki/Katakana).
pub fn to_kana_with_opt(input: &str, options: Options) -> String {
    let chars = input.chars().collect::<Vec<_>>();
    // Final output array
    let mut ouput = String::with_capacity(input.len());
    let len = chars.len();
    // Position in the string that is being evaluated
    let mut curr_pos = 0;

    while curr_pos != len {
        let result = if options.use_obsolete_kana {
            TO_KANA_NODE_TREE_OBSOLETE.get(&chars[curr_pos..])
        } else if options.imemode {
            TO_KANA_NODE_TREE_IMEMODE.get(&chars[curr_pos..])
        } else {
            TO_KANA_NODE_TREE.get(&chars[curr_pos..])
        };

        // nothing found, pass through
        if result.1 == 0 {
            ouput.push(chars[curr_pos]);
            curr_pos += 1;
        } else {
            if chars[curr_pos..curr_pos + result.1]
                .iter()
                .all(|c| char::is_uppercase(*c))
            {
                ouput.push_str(&hiragana_to_katakana(result.0.unwrap()));
            } else {
                ouput.push_str(result.0.unwrap());
            }
            curr_pos += result.1;
        }
    }

    ouput
}

#[cfg(test)]
mod tests {
    fn with_obsolete_kana() -> Options {
        Options {
            use_obsolete_kana: true,
            ..Default::default()
        }
    }

    fn with_ime_mode() -> Options {
        Options {
            imemode: true,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub const JA_PUNC: [char; 18] = [
        '！', '？', '。', '：', '・', '、', '〜', 'ー', '「', '」', '『', '』', '［', '］', '（',
        '）', '｛', '｝',
    ];

    #[allow(dead_code)]
    pub const EN_PUNC: [char; 18] = [
        '!', '?', '.', ':', '/', ',', '~', '-', '‘', '’', '“', '”', '[', ']', '(', ')', '{', '}',
    ];

    use super::*;

    #[test]
    fn sane_defaults() {
        assert_eq!(to_kana(""), "");
    }
    #[test]
    fn uppercase_ist_katakana_defaults() {
        assert_eq!(to_kana("WANAKANA"), "ワナカナ");
    }

    #[test]
    fn lowercase_characters_are_transliterated_to_hiragana() {
        assert_eq!(to_kana("onaji"), "おなじ");
    }

    #[test]
    fn lowercase_with_double_consonants_and_double_vowels_are_transliterated_to_hiragana() {
        assert_eq!(to_kana("buttsuuji"), "ぶっつうじ");
    }

    #[test]
    fn uppercase_characters_are_transliterated_to_katakana() {
        assert_eq!(to_kana("ONAJI"), "オナジ");
    }

    #[test]
    fn uppercase_with_double_consonants_and_double_vowels_are_transliterated_to_katakana() {
        assert_eq!(to_kana("BUTTSUUJI"), "ブッツウジ");
    }

    #[test]
    fn wani_kani_わにかに_mixed_case_returns_hiragana_katakana_only_if_all_letters_of_mora_are_uppercased(
    ) {
        assert_eq!(to_kana("WaniKani"), "わにかに");
    }

    #[test]
    fn non_romaji_will_be_passed_through() {
        assert_eq!(
            to_kana("ワニカニ AiUeO 鰐蟹 12345 @#$%"),
            "ワニカニ アいウえオ 鰐蟹 12345 @#$%"
        );
    }

    #[test]
    fn it_handles_mixed_syllabaries() {
        assert_eq!(to_kana("座禅‘zazen’スタイル"), "座禅「ざぜん」スタイル");
    }

    #[test]
    fn will_convert_short_to_long_dashes() {
        assert_eq!(to_kana("batsuge-mu"), "ばつげーむ");
    }

    #[test]
    fn will_convert_punctuation_but_pass_through_spaces() {
        let en: String = EN_PUNC
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let ja = JA_PUNC
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(to_kana(&en), ja)
    }

    mod without_ime_mode {
        use super::*;
        #[test]
        fn solo_ns_are_transliterated_regardless_of_following_chars() {
            assert_eq!(to_kana("n"), "ん");
            assert_eq!(to_kana("shin"), "しん");
        }
        #[test]
        fn double_ns_are_transliterated_to_double_ん() {
            assert_eq!(to_kana("nn"), "んん");
        }
    }

    mod with_ime_mode {
        use super::*;
        #[test]
        fn solo_ns_are_not_transliterated_unless_chars_follow() {
            assert_eq!(to_kana_with_opt("n", with_ime_mode()), "n");
            assert_eq!(to_kana_with_opt("shin", with_ime_mode()), "しn");
            assert_eq!(to_kana_with_opt("shinyou", with_ime_mode()), "しにょう");
            assert_eq!(to_kana_with_opt("shin'you", with_ime_mode()), "しんよう");
            assert_eq!(to_kana_with_opt("shin you", with_ime_mode()), "しんよう");
        }
        #[test]
        fn double_ns_are_transliterated_to_singleん() {
            assert_eq!(to_kana_with_opt("nn", with_ime_mode()), "ん");
        }
    }

    mod use_obsolete_kana {
        use super::*;
        #[test]
        fn use_obsolete_kana_is_false_by_default() {
            assert_eq!(to_kana("wi"), "うぃ");
            assert_eq!(to_kana("WI"), "ウィ");
        }
        #[test]
        fn wi_ゐ_when_use_obsolete_kana_is_true() {
            assert_eq!(to_kana_with_opt("wi", with_obsolete_kana()), "ゐ");
        }
        #[test]
        fn we_ゑ_when_use_obsolete_kana_is_true() {
            assert_eq!(to_kana_with_opt("we", with_obsolete_kana()), "ゑ");
        }
        #[test]
        fn wi_ヰ_when_use_obsolete_kana_is_true() {
            assert_eq!(to_kana_with_opt("WI", with_obsolete_kana()), "ヰ");
        }
        #[test]
        fn we_ヱ_when_use_obsolete_kana_is_true() {
            assert_eq!(to_kana_with_opt("WE", with_obsolete_kana()), "ヱ");
        }
    }
}

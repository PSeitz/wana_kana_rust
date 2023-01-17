use crate::options::Options;
pub(crate) use crate::to_romaji_node_tree::TO_ROMAJI_NODE_TREE;
use crate::utils::is_char_katakana::is_char_katakana;
use crate::utils::katakana_to_hiragana::*;

/// Convert kana to romaji
pub fn to_romaji(input: &str) -> String {
    to_romaji_with_opt(input, Options::default())
}

/// Convert kana to romaji
pub fn to_romaji_with_opt(orig: &str, options: Options) -> String {
    let kana = katakana_to_hiragana_with_opt(orig, true);
    let orig_chars = orig.chars().collect::<Vec<_>>();
    let chars = kana.chars().collect::<Vec<_>>();
    let mut ouput = String::with_capacity(orig.len());
    let len = chars.len();
    // Position in the string that is being evaluated
    let mut curr_pos = 0;

    while curr_pos != len {
        let result = TO_ROMAJI_NODE_TREE.get(&chars[curr_pos..]);
        // nothing found, pass through
        if result.1 == 0 {
            ouput.push(chars[curr_pos]);
            curr_pos += 1;
        } else {
            let convert_romaji_to_uppercase = {
                if orig_chars[curr_pos..curr_pos + result.1]
                    .iter()
                    .all(|c| is_char_katakana(*c))
                {
                    options.upcase_katakana
                } else {
                    false
                }
            };

            if convert_romaji_to_uppercase {
                ouput.push_str(&result.0.to_uppercase());
            } else {
                ouput.push_str(result.0);
            }
            curr_pos += result.1;
        }
    }

    ouput
}

#[cfg(test)]
mod tests {
    use super::{Options, *};
    #[test]
    fn sane_defaults() {
        assert_eq!(to_romaji(""), "");
    }

    #[test]
    fn convert_katakana_to_romaji() {
        assert_eq!(
            to_romaji("ワニカニ　ガ　スゴイ　ダ"),
            "wanikani ga sugoi da"
        );
    }

    #[test]
    fn convert_hiragana_to_romaji() {
        assert_eq!(
            to_romaji("わにかに　が　すごい　だ"),
            "wanikani ga sugoi da"
        );
    }

    #[test]
    fn convert_mixed_kana_to_romaji() {
        assert_eq!(
            to_romaji("ワニカニ　が　すごい　だ"),
            "wanikani ga sugoi da"
        );
    }

    //#[test]
    // fn will_convert_punctuation_and_full_width_spaces() {
    // assert_eq!(to_romaji(JA_PUNC.join("")), EN_PUNC.join(""));
    //}

    #[test]
    fn use_the_upcase_katakana_flag_to_preserve_casing_works_for_katakana() {
        assert_eq!(
            to_romaji_with_opt(
                "ワニカニ",
                Options {
                    upcase_katakana: true,
                    ..Default::default()
                }
            ),
            "WANIKANI"
        );
    }

    #[test]
    fn use_the_upcase_katakana_flag_to_preserve_casing_works_for_mixed_kana() {
        assert_eq!(
            to_romaji_with_opt(
                "ワニカニ　が　すごい　だ",
                Options {
                    upcase_katakana: true,
                    ..Default::default()
                }
            ),
            "WANIKANI ga sugoi da"
        );
    }

    #[test]
    fn converts_long_dash_in_hiragana_to_hyphen() {
        assert_eq!(to_romaji("ばつげーむ"), "batsuge-mu");
    }

    #[test]
    fn doesnt_confuse_一one_kanji_for_long_dash_ー() {
        assert_eq!(to_romaji("一抹げーむ"), "一抹ge-mu");
    }

    #[test]
    fn converts_long_dash_ー_chōonpu_in_katakana_to_long_vowel() {
        assert_eq!(to_romaji("スーパー"), "suupaa");
    }

    #[test]
    fn doesnt_convert_オー_to_ou_which_occurs_with_hiragana() {
        assert_eq!(to_romaji("缶コーヒー"), "缶koohii");
    }

    #[test]
    fn spaces_must_be_manually_entered() {
        assert_ne!(to_romaji("わにかにがすごいだ"), "wanikani ga sugoi da");
    }

    mod double_ns_and_double_consonants {
        use super::*;

        #[test]
        fn double_and_single_n() {
            assert_eq!(to_romaji("きんにくまん"), "kinnikuman");
        }
        #[test]
        fn n_extravaganza() {
            assert_eq!(to_romaji("んんにんにんにゃんやん"), "nnninninnyan'yan");
        }
        #[test]
        fn double_consonants() {
            assert_eq!(
                to_romaji("かっぱ　たった　しゅっしゅ ちゃっちゃ　やっつ"),
                "kappa tatta shusshu chatcha yattsu"
            );
        }
        #[test]
        fn xx_double_consonants() {
            assert_eq!(to_romaji("かっぱ"), "kappa");
        }
    }

    mod small_kana {
        use super::*;

        #[test]
        fn small_tsu_doesnt_transliterate() {
            assert_eq!(to_romaji("っ"), "");
        }
        #[test]
        fn small_kata_ke_doesnt_transliterate() {
            assert_eq!(to_romaji("ヶ"), "ヶ");
        }
        #[test]
        fn small_kata_ka_doesnt_transliterate() {
            assert_eq!(to_romaji("ヵ"), "ヵ");
        }
        #[test]
        fn small_ya() {
            assert_eq!(to_romaji("ゃ"), "ya");
        }
        #[test]
        fn small_yu() {
            assert_eq!(to_romaji("ゅ"), "yu");
        }
        #[test]
        fn small_yo() {
            assert_eq!(to_romaji("ょ"), "yo");
        }
        #[test]
        fn small_a() {
            assert_eq!(to_romaji("ぁ"), "a");
        }
        #[test]
        fn small_i() {
            assert_eq!(to_romaji("ぃ"), "i");
        }
        #[test]
        fn small_u() {
            assert_eq!(to_romaji("ぅ"), "u");
        }
        #[test]
        fn small_e() {
            assert_eq!(to_romaji("ぇ"), "e");
        }
        #[test]
        fn small_o() {
            assert_eq!(to_romaji("ぉ"), "o");
        }
    }

    mod apostrophes_in_ambiguous_consonant_vowel_combos {
        use super::*;

        #[test]
        fn おんよみ() {
            assert_eq!(to_romaji("おんよみ"), "on'yomi");
        }
        #[test]
        fn んよ_んあ_んゆ() {
            assert_eq!(to_romaji("んよ んあ んゆ"), "n'yo n'a n'yu");
        }
        #[test]
        fn シンヨ() {
            assert_eq!(to_romaji("シンヨ"), "shin'yo");
        }
    }
}

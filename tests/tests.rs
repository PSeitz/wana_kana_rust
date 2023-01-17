use wana_kana::Options;

mod conversion_tables;
use conversion_tables::*;

#[cfg(test)]
mod tests {
    use wana_kana::ConvertJapanese;

    use super::*;

    mod character_conversion {
        use super::*;

        mod test_every_conversion_table_char {

            use super::*;
            #[test]
            fn to_kana_test() {
                for &[romaji, hiragana, katakana] in ROMA_TO_HIRA_KATA.iter() {
                    let lower = romaji.to_kana();
                    let upper = (&romaji.to_uppercase()).to_kana();
                    assert_eq!(lower, hiragana);
                    assert_eq!(upper, katakana);
                }
            }

            #[test]
            fn to_hiragana_test() {
                for &[romaji, hiragana, _katakana] in ROMA_TO_HIRA_KATA.iter() {
                    let lower = romaji.to_hiragana();
                    let upper = (&romaji.to_uppercase()).to_hiragana();
                    assert_eq!(lower, hiragana);
                    assert_eq!(upper, hiragana);
                }
            }

            #[test]
            fn hiragana_input_to_romaji() {
                for &[hiragana, _, romaji] in HIRA_KATA_TO_ROMA.iter() {
                    if !hiragana.is_empty() {
                        assert_eq!(hiragana.to_romaji(), romaji);
                    }
                }
            }

            #[test]
            fn katakana_input_to_romaji() {
                for &[_, katakana, romaji] in HIRA_KATA_TO_ROMA.iter() {
                    if !katakana.is_empty() {
                        assert_eq!(katakana.to_romaji(), romaji);
                    }
                }
            }
        }

        mod converting_kana_to_kana {
            use super::*;
            #[test]
            fn k_to_h() {
                assert_eq!("バケル".to_hiragana(), "ばける");
            }
            #[test]
            fn h_to_k() {
                assert_eq!(("ばける".to_katakana()), "バケル");
            }

            #[test]
            fn it_survives_only_katakana_to_katakana() {
                assert_eq!(("スタイル".to_katakana()), "スタイル");
            }
            #[test]
            fn it_survives_only_hiragana_to_hiragana() {
                assert_eq!(("すたーいる".to_hiragana()), "すたーいる");
            }
            #[test]
            fn mixed_kana_converts_every_char_k_to_h() {
                assert_eq!(("アメリカじん".to_katakana()), "アメリカジン");
            }
            #[test]
            fn mixed_kana_converts_every_char_h_to_k() {
                assert_eq!(("アメリカじん".to_hiragana()), "あめりかじん");
            }
        }
        mod long_vowels {
            use super::*;
            #[test]
            fn converts_long_vowels_correctly_from_k_to_h() {
                assert_eq!(("バツゴー".to_hiragana()), "ばつごう");
            }
            #[test]
            fn preserves_long_dash_from_h_to_k() {
                assert_eq!("ばつゲーム".to_katakana(), "バツゲーム");
            }
            #[test]
            fn preserves_long_dash_from_h_to_h() {
                assert_eq!("ばつげーむ".to_hiragana(), "ばつげーむ");
            }
            #[test]
            fn preserves_long_dash_from_k_to_k() {
                assert_eq!("バツゲーム".to_katakana(), "バツゲーム");
            }
            #[test]
            fn preserves_long_dash_from_mixed_to_k_1() {
                assert_eq!("バツゲーム".to_katakana(), "バツゲーム");
            }
            #[test]
            fn preserves_long_dash_from_mixed_to_k_2() {
                assert_eq!("テスーと".to_katakana(), "テスート");
            }
            #[test]
            fn preserves_long_dash_from_mixed_to_h_1() {
                assert_eq!("てすート".to_hiragana(), "てすーと");
            }
            #[test]
            fn preserves_long_dash_from_mixed_to_h_2() {
                assert_eq!("てすー戸".to_hiragana(), "てすー戸");
            }
            #[test]
            fn preserves_long_dash_from_mixed_to_h_3() {
                assert_eq!("手巣ート".to_hiragana(), "手巣ーと");
            }
            #[test]
            fn preserves_long_dash_from_mixed_to_h_4() {
                assert_eq!("tesート".to_hiragana(), "てsーと");
            }
            #[test]
            fn preserves_long_dash_from_mixed_to_h_5() {
                assert_eq!("ートtesu".to_hiragana(), "ーとてす");
            }
        }

        mod mixed_syllabaries {
            use super::*;
            #[test]
            fn it_passes_non_katakana_through_when_pass_romaji_is_true_k_to_h() {
                assert_eq!(
                    "座禅‘zazen’スタイル".to_hiragana_with_opt(Options {
                        pass_romaji: true,
                        ..Default::default()
                    }),
                    "座禅‘zazen’すたいる"
                );
            }
            #[test]
            fn it_passes_non_hiragana_through_when_pass_romaji_is_true_h_to_k() {
                assert_eq!(
                    "座禅‘zazen’すたいる".to_katakana_with_opt(Options {
                        pass_romaji: true,
                        ..Default::default()
                    }),
                    "座禅‘zazen’スタイル"
                );
            }
            #[test]
            fn it_converts_non_katakana_when_pass_romaji_is_false_k_to_h() {
                assert_eq!(
                    "座禅‘zazen’スタイル".to_hiragana(),
                    "座禅「ざぜん」すたいる"
                );
            }
            #[test]
            fn it_converts_non_hiragana_when_pass_romaji_is_false_h_to_k() {
                assert_eq!(
                    "座禅‘zazen’すたいる".to_katakana(),
                    "座禅「ザゼン」スタイル"
                );
            }
        }
    }

    mod case_sensitivity {

        use super::*;
        #[test]
        fn c_ase_do_esn_t_mat_ter_for_to_hiragana() {
            assert_eq!("aiueo".to_hiragana(), "AIUEO".to_hiragana());
        }
        #[test]
        fn c_ase_do_esn_t_mat_ter_for_to_katakana() {
            assert_eq!("aiueo".to_katakana(), "AIUEO".to_katakana());
        }
        #[test]
        fn case_does_matter_for_to_kana() {
            assert_ne!("aiueo".to_kana(), "AIUEO".to_kana());
        }
    }

    mod n_edge_cases {

        use super::*;
        #[test]
        fn solo_n() {
            assert_eq!("n".to_kana(), "ん");
        }
        #[test]
        fn double_n() {
            assert_eq!("onn".to_kana(), "おんん");
        }
        #[test]
        fn n_followed_by_n_syllable() {
            assert_eq!("onna".to_kana(), "おんな");
        }
        #[test]
        fn triple_n() {
            assert_eq!("nnn".to_kana(), "んんん");
        }
        #[test]
        fn triple_n_followed_by_n_syllable() {
            assert_eq!("onnna".to_kana(), "おんんな");
        }
        #[test]
        fn quadruple_n() {
            assert_eq!("nnnn".to_kana(), "んんんん");
        }
        #[test]
        fn nya_to_にゃ() {
            assert_eq!("nyan".to_kana(), "にゃん");
        }
        #[test]
        fn nnya_to_んにゃ() {
            assert_eq!("nnyann".to_kana(), "んにゃんん");
        }
        #[test]
        fn nnnya_to_んにゃ() {
            assert_eq!("nnnyannn".to_kana(), "んんにゃんんん");
        }
        #[test]
        fn nya_to_んや() {
            assert_eq!("n'ya".to_kana(), "んや");
        }
        #[test]
        fn kinya_to_きんや() {
            assert_eq!("kin'ya".to_kana(), "きんや");
        }
        #[test]
        fn shinya_to_しんや() {
            assert_eq!("shin'ya".to_kana(), "しんや");
        }
        #[test]
        fn kinyou_to_きにょう() {
            assert_eq!("kinyou".to_kana(), "きにょう");
        }
        #[test]
        fn kinyou_to_きんよう() {
            assert_eq!("kin'you".to_kana(), "きんよう");
        }
        #[test]
        fn kinyu_to_きんゆ() {
            assert_eq!("kin'yu".to_kana(), "きんゆ");
        }
        #[test]
        fn properly_add_space_after_nspace() {
            assert_eq!("ichiban warui".to_kana(), "いちばん わるい");
        }
    }

    mod bogus_4_character_sequences {

        use super::*;
        #[test]
        fn non_bogus_sequences_work() {
            assert_eq!("chya".to_kana(), "ちゃ");
        }
        #[test]
        fn bogus_sequences_do_not_work() {
            assert_eq!("chyx".to_kana(), "chyx");
        }
        #[test]
        fn bogus_sequences_do_not_work_2() {
            assert_eq!("shyp".to_kana(), "shyp");
        }
        #[test]
        fn bogus_sequences_do_not_work_3() {
            assert_eq!("ltsb".to_kana(), "ltsb");
        }
    }
}

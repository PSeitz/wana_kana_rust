use crate::is_mixed::*;
use crate::is_romaji::*;
use crate::options::Options;
use crate::utils::hiragana_to_katakana::*;
use crate::utils::romaji_to_hiragana::*;

/// Convert input to [Katakana](https://en.wikipedia.org/wiki/Katakana)
pub fn to_katakana(input: &str) -> String {
    to_katakana_with_opt(input, Options::default())
}
/// Convert input to [Katakana](https://en.wikipedia.org/wiki/Katakana)
pub fn to_katakana_with_opt(input: &str, options: Options) -> String {
    let config = options;
    if config.pass_romaji {
        hiragana_to_katakana(input)
    } else if is_romaji(input) || is_mixed(input) {
        let romaji = romaji_to_hiragana(input, config);
        hiragana_to_katakana(&romaji)
    } else {
        hiragana_to_katakana(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_obsolete_kana() -> Options {
        Options {
            use_obsolete_kana: true,
            ..Default::default()
        }
    }

    #[test]
    fn sane_defaults() {
        assert_eq!(to_katakana(""), "");
    }

    #[test]
    fn quick_brown_fox_romaji_to_katakana() {
        // https://en.wikipedia.org/wiki/Iroha
        // Even the colorful fragrant flowers'
        assert_eq!(
            to_katakana_with_opt("IROHANIHOHETO", with_obsolete_kana()),
            "イロハニホヘト"
        );
        // die sooner or later.'
        assert_eq!(
            to_katakana_with_opt("CHIRINURUWO", with_obsolete_kana()),
            "チリヌルヲ"
        );
        // Us who live in this world'
        assert_eq!(
            to_katakana_with_opt("WAKAYOTARESO", with_obsolete_kana()),
            "ワカヨタレソ"
        );
        // cannot live forever, either.'
        assert_eq!(
            to_katakana_with_opt("TSUNENARAMU", with_obsolete_kana()),
            "ツネナラム"
        );
        // This transient mountain with shifts and changes,'
        assert_eq!(
            to_katakana_with_opt("UWINOOKUYAMA", with_obsolete_kana()),
            "ウヰノオクヤマ"
        );
        // today we are going to overcome, and reach the world of enlightenment.'
        assert_eq!(
            to_katakana_with_opt("KEFUKOETE", with_obsolete_kana()),
            "ケフコエテ"
        );
        // We are not going to have meaningless dreams'
        assert_eq!(
            to_katakana_with_opt("ASAKIYUMEMISHI", with_obsolete_kana()),
            "アサキユメミシ"
        );
        // nor become intoxicated with the fake world anymore.'
        assert_eq!(
            to_katakana_with_opt("WEHIMOSESU", with_obsolete_kana()),
            "ヱヒモセス"
        );
        // *not in iroha*
        assert_eq!(to_katakana("NLTU"), "ンッ");
    }

    mod use_obsolete_kana {
        use super::*;

        #[test]
        fn use_obsolete_kana_is_false_by_default() {
            assert_eq!(to_katakana("wi"), "ウィ");
        }
        #[test]
        fn wi_ゐ_when_use_obsolete_kana_is_true() {
            assert_eq!(to_katakana_with_opt("wi", with_obsolete_kana()), "ヰ");
        }
        #[test]
        fn we_ゑ_when_use_obsolete_kana_is_true() {
            assert_eq!(to_katakana_with_opt("we", with_obsolete_kana()), "ヱ");
        }
    }
    mod pass_romaji {
        use super::*;

        #[test]
        fn false_by_default() {
            assert_eq!(to_katakana("only かな"), "オンly カナ");
            assert_eq!(
                to_katakana_with_opt(
                    "only かな",
                    Options {
                        pass_romaji: true,
                        ..Default::default()
                    }
                ),
                "only カナ"
            );
        }
    }
}

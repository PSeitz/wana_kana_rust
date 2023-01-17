#[cfg(feature = "tokenize")]
use wana_kana::trim_okurigana::*;

#[cfg(feature = "tokenize")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_okurigana_with_no_input() {
        assert_eq!(trim_okurigana(""), "");
    }

    #[test]
    fn passes_default_parameter_tests() {
        assert_eq!(trim_okurigana("踏み込む"), "踏み込");
        assert_eq!(trim_okurigana("使い方"), "使い方");
        assert_eq!(trim_okurigana("申し申し"), "申し申");
        assert_eq!(trim_okurigana("お腹"), "お腹");
        assert_eq!(trim_okurigana("お祝い"), "お祝");
    }

    #[test]
    fn strips_leading_when_passed_optional_config() {
        assert_eq!(trim_okurigana_with_opt("踏み込む", true, None), "踏み込む");
        assert_eq!(trim_okurigana_with_opt("お腹", true, None), "腹");
        assert_eq!(trim_okurigana_with_opt("お祝い", true, None), "祝い");
    }

    #[test]
    fn strips_reading_by_matching_original_word_when_passed_match_kanji() {
        assert_eq!(
            trim_okurigana_with_opt("おはら", false, Some("お腹")),
            "おはら"
        );
        assert_eq!(
            trim_okurigana_with_opt("おはら", true, Some("お腹")),
            "はら"
        );
        assert_eq!(
            trim_okurigana_with_opt("ふみこむ", false, Some("踏み込む")),
            "ふみこ"
        );
        assert_eq!(
            trim_okurigana_with_opt("おみまい", true, Some("お祝い")),
            "みまい"
        );
    }
}

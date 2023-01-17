#[cfg(test)]
mod tests {
    use wana_kana::IsJapaneseStr;

    #[test]
    fn sane_defaults() {
        assert_eq!("".is_japanese(), false);
    }
    #[test]
    fn 泣き虫_is_japanese() {
        assert_eq!("泣き虫".is_japanese(), true);
    }

    #[cfg(feature = "enable_regex")]
    #[test]
    fn is_japanese_with_whitelist_test() {
        use regex::Regex;
        assert_eq!(
            ("≪偽括弧≫".is_japanese_with_whitelist(Some(&Regex::new(r"[≪≫]").unwrap()))),
            true
        );
    }
    #[test]
    fn あア_is_japanese() {
        assert_eq!(("あア".is_japanese()), true);
    }
    #[test]
    fn a泣き虫_is_not_japanese() {
        assert_eq!(("A泣き虫".is_japanese()), false);
    }
    #[test]
    fn a_is_not_japanese() {
        assert_eq!(("A".is_japanese()), false);
    }
    #[test]
    fn ja_space_is_japanese() {
        assert_eq!(("　".is_japanese()), true);
    }
    #[test]
    fn en_space_is_not_japanese() {
        assert_eq!((" ".is_japanese()), false);
    }
    #[test]
    fn kanji_with_zenkaku_punctuation_is_japanese() {
        assert_eq!(
            "泣き虫。＃！〜〈〉《》〔〕［］【】（）｛｝〝〟".is_japanese(),
            true
        );
    }
    #[test]
    fn kanji_with_romaji_punctuation_is_not_japanese() {
        assert_eq!(("泣き虫.!~".is_japanese()), false);
    }
    #[test]
    fn zenkaku_numbers_are_considered_neutral() {
        assert_eq!(("０１２３４５６７８９".is_japanese()), true);
    }
    #[test]
    fn latin_numbers_are_not_japanese() {
        assert_eq!(("0123456789".is_japanese()), false);
    }
    #[test]
    fn zenkaku_latin_letters_are_considered_neutral() {
        assert_eq!(("ＭｅＴｏｏ".is_japanese()), true);
    }
    #[test]
    fn mixed_with_numbers_is_japanese() {
        assert_eq!(("２０１１年".is_japanese()), true);
    }
    #[test]
    fn hankaku_katakana_is_allowed() {
        assert_eq!(("ﾊﾝｶｸｶﾀｶﾅ".is_japanese()), true);
    }
    #[test]
    fn randomly_sliced_nhk_news_text_is_japanese() {
        assert_eq!(
            "＃ＭｅＴｏｏ、これを前に「ＫＵＲＯＳＨＩＯ」は、都内で報道陣を前に水中探査ロボットの最終点検の様子を公開しました。イルカのような形をした探査ロボットは、全長３メートル、重さは３５０キロあります。《はじめに》冒頭、安倍総理大臣は、ことしが明治元年から１５０年にあたることに触れ「明治という新しい時代が育てたあまたの人材が、技術優位の欧米諸国が迫る『国難』とも呼ぶべき危機の中で、わが国が急速に近代化を遂げる原動力となった。今また、日本は少子高齢化という『国難』とも呼ぶべき危機に直面している。もう１度、あらゆる日本人にチャンスを創ることで、少子高齢化も克服できる」と呼びかけました。《働き方改革》続いて安倍総理大臣は、具体的な政策課題の最初に「働き方改革」を取り上げ、「戦後の労働基準法制定以来、７０年ぶりの大改革だ。誰もが生きがいを感じて、その能力を思う存分発揮すれば少子高齢化も克服できる」と述べました。そして、同一労働同一賃金の実現や、時間外労働の上限規制の導入、それに労働時間でなく成果で評価するとして労働時間の規制から外す「高度プロフェッショナル制度」の創設などに取り組む考えを強調しました。".is_japanese()
        , true)
    }
}

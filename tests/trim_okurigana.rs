#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::trim_okurigana::*;

speculate! {
    it "trim_okurigana() with no input" {
        assert_eq!(trim_okurigana(""), "");
    }

    it "passes default parameter tests" {
        assert_eq!(trim_okurigana("踏み込む"), "踏み込");
        assert_eq!(trim_okurigana("使い方"), "使い方");
        assert_eq!(trim_okurigana("申し申し"), "申し申");
        assert_eq!(trim_okurigana("お腹"), "お腹");
        assert_eq!(trim_okurigana("お祝い"), "お祝");
    }

    it "strips leading when passed optional config" {
        assert_eq!(trim_okurigana_with_opt("踏み込む", true, None ), "踏み込む");
        assert_eq!(trim_okurigana_with_opt("お腹", true, None ), "腹");
        assert_eq!(trim_okurigana_with_opt("お祝い", true, None ), "祝い");
    }

    it "strips reading by matching original word when passed matchKanji" {
        assert_eq!(trim_okurigana_with_opt("おはら", false, Some("お腹")), "おはら");
        assert_eq!(trim_okurigana_with_opt("ふみこむ", false, Some("踏み込む")), "ふみこ");
        assert_eq!(trim_okurigana_with_opt("おみまい", true, Some("お祝い")), "みまい");
        assert_eq!(trim_okurigana_with_opt("おはら", true, Some("お腹")), "はら");
    }

}

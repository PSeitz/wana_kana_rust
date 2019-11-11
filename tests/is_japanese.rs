#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::is_japanese::*;


speculate!{
    it "sane defaults" {
        assert_eq!(is_japanese(""), false);
    }
    it "泣き虫 is japanese" {
        assert_eq!(is_japanese("泣き虫"), true);
    }
    it "あア is japanese" {
        assert_eq!(is_japanese("あア"), true);
    }
    it "A泣き虫 is not japanese" {
        assert_eq!(is_japanese("A泣き虫"), false);
    }
    it "A is not japanese" {
        assert_eq!(is_japanese("A"), false);
    }
    it "ja space is japanese" {
        assert_eq!(is_japanese("　"), true);
    }
    it "en space is not japanese" {
        assert_eq!(is_japanese(" "), false);
    }
    it "泣き虫。！〜 (w. zenkaku punctuation) is japanese" {
        assert_eq!(is_japanese("泣き虫。＃！〜〈〉《》〔〕［］【】（）｛｝〝〟"), true);
    }
    it "泣き虫.!~ (w. romaji punctuation) is not japanese" {
        assert_eq!(is_japanese("泣き虫.!~"), false);
    }
    it "zenkaku numbers are considered neutral" {
        assert_eq!(is_japanese("０１２３４５６７８９"), true);
    }
    it "latin numbers are not japanese" {
        assert_eq!(is_japanese("0123456789"), false);
    }
    it "zenkaku latin letters are considered neutral" {
        assert_eq!(is_japanese("ＭｅＴｏｏ"), true);
    }
    it "mixed with numbers is japanese" {
        assert_eq!(is_japanese("２０１１年"), true);
    }
    it "hankaku katakana is allowed" {
        assert_eq!(is_japanese("ﾊﾝｶｸｶﾀｶﾅ"), true);
    }
    it "randomly sliced nhk news text is japanese" {
        assert_eq!(
          is_japanese(
            "＃ＭｅＴｏｏ、これを前に「ＫＵＲＯＳＨＩＯ」は、都内で報道陣を前に水中探査ロボットの最終点検の様子を公開しました。イルカのような形をした探査ロボットは、全長３メートル、重さは３５０キロあります。《はじめに》冒頭、安倍総理大臣は、ことしが明治元年から１５０年にあたることに触れ「明治という新しい時代が育てたあまたの人材が、技術優位の欧米諸国が迫る『国難』とも呼ぶべき危機の中で、わが国が急速に近代化を遂げる原動力となった。今また、日本は少子高齢化という『国難』とも呼ぶべき危機に直面している。もう１度、あらゆる日本人にチャンスを創ることで、少子高齢化も克服できる」と呼びかけました。《働き方改革》続いて安倍総理大臣は、具体的な政策課題の最初に「働き方改革」を取り上げ、「戦後の労働基準法制定以来、７０年ぶりの大改革だ。誰もが生きがいを感じて、その能力を思う存分発揮すれば少子高齢化も克服できる」と述べました。そして、同一労働同一賃金の実現や、時間外労働の上限規制の導入、それに労働時間でなく成果で評価するとして労働時間の規制から外す「高度プロフェッショナル制度」の創設などに取り組む考えを強調しました。"
          )
        , true)
    }

}
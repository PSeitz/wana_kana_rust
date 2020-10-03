#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::to_hiragana::*;
use wana_kana::Options;

fn with_obsolete_kana() -> Options {
    Options {
        use_obsolete_kana: true,
        ..Default::default()
    }
}

speculate! {

    it "sane defaults" {
        assert_eq!(to_hiragana(""), "");
    }

  it "Quick Brown Fox - Romaji to Hiragana" {
    // https://en.wikipedia.org/wiki/Iroha
    // Even the colorful fragrant flowers'
    assert_eq!(to_hiragana_with_opt("IROHANIHOHETO", with_obsolete_kana()), "いろはにほへと");
    // die sooner or later.'
    assert_eq!(to_hiragana_with_opt("CHIRINURUWO", with_obsolete_kana()), "ちりぬるを");
    // Us who live in this world'
    assert_eq!(to_hiragana_with_opt("WAKAYOTARESO", with_obsolete_kana()), "わかよたれそ");
    // cannot live forever, either.'
    assert_eq!(to_hiragana_with_opt("TSUNENARAMU", with_obsolete_kana()), "つねならむ");
    // This transient mountain with shifts and changes,'
    assert_eq!(to_hiragana_with_opt("UWINOOKUYAMA", with_obsolete_kana()), "うゐのおくやま");
    // today we are going to overcome, and reach the world of enlightenment.'
    assert_eq!(to_hiragana_with_opt("KEFUKOETE", with_obsolete_kana()), "けふこえて");
    // We are not going to have meaningless dreams'
    assert_eq!(to_hiragana_with_opt("ASAKIYUMEMISHI", with_obsolete_kana()), "あさきゆめみし");
    // nor become intoxicated with the fake world anymore.'
    assert_eq!(to_hiragana_with_opt("WEHIMOSESU", with_obsolete_kana()), "ゑひもせす");
    // *not in iroha*
    assert_eq!(to_hiragana("NLTU"), "んっ");
  }

  describe "useObsoleteKana" {
    it "useObsoleteKana is false by default" {
      assert_eq!(to_hiragana("wi"), "うぃ");
    }
    it "wi = ゐ (when useObsoleteKana is true)" {
      assert_eq!(to_hiragana_with_opt("wi", with_obsolete_kana()), "ゐ");
    }
    it "we = ゑ (when useObsoleteKana is true)" {
      assert_eq!(to_hiragana_with_opt("we", with_obsolete_kana()), "ゑ");
    }
    it "wi = うぃ when useObsoleteKana is false" {
      assert_eq!(to_hiragana_with_opt("wi", Options{ use_obsolete_kana: false, .. Default::default() }), "うぃ");
    }
  }

  describe "passRomaji" {
    it "false by default" {
      assert_eq!(to_hiragana("only カナ"), "おんly かな");
      assert_eq!(to_hiragana_with_opt("only カナ", Options{ pass_romaji: true, .. Default::default() }), "only かな");
    }
  }

  describe "katakana choōnpu" {
    it "converts to hiragana long vowels" {
      assert_eq!(to_hiragana("スーパー"), "すうぱあ");
      assert_eq!(to_hiragana("バンゴー"), "ばんごう");
    }
  }

  it "mixed input" {
    assert_eq!(to_hiragana("#22 ２２漢字、toukyou, オオサカ"), "#22 ２２漢字、とうきょう、 おおさか");
  }

}

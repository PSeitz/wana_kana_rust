#![feature(test)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::to_katakana::*;
use wana_kana::Options;

fn with_obsolete_kana() -> Options {
    Options {
        use_obsolete_kana: true,
        ..Default::default()
    }
}

speculate! {

    it "sane defaults" {
        assert_eq!(to_katakana(""), "");
    }

  it "Quick Brown Fox - Romaji to Katakana" {
    // https://en.wikipedia.org/wiki/Iroha
    // Even the colorful fragrant flowers'
    assert_eq!(to_katakana_with_opt("IROHANIHOHETO", with_obsolete_kana()), "イロハニホヘト");
    // die sooner or later.'
    assert_eq!(to_katakana_with_opt("CHIRINURUWO", with_obsolete_kana()), "チリヌルヲ");
    // Us who live in this world'
    assert_eq!(to_katakana_with_opt("WAKAYOTARESO", with_obsolete_kana()), "ワカヨタレソ");
    // cannot live forever, either.'
    assert_eq!(to_katakana_with_opt("TSUNENARAMU", with_obsolete_kana()), "ツネナラム");
    // This transient mountain with shifts and changes,'
    assert_eq!(to_katakana_with_opt("UWINOOKUYAMA", with_obsolete_kana()), "ウヰノオクヤマ");
    // today we are going to overcome, and reach the world of enlightenment.'
    assert_eq!(to_katakana_with_opt("KEFUKOETE", with_obsolete_kana()), "ケフコエテ");
    // We are not going to have meaningless dreams'
    assert_eq!(to_katakana_with_opt("ASAKIYUMEMISHI", with_obsolete_kana()), "アサキユメミシ");
    // nor become intoxicated with the fake world anymore.'
    assert_eq!(to_katakana_with_opt("WEHIMOSESU", with_obsolete_kana()), "ヱヒモセス");
    // *not in iroha*
    assert_eq!(to_katakana("NLTU"), "ンッ");
  }

  describe "useObsoleteKana" {
    it "useObsoleteKana is false by default" {
      assert_eq!(to_katakana("wi"), "ウィ");
    }
    it "wi = ゐ (when useObsoleteKana is true)" {
      assert_eq!(to_katakana_with_opt("wi", with_obsolete_kana()), "ヰ");
    }
    it "we = ゑ (when useObsoleteKana is true)" {
      assert_eq!(to_katakana_with_opt("we", with_obsolete_kana()), "ヱ");
    }
  }

  describe "passRomaji" {
    it "false by default" {
      assert_eq!(to_katakana("only かな"), "オンly カナ");
      assert_eq!(to_katakana_with_opt("only かな", Options{ pass_romaji: true, .. Default::default() }), "only カナ");
    }
  }


}

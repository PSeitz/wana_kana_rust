#![feature(test)]
#![feature(plugin)]
#![feature(non_ascii_idents)]

#[cfg(test)]
extern crate test;
extern crate wana_kana;

#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::is_hiragana::*;
use wana_kana::is_japanese::*;
use wana_kana::is_kana::*;
use wana_kana::is_kanji::*;
use wana_kana::is_katakana::*;
use wana_kana::is_mixed::*;
use wana_kana::is_romaji::*;
use wana_kana::strip_okurigana::*;
use wana_kana::to_hiragana::*;
use wana_kana::to_kana;
use wana_kana::to_kana::*;
use wana_kana::to_katakana::*;
use wana_kana::to_romaji;
use wana_kana::to_romaji::*;
use wana_kana::tokenize::*;
use wana_kana::Options;

speculate!{
describe "methods_should_return_valid_defaults_when_given_no_input" {
    it "is_kana() with no input" {
        assert_eq!(is_kana(""), false);
    }
    it "is_kanji() with no input" {
        assert_eq!(is_kanji(""), false);
    }
    it "is_japanese() with no input" {
        assert_eq!(is_japanese(""), false);
    }
    it "is_katakana() with no input" {
        assert_eq!(is_katakana(""), false);
    }
    it "is_hiragana() with no input" {
        assert_eq!(is_hiragana(""), false);
    }
    it "is_romaji() with no input" {
        assert_eq!(is_romaji(""), false);
    }
    it "is_mixed() with no input" {
        assert_eq!(is_mixed(""), false);
    }
    it "to_kana() with no input" {
        assert_eq!(to_kana(""), "");
    }
    it "to_katakana() with no input" {
        assert_eq!(to_katakana(""), "");
    }
    it "to_hiragana() with no input" {
        assert_eq!(to_hiragana(""), "");
    }
    it "to_romaji() with no input" {
        assert_eq!(to_romaji(""), "");
    }
    it "strip_okurigana() with no input" {
        assert_eq!(strip_okurigana_all("", false), "");
    }
    it "tokenize() with no input" {
        let empty: Vec<String> = vec![];
        assert_eq!(tokenize(""), empty);
    }
}

describe "character_type_detection" {
    describe "is_hiragana" {
        it "あ is hiragana" {
            assert_eq!(is_hiragana("あ"), true);
        }
        it "ああ is hiragana" {
            assert_eq!(is_hiragana("ああ"), true);
        }
        it "ア is not hiragana" {
            assert_eq!(is_hiragana("ア"), false);
        }
        it "A is not hiragana" {
            assert_eq!(is_hiragana("A"), false);
        }
        it "あア is not hiragana" {
            assert_eq!(is_hiragana("あア"), false);
        }
        it "ignores long dash in hiragana" {
            assert_eq!(is_hiragana("げーむ"), true);
        }
    }

    describe "is_katakana" {
        it "アア is katakana" {
            assert_eq!(is_katakana("アア"), true);
        }
        it "ア is katakana" {
            assert_eq!(is_katakana("ア"), true);
        }
        it "あ is not katakana" {
            assert_eq!(is_katakana("あ"), false);
        }
        it "A is not katakana" {
            assert_eq!(is_katakana("A"), false);
        }
        it "あア is not katakana" {
            assert_eq!(is_katakana("あア"), false);
        }
        it "ignores long dash in katakana" {
            assert_eq!(is_katakana("ゲーム"), true);
        }
    }

    describe "is_kana" {
        it "あ is kana" {
            assert_eq!(is_kana("あ"), true);
        }
        it "ア is kana" {
            assert_eq!(is_kana("ア"), true);
        }
        it "あア is kana" {
            assert_eq!(is_kana("あア"), true);
        }
        it "A is not kana" {
            assert_eq!(is_kana("A"), false);
        }
        it "あAア is not kana" {
            assert_eq!(is_kana("あAア"), false);
        }
        it "ignores long dash in mixed kana" {
            assert_eq!(is_kana("アーあ"), true);
        }
    }

    describe "is_kanji" {
        it "切腹 is kanji" {
            assert_eq!(is_kanji("切腹"), true);
        }
        it "刀 is kanji" {
            assert_eq!(is_kanji("刀"), true);
        }
        it "🐸 is not kanji" {
            assert_eq!(is_kanji("🐸"), false);
        }
        it "あ is not kanji" {
            assert_eq!(is_kanji("あ"), false);
        }
        it "ア is not kanji" {
            assert_eq!(is_kanji("ア"), false);
        }
        it "あア is not kanji" {
            assert_eq!(is_kanji("あア"), false);
        }
        it "A is not kanji" {
            assert_eq!(is_kanji("A"), false);
        }
        it "あAア is not kanji" {
            assert_eq!(is_kanji("あAア"), false);
        }
        it "１２隻 is not kanji" {
            assert_eq!(is_kanji("１２隻"), false);
        }
        it "12隻 is not kanji" {
            assert_eq!(is_kanji("12隻"), false);
        }
        it "隻。 is not kanji" {
            assert_eq!(is_kanji("隻。"), false);
        }
    }

    describe "is_japanese" {
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

    describe "is_romaji" {
        it "A is romaji" {
            assert_eq!(is_romaji("A"), true);
        }
        it "xYz is romaji" {
            assert_eq!(is_romaji("xYz"), true);
        }
        it "Tōkyō and Ōsaka is romaji" {
            assert_eq!(is_romaji("Tōkyō and Ōsaka"), true);
        }
        it "あアA is not romaji" {
            assert_eq!(is_romaji("あアA"), false);
        }
        it "お願い is not romaji" {
            assert_eq!(is_romaji("お願い"), false);
        }
        it "熟成 is not romaji" {
            assert_eq!(is_romaji("熟成"), false);
        }
        it "passes latin punctuation" {
            assert_eq!(is_romaji("a*b&c-d"), true);
        }
        it "passes latin numbers" {
            assert_eq!(is_romaji("0123456789"), true);
        }
        it "fails zenkaku punctuation" {
            assert_eq!(is_romaji("a！b&cーd"), false);
        }
        it "fails zenkaku latin" {
            assert_eq!(is_romaji("ｈｅｌｌｏ"), false);
        }
    }

    describe "is_mixed" {
        it "Aア is mixed" {
            assert_eq!(is_mixed("Aア"), true);
        }
        it "Aあ is mixed" {
            assert_eq!(is_mixed("Aあ"), true);
        }
        it "Aあア is mixed" {
            assert_eq!(is_mixed("Aあア"), true);
        }
        it "２あア is not mixed" {
            assert_eq!(is_mixed("２あア"), false);
        }
        it "お腹A is mixed" {
            assert_eq!(is_mixed("お腹A"), true);
        }
        it "お腹A is not mixed when { passKanji: false }" {
            assert_eq!(is_mixed_pass_kanji("お腹A", false), false);
        }
        it "お腹 is not mixed" {
            assert_eq!(is_mixed("お腹"), false);
        }
        it "腹 is not mixed" {
            assert_eq!(is_mixed("腹"), false);
        }
        it "A is not mixed" {
            assert_eq!(is_mixed("A"), false);
        }
        it "あ is not mixed" {
            assert_eq!(is_mixed("あ"), false);
        }
        it "ア is not mixed" {
            assert_eq!(is_mixed("ア"), false);
        }
    }
}

describe "character_conversion" {
    describe "double_consonants_transliterate_to_glottal_stops_small_tsu" {
        it "double B" {
            assert_eq!(to_kana("babba"), "ばっば");
        }
        it "double C" {
            assert_eq!(to_kana("cacca"), "かっか");
        }
        it "double Ch" {
            assert_eq!(to_kana("chaccha"), "ちゃっちゃ");
        }
        it "double D" {
            assert_eq!(to_kana("dadda"), "だっだ");
        }
        it "double F" {
            assert_eq!(to_kana("fuffu"), "ふっふ");
        }
        it "double G" {
            assert_eq!(to_kana("gagga"), "がっが");
        }
        it "double H" {
            assert_eq!(to_kana("hahha"), "はっは");
        }
        it "double J" {
            assert_eq!(to_kana("jajja"), "じゃっじゃ");
        }
        it "double K" {
            assert_eq!(to_kana("kakka"), "かっか");
        }
        it "double L" {
            assert_eq!(to_kana("lalla"), "らっら");
        }
        it "double M" {
            assert_eq!(to_kana("mamma"), "まっま");
        }
        it "double N" {
            assert_eq!(to_kana("nanna"), "なんな");
        }
        it "double P" {
            assert_eq!(to_kana("pappa"), "ぱっぱ");
        }
        it "double Q" {
            assert_eq!(to_kana("qaqqa"), "くぁっくぁ");
        }
        it "double R" {
            assert_eq!(to_kana("rarra"), "らっら");
        }
        it "double S" {
            assert_eq!(to_kana("sassa"), "さっさ");
        }
        it "double Sh" {
            assert_eq!(to_kana("shassha"), "しゃっしゃ");
        }
        it "double T" {
            assert_eq!(to_kana("tatta"), "たった");
        }
        it "double Ts" {
            assert_eq!(to_kana("tsuttsu"), "つっつ");
        }
        it "double V" {
            assert_eq!(to_kana("vavva"), "ゔぁっゔぁ");
        }
        it "double W" {
            assert_eq!(to_kana("wawwa"), "わっわ");
        }
        it "double X" {
            assert_eq!(to_kana("yayya"), "やっや");
        }
        it "double Z" {
            assert_eq!(to_kana("zazza"), "ざっざ");
        }
    }

    describe "to_kana" {
        it "Lowercase characters are transliterated to hiragana." {
            assert_eq!(to_kana("onaji"), "おなじ");
        }
        it "Lowercase with double consonants and double vowels are transliterated to hiragana." {
            assert_eq!(to_kana("buttsuuji"), "ぶっつうじ");
        }
        it "Uppercase characters are transliterated to katakana." {
            assert_eq!(to_kana("ONAJI"), "オナジ");
        }
        it "Uppercase with double consonants and double vowels are transliterated to katakana." {
            assert_eq!(to_kana("BUTTSUUJI"), "ブッツウジ");
        }
        it "WaniKani -> ワにカに - Mixed case uses the first character for each syllable." {
            assert_eq!(to_kana("WaniKani"), "ワにカに");
        }
        it "Non-romaji will be passed through." {
            assert_eq!(to_kana("ワニカニ AiUeO 鰐蟹 12345 @#$%"), "ワニカニ アいウえオ 鰐蟹 12345 @#$%");
        }
        it "It handles mixed syllabaries" {
            assert_eq!(to_kana("座禅‘zazen’スタイル"), "座禅「ざぜん」スタイル");
        }
        it "Will convert short to long dashes" {
            assert_eq!(to_kana("batsuge-mu"), "ばつげーむ");
        }
        // it "Will convert punctuation but pass through spaces" {
            //     assert_eq!(to_kana(EN_PUNC.join(" ")), JA_PUNC.join(" "));
            // }
    }

    describe "converting_kana_to_kana" {
        it "k -> h" {
            assert_eq!(to_hiragana("バケル"), "ばける");
        }
        it "h -> k" {
            assert_eq!(to_katakana("ばける"), "バケル");
        }

        it "It survives only katakana to_katakana" {

            assert_eq!(to_katakana("スタイル"), "スタイル");

        }
        it "It survives only hiragana to_hiragana" {
            assert_eq!(to_hiragana("すたーいる"), "すたーいる");
        }
        it "Mixed kana converts every char k -> h" {
            assert_eq!(to_katakana("アメリカじん"), "アメリカジン");
        }
        it "Mixed kana converts every char h -> k" {
            assert_eq!(to_hiragana("アメリカじん"), "あめりかじん");
        }
    }
    describe "long_vowels" {
        it "Converts long vowels correctly from k -> h" {
          assert_eq!(to_hiragana("バツゴー"), "ばつごう");
        }
        it "Preserves long dash from h -> k" {
          assert_eq!(to_katakana("ばつゲーム"), "バツゲーム");
        }
        it "Preserves long dash from h -> h" {
          assert_eq!(to_hiragana("ばつげーむ"), "ばつげーむ");
        }
        it "Preserves long dash from k -> k" {
          assert_eq!(to_katakana("バツゲーム"), "バツゲーム");
        }
        it "Preserves long dash from mixed -> k 1" {
          assert_eq!(to_katakana("バツゲーム"), "バツゲーム");
        }
        it "Preserves long dash from mixed -> k 2" {
          assert_eq!(to_katakana("テスーと"), "テスート");
        }
        it "Preserves long dash from mixed -> h 1" {
          assert_eq!(to_hiragana("てすート"), "てすーと");
        }
        it "Preserves long dash from mixed -> h 2" {
          assert_eq!(to_hiragana("てすー戸"), "てすー戸");
        }
        it "Preserves long dash from mixed -> h 3" {
          assert_eq!(to_hiragana("手巣ート"), "手巣ーと");
        }
        it "Preserves long dash from mixed -> h 4" {
          assert_eq!(to_hiragana("tesート"), "てsーと");
        }
        it "Preserves long dash from mixed -> h 5" {
          assert_eq!(to_hiragana("ートtesu"), "ーとてす");
        }
    }

    describe "mixed_syllabaries" {
        it "It passes non-katakana through when pass_romaji is true k -> h" {
          assert_eq!(to_hiragana_with_opt("座禅‘zazen’スタイル", Options{ pass_romaji: true, .. Default::default() }), "座禅‘zazen’すたいる");
        }
        it "It passes non-hiragana through when pass_romaji is true h -> k" {
          assert_eq!(to_katakana_with_opt("座禅‘zazen’すたいる", Options{ pass_romaji: true, .. Default::default() }), "座禅‘zazen’スタイル");
        }
        it "It converts non-katakana when pass_romaji is false k -> h" {
          assert_eq!(to_hiragana("座禅‘zazen’スタイル"), "座禅「ざぜん」すたいる");
        }
        it "It converts non-hiragana when pass_romaji is false h -> k" {
          assert_eq!(to_katakana("座禅‘zazen’すたいる"), "座禅「ザゼン」スタイル");
        }
    }
}

describe "case_sensitivity" {
    it "cAse DoEsnT MatTER for to_hiragana()" {
        assert_eq!(to_hiragana("aiueo"), to_hiragana("AIUEO"));
    }
    it "cAse DoEsnT MatTER for to_katakana()" {
        assert_eq!(to_katakana("aiueo"), to_katakana("AIUEO"));
    }
    it "Case DOES matter for to_kana()" {
        assert_ne!(to_kana("aiueo"), to_kana("AIUEO"));
    }
}

describe "n_edge_cases" {
    it "Solo N" {
        assert_eq!(to_kana("n"), "ん");
    }
    it "double N" {
        assert_eq!(to_kana("onn"), "おん");
    }
    it "N followed by N* syllable" {
        assert_eq!(to_kana("onna"), "おんな");
    }
    it "Triple N" {
        assert_eq!(to_kana("nnn"), "んん");
    }
    it "Triple N followed by N* syllable" {
        assert_eq!(to_kana("onnna"), "おんな");
    }
    it "Quadruple N" {
        assert_eq!(to_kana("nnnn"), "んん");
    }
    it "nya -> にゃ" {
        assert_eq!(to_kana("nyan"), "にゃん");
    }
    it "nnya -> んにゃ" {
        assert_eq!(to_kana("nnyann"), "んにゃん");
    }
    it "nnnya -> んにゃ" {
        assert_eq!(to_kana("nnnyannn"), "んにゃんん");
    }
    it "n'ya -> んや" {
        assert_eq!(to_kana("n'ya"), "んや");
    }
    it "kin'ya -> きんや" {
        assert_eq!(to_kana("kin'ya"), "きんや");
    }
    it "shin'ya -> しんや" {
        assert_eq!(to_kana("shin'ya"), "しんや");
    }
    it "kinyou -> きにょう" {
        assert_eq!(to_kana("kinyou"), "きにょう");
    }
    it "kin'you -> きんよう" {
        assert_eq!(to_kana("kin'you"), "きんよう");
    }
    it "kin'yu -> きんゆ" {
        assert_eq!(to_kana("kin'yu"), "きんゆ");
    }
    // it "Properly add space after "n[space]"" {
        //     assert_eq!(to_kana("ichiban warui"), "いちばん わるい");
        // }
}

describe "bogus_4_character_sequences" {
    it "Non bogus sequences work" {
        assert_eq!(to_kana("chya"), "ちゃ");
    }
    it "Bogus sequences do not work" {
        assert_eq!(to_kana("chyx"), "chyx");
    }
    it "Bogus sequences do not work 2" {
        assert_eq!(to_kana("shyp"), "shyp");
    }
    it "Bogus sequences do not work 3" {
        assert_eq!(to_kana("ltsb"), "ltsb");
    }
}

describe "kana_to_romaji" {
    describe "to_romaji" {
        it "Convert katakana to romaji"{
            assert_eq!(to_romaji("ワニカニ　ガ　スゴイ　ダ"),"wanikani ga sugoi da");
        }
        it "Convert hiragana to romaji"{
            assert_eq!(to_romaji("わにかに　が　すごい　だ"),"wanikani ga sugoi da");
        }
        it "Convert mixed kana to romaji"{
            assert_eq!(to_romaji("ワニカニ　が　すごい　だ"),"wanikani ga sugoi da");
        }
        // it "Will convert punctuation and full-width spaces"{
        //     assert_eq!(to_romaji(JA_PUNC.join("")).toBe(EN_PUNC.join(""))));
        // }
        it "Use the upcase_katakana flag to preserve casing. Works for katakana."{
            assert_eq!(to_romaji_with_opt("ワニカニ", Options{ upcase_katakana: true, ..Default::default() }),"WANIKANI");
        }
        it "Use the upcase_katakana flag to preserve casing. Works for mixed kana."{
            assert_eq!(to_romaji_with_opt("ワニカニ　が　すごい　だ", Options{ upcase_katakana: true, ..Default::default() }),"WANIKANI ga sugoi da");
        }
        it "Doesn't mangle the long dash 'ー' or slashdot '・'"{
            assert_eq!(to_romaji("罰ゲーム・ばつげーむ"),"罰ge-mu/batsuge-mu");
        }
        it "Spaces must be manually entered"{
            assert_ne!(to_romaji("わにかにがすごいだ"),"wanikani ga sugoi da");
        }
    }

    describe "quick_brown_fox_hiragana_to_romaji" {
        it "Quick_Brown_Fox_Hiragana_to_Romaji" {
            assert_eq!(to_romaji("いろはにほへと"),"irohanihoheto");
            assert_eq!(to_romaji("ちりぬるを"),"chirinuruwo");
            assert_eq!(to_romaji("わかよたれそ"),"wakayotareso");
            assert_eq!(to_romaji("つねならむ"),"tsunenaramu");
            assert_eq!(to_romaji("うゐのおくやま"),"uwinookuyama");
            assert_eq!(to_romaji("けふこえて"),"kefukoete");
            assert_eq!(to_romaji("あさきゆめみし"),"asakiyumemishi");
            assert_eq!(to_romaji("ゑひもせすん"),"wehimosesun");
        }
    }

    describe "double_ns_and_double_consonants" {
        it "Double and single n" {
            assert_eq!(to_romaji("きんにくまん"), "kinnikuman");
        }
        it "N extravaganza" {
            assert_eq!(to_romaji("んんにんにんにゃんやん"), "nnninninnyan'yan");
        }
        it "Double consonants" {
            assert_eq!(to_romaji("かっぱ　たった　しゅっしゅ ちゃっちゃ　やっつ"), "kappa tatta shusshu chaccha yattsu");
        }
    }

    describe "small_kana" {
        it "Small tsu doesn't transliterate"{
            assert_eq!(to_romaji("っ"),"");
        }
        it "Small ya"{
            assert_eq!(to_romaji("ゃ"),"ya");
        }
        it "Small yu"{
            assert_eq!(to_romaji("ゅ"),"yu");
        }
        it "Small yo"{
            assert_eq!(to_romaji("ょ"),"yo");
        }
        it "Small a"{
            assert_eq!(to_romaji("ぁ"),"a");
        }
        it "Small i"{
            assert_eq!(to_romaji("ぃ"),"i");
        }
        it "Small u"{
            assert_eq!(to_romaji("ぅ"),"u");
        }
        it "Small e"{
            assert_eq!(to_romaji("ぇ"),"e");
        }
        it "Small o"{
            assert_eq!(to_romaji("ぉ"),"o");
        }
        it "Small ke (ka)"{
            assert_eq!(to_romaji("ヶ"),"ka");
        }
        it "Small ka"{
            assert_eq!(to_romaji("ヵ"),"ka");
        }
        it "Small wa"{
            assert_eq!(to_romaji("ゎ"),"wa");
        }
    }

    describe "apostrophes_in_vague_consonant_vowel_combos" {
        it "おんよみ" {
            assert_eq!(to_romaji("おんよみ"),"on'yomi");
        }
        it "んよ んあ んゆ" {
            assert_eq!(to_romaji("んよ んあ んゆ"),"n'yo n'a n'yu");
        }
    }
}

describe "strip_okurigana" {
    it "passes default parameter tests" {
        assert_eq!(strip_okurigana("ふふフフ"),"ふふフフ");
        assert_eq!(strip_okurigana("ふaふbフcフ"),"ふaふbフcフ");
        assert_eq!(strip_okurigana("お腹"),"お腹");
        assert_eq!(strip_okurigana("踏み込む"),"踏み込");
        assert_eq!(strip_okurigana("お祝い"),"お祝");
        assert_eq!(strip_okurigana("粘り"),"粘");
        assert_eq!(strip_okurigana("〜い海軍い、。"),"〜い海軍、。");
    }
    it "strips all kana when passed optional config" {
        assert_eq!(strip_okurigana_all("お腹", true ),"腹");
        assert_eq!(strip_okurigana_all("踏み込む", true ),"踏込");
        assert_eq!(strip_okurigana_all("お祝い", true ),"祝");
        assert_eq!(strip_okurigana_all("お踏み込む", true ),"踏込");
        assert_eq!(strip_okurigana_all("〜い海軍い、。", true ),"〜海軍、。");
    }
}

describe "tokenize" {
    it "passes default parameter tests" {
        assert_eq!(tokenize("ふふ"), vec!["ふふ"]);
        assert_eq!(tokenize("フフ"), vec!["フフ"]);
        assert_eq!(tokenize("ふふフフ"), vec!["ふふ", "フフ"]);
        assert_eq!(tokenize("阮咸"), vec!["阮咸"]);
        assert_eq!(tokenize("感じ"), vec!["感", "じ"]);
        assert_eq!(tokenize("私は悲しい"), vec!["私", "は", "悲", "しい"]);
        assert_eq!(tokenize("what the...私は「悲しい」。"), vec![
          "what the...",
          "私",
          "は",
          "「",
          "悲",
          "しい",
          "」。",
        ]);
    }
}

/// Simulate real typing by calling the function on every character in sequence
///
/// @param  {String} input
///
/// @param  {Object} options
///

///
fn test_typing(input: &str, options: Options) -> String {
    let mut pos = 1;
    let mut text = input.to_string();
    let len = text.chars().count();
    while pos <= len {
        let mut buffer: String = text.chars().take(pos).collect();
        let rest: String = text.chars().skip(pos).collect();
        buffer = to_kana_with_opt(&buffer, options.clone());
        text = buffer + &rest;
        pos += 1;
    }
    return text;
}

describe "imemode" {


    it "Without IME mode, solo n's are transliterated."{
        assert_eq!(to_kana("n"),"ん");
    }
    it "Without IME mode, double n's are transliterated."{
        assert_eq!(to_kana("nn"),"ん");
    }
    it "With IME mode, solo n's are not transliterated."{
        assert_eq!(test_typing("n", Options{ imemode: true, ..Default::default() }), "n");
    }
    it "With IME mode, double n's are transliterated."{
        assert_eq!(test_typing("nn", Options{ imemode: true, ..Default::default() }), "ん");
    }
    it "With IME mode, n + space are transliterated."{
        assert_eq!(test_typing("n ", Options{ imemode: true, ..Default::default() }), "ん");
    }
    it "With IME mode, n + ' are transliterated."{
        assert_eq!(test_typing("n'", Options{ imemode: true, ..Default::default() }), "ん");
    }
    it "With IME mode, ni."{
        assert_eq!(test_typing("ni", Options{ imemode: true, ..Default::default() }), "に");
    }
    it "kan"{
        assert_eq!(test_typing("kan", Options{ imemode: true, ..Default::default() }), "かn");
    }
    it "kanp"{
        assert_eq!(test_typing("kanp", Options{ imemode: true, ..Default::default() }), "かんp");
    }
    it "kanpai!"{
        assert_eq!(test_typing("kanpai", Options{ imemode: true, ..Default::default() }), "かんぱい");
    }
    it "nihongo"{
        assert_eq!(test_typing("nihongo", Options{ imemode: true, ..Default::default() }), "にほんご");
    }
    it "y doesn't count as a consonant for IME"{
        assert_eq!(test_typing("ny", Options{ imemode: true, ..Default::default() }), "ny");
    }
    it "nya works as expected"{
        assert_eq!(test_typing("nya", Options{ imemode: true, ..Default::default() }), "にゃ");
    }
    it "With IME mode, solo N's are not transliterated - katakana."{
        assert_eq!(test_typing("N", Options{ imemode: true, ..Default::default() }), "N");
    }
    it "With IME mode, double N's are transliterated - katakana."{
        assert_eq!(test_typing("NN", Options{ imemode: true, ..Default::default() }), "ン");
    }
    it "With IME mode, NI - katakana."{
        assert_eq!(test_typing("NI", Options{ imemode: true, ..Default::default() }), "ニ");
    }
    it "With IME mode - KAN - katakana"{
        assert_eq!(test_typing("KAN", Options{ imemode: true, ..Default::default() }), "カN");
    }
    it "With IME mode - NIHONGO - katakana"{
        assert_eq!(test_typing("NIHONGO", Options{ imemode: true, ..Default::default() }), "ニホンゴ");
    }
}

describe "optionso" {
    describe "use_obsolete_kana" {
        describe "to_kana" {
            it "use_obsolete_kana is false by default" {
                assert_eq!(to_kana("wi"), "うぃ");
            }
            it "wi = ゐ (when use_obsolete_kana is true)" {
                assert_eq!(to_kana_with_opt("wi", Options{ use_obsolete_kana: true, .. Default::default() }), "ゐ");
            }
            it "we = ゑ (when use_obsolete_kana is true)" {
                assert_eq!(to_kana_with_opt("we", Options{ use_obsolete_kana: true, .. Default::default() }), "ゑ");
            }
            it "WI = ヰ (when use_obsolete_kana is true)" {
                assert_eq!(to_kana_with_opt("WI", Options{ use_obsolete_kana: true, .. Default::default() }), "ヰ");
            }
            it "WE = ヱ (when use_obsolete_kana is true)" {
                assert_eq!(to_kana_with_opt("WE", Options{ use_obsolete_kana: true, .. Default::default() }), "ヱ");
            }
        }

        describe "to_hiragana" {
            it "use_obsolete_kana is false by default" {
                assert_eq!(to_hiragana("wi"), "うぃ");
            }
            it "wi = ゐ (when use_obsolete_kana is true)" {
                assert_eq!(to_hiragana_with_opt("wi", Options{ use_obsolete_kana: true, .. Default::default() }), "ゐ");
            }
            it "we = ゑ (when use_obsolete_kana is true)" {
                assert_eq!(to_hiragana_with_opt("we", Options{ use_obsolete_kana: true, .. Default::default() }), "ゑ");
            }
            it "wi = うぃ when use_obsolete_kana is false" {
                assert_eq!(to_hiragana_with_opt("wi", Options{ use_obsolete_kana: false, .. Default::default() }), "うぃ");
            }
        }

        describe "to_kata_kana" {
            it "wi = ウィ when use_obsolete_kana is false" {
                assert_eq!(to_katakana_with_opt("WI", Options{ use_obsolete_kana: false, .. Default::default() }), "ウィ");
            }
            it "WI = ヰ (when use_obsolete_kana is true)" {
                assert_eq!(to_katakana_with_opt("wi", Options{ use_obsolete_kana: true, .. Default::default() }), "ヰ");
            }
            it "WE = ヱ (when use_obsolete_kana is true)" {
                assert_eq!(to_katakana_with_opt("we", Options{ use_obsolete_kana: true, .. Default::default() }), "ヱ");
            }
        }
    }
}

}

#[bench]
fn bench_kana_1(b: &mut test::Bencher) {
    b.iter(|| to_kana::to_kana("aiueosashisusesonaninunenokakikukeko"))
}
#[bench]
fn bench_kana_2(b: &mut test::Bencher) {
    b.iter(|| to_kana::to_kana("AIUEOSASHISUSESONANINUNENOKAKIKUKEKO"))
}

#[bench]
fn bench_romaji_1(b: &mut test::Bencher) {
    b.iter(|| to_romaji::to_romaji("あいうえおさしすせそなにぬねのかきくけこ"))
}
#[bench]
fn bench_romaji_2(b: &mut test::Bencher) {
    b.iter(|| to_romaji::to_romaji("アイウエオサシスセソナニヌネノカキクケコ"))
}

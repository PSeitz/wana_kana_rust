#[cfg(test)]
mod tests {
    use super::*;

    use is_kanji::*;
    use is_kana::*;
    use is_katakana::*;
    use is_romaji::*;
    use is_japanese::*;
    use is_hiragana::*;
    use is_mixed::*;
    use to_kana::*;
    use to_katakana::*;
    use to_hiragana::*;
    use to_romaji::*;
    use strip_okurigana::*;
    use tokenize::*;
    use options::*;

    describe! Methods_should_return_valid_defaults_when_given_no_input {
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
          assert_eq!(strip_okurigana("", false), "");
      }
      // it "tokenize() with no input" {
      //     assert_eq!(tokenize(""), Vec::<Vec<String>>::new());
      // }
    }

    describe! character_type_detection {
      describe! is_hiragana {
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

      describe! is_katakana {
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

      describe! is_kana {
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

      describe! is_kanji {
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

      describe! is_japanese {
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
        it "泣き虫。！〜 (w. zenkaku punctuation) is japanese" {
            assert_eq!(is_japanese("泣き虫。！〜"), true);
        }
        it "泣き虫.!~ (w. romaji punctuation) is not japanese" {
            assert_eq!(is_japanese("泣き虫.!~"), false);
        }
        it "zenkaku numbers are considered neutral" {
            assert_eq!(is_japanese("０１２３４５６７８９"), true);
        }
        it "latin numbers are considered neutral" {
            assert_eq!(is_japanese("0123456789"), true);
        }
        it "mixed with numbers is japanese" {
            assert_eq!(is_japanese("２０１１年"), true);
        }
        it "hankaku katakana is allowed" {
            assert_eq!(is_japanese("ﾊﾝｶｸｶﾀｶﾅ"), true);
        }
      }

      describe! is_romaji {
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

      describe! is_mixed {
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

    describe! Character_conversion {
      describe! Double_consonants_transliterate_to_glottal_stops_small_tsu {
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

      describe! to_kana {
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

      describe! Converting_kana_to_kana {
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
        describe! long_vowels {
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

        describe! Mixed_syllabaries {
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

      describe! Case_sensitivity {
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

      describe! N_edge_cases {
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

      describe! Bogus_4_character_sequences {
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
    // }
    











    // describe! Kana_to_Romaji {
    //   describe! toRomaji {
    //     it('Convert katakana to romaji', () => expect(toRomaji('ワニカニ　ガ　スゴイ　ダ')).toBe('wanikani ga sugoi da'));
    //     it('Convert hiragana to romaji', () => expect(toRomaji('わにかに　が　すごい　だ')).toBe('wanikani ga sugoi da'));
    //     it('Convert mixed kana to romaji', () => expect(toRomaji('ワニカニ　が　すごい　だ')).toBe('wanikani ga sugoi da'));
    //     it('Will convert punctuation and full-width spaces', () => expect(toRomaji(JA_PUNC.join(''))).toBe(EN_PUNC.join('')));
    //     it('Use the upcaseKatakana flag to preserve casing. Works for katakana.', () => expect(toRomaji('ワニカニ', { upcaseKatakana: true })).toBe('WANIKANI'));
    //     it('Use the upcaseKatakana flag to preserve casing. Works for mixed kana.', () => expect(toRomaji('ワニカニ　が　すごい　だ', { upcaseKatakana: true })).toBe('WANIKANI ga sugoi da'));
    //     it("Doesn't mangle the long dash 'ー' or slashdot '・'", () => expect(toRomaji('罰ゲーム・ばつげーむ')).toBe('罰ge-mu/batsuge-mu'));
    //     it('Spaces must be manually entered', () => expect(toRomaji('わにかにがすごいだ')).not.toBe('wanikani ga sugoi da'));
    //   }

    //   describe('Quick Brown Fox - Hiragana to Romaji', () => {
    //     expect(toRomaji('いろはにほへと')).toBe('irohanihoheto');
    //     expect(toRomaji('ちりぬるを')).toBe('chirinuruwo');
    //     expect(toRomaji('わかよたれそ')).toBe('wakayotareso');
    //     expect(toRomaji('つねならむ')).toBe('tsunenaramu');
    //     expect(toRomaji('うゐのおくやま')).toBe('uwinookuyama');
    //     expect(toRomaji('けふこえて')).toBe('kefukoete');
    //     expect(toRomaji('あさきゆめみし')).toBe('asakiyumemishi');
    //     expect(toRomaji('ゑひもせすん')).toBe('wehimosesun');
    //   });

    //   describe("double n's and double consonants", () => {
    //     it('Double and single n', () => expect(toRomaji('きんにくまん')).toBe('kinnikuman'));
    //     it('N extravaganza', () => expect(toRomaji('んんにんにんにゃんやん')).toBe("nnninninnyan'yan"));
    //     it('Double consonants', () => expect(toRomaji('かっぱ　たった　しゅっしゅ ちゃっちゃ　やっつ')).toBe('kappa tatta shusshu chaccha yattsu'));
    //   });

    //   describe! Small_kana {
    //     it("Small tsu doesn't transliterate", () => expect(toRomaji('っ')).toBe(''));
    //     it('Small ya', () => expect(toRomaji('ゃ')).toBe('ya'));
    //     it('Small yu', () => expect(toRomaji('ゅ')).toBe('yu'));
    //     it('Small yo', () => expect(toRomaji('ょ')).toBe('yo'));
    //     it('Small a', () => expect(toRomaji('ぁ')).toBe('a'));
    //     it('Small i', () => expect(toRomaji('ぃ')).toBe('i'));
    //     it('Small u', () => expect(toRomaji('ぅ')).toBe('u'));
    //     it('Small e', () => expect(toRomaji('ぇ')).toBe('e'));
    //     it('Small o', () => expect(toRomaji('ぉ')).toBe('o'));
    //     it('Small ke (ka)', () => expect(toRomaji('ヶ')).toBe('ka'));
    //     it('Small ka', () => expect(toRomaji('ヵ')).toBe('ka'));
    //     it('Small wa', () => expect(toRomaji('ゎ')).toBe('wa'));
    //   });

    //   describe! Apostrophes_in_vague_consonant_vowel_combos {
    //     it('おんよみ', () => expect(toRomaji('おんよみ')).toBe("on'yomi"));
    //     it('んよ んあ んゆ', () => expect(toRomaji('んよ んあ んゆ')).toBe("n'yo n'a n'yu"));
    //   });
    // });

    // describe! stripOkurigana {
    //   it('passes default parameter tests', () => {
    //     expect(stripOkurigana('ふふフフ')).toBe('ふふフフ');
    //     expect(stripOkurigana('ふaふbフcフ')).toBe('ふaふbフcフ');
    //     expect(stripOkurigana('お腹')).toBe('お腹');
    //     expect(stripOkurigana('踏み込む')).toBe('踏み込');
    //     expect(stripOkurigana('お祝い')).toBe('お祝');
    //     expect(stripOkurigana('粘り')).toBe('粘');
    //     expect(stripOkurigana('〜い海軍い、。')).toBe('〜い海軍、。');
    //   });
    //   it('strips all kana when passed optional config', () => {
    //     expect(stripOkurigana('お腹', { all: true })).toBe('腹');
    //     expect(stripOkurigana('踏み込む', { all: true })).toBe('踏込');
    //     expect(stripOkurigana('お祝い', { all: true })).toBe('祝');
    //     expect(stripOkurigana('お踏み込む', { all: true })).toBe('踏込');
    //     expect(stripOkurigana('〜い海軍い、。', { all: true })).toBe('〜海軍、。');
    //   });
    // });

    // describe! tokenize {
    //   it('passes default parameter tests', () => {
    //     expect(tokenize('ふふ')).toEqual(['ふふ']);
    //     expect(tokenize('フフ')).toEqual(['フフ']);
    //     expect(tokenize('ふふフフ')).toEqual(['ふふ', 'フフ']);
    //     expect(tokenize('阮咸')).toEqual(['阮咸']);
    //     expect(tokenize('感じ')).toEqual(['感', 'じ']);
    //     expect(tokenize('私は悲しい')).toEqual(['私', 'は', '悲', 'しい']);
    //     expect(tokenize('what the...私は「悲しい」。')).toEqual([
    //       'what the...',
    //       '私',
    //       'は',
    //       '「',
    //       '悲',
    //       'しい',
    //       '」。',
    //     ]);
    //   });
    // });

    

    // describe! IMEMode {
    //   /**
    //      * Simulate real typing by calling the function on every character in sequence
    //      * @param  {String} input
    //      * @param  {Object} options
    //      * @return {String} converted romaji as kana
    //      */
    //   function testTyping(input, options) {
    //     let pos = 1;
    //     let text = input;
    //     const len = text.length;
    //     // console.log(`--${text}--`);
    //     while (pos <= len) {
    //       let buffer = text.slice(0, pos);
    //       const rest = text.slice(pos);
    //       buffer = toKana(buffer, options);
    //       // console.log(`${pos}:${buffer} <-${rest}`);
    //       text = buffer + rest;
    //       pos += 1;
    //     }
    //     return text;
    //   }

    //   it("Without IME mode, solo n's are transliterated.", () =>
    //     expect(toKana('n')).toBe('ん'));
    //   it("Without IME mode, double n's are transliterated.", () =>
    //     expect(toKana('nn')).toBe('ん'));

    //   it("With IME mode, solo n's are not transliterated.", () =>
    //     expect(testTyping('n', { IMEMode: true })).toBe('n'));
    //   it("With IME mode, double n's are transliterated.", () =>
    //     expect(testTyping('nn', { IMEMode: true })).toBe('ん'));
    //   it('With IME mode, n + space are transliterated.', () =>
    //     expect(testTyping('n ', { IMEMode: true })).toBe('ん'));
    //   it("With IME mode, n + ' are transliterated.", () =>
    //     expect(testTyping("n'", { IMEMode: true })).toBe('ん'));
    //   it('With IME mode, ni.', () => expect(testTyping('ni', { IMEMode: true })).toBe('に'));

    //   it('kan', () => expect(testTyping('kan', { IMEMode: true })).toBe('かn'));
    //   it('kanp', () => expect(testTyping('kanp', { IMEMode: true })).toBe('かんp'));
    //   it('kanpai!', () => expect(testTyping('kanpai', { IMEMode: true })).toBe('かんぱい'));
    //   it('nihongo', () => expect(testTyping('nihongo', { IMEMode: true })).toBe('にほんご'));

    //   it("y doesn't count as a consonant for IME", () =>
    //     expect(testTyping('ny', { IMEMode: true })).toBe('ny'));
    //   it('nya works as expected', () =>
    //     expect(testTyping('nya', { IMEMode: true })).toBe('にゃ'));

    //   it("With IME mode, solo N's are not transliterated - katakana.", () =>
    //     expect(testTyping('N', { IMEMode: true })).toBe('N'));
    //   it("With IME mode, double N's are transliterated - katakana.", () =>
    //     expect(testTyping('NN', { IMEMode: true })).toBe('ン'));
    //   it('With IME mode, NI - katakana.', () =>
    //     expect(testTyping('NI', { IMEMode: true })).toBe('ニ'));
    //   it('With IME mode - KAN - katakana', () =>
    //     expect(testTyping('KAN', { IMEMode: true })).toBe('カN'));
    //   it('With IME mode - NIHONGO - katakana', () =>
    //     expect(testTyping('NIHONGO', { IMEMode: true })).toBe('ニホンゴ'));
    // });

    // describe! Options {
    //   describe! useObsoleteKana {
    //     describe! toKana {
    //       it('useObsoleteKana is false by default', () => expect(toKana('wi')).toBe('うぃ'));
    //       it('wi = ゐ (when useObsoleteKana is true)', () =>
    //         expect(toKana('wi', { useObsoleteKana: true })).toBe('ゐ'));
    //       it('we = ゑ (when useObsoleteKana is true)', () =>
    //         expect(toKana('we', { useObsoleteKana: true })).toBe('ゑ'));
    //       it('WI = ヰ (when useObsoleteKana is true)', () =>
    //         expect(toKana('WI', { useObsoleteKana: true })).toBe('ヰ'));
    //       it('WE = ヱ (when useObsoleteKana is true)', () =>
    //         expect(toKana('WE', { useObsoleteKana: true })).toBe('ヱ'));
    //     });

    //     describe! toHiragana {
    //       it('useObsoleteKana is false by default', () =>
    //         expect(toHiragana('wi')).toBe('うぃ'));
    //       it('wi = ゐ (when useObsoleteKana is true)', () =>
    //         expect(toHiragana('wi', { useObsoleteKana: true })).toBe('ゐ'));
    //       it('we = ゑ (when useObsoleteKana is true)', () =>
    //         expect(toHiragana('we', { useObsoleteKana: true })).toBe('ゑ'));
    //       it('wi = うぃ when useObsoleteKana is false', () =>
    //         expect(toHiragana('wi', { useObsoleteKana: false })).toBe('うぃ'));
    //     });

    //     describe! toKataKana {
    //       it('wi = ウィ when useObsoleteKana is false', () =>
    //         expect(toKatakana('WI', { useObsoleteKana: false })).toBe('ウィ'));
    //       it('WI = ヰ (when useObsoleteKana is true)', () =>
    //         expect(toKatakana('wi', { useObsoleteKana: true })).toBe('ヰ'));
    //       it('WE = ヱ (when useObsoleteKana is true)', () =>
    //         expect(toKatakana('we', { useObsoleteKana: true })).toBe('ヱ'));
    //     });
    //   });
    // });



}

#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use wana_kana::*;

    #[bench]
    fn bench_kana_1(b: &mut test::Bencher) {
        b.iter(|| "aiueosashisusesonaninunenokakikukeko".to_kana())
    }

    #[bench]
    fn bench_kana_2(b: &mut test::Bencher) {
        b.iter(|| "AIUEOSASHISUSESONANINUNENOKAKIKUKEKO".to_kana())
    }

    #[bench]
    fn bench_romaji_to_hiragana(b: &mut test::Bencher) {
        b.iter(|| "aiueosashisusesonaninunenokakikukeko".to_hiragana())
    }

    #[bench]
    fn bench_katakana_to_hiragana(b: &mut test::Bencher) {
        b.iter(|| "アイウエオサシスセソナニヌネノカキクケコ".to_hiragana())
    }

    #[bench]
    fn bench_romaji_to_katakana(b: &mut test::Bencher) {
        b.iter(|| "aiueosashisusesonaninunenokakikukeko".to_katakana())
    }

    #[bench]
    fn bench_katakana_to_katakana(b: &mut test::Bencher) {
        b.iter(|| "あいうえおさしすせそなにぬねのかきくけこ".to_katakana())
    }

    #[bench]
    fn bench_hiragana_to_romaji(b: &mut test::Bencher) {
        b.iter(|| "あいうえおさしすせそなにぬねのかきくけこ".to_romaji())
    }

    #[bench]
    fn bench_katakana_to_romaji(b: &mut test::Bencher) {
        b.iter(|| "アイウエオサシスセソナニヌネノカキクケコ".to_romaji())
    }
}

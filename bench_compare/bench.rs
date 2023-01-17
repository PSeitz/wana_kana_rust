extern crate wana_kana;
use wana_kana::to_hiragana::to_hiragana;
use wana_kana::to_katakana::to_katakana;
use wana_kana::to_romaji::to_romaji;
use wana_kana::to_kana::*;


fn main() {
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        to_kana("aiueosashisusesonaninunenokakikukeko");
        to_kana("AIUEOSASHISUSESONANINUNENOKAKIKUKEKO");
        to_hiragana("aiueosashisusesonaninunenokakikukeko");
        to_hiragana("アイウエオサシスセソナニヌネノカキクケコ");
        to_katakana("aiueosashisusesonaninunenokakikukeko");
        to_katakana("あいうえおさしすせそなにぬねのかきくけこ");
        to_romaji("あいうえおさしすせそなにぬねのかきくけこ");
        to_romaji("アイウエオサシスセソナニヌネノカキクケコ");
    }

    println!("{:.2}", start.elapsed().as_nanos() as f32/1_000_000_f32);
}
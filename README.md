 [![Crates.io](https://img.shields.io/crates/v/wana_kana.svg)](https://crates.io/crates/wana_kana)
 [![Docs](https://docs.rs/wana_kana/badge.svg)](https://docs.rs/crate/wana_kana/)
 [![Build Status](https://travis-ci.org/PSeitz/wana_kana_rust.svg?branch=master)](https://travis-ci.org/PSeitz/wana_kana_rust)
 [![Coverage Status](https://coveralls.io/repos/github/PSeitz/wana_kana_rust/badge.svg?branch=master)](https://coveralls.io/github/PSeitz/wana_kana_rust?branch=master)

 ## WanaKana Rust
 ### ワナカナ <--> WanaKana <--> わなかな
```toml,ignore
[dependencies]
wana_kana = "2.0"
```


 Utility library for checking and converting between Japanese characters - Hiragana, Katakana - and Romaji (Ported from https://github.com/WaniKani/WanaKana V4.0.2)
 ## Examples
 ```
 use wana_kana::to_romaji::*;
 use wana_kana::to_kana::*;
 use wana_kana::to_hiragana::*;
 use wana_kana::Options;
 assert_eq!(to_romaji("ワナカナ"), "wanakana");
 assert_eq!(to_hiragana("WanaKana"), "わなかな");
 assert_eq!(to_kana("WANAKANA"), "ワナカナ");
 ```

## Tests

![100% coverage](https://raw.githubusercontent.com/PSeitz/wana_kana_rust/master/coverage_good.png)

## Performance
On Migrating to 2.0 some performance improvements have been implemented by using more efficient lookup structures and avoiding allocations. 
According to these results around 1000 words can be converted per millisecond on a Core i7-6700.

```
 bench_hiragana_to_romaji    3,519            1,070              -2,449  -69.59%   x 3.29
 bench_kana_1                3,066            567                -2,499  -81.51%   x 5.41
 bench_kana_2                8,006            1,831              -6,175  -77.13%   x 4.37
 bench_katakana_to_hiragana  2,512            622                -1,890  -75.24%   x 4.04
 bench_katakana_to_katakana  1,664            629                -1,035  -62.20%   x 2.65
 bench_katakana_to_romaji    6,922            1,067              -5,855  -84.59%   x 6.49
 bench_romaji_to_hiragana    3,802            1,300              -2,502  -65.81%   x 2.92
 bench_romaji_to_katakana    4,361            1,929              -2,432  -55.77%   x 2.26
```

### Comparison To [WanaKana](https://github.com/WaniKani/WanaKana)

A detailed analysis has been done in the [bench_compare](bench_compare/README.md) subfolder, the analysis below may be inaccurate.

A short comparison suggests around 25x performance

```javascript
import toKana from './src/toKana';
import toHiragana from './src/toHiragana';
import toKatakana from './src/toKatakana';
import toRomaji from './src/toRomaji';


console.time("yo")
for (var i = 0; i < 1000; i++) {
    toKana('aiueosashisusesonaninunenokakikukeko')
    toKana('AIUEOSASHISUSESONANINUNENOKAKIKUKEKO')
    toHiragana('aiueosashisusesonaninunenokakikukeko')
    toHiragana('アイウエオサシスセソナニヌネノカキクケコ')
    toKatakana('aiueosashisusesonaninunenokakikukeko')
    toKatakana('あいうえおさしすせそなにぬねのかきくけこ')
    toRomaji('あいうえおさしすせそなにぬねのかきくけこ')
    toRomaji('アイウエオサシスセソナニヌネノカキクケコ')
}

console.timeEnd("yo")
```
`node -r esm run.js`

```rust
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

    println!("{:?}", start.elapsed().as_millis());
}

```

`node -r esm run.js`  *253.231ms*

`cargo run --release --bin bench`  *9ms*


### CLI
#### Convert to kana and back for fun and profit
`cargo install wana_kana` will install 2 CLI tools: `to_kana` and `to_romaji`.

Both commands support piping `ls | to_kana` and parameters `to_romaji へろ　をるど`.

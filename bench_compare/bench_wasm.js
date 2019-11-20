
var wana_kana_wasm = require('../wana_kana_wasm/wasmbinding/wana_kana_wasm.js');

console.time("wasm")
for (var i = 0; i < 1000; i++) {
    wana_kana_wasm.to_kana('aiueosashisusesonaninunenokakikukeko')
    wana_kana_wasm.to_kana('AIUEOSASHISUSESONANINUNENOKAKIKUKEKO')
    wana_kana_wasm.to_hiragana('aiueosashisusesonaninunenokakikukeko')
    wana_kana_wasm.to_hiragana('アイウエオサシスセソナニヌネノカキクケコ')
    wana_kana_wasm.to_katakana('aiueosashisusesonaninunenokakikukeko')
    wana_kana_wasm.to_katakana('あいうえおさしすせそなにぬねのかきくけこ')
    wana_kana_wasm.to_romaji('あいうえおさしすせそなにぬねのかきくけこ')
    wana_kana_wasm.to_romaji('アイウエオサシスセソナニヌネノカキクケコ')
}

console.timeEnd("wasm")

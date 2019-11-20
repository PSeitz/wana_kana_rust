var wana_kana_wasm = require('../native');
module.exports = wana_kana_wasm;


// console.time("bindings")
// for (var i = 0; i < 1000; i++) {
//     wana_kana_wasm.to_kana('aiueosashisusesonaninunenokakikukeko')
//     wana_kana_wasm.to_kana('AIUEOSASHISUSESONANINUNENOKAKIKUKEKO')
//     wana_kana_wasm.to_hiragana('aiueosashisusesonaninunenokakikukeko')
//     wana_kana_wasm.to_hiragana('アイウエオサシスセソナニヌネノカキクケコ')
//     wana_kana_wasm.to_katakana('aiueosashisusesonaninunenokakikukeko')
//     wana_kana_wasm.to_katakana('あいうえおさしすせそなにぬねのかきくけこ')
//     wana_kana_wasm.to_romaji('あいうえおさしすせそなにぬねのかきくけこ')
//     wana_kana_wasm.to_romaji('アイウエオサシスセソナニヌネノカキクケコ')
// }

// console.timeEnd("bindings")
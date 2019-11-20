const wana_kana_wasm = require('./wasmbinding/wana_kana_wasm.js');

// // Call wasm method 'add' typical stack method
// let result = wana_kana_wasm.add(10, 2);
// console.log('add result:' + result);
// // Call wasm method 'add' typical stack method
// console.log('add result:' + wana_kana_wasm.to_kana("nice"));




// import toKana from './src/toKana';
// import toHiragana from './src/toHiragana';
// import toKatakana from './src/toKatakana';
// import toRomaji from './src/toRomaji';


console.time("yo")
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

console.timeEnd("yo")

var wanakana = require('../nodejs-bindings/lib');

console.time("bindings")
for (var i = 0; i < 1000; i++) {
    wanakana.to_kana('aiueosashisusesonaninunenokakikukeko')
    wanakana.to_kana('AIUEOSASHISUSESONANINUNENOKAKIKUKEKO')
    wanakana.to_hiragana('aiueosashisusesonaninunenokakikukeko')
    wanakana.to_hiragana('アイウエオサシスセソナニヌネノカキクケコ')
    wanakana.to_katakana('aiueosashisusesonaninunenokakikukeko')
    wanakana.to_katakana('あいうえおさしすせそなにぬねのかきくけこ')
    wanakana.to_romaji('あいうえおさしすせそなにぬねのかきくけこ')
    wanakana.to_romaji('アイウエオサシスセソナニヌネノカキクケコ')
}

console.timeEnd("bindings")
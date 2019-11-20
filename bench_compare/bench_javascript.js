let wanakana = require("wanakana");
console.time("javascript")

for (var i = 0; i < 1000; i++) {
    wanakana.toKana('aiueosashisusesonaninunenokakikukeko')
    wanakana.toKana('AIUEOSASHISUSESONANINUNENOKAKIKUKEKO')
    wanakana.toHiragana('aiueosashisusesonaninunenokakikukeko')
    wanakana.toHiragana('アイウエオサシスセソナニヌネノカキクケコ')
    wanakana.toKatakana('aiueosashisusesonaninunenokakikukeko')
    wanakana.toKatakana('あいうえおさしすせそなにぬねのかきくけこ')
    wanakana.toRomaji('あいうえおさしすせそなにぬねのかきくけこ')
    wanakana.toRomaji('アイウエオサシスセソナニヌネノカキクケコ')
}

console.timeEnd("javascript")

## Benchmarks

This compares 4 projects:
- [pure javascript](bench_javascript.js)
- [pure rust](bench.rs)
- [wasm generated from rust](bench_wasm.js)
- [rust with node using neon](bench_nodejs_bindings.js)

There are be some differences in the datastructures between javascript and the port to rust, but I think these differences are idiomatic. Overall the algorithms are implemented in a similar way and behave equally, the tests were also ported. Everything is singlethreaded. 

### Test

```javascript

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

```



### Results


|               | javascript    | rust          | wasm  | node with rust bindings  |
| ------------- | ------------- |---------------| ------| -------------------------|
| Wall Time[ms]      | 253ms         | 9ms           | 72ms  | 18ms                     |
| Wall Time Relative | 28x            | 1x             | 8x     | 2x                        |


### Conclusion

It can make sense to port performance sensitive code to wasm if total compatiblity is required (e.g. browser and node support, or no rust compiler feasible). Much more gains are possible when using native rust code, calling from nodejs has 2x overhead in this case.

### Open
No analysis regarding memory analysis has been done yet.


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


| Num Loops |                     | javascript    | rust          | wasm (rust) | node with rust bindings   |
|-------    | -------------       | ------------- |---------------| ------------|------------------------   |
| 1_000     | Wall Time[ms]       | 253           | 9             | 72          | 18                        |
|           | Wall Time Relative  | 28x           | 1x            | 8x          | 2x                        |
| 100_000   | Wall Time[ms]       | 16149         | 1959          | 3783        | 3131                      |
|           | Wall Time Relative  | 8,2x          | 1x            | 1,93x       | 1,59x                     |
| 1_000_000 | Wall Time[ms]       | 181879        | 21956         | 36885       | 31294                     |
|           | Wall Time Relative  | 8,2x          | 1x            | 1,67x       | 1,42x                     |

### Conclusion

It can make sense to port performance sensitive code to wasm if total compatiblity is required (e.g. browser and node support, or no rust compiler feasible).
More gains are possible when using native rust code or calling rust from nodejs.

### Open
No analysis regarding memory analysis has been done yet.

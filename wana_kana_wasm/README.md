`cargo build --target wasm32-unknown-unknown --release --target-dir target`

`wasm-bindgen target/wasm32-unknown-unknown/release/deps/wana_kana_wasm.wasm --nodejs --out-dir ./wasmbinding`

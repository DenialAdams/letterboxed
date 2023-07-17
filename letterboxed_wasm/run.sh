cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web ../target/wasm32-unknown-unknown/release/letterboxed_wasm.wasm --out-dir ./pkg
sfz

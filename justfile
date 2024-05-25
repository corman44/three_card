build-wasm:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name "hello_world" ./target/wasm32-unknown-unknown/release/three_card.wasm
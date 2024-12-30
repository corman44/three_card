build-wasm:
    cargo build --profile wasm-release --target wasm32-unknown-unknown
    wasm-bindgen --out-name three_card \
        --out-dir out \
        --target web target/wasm32-unknown-unknown/release/three_card.wasm

run-wasm:
    basic-http-server out/

wasm:
    just build-wasm
    just run-wasm
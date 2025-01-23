build-wasm:
    cargo build --profile wasm-release --target wasm32-unknown-unknown
    wasm-bindgen --out-name three_card \
        --out-dir out \
        --target web target/wasm32-unknown-unknown/wasm-release/three_card.wasm

run-wasm:
    basic-http-server out/

run-wasm-remote:
    basic-http-server --addr 0.0.0.0:4000 out/

small-wasm:
    cargo build --profile wasm-release --target wasm32-unknown-unknown
    target/wasm32-unknown-unknown/wasm-release/binaryen-version_121/bin/wasm-opt.exe -Os -o three_card_opt.wasm target/wasm32-unknown-unknown/wasm-release/three_card.wasm
    wasm-bindgen --out-name three_card \
        --out-dir out \
        --target web target/wasm32-unknown-unknown/wasm-release/three_card_opt.wasm
    basic-http-server out-small/

wasm:
    just build-wasm
    just run-wasm
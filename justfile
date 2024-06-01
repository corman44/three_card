build-wasm:
    RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo build --release --target wasm32-unknown-unknown
    # wasm-bindgen --no-typescript --target web --out-dir ./out/ --out-name 
    
run-wasm:
    RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo build --release --target wasm32-unknown-unknown
    wasm-server-runner target/wasm32-unknown-unknown/release/three_card.wasm

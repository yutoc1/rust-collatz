FROM --platform=wasi/wasm32 scratch
COPY ./target/wasm32-wasi/release/rust-collatz.wasm /rust-collatz.wasm
ENTRYPOINT ["rust-collatz.wasm"]

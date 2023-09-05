FROM scratch
COPY --chmod=755 ./target/wasm32-wasi/release/rust-collatz.wasm /rust-collatz.wasm
ENTRYPOINT ["/rust-collatz.wasm"]

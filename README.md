# rust-collatz

## 目的

RustとWASMの処理時間を計測する。
単純なアルゴリズムであるcollatz数列の処理時間を計測して比較する。

N = 10^6

## build

### Rust

    cargo run

### WASM

    cargo build --target wasm32-wasi --release

    sudo docker buildx create --name wasmbuilder --driver docker-container --bootstrap↲↲
    sudo docker buildx use wasmbuilder↲

    sudo docker buildx build --platform wasi/wasm32 --load -t wasm-collatz:0.1 .
    sudo docker container run --rm --name=rust-collatz --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 wasm-wasm:0.1

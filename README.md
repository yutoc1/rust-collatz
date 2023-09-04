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
    sudo docker buildx build -t wasm-collatz:0.1 .
    sudo docker container run --rm --name=mydockerwasm --runtime=io.containerd.wasmedge.v1 wasm-wasm:0.1

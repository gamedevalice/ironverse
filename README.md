## Setup
Install rust-lang: https://www.rust-lang.org/
Install cargo plugins:
```
cargo install wasm-pack
cargo install wasm-bindgen-cli
cargo install wasm-opt
cargo install cargo-make
cargo install duckscript_cli
cargo install basic-http-server
```

## Build and Run

### Desktop
```
cargo run --release
```
### Web
```
cargo make web
```
## Setup
Install rust-lang: https://www.rust-lang.org/
Install cargo plugins:
```
cargo install wasm-pack
cargo install wasm-bindgen-cli
cargo install cargo-make
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
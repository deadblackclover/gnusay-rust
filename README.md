# gnusay-rust
GNU say made using Rust

```
   +-----------+ 
   |I love GNU!|
   +-----------+ 
    \   / 
     \ / 
 .= .-_-. =.
((_/)o o(\_)) 
 `-'(. .)`-' 
  /| \_/ |\ 
 ( | GNU | ) 
 /'\_____/'\ 
 \__)   (__/
```

## Install

If you have Rust: `cargo install gnusay`

## Build for [wasmtime](https://github.com/bytecodealliance/wasmtime)

```
rustup target add wasm32-wasi
cargo wasm-build
```

and run it

```
wasmtime target/wasm32-wasi/release/gnusay.wasm
```

## Usage

```bash
GNU say made using Rust

Usage: gnusay <TEXT>

Arguments:
  <TEXT>  

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# gnusay-rust
GNU say made using Rust

```
   ___________ 
  <I love GNU!>
   ----------- 
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
wasmtime target/wasm32-wasi/release/cube-rs.wasm
```

## Usage

```bash
gnusay 0.2.0
DEADBLACKCLOVER <deadblackclover@protonmail.com>
GNU say made using Rust

USAGE:
    gnusay <TEXT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <TEXT>    Text to display
```

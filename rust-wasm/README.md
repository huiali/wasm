cargo install wasm-pack

cargo new --lib rust-wasm

use wasm_bindgen::prelude::*;

[dependencies]
wasm-bindgen = "0.2"
md5="0.7"


wasm-pack build


static -->wasm-pack build --target no-modules
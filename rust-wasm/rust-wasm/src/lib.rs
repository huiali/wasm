extern crate wasm_bindgen;
use md5;
use wasm_bindgen::prelude::*;

//导入 JavaScript window.alert 函数
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//导出 wasm hello 函数
#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("huiali---hello, {}!", name));
}

//导出 wasm hello 函数
#[wasm_bindgen]
pub fn to_md5(param: &str) -> String {
    let digest = md5::compute(param);
    return format!("{:x}", digest);
}

//导出 wasm hello 函数
#[wasm_bindgen]
pub fn fib(i: i32) -> i64 {
    if i <= 0 {
        return 0;
    }
    if i <= 2 {
        return 1;
    }

    fib(i - 1) + fib(i - 2)
}

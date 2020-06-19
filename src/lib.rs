mod utils;

use wasm_bindgen::prelude::*;
use js_sys::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen (js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn hello_string(s: &str) -> String {
    format!("Hello, {}!", s)
}

#[wasm_bindgen]
pub fn greet() -> String {
    String::from("Hello, rust-fn!")
}

#[wasm_bindgen]
pub fn take_jsvalue(j: JsValue) {
    log(&format!("{:#?}",j));
    let name = js_sys::Reflect::get(&j, &"name".into());
    log(&format!("{:#?}",name));
}

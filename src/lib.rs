mod utils;

use wasm_bindgen::prelude::*;
use serde::*;

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

#[derive(Deserialize)]
pub struct Name {
    pub name: String
}

#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

#[wasm_bindgen]
pub fn greet_json() -> JsValue {
    let msg_string = String::from("Hello, rust-fn!");
    let msg = Message {
        message: msg_string
    };
    JsValue::from_serde(&msg).unwrap()
}

#[wasm_bindgen]
pub fn hello_json(i: &JsValue) -> JsValue {
    let i_name: Name = i.into_serde()
        .unwrap_or_else( |_e| {
            Name { name: "World".to_string() }
        });
    let msg = Message {
        message: format!("Hello, {}!", i_name.name)
    };
    JsValue::from_serde(&msg).unwrap()
}

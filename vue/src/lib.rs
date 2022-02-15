mod future_exec;
#[doc(hidden)]
pub mod macros_utils;
mod reactive;
mod utils;

pub use reactive::Reactive;
#[cfg(feature = "derive")]
pub use vue_derive::{methods, Vue};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, vue-rs!");
}

#[wasm_bindgen(module = "vue")]
extern "C" {
    #[wasm_bindgen(js_namespace = default)]
    pub fn emit(target: &JsValue, name: &str, value: JsValue);

    #[wasm_bindgen(js_namespace = default)]
    pub fn get(o: &JsValue, property_key: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = default)]
    pub fn observable(o: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = default)]
    pub fn set(target: &JsValue, property_key: &str, value: JsValue);
}

pub fn new_obj() -> JsValue {
    observable(js_sys::Object::new().into())
}

mod utils;

use wasm_bindgen::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(n: i32, m: i32) -> i32 {
    n + m
}

#[wasm_bindgen]
//pub fn filter_even(n: Vec<i32>) -> Vec<i32> {
pub fn filter_even(n: Vec<i32>) -> js_sys::Array {
    n.iter().filter(|x| **x % 2 == 0).cloned().map(|x| JsValue::from_f64(x as f64)).collect()
}


#[wasm_bindgen]
pub fn concat_str(l: String, r: String, sep: String) -> js_sys::JsString {
    //js_sys::JsString::from(JsValue::from_str(&(l + &sep + &r)))
    JsValue::from_str(&(l + &sep + &r)).into()
}

#[wasm_bindgen]
pub fn load_bytes(bytes: Vec<u8>) -> js_sys::ArrayBuffer {
    //web_sys::console::log_1(&JsValue::from_serde(&bytes).unwrap());
    JsValue::from_serde(&bytes).unwrap().into()
}
#[wasm_bindgen]
pub async fn load_file(file: web_sys::File) {
    let cb = Closure::wrap(Box::new(move |text| {
        web_sys::console::log_1(&text);
    }) as Box<dyn FnMut(JsValue)>);
    let promise = file.text().then(&cb);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();

    cb.forget();
}

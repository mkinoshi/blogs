use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(numbers: Vec<i32>) -> i32 {
    return numbers.iter().sum();
}

// First version of get_total_characters func
//#[wasm_bindgen]
//pub fn get_total_characters(items: Vec<String>) -> i32 {
//return items
//.iter()
//.fold(0, |acc, item| acc + item.chars().count() as i32);
//}

#[wasm_bindgen]
pub fn get_total_characters(items: &JsValue) -> i32 {
    let items_vec: Vec<String> = items.into_serde().unwrap();
    return items_vec
        .iter()
        .fold(0, |acc, item| acc + item.chars().count() as i32);
}

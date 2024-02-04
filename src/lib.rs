use wasm_bindgen::prelude::*;
include!("main.rs");

#[wasm_bindgen]
pub fn start(){
    init();
}
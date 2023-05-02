use wasm_bindgen::prelude::*;
mod data;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
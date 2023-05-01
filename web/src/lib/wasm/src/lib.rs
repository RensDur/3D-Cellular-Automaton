use wasm_bindgen::prelude::*;
mod voxels;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Good afternoon, {}!", name));
}
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn displayOutput(msg: &str);
    pub fn clear();
}

#[wasm_bindgen]
pub fn read_line(line: &str) {
    match line {
        "clear" => clear(),
        _ => displayOutput(line),
    }
}

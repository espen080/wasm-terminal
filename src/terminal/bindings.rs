use super::help::help;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Functions that lets Rust call into JS.
    pub fn displayOutput(msg: &str);
    pub fn clear();
}

// Functions that lets JS call into Rust.
#[wasm_bindgen]
pub struct Terminal {
    line_buffer: Vec<String>,
}
#[wasm_bindgen]
impl Terminal {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Terminal {
        Terminal {
            line_buffer: Vec::new(),
        }
    }
    pub fn read_line(&mut self, line: &str) {
        self.line_buffer.push(String::from(line));
        for l in &self.line_buffer {
            displayOutput(&format!("> {}", l));
        }
        match line {
            "clear" => clear(),
            "help" => help(),
            _ => displayOutput(&format!(
                "Command '{}' not recognized. (use 'help' to show available commands)",
                line
            )),
        }
    }
}

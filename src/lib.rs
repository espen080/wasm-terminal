use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn displayOutput(msg: &str);
    pub fn clear();
}

#[wasm_bindgen]
pub fn read_line(line: &str) {
    displayOutput(&format!("> {}", line));
    match line {
        "clear" => clear(),
        "help" => help(),
        _ => displayOutput(&format!(
            "Command '{}' not recognized. (use 'help' to show available commands)",
            line
        )),
    }
}

fn help() {
    let message: &str = "
================================

WASM Terminal

================================

Commands:
clear    - Clears the terminal window history.
help     - Show this message.
";
    displayOutput(message);
}

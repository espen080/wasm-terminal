use super::bindings::displayOutput;

pub fn help() {
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

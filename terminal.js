      import init, { read_line } from "./pkg/wasm_terminal.js";

      const terminalViewPort = document.getElementById("viewPort")
      const terminalInput = document.getElementById("terminal");

      terminalInput.addEventListener('keydown', function(event) {
        if (event.key === 'Enter') {
          handleInput();
        }
      })

      function handleInput() {
        const value = terminalInput.value;
        terminalInput.value = null
        read_line(value);
      }

      function clear() {
        terminalViewPort.innerHTML = null
      }
      window.clear = clear

      function displayOutput(msg) {
        const newLine = document.createElement("div");
        newLine.innerHTML = msg;
        terminalViewPort.appendChild(newLine);
        
      }
      window.displayOutput = displayOutput

      init();

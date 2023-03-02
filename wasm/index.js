import wasmInit from "./pkg/wasm.js";

const runWasm = async () => {
  // Instantiate wasm module
  const wasm_fxns = await wasmInit("./pkg/wasm_bg.wasm");

  // Call the Add function export from wasm, save the result
  const rand_visitor = wasm_fxns.random();

  // Set the result onto the body
  document.getElementById('visitorCounter').innerText = `Visitor # ${rand_visitor}`;
};
runWasm();
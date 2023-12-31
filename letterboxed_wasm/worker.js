importScripts("./pkg/letterboxed_wasm.js");
const { setup, app_init, next_word } = wasm_bindgen;

let app = wasm_bindgen('./pkg/letterboxed_wasm_bg.wasm');

app.then(_obj => {
   wasm_bindgen.app_init();
   postMessage(["ready", null]);
});

onmessage = async function(e) {
   wasm_bindgen.setup(e.data);
   while (true) {
      let nw = wasm_bindgen.next_word();
      if (nw) {
        postMessage(["new", nw]);
      } else {
        break;
      }
   }
   postMessage(["finished", null]);
};

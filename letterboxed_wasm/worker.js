importScripts("./pkg/letterboxed.js");
const { setup, app_init, next_word } = wasm_bindgen;

let app = wasm_bindgen('./pkg/letterboxed_bg.wasm');

app.then(_obj => {
   wasm_bindgen.app_init();
   postMessage(["ready", null]);
});

onmessage = async function(e) {
   let board = e.data[0];
   wasm_bindgen.setup(board);
   while (true) {
      let nw = next_word();
      if (nw) {
        postMessage(["new", nw]);
      } else {
        break;
      }
   }
   postMessage(["finished", null]);
};

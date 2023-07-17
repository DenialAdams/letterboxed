import { setup, next_word, default as init } from './pkg/letterboxed_wasm.js';

function isAlpha(str) {
   return /^[a-z]+$/.test(str);
}

function wordIsValid(word) {
   if (word.length != 12) {
      return false;
   }
   if (!isAlpha(word)) {
      return false;
   }

   return true;
}

function getValidWord() {
   let theWord = document.getElementById("word").value.trim().toLowerCase();
   if (!wordIsValid(theWord)) {
      return null;
   }
   return theWord;
}

function solve() {
   let theWord = getValidWord();
   if (!theWord) {
      return;
   }
   document.getElementById("solveButton").disabled = false;
   let rl = document.getElementById("resultlist");
   while (true) {
      let nw = next_word();
      if (nw) {
         rl.innerText = nw + "\n" + rl.innerText;
      } else {
         break;
      }
   }
   document.getElementById("solveButton").disabled = false;
}

window.solveWord = async function onSolvePress() {
   solve();
};

window.initApp = async function initApp() {
   await init('./pkg/letterboxed_wasm_bg.wasm');
   app_init();
   document.getElementById("solveButton").disabled = false;
};

window.addEventListener('DOMContentLoaded', (event) => {
   initApp();
});

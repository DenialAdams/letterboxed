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

let worker = new Worker('./worker.js');
worker.onmessage = function(e) {
   if (e.data[0] == "ready") {
      document.getElementById("solveButton").disabled = false;
   } else if (e.data[0] == "new") {
      let rl = document.getElementById("resultlist");
      rl.innerText = e.data[1] + "\n" + rl.innerText;
   } else if (e.data[0] == "finished") {
      document.getElementById("solveButton").disabled = false;
   }
};

window.solve = function solve() {
   let theWord = getValidWord();
   if (!theWord) {
      return;
   }
   document.getElementById("solveButton").disabled = false;
   worker.postMessage(theWord);
}

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
      let rlns = document.getElementById("resultlistNoSpoilers");
      let wordCount = e.data[1].split(' ').length;
      rlns.innerText = wordCount.toString() + " word solution";
   } else if (e.data[0] == "finished") {
      document.getElementById("solveButton").disabled = false;
   }
};

window.solve = function solve() {
   let theWord = getValidWord();
   if (!theWord) {
      return;
   }
   document.getElementById("resultlist").innerText = "";
   document.getElementById("solveButton").disabled = false;
   worker.postMessage(theWord);
}

window.toggleSpoilers = function toggleSpoilers(hideSpoilers) {
   document.getElementById("resultlist").hidden = hideSpoilers;
   document.getElementById("resultlistNoSpoilers").hidden = !hideSpoilers;
}

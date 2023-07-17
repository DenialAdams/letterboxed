pipeline {
   agent any

   stages {
      stage('Clean') {
         steps {
            sh 'cargo clean'
         }
      }

      stage('Wasm Build') {
         steps {
            dir('letterboxed_wasm') {
               sh 'cargo build --release --target wasm32-unknown-unknown'
               sh 'wasm-bindgen --target no-modules ../target/wasm32-unknown-unknown/release/letterboxed_wasm.wasm --out-dir ./pkg'
            }
         }
      }

      stage('Deploy') {
         when {
            expression { env.BRANCH_NAME == "master" }
         }
         steps {
            dir('publish') {
               dir('pkg') {
                  sh 'cp ../../letterboxed_wasm/pkg/letterboxed_wasm.js .'
                  sh 'cp ../../letterboxed_wasm/pkg/letterboxed_wasm_bg.wasm .'
               }
               sh 'cp ../letterboxed_wasm/index.html .'
               sh 'cp ../letterboxed_wasm/index.js .'
               sh 'cp ../letterboxed_wasm/worker.js .'
               sh 'cp ../letterboxed_wasm/stylesheet.css .'
               sshagent (credentials: ['jenkins-ssh-nfs']) {
                  sh 'rsync -avr -e "ssh -l flandoo_brickcodes -o StrictHostKeyChecking=no" --exclude ".git" --exclude "pkg@tmp" . ssh.phx.nearlyfreespeech.net:/home/public/letterboxed'
               }
            }
         }
      }
   }
}

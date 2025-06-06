use std::sync::Mutex;

use letterboxed_lib::SolverState;
use wasm_bindgen::prelude::*;

static STATE: Mutex<Option<SolverState>> = Mutex::new(None);

#[wasm_bindgen]
pub fn app_init() {
   std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn setup(board_str: String) -> bool {
   let chars: Vec<char> = board_str
      .chars()
      .filter(|x| !x.is_whitespace())
      .flat_map(|x| x.to_uppercase())
      .collect();

   if let Ok(arr) = chars.try_into() {
      *STATE.lock().unwrap() = Some(SolverState::setup(arr));
      return true;
   }

   false
}

#[wasm_bindgen]
pub fn next_word() -> Option<String> {
   let mut state_lock = STATE.lock().unwrap();
   let s = state_lock.as_mut().unwrap();

   s.next_solution()
}

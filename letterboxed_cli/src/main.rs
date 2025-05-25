#![allow(clippy::uninlined_format_args)] // I'm an old man and I like the way it was before
use letterboxed_lib::SolverState;

fn main() {
   //let board = ['S', 'O', 'F', 'T', 'I', 'N', 'D', 'G', 'A', 'R', 'U', 'H'];
   //let board = ['L', 'A', 'B', 'L', 'A', 'B', 'L', 'A', 'B', 'L', 'A', 'B'];
   //let board = ['E', 'A', 'B', 'D', 'R', 'U', 'S', 'I', 'T', 'O', 'M', 'X'];
   //let board = ['T', 'A', 'M', 'U', 'R', 'D', 'I', 'B', 'Q', 'E', 'N', 'O'];
   let board = ['C', 'A', 'N', 'C', 'A', 'N', 'C', 'A', 'N', 'C', 'A', 'N'];
   let mut s = SolverState::setup(board);

   while let Some(ans) = s.next_solution() {
      println!("{}", ans);
   }
}

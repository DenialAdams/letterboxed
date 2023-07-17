use letterboxed_lib::SolverState;

fn main() {
   let board = ['S', 'O', 'F', 'T', 'I', 'N', 'D', 'G', 'A', 'R', 'U', 'H'];
   let mut s = SolverState::setup(board);

   while let Some(ans) = s.next_solution() {
      println!("{}", ans);
   }
}

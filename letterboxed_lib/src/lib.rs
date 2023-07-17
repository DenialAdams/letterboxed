use std::sync::OnceLock;

use qp_trie::wrapper::BString;
use qp_trie::Trie;

#[derive(Clone)]
struct SolutionState {
   cur_sequence: String,
   words: Vec<String>,
   position: usize,
   visited: u16,
   last_char: char,
   total_letters: usize,
}

static DICTIONARY: OnceLock<Trie<BString, ()>> = OnceLock::new();

type GameGrid = [char; 12];

pub struct SolverState {
   dict: &'static Trie<BString, ()>,
   best_solution: Option<SolutionState>,
   board: GameGrid,
   stack: Vec<SolutionState>,
}

impl SolverState {
   pub fn setup(board: GameGrid) -> SolverState {
      let dict = DICTIONARY.get_or_init(|| {
         let mut dict = Trie::new();
         let f = include_str!("words_easy.txt");
         for line in f.lines() {
            if line.len() < 3 {
               continue;
            }
            dict.insert_str(line, ());
         }
         dict
      });
   
      let mut s = SolverState {
         dict,
         best_solution: None,
         board,
         stack: Vec::new(),
      };
   
      s.reset(board);
   
      s
   }

   pub fn reset(&mut self, board: GameGrid) {
      self.stack.clear();
      self.stack.push(SolutionState {
         cur_sequence: String::new(),
         words: vec![],
         position: 0,
         visited: 0xF000,
         total_letters: 0,
         last_char: '\0',
      });
      self.board = board;
   }

   pub fn next_solution(&mut self) -> Option<String> {
      while let Some(solution) = self.stack.pop() {
         // Bound
         if let Some(bs) = self.best_solution.as_ref() {
            match bs.words.len().cmp(&(solution.words.len() + 1)) {
               std::cmp::Ordering::Less => continue,
               std::cmp::Ordering::Equal => {
                  if bs.total_letters <= solution.total_letters {
                     continue;
                  }
               }
               std::cmp::Ordering::Greater => (),
            }
         }

         if self.dict.subtrie_str(&solution.cur_sequence).is_empty() {
            continue;
         }

         if self.dict.contains_key_str(&solution.cur_sequence) && !solution.words.contains(&solution.cur_sequence) {
            let mut new_solution = solution.clone();
            new_solution.words.push(std::mem::take(&mut new_solution.cur_sequence));
            new_solution.cur_sequence.push(new_solution.last_char);
            if new_solution.visited == 0xFFFF {
               let solution_str = new_solution.words.join(", ");
               self.best_solution = Some(new_solution);
               return Some(solution_str);
            }
            self.stack.push(new_solution);
         }

         let reachable_letters = if solution.total_letters == 0 {
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
         } else {
            reachable(solution.position)
         };

         for dest in reachable_letters {
            let letter = self.board[*dest];
            let mut new_solution = solution.clone();
            new_solution.visited |= 1 << *dest as u16;
            new_solution.cur_sequence.push(letter);
            new_solution.total_letters += 1;
            new_solution.position = *dest;
            new_solution.last_char = letter;
            self.stack.push(new_solution);
         }
      }

      None
   }
}

fn reachable(x: usize) -> &'static [usize] {
   if x < 3 {
      &[3, 4, 5, 6, 7, 8, 9, 10, 11]
   } else if x < 6 {
      &[0, 1, 2, 6, 7, 8, 9, 10, 11]
   } else if x < 9 {
      &[0, 1, 2, 3, 4, 5, 9, 10, 11]
   } else {
      &[0, 1, 2, 3, 4, 5, 6, 7, 8]
   }
}

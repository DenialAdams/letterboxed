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

static DICTIONARY: OnceLock<Vec<String>> = OnceLock::new();

type GameGrid = [char; 12];

pub struct SolverState {
   dict: Trie<BString, ()>,
   best_solution: Option<SolutionState>,
   board: GameGrid,
   stack: Vec<SolutionState>,
   attempted_heuristic: bool,
}

fn word_can_be_made(position: Option<usize>, mut remaining_word: Vec<char>, board: &GameGrid) -> bool {
   let next_letter = match remaining_word.pop() {
      Some(nl) => nl,
      None => return true,
   };

   let reachable_letters = if let Some(pos) = position {
      reachable(pos)
   } else {
      &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
   };

   for possible_dest in reachable_letters.iter() {
      if board[*possible_dest] == next_letter && word_can_be_made(Some(*possible_dest), remaining_word.clone(), board) {
         return true;
      }
   }

   false
}

fn word_path(position: Option<usize>, mut remaining_word: Vec<char>, board: &GameGrid, path: Vec<usize>) -> Option<Vec<usize>> {
   let next_letter = match remaining_word.pop() {
      Some(nl) => nl,
      None => return Some(path),
   };

   let reachable_letters = if let Some(pos) = position {
      reachable(pos)
   } else {
      &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
   };

   for possible_dest in reachable_letters.iter() {
      if board[*possible_dest] == next_letter {
         let mut next_path = path.clone();
         next_path.reserve_exact(1);
         next_path.push(*possible_dest);

         let final_path = word_path(Some(*possible_dest), remaining_word.clone(), board, next_path);
         if final_path.is_some() {
            return final_path;
         }
      }
   }

   None
}

fn heuristic_solution(dict: &Trie<BString, ()>, board: &GameGrid) -> Option<SolutionState> {
   let mut visited: u16 = 0xF000;
   let mut words: Vec<String> = vec![];

   let mut position = None;

   let mut flat_dict_again: Vec<Vec<char>> = dict.keys().map(|x| x.as_str().chars().collect()).collect();

   while visited != 0xFFFF {
      flat_dict_again.sort_by_key(|x| {
         let mut chars: Vec<char> = x.clone();
         chars.sort();
         chars.dedup();
         chars.len()
      });

      if let Some(i) = flat_dict_again.iter().enumerate().rev().find(|x| word_can_be_made(position, x.1.clone(), board)).map(|x| x.0) {
         let greedy_best_next_word = flat_dict_again.swap_remove(i);
         words.push(greedy_best_next_word.iter().collect());
         let path = word_path(position, greedy_best_next_word, board, vec![]).unwrap();
         for dest in path.iter().copied() {
            visited |= 1 << dest as u16;
         }
         position = Some(*path.last().unwrap());
      } else {
         return None;
      }
   }

   let total_letters = words.iter().map(|x| x.chars().count()).sum();
   Some(SolutionState { cur_sequence: String::new(), words, position: position.unwrap(), visited: 0xFFFF, last_char: '\0', total_letters })
}

impl SolverState {
   pub fn setup(board: GameGrid) -> SolverState {
      let dict = DICTIONARY.get_or_init(|| {
         let mut dict = Vec::new();
         let f = include_str!("words_easy.txt");
         for line in f.lines() {
            if line.len() < 3 {
               continue;
            }
            dict.push(line.to_string());
         }
         dict
      });

      let mut filtered_dict = Trie::new();
      for entry in dict.iter() {
         if word_can_be_made(None, entry.chars().collect(), &board) {
            filtered_dict.insert_str(entry, ());
         }
      }

      SolverState {
         dict: filtered_dict,
         best_solution: None,
         board,
         stack: vec![SolutionState {
            cur_sequence: String::new(),
            words: vec![],
            position: 0,
            visited: 0xF000,
            total_letters: 0,
            last_char: '\0',
         }],
         attempted_heuristic: false,
      }
   }

   pub fn next_solution(&mut self) -> Option<String> {
      if !self.attempted_heuristic {
         self.attempted_heuristic = true;
         if let Some(h) = heuristic_solution(&self.dict, &self.board) {
            let solution_str = h.words.join(", ");
            self.best_solution = Some(h);
            return Some(solution_str);
         }
      }

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

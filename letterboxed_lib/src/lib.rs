use std::sync::OnceLock;

use qp_trie::wrapper::BString;
use qp_trie::Trie;

#[derive(Clone)]
struct SolutionState {
   cur_sequence: String,
   words: Vec<String>,
   position: usize,
   visited: u16,
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

// Not designed to return correct answers when making multiple words in succession!
fn word_can_be_made(position: Option<usize>, mut remaining_word: Vec<char>, board: &GameGrid) -> bool {
   // we pop. if the word can be made backwards, it can be made forwards
   // (again, this function is designed to check whether this word can be made on the board _at all_)
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

fn word_path(position: Option<usize>, remaining_word: Vec<char>, board: &GameGrid, visited: u16) -> Option<Vec<usize>> {
   if position.is_some() {
      word_path_inner(position, remaining_word[1..].to_vec(), board, vec![], visited).map(|x| x.0)
   } else {
      word_path_inner(position, remaining_word, board, vec![], visited).map(|x| x.0)
   }
}

fn word_path_inner(position: Option<usize>, mut remaining_word: Vec<char>, board: &GameGrid, path: Vec<usize>, visited: u16) -> Option<(Vec<usize>, u16)> {
   let next_letter = if remaining_word.is_empty() {
      return Some((path, visited))
   } else {
      remaining_word.remove(0)
   };

   let reachable_letters = if let Some(pos) = position {
      reachable(pos)
   } else {
      &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
   };

   let mut possible_paths = vec![];
   for possible_dest in reachable_letters.iter() {
      if board[*possible_dest] == next_letter {
         let mut next_path = path.clone();
         next_path.reserve_exact(1);
         next_path.push(*possible_dest);

         if let Some(res) = word_path_inner(Some(*possible_dest), remaining_word.clone(), board, next_path, visited | (1 << *possible_dest as u16)) {
            possible_paths.push(res);
         }
      }
   }

   possible_paths.iter().max_by_key(|x| x.1.count_ones()).cloned()
}

fn heuristic_solution(dict: &Trie<BString, ()>, board: &GameGrid) -> Option<SolutionState> {
   let mut visited: u16 = 0xF000;
   let mut words: Vec<String> = vec![];

   let mut position = None;

   let flat_dict_again: Vec<Vec<char>> = dict.keys().map(|x| x.as_str().chars().collect()).collect();

   while visited != 0xFFFF {
      let visited_before = visited;
   
      let greedy_best_next = flat_dict_again.iter().filter(|x| {
         if let Some(last_char) = words.last().map(|w| w.chars().last().unwrap()) {
            if x[0] != last_char {
               return false;
            }
         }

         true
      }).max_by_key(|x| {
         let mut trial_visited = visited;
         let path = word_path(position, x.to_vec(), board, trial_visited).unwrap_or_default();
         for dest in path.iter().copied() {
            trial_visited |= 1 << dest as u16;
         }
         (trial_visited.count_ones(), std::cmp::Reverse(x.len()))
      });

      if let Some(w) = greedy_best_next {
         if let Some(path) = word_path(position, w.to_vec(), board, visited) {
            for dest in path.iter().copied() {
               visited |= 1 << dest as u16;
            }
   
            words.push(w.iter().collect());
            position = Some(*path.last().unwrap());
         }
      }

      if visited_before == visited {
         // We did not make progress; bail
         return None;
      }
   }

   Some(SolutionState { cur_sequence: String::new(), words, position: position.unwrap(), visited: 0xFFFF })
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
                  let bstl = bs.words.iter().map(|x| x.chars().count()).sum::<usize>();
                  let stl = solution.words.iter().map(|x| x.chars().count()).sum::<usize>() + solution.cur_sequence.chars().count();
                  if bstl <= stl {
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
            let last_char = new_solution.cur_sequence.chars().last().unwrap();
            new_solution.words.push(std::mem::take(&mut new_solution.cur_sequence));
            if new_solution.visited == 0xFFFF {
               let solution_str = new_solution.words.join(", ");
               self.best_solution = Some(new_solution);
               return Some(solution_str);
            }
            new_solution.cur_sequence.push(last_char);
            self.stack.push(new_solution);
         }

         let reachable_letters = if solution.words.is_empty() && solution.cur_sequence.is_empty() {
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
         } else {
            reachable(solution.position)
         };

         for dest in reachable_letters {
            let letter = self.board[*dest];
            let mut new_solution = solution.clone();
            new_solution.visited |= 1 << *dest as u16;
            new_solution.cur_sequence.push(letter);
            new_solution.position = *dest;
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

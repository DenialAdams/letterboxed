use qp_trie::{Trie, wrapper::BString};

#[derive(Clone)]
struct SolutionState {
    cur_sequence: String,
    words: Vec<String>,
    position: usize,
    visited: u16,
    last_char: char,
    total_letters: usize,
}

type GameGrid = [char; 12];

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

fn main() {
    let mut dict = Trie::new();
    {
        let f = std::fs::read_to_string("words_easy.txt").unwrap();
        for line in f.lines() {
            if line.len() < 3 {
                continue;
            }
            dict.insert_str(line, ());
        }
    }
    let grid = ['I', 'N', 'W', 'S', 'C', 'O', 'A', 'H', 'R', 'D', 'K', 'E'];
    let mut best_solution = None;
    go(&dict, grid, &mut best_solution);
}

fn go(dict: &Trie<BString, ()>, board: GameGrid, best_solution: &mut Option<SolutionState>) {
    let mut stack = vec![SolutionState {
        cur_sequence: String::new(),
        words: vec![],
        position: 0,
        visited: 0xF000,
        total_letters: 0,
        last_char: '\0',
    }];

    while let Some(solution) = stack.pop() {
        // Bound
        if let Some(bs) = best_solution.as_ref() {
            match bs.words.len().cmp(&(solution.words.len() + 1)) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Equal => {
                    if bs.total_letters <= solution.total_letters {
                        continue;
                    }    
                },
                std::cmp::Ordering::Greater => (),
            }
        }

        if dict.subtrie_str(&solution.cur_sequence).is_empty() {
            continue;
        }

        if dict.contains_key_str(&solution.cur_sequence) && !solution.words.contains(&solution.cur_sequence) {
            let mut new_solution = solution.clone();
            new_solution.words.push(std::mem::take(&mut new_solution.cur_sequence));
            new_solution.cur_sequence.push(new_solution.last_char);
            if new_solution.visited == 0xFFFF {
                println!("{}", new_solution.words.join(", "));
                *best_solution = Some(new_solution);
                continue;
            }
            stack.push(new_solution);
        }

        let reachable_letters = if solution.total_letters == 0 {
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        } else {
            reachable(solution.position)
        };

        for dest in reachable_letters {
            let letter = board[*dest];
            let mut new_solution = solution.clone();
            new_solution.visited |= 1 << *dest as u16;
            new_solution.cur_sequence.push(letter);
            new_solution.total_letters += 1;
            new_solution.position = *dest;
            new_solution.last_char = letter;
            stack.push(new_solution);
        }
    }
}

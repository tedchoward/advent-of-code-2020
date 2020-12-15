use std::collections::HashMap;

struct MemoryGame {
    current_index: usize,
    last_number_spoken: usize,
    reverse_index: HashMap<usize, [usize; 2]>,
}

impl MemoryGame {
    fn new() -> Self {
        Self {
            current_index: 0,
            last_number_spoken: 0,
            reverse_index: HashMap::new(),
        }
    }

    fn speak_number(&mut self, number: usize) {
        self.last_number_spoken = number;
        self.current_index += 1;

        match self.reverse_index.get_mut(&number) {
            Some(indicies) => {
                indicies[0] = indicies[1];
                indicies[1] = self.current_index;
            }
            None => {
                self.reverse_index.insert(number, [self.current_index, self.current_index]);
            }
        }

    }

    fn get_next_number(&self) -> usize {
        let indicies = &self.reverse_index[&self.last_number_spoken];
        indicies[1] - indicies[0]
    }
}

fn puzzle_1() -> usize {
    let input = String::from("2,20,0,4,1,17");

    let mut memory_game = MemoryGame::new();

    for number in input.split(',').map(|n| n.parse().unwrap()) {
        memory_game.speak_number(number);
    }

    for _ in memory_game.current_index..2020 {
        let next_number = memory_game.get_next_number();
        memory_game.speak_number(next_number);
    }

    memory_game.last_number_spoken
}

fn puzzle_2() -> usize {
    let input = String::from("2,20,0,4,1,17");

    let mut memory_game = MemoryGame::new();

    for number in input.split(',').map(|n| n.parse().unwrap()) {
        memory_game.speak_number(number);
    }

    for _ in memory_game.current_index..30000000 {
        let next_number = memory_game.get_next_number();
        memory_game.speak_number(next_number);
    }

    memory_game.last_number_spoken
}

fn main() {
    let start = std::time::Instant::now();
    let result = puzzle_1();
    let duration = start.elapsed().as_millis();
    println!("Puzzle 1 result: {}, took {}ms", result, duration);

    let start = std::time::Instant::now();
    let result = puzzle_2();
    let duration = start.elapsed().as_millis();
    println!("Puzzle 2 result: {}, took {}ms", result, duration);
}

fn load_input() -> Vec<u32> {
    let input = std::fs::read_to_string("dec10/src/input.txt").expect("Unable to read input file");

    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

fn puzzle_1() -> u32 {
    // let mut input = [
    //     28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
    //     17, 7, 9, 4, 2, 34, 10, 3,
    // ].to_vec();

    let mut input = load_input();

    input.insert(input.len() - 1, 0);

    input.sort();
    let last_index = input.len() - 1;

    let mut ones = 0;
    let mut threes = 0;

    for (index, num) in input.iter().enumerate() {
        let next = if index == last_index {
            num + 3
        } else {
            input[index + 1]
        };

        let diff = next - num;

        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
    }

    ones * threes
}

struct ArrangementCounter {
    arrangements: std::collections::HashMap<u32, u64>,
}

impl ArrangementCounter {
    fn new() -> Self {
        Self {
            arrangements: std::collections::HashMap::new(),
        }
    }

    fn get(&self, voltage: &u32) -> u64 {
        match self.arrangements.get(voltage) {
            Some(arrangements) => *arrangements,
            None => 1,
        }
    }

    fn set(&mut self, voltage: u32, arrangements: u64) {
        self.arrangements.insert(voltage, arrangements);
    }
}

fn puzzle_2() -> u64 {
    let mut input = load_input();
    input.insert(0, 0);
    input.sort();
    input.insert(input.len(), input.last().unwrap() + 3);

    let length = input.len();
    let mut counter = ArrangementCounter::new();

    for (index, adapter) in input.iter().enumerate().rev() {

        if index + 1 < length {
            let child = &input[index + 1];
            let child_count = counter.get(child);
            counter.set(*adapter, child_count);
        }

        if index + 2 < length {
            let child = &input[index + 2];
            if child - adapter <= 3 {
                let child_count = counter.get(child);
                let count = counter.get(adapter);
                counter.set(*adapter, count + child_count);
            }
        }

        if index + 3 < length {
            let child = &input[index + 3];
            if child - adapter <= 3 {
                let child_count = counter.get(child);
                let count = counter.get(adapter);
                counter.set(*adapter, count + child_count);
            }
        }
    }

    counter.get(&0)
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2 output: {}", result);
}

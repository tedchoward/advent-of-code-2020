fn load_input() -> Vec<u64> {
    let input = std::fs::read_to_string("dec09/src/input.txt").expect("Unable to read input file");

    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

fn validate_number(preamble: &[u64], number: u64) -> bool {
    for i in 0..(preamble.len() - 1) {
        for j in (i + 1)..preamble.len() {
            if number == (preamble[i] + preamble[j]) {
                return true
            }
        }
    }

    false
}

fn puzzle_1() -> u64 {
    // let input = [
    //     35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    // ];

    let input = load_input();

    let preamble_size = 25;

    for i in preamble_size..(input.len()) {
        let number = input[i];

        if !validate_number(&input[(i - preamble_size)..i], number) {
            return number;
        }
    }

    panic!("All numbers are valid.");
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}

const PREAMBLE_SIZE: usize = 25;

fn load_input() -> Vec<u64> {
    let input = std::fs::read_to_string("dec09/src/input.txt").expect("Unable to read input file");

    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

fn validate_number(preamble: &[u64], number: u64) -> bool {
    for i in 0..(preamble.len() - 1) {
        for j in (i + 1)..preamble.len() {
            if number == (preamble[i] + preamble[j]) {
                return true;
            }
        }
    }

    false
}

fn find_invalid_number(input: &[u64]) -> u64 {
    for i in PREAMBLE_SIZE..(input.len()) {
        let number = input[i];

        if !validate_number(&input[(i - PREAMBLE_SIZE)..i], number) {
            return number;
        }
    }

    panic!("All numbers are valid.");
}

fn puzzle_1() -> u64 {
    let input = load_input();

    find_invalid_number(&input)
}

fn puzzle_2() -> u64 {
    let input = load_input();

    let invalid_number = find_invalid_number(&input);

    for (index, num1) in input.iter().enumerate() {
        let mut sum = *num1;
        let mut min = sum;
        let mut max = sum;

        for num2 in input.iter().skip(index + 1) {
            let num2 = *num2;
            if num2 < min {
                min = num2;
            }

            if num2 > max {
                max = num2;
            }

            sum += num2;

            if sum == invalid_number {
                return min + max;
            }

            if sum > invalid_number {
                break;
            }
        }
    }

    panic!("Encryption weakness not found.");
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2 output: {}", result);
}

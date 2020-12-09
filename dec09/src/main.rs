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

fn find_contiguous_slice(input: &[u64], value: u64) -> &[u64] {
    let input_size = input.len();

    let mut start = 0;
    let mut end = 1;
    let mut sum = input[start] + input[end];

    while end < input_size {
        if sum > value {
            sum -= input[start];
            start += 1;
        } else if sum == value {
            return &input[start..=end];
        } else {
            end += 1;
            sum += input[end];
        }
    }

    panic!("No slice found that adds up to {}", value);
}

fn puzzle_1() -> u64 {
    let input = load_input();

    find_invalid_number(&input)
}

fn puzzle_2() -> u64 {
    let input = load_input();

    let invalid_number = find_invalid_number(&input);

    let contiguous_slice = find_contiguous_slice(&input, invalid_number);

    let min = contiguous_slice.iter().min().unwrap();
    let max = contiguous_slice.iter().max().unwrap();

    min + max
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2 output: {}", result);
}

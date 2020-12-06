use std::collections::HashSet;

fn load_input() -> String {
    std::fs::read_to_string("dec06/src/input.txt").expect("Unable to read input file")
}

fn puzzle_1() -> usize {
    //     let input = String::from(
    //         "abc

    // a
    // b
    // c

    // ab
    // ac

    // a
    // a
    // a
    // a

    // b",
    //     );

    let input = load_input();

    input
        .split("\n\n")
        .map(|group| {
            group
                .replace("\n", "")
                .chars()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}

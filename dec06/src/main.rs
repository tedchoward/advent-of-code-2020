use std::collections::HashSet;

fn load_input() -> String {
    std::fs::read_to_string("dec06/src/input.txt").expect("Unable to read input file")
}

fn puzzle_1() -> usize {
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

fn puzzle_2() -> usize {
    let input = load_input();

    input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|p| p.chars().collect::<HashSet<char>>())
                .fold(None, |acc, person| -> Option<HashSet<char>> {
                    if let Some(prev) = acc {
                        Some(prev.intersection(&person).cloned().collect())
                    } else {
                        Some(person)
                    }
                })
                .unwrap()
                .len()
        })
        .sum()
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2 output: {}", result);
}

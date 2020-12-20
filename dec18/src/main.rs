mod advanced_parser;
mod expr;
mod parser;
mod scanner;

use advanced_parser::AdvancedParser;
use parser::Parser;
use scanner::Scanner;

fn load_input() -> String {
    std::fs::read_to_string("dec18/src/input.txt").expect("Unable to read input file")
}

fn puzzle_1() -> u64 {
    let input = load_input();

    let sum: u64 = input
        .split('\n')
        .map(|expr| {
            let mut scanner = Scanner::new(expr);
            let tokens = scanner.scan_tokens();
            let mut parser = Parser::new(&tokens);
            let expr = parser.parse();
            expr.evaluate()
        })
        .sum();

    sum
}

fn puzzle_2() -> u64 {
    let input = load_input();

    let sum: u64 = input
        .split('\n')
        .map(|expr| {
            let mut scanner = Scanner::new(expr);
            let tokens = scanner.scan_tokens();
            let mut parser = AdvancedParser::new(&tokens);
            let expr = parser.parse();
            expr.evaluate()
        })
        .sum();

    sum
}

fn main() {
    let start = std::time::Instant::now();
    let result = puzzle_1();
    let elapsed = start.elapsed().as_millis();
    println!("Puzzle 1 result: {}, took {}ms", result, elapsed);

    let start = std::time::Instant::now();
    let result = puzzle_2();
    let elapsed = start.elapsed().as_millis();
    println!("Puzzle 2 result: {}, took {}ms", result, elapsed);
}

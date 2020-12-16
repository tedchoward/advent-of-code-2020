#[macro_use]
extern crate lazy_static;

use std::ops::RangeInclusive;
use regex::Regex;

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}

fn load_input() -> String {
    std::fs::read_to_string("dec16/src/input.txt").expect("Unable to read input file")
}

struct Rule {
    field_name: String,
    range_1: RangeInclusive<u32>,
    range_2: RangeInclusive<u32>,
}

impl Rule {
    fn new(rule: &str)  -> Self {
        if let Some(captures) = RULE_REGEX.captures(rule) {
            let field_name = String::from(&captures[1]);
            let a: u32 = captures[2].parse().unwrap();
            let b: u32 = captures[3].parse().unwrap();
            let range_1 = a..=b;

            let a: u32 = captures[4].parse().unwrap();
            let b: u32 = captures[5].parse().unwrap();
            let range_2 = a..=b;

            Self {
                field_name,
                range_1,
                range_2,
            }
        } else {
            panic!("Unable to parse rule: {}", rule);
        }
    }

    fn is_invalid_value(&self, value: &u32) -> bool {
        !self.range_1.contains(value) && !self.range_2.contains(value)
    }
}

fn puzzle_1() -> u32 {
//     let input = String::from("class: 1-3 or 5-7
// row: 6-11 or 33-44
// seat: 13-40 or 45-50

// your ticket:
// 7,1,14

// nearby tickets:
// 7,3,47
// 40,4,50
// 55,2,20
// 38,6,12");

    let input = load_input();

    let rules: Vec<Rule> = input.split("\n\n").nth(0).unwrap().split('\n').map(Rule::new).collect();

    let scanning_error_rate: u32 = input.split("\n\n").nth(2).unwrap().split('\n').skip(1).flat_map(|ticket| {
        ticket.split(',').map(|val| val.parse::<u32>().unwrap()).filter(|value| {
            let mut invalid = true;

            for rule in &rules {
                if !rule.is_invalid_value(value) {
                    invalid = false;
                    break;
                }
            }

            invalid
        })
    }).sum();

    scanning_error_rate
}

fn main() {
    let start = std::time::Instant::now();
    let result = puzzle_1();
    let duration = start.elapsed().as_millis();
    println!("Puzzle 1 result: {}, took {}ms", result, duration);
}

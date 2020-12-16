#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashSet;
use std::ops::RangeInclusive;

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}

fn load_input() -> String {
    std::fs::read_to_string("dec16/src/input.txt").expect("Unable to read input file")
}

#[derive(Clone, Debug)]
struct Rule {
    field_name: String,
    range_1: RangeInclusive<u64>,
    range_2: RangeInclusive<u64>,
    possible_sequences: HashSet<u64>,
    only_possible_sequence: Option<u64>,
}

impl Rule {
    fn new(rule: &str) -> Self {
        if let Some(captures) = RULE_REGEX.captures(rule) {
            let field_name = String::from(&captures[1]);
            let a: u64 = captures[2].parse().unwrap();
            let b: u64 = captures[3].parse().unwrap();
            let range_1 = a..=b;

            let a: u64 = captures[4].parse().unwrap();
            let b: u64 = captures[5].parse().unwrap();
            let range_2 = a..=b;

            Self {
                field_name,
                range_1,
                range_2,
                possible_sequences: HashSet::new(),
                only_possible_sequence: None,
            }
        } else {
            panic!("Unable to parse rule: {}", rule);
        }
    }

    fn is_valid_value(&self, value: &u64) -> bool {
        self.range_1.contains(value) || self.range_2.contains(value)
    }
}

fn puzzle_1() -> u64 {
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

    let rules: Vec<Rule> = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .split('\n')
        .map(Rule::new)
        .collect();

    let scanning_error_rate: u64 = input
        .split("\n\n")
        .nth(2)
        .unwrap()
        .split('\n')
        .skip(1)
        .flat_map(|ticket| {
            ticket
                .split(',')
                .map(|val| val.parse::<u64>().unwrap())
                .filter(|value| {
                    let mut invalid = true;

                    for rule in &rules {
                        if rule.is_valid_value(value) {
                            invalid = false;
                            break;
                        }
                    }

                    invalid
                })
        })
        .sum();

    scanning_error_rate
}

fn puzzle_2() -> u64 {
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

    let mut rules: Vec<Rule> = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .split('\n')
        .map(Rule::new)
        .collect();
    let field_count = rules.len() as u64;
    for rule in &mut rules {
        rule.possible_sequences = (0..field_count).collect();
    }

    let valid_tickets: Vec<Vec<u64>> = input
        .split("\n\n")
        .nth(2)
        .unwrap()
        .split('\n')
        .skip(1)
        .map(|ticket| {
            ticket
                .split(',')
                .map(|val| val.parse::<u64>().unwrap())
                .filter(|value| {
                    let mut valid = false;

                    for rule in &rules {
                        if rule.is_valid_value(value) {
                            valid = true;
                            break;
                        }
                    }

                    valid
                })
                .collect()
        })
        .collect();

    'outer: for ticket in valid_tickets {
        for (index, value) in ticket.iter().enumerate() {
            let mut validated_rules = 0;
            for rule in rules.iter_mut() {
                if rule.possible_sequences.len() > 1 {
                    if !rule.is_valid_value(&value) {
                        rule.possible_sequences.remove(&(index as u64));
                        if rule.possible_sequences.len() == 1 {
                            let sequence = rule.possible_sequences.iter().nth(0).unwrap();
                            rule.only_possible_sequence = Some(*sequence);
                        }
                    }
                } else {
                    validated_rules += 1;
                }
            }

            if validated_rules == field_count {
                println!("break 'outer");
                break 'outer;
            }
        }
    }

    'remove_dups: loop {
        let validated_rules: Vec<(String, u64)> = rules
            .iter()
            .cloned()
            .filter(|r| r.only_possible_sequence != None)
            .map(|r| (r.field_name.clone(), r.only_possible_sequence.unwrap()))
            .collect();

        if validated_rules.len() == field_count as usize {
            break 'remove_dups;
        }

        for (rule_name, rule_sequence) in validated_rules {
            // let position = validated_rule.possible_sequences.iter().nth(0).unwrap();

            for rule in rules.iter_mut() {
                if rule.field_name != *rule_name {
                    rule.possible_sequences.remove(&rule_sequence);
                    if rule.possible_sequences.len() == 1 {
                        let sequence = rule.possible_sequences.iter().nth(0).unwrap();
                        rule.only_possible_sequence = Some(*sequence);
                    }
                }
            }
        }
    }

    rules.sort_by(|a, b| {
        a.only_possible_sequence
            .partial_cmp(&b.only_possible_sequence)
            .unwrap()
    });

    let departure_product: u64 = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u64>().unwrap())
        .enumerate()
        .map(|(index, value)| {
            let field_name = rules[index].field_name.clone();
            (field_name, value)
        })
        .filter(|(field, _)| field.starts_with("departure"))
        .fold(1, |acc, (_, value)| acc * value);

    departure_product
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

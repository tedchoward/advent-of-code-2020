#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref LITERAL_RULE: Regex = Regex::new("\"(.)\"$").unwrap();
}

fn load_input() -> String {
    std::fs::read_to_string("dec19/src/input.txt").expect("Unable to read input file")
}

#[derive(Debug)]
enum Rule {
    Literal(String),
    Unary(Vec<usize>),
    Binary(Vec<usize>, Vec<usize>),
}

impl Rule {
    fn to_string(&self, all_rules: &HashMap<usize, Rule>) -> String {
        match self {
            Rule::Literal(literal) => literal.clone(),
            Rule::Unary(left) => {
                let rule_str: String = left
                    .iter()
                    .filter_map(|rule_num| all_rules.get(&rule_num))
                    .map(|rule| rule.to_string(all_rules))
                    .collect();
                rule_str
            }
            Rule::Binary(left, right) => {
                let left_str: String = left
                    .iter()
                    .filter_map(|rule_num| all_rules.get(&rule_num))
                    .map(|rule| rule.to_string(all_rules))
                    .collect();
                let right_str: String = right
                    .iter()
                    .filter_map(|rule_num| all_rules.get(&rule_num))
                    .map(|rule| rule.to_string(all_rules))
                    .collect();
                format!("({}|{})", left_str, right_str)
            }
        }
    }
}

fn puzzle_1() -> usize {
    let input = load_input();
    let mut rules: HashMap<usize, Rule> = HashMap::new();

    let rule_definitions = input.split("\n\n").nth(0).unwrap();
    for rule_def in rule_definitions.split('\n') {
        let parts: Vec<&str> = rule_def.split(": ").collect();
        let rule_num: usize = parts[0].parse().unwrap();

        if let Some(captures) = LITERAL_RULE.captures(parts[1]) {
            let literal = &captures[1];
            rules.insert(rule_num, Rule::Literal(String::from(literal)));
        } else if parts[1].contains('|') {
            let binary_parts: Vec<&str> = parts[1].split(" | ").collect();
            let left: Vec<usize> = binary_parts[0]
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect();
            let right: Vec<usize> = binary_parts[1]
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect();
            rules.insert(rule_num, Rule::Binary(left, right));
        } else {
            let left: Vec<usize> = parts[1].split(' ').map(|n| n.parse().unwrap()).collect();
            rules.insert(rule_num, Rule::Unary(left));
        }
    }

    // println!("{:#?}", rules);

    let rule_0 = rules.get(&0).unwrap();
    let rule_0_string = rule_0.to_string(&rules);
    let re_string = format!("^{}$", rule_0_string);

    // println!("{:#?}", rule_0.to_string(&rules));

    let re = Regex::new(&re_string).unwrap();

    let messages = input.split("\n\n").nth(1).unwrap();
    let valid_message_count = messages
        .split('\n')
        .filter(|message| re.is_match(message))
        .count();

    valid_message_count
}

fn puzzle_2() -> usize {
    let input = load_input();
    let mut rules: HashMap<usize, Rule> = HashMap::new();

    let rule_definitions = input.split("\n\n").nth(0).unwrap();
    for rule_def in rule_definitions.split('\n') {
        let parts: Vec<&str> = rule_def.split(": ").collect();
        let rule_num: usize = parts[0].parse().unwrap();

        if let Some(captures) = LITERAL_RULE.captures(parts[1]) {
            let literal = &captures[1];
            rules.insert(rule_num, Rule::Literal(String::from(literal)));
        } else if parts[1].contains('|') {
            let binary_parts: Vec<&str> = parts[1].split(" | ").collect();
            let left: Vec<usize> = binary_parts[0]
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect();
            let right: Vec<usize> = binary_parts[1]
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect();
            rules.insert(rule_num, Rule::Binary(left, right));
        } else {
            let left: Vec<usize> = parts[1].split(' ').map(|n| n.parse().unwrap()).collect();
            rules.insert(rule_num, Rule::Unary(left));
        }
    }

    // 8: 42 | 42 8 (aka a series of 42s)
    // 11: 42 31 | 42 11 31 (aka a series of 42s followed by a series of 31s of the same length)
    // 0: 8 11 (aka a series of 42s followed by a shorter series of 31s)
    let rule_42_string = rules.get(&42).unwrap().to_string(&rules);
    let rule_42_re = Regex::new(&rule_42_string).unwrap();

    let rule_31_string = rules.get(&31).unwrap().to_string(&rules);
    let rule_31_re = Regex::new(&rule_31_string).unwrap();

    let re_string = format!("^(?P<rule_42>({})+)(?P<rule_31>({})+)$", rule_42_re, rule_31_re);
    let re = Regex::new(&re_string).unwrap();

    let messages = input.split("\n\n").nth(1).unwrap();
    let valid_message_count = messages
        .split('\n')
        .filter(|message| {
            if let Some(captures) = re.captures(message) {
                let first = captures.name("rule_42").unwrap().as_str();
                let rule_42_count = rule_42_re.captures_iter(first).count();
                let second = captures.name("rule_31").unwrap().as_str();
                let rule_31_count = rule_31_re.captures_iter(second).count();

                if rule_42_count > rule_31_count {
                    return true;
                }
            }

            false
        })
        .count();

    valid_message_count
}

fn main() {
    let start = std::time::Instant::now();
    let result = puzzle_1();
    let elapsed = start.elapsed().as_millis();
    println!("Puzzle 1 result: {}, duration {}ms", result, elapsed);

    let start = std::time::Instant::now();
    let result = puzzle_2();
    let elapsed = start.elapsed().as_millis();
    println!("Puzzle 2 result: {}, duration {}ms", result, elapsed);
}

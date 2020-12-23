#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use regex::Regex;

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
//     let input = String::from(
//         "0: 4 1 5
// 1: 2 3 | 3 2
// 2: 4 4 | 5 5
// 3: 4 5 | 5 4
// 4: \"a\"
// 5: \"b\"

// ababbb
// bababa
// abbbab
// aaabbb
// aaaabbb",
//     );

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

fn main() {
    let start = std::time::Instant::now();
    let result = puzzle_1();
    let elapsed = start.elapsed().as_millis();
    println!("Puzzle 1 result: {}, took {}ms", result, elapsed);
}

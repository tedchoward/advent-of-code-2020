#[macro_use]
extern crate lazy_static;

mod insert_all;

use crate::insert_all::InsertAll;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn load_input() -> String {
    std::fs::read_to_string("dec07/src/input.txt").expect("Unable to read input file")
}

#[derive(Debug)]
struct NestedBags {
    child_to_parents: HashMap<String, HashSet<String>>,
}

impl NestedBags {
    fn new() -> Self {
        Self {
            child_to_parents: HashMap::new(),
        }
    }

    fn insert<S: ToString>(&mut self, child_color: S, parent_color: S) {
        let child_color = child_color.to_string();
        let parent_color = parent_color.to_string();
        if let Some(parents) = self.child_to_parents.get_mut(&child_color) {
            parents.insert(parent_color);
        } else {
            let mut parents = HashSet::new();
            parents.insert(parent_color);
            self.child_to_parents.insert(child_color, parents);
        }
    }

    fn count_parents(&self, child_color: &str) -> usize {
        self.find_parents(child_color).len()
    }

    fn find_parents(&self, child_color: &str) -> HashSet<&String> {
        let mut result = HashSet::new();
        if let Some(parents) = self.child_to_parents.get(child_color) {
            for parent in parents {
                result.insert(parent);
                result.insert_all(self.find_parents(parent));
            }
        }

        result
    }
}

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^([a-z ]+) bags contain ([^.]+).$").unwrap();
    static ref CHILD_COLOR_RE: Regex = Regex::new(r"\d ([a-z ]+) bags?").unwrap();
}

fn puzzle_1() -> usize {
    //     let input = String::from(
    //         "light red bags contain 1 bright white bag, 2 muted yellow bags.
    // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    // bright white bags contain 1 shiny gold bag.
    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    // dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    // faded blue bags contain no other bags.
    // dotted black bags contain no other bags.",
    //     );

    let input = load_input();

    let mut nested_bags = NestedBags::new();

    for rule in input.split('\n') {
        let captures = RULE_RE.captures(rule).unwrap();
        let parent_color = String::from(&captures[1]);
        let child_captures = &captures[2];

        for c in CHILD_COLOR_RE.captures_iter(child_captures) {
            let child_color = String::from(&c[1]);
            nested_bags.insert(child_color, parent_color.clone());
        }
    }

    nested_bags.count_parents("shiny gold")
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}

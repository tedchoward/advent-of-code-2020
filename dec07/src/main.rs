#[macro_use]
extern crate lazy_static;

mod insert_all;

use crate::insert_all::InsertAll;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn load_input() -> String {
    std::fs::read_to_string("dec07/src/input.txt").expect("Unable to read input file")
}

struct NestedBags {
    child_to_parents: HashMap<String, HashSet<String>>,
    parent_to_children: HashMap<String, HashMap<String, usize>>,
}

impl NestedBags {
    fn new() -> Self {
        Self {
            child_to_parents: HashMap::new(),
            parent_to_children: HashMap::new(),
        }
    }

    fn insert<S: ToString>(&mut self, child_count: usize, child_color: &S, parent_color: &S) {
        self.insert_child_to_parent(child_color, parent_color);
        self.insert_parent_to_children(child_count, child_color, parent_color);
    }

    fn insert_child_to_parent<S: ToString>(&mut self, child_color: &S, parent_color: &S) {
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

    fn insert_parent_to_children<S: ToString>(
        &mut self,
        child_count: usize,
        child_color: &S,
        parent_color: &S,
    ) {
        let child_color = child_color.to_string();
        let parent_color = parent_color.to_string();
        if let Some(bag_rules) = self.parent_to_children.get_mut(&parent_color) {
            bag_rules.insert(child_color, child_count);
        } else {
            let mut bag_rules = HashMap::new();
            bag_rules.insert(child_color, child_count);
            self.parent_to_children.insert(parent_color, bag_rules);
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

    fn count_children(&self, bag_color: &str) -> usize {
        let mut count = 0;

        if let Some(bag_rules) = self.parent_to_children.get(bag_color) {
            for (bag_color, bag_count) in bag_rules {
                count += bag_count;
                count += bag_count * self.count_children(bag_color);
            }
        }

        count
    }
}

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^([a-z ]+) bags contain ([^.]+).$").unwrap();
    static ref CHILD_COLOR_RE: Regex = Regex::new(r"(\d) ([a-z ]+) bags?").unwrap();
}

fn both_puzzles() -> (usize, usize) {
    let input = load_input();

    let mut nested_bags = NestedBags::new();

    for rule in input.split('\n') {
        let captures = RULE_RE.captures(rule).unwrap();
        let parent_color = String::from(&captures[1]);
        let child_captures = &captures[2];

        for c in CHILD_COLOR_RE.captures_iter(child_captures) {
            let child_count = usize::from_str_radix(&c[1], 10).unwrap();
            let child_color = String::from(&c[2]);
            nested_bags.insert(child_count, &child_color, &parent_color);
        }
    }

    let parent_count = nested_bags.count_parents("shiny gold");
    let child_count = nested_bags.count_children("shiny gold");

    (parent_count, child_count)
}

fn main() {
    let (puzzle_1, puzzle_2) = both_puzzles();
    println!("Puzzle 1 output: {}", puzzle_1);
    println!("Puzzle 2 output: {}", puzzle_2);
}

use regex::Regex;
use std::collections::{HashMap, HashSet};

fn load_input() -> String {
    std::fs::read_to_string("dec04/src/input.txt").expect("Unable to read input file")
}

fn puzzle_1() -> u32 {
    let input = load_input();
    let mut valid_passports = 0;
    let required_fields: HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();

    for passport in input.split("\n\n") {
        let fields: HashSet<&str> = passport
            .split_ascii_whitespace()
            .map(|field| field.split(':').nth(0).unwrap())
            .collect();

        let differences = required_fields.difference(&fields);
        if differences.count() == 0 {
            valid_passports += 1;
        }
    }

    valid_passports
}

type Callback<'a> = Box<(dyn Fn(&'a str) -> bool + 'static)>;

fn make_callback<'a, F>(f: F) -> Callback<'a>
where
    F: Fn(&'a str) -> bool + 'static,
{
    Box::new(f) as Callback
}

struct FieldDefinition<'a> {
    name: String,
    validator: Callback<'a>,
}

impl<'a> FieldDefinition<'a> {
    fn new<S: ToString>(name: S, validator: Callback<'a>) -> Self {
        Self {
            name: name.to_string(),
            validator,
        }
    }

    fn validate(&self, value: &'a str) -> bool {
        (self.validator)(value)
    }
}

fn puzzle_2() -> u32 {
    let input = load_input();
    let mut valid_passports = 0;
    let eye_colors: HashSet<&'static str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .cloned()
        .collect();

    let required_fields = vec![
        FieldDefinition::new(
            "byr",
            make_callback(|value| match value.parse::<u32>() {
                Ok(byr) => byr >= 1920 && byr <= 2002,
                Err(_) => false,
            }),
        ),
        FieldDefinition::new(
            "iyr",
            make_callback(|value| match value.parse::<u32>() {
                Ok(iyr) => iyr >= 2010 && iyr <= 2020,
                Err(_) => false,
            }),
        ),
        FieldDefinition::new(
            "eyr",
            make_callback(|value| match value.parse::<u32>() {
                Ok(eyr) => eyr >= 2020 && eyr <= 2030,
                Err(_) => false,
            }),
        ),
        FieldDefinition::new(
            "hgt",
            make_callback(|value| {
                if value.contains("cm") {
                    return match value.split("cm").nth(0).unwrap().parse::<u32>() {
                        Ok(hgt) => hgt >= 150 && hgt <= 193,
                        Err(_) => false,
                    };
                }

                if value.contains("in") {
                    return match value.split("in").nth(0).unwrap().parse::<u32>() {
                        Ok(hgt) => hgt >= 59 && hgt <= 76,
                        Err(_) => false,
                    };
                }

                false
            }),
        ),
        FieldDefinition::new(
            "hcl",
            make_callback(|value| Regex::new(r"^#[0-9,a-f]{6}$").unwrap().is_match(value)),
        ),
        FieldDefinition::new(
            "ecl",
            make_callback(move |value| eye_colors.contains(value)),
        ),
        FieldDefinition::new(
            "pid",
            make_callback(|value| Regex::new(r"^[0-9]{9}$").unwrap().is_match(value)),
        ),
    ];

    for passport in input.split("\n\n") {
        let fields: HashMap<&str, &str> = passport
            .split_ascii_whitespace()
            .map(|field| {
                let split: Vec<&str> = field.split(':').collect();
                let key = split[0];
                let value = split[1];
                (key, value)
            })
            .collect();

        let mut valid = true;
        for field_definition in &required_fields {
            let field_name = field_definition.name.as_str();
            if fields.contains_key(field_name) {
                let value = fields[field_name];
                if !field_definition.validate(value) {
                    valid = false;
                }
            } else {
                valid = false;
            }

            if !valid {
                break;
            }
        }

        if valid {
            valid_passports += 1;
        }
    }

    valid_passports
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2 output: {}", result);
}

use std::collections::HashSet;

const SAMPLE_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

fn load_input() -> String {
    std::fs::read_to_string("dec04/src/input.txt").expect("Unable to read input file")
}

fn puzzle_1() -> u32{
    let input = load_input();
    let mut valid_passports = 0;
    let required_fields: HashSet<&'static str> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
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

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}

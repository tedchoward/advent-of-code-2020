#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

impl PasswordPolicy {
    fn new(policy_string: &str) -> Self {
        let parts = policy_string
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();
        let range = parts[0].trim();
        let letter = parts[1].trim().chars().nth(0).unwrap();

        let parts = range.split('-').collect::<Vec<&str>>();
        let min = parts[0].trim().parse().unwrap();
        let max = parts[1].trim().parse().unwrap();

        Self { min, max, letter }
    }
}

#[derive(Debug)]
struct Entry {
    policy: PasswordPolicy,
    password: String,
}

impl Entry {
    fn new(entry_string: &str) -> Self {
        let parts = entry_string.split(':').collect::<Vec<&str>>();
        let policy = parts[0].trim();
        let password = parts[1].trim();

        let policy = PasswordPolicy::new(policy);
        let password = String::from(password);

        Self { policy, password }
    }

    /// Counts the number of occurrences of `policy.letter` in the password.
    /// Returns true if the count is within the range of `policy.max` -
    /// `policy.min`
    fn is_valid_old_rules(&self) -> bool {
        let matches = self
            .password
            .matches(self.policy.letter)
            .count();
        matches >= self.policy.min && matches <= self.policy.max
    }

    /// Grabs the characters using `policy.max` and `policy.min` as 1-based
    /// indices. Returns true if the first character matches `policy.letter`
    /// but not the second or vice-versa
    fn is_valid_new_rules(&self) -> bool {
        let first_char = self.password.chars().nth(self.policy.min - 1).unwrap();
        let last_char = self.password.chars().nth(self.policy.max - 1).unwrap();

        first_char == self.policy.letter && last_char != self.policy.letter
            || last_char == self.policy.letter && first_char != self.policy.letter
    }
}

fn load_input() -> Vec<Entry> {
    let input = std::fs::read_to_string("dec02/src/input.txt").expect("Unable to read input file");

    input.split('\n').map(Entry::new).collect::<Vec<Entry>>()
}

/// Count the number of entries that have valid passwords
fn puzzle_1() -> usize {
    let input = load_input();
    input.iter().filter(|e| e.is_valid_old_rules()).count()
}

/// Count the number of entries that have valid passwords based on the new
/// rules
fn puzzle_2() -> usize {
    let input = load_input();
    input.iter().filter(|e| e.is_valid_new_rules()).count()
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1, Number of valid entries: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2, Number of valid entries: {}", result);
}

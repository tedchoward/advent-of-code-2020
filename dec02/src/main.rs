#[derive(Debug)]
struct PasswordPolicy {
    min: u32,
    max: u32,
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

        Self {
            min,
            max,
            letter,
        }
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

    fn is_valid(&self) -> bool {
        let matches = self.password.matches(self.policy.letter).collect::<Vec<&str>>().len();
        matches >= self.policy.min as usize && matches <= self.policy.max as usize
    }
}

fn load_input() -> Vec<Entry> {
    let input = std::fs::read_to_string("dec02/src/input.txt").expect("Unable to read input file");

    input
        .split('\n')
        .map(Entry::new)
        .collect::<Vec<Entry>>()
}

/// Count the number of entries that have valid passwords
fn puzzle_1() -> usize {
    let input = load_input();
    input.iter().filter(|e| e.is_valid()).count()
}

fn main() {
    let result = puzzle_1();
    println!("Number of valid entries: {}", result);
}

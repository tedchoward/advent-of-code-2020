fn load_input() -> String {
    std::fs::read_to_string("dec08/src/input.txt").expect("Unable to read input file")
}

#[derive(Debug)]
struct Instruction<'a> {
    operation: &'a str,
    argument: i32,
    called: bool,
}

impl<'a> Instruction<'a> {
    fn new(instruction: &'a str) -> Self {
        let parts = instruction.split_ascii_whitespace().collect::<Vec<&str>>();
        let operation = parts[0];
        let argument = parts[1].parse::<i32>().unwrap();

        Self {
            operation,
            argument,
            called: false,
        }
    }
}

fn puzzle_1() -> i32 {
    // let input = String::from("nop +0
    // acc +1
    // jmp +4
    // acc +3
    // jmp -3
    // acc -99
    // acc +1
    // jmp -4
    // acc +6");

    let input = load_input();

    let mut accumulator = 0;
    let mut index = 0;

    let mut instructions = input
        .split('\n')
        .map(Instruction::new)
        .collect::<Vec<Instruction>>();

    while index < instructions.len() {
        let instruction = &mut instructions[index];
        if instruction.called {
            break;
        }

        instruction.called = true;

        match instruction.operation {
            "acc" => {
                accumulator += instruction.argument;
                index += 1;
            }
            "jmp" => {
                index = ((index as i32) + instruction.argument) as usize;
            }
            "nop" => {
                index += 1;
            }
            _ => panic!("unknown operation {}", instruction.operation),
        }
    }

    accumulator
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}

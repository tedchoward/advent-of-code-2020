fn load_input() -> String {
    std::fs::read_to_string("dec08/src/input.txt").expect("Unable to read input file")
}

#[derive(Clone, Debug)]
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

enum ExecutionResult {
    Loop(i32),
    Exit(i32),
}

fn execute(instructions: &[Instruction]) -> ExecutionResult {
    let mut instructions = instructions.iter().cloned().collect::<Vec<Instruction>>();
    let mut accumulator = 0;
    let mut index = 0;

    while index < instructions.len() {
        let instruction = &mut instructions[index];
        if instruction.called {
            return ExecutionResult::Loop(accumulator);
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

    ExecutionResult::Exit(accumulator)
}

fn puzzle_1() -> i32 {
    let input = load_input();

    let instructions = input
        .split('\n')
        .map(Instruction::new)
        .collect::<Vec<Instruction>>();

    match execute(&instructions) {
        ExecutionResult::Loop(acc) => acc,
        _ => panic!("Should cause loop"),
    }
}

fn puzzle_2() -> i32 {
    let input = load_input();

    let instructions = input
        .split('\n')
        .map(Instruction::new)
        .collect::<Vec<Instruction>>();
    let mut changed_index = 0;

    loop {
        let mut instructions = instructions.iter().cloned().collect::<Vec<Instruction>>();
        let (index, inst) = instructions[changed_index..]
            .iter_mut()
            .enumerate()
            .find(|(_, i)| i.operation == "jmp" || i.operation == "nop")
            .unwrap();

        if inst.operation == "jmp" {
            inst.operation = "nop";
        } else {
            inst.operation = "jmp";
        }

        changed_index += index + 1;

        match execute(&instructions) {
            ExecutionResult::Exit(acc) => {
                return acc;
            }
            ExecutionResult::Loop(_) => (),
        }
    }
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2 output: {}", result);
}

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;

const THIRTY_SIX_BITMASK: u64 = 0xFFFFFFFFF;

lazy_static! {
    static ref SET_MASK: Regex = Regex::new(r"^mask = ([X10]+)$").unwrap();
    static ref SET_MEM: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

enum Instruction {
    SetMask(String),
    SetMem(u64, u64),
}

trait Decoder {
    fn execute(&mut self, instruction: Instruction);
    fn memory_sum(&self) -> u64;
}

struct DecoderV1 {
    memory: HashMap<u64, u64>,
    ones_mask: u64,
    zeroes_mask: u64,
}

impl DecoderV1 {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
            ones_mask: 0,
            zeroes_mask: THIRTY_SIX_BITMASK,
        }
    }
}

impl Decoder for DecoderV1 {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(mask) => {
                let mut ones_mask: u64 = 0;
                let mut zeros_mask: u64 = THIRTY_SIX_BITMASK;
                for (index, bit) in mask.chars().rev().enumerate() {
                    if bit == '1' {
                        let val = 1 << index;
                        ones_mask |= val;
                    } else if bit == '0' {
                        let val = 1 << index;
                        zeros_mask ^= val;
                    }
                }

                self.ones_mask = ones_mask;
                self.zeroes_mask = zeros_mask;
            }
            Instruction::SetMem(memory_loc, value) => {
                let adjusted_value = (value | self.ones_mask) & self.zeroes_mask;
                self.memory.insert(memory_loc, adjusted_value);
            }
        }
    }

    fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

struct DecoderV2 {
    memory: HashMap<u64, u64>,
    ones_mask: u64,
    floating_mask: Vec<u64>,
}

impl DecoderV2 {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
            ones_mask: 0,
            floating_mask: vec![],
        }
    }

    fn calculate_memory_locations(&mut self, memory_loc: u64, value: u64, floats: &[u64]) {
        for (index, float) in floats.iter().enumerate() {
            let adjusted_loc_one = memory_loc | *float;
            self.memory.insert(adjusted_loc_one, value);
            self.calculate_memory_locations(adjusted_loc_one, value, &floats[(index + 1)..]);

            let adjusted_loc_zero = memory_loc & (*float ^ THIRTY_SIX_BITMASK);
            self.memory.insert(adjusted_loc_zero, value);
            self.calculate_memory_locations(adjusted_loc_zero, value, &floats[(index + 1)..]);
        }
    }
}

impl Decoder for DecoderV2 {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SetMask(mask) => {
                let mut ones_mask: u64 = 0;
                let mut floating_mask = Vec::new();
                for (index, bit) in mask.chars().rev().enumerate() {
                    if bit == '1' {
                        let val = 1 << index;
                        ones_mask |= val;
                    } else if bit == 'X' {
                        let val = 1 << index;
                        floating_mask.push(val);
                    }
                }

                self.ones_mask = ones_mask;
                self.floating_mask = floating_mask;
            }
            Instruction::SetMem(memory_loc, value) => {
                let adjusted_loc = memory_loc | self.ones_mask;
                let floats = self.floating_mask.clone();
                self.calculate_memory_locations(adjusted_loc, value, &floats);
                // for float in &self.floating_mask {
                //     let adjusted_loc_one = adjusted_loc | *float;
                //     self.memory.insert(adjusted_loc_one, value);

                //     let adjusted_loc_zero = adjusted_loc & (*float ^ THIRTY_SIX_BITMASK);
                //     self.memory.insert(adjusted_loc_zero, value);
                // }
            }
        }
    }

    fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

fn load_input() -> String {
    std::fs::read_to_string("dec14/src/input.txt").expect("Unable to read input file")
}

fn puzzle_1() -> u64 {
    //     let input = String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    // mem[8] = 11
    // mem[7] = 101
    // mem[8] = 0");

    let input = load_input();

    let instructions = input.split('\n').map(|ins| {
        if let Some(captures) = SET_MASK.captures(ins) {
            let mask = &captures[1];
            Instruction::SetMask(String::from(mask))
        } else if let Some(captures) = SET_MEM.captures(ins) {
            let mem_loc = captures[1].parse().unwrap();
            let new_value: u64 = captures[2].parse::<u64>().unwrap() & THIRTY_SIX_BITMASK;
            Instruction::SetMem(mem_loc, new_value)
        } else {
            panic!("Unparsed instruction: {}", ins);
        }
    });

    let mut decoder = DecoderV1::new();
    for instruction in instructions {
        decoder.execute(instruction);
    }

    decoder.memory_sum()
}

fn puzzle_2() -> u64 {
//     let input = String::from(
//         "mask = 000000000000000000000000000000X1001X
// mem[42] = 100
// mask = 00000000000000000000000000000000X0XX
// mem[26] = 1",
//     );

    let input = load_input();

    let instructions = input.split('\n').map(|ins| {
        if let Some(captures) = SET_MASK.captures(ins) {
            let mask = &captures[1];
            Instruction::SetMask(String::from(mask))
        } else if let Some(captures) = SET_MEM.captures(ins) {
            let mem_loc = captures[1].parse().unwrap();
            let new_value: u64 = captures[2].parse::<u64>().unwrap() & THIRTY_SIX_BITMASK;
            Instruction::SetMem(mem_loc, new_value)
        } else {
            panic!("Unparsed instruction: {}", ins);
        }
    });

    let mut decoder = DecoderV2::new();
    for instruction in instructions {
        decoder.execute(instruction);
    }

    decoder.memory_sum()
}

fn main() {
    let start = std::time::Instant::now();
    let result = puzzle_1();
    let duration = start.elapsed().as_millis();
    println!("Puzzle 1 result: {}", result);
    println!("Took {}ms", duration);

    let start = std::time::Instant::now();
    let result = puzzle_2();
    let duration = start.elapsed().as_millis();
    println!("Puzzle 2 result: {}", result);
    println!("Took {}ms", duration);
}

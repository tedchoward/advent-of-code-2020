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
    SetMem(usize, u64),
}

struct Decoder {
    memory: HashMap<usize, u64>,
    ones_mask: u64,
    zeroes_mask: u64,
}

impl Decoder {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
            ones_mask: 0,
            zeroes_mask: THIRTY_SIX_BITMASK,
        }
    }

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
            let mem_loc: usize = captures[1].parse().unwrap();
            let new_value: u64 = captures[2].parse::<u64>().unwrap() & THIRTY_SIX_BITMASK;
            Instruction::SetMem(mem_loc, new_value)
        } else {
            panic!("Unparsed instruction: {}", ins);
        }
    });

    let mut decoder = Decoder::new();
    for instruction in instructions {
        decoder.execute(instruction);
    }

    decoder.memory_sum()
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 result: {}", result);
}

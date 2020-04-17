use std::collections::VecDeque;
use std::io::{stdin, Read};
use std::num::Wrapping;

use crate::parser::Instruction;

pub struct Interpreter {
    instructions: VecDeque<Instruction>,
    memory: Vec<Wrapping<u8>>,
    ptr: isize,
}

impl Interpreter {
    pub fn new(instructions: VecDeque<Instruction>) -> Self {
        Self {
            instructions,
            memory: vec![Wrapping(0); 30_000],
            ptr: 0,
        }
    }

    pub fn run(&mut self) {
        for _ in self {
            continue;
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::IncrementPtr => self.ptr += 1,
            Instruction::DecrementPtr => self.ptr -= 1,
            Instruction::IncrementValue => {
                let index = self.get_index();
                self.memory[index] += Wrapping(1);
            }
            Instruction::DecrementValue => {
                let index = self.get_index();
                self.memory[index] -= Wrapping(1);
            }
            Instruction::Output => {
                let index = self.get_index();
                print!("{}", self.memory[index].0 as char);
            }
            Instruction::Input => {
                let mut buffer = vec![0];
                stdin()
                    .read_exact(&mut buffer)
                    .unwrap();
                let index = self.get_index();
                self.memory[index] = Wrapping(buffer[0]);
            }
            Instruction::Loop(instructions) => {
                while {
                    let index = self.get_index();
                    self.memory[index].0 != 0
                } {
                    for instruction in instructions.iter() {
                        self.execute_instruction(instruction);
                    }
                }
            }
        }
    }

    fn get_index(&mut self) -> usize {
        while self.ptr < 0 {
            self.ptr += self.memory.len() as isize;
        }
        while self.ptr > self.memory.len() as isize {
            self.ptr -= self.memory.len() as isize;
        }
        self.ptr as usize
    }
}

impl Iterator for Interpreter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(instruction) = self.instructions.pop_front() {
            self.execute_instruction(&instruction);
            Some(())
        } else {
            None
        }
    }
}

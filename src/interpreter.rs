use std::collections::VecDeque;
use std::io::{stdin, Read};

use crate::parser::Instruction;

pub struct Interpreter {
    instructions: VecDeque<Instruction>,
    memory: Vec<u8>,
    ptr: isize,
}

impl Interpreter {
    pub fn new(instructions: VecDeque<Instruction>) -> Self {
        Self {
            instructions,
            memory: vec![0; 30_000],
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
                self.memory[index] = self.memory[index].wrapping_add(1);
            }
            Instruction::DecrementValue => {
                let index = self.get_index();
                self.memory[index] = self.memory[index].wrapping_sub(1);
            }
            Instruction::Output => {
                let index = self.get_index();
                print!("{}", self.memory[index] as char);
            }
            Instruction::Input => {
                let index = self.get_index();
                stdin()
                    .read_exact(&mut self.memory[index..index + 1])
                    .unwrap();
            }
            Instruction::Loop(instructions) => {
                while {
                    let index = self.get_index();
                    self.memory[index] != 0
                } {
                    for instruction in instructions.iter() {
                        self.execute_instruction(instruction);
                    }
                }
            }
        }
    }

    #[inline]
    fn get_index(&self) -> usize {
        (self.ptr.rem_euclid(self.memory.len() as isize)) as usize
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

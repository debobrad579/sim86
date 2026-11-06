use std::fs::File;
use std::io::Write;

use crate::{
    sim::Sim86,
    types::{Instruction, InstructionData},
};

impl Sim86 {
    pub fn run(&mut self, print: bool) {
        loop {
            let instruction = Instruction::from(&self.memory[self.ip..]);

            self.ip += instruction.size;

            if print {
                println!("{}", instruction);
            }

            if let InstructionData::HLT = instruction.data {
                break;
            }

            self.execute_instruction(instruction);
        }

        if print {
            println!("{:?}", self);
        }

        let mut file = File::create("memory.data").unwrap();
        file.write_all(&self.memory).unwrap();
    }

    pub(super) fn update_ip(&mut self, offset: i8) {
        if offset >= 0 {
            self.ip += offset as usize
        } else {
            self.ip -= (-offset) as usize
        }
    }
}

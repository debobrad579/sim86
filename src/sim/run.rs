use std::fs::File;
use std::io::Write;

use crate::{
    sim::Sim86,
    types::instruction::{Instruction, InstructionData},
};

impl Sim86 {
    pub fn run(&mut self, print: bool) {
        loop {
            let instruction = Instruction::from(&self.memory[self.ip as usize..]);

            self.ip = self.ip.wrapping_add(instruction.size);

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

    pub(super) fn increment_ip(&mut self, inc: i16) {
        if inc >= 0 {
            self.ip += inc as u16
        } else {
            self.ip -= (-inc) as u16
        }
    }
}

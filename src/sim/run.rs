use std::io::Write;
use std::{fs::File, path::PathBuf};

use crate::{
    sim::{Sim86, utils::get_memory_address},
    types::instruction::{Instruction, InstructionData},
};

impl Sim86 {
    pub fn run(&mut self, print: bool, output: Option<PathBuf>) {
        loop {
            let instruction = Instruction::from(
                &self.memory[get_memory_address(self.segment_registers[0b01], self.ip)..],
            );

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

        if let Some(output_path) = output {
            let mut file = File::create(output_path.into_os_string()).unwrap();
            file.write_all(&self.memory).unwrap();
        }
    }

    pub(super) fn increment_ip(&mut self, inc: i16) {
        if inc >= 0 {
            self.ip += inc as u16
        } else {
            self.ip -= (-inc) as u16
        }
    }
}

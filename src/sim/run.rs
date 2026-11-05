use crate::{sim::Sim86, types::Instruction};

impl Sim86 {
    pub fn run(&mut self, print: bool) {
        let len = self.program.len();

        while self.ip < len {
            let instruction = Instruction::from(&self.program[self.ip..]);

            self.ip += instruction.size;

            if print {
                println!("\n{}", instruction);
            }

            self.execute_instruction(instruction);

            if print {
                println!("{:?}", self);
            }
        }
    }

    pub(super) fn update_ip(&mut self, offset: i8) {
        if offset >= 0 {
            self.ip += offset as usize
        } else {
            self.ip -= (-offset) as usize
        }
    }
}

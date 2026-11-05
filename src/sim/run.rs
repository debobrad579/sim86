use crate::{sim::Sim86, types::Instruction};

impl Sim86 {
    pub fn run(&mut self, print: bool) {
        let len = self.program.len();

        while self.ip < len {
            let mut bytes = [0u8; 6];

            let bytes_left = (len - self.ip).min(6);

            bytes[..bytes_left].copy_from_slice(&self.program[self.ip..self.ip + bytes_left]);

            let instruction = Instruction::from(bytes);

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

    pub(super) fn update_ip(&mut self, amount: i8) {
        if amount >= 0 {
            self.ip += amount as usize
        } else {
            self.ip -= (-amount) as usize
        }
    }
}

use crate::{sim::Sim86, types::Instruction};

impl Sim86 {
    pub fn run(&mut self) {
        let len = self.program.len();

        while self.instruction_pointer < len {
            let i = self.instruction_pointer;
            let mut bytes = [0u8; 6];

            let bytes_left = (len - i).min(6);

            bytes[..bytes_left].copy_from_slice(&self.program[i..i + bytes_left]);

            let instruction = Instruction::from(bytes);

            self.instruction_pointer += instruction.size;

            println!("\n{}", instruction);

            self.execute_instruction(instruction);

            println!("{:?}", self);
        }
    }

    pub(super) fn update_instruction_pointer(&mut self, amount: i8) {
        if amount >= 0 {
            self.instruction_pointer += amount as usize
        } else {
            self.instruction_pointer -= (-amount) as usize
        }
    }
}

use crate::{
    sim::Sim86,
    types::{Instruction, InstructionData},
};

impl Sim86 {
    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.data {
            InstructionData::MOV {
                destination,
                source,
                ..
            } => self.set_value(destination, self.get_value(source)),
            InstructionData::ALU { .. } => todo!(),
            InstructionData::Jump { .. } => todo!(),
        }
    }
}

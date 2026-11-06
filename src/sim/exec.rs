use crate::{
    sim::Sim86,
    types::{Instruction, InstructionData},
};

impl Sim86 {
    pub(super) fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.data {
            InstructionData::MOV {
                destination,
                source,
                width,
            } => self.set_value(destination, self.get_value(&source, &width), &width),
            InstructionData::ALU {
                op_code,
                destination,
                source,
                width,
            } => self.exec_alu_instruction(op_code, destination, source, width),
            InstructionData::Jump { op_code, target } => {
                self.exec_jump_instruction(op_code, target)
            }
            InstructionData::HLT => unreachable!(),
        }
    }
}

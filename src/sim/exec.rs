use crate::{
    sim::Sim86,
    types::{Instruction, InstructionData, Width},
};

impl Sim86 {
    pub(super) fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.data {
            InstructionData::MOV {
                destination,
                source,
                width,
            } => self.set_value(destination, self.get_value(&source, &width), &width),
            InstructionData::PUSH { operand } => todo!(),
            InstructionData::POP { operand } => todo!(),
            InstructionData::LEA { operand, eac } => {
                self.set_value(operand, self.get_effective_address(&eac), &Width::Word)
            }
            InstructionData::AluBinary {
                op_code,
                destination,
                source,
                width,
            } => self.exec_alu_binary_instruction(op_code, destination, source, width),
            InstructionData::AluUnary {
                op_code,
                width,
                operand,
            } => self.exec_alu_unary_instruction(op_code, operand, width),
            InstructionData::ShiftRotate {
                op_code,
                operand,
                amount,
                width,
            } => self.exec_shift_rotate_instruction(op_code, operand, amount, width),
            InstructionData::Jump { op_code, target } => {
                self.exec_jump_instruction(op_code, target)
            }
            InstructionData::HLT => unreachable!(),
        }
    }
}

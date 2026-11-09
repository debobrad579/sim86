use crate::{
    sim::Sim86,
    types::{
        instruction::{Instruction, InstructionData},
        operand::{EffectiveAddressCalculation, Memory, Operand, Register},
        width::Width,
    },
};

impl Sim86 {
    pub(super) fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.data {
            InstructionData::MOV {
                destination,
                source,
                width,
            } => self.set_value(destination, self.get_value(&source, &width), &width),
            InstructionData::PUSH { operand } => {
                self.registers[4] -= 2;
                self.set_value(
                    Operand::Memory(Memory {
                        address: EffectiveAddressCalculation::Calculation {
                            base: Register {
                                width: Width::Word,
                                value: 0b100,
                            },
                            index: None,
                            displacement: 0,
                        },
                        segment_override: None,
                    }),
                    self.get_value(&operand, &Width::Word),
                    &Width::Word,
                );
            }
            InstructionData::POP { operand } => {
                self.set_value(
                    operand,
                    self.get_value(
                        &Operand::Memory(Memory {
                            address: EffectiveAddressCalculation::Calculation {
                                base: Register {
                                    width: Width::Word,
                                    value: 0b100,
                                },
                                index: None,
                                displacement: 0,
                            },
                            segment_override: None,
                        }),
                        &Width::Word,
                    ),
                    &Width::Word,
                );
                self.registers[4] += 2;
            }
            InstructionData::JMP(_) => todo!(),
            InstructionData::CALL(_) => todo!(),
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
            InstructionData::ConditionalJump { op_code, target } => {
                self.exec_jump_instruction(op_code, target)
            }
            InstructionData::HLT => unreachable!(),
        }
    }
}

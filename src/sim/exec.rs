use crate::{
    sim::Sim86,
    types::{ALUOpCode, Instruction, InstructionData},
};

impl Sim86 {
    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.data {
            InstructionData::MOV {
                destination,
                source,
                ..
            } => self.set_value(destination, self.get_value(&source)),
            InstructionData::ALU {
                op_code,
                destination,
                source,
                ..
            } => match op_code {
                ALUOpCode::ADD => {
                    let (a, b) = (self.get_value(&destination), self.get_value(&source));
                    let (result, carry) = a.overflowing_add(b);
                    let auxiliary_carry = ((a & 0x0F) + (b & 0x0F)) > 0x0F;
                    let overflow = (!(a ^ b) & (a ^ result) & 0x8000) != 0;
                    self.update_alu_flags(result, carry, auxiliary_carry, overflow);
                    self.set_value(destination, result);
                }
                ALUOpCode::SUB => {
                    let (a, b) = (self.get_value(&destination), self.get_value(&source));
                    let (result, carry) = a.overflowing_sub(b);
                    let auxiliary_carry = (a & 0x0F) < (b & 0x0F);
                    let overflow = ((a ^ b) & (a ^ result) & 0x8000) != 0;
                    self.update_alu_flags(result, carry, auxiliary_carry, overflow);
                    self.set_value(destination, result);
                }
                ALUOpCode::CMP => {
                    let (a, b) = (self.get_value(&destination), self.get_value(&source));
                    let (result, carry) = a.overflowing_sub(b);
                    let auxiliary_carry = (a & 0x0F) < (b & 0x0F);
                    let overflow = ((a ^ b) & (a ^ result) & 0x8000) != 0;
                    self.update_alu_flags(result, carry, auxiliary_carry, overflow);
                }
                _ => unimplemented!(),
            },
            InstructionData::Jump { .. } => todo!(),
        }
    }
}

use crate::{
    sim::Sim86,
    types::{ALUOpCode, Instruction, InstructionData, JumpOpCode},
};

impl Sim86 {
    pub(super) fn execute_instruction(&mut self, instruction: Instruction) {
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
            InstructionData::Jump { op_code, target } => {
                if match op_code {
                    JumpOpCode::JO => self.get_overflow_flag(),
                    JumpOpCode::JNO => !self.get_overflow_flag(),
                    JumpOpCode::JB => self.get_carry_flag(),
                    JumpOpCode::JNB => !self.get_carry_flag(),
                    JumpOpCode::JE => self.get_zero_flag(),
                    JumpOpCode::JNE => !self.get_zero_flag(),
                    JumpOpCode::JBE => self.get_carry_flag() || self.get_zero_flag(),
                    JumpOpCode::JNBE => !(self.get_carry_flag() || self.get_zero_flag()),
                    JumpOpCode::JS => self.get_sign_flag(),
                    JumpOpCode::JNS => !self.get_sign_flag(),
                    JumpOpCode::JP => self.get_parity_flag(),
                    JumpOpCode::JNP => !self.get_parity_flag(),
                    JumpOpCode::JL => self.get_sign_flag() != self.get_overflow_flag(),
                    JumpOpCode::JNL => self.get_sign_flag() == self.get_overflow_flag(),
                    JumpOpCode::JLE => {
                        self.get_sign_flag() != self.get_overflow_flag() || self.get_zero_flag()
                    }
                    JumpOpCode::JNLE => {
                        self.get_sign_flag() == self.get_overflow_flag() && !self.get_zero_flag()
                    }
                    JumpOpCode::LOOPNZ => {
                        self.registers[1] -= 1;
                        self.registers[1] != 0 && !self.get_zero_flag()
                    }
                    JumpOpCode::LOOPZ => {
                        self.registers[1] -= 1;
                        self.registers[1] != 0 && self.get_zero_flag()
                    }
                    JumpOpCode::LOOP => {
                        self.registers[1] -= 1;
                        self.registers[1] != 0
                    }
                    JumpOpCode::JCXZ => self.registers[1] == 0,
                } {
                    self.update_ip(target - 2)
                }
            }
        }
    }
}

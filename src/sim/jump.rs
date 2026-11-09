use crate::{sim::Sim86, types::jump::ConditionalJumpOpCode};

impl Sim86 {
    pub(super) fn exec_jump_instruction(&mut self, op_code: ConditionalJumpOpCode, target: i8) {
        if match op_code {
            ConditionalJumpOpCode::JO => self.get_overflow_flag(),
            ConditionalJumpOpCode::JNO => !self.get_overflow_flag(),
            ConditionalJumpOpCode::JB => self.get_carry_flag(),
            ConditionalJumpOpCode::JNB => !self.get_carry_flag(),
            ConditionalJumpOpCode::JE => self.get_zero_flag(),
            ConditionalJumpOpCode::JNE => !self.get_zero_flag(),
            ConditionalJumpOpCode::JBE => self.get_carry_flag() || self.get_zero_flag(),
            ConditionalJumpOpCode::JNBE => !(self.get_carry_flag() || self.get_zero_flag()),
            ConditionalJumpOpCode::JS => self.get_sign_flag(),
            ConditionalJumpOpCode::JNS => !self.get_sign_flag(),
            ConditionalJumpOpCode::JP => self.get_parity_flag(),
            ConditionalJumpOpCode::JNP => !self.get_parity_flag(),
            ConditionalJumpOpCode::JL => self.get_sign_flag() != self.get_overflow_flag(),
            ConditionalJumpOpCode::JNL => self.get_sign_flag() == self.get_overflow_flag(),
            ConditionalJumpOpCode::JLE => {
                self.get_sign_flag() != self.get_overflow_flag() || self.get_zero_flag()
            }
            ConditionalJumpOpCode::JNLE => {
                self.get_sign_flag() == self.get_overflow_flag() && !self.get_zero_flag()
            }
            ConditionalJumpOpCode::LOOPNZ => {
                self.registers[1] -= 1;
                self.registers[1] != 0 && !self.get_zero_flag()
            }
            ConditionalJumpOpCode::LOOPZ => {
                self.registers[1] -= 1;
                self.registers[1] != 0 && self.get_zero_flag()
            }
            ConditionalJumpOpCode::LOOP => {
                self.registers[1] -= 1;
                self.registers[1] != 0
            }
            ConditionalJumpOpCode::JCXZ => self.registers[1] == 0,
        } {
            self.update_ip(target)
        }
    }
}

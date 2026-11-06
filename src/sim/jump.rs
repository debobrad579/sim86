use crate::{sim::Sim86, types::JumpOpCode};

impl Sim86 {
    pub(super) fn exec_jump_instruction(&mut self, op_code: JumpOpCode, target: i8) {
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

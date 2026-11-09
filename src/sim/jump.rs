use crate::{
    sim::Sim86,
    types::{
        jump::{ConditionalJumpOpCode, Jump},
        width::Width,
    },
};

impl Sim86 {
    pub(super) fn get_new_ip_cs(&mut self, jump: Jump) -> (u16, Option<u16>) {
        match jump {
            Jump::DirectWithinSegment(target) => (
                if target >= 0 {
                    self.ip + target as u16
                } else {
                    self.ip - (-target) as u16
                },
                None,
            ),
            Jump::IndirectWithinSegment(operand) => (self.get_value(&operand, &Width::Word), None),
            Jump::DirectIntersegment { ip, cs } => (ip, Some(cs)),
            Jump::IndirectIntersegment(memory) => {
                let address = self.get_memory_address(&memory);

                (
                    u16::from_le_bytes([self.memory[address], self.memory[address + 1]]),
                    Some(u16::from_le_bytes([
                        self.memory[address + 2],
                        self.memory[address + 3],
                    ])),
                )
            }
        }
    }

    pub(super) fn exec_conditional_jump(&mut self, op_code: ConditionalJumpOpCode, target: i8) {
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
            self.increment_ip(target as i16)
        }
    }
}

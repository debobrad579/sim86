use crate::{
    sim::Sim86,
    types::{Immediate, Instruction, OpCode, Operand, Register, Width},
};

impl Sim86 {
    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.op_code {
            OpCode::MOV => {
                let data = match instruction.operand2 {
                    Operand::Register(register) => self.get_value(register),
                    Operand::Memory(_memory) => todo!(),
                    Operand::Immediate(immediate) => match immediate {
                        Immediate::SignExtended(value) => value as u16,
                        Immediate::Unsigned(value) => value,
                    },
                    _ => unreachable!(),
                };

                match instruction.operand1 {
                    Operand::Register(register) => self.set_value(register, data),
                    Operand::Memory(_memory) => todo!(),
                    _ => unreachable!(),
                }
            }
            _ => todo!(),
        }
    }

    fn get_value(&self, register: Register) -> u16 {
        match register.width {
            Width::Byte => {
                if register.value < 4 {
                    self.registers[register.value as usize] & 0x00FF
                } else {
                    self.registers[register.value as usize - 4] >> 8
                }
            }
            Width::Word => self.registers[register.value as usize],
        }
    }

    fn set_value(&mut self, register: Register, value: u16) {
        match register.width {
            Width::Byte => {
                if register.value < 4 {
                    self.registers[register.value as usize] =
                        (self.registers[register.value as usize] & 0xFF00) | (value & 0x00FF);
                } else {
                    let index = register.value as usize - 4;
                    self.registers[index] = (self.registers[index] & 0x00FF) | value << 8;
                }
            }
            Width::Word => self.registers[register.value as usize] = value,
        }
    }
}

use crate::{
    sim::Sim86,
    types::{Immediate, Operand, Register, Width},
};

impl Sim86 {
    pub(super) fn get_value(&self, operand: Operand) -> u16 {
        match operand {
            Operand::Register(register) => self.get_register(register),
            Operand::Memory(_memory) => todo!(),
            Operand::Immediate(immediate) => match immediate {
                Immediate::SignExtended(value) => value as u16,
                Immediate::Unsigned(value) => value,
            },
        }
    }

    pub(super) fn set_value(&mut self, operand: Operand, value: u16) {
        match operand {
            Operand::Register(register) => self.set_register(register, value),
            Operand::Memory(_memory) => todo!(),
            _ => unreachable!(),
        }
    }

    pub(super) fn get_register(&self, register: Register) -> u16 {
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

    pub(super) fn set_register(&mut self, register: Register, value: u16) {
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

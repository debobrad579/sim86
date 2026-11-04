use std::fmt::Debug;

use crate::types::{Immediate, Instruction, OpCode, Operand, Register};

#[derive(Default)]
pub struct Sim86 {
    pub registers: [u16; 8],
}

impl Debug for Sim86 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AX:{:02X} | SP:{:02X}\nBX:{:02X} | BP:{:02X}\nCX:{:02X} | SI:{:02X}\nDX:{:02X} | DI:{:02X}",
            self.registers[0],
            self.registers[4],
            self.registers[3],
            self.registers[5],
            self.registers[1],
            self.registers[6],
            self.registers[2],
            self.registers[7]
        )
    }
}

impl Sim86 {
    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.op_code {
            OpCode::MOV => {
                let data = match instruction.operand2 {
                    Operand::Register(register) => self.registers[register.value as usize],
                    Operand::Memory(memory) => todo!(),
                    Operand::Immediate(immediate) => match immediate {
                        Immediate::SignExtended(value) => value as u16,
                        Immediate::Unsigned(value) => value,
                    },
                    _ => unreachable!(),
                };

                match instruction.operand1 {
                    Operand::Register(register) => self.registers[register.value as usize] = data,
                    Operand::Memory(memory) => todo!(),
                    Operand::Immediate(immediate) => todo!(),
                    _ => unreachable!(),
                }
            }
            _ => todo!(),
        }
    }
}

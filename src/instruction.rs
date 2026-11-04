use std::fmt::Display;

use crate::op_code::OpCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    Byte,
    Word,
}

impl From<u8> for Width {
    fn from(value: u8) -> Self {
        match value {
            0b0 => Self::Byte,
            0b1 => Self::Word,
            _ => unreachable!(),
        }
    }
}

pub struct Instruction {
    pub op_code: OpCode,
    pub width: Width,
    pub destination: Operand,
    pub source: Operand,
    pub size: usize,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {},{}", self.op_code, self.destination, self.source)
    }
}

pub enum Operand {
    Register(Register),
    Memory(Memory),
    Immediate(u16),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(register) => write!(f, "{}", register),
            Operand::Memory(memory) => write!(f, "{}", memory),
            Operand::Immediate(value) => write!(f, "{}", value),
        }
    }
}

pub struct Register {
    pub width: Width,
    pub value: u8,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.width {
            Width::Byte => match self.value {
                0b000 => write!(f, "AL"),
                0b001 => write!(f, "CL"),
                0b010 => write!(f, "DL"),
                0b011 => write!(f, "BL"),
                0b100 => write!(f, "AH"),
                0b101 => write!(f, "CH"),
                0b110 => write!(f, "DH"),
                0b111 => write!(f, "BH"),
                _ => unreachable!(),
            },
            Width::Word => match self.value {
                0b000 => write!(f, "AX"),
                0b001 => write!(f, "CX"),
                0b010 => write!(f, "DX"),
                0b011 => write!(f, "BX"),
                0b100 => write!(f, "SP"),
                0b101 => write!(f, "BP"),
                0b110 => write!(f, "SI"),
                0b111 => write!(f, "DI"),
                _ => unreachable!(),
            },
        }
    }
}

pub struct Memory {
    pub base: Register,
    pub index: Option<Register>,
    pub displacement: i16,
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.index {
            Some(index) => write!(f, "[{}+{}{:+}]", self.base, index, self.displacement),
            None => write!(f, "[{}{:+}]", self.base, self.displacement),
        }
    }
}

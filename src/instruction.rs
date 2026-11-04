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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    RegToRM,
    RMToReg,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0b0 => Self::RegToRM,
            0b1 => Self::RMToReg,
            _ => unreachable!(),
        }
    }
}

pub struct Instruction {
    pub op_code: OpCode,
    pub width: Width,
    pub direction: Direction,
    pub destination: Operand,
    pub source: Operand,
    pub size: usize,
}

impl From<[u8; 6]> for Instruction {
    fn from(bytes: [u8; 6]) -> Self {
        if bytes[0] >> 2 == 0b100010 {
            let direction = Direction::from((bytes[0] & 0b00000010) >> 1);
            let width = Width::from(bytes[0] & 0b00000001);
            let mode = bytes[1] >> 6;
            let reg = (bytes[1] & 0b00111000) >> 3;
            let rm = bytes[1] & 0b00000111;

            match mode {
                0b00 => todo!(),
                0b01 => todo!(),
                0b10 => todo!(),
                0b11 => {
                    return Self {
                        op_code: OpCode::MOV,
                        width,
                        direction,
                        destination: Operand::Register(Register {
                            width,
                            value: match direction {
                                Direction::RegToRM => rm,
                                Direction::RMToReg => reg,
                            },
                        }),
                        source: Operand::Register(Register {
                            width,
                            value: match direction {
                                Direction::RegToRM => reg,
                                Direction::RMToReg => rm,
                            },
                        }),
                        size: 2,
                    };
                }
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {},{}", self.op_code, self.destination, self.source)
    }
}

pub enum Operand {
    Register(Register),
    Memory {
        base: Register,
        index: Option<Register>,
        displacement: i16,
    },
    Immediate(i16),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(register) => write!(f, "{}", register),
            Operand::Memory {
                base,
                index,
                displacement,
            } => match index {
                Some(index) => write!(f, "[{}+{}{:+}]", base, index, displacement),
                None => write!(f, "[{}{:+}]", base, displacement),
            },
            Operand::Immediate(value) => write!(f, "{}", value),
        }
    }
}

pub struct Register {
    width: Width,
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

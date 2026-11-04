use crate::{
    effective_address_calculation::get_effective_address_expression,
    instruction::{Instruction, Memory, Operand, Register, Width},
    op_code::OpCode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
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

impl From<[u8; 6]> for Instruction {
    fn from(bytes: [u8; 6]) -> Self {
        if bytes[0] >> 2 == 0b100010 {
            let direction = Direction::from((bytes[0] & 0b00000010) >> 1);
            let width = Width::from(bytes[0] & 0b00000001);
            let mode = bytes[1] >> 6;
            let reg = (bytes[1] & 0b00111000) >> 3;
            let rm = bytes[1] & 0b00000111;

            return match mode {
                0b00 => Self {
                    op_code: OpCode::MOV,
                    width,
                    destination: match direction {
                        Direction::RegToRM => {
                            Operand::Memory(get_effective_address_expression(rm, 0))
                        }
                        Direction::RMToReg => Operand::Register(Register { width, value: reg }),
                    },
                    source: match direction {
                        Direction::RegToRM => Operand::Register(Register { width, value: reg }),
                        Direction::RMToReg => {
                            Operand::Memory(get_effective_address_expression(rm, 0))
                        }
                    },
                    size: 2,
                },
                0b01 => {
                    Self {
                        op_code: OpCode::MOV,
                        width,
                        destination: match direction {
                            Direction::RegToRM => Operand::Memory(
                                get_effective_address_expression(rm, bytes[2] as i8 as i16),
                            ),
                            Direction::RMToReg => Operand::Register(Register { width, value: reg }),
                        },
                        source: match direction {
                            Direction::RegToRM => Operand::Register(Register { width, value: reg }),
                            Direction::RMToReg => Operand::Memory(
                                get_effective_address_expression(rm, bytes[2] as i8 as i16),
                            ),
                        },
                        size: 3,
                    }
                }
                0b10 => Self {
                    op_code: OpCode::MOV,
                    width,
                    destination: match direction {
                        Direction::RegToRM => Operand::Memory(get_effective_address_expression(
                            rm,
                            u16::from_le_bytes([bytes[2], bytes[3]]) as i16,
                        )),
                        Direction::RMToReg => Operand::Register(Register { width, value: reg }),
                    },
                    source: match direction {
                        Direction::RegToRM => Operand::Register(Register { width, value: reg }),
                        Direction::RMToReg => Operand::Memory(get_effective_address_expression(
                            rm,
                            u16::from_le_bytes([bytes[2], bytes[3]]) as i16,
                        )),
                    },
                    size: 4,
                },
                0b11 => Self {
                    op_code: OpCode::MOV,
                    width,
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
                },
                _ => unreachable!(),
            };
        } else if bytes[0] >> 4 == 0b1011 {
            let width = Width::from((bytes[0] & 0b00001000) >> 3);
            let reg = bytes[0] & 0b00000111;

            return Self {
                op_code: OpCode::MOV,
                width,
                destination: Operand::Register(Register { width, value: reg }),
                source: Operand::Immediate(if width == Width::Byte {
                    bytes[1] as u16
                } else {
                    u16::from_le_bytes([bytes[1], bytes[2]])
                }),
                size: if width == Width::Word { 3 } else { 2 },
            };
        }

        unreachable!()
    }
}

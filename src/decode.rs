use crate::{
    instruction::{Instruction, Operand, Register, Width},
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

            match mode {
                0b00 => todo!(),
                0b01 => todo!(),
                0b10 => todo!(),
                0b11 => {
                    return Self {
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
                    };
                }
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}

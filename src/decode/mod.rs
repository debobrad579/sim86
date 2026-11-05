pub mod memory;
pub mod op_code;

use crate::{
    decode::{
        memory::{ModRMType, decode_modrm},
        op_code::{get_alu_op_code, get_conditional_jump_op_code},
    },
    types::{Immediate, Instruction, InstructionData, Memory, Operand, Register, Width},
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

            let mod_rm = decode_modrm(bytes, ModRMType::Register);

            return match direction {
                Direction::RegToRM => Self {
                    data: InstructionData::MOV {
                        width: mod_rm.width,
                        destination: mod_rm.rm,
                        source: mod_rm.reg,
                    },
                    size: mod_rm.size,
                },
                Direction::RMToReg => Self {
                    data: InstructionData::MOV {
                        width: mod_rm.width,
                        destination: mod_rm.reg,
                        source: mod_rm.rm,
                    },
                    size: mod_rm.size,
                },
            };
        } else if bytes[0] >> 1 == 0b1100011 {
            let mod_rm = decode_modrm(bytes, ModRMType::Immediate(false));

            return Self {
                data: InstructionData::MOV {
                    width: mod_rm.width,
                    destination: mod_rm.rm,
                    source: mod_rm.reg,
                },
                size: mod_rm.size,
            };
        } else if bytes[0] >> 4 == 0b1011 {
            let width = Width::from((bytes[0] & 0b00001000) >> 3);
            let reg = bytes[0] & 0b00000111;

            return Self {
                data: InstructionData::MOV {
                    width,
                    destination: Operand::Register(Register { width, value: reg }),
                    source: Operand::Immediate(Immediate::Unsigned(if width == Width::Byte {
                        bytes[1] as u16
                    } else {
                        u16::from_le_bytes([bytes[1], bytes[2]])
                    })),
                },
                size: if width == Width::Word { 3 } else { 2 },
            };
        } else if bytes[0] >> 2 == 0b101000 {
            let direction = Direction::from((bytes[0] & 0b00000010) >> 1);
            let width = Width::from(bytes[0] & 0b00000001);
            let addr = u16::from_le_bytes([bytes[1], bytes[2]]);

            let memory = Operand::Memory(Memory::DirectAccess(addr));
            let register = Operand::Register(Register { width, value: 0 });

            return match direction {
                Direction::RegToRM => Self {
                    data: InstructionData::MOV {
                        width,
                        destination: register,
                        source: memory,
                    },
                    size: 3,
                },
                Direction::RMToReg => Self {
                    data: InstructionData::MOV {
                        width,
                        destination: memory,
                        source: register,
                    },
                    size: 3,
                },
            };
        } else if bytes[0] >> 2 == 0b100011 {
            let direction = Direction::from((bytes[0] & 0b00000010) >> 1);

            let mod_rm = decode_modrm(bytes, ModRMType::SegmentRegister);

            return match direction {
                Direction::RegToRM => Self {
                    data: InstructionData::MOV {
                        width: mod_rm.width,
                        destination: mod_rm.rm,
                        source: mod_rm.reg,
                    },
                    size: mod_rm.size,
                },
                Direction::RMToReg => Self {
                    data: InstructionData::MOV {
                        width: mod_rm.width,
                        destination: mod_rm.reg,
                        source: mod_rm.rm,
                    },
                    size: mod_rm.size,
                },
            };
        } else if bytes[0] & 0b11000100 == 0 {
            let direction = Direction::from((bytes[0] & 0b00000010) >> 1);
            let literal = (bytes[0] & 0b00111000) >> 3;

            let op_code = get_alu_op_code(literal);
            let mod_rm = decode_modrm(bytes, ModRMType::Register);

            return match direction {
                Direction::RegToRM => Self {
                    data: InstructionData::ALU {
                        op_code,
                        width: mod_rm.width,
                        destination: mod_rm.rm,
                        source: mod_rm.reg,
                    },
                    size: mod_rm.size,
                },
                Direction::RMToReg => Self {
                    data: InstructionData::ALU {
                        op_code,
                        width: mod_rm.width,
                        destination: mod_rm.reg,
                        source: mod_rm.rm,
                    },
                    size: mod_rm.size,
                },
            };
        } else if bytes[0] >> 6 == 0b10 && bytes[0] & 0b00000100 == 0 {
            let sign_extend = bytes[0] & 0b00000010 != 0;
            let literal = (bytes[1] & 0b00111000) >> 3;

            let op_code = get_alu_op_code(literal);
            let mod_rm = decode_modrm(bytes, ModRMType::Immediate(sign_extend));

            return Self {
                data: InstructionData::ALU {
                    op_code,
                    width: mod_rm.width,
                    destination: mod_rm.rm,
                    source: mod_rm.reg,
                },
                size: mod_rm.size,
            };
        } else if bytes[0] >> 6 == 0b00 && (bytes[0] & 0b00000110) >> 1 == 0b10 {
            let width = Width::from(bytes[0] & 0b00000001);
            let literal = (bytes[0] & 0b00111000) >> 3;

            let op_code = get_alu_op_code(literal);

            let (data, size) = match width {
                Width::Byte => (bytes[1] as u16, 2),
                Width::Word => (u16::from_le_bytes([bytes[1], bytes[2]]), 3),
            };

            let immediate = Operand::Immediate(Immediate::Unsigned(data));
            let register = Operand::Register(Register { width, value: 0 });

            return Self {
                data: InstructionData::ALU {
                    op_code,
                    width,
                    destination: register,
                    source: immediate,
                },
                size,
            };
        } else if let Some(op_code) = get_conditional_jump_op_code(bytes[0]) {
            return Self {
                data: InstructionData::Jump {
                    op_code,
                    target: bytes[1] as i8 + 2,
                },
                size: 2,
            };
        }

        unreachable!("operation not found: {:08b}", bytes[0])
    }
}

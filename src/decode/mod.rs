pub mod memory;
pub mod op_code;

use crate::{
    decode::{
        memory::{ModRMType, decode_modrm},
        op_code::{get_alu_binary_op_code, get_alu_unary_op_code, get_conditional_jump_op_code},
    },
    types::{
        EAC, Immediate, Instruction, InstructionData, Memory, Operand, Register, SegmentRegister,
        Width,
    },
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

impl From<&[u8]> for Instruction {
    fn from(bytes: &[u8]) -> Self {
        let mut offset = 0;
        let mut segment_override: Option<SegmentRegister> = None;

        loop {
            if bytes[offset] >> 5 == 0b001 && bytes[offset] & 0b00000111 == 0b110 {
                segment_override = Some(SegmentRegister::from((bytes[offset] & 0b00011000) >> 3));
            } else {
                break;
            }

            offset += 1;
        }

        if bytes[offset] >> 2 == 0b100010 {
            let direction = Direction::from((bytes[offset] & 0b00000010) >> 1);

            let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override, offset);

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
        } else if bytes[offset] >> 1 == 0b1100011 {
            let mod_rm = decode_modrm(bytes, ModRMType::Immediate(false), segment_override, offset);

            return Self {
                data: InstructionData::MOV {
                    width: mod_rm.width,
                    destination: mod_rm.rm,
                    source: mod_rm.reg,
                },
                size: mod_rm.size,
            };
        } else if bytes[offset] >> 4 == 0b1011 {
            let width = Width::from((bytes[offset] & 0b00001000) >> 3);
            let reg = bytes[offset] & 0b00000111;

            return Self {
                data: InstructionData::MOV {
                    width,
                    destination: Operand::Register(Register { width, value: reg }),
                    source: Operand::Immediate(Immediate::Unsigned(if width == Width::Byte {
                        bytes[offset + 1] as u16
                    } else {
                        u16::from_le_bytes([bytes[offset + 1], bytes[offset + 2]])
                    })),
                },
                size: if width == Width::Word { 3 } else { 2 } + offset,
            };
        } else if bytes[offset] >> 2 == 0b101000 {
            let direction = Direction::from((bytes[offset] & 0b00000010) >> 1);
            let width = Width::from(bytes[offset] & 0b00000001);
            let addr = u16::from_le_bytes([bytes[offset + 1], bytes[offset + 2]]);

            let memory = Operand::Memory(Memory {
                address: EAC::DirectAccess(addr),
                segment_override,
            });
            let register = Operand::Register(Register { width, value: 0 });

            return match direction {
                Direction::RegToRM => Self {
                    data: InstructionData::MOV {
                        width,
                        destination: register,
                        source: memory,
                    },
                    size: 3 + offset,
                },
                Direction::RMToReg => Self {
                    data: InstructionData::MOV {
                        width,
                        destination: memory,
                        source: register,
                    },
                    size: 3 + offset,
                },
            };
        } else if bytes[offset] >> 2 == 0b100011 {
            let direction = Direction::from((bytes[offset] & 0b00000010) >> 1);

            let mod_rm = decode_modrm(bytes, ModRMType::SegmentRegister, segment_override, offset);

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
        } else if bytes[offset] & 0b11000100 == 0 {
            let direction = Direction::from((bytes[offset] & 0b00000010) >> 1);
            let literal = (bytes[offset] & 0b00111000) >> 3;

            let op_code = get_alu_binary_op_code(literal);
            let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override, offset);

            return match direction {
                Direction::RegToRM => Self {
                    data: InstructionData::AluBinary {
                        op_code,
                        width: mod_rm.width,
                        destination: mod_rm.rm,
                        source: mod_rm.reg,
                    },
                    size: mod_rm.size,
                },
                Direction::RMToReg => Self {
                    data: InstructionData::AluBinary {
                        op_code,
                        width: mod_rm.width,
                        destination: mod_rm.reg,
                        source: mod_rm.rm,
                    },
                    size: mod_rm.size,
                },
            };
        } else if bytes[offset] >> 1 == 0b1111111 || bytes[offset] >> 1 == 0b1111011 {
            let literal = (bytes[offset + 1] & 0b00111000) >> 3;

            let op_code = get_alu_unary_op_code(literal);
            let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override, offset);

            return Self {
                data: InstructionData::AluUnary {
                    op_code,
                    width: mod_rm.width,
                    operand: mod_rm.rm,
                },
                size: mod_rm.size,
            };
        } else if bytes[offset] >> 6 == 0b10 && bytes[offset] & 0b00000100 == 0 {
            let sign_extend = bytes[offset] & 0b00000010 != 0;
            let literal = (bytes[offset + 1] & 0b00111000) >> 3;

            let op_code = get_alu_binary_op_code(literal);
            let mod_rm = decode_modrm(
                bytes,
                ModRMType::Immediate(sign_extend),
                segment_override,
                offset,
            );

            return Self {
                data: InstructionData::AluBinary {
                    op_code,
                    width: mod_rm.width,
                    destination: mod_rm.rm,
                    source: mod_rm.reg,
                },
                size: mod_rm.size,
            };
        } else if bytes[offset] >> 6 == 0b00 && (bytes[offset] & 0b00000110) >> 1 == 0b10 {
            let width = Width::from(bytes[offset] & 0b00000001);
            let literal = (bytes[offset] & 0b00111000) >> 3;

            let op_code = get_alu_binary_op_code(literal);

            let (data, size) = match width {
                Width::Byte => (bytes[offset + 1] as u16, 2),
                Width::Word => (
                    u16::from_le_bytes([bytes[offset + 1], bytes[offset + 2]]),
                    3,
                ),
            };

            let immediate = Operand::Immediate(Immediate::Unsigned(data));
            let register = Operand::Register(Register { width, value: 0 });

            return Self {
                data: InstructionData::AluBinary {
                    op_code,
                    width,
                    destination: register,
                    source: immediate,
                },
                size,
            };
        } else if let Some(op_code) = get_conditional_jump_op_code(bytes[offset]) {
            return Self {
                data: InstructionData::Jump {
                    op_code,
                    target: bytes[offset + 1] as i8 + 2,
                },
                size: offset + 2,
            };
        } else if bytes[offset] >> 6 == 0b01 {
            let literal = (bytes[offset] & 0b00111000) >> 3;
            let reg = bytes[offset] & 0b00000111;

            return Self {
                data: InstructionData::AluUnary {
                    op_code: get_alu_unary_op_code(literal),
                    width: Width::Word,
                    operand: Operand::Register(Register {
                        width: Width::Word,
                        value: reg,
                    }),
                },
                size: offset + 1,
            };
        } else if bytes[offset] == 0b11110100 {
            return Self {
                data: InstructionData::HLT,
                size: 1,
            };
        }

        unreachable!("operation not found: {:08b}", bytes[0])
    }
}

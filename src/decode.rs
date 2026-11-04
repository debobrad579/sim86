use crate::{
    instruction::{Instruction, Memory, Operand, Register, Width},
    memory::decode_modrm,
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

            let mod_rm = decode_modrm(bytes, false);

            return match direction {
                Direction::RegToRM => Self {
                    op_code: OpCode::MOV,
                    width: mod_rm.width,
                    destination: mod_rm.rm,
                    source: mod_rm.reg,
                    size: mod_rm.size,
                },
                Direction::RMToReg => Self {
                    op_code: OpCode::MOV,
                    width: mod_rm.width,
                    destination: mod_rm.reg,
                    source: mod_rm.rm,
                    size: mod_rm.size,
                },
            };
        } else if bytes[0] >> 1 == 0b1100011 {
            let mod_rm = decode_modrm(bytes, true);

            return Self {
                op_code: OpCode::MOV,
                width: mod_rm.width,
                destination: mod_rm.rm,
                source: mod_rm.reg,
                size: mod_rm.size,
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
        } else if bytes[0] >> 2 == 0b101000 {
            let direction = Direction::from((bytes[0] & 0b00000010) >> 1);
            let width = Width::from(bytes[0] & 0b00000001);
            let addr = u16::from_le_bytes([bytes[1], bytes[2]]);

            let memory = Operand::Memory(Memory::DirectAccess(addr));
            let register = Operand::Register(Register { width, value: 0 });

            return match direction {
                Direction::RegToRM => Self {
                    op_code: OpCode::MOV,
                    width,
                    destination: register,
                    source: memory,
                    size: 3,
                },
                Direction::RMToReg => Self {
                    op_code: OpCode::MOV,
                    width,
                    destination: memory,
                    source: register,
                    size: 3,
                },
            };
        }

        unreachable!("operation not found: {:08b}", bytes[0])
    }
}

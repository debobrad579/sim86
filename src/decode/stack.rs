use crate::{
    decode::memory::{ModRMType, decode_modrm},
    types::{
        instruction::{Instruction, InstructionData},
        operand::{Operand, Register, SegmentRegister},
        width::Width,
    },
};

pub fn push_rm(bytes: &[u8], segment_override: Option<SegmentRegister>) -> Instruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    Instruction {
        data: InstructionData::PUSH { operand: mod_rm.rm },
        size: mod_rm.size,
    }
}

pub fn push_reg(bytes: &[u8]) -> Instruction {
    Instruction {
        data: InstructionData::PUSH {
            operand: Operand::Register(Register {
                width: Width::Word,
                value: bytes[0] & 0b111,
            }),
        },
        size: 1,
    }
}

pub fn push_sr(bytes: &[u8]) -> Instruction {
    Instruction {
        data: InstructionData::PUSH {
            operand: Operand::SegmentRegistry(SegmentRegister::from((bytes[0] >> 3) & 0b11)),
        },
        size: 1,
    }
}

pub fn pop_rm(bytes: &[u8], segment_override: Option<SegmentRegister>) -> Instruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    Instruction {
        data: InstructionData::POP { operand: mod_rm.rm },
        size: mod_rm.size,
    }
}

pub fn pop_reg(bytes: &[u8]) -> Instruction {
    Instruction {
        data: InstructionData::POP {
            operand: Operand::Register(Register {
                width: Width::Word,
                value: bytes[0] & 0b111,
            }),
        },
        size: 1,
    }
}

pub fn pop_sr(bytes: &[u8]) -> Instruction {
    Instruction {
        data: InstructionData::POP {
            operand: Operand::SegmentRegistry(SegmentRegister::from((bytes[0] >> 3) & 0b11)),
        },
        size: 1,
    }
}

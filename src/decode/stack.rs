use crate::{
    decode::memory::{ModRMType, decode_modrm},
    types::{
        instruction::{Instruction, InstructionData},
        jump::Jump,
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

pub fn pushf() -> Instruction {
    Instruction {
        data: InstructionData::PUSHF,
        size: 1,
    }
}

pub fn popf() -> Instruction {
    Instruction {
        data: InstructionData::POPF,
        size: 1,
    }
}

pub fn call_direct_within_segment(bytes: &[u8]) -> Instruction {
    Instruction {
        data: InstructionData::CALL(Jump::DirectWithinSegment(u16::from_le_bytes([
            bytes[1], bytes[2],
        ]) as i16)),
        size: 3,
    }
}

pub fn call_indirect_within_segment(
    bytes: &[u8],
    segment_override: Option<SegmentRegister>,
) -> Instruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    Instruction {
        data: InstructionData::CALL(Jump::IndirectWithinSegment(mod_rm.rm)),
        size: mod_rm.size,
    }
}

pub fn call_direct_intersegment(bytes: &[u8]) -> Instruction {
    Instruction {
        data: InstructionData::CALL(Jump::DirectIntersegment {
            ip: u16::from_le_bytes([bytes[1], bytes[2]]),
            cs: u16::from_le_bytes([bytes[3], bytes[4]]),
        }),
        size: 5,
    }
}

pub fn call_indirect_intersegment(
    bytes: &[u8],
    segment_override: Option<SegmentRegister>,
) -> Instruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    if let Operand::Memory(memory) = mod_rm.rm {
        Instruction {
            data: InstructionData::CALL(Jump::IndirectIntersegment(memory)),
            size: mod_rm.size,
        }
    } else {
        unreachable!("indirect intersegment call must have memory operand")
    }
}

pub fn ret() -> Instruction {
    Instruction {
        data: InstructionData::RET(0),
        size: 1,
    }
}

pub fn ret_adding_immediate_to_sp(bytes: &[u8]) -> Instruction {
    Instruction {
        data: InstructionData::RET(u16::from_le_bytes([bytes[1], bytes[2]])),
        size: 3,
    }
}

pub fn retf() -> Instruction {
    Instruction {
        data: InstructionData::RETF(0),
        size: 1,
    }
}

pub fn retf_adding_immediate_to_sp(bytes: &[u8]) -> Instruction {
    Instruction {
        data: InstructionData::RETF(u16::from_le_bytes([bytes[1], bytes[2]])),
        size: 3,
    }
}

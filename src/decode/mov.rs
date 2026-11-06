use crate::{
    decode::{
        Direction,
        memory::{ModRMType, decode_modrm},
    },
    types::{
        EAC, Immediate, Instruction, InstructionData, Memory, Operand, Register, SegmentRegister,
        Width,
    },
};

pub fn reg_to_from_rm(
    bytes: &[u8],
    offset: usize,
    segment_override: Option<SegmentRegister>,
) -> Instruction {
    let direction = Direction::from((bytes[offset] & 0b00000010) >> 1);

    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override, offset);

    match direction {
        Direction::RegToRM => Instruction {
            data: InstructionData::MOV {
                width: mod_rm.width,
                destination: mod_rm.rm,
                source: mod_rm.reg,
            },
            size: mod_rm.size,
        },
        Direction::RMToReg => Instruction {
            data: InstructionData::MOV {
                width: mod_rm.width,
                destination: mod_rm.reg,
                source: mod_rm.rm,
            },
            size: mod_rm.size,
        },
    }
}

pub fn immediate_to_rm(
    bytes: &[u8],
    offset: usize,
    segment_override: Option<SegmentRegister>,
) -> Instruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Immediate(false), segment_override, offset);

    Instruction {
        data: InstructionData::MOV {
            width: mod_rm.width,
            destination: mod_rm.rm,
            source: mod_rm.reg,
        },
        size: mod_rm.size,
    }
}

pub fn immediate_to_register(bytes: &[u8], offset: usize) -> Instruction {
    let width = Width::from((bytes[offset] & 0b00001000) >> 3);
    let reg = bytes[offset] & 0b00000111;

    Instruction {
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
    }
}

pub fn memory_to_from_accumulator(
    bytes: &[u8],
    offset: usize,
    segment_override: Option<SegmentRegister>,
) -> Instruction {
    let direction = Direction::from((bytes[offset] & 0b00000010) >> 1);
    let width = Width::from(bytes[offset] & 0b00000001);
    let addr = u16::from_le_bytes([bytes[offset + 1], bytes[offset + 2]]);

    let memory = Operand::Memory(Memory {
        address: EAC::DirectAccess(addr),
        segment_override,
    });
    let register = Operand::Register(Register { width, value: 0 });

    match direction {
        Direction::RegToRM => Instruction {
            data: InstructionData::MOV {
                width,
                destination: register,
                source: memory,
            },
            size: 3 + offset,
        },
        Direction::RMToReg => Instruction {
            data: InstructionData::MOV {
                width,
                destination: memory,
                source: register,
            },
            size: 3 + offset,
        },
    }
}

pub fn sr_to_from_rm(
    bytes: &[u8],
    offset: usize,
    segment_override: Option<SegmentRegister>,
) -> Instruction {
    let direction = Direction::from((bytes[offset] & 0b00000010) >> 1);

    let mod_rm = decode_modrm(bytes, ModRMType::SegmentRegister, segment_override, offset);

    match direction {
        Direction::RegToRM => Instruction {
            data: InstructionData::MOV {
                width: mod_rm.width,
                destination: mod_rm.rm,
                source: mod_rm.reg,
            },
            size: mod_rm.size,
        },
        Direction::RMToReg => Instruction {
            data: InstructionData::MOV {
                width: mod_rm.width,
                destination: mod_rm.reg,
                source: mod_rm.rm,
            },
            size: mod_rm.size,
        },
    }
}

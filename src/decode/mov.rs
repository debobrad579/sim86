use crate::{
    decode::{
        DecodeInstruction, Direction,
        memory::{ModRMType, decode_modrm},
    },
    types::{
        instruction::InstructionData,
        operand::{
            EffectiveAddressCalculation, Immediate, Memory, Operand, Register, SegmentRegister,
        },
        width::Width,
    },
};

pub fn reg_to_from_rm(
    bytes: &[u8],
    segment_override: Option<SegmentRegister>,
) -> DecodeInstruction {
    let direction = Direction::from((bytes[0] & 0b00000010) >> 1);

    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    match direction {
        Direction::RegToRM => DecodeInstruction {
            data: InstructionData::MOV {
                width: mod_rm.width,
                destination: mod_rm.rm,
                source: mod_rm.reg,
            },
            size: mod_rm.size,
        },
        Direction::RMToReg => DecodeInstruction {
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
    segment_override: Option<SegmentRegister>,
) -> DecodeInstruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Immediate(false), segment_override);

    DecodeInstruction {
        data: InstructionData::MOV {
            width: mod_rm.width,
            destination: mod_rm.rm,
            source: mod_rm.reg,
        },
        size: mod_rm.size,
    }
}

pub fn immediate_to_register(bytes: &[u8]) -> DecodeInstruction {
    let width = Width::from((bytes[0] & 0b00001000) >> 3);
    let reg = bytes[0] & 0b00000111;

    DecodeInstruction {
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
    }
}

pub fn memory_to_from_accumulator(
    bytes: &[u8],
    segment_override: Option<SegmentRegister>,
) -> DecodeInstruction {
    let direction = Direction::from((bytes[0] & 0b00000010) >> 1);
    let width = Width::from(bytes[0] & 0b00000001);
    let addr = u16::from_le_bytes([bytes[1], bytes[2]]);

    let memory = Operand::Memory(Memory {
        address: EffectiveAddressCalculation::DirectAccess(addr),
        segment_override,
    });
    let register = Operand::Register(Register { width, value: 0 });

    match direction {
        Direction::RegToRM => DecodeInstruction {
            data: InstructionData::MOV {
                width,
                destination: register,
                source: memory,
            },
            size: 3,
        },
        Direction::RMToReg => DecodeInstruction {
            data: InstructionData::MOV {
                width,
                destination: memory,
                source: register,
            },
            size: 3,
        },
    }
}

pub fn sr_to_from_rm(bytes: &[u8], segment_override: Option<SegmentRegister>) -> DecodeInstruction {
    let direction = Direction::from((bytes[0] & 0b00000010) >> 1);

    let mod_rm = decode_modrm(bytes, ModRMType::SegmentRegister, segment_override);

    match direction {
        Direction::RegToRM => DecodeInstruction {
            data: InstructionData::MOV {
                width: mod_rm.width,
                destination: mod_rm.rm,
                source: mod_rm.reg,
            },
            size: mod_rm.size,
        },
        Direction::RMToReg => DecodeInstruction {
            data: InstructionData::MOV {
                width: mod_rm.width,
                destination: mod_rm.reg,
                source: mod_rm.rm,
            },
            size: mod_rm.size,
        },
    }
}

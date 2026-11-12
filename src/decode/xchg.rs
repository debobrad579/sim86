use crate::{
    decode::{
        DecodeInstruction,
        memory::{ModRMType, decode_modrm},
    },
    types::{
        instruction::InstructionData,
        operand::{Operand, Register, SegmentRegister},
        width::Width,
    },
};

pub fn rm_with_register(
    bytes: &[u8],
    segment_override: Option<SegmentRegister>,
) -> DecodeInstruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    DecodeInstruction {
        data: InstructionData::XCHG {
            width: mod_rm.width,
            destination: mod_rm.reg,
            source: mod_rm.rm,
        },
        size: mod_rm.size,
    }
}

pub fn register_with_accumulator(bytes: &[u8]) -> DecodeInstruction {
    let width = Width::Word;

    DecodeInstruction {
        data: InstructionData::XCHG {
            width,
            destination: Operand::Register(Register { width, value: 0 }),
            source: Operand::Register(Register {
                width,
                value: bytes[0] & 0b111,
            }),
        },
        size: 1,
    }
}

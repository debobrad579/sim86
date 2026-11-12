use crate::{
    decode::DecodeInstruction,
    types::{instruction::InstructionData, operand::SegmentRegister},
};

pub fn xlat(segment_override: Option<SegmentRegister>) -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::XLAT { segment_override },
        size: 1,
    }
}

pub fn cbw() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::CBW,
        size: 1,
    }
}

use crate::types::{
    instruction::{Instruction, InstructionData},
    operand::SegmentRegister,
};

pub fn xlat(segment_override: Option<SegmentRegister>) -> Instruction {
    Instruction {
        data: InstructionData::XLAT { segment_override },
        size: 1,
    }
}

pub fn cbw() -> Instruction {
    Instruction {
        data: InstructionData::CBW,
        size: 1,
    }
}

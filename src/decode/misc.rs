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

pub fn aaa() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::AAA,
        size: 1,
    }
}

pub fn daa() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::DAA,
        size: 1,
    }
}

pub fn aas() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::AAS,
        size: 1,
    }
}

pub fn das() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::DAS,
        size: 1,
    }
}

pub fn aam() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::AAM,
        size: 2,
    }
}

pub fn aad() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::AAD,
        size: 2,
    }
}

pub fn cbw() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::CBW,
        size: 1,
    }
}

pub fn cwd() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::CWD,
        size: 1,
    }
}

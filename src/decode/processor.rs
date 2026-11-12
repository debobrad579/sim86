use crate::{decode::DecodeInstruction, types::instruction::InstructionData};

pub fn clear_carry() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::CLC,
        size: 1,
    }
}

pub fn complement_carry() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::CMC,
        size: 1,
    }
}

pub fn set_carry() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::STC,
        size: 1,
    }
}

pub fn clear_direction() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::CLD,
        size: 1,
    }
}

pub fn set_direction() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::STD,
        size: 1,
    }
}

pub fn clear_interrupt() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::CLI,
        size: 1,
    }
}

pub fn set_interrupt() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::STI,
        size: 1,
    }
}

pub fn halt() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::HLT,
        size: 1,
    }
}

pub fn wait() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::WAIT,
        size: 1,
    }
}

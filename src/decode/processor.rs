use crate::types::instruction::{Instruction, InstructionData};

pub fn clear_carry() -> Instruction {
    Instruction {
        data: InstructionData::CLC,
        size: 1,
    }
}

pub fn complement_carry() -> Instruction {
    Instruction {
        data: InstructionData::CMC,
        size: 1,
    }
}

pub fn set_carry() -> Instruction {
    Instruction {
        data: InstructionData::STC,
        size: 1,
    }
}

pub fn clear_direction() -> Instruction {
    Instruction {
        data: InstructionData::CLD,
        size: 1,
    }
}

pub fn set_direction() -> Instruction {
    Instruction {
        data: InstructionData::STD,
        size: 1,
    }
}

pub fn clear_interrupt() -> Instruction {
    Instruction {
        data: InstructionData::CLI,
        size: 1,
    }
}

pub fn set_interrupt() -> Instruction {
    Instruction {
        data: InstructionData::STI,
        size: 1,
    }
}

pub fn halt() -> Instruction {
    Instruction {
        data: InstructionData::HLT,
        size: 1,
    }
}

pub fn wait() -> Instruction {
    Instruction {
        data: InstructionData::WAIT,
        size: 1,
    }
}

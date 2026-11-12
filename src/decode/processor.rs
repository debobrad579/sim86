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

pub fn interrupt(bytes: &[u8]) -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::INT(bytes[1]),
        size: 2,
    }
}

pub fn interrupt_type3() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::INT3,
        size: 1,
    }
}

pub fn interrupt_on_overflow() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::INTO,
        size: 1,
    }
}

pub fn interrupt_return() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::IRET,
        size: 1,
    }
}

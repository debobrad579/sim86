use crate::types::{Instruction, InstructionData, JumpOpCode};

pub fn conditional_jump(bytes: &[u8], op_code: JumpOpCode) -> Instruction {
    Instruction {
        data: InstructionData::Jump {
            op_code,
            target: bytes[1] as i8 + 2,
        },
        size: 2,
    }
}

pub fn halt() -> Instruction {
    Instruction {
        data: InstructionData::HLT,
        size: 1,
    }
}

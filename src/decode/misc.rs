use crate::types::{Instruction, InstructionData, JumpOpCode};

pub fn conditional_jump(bytes: &[u8], offset: usize, op_code: JumpOpCode) -> Instruction {
    Instruction {
        data: InstructionData::Jump {
            op_code,
            target: bytes[offset + 1] as i8 + 2,
        },
        size: offset + 2,
    }
}

pub fn halt(offset: usize) -> Instruction {
    Instruction {
        data: InstructionData::HLT,
        size: offset + 1,
    }
}

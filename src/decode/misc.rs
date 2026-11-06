use crate::{
    decode::memory::{ModRMType, decode_modrm},
    types::{Instruction, InstructionData, JumpOpCode, Operand},
};

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

pub fn load_effective_address(bytes: &[u8]) -> Instruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Register, None);

    let eac = match mod_rm.rm {
        Operand::Memory(memory) => memory.address,
        _ => unreachable!("second LEA operand must be effective address calculation"),
    };

    Instruction {
        data: InstructionData::LEA {
            operand: mod_rm.reg,
            eac,
        },
        size: mod_rm.size,
    }
}

use crate::{
    decode::{
        DecodeInstruction,
        memory::{ModRMType, decode_modrm},
    },
    types::{
        instruction::InstructionData,
        operand::{Operand, Register},
        width::Width,
    },
};

pub fn effective_address(bytes: &[u8]) -> DecodeInstruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Register, None);

    let eac = match mod_rm.rm {
        Operand::Memory(memory) => memory.address,
        _ => unreachable!("second LEA operand must be effective address calculation"),
    };

    DecodeInstruction {
        data: InstructionData::LEA {
            operand: mod_rm.reg,
            eac,
        },
        size: mod_rm.size,
    }
}

pub fn pointer_to_ds(bytes: &[u8]) -> DecodeInstruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Register, None);

    let eac = match mod_rm.rm {
        Operand::Memory(memory) => memory.address,
        _ => unreachable!("second LDS operand must be effective address calculation"),
    };

    DecodeInstruction {
        data: InstructionData::LDS {
            operand: mod_rm.reg,
            eac,
        },
        size: mod_rm.size,
    }
}

pub fn pointer_to_es(bytes: &[u8]) -> DecodeInstruction {
    let mod_rm = decode_modrm(bytes, ModRMType::Register, None);

    let eac = match mod_rm.rm {
        Operand::Memory(memory) => memory.address,
        _ => unreachable!("second LES operand must be effective address calculation"),
    };

    DecodeInstruction {
        data: InstructionData::LES {
            operand: Operand::Register(Register {
                width: Width::Word,
                value: (bytes[1] >> 3) & 0b111,
            }),
            eac,
        },
        size: mod_rm.size,
    }
}

pub fn ah_with_flags() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::LAHF,
        size: 1,
    }
}

pub fn ah_into_flags() -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::SAHF,
        size: 1,
    }
}

use crate::{
    decode::DecodeInstruction,
    types::{
        instruction::InstructionData,
        operand::{Immediate, Operand, Register},
        width::Width,
    },
};

pub fn in_fixed(bytes: &[u8]) -> DecodeInstruction {
    let width = Width::from(bytes[0] & 1);
    DecodeInstruction {
        data: InstructionData::IN {
            width,
            operand: Operand::Register(Register { width, value: 0 }),
            port: Operand::Immediate(Immediate::Unsigned(bytes[1] as u16)),
        },
        size: 2,
    }
}

pub fn in_variable(bytes: &[u8]) -> DecodeInstruction {
    let width = Width::from(bytes[0] & 1);
    DecodeInstruction {
        data: InstructionData::IN {
            width,
            operand: Operand::Register(Register { width, value: 0 }),
            port: Operand::Register(Register {
                width: Width::Word,
                value: 0b010,
            }),
        },
        size: 1,
    }
}

pub fn out_fixed(bytes: &[u8]) -> DecodeInstruction {
    let width = Width::from(bytes[0] & 1);
    DecodeInstruction {
        data: InstructionData::OUT {
            width,
            operand: Operand::Register(Register { width, value: 0 }),
            port: Operand::Immediate(Immediate::Unsigned(bytes[1] as u16)),
        },
        size: 2,
    }
}

pub fn out_variable(bytes: &[u8]) -> DecodeInstruction {
    let width = Width::from(bytes[0] & 1);
    DecodeInstruction {
        data: InstructionData::OUT {
            width,
            operand: Operand::Register(Register { width, value: 0 }),
            port: Operand::Register(Register {
                width: Width::Word,
                value: 0b010,
            }),
        },
        size: 1,
    }
}
